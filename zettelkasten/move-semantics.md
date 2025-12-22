# Move Semantics - Ownership Transfer in Rust

**Tags:** #move #ownership #heap #transfer #rust-for-rustaceans-ch1 #zero-cost #memory-safety

**Related:** [[copy-trait]], [[drop-trait]], [[ownership-fundamentals]], [[ownership]], [[Ownership and Borrowing]], [[raii-pattern]], [[Clone vs Copy]], [[Ownership Mental Model - The Library Analogy]]

---

## Core Concept

**Move semantics** is Rust's default ownership transfer mechanism for types that manage resources (heap-allocated data, file handles, etc.). When a value is moved, ownership transfers to the new location and the original location becomes **invalid**.

### **Fundamental Principle**

```rust
let s1 = String::from("hello");
let s2 = s1;  // Ownership MOVED from s1 to s2
// s1 is now invalid ❌
// s2 is the sole owner ✅
```

**Key Insight**: Move is **cheap** (just copying a few machine words on the stack) but **prevents double-free** bugs by ensuring only one owner exists.

---

## Why Moves Exist

### **Problem: Heap-Allocated Data**

```rust
// String's internal representation:
// Stack: [pointer, length, capacity]  ← 3 machine words
// Heap:  "hello world\0"               ← actual data

let s1 = String::from("hello");
```

### **Without Moves (Hypothetical)**

```rust
// If Rust allowed this (it doesn't):
let s1 = String::from("hello");
let s2 = s1;  // Both point to same heap data
// drop(s1);  // Free heap memory
// drop(s2);  // ❌ DOUBLE FREE - memory already freed!
```

### **With Moves (Actual Rust)**

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 invalidated, s2 owns heap data
// drop(s2);  // ✅ Only one owner, one free
```

---

## Move Semantics in Practice

### **Variable Assignment**

```rust
// Move types (heap-allocated)
let s1 = String::from("hello");
let s2 = s1;  // s1 moved to s2
// println!("{}", s1);  // ❌ Compile error: value used after move
println!("{}", s2);     // ✅ s2 is valid

// Move also happens with Vec, Box, HashMap, etc.
let v1 = vec![1, 2, 3];
let v2 = v1;  // v1 moved to v2
```

### **Function Calls**

```rust
fn take_ownership(s: String) {
    println!("{}", s);
}  // s dropped here

fn main() {
    let text = String::from("hello");
    take_ownership(text);  // text moved into function
    // println!("{}", text);  // ❌ Compile error: text was moved
}
```

### **Function Returns**

```rust
fn give_ownership() -> String {
    let s = String::from("hello");
    s  // Ownership transferred to caller
}

fn main() {
    let s = give_ownership();  // Receives ownership
    println!("{}", s);         // ✅ s is valid here
}
```

### **Collection Operations**

```rust
let mut vec = Vec::new();
let data = String::from("important");

vec.push(data);  // data moved into vec
// println!("{}", data);  // ❌ Compile error: data was moved

// Getting ownership back
let retrieved = vec.pop().unwrap();  // Ownership returned
println!("{}", retrieved);  // ✅ We own it again
```

---

## Move vs Copy: The Distinction

| **Aspect** | **Move Semantics** | **Copy Semantics** |
|------------|-------------------|-------------------|
| **When** | Default for heap types | Only if type implements `Copy` trait |
| **Effect** | Old location invalidated | Old location remains valid |
| **Cost** | Cheap (pointer copy) | Cheap (bit-for-bit copy) |
| **Types** | `String`, `Vec`, `Box`, custom structs | `i32`, `bool`, `char`, `f64`, `(i32, i32)` |
| **Safety** | Prevents double-free | Safe because no resources owned |

```rust
// Move example
let s1 = String::from("hello");
let s2 = s1;  // s1 invalid ❌

// Copy example  
let x = 42;
let y = x;  // x still valid ✅
```

---

## Advanced Move Patterns

### **Partial Moves**

```rust
struct Point {
    x: String,
    y: String,
}

let p = Point {
    x: String::from("10"),
    y: String::from("20"),
};

let x_val = p.x;  // Moves p.x out
// let point = p;  // ❌ Can't use p: partial move occurred
let y_val = p.y;  // ✅ Can still move p.y
```

### **Move Closures**

```rust
let data = vec![1, 2, 3];

