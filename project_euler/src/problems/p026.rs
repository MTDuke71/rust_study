//! # Problem 26: Reciprocal Cycles
//!
//! A unit fraction contains 1 in the numerator. The decimal representation
//! of the unit fractions with denominators 2 to 10 are given:
//!
//! ```text
//! 1/2  = 0.5
//! 1/3  = 0.(3)          cycle length 1
//! 1/4  = 0.25
//! 1/5  = 0.2
//! 1/6  = 0.1(6)         cycle length 1
//! 1/7  = 0.(142857)     cycle length 6
//! 1/8  = 0.125
//! 1/9  = 0.(1)          cycle length 1
//! 1/10 = 0.1
//! ```
//!
//! Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle.
//!
//! Find the value of d < 1000 for which 1/d contains the longest
//! recurring cycle in its decimal fraction part.
//!
//! ## Mathematical Foundation
//!
//! The cycle length of 1/d equals the **multiplicative order** of 10 mod d:
//! the smallest positive k such that 10^k ≡ 1 (mod d).
//!
//! This only applies after removing factors of 2 and 5 from d (which
//! contribute non-repeating prefix digits, not cycle digits).
//!
//! Maximum possible cycle length for 1/d is d-1 (achieved when d is prime
//! and 10 is a primitive root mod d).
//!
//! See [[math-foundations/project-euler-p026]] for full analysis.
//!
//! ## Complexity
//!
//! - Time: O(n × d_max) where n = limit, d_max = longest cycle ≤ 983
//! - Space: O(1)

/// Find the repeating cycle length of 1/d using long division simulation.
///
/// Returns 0 if 1/d terminates (no repeating cycle).
///
/// # Examples
/// ```
/// use project_euler::problems::p026::cycle_length;
/// assert_eq!(cycle_length(2), 0);   // 0.5 terminates
/// assert_eq!(cycle_length(3), 1);   // 0.(3)
/// assert_eq!(cycle_length(7), 6);   // 0.(142857)
/// ```
pub fn cycle_length(d: u64) -> u64 {
    // Remove factors of 2 and 5 (they produce non-repeating prefix)
    let mut reduced = d;
    while reduced.is_multiple_of(2) {
        reduced /= 2;
    }
    while reduced.is_multiple_of(5) {
        reduced /= 5;
    }

    // If fully reduced to 1, the decimal terminates
    if reduced == 1 {
        return 0;
    }

    // Find multiplicative order of 10 mod reduced:
    // smallest k where 10^k ≡ 1 (mod reduced)
    let mut remainder = 10 % reduced;
    let mut k = 1;
    while remainder != 1 {
        remainder = (remainder * 10) % reduced;
        k += 1;
    }

    k
}

/// Find d < limit with the longest recurring cycle in 1/d.
///
/// # Examples
/// ```
/// use project_euler::problems::p026::longest_cycle_under;
/// assert_eq!(longest_cycle_under(10), 7);  // 1/7 has cycle length 6
/// ```
pub fn longest_cycle_under(limit: u64) -> u64 {
    (2..limit)
        .max_by_key(|&d| cycle_length(d))
        .unwrap()
}

/// Solve Problem 26: d < 1000 with longest reciprocal cycle.
pub fn solve() -> u64 {
    longest_cycle_under(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminating_decimals() {
        assert_eq!(cycle_length(2), 0);   // 0.5
        assert_eq!(cycle_length(4), 0);   // 0.25
        assert_eq!(cycle_length(5), 0);   // 0.2
        assert_eq!(cycle_length(8), 0);   // 0.125
        assert_eq!(cycle_length(10), 0);  // 0.1
        assert_eq!(cycle_length(16), 0);  // 0.0625
    }

    #[test]
    fn test_cycle_lengths() {
        assert_eq!(cycle_length(3), 1);   // 0.(3)
        assert_eq!(cycle_length(6), 1);   // 0.1(6)
        assert_eq!(cycle_length(7), 6);   // 0.(142857)
        assert_eq!(cycle_length(9), 1);   // 0.(1)
        assert_eq!(cycle_length(11), 2);  // 0.(09)
        assert_eq!(cycle_length(13), 6);  // 0.(076923)
    }

    #[test]
    fn test_max_cycle_is_d_minus_1() {
        // 1/7 achieves maximum cycle length of 6 = 7-1
        // (10 is a primitive root mod 7)
        assert_eq!(cycle_length(7), 6);
    }

    #[test]
    fn test_longest_under_10() {
        assert_eq!(longest_cycle_under(10), 7);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 983);
    }
}
