// Step 6: Runtime Borrow Checking
// Run with: cargo run --example step6_borrow_checking

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
            next: self.head.clone(),
        }));
        self.head = Some(new_node);
        self.length += 1;
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
            let next = current.borrow().next.clone().ok_or(LinkedListError::IndexOutOfBounds)?;
            current = next;
        }
        Ok(current)
    }
    
    pub fn len(&self) -> usize {
        self.length
    }
}

fn demonstrate_successful_borrowing() {
    println!("=== Demonstrating Successful Borrowing ===");
    
    let mut list = RcLinkedList::new();
    list.push_front(42);
    list.push_front(24);
    
    // Multiple immutable borrows are OK
    match list.try_peek_front() {
        Ok(Some(borrow1)) => {
            println!("First borrow: {}", *borrow1);
            // borrow1 is still alive here
            
            match list.try_peek_front() {
                Ok(Some(borrow2)) => {
                    println!("Second borrow: {}", *borrow2);
                    println!("Both borrows active simultaneously: OK!");
                }
                Err(e) => println!("Second borrow failed: {:?}", e),
            }
            // Both borrows dropped here
        }
        Err(e) => println!("First borrow failed: {:?}", e),
    }
}

fn demonstrate_borrow_conflicts() {
    println!("=== Demonstrating Borrow Conflicts ===");
    
    let mut list = RcLinkedList::new();
    list.push_front(100);
    
    // Get a node reference
    if let Ok(node_ref) = list.get_node_ref(0) {
        // Hold a mutable borrow
        let _mut_borrow = node_ref.borrow_mut();
        println!("Acquired mutable borrow");
        
        // Try to get an immutable borrow - this should fail!
        match node_ref.try_borrow() {
            Ok(_) => println!("❌ This shouldn't succeed!"),
            Err(_) => println!("✅ Correctly detected borrow conflict (mut + immut)"),
        }
        
        // Try to get another mutable borrow - this should also fail!
        match node_ref.try_borrow_mut() {
            Ok(_) => println!("❌ This shouldn't succeed!"),
            Err(_) => println!("✅ Correctly detected borrow conflict (mut + mut)"),
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

fn demonstrate_scope_management() {
    println!("=== Demonstrating Scope Management ===");
    
    let mut list = RcLinkedList::new();
    list.push_front(String::from("Hello"));
    
    let node_ref = list.get_node_ref(0).unwrap();
    
    // Careful scope management prevents conflicts
    {
        let immut_borrow = node_ref.borrow();
        println!("Immutable borrow: {}", immut_borrow.data);
        // immut_borrow dropped at end of this scope
    }
    
    {
        let mut mut_borrow = node_ref.borrow_mut();
        println!("Mutable borrow: {}", mut_borrow.data);
        mut_borrow.push_str(" World");
        println!("Modified: {}", mut_borrow.data);
        // mut_borrow dropped at end of this scope
    }
    
    {
        let final_borrow = node_ref.borrow();
        println!("Final value: {}", final_borrow.data);
    }
}

fn demonstrate_error_handling() {
    println!("=== Demonstrating Error Handling ===");
    
    let mut list = RcLinkedList::new();
    list.push_front("Test");
    
    // Get a node and hold a mutable borrow
    if let Ok(node_ref) = list.get_node_ref(0) {
        let _mut_borrow = node_ref.borrow_mut();
        
        // Try operations that should fail gracefully
        match list.try_peek_front() {
            Ok(Some(_)) => println!("❌ peek succeeded when it shouldn't"),
            Ok(None) => println!("❌ peek returned None when it shouldn't"),
            Err(LinkedListError::BorrowConflict) => println!("✅ peek correctly reported borrow conflict"),
            Err(e) => println!("❌ peek failed with unexpected error: {:?}", e),
        }
        
        match list.try_peek_front_mut() {
            Ok(Some(_)) => println!("❌ peek_mut succeeded when it shouldn't"),
            Ok(None) => println!("❌ peek_mut returned None when it shouldn't"),
            Err(LinkedListError::BorrowConflict) => println!("✅ peek_mut correctly reported borrow conflict"),
            Err(e) => println!("❌ peek_mut failed with unexpected error: {:?}", e),
        }
    }
}

fn main() {
    println!("=== Step 6: Runtime Borrow Checking ===");
    
    demonstrate_successful_borrowing();
    println!();
    
    demonstrate_borrow_conflicts();
    println!();
    
    demonstrate_scope_management();
    println!();
    
    demonstrate_error_handling();
    
    println!();
    println!("📝 Key Insights:");
    println!("- RefCell enforces borrowing rules at RUNTIME, not compile time");
    println!("- Multiple immutable borrows are allowed simultaneously");
    println!("- Mutable borrows are exclusive (no other borrows allowed)");
    println!("- Use try_borrow() and try_borrow_mut() for graceful error handling");
    println!("- Manage borrow lifetimes carefully with explicit scopes");
    println!("- Always handle potential BorrowError and BorrowMutError");
    
    println!("✅ Runtime borrow checking demonstrated!");
}