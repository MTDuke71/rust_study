// Common test utilities shared between integration tests
// This module provides helper functions for integration tests

use test_organization::{Calculator, Stack};

pub fn setup_test_calculator() -> Calculator {
    let mut calc = Calculator::new();
    calc.add(10.0); // Start with a base value
    calc
}

pub fn setup_test_stack() -> Stack<i32> {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack
}

pub fn create_large_stack(size: usize) -> Stack<usize> {
    let mut stack = Stack::new();
    for i in 0..size {
        stack.push(i);
    }
    stack
}

pub fn calculate_factorial(n: u32) -> f64 {
    let mut calc = Calculator::new();
    calc.add(1.0); // Start with 1
    
    for i in 2..=n {
        calc.multiply(i as f64);
    }
    
    calc.get_value()
}

pub fn verify_stack_order(stack: &mut Stack<i32>, expected: Vec<i32>) -> bool {
    for expected_value in expected.into_iter().rev() {
        match stack.pop() {
            Some(actual) => {
                if actual != expected_value {
                    return false;
                }
            }
            None => return false,
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod common_tests {
    use super::*;

    #[test]
    fn test_setup_functions() {
        let calc = setup_test_calculator();
        assert_eq!(calc.get_value(), 10.0);
        
        let mut stack = setup_test_stack();
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(3));
    }
    
    #[test]
    fn test_factorial_calculation() {
        assert_eq!(calculate_factorial(5), 120.0);
        assert_eq!(calculate_factorial(0), 1.0);
        assert_eq!(calculate_factorial(1), 1.0);
    }
    
    #[test]
    fn test_verify_stack_order() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert!(verify_stack_order(&mut stack, vec![1, 2, 3]));
    }
}