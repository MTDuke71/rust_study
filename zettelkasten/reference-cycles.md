# Reference Cycles - Memory Leaks with Smart Pointers

*Reference cycles occur when smart pointers create circular ownership chains, preventing reference counts from reaching zero and causing memory leaks despite Rust's safety guarantees.*

---

## 🎯 **Core Concept**

A **reference cycle** is a situation where two or more smart pointers (typically `Rc<T>`) reference each other in a loop, creating a circular dependency. Because each pointer's reference count never reaches zero, the memory is never deallocated, causing a **memory leak**. This is one of the few ways to leak memory in safe Rust.

**Key Characteristics**:

- **Creates memory leaks**: Circular references prevent deallocation
- **Runtime problem**: Compile-time borrow checker doesn't prevent cycles
- **Only affects reference-counted types**: `Rc<T>`, `Arc<T>` (not `Box<T>`)
- **Solution**: Use `Weak<T>` for back-references to break cycles

**Why Reference Cycles Matter**:

1. **Memory safety gap**: One of few safe Rust patterns that can leak memory
2. **Graph structures**: Common in trees, doubly-linked lists, and graphs
3. **Prevention required**: Must consciously design ownership to avoid cycles
4. **Weak references**: Understanding `Weak<T>` is essential for cycle-free designs

---

## 🧠 **Mental Models**

### **The Mutual Dependency Trap**

**The Problem**:

```
Node A references Node B (Rc count: B = 1)
Node B references Node A (Rc count: A = 1)

Drop Node A? No! B still references it (count = 1)
Drop Node B? No! A still references it (count = 1)

Result: Both stuck in memory forever! 🔄💥
```

**The Analogy**:

```
Two people holding hands in a circle:
- Person A won't let go until Person B does
- Person B won't let go until Person A does
- Neither ever lets go = deadlock
```

### **The Strong vs Weak Reference Hierarchy**

```
Parent Node
   │
   │ Strong reference (Rc<Child>)
   │ "I own you, you stay alive as long as I do"
   ↓
Child Node
   │
   │ Weak reference (Weak<Parent>)
   │ "I know about you, but don't keep you alive"
   ↓
Parent Node (cycle broken!)
```

**Key Insight**: Strong references keep things alive, weak references just observe.

### **The Reference Counting Lifecycle**

```rust
// Normal (no cycle):
let a = Rc::new(Node);  // Rc count = 1
let b = Rc::clone(&a);  // Rc count = 2
drop(a);                // Rc count = 1
drop(b);                // Rc count = 0 → deallocated ✅

// With cycle:
let a = Rc::new(Node);     // a.count = 1
let b = Rc::new(Node);     // b.count = 1
a.next = Rc::clone(&b);    // b.count = 2
b.prev = Rc::clone(&a);    // a.count = 2 (CYCLE!)
drop(a);                   // a.count = 1 (still held by b)
drop(b);                   // b.count = 1 (still held by a)
// Both still alive! Memory leak! ❌
```

### **The Ownership Graph Analysis**

```
Safe Ownership (No Cycles):
     Root
    /    \
   A      B
  /        \
 C          D

Each edge is strong reference
Forms a tree (acyclic) ✅

Cyclic Ownership (Memory Leak):
     A ←──────┐
    ↓         │
    B → C → D─┘

D points back to A = cycle ❌
All reference counts stay > 0
```

---

## 🔍 **Detailed Content**

### **Creating a Reference Cycle (The Problem)**

#### **Classic Cycle Example**

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // Create two nodes
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a rc count = {}", Rc::strong_count(&a)); // 1
    
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b = {}", Rc::strong_count(&a)); // 2
    println!("b rc count = {}", Rc::strong_count(&b)); // 1
    
    // CREATE CYCLE: Make 'a' point back to 'b'
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    
    // Now we have: a → b → a (cycle!)
    println!("b rc count after cycle = {}", Rc::strong_count(&b)); // 2
    println!("a rc count after cycle = {}", Rc::strong_count(&a)); // 2
    
    // When main ends:
    // - Drop 'a': a.count becomes 1 (still held by b)
    // - Drop 'b': b.count becomes 1 (still held by a)
    // - Both leak! Neither deallocated!
}

