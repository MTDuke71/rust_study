// Solution for Exercise 2: Rc<RefCell<T>> Patterns
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
        Self {
            head: None,
            length: 0,
        }
    }
    
    pub fn push_front(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(RcNode {
            data,
            next: self.head.clone(), // Clone the Rc, not the data!
        }));
        self.head = Some(new_node);
        self.length += 1;
    }
    
    pub fn try_pop_front(&mut self) -> Result<Option<T>, LinkedListError> {
        match self.head.take() {
            Some(node) => {
                // Check if we have the only reference
                match Rc::try_unwrap(node) {
                    Ok(cell) => {
                        // We have exclusive access, safe to extract
                        let node_data = cell.into_inner();
                        self.head = node_data.next;
                        self.length -= 1;
                        Ok(Some(node_data.data))
                    }
                    Err(node) => {
                        // Other references exist, put the node back and return error
                        self.head = Some(node);
                        Err(LinkedListError::BorrowConflict)
                    }
                }
            }
            None => Ok(None),
        }
    }
    
    pub fn try_peek_front(&self) -> Result<Option<std::cell::Ref<T>>, LinkedListError> {
        match &self.head {
            Some(node) => {
                let borrowed = node.try_borrow()?;
                Ok(Some(std::cell::Ref::map(borrowed, |n| &n.data)))
            }
            None => Ok(None),
        }
    }
    
    pub fn try_peek_front_mut(&mut self) -> Result<Option<std::cell::RefMut<T>>, LinkedListError> {
        match &self.head {
            Some(node) => {
                let borrowed = node.try_borrow_mut()?;
                Ok(Some(std::cell::RefMut::map(borrowed, |n| &mut n.data)))
            }
            None => Ok(None),
        }
    }
    
    pub fn get_node_ref(&self, index: usize) -> Result<NodeRef<T>, LinkedListError> {
        if index >= self.length {
            return Err(LinkedListError::IndexOutOfBounds);
        }
        
        let mut current = self.head.clone().ok_or(LinkedListError::EmptyList)?;
        for _ in 0..index {
            let next = {
                let borrowed = current.try_borrow()?;
                borrowed.next.clone()
            };
            current = next.ok_or(LinkedListError::IndexOutOfBounds)?;
        }
        Ok(current)
    }
    
    pub fn len(&self) -> usize {
        self.length
    }
    
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
    
    // Bonus implementations
    pub fn insert_at(&mut self, index: usize, data: T) -> Result<(), LinkedListError> {
        if index > self.length {
            return Err(LinkedListError::IndexOutOfBounds);
        }
        
        if index == 0 {
            self.push_front(data);
            return Ok(());
        }
        
        let prev_node = self.get_node_ref(index - 1)?;
        let new_node = Rc::new(RefCell::new(RcNode {
            data,
            next: None,
        }));
        
        {
            let mut prev_borrow = prev_node.try_borrow_mut()?;
            new_node.borrow_mut().next = prev_borrow.next.clone();
            prev_borrow.next = Some(new_node);
        }
        
        self.length += 1;
        Ok(())
    }
    
    pub fn remove_at(&mut self, index: usize) -> Result<Option<T>, LinkedListError> {
        if index >= self.length {
            return Err(LinkedListError::IndexOutOfBounds);
        }
        
        if index == 0 {
            return self.try_pop_front();
        }
        
        let prev_node = self.get_node_ref(index - 1)?;
        let node_to_remove = {
            let prev_borrow = prev_node.try_borrow()?;
            prev_borrow.next.clone().ok_or(LinkedListError::IndexOutOfBounds)?
        };
        
        // Check if we can safely remove this node
        match Rc::try_unwrap(node_to_remove) {
            Ok(cell) => {
                let node_data = cell.into_inner();
                prev_node.try_borrow_mut()?.next = node_data.next;
                self.length -= 1;
                Ok(Some(node_data.data))
            }
            Err(_) => Err(LinkedListError::BorrowConflict),
        }
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
    
    // Demonstrate that they point to the same memory
    let first_addr = first_node.as_ptr();
    let first_again = list.get_node_ref(0)?;
    let first_again_addr = first_again.as_ptr();
    println!("Same memory address? {}", first_addr == first_again_addr);
    
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
            Err(_) => println!("✅ Correctly caught borrow conflict (mut + immut)"),
        }
        
        // Try to get another mutable borrow - this should also fail!
        match node_ref.try_borrow_mut() {
            Ok(_) => println!("❌ This shouldn't work!"),
            Err(_) => println!("✅ Correctly caught borrow conflict (mut + mut)"),
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

fn demonstrate_modification_through_shared_refs() -> Result<(), LinkedListError> {
    println!("=== Demonstrating Modification Through Shared References ===");
    
    let mut list = RcLinkedList::new();
    list.push_front(String::from("Original"));
    
    // Get a shared reference to the node
    let node_ref = list.get_node_ref(0)?;
    
    // Modify through the shared reference
    {
        let mut borrowed = node_ref.try_borrow_mut()?;
        borrowed.data.push_str(" - Modified!");
    }
    
    // Check that the modification is visible through the list
    if let Ok(Some(front)) = list.try_peek_front() {
        println!("Data after modification: {}", *front);
    }
    
    Ok(())
}

fn test_edge_cases() -> Result<(), LinkedListError> {
    println!("=== Testing Edge Cases ===");
    
    let mut list = RcLinkedList::new();
    
    // Test operations on empty list
    assert_eq!(list.try_pop_front()?, None);
    assert!(list.try_peek_front()?.is_none());
    assert!(list.get_node_ref(0).is_err());
    
    // Test single element
    list.push_front(42);
    assert_eq!(list.len(), 1);
    
    let node_ref = list.get_node_ref(0)?;
    assert_eq!(Rc::strong_count(&node_ref), 2); // list + our reference
    
    // Can't pop while we hold a reference
    assert!(list.try_pop_front().is_err());
    
    // Drop our reference
    drop(node_ref);
    
    // Now we can pop
    assert_eq!(list.try_pop_front()?, Some(42));
    assert_eq!(list.len(), 0);
    
    println!("✅ All edge cases passed!");
    Ok(())
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
    
    // Test popping (but first we need to ensure no shared references)
    assert_eq!(list.try_pop_front()?, Some(3));
    assert_eq!(list.len(), 2);
    
    println!("✅ Basic operations work!");
    
    // Demonstrate advanced features
    demonstrate_shared_references()?;
    demonstrate_borrow_conflicts();
    demonstrate_modification_through_shared_refs()?;
    test_edge_cases()?;
    
    // Test bonus features
    println!("=== Testing Bonus Features ===");
    let mut test_list = RcLinkedList::new();
    test_list.push_front(1);
    test_list.push_front(2);
    test_list.push_front(3);
    
    // Insert at position 1
    test_list.insert_at(1, 99)?;
    assert_eq!(test_list.len(), 4);
    
    // Remove from position 1
    assert_eq!(test_list.remove_at(1)?, Some(99));
    assert_eq!(test_list.len(), 3);
    
    println!("✅ Bonus features work!");
    
    Ok(())
}