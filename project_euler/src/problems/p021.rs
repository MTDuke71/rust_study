//! # Problem 21: Amicable Numbers
//!
//! Let d(n) = sum of proper divisors of n. If d(a) = b and d(b) = a,
//! where a ≠ b, then a and b are an **amicable pair**.
//!
//! Example: d(220) = 284 and d(284) = 220 → amicable pair.
//!
//! Find the sum of all amicable numbers under 10,000.
//!
//! ## Mathematical Foundation
//!
//! Uses a **divisor sum sieve** to compute d(n) for all n < 10,000 in one
//! pass (O(n log n)), then checks the amicable condition: d(d(n)) == n and
//! d(n) ≠ n. The sieve works like the Sieve of Eratosthenes but accumulates
//! divisor sums instead of marking composites.
//!
//! Note: d(n) = n means n is a **perfect number** (e.g., 6, 28), which is
//! explicitly excluded by the a ≠ b requirement.
//!
//! See [[math-foundations/project-euler-p021]] for full analysis.
//!
//! ## Complexity
//!
//! - Time: O(n log n) for sieve + O(n) for amicable check
//! - Space: O(n)

use crate::utils::number_theory::proper_divisor_sum_sieve;

/// Find the sum of all amicable numbers below `limit`.
///
/// A number n is amicable if d(d(n)) == n and d(n) ≠ n,
/// where d(n) = sum of proper divisors.
///
/// # Examples
/// ```
/// use project_euler::problems::p021::sum_amicable_below;
/// // 220 and 284 are the only amicable numbers below 300
/// assert_eq!(sum_amicable_below(300), 220 + 284);
/// ```
pub fn sum_amicable_below(limit: usize) -> u64 {
    let d = proper_divisor_sum_sieve(limit);

    (2..limit)
        .filter(|&n| {
            let partner = d[n] as usize;
            // Partner must be different from n, and d(partner) must point back to n
            partner != n && partner < limit && d[partner] == n as u64
        })
        .map(|n| n as u64)
        .sum()
}

/// Solve Problem 21: sum of amicable numbers under 10,000.
pub fn solve() -> u64 {
    sum_amicable_below(10_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_amicable_pair() {
        // 220 and 284 are the smallest amicable pair
        assert_eq!(sum_amicable_below(300), 220 + 284);
    }

    #[test]
    fn test_below_amicable_pair() {
        // No amicable numbers below 220
        assert_eq!(sum_amicable_below(220), 0);
    }

    #[test]
    fn test_perfect_numbers_excluded() {
        // 6 and 28 are perfect (d(n) = n), NOT amicable
        assert_eq!(sum_amicable_below(30), 0);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 31626);
    }
}