// ⚠️ WARNING: Uncommenting this would cause stack overflow!
// println!("a next item = {:?}", a.tail());
// (infinite recursion trying to print the cycle)
```

---

### **Preventing Cycles with Weak<T>**

#### **The Weak<T> Solution**

`Weak<T>` provides non-owning references that don't prevent deallocation:

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,      // ✅ Weak reference (breaks cycle)
    children: RefCell<Vec<Rc<Node>>>, // Strong references to children
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    println!("leaf strong = {}, weak = {}", 
        Rc::strong_count(&leaf),  // 1
        Rc::weak_count(&leaf)     // 0
    );
    
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        // Set leaf's parent to branch using Weak
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        println!("branch strong = {}, weak = {}",
            Rc::strong_count(&branch), // 1
            Rc::weak_count(&branch)    // 1 (from leaf)
        );
        
        println!("leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),   // 2 (original + branch's child)
            Rc::weak_count(&leaf)      // 0
        );
        
        // Access parent through Weak::upgrade
        if let Some(parent) = leaf.parent.borrow().upgrade() {
            println!("leaf's parent value = {}", parent.value); // 5
        }
    } // branch goes out of scope
    
    // After branch dropped:
    println!("leaf parent after branch dropped = {:?}",
        leaf.parent.borrow().upgrade() // None - parent was deallocated!
    );
    
    println!("leaf strong = {}", Rc::strong_count(&leaf)); // 1
}
```

**Key Operations**:

- `Rc::downgrade(&rc)` - Create `Weak<T>` from `Rc<T>`
- `weak.upgrade()` - Try to get `Rc<T>` (returns `Option<Rc<T>>`)
- `Rc::weak_count(&rc)` - Count weak references
- `Rc::strong_count(&rc)` - Count strong references

---

### **Common Cycle-Prone Patterns**

#### **1. Parent-Child Relationships (Trees)**

**Problem**: Children need to know their parent, parent owns children

```rust
// ❌ Creates cycle
struct Node {
    parent: Rc<RefCell<Node>>,      // Strong both ways = cycle!
    children: Vec<Rc<RefCell<Node>>>,
}
```

**Solution**: Parent → Child (strong), Child → Parent (weak)

```rust
// ✅ Cycle-free
struct Node {
    parent: RefCell<Weak<Node>>,    // Weak reference up
    children: RefCell<Vec<Rc<Node>>>, // Strong reference down
}
```

#### **2. Doubly Linked Lists**

**Problem**: Each node needs to reference next AND previous

```rust
// ❌ Creates cycle
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>, // Strong both ways!
}
```

**Solution**: Use `Weak<T>` for back-references

```rust
// ✅ Cycle-free
struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>, // Weak backward
}
```

#### **3. Graph Structures with Cycles**

**Problem**: Graphs can have arbitrary cycles

```rust
// ❌ Direct cycles
struct GraphNode {
    neighbors: Vec<Rc<RefCell<GraphNode>>>, // All strong!
}
```

**Solutions**:

1. **Use indices instead of Rc**: [[arena-allocation]]
2. **Weak references for back-edges**: Designate some edges as weak
3. **Break cycles manually**: Explicitly clear references before drop

---

### **Detecting and Debugging Cycles**

#### **Signs of Reference Cycles**

1. **Memory usage grows continuously** despite dropping references
2. **Reference counts never reach zero**
3. **Stack overflow** when printing/traversing structures (infinite recursion)
4. **Valgrind/Memory profiler** shows unreachable allocations

#### **Debugging Techniques**

**Manual Reference Count Inspection**:

```rust
println!("Node strong count = {}", Rc::strong_count(&node));
println!("Node weak count = {}", Rc::weak_count(&node));

// If strong count doesn't decrease when dropping references = cycle likely!
```

**Cycle Detection with Visited Set**:

```rust
use std::collections::HashSet;

fn has_cycle(node: &Rc<RefCell<Node>>) -> bool {
    let mut visited = HashSet::new();
    detect_cycle(node, &mut visited)
}

fn detect_cycle(node: &Rc<RefCell<Node>>, visited: &mut HashSet<usize>) -> bool {
    let addr = Rc::as_ptr(node) as usize;
    
    if visited.contains(&addr) {
        return true; // Found cycle!
    }
    
    visited.insert(addr);
    
    for child in &node.borrow().children {
        if detect_cycle(child, visited) {
            return true;
        }
    }
    
    false
}
```

**Memory Profiling**:

```bash
# Use Valgrind to detect leaks
valgrind --leak-check=full ./target/debug/my_program

# Use heaptrack for detailed analysis
heaptrack ./target/debug/my_program
heaptrack --analyze heaptrack.my_program.*.gz
```

---

### **Prevention Strategies**

#### **1. Design Ownership Hierarchies**

