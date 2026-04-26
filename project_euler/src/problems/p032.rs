//! # Problem 32: Pandigital Products
//!
//! We shall say that an n-digit number is pandigital if it makes use of all
//! the digits 1 to n exactly once; for example, the 5-digit number 15234 is
//! 1 through 5 pandigital.
//!
//! The product 7254 is unusual, as the identity 39 × 186 = 7254, containing
//! multiplicand, multiplier, and product, is 1 through 9 pandigital.
//!
//! Find the sum of all products whose multiplicand/multiplier/product identity
//! can be written as a 1 through 9 pandigital. (Some products can be obtained
//! in more than one way; include each product only once.)
//!
//! ## Mathematical Foundation
//!
//! Let the multiplicand have `a` digits, the multiplier `b` digits, and the
//! product `c` digits. Pandigitality requires a + b + c = 9.
//!
//! A multiplication of an `a`-digit by a `b`-digit number yields a result with
//! either `a + b - 1` or `a + b` digits. Setting c = a + b gives 2(a+b) = 9,
//! which has no integer solution. Setting c = a + b - 1 gives 2(a+b) - 1 = 9,
//! so **a + b = 5** and **c = 4**.
//!
//! Therefore exactly two shapes can produce a pandigital identity:
//!
//! - 1-digit × 4-digit = 4-digit  (e.g., 4 × 1738 = 6952)
//! - 2-digit × 3-digit = 4-digit  (e.g., 39 × 186 = 7254)
//!
//! ## Approach
//!
//! Enumerate the two shapes separately. For each (a, b) pair compute the
//! product and verify the concatenated digits of (a, b, a*b) form a 1–9
//! pandigital using a 9-bit digit-set bitmask. Collect successful products
//! in a HashSet to deduplicate.
//!
//! ## Complexity
//!
//! - Time: O(N) where N ≈ 9 × 9876 + 99 × 999 ≈ 200K candidates, each
//!   pandigital check is O(9) digit operations.
//! - Space: O(k) where k = number of pandigital products found.

use std::collections::HashSet;

/// Check whether the concatenation of the digits of `nums` is exactly the
/// 1-through-9 pandigital — uses each digit 1..=9 once, no zeros, no duplicates.
///
/// Uses a 16-bit mask where bit `d` (1..=9) is set when digit `d` is seen.
/// Detects duplicates by checking the bit before setting it. The combined
/// loop short-circuits on the very first invalid digit, which is faster
/// than computing per-number masks separately.
///
/// # Examples
/// ```
/// use project_euler::problems::p032::is_pandigital_1_9;
/// assert!(is_pandigital_1_9(&[39, 186, 7254]));
/// assert!(!is_pandigital_1_9(&[39, 186, 7000])); // contains 0
/// assert!(!is_pandigital_1_9(&[12, 34, 5678]));  // missing digit 9
/// ```
pub fn is_pandigital_1_9(nums: &[u64]) -> bool {
    const TARGET: u16 = 0b11_1111_1110; // bits 1..=9
    let mut bits: u16 = 0;
    for &n in nums {
        let mut x = n;
        while x > 0 {
            let d = (x % 10) as u16;
            if d == 0 {
                return false;
            }
            let bit = 1u16 << d;
            if bits & bit != 0 {
                return false; // duplicate digit (within or across numbers)
            }
            bits |= bit;
            x /= 10;
        }
    }
    bits == TARGET
}

/// Find all products c such that a × b = c is 1-through-9 pandigital.
///
/// Only the shapes 1×4=4 and 2×3=4 are searched (proven exhaustive in module docs).
/// Returns the deduplicated set of products.
pub fn pandigital_products() -> HashSet<u64> {
    let mut products = HashSet::new();

    // Shape 1: 1-digit × 4-digit = 4-digit
    for a in 1..=9u64 {
        for b in 1234..=9876u64 {
            let c = a * b;
            if c > 9876 {
                break;
            }
            if c < 1000 {
                continue;
            }
            if is_pandigital_1_9(&[a, b, c]) {
                products.insert(c);
            }
        }
    }

    // Shape 2: 2-digit × 3-digit = 4-digit
    for a in 12..=98u64 {
        for b in 123..=987u64 {
            let c = a * b;
            if c > 9876 {
                break;
            }
            if c < 1000 {
                continue;
            }
            if is_pandigital_1_9(&[a, b, c]) {
                products.insert(c);
            }
        }
    }

    products
}

/// Solve Problem 32: sum of all distinct pandigital products.
pub fn solve() -> u64 {
    pandigital_products().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pandigital_problem_example() {
        // 39 × 186 = 7254 is the canonical example
        assert!(is_pandigital_1_9(&[39, 186, 7254]));
    }

    #[test]
    fn test_is_pandigital_rejects_zero() {
        assert!(!is_pandigital_1_9(&[10, 234, 5678]));
    }

    #[test]
    fn test_is_pandigital_rejects_duplicates() {
        // Duplicate 1: 11 × 234 = 2574, but 11 has duplicate 1
        assert!(!is_pandigital_1_9(&[11, 234, 5678]));
    }

    #[test]
    fn test_is_pandigital_rejects_wrong_digit_count() {
        // Total digits = 8, missing one
        assert!(!is_pandigital_1_9(&[1, 234, 5678]));
    }

    #[test]
    fn test_pandigital_products_contains_examples() {
        let products = pandigital_products();
        // Known pandigital identities:
        //   39 × 186 = 7254
        //   18 × 297 = 5346
        //   27 × 198 = 5346 (same product, dedup)
        //   28 × 157 = 4396
        //   12 × 483 = 5796
        //   42 × 138 = 5796
        //   48 × 159 = 7632
        //   4 × 1738 = 6952
        //   4 × 1963 = 7852
        assert!(products.contains(&7254));
        assert!(products.contains(&5346));
        assert!(products.contains(&5796));
        assert!(products.contains(&6952));
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 45228);
    }
}
