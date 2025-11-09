use anyhow::{Context, Result};

/// Parse input into a 2D grid of characters
/// 
/// Internal function used by solve_part1 and solve_part2.
/// Creates a Vec<Vec<char>> for efficient 2D indexing and bounds checking.
fn parse_grid(input: &str) -> Result<Vec<Vec<char>>> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();
    
    if grid.is_empty() {
        anyhow::bail!("Input grid is empty");
    }
    
    // Verify all rows have the same length (rectangular grid)
    let expected_width = grid[0].len();
    for (row_idx, row) in grid.iter().enumerate() {
        if row.len() != expected_width {
            anyhow::bail!("Grid row {} has length {}, expected {}", 
                         row_idx, row.len(), expected_width);
        }
    }
    
    Ok(grid)
}

/// Check if "XMAS" can be found starting at (row, col) in the given direction
/// 
/// Returns true if the complete word "XMAS" is found in bounds.
/// Direction is specified as (row_delta, col_delta) where:
/// - (-1, -1) = diagonal up-left
/// - (-1, 0)  = straight up  
/// - (-1, 1)  = diagonal up-right
/// - (0, -1)  = straight left
/// - (0, 1)   = straight right
/// - (1, -1)  = diagonal down-left
/// - (1, 0)   = straight down
/// - (1, 1)   = diagonal down-right
fn check_xmas_direction(grid: &[Vec<char>], start_row: usize, start_col: usize, 
                       row_delta: i32, col_delta: i32) -> bool {
    let target = ['X', 'M', 'A', 'S'];
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    
    // Check all 4 characters of "XMAS"
    for (i, &expected_char) in target.iter().enumerate() {
        let row = start_row as i32 + (i as i32) * row_delta;
        let col = start_col as i32 + (i as i32) * col_delta;
        
        // Bounds check
        if row < 0 || row >= rows || col < 0 || col >= cols {
            return false;
        }
        
        // Character match check
        if grid[row as usize][col as usize] != expected_char {
            return false;
        }
    }
    
    true
}

/// Check if there's an X-MAS pattern centered at (row, col)
/// 
/// An X-MAS consists of two "MAS" words arranged in an X shape:
/// - One MAS diagonal (top-left to bottom-right or bottom-right to top-left)
/// - One MAS diagonal (top-right to bottom-left or bottom-left to top-right)
/// - Both diagonals intersect at 'A'
/// 
/// Example valid patterns:
/// M.S    S.M    M.M    S.S
/// .A.    .A.    .A.    .A.
/// M.S    S.M    S.S    M.M
fn check_x_mas_pattern(grid: &[Vec<char>], center_row: usize, center_col: usize) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();
    
    // Must be 'A' at center and have space for diagonals
    if center_row == 0 || center_row >= rows - 1 || 
       center_col == 0 || center_col >= cols - 1 {
        return false;
    }
    
    if grid[center_row][center_col] != 'A' {
        return false;
    }
    
    // Get the four diagonal positions
    let top_left = grid[center_row - 1][center_col - 1];
    let top_right = grid[center_row - 1][center_col + 1];
    let bottom_left = grid[center_row + 1][center_col - 1];
    let bottom_right = grid[center_row + 1][center_col + 1];
    
    // Check first diagonal (top-left to bottom-right)
    let diagonal1_mas = (top_left == 'M' && bottom_right == 'S') ||
                        (top_left == 'S' && bottom_right == 'M');
    
    // Check second diagonal (top-right to bottom-left)  
    let diagonal2_mas = (top_right == 'M' && bottom_left == 'S') ||
                        (top_right == 'S' && bottom_left == 'M');
    
    diagonal1_mas && diagonal2_mas
}

/// Part 1: Count all occurrences of "XMAS" in any direction
/// 
/// Searches for "XMAS" in all 8 directions from every position.
/// Includes horizontal, vertical, and diagonal searches, both forwards and backwards.
pub fn solve_part1(input: &str) -> Result<String> {
    let grid = parse_grid(input)
        .context("Failed to parse grid for Part 1")?;
    
    let rows = grid.len();
    let cols = grid[0].len();
    
    // All 8 directions: up, down, left, right, and 4 diagonals
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),  // Up-left, Up, Up-right
        (0, -1),           (0, 1),    // Left, Right  
        (1, -1),  (1, 0),  (1, 1),   // Down-left, Down, Down-right
    ];
    
    let mut count = 0;
    
    // Check every position as a potential start of "XMAS"
    for row in 0..rows {
        for col in 0..cols {
            // Try all 8 directions from this position
            for &(row_delta, col_delta) in &directions {
                if check_xmas_direction(&grid, row, col, row_delta, col_delta) {
                    count += 1;
                }
            }
        }
    }
    
    Ok(count.to_string())
}

