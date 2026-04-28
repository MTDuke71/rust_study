//! # Problem 36: Double-base Palindromes
//!
//! The decimal number, 585 = 1001001001₂ (binary), is palindromic in both bases.
//!
//! Find the sum of all numbers, less than one million, which are palindromic in
//! base 10 and base 2.
//!
//! (Please note that the palindromic number, in either base, may not include
//! leading zeros.)
//!
//! ## Mathematical Foundation
//!
//! A **palindrome** in base b is a number whose digit sequence in that base
//! reads the same forward and backward. The "no leading zeros" rule equivalently
//! forbids trailing zeros: if the most-significant digit is non-zero (always
//! true for n > 0), the least-significant digit must equal it on reversal, and
//! a trailing zero would force a leading zero.
//!
//! **Binary corollary**: Every double-base palindrome is **odd**. A binary
//! palindrome with no leading zeros has its most-significant bit set to 1, so
//! its least-significant bit must also be 1 → the number is odd. This halves
//! the candidate space.
//!
//! See [[math-foundations/palindromes]] for general palindrome theory and
//! [[math-foundations/base-conversion]] for binary representation mechanics.
//!
//! ## Approach
//!
//! Brute force suffices given the scope (< 10⁶):
//!
//! 1. Iterate odd n from 1 to 999,999.
//! 2. For each n, check decimal palindrome via integer reversal.
//! 3. If decimal-palindromic, check binary palindrome via bit reversal.
//! 4. Sum matches.
//!
//! Decimal palindrome is checked first because the test is faster (smaller
//! digit count) and rejects most candidates immediately.
//!
//! ## Complexity
//!
//! - Time: O(N) over odd integers below N, with O(log N) digit work each
//! - Space: O(1)

/// Test whether `n` is a palindrome when written in base 10 (no leading zeros
/// since we work with the natural unsigned representation of n).
///
/// # Examples
/// ```
/// use project_euler::problems::p036::is_decimal_palindrome;
/// assert!(is_decimal_palindrome(585));
/// assert!(is_decimal_palindrome(7));      // single digit
/// assert!(!is_decimal_palindrome(123));
/// ```
pub fn is_decimal_palindrome(n: u64) -> bool {
    let mut x = n;
    let mut reversed = 0u64;
    while x > 0 {
        reversed = reversed * 10 + x % 10;
        x /= 10;
    }
    reversed == n
}

/// Test whether `n` is a palindrome when written in base 2.
///
/// Constructs the bit-reverse of n's significant bits and compares.
///
/// # Examples
/// ```
/// use project_euler::problems::p036::is_binary_palindrome;
/// assert!(is_binary_palindrome(585)); // 1001001001
/// assert!(is_binary_palindrome(9));   // 1001
/// assert!(!is_binary_palindrome(10)); // 1010 reversed is 0101 = 0101
/// ```
pub fn is_binary_palindrome(n: u64) -> bool {
    let mut x = n;
    let mut reversed = 0u64;
    while x > 0 {
        reversed = (reversed << 1) | (x & 1);
        x >>= 1;
    }
    reversed == n
}

/// Sum of all positive integers below `limit` that are palindromic in both
/// base 10 and base 2.
///
/// Iterates only odd candidates because every binary palindrome with no leading
/// zeros is odd (its highest and lowest set bits must both be 1).
///
/// # Examples
/// ```
/// use project_euler::problems::p036::sum_double_base_palindromes_below;
/// // Below 10: 1, 3, 5, 7, 9 are decimal-palindromic; binary palindromes
/// // among them are 1 (1), 3 (11), 5 (101), 7 (111), 9 (1001). Sum = 25.
/// assert_eq!(sum_double_base_palindromes_below(10), 25);
/// ```
pub fn sum_double_base_palindromes_below(limit: u64) -> u64 {
    (1..limit)
        .step_by(2)
        .filter(|&n| is_decimal_palindrome(n) && is_binary_palindrome(n))
        .sum()
}

/// Solve Problem 36: sum of all numbers below one million palindromic in both
/// base 10 and base 2.
pub fn solve() -> u64 {
    sum_double_base_palindromes_below(1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_palindrome_basic() {
        assert!(is_decimal_palindrome(0));
        assert!(is_decimal_palindrome(1));
        assert!(is_decimal_palindrome(9));
        assert!(is_decimal_palindrome(11));
        assert!(is_decimal_palindrome(121));
        assert!(is_decimal_palindrome(12321));

        assert!(!is_decimal_palindrome(10));
        assert!(!is_decimal_palindrome(123));
        assert!(!is_decimal_palindrome(1234));
    }

    #[test]
    fn test_binary_palindrome_basic() {
        assert!(is_binary_palindrome(0));
        assert!(is_binary_palindrome(1)); // 1
        assert!(is_binary_palindrome(3)); // 11
        assert!(is_binary_palindrome(5)); // 101
        assert!(is_binary_palindrome(7)); // 111
        assert!(is_binary_palindrome(9)); // 1001

        assert!(!is_binary_palindrome(2)); // 10
        assert!(!is_binary_palindrome(4)); // 100
        assert!(!is_binary_palindrome(10)); // 1010
    }

    #[test]
    fn test_problem_statement_example() {
        // From problem: 585 = 1001001001 in binary, palindromic in both bases
        assert!(is_decimal_palindrome(585));
        assert!(is_binary_palindrome(585));
    }

    #[test]
    fn test_sum_below_ten() {
        // Doctest case: 1 + 3 + 5 + 7 + 9 = 25
        assert_eq!(sum_double_base_palindromes_below(10), 25);
    }

    #[test]
    fn test_sum_below_thousand() {
        // Double-base palindromes < 1000:
        //   1, 3, 5, 7, 9, 33, 99, 313, 585, 717
        // Sum = 1 + 3 + 5 + 7 + 9 + 33 + 99 + 313 + 585 + 717 = 1772
        assert_eq!(sum_double_base_palindromes_below(1_000), 1772);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 872187);
    }
}
