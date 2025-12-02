# Rust Book Chapters 13-15: Functional Features & Smart Pointers

> **Knowledge Integration**: Mastering functional programming patterns, professional Cargo workflows, and advanced memory management with smart pointers

---

## 📚 Overview

This review covers three chapters that elevate Rust development from intermediate to advanced: functional programming with closures and iterators, professional project management with Cargo, and sophisticated memory management through smart pointers. Together, these concepts enable writing expressive, performant, and maintainable Rust code that leverages zero-cost abstractions while maintaining memory safety.

**Chapter Coverage:**
- **Chapter 13**: Functional Language Features - Closures and Iterators
- **Chapter 14**: More about Cargo and Crates.io - Professional project workflows
- **Chapter 15**: Smart Pointers - Advanced memory management patterns

**Cross-References:**
- [[rust-book-ch9-12-review]] - Foundation: Error handling, generics, testing, CLI projects
- [[zettelkasten/rust_book/rust-book-ch13]] - Closures and iterators deep dive
- [[zettelkasten/rust_book/rust-book-ch14]] - Cargo and crates.io workflows
- [[zettelkasten/rust_book/rust-book-ch15]] - Smart pointers comprehensive guide

---

## 🎯 Chapter 13: Functional Language Features - Iterators and Closures

### **Core Philosophy**

Rust borrows heavily from functional programming languages while maintaining its performance-first philosophy. Closures and iterators enable expressive, declarative code that compiles to the same efficient machine code as imperative loops - true zero-cost abstractions. This chapter demonstrates how Rust achieves the seemingly impossible: the ergonomics of high-level functional code with the performance of low-level systems programming.

---

### **13.1 - Closures: Anonymous Functions that Capture Their Environment**

#### **Closure Fundamentals**

Closures are anonymous functions that can capture values from their enclosing scope. Unlike regular functions, closures automatically infer their parameter and return types from usage context, reducing boilerplate while maintaining type safety.

**Basic Syntax Evolution**:
```rust
// Full annotation (rarely needed)
let add_one = |x: i32| -> i32 { x + 1 };

// Type inference (typical)
let add_one = |x| x + 1;

// Multiple parameters
let multiply = |x, y| x * y;

// Multi-statement body
let complex = |x| {
    let temp = x * 2;
    temp + 1
};
```

#### **Environment Capture Modes**

Closures capture variables from their environment in three ways, determined automatically by the compiler based on usage:

**1. Immutable Borrow** (`&T`):
```rust
let list = vec![1, 2, 3];
let only_borrows = || println!("list: {:?}", list);

only_borrows();
println!("Can still use list: {:?}", list); // ✅ list still valid
```

**2. Mutable Borrow** (`&mut T`):
```rust
let mut list = vec![1, 2, 3];
let mut borrows_mutably = || list.push(4);

borrows_mutably();
// println!("{:?}", list); // ❌ Can't borrow while closure exists
drop(borrows_mutably);
println!("{:?}", list); // ✅ OK after closure dropped
```

**3. Taking Ownership** (move):
```rust
let list = vec![1, 2, 3];
let takes_ownership = move || println!("list: {:?}", list);

takes_ownership();
// println!("{:?}", list); // ❌ list was moved into closure
```

#### **The Fn Trait Family**

Every closure implements one or more of three traits that determine how it can be called:

**`FnOnce`** - Consumes captured variables (can only be called once):
```rust
let consume = move || {
    drop(list); // Takes ownership and consumes
};
consume();
// consume(); // ❌ Can't call again - closure consumed captured values
```

**`FnMut`** - Mutably borrows captured variables:
```rust
let mut count = 0;
let mut increment = || count += 1;

increment(); // count is now 1
increment(); // count is now 2
```

**`Fn`** - Immutably borrows captured variables:
```rust
let text = String::from("hello");
let printer = || println!("{}", text);

printer(); // Can call many times
printer();
```

**Trait Hierarchy**: `Fn` ⊂ `FnMut` ⊂ `FnOnce`
- Every `Fn` implements `FnMut` and `FnOnce`
- Every `FnMut` implements `FnOnce`
- Functions requiring `FnOnce` accept any closure
- Functions requiring `Fn` accept only non-mutating closures

