//! # Chapter 7.1: Library Crate Example
//!
//! This demonstrates a **library crate** - it provides functionality to other programs
//! but cannot be executed directly (no main() function).
//!
//! ## Key Characteristics of Library Crates:
//! - No `main()` function
//! - Provides functions, structs, enums, traits, etc.
//! - Can be used as a dependency by other crates
//! - Entry point is `src/lib.rs`
//!
//! ## Usage:
//! ```rust
//! use crates::greet;
//! use crates::Calculator;
//!
//! greet("World");
//! let mut calc = Calculator::new();
//! calc.add(42);
//! ```

/// A simple greeting function
pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}

/// Calculate the sum of two integers
pub fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

/// A simple calculator struct
pub struct Calculator {
    value: i32,
}

impl Calculator {
    /// Create a new calculator instance
    pub fn new() -> Self {
        Calculator { value: 0 }
    }

    /// Add a number to the current value
    pub fn add(&mut self, n: i32) {
        self.value += n;
    }

    /// Get the current value
    pub fn get_value(&self) -> i32 {
        self.value
    }

    /// Reset the calculator to zero
    pub fn reset(&mut self) {
        self.value = 0;
    }
}

/// A utility module for mathematical operations
pub mod math {
    /// Calculate the factorial of a number
    pub fn factorial(n: u32) -> u32 {
        match n {
            0 | 1 => 1,
            _ => n * factorial(n - 1),
        }
    }

    /// Check if a number is prime
    pub fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }

        for i in 2..=(n as f64).sqrt() as u32 {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator() {
        let mut calc = Calculator::new();
        assert_eq!(calc.get_value(), 0);

        calc.add(5);
        assert_eq!(calc.get_value(), 5);

        calc.add(3);
        assert_eq!(calc.get_value(), 8);

        calc.reset();
        assert_eq!(calc.get_value(), 0);
    }

    #[test]
    fn test_math_factorial() {
        assert_eq!(math::factorial(0), 1);
        assert_eq!(math::factorial(1), 1);
        assert_eq!(math::factorial(5), 120);
    }

    #[test]
    fn test_math_is_prime() {
        assert!(!math::is_prime(0));
        assert!(!math::is_prime(1));
        assert!(math::is_prime(2));
        assert!(math::is_prime(3));
        assert!(!math::is_prime(4));
        assert!(math::is_prime(5));
        assert!(math::is_prime(17));
    }
}
