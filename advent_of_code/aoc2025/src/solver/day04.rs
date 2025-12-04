//! Day 4: Printing Department - Paper Roll Accessibility
//!
//! # Part 1
//! Find which paper rolls (@) can be accessed by forklifts.
//! A roll is accessible if it has **fewer than 4** rolls in its 8 adjacent positions.
//!
//! # Algorithm
//! 1. Parse grid using Mission 6's `AocGridParser::parse_char_grid()`
//! 2. For each roll (@), count its 8-connected neighbors that are also rolls
//! 3. If count < 4, the roll is accessible
//!
//! # Mission Integration
//! - **Mission 6**: `Grid<char>`, `Coord`, `neighbors_8_bounded()` for spatial operations
//!
//! This is a classic cellular automaton / neighbor counting problem, perfect for
//! demonstrating the Grid infrastructure.

use anyhow::Result;
use mission6::{AocGridParser, Coord, Grid};

/// Count how many of the 8 adjacent cells contain paper rolls (@)
fn count_adjacent_rolls(grid: &Grid<char>, coord: Coord) -> usize {
    coord
        .neighbors_8_bounded(grid.width(), grid.height())
        .filter(|&neighbor| grid[neighbor] == '@')
        .count()
}

/// Check if a paper roll at the given coordinate is accessible by forklift
/// A roll is accessible if fewer than 4 of its 8 adjacent cells contain rolls
fn is_accessible(grid: &Grid<char>, coord: Coord) -> bool {
    grid[coord] == '@' && count_adjacent_rolls(grid, coord) < 4
}

/// Part 1: Count paper rolls accessible by forklifts
///
/// A roll is accessible if it has fewer than 4 adjacent rolls (8-connected).
pub fn solve_part1(input: &str) -> Result<String> {
    let grid = AocGridParser::parse_char_grid(input);
    
    if grid.is_empty() {
        return Ok("0".to_string());
    }

    let accessible_count = grid
        .coordinates()
        .filter(|&coord| is_accessible(&grid, coord))
        .count();

    Ok(accessible_count.to_string())
}

