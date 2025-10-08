# Exercise 1: Basic Box Implementation

**Learning Objective**: Implement basic linked list operations using `Box<T>`

## Your Task

Complete the implementation of `SimpleLinkedList<T>` with the following methods:

```rust
// exercises/exercise1_box_basics.rs
use std::fmt::Debug;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        // TODO: Implement
        todo!()
    }
    
    pub fn push_front(&mut self, data: T) {
        // TODO: Implement
        // Hint: Create a new node with the current head as its next
        todo!()
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        // TODO: Implement
        // Hint: Use self.head.take() and pattern matching
        todo!()
    }
    
    pub fn peek_front(&self) -> Option<&T> {
        // TODO: Implement
        // Hint: Use as_ref() to get a reference without taking ownership
        todo!()
    }
    
    pub fn len(&self) -> usize {
        self.length
    }
    
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

// Test your implementation
fn main() {
    let mut list = SimpleLinkedList::new();
    
    println!("=== Testing Basic Operations ===");
    
    // Test empty list
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.peek_front(), None);
    
    // Test push_front
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    
    assert!(!list.is_empty());
    assert_eq!(list.len(), 3);
    assert_eq!(list.peek_front(), Some(&3));
    
    // Test pop_front
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.len(), 2);
    assert_eq!(list.peek_front(), Some(&2));
    
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
    
    assert!(list.is_empty());
    
    println!("✅ All tests passed!");
}
```

## Self-Assessment Questions

1. **Why do we use `Option<Box<Node<T>>>` instead of just `Box<Node<T>>`?**
   <details>
   <summary>Click for answer</summary>
   
   Because the list might be empty! `Option` allows us to represent "no node" with `None`.
   </details>

2. **What does `take()` do and why is it useful here?**
   <details>
   <summary>Click for answer</summary>
   
   `take()` moves the value out of an `Option`, leaving `None` behind. This transfers ownership without cloning, which is essential for moving nodes around.
   </details>

3. **Why don't we need to manually free memory?**
   <details>
   <summary>Click for answer</summary>
   
   Rust's ownership system ensures that when a `Box` goes out of scope, its memory is automatically deallocated. When we drop a node, it automatically drops its `next` node, creating a chain reaction.
   </details>

## Common Mistakes to Avoid

- ❌ **Cloning unnecessarily**: Don't `clone()` the entire node when you can move it
- ❌ **Forgetting to update length**: Always increment/decrement the length counter
- ❌ **Not handling empty lists**: Always check if the list is empty before operations

## Next Steps

Once you complete this exercise:
1. Run `cargo run --bin exercise1_box_basics` to test your implementation
2. Add a `display()` method that prints all elements
3. Implement `push_back()` for practice (harder!)

---

## 🔗 Related Zettelkasten Concepts

**Core Smart Pointers:**
- [[Box Smart Pointer Patterns]] - Heap allocation and ownership management
- [[box]] - Box<T> for single ownership on the heap
- [[ownership]] - Rust ownership system fundamentals
- [[move-semantics]] - Transfer of ownership without copying

**Data Structures:**
- [[linked-list]] - Linked list patterns and implementations
- [[option]] - Option<T> for null safety
- [[take-method]] - Option::take() for ownership transfer

**Memory Management:**
- [[stack-vs-heap]] - Stack vs heap allocation strategies
- [[raii]] - Resource Acquisition Is Initialization pattern
- [[drop-trait]] - Automatic cleanup on scope exit

**Mission Integration:**
- [[Mission4 Overview]] - Linked list mission context
- [[Mission4_tut Overview]] - Complete tutorial progression
- [[exercise2_rc_patterns]] - Next exercise: shared ownership

**Learning Resources:**
- [[rust-book-ch15]] - Smart Pointers chapter
- [[Daily Study MOC]] - Ownership and borrowing days

*Tags: #mission4 #tutorial #box #linked-list #ownership #exercise #hands-on*