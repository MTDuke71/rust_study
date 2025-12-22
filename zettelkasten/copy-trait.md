# Copy Trait - Bitwise Duplication in Rust

**Tags:** #copy #stack #trait #bitwise #rust-for-rustaceans-ch1 #zero-cost #simple-types

**Related:** [[move-semantics]], [[drop-trait]], [[Clone vs Copy]], [[ownership-fundamentals]], [[ownership]], [[Ownership and Borrowing]]

---

## Core Concept

The **Copy trait** marks types that can be **duplicated by bitwise copy** rather than moved. Copy types are **simple, stack-only** values with no ownership of resources like heap memory, file handles, or locks.

```rust
pub trait Copy: Clone { }
```

**Key Distinction**: Copy is a **marker trait** with no methods. It signals to the compiler: "This type is safe to duplicate by copying bits."

---

## Copy vs Move: The Fundamental Difference

### **Copy Types (Stack-Only)**

```rust
let x = 42;
let y = x;  // x COPIED to y
println!("x: {}, y: {}", x, y);  // ✅ Both valid!
```

### **Move Types (Heap-Allocated)**

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 MOVED to s2
// println!("{}", s1);  // ❌ Compile error: s1 invalidated
```

---

## Which Types Implement Copy?

### **Built-in Copy Types**

| **Category** | **Types** | **Why Copy?** |
|--------------|-----------|---------------|
| **Integers** | `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | Fixed size, stack-only |
| **Floats** | `f32`, `f64` | Fixed size, stack-only |
| **Boolean** | `bool` | 1 byte, trivial to copy |
| **Character** | `char` | 4 bytes (Unicode scalar), stack-only |
| **Raw Pointers** | `*const T`, `*mut T` | Just addresses (word-sized) |
| **Function Pointers** | `fn(i32) -> i32` | Address of code, stack-only |
| **Shared References** | `&T`, `&mut T` | Just pointers, no ownership |

### **Tuple/Array Copy Rules**

```rust
// Tuple is Copy if ALL elements are Copy
let t1 = (42, true, 'x');  // (i32, bool, char) - all Copy
let t2 = t1;  // ✅ Copied
println!("{:?} {:?}", t1, t2);  // Both valid

// Array is Copy if element type is Copy
let arr1 = [1, 2, 3];  // [i32; 3] - i32 is Copy
let arr2 = arr1;  // ✅ Copied
println!("{:?} {:?}", arr1, arr2);  // Both valid

// But this doesn't work:
let strs = [String::from("a"), String::from("b")];
let strs2 = strs;  // ❌ String is not Copy, so array isn't either
```

---

## Types That CANNOT Be Copy

### **Heap-Allocated Types**

```rust
// These manage heap memory - moving is safer than copying
String       // Owns heap buffer
Vec<T>       // Owns heap allocation
Box<T>       // Owns heap allocation
HashMap<K,V> // Owns heap storage
Rc<T>        // Reference counted pointer
Arc<T>       // Atomic reference counted pointer
```

**Why not Copy?**: Copying these would mean **two owners** of the same heap memory → **double-free** bug!

### **Types with Drop Implementations**

```rust
// Copy and Drop are mutually exclusive
struct FileHandle {
    fd: i32,
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        println!("Closing file descriptor {}", self.fd);
    }
}

// ❌ Can't derive Copy - has custom Drop
// #[derive(Copy, Clone)]
// struct FileHandle { ... }
```

**Rule**: If a type implements `Drop`, it **cannot** implement `Copy`. This prevents duplicate cleanup (double-close, double-free, etc.).

---

## Implementing Copy

### **Derive Copy (Automatic)**

```rust
// Simple struct with all Copy fields
#[derive(Copy, Clone)]  // Must derive Clone too!
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 10, y: 20 };
let p2 = p1;  // Copied
println!("{:?} {:?}", p1, p2);  // Both valid
```

### **Requirements for Deriving Copy**

1. **All fields must be Copy**: Can't have `String`, `Vec`, etc.
2. **Must also derive Clone**: `Copy` is a subtrait of `Clone`
3. **No custom Drop**: Can't implement `Drop` trait

```rust
// ✅ Valid - all fields are Copy
#[derive(Copy, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// ❌ Invalid - String is not Copy
#[derive(Copy, Clone)]
struct Person {
    name: String,  // ❌ Compile error!
    age: i32,
}

// ✅ Valid alternative - use references (but lifetime complexity)
#[derive(Copy, Clone)]
struct PersonRef<'a> {
    name: &'a str,  // Reference is Copy
    age: i32,
}
```

---

## Copy Semantics in Action

### **Variable Assignment**

```rust
let x = 42;
let y = x;  // Bitwise copy
assert_eq!(x, 42);  // x still valid
assert_eq!(y, 42);  // y is independent copy
```

