// This file contains examples that SHOULD NOT compile
// These demonstrate Rust's compile-time memory safety guarantees
// To test these, uncomment one example at a time and run `cargo check`

use crate::Stack;

#[allow(dead_code)]
fn ownership_violations() {
    let mut s: Stack<String> = Stack::new();

    // EXAMPLE 1: Use after move (REQ-4 violation)
    let val = String::from("test");
    s.push(val);
    // println!("{}", val); // ERROR: val was moved into stack

    // EXAMPLE 2: Using popped value from stack perspective
    // s.push(String::from("hello"));
    // let popped = s.pop().unwrap();
    // // Pretend we had a reference to the internal Vec and tried to access it
    // // This is impossible in safe Rust - no way to get references into Vec after pop

    // EXAMPLE 3: Borrowing violations (REQ-5)
    // s.push(String::from("test"));
    // let imm_ref = s.peek().unwrap();
    // s.push(String::from("another")); // ERROR: cannot borrow as mutable while immutable borrow exists
    // println!("{}", imm_ref);

    // EXAMPLE 2: Using popped value from stack perspective
    // s.push(String::from("hello"));
    // let popped = s.pop().unwrap();
    // // Pretend we had a reference to the internal Vec and tried to access it
    // // This is impossible in safe Rust - no way to get references into Vec after pop

    // EXAMPLE 3: Borrowing violations (REQ-5)
    // s.push(String::from("test"));
    // let imm_ref = s.peek().unwrap();
    // s.push(String::from("another")); // ERROR: cannot borrow as mutable while immutable borrow exists
    // println!("{}", imm_ref);

    // EXAMPLE 4: Multiple mutable borrows
    // s.push(String::from("test"));
    // let mut_ref1 = s.peek_mut().unwrap();
    // let mut_ref2 = s.peek_mut().unwrap(); // ERROR: cannot have two mutable borrows
    // *mut_ref1 = String::from("new1");
    // *mut_ref2 = String::from("new2");
}

// Let's also create a working example that shows CORRECT usage
#[allow(dead_code)]
fn correct_ownership_patterns() {
    let mut s = Stack::new();

    // CORRECT: Use value before moving it
    let val = String::from("test");
    println!("About to move: {}", val);
    s.push(val); // val is moved here

    // CORRECT: Get ownership back via pop
    if let Some(popped) = s.pop() {
        println!("Popped: {}", popped); // We own popped, can use it
    }

    // CORRECT: Borrowing patterns
    s.push(String::from("hello"));

    // Multiple immutable borrows are fine
    {
        let peek1 = s.peek().unwrap();
        let peek2 = s.peek().unwrap();
        println!("Both see: {} and {}", peek1, peek2);
    } // borrows end here

    // Now we can mutably borrow
    {
        let mut_ref = s.peek_mut().unwrap();
        *mut_ref = String::from("modified");
    } // mutable borrow ends here

    // And immutable borrow again
    println!("Modified to: {}", s.peek().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_violations_compiles() {
        // This test ensures the ownership_violations function exists and compiles
        // We can't actually call it because it has intentionally broken examples
        // But we can verify the function signature is valid
        let _fn_exists = ownership_violations as fn();
    }

    #[test]
    fn test_correct_ownership_patterns() {
        // Test that correct_ownership_patterns runs without panicking
        correct_ownership_patterns();
    }

    #[test]
    fn test_ownership_move_and_pop() {
        let mut s = Stack::new();
        let val = String::from("test");
        s.push(val);

        let popped = s.pop().unwrap();
        assert_eq!(popped, "test");
        assert!(s.is_empty());
    }

    #[test]
    fn test_multiple_immutable_borrows() {
        let mut s = Stack::new();
        s.push(String::from("hello"));

        let peek1 = s.peek().unwrap();
        let peek2 = s.peek().unwrap();
        assert_eq!(peek1, "hello");
        assert_eq!(peek2, "hello");
        assert_eq!(peek1, peek2);
    }

    #[test]
    fn test_mutable_borrow_modification() {
        let mut s = Stack::new();
        s.push(String::from("original"));

        {
            let mut_ref = s.peek_mut().unwrap();
            *mut_ref = String::from("modified");
        }

        assert_eq!(s.peek().unwrap(), "modified");
    }

    #[test]
    fn test_ownership_sequence() {
        let mut s = Stack::new();

        // Push and verify
        s.push(42);
        assert_eq!(s.len(), 1);
        assert_eq!(*s.peek().unwrap(), 42);

        // Modify via mutable reference
        *s.peek_mut().unwrap() = 99;
        assert_eq!(*s.peek().unwrap(), 99);

        // Pop and verify ownership transfer
        let owned_value = s.pop().unwrap();
        assert_eq!(owned_value, 99);
        assert!(s.is_empty());
    }
}
