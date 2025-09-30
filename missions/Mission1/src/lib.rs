pub mod stack;
mod experiments;
mod integration_tests;

pub use stack::Stack;

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // REQ-1: Generic support
    fn req1_generic_support() {
        // Each of these creates a different type of stack
        let mut string_stack = Stack::new();
        string_stack.push("hello");
        
        let mut int_stack = Stack::new();
        int_stack.push(42);
        
        let mut float_stack = Stack::new();
        float_stack.push(3.14);
        
        // All compile because Stack<T> works with any type T
        assert_eq!(string_stack.len(), 1);
        assert_eq!(int_stack.len(), 1);
        assert_eq!(float_stack.len(), 1);
    }

    #[test] // REQ-2: Push in amortized O(1)
    fn req2_push_amortized_constant() {
        let mut s = Stack::with_capacity(2);
        
        // Even though we start with capacity 2, we can push 100 items
        // Vec will automatically grow as needed
        for i in 0..100 {
            s.push(i);
        }
        
        assert_eq!(s.len(), 100);
        
        // Let's verify LIFO order while we're at it
        assert_eq!(s.peek(), Some(&99)); // Last pushed should be on top
    }

    #[test] // REQ-3: Pop transfers ownership
    fn req3_pop_transfers_ownership() {
        let mut s = Stack::new();
        
        // Create a String (owns its data)
        let val = String::from("owned");
        s.push(val);
        // At this point, 'val' is moved into the stack
        
        // Pop returns ownership of the String
        let popped = s.pop().unwrap();
        assert_eq!(popped, "owned");
        
        // Stack is now empty
        assert!(s.pop().is_none());
        assert!(s.is_empty());
    }

    #[test] // REQ-4: No use after pop
    fn req4_no_use_after_pop() {
        let mut s = Stack::new();
        
        let a = String::from("test");
        s.push(a);
        // If we tried to use 'a' here, it wouldn't compile because
        // ownership was moved into the stack
        
        let b = s.pop().unwrap();
        assert_eq!(b, "test");
        
        // 'b' now owns the string, and the stack no longer has access to it
        // This is REQ-4 in action - no aliases to popped elements remain
    }

    #[test] // REQ-5: Peek aliasing rules
    fn req5_peek_aliasing_rules() {
        let mut s = Stack::new();
        s.push(1);
        
        // Immutable borrowing - we can have multiple immutable references
        {
            let imm1 = s.peek().unwrap();
            let imm2 = s.peek().unwrap();
            assert_eq!(*imm1, 1);
            assert_eq!(*imm2, 1);
            
            // While immutable borrows exist, we CAN'T mutate
            // Uncommenting the next line would cause a compile error:
            // s.push(2); // ERROR: cannot borrow `s` as mutable
        } // immutable borrows end here
        
        // Now we can get a mutable reference
        {
            let mut_ref = s.peek_mut().unwrap();
            *mut_ref = 99; // Modify through mutable reference
            
            // While we have a mutable borrow, we CAN'T have immutable borrows
            // Uncommenting the next line would cause a compile error:
            // let imm = s.peek().unwrap(); // ERROR: cannot borrow `s` as immutable
        } // mutable borrow ends here
        
        // Now we can borrow again
        assert_eq!(*s.peek().unwrap(), 99);
    }
}