#### **Practical Applications**

**Caching/Memoization**:
```rust
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

**Thread Spawning**:
```rust
use std::thread;

let data = vec![1, 2, 3];
let handle = thread::spawn(move || {
    // Must use `move` to transfer ownership to new thread
    println!("data: {:?}", data);
});

handle.join().unwrap();
```

---

### **13.2 - Processing a Series of Items with Iterators**

#### **The Iterator Trait**

The `Iterator` trait is the foundation of Rust's iterator ecosystem:

```rust
pub trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
    
    // Hundreds of provided methods build on next()
}
```

**Key Characteristics**:
- Iterators are **lazy** - they do nothing until consumed
- The `next()` method advances the iterator and returns `Option<Item>`
- `None` signals iteration completion
- All other iterator methods are built on `next()`

#### **Iterator Categories**

**Consuming Adaptors** - Use up the iterator:
```rust
let sum: i32 = vec![1, 2, 3].iter().sum(); // sum = 6

let collected: Vec<_> = vec![1, 2, 3].iter().collect();

// for_each consumes iterator
vec![1, 2, 3].iter().for_each(|x| println!("{}", x));
```

**Iterator Adaptors** - Transform iterators into new iterators:
```rust
let v = vec![1, 2, 3];

// map - transform each element (lazy!)
let mapped = v.iter().map(|x| x + 1);

// filter - select elements (lazy!)
let filtered = v.iter().filter(|x| *x > 1);

// Chain adaptors together (still lazy!)
let result: Vec<_> = v.iter()
    .map(|x| x * 2)
    .filter(|x| *x > 2)
    .collect(); // collect() consumes and executes the chain
```

#### **Common Iterator Methods**

**Transformation**:
```rust
// map - transform elements
let doubled: Vec<_> = vec![1, 2, 3].iter().map(|x| x * 2).collect();

// filter - select elements matching predicate
let evens: Vec<_> = vec![1, 2, 3, 4].iter().filter(|x| *x % 2 == 0).collect();

// filter_map - map and filter in one step
let parsed: Vec<i32> = vec!["1", "two", "3"]
    .iter()
    .filter_map(|s| s.parse().ok())
    .collect();

// flat_map - map then flatten
let nested = vec![vec![1, 2], vec![3, 4]];
let flattened: Vec<_> = nested.iter().flat_map(|v| v.iter()).collect();
```

**Reduction**:
```rust
// fold - reduce to single value
let sum = vec![1, 2, 3].iter().fold(0, |acc, x| acc + x);

// reduce - like fold but returns Option
let product = vec![1, 2, 3].iter().reduce(|acc, x| acc * x);

// all/any - test predicates
let all_positive = vec![1, 2, 3].iter().all(|x| *x > 0); // true
let has_even = vec![1, 2, 3].iter().any(|x| *x % 2 == 0); // true
```

**Searching**:
```rust
// find - first matching element
let first_even = vec![1, 2, 3, 4].iter().find(|x| *x % 2 == 0); // Some(&2)

// position - index of first match
let pos = vec![1, 2, 3].iter().position(|x| *x == 2); // Some(1)

// min/max
let min = vec![3, 1, 2].iter().min(); // Some(&1)
```

#### **Creating Custom Iterators**

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Usage
let sum: u32 = Counter::new().sum(); // 1 + 2 + 3 + 4 + 5 = 15
```

---

### **13.3 & 13.4 - Zero-Cost Abstractions & Performance**

#### **The Zero-Cost Promise**

Rust's iterators demonstrate "zero-cost abstraction": high-level functional code compiles to the same machine code as hand-written loops. The compiler:
1. **Inlines** all iterator methods
2. **Eliminates** closure overhead
3. **Optimizes** chains into tight loops
4. **Vectorizes** operations when possible

#### **Performance Comparison**

**Loop Version**:
```rust
let mut sum = 0;
for i in 0..1000 {
    sum += i;
}
```

**Iterator Version**:
```rust
let sum: i32 = (0..1000).sum();
```

