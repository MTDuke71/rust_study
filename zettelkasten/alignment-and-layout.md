# 🧱 Memory Alignment and Layout

**Understanding memory alignment, repr attributes, and type layout optimization in Rust**

**Tags:** #memory-layout #alignment #repr-attributes #rust-for-rustaceans-ch2 #performance #ffi #abi

**Related:** [[memory-address-analysis]], [[Memory Management]], [[phantom-data-type-safety]], [[rust-concepts-MOC]]

---

## 🎯 Core Concept

**Memory alignment** determines where data can be placed in memory. **Layout** describes how struct fields are arranged and padded to satisfy alignment requirements.

Rust's type system allows you to control layout through **`repr` attributes**, balancing performance, size, and compatibility needs.

---

## 📏 What is Alignment?

### **Definition**

A type's **alignment** is the memory address boundary it must start at. An `n`-byte aligned type must be placed at addresses divisible by `n`.

```rust
use std::mem;

// Alignment examples
println!("u8:  align {}", mem::align_of::<u8>());   // 1 byte
println!("u16: align {}", mem::align_of::<u16>());  // 2 bytes
println!("u32: align {}", mem::align_of::<u32>());  // 4 bytes
println!("u64: align {}", mem::align_of::<u64>());  // 8 bytes
```

**Output:**
```
u8:  align 1
u16: align 2
u32: align 4
u64: align 8
```

### **Why Alignment Matters**

**Performance**: CPUs can access aligned data faster (single memory read)
```rust
// u32 at address 0x1000 (4-byte aligned) → 1 memory access
// u32 at address 0x1001 (misaligned)     → 2 memory accesses + shift/merge
```

**Correctness**: Some architectures (ARM, MIPS) **crash** on unaligned access
```rust
// x86 allows unaligned access (slow)
// ARM may fault (illegal instruction)
```

**Atomic operations**: Require natural alignment
```rust
use std::sync::atomic::AtomicU32;

// AtomicU32 MUST be 4-byte aligned for atomic operations to work
let atomic = AtomicU32::new(0);
```

---

## 📦 Struct Layout and Padding

### **Padding for Alignment**

Compilers insert **padding bytes** to ensure each field meets its alignment requirement:

```rust
#[repr(C)]
struct Example {
    a: u8,   // 1 byte at offset 0
    // 3 padding bytes here!
    b: u32,  // 4 bytes at offset 4 (needs 4-byte alignment)
}

println!("Size: {} bytes", mem::size_of::<Example>());  // 8 bytes (1 + 3 padding + 4)
```

**Memory layout:**
```
Offset: 0    1    2    3    4    5    6    7
Data:  [a ] [pad][pad][pad][ b  b  b  b ]
```

### **Field Reordering Impact**

Field order affects total size:

```rust
// Bad order: lots of padding
#[repr(C)]
struct Inefficient {
    a: u8,   // 1 byte + 7 padding
    b: u64,  // 8 bytes
    c: u8,   // 1 byte + 7 padding
    d: u64,  // 8 bytes
}
// Total: 32 bytes (50% padding!)

// Better order: minimal padding
#[repr(C)]
struct Efficient {
    b: u64,  // 8 bytes
    d: u64,  // 8 bytes
    a: u8,   // 1 byte
    c: u8,   // 1 byte + 6 padding
}
// Total: 24 bytes (25% padding)
```

---

## 🔧 repr Attributes

### **repr(Rust) - Default Optimization**

Rust's default layout allows the compiler to **reorder fields** for optimal size:

```rust
struct OptimizedLayout {
    tiny: bool,    // 1 byte
    normal: u32,   // 4 bytes
    small: u8,     // 1 byte
    long: u64,     // 8 bytes
    short: u16,    // 2 bytes
}

// Compiler reorders to: [long, normal, short, tiny, small] → 16 bytes
println!("Size: {}", mem::size_of::<OptimizedLayout>());  // 16 bytes
```

**Characteristics:**
- ✅ **Best performance and size** - compiler optimizes
- ✅ **Zero-cost abstraction** - no runtime overhead
- ❌ **Unstable layout** - can change between compiler versions
- ❌ **Not FFI-safe** - can't use with C code

### **repr(C) - C Compatibility**

Preserves field order and follows C's layout rules:

```rust
#[repr(C)]
struct CLayout {
    tiny: bool,    // 1 byte + 3 padding
    normal: u32,   // 4 bytes
    small: u8,     // 1 byte + 7 padding
    long: u64,     // 8 bytes
    short: u16,    // 2 bytes + 6 padding
}

println!("Size: {}", mem::size_of::<CLayout>());  // 32 bytes
```

**Memory layout:**
```
Offset:  0    1 2 3   4 5 6 7   8 9...15  16...23   24 25 26...31
Data:   [tiny][pad ]  [normal]  [sm][pad] [ long ]  [sh][  pad  ]
```

**Use Cases:**
- ✅ **FFI (Foreign Function Interface)** - calling C libraries
- ✅ **Binary protocols** - network packets, file formats
- ✅ **Memory-mapped I/O** - hardware register layouts
- ✅ **Stable layout** - guaranteed across compiler versions

