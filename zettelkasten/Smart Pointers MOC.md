# Smart Pointers MOC - Map of Content

*A comprehensive guide to Rust's smart pointer ecosystem: Box, Rc, RefCell, Weak, and advanced memory management patterns.*

---

## 🎯 **Overview**

Smart pointers are types that implement the `Deref` and/or `Drop` traits to provide pointer-like behavior with additional capabilities. They enable patterns that ordinary references can't support while maintaining Rust's memory safety guarantees.

**Core Smart Pointer Types**:

- **Box<T>** - Heap allocation, single ownership, recursive types
- **Rc<T>** - Reference counting, multiple ownership (single-threaded)
- **RefCell<T>** - Interior mutability, runtime borrow checking
- **Weak<T>** - Non-owning references, cycle prevention
- **Arc<T>** - Atomic reference counting (thread-safe Rc)
- **Mutex<T> / RwLock<T>** - Thread-safe interior mutability

---

## 📚 **Fundamental Concepts**

- [[smart-pointers]] - Overview and selection guide for pointer types

### **Smart Pointer Traits**

- [[deref-trait]] - Dereference operator overloading and coercion
- [[drop-trait]] - Automatic cleanup and RAII patterns
- [[smart-pointer-patterns]] - Common idioms and design patterns

### **Core Smart Pointers**

- [[box-heap-allocation]] - Box<T> for heap allocation and recursive types
- [[rc-shared-ownership]] - Rc<T> for multiple ownership and shared data
- [[refcell-interior-mutability]] - RefCell<T> for runtime-checked mutation
- [[reference-cycles]] - Memory leaks with Rc and prevention with Weak<T>

### **Memory Management**

- [[arena-allocation]] - Alternative to reference counting using indices
- [[memory-leaks-in-rust]] - How safe Rust can still leak memory
- [[ownership-fundamentals]] - Foundation for understanding smart pointers

---

## 🔍 **Smart Pointer Comparison**

### **Quick Reference Table**

| Smart Pointer | Ownership | Borrowing | Borrow Check | Thread Safe | Use Case |
|---------------|-----------|-----------|--------------|-------------|----------|
| **Box<T>** | Single | Immutable or mutable | Compile-time | ✅ (if T: Send) | Heap allocation, recursive types |
| **Rc<T>** | Multiple | Immutable only | Compile-time | ❌ Single-threaded | Shared read-only data |
| **RefCell<T>** | Single | Immutable or mutable | **Runtime** | ❌ Single-threaded | Interior mutability |
| **Weak<T>** | Non-owning | Immutable | Compile-time | ❌ Single-threaded | Break reference cycles |
| **Arc<T>** | Multiple | Immutable only | Compile-time | ✅ Thread-safe | Shared data across threads |
| **Mutex<T>** | Single | Exclusive mutable | Runtime | ✅ Thread-safe | Mutate across threads |
| **RwLock<T>** | Single | Multiple readers / single writer | Runtime | ✅ Thread-safe | Read-heavy scenarios |

### **Common Combinations**

**Shared Mutable State (Single-threaded)**:

```rust
Rc<RefCell<T>>
// Multiple ownership + interior mutability
// Use: Graphs, trees, shared caches
```

**Shared Mutable State (Multi-threaded)**:

```rust
Arc<Mutex<T>>
// Thread-safe shared ownership + exclusive mutation
// Use: Concurrent data structures
```

**Tree with Parent Pointers**:

```rust
Rc<Node> {
    children: Vec<Rc<Node>>,     // Strong references down
    parent: RefCell<Weak<Node>>, // Weak reference up
}
```

---

## 🧠 **When to Use Which Smart Pointer**

### **Use Box<T> When**

- ✅ Allocating large data on heap (avoid stack overflow)
- ✅ Transferring ownership without copying large values
- ✅ Implementing recursive types (linked lists, trees)
- ✅ Using trait objects for dynamic dispatch
- ✅ Returning heap-allocated values from functions

**Example**: Binary search tree, recursive enums

### **Use Rc<T> When**

- ✅ Multiple parts of program need read access
- ✅ Shared ownership of immutable data
- ✅ Graph structures with shared nodes
- ✅ Implementing graphs, DAGs (Directed Acyclic Graphs)
- ❌ NOT for thread-safe code (use Arc instead)

**Example**: Graph nodes with multiple incoming edges

