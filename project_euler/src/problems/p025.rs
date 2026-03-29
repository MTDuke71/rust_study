//! # Problem 25: 1000-Digit Fibonacci Number
//!
//! The Fibonacci sequence is defined by the recurrence relation:
//! F(1) = 1, F(2) = 1, F(n) = F(n−1) + F(n−2)
//!
//! The 12th term, F(12) = 144, is the first term to contain three digits.
//!
//! What is the index of the first term in the Fibonacci sequence to
//! contain 1000 digits?
//!
//! ## Mathematical Foundation
//!
//! Fibonacci numbers grow exponentially: F(n) ≈ φ^n / √5 where φ = (1+√5)/2.
//! A number has d digits when it ≥ 10^(d-1).
//!
//! Estimated index: n ≈ (d-1) × ln(10) / ln(φ) + ln(√5) / ln(φ)
//! For d=1000: n ≈ 999 × 2.078 + 1.672 ≈ 4782 (close to actual answer)
//!
//! Since F(4782) has ~1000 digits, we need big number arithmetic.
//! We reuse the digit-array approach from Problems 16 and 20.
//!
//! ## Complexity
//!
//! - Time: O(n × d) where n ≈ 4782 iterations, d ≈ 1000 digits per addition
//! - Space: O(d) for storing two Fibonacci numbers as digit arrays

/// Add two big numbers represented as digit arrays (least-significant first).
///
/// # Examples
/// ```
/// use project_euler::problems::p025::big_add;
/// // 99 + 1 = 100
/// assert_eq!(big_add(&[9, 9], &[1]), vec![0, 0, 1]);
/// // 144 + 89 = 233
/// assert_eq!(big_add(&[4, 4, 1], &[9, 8]), vec![3, 3, 2]);
/// ```
pub fn big_add(a: &[u8], b: &[u8]) -> Vec<u8> {
    let max_len = a.len().max(b.len());
    let mut result = Vec::with_capacity(max_len + 1);
    let mut carry = 0u8;

    for i in 0..max_len {
        let da = if i < a.len() { a[i] } else { 0 };
        let db = if i < b.len() { b[i] } else { 0 };
        let sum = da + db + carry;
        result.push(sum % 10);
        carry = sum / 10;
    }
    if carry > 0 {
        result.push(carry);
    }

    result
}

/// Find the index of the first Fibonacci number with at least `digits` digits.
///
/// Uses F(1) = 1, F(2) = 1, ... indexing (1-based).
///
/// # Examples
/// ```
/// use project_euler::problems::p025::first_fib_with_digits;
/// // F(7) = 13 is the first 2-digit Fibonacci number
/// assert_eq!(first_fib_with_digits(2), 7);
/// // F(12) = 144 is the first 3-digit Fibonacci number
/// assert_eq!(first_fib_with_digits(3), 12);
/// ```
pub fn first_fib_with_digits(digits: usize) -> u64 {
    let mut a: Vec<u8> = vec![1]; // F(1) = 1
    let mut b: Vec<u8> = vec![1]; // F(2) = 1
    let mut index = 2u64;

    while b.len() < digits {
        let next = big_add(&a, &b);
        a = b;
        b = next;
        index += 1;
    }

    index
}

/// Solve Problem 25: index of first Fibonacci number with 1000 digits.
pub fn solve() -> u64 {
    first_fib_with_digits(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_add_simple() {
        // 5 + 3 = 8
        assert_eq!(big_add(&[5], &[3]), vec![8]);
    }

    #[test]
    fn test_big_add_with_carry() {
        // 99 + 1 = 100
        assert_eq!(big_add(&[9, 9], &[1]), vec![0, 0, 1]);
    }

    #[test]
    fn test_big_add_different_lengths() {
        // 1000 + 99 = 1099
        assert_eq!(big_add(&[0, 0, 0, 1], &[9, 9]), vec![9, 9, 0, 1]);
    }

    #[test]
    fn test_first_fib_2_digits() {
        // F(7) = 13 is the first 2-digit Fibonacci number
        assert_eq!(first_fib_with_digits(2), 7);
    }

    #[test]
    fn test_first_fib_3_digits() {
        // F(12) = 144 is the first 3-digit Fibonacci number (from problem statement)
        assert_eq!(first_fib_with_digits(3), 12);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 4782);
    }
}
