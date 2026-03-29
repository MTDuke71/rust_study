//! # Problem 27: Quadratic Primes
//!
//! Euler discovered the remarkable quadratic formula: n² + n + 41
//!
//! It turns out that the formula will produce 40 primes for the consecutive
//! integer values 0 ≤ n ≤ 39. However, when n = 40, 40² + 40 + 41 =
//! 40(40 + 1) + 41 = 41², which is divisible by 41.
//!
//! The incredible formula n² − 79n + 1601 produces 80 primes for 0 ≤ n ≤ 79.
//!
//! Find the product of the coefficients a and b, for the quadratic expression
//! n² + an + b, where |a| < 1000 and |b| ≤ 1000, that produces the maximum
//! number of primes for consecutive values of n, starting with n = 0.
//!
//! ## Mathematical Foundation
//!
//! Key constraints that prune the search space:
//! - **n=0**: f(0) = b, so b must be prime (positive)
//! - **n=1**: f(1) = 1 + a + b must be prime, so a > -(b+1)
//! - b must be positive since primes are positive
//!
//! By restricting b to primes ≤ 1000 (~168 values), the search drops from
//! ~4 million pairs to ~336,000.
//!
//! See [[math-foundations/project-euler-p027]] for full analysis.
//!
//! ## Complexity
//!
//! - Time: O(primes(1000) × 2000 × avg_chain) ≈ O(168 × 2000 × ~40)
//! - Space: O(1) beyond the prime sieve

use crate::utils::primes::{is_prime, sieve};

/// Count consecutive primes produced by n² + an + b starting from n = 0.
///
/// # Examples
/// ```
/// use project_euler::problems::p027::consecutive_primes;
/// // Euler's n² + n + 41 produces 40 primes (n = 0..39)
/// assert_eq!(consecutive_primes(1, 41), 40);
/// ```
pub fn consecutive_primes(a: i64, b: i64) -> u64 {
    let mut n = 0i64;
    loop {
        let val = n * n + a * n + b;
        if val < 2 || !is_prime(val as u64) {
            return n as u64;
        }
        n += 1;
    }
}

/// Find coefficients (a, b) that produce the most consecutive primes.
///
/// Returns (a, b, count) where n² + an + b produces `count` primes for n = 0..count-1.
///
/// # Examples
/// ```
/// use project_euler::problems::p027::best_quadratic;
/// let (a, b, count) = best_quadratic(1000);
/// assert_eq!(a * b, -59231);
/// assert!(count > 40); // must beat Euler's n²+n+41
/// ```
pub fn best_quadratic(limit: i64) -> (i64, i64, u64) {
    // b must be prime (since f(0) = b)
    let primes_b: Vec<i64> = sieve(limit as usize)
        .into_iter()
        .map(|p| p as i64)
        .collect();

    let mut best_a = 0i64;
    let mut best_b = 0i64;
    let mut best_count = 0u64;

    for &b in &primes_b {
        // a ranges from -(limit-1) to (limit-1)
        // But f(1) = 1 + a + b must be > 0, so a > -(b+1)
        let a_min = -(b + 1) + 1;
        for a in a_min..limit {
            let count = consecutive_primes(a, b);
            if count > best_count {
                best_count = count;
                best_a = a;
                best_b = b;
            }
        }
    }

    (best_a, best_b, best_count)
}

/// Solve Problem 27: product of coefficients a × b.
///
/// Note: Answer is negative (-59231). The registry stores as u64 bit pattern;
/// the main runner displays it as i64.
pub fn solve() -> i64 {
    let (a, b, _) = best_quadratic(1000);
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_formula() {
        // n² + n + 41 produces 40 primes
        assert_eq!(consecutive_primes(1, 41), 40);
    }

    #[test]
    fn test_remarkable_formula() {
        // n² − 79n + 1601 produces 80 primes
        assert_eq!(consecutive_primes(-79, 1601), 80);
    }

    #[test]
    fn test_simple_cases() {
        // n² + 0n + 2: f(0)=2 prime, f(1)=3 prime, f(2)=6 not prime
        assert_eq!(consecutive_primes(0, 2), 2);
    }

    #[test]
    fn test_best_beats_euler() {
        let (_, _, count) = best_quadratic(1000);
        assert!(count > 40, "Should beat Euler's 40 consecutive primes");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), -59231);
    }
}