### **Function Calls**

```rust
fn process(n: i32) {  // n is a copy
    println!("{}", n);
}

let value = 100;
process(value);  // value copied into function
println!("{}", value);  // ✅ value still valid
```

### **Method Calls**

```rust
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }

impl Point {
    fn distance_from_origin(self) -> f64 {  // self is copied
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

let p = Point { x: 3, y: 4 };
let dist = p.distance_from_origin();  // p copied into method
println!("{:?}", p);  // ✅ p still valid
```

---

## Copy vs Clone

| **Aspect** | **Copy** | **Clone** |
|------------|----------|-----------|
| **Invocation** | Implicit (automatic) | Explicit (`.clone()`) |
| **Cost** | Always cheap (bitwise) | Can be expensive (deep copy) |
| **Safety** | Compiler-enforced simple types | Programmer's responsibility |
| **Trait** | Marker trait (no methods) | Has `clone()` method |
| **Types** | Stack-only types | Any type can implement |

```rust
// Copy - implicit
let x = 42;
let y = x;  // Automatic bitwise copy

// Clone - explicit
let s1 = String::from("hello");
let s2 = s1.clone();  // Explicit deep copy
println!("{} {}", s1, s2);  // Both valid
```

### **Copy Implies Clone**

```rust
// Copy is a subtrait of Clone
pub trait Copy: Clone { }

// So all Copy types can also be cloned:
let x = 42;
let y = x.clone();  // Works, but unnecessary (implicit copy already happened)
let z = x;          // Same as above, but more idiomatic
```

---

## Mental Model

### **Stack-Only = Copy-Safe**

```
Stack (Copy):                    Heap (Move):
┌─────────────┐                 ┌───────────┐      ┌──────────┐
│ x: 42       │                 │ ptr ─────┼──→   │ "hello"  │
├─────────────┤                 ├───────────┤      └──────────┘
│ y: 42       │ ← Duplicate     │ len: 5    │      ↑ Shared!
└─────────────┘                 │ cap: 5    │      (Dangerous!)
                                └───────────┘
```

**Copy is safe** because there's **no shared ownership**. Each copy is **independent**.

**Move is necessary** for heap types to prevent **two owners** from freeing the same memory.

---

## Common Patterns

### **Numeric Computations**

```rust
// Copy types make math ergonomic
fn add(a: i32, b: i32) -> i32 {
    a + b  // a and b copied in, both still valid after
}

let x = 10;
let y = 20;
let sum = add(x, y);
println!("{} + {} = {}", x, y, sum);  // All valid
```

### **Option<Copy Type>**

```rust
// Option<i32> is Copy because i32 is Copy
let x = Some(42);
let y = x;  // Copied
assert_eq!(x, Some(42));  // ✅ x still valid

// But Option<String> is NOT Copy
let s = Some(String::from("hello"));
let t = s;  // Moved
// println!("{:?}", s);  // ❌ s invalidated
```

### **Small Value Types**

```rust
// Custom small types benefit from Copy
#[derive(Copy, Clone, Debug, PartialEq)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

let red = RGB { r: 255, g: 0, b: 0 };
let colors = vec![red, red, red];  // red copied 3 times
assert_eq!(red, RGB { r: 255, g: 0, b: 0 });  // Still valid
```

---

## When to Use Copy

### **✅ Use Copy For:**

1. **Small, stack-only data** - IDs, coordinates, colors, flags
2. **Primitive-like types** - Wrappers around integers/floats
3. **Frequently passed types** - Reduces borrow complexity
4. **No resource ownership** - No heap, files, network, etc.

```rust
#[derive(Copy, Clone)]
struct Id(u64);  // ✅ Just a number wrapper

#[derive(Copy, Clone)]
struct Point2D { x: f64, y: f64 }  // ✅ Two floats

#[derive(Copy, Clone)]
enum Direction { North, South, East, West }  // ✅ Simple enum
```

### **❌ Don't Use Copy For:**

1. **Heap-allocated types** - Would cause double-free
2. **Large structs** - Copying is expensive
3. **Types with Drop** - Cleanup would run twice
4. **Resource handles** - Files, sockets, locks

```rust
struct BigData {
    buffer: [u8; 1_000_000],  // ❌ 1MB - too large for Copy!
}

struct Database {
    connection: Connection,  // ❌ Owns network resource
}
```

---

## Copy in Generic Code

### **Constraining with Copy**

```rust
// Only accept Copy types
fn duplicate<T: Copy>(value: T) -> (T, T) {
    (value, value)  // Can use value twice!
}

let x = 42;
let (a, b) = duplicate(x);
assert_eq!(a, b);

// Won't compile with non-Copy types:
// let s = String::from("hello");
// duplicate(s);  // ❌ String doesn't implement Copy
```

