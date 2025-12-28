# 🔄 Clone vs Copy

**When to use Clone vs Copy - Understanding explicit vs implicit duplication**

**Tags:** #clone #copy #traits #value-semantics #ownership #rust-for-rustaceans-ch1

**Related:** [[copy-trait]], [[move-semantics]], [[ownership-fundamentals]], [[Ownership and Borrowing]], [[drop-trait]]

---

## 🎯 Core Distinction

| **Aspect** | **Copy** | **Clone** |
|------------|----------|-----------|
| **Duplication** | Implicit (automatic) | Explicit (must call `.clone()`) |
| **Performance** | Cheap (bitwise copy) | Potentially expensive (deep copy) |
| **Trait Signature** | `trait Copy: Clone` (marker) | `trait Clone` (has method) |
| **Heap Allocation** | Never | Can allocate heap memory |
| **Drop Compatibility** | Cannot implement Drop | Can implement Drop |
| **Use Case** | Simple stack-only types | Complex types with heap data |

---

## 📋 The Copy Trait

### **Characteristics**

```rust
let x = 42;
let y = x;  // Implicit copy - x still valid
println!("{} {}", x, y);  // ✅ Both accessible
```

**Copy Requirements:**
1. Type is **stack-only** (no heap allocations)
2. Type does **not implement Drop**
3. All fields also implement Copy

### **Types That Are Copy**

```rust
// Primitive types
i32, u64, f64, bool, char

// Tuples of Copy types
(i32, bool, char)

// Arrays of Copy types
[i32; 5]

// Shared references (always Copy)
&T  // for any T

// Function pointers
fn(i32) -> String
```

### **Deriving Copy**

```rust
#[derive(Debug, Clone, Copy)]  // Copy requires Clone
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 0, y: 0 };
let p2 = p1;  // Implicit copy
assert_eq!(p1.x, p2.x);  // ✅ p1 still valid
```

---

## 🛠️ The Clone Trait

### **Characteristics**

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // Explicit clone - deep copy
println!("{} {}", s1, s2);  // ✅ Both valid
```

**Clone Capabilities:**
1. **Explicit control** - must call `.clone()`
2. **Heap allocation** - can copy heap data
3. **Custom logic** - implement custom duplication
4. **Works with Drop** - compatible with destructors

### **Types That Are Clone (but not Copy)**

```rust
// Heap-allocated collections
String
Vec<T>
HashMap<K, V>
Box<T>

// Reference-counted pointers
Rc<T>
Arc<T>

// Complex types with Drop
File
Mutex<T>
```

### **Implementing Clone**

```rust
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

impl Clone for Person {
    fn clone(&self) -> Self {
        Person {
            name: self.name.clone(),  // Deep copy String
            age: self.age,            // Copy u32
        }
    }
}
```

---

## 🤔 When to Use Which?

### **Use Copy When:**

✅ Type is **simple and stack-only**
```rust
#[derive(Copy, Clone)]
struct Point2D { x: f64, y: f64 }  // Small, stack-only → Copy
```

✅ Type represents a **primitive value**
```rust
#[derive(Copy, Clone)]
struct PlayerId(u64);  // Wrapper around primitive → Copy
```

✅ **Frequent duplication** expected (no performance cost)
```rust
// Used in tight loops
let coords = [(1, 2), (3, 4), (5, 6)];
for &(x, y) in &coords {  // Implicit copy, zero overhead
    process(x, y);
}
```

### **Use Clone When:**

✅ Type owns **heap-allocated data**
```rust
#[derive(Clone)]
struct Document {
    title: String,       // Heap allocation
    content: Vec<u8>,    // Heap allocation
}
```

✅ Type implements **Drop** (cleanup required)
```rust
#[derive(Clone)]
struct FileHandle {
    path: String,
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        println!("Closing file: {}", self.path);
    }
}
```

✅ **Expensive duplication** should be explicit
```rust
let large_data = vec![0u8; 1_000_000];
let copy = large_data.clone();  // Explicit - signals cost
```

✅ **Selective cloning** (not always needed)
```rust
fn process(data: &Document) {
    // Work with reference most of the time
    analyze(&data);
    
    // Only clone when modification needed
    if needs_modification {
        let mut modified = data.clone();
        modified.title.push_str(" (edited)");
        save(modified);
    }
}
```

---

## ⚠️ Common Pitfalls

### **1. Copy + Drop Conflict**

```rust
// ❌ Cannot have both Copy and Drop
#[derive(Copy, Clone)]
struct BadExample {
    value: i32,
}

