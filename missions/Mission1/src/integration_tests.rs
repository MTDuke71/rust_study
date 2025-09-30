#[cfg(test)]
mod integration_tests {
    use crate::Stack;

    #[test]
    fn test_lifo_behavior_comprehensive() {
        let mut stack = Stack::new();
        
        // Test empty stack
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
        
        // Push several items
        let items = vec!["first", "second", "third", "fourth"];
        for item in &items {
            stack.push(item.to_string());
        }
        
        assert_eq!(stack.len(), 4);
        assert!(!stack.is_empty());
        
        // Peek should show last item pushed (LIFO)
        assert_eq!(stack.peek(), Some(&"fourth".to_string()));
        assert_eq!(stack.len(), 4); // peek doesn't change length
        
        // Pop items - should come out in reverse order (LIFO)
        let expected_pop_order = vec!["fourth", "third", "second", "first"];
        for expected in expected_pop_order {
            let popped = stack.pop().unwrap();
            assert_eq!(popped, expected);
        }
        
        // Stack should be empty again
        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);
    }
    
    #[test]
    fn test_real_world_scenario_undo_system() {
        // Simulate an undo system for a text editor
        let mut undo_stack: Stack<String> = Stack::new();
        
        // User types some text
        let mut document = String::new();
        
        // Each edit saves the previous state
        undo_stack.push(document.clone()); // Save empty state
        document.push_str("Hello");
        
        undo_stack.push(document.clone()); // Save "Hello"
        document.push_str(" World");
        
        undo_stack.push(document.clone()); // Save "Hello World"
        document.push_str("!");
        
        // Document now contains "Hello World!"
        assert_eq!(document, "Hello World!");
        
        // User hits undo - restore previous state
        if let Some(previous_state) = undo_stack.pop() {
            document = previous_state;
        }
        assert_eq!(document, "Hello World");
        
        // User hits undo again
        if let Some(previous_state) = undo_stack.pop() {
            document = previous_state;
        }
        assert_eq!(document, "Hello");
        
        // One more undo - back to empty
        if let Some(previous_state) = undo_stack.pop() {
            document = previous_state;
        }
        assert_eq!(document, "");
        
        // No more undos available
        assert_eq!(undo_stack.pop(), None);
    }
}