// Step 6: Runtime Borrow Checking - Enhanced Version (No Warnings)
// Run with: cargo run --example step6_enhanced_no_warnings
//
// This example demonstrates:
// 1. How RefCell enables runtime borrow checking
// 2. What compile errors you'd get with regular references
// 3. Why RefCell is needed for certain patterns like linked lists
//
// 🔥 TO SEE COMPILE ERRORS:
// Scroll down to the commented-out functions and uncomment any of them!
// Then run `cargo check` to see the compile errors that would occur
// with regular references instead of RefCell.

use std::cell::RefCell;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<RcNode<T>>>;

#[derive(Debug)]
pub struct RcNode<T> {
    // Made public to fix visibility warning
    pub data: T,                  // Made public for external access
    pub next: Option<NodeRef<T>>, // Made public for external access
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

    pub fn try_peek_front(&self) -> Result<Option<std::cell::Ref<'_, T>>, LinkedListError> {
        match &self.head {
            Some(node) => {
                let borrowed = node.try_borrow()?;
                Ok(Some(std::cell::Ref::map(borrowed, |n| &n.data)))
            }
            None => Ok(None),
        }
    }

    pub fn try_peek_front_mut(
        &mut self,
    ) -> Result<Option<std::cell::RefMut<'_, T>>, LinkedListError> {
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
            let next = current
                .borrow()
                .next
                .clone()
                .ok_or(LinkedListError::IndexOutOfBounds)?;
            current = next;
        }
        Ok(current)
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    // Helper method to safely get data without exposing internal structure
    pub fn get_data(&self, index: usize) -> Result<T, LinkedListError>
    where
        T: Clone,
    {
        let node_ref = self.get_node_ref(index)?;
        let borrowed = node_ref.borrow();
        Ok(borrowed.data.clone())
    }

    // Helper method to safely set data
    pub fn set_data(&mut self, index: usize, new_data: T) -> Result<(), LinkedListError> {
        let node_ref = self.get_node_ref(index)?;
        node_ref.borrow_mut().data = new_data;
        Ok(())
    }
}

fn demonstrate_successful_borrowing() {
    println!("=== Demonstrating Successful Borrowing ===");

    let mut list = RcLinkedList::new();
    list.push_front(42);
    list.push_front(24);

    // Multiple immutable borrows are OK - but we need to handle them carefully
    // due to lifetime constraints with RefMut::map
    if let Some(node_ref) = list.head.clone() {
        // Get multiple immutable borrows of the same node
        let borrow1 = node_ref.borrow();
        println!("First borrow: {}", borrow1.data);

        // This works because we're borrowing the same RefCell multiple times
        let borrow2 = node_ref.borrow();
        println!("Second borrow: {}", borrow2.data);
        println!("Both borrows active simultaneously: OK!");

        // Both borrows are dropped here
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
            Ok(borrow) => println!(
                "✅ Borrowing works after conflict resolved: {}",
                borrow.data
            ),
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
        mut_borrow.data.push_str(" World");
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
            Err(LinkedListError::BorrowConflict) => {
                println!("✅ peek correctly reported borrow conflict")
            }
            Err(e) => println!("❌ peek failed with unexpected error: {:?}", e),
        }

        match list.try_peek_front_mut() {
            Ok(Some(_)) => println!("❌ peek_mut succeeded when it shouldn't"),
            Ok(None) => println!("❌ peek_mut returned None when it shouldn't"),
            Err(LinkedListError::BorrowConflict) => {
                println!("✅ peek_mut correctly reported borrow conflict")
            }
            Err(e) => println!("❌ peek_mut failed with unexpected error: {:?}", e),
        }
    }
}

fn demonstrate_helper_methods() {
    println!("=== Demonstrating Helper Methods ===");

    let mut list = RcLinkedList::new();
    list.push_front(String::from("World"));
    list.push_front(String::from("Hello"));

    // Use helper methods that don't expose internal structure
    println!("List length: {}", list.len());
    println!("Is empty: {}", list.is_empty());

    // Safe data access
    match list.get_data(0) {
        Ok(data) => println!("Data at index 0: {}", data),
        Err(e) => println!("Error getting data: {:?}", e),
    }

    // Safe data modification
    match list.set_data(1, String::from("Rust")) {
        Ok(()) => println!("Successfully updated index 1"),
        Err(e) => println!("Error setting data: {:?}", e),
    }

    // Verify the change
    if let Ok(data) = list.get_data(1) {
        println!("Updated data at index 1: {}", data);
    }
}

