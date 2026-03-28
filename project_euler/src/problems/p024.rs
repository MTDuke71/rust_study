//! # Problem 24: Lexicographic Permutations
//!
//! A permutation is an ordered arrangement of objects. The lexicographic
//! permutations of 0, 1 and 2 are: 012, 021, 102, 120, 201, 210.
//!
//! What is the millionth lexicographic permutation of 0–9?
//!
//! ## Mathematical Foundation
//!
//! Uses the **factoradic number system** to directly compute the nth
//! permutation without generating all permutations. Each digit position
//! is determined by dividing the remaining index by (remaining digits - 1)!.
//!
//! Key insight: With n items, each "block" of permutations sharing the same
//! first element contains (n-1)! permutations. So `index / (n-1)!` tells
//! us which element goes first.
//!
//! See [[math-foundations/project-euler-p024]] for full analysis.
//!
//! ## Complexity
//!
//! - Time: O(n²) where n = number of digits (removal from vec is O(n))
//! - Space: O(n)

use crate::utils::combinatorics::factorial;

/// Find the nth lexicographic permutation of digits 0..num_digits (1-indexed).
///
/// # Examples
/// ```
/// use project_euler::problems::p024::nth_permutation;
/// // Permutations of [0,1,2]: 012, 021, 102, 120, 201, 210
/// assert_eq!(nth_permutation(3, 1), vec![0, 1, 2]);
/// assert_eq!(nth_permutation(3, 3), vec![1, 0, 2]);
/// assert_eq!(nth_permutation(3, 6), vec![2, 1, 0]);
/// ```
pub fn nth_permutation(num_digits: u64, n: u64) -> Vec<u64> {
    let mut digits: Vec<u64> = (0..num_digits).collect();
    let mut result = Vec::with_capacity(num_digits as usize);
    let mut remainder = n - 1; // convert to 0-indexed

    for i in (1..num_digits).rev() {
        let fact = factorial(i);
        let index = remainder / fact;
        remainder %= fact;
        result.push(digits.remove(index as usize));
    }
    // Last digit remaining
    result.push(digits[0]);

    result
}

/// Convert a digit sequence to a number string.
///
/// # Examples
/// ```
/// use project_euler::problems::p024::digits_to_string;
/// assert_eq!(digits_to_string(&[2, 7, 8, 3, 9, 1, 5, 4, 6, 0]), "2783915460");
/// ```
pub fn digits_to_string(digits: &[u64]) -> String {
    digits.iter().map(|d| d.to_string()).collect()
}

/// Solve Problem 24: the millionth lexicographic permutation of 0–9.
pub fn solve() -> u64 {
    let digits = nth_permutation(10, 1_000_000);
    // Convert digit sequence to a single number
    digits.iter().fold(0u64, |acc, &d| acc * 10 + d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations_of_012() {
        // All 6 permutations of [0, 1, 2]
        assert_eq!(nth_permutation(3, 1), vec![0, 1, 2]);
        assert_eq!(nth_permutation(3, 2), vec![0, 2, 1]);
        assert_eq!(nth_permutation(3, 3), vec![1, 0, 2]);
        assert_eq!(nth_permutation(3, 4), vec![1, 2, 0]);
        assert_eq!(nth_permutation(3, 5), vec![2, 0, 1]);
        assert_eq!(nth_permutation(3, 6), vec![2, 1, 0]);
    }

    #[test]
    fn test_first_permutation() {
        assert_eq!(nth_permutation(10, 1), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_last_permutation() {
        // 10! = 3,628,800th permutation is the reverse
        assert_eq!(nth_permutation(10, 3_628_800), vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 2783915460);
    }
}
