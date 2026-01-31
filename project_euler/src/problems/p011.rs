//! # Problem 11: Largest Product in a Grid
//!
//! Find the greatest product of four adjacent numbers in the same direction
//! (up, down, left, right, or diagonally) in a 20×20 grid.
//!
//! ## Diagonal Pattern Explanation
//!
//! Visual guide for checking diagonals:
//!
//! ```text
//! Grid indices (row, col):
//!     0   1   2   3
//! 0  [A] [B] [C] [D]
//! 1  [E] [F] [G] [H]
//! 2  [I] [J] [K] [L]
//! 3  [M] [N] [O] [P]
//!
//! Diagonal Down-Right (↘) from A:
//!   A(0,0) → F(1,1) → K(2,2) → P(3,3)
//!   Pattern: row+i, col+i
//!
//! Diagonal Down-Left (↙) from D:
//!   D(0,3) → G(1,2) → J(2,1) → M(3,0)
//!   Pattern: row+i, col-i
//! ```
//!
//! ## Approach
//!
//! For each valid starting position, check all 4 directions:
//! - **Horizontal (→):** row, col+i
//! - **Vertical (↓):** row+i, col
//! - **Diagonal down-right (↘):** row+i, col+i
//! - **Diagonal down-left (↙):** row+i, col-i
//!
//! ## Complexity
//!
//! - Time: O(n²) where n is grid size (20×20 = 400 cells, each checks up to 4 directions)
//! - Space: O(n²) to store the grid

const GRID_DATA: &str = "\
08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";

/// Parse the grid data into a 2D vector
fn parse_grid(data: &str) -> Vec<Vec<u32>> {
    data.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

/// Calculate product of 4 numbers at given position and direction
fn product_at(grid: &[Vec<u32>], row: usize, col: usize, dir: Direction) -> Option<u64> {
    let rows = grid.len();
    let cols = grid[0].len();
    
    // Check bounds for all 4 positions
    match dir {
        Direction::Horizontal => {
            if col + 3 < cols {
                Some(
                    grid[row][col] as u64
                        * grid[row][col + 1] as u64
                        * grid[row][col + 2] as u64
                        * grid[row][col + 3] as u64
                )
            } else {
                None
            }
        }
        Direction::Vertical => {
            if row + 3 < rows {
                Some(
                    grid[row][col] as u64
                        * grid[row + 1][col] as u64
                        * grid[row + 2][col] as u64
                        * grid[row + 3][col] as u64
                )
            } else {
                None
            }
        }
        Direction::DiagonalDownRight => {
            if row + 3 < rows && col + 3 < cols {
                Some(
                    grid[row][col] as u64
                        * grid[row + 1][col + 1] as u64
                        * grid[row + 2][col + 2] as u64
                        * grid[row + 3][col + 3] as u64
                )
            } else {
                None
            }
        }
        Direction::DiagonalDownLeft => {
            if row + 3 < rows && col >= 3 {
                Some(
                    grid[row][col] as u64
                        * grid[row + 1][col - 1] as u64
                        * grid[row + 2][col - 2] as u64
                        * grid[row + 3][col - 3] as u64
                )
            } else {
                None
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Horizontal,
    Vertical,
    DiagonalDownRight,
    DiagonalDownLeft,
}

/// Find the largest product of 4 adjacent numbers in any direction
///
/// # Examples
/// ```
/// use project_euler::problems::p011::largest_product_in_grid;
/// 
/// let grid = vec![
///     vec![1, 2, 3, 4],
///     vec![5, 6, 7, 8],
///     vec![9, 10, 11, 12],
///     vec![13, 14, 15, 16],
/// ];
/// 
/// let product = largest_product_in_grid(&grid);
/// // Horizontal from (3,0): 13*14*15*16 = 43,680
/// assert_eq!(product, 43680);
/// ```
pub fn largest_product_in_grid(grid: &[Vec<u32>]) -> u64 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut max_product = 0u64;
    
    let directions = [
        Direction::Horizontal,
        Direction::Vertical,
        Direction::DiagonalDownRight,
        Direction::DiagonalDownLeft,
    ];
    
    for row in 0..rows {
        for col in 0..cols {
            for &dir in &directions {
                if let Some(product) = product_at(grid, row, col, dir) {
                    max_product = max_product.max(product);
                }
            }
        }
    }
    
    max_product
}

/// Solve Problem 11: Find largest product in the 20×20 grid
pub fn solve() -> u64 {
    let grid = parse_grid(GRID_DATA);
    largest_product_in_grid(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid() {
        let grid = parse_grid(GRID_DATA);
        assert_eq!(grid.len(), 20);
        assert_eq!(grid[0].len(), 20);
        assert_eq!(grid[0][0], 8);
        assert_eq!(grid[19][19], 48);
    }

    #[test]
    fn test_horizontal_product() {
        let grid = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
        ];
        let product = product_at(&grid, 0, 0, Direction::Horizontal);
        assert_eq!(product, Some(2 * 3 * 4));
        
        let product = product_at(&grid, 0, 1, Direction::Horizontal);
        assert_eq!(product, Some(2 * 3 * 4 * 5));
        
        // Out of bounds
        let product = product_at(&grid, 0, 2, Direction::Horizontal);
        assert_eq!(product, None);
    }

    #[test]
    fn test_vertical_product() {
        let grid = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
            vec![7, 8],
        ];
        let product = product_at(&grid, 0, 0, Direction::Vertical);
        assert_eq!(product, Some(3 * 5 * 7));
        
        // Out of bounds
        let product = product_at(&grid, 1, 0, Direction::Vertical);
        assert_eq!(product, None);
    }

    #[test]
    fn test_diagonal_down_right() {
        let grid = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let product = product_at(&grid, 0, 0, Direction::DiagonalDownRight);
        assert_eq!(product, Some(6 * 11 * 16));
    }

    #[test]
    fn test_diagonal_down_left() {
        let grid = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let product = product_at(&grid, 0, 3, Direction::DiagonalDownLeft);
        assert_eq!(product, Some(4 * 7 * 10 * 13));
    }

    #[test]
    fn test_largest_product_small_grid() {
        let grid = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let max_product = largest_product_in_grid(&grid);
        // Horizontal from (3,0): 13*14*15*16 = 43,680
        assert_eq!(max_product, 43680);
    }

    #[test]
    fn test_solution() {
        let result = solve();
        println!("Problem 11 solution: {}", result);
        assert!(result > 0);
    }
}
