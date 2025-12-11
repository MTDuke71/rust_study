# Unsafe Rust Superpowers

*Tags: #rust #unsafe #pointers #ffi #advanced #systems-programming #memory-safety*

## Overview

**Unsafe Rust** provides five superpowers that allow developers to bypass Rust's safety guarantees when necessary for performance, low-level systems programming, or FFI (Foreign Function Interface) with other languages. These powers come with great responsibility - the programmer must manually uphold memory safety invariants.

## The Five Unsafe Superpowers

### 1. **Dereference Raw Pointers**
Raw pointers (`*const T` and `*mut T`) are similar to references but without Rust's safety guarantees.

**Key Differences from References:**
- Can be null or point to invalid memory
- Can have multiple mutable pointers to same location
- No automatic cleanup (no Drop)
- Can ignore borrowing rules
- Not guaranteed to point to valid memory

```rust
fn raw_pointer_basics() {
    let mut num = 5;
    
    // Create raw pointers from references (safe)
    let r1 = &num as *const i32;        // Immutable raw pointer
    let r2 = &mut num as *mut i32;      // Mutable raw pointer
    
    // Create raw pointers to arbitrary memory addresses (unsafe creation is safe!)
    let address = 0x012345usize;
    let r3 = address as *const i32;
    
    // Dereferencing requires unsafe block
    unsafe {
        println!("r1 points to: {}", *r1);
        *r2 = 10;  // Modify through mutable raw pointer
        println!("num is now: {}", num);
    }
}
```

**Use Cases:**
- Interfacing with C code
- Building safe abstractions over unsafe operations
- Implementing custom data structures (e.g., linked lists, trees)
- Performance-critical code avoiding bounds checks

### 2. **Call Unsafe Functions or Methods**
Functions marked `unsafe` indicate they have requirements the compiler can't verify.

```rust
unsafe fn dangerous() {
    // This function has invariants that must be upheld by caller
    println!("Doing something dangerous!");
}

fn call_unsafe_function() {
    unsafe {
        dangerous();  // Caller must ensure safety invariants
    }
}
```

**Creating Safe Abstractions:**
```rust
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();  // Get raw pointer
    
    assert!(mid <= len);  // Safety check
    
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// This safe function internally uses unsafe code but provides safe API
fn example_safe_abstraction() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);
    
    assert_eq!(left, &mut [1, 2, 3]);
    assert_eq!(right, &mut [4, 5, 6]);
}
```

### 3. **Access or Modify Mutable Static Variables**
Global mutable state requires unsafe access due to data race potential.

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;  // Unsafe: potential data race
    }
}

fn get_count() -> u32 {
    unsafe {
        COUNTER  // Reading mutable static is also unsafe
    }
}

// Better alternative: use thread-safe types
use std::sync::atomic::{AtomicU32, Ordering};

static SAFE_COUNTER: AtomicU32 = AtomicU32::new(0);

fn safe_add_to_count(inc: u32) {
    SAFE_COUNTER.fetch_add(inc, Ordering::SeqCst);  // Safe!
}
```

### 4. **Implement Unsafe Traits**
Some traits are marked `unsafe` because implementing them incorrectly can cause undefined behavior.

```rust
unsafe trait Foo {
    // Methods that have safety requirements
}

unsafe impl Foo for i32 {
    // Implementation must uphold safety invariants
}
```

**Example: Send and Sync**
```rust
use std::cell::UnsafeCell;

// Send: safe to transfer ownership between threads
// Sync: safe to share references between threads

struct MyBox<T>(*mut T);

// SAFETY: We ensure T is Send, so transferring ownership is safe
unsafe impl<T: Send> Send for MyBox<T> {}

// SAFETY: We never provide aliased mutable access from multiple threads
// (This is a simplified example - real implementation would need more care)
unsafe impl<T: Sync> Sync for MyBox<T> {}
```

### 5. **Access Fields of Unions**
Unions allow different types to share the same memory location.

```rust
union MyUnion {
    f: f32,
    i: i32,
}

fn union_example() {
    let u = MyUnion { f: 1.0 };
    
    unsafe {
        // Reading union fields is unsafe - programmer must know which is valid
        println!("f32: {}", u.f);
        println!("i32: {}", u.i);  // Reinterpreting bits as i32
    }
}

// Common use case: Type punning for bit manipulation
#[repr(C)]
union FloatBytes {
    float_value: f32,
    byte_array: [u8; 4],
}

