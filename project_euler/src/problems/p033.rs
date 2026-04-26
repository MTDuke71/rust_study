//! # Problem 33: Digit Cancelling Fractions
//!
//! The fraction 49/98 is a curious fraction, as an inexperienced mathematician
//! in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which
//! is correct, by cancelling the 9s.
//!
//! We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
//!
//! There are exactly four non-trivial examples of this type of fraction, less
//! than one in value, and containing two digits in the numerator and denominator.
//!
//! If the product of these four fractions is given in its lowest common terms,
//! find the value of the denominator.
//!
//! ## Mathematical Foundation
//!
//! For a two-digit numerator n = 10·n₁ + n₂ and denominator d = 10·d₁ + d₂,
//! "cancelling" a shared digit gives a reduced fraction. There are four
//! position pairings to test for a shared digit:
//!
//!   (n₁, d₁) → n₂ / d₂
//!   (n₁, d₂) → n₂ / d₁
//!   (n₂, d₁) → n₁ / d₂
//!   (n₂, d₂) → n₁ / d₁
//!
//! For the cancellation to be "correct" we need the reduced fraction to equal
//! the original: n · new_d = d · new_n (cross-multiplication, avoids floats).
//!
//! "Trivial" fractions are those where both the numerator and denominator end
//! in 0 (e.g., 30/50 = 3/5 by cancelling the trailing zeros).
//!
//! See [[math-foundations/lcm-gcd-euclidean]] for reducing the product.
//!
//! ## Approach
//!
//! 1. For each pair (n, d) with 10 ≤ n < d ≤ 99, test all four cancellation
//!    pairings using exact integer cross-multiplication.
//! 2. Skip the trivial case where n₂ = 0 and d₂ = 0.
//! 3. Multiply numerators and denominators of all matches.
//! 4. Reduce the product fraction by GCD; return the denominator.
//!
//! ## Complexity
//!
//! - Time: O(N²) where N = 90 (≤ 8100 pairs, constant work per pair)
//! - Space: O(k) for the (very few) matching fractions

use crate::utils::number_theory::gcd;

/// Test whether fraction n/d (both 2-digit) is a non-trivial digit-cancelling
/// fraction. Returns true iff some position-pair shares a digit and the
/// resulting cancelled fraction equals n/d, excluding the trailing-zero case.
fn is_curious(n: u64, d: u64) -> bool {
    let (n1, n2) = (n / 10, n % 10);
    let (d1, d2) = (d / 10, d % 10);

    // Trivial: both end in 0 (e.g., 30/50)
    if n2 == 0 && d2 == 0 {
        return false;
    }

    // (cancelled_in_n, cancelled_in_d, kept_n, kept_d)
    let pairings = [
        (n1, d1, n2, d2),
        (n1, d2, n2, d1),
        (n2, d1, n1, d2),
        (n2, d2, n1, d1),
    ];

    for (cn, cd, new_n, new_d) in pairings {
        // Need a shared digit AND a non-zero new denominator
        if cn != cd || new_d == 0 {
            continue;
        }
        // Cross-multiply to compare n/d to new_n/new_d exactly
        if n * new_d == d * new_n {
            return true;
        }
    }
    false
}

/// Find all non-trivial digit-cancelling fractions n/d with n < d, both 2-digit.
///
/// # Examples
/// ```
/// use project_euler::problems::p033::find_curious_fractions;
/// let frs = find_curious_fractions();
/// assert_eq!(frs, vec![(16, 64), (19, 95), (26, 65), (49, 98)]);
/// ```
pub fn find_curious_fractions() -> Vec<(u64, u64)> {
    let mut results = Vec::new();
    for n in 10..=99u64 {
        for d in (n + 1)..=99u64 {
            if is_curious(n, d) {
                results.push((n, d));
            }
        }
    }
    results
}

/// Solve Problem 33: denominator of the product of the four curious fractions
/// in lowest terms.
pub fn solve() -> u64 {
    let fractions = find_curious_fractions();
    let num_prod: u64 = fractions.iter().map(|(n, _)| n).product();
    let den_prod: u64 = fractions.iter().map(|(_, d)| d).product();
    den_prod / gcd(num_prod, den_prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_example_49_98() {
        // 49/98 = 4/8 by cancelling 9s — the canonical example
        assert!(is_curious(49, 98));
    }

    #[test]
    fn test_trivial_30_50_excluded() {
        // 30/50 = 3/5 trivially by cancelling trailing zeros — must be excluded
        assert!(!is_curious(30, 50));
    }

    #[test]
    fn test_non_curious_fraction() {
        // 12/24 = 1/2 ≠ 1/4 (cancelling 2s gives wrong answer)
        assert!(!is_curious(12, 24));
        // 13/24 has no shared digits
        assert!(!is_curious(13, 24));
    }

    #[test]
    fn test_all_four_fractions() {
        let frs = find_curious_fractions();
        assert_eq!(frs, vec![(16, 64), (19, 95), (26, 65), (49, 98)]);
    }

    #[test]
    fn test_solve() {
        // Product: (16·19·26·49) / (64·95·65·98) = 1/100
        assert_eq!(solve(), 100);
    }
}
