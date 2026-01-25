# `Box<T>` for Recursive Data Structures

*Tags: #rust #box #smart-pointers #recursive-types #ownership #heap-allocation*

---

## The Problem: Infinite Size

Rust needs to know the size of every type at compile time. Recursive types create infinite size:

```rust
// ❌ DOESN'T COMPILE - infinite size!
struct Node<T> {
    value: T,
    left: Node<T>,   // Contains another Node...
    right: Node<T>,  // which contains another Node... (infinite!)
}
```

**Compiler error**: "recursive type has infinite size"

---

## The Solution: `Box<T>`

`Box<T>` is a **heap-allocated pointer** with known size (pointer size, typically 8 bytes on 64-bit):

```rust
// ✅ COMPILES - Box has known size!
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,   // Pointer to heap (8 bytes)
    right: Option<Box<Node<T>>>,  // Pointer to heap (8 bytes)
}
```

**Why this works:**
- `Box<Node<T>>` is a **pointer** (fixed size: 8 bytes on 64-bit)
- Actual `Node<T>` lives on the **heap** (size doesn't matter for stack frame)
- `Option` adds ability to have "no child" (None)

---

## The Pattern: `Option<Box<Node<T>>>`

This is the **standard Rust pattern** for recursive structures:

| Component | Purpose                                         |
| --------- | ----------------------------------------------- |
| `Option`  | Allows None (no child) or Some (child exists)   |
| `Box`     | Heap allocation, breaks infinite size recursion |
| `Node<T>` | Your recursive type                             |

**Memory Layout:**
```
Stack:                  Heap:
┌─────────────┐        ┌──────────────┐
│ Node        │        │ Node         │
│  value: 10  │        │  value: 5    │
│  left: Some │───────>│  left: None  │
│  right: Some│─┐      │  right: None │
└─────────────┘ │      └──────────────┘
                │      ┌──────────────┐
                └─────>│ Node         │
                       │  value: 15   │
                       │  left: None  │
                       │  right: None │
                       └──────────────┘
```

---

## Bottom-Up Construction

Rust's ownership rules force **bottom-up** construction (build leaves first):

```rust
// Build leaves (no children)
let left = Box::new(Node::new(5));
let right = Box::new(Node::new(15));

// Build parent (consumes children - ownership transferred)
let root = Node {
    value: 10,
    left: Some(left),   // Ownership of left moved here
    right: Some(right), // Ownership of right moved here
};

// ❌ Can't use left or right anymore - ownership moved!
```

**Why bottom-up?**
- Prevents dangling references (child can't outlive parent)
- Ensures memory safety (no use-after-free)
- Forces clear ownership hierarchy

---

## Common Recursive Structures

### Binary Search Tree (Mission 12)
```rust
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}
```

### Linked List (Mission 4)
```rust
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
```

### Graph (Mission 8)
```rust
struct Node<T> {
    value: T,
    neighbors: Vec<Box<Node<T>>>,  // Multiple children
}
```

---

## Key Operations

### Insertion (BST Example)
```rust
fn insert(&mut self, value: T) {
    match self.root {
        None => {
            // Create new heap-allocated node
            self.root = Some(Box::new(Node::new(value)));
        }
        Some(ref mut node) => {
            // Recursive insertion
            if value < node.value {
                Self::insert_recursive(&mut node.left, value);
            } else {
                Self::insert_recursive(&mut node.right, value);
            }
        }
    }
}
```

### Traversal (Recursive)
```rust
fn traverse(node: &Option<Box<Node<T>>>) {
    if let Some(current) = node {
        traverse(&current.left);   // Visit left
        println!("{:?}", current.value); // Process
        traverse(&current.right);  // Visit right
    }
}
```

### Memory Cleanup
```rust
// Automatic! Box implements Drop
// When node goes out of scope:
//   1. Recursively drops all children
//   2. Deallocates heap memory
//   3. No manual memory management needed
```

---

## When to Use `Box<T>`

**Use Box when:**
- ✅ Building recursive data structures (trees, lists, graphs)
- ✅ Need heap allocation with single ownership
- ✅ Want to transfer large data without copying
- ✅ Need known size for recursive types

**Don't use Box when:**
- ❌ Multiple owners needed (use `Rc<T>` or `Arc<T>`)
- ❌ Interior mutability required (use `RefCell<T>` with `Rc`)
- ❌ Data fits comfortably on stack
- ❌ Zero-copy semantics needed (use references)

---

## Related Patterns

**`Rc<RefCell<Node<T>>>`** (Multiple owners + mutation):
```rust
use std::rc::Rc;
use std::cell::RefCell;

// Mission 4 LinkedList with shared ownership
type `Link<T>` = Option<Rc<RefCell<Node<T>>>>;
```

**`Arc<Mutex<Node<T>>>`** (Thread-safe shared ownership):
```rust
use std::sync::{Arc, Mutex};

// Multi-threaded graph traversal
type Link<T> = Option<Arc<Mutex<Node<T>>>>;
```

---

## Performance Characteristics

| Operation          | Stack Allocation    | Heap Allocation (Box)   |
| ------------------ | ------------------- | ----------------------- |
| **Allocation**     | Fast (pointer bump) | Slower (heap allocator) |
| **Access**         | Direct              | One indirection         |
| **Cache locality** | Better              | Worse (scattered)       |
| **Size limit**     | Stack size (~MB)    | Heap size (GB)          |
| **Deallocation**   | Automatic (scope)   | Automatic (Drop)        |

**Trade-off:** Box trades some performance (heap allocation, indirection) for **flexibility and safety** in recursive structures.

---

## Real-World Examples

### Mission 12 Tutorial Step 2: BST
```rust
// tutorials/Mission12_tut/examples/step2_binary_search_tree.rs
pub struct BinarySearchTree<T> {
    root: Option<Box<Node<T>>>,  // ← This pattern!
    size: usize,
}
```

### Standard Library: LinkedList
```rust
// std::collections::LinkedList (internal implementation)
struct Node<T> {
    next: Option<Box<Node<T>>>,
    prev: *mut Node<T>,  // Raw pointer for doubly-linked
    element: T,
}
```

---

## Common Mistakes

**1. Trying to return owned Box from Option:**
```rust
// ❌ Wrong - moves value out of Option
fn get_left(&self) -> Option<Box<Node<T>>> {
    self.left  // Can't move out of borrowed reference!
}

// ✅ Correct - return reference
fn get_left(&self) -> Option<&Node<T>> {
    self.left.as_ref().map(|boxed| boxed.as_ref())
}
```

**2. Forgetting Option:**
```rust
// ❌ Wrong - every node must have both children!
struct Node<T> {
    left: Box<Node<T>>,   // Can't be empty!
    right: Box<Node<T>>,
}

// ✅ Correct - Option allows missing children
struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}
```

---

## Integration Points

**Mission 4 (Linked List):**
- Uses `Option<Box<Node<T>>>` for singly-linked list
- Advanced: `Rc<RefCell<Node<T>>>` for doubly-linked with shared ownership

**Mission 8 (Graphs):**
- Trees are directed acyclic graphs (DAGs)
- BST is a special case: directed, acyclic, ordered

**Mission 12 (Trees):**
- Step 1: Basic binary tree with Box
- Step 2: Binary search tree (ordered)
- Step 4: Self-balancing trees (AVL, Red-Black)

**Rust Book Ch15 (Smart Pointers):**
- `Box<T>` fundamentals
- `Rc<T>` and `Arc<T>` for shared ownership
- `RefCell<T>` for interior mutability

---

## Mathematical Connection

**Trees as Graphs:**
- **Nodes**: Vertices in graph theory
- **Box pointers**: Directed edges
- **None**: No edge (leaf node)
- **Recursive structure**: Natural graph representation

See: [[graph-theory-fundamentals]], [[tree-traversal-algorithms]]

---

*Links:*
- [[mission-4]] - LinkedList implementation
- [[mission-8]] - Graph algorithms
- [[mission-12]] - Tree structures and traversals
- [[rust-book-ch15]] - Smart Pointers chapter
- [[rust-ownership]] - Ownership fundamentals
- [[cargo-feature-patterns]] - When to use Box vs Rc vs Arc based on features

*Related Concepts:*
- [[graph-theory-fundamentals]] - Trees as special graphs
- [[tree-traversal-algorithms]] - Recursive traversal patterns
- [[rust-for-rustaceans]] - Advanced ownership patterns

*Created: 2026-01-25*  
*Context: Mission 12 Tutorial Step 2 (BST), understanding recursive data structures in Rust*