let closure = move || {
    println!("{:?}", data);  // data moved into closure
};

closure();
// println!("{:?}", data);  // ❌ data was moved
```

### **Mem Utilities for Moves**

```rust
use std::mem;

// mem::take - Move out of &mut, leave default value
let mut s = Some(String::from("hello"));
let taken = mem::take(&mut s);  // s now None
assert_eq!(s, None);
assert_eq!(taken, Some(String::from("hello")));

// mem::replace - Swap values atomically
let mut s = String::from("old");
let old = mem::replace(&mut s, String::from("new"));
assert_eq!(old, "old");
assert_eq!(s, "new");

// mem::swap - Exchange two values
let mut x = Box::new(5);
let mut y = Box::new(10);
mem::swap(&mut x, &mut y);
assert_eq!(*x, 10);
assert_eq!(*y, 5);
```

---

## Mental Model

**The Single Key Analogy**: Think of ownership like having the **only key** to a house:

- **Move**: You hand your friend the key → They own the house now, you don't
- **Borrow**: You let them use your key temporarily → You still own the house
- **Copy**: You make a duplicate house (expensive) → Both have houses (but Rust does this only for cheap types)

**Stack vs Heap**:
```
Stack (cheap to copy):         Heap (expensive to copy):
┌─────────┐                    ┌─────────┐      ┌─────────────┐
│ x: 42   │ ← Copy            │ ptr ───┼──→   │ "hello"     │
├─────────┤                    ├─────────┤      └─────────────┘
│ y: 42   │ ← Duplicate        │ len: 5  │      ↑ Only pointer moved
└─────────┘                    │ cap: 5  │      
                               └─────────┘
```

---

## Common Patterns

### **Move Then Use Pattern**

```rust
// ❌ Wrong: Try to use after move
let s = String::from("hello");
process(s);
println!("{}", s);  // Error!

// ✅ Correct: Clone if need to keep original
let s = String::from("hello");
process(s.clone());
println!("{}", s);  // Works!

// ✅ Better: Return ownership if possible
fn process_and_return(s: String) -> String {
    // ... do work ...
    s  // Return ownership
}
```

### **Temporary Ownership**

```rust
// Take ownership, do work, return it
fn process(mut s: String) -> String {
    s.push_str(" processed");
    s  // Return ownership
}

let text = String::from("data");
let result = process(text);  // Ownership moved and returned
```

### **Builder Pattern**

```rust
struct Config {
    name: String,
    value: i32,
}

impl Config {
    fn new(name: String) -> Self {
        Config { name, value: 0 }
    }
    
    fn set_value(mut self, value: i32) -> Self {
        self.value = value;
        self  // Return ownership for chaining
    }
}

// Usage
let config = Config::new(String::from("app"))
    .set_value(42);  // Each method consumes and returns self
```

---

## Performance Implications

### **Zero-Cost Abstraction**

```rust
// These are identical in performance:
let s1 = String::from("hello");
let s2 = s1;  // Just copies 3 words (ptr, len, cap)

// vs in C++:
// std::string s1 = "hello";
// std::string s2 = std::move(s1);  // Also just pointer move
```

**Move is cheap**: Only stack data is copied (pointers, metadata). Heap data stays put.

### **Preventing Expensive Copies**

```rust
// Without moves, this would be expensive:
fn process_large_data(data: Vec<u8>) {  // Takes ownership
    // ... process megabytes of data ...
}  // data dropped here

let big_vec = vec![0u8; 1_000_000];
process_large_data(big_vec);  // ✅ Move is cheap (just pointer)

// vs if it copied:
// process_large_data(big_vec);  // ❌ Would copy 1MB!
```

---

## Compile-Time Guarantees

Move semantics enable **zero-cost memory safety**:

1. **No double-free**: Only one owner, so only one `drop` call
2. **No use-after-free**: Moved values are compile-time invalid
3. **No dangling pointers**: Can't reference moved data
4. **No data races**: Can't share mutable ownership

```rust
let s1 = String::from("hello");
let s2 = s1;
// All these are compile errors:
// println!("{}", s1);       // ❌ use after move
// let s3 = s1;              // ❌ move of moved value
// modify(&mut s1);          // ❌ borrow of moved value
```

---

## When NOT to Move

### **Use Borrowing Instead**

```rust
// ❌ Inefficient: move and return
fn get_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)  // Return ownership back
}

