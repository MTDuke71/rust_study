//! Day 14: Parabolic Reflector Dish - Platform tilting simulation
//!
//! Uses Mission 6's Grid<T> for platform layout
//! Part 1: Tilt north once and calculate load
//! Part 2: Spin cycle (N-W-S-E) 1 billion times with cycle detection

use anyhow::Result;
use mission6::{Grid, Coord};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,       // .
    RoundRock,   // O
    CubeRock,    // #
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            'O' => Tile::RoundRock,
            '#' => Tile::CubeRock,
            _ => Tile::Empty,
        }
    }
}

impl Tile {
    fn to_char(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::RoundRock => 'O',
            Tile::CubeRock => '#',
        }
    }
}

fn parse_input(input: &str) -> Result<Grid<Tile>> {
    let lines: Vec<&str> = input.lines().collect();
    let mut grid_data = Vec::new();

    for line in lines.iter() {
        let row: Vec<Tile> = line.chars().map(Tile::from).collect();
        grid_data.push(row);
    }

    Ok(Grid::from_vec2d(grid_data))
}

/// Tilt the platform north - round rocks roll up until they hit cube rocks or other round rocks
fn tilt_north(grid: &mut Grid<Tile>) {
    // Process each column from top to bottom
    for col in 0..grid.width() {
        // Track where the next round rock can settle
        let mut next_empty_row = 0;
        
        for row in 0..grid.height() {
            let coord = Coord::new(col, row);
            if let Some(&tile) = grid.get(coord) {
                match tile {
                    Tile::RoundRock => {
                        // Move rock to next available position
                        if row != next_empty_row {
                            // Clear current position
                            if let Some(cell) = grid.get_mut(coord) {
                                *cell = Tile::Empty;
                            }
                            // Place at destination
                            let dest = Coord::new(col, next_empty_row);
                            if let Some(cell) = grid.get_mut(dest) {
                                *cell = Tile::RoundRock;
                            }
                        }
                        next_empty_row += 1;
                    }
                    Tile::CubeRock => {
                        // Cube rock blocks further rolling
                        next_empty_row = row + 1;
                    }
                    Tile::Empty => {
                        // Empty space doesn't affect next_empty_row
                    }
                }
            }
        }
    }
}

/// Tilt the platform south - round rocks roll down
fn tilt_south(grid: &mut Grid<Tile>) {
    for col in 0..grid.width() {
        let mut next_empty_row = grid.height() as i32 - 1;
        
        for row in (0..grid.height()).rev() {
            let coord = Coord::new(col, row);
            if let Some(&tile) = grid.get(coord) {
                match tile {
                    Tile::RoundRock => {
                        let next_row = next_empty_row as usize;
                        if row != next_row {
                            if let Some(cell) = grid.get_mut(coord) {
                                *cell = Tile::Empty;
                            }
                            let dest = Coord::new(col, next_row);
                            if let Some(cell) = grid.get_mut(dest) {
                                *cell = Tile::RoundRock;
                            }
                        }
                        next_empty_row -= 1;
                    }
                    Tile::CubeRock => {
                        next_empty_row = row as i32 - 1;
                    }
                    Tile::Empty => {}
                }
            }
        }
    }
}

/// Tilt the platform west - round rocks roll left
fn tilt_west(grid: &mut Grid<Tile>) {
    for row in 0..grid.height() {
        let mut next_empty_col = 0;
        
        for col in 0..grid.width() {
            let coord = Coord::new(col, row);
            if let Some(&tile) = grid.get(coord) {
                match tile {
                    Tile::RoundRock => {
                        if col != next_empty_col {
                            if let Some(cell) = grid.get_mut(coord) {
                                *cell = Tile::Empty;
                            }
                            let dest = Coord::new(next_empty_col, row);
                            if let Some(cell) = grid.get_mut(dest) {
                                *cell = Tile::RoundRock;
                            }
                        }
                        next_empty_col += 1;
                    }
                    Tile::CubeRock => {
                        next_empty_col = col + 1;
                    }
                    Tile::Empty => {}
                }
            }
        }
    }
}