**Both compile to identical assembly** - the iterator version is NOT slower despite being more abstract!

#### **When Iterators Outperform Loops**

**Automatic Vectorization**:
```rust
// Iterator - compiler can auto-vectorize
let sum: i32 = data.iter().map(|x| x * 2).sum();

// Loop - harder for compiler to vectorize
let mut sum = 0;
for x in &data {
    sum += x * 2;
}
```

**Chaining Eliminates Intermediate Allocations**:
```rust
// Efficient - single pass, no intermediate allocations
let result: Vec<_> = data.iter()
    .filter(|x| *x > 0)
    .map(|x| x * 2)
    .collect();

// Less efficient - multiple allocations
let filtered: Vec<_> = data.iter().filter(|x| *x > 0).collect();
let result: Vec<_> = filtered.iter().map(|x| x * 2).collect();
```

#### **Guidelines for Choosing Iterators vs Loops**

**Prefer Iterators When**:
- Transforming collections
- Chaining operations
- Leveraging existing iterator methods
- Writing functional-style code
- Performance is critical (compiler optimizes better)

**Prefer Loops When**:
- Early exit with complex conditions
- Maintaining external mutable state
- Control flow is too complex for iterators
- Clarity suffers from iterator chains

---

### **Mission Integration**

**Mission 5 - HashMap Iterator Implementation**:
```rust
// Custom iterator for HashMap entries
impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);
    
    fn next(&mut self) -> Option<Self::Item> {
        // Iterator over buckets, handling collisions
    }
}
```

**Mission 7 - Graph Traversal with Iterators**:
```rust
// BFS using iterators instead of explicit queue
fn bfs_iter(&self, start: usize) -> impl Iterator<Item = usize> + '_ {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([start]);
    
    std::iter::from_fn(move || {
        queue.pop_front().map(|node| {
            if visited.insert(node) {
                queue.extend(self.neighbors(node));
                Some(node)
            } else {
                None
            }
        }).flatten()
    })
}
```

---

## 🚀 Chapter 14: More about Cargo and Crates.io

### **Core Philosophy**

Cargo is more than a build tool - it's an ecosystem enabler that makes Rust development productive and collaborative. Understanding Cargo's advanced features transforms individual programming into participation in the global Rust community.

---

### **14.1 - Customizing Builds with Release Profiles**

#### **Profile Fundamentals**

Cargo provides two default profiles that optimize for different scenarios:
- **`dev`** - Fast compilation, debug information, no optimization
- **`release`** - Slow compilation, optimized code, minimal debug info

**Default Settings**:
```toml
[profile.dev]
opt-level = 0          # No optimization - fast compile
debug = true           # Full debug symbols
overflow-checks = true # Panic on integer overflow

[profile.release]
opt-level = 3          # Maximum optimization
debug = false          # No debug symbols
overflow-checks = false # No overflow checks
```

#### **Customization Options**

**Optimization Levels** (`opt-level`):
- `0` - No optimization (dev default)
- `1` - Basic optimization, faster than 0
- `2` - Good optimization, smaller than 3
- `3` - Maximum optimization (release default)
- `"s"` - Optimize for size
- `"z"` - Aggressively optimize for size

**Debug Information** (`debug`):
- `true` - Full debug symbols
- `false` - No debug symbols
- `1` - Line tables only
- `2` - Full debug info (equivalent to `true`)

**Practical Customizations**:
```toml
# Custom profile for profiling with optimizations
[profile.profiling]
inherits = "release"
debug = true # Keep symbols for profiler

# Faster debug builds
[profile.dev]
opt-level = 1 # Some optimization
debug = 1     # Reduced debug info

# Smaller release builds
[profile.release]
opt-level = "z"  # Size optimization
lto = true       # Link-time optimization
codegen-units = 1 # Better optimization, slower compile
strip = true     # Remove symbols
```

---

### **14.2 - Publishing a Crate to Crates.io**

#### **Documentation Comments**

Documentation is first-class in Rust. Use `///` for item docs and `//!` for module/crate docs:

