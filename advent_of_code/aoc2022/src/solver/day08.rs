//! # Day 8: Treetop Tree House
//!
//! **Problem**: Count trees visible from outside the grid.
//!
//! ## Strategy
//!
//! A tree is visible if all trees between it and ANY edge are shorter.
//! Edge trees are always visible.
//!
//! ## Visibility Algorithm
//!
//! For each interior tree at (row, col):
//! 1. Check left: all trees in row[0..col] shorter?
//! 2. Check right: all trees in row[col+1..] shorter?
//! 3. Check up: all trees in col[0..row] shorter?
//! 4. Check down: all trees in col[row+1..] shorter?
//! 5. Visible if ANY direction returns true
//!
//! ## Parse-Once Pattern
//!
//! Parse grid once in `solve()`, pass &[Vec<u8>] to both parts when Part 2 arrives.

use rayon::prelude::*;

type Grid = Vec<Vec<u8>>;

/// Parse input into 2D grid of tree heights.
///
/// Each character '0'-'9' becomes height 0-9.
fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).expect("valid digit") as u8)
                .collect()
        })
        .collect()
}

/// **Part 1**: Count visible trees.
///
/// **Optimization**: Pre-compute maximum heights from each direction.
/// Instead of scanning all 4 directions per tree (O(N³)), do 4 passes
/// to compute max heights, then O(1) visibility check per tree (O(N²) total).
fn solve_part1(grid: &Grid) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    
    // Pre-compute max heights from each direction
    // max_from_left[row][col] = max height from left edge to col-1
    let mut max_from_left = vec![vec![0u8; cols]; rows];
    let mut max_from_right = vec![vec![0u8; cols]; rows];
    let mut max_from_top = vec![vec![0u8; cols]; rows];
    let mut max_from_bottom = vec![vec![0u8; cols]; rows];
    
    // Pass 1: Left to right
    for row in 0..rows {
        let mut max_height = 0;
        for col in 0..cols {
            max_from_left[row][col] = max_height;
            max_height = max_height.max(grid[row][col]);
        }
    }
    
    // Pass 2: Right to left
    for row in 0..rows {
        let mut max_height = 0;
        for col in (0..cols).rev() {
            max_from_right[row][col] = max_height;
            max_height = max_height.max(grid[row][col]);
        }
    }
    
    // Pass 3: Top to bottom
    for col in 0..cols {
        let mut max_height = 0;
        for row in 0..rows {
            max_from_top[row][col] = max_height;
            max_height = max_height.max(grid[row][col]);
        }
    }
    
    // Pass 4: Bottom to top
    for col in 0..cols {
        let mut max_height = 0;
        for row in (0..rows).rev() {
            max_from_bottom[row][col] = max_height;
            max_height = max_height.max(grid[row][col]);
        }
    }
    
    // Count visible trees: visible if height > max from ANY direction
    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            // Edge trees are always visible
            if row == 0 || row == rows - 1 || col == 0 || col == cols - 1 {
                count += 1;
                continue;
            }
            
            // Interior trees: visible if taller than max from ANY direction
            let height = grid[row][col];
            if height > max_from_left[row][col]
                || height > max_from_right[row][col]
                || height > max_from_top[row][col]
                || height > max_from_bottom[row][col]
            {
                count += 1;
            }
        }
    }
    count
}

/// Calculate viewing distance in one direction.
///
/// Counts trees until hitting edge or tree >= height.
/// **Includes** the blocking tree in the count.
fn viewing_distance(grid: &Grid, row: usize, col: usize, dr: isize, dc: isize) -> usize {
    let height = grid[row][col];
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    
    let mut count = 0;
    let mut r = row as isize + dr;
    let mut c = col as isize + dc;
    
    while r >= 0 && r < rows && c >= 0 && c < cols {
        count += 1;
        if grid[r as usize][c as usize] >= height {
            break; // Blocked - but we counted this tree
        }
        r += dr;
        c += dc;
    }
    
    count
}

/// Calculate scenic score for tree at (row, col).
///
/// Scenic score = product of viewing distances in 4 directions.
fn scenic_score(grid: &Grid, row: usize, col: usize) -> usize {
    let up = viewing_distance(grid, row, col, -1, 0); // North
    let down = viewing_distance(grid, row, col, 1, 0); // South
    let left = viewing_distance(grid, row, col, 0, -1); // West
    let right = viewing_distance(grid, row, col, 0, 1); // East
    
    up * down * left * right
}

/// **Part 2**: Find maximum scenic score.
///
/// **Optimization**: Parallel row processing using Rayon.
/// Each row is processed independently, leveraging multiple CPU cores.
fn solve_part2(grid: &Grid) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    
    // Process each row in parallel, find max scenic score per row,
    // then take the global maximum
    (0..rows)
        .into_par_iter()
        .map(|row| {
            (0..cols)
                .map(|col| scenic_score(grid, row, col))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0)
}

/// **Main solver**: Parse once, solve both parts.
pub fn solve(input: &str) -> (usize, usize) {
    let grid = parse_grid(input);
    let part1 = solve_part1(&grid);
    let part2 = solve_part2(&grid);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
30373
25512
65332
33549
35390";

    #[test]
    fn test_parse_grid() {
        let grid = parse_grid(EXAMPLE);
        assert_eq!(grid.len(), 5); // rows
        assert_eq!(grid[0].len(), 5); // cols
        assert_eq!(grid[0][0], 3);
        assert_eq!(grid[0][4], 3);
        assert_eq!(grid[4][4], 0);
    }

    #[test]
    fn test_part1_example() {
        let grid = parse_grid(EXAMPLE);
        assert_eq!(solve_part1(&grid), 21);
    }

    #[test]
    fn test_scenic_score_examples() {
        let grid = parse_grid(EXAMPLE);
        
        // Middle 5 in second row [1][2]: score = 1 * 1 * 2 * 2 = 4
        // up=1 (sees 1 tree), left=1 (blocked immediately), right=2, down=2
        assert_eq!(scenic_score(&grid, 1, 2), 4);
        
        // Middle 5 in fourth row [3][2]: score = 2 * 2 * 1 * 2 = 8
        // up=2 (blocked by height 5), left=2, down=1 (edge), right=2 (blocked by 9)
        assert_eq!(scenic_score(&grid, 3, 2), 8);
    }

    #[test]
    fn test_part2_example() {
        let grid = parse_grid(EXAMPLE);
        assert_eq!(solve_part2(&grid), 8);
    }
}
