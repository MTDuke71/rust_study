//! # Problem 5: Smallest Multiple
//!
//! Find the smallest positive number divisible by all numbers 1-20.
//!
//! ## Approach
//!
//! Use the mathematical relationship:
//! - LCM(a, b) = (a × b) / GCD(a, b)
//! - LCM is associative: LCM(1..n) = LCM(LCM(1..n-1), n)
//!
//! ## Algorithm
//!
//! 1. Start with LCM = 1
//! 2. For each number k in range, compute LCM(current_lcm, k)
//! 3. Use Euclidean algorithm for GCD (from utils::number_theory)
//!
//! ## Complexity
//!
//! - Time: O(n log(max)) where max is the largest number
//! - Space: O(1)

use crate::utils::number_theory::lcm;

/// Find smallest number divisible by all numbers 1..=n
pub fn smallest_multiple(n: u64) -> u64 {
    (1..=n).fold(1, lcm)
}

/// Solve Problem 5 for numbers 1-20
pub fn solve() -> u64 {
    smallest_multiple(20)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::number_theory::{gcd, lcm};

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 19), 1); // Coprime
        assert_eq!(gcd(100, 50), 50);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(12, 8), 24);
        assert_eq!(lcm(15, 20), 60);
        assert_eq!(lcm(7, 11), 77); // Coprime primes
    }

    #[test]
    fn test_example_1_to_10() {
        assert_eq!(smallest_multiple(10), 2520);
    }

    #[test]
    fn test_small_cases() {
        assert_eq!(smallest_multiple(1), 1);
        assert_eq!(smallest_multiple(2), 2);
        assert_eq!(smallest_multiple(3), 6);
        assert_eq!(smallest_multiple(4), 12);
        assert_eq!(smallest_multiple(5), 60);
    }

    #[test]
    fn test_verify_divisibility_1_to_10() {
        let result = smallest_multiple(10);
        for i in 1..=10 {
            assert_eq!(result % i, 0, "{} should divide {}", i, result);
        }
    }

    #[test]
    fn test_solution_1_to_20() {
        let result = smallest_multiple(20);
        // Verify divisibility
        for i in 1..=20 {
            assert_eq!(result % i, 0, "{} should divide {}", i, result);
        }
        // The answer is 232792560
        assert_eq!(result, 232792560);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 232792560);
    }
}
