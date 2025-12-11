# Rust FFI Linking Deep Dive

*Tags: #rust #ffi #linking #compilation #systems-programming #internals #deep-dive #nerdy*

## Overview

When you declare an `extern "C"` function in Rust, you're not compiling C code - you're creating a **symbol reference** that gets resolved at link time. This is the magic behind FFI (Foreign Function Interface) and understanding it reveals how compiled programs actually work under the hood.

## The Three-Stage Journey: Compilation → Linking → Loading

### Stage 1: Compilation (Rust → Object Code)

When Rust encounters:
```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe {
    let result = abs(-3);
}
```

The compiler does **NOT**:
- ❌ Look for `abs` implementation
- ❌ Compile any C code
- ❌ Check if `abs` actually exists

The compiler **DOES**:
- ✅ Generate a **call instruction** to an external symbol
- ✅ Add `abs` to the **unresolved symbols table**
- ✅ Trust you that it exists somewhere
- ✅ Create the calling convention setup (C ABI)

**Compiler Output (Conceptual Assembly):**
```asm
; Your Rust code
mov edi, -3           ; Move argument into register (C calling convention)
call abs              ; Call external symbol (unresolved)
mov [result], eax     ; Store return value
```

The object file contains:
```
UNDEFINED SYMBOLS:
  abs  (external, needs linking)
  
DEFINED SYMBOLS:
  main
  demonstrate_unsafe_functions
  ...
```

### Stage 2: Linking (Object Files → Executable)

The linker's job is to **resolve all undefined symbols**:

```
Input:
  - your_program.o    (contains: "I need abs!")
  - libc.lib/msvcrt.lib (contains: "I provide abs")
  
Linker Process:
  1. Scan your object file for undefined symbols
  2. Search library paths for those symbols
  3. Create import table entries
  4. Generate final executable with references
  
Output:
  - ch20_1_unsafe_rust.exe (contains: import table pointing to DLL)
```

**On Windows**, the linker finds `abs` in the Universal C Runtime (UCRT):
```
api-ms-win-crt-utility-l1-1-0.dll
  Exports:
    Ordinal 15: abs
```

**Real Evidence from Your Executable:**
```powershell
PS> rust-objdump -p target\debug\examples\ch20_1_unsafe_rust.exe | 
    Select-String -Pattern "DLL Name|abs" -Context 0,1

> DLL Name: api-ms-win-crt-utility-l1-1-0.dll
  Hint/Ord  Name
>       15  abs
```

### Stage 3: Runtime Loading (OS Loader)

When you run `ch20_1_unsafe_rust.exe`:

1. **Windows PE Loader** reads the import table
2. **Locates required DLLs** in system paths
3. **Loads DLLs into memory** if not already loaded
4. **Resolves addresses** - patches the call instruction with actual memory address
5. **Your code runs** - `call abs` now jumps to the real implementation

```
Before Loading (in executable):
  call [Import Address Table entry for abs]
  
After Loading (in memory):
  call 0x7FF8A2B40123  ; Actual address of abs in UCRT DLL
```

## Platform Differences

### Windows (MSVC Toolchain)

**C Runtime**: Universal C Runtime (UCRT)
- Split into multiple API sets: `api-ms-win-crt-*.dll`
- `abs` lives in: `api-ms-win-crt-utility-l1-1-0.dll`
- Actual implementation: in `ucrtbase.dll` (forwarding)

**Link-time**: 
```toml
# Cargo automatically links against msvcrt
# Via rustc's default-libs
```

**Tools**:
```powershell
rust-objdump -p executable.exe    # View imports/exports
rust-nm executable.exe             # View symbols
dumpbin /IMPORTS executable.exe   # MSVC tool (if available)
```

### Linux (GNU Toolchain)

**C Runtime**: GNU C Library (glibc)
- Single library: `libc.so.6`
- `abs` in: `/lib/x86_64-linux-gnu/libc.so.6`

**Link-time**:
```bash
# Rust automatically links against libc
rustc main.rs -C link-arg=-lc  # Explicit if needed
```

**Tools**:
```bash
ldd executable                  # Show shared library dependencies
nm -D executable                # View dynamic symbols
objdump -T executable           # View dynamic symbol table
readelf -d executable           # View dynamic section
```

### macOS (Clang Toolchain)

