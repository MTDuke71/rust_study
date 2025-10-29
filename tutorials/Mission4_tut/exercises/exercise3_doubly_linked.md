# Exercise 3: Advanced Challenge - Doubly Linked List

**Learning Objective**: Combine `Rc<RefCell<T>>` and `Weak<T>` to build a doubly-linked list without reference cycles

## Your Mission

Implement a doubly-linked list that supports:
- Bidirectional traversal
- Insertion/deletion at both ends
- No memory leaks (prevent reference cycles)

```rust
// exercises/exercise3_doubly_linked.rs
use std::rc::{Rc, Weak};
use std::cell::RefCell;

type NodeRef<T> = Rc<RefCell<DoublyNode<T>>>;
type WeakNodeRef<T> = Weak<RefCell<DoublyNode<T>>>;

#[derive(Debug)]
struct DoublyNode<T> {
    data: T,
    next: Option<NodeRef<T>>,
    prev: Option<WeakNodeRef<T>>, // Weak reference to prevent cycles!
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Option<NodeRef<T>>,
    tail: Option<WeakNodeRef<T>>, // Weak reference to tail
    length: usize,
}

#[derive(Debug)]
pub enum DoublyListError {
    BorrowConflict,
    EmptyList,
    InvalidReference,
}

impl From<std::cell::BorrowError> for DoublyListError {
    fn from(_: std::cell::BorrowError) -> Self {
        DoublyListError::BorrowConflict
    }
}

impl From<std::cell::BorrowMutError> for DoublyListError {
    fn from(_: std::cell::BorrowMutError) -> Self {
        DoublyListError::BorrowConflict
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }
    
    pub fn push_front(&mut self, data: T) -> Result<(), DoublyListError> {
        let new_node = Rc::new(RefCell::new(DoublyNode {
            data,
            next: None,
            prev: None,
        }));
        
        match &self.head {
            Some(old_head) => {
                // TODO: Implement
                // 1. Set new_node.next to point to old_head
                // 2. Set old_head.prev to point to new_node (weak reference)
                // 3. Update self.head to new_node
                todo!()
            }
            None => {
                // TODO: Implement
                // 1. Set self.head to new_node
                // 2. Set self.tail to weak reference to new_node
                todo!()
            }
        }
        
        self.length += 1;
        Ok(())
    }
    
    pub fn push_back(&mut self, data: T) -> Result<(), DoublyListError> {
        // TODO: Implement
        // Hint: This is trickier because tail is a weak reference
        // You'll need to upgrade it to a strong reference first
        todo!()
    }
    
    pub fn pop_front(&mut self) -> Result<Option<T>, DoublyListError> {
        // TODO: Implement
        // 1. Take the head node
        // 2. If there's a next node, make it the new head and clear its prev
        // 3. If no next node, clear the tail as well
        // 4. Extract and return the data
        todo!()
    }
    
    pub fn pop_back(&mut self) -> Result<Option<T>, DoublyListError> {
        // TODO: Implement
        // This is the most complex operation!
        // You need to navigate from tail to the previous node
        todo!()
    }
    
    pub fn peek_front(&self) -> Result<Option<std::cell::Ref<T>>, DoublyListError> {
        // TODO: Implement
        todo!()
    }
    
    pub fn peek_back(&self) -> Result<Option<std::cell::Ref<T>>, DoublyListError> {
        // TODO: Implement
        // Hint: Use tail.upgrade() to get a strong reference
        todo!()
    }
    
    // Bonus: Implement bidirectional iteration
    pub fn iter_forward(&self) -> ForwardIter<T> {
        ForwardIter {
            current: self.head.clone(),
        }
    }
    
    pub fn iter_backward(&self) -> BackwardIter<T> {
        BackwardIter {
            current: self.tail.clone(),
        }
    }
    
    pub fn len(&self) -> usize {
        self.length
    }
    
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

// Forward iterator
pub struct ForwardIter<T> {
    current: Option<NodeRef<T>>,
}

impl<T> Iterator for ForwardIter<T>
where
    T: Clone,
{
    type Item = Result<T, DoublyListError>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Implement
        // Move to the next node and return the current data
        todo!()
    }
}

// Backward iterator
pub struct BackwardIter<T> {
    current: Option<WeakNodeRef<T>>,
}

impl<T> Iterator for BackwardIter<T>
where
    T: Clone,
{
    type Item = Result<T, DoublyListError>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Implement
        // This is tricky - you need to upgrade weak references
        todo!()
    }
}

// Helper function to demonstrate the power of doubly-linked lists
fn demonstrate_bidirectional_access() -> Result<(), DoublyListError> {
    let mut list = DoublyLinkedList::new();
    
    // Build a list: 1 <-> 2 <-> 3
    list.push_back(1)?;
    list.push_back(2)?;
    list.push_back(3)?;
    
    println!("=== Forward iteration ===");
    for item in list.iter_forward() {
        match item {
            Ok(data) => println!("Forward: {}", data),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    
    println!("=== Backward iteration ===");
    for item in list.iter_backward() {
        match item {
            Ok(data) => println!("Backward: {}", data),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    
    Ok(())
}

fn test_all_operations() -> Result<(), DoublyListError> {
    let mut list = DoublyLinkedList::new();
    
    println!("=== Testing All Operations ===");
    
    // Test empty list
    assert!(list.is_empty());
    assert_eq!(list.pop_front()?, None);
    assert_eq!(list.pop_back()?, None);
    
    // Test push_front
    list.push_front(2)?;
    list.push_front(1)?;
    assert_eq!(list.len(), 2);
    
    // Test push_back
    list.push_back(3)?;
    list.push_back(4)?;
    assert_eq!(list.len(), 4);
    
    // Should be: 1 <-> 2 <-> 3 <-> 4
    
    // Test peeking
    if let Ok(Some(front)) = list.peek_front() {
        assert_eq!(*front, 1);
    }
    
    if let Ok(Some(back)) = list.peek_back() {
        assert_eq!(*back, 4);
    }
    
    // Test popping from front
    assert_eq!(list.pop_front()?, Some(1));
    assert_eq!(list.len(), 3);
    
    // Test popping from back
    assert_eq!(list.pop_back()?, Some(4));
    assert_eq!(list.len(), 2);
    
    // Should be: 2 <-> 3
    
    println!("✅ All basic operations work!");
    Ok(())
}

fn main() -> Result<(), DoublyListError> {
    test_all_operations()?;
    demonstrate_bidirectional_access()?;
    
    // Memory leak test
    println!("=== Memory Leak Test ===");
    {
        let mut list = DoublyLinkedList::new();
        for i in 0..1000 {
            list.push_back(i)?;
        }
        println!("Created list with 1000 elements");
        // List should be properly cleaned up when it goes out of scope
    }
    println!("✅ List cleaned up without manual intervention");
    
    Ok(())
}
```