```rust
//! # My Awesome Crate
//!
//! This crate provides utilities for awesome things.
//!
//! ## Examples
//!
//! ```
//! use my_crate::do_awesome_thing;
//! assert_eq!(do_awesome_thing(), 42);
//! ```

/// Performs an awesome operation.
///
/// # Examples
///
/// ```
/// use my_crate::do_awesome_thing;
/// let result = do_awesome_thing();
/// assert_eq!(result, 42);
/// ```
///
/// # Panics
///
/// Panics if the universe isn't ready.
///
/// # Errors
///
/// Returns `Err` if awesome resources are unavailable.
pub fn do_awesome_thing() -> Result<i32, Error> {
    Ok(42)
}
```

#### **Publishing Workflow**

**1. Prepare `Cargo.toml`**:
```toml
[package]
name = "my_awesome_crate"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
license = "MIT OR Apache-2.0"
description = "A short description"
repository = "https://github.com/yourusername/my_awesome_crate"
keywords = ["awesome", "utility"]
categories = ["command-line-utilities"]
```

**2. Semantic Versioning**:
- `0.1.0` - Initial development (breaking changes OK)
- `1.0.0` - First stable release
- `1.1.0` - Backward-compatible features
- `1.1.1` - Backward-compatible bug fixes
- `2.0.0` - Breaking changes

**3. Publishing Commands**:
```bash
# Login to crates.io (one-time)
cargo login <api-token>

# Verify package is ready
cargo publish --dry-run

# Publish!
cargo publish

# Yank a broken version (doesn't delete)
cargo yank --vers 1.0.1

# Un-yank
cargo yank --vers 1.0.1 --undo
```

---

### **14.3 - Cargo Workspaces**

#### **Workspace Structure**

Workspaces organize related crates with shared dependencies:

```
workspace_root/
├── Cargo.toml          # Workspace root
├── Cargo.lock          # Shared lock file
├── crate_a/
│   ├── Cargo.toml
│   └── src/
└── crate_b/
    ├── Cargo.toml
    └── src/
```

**Root `Cargo.toml`**:
```toml
[workspace]
members = [
    "crate_a",
    "crate_b",
]

# Shared dependencies across workspace
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
```

**Member `Cargo.toml`**:
```toml
[package]
name = "crate_a"
version = "0.1.0"
edition = "2021"

[dependencies]
crate_b = { path = "../crate_b" }
serde = { workspace = true }
```

#### **Workspace Commands**

```bash
# Build entire workspace
cargo build --workspace

# Test all crates
cargo test --workspace

# Build specific crate
cargo build -p crate_a

# Run binary from specific crate
cargo run -p crate_b --bin my_tool
```

---

### **14.4 - Installing Binaries with cargo install**

```bash
# Install ripgrep
cargo install ripgrep

# Install from git
cargo install --git https://github.com/user/repo

# Install specific version
cargo install cargo-watch --version 8.4.0

# List installed binaries
cargo install --list

# Uninstall
cargo uninstall ripgrep
```

---

### **14.5 - Extending Cargo with Custom Commands**

Create `cargo-mycmd` binary, and it becomes available as `cargo mycmd`:

```rust
// src/main.rs for cargo-mycmd
fn main() {
    println!("My custom Cargo command!");
}
```

```bash
cargo install --path .
cargo mycmd  # Runs your custom command!
```

---

## 🧠 Chapter 15: Smart Pointers

### **Core Philosophy**

Smart pointers are types that behave like pointers but provide additional metadata and capabilities. They enable patterns that ordinary references can't support while maintaining Rust's safety guarantees. Understanding when and how to use each smart pointer type is essential for mastering advanced Rust patterns.

---

### **15.1 - Box<T>: Heap Allocation**

#### **When to Use Box<T>**

1. **Recursive Types** - Types whose size depends on themselves:
```rust
// ❌ Infinite size
enum List {
    Cons(i32, List),
    Nil,
}

// ✅ Known size - pointer on stack, data on heap
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};
let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
```

2. **Large Data Transfer** - Move ownership without copying:
```rust
// 1MB struct
struct LargeData([u8; 1024 * 1024]);

