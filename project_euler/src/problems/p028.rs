//! # Problem 28: Number Spiral Diagonals
//!
//! Starting with the number 1 and moving to the right in a clockwise direction,
//! a 5 by 5 spiral is formed as follows:
//!
//! ```text
//! 21 22 23 24 25
//! 20  7  8  9 10
//! 19  6  1  2 11
//! 18  5  4  3 12
//! 17 16 15 14 13
//! ```
//!
//! The sum of the diagonals is 101.
//!
//! What is the sum of the numbers on the diagonals of a 1001 by 1001 spiral?
//!
//! ## Mathematical Foundation
//!
//! For an n×n spiral (n odd), layer k (k = 1, 2, ..., (n-1)/2) has side length 2k+1.
//! The four corner values of layer k are:
//! - Top-right: (2k+1)²
//! - Top-left: (2k+1)² − 2k
//! - Bottom-left: (2k+1)² − 4k
//! - Bottom-right: (2k+1)² − 6k
//!
//! Sum of corners at layer k = 4(2k+1)² − 12k
//!
//! **Closed-form derivation:**
//! Let m = (n−1)/2. Total = 1 + Σ_{k=1}^{m} [4(2k+1)² − 12k]
//!
//! Expanding: 4(4k² + 4k + 1) − 12k = 16k² + 4k + 4
//!
//! Using Σk² = m(m+1)(2m+1)/6 and Σk = m(m+1)/2:
//! Total = 1 + 16·m(m+1)(2m+1)/6 + 4·m(m+1)/2 + 4m
//!       = 1 + 8m(m+1)(2m+1)/3 + 2m(m+1) + 4m
//!
//! For n = 1001, m = 500: answer = 669,171,001
//!
//! ## Complexity
//! - Time: O(1) — closed-form formula
//! - Space: O(1)

/// Computes the sum of diagonal elements in an n×n number spiral.
///
/// Uses the closed-form formula derived from corner values at each layer.
///
/// # Arguments
/// * `n` - Side length of the spiral (must be odd and ≥ 1)
///
/// # Examples
/// ```
/// use project_euler::problems::p028::spiral_diagonal_sum;
/// assert_eq!(spiral_diagonal_sum(1), 1);
/// assert_eq!(spiral_diagonal_sum(3), 25);
/// assert_eq!(spiral_diagonal_sum(5), 101);
/// ```
pub fn spiral_diagonal_sum(n: u64) -> u64 {
    if n == 1 {
        return 1;
    }
    let m = (n - 1) / 2;
    // 1 + 8m(m+1)(2m+1)/3 + 2m(m+1) + 4m
    1 + 8 * m * (m + 1) * (2 * m + 1) / 3 + 2 * m * (m + 1) + 4 * m
}

/// Iterative approach: sum corners layer by layer.
///
/// Useful as a verification method and benchmark comparison.
///
/// # Examples
/// ```
/// use project_euler::problems::p028::spiral_diagonal_sum_iterative;
/// assert_eq!(spiral_diagonal_sum_iterative(5), 101);
/// assert_eq!(spiral_diagonal_sum_iterative(1001), 669_171_001);
/// ```
pub fn spiral_diagonal_sum_iterative(n: u64) -> u64 {
    let layers = (n - 1) / 2;
    let mut sum = 1u64;
    for k in 1..=layers {
        let side = 2 * k + 1;
        let top_right = side * side;
        sum += top_right + (top_right - 2 * k) + (top_right - 4 * k) + (top_right - 6 * k);
    }
    sum
}

pub fn solve() -> u64 {
    spiral_diagonal_sum(1001)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_1x1() {
        assert_eq!(spiral_diagonal_sum(1), 1);
    }

    #[test]
    fn test_spiral_3x3() {
        // 1 + 3 + 5 + 7 + 9 = 25
        assert_eq!(spiral_diagonal_sum(3), 25);
    }

    #[test]
    fn test_spiral_5x5() {
        assert_eq!(spiral_diagonal_sum(5), 101);
    }

    #[test]
    fn test_closed_form_matches_iterative() {
        for n in (1..=101).step_by(2) {
            assert_eq!(
                spiral_diagonal_sum(n),
                spiral_diagonal_sum_iterative(n),
                "Mismatch at n={n}"
            );
        }
    }

    #[test]
    fn test_solution() {
        assert_eq!(solve(), 669_171_001);
    }
}
