//! # Problem 1: Multiples of 3 or 5
//!
//! Find the sum of all multiples of 3 or 5 below 1000.
//!
//! ## Mathematical Foundation
//!
//! Uses **arithmetic series** formula: sum = n × (first + last) / 2
//! Uses **inclusion-exclusion principle**: |A ∪ B| = |A| + |B| - |A ∩ B|
//!
//! See `zettelkasten/math-foundations/arithmetic-series.md` for theory.
//! See `zettelkasten/math-foundations/inclusion-exclusion.md` for technique.
//!
//! Full write-up: `zettelkasten/math-foundations/project-euler-p001.md`
//!
//! ## Approach
//!
//! Instead of iterating through all numbers (O(n)), use arithmetic series formula (O(1)):
//!
//! 1. Sum multiples of 3: 3 + 6 + 9 + ... + 999 = 3(1 + 2 + 3 + ... + 333)
//! 2. Sum multiples of 5: 5 + 10 + 15 + ... + 995 = 5(1 + 2 + 3 + ... + 199)
//! 3. Sum multiples of 15: avoid double-counting (3 and 5 overlap at 15)
//! 4. Result = sum(3) + sum(5) - sum(15)
//!
//! ## Complexity
//!
//! - Time: O(1) - constant time arithmetic
//! - Space: O(1) - no additional memory

/// Sum of multiples of `divisor` below `limit` using arithmetic series
///
/// Formula: sum = divisor × n × (n + 1) / 2, where n = (limit - 1) / divisor
///
/// # Examples
/// ```
/// # use project_euler::problems::p001::sum_multiples;
/// assert_eq!(sum_multiples(3, 10), 18); // 3 + 6 + 9 = 18
/// assert_eq!(sum_multiples(5, 10), 5);  // 5
/// ```
pub fn sum_multiples(divisor: u64, limit: u64) -> u64 {
    if divisor >= limit {
        return 0;
    }

    let n = (limit - 1) / divisor; // How many multiples fit below limit
    divisor * n * (n + 1) / 2 // Arithmetic series: divisor × (1 + 2 + ... + n)
}

/// Solve Problem 1: Sum of multiples of 3 or 5 below 1000
///
/// Uses inclusion-exclusion principle to avoid double-counting
/// multiples of both 3 and 5 (i.e., multiples of 15).
///
/// # Examples
/// ```
/// use project_euler::problems::p001::solve;
/// assert_eq!(solve(), 233168);
/// ```
pub fn solve() -> u64 {
    const LIMIT: u64 = 1000;

    let sum_3 = sum_multiples(3, LIMIT);
    let sum_5 = sum_multiples(5, LIMIT);
    let sum_15 = sum_multiples(15, LIMIT); // LCM(3, 5) = 15

    // Inclusion-exclusion: add both sets, subtract overlap
    sum_3 + sum_5 - sum_15
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Example from problem: multiples below 10
        const LIMIT: u64 = 10;
        let result = sum_multiples(3, LIMIT) + sum_multiples(5, LIMIT) - sum_multiples(15, LIMIT);
        assert_eq!(result, 23); // 3 + 5 + 6 + 9 = 23
    }

    #[test]
    fn test_sum_multiples() {
        assert_eq!(sum_multiples(3, 10), 18); // 3 + 6 + 9 = 18
        assert_eq!(sum_multiples(5, 10), 5); // 5
        assert_eq!(sum_multiples(15, 10), 0); // No multiples of 15 below 10
    }

    #[test]
    fn test_solution() {
        assert_eq!(solve(), 233168);
    }
}
