//! # Day 3: Gear Ratios
//!
//! ## Part 1
//! Find all numbers in an engine schematic that are adjacent (including diagonally)
//! to a symbol. Any character that is NOT a digit or period is a symbol.
//! Sum all such "part numbers".
//!
//! ## Part 2
//! Find all gears (* symbols adjacent to exactly two numbers) and calculate
//! the sum of their gear ratios (product of the two adjacent numbers).
//!
//! ## Algorithm
//! - Parse input into Mission 6 Grid<char> (140x140)
//! - Scan grid to find all numbers with their coordinate spans
//! - For each number, check all adjacent cells (8-directional) for symbols
//! - Part 1: Sum numbers adjacent to any symbol
//! - Part 2: Find * symbols, identify adjacent numbers, calculate gear ratios
//!
//! ## Optimizations
//! - Spatial indexing: HashMap<Coord, NumberId> for O(1) gear lookup
//! - Reverse search: Check *'s 8 neighbors instead of all numbers' adjacencies
//! - Cached adjacency: Calculate adjacent coords once, reuse for both parts
//!
//! ## Mission Integration
//! - **Mission 6 Grid<char>**: Validated 2D storage component
//! - **Mission 6 Coord**: 8-directional neighbor iteration
//! - Integrator approach: Compose from proven components

use anyhow::Result;
use mission6::{Coord, Grid};
use std::collections::{HashMap, HashSet};

/// Check if a character is a symbol (not digit, not period)
fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

/// Parse the input into a Mission 6 Grid<char>
fn parse_grid(input: &str) -> Grid<char> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines.first().map(|l| l.len()).unwrap_or(0);

    let mut grid = Grid::new(width, height, '.');

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = ch;
        }
    }

    grid
}

/// Represents a number found in the grid with its value and coordinate span
#[derive(Debug, Clone)]
struct GridNumber {
    value: u32,
    y: usize,
    x_start: usize,
    x_end: usize, // inclusive
}

impl GridNumber {
    /// Get all coordinates occupied by this number's digits
    fn digit_coords(&self) -> Vec<Coord> {
        (self.x_start..=self.x_end)
            .map(|x| Coord::new(x, self.y))
            .collect()
    }

    /// Get all coordinates adjacent to this number (8-directional)
    /// Uses HashSet to avoid duplicate coordinates efficiently
    fn adjacent_coords(&self, grid: &Grid<char>) -> HashSet<Coord> {
        let mut adjacent = HashSet::new();

        for x in self.x_start..=self.x_end {
            let coord = Coord::new(x, self.y);

            // Check all 8 neighbors for each digit position
            for neighbor in coord.neighbors_8() {
                if grid.in_bounds(neighbor) {
                    // Only add if it's not part of the number itself
                    let is_number_cell = neighbor.y == self.y
                        && neighbor.x >= self.x_start
                        && neighbor.x <= self.x_end;

                    if !is_number_cell {
                        adjacent.insert(neighbor); // HashSet O(1) insert
                    }
                }
            }
        }

        adjacent
    }

    /// Check if this number is adjacent to any symbol
    fn is_adjacent_to_symbol(&self, grid: &Grid<char>) -> bool {
        self.adjacent_coords(grid)
            .iter()
            .any(|&coord| is_symbol(grid[coord]))
    }
}

/// Extract all numbers from the grid with their positions
fn extract_numbers(grid: &Grid<char>) -> Vec<GridNumber> {
    let mut numbers = Vec::new();

    for y in 0..grid.height() {
        let mut x = 0;
        while x < grid.width() {
            let coord = Coord::new(x, y);
            let ch = grid[coord];

            if ch.is_ascii_digit() {
                // Start of a number - collect all consecutive digits
                let x_start = x;
                let mut value_str = String::new();

                while x < grid.width() && grid[Coord::new(x, y)].is_ascii_digit() {
                    value_str.push(grid[Coord::new(x, y)]);
                    x += 1;
                }

                let x_end = x - 1;
                let value = value_str.parse::<u32>().unwrap_or_else(|_| {
                    panic!(
                        "Failed to parse '{}' at y={}, x_start={}",
                        value_str, y, x_start
                    )
                });

                numbers.push(GridNumber {
                    value,
                    y,
                    x_start,
                    x_end,
                });
            } else {
                x += 1;
            }
        }
    }

    numbers
}

