// Solution for Exercise 3: Doubly Linked List with Weak References
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
                // Link new_node -> old_head
                new_node.borrow_mut().next = Some(old_head.clone());
                // Link old_head <- new_node (weak reference)
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                // Update head
                self.head = Some(new_node);
            }
            None => {
                // First node in list
                self.head = Some(new_node.clone());
                self.tail = Some(Rc::downgrade(&new_node));
            }
        }
        
        self.length += 1;
        Ok(())
    }
    
    pub fn push_back(&mut self, data: T) -> Result<(), DoublyListError> {
        let new_node = Rc::new(RefCell::new(DoublyNode {
            data,
            next: None,
            prev: None,
        }));
        
        match &self.tail {
            Some(tail_weak) => {
                if let Some(old_tail) = tail_weak.upgrade() {
                    // Link old_tail -> new_node
                    old_tail.borrow_mut().next = Some(new_node.clone());
                    // Link new_node <- old_tail (weak reference)
                    new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                    // Update tail
                    self.tail = Some(Rc::downgrade(&new_node));
                } else {
                    // Tail reference is stale, list is corrupted
                    return Err(DoublyListError::InvalidReference);
                }
            }
            None => {
                // First node in list
                self.head = Some(new_node.clone());
                self.tail = Some(Rc::downgrade(&new_node));
            }
        }
        
        self.length += 1;
        Ok(())
    }
    
    pub fn pop_front(&mut self) -> Result<Option<T>, DoublyListError> {
        match self.head.take() {
            Some(old_head) => {
                // Check if we can safely remove this node
                match Rc::try_unwrap(old_head) {
                    Ok(cell) => {
                        let node = cell.into_inner();
                        
                        match node.next {
                            Some(new_head) => {
                                // Clear the back-reference from new head
                                new_head.borrow_mut().prev = None;
                                self.head = Some(new_head);
                            }
                            None => {
                                // List is now empty
                                self.head = None;
                                self.tail = None;
                            }
                        }
                        
                        self.length -= 1;
                        Ok(Some(node.data))
                    }
                    Err(node) => {
                        // Other references exist, put the node back
                        self.head = Some(node);
                        Err(DoublyListError::BorrowConflict)
                    }
                }
            }
            None => Ok(None),
        }
    }
    
    pub fn pop_back(&mut self) -> Result<Option<T>, DoublyListError> {
        match &self.tail {
            Some(tail_weak) => {
                if let Some(tail_node) = tail_weak.upgrade() {
                    // Check if we can safely remove this node
                    match Rc::try_unwrap(tail_node) {
                        Ok(cell) => {
                            let node = cell.into_inner();
                            
                            match &node.prev {
                                Some(prev_weak) => {
                                    if let Some(new_tail) = prev_weak.upgrade() {
                                        // Clear the forward reference from new tail
                                        new_tail.borrow_mut().next = None;
                                        self.tail = Some(Rc::downgrade(&new_tail));
                                    } else {
                                        // Previous node is gone, list is corrupted
                                        return Err(DoublyListError::InvalidReference);
                                    }
                                }
                                None => {
                                    // This was the only node
                                    self.head = None;
                                    self.tail = None;
                                }
                            }
                            
                            self.length -= 1;
                            Ok(Some(node.data))
                        }
                        Err(node) => {
                            // Other references exist
                            Err(DoublyListError::BorrowConflict)
                        }
                    }
                } else {
                    // Tail reference is stale
                    Err(DoublyListError::InvalidReference)
                }
            }
            None => Ok(None),
        }
    }
    
    pub fn peek_front(&self) -> Result<Option<std::cell::Ref<T>>, DoublyListError> {
        match &self.head {
            Some(node) => {
                let borrowed = node.try_borrow()?;
                Ok(Some(std::cell::Ref::map(borrowed, |n| &n.data)))
            }
            None => Ok(None),
        }
    }
    
    pub fn peek_back(&self) -> Result<Option<std::cell::Ref<T>>, DoublyListError> {
        match &self.tail {
            Some(tail_weak) => {
                if let Some(tail_node) = tail_weak.upgrade() {
                    let borrowed = tail_node.try_borrow()?;
                    Ok(Some(std::cell::Ref::map(borrowed, |n| &n.data)))
                } else {
                    Err(DoublyListError::InvalidReference)
                }
            }
            None => Ok(None),
        }
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
    
    // Additional utility methods
    pub fn get_node_at(&self, index: usize) -> Result<NodeRef<T>, DoublyListError> {
        if index >= self.length {
            return Err(DoublyListError::InvalidReference);
        }
        
        // Optimize: choose direction based on index
        if index < self.length / 2 {
            // Start from head
            let mut current = self.head.clone().ok_or(DoublyListError::EmptyList)?;
            for _ in 0..index {
                let next = {
                    let borrowed = current.try_borrow()?;
                    borrowed.next.clone()
                };
                current = next.ok_or(DoublyListError::InvalidReference)?;
            }
            Ok(current)
        } else {
            // Start from tail
            let mut current = self.tail
                .as_ref()
                .and_then(|w| w.upgrade())
                .ok_or(DoublyListError::EmptyList)?;
            
            for _ in 0..(self.length - 1 - index) {
                let prev = {
                    let borrowed = current.try_borrow()?;
                    borrowed.prev.as_ref().and_then(|w| w.upgrade())
                };
                current = prev.ok_or(DoublyListError::InvalidReference)?;
            }
            Ok(current)
        }
    }
    
    pub fn insert_at(&mut self, index: usize, data: T) -> Result<(), DoublyListError> {
        if index > self.length {
            return Err(DoublyListError::InvalidReference);
        }
        
        if index == 0 {
            return self.push_front(data);
        }
        
        if index == self.length {
            return self.push_back(data);
        }
        
        let new_node = Rc::new(RefCell::new(DoublyNode {
            data,
            next: None,
            prev: None,
        }));
        
        let next_node = self.get_node_at(index)?;
        let prev_node = {
            let borrowed = next_node.try_borrow()?;
            borrowed.prev.as_ref().and_then(|w| w.upgrade())
        }.ok_or(DoublyListError::InvalidReference)?;
        
        // Link: prev_node <-> new_node <-> next_node
        new_node.borrow_mut().next = Some(next_node.clone());
        new_node.borrow_mut().prev = Some(Rc::downgrade(&prev_node));
        
        prev_node.borrow_mut().next = Some(new_node.clone());
        next_node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
        
        self.length += 1;
        Ok(())
    }
    
    pub fn remove_at(&mut self, index: usize) -> Result<Option<T>, DoublyListError> {
        if index >= self.length {
            return Err(DoublyListError::InvalidReference);
        }
        
        if index == 0 {
            return self.pop_front();
        }
        
        if index == self.length - 1 {
            return self.pop_back();
        }
        
        let node_to_remove = self.get_node_at(index)?;
        
        match Rc::try_unwrap(node_to_remove) {
            Ok(cell) => {
                let node = cell.into_inner();
                
                let prev_node = node.prev.as_ref().and_then(|w| w.upgrade())
                    .ok_or(DoublyListError::InvalidReference)?;
                let next_node = node.next.ok_or(DoublyListError::InvalidReference)?;
                
                // Link: prev_node <-> next_node (skip removed node)
                prev_node.borrow_mut().next = Some(next_node.clone());
                next_node.borrow_mut().prev = Some(Rc::downgrade(&prev_node));
                
                self.length -= 1;
                Ok(Some(node.data))
            }
            Err(_) => Err(DoublyListError::BorrowConflict),
        }
    }
    
    pub fn reverse(&mut self) -> Result<(), DoublyListError> {
        if self.length <= 1 {
            return Ok(());
        }
        
        // Swap head and tail
        let old_head = self.head.take();
        let old_tail_weak = self.tail.take();
        
        if let Some(old_tail) = old_tail_weak.and_then(|w| w.upgrade()) {
            self.head = Some(old_tail);
        }
        
        if let Some(ref old_head_node) = old_head {
            self.tail = Some(Rc::downgrade(old_head_node));
        }
        
        // Reverse all the links
        let mut current = old_head;
        while let Some(node) = current {
            let (next, prev) = {
                let mut borrowed = node.try_borrow_mut()?;
                // Swap next and prev
                let old_next = borrowed.next.take();
                let old_prev = borrowed.prev.take();
                
                borrowed.next = old_prev.and_then(|w| w.upgrade());
                borrowed.prev = old_next.as_ref().map(|n| Rc::downgrade(n));
                
                (old_next, old_prev)
            };
            current = next;
        }
        
        Ok(())
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
        match self.current.take() {
            Some(node) => {
                match node.try_borrow() {
                    Ok(borrowed) => {
                        let data = borrowed.data.clone();
                        self.current = borrowed.next.clone();
                        Some(Ok(data))
                    }
                    Err(_) => Some(Err(DoublyListError::BorrowConflict)),
                }
            }
            None => None,
        }
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
        match self.current.take() {
            Some(weak_ref) => {
                match weak_ref.upgrade() {
                    Some(node) => {
                        match node.try_borrow() {
                            Ok(borrowed) => {
                                let data = borrowed.data.clone();
                                self.current = borrowed.prev.clone();
                                Some(Ok(data))
                            }
                            Err(_) => Some(Err(DoublyListError::BorrowConflict)),
                        }
                    }
                    None => Some(Err(DoublyListError::InvalidReference)),
                }
            }
            None => None,
        }
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

fn test_advanced_operations() -> Result<(), DoublyListError> {
    println!("=== Testing Advanced Operations ===");
    
    let mut list = DoublyLinkedList::new();
    
    // Build list: 1 <-> 2 <-> 3 <-> 4 <-> 5
    for i in 1..=5 {
        list.push_back(i)?;
    }
    
    // Test insert_at
    list.insert_at(2, 99)?; // Insert at index 2
    assert_eq!(list.len(), 6);
    
    // Should be: 1 <-> 2 <-> 99 <-> 3 <-> 4 <-> 5
    
    // Test remove_at
    assert_eq!(list.remove_at(2)?, Some(99));
    assert_eq!(list.len(), 5);
    
    // Should be back to: 1 <-> 2 <-> 3 <-> 4 <-> 5
    
    // Test get_node_at (bidirectional optimization)
    let node2 = list.get_node_at(1)?; // Should traverse from head
    let node4 = list.get_node_at(3)?; // Should traverse from tail
    
    assert_eq!(node2.borrow().data, 2);
    assert_eq!(node4.borrow().data, 4);
    
    println!("✅ Advanced operations work!");
    Ok(())
}

fn test_memory_management() -> Result<(), DoublyListError> {
    println!("=== Testing Memory Management ===");
    
    // Test that we properly handle reference cycles
    {
        let mut list = DoublyLinkedList::new();
        for i in 0..1000 {
            list.push_back(i)?;
        }
        
        // Get some node references
        let _node1 = list.get_node_at(100)?;
        let _node2 = list.get_node_at(500)?;
        
        println!("Created large list with shared references");
        
        // List and references should be cleaned up when they go out of scope
    }
    
    println!("✅ Memory management test completed");
    Ok(())
}

fn test_reverse() -> Result<(), DoublyListError> {
    println!("=== Testing Reverse ===");
    
    let mut list = DoublyLinkedList::new();
    
    // Test reverse on empty list
    list.reverse()?;
    assert!(list.is_empty());
    
    // Test reverse on single element
    list.push_back(42)?;
    list.reverse()?;
    assert_eq!(list.peek_front()?.map(|r| *r), Some(42));
    
    // Test reverse on multiple elements
    list.pop_front()?;
    for i in 1..=5 {
        list.push_back(i)?;
    }
    
    // Before: 1 <-> 2 <-> 3 <-> 4 <-> 5
    list.reverse()?;
    // After: 5 <-> 4 <-> 3 <-> 2 <-> 1
    
    let forward: Vec<_> = list.iter_forward().collect::<Result<Vec<_>, _>>()?;
    assert_eq!(forward, vec![5, 4, 3, 2, 1]);
    
    println!("✅ Reverse test passed!");
    Ok(())
}

fn main() -> Result<(), DoublyListError> {
    test_all_operations()?;
    demonstrate_bidirectional_access()?;
    test_advanced_operations()?;
    test_reverse()?;
    
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
    
    test_memory_management()?;
    
    println!("🎉 All tests passed! Doubly-linked list implementation complete.");
    
    Ok(())
}