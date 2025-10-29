# Exercise 2: Rc<RefCell<T>> Patterns

**Learning Objective**: Understand shared ownership and interior mutability

## Your Task

Implement a linked list that allows sharing references to nodes using `Rc<RefCell<T>>`:

```rust
// exercises/exercise2_rc_patterns.rs
use std::rc::Rc;
use std::cell::RefCell;

type NodeRef<T> = Rc<RefCell<RcNode<T>>>;

#[derive(Debug)]
struct RcNode<T> {
    data: T,
    next: Option<NodeRef<T>>,
}

#[derive(Debug)]
pub struct RcLinkedList<T> {
    head: Option<NodeRef<T>>,
    length: usize,
}

#[derive(Debug)]
pub enum LinkedListError {
    BorrowConflict,
    EmptyList,
    IndexOutOfBounds,
}

impl From<std::cell::BorrowError> for LinkedListError {
    fn from(_: std::cell::BorrowError) -> Self {
        LinkedListError::BorrowConflict
    }
}

impl From<std::cell::BorrowMutError> for LinkedListError {
    fn from(_: std::cell::BorrowMutError) -> Self {
        LinkedListError::BorrowConflict
    }
}

impl<T> RcLinkedList<T> {
    pub fn new() -> Self {
        // TODO: Implement
        todo!()
    }
    
    pub fn push_front(&mut self, data: T) {
        // TODO: Implement
        // Hint: Create Rc::new(RefCell::new(...)) and clone the current head
        todo!()
    }
    
    pub fn try_pop_front(&mut self) -> Result<Option<T>, LinkedListError> {
        // TODO: Implement
        // Hint: Check if there are multiple references before modifying
        // Use Rc::try_unwrap() to get exclusive access
        todo!()
    }
    
    pub fn try_peek_front(&self) -> Result<Option<std::cell::Ref<T>>, LinkedListError> {
        // TODO: Implement
        // Hint: Use node.try_borrow() and Ref::map to access the data
        todo!()
    }
    
    pub fn get_node_ref(&self, index: usize) -> Result<NodeRef<T>, LinkedListError> {
        // TODO: Implement
        // This method returns a reference to the node at the given index
        // This allows sharing references to internal nodes!
        todo!()
    }
    
    pub fn len(&self) -> usize {
        self.length
    }
}

// Advanced: Reference sharing demonstration
fn demonstrate_shared_references() -> Result<(), LinkedListError> {
    let mut list = RcLinkedList::new();
    
    // Add some data
    list.push_front("Third");
    list.push_front("Second");
    list.push_front("First");
    
    println!("=== Demonstrating Shared References ===");
    
    // Get references to different nodes
    let first_node = list.get_node_ref(0)?;
    let second_node = list.get_node_ref(1)?;
    
    println!("First node data: {:?}", first_node.try_borrow()?.data);
    println!("Second node data: {:?}", second_node.try_borrow()?.data);
    
    // We can hold multiple references to the same structure!
    println!("Reference count for first node: {}", Rc::strong_count(&first_node));
    
    Ok(())
}

// Test borrow checking at runtime
fn demonstrate_borrow_conflicts() {
    let mut list = RcLinkedList::new();
    list.push_front(42);
    
    println!("=== Demonstrating Borrow Conflicts ===");
    
    // Get a node reference
    if let Ok(node_ref) = list.get_node_ref(0) {
        // Hold a mutable borrow
        let _mut_borrow = node_ref.borrow_mut();
        
        // Try to get another borrow - this should fail!
        match node_ref.try_borrow() {
            Ok(_) => println!("❌ This shouldn't work!"),
            Err(_) => println!("✅ Correctly caught borrow conflict"),
        }
        
        // _mut_borrow is dropped here
    }
    
    // Now borrowing should work again
    if let Ok(node_ref) = list.get_node_ref(0) {
        match node_ref.try_borrow() {
            Ok(borrow) => println!("✅ Borrowing works after conflict resolved: {}", borrow.data),
            Err(_) => println!("❌ Should be able to borrow now"),
        }
    }
}

fn main() -> Result<(), LinkedListError> {
    println!("=== Testing Rc<RefCell<T>> Implementation ===");
    
    let mut list = RcLinkedList::new();
    
    // Basic operations
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    
    assert_eq!(list.len(), 3);
    
    // Test peeking
    if let Ok(Some(front)) = list.try_peek_front() {
        assert_eq!(*front, 3);
    }
    
    // Test popping
    assert_eq!(list.try_pop_front()?, Some(3));
    assert_eq!(list.len(), 2);
    
    println!("✅ Basic operations work!");
    
    // Demonstrate advanced features
    demonstrate_shared_references()?;
    demonstrate_borrow_conflicts();
    
    Ok(())
}
```

