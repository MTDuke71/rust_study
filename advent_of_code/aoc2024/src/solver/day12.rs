use anyhow::Result;
use mission6::{Coord, FloodFill, Grid};
use std::collections::HashSet;

/// Parse the input grid of characters.
/// Each character represents a plot type (e.g., 'C', 'K', 'X', etc.)
fn parse_grid(input: &str) -> Result<Vec<Vec<char>>> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        anyhow::bail!("Empty grid");
    }

    // Validate that all rows have the same length
    let expected_width = grid[0].len();
    for (row_idx, row) in grid.iter().enumerate() {
        if row.len() != expected_width {
            anyhow::bail!(
                "Row {} has length {}, expected {}",
                row_idx,
                row.len(),
                expected_width
            );
        }
    }

    Ok(grid)
}

/// Generic function to calculate total cost for all regions in a grid.
///
/// Uses Mission 6's Grid and FloodFill algorithms for region detection,
/// then applies a cost function to each region.
///
/// # Arguments
/// * `grid` - The grid to analyze
/// * `cost_fn` - Function that calculates cost for a region given (area, perimeter, coords)
fn calculate_total_cost<F>(grid: &Grid<char>, cost_fn: F) -> usize
where
    F: Fn(usize, usize, &[Coord]) -> usize,
{
    let mut visited = HashSet::new();
    let mut total_cost = 0;

    // Scan through every cell in the grid
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);

            // Skip if already processed as part of another region
            if visited.contains(&coord) {
                continue;
            }

            // Find this region using Mission 6's flood fill
            let plant_type = grid[coord];
            let region_info = FloodFill::analyze_region_4(grid, coord, &plant_type);

            // Mark all cells in this region as visited
            for &cell in &region_info.filled_coords {
                visited.insert(cell);
            }

            // Calculate cost using the provided function
            let cost = cost_fn(
                region_info.area,
                region_info.perimeter,
                &region_info.filled_coords,
            );
            total_cost += cost;
        }
    }

    total_cost
}

/// Calculate total fencing cost (area × perimeter) for all regions
fn calculate_total_fencing_cost(grid: &Grid<char>) -> usize {
    calculate_total_cost(grid, |area, perimeter, _coords| area * perimeter)
}

/// Count the number of sides (continuous straight fence sections) for a region.
///
/// Key insight: The number of sides equals the number of corners.
/// We check each cell in the region for corner configurations.
fn count_sides(region_coords: &[Coord]) -> usize {
    let region_set: HashSet<_> = region_coords.iter().copied().collect();
    let mut corners = 0;

    for &coord in region_coords {
        let x = coord.x as i32;
        let y = coord.y as i32;

        // Check all 4 possible corner configurations at this cell
        // Each cell can contribute 0-4 corners depending on its neighbors

        // Helper to check if a coordinate is in the region
        let in_region = |dx: i32, dy: i32| -> bool {
            let check_x = x + dx;
            let check_y = y + dy;
            if check_x < 0 || check_y < 0 {
                return false;
            }
            region_set.contains(&Coord::new(check_x as usize, check_y as usize))
        };

        // For each of the 4 corners around this cell, check if it's an outer or inner corner

        // Top-left corner
        let n = in_region(0, -1);  // North
        let w = in_region(-1, 0);  // West
        let nw = in_region(-1, -1); // Northwest

        // Outer corner: neither N nor W is in region
        if !n && !w {
            corners += 1;
        }
        // Inner corner: both N and W are in region, but NW is not
        if n && w && !nw {
            corners += 1;
        }

        // Top-right corner
        let n = in_region(0, -1);  // North
        let e = in_region(1, 0);   // East
        let ne = in_region(1, -1); // Northeast

        if !n && !e {
            corners += 1;
        }
        if n && e && !ne {
            corners += 1;
        }

        // Bottom-left corner
        let s = in_region(0, 1);   // South
        let w = in_region(-1, 0);  // West
        let sw = in_region(-1, 1); // Southwest

        if !s && !w {
            corners += 1;
        }
        if s && w && !sw {
            corners += 1;
        }

        // Bottom-right corner
        let s = in_region(0, 1);   // South
        let e = in_region(1, 0);   // East
        let se = in_region(1, 1);  // Southeast

        if !s && !e {
            corners += 1;
        }
        if s && e && !se {
            corners += 1;
        }
    }

    corners
}