## Key Concepts in This Exercise

### 1. Weak References (`Weak<T>`)

```rust
// Strong reference (owns the data)
let strong: Rc<RefCell<i32>> = Rc::new(RefCell::new(42));

// Weak reference (doesn't own the data)
let weak: Weak<RefCell<i32>> = Rc::downgrade(&strong);

// Convert weak back to strong (may fail if original was dropped)
if let Some(upgraded) = weak.upgrade() {
    println!("Still alive: {}", upgraded.borrow());
}
```

### 2. Breaking Reference Cycles

```rust
// ❌ This creates a cycle and leaks memory:
// A -> B -> A (both are Rc, so reference count never reaches 0)

// ✅ This breaks the cycle:
// A -> B (Rc)
// A <- B (Weak) - doesn't count toward reference count
```

### 3. Bidirectional Navigation

The key insight is that `prev` pointers are `Weak` references, so you need to upgrade them:

```rust
// Going backward requires upgrading weak references
let current_weak = &node.borrow().prev;
if let Some(prev_node) = current_weak.as_ref().and_then(|w| w.upgrade()) {
    // Now we have a strong reference to the previous node
}
```

## Self-Assessment Questions

1. **Why do we use `Weak<T>` for `prev` pointers?**
   <details>
   <summary>Click for answer</summary>
   
   To prevent reference cycles. If both `next` and `prev` were `Rc`, we'd have cycles where nodes keep each other alive forever, causing memory leaks.
   </details>

2. **What happens when you call `upgrade()` on a `Weak<T>`?**
   <details>
   <summary>Click for answer</summary>
   
   It attempts to convert the weak reference back to a strong reference (`Rc`). Returns `Some(Rc<T>)` if the original data is still alive, or `None` if it was dropped.
   </details>

3. **Why is `pop_back()` more complex than `pop_front()`?**
   <details>
   <summary>Click for answer</summary>
   
   Because you need to navigate backward from the tail using weak references, and update the previous node's `next` pointer to None.
   </details>

## Debugging Tips

1. **Use `Rc::strong_count()`** to check reference counts
2. **Always check `upgrade()` results** - weak references can become invalid
3. **Draw diagrams** of your pointer relationships
4. **Test with single elements** before testing complex scenarios

## Success Criteria

Your implementation should:
- ✅ Support insertion/deletion at both ends
- ✅ Allow bidirectional iteration
- ✅ Never leak memory (no reference cycles)
- ✅ Handle edge cases (empty list, single element)
- ✅ Provide proper error handling for borrow conflicts

## Bonus Challenges

1. **Implement `insert_at(index, data)`** for insertion at any position
2. **Add `remove_at(index)`** for removal at any position  
3. **Create a `find(predicate)` method** that returns node references
4. **Implement `reverse()`** that reverses the list in-place

---

## 🔗 Related Zettelkasten Concepts

**Advanced Smart Pointers:**
- [[weak-references]] - Weak<T> for breaking reference cycles
- [[rc-weak-patterns]] - Rc/Weak combination patterns
- [[reference-cycles]] - Detecting and preventing memory leaks
- [[PhantomData Type Safety Patterns]] - Advanced type safety

**Interior Mutability:**
- [[Interior Mutability Deep Dive]] - Comprehensive guide
- [[refcell-patterns]] - RefCell usage patterns
- [[Borrow Checker Patterns and Troubleshooting]] - Handling runtime borrow errors

**Data Structure Design:**
- [[doubly-linked-list]] - Bidirectional linked lists
- [[graph-structures]] - Graph-like data structures
- [[memory-leak-prevention]] - Preventing cycles and leaks

**Mission Integration:**
- [[Mission4 Overview]] - Linked list mission
- [[exercise1_box_basics]] - Basic ownership patterns
- [[exercise2_rc_patterns]] - Shared ownership foundation
- [[Mission4_tut Overview]] - Complete tutorial series

**Debugging & Testing:**
- [[strong-count]] - Rc::strong_count() debugging
- [[weak-count]] - Rc::weak_count() monitoring
- [[upgrade-pattern]] - Weak::upgrade() usage

**Learning Resources:**
- [[rust-book-ch15]] - Smart Pointers (Weak<T>)
- [[rust-book-ch16]] - Fearless Concurrency patterns
- [[Daily Study MOC]] - Advanced ownership patterns

*Tags: #mission4 #tutorial #doubly-linked-list #weak #rc #advanced #memory-management #exercise*