**C Runtime**: libSystem (Apple's unified library)
- `abs` in: `/usr/lib/libSystem.B.dylib`
- Contains libc, libm, libpthread, etc.

**Link-time**:
```bash
# Rust automatically links against libSystem
```

**Tools**:
```bash
otool -L executable             # Show linked libraries
nm executable                   # View symbols
```

## Symbol Resolution Deep Dive

### Static vs Dynamic Linking

**Static Linking** (not typical for libc):
```rust
// If abs were statically linked:
// The compiled code of abs would be COPIED into your executable
// Executable size: LARGER
// No runtime dependency on DLL/so
```

**Dynamic Linking** (default for system libraries):
```rust
// With dynamic linking:
// Only a REFERENCE to abs is stored
// Executable size: SMALLER
// Requires DLL/so at runtime
// Multiple programs share one copy in memory
```

### The Import Address Table (IAT)

On Windows, your executable contains:
```
Import Directory:
  DLL: api-ms-win-crt-utility-l1-1-0.dll
    Import Address Table (IAT):
      [0x1400020A0] -> abs  (unresolved)
      [0x1400020A8] -> labs
      ...

Your code:
  call [0x1400020A0]  ; Indirect call through IAT
```

At runtime, the loader fills in the IAT:
```
Import Address Table (after loading):
  [0x1400020A0] -> 0x7FF8A2B40123  ; Real address of abs
```

### Symbol Mangling and extern "C"

**Why `extern "C"`?**

Rust (and C++) uses **name mangling** for function names:
```rust
fn calculate(x: i32) -> i32 { x * 2 }

// Mangled name in object file:
// _ZN7example9calculate17h8b3c9d2a1e5f4g6hE
//  ^   ^       ^         ^
//  |   |       |         |
//  |   crate   function  hash
//  Rust prefix
```

But C functions have **no mangling**:
```c
int abs(int x);

// Symbol name in object file:
// abs  (exactly as written)
```

**`extern "C"` tells Rust**:
- "Don't mangle this name"
- "Use C calling convention"
- "Look for exact name `abs` in libraries"

Without it:
```rust
extern "Rust" {  // Default - will fail!
    fn abs(input: i32) -> i32;
}
// Linker looks for: _ZN...abs...hash
// Library provides: abs
// Error: undefined reference to _ZN...abs...hash
```

## Calling Conventions

The `"C"` specifies the **calling convention** - how arguments are passed:

### x86-64 C Calling Convention (System V ABI - Linux/Mac)

**Registers for arguments (in order)**:
1. `rdi` - first integer/pointer
2. `rsi` - second integer/pointer
3. `rdx` - third integer/pointer
4. `rcx` - fourth integer/pointer
5. `r8`  - fifth integer/pointer
6. `r9`  - sixth integer/pointer
7. Stack for remaining arguments

**Return value**: `rax` register

**Example**:
```rust
extern "C" {
    fn my_func(a: i32, b: i32, c: i32) -> i32;
}

// Compiled to (x86-64 Linux):
mov edi, [a]   ; First arg
mov esi, [b]   ; Second arg
mov edx, [c]   ; Third arg
call my_func
; Return value in eax
```

### x86-64 Windows (Microsoft x64 ABI)

**Registers for arguments**:
1. `rcx` - first integer/pointer
2. `rdx` - second integer/pointer
3. `r8`  - third integer/pointer
4. `r9`  - fourth integer/pointer
5. Stack for remaining arguments

**Shadow space**: Caller must allocate 32 bytes on stack (even if unused)

**Example**:
```rust
// Same Rust code on Windows:
mov ecx, [a]   ; First arg (different register!)
mov edx, [b]   ; Second arg
mov r8d, [c]   ; Third arg
call my_func
; Return value in eax
```

### Other Calling Conventions

Rust supports multiple conventions:
```rust
extern "stdcall" { }  // Windows API standard
extern "fastcall" { } // Optimized register-based
extern "system" { }   // Platform default (C on *nix, stdcall on Win32)
extern "Rust" { }     // Rust's own convention (default)
extern "C" { }        // Standard C convention
extern "cdecl" { }    // C declaration (x86)
```

## Inspecting Your Binary

### Tools We Installed

```bash
rustup component add llvm-tools      # LLVM binary utilities
cargo install cargo-binutils         # Cargo wrappers
```

### Examining Symbols

**List all symbols**:
```bash
rust-nm target/debug/examples/ch20_1_unsafe_rust.exe
# Output: symbol table (if not stripped)
```

**View imports** (what your program needs):
```bash
rust-objdump -p target/debug/executable.exe
# Shows: Import Directory, DLL dependencies
```

**View exports** (what your program provides):
```bash
rust-objdump -p my_library.dll
# Shows: Export Directory
```

**Disassemble**:
```bash
rust-objdump -d -C target/debug/executable.exe
# Shows: Assembly code with demangled names
```

### Finding abs in Your Executable

**Full command**:
```powershell
rust-objdump -p target\debug\examples\ch20_1_unsafe_rust.exe | 
  Select-String -Pattern "DLL Name|abs" -Context 0,1
```

**Output interpretation**:
```
> DLL Name: api-ms-win-crt-utility-l1-1-0.dll
  Hint/Ord  Name
>       15  abs
```

- **DLL Name**: Which library provides the function
- **Hint/Ord 15**: Position in the export table (optimization hint)
- **Name: abs**: The exact symbol name (unmangled)

## Creating Your Own C Library

### Example: Building a C Library for Rust

**my_math.c**:
```c
#include <stdio.h>

int double_value(int x) {
    printf("C: Doubling %d\n", x);
    return x * 2;
}

int triple_value(int x) {
    printf("C: Tripling %d\n", x);
    return x * 3;
}
```

**build.rs**:
```rust
fn main() {
    // Compile C code
    cc::Build::new()
        .file("my_math.c")
        .compile("my_math");
    
    // Tell Cargo where to find it
    println!("cargo:rerun-if-changed=my_math.c");
}
```

**Cargo.toml**:
```toml
[build-dependencies]
cc = "1.0"
```

**main.rs**:
```rust
extern "C" {
    fn double_value(x: i32) -> i32;
    fn triple_value(x: i32) -> i32;
}

fn main() {
    unsafe {
        let result = double_value(21);
        println!("Rust: Got {}", result);
        
        let result = triple_value(14);
        println!("Rust: Got {}", result);
    }
}
```

**What Happens**:
1. `build.rs` runs before Rust compilation
2. `cc` crate invokes C compiler: `gcc my_math.c -c -o my_math.o`
3. Creates static library: `ar rcs libmy_math.a my_math.o`
4. Rust links against it: `rustc ... -l my_math`
5. Final executable contains C code (static linking)

**Verify**:
```bash
rust-nm target/debug/your_program | grep -E "double_value|triple_value"
# Should show: T double_value, T triple_value (defined in binary)
```

## Advanced: Creating a Rust Library for C

**lib.rs**:
```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]  // CRITICAL: Don't mangle the name!
pub extern "C" fn rust_hello(name: *const c_char) -> *mut c_char {
    // SAFETY: Caller must pass valid C string
    let c_str = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };
    
    let rust_str = c_str.to_str().unwrap();
    let greeting = format!("Hello from Rust, {}!", rust_str);
    
    // Transfer ownership to C
    CString::new(greeting).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn rust_hello_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            // Take ownership back and drop
            let _ = CString::from_raw(ptr);
        }
    }
}
```

**Cargo.toml**:
```toml
[lib]
crate-type = ["cdylib"]  # Dynamic library for C
```

**Compile**:
```bash
cargo build --release
# Creates: target/release/your_lib.dll (Windows)
#          target/release/libyour_lib.so (Linux)
#          target/release/libyour_lib.dylib (macOS)
```

**C code using it**:
```c
// rust_lib.h
extern char* rust_hello(const char* name);
extern void rust_hello_free(char* ptr);

// main.c
#include <stdio.h>
#include "rust_lib.h"

int main() {
    char* greeting = rust_hello("Alice");
    printf("%s\n", greeting);
    rust_hello_free(greeting);  // Important: free Rust memory
    return 0;
}
```

**Compile C program**:
```bash
# Linux
gcc main.c -L./target/release -lyour_lib -o main
LD_LIBRARY_PATH=./target/release ./main

# Windows
cl main.c your_lib.lib
# Copy your_lib.dll next to main.exe
main.exe
```

## Symbol Visibility and Export Tables

### Viewing Exports from Your Rust DLL

```bash
rust-objdump -p target/release/your_lib.dll

# Output:
Export Table:
  Ordinal  Name
        1  rust_hello
        2  rust_hello_free
```

### Controlling Visibility

**Rust side**:
```rust
#[no_mangle]
pub extern "C" fn public_api() { }  // Exported

#[no_mangle]
extern "C" fn private_impl() { }    // NOT exported (not pub)
```

**Windows .def file** (fine-grained control):
```
LIBRARY your_lib
EXPORTS
    rust_hello @1
    rust_hello_free @2
```

## Common Pitfalls and Solutions

### 1. Forgetting `unsafe`

```rust
extern "C" {
    fn abs(x: i32) -> i32;
}

let result = abs(-3);  // ❌ Error: call to unsafe function
```

**Why**: Rust can't verify C code's safety guarantees.

**Fix**:
```rust
let result = unsafe { abs(-3) };  // ✅
```

### 2. Wrong Calling Convention

```rust
// Wrong - uses Rust ABI
extern "Rust" {
    fn abs(x: i32) -> i32;
}

// Linker error: undefined reference to _ZN3abs...
```

**Fix**: Use `extern "C"`

### 3. Name Mangling

```rust
// Will look for mangled name in library
pub extern "C" fn my_func() { }  // Name: _ZN...my_func...

// Link error when called from C
```

**Fix**:
```rust
#[no_mangle]
pub extern "C" fn my_func() { }  // Name: my_func
```

### 4. Memory Ownership Issues

```rust
#[no_mangle]
pub extern "C" fn get_string() -> *const c_char {
    let s = CString::new("Hello").unwrap();
    s.as_ptr()  // ❌ Dangling pointer! s is dropped
}
```

**Fix**:
```rust
#[no_mangle]
pub extern "C" fn get_string() -> *mut c_char {
    let s = CString::new("Hello").unwrap();
    s.into_raw()  // ✅ Transfer ownership to C
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { let _ = CString::from_raw(ptr); }  // Take back and drop
    }
}
```

### 5. Struct Layout Mismatch

```rust
// C code
struct Point {
    int x;
    int y;
};

// Rust code - WRONG
struct Point {
    x: i32,
    y: i32,
}
```

Rust can reorder fields! **Fix**:
```rust
#[repr(C)]  // Use C layout
struct Point {
    x: i32,
    y: i32,
}
```

## Performance Considerations

### FFI Call Overhead

**Compared to native Rust call**:
- ✅ Same performance for optimized builds
- ⚠️ Prevents some inlining optimizations
- ⚠️ Crosses ABI boundary (small cost)

**Benchmark**:
```rust
// Native Rust call: ~0.3ns
fn rust_abs(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

// FFI call to C abs: ~1-2ns (includes call overhead)
extern "C" { fn abs(x: i32) -> i32; }
```

### When FFI is Worth It

✅ **Good use cases**:
- Existing optimized C libraries (BLAS, LAPACK, etc.)
- Hardware-specific code (SIMD, GPU drivers)
- Large computations (overhead amortized)
- System APIs (unavoidable)

❌ **Poor use cases**:
- Simple functions called in tight loops
- When equivalent Rust code exists
- Crossing FFI boundary repeatedly for small operations

### Optimization: Batch FFI Calls

```rust
// ❌ Bad: Many FFI calls
for i in 0..1000 {
    unsafe { c_process_single(i); }
}

// ✅ Good: One FFI call with batch
let data: Vec<i32> = (0..1000).collect();
unsafe { c_process_batch(data.as_ptr(), data.len()); }
```

## Debugging FFI Code

### Common Tools

**Valgrind** (Linux):
```bash
valgrind --leak-check=full ./your_program
# Detects: memory leaks, invalid accesses across FFI
```

**Address Sanitizer** (All platforms):
```bash
RUSTFLAGS="-Z sanitizer=address" cargo +nightly run
# Detects: use-after-free, buffer overflows, etc.
```

**GDB/LLDB** (Stepping through C code from Rust):
```bash
rust-gdb target/debug/your_program
# or
rust-lldb target/debug/your_program

(gdb) break rust_hello
(gdb) run
(gdb) step  # Steps into C code!
```

**Windows Debugger** (WinDbg):
```
windbg target\debug\your_program.exe
# Can inspect both Rust and C frames
```

### Adding Debug Symbols

**Cargo.toml**:
```toml
[profile.dev]
debug = true  # Full debug info

[profile.release]
debug = true  # Debug info in optimized builds
```

## The Binding Generator Ecosystem

### bindgen - C → Rust

Automatically generate Rust FFI bindings from C headers:

```bash
cargo install bindgen-cli
```

**my_lib.h**:
```c
int add(int a, int b);
void process(const char* str);
```

**Generate bindings**:
```bash
bindgen my_lib.h -o bindings.rs
```

**bindings.rs** (generated):
```rust
extern "C" {
    pub fn add(a: ::std::os::raw::c_int, b: ::std::os::raw::c_int) 
        -> ::std::os::raw::c_int;
    pub fn process(str: *const ::std::os::raw::c_char);
}
```

### cbindgen - Rust → C

Generate C headers from Rust code:

```bash
cargo install cbindgen
```

**lib.rs**:
```rust
#[no_mangle]
pub extern "C" fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

**Generate header**:
```bash
cbindgen --lang c -o my_rust_lib.h
```

**my_rust_lib.h** (generated):
```c
#include <stdint.h>

int32_t multiply(int32_t a, int32_t b);
```

## Real-World Examples

### Example 1: Using libsqlite3

**Cargo.toml**:
```toml
[dependencies]
libsqlite3-sys = "0.26"  # FFI bindings
```

**Usage**:
```rust
use libsqlite3_sys as ffi;
use std::ffi::CString;
use std::ptr;

fn main() {
    unsafe {
        let mut db: *mut ffi::sqlite3 = ptr::null_mut();
        let filename = CString::new("test.db").unwrap();
        
        let rc = ffi::sqlite3_open(filename.as_ptr(), &mut db);
        if rc != ffi::SQLITE_OK {
            panic!("Can't open database");
        }
        
        // Use database...
        
        ffi::sqlite3_close(db);
    }
}
```

### Example 2: SIMD via C Intrinsics

**my_simd.c**:
```c
#include <immintrin.h>

void add_arrays(const float* a, const float* b, float* result, int len) {
    for (int i = 0; i < len; i += 8) {
        __m256 va = _mm256_loadu_ps(&a[i]);
        __m256 vb = _mm256_loadu_ps(&b[i]);
        __m256 vr = _mm256_add_ps(va, vb);
        _mm256_storeu_ps(&result[i], vr);
    }
}
```

**Rust wrapper**:
```rust
extern "C" {
    fn add_arrays(a: *const f32, b: *const f32, result: *mut f32, len: i32);
}

pub fn add_vectors(a: &[f32], b: &[f32]) -> Vec<f32> {
    assert_eq!(a.len(), b.len());
    let mut result = vec![0.0; a.len()];
    
    unsafe {
        add_arrays(a.as_ptr(), b.as_ptr(), result.as_mut_ptr(), a.len() as i32);
    }
    
    result
}
```

## Summary: The Complete Picture

```
┌─────────────────────────────────────────────────────────────┐
│ 1. DECLARATION (Rust Source)                                 │
│    extern "C" { fn abs(x: i32) -> i32; }                     │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. COMPILATION (rustc)                                        │
│    - Generate machine code for Rust functions                 │
│    - Create symbol references for extern functions            │
│    - Output: object file with unresolved symbols             │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. LINKING (linker)                                           │
│    - Scan object file for undefined symbols                   │
│    - Search system libraries (msvcrt.lib, libc.a, etc.)      │
│    - Create import table entries                              │
│    - Output: executable with external references             │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. LOADING (OS Loader)                                        │
│    - Load executable into memory                              │
│    - Load required DLLs/shared libraries                      │
│    - Resolve import table (patch addresses)                   │
│    - Program ready to run!                                    │
└─────────────────────────────────────────────────────────────┘
```

## Key Insights

1. **Rust never compiles C code** - it just references it
2. **Symbol resolution happens at link time** - not compile time
3. **The OS loader patches addresses at runtime** - dynamic linking magic
4. **`extern "C"` is about ABI** - not about the language of implementation
5. **`#[no_mangle]` is critical** - for C to find Rust functions
6. **Memory ownership crosses boundaries** - you manage it manually
7. **FFI is unsafe** - but you can wrap it safely

## Further Exploration

- **The Rustonomicon**: Deep dive into unsafe Rust and FFI
- **Rust FFI Omnibus**: Practical FFI examples for many languages
- **System V ABI**: Linux/Unix calling conventions
- **PE Format**: Windows executable structure
- **ELF Format**: Linux executable structure
- **Dynamic Linking**: How shared libraries work at runtime
- **LLVM Documentation**: Understanding the compiler backend

---

*Links: [[unsafe-rust-superpowers]] | [[rust-book-ch19]] | [[systems-programming]] | [[compilation-process]] | [[linking-loading]] | [[c-interop]] | [[binary-analysis]]*
