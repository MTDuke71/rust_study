//! # Problem 30: Digit Fifth Powers
//!
//! Surprisingly there are only three numbers that can be written as the sum of
//! fourth powers of their digits: 1634, 8208, 9474 (sum = 19316).
//!
//! Find the sum of all the numbers that can be written as the sum of fifth
//! powers of their digits. (Note: as 1 = 1⁵ is not a sum, it is excluded.)
//!
//! ## Mathematical Foundation
//!
//! **Upper bound derivation**: A d-digit number is at least 10^(d−1). The
//! maximum digit-power sum for d digits is d × 9^p. We need d × 9^p ≥ 10^(d−1).
//!
//! For fifth powers (p = 5, 9⁵ = 59,049):
//! - d=6: 6 × 59,049 = 354,294 (6 digits ✓ — valid range)
//! - d=7: 7 × 59,049 = 413,343 (6 digits — cannot produce a 7-digit number)
//!
//! So the upper bound is 354,294. No number above this can equal the sum of
//! fifth powers of its digits.
//!
//! ## Approach
//!
//! 1. Precompute `digit^power` lookup table for digits 0–9
//! 2. For each number from 2 to upper_bound, compute digit power sum
//! 3. If number equals its digit power sum, it's a match (narcissistic number)
//! 4. Sum all matches
//!
//! ## Complexity
//!
//! - Time: O(upper_bound × d) where d = number of digits (~6)
//! - Space: O(1) beyond the results vector

/// Compute the sum of each digit raised to the given power.
///
/// # Examples
/// ```
/// use project_euler::problems::p030::digit_power_sum;
/// assert_eq!(digit_power_sum(1634, 4), 1634); // 1⁴ + 6⁴ + 3⁴ + 4⁴
/// assert_eq!(digit_power_sum(9474, 4), 9474); // 9⁴ + 4⁴ + 7⁴ + 4⁴
/// assert_eq!(digit_power_sum(4150, 5), 4150); // 4⁵ + 1⁵ + 5⁵ + 0⁵
/// ```
pub fn digit_power_sum(n: u64, power: u32) -> u64 {
    let mut sum = 0u64;
    let mut remaining = n;
    while remaining > 0 {
        let digit = remaining % 10;
        sum += digit.pow(power);
        remaining /= 10;
    }
    sum
}

/// Find all numbers equal to the sum of their digits raised to `power`.
///
/// Excludes single-digit trivial cases (e.g., 1 = 1^p).
/// Uses the upper bound: d × 9^p where d is the number of digits at which
/// the maximum digit-power sum can no longer reach a d-digit number.
///
/// # Examples
/// ```
/// use project_euler::problems::p030::find_digit_power_numbers;
/// let fourth = find_digit_power_numbers(4);
/// assert_eq!(fourth, vec![1634, 8208, 9474]);
/// ```
pub fn find_digit_power_numbers(power: u32) -> Vec<u64> {
    // Compute upper bound: find largest d where d × 9^p has d digits
    let nine_pow = 9u64.pow(power);
    let mut d = 2u64;
    while d * nine_pow >= 10u64.pow((d - 1) as u32) {
        d += 1;
    }
    let upper_bound = (d - 1) * nine_pow;

    let mut results = Vec::new();
    for n in 2..=upper_bound {
        if digit_power_sum(n, power) == n {
            results.push(n);
        }
    }
    results
}

/// Solve Problem 30: sum of all numbers equal to the sum of fifth powers of their digits.
pub fn solve() -> u64 {
    find_digit_power_numbers(5).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_power_sum_basic() {
        assert_eq!(digit_power_sum(0, 5), 0);
        assert_eq!(digit_power_sum(1, 5), 1);
        assert_eq!(digit_power_sum(10, 5), 1); // 1⁵ + 0⁵
        assert_eq!(digit_power_sum(99, 5), 2 * 9u64.pow(5)); // 9⁵ + 9⁵
    }

    #[test]
    fn test_digit_power_sum_fourth() {
        assert_eq!(digit_power_sum(1634, 4), 1634);
        assert_eq!(digit_power_sum(8208, 4), 8208);
        assert_eq!(digit_power_sum(9474, 4), 9474);
    }

    #[test]
    fn test_digit_power_sum_fifth() {
        assert_eq!(digit_power_sum(4150, 5), 4150);
        assert_eq!(digit_power_sum(4151, 5), 4151);
        assert_eq!(digit_power_sum(54748, 5), 54748);
        assert_eq!(digit_power_sum(92727, 5), 92727);
        assert_eq!(digit_power_sum(93084, 5), 93084);
        assert_eq!(digit_power_sum(194979, 5), 194979);
    }

    #[test]
    fn test_fourth_power_numbers() {
        let result = find_digit_power_numbers(4);
        assert_eq!(result, vec![1634, 8208, 9474]);
    }

    #[test]
    fn test_fourth_power_sum() {
        let sum: u64 = find_digit_power_numbers(4).iter().sum();
        assert_eq!(sum, 19316);
    }

    #[test]
    fn test_fifth_power_numbers() {
        let result = find_digit_power_numbers(5);
        assert_eq!(result, vec![4150, 4151, 54748, 92727, 93084, 194979]);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 443839);
    }
}