// ✅ Better: borrow
fn get_length(s: &String) -> usize {
    s.len()  // Just borrow, no ownership transfer
}
```

### **Use Copy Types**

```rust
// For small, stack-only data, derive Copy
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 0, y: 0 };
let p2 = p1;  // Copied, not moved
println!("{:?} {:?}", p1, p2);  // Both valid
```

---

## Real-World Examples

### **Mission 1: Stack Implementation**

```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn push(&mut self, item: T) {
        self.items.push(item);  // item moved into Vec
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()  // Ownership returned to caller
    }
}

// Usage
let mut stack = Stack::new();
let s = String::from("hello");
stack.push(s);  // s moved into stack
// Can't use s here anymore
```

### **Mission 10: REST API State Management**

```rust
use std::collections::HashMap;
use uuid::Uuid;

fn create_instance(state: &mut HashMap<Uuid, UnionFind>, size: usize) -> Uuid {
    let id = Uuid::new_v4();
    let uf = UnionFind::new(size);
    state.insert(id, uf);  // uf moved into HashMap
    id  // Return ID (Copy type, so no move)
}

fn delete_instance(state: &mut HashMap<Uuid, UnionFind>, id: Uuid) -> bool {
    state.remove(&id).is_some()  // UnionFind moved out and dropped
}
```

### **AoC: Building Complex State**

```rust
// Day 21: Move graph state into solver
fn solve(mut grid: Grid, start: Pos) -> usize {
    // grid owned by function, modified in place
    bfs(&mut grid, start)  // Borrow, don't move
}  // grid dropped here

let grid = parse_input(input);  // Owned grid
let result = solve(grid);  // Move grid into solver
// Can't use grid after this point
```

---

## Common Mistakes

### **1. Using Moved Value**

```rust
let s = String::from("hello");
let t = s;
println!("{}", s);  // ❌ ERROR: value used after move
```

**Fix**: Clone before move or borrow instead
```rust
let s = String::from("hello");
let t = s.clone();  // Clone instead of move
println!("{} {}", s, t);  // ✅ Both valid
```

### **2. Moving in Loop**

```rust
let data = vec![1, 2, 3];
for _ in 0..3 {
    process(data);  // ❌ ERROR: move on first iteration
}
```

**Fix**: Borrow or clone in loop
```rust
for _ in 0..3 {
    process(&data);  // ✅ Borrow
    // or process(data.clone());  if mutation needed
}
```

### **3. Partial Move Confusion**

```rust
let tuple = (String::from("a"), String::from("b"));
let first = tuple.0;  // Moves tuple.0
println!("{:?}", tuple);  // ❌ ERROR: partial move
```

**Fix**: Destructure entirely
```rust
let (first, second) = tuple;  // ✅ Move both
```

---

## Key Takeaways

1. ✅ **Move transfers ownership** - old location becomes invalid
2. ✅ **Cheap operation** - only stack data copied (pointers, metadata)
3. ✅ **Prevents double-free** - only one owner can drop heap data
4. ✅ **Default for heap types** - `String`, `Vec`, `Box`, custom structs
5. ✅ **Compile-time safety** - use-after-move is a compile error
6. ✅ **Use borrowing** when you don't need ownership transfer
7. ✅ **Use `mem::` utilities** for advanced move patterns
8. ✅ **Builder pattern** leverages moves for ergonomic APIs

---

## Related Concepts

- [[copy-trait]] - Types that duplicate instead of moving
- [[drop-trait]] - Automatic cleanup when owner goes out of scope
- [[ownership-fundamentals]] - Core ownership rules and mental models
- [[Ownership and Borrowing]] - Full ownership and borrowing system
- [[Clone vs Copy]] - Explicit vs implicit duplication
- [[raii-pattern]] - Resource management through ownership

---

*Created*: 2025-12-22 (Rust for Rustaceans Ch1.2 - ownership_semantics)  
*Source*: [[rust-for-rustaceans/Ch01]], [[rust-book-ch1-4-review]]  
*Examples*: [[missions/Mission1]], [[missions/Mission10]], [[advent_of_code]]