/// Calculate total bulk discount cost (area × sides) for all regions
fn calculate_total_bulk_discount_cost(grid: &Grid<char>) -> usize {
    calculate_total_cost(grid, |area, _perimeter, coords| {
        let sides = count_sides(coords);
        area * sides
    })
}

pub fn solve_part1(input: &str) -> Result<String> {
    let vec_grid = parse_grid(input)?;
    let grid = Grid::from_vec2d(vec_grid);

    let total_cost = calculate_total_fencing_cost(&grid);
    Ok(total_cost.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let vec_grid = parse_grid(input)?;
    let grid = Grid::from_vec2d(vec_grid);

    let total_cost = calculate_total_bulk_discount_cost(&grid);
    Ok(total_cost.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_EXAMPLE: &str = "\
AAAA
BBCD
BBCC
EEEC";

    const OXOXO_EXAMPLE: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const LARGE_EXAMPLE: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_parse_simple_grid() {
        let grid = parse_grid(SIMPLE_EXAMPLE).unwrap();
        assert_eq!(grid.len(), 4); // 4 rows
        assert_eq!(grid[0].len(), 4); // 4 columns
        assert_eq!(grid[0], vec!['A', 'A', 'A', 'A']);
        assert_eq!(grid[1], vec!['B', 'B', 'C', 'D']);
        assert_eq!(grid[2], vec!['B', 'B', 'C', 'C']);
        assert_eq!(grid[3], vec!['E', 'E', 'E', 'C']);
    }

    #[test]
    fn test_parse_oxoxo_example() {
        let grid = parse_grid(OXOXO_EXAMPLE).unwrap();
        assert_eq!(grid.len(), 5);
        assert_eq!(grid[0].len(), 5);
        assert_eq!(grid[1][1], 'X'); // Middle X
        assert_eq!(grid[1][3], 'X'); // Another X
    }

    #[test]
    fn test_parse_large_example() {
        let grid = parse_grid(LARGE_EXAMPLE).unwrap();
        assert_eq!(grid.len(), 10);
        assert_eq!(grid[0].len(), 10);
        assert_eq!(grid[0][0], 'R');
        assert_eq!(grid[9][9], 'E');
    }

    #[test]
    fn test_parse_handles_empty_input() {
        assert!(parse_grid("").is_err());
    }

    #[test]
    fn test_parse_handles_whitespace_only() {
        assert!(parse_grid("   \n\n  \n").is_err());
    }

    #[test]
    fn test_parse_single_cell() {
        let grid = parse_grid("A").unwrap();
        assert_eq!(grid.len(), 1);
        assert_eq!(grid[0].len(), 1);
        assert_eq!(grid[0][0], 'A');
    }

    #[test]
    fn test_parse_single_row() {
        let grid = parse_grid("ABCDE").unwrap();
        assert_eq!(grid.len(), 1);
        assert_eq!(grid[0].len(), 5);
        assert_eq!(grid[0], vec!['A', 'B', 'C', 'D', 'E']);
    }

    #[test]
    fn test_parse_single_column() {
        let input = "A\nB\nC\nD\nE";
        let grid = parse_grid(input).unwrap();
        assert_eq!(grid.len(), 5);
        assert_eq!(grid[0].len(), 1);
        assert_eq!(grid[0][0], 'A');
        assert_eq!(grid[4][0], 'E');
    }

    #[test]
    fn test_parse_handles_trailing_newline() {
        let input = "AA\nBB\n";
        let grid = parse_grid(input).unwrap();
        assert_eq!(grid.len(), 2);
        assert_eq!(grid[0], vec!['A', 'A']);
        assert_eq!(grid[1], vec!['B', 'B']);
    }

    #[test]
    fn test_parse_rejects_jagged_grid() {
        let input = "AAA\nBB\nCCC"; // Second row is shorter
        assert!(parse_grid(input).is_err());
    }

    #[test]
    fn test_parse_handles_various_characters() {
        let input = "ABC\n123\n!@#";
        let grid = parse_grid(input).unwrap();
        assert_eq!(grid[0], vec!['A', 'B', 'C']);
        assert_eq!(grid[1], vec!['1', '2', '3']);
        assert_eq!(grid[2], vec!['!', '@', '#']);
    }

    // Part 1 tests - calculating fencing cost (area × perimeter)

    #[test]
    fn test_part1_simple_example() {
        // Expected from problem:
        // Region A: area 4 × perimeter 10 = 40
        // Region B: area 4 × perimeter 8 = 32
        // Region C: area 4 × perimeter 10 = 40
        // Region D: area 1 × perimeter 4 = 4
        // Region E: area 3 × perimeter 8 = 24
        // Total: 140
        let result = solve_part1(SIMPLE_EXAMPLE).unwrap();
        assert_eq!(result, "140");
    }

    #[test]
    fn test_part1_oxoxo_example() {
        // Expected from problem:
        // 1 large O region: area 21 × perimeter 36 = 756
        // 4 small X regions: each area 1 × perimeter 4 = 4
        // Total: 756 + 4 + 4 + 4 + 4 = 772
        let result = solve_part1(OXOXO_EXAMPLE).unwrap();
        assert_eq!(result, "772");
    }

    #[test]
    fn test_part1_large_example() {
        // Expected from problem: 1930
        let result = solve_part1(LARGE_EXAMPLE).unwrap();
        assert_eq!(result, "1930");
    }

    #[test]
    fn test_part1_single_cell() {
        let input = "A";
        let result = solve_part1(input).unwrap();
        // Single cell: area 1, perimeter 4
        assert_eq!(result, "4");
    }

    #[test]
    fn test_part1_two_regions() {
        let input = "AA\nBB";
        let result = solve_part1(input).unwrap();
        // Region A: area 2, perimeter 6 = 12
        // Region B: area 2, perimeter 6 = 12
        // Total: 24
        assert_eq!(result, "24");
    }

    // Part 2 tests - calculating bulk discount cost (area × sides)

    #[test]
    fn test_part2_simple_example() {
        // Expected from problem:
        // Region A: area 4 × 4 sides = 16
        // Region B: area 4 × 4 sides = 16
        // Region C: area 4 × 8 sides = 32
        // Region D: area 1 × 4 sides = 4
        // Region E: area 3 × 4 sides = 12
        // Total: 80
        let result = solve_part2(SIMPLE_EXAMPLE).unwrap();
        assert_eq!(result, "80");
    }

    #[test]
    fn test_part2_oxoxo_example() {
        // Expected from problem: 436
        let result = solve_part2(OXOXO_EXAMPLE).unwrap();
        assert_eq!(result, "436");
    }

    #[test]
    fn test_part2_e_shape_example() {
        // E-shaped region from problem
        const E_SHAPE: &str = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        // E region: area 17 × 12 sides = 204
        // 2 X regions: each area 4 × 4 sides = 16
        // Total: 204 + 16 + 16 = 236
        let result = solve_part2(E_SHAPE).unwrap();
        assert_eq!(result, "236");
    }

    #[test]
    fn test_part2_nested_example() {
        // Nested B regions inside A from problem
        const NESTED: &str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        // 2 B regions: each area 4 × 4 sides = 16
        // 1 A region: area 28 × 12 sides = 336 (4 outside + 8 inside)
        // Total: 368
        let result = solve_part2(NESTED).unwrap();
        assert_eq!(result, "368");
    }

    #[test]
    fn test_part2_large_example() {
        // Expected from problem: 1206
        let result = solve_part2(LARGE_EXAMPLE).unwrap();
        assert_eq!(result, "1206");
    }

    #[test]
    fn test_part2_single_cell() {
        let input = "A";
        let result = solve_part2(input).unwrap();
        // Single cell: area 1, 4 sides (it's a square)
        assert_eq!(result, "4");
    }
}
