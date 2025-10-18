// Another integration test file demonstrating test organization
// Each file in tests/ is compiled as a separate crate

use test_organization::{Calculator, Stack};

mod common;

#[test]
fn test_calculator_advanced_operations() {
    let mut calc = Calculator::new();
    
    // Test complex chaining
    calc.add(100.0)
        .multiply(0.5)
        .add(25.0)
        .multiply(2.0);
    
    // Should be: ((100 * 0.5) + 25) * 2 = (50 + 25) * 2 = 150
    assert_eq!(calc.get_value(), 150.0);
}

#[test]
fn test_stack_performance() {
    let mut stack = Stack::new();
    
    // Test with a reasonable number of items
    let test_size = 1000;
    
    // Push items
    for i in 0..test_size {
        stack.push(i);
    }
    
    assert_eq!(stack.len(), test_size);
    assert!(!stack.is_empty());
    
    // Pop half the items
    for _ in 0..test_size/2 {
        assert!(stack.pop().is_some());
    }
    
    assert_eq!(stack.len(), test_size/2);
    
    // Pop remaining items
    for _ in 0..test_size/2 {
        assert!(stack.pop().is_some());
    }
    
    assert!(stack.is_empty());
}

#[test]
fn test_interleaved_operations() {
    let mut calc = Calculator::new();
    let mut stack = Stack::new();
    
    // Interleave calculator and stack operations
    calc.add(10.0);
    stack.push(calc.get_value());
    
    calc.multiply(2.0);
    stack.push(calc.get_value());
    
    calc.add(5.0);
    stack.push(calc.get_value());
    
    // Verify stack contents
    let last_value = stack.pop().unwrap();
    assert_eq!(last_value, 25.0); // (10 * 2) + 5
    
    let middle_value = stack.pop().unwrap();
    assert_eq!(middle_value, 20.0); // 10 * 2
    
    let first_value = stack.pop().unwrap();
    assert_eq!(first_value, 10.0); // Initial value
    
    assert!(stack.is_empty());
}

#[test]
fn test_error_recovery() {
    use test_organization::divide;
    
    // Test that we can continue after errors
    let result1 = divide(10.0, 0.0);
    assert!(result1.is_err());
    
    // Subsequent operations should still work
    let result2 = divide(10.0, 2.0);
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), 5.0);
    
    // Test with calculator
    let mut calc = Calculator::new();
    calc.add(15.0);
    
    // Even after error operations elsewhere, calculator should work
    assert_eq!(calc.get_value(), 15.0);
}

#[test]
fn test_boundary_conditions() {
    let mut calc = Calculator::new();
    
    // Test with very small numbers
    calc.add(0.000001);
    assert!(calc.get_value() > 0.0);
    
    calc.reset();
    
    // Test with large numbers
    calc.add(1_000_000.0);
    calc.multiply(1000.0);
    assert_eq!(calc.get_value(), 1_000_000_000.0);
    
    // Test stack with edge cases
    let mut stack: Stack<i32> = Stack::new();
    assert_eq!(stack.pop(), None); // Pop from empty stack
    
    stack.push(i32::MAX);
    assert_eq!(stack.pop(), Some(i32::MAX));
    
    stack.push(i32::MIN);
    assert_eq!(stack.pop(), Some(i32::MIN));
}