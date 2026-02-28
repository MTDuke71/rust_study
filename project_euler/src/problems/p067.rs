//! # Problem 67: Maximum Path Sum II
//!
//! By starting at the top of the triangle in the file and moving to adjacent
//! numbers on the row below, find the maximum total from top to bottom.
//!
//! ## Same Problem, Larger Scale
//!
//! This is identical to Problem 18, but with a 100-row triangle loaded from
//! a file. Brute force (2^99 ≈ 10^30 paths) is impossible — the same bottom-up
//! DP from Problem 18 solves it in O(n²) time.
//!
//! ## Reuse
//!
//! Uses `p018::parse_triangle` and `p018::max_path_sum` directly —
//! the integrator approach: compose validated components.
//!
//! See [[math-foundations/project-euler-p018]] for the full DP analysis.
//!
//! ## Complexity
//!
//! - Time: O(n²) where n = 100 rows → 5,050 operations
//! - Space: O(n²) for the triangle (modified in-place)

use super::p018::{max_path_sum, parse_triangle};

const TRIANGLE_INPUT: &str =
    include_str!("../../Problem_Statements/0067_triangle.txt");

/// Solve Problem 67: maximum path sum through the 100-row triangle.
pub fn solve() -> u64 {
    let mut triangle = parse_triangle(TRIANGLE_INPUT);
    max_path_sum(&mut triangle) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_shape() {
        let t = parse_triangle(TRIANGLE_INPUT);
        assert_eq!(t.len(), 100);
        for (i, row) in t.iter().enumerate() {
            assert_eq!(row.len(), i + 1, "Row {i} has wrong length");
        }
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 7273);
    }
}