**Principle**: Establish clear parent-child relationships

```rust
// Clear hierarchy: Parent owns children, children weakly reference parent
struct Parent {
    children: Vec<Rc<Child>>, // Strong down
}

struct Child {
    parent: Weak<Parent>, // Weak up
}
```

#### **2. Use Arena Allocation**

**Alternative**: Store all nodes in arena, use indices

```rust
struct Graph {
    nodes: Vec<Node>, // Arena owns all nodes
}

struct Node {
    neighbors: Vec<usize>, // Indices, not Rc!
}

// No reference counting = no cycles possible!
```

See [[arena-allocation]] for full pattern.

#### **3. Explicit Cleanup Methods**

**Pattern**: Provide methods to break cycles before drop

```rust
impl Node {
    /// Break all circular references before dropping
    pub fn cleanup(&mut self) {
        // Clear all Rc references
        self.children.clear();
        *self.parent.borrow_mut() = Weak::new();
    }
}
```

#### **4. Use Weak for Back-References**

**Rule of Thumb**:

- **Strong (Rc)**: Ownership edges in ownership tree
- **Weak**: Back-references, caches, observers, siblings

```rust
// Ownership tree
parent → child (Strong)
child → parent (Weak)

// Observer pattern
subject → observers (Strong)
observer → subject (Weak)

// Sibling references
node → sibling (Weak)
```

---

## 💡 **Key Takeaways**

1. **Reference cycles create memory leaks** - `Rc<T>` circular references prevent deallocation
2. **Weak<T> breaks cycles** - Non-owning references that don't keep values alive
3. **Design ownership hierarchies** - Clear parent-child relationships with strong down, weak up
4. **Common patterns**: Trees (parent/child), doubly-linked lists, graphs with back-edges
5. **Alternative: Arena allocation** - Use indices instead of `Rc` to eliminate cycles entirely
6. **Detection**: Monitor reference counts, use memory profilers, implement cycle detection
7. **Prevention is better than cure** - Design ownership graph to be acyclic from start

---

## 🔗 **Integration Points**

### **Builds On**

- [[rc-shared-ownership]] - Understanding Rc<T> reference counting
- [[refcell-interior-mutability]] - Often used with Rc for mutable shared state
- [[ownership-fundamentals]] - Core ownership and borrowing rules

### **Enables**

- [[tree-data-structures]] - Safe tree implementations with parent pointers
- [[graph-algorithms]] - Cycle-free graph structures
- [[observer-pattern]] - Observer pattern without memory leaks
- [[arena-allocation]] - Alternative memory management avoiding cycles

### **Related Concepts**

- [[weak-references]] - Weak<T> detailed usage patterns
- [[memory-leaks-in-rust]] - How safe Rust can still leak memory
- [[smart-pointer-patterns]] - Advanced smart pointer usage
- [[drop-trait]] - Understanding when values are deallocated

### **Mission Applications**

- [[mission-4]] - LinkedList using `Rc<RefCell<T>>` with cycle prevention
- [[mission-7]] - Graph implementation with weak references or arena allocation
- [[mission-3]] - Tree structures with proper parent-child relationships

### **Rust Book Reference**

- [[rust_book/rust-book-ch15]] - Chapter 15.6 on reference cycles
- [[rust-book-ch13-15-review]] - Smart pointers comprehensive review

---

## 📚 **Further Reading**

### **Official Documentation**

- [The Rust Book - Chapter 15.6](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)
- [std::rc::Weak Documentation](https://doc.rust-lang.org/std/rc/struct.Weak.html)
- [Rust Reference - Memory Leaks](https://doc.rust-lang.org/reference/destructors.html)

### **Workspace Examples**

- `rust_book/Ch15/examples/ch15_6_cycles.rs` - Reference cycle demonstrations
- [[mission-4]] - Practical doubly-linked list avoiding cycles
- [[daily-study/Day20]] - Smart pointer patterns and safety

### **Alternative Patterns**

- [[arena-allocation]] - Index-based graph storage (no reference counting)
- [[generational-indices]] - Safe deletion with index validation
- [[gc-alternatives]] - Rust alternatives to garbage collection

---

*Tags: #smart-pointers #rc #weak #reference-cycles #memory-leaks #advanced #memory-safety #graph-structures #tree-structures*

*Links: [[zettel-index]] | [[Smart Pointers MOC]] | [[rc-shared-ownership]] | [[refcell-interior-mutability]] | [[arena-allocation]] | [[weak-references]] | [[rust_book/rust-book-ch15]]*