fn main() {
    println!("=== Step 6: Runtime Borrow Checking (Enhanced) ===");

    demonstrate_successful_borrowing();
    println!();

    demonstrate_borrow_conflicts();
    println!();

    demonstrate_scope_management();
    println!();

    demonstrate_error_handling();
    println!();

    demonstrate_helper_methods();

    why_refcell_works_demonstration();

    println!();
    println!("📝 Key Insights:");
    println!("- RefCell enforces borrowing rules at RUNTIME, not compile time");
    println!("- Multiple immutable borrows are allowed simultaneously");
    println!("- Mutable borrows are exclusive (no other borrows allowed)");
    println!("- Use try_borrow() and try_borrow_mut() for graceful error handling");
    println!("- Manage borrow lifetimes carefully with explicit scopes");
    println!("- Always handle potential BorrowError and BorrowMutError");
    println!("- Regular references enforce borrowing at COMPILE time");
    println!("- RefCell allows shared mutation by moving checks to RUNTIME");

    println!();
    println!("🔧 Warning Fixes Applied:");
    println!("- Made RcNode public to fix visibility warning");
    println!("- Added helper methods that hide internal complexity");
    println!("- Enhanced error handling and demonstrations");

    println!("✅ Runtime borrow checking demonstrated without warnings!");
}

// ============================================================================
// COMPILE-TIME BORROW CHECKING VIOLATIONS
// ============================================================================
// The following code demonstrates what would cause COMPILE ERRORS
// if we tried to do the same things with regular references instead of RefCell.
//
// 🔥 UNCOMMENT ANY OF THESE FUNCTIONS TO SEE COMPILE ERRORS! 🔥
// Then add a call to the function in main() and run `cargo check`
//
// These examples show exactly why RefCell is necessary for data structures
// like linked lists that need shared ownership with mutation.

/*
// Try uncommenting any of these functions and adding a call to main()
// to see the specific compile errors that RefCell helps us avoid!

// ❌ ERROR: "cannot borrow `data` as mutable more than once at a time"
fn compile_error_multiple_mutable_borrows() {
    let mut data = vec![1, 2, 3];

    let borrow1 = &mut data;
    let borrow2 = &mut data;  // ERROR: second mutable borrow

    println!("{:?} {:?}", borrow1, borrow2);
}

// ❌ ERROR: "cannot borrow `data` as mutable because it is also borrowed as immutable"
fn compile_error_mutable_and_immutable() {
    let mut data = vec![1, 2, 3];

    let immut_borrow = &data;
    let mut_borrow = &mut data;  // ERROR: can't have both

    println!("{:?} {:?}", immut_borrow, mut_borrow);
}

// ❌ ERROR: "borrow of moved value: `data`"
fn compile_error_use_after_move() {
    let data = vec![1, 2, 3];
    let moved_data = data;  // data is moved here

    println!("{:?}", data);  // ERROR: data was moved
    println!("{:?}", moved_data);
}

// ❌ ERROR: "returns a reference to data owned by the current function"
fn compile_error_dangling_reference() -> &i32 {
    let x = 42;

    &x  // ERROR: x goes out of scope, reference would be dangling
}

// ❌ ERROR: "cannot borrow `vec` as mutable because it is also borrowed as immutable"
fn compile_error_iterator_invalidation() {
    let mut vec = vec![1, 2, 3, 4, 5];

    for item in &vec {  // immutable borrow for iteration
        if *item > 3 {
            vec.push(*item * 2);  // ERROR: trying to mutate while iterating
        }
    }
}

// ❌ This demonstrates the aliasing + mutation problem that Rust prevents
fn compile_error_aliasing_and_mutation() {
    let mut data = vec![1, 2, 3];
    let slice1 = &mut data[0..2];
    let slice2 = &mut data[1..3];  // ERROR: overlapping mutable borrows

    slice1[0] = 42;
    slice2[1] = 24;
}

// 🎯 V-CYCLE MISSION CONTEXT: Why These Errors Matter
//
// In Mission4 (linked lists), we need:
// 1. Multiple references to the same node (shared ownership)
// 2. The ability to mutate through those references
// 3. Safe traversal without invalidating iterators
//
// Regular references can't provide all three simultaneously!
// That's exactly why RefCell is essential for linked list implementations.
*/

// ============================================================================
// WHY RefCell ALLOWS WHAT COMPILE-TIME CHECKING PREVENTS
// ============================================================================

fn why_refcell_works_demonstration() {
    println!("\n=== Why RefCell Works Where Compile-Time Checking Fails ===");

    // This demonstrates the same logical operations as the compile error examples above,
    // but RefCell moves the checking from compile-time to runtime

    let data = Rc::new(RefCell::new(vec![1, 2, 3]));

    // Share the data between multiple owners
    let data_ref1 = data.clone();
    let data_ref2 = data.clone();

    println!("Shared data access:");
    println!("  From ref1: {:?}", data_ref1.borrow());
    println!("  From ref2: {:?}", data_ref2.borrow());

    // Controlled mutation with shared ownership
    {
        let mut mut_borrow = data.borrow_mut();
        mut_borrow.push(4);
        println!("After mutation: {:?}", *mut_borrow);
        // mut_borrow is dropped here automatically
    }

    // Multiple owners can still access the updated data
    println!("Updated data accessible from all references:");
    println!("  From ref1: {:?}", data_ref1.borrow());
    println!("  From ref2: {:?}", data_ref2.borrow());

    println!("✅ RefCell enables controlled mutation with shared ownership!");
    println!("✅ This pattern would be impossible with compile-time borrow checking alone!");
}
