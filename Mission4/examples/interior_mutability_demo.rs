//! Interior Mutability Demo for Mission 4
//! 
//! This example demonstrates advanced Rc<RefCell<T>> patterns and their challenges,
//! including borrow checking at runtime and shared ownership scenarios.

use mission4::{RcLinkedList, LinkedListError};
use std::rc::Rc;

fn main() {
    println!("🔬 Interior Mutability Demo: Rc<RefCell<T>> Patterns");
    println!("=====================================================\n");

    demo_basic_interior_mutability();
    println!();
    demo_runtime_borrow_checking();
    println!();
    demo_shared_ownership_patterns();
    println!();
    demo_cycle_prevention();
}

fn demo_basic_interior_mutability() {
    println!("📝 Basic Interior Mutability");
    println!("----------------------------");
    
    let mut list = RcLinkedList::new();
    list.push_front(String::from("Hello"));
    list.push_front(String::from("World"));
    
    println!("Created list with strings");
    
    // Demonstrate safe borrowing
    match list.try_peek_front() {
        Ok(Some(value_ref)) => {
            println!("Front element: '{}'", *value_ref);
            println!("Front element length: {}", value_ref.len());
        },
        Ok(None) => println!("List is empty"),
        Err(e) => println!("Borrow failed: {}", e),
    }
    
    // Demonstrate mutable borrowing
    match list.try_peek_front_mut() {
        Ok(Some(mut value_ref)) => {
            println!("Before mutation: '{}'", *value_ref);
            value_ref.push_str("!");
            println!("After mutation: '{}'", *value_ref);
        },
        Ok(None) => println!("List is empty"),
        Err(e) => println!("Mutable borrow failed: {}", e),
    };
}

fn demo_runtime_borrow_checking() {
    println!("⏰ Runtime Borrow Checking");
    println!("--------------------------");
    
    let mut list = RcLinkedList::new();
    list.push_front(42);
    
    // Get an immutable reference
    let immutable_ref = list.try_peek_front().unwrap().unwrap();
    println!("Got immutable reference to: {}", *immutable_ref);
    
    // Try to get a mutable reference while immutable one exists
    match list.try_peek_front_mut() {
        Ok(_) => println!("ERROR: Should not be able to get mutable reference!"),
        Err(LinkedListError::BorrowError) => {
            println!("✓ Correctly prevented mutable borrow (immutable exists)");
        },
        Err(e) => println!("Unexpected error: {:?}", e),
    }
    
    // Drop the immutable reference
    drop(immutable_ref);
    println!("Dropped immutable reference");
    
    // Now mutable reference should work
    match list.try_peek_front_mut() {
        Ok(Some(mut value)) => {
            println!("✓ Successfully got mutable reference after dropping immutable");
            *value = 100;
            println!("Modified value to: {}", *value);
        },
        Ok(None) => println!("List is empty"),
        Err(e) => println!("Unexpected error: {:?}", e),
    };
}

fn demo_shared_ownership_patterns() {
    println!("🤝 Shared Ownership Patterns");
    println!("----------------------------");
    
    let mut list = RcLinkedList::new();
    list.push_front(vec![1, 2, 3, 4, 5]);
    
    // Get references to the head node
    let head_ref1 = list.get_head_ref().unwrap();
    let head_ref2 = list.get_head_ref().unwrap();
    let head_ref3 = head_ref1.clone();
    
    println!("Created multiple references to head node");
    println!("Reference count: {}", Rc::strong_count(&head_ref1));
    
    // Try to pop with multiple references
    match list.pop_front() {
        Ok(_) => println!("ERROR: Should not be able to pop with multiple refs!"),
        Err(LinkedListError::MultipleReferences) => {
            println!("✓ Correctly prevented pop (multiple references exist)");
        },
        Err(e) => println!("Unexpected error: {:?}", e),
    }
    
    // Show that we can still access data through references
    {
        let borrowed = head_ref1.borrow();
        println!("Data through ref1: {:?}", borrowed.data);
    }
    
    // Drop references one by one
    drop(head_ref3);
    println!("After dropping ref3, count: {}", Rc::strong_count(&head_ref1));
    
    drop(head_ref2);
    println!("After dropping ref2, count: {}", Rc::strong_count(&head_ref1));
    
    drop(head_ref1);
    println!("Dropped all external references");
    
    // Now pop should work
    match list.pop_front() {
        Ok(Some(data)) => println!("✓ Successfully popped: {:?}", data),
        Ok(None) => println!("List is empty"),
        Err(e) => println!("Unexpected error: {:?}", e),
    }
}

fn demo_cycle_prevention() {
    println!("🔄 Cycle Prevention with Weak References");
    println!("----------------------------------------");
    
    let mut list = RcLinkedList::new();
    list.push_front("Node 3");
    list.push_front("Node 2");
    list.push_front("Node 1");
    
    println!("Created 3-node list");
    
    // The implementation uses weak references internally to prevent cycles
    if let Some(head_ref) = list.get_head_ref() {
        println!("Head node strong count: {}", Rc::strong_count(&head_ref));
        
        // Access the next node
        let next_node = {
            let head_borrowed = head_ref.borrow();
            head_borrowed.next.clone()
        };
        
        if let Some(next_ref) = next_node {
            println!("Next node strong count: {}", Rc::strong_count(&next_ref));
            
            // Check if next node has weak reference back
            let next_borrowed = next_ref.borrow();
            if next_borrowed.prev_weak.is_some() {
                println!("✓ Next node has weak back-reference (prevents cycles)");
            } else {
                println!("Next node has no back-reference");
            }
        }
    }
    
    // Clear the list - this should not cause infinite loops or memory leaks
    list.clear();
    println!("✓ List cleared successfully (no cycles to break)");
    println!("Final list length: {}", list.len());
}

// Helper function to demonstrate custom scenarios
#[allow(dead_code)]
fn demonstrate_advanced_sharing() {
    println!("🔗 Advanced Sharing Scenarios");
    println!("-----------------------------");
    
    // Create a list
    let mut list1 = RcLinkedList::new();
    list1.push_front("Shared Node");
    
    // In a real scenario, you might want to share nodes between structures
    // This demonstrates the flexibility of Rc<RefCell<T>>
    if let Some(shared_node) = list1.get_head_ref() {
        println!("Node can be shared with other data structures");
        println!("Reference count: {}", Rc::strong_count(&shared_node));
        
        // You could theoretically add this node to another structure
        // (though this specific implementation doesn't support it directly)
        let _another_ref = shared_node.clone();
        println!("After sharing: {}", Rc::strong_count(&shared_node));
    }
}