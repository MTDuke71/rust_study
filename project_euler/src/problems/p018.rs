//! # Problem 18: Maximum Path Sum I
//!
//! By starting at the top of the triangle below and moving to adjacent
//! numbers on the row below, find the maximum total from top to bottom.
//!
//! Example (4 rows, answer = 23):
//! ```text
//!    3
//!   7 4
//!  2 4 6
//! 8 5 9 3
//! ```
//! Path: 3 → 7 → 4 → 9 = 23
//!
//! ## Mathematical Foundation
//!
//! The triangle is a **DAG (Directed Acyclic Graph)** where each node
//! `(row, col)` has two children: `(row+1, col)` and `(row+1, col+1)`.
//! We want the **maximum weight path** from root to any leaf.
//!
//! ### Why Greedy Fails
//!
//! At row 1, choosing 7 (> 4) looks correct. But consider:
//! ```text
//!    1
//!   2 9
//!  99 1 1
//! ```
//! Greedy picks 9 → 1 = 10. DP picks 2 → 99 = 100. Local best ≠ global best.
//!
//! ### Optimal Substructure
//!
//! Define `best(r, c)` = max path sum from `(r, c)` to any leaf.
//! ```text
//! best(r, c) = T[r][c] + max(best(r+1, c), best(r+1, c+1))
//! base case: best(last_row, c) = T[last_row][c]
//! ```
//! This is classic DP on a DAG. Evaluated bottom-up = O(n²) with O(1) extra space.
//!
//! ### Bottom-Up In-Place Algorithm
//!
//! Process rows from `n-2` down to `0`:
//! ```text
//! T[r][c] += max(T[r+1][c], T[r+1][c+1])
//! ```
//! After all rows, `T[0][0]` contains the answer.
//!
//! See [[math-foundations/project-euler-p018]] for full analysis.
//!
//! ## Complexity
//!
//! - Time: O(n²) where n = number of rows
//! - Space: O(n²) for the triangle (O(1) extra — modified in-place)