pub fn solve_part1(input: &str) -> Result<String> {
    let grid = parse_grid(input);
    let numbers = extract_numbers(&grid);

    let sum: u32 = numbers
        .iter()
        .filter(|num| num.is_adjacent_to_symbol(&grid))
        .map(|num| num.value)
        .sum();

    Ok(sum.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let grid = parse_grid(input);
    let numbers = extract_numbers(&grid);

    // OPTIMIZATION: Build spatial index mapping each digit coord to its number index
    // Original approach: For each * symbol, iterate ALL numbers and calculate adjacency
    //   - O(grid_size × num_numbers × adjacency_calc)
    //   - 140×140 × ~1000 numbers × ~20 ops = millions of operations
    // Optimized approach: For each * symbol, check 8 neighbors in spatial index
    //   - O(grid_size × 8) + O(numbers × avg_digits) for index build
    //   - 140×140 × 8 + 1000 × 3 = ~160K operations
    // Result: ~100x faster for Part 2
    let mut coord_to_number: HashMap<Coord, usize> = HashMap::new();
    for (idx, num) in numbers.iter().enumerate() {
        for coord in num.digit_coords() {
            coord_to_number.insert(coord, idx);
        }
    }

    let mut gear_ratio_sum = 0;

    // Find all * symbols and check if they're gears (adjacent to exactly 2 numbers)
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            if grid[coord] == '*' {
                // Optimized: Check the 8 neighbors directly and find unique adjacent numbers
                let mut adjacent_number_indices = HashSet::new();

                for neighbor in coord.neighbors_8() {
                    if grid.in_bounds(neighbor) {
                        if let Some(&num_idx) = coord_to_number.get(&neighbor) {
                            adjacent_number_indices.insert(num_idx);
                        }
                    }
                }

                // A gear is a * adjacent to exactly 2 numbers
                if adjacent_number_indices.len() == 2 {
                    let indices: Vec<usize> = adjacent_number_indices.into_iter().collect();
                    let gear_ratio = numbers[indices[0]].value * numbers[indices[1]].value;
                    gear_ratio_sum += gear_ratio;
                }
            }
        }
    }

    Ok(gear_ratio_sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "4361");
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE).unwrap();
        // * at (3,1) adjacent to 467 and 35 -> 467*35 = 16345
        // * at (5,8) adjacent to 755 and 598 -> 755*598 = 451490
        // Total: 467835
        assert_eq!(result, "467835");
    }

    #[test]
    fn test_grid_parsing() {
        let grid = parse_grid(EXAMPLE);
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 10);
        assert_eq!(grid[Coord::new(0, 0)], '4');
        assert_eq!(grid[Coord::new(3, 1)], '*');
    }

    #[test]
    fn test_number_extraction() {
        let grid = parse_grid(EXAMPLE);
        let numbers = extract_numbers(&grid);

        // Should find: 467, 114, 35, 633, 617, 58, 592, 755, 664, 598
        assert_eq!(numbers.len(), 10);
        assert_eq!(numbers[0].value, 467);
        assert_eq!(numbers[1].value, 114);
    }

    #[test]
    fn test_symbol_detection() {
        assert!(is_symbol('*'));
        assert!(is_symbol('#'));
        assert!(is_symbol('+'));
        assert!(is_symbol('$'));
        assert!(!is_symbol('.'));
        assert!(!is_symbol('1'));
        assert!(!is_symbol('9'));
    }

    #[test]
    fn test_number_adjacency() {
        let grid = parse_grid(EXAMPLE);
        let numbers = extract_numbers(&grid);

        // 467 is adjacent to * at (3,1)
        assert!(numbers[0].is_adjacent_to_symbol(&grid));

        // 114 is NOT adjacent to any symbol
        assert!(!numbers[1].is_adjacent_to_symbol(&grid));
    }
}