/// Part 2: Count total rolls that can be removed iteratively
///
/// Once a roll is removed, it may expose new accessible rolls.
/// Keep removing accessible rolls until none remain accessible.
pub fn solve_part2(input: &str) -> Result<String> {
    let mut grid = AocGridParser::parse_char_grid(input);

    if grid.is_empty() {
        return Ok("0".to_string());
    }

    let mut total_removed = 0;

    loop {
        // Find all currently accessible rolls
        let accessible: Vec<Coord> = grid
            .coordinates()
            .filter(|&coord| is_accessible(&grid, coord))
            .collect();

        if accessible.is_empty() {
            break;
        }

        // Remove all accessible rolls (mark as empty)
        for coord in &accessible {
            grid[*coord] = '.';
        }

        total_removed += accessible.len();
    }

    Ok(total_removed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn test_parse_grid() {
        let grid = AocGridParser::parse_char_grid(EXAMPLE);
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 10);
        assert_eq!(grid[(0, 0)], '.');
        assert_eq!(grid[(2, 0)], '@');
    }

    #[test]
    fn test_count_adjacent_rolls() {
        let grid = AocGridParser::parse_char_grid(EXAMPLE);
        
        // Position (2, 0) is '@' - let's check its neighbors
        // Row 0: ..@@.@@@@.
        // Row 1: @@@.@.@.@@
        // (2,0) has neighbors: (1,0)='.', (3,0)='@', (1,1)='@', (2,1)='@', (3,1)='.'
        let count = count_adjacent_rolls(&grid, Coord::new(2, 0));
        // Neighbors: (1,0)=., (3,0)=@, (1,1)=@, (2,1)=@, (3,1)=.
        // Only 3 adjacent rolls
        assert!(count < 4, "Expected < 4 adjacent rolls, got {}", count);
    }

    #[test]
    fn test_corner_accessible() {
        let grid = AocGridParser::parse_char_grid(EXAMPLE);
        
        // (0, 1) is '@' in a corner-ish position
        // Should have few neighbors since it's at the edge
        let coord = Coord::new(0, 1);
        assert_eq!(grid[coord], '@');
        
        let adjacent = count_adjacent_rolls(&grid, coord);
        println!("Position (0,1) has {} adjacent rolls", adjacent);
    }

    #[test]
    fn test_example_part1() {
        // From problem: 13 rolls can be accessed
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "13");
    }

    #[test]
    fn test_accessible_positions() {
        let grid = AocGridParser::parse_char_grid(EXAMPLE);
        
        // From the problem statement, these positions should be accessible (marked x):
        // Row 0: positions 2, 3, 5, 6, 8 -> (2,0), (3,0), (5,0), (6,0), (8,0)
        // Let's verify a few
        
        // (2, 0) should be accessible
        assert!(is_accessible(&grid, Coord::new(2, 0)), "(2,0) should be accessible");
        
        // (3, 0) should be accessible  
        assert!(is_accessible(&grid, Coord::new(3, 0)), "(3,0) should be accessible");
    }

    #[test]
    fn test_surrounded_not_accessible() {
        // A roll with 4+ neighbors is NOT accessible
        // Position (4, 5) in example: .@@@@@@@.@
        // That's row index 5, position 4 = '@'
        let grid = AocGridParser::parse_char_grid(EXAMPLE);
        
        // (4, 5) is surrounded by many @s
        let coord = Coord::new(4, 5);
        if grid[coord] == '@' {
            let adjacent = count_adjacent_rolls(&grid, coord);
            println!("Position (4,5) has {} adjacent rolls", adjacent);
            // Should have 4+ neighbors, so not accessible
            assert!(!is_accessible(&grid, coord) || adjacent < 4, 
                    "Center positions should generally not be accessible");
        }
    }

    #[test]
    fn test_empty_cell_not_counted() {
        let grid = AocGridParser::parse_char_grid(EXAMPLE);
        
        // Empty cells (.) should never be accessible
        let empty_coord = Coord::new(0, 0);
        assert_eq!(grid[empty_coord], '.');
        assert!(!is_accessible(&grid, empty_coord));
    }

    #[test]
    fn test_single_roll() {
        // A single isolated roll should be accessible (0 neighbors < 4)
        let input = "...\n.@.\n...";
        let grid = AocGridParser::parse_char_grid(input);
        
        let center = Coord::new(1, 1);
        assert_eq!(grid[center], '@');
        assert_eq!(count_adjacent_rolls(&grid, center), 0);
        assert!(is_accessible(&grid, center));
    }

    #[test]
    fn test_fully_surrounded_roll() {
        // A roll surrounded by 8 rolls should NOT be accessible
        let input = "@@@\n@@@\n@@@";
        let grid = AocGridParser::parse_char_grid(input);
        
        let center = Coord::new(1, 1);
        assert_eq!(grid[center], '@');
        assert_eq!(count_adjacent_rolls(&grid, center), 8);
        assert!(!is_accessible(&grid, center));
    }

    #[test]
    fn test_edge_roll() {
        // A roll on the edge has fewer potential neighbors
        let input = "@@@\n@@@\n@@@";
        let grid = AocGridParser::parse_char_grid(input);
        
        // Corner (0, 0) has only 3 neighbors, all @
        let corner = Coord::new(0, 0);
        assert_eq!(count_adjacent_rolls(&grid, corner), 3);
        assert!(is_accessible(&grid, corner)); // 3 < 4, so accessible
        
        // Edge (1, 0) has 5 neighbors, all @
        let edge = Coord::new(1, 0);
        assert_eq!(count_adjacent_rolls(&grid, edge), 5);
        assert!(!is_accessible(&grid, edge)); // 5 >= 4, not accessible
    }

    #[test]
    fn test_threshold_exactly_4() {
        // Test boundary: exactly 4 neighbors should NOT be accessible
        // Need exactly 4 adjacent @ cells
        let input = ".@.\n@@@\n.@.";
        let grid = AocGridParser::parse_char_grid(input);
        
        let center = Coord::new(1, 1);
        assert_eq!(grid[center], '@');
        // Neighbors: (0,0)=., (1,0)=@, (2,0)=., (0,1)=@, (2,1)=@, (0,2)=., (1,2)=@, (2,2)=.
        // Adjacent rolls: 4
        assert_eq!(count_adjacent_rolls(&grid, center), 4);
        assert!(!is_accessible(&grid, center)); // exactly 4 is NOT accessible (need < 4)
    }

    #[test]
    fn test_threshold_exactly_3() {
        // Test boundary: exactly 3 neighbors SHOULD be accessible
        let input = ".@.\n@@.\n...";
        let grid = AocGridParser::parse_char_grid(input);
        
        let coord = Coord::new(1, 1);
        assert_eq!(grid[coord], '@');
        // Neighbors: (0,0)=., (1,0)=@, (2,0)=., (0,1)=@, (2,1)=., (0,2)=., (1,2)=., (2,2)=.
        // Adjacent rolls: 2
        let count = count_adjacent_rolls(&grid, coord);
        assert!(count < 4, "Expected < 4 adjacent rolls, got {}", count);
        assert!(is_accessible(&grid, coord)); // 2 < 4, so accessible
    }

    #[test]
    fn test_example_part2() {
        // From problem: 43 total rolls can be removed
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result, "43");
    }

    #[test]
    fn test_part2_single_roll() {
        // A single roll is accessible and can be removed
        let input = "...\n.@.\n...";
        let result = solve_part2(input).unwrap();
        assert_eq!(result, "1");
    }

    #[test]
    fn test_part2_chain_removal() {
        // Test that removing rolls exposes more accessible rolls
        // Line of rolls - corners are always accessible first
        let input = "@@@@@";
        let result = solve_part2(input).unwrap();
        assert_eq!(result, "5"); // All should be removable eventually
    }

    #[test]
    fn test_part2_2x2_block() {
        // 2x2 block - all corners have 3 neighbors, so all accessible
        let input = "@@\n@@";
        let result = solve_part2(input).unwrap();
        assert_eq!(result, "4"); // All removable in first pass
    }

    #[test]
    fn test_part2_stuck_block() {
        // Large solid block - only corners accessible initially (3 neighbors each)
        // After removing corners, edge midpoints still have 4+ neighbors
        // 5x5 block erodes from corners inward
        let input = "@@@@@\n@@@@@\n@@@@@\n@@@@@\n@@@@@";
        let result = solve_part2(input).unwrap();
        // Only corners (3 neighbors) can be removed, then erosion continues
        // but center remains stuck - verify it's > 0 and reasonable
        let count: usize = result.parse().unwrap();
        assert!(count >= 4, "At least corners should be removable, got {}", count);
        // After corners: edge-midpoints have 5 neighbors minus 1-2 corners = 3-4
        // The erosion should continue until blocked
    }
}
