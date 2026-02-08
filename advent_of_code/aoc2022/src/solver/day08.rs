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

/// Check if tree at (row, col) is visible from any direction.
fn is_visible(grid: &Grid, row: usize, col: usize) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();
    let height = grid[row][col];
    
    // Edge trees always visible
    if row == 0 || row == rows - 1 || col == 0 || col == cols - 1 {
        return true;
    }
    
    // Check left: all trees in row[0..col] shorter?
    let visible_left = (0..col).all(|c| grid[row][c] < height);
    if visible_left {
        return true;
    }
    
    // Check right: all trees in row[col+1..] shorter?
    let visible_right = (col + 1..cols).all(|c| grid[row][c] < height);
    if visible_right {
        return true;
    }
    
    // Check up: all trees in col[0..row] shorter?
    let visible_up = (0..row).all(|r| grid[r][col] < height);
    if visible_up {
        return true;
    }
    
    // Check down: all trees in col[row+1..] shorter?
    (row + 1..rows).all(|r| grid[r][col] < height)
}

/// **Part 1**: Count visible trees.
fn solve_part1(grid: &Grid) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    
    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if is_visible(grid, row, col) {
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
/// **Optimization**: Process trees from center outward (by min edge distance).
/// Skip outer rings when theoretical maximum score ≤ current best.
///
/// For a tree at distance `d` from nearest edge in a 99×99 grid:
/// - Theoretical max score = d² × (98-d)²
/// - Example: d=8 → max = 518,400 (below actual answer 535,680)
/// - Can skip all rings d ≤ 8 once we find a score > 518,400
fn solve_part2(grid: &Grid) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    
    // Group trees by minimum edge distance (ring number)
    let max_dist = (rows / 2).min(cols / 2);
    let mut rings: Vec<Vec<(usize, usize)>> = vec![Vec::new(); max_dist + 1];
    
    for row in 0..rows {
        for col in 0..cols {
            let min_dist = row.min(col).min(rows - 1 - row).min(cols - 1 - col);
            rings[min_dist].push((row, col));
        }
    }
    
    // Process from center outward (decreasing distance)
    let mut max_score = 0;
    
    for dist in (0..=max_dist).rev() {
        // Theoretical maximum for trees at this distance from edge
        // In worst case: viewing distances are [dist, dist, rows-1-dist, cols-1-dist]
        let theo_max = dist * dist * (rows - 1 - dist) * (cols - 1 - dist);
        
        // Early exit: remaining rings can't beat current best
        if theo_max <= max_score {
            break;
        }
        
        // Check all trees in this ring
        for &(row, col) in &rings[dist] {
            let score = scenic_score(grid, row, col);
            max_score = max_score.max(score);
        }
    }
    
    max_score
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
    fn test_visibility_edge_trees() {
        let grid = parse_grid(EXAMPLE);
        // All edge trees should be visible
        assert!(is_visible(&grid, 0, 0)); // top-left
        assert!(is_visible(&grid, 0, 4)); // top-right
        assert!(is_visible(&grid, 4, 0)); // bottom-left
        assert!(is_visible(&grid, 4, 4)); // bottom-right
        assert!(is_visible(&grid, 2, 0)); // left edge
        assert!(is_visible(&grid, 0, 2)); // top edge
    }

    #[test]
    fn test_visibility_interior() {
        let grid = parse_grid(EXAMPLE);
        // Interior tree examples from problem:
        // Row 1, Col 1: top-left 5 - visible from left and top
        assert!(is_visible(&grid, 1, 1));
        // Row 1, Col 2: top-middle 5 - visible from top and right
        assert!(is_visible(&grid, 1, 2));
        // Row 1, Col 3: top-right 1 - NOT visible
        assert!(!is_visible(&grid, 1, 3));
        // Row 2, Col 1: left-middle 5 - visible from right
        assert!(is_visible(&grid, 2, 1));
        // Row 2, Col 2: center 3 - NOT visible
        assert!(!is_visible(&grid, 2, 2));
        // Row 2, Col 3: right-middle 3 - visible from right
        assert!(is_visible(&grid, 2, 3));
        // Row 3, Col 2: bottom-middle 5 - visible
        assert!(is_visible(&grid, 3, 2));
        // Row 3, Col 1: 3 - NOT visible
        assert!(!is_visible(&grid, 3, 1));
        // Row 3, Col 3: 4 - NOT visible
        assert!(!is_visible(&grid, 3, 3));
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
