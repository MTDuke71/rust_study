//! # Problem 6: Sum Square Difference
//!
//! Find the difference between the sum of the squares and the square of the sum
//! of the first 100 natural numbers.
//!
//! ## Mathematical Foundation
//!
//! Uses **arithmetic series** formulas:
//! - Sum of first n numbers: n(n+1)/2
//! - Sum of squares of first n numbers: n(n+1)(2n+1)/6
//!
//! See `zettelkasten/math-foundations/arithmetic-series.md` for theory.
//!
//! ## Approach
//!
//! Instead of iterating and computing, we use closed-form formulas:
//! 1. Square of sum: [n(n+1)/2]²
//! 2. Sum of squares: n(n+1)(2n+1)/6
//! 3. Difference: square_of_sum - sum_of_squares
//!
//! ## Complexity
//!
//! - Time: O(1) - constant time using mathematical formulas
//! - Space: O(1)

/// Sum of first n natural numbers: 1 + 2 + ... + n = n(n+1)/2
fn sum_of_naturals(n: u64) -> u64 {
    n * (n + 1) / 2
}

/// Sum of squares: 1² + 2² + ... + n² = n(n+1)(2n+1)/6
fn sum_of_squares(n: u64) -> u64 {
    n * (n + 1) * (2 * n + 1) / 6
}

/// Calculate difference between square of sum and sum of squares
pub fn sum_square_difference(n: u64) -> u64 {
    let sum = sum_of_naturals(n);
    let square_of_sum = sum * sum;
    let sum_of_sq = sum_of_squares(n);

    square_of_sum - sum_of_sq
}

/// Solve Problem 6 for the first 100 natural numbers
pub fn solve() -> u64 {
    sum_square_difference(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_naturals() {
        assert_eq!(sum_of_naturals(10), 55);
        assert_eq!(sum_of_naturals(5), 15);
        assert_eq!(sum_of_naturals(1), 1);
    }

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(10), 385);
        assert_eq!(sum_of_squares(5), 55);
        assert_eq!(sum_of_squares(1), 1);
    }

    #[test]
    fn test_example_10() {
        // Example: n=10
        // Sum: 1+2+...+10 = 55, square = 3025
        // Sum of squares: 1+4+9+...+100 = 385
        // Difference: 3025 - 385 = 2640
        assert_eq!(sum_square_difference(10), 2640);
    }

    #[test]
    fn test_small_cases() {
        assert_eq!(sum_square_difference(1), 0); // 1² - 1² = 0
        assert_eq!(sum_square_difference(2), 4); // 9 - 5 = 4
        assert_eq!(sum_square_difference(3), 22); // 36 - 14 = 22
    }

    #[test]
    fn test_solve() {
        let result = solve();
        // Verify it's positive and reasonable
        assert!(result > 0);
        // The answer is 25164150
        assert_eq!(result, 25164150);
    }
}