fn float_to_bytes(f: f32) -> [u8; 4] {
    let fb = FloatBytes { float_value: f };
    unsafe { fb.byte_array }
}
```

## FFI (Foreign Function Interface)

### Calling C Code from Rust

**Declaring External Functions:**
```rust
extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(x: f64) -> f64;
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

fn call_c_functions() {
    unsafe {
        let result = abs(-42);
        println!("abs(-42) = {}", result);
        
        let root = sqrt(9.0);
        println!("sqrt(9.0) = {}", root);
    }
}
```

**Working with C Strings:**
```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
}

fn c_string_example() {
    // Rust string to C string
    let rust_str = "Hello, FFI!";
    let c_string = CString::new(rust_str).expect("CString::new failed");
    
    unsafe {
        let len = strlen(c_string.as_ptr());
        println!("String length: {}", len);
    }
    
    // C string to Rust string
    unsafe {
        let c_str = CStr::from_ptr(c_string.as_ptr());
        let rust_str_back = c_str.to_str().expect("Invalid UTF-8");
        println!("Back to Rust: {}", rust_str_back);
    }
}
```

**Creating Rust Library for C:**
```rust
// src/lib.rs
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]  // Don't mangle function name for C linkage
pub extern "C" fn rust_greeting(name: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };
    
    let rust_str = c_str.to_str().unwrap();
    let greeting = format!("Hello from Rust, {}!", rust_str);
    
    // Allocate C-compatible string
    let c_string = std::ffi::CString::new(greeting).unwrap();
    c_string.into_raw()  // Transfer ownership to C
}

#[no_mangle]
pub extern "C" fn rust_greeting_free(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        // Take ownership back and drop
        let _ = std::ffi::CString::from_raw(ptr);
    }
}
```

**Cargo.toml for C Library:**
```toml
[lib]
crate-type = ["cdylib"]  # Create dynamic library for C

# Or for static library:
# crate-type = ["staticlib"]
```

### Advanced FFI Patterns

**Opaque Pointers (Handles):**
```rust
// Don't expose internal structure to C
pub struct RustObject {
    data: Vec<u8>,
    count: usize,
}

#[no_mangle]
pub extern "C" fn rust_object_new() -> *mut RustObject {
    let obj = Box::new(RustObject {
        data: Vec::new(),
        count: 0,
    });
    Box::into_raw(obj)  // Transfer to C as opaque pointer
}

#[no_mangle]
pub extern "C" fn rust_object_add_data(ptr: *mut RustObject, value: u8) {
    let obj = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    obj.data.push(value);
    obj.count += 1;
}

#[no_mangle]
pub extern "C" fn rust_object_free(ptr: *mut RustObject) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);  // Drop and free
        }
    }
}
```

**Callbacks from C to Rust:**
```rust
type Callback = extern "C" fn(i32) -> i32;

extern "C" {
    fn register_callback(cb: Callback);
    fn trigger_callback(value: i32) -> i32;
}

extern "C" fn my_callback(x: i32) -> i32 {
    println!("Callback called with: {}", x);
    x * 2
}

fn callback_example() {
    unsafe {
        register_callback(my_callback);
        let result = trigger_callback(21);
        println!("Result: {}", result);
    }
}
```

## Safety Guidelines and Best Practices

### 1. **Minimize Unsafe Scope**
Keep `unsafe` blocks as small as possible:

```rust
// ❌ Bad: Large unsafe block
unsafe {
    let ptr = get_raw_pointer();
    do_safe_operation_1();
    do_safe_operation_2();
    *ptr = 42;
    do_safe_operation_3();
}

// ✅ Good: Minimal unsafe scope
let ptr = get_raw_pointer();
do_safe_operation_1();
do_safe_operation_2();
unsafe {
    *ptr = 42;
}
do_safe_operation_3();
```

### 2. **Document Safety Invariants**
Always explain why unsafe code is safe:

```rust
/// # Safety
///
/// `ptr` must be:
/// - Non-null
/// - Properly aligned for type T
/// - Valid for reads of size_of::<T>() bytes
/// - Pointing to initialized data
unsafe fn read_from_ptr<T>(ptr: *const T) -> T {
    std::ptr::read(ptr)
}
```

### 3. **Create Safe Abstractions**
Wrap unsafe code in safe APIs:

```rust
pub struct SafeBuffer {
    ptr: *mut u8,
    len: usize,
    capacity: usize,
}

impl SafeBuffer {
    pub fn new(capacity: usize) -> Self {
        let layout = std::alloc::Layout::array::<u8>(capacity).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };
        
