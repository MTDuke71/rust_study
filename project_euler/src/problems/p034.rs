//! # Problem 34: Digit Factorials
//!
//! 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
//!
//! Find the sum of all numbers which are equal to the sum of the factorial of
//! their digits.
//!
//! Note: As 1! = 1 and 2! = 2 are not sums, they are not included.
//!
//! ## Mathematical Foundation
//!
//! Define f(n) = Σ d! for each digit d of n. We seek all n with n = f(n) and
//! n ≥ 10 (excluding the trivial single-digit fixed points 1 and 2).
//!
//! **Upper bound derivation**: A d-digit number is at least 10^(d−1). The
//! maximum digit-factorial sum for d digits is d × 9!. We need d × 9! ≥ 10^(d−1).
//!
//! With 9! = 362,880:
//! - d = 7: 7 × 362,880 = 2,540,160 (7 digits ✓ — valid range)
//! - d = 8: 8 × 362,880 = 2,903,040 (still 7 digits — cannot reach an 8-digit number)
//!
//! So no number above 2,540,160 can equal the sum of factorials of its digits.
//! This mirrors the bound argument for [[project-euler-p030]] (digit fifth powers).
//!
//! See [[math-foundations/combinatorics-fundamentals]] for factorials and
//! [[math-foundations/order-of-magnitude]] for the bounding technique.
//!
//! ## Approach
//!
//! 1. Precompute a 10-element lookup table `fact[0..=9]` of digit factorials.
//! 2. For each n from 10 to 2,540,160, compute the digit-factorial sum.
//! 3. Collect numbers where n equals their digit-factorial sum.
//! 4. Sum the matches.
//!
//! ## Complexity
//!
//! - Time: O(N · d) where N ≈ 2.5 × 10⁶ and d ≤ 7 → ~17M digit lookups
//! - Space: O(1) table + O(k) for the few matches (only two: 145 and 40585)

/// Compute n! for digits 0–9. Used as a small lookup table.
const fn digit_factorial_table() -> [u64; 10] {
    let mut table = [1u64; 10];
    let mut i = 1;
    let mut acc = 1u64;
    while i < 10 {
        acc *= i as u64;
        table[i] = acc;
        i += 1;
    }
    table
}

const FACT: [u64; 10] = digit_factorial_table();

/// Sum of factorials of the decimal digits of `n`.
///
/// # Examples
/// ```
/// use project_euler::problems::p034::digit_factorial_sum;
/// assert_eq!(digit_factorial_sum(145), 145); // 1! + 4! + 5! = 1 + 24 + 120
/// assert_eq!(digit_factorial_sum(0), 1);     // 0! = 1 by convention
/// assert_eq!(digit_factorial_sum(9), 362880); // 9! = 362,880
/// ```
pub fn digit_factorial_sum(n: u64) -> u64 {
    if n == 0 {
        return FACT[0];
    }
    let mut sum = 0u64;
    let mut remaining = n;
    while remaining > 0 {
        sum += FACT[(remaining % 10) as usize];
        remaining /= 10;
    }
    sum
}

/// Find all n ≥ 10 with n = sum of factorials of its digits.
///
/// The single-digit fixed points 1 (1!) and 2 (2!) are excluded by the
/// problem statement (not "sums").
///
/// # Examples
/// ```
/// use project_euler::problems::p034::find_digit_factorial_numbers;
/// assert_eq!(find_digit_factorial_numbers(), vec![145, 40585]);
/// ```
pub fn find_digit_factorial_numbers() -> Vec<u64> {
    // Upper bound: largest d × 9! that still fits in d digits.
    let nine_fact = FACT[9];
    let mut d = 2u64;
    while d * nine_fact >= 10u64.pow((d - 1) as u32) {
        d += 1;
    }
    let upper_bound = (d - 1) * nine_fact;

    (10..=upper_bound)
        .filter(|&n| digit_factorial_sum(n) == n)
        .collect()
}

/// Solve Problem 34: sum of all numbers equal to the sum of factorials of their digits.
pub fn solve() -> u64 {
    find_digit_factorial_numbers().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_table() {
        assert_eq!(FACT[0], 1);
        assert_eq!(FACT[1], 1);
        assert_eq!(FACT[5], 120);
        assert_eq!(FACT[9], 362880);
    }

    #[test]
    fn test_digit_factorial_sum_example() {
        // From problem statement: 1! + 4! + 5! = 145
        assert_eq!(digit_factorial_sum(145), 145);
    }

    #[test]
    fn test_digit_factorial_sum_other_curious() {
        // The only other digit-factorial number, found by exhaustive search
        assert_eq!(digit_factorial_sum(40585), 40585);
    }

    #[test]
    fn test_digit_factorial_sum_non_match() {
        // 100: 1! + 0! + 0! = 1 + 1 + 1 = 3, not 100
        assert_eq!(digit_factorial_sum(100), 3);
    }

    #[test]
    fn test_find_digit_factorial_numbers() {
        // Only two such numbers exist (excluding trivial 1, 2)
        assert_eq!(find_digit_factorial_numbers(), vec![145, 40585]);
    }

    #[test]
    fn test_solve() {
        // 145 + 40585 = 40730
        assert_eq!(solve(), 40730);
    }
}
