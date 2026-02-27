//! Day 14: Regolith Reservoir
//!
//! Simulate falling sand in a cave system with rock structures.
//! Sand falls from (500, 0) and settles according to physics rules.

use mission6::{Coord, Grid};

// ============================================================================
// Constants
// ============================================================================

const SOURCE_X: usize = 500;
const SOURCE_Y: usize = 0;

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

#[derive(Debug)]
pub struct ParsedData {
    grid: Grid<Tile>,
    source: Coord,
    max_y: usize,
}

// ============================================================================
// Parsing
// ============================================================================

/// Parse a coordinate from "x,y" format
fn parse_coord(s: &str) -> (usize, usize) {
    let mut parts = s.trim().split(',');
    let x = parts.next().unwrap().parse().unwrap();
    let y = parts.next().unwrap().parse().unwrap();
    (x, y)
}

/// Parse all rock paths and determine bounds
fn parse_rock_paths(input: &str) -> Vec<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|line| line.split(" -> ").map(parse_coord).collect())
        .collect()
}

/// Find min/max coordinates from all rock paths
fn find_bounds(paths: &[Vec<(usize, usize)>]) -> (usize, usize, usize, usize) {
    let mut min_x = SOURCE_X;
    let mut max_x = SOURCE_X;
    let mut min_y = SOURCE_Y;
    let mut max_y = SOURCE_Y;

    for path in paths {
        for &(x, y) in path {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
    }

    (min_x, max_x, min_y, max_y)
}

pub fn parse(input: &str) -> ParsedData {
    let paths = parse_rock_paths(input);
    let (min_x, max_x, _min_y, max_y) = find_bounds(&paths);

    // Add padding for sand spread (sand can move diagonally)
    let x_offset = min_x.saturating_sub(2);
    let width = max_x - x_offset + 3;
    let height = max_y + 1;

    // Create grid filled with Air
    let mut grid = Grid::new(width, height, Tile::Air);

    // Draw rock structures
    for path in paths {
        for window in path.windows(2) {
            let (x1, y1) = window[0];
            let (x2, y2) = window[1];

            // Normalize coordinates
            let x1_norm = x1 - x_offset;
            let x2_norm = x2 - x_offset;

            // Draw horizontal or vertical line
            if x1 == x2 {
                // Vertical line
                let x = x1_norm;
                for y in y1.min(y2)..=y1.max(y2) {
                    grid[Coord::new(x, y)] = Tile::Rock;
                }
            } else {
                // Horizontal line
                let y = y1;
                for x in x1_norm.min(x2_norm)..=x1_norm.max(x2_norm) {
                    grid[Coord::new(x, y)] = Tile::Rock;
                }
            }
        }
    }

    // Calculate normalized source position
    let source = Coord::new(SOURCE_X - x_offset, SOURCE_Y);

    ParsedData {
        grid,
        source,
        max_y,
    }
}

// ============================================================================
// Sand Simulation
// ============================================================================

/// Try to move sand one step. Returns new position if moved, None if at rest.
/// Order: down, down-left, down-right
fn try_move_sand(grid: &Grid<Tile>, pos: Coord) -> Option<Coord> {
    // Try down
    let down = Coord::new(pos.x, pos.y + 1);
    if down.y < grid.height() && grid[down] == Tile::Air {
        return Some(down);
    }
    if down.y >= grid.height() {
        return Some(down); // Falls into abyss
    }

    // Try down-left (check for underflow)
    if pos.x > 0 {
        let down_left = Coord::new(pos.x - 1, pos.y + 1);
        if down_left.y < grid.height() && grid[down_left] == Tile::Air {
            return Some(down_left);
        }
    }

    // Try down-right
    let down_right = Coord::new(pos.x + 1, pos.y + 1);
    if down_right.x < grid.width() && down_right.y < grid.height() && grid[down_right] == Tile::Air
    {
        return Some(down_right);
    }
    if down_right.x >= grid.width() {
        return Some(down_right); // Falls off edge
    }

    None // Sand comes to rest
}

/// Simulate one unit of sand falling. Returns true if it came to rest, false if into abyss.
fn drop_sand(grid: &mut Grid<Tile>, source: Coord, max_y: usize) -> bool {
    let mut pos = source;

    loop {
        match try_move_sand(grid, pos) {
            Some(next) => {
                // Check if fallen into abyss (below max_y)
                if next.y > max_y {
                    return false;
                }
                pos = next;
            }
            None => {
                // Sand comes to rest
                grid[pos] = Tile::Sand;
                return true;
            }
        }
    }
}

// ============================================================================
// Part 1
// ============================================================================

pub fn part1(data: &ParsedData) -> usize {
    let mut grid = data.grid.clone();
    let mut count = 0;

    while drop_sand(&mut grid, data.source, data.max_y) {
        count += 1;
    }

    count
}

// ============================================================================
// Part 2
// ============================================================================

/// Simulate sand with infinite floor at y = max_y + 2
/// Returns true if sand came to rest, false if source is blocked
fn drop_sand_with_floor(grid: &mut Grid<Tile>, source: Coord, floor_y: usize) -> bool {
    // Check if source is already blocked
    if grid[source] == Tile::Sand {
        return false;
    }

    let mut pos = source;

    loop {
        // Check if we hit the floor
        if pos.y + 1 == floor_y {
            // Come to rest on floor
            grid[pos] = Tile::Sand;
            return true;
        }

        match try_move_sand(grid, pos) {
            Some(next) => {
                // Check if next position is out of bounds (need to expand grid)
                if next.x >= grid.width() {
                    // For now, assume grid is wide enough
                    // In practice, max spread is pyramid: max_y + 2 distance from source
                    grid[pos] = Tile::Sand;
                    return true;
                }
                pos = next;
            }
            None => {
                // Sand comes to rest
                grid[pos] = Tile::Sand;
                return true;
            }
        }
    }
}

pub fn part2(data: &ParsedData) -> usize {
    let floor_y = data.max_y + 2;

    // Need wider grid for Part 2 - sand spreads in pyramid
    // Maximum horizontal spread is floor_y from source in both directions
    let left_padding = floor_y;
    let right_padding = floor_y;
    let width_needed = data.grid.width() + left_padding + right_padding;

    // Create new wider grid with x-offset shift
    let mut grid = Grid::new(width_needed, floor_y, Tile::Air);

    // Copy rock structures from original grid with left padding offset
    for y in 0..data.grid.height() {
        for x in 0..data.grid.width() {
            let old_coord = Coord::new(x, y);
            let new_coord = Coord::new(x + left_padding, y);
            grid[new_coord] = data.grid[old_coord];
        }
    }

    // Adjust source position for new grid
    let new_source = Coord::new(data.source.x + left_padding, data.source.y);

    let mut count = 0;
    while drop_sand_with_floor(&mut grid, new_source, floor_y) {
        count += 1;
    }

    count
}

// ============================================================================
// Visualization
// ============================================================================

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

impl Tile {
    fn to_char(self) -> char {
        match self {
            Tile::Air => '.',
            Tile::Rock => '█',
            Tile::Sand => '░',
        }
    }
}

/// Display the grid to terminal
fn display_grid(grid: &Grid<Tile>, source: Coord) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            if coord == source {
                print!("+"); // Show source
            } else {
                print!("{}", grid[coord].to_char());
            }
        }
        println!();
    }
}

