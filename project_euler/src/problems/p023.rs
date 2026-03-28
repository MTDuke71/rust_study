//! # Problem 23: Non-Abundant Sums
//!
//! A perfect number is one where the sum of its proper divisors equals it.
//! A number is **deficient** if this sum is less, and **abundant** if greater.
//!
//! 12 is the smallest abundant number: 1+2+3+4+6 = 16 > 12.
//!
//! It can be shown that all integers greater than 28123 can be written as
//! the sum of two abundant numbers. Find the sum of all positive integers
//! which **cannot** be expressed as the sum of two abundant numbers.
//!
//! ## Mathematical Foundation
//!
//! Uses the **divisor sum sieve** from Problem 21 (`proper_divisor_sum_sieve`)
//! to classify all numbers as abundant in O(n log n). Then a boolean sieve
//! marks every pairwise sum of abundants. The answer is the sum of unmarked values.
//!
//! Key fact: The upper bound 28123 is given by the problem. In practice, the
//! largest non-expressible number is 20161.
//!
//! See [[math-foundations/project-euler-p023]] for full analysis.
//!
//! ## Complexity
//!
//! - Time: O(n log n) for divisor sieve + O(a²) for pairwise sums (a = count of abundants)
//! - Space: O(n)

use crate::utils::number_theory::proper_divisor_sum_sieve;

const LIMIT: usize = 28124; // all integers > 28123 are sums of two abundants

/// Find all abundant numbers below `limit`.
///
/// A number n is abundant if the sum of its proper divisors exceeds n.
///
/// # Examples
/// ```
/// use project_euler::problems::p023::abundant_numbers;
/// let abundants = abundant_numbers(25);
/// assert_eq!(abundants, vec![12, 18, 20, 24]);
/// ```
pub fn abundant_numbers(limit: usize) -> Vec<usize> {
    let sieve = proper_divisor_sum_sieve(limit);
    (1..limit)
        .filter(|&n| sieve[n] > n as u64)
        .collect()
}

/// Sum of all positive integers that cannot be written as the sum of two abundant numbers.
///
/// # Examples
/// ```
/// use project_euler::problems::p023::sum_non_abundant_sums;
/// // All numbers <= 23 cannot be expressed as sum of two abundants
/// // (smallest abundant is 12, so smallest sum is 12+12=24)
/// let result = sum_non_abundant_sums(24);
/// assert_eq!(result, (1..24u64).sum::<u64>()); // 1+2+...+23 = 276
/// ```
pub fn sum_non_abundant_sums(limit: usize) -> u64 {
    let abundants = abundant_numbers(limit);

    // Mark all numbers that ARE expressible as sum of two abundants
    let mut is_abundant_sum = vec![false; limit];
    for (i, &a) in abundants.iter().enumerate() {
        for &b in &abundants[i..] {
            let sum = a + b;
            if sum >= limit {
                break;
            }
            is_abundant_sum[sum] = true;
        }
    }

    // Sum all numbers that are NOT expressible
    (1..limit)
        .filter(|&n| !is_abundant_sum[n])
        .map(|n| n as u64)
        .sum()
}

/// Solve Problem 23 with the proven upper bound of 28123.
pub fn solve() -> u64 {
    sum_non_abundant_sums(LIMIT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_abundant() {
        // 12 is the smallest abundant number
        let abundants = abundant_numbers(13);
        assert_eq!(abundants, vec![12]);
    }

    #[test]
    fn test_abundant_numbers_below_25() {
        let abundants = abundant_numbers(25);
        assert_eq!(abundants, vec![12, 18, 20, 24]);
    }

    #[test]
    fn test_below_smallest_abundant_sum() {
        // 12+12=24 is smallest sum of two abundants
        // So all numbers 1..24 cannot be expressed → sum = 276
        assert_eq!(sum_non_abundant_sums(24), (1..24u64).sum::<u64>());
    }

    #[test]
    fn test_24_is_abundant_sum() {
        // 24 = 12+12, so it IS expressible
        // sum_non_abundant_sums(25) should exclude 24
        let with_24 = sum_non_abundant_sums(25);
        let without_24 = sum_non_abundant_sums(24);
        assert_eq!(with_24, without_24); // 24 excluded, so same total
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 4179871);
    }
}