        SafeBuffer {
            ptr,
            len: 0,
            capacity,
        }
    }
    
    pub fn push(&mut self, value: u8) {
        assert!(self.len < self.capacity, "Buffer full");
        unsafe {
            *self.ptr.add(self.len) = value;
        }
        self.len += 1;
    }
    
    pub fn get(&self, index: usize) -> Option<u8> {
        if index < self.len {
            Some(unsafe { *self.ptr.add(index) })
        } else {
            None
        }
    }
}

impl Drop for SafeBuffer {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::array::<u8>(self.capacity).unwrap();
        unsafe {
            std::alloc::dealloc(self.ptr, layout);
        }
    }
}
```

### 4. **Use `assert!` for Runtime Checks**
Verify assumptions in unsafe code:

```rust
unsafe fn write_at_index<T>(slice: &mut [T], index: usize, value: T) {
    assert!(index < slice.len(), "Index out of bounds");
    let ptr = slice.as_mut_ptr();
    std::ptr::write(ptr.add(index), value);
}
```

### 5. **Prefer Safe Alternatives**
Consider these before reaching for unsafe:

- `std::sync::Mutex` instead of mutable statics
- `std::sync::atomic` types for lock-free operations
- `Pin` for self-referential structures
- Standard library data structures over custom unsafe implementations

## Common Unsafe Patterns

### Pattern 1: Custom Allocator
```rust
use std::alloc::{GlobalAlloc, Layout, System};

struct CountingAllocator;

static mut ALLOCATED: usize = 0;

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        if !ptr.is_null() {
            ALLOCATED += layout.size();
        }
        ptr
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED -= layout.size();
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;
```

### Pattern 2: Intrusive Data Structures
```rust
use std::ptr::NonNull;

struct Node<T> {
    data: T,
    next: Option<NonNull<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }
    
    pub fn push_back(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: None,
        });
        let new_node_ptr = NonNull::new(Box::into_raw(new_node));
        
        match self.tail {
            None => {
                self.head = new_node_ptr;
                self.tail = new_node_ptr;
            }
            Some(mut tail) => {
                unsafe {
                    tail.as_mut().next = new_node_ptr;
                }
                self.tail = new_node_ptr;
            }
        }
        
        self.len += 1;
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(node) = self.head {
            unsafe {
                let boxed = Box::from_raw(node.as_ptr());
                self.head = boxed.next;
            }
        }
    }
}
```

### Pattern 3: Zero-Copy Parsing
```rust
use std::slice;

#[repr(C)]
struct Header {
    magic: u32,
    length: u32,
    checksum: u32,
}

fn parse_header(data: &[u8]) -> Option<&Header> {
    if data.len() < std::mem::size_of::<Header>() {
        return None;
    }
    
    let ptr = data.as_ptr() as *const Header;
    
    // Check alignment
    if ptr.align_offset(std::mem::align_of::<Header>()) != 0 {
        return None;
    }
    
    Some(unsafe { &*ptr })
}
```

## Performance Considerations

### When Unsafe Helps Performance:

1. **Eliminating Bounds Checks:**
```rust
// Safe version with bounds checks
fn sum_safe(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..slice.len() {
        sum += slice[i];  // Bounds check on each access
    }
    sum
}

// Unsafe version without bounds checks
fn sum_unsafe(slice: &[i32]) -> i32 {
    let mut sum = 0;
    let ptr = slice.as_ptr();
    let len = slice.len();
    
    for i in 0..len {
        sum += unsafe { *ptr.add(i) };  // No bounds check
    }
    sum
}

// Best: Let compiler optimize safe code
fn sum_iterator(slice: &[i32]) -> i32 {
    slice.iter().sum()  // Often optimizes to same assembly as unsafe
}
```

2. **Avoiding Initialization:**
```rust
use std::mem::MaybeUninit;

fn create_large_array_safe() -> [i32; 1000] {
    [0; 1000]  // Initializes all elements to 0
}

fn create_large_array_unsafe() -> [i32; 1000] {
    let mut arr: [MaybeUninit<i32>; 1000] = unsafe {
        MaybeUninit::uninit().assume_init()
    };
    
    for (i, elem) in arr.iter_mut().enumerate() {
        *elem = MaybeUninit::new(i as i32);
    }
    
    unsafe { std::mem::transmute(arr) }
}
```

3. **Custom Memory Layout:**
```rust
#[repr(C)]
struct PackedData {
    flags: u8,
    value: u32,
    // Natural alignment would waste 3 bytes padding
}

