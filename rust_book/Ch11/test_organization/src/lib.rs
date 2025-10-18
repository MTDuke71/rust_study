//! # Chapter 11.3: Test Organization - Library
//!
//! This library demonstrates test organization patterns including
//! unit tests, integration tests, and testing private functions.

// Public API - what external users can access
pub fn add_two(a: i32) -> i32 {
    internal_add_one(a) + 1
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Public struct for testing
#[derive(Debug, PartialEq)]
pub struct Calculator {
    value: f64,
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { value: 0.0 }
    }
    
    pub fn add(&mut self, n: f64) -> &mut Self {
        if self.validate_input(n) {
            self.value += n;
        }
        self
    }
    
    pub fn multiply(&mut self, n: f64) -> &mut Self {
        if self.validate_input(n) {
            self.value *= n;
        }
        self
    }
    
    pub fn get_value(&self) -> f64 {
        self.value
    }
    
    pub fn reset(&mut self) -> &mut Self {
        self.value = 0.0;
        self
    }
    
    // Private method
    fn validate_input(&self, input: f64) -> bool {
        !input.is_nan() && !input.is_infinite()
    }
}

// Stack implementation for testing
#[derive(Debug)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

// Private function - can be tested from unit tests
fn internal_add_one(a: i32) -> i32 {
    a + 1
}

// Private helper function used in tests
#[allow(dead_code)]
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// Unit tests module
#[cfg(test)]
mod tests {
    use super::*;

    // Basic unit tests
    #[test]
    fn test_add_two() {
        assert_eq!(add_two(3), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
    }

    #[test]
    fn test_divide_success() {
        let result = divide(10.0, 2.0).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());
    }

    // Testing private functions
    #[test]
    fn test_internal_add_one() {
        assert_eq!(internal_add_one(5), 6);
        assert_eq!(internal_add_one(-1), 0);
        assert_eq!(internal_add_one(0), 1);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(-4));
        assert!(!is_even(1));
        assert!(!is_even(-3));
    }

    // Calculator tests organized in submodule
    mod calculator_tests {
        use super::*;

        #[test]
        fn test_new_calculator() {
            let calc = Calculator::new();
            assert_eq!(calc.get_value(), 0.0);
        }

        #[test]
        fn test_calculator_add() {
            let mut calc = Calculator::new();
            calc.add(5.0);
            assert_eq!(calc.get_value(), 5.0);
        }

        #[test]
        fn test_calculator_chaining() {
            let mut calc = Calculator::new();
            calc.add(10.0).multiply(2.0).add(5.0);
            assert_eq!(calc.get_value(), 25.0);
        }

        #[test]
        fn test_calculator_reset() {
            let mut calc = Calculator::new();
            calc.add(100.0);
            calc.reset();
            assert_eq!(calc.get_value(), 0.0);
        }

        #[test]
        fn test_calculator_private_method() {
            let calc = Calculator::new();
            // We can test private methods from unit tests
            assert!(calc.validate_input(5.0));
            assert!(!calc.validate_input(f64::NAN));
            assert!(!calc.validate_input(f64::INFINITY));
        }
    }

    // Stack tests organized in submodule
    mod stack_tests {
        use super::*;

        #[test]
        fn test_new_stack() {
            let stack: Stack<i32> = Stack::new();
            assert!(stack.is_empty());
            assert_eq!(stack.len(), 0);
        }

        #[test]
        fn test_stack_push() {
            let mut stack = Stack::new();
            stack.push(1);
            stack.push(2);
            assert_eq!(stack.len(), 2);
            assert!(!stack.is_empty());
        }

        #[test]
        fn test_stack_pop() {
            let mut stack = Stack::new();
            stack.push(1);
            stack.push(2);
            
            assert_eq!(stack.pop(), Some(2));
            assert_eq!(stack.pop(), Some(1));
            assert_eq!(stack.pop(), None);
        }

        #[test]
        fn test_stack_peek() {
            let mut stack = Stack::new();
            assert_eq!(stack.peek(), None);
            
            stack.push(1);
            assert_eq!(stack.peek(), Some(&1));
            
            stack.push(2);
            assert_eq!(stack.peek(), Some(&2));
            
            // Peek doesn't remove the item
            assert_eq!(stack.len(), 2);
        }

        #[test]
        fn test_stack_with_strings() {
            let mut stack = Stack::new();
            stack.push("hello".to_string());
            stack.push("world".to_string());
            
            assert_eq!(stack.pop(), Some("world".to_string()));
            assert_eq!(stack.peek(), Some(&"hello".to_string()));
        }
    }

    // Performance tests submodule
    mod performance_tests {
        use super::*;

        #[test]
        fn test_large_stack_operations() {
            let mut stack = Stack::new();
            
            // Push many items
            for i in 0..1000 {
                stack.push(i);
            }
            
            assert_eq!(stack.len(), 1000);
            
            // Pop all items
            for i in (0..1000).rev() {
                assert_eq!(stack.pop(), Some(i));
            }
            
            assert!(stack.is_empty());
        }

        #[test]
        fn test_calculator_many_operations() {
            let mut calc = Calculator::new();
            
            for i in 1..=100 {
                calc.add(i as f64);
            }
            
            // Sum of 1 to 100 is 5050
            assert_eq!(calc.get_value(), 5050.0);
        }
    }

    // Error handling tests
    mod error_tests {
        use super::*;

        #[test]
        fn test_divide_edge_cases() {
            // Test positive division
            assert!(divide(1.0, 1.0).is_ok());
            
            // Test negative numbers
            assert!(divide(-10.0, 2.0).is_ok());
            assert_eq!(divide(-10.0, 2.0).unwrap(), -5.0);
            
            // Test division by zero
            assert!(divide(5.0, 0.0).is_err());
            assert!(divide(-5.0, 0.0).is_err());
            assert!(divide(0.0, 0.0).is_err());
        }

        #[test]
        fn test_error_messages() {
            let result = divide(10.0, 0.0);
            match result {
                Err(msg) => assert_eq!(msg, "Division by zero"),
                Ok(_) => panic!("Expected error but got success"),
            }
        }
    }

    // Integration-style tests within unit test module
    mod integration_style_tests {
        use super::*;

        #[test]
        fn test_calculator_and_functions_together() {
            let mut calc = Calculator::new();
            
            // Use both Calculator and free functions
            let value = add_two(3); // = 5
            calc.add(value as f64);
            
            let multiplied = multiply(2, 3); // = 6
            calc.multiply(multiplied as f64);
            
            // calc should now have (0 + 5) * 6 = 30
            assert_eq!(calc.get_value(), 30.0);
        }

        #[test]
        fn test_stack_with_calculated_values() {
            let mut stack = Stack::new();
            let mut calc = Calculator::new();
            
            // Calculate some values and put them in stack
            for i in 1..=5 {
                calc.reset().add(i as f64).multiply(2.0);
                stack.push(calc.get_value() as i32);
            }
            
            // Stack should contain [2, 4, 6, 8, 10]
            assert_eq!(stack.pop(), Some(10));
            assert_eq!(stack.pop(), Some(8));
            assert_eq!(stack.pop(), Some(6));
            assert_eq!(stack.pop(), Some(4));
            assert_eq!(stack.pop(), Some(2));
        }
    }
}