## Key Concepts to Understand

### 1. Reference Counting with `Rc<T>`

```rust
let node = Rc::new(RefCell::new(RcNode { data: 42, next: None }));
let node_ref1 = node.clone(); // Increments reference count
let node_ref2 = node.clone(); // Increments reference count

println!("Reference count: {}", Rc::strong_count(&node)); // Should be 3
```

### 2. Interior Mutability with `RefCell<T>`

```rust
// Multiple immutable borrows OK
let borrow1 = node.borrow();
let borrow2 = node.borrow();

// But can't mix with mutable borrow
let mut_borrow = node.borrow_mut(); // This would panic if borrow1 is still alive
```

### 3. Runtime vs Compile-time Checking

- **Compile-time**: Regular references (`&T`, `&mut T`)
- **Runtime**: `RefCell` borrows (`borrow()`, `borrow_mut()`)

## Self-Assessment Questions

1. **When would you choose `Rc<RefCell<T>>` over `Box<T>`?**
   <details>
   <summary>Click for answer</summary>
   
   When you need to share references to the same data across multiple parts of your program, or when you need to mutate data through shared references.
   </details>

2. **What happens if you try to `borrow_mut()` while another borrow exists?**
   <details>
   <summary>Click for answer</summary>
   
   The program will panic at runtime with "already borrowed" error. Use `try_borrow_mut()` to handle this gracefully.
   </details>

3. **Why use `Rc::try_unwrap()` in `pop_front()`?**
   <details>
   <summary>Click for answer</summary>
   
   Because if there are other references to the node, we can't safely remove it from the list. `try_unwrap()` only succeeds if we have the only reference.
   </details>

## Challenge Extensions

1. **Implement `try_peek_front_mut()`** that returns a mutable reference
2. **Add `insert_at(index, data)`** that inserts at any position
3. **Implement `remove_at(index)`** that removes from any position
4. **Add cycle detection** to prevent infinite loops

## Common Pitfalls

- 🚨 **Forgetting to use `try_` methods**: Always handle potential borrow failures
- 🚨 **Reference cycles**: Be careful not to create cycles that prevent cleanup
- 🚨 **Holding borrows too long**: Drop `Ref` and `RefMut` guards quickly

---

## 🔗 Related Zettelkasten Concepts

**Core Smart Pointers:**
- [[rc-refcell-patterns]] - Rc<RefCell<T>> usage patterns
- [[interior-mutability]] - Interior mutability and RefCell
- [[Interior Mutability Deep Dive]] - Comprehensive interior mutability guide
- [[reference-counting]] - Rc<T> and reference counting mechanics

**Borrow Checking:**
- [[runtime-borrow-checking]] - RefCell runtime checks
- [[Borrow Checker Fundamentals]] - Rust borrowing rules and enforcement
- [[try-borrow]] - Graceful borrow conflict handling

**Error Handling:**
- [[error-handling]] - Custom error types and Result<T, E>
- [[from-trait]] - Error conversion with From trait
- [[panic-vs-result]] - When to panic vs return errors

**Data Structures:**
- [[linked-list]] - Linked list implementations
- [[shared-ownership]] - Rc<T> for shared ownership
- [[weak-references]] - Weak<T> for breaking cycles

**Mission Integration:**
- [[Mission4 Overview]] - Linked list mission
- [[exercise1_box_basics]] - Previous exercise: Box basics
- [[exercise3_doubly_linked]] - Next exercise: doubly linked list
- [[Mission4_tut Overview]] - Complete tutorial guide

**Learning Resources:**
- [[rust-book-ch15]] - Smart Pointers
- [[rust-book-ch9]] - Error Handling

*Tags: #mission4 #tutorial #rc #refcell #interior-mutability #shared-ownership #exercise*