/// Parse a whitespace-separated triangle string into a `Vec<Vec<u32>>`.
///
/// # Examples
/// ```
/// use project_euler::problems::p018::parse_triangle;
/// let t = parse_triangle("3\n7 4\n2 4 6\n8 5 9 3");
/// assert_eq!(t.len(), 4);
/// assert_eq!(t[0], vec![3]);
/// assert_eq!(t[3], vec![8, 5, 9, 3]);
/// ```
pub fn parse_triangle(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

/// Find the maximum path sum from top to bottom of a triangle using
/// bottom-up dynamic programming.
///
/// Modifies the triangle in-place: each cell becomes the max path sum
/// through that cell. Returns `triangle[0][0]` after propagation.
///
/// # Examples
/// ```
/// use project_euler::problems::p018::max_path_sum;
/// // 3 → 7 → 4 → 9 = 23
/// let mut t: Vec<Vec<u32>> = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
/// assert_eq!(max_path_sum(&mut t), 23);
/// ```
pub fn max_path_sum(triangle: &mut [Vec<u32>]) -> u32 {
    let n = triangle.len();
    // Walk from second-to-last row up to the top
    for row in (0..n - 1).rev() {
        for col in 0..triangle[row].len() {
            let best_child = triangle[row + 1][col].max(triangle[row + 1][col + 1]);
            triangle[row][col] += best_child;
        }
    }
    triangle[0][0]
}

/// Flatten a parsed triangle into a contiguous 1D slice, row by row.
///
/// The flat layout is used by `brute_force_max_path`. Row `r` (0-indexed)
/// starts at flat index `r*(r+1)/2`.
///
/// # Examples
/// ```
/// use project_euler::problems::p018::flatten_triangle;
/// let t = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
/// assert_eq!(flatten_triangle(&t), vec![3, 7, 4, 2, 4, 6, 8, 5, 9, 3]);
/// ```
pub fn flatten_triangle(triangle: &[Vec<u32>]) -> Vec<u32> {
    triangle.iter().flatten().copied().collect()
}

/// Brute-force maximum path sum by enumerating all 2^(rows−1) paths.
///
/// Port of bitRAKE's x86 assembly bit-manipulation approach. Each path is
/// encoded as `(rows-1)` bits (0 = go left, 1 = go right). The flat index
/// advances using the same trick as the assembly's `adc edx, esi`:
///
/// ```text
/// idx += row + go_right   // row = 1-indexed destination row
/// ```
///
/// Left child of `(r, c)` sits `r+1` cells ahead in flat storage;
/// right child sits `r+2` cells ahead — so the carry bit naturally
/// selects between them.
///
/// **Only practical for small triangles.** P018 (15 rows) has 2^14 = 16,384
/// paths. P067 (100 rows) has 2^99 ≈ 10^30 — completely infeasible.
///
/// # Examples
/// ```
/// use project_euler::problems::p018::{flatten_triangle, brute_force_max_path};
/// let t = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
/// assert_eq!(brute_force_max_path(&flatten_triangle(&t), 4), 23);
/// ```
pub fn brute_force_max_path(flat: &[u32], rows: usize) -> u32 {
    if rows <= 1 {
        return flat.first().copied().unwrap_or(0);
    }
    let num_paths = 1usize << (rows - 1);
    let shift = 32u32 - (rows as u32 - 1); // align top choice bit to bit 31
    let mut best = 0u32;

    for path in 0..num_paths {
        // Mirrors: shl ecx, 32-(rows-1)  — put first choice into MSB
        let mut bits = (path as u32) << shift;
        let mut idx = 0usize;
        let mut sum = flat[0];

        for row in 1..rows {
            // Mirrors: shl ecx, 1  →  carry = direction bit
            let go_right = (bits >> 31) as usize;
            bits <<= 1;
            // Mirrors: adc edx, esi  (esi = row, 1-indexed)
            // Left child  = +row,   right child = +row+1
            idx += row + go_right;
            sum += flat[idx];
        }

        best = best.max(sum);
    }

    best
}

/// The 15-row triangle from Project Euler Problem 18.
pub const TRIANGLE_INPUT: &str = "\
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

/// Solve Problem 18: maximum path sum through the 15-row triangle.
pub fn solve() -> u64 {
    let mut triangle = parse_triangle(TRIANGLE_INPUT);
    max_path_sum(&mut triangle) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_from_problem() {
        // Example given in problem statement: 3 → 7 → 4 → 9 = 23
        let mut t = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
        assert_eq!(max_path_sum(&mut t), 23);
    }

    #[test]
    fn test_greedy_would_fail() {
        // Greedy at row 0 picks 9 (>1), then from 9 picks max(1,1)=1: total = 5+9+1 = 15
        // DP finds 5 → 1 → 100 = 106
        let mut t = vec![vec![5], vec![9, 1], vec![1, 1, 100]];
        assert_eq!(max_path_sum(&mut t), 106);
    }

    #[test]
    fn test_single_row() {
        let mut t = vec![vec![42]];
        assert_eq!(max_path_sum(&mut t), 42);
    }

    #[test]
    fn test_triangle_shape() {
        // Verify embedded data parses to a valid 15-row triangle
        let t = parse_triangle(TRIANGLE_INPUT);
        assert_eq!(t.len(), 15);
        for (i, row) in t.iter().enumerate() {
            assert_eq!(row.len(), i + 1, "Row {i} has wrong length");
        }
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 1074);
    }

    // --- brute force ---

    #[test]
    fn test_brute_force_example() {
        let t = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
        assert_eq!(brute_force_max_path(&flatten_triangle(&t), 4), 23);
    }

    #[test]
    fn test_brute_force_matches_dp() {
        // Both approaches must agree on the full P018 triangle
        let flat = flatten_triangle(&parse_triangle(TRIANGLE_INPUT));
        assert_eq!(brute_force_max_path(&flat, 15), 1074);
    }

    #[test]
    fn test_brute_force_greedy_counterexample() {
        let t = vec![vec![5], vec![9, 1], vec![1, 1, 100]];
        assert_eq!(brute_force_max_path(&flatten_triangle(&t), 3), 106);
    }
}