// Moving just pointer, not 1MB
let data = Box::new(LargeData([0; 1024 * 1024]));
let transferred = data; // Only pointer moved!
```

3. **Trait Objects** - Dynamic dispatch:
```rust
trait Draw {
    fn draw(&self);
}

// Can hold any type implementing Draw
let shapes: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle { radius: 5 }),
    Box::new(Rectangle { width: 10, height: 20 }),
];
```

---

### **15.2 - Deref Trait: Smart Pointer Behavior**

The `Deref` trait enables `*` operator and automatic dereferencing:

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

let x = MyBox(5);
assert_eq!(5, *x); // Deref coercion: *x -> *(x.deref())
```

**Deref Coercion** - Automatic conversion:
```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

let m = MyBox::new(String::from("Rust"));
hello(&m); // &MyBox<String> -> &String -> &str
```

---

### **15.3 - Drop Trait: Cleanup Code**

The `Drop` trait runs cleanup code when values go out of scope:

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

{
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
} // Drops d then c (reverse order of creation)
```

**Early Drop**:
```rust
let c = CustomSmartPointer { data: String::from("some data") };
drop(c); // Explicit drop
// c is no longer valid here
```

---

### **15.4 - Rc<T>: Reference Counting**

`Rc<T>` enables multiple ownership through reference counting (single-threaded only):

```rust
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
println!("count after creating a = {}", Rc::strong_count(&a)); // 1

let b = Cons(3, Rc::clone(&a));
println!("count after creating b = {}", Rc::strong_count(&a)); // 2

{
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a)); // 3
} // c dropped
println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
```

**Key Points**:
- `Rc::clone` increments reference count (cheap!)
- Value dropped when count reaches zero
- Immutable data only (for mutable, use `Rc<RefCell<T>>`)
- Single-threaded (use `Arc<T>` for threads)

---

### **15.5 - RefCell<T>: Interior Mutability**

`RefCell<T>` enforces borrowing rules at **runtime** instead of compile-time:

**Box vs Rc vs RefCell Comparison**:

| Feature | `Box<T>` | `Rc<T>` | `RefCell<T>` |
|---------|----------|---------|--------------|
| **Ownership** | Single owner | Multiple owners | Single owner |
| **Borrowing Type** | Immutable or mutable | Immutable only | Immutable or mutable |
| **Borrow Check** | Compile-time | Compile-time | **Runtime** |
| **Use Case** | Heap allocation | Shared data | Interior mutability |
| **Thread Safety** | ✅ (if `T: Send`) | ❌ Single-threaded | ❌ Single-threaded |

**Basic Usage**:
```rust
use std::cell::RefCell;

let data = RefCell::new(5);

// Immutable borrow
let r1 = data.borrow();
let r2 = data.borrow(); // Multiple immutable OK
println!("r1: {}, r2: {}", *r1, *r2);
drop(r1);
drop(r2);

// Mutable borrow
let mut w = data.borrow_mut(); // Exclusive access
*w += 10;
drop(w);

println!("Final: {}", data.borrow());
```

**Rc<RefCell<T>> Pattern** - Multiple owners with mutation:
```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

let value = Rc::new(RefCell::new(5));

let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

// Mutate shared value through any owner
*value.borrow_mut() += 10;

println!("a: {:?}", a); // Shows 15
println!("b: {:?}", b); // Shows 15
println!("c: {:?}", c); // Shows 15
```

---

### **15.6 - Reference Cycles and Weak<T>**

#### **The Problem: Memory Leaks**

`Rc<T>` can create reference cycles that leak memory:

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Option<Rc<Node>>>, // ⚠️ Creates cycle!
}

// Parent -> Child (Rc)
// Child -> Parent (Rc) = CYCLE! Memory leak!
```

#### **The Solution: Weak<T>**

`Weak<T>` provides non-owning references that don't prevent deallocation:

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>, // ✅ Weak reference breaks cycle
}

let leaf = Rc::new(Node {
    value: 3,
    children: RefCell::new(vec![]),
    parent: RefCell::new(Weak::new()),
});