#[repr(packed)]
struct TightlyPacked {
    flags: u8,
    value: u32,  // No padding, but unaligned access!
}

fn access_packed(data: &TightlyPacked) -> u32 {
    // Direct field access would be undefined behavior
    unsafe {
        std::ptr::read_unaligned(&data.value)
    }
}
```

## Tools for Unsafe Code

### Miri - Undefined Behavior Detector
```bash
# Install Miri
rustup +nightly component add miri

# Run tests under Miri
cargo +nightly miri test
```

### AddressSanitizer
```bash
# Detect memory errors
RUSTFLAGS="-Z sanitizer=address" cargo +nightly run
```

### ThreadSanitizer
```bash
# Detect data races
RUSTFLAGS="-Z sanitizer=thread" cargo +nightly run
```

## Real-World Examples

### Example 1: Vec Implementation (Simplified)
```rust
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

pub struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        MyVec {
            ptr: std::ptr::NonNull::dangling().as_ptr(),
            len: 0,
            capacity: 0,
        }
    }
    
    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }
        
        unsafe {
            ptr::write(self.ptr.add(self.len), value);
        }
        self.len += 1;
    }
    
    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
        let new_layout = Layout::array::<T>(new_capacity).unwrap();
        
        let new_ptr = if self.capacity == 0 {
            unsafe { alloc(new_layout) as *mut T }
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                std::alloc::realloc(
                    self.ptr as *mut u8,
                    old_layout,
                    new_layout.size()
                ) as *mut T
            }
        };
        
        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(unsafe { &*self.ptr.add(index) })
        } else {
            None
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            for i in 0..self.len {
                unsafe {
                    ptr::drop_in_place(self.ptr.add(i));
                }
            }
            
            let layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}
```

### Example 2: Memory-Mapped File
```rust
use std::fs::File;
use std::io::Result;
use std::slice;

#[cfg(unix)]
use std::os::unix::io::AsRawFd;

struct MemoryMappedFile {
    ptr: *mut u8,
    len: usize,
}

impl MemoryMappedFile {
    #[cfg(unix)]
    pub fn open(file: &File, len: usize) -> Result<Self> {
        use libc::{mmap, PROT_READ, MAP_SHARED, MAP_FAILED};
        
        let ptr = unsafe {
            mmap(
                std::ptr::null_mut(),
                len,
                PROT_READ,
                MAP_SHARED,
                file.as_raw_fd(),
                0,
            )
        };
        
        if ptr == MAP_FAILED {
            return Err(std::io::Error::last_os_error());
        }
        
        Ok(MemoryMappedFile {
            ptr: ptr as *mut u8,
            len,
        })
    }
    
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl Drop for MemoryMappedFile {
    #[cfg(unix)]
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, self.len);
        }
    }
}
```

## Key Takeaways

1. **Unsafe is a scalpel, not a sledgehammer** - Use minimally and purposefully
2. **Document safety invariants** - Future you will thank present you
3. **Create safe abstractions** - Hide unsafe implementation details behind safe APIs
4. **Test rigorously** - Use Miri, sanitizers, and extensive testing
5. **Consider alternatives first** - Safe Rust is often fast enough
6. **FFI requires unsafe** - But wrap it in safe interfaces immediately
7. **Unsafe ≠ uncontrolled** - You're still responsible for correctness

## Common Pitfalls to Avoid

- ❌ Dereferencing null or dangling pointers
- ❌ Creating multiple mutable aliases
- ❌ Reading uninitialized memory
- ❌ Breaking pointer aliasing rules (violating LLVM assumptions)
- ❌ Using freed memory (use-after-free)
- ❌ Buffer overflows with raw pointer arithmetic
- ❌ Data races with mutable statics
- ❌ Incorrect alignment assumptions
- ❌ Memory leaks from forgetting to free allocated memory

## Further Reading

- **The Rustonomicon**: Official guide to unsafe Rust programming
- **Rust FFI Omnibus**: Collection of FFI examples for many languages
- **`bindgen`**: Automatically generate Rust FFI bindings from C headers
- **`cbindgen`**: Generate C/C++ headers from Rust code

---

*Links: [[rust-book-ch19]] | [[rust-ownership]] | [[rust-lifetimes]] | [[ffi-patterns]] | [[systems-programming]] | [[memory-management]] | [[concurrency-patterns]]*