**Example - C Interop:**
```rust
#[repr(C)]
struct Point {
    x: f64,
    y: f64,
}

extern "C" {
    fn process_point(p: *const Point);  // C expects this layout
}
```

### **repr(packed) - Minimal Size**

Removes **all padding** between fields:

```rust
#[repr(packed)]
#[allow(clippy::repr_packed_without_abi)]
struct Packed {
    a: u8,   // 1 byte at offset 0
    b: u32,  // 4 bytes at offset 1 (NO padding!)
}

println!("Size: {}", mem::size_of::<Packed>());  // 5 bytes
```

**Memory layout:**
```
Offset: 0    1    2    3    4
Data:  [a ] [ b  b  b  b ]
```

**⚠️ Dangers:**
- ❌ **Unaligned access** - `b` is at offset 1 (not 4-byte aligned!)
- ❌ **Performance penalty** - CPU may need multiple accesses
- ❌ **Potential crashes** - some architectures fault on misalignment
- ❌ **No references** - can't take `&` to fields (UB if misaligned)

**Modern Best Practice:**
```rust
#[repr(packed(2))]  // Pack to 2-byte boundaries (safer)
struct BetterPacked {
    a: u8,
    b: u32,
}
```

**Use Cases:**
- Network packet headers (exact byte layout required)
- Binary file formats (space-critical)
- Embedded systems (limited memory)

### **repr(transparent) - Zero-Cost Wrapper**

Guarantees wrapper type has **identical layout** to inner type:

```rust
#[repr(transparent)]
struct Meters(f64);

#[repr(transparent)]
struct Seconds(f64);

// Both are exactly 8 bytes, same alignment as f64
println!("Meters:  size {}, align {}", 
    mem::size_of::<Meters>(), 
    mem::align_of::<Meters>());   // size 8, align 8

println!("Seconds: size {}, align {}", 
    mem::size_of::<Seconds>(), 
    mem::align_of::<Seconds>());  // size 8, align 8
```

**Requirements:**
- Must have **exactly one** non-zero-sized field
- Can have any number of zero-sized fields (PhantomData, etc.)

**Use Cases:**
- ✅ **Type-safe wrappers** - prevent mixing incompatible types
- ✅ **FFI newtypes** - wrap C types with Rust safety
- ✅ **Zero-cost abstraction** - no runtime overhead

**Example - Type Safety:**
```rust
#[repr(transparent)]
struct FileDescriptor(i32);

#[repr(transparent)]
struct SocketDescriptor(i32);

fn close_file(fd: FileDescriptor) { /* ... */ }
fn close_socket(sd: SocketDescriptor) { /* ... */ }

// Compile-time safety - can't mix them up!
let file = FileDescriptor(3);
let sock = SocketDescriptor(5);

close_file(file);     // ✅ OK
// close_file(sock);  // ❌ Compile error!
```

---

## 📊 Compound Type Alignment

### **Tuples and Structs**

Alignment = **maximum** of field alignments:

```rust
// Tuple alignment is max(align(u8), align(u32)) = 4
let tuple: (u8, u32) = (1, 2);
println!("Tuple align: {}", mem::align_of_val(&tuple));  // 4

// Struct follows same rule
struct MyStruct {
    a: u8,   // align 1
    b: u32,  // align 4
}
println!("Struct align: {}", mem::align_of::<MyStruct>());  // 4
```

### **Arrays**

Arrays inherit element alignment:

```rust
println!("[u8; 4]  align: {}", mem::align_of::<[u8; 4]>());   // 1
println!("[u32; 2] align: {}", mem::align_of::<[u32; 2]>());  // 4
println!("[u64; 8] align: {}", mem::align_of::<[u64; 8]>());  // 8
```

**No padding between elements:**
```rust
let arr: [u32; 3] = [1, 2, 3];
// Layout: [1][2][3] - contiguous, no gaps
```

### **Enums**

Enums = discriminant + largest variant:

```rust
enum Message {
    Quit,                      // 0 bytes (unit variant)
    Move { x: i32, y: i32 },   // 8 bytes
    Write(String),             // 24 bytes (String = 3×usize)
}

println!("Size: {}", mem::size_of::<Message>());
// Discriminant (typically 1-8 bytes) + largest variant (24 bytes) + padding
```

---

## 🧪 Practical Examples

### **Example 1: FFI with C**

```rust
// C header: struct Point { double x; double y; };
#[repr(C)]
struct Point {
    x: f64,
    y: f64,
}

extern "C" {
    fn distance(p1: *const Point, p2: *const Point) -> f64;
}

fn main() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    
    unsafe {
        let d = distance(&p1, &p2);
        println!("Distance: {}", d);  // 5.0
    }
}
```

### **Example 2: Optimizing Struct Size**

