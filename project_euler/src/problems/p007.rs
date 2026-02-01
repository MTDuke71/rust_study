//! # Problem 7: 10001st Prime
//!
//! Find the 10,001st prime number.
//!
//! ## Mathematical Foundation
//!
//! Uses the **Prime Number Theorem** for estimation:
//! - The nth prime is approximately n * ln(n) for large n
//! - This gives us an upper bound for the sieve
//!
//! See `zettelkasten/math-foundations/prime-number-theory.md` for theory.
//!
//! ## Approach
//!
//! 1. Estimate upper bound using Prime Number Theorem: p_n ≈ n * ln(n) * 1.3
//! 2. Generate primes up to that limit using Sieve of Eratosthenes
//! 3. Return the nth prime from the list
//!
//! ## Complexity
//!
//! - Time: O(n log log n) - Sieve of Eratosthenes
//! - Space: O(n) - boolean array for sieve

use crate::utils::primes::sieve;

/// Find the nth prime number (1-indexed)
///
/// # Examples
/// ```
/// use project_euler::problems::p007::nth_prime;
/// assert_eq!(nth_prime(6), 13);
/// ```
pub fn nth_prime(n: usize) -> u64 {
    if n == 0 {
        panic!("Prime indexing is 1-based");
    }

    // Estimate upper bound using Prime Number Theorem
    // p_n ≈ n * ln(n) * 1.3 for n > 6
    let estimate = if n < 6 {
        30 // Small cases
    } else {
        let n_f64 = n as f64;
        (n_f64 * n_f64.ln() * 1.3) as usize
    };

    let primes = sieve(estimate);

    if primes.len() >= n {
        primes[n - 1] as u64
    } else {
        // If estimate was too low, expand search
        let expanded_estimate = estimate * 2;
        let expanded_primes = sieve(expanded_estimate);
        expanded_primes[n - 1] as u64
    }
}

/// Solve Problem 7: Find the 10,001st prime
pub fn solve() -> u64 {
    nth_prime(10001)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_primes() {
        assert_eq!(nth_prime(1), 2);
        assert_eq!(nth_prime(2), 3);
        assert_eq!(nth_prime(3), 5);
        assert_eq!(nth_prime(4), 7);
        assert_eq!(nth_prime(5), 11);
        assert_eq!(nth_prime(6), 13);
    }

    #[test]
    fn test_example_6th_prime() {
        // Given in problem: 6th prime is 13
        assert_eq!(nth_prime(6), 13);
    }

    #[test]
    fn test_larger_primes() {
        assert_eq!(nth_prime(10), 29);
        assert_eq!(nth_prime(100), 541);
        assert_eq!(nth_prime(1000), 7919);
    }

    #[test]
    fn test_solve() {
        let result = solve();
        // The 10,001st prime is 104743
        assert_eq!(result, 104743);
    }

    #[test]
    #[should_panic(expected = "Prime indexing is 1-based")]
    fn test_zero_index_panics() {
        nth_prime(0);
    }
}