### **Use RefCell<T> When**

- ✅ Need mutation through `&self` (immutable reference)
- ✅ Borrow rules correct but compiler can't verify
- ✅ Implementing mock objects for testing
- ✅ Caching/memoization with lazy initialization
- ⚠️ Runtime panics if borrow rules violated

**Example**: Cache that mutates on read, mock objects

### **Use Rc<RefCell<T>> When**

- ✅ Multiple ownership + mutation needed
- ✅ Shared mutable state (single-threaded)
- ✅ Graph nodes that need to be mutated
- ✅ Observer pattern, event systems
- ⚠️ Watch for reference cycles!

**Example**: Doubly-linked list, graph with mutable nodes

### **Use Weak<T> When**

- ✅ Preventing reference cycles in Rc graphs
- ✅ Child → parent references in trees
- ✅ Cache entries that can be evicted
- ✅ Observer pattern (observers don't own subject)
- ✅ Sibling references in data structures

**Example**: Tree node referencing parent, observer pattern

---

## 🏗️ **Common Patterns & Idioms**

### **Pattern: Recursive Data Structures**

```rust
// Binary tree with Box
enum Tree<T> {
    Leaf(T),
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
}
```

**See**: [[box-heap-allocation]], [[mission-3]]

### **Pattern: Shared Ownership Graph**

```rust
// Graph with Rc
struct GraphNode {
    value: i32,
    neighbors: Vec<Rc<RefCell<GraphNode>>>,
}
```

**See**: [[rc-shared-ownership]], [[mission-7]]

### **Pattern: Tree with Parent Pointers**

```rust
// Cycle-free tree
struct TreeNode {
    value: i32,
    children: Vec<Rc<RefCell<TreeNode>>>,
    parent: RefCell<Weak<RefCell<TreeNode>>>,
}
```

**See**: [[reference-cycles]], [[weak-references]]

### **Pattern: Interior Mutability**

```rust
// Cache with lazy initialization
struct Cache {
    data: RefCell<HashMap<String, Value>>,
}

impl Cache {
    fn get(&self, key: &str) -> Option<Value> {
        if !self.data.borrow().contains_key(key) {
            // Mutate through &self!
            self.data.borrow_mut().insert(key.to_string(), compute(key));
        }
        self.data.borrow().get(key).cloned()
    }
}
```

**See**: [[refcell-interior-mutability]]

### **Pattern: Arena Allocation (Alternative)**

```rust
// Graph with indices instead of Rc
struct Graph {
    nodes: Vec<Node>, // Arena
}

struct Node {
    neighbors: Vec<usize>, // Indices, not Rc!
}
```

**See**: [[arena-allocation]]

---

## 🎓 **Learning Path**

### **Beginner: Understanding the Basics**

1. [[ownership-fundamentals]] - Start with ownership rules
2. [[box-heap-allocation]] - First smart pointer: Box<T>
3. [[deref-trait]] - How smart pointers act like references
4. [[drop-trait]] - Automatic cleanup and RAII

**Practice**: Implement binary search tree with Box<T>

### **Intermediate: Shared Ownership**

1. [[rc-shared-ownership]] - Multiple ownership with Rc<T>
2. [[refcell-interior-mutability]] - Runtime borrow checking
3. [[reference-cycles]] - Understanding memory leaks
4. **Combine**: Rc<RefCell<T>> pattern for shared mutable state

**Practice**: Implement graph with Rc<RefCell<T>>

### **Advanced: Cycle Prevention & Optimization**

1. [[weak-references]] - Breaking cycles with Weak<T>
2. [[arena-allocation]] - Index-based alternative to Rc
3. [[generational-indices]] - Safe deletion with arenas
4. **Optimize**: Choose right pattern for use case

**Practice**: Implement tree with parent pointers using Weak<T>

---

## 🚀 **Mission & Study Integration**

### **Missions Using Smart Pointers**

- [[mission-3]] - Binary Search Tree with `Box<T>`
- [[mission-4]] - Doubly Linked List with `Rc<RefCell<T>>` and `Weak<T>`
- [[mission-7]] - Graph Algorithms with shared ownership patterns

### **Daily Study Coverage**

- [[daily-study/Day20]] - Introduction to smart pointers
- [[daily-study/Day21]] - Deref and Drop traits
- [[daily-study/Day22]] - Rc and RefCell patterns

### **Rust Book Integration**

- [[rust_book/rust-book-ch15]] - Complete smart pointers chapter
- [[rust-book-ch13-15-review]] - Comprehensive review with examples

### **Advent of Code Applications**

- Tree problems → Box<T> for recursive structures
- Graph traversal → Rc<T> for shared nodes
- State machines → RefCell<T> for interior mutability

---

## 🔧 **Troubleshooting & Common Pitfalls**

### **Common Errors**

**1. Reference Cycles with Rc**

```rust
// ❌ Problem: Cycle causes memory leak
a.next = Rc::clone(&b);
b.prev = Rc::clone(&a); // Cycle!

// ✅ Solution: Use Weak for back-references
b.prev = Rc::downgrade(&a);
```

**See**: [[reference-cycles]]

**2. Runtime Borrow Panics with RefCell**

```rust
// ❌ Problem: Multiple mutable borrows
let mut a = data.borrow_mut();
let mut b = data.borrow_mut(); // Panic!

// ✅ Solution: Drop first borrow
{
    let mut a = data.borrow_mut();
    // use a
} // a dropped here
let mut b = data.borrow_mut(); // OK
```

**See**: [[refcell-interior-mutability]]

**3. Trying to Use Rc Across Threads**

```rust
// ❌ Problem: Rc is not Send
let data = Rc::new(42);
thread::spawn(move || println!("{}", data)); // Error!

// ✅ Solution: Use Arc instead
let data = Arc::new(42);
thread::spawn(move || println!("{}", data)); // OK
```

---

## 📊 **Performance Considerations**

### **Memory Overhead**

| Type | Stack Size | Heap Overhead | Notes |
|------|------------|---------------|-------|
| `Box<T>` | 8 bytes (pointer) | `size_of::<T>()` | Zero runtime cost |
| `Rc<T>` | 8 bytes (pointer) | `size_of::<T>() + 16` | Strong + weak counters |
| `RefCell<T>` | `size_of::<T>() + 16` | 0 | Borrow state tracking |
| `Rc<RefCell<T>>` | 8 bytes | `size_of::<T>() + 32` | Counters + borrow state |

### **Runtime Costs**

- **Box<T>**: Zero abstraction cost (just a pointer)
- **Rc<T>**: Clone increments counter (non-atomic, cheap)
- **RefCell<T>**: Runtime borrow tracking (small cost on borrow/release)
- **Arc<T>**: Atomic counter operations (more expensive than Rc)

### **When to Optimize**

1. **Profile first**: Don't optimize without measurements
2. **Consider arena allocation**: For many small objects
3. **Use Box when possible**: Simplest, fastest smart pointer
4. **Avoid unnecessary clones**: Borrow instead when possible

---

## 🔗 **Related Topics**

### **Core Concepts**

- [[ownership-fundamentals]] | [[borrowing-rules]] | [[lifetimes]]
- [[trait-system]] | [[generics]] | [[type-system]]

### **Memory Management**

- [[heap-vs-stack]] | [[raii-pattern]] | [[memory-safety]]
- [[zero-cost-abstractions]] | [[compile-time-guarantees]]

### **Advanced Patterns**

- [[interior-mutability]] | [[shared-state]] | [[observer-pattern]]
- [[state-machine-patterns]] | [[cache-patterns]]

### **Alternative Approaches**

- [[arena-allocation]] | [[generational-indices]] | [[pool-allocation]]
- [[lifetime-based-ownership]] | [[borrow-splitting]]

---

## 📚 **External Resources**

### **Official Documentation**

- [The Rust Book - Chapter 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [std::boxed::Box](https://doc.rust-lang.org/std/boxed/struct.Box.html)
- [std::rc::Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)
- [std::cell::RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html)

### **Further Reading**

- [Rust RFC: Smart Pointers](https://github.com/rust-lang/rfcs)
- [Rust Nomicon: Ownership](https://doc.rust-lang.org/nomicon/ownership.html)
- [Too Many Lists](https://rust-unofficial.github.io/too-many-lists/) - Comprehensive linked list guide

---

*Last Updated: November 17, 2025*

*Tags: #moc #smart-pointers #box #rc #refcell #weak #memory-management #ownership #rust-book-ch15*

*Navigation: [[zettel-index]] | [[rust-concepts-MOC]] | [[Memory Management]] | [[ownership-fundamentals]]*