```rust
// Before: 32 bytes (poor field order)
#[repr(C)]
struct Before {
    a: u8,   // 1 + 7 padding
    b: u64,  // 8
    c: u8,   // 1 + 7 padding
    d: u64,  // 8
}

// After: 24 bytes (optimized order)
#[repr(C)]
struct After {
    b: u64,  // 8
    d: u64,  // 8
    a: u8,   // 1
    c: u8,   // 1 + 6 padding
}

println!("Before: {} bytes", mem::size_of::<Before>());  // 32
println!("After:  {} bytes", mem::size_of::<After>());   // 24
// Saved 25% memory!
```

### **Example 3: Type-Safe Wrappers**

```rust
#[repr(transparent)]
struct Meters(f64);

#[repr(transparent)]
struct Seconds(f64);

fn speed(distance: Meters, time: Seconds) -> f64 {
    distance.0 / time.0  // m/s
}

let d = Meters(100.0);
let t = Seconds(9.58);

println!("Speed: {:.2} m/s", speed(d, t));  // 10.44 m/s
// println!("Wrong: {:.2}", speed(t, d));   // Compile error!
```

---

## 🎓 Decision Tree: Which repr?

```
What is your use case?

├─ Need C compatibility / FFI?
│  └─ Use #[repr(C)]
│
├─ Need exact byte layout (protocol/format)?
│  ├─ Space critical + can handle unaligned access?
│  │  └─ Use #[repr(packed)] or #[repr(packed(N))]
│  └─ Otherwise use #[repr(C)]
│
├─ Type-safe wrapper with same ABI as inner type?
│  └─ Use #[repr(transparent)]
│
└─ Normal Rust code (no external constraints)?
   └─ Use default (repr(Rust)) - let compiler optimize!
```

---

## ⚖️ Tradeoffs Summary

| **repr** | **Size** | **Performance** | **Stability** | **FFI-Safe** | **Use Case** |
|----------|----------|-----------------|---------------|--------------|--------------|
| **Rust** | Optimal | Best | Unstable | ❌ No | Default - pure Rust code |
| **C** | Good | Good | Stable | ✅ Yes | FFI, binary formats |
| **packed** | Minimal | Poor* | Stable | ⚠️ Careful | Space-critical protocols |
| **transparent** | Exact | Same as inner | Stable | ✅ Yes | Type-safe wrappers |

*Poor performance due to unaligned access penalties

---

## ⚠️ Common Pitfalls

### **1. Taking References to Packed Fields**

```rust
#[repr(packed)]
struct Packed {
    a: u8,
    b: u32,
}

let p = Packed { a: 1, b: 2 };
// let r = &p.b;  // ❌ UB! b is misaligned, reference is undefined behavior
let v = p.b;      // ✅ OK - copy value
```

### **2. Assuming Default Layout**

```rust
struct Unstable {
    a: u8,
    b: u32,
}

// Layout may change:
// - Compiler v1: [a][pad][b] = 8 bytes
// - Compiler v2: [b][a][pad] = 8 bytes (reordered!)
// Don't rely on field offsets without repr(C)!
```

### **3. repr(C) Performance Cost**

```rust
// May have suboptimal layout
#[repr(C)]
struct Bloated {
    tiny: bool,    // 1 + 7 padding
    big: u64,      // 8
}
// 16 bytes vs potentially 12 bytes with repr(Rust)
```

---

## 🧠 Integrator Perspective

Think of memory layout like **circuit board component placement**:

- **Alignment** = components must align to grid positions (can't place anywhere)
- **Padding** = empty space between components (wasted PCB area)
- **repr(Rust)** = let the PCB auto-router optimize placement (best efficiency)
- **repr(C)** = follow the reference design exactly (interoperability)
- **repr(packed)** = cram components as tight as possible (may cause signal issues)
- **repr(transparent)** = protective case that matches component dimensions exactly

You specify the interface contract (repr(C) for external connections) but trust the compiler (repr(Rust)) for internal optimization.

---

## 📚 Related Concepts

- **[[memory-address-analysis]]** - Low-level memory debugging and address inspection
- **[[Memory Management]]** - Stack, heap, and static memory regions
- **[[phantom-data-type-safety]]** - Zero-sized types and variance
- **[[Cache-Friendly Data Structures]]** - Cache line alignment for performance
- **[[SIMD Optimization Patterns]]** - SIMD alignment requirements

---

## 🔗 External Resources

From **Rust for Rustaceans Ch2.1**:
- Memory alignment prevents crashes and improves performance
- repr attributes control layout for FFI and optimization
- Default repr(Rust) provides best performance for pure Rust code
- Always use repr(C) for C interop, repr(transparent) for safe wrappers

From **The Rustonomicon**:
- [Data Layout](https://doc.rust-lang.org/nomicon/data-layout.html)
- [repr(Rust)](https://doc.rust-lang.org/nomicon/repr-rust.html)
- [Other reprs](https://doc.rust-lang.org/nomicon/other-reprs.html)

---

*Links: [[zettel-index]] | [[rust-concepts-MOC]] | [[memory-address-analysis]] | [[Memory Management]] | [[phantom-data-type-safety]] | [[rfr-ch02-summary]]*

*Tags: #memory-layout #alignment #repr-attributes #rust-for-rustaceans-ch2 #performance #ffi #abi #optimization*
