//! # Publishing Example
//!
//! This crate demonstrates how to prepare a library for publishing to crates.io.
//!
//! ## Features
//!
//! - Comprehensive documentation
//! - Usage examples
//! - Proper error handling
//! - Semantic versioning
//!
//! ## Example
//!
//! ```rust
//! use publishing_example::Calculator;
//!
//! let calc = Calculator::new();
//! let result = calc.add(5, 3);
//! assert_eq!(result, 8);
//! ```

/// A simple calculator for demonstration purposes.
///
/// This struct provides basic arithmetic operations.
///
/// # Examples
///
/// ```
/// use publishing_example::Calculator;
///
/// let calc = Calculator::new();
/// assert_eq!(calc.add(2, 3), 5);
/// assert_eq!(calc.multiply(4, 5), 20);
/// ```
pub struct Calculator {
    // Private fields demonstrate encapsulation
}

impl Calculator {
    /// Creates a new `Calculator` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use publishing_example::Calculator;
    ///
    /// let calc = Calculator::new();
    /// ```
    pub fn new() -> Self {
        Calculator {}
    }

    /// Adds two numbers.
    ///
    /// # Arguments
    ///
    /// * `a` - The first number
    /// * `b` - The second number
    ///
    /// # Returns
    ///
    /// The sum of `a` and `b`.
    ///
    /// # Examples
    ///
    /// ```
    /// use publishing_example::Calculator;
    ///
    /// let calc = Calculator::new();
    /// assert_eq!(calc.add(5, 3), 8);
    /// ```
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    /// Multiplies two numbers.
    ///
    /// # Arguments
    ///
    /// * `a` - The first number
    /// * `b` - The second number
    ///
    /// # Returns
    ///
    /// The product of `a` and `b`.
    ///
    /// # Examples
    ///
    /// ```
    /// use publishing_example::Calculator;
    ///
    /// let calc = Calculator::new();
    /// assert_eq!(calc.multiply(4, 5), 20);
    /// ```
    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }

    /// Subtracts the second number from the first.
    ///
    /// # Arguments
    ///
    /// * `a` - The number to subtract from
    /// * `b` - The number to subtract
    ///
    /// # Returns
    ///
    /// The difference of `a` and `b`.
    ///
    /// # Examples
    ///
    /// ```
    /// use publishing_example::Calculator;
    ///
    /// let calc = Calculator::new();
    /// assert_eq!(calc.subtract(10, 3), 7);
    /// ```
    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }

    /// Divides the first number by the second.
    ///
    /// # Arguments
    ///
    /// * `a` - The dividend
    /// * `b` - The divisor
    ///
    /// # Returns
    ///
    /// `Some(result)` if division is successful, `None` if dividing by zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use publishing_example::Calculator;
    ///
    /// let calc = Calculator::new();
    /// assert_eq!(calc.divide(10, 2), Some(5));
    /// assert_eq!(calc.divide(10, 0), None);
    /// ```
    pub fn divide(&self, a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let calc = Calculator::new();
        assert_eq!(calc.add(2, 3), 5);
        assert_eq!(calc.add(-1, 1), 0);
    }

    #[test]
    fn test_multiply() {
        let calc = Calculator::new();
        assert_eq!(calc.multiply(4, 5), 20);
        assert_eq!(calc.multiply(0, 100), 0);
    }

    #[test]
    fn test_subtract() {
        let calc = Calculator::new();
        assert_eq!(calc.subtract(10, 3), 7);
        assert_eq!(calc.subtract(5, 5), 0);
    }

    #[test]
    fn test_divide() {
        let calc = Calculator::new();
        assert_eq!(calc.divide(10, 2), Some(5));
        assert_eq!(calc.divide(10, 0), None);
        assert_eq!(calc.divide(0, 5), Some(0));
    }
}

