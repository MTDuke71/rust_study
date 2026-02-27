//! # Problem 15: Lattice Paths
//!
//! Starting in the top left corner of a 2×2 grid, and only being able to
//! move to the right and down, there are exactly 6 routes to the bottom
//! right corner. How many such routes are there through a 20×20 grid?
//!
//! ## Mathematical Foundation
//!
//! Every path through an n×n grid requires exactly n moves right and n
//! moves down — 2n moves total. The number of distinct paths is the
//! number of ways to choose which n of the 2n steps are "right":
//!
//!   C(2n, n) = (2n)! / (n! × n!)
//!
//! This is the **central binomial coefficient**.
//!
//! See [[math-foundations/combinatorics-basics]] for theory.
//!
//! ## Complexity
//!
//! - Time: O(n) — single pass multiplication in `binomial()`
//! - Space: O(1)

use crate::utils::combinatorics;

/// Count lattice paths through an `n × n` grid.
///
/// # Examples
/// ```
/// use project_euler::problems::p015::lattice_paths;
/// assert_eq!(lattice_paths(2), 6);
/// ```
pub fn lattice_paths(n: u64) -> u64 {
    combinatorics::binomial(2 * n, n)
}

/// Solve Problem 15: lattice paths through a 20×20 grid.
pub fn solve() -> u64 {
    lattice_paths(20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_2x2() {
        // Example from problem statement: 6 routes through 2×2 grid
        assert_eq!(lattice_paths(2), 6);
    }

    #[test]
    fn test_1x1() {
        // 1×1 grid: right-down or down-right = 2 paths
        assert_eq!(lattice_paths(1), 2);
    }

    #[test]
    fn test_3x3() {
        // C(6,3) = 20
        assert_eq!(lattice_paths(3), 20);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 137846528820);
    }
}