impl Drop for BadExample {  // ❌ Compile error!
    fn drop(&mut self) { /* ... */ }
}
```

**Why?** Copy creates implicit duplicates - which one should Drop?

### **2. Accidentally Making Non-Copy Types**

```rust
#[derive(Copy, Clone)]  // ❌ Won't compile
struct Container {
    data: Vec<i32>,  // Vec is not Copy!
}
```

**Fix:** Remove Copy or replace Vec with array:
```rust
#[derive(Copy, Clone)]
struct Container {
    data: [i32; 10],  // Arrays are Copy if T is Copy
}
```

### **3. Unnecessary Clones**

```rust
// ❌ Wasteful - clones on every call
fn process(data: String) {
    println!("{}", data);
}

let s = String::from("hello");
process(s.clone());  // Expensive heap copy!
process(s.clone());  // Another copy!

// ✅ Better - borrow instead
fn process(data: &str) {
    println!("{}", data);
}

let s = String::from("hello");
process(&s);  // No copy
process(&s);  // No copy
```

---

## 🧪 Complete Example: Copy vs Clone in Action

```rust
use std::rc::Rc;

// Copy type - simple stack value
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

// Clone-only type - owns heap data
#[derive(Debug, Clone)]
struct Path {
    points: Vec<Point>,
    name: String,
}

fn main() {
    // Copy: Implicit duplication
    let p1 = Point { x: 0, y: 0 };
    let p2 = p1;  // Bitwise copy (8 bytes)
    println!("p1: {:?}, p2: {:?}", p1, p2);  // Both valid
    
    // Clone: Explicit duplication
    let path1 = Path {
        points: vec![p1, p2],
        name: String::from("Route A"),
    };
    let path2 = path1.clone();  // Explicit - deep copy of Vec and String
    // let path2 = path1;  // Would MOVE, path1 invalidated
    
    println!("path1: {:?}", path1);  // Still valid
    println!("path2: {:?}", path2);  // Also valid
    
    // Rc: Clone increments reference count (cheap)
    let shared = Rc::new(Point { x: 10, y: 20 });
    let shared_ref1 = Rc::clone(&shared);  // Increment count
    let shared_ref2 = Rc::clone(&shared);  // Increment count
    println!("Rc count: {}", Rc::strong_count(&shared));  // 3
}
```

**Output:**
```
p1: Point { x: 0, y: 0 }, p2: Point { x: 0, y: 0 }
path1: Path { points: [Point { x: 0, y: 0 }, Point { x: 0, y: 0 }], name: "Route A" }
path2: Path { points: [Point { x: 0, y: 0 }, Point { x: 0, y: 0 }], name: "Route A" }
Rc count: 3
```

---

## 🎓 Key Takeaways

1. **Copy = Implicit + Cheap**: Use for small stack-only types (primitives, simple structs)
2. **Clone = Explicit + Potentially Expensive**: Use for types with heap data or Drop
3. **Copy requires Clone**: All Copy types must implement Clone (but not vice versa)
4. **No Copy + Drop**: Cannot implement both - compiler enforces safety
5. **Rc::clone() vs .clone()**: 
   - `Rc::clone(&rc)` → cheap (reference count increment)
   - `(*rc).clone()` → expensive (deep clone of inner data)

---

## 🔗 Decision Tree

```
Does your type own heap memory or implement Drop?
├─ YES → Use Clone only (explicit control)
│         Examples: String, Vec, Box, File
│
└─ NO → Can use Copy (implicit duplication)
          ├─ Is it small and frequently copied?
          │  └─ YES → Derive Copy + Clone
          │            Examples: Point, Color, PlayerId
          │
          └─ NO → Use Clone anyway (explicit intent)
                   Examples: Large arrays, complex calculations
```

---

## 📚 Related Patterns

- **[[copy-trait]]** - Deep dive into Copy trait mechanics
- **[[move-semantics]]** - Understanding when values move vs copy
- **[[ownership-fundamentals]]** - Foundation of ownership system
- **[[Ownership and Borrowing]]** - Borrowing to avoid unnecessary copies
- **[[rc-shared-ownership]]** - Rc::clone() for cheap reference counting
- **[[drop-trait]]** - Why Drop and Copy are incompatible

---

## 🔍 See Also

From **Rust for Rustaceans Ch1**:
- Copy is a **marker trait** signaling cheap duplication
- Clone requires **explicit call** to signal potentially expensive operations
- Understanding this distinction is crucial for **performance optimization**

---

*Links: [[zettel-index]] | [[rust-concepts-MOC]] | [[copy-trait]] | [[move-semantics]] | [[ownership-fundamentals]] | [[Ownership and Borrowing]] | [[rc-shared-ownership]] | [[drop-trait]]*

*Tags: #clone #copy #traits #value-semantics #ownership #rust-for-rustaceans-ch1 #performance #heap #stack*