/// Clear terminal screen (cross-platform)
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

/// Simulate one grain of sand with visual feedback
/// Returns true if it came to rest, false if into abyss
fn drop_sand_visual(grid: &mut Grid<Tile>, source: Coord, max_y: usize, delay_ms: u64) -> bool {
    let mut pos = source;

    loop {
        // Show current position (temporary)
        if pos.y < grid.height() && pos.x < grid.width() {
            clear_screen();
            let mut temp_grid = grid.clone();
            temp_grid[pos] = Tile::Sand;
            display_grid(&temp_grid, source);
            thread::sleep(Duration::from_millis(delay_ms));
        }

        match try_move_sand(grid, pos) {
            Some(next) => {
                if next.y > max_y {
                    return false; // Into abyss
                }
                pos = next;
            }
            None => {
                // Comes to rest
                grid[pos] = Tile::Sand;

                // Show final resting position
                clear_screen();
                display_grid(grid, source);
                thread::sleep(Duration::from_millis(delay_ms));

                return true;
            }
        }
    }
}

/// Run Part 1 with visual simulation
pub fn part1_visual(data: &ParsedData, delay_ms: u64, max_grains: Option<usize>) -> usize {
    let mut grid = data.grid.clone();
    let mut count = 0;

    println!("Sand simulation starting...");
    println!("Source: ({}, {})", data.source.x, data.source.y);
    println!("Max Y: {}", data.max_y);
    println!();
    thread::sleep(Duration::from_secs(1));

    loop {
        if let Some(max) = max_grains {
            if count >= max {
                break;
            }
        }

        if !drop_sand_visual(&mut grid, data.source, data.max_y, delay_ms) {
            break; // Sand fell into abyss
        }
        count += 1;
    }

    // Final display
    clear_screen();
    display_grid(&grid, data.source);
    println!("\nTotal grains: {}", count);

    count
}