let branch = Rc::new(Node {
    value: 5,
    children: RefCell::new(vec![Rc::clone(&leaf)]),
    parent: RefCell::new(Weak::new()),
});

// Set parent using downgrade
*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

// Access parent through upgrade
if let Some(parent) = leaf.parent.borrow().upgrade() {
    println!("leaf's parent: {}", parent.value);
}
```

**Weak Reference Patterns**:
- Parent -> Child: Strong (`Rc<T>`)
- Child -> Parent: Weak (`Weak<T>`)
- Tree node -> Sibling: Weak
- Observer pattern: Weak references to observers

---

### **Mission Applications**

**Mission 3 - Binary Search Tree with Box**:
```rust
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}
```

**Mission 4 - Doubly Linked List with Rc<RefCell<T>>**:
```rust
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
    prev: Weak<RefCell<Node<T>>>, // Weak to prevent cycles
}
```

**Mission 7 - Graph with Smart Pointers**:
```rust
struct GraphNode {
    value: i32,
    neighbors: Vec<Rc<RefCell<GraphNode>>>,
}
```

---

## 🎓 Key Takeaways

### **Chapter 13: Functional Features**
1. **Closures capture environment** with minimal syntax, inferred types
2. **Fn trait family** (`Fn`, `FnMut`, `FnOnce`) determines closure capabilities
3. **Iterators are lazy** and enable zero-cost abstractions
4. **Iterator chains optimize** better than manual loops
5. **Custom iterators** implement simple `next()` method

### **Chapter 14: Cargo & Crates**
1. **Release profiles** customize builds for different scenarios
2. **Documentation** is first-class with `///` comments
3. **Workspaces** manage multi-crate projects efficiently
4. **Publishing** to crates.io requires semantic versioning
5. **Custom commands** extend Cargo's functionality

### **Chapter 15: Smart Pointers**
1. **Box<T>** enables heap allocation and recursive types
2. **Deref** makes smart pointers act like references
3. **Drop** runs cleanup automatically when values go out of scope
4. **Rc<T>** provides multiple ownership via reference counting
5. **RefCell<T>** allows runtime-checked mutation
6. **Weak<T>** prevents reference cycles and memory leaks

---

## 🔗 Integration Points

### **Builds On**
- [[rust-book-ch9-12-review]] - Error handling, generics, traits, lifetimes
- [[zettelkasten/ownership-fundamentals]] - Ownership and borrowing rules
- [[zettelkasten/Collections MOC]] - Vec, HashMap, iterators

### **Enables**
- [[mission-3]] - BST with Box<T>
- [[mission-4]] - LinkedList with Rc<RefCell<T>>
- [[mission-5]] - HashMap custom iterators
- [[mission-7]] - Graph traversal with iterators
- [[rust_book/rust-book-ch16]] - Concurrency patterns

### **Related Concepts**
- [[zettelkasten/zero-cost-abstractions]] - Compiler optimization philosophy
- [[zettelkasten/interior-mutability]] - RefCell and Cell patterns
- [[zettelkasten/refcell-interior-mutability]] - RefCell deep dive
- [[zettelkasten/smart-pointer-patterns]] - Advanced smart pointer usage

---

## 📚 Further Reading

### **Official Documentation**
- [The Rust Book - Chapter 13](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- [The Rust Book - Chapter 14](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html)
- [The Rust Book - Chapter 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

### **Workspace Resources**
- [[rust_book/Ch13/README.md]] - Chapter 13 examples and exercises
- [[rust_book/Ch14/README.md]] - Chapter 14 Cargo projects
- [[rust_book/Ch15/README.md]] - Chapter 15 smart pointer examples

---

*Tags: #rust-book #functional-programming #closures #iterators #cargo #smart-pointers #box #rc #refcell #ch13 #ch14 #ch15 #review*

*Links: [[rust-book-ch9-12-review]] | [[rust-book]] | [[zettelkasten/rust_book/rust-book-ch13]] | [[zettelkasten/rust_book/rust-book-ch14]] | [[zettelkasten/rust_book/rust-book-ch15]] | [[zettelkasten/zettel-index]]*
