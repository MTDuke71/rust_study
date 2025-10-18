// Integration test - tests the public API from an external perspective
// This file is compiled as a separate crate

use test_organization::{add_two, multiply, divide, Calculator, Stack};

#[test]
fn test_basic_functions() {
    assert_eq!(add_two(2), 4);
    assert_eq!(multiply(3, 4), 12);
    assert!(divide(10.0, 2.0).is_ok());
}

#[test]
fn test_calculator_integration() {
    let mut calc = Calculator::new();
    calc.add(10.0).multiply(2.0).add(5.0);
    assert_eq!(calc.get_value(), 25.0);
}

#[test]
fn test_stack_integration() {
    let mut stack = Stack::new();
    
    // Test basic operations
    stack.push("first");
    stack.push("second");
    stack.push("third");
    
    assert_eq!(stack.len(), 3);
    assert_eq!(stack.pop(), Some("third"));
    assert_eq!(stack.peek(), Some(&"second"));
    assert_eq!(stack.len(), 2);
}

#[test]
fn test_combined_functionality() {
    // Test using multiple components together
    let mut calc = Calculator::new();
    let mut stack = Stack::new();
    
    // Calculate some values and store in stack
    calc.add(5.0);
    stack.push(calc.get_value());
    
    calc.reset().add(10.0).multiply(2.0);
    stack.push(calc.get_value());
    
    calc.reset().add(3.0);
    stack.push(calc.get_value());
    
    // Verify the stack contains our calculated values
    assert_eq!(stack.pop(), Some(3.0));
    assert_eq!(stack.pop(), Some(20.0));
    assert_eq!(stack.pop(), Some(5.0));
    assert!(stack.is_empty());
}

#[test]
fn test_error_handling_integration() {
    // Test error conditions from an external perspective
    let result = divide(10.0, 0.0);
    assert!(result.is_err());
    
    if let Err(msg) = result {
        assert_eq!(msg, "Division by zero");
    }
}

#[test]
fn test_realistic_calculator_usage() {
    // Test a realistic scenario
    let mut calc = Calculator::new();
    
    // Calculate: ((10 + 5) * 3) + 7 = 52
    calc.add(10.0)
        .add(5.0)
        .multiply(3.0)
        .add(7.0);
    
    assert_eq!(calc.get_value(), 52.0);
    
    // Reset and try another calculation
    calc.reset();
    assert_eq!(calc.get_value(), 0.0);
    
    // Calculate: (20 * 2) + 10 = 50
    calc.add(20.0)
        .multiply(2.0)
        .add(10.0);
    
    assert_eq!(calc.get_value(), 50.0);
}

#[test]
fn test_stack_with_different_types() {
    // Test stack with different types (as external code would)
    let mut int_stack = Stack::new();
    let mut string_stack = Stack::new();
    
    // Integer stack
    for i in 1..=5 {
        int_stack.push(i);
    }
    assert_eq!(int_stack.len(), 5);
    
    // String stack
    string_stack.push("apple".to_string());
    string_stack.push("banana".to_string());
    string_stack.push("cherry".to_string());
    
    assert_eq!(string_stack.pop(), Some("cherry".to_string()));
    assert_eq!(string_stack.peek(), Some(&"banana".to_string()));
}

// Note: We cannot test private functions like internal_add_one here
// because integration tests only have access to the public API