/// Run Part 2 with visual simulation (warning: takes a while!)
pub fn part2_visual(data: &ParsedData, delay_ms: u64, max_grains: Option<usize>) -> usize {
    let floor_y = data.max_y + 2;
    let left_padding = floor_y;
    let right_padding = floor_y;
    let width_needed = data.grid.width() + left_padding + right_padding;

    let mut grid = Grid::new(width_needed, floor_y, Tile::Air);

    // Copy rocks
    for y in 0..data.grid.height() {
        for x in 0..data.grid.width() {
            grid[Coord::new(x + left_padding, y)] = data.grid[Coord::new(x, y)];
        }
    }

    let new_source = Coord::new(data.source.x + left_padding, data.source.y);

    println!("Part 2 visual simulation (with floor at y={})", floor_y);
    println!("Grid size: {}x{}", grid.width(), grid.height());
    println!();
    thread::sleep(Duration::from_secs(1));

    let mut count = 0;
    loop {
        if let Some(max) = max_grains {
            if count >= max {
                break;
            }
        }

        if grid[new_source] == Tile::Sand {
            break; // Source blocked
        }

        // Modified drop_sand_with_floor for visualization
        let mut pos = new_source;
        loop {
            // Visual feedback
            if pos.y < grid.height() && pos.x < grid.width() {
                clear_screen();
                let mut temp_grid = grid.clone();
                temp_grid[pos] = Tile::Sand;
                display_grid(&temp_grid, new_source);
                println!("Grains: {}", count);
                thread::sleep(Duration::from_millis(delay_ms));
            }

            if pos.y + 1 == floor_y {
                grid[pos] = Tile::Sand;
                // Show final resting position
                clear_screen();
                display_grid(&grid, new_source);
                println!("Grains: {}", count);
                thread::sleep(Duration::from_millis(delay_ms));
                break;
            }

            match try_move_sand(&grid, pos) {
                Some(next) => {
                    if next.x >= grid.width() {
                        grid[pos] = Tile::Sand;
                        // Show final resting position
                        clear_screen();
                        display_grid(&grid, new_source);
                        println!("Grains: {}", count);
                        thread::sleep(Duration::from_millis(delay_ms));
                        break;
                    }
                    pos = next;
                }
                None => {
                    grid[pos] = Tile::Sand;
                    // Show final resting position
                    clear_screen();
                    display_grid(&grid, new_source);
                    println!("Grains: {}", count);
                    thread::sleep(Duration::from_millis(delay_ms));
                    break;
                }
            }
        }

        count += 1;
    }

    clear_screen();
    display_grid(&grid, new_source);
    println!("\nTotal grains: {}", count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_parse() {
        let data = parse(EXAMPLE);
        println!("Grid size: {}x{}", data.grid.width(), data.grid.height());
        println!("Source: {:?}", data.source);
        println!("Max Y: {}", data.max_y);
    }

    #[test]
    fn test_part1() {
        let data = parse(EXAMPLE);
        assert_eq!(part1(&data), 24);
    }

    #[test]
    fn test_part2() {
        let data = parse(EXAMPLE);
        assert_eq!(part2(&data), 93);
    }

    #[test]
    fn test_parse_coordinates() {
        let data = parse(EXAMPLE);
        // Verify source is correctly normalized
        assert_eq!(data.source.y, 0);
        // Max y should be 9 (from example)
        assert_eq!(data.max_y, 9);
    }

    #[test]
    fn test_parse_rock_structures() {
        let data = parse(EXAMPLE);
        // First rock path: 498,4 -> 498,6 -> 496,6
        // After normalization with x_offset
        // Should have rocks at the correct positions
        let mut rock_count = 0;
        for y in 0..data.grid.height() {
            for x in 0..data.grid.width() {
                if data.grid[Coord::new(x, y)] == Tile::Rock {
                    rock_count += 1;
                }
            }
        }
        // Example has 20 rock tiles total
        assert_eq!(rock_count, 20);
    }

    #[test]
    fn test_sand_falls_straight_down() {
        // Minimal test: sand falls straight down to floor
        let input = "498,4 -> 502,4";
        let data = parse(input);
        let result = part1(&data);
        // Sand should fall from (500,0) straight down until it hits rock at y=4
        // Then accumulate: 3 units can rest at y=3, then 2 at y=2, then 1 at y=1
        assert!(result > 0); // At least some sand should accumulate
    }

    #[test]
    fn test_part2_blocks_source() {
        let data = parse(EXAMPLE);
        let result = part2(&data);
        // Part 2 should fill until source is blocked
        assert_eq!(result, 93);

        // Verify it's more than Part 1
        let part1_result = part1(&data);
        assert!(result > part1_result);
    }

    #[test]
    fn test_empty_input() {
        // No rock structures - sand falls into abyss immediately
        let input = "";
        let data = parse(input);
        let result = part1(&data);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_point_rock() {
        let input = "500,5 -> 500,5";
        let data = parse(input);
        assert_eq!(data.max_y, 5);
        // Should parse successfully and run without panic
        let _result = part1(&data);
    }

    #[test]
    fn test_horizontal_rock_line() {
        let input = "495,5 -> 505,5";
        let data = parse(input);
        // Verify horizontal line is drawn
        let mut rocks_at_y5 = 0;
        for x in 0..data.grid.width() {
            if data.grid[Coord::new(x, 5)] == Tile::Rock {
                rocks_at_y5 += 1;
            }
        }
        assert_eq!(rocks_at_y5, 11); // 495 to 505 inclusive = 11 positions
    }

    #[test]
    fn test_vertical_rock_line() {
        let input = "500,2 -> 500,8";
        let data = parse(input);
        // Verify vertical line is drawn (should be at normalized x position)
        let mut rocks_in_column = 0;
        let source_x = data.source.x; // Source is at 500, find its normalized x
        for y in 0..data.grid.height() {
            if data.grid[Coord::new(source_x, y)] == Tile::Rock {
                rocks_in_column += 1;
            }
        }
        assert_eq!(rocks_in_column, 7); // y=2 to y=8 inclusive = 7 positions
    }

    #[test]
    fn test_actual_input() {
        let input = include_str!("../../inputs/day14.txt");
        let data = parse(input);

        // Verified solutions from successful submission
        assert_eq!(part1(&data), 763);
        assert_eq!(part2(&data), 23921);
    }
}