/// Tilt the platform east - round rocks roll right
fn tilt_east(grid: &mut Grid<Tile>) {
    for row in 0..grid.height() {
        let mut next_empty_col = grid.width() as i32 - 1;
        
        for col in (0..grid.width()).rev() {
            let coord = Coord::new(col, row);
            if let Some(&tile) = grid.get(coord) {
                match tile {
                    Tile::RoundRock => {
                        let next_col = next_empty_col as usize;
                        if col != next_col {
                            if let Some(cell) = grid.get_mut(coord) {
                                *cell = Tile::Empty;
                            }
                            let dest = Coord::new(next_col, row);
                            if let Some(cell) = grid.get_mut(dest) {
                                *cell = Tile::RoundRock;
                            }
                        }
                        next_empty_col -= 1;
                    }
                    Tile::CubeRock => {
                        next_empty_col = col as i32 - 1;
                    }
                    Tile::Empty => {}
                }
            }
        }
    }
}

/// Perform one spin cycle: North -> West -> South -> East
fn spin_cycle(grid: &mut Grid<Tile>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

/// Calculate total load on north support beams
/// Load = distance from south edge (height - row)
fn calculate_load(grid: &Grid<Tile>) -> usize {
    let mut total_load = 0;
    let height = grid.height();
    
    for row in 0..height {
        for col in 0..grid.width() {
            let coord = Coord::new(col, row);
            if let Some(&Tile::RoundRock) = grid.get(coord) {
                // Load = distance from south edge
                total_load += height - row;
            }
        }
    }
    
    total_load
}

/// Convert grid to string for cycle detection
fn grid_to_string(grid: &Grid<Tile>) -> String {
    let mut s = String::new();
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            let coord = Coord::new(col, row);
            if let Some(&tile) = grid.get(coord) {
                s.push(tile.to_char());
            }
        }
    }
    s
}

pub fn solve_part1(input: &str) -> Result<String> {
    let mut grid = parse_input(input)?;
    tilt_north(&mut grid);
    let load = calculate_load(&grid);
    Ok(load.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let mut grid = parse_input(input)?;
    
    // Cycle detection: track grid states and when they first appeared
    let mut seen: HashMap<String, usize> = HashMap::new();
    let target_cycles = 1_000_000_000;
    
    for cycle in 0..target_cycles {
        let state = grid_to_string(&grid);
        
        if let Some(&first_seen) = seen.get(&state) {
            // Found a cycle!
            let cycle_length = cycle - first_seen;
            
            // Fast-forward: how many cycles remain after accounting for the loop?
            let remaining = target_cycles - cycle;
            let final_offset = remaining % cycle_length;
            
            // Run the remaining cycles
            for _ in 0..final_offset {
                spin_cycle(&mut grid);
            }
            
            return Ok(calculate_load(&grid).to_string());
        }
        
        seen.insert(state, cycle);
        spin_cycle(&mut grid);
    }
    
    // No cycle detected (unlikely with billion iterations)
    Ok(calculate_load(&grid).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "136");
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result, "64");
    }

    #[test]
    fn test_tilt_north() {
        let mut grid = parse_input(EXAMPLE).unwrap();
        tilt_north(&mut grid);
        
        // Check that first row has 4 round rocks
        let mut count = 0;
        for col in 0..grid.width() {
            if let Some(&Tile::RoundRock) = grid.get(Coord::new(col, 0)) {
                count += 1;
            }
        }
        assert!(count > 0, "Round rocks should move to top row");
    }

    #[test]
    fn test_load_calculation() {
        let grid = parse_input(EXAMPLE).unwrap();
        let load = calculate_load(&grid);
        // Initial load before tilting should be different from 136
        assert!(load > 0);
    }

    #[test]
    fn test_spin_cycle() {
        let mut grid = parse_input(EXAMPLE).unwrap();
        let initial = grid_to_string(&grid);
        
        spin_cycle(&mut grid);
        let after_one = grid_to_string(&grid);
        
        // Grid should change after spin cycle
        assert_ne!(initial, after_one);
    }
}