### **Copy + Clone Constraints**

```rust
// Sometimes need explicit clone for clarity
fn process<T: Clone>(value: T) -> T {
    value.clone()  // Explicit clone
}

// Works with both Copy and non-Copy types
let x = 42;
let y = process(x);  // Works (copies via Clone)

let s = String::from("hello");
let t = process(s);  // Works (clones heap data)
```

---

## Performance Implications

### **Zero-Cost for Small Types**

```rust
// Copying is essentially free for small types
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }  // 8 bytes

let p1 = Point { x: 1, y: 2 };
let p2 = p1;  // Just copies 8 bytes - trivial!
```

### **Cost of Non-Copy**

```rust
// Without Copy, need explicit cloning
struct NonCopyPoint {
    x: i32,
    y: i32,
    // Maybe has some complex field...
}

let p1 = NonCopyPoint { x: 1, y: 2 };
// let p2 = p1;  // Would move p1
let p2 = p1.clone();  // Explicit clone required
// Both valid, but more verbose
```

---

## Real-World Examples

### **Mission 10: UUID Identifiers**

```rust
use uuid::Uuid;

// Uuid implements Copy - cheap to pass around
fn create_instance(id: Uuid, size: usize) {
    println!("Creating instance {}", id);
}

let id = Uuid::new_v4();
create_instance(id, 10);  // id copied
println!("Created {}", id);  // ✅ id still valid
```

### **AoC: Grid Coordinates**

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Pos {
    row: i32,
    col: i32,
}

let start = Pos { row: 0, col: 0 };
let mut visited = HashSet::new();
visited.insert(start);  // start copied into set
visited.insert(start);  // Can reuse start - it's Copy!
assert_eq!(visited.len(), 1);  // start deduplicated
```

### **Rust Book: Simple Wrappers**

```rust
// NewType pattern with Copy
#[derive(Copy, Clone, Debug, PartialEq)]
struct Meters(f64);

#[derive(Copy, Clone, Debug, PartialEq)]
struct Seconds(f64);

fn speed(distance: Meters, time: Seconds) -> f64 {
    distance.0 / time.0
}

let d = Meters(100.0);
let t = Seconds(10.0);
let v = speed(d, t);  // d and t copied
println!("Distance: {:?}, Time: {:?}, Speed: {}", d, t, v);  // All valid
```

---

## Common Mistakes

### **1. Forgetting Clone on Copy Types**

```rust
// Copy requires Clone
#[derive(Copy)]  // ❌ ERROR: Copy requires Clone
struct Point { x: i32, y: i32 }

// Fix:
#[derive(Copy, Clone)]  // ✅ Correct
struct Point { x: i32, y: i32 }
```

### **2. Copy on Heap-Allocated Types**

```rust
#[derive(Copy, Clone)]  // ❌ ERROR: String is not Copy
struct Person {
    name: String,
    age: i32,
}

// Fix: Remove Copy, or use &str with lifetimes
struct Person {
    name: String,  // Just use move semantics
    age: i32,
}
```

### **3. Copy with Drop**

```rust
struct Resource {
    id: i32,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Dropping {}", self.id);
    }
}

// ❌ Can't derive Copy - has Drop
// #[derive(Copy, Clone)]

// Fix: Choose one or the other
// Either: Remove Drop (if no cleanup needed)
// Or: Remove Copy (if cleanup needed)
```

---

## Key Takeaways

1. ✅ **Copy = implicit duplication** - no `.clone()` needed
2. ✅ **Stack-only types** - no heap, files, or other resources
3. ✅ **Bitwise copy** - cheap and always safe
4. ✅ **Mutually exclusive with Drop** - can't have both
5. ✅ **Requires Clone** - Copy is subtrait of Clone
6. ✅ **All fields must be Copy** - to derive Copy
7. ✅ **Use for small, simple types** - IDs, coordinates, primitives
8. ✅ **Zero runtime cost** - purely compile-time distinction

---

## Related Concepts

- [[move-semantics]] - Default ownership transfer for non-Copy types
- [[drop-trait]] - Automatic cleanup (mutually exclusive with Copy)
- [[Clone vs Copy]] - Explicit vs implicit duplication
- [[ownership-fundamentals]] - Core ownership rules
- [[Ownership and Borrowing]] - Complete ownership system

---

*Created*: 2025-12-22 (Rust for Rustaceans Ch1.2 - ownership_semantics)  
*Source*: [[rust-for-rustaceans/Ch01]], [[rust-book-ch1-4-review]]  
*Examples*: [[missions/Mission1]], [[missions/Mission10]], [[advent_of_code]]