/// Part 2: Count all X-MAS patterns 
/// 
/// Finds X-shaped patterns where two "MAS" words intersect at their 'A'.
/// Each MAS can be forwards or backwards along its diagonal.
pub fn solve_part2(input: &str) -> Result<String> {
    let grid = parse_grid(input)
        .context("Failed to parse grid for Part 2")?;
    
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    
    // Check every interior position as potential center of X-MAS
    // Skip edges since X-MAS needs space for diagonals
    for row in 1..rows.saturating_sub(1) {
        for col in 1..cols.saturating_sub(1) {
            if check_x_mas_pattern(&grid, row, col) {
                count += 1;
            }
        }
    }
    
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    const SIMPLE_HORIZONTAL: &str = r#"XMAS
XMAS
XMAS
XMAS"#;

    const SIMPLE_VERTICAL: &str = r#"XMXM
MAXM
ASAX
SMAS"#;

    const SIMPLE_X_MAS: &str = r#"M.S
.A.
M.S"#;

    #[test]
    fn test_parse_grid() {
        let grid = parse_grid("ABC\nDEF\nGHI").unwrap();
        assert_eq!(grid.len(), 3);
        assert_eq!(grid[0], vec!['A', 'B', 'C']);
        assert_eq!(grid[1], vec!['D', 'E', 'F']);
        assert_eq!(grid[2], vec!['G', 'H', 'I']);
    }

    #[test]
    fn test_parse_grid_uneven_rows() {
        let result = parse_grid("ABC\nDE\nGHI");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected 3"));
    }

    #[test]
    fn test_check_xmas_direction_horizontal() {
        let grid = parse_grid("XMAS").unwrap();
        
        // Forward direction (left to right)
        assert!(check_xmas_direction(&grid, 0, 0, 0, 1));
        
        // Should not find XMAS starting from position 1
        assert!(!check_xmas_direction(&grid, 0, 1, 0, 1));
    }

    #[test]
    fn test_check_xmas_direction_backwards() {
        let grid = parse_grid("SAMX").unwrap();
        
        // Backwards direction (right to left, starting from S)
        assert!(check_xmas_direction(&grid, 0, 3, 0, -1));
    }

    #[test]
    fn test_check_xmas_direction_vertical() {
        let grid = parse_grid(SIMPLE_VERTICAL).unwrap();
        
        // Vertical down from top-left (X-M-A-S in column 0)
        assert!(check_xmas_direction(&grid, 0, 0, 1, 0));
        
        // Test that we can detect when there's no XMAS pattern
        assert!(!check_xmas_direction(&grid, 0, 1, 1, 0)); // Column 1 doesn't have XMAS
    }

    #[test]
    fn test_check_x_mas_pattern_simple() {
        let grid = parse_grid(SIMPLE_X_MAS).unwrap();
        
        // Center should be at (1, 1) - the 'A'
        assert!(check_x_mas_pattern(&grid, 1, 1));
        
        // Other positions should not work
        assert!(!check_x_mas_pattern(&grid, 0, 0));
        assert!(!check_x_mas_pattern(&grid, 0, 1));
    }

    #[test]
    fn test_check_x_mas_pattern_variants() {
        // Test different valid X-MAS patterns
        let patterns = [
            "M.S\n.A.\nM.S", // M-A-S both ways
            "S.M\n.A.\nS.M", // S-A-M both ways  
            "M.M\n.A.\nS.S", // M-A-S one way, S-A-M the other
            "S.S\n.A.\nM.M", // S-A-M one way, M-A-S the other
        ];
        
        for pattern in &patterns {
            let grid = parse_grid(pattern).unwrap();
            assert!(check_x_mas_pattern(&grid, 1, 1), 
                   "Pattern should be valid:\n{}", pattern);
        }
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "18");
    }

    #[test]
    fn test_part1_simple_horizontal() {
        let result = solve_part1(SIMPLE_HORIZONTAL).unwrap();
        // Each row is "XMAS", so we find:
        // - 4 times XMAS going left-to-right (horizontal)
        // - 2 times XMAS going top-to-bottom (vertical in columns 0 and 1)
        // Total: 6 occurrences
        assert_eq!(result, "6");
    }

    #[test]
    fn test_part1_simple_vertical() {
        let result = solve_part1(SIMPLE_VERTICAL).unwrap();
        // Should find XMAS patterns in various directions
        // Let's just verify it finds some patterns
        let count: i32 = result.parse().unwrap();
        assert!(count > 0);
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "9");
    }

    #[test]
    fn test_part2_simple() {
        let result = solve_part2(SIMPLE_X_MAS).unwrap();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_empty_grid() {
        let result = parse_grid("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[test]
    fn test_bounds_checking() {
        let grid = parse_grid("XM").unwrap(); // Too short for XMAS
        
        // Should not find XMAS when it goes out of bounds
        assert!(!check_xmas_direction(&grid, 0, 0, 0, 1));
    }

    #[test]
    fn test_x_mas_bounds_checking() {
        let grid = parse_grid("A").unwrap(); // Single character, no room for X pattern
        
        // Should not find X-MAS when there's no room for diagonals
        assert!(!check_x_mas_pattern(&grid, 0, 0));
    }
}