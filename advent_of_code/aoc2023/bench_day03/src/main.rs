// Benchmark script to compare old vs new Day 3 implementations
// This recreates the old Part 2 implementation to measure actual speedup

use std::collections::HashMap;
use std::time::Instant;
use mission6::{Grid, Coord};

// Old implementation (inefficient)
fn solve_part2_old(input: &str) -> String {
    let grid = parse_grid(input);
    let numbers = extract_numbers(&grid);
    
    let mut gear_ratio_sum = 0;
    
    // ORIGINAL: For each *, iterate ALL numbers and calculate adjacency
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            if grid[coord] == '*' {
                let adjacent_numbers: Vec<&GridNumber> = numbers
                    .iter()
                    .filter(|num| {
                        // EXPENSIVE: Recalculates Vec of adjacent coords every time!
                        num.adjacent_coords_vec(&grid).contains(&coord)
                    })
                    .collect();
                
                if adjacent_numbers.len() == 2 {
                    let gear_ratio = adjacent_numbers[0].value * adjacent_numbers[1].value;
                    gear_ratio_sum += gear_ratio;
                }
            }
        }
    }
    
    gear_ratio_sum.to_string()
}

// New implementation (optimized)
fn solve_part2_new(input: &str) -> String {
    let grid = parse_grid(input);
    let numbers = extract_numbers(&grid);
    
    // Build spatial index
    let mut coord_to_number: HashMap<Coord, usize> = HashMap::new();
    for (idx, num) in numbers.iter().enumerate() {
        for coord in num.digit_coords() {
            coord_to_number.insert(coord, idx);
        }
    }
    
    let mut gear_ratio_sum = 0;
    
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            if grid[coord] == '*' {
                let mut adjacent_number_indices = std::collections::HashSet::new();
                
                for neighbor in coord.neighbors_8() {
                    if grid.in_bounds(neighbor) {
                        if let Some(&num_idx) = coord_to_number.get(&neighbor) {
                            adjacent_number_indices.insert(num_idx);
                        }
                    }
                }
                
                if adjacent_number_indices.len() == 2 {
                    let indices: Vec<usize> = adjacent_number_indices.into_iter().collect();
                    let gear_ratio = numbers[indices[0]].value * numbers[indices[1]].value;
                    gear_ratio_sum += gear_ratio;
                }
            }
        }
    }
    
    gear_ratio_sum.to_string()
}

// Helper functions (same for both)
#[derive(Debug, Clone)]
struct GridNumber {
    value: u32,
    y: usize,
    x_start: usize,
    x_end: usize,
}

impl GridNumber {
    fn digit_coords(&self) -> Vec<Coord> {
        (self.x_start..=self.x_end)
            .map(|x| Coord::new(x, self.y))
            .collect()
    }
    
    fn adjacent_coords_vec(&self, grid: &Grid<char>) -> Vec<Coord> {
        let mut adjacent = Vec::new();
        
        for x in self.x_start..=self.x_end {
            let coord = Coord::new(x, self.y);
            
            for neighbor in coord.neighbors_8() {
                if grid.in_bounds(neighbor) {
                    let is_number_cell = neighbor.y == self.y 
                        && neighbor.x >= self.x_start 
                        && neighbor.x <= self.x_end;
                    
                    if !is_number_cell && !adjacent.contains(&neighbor) {
                        adjacent.push(neighbor);
                    }
                }
            }
        }
        
        adjacent
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

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

fn extract_numbers(grid: &Grid<char>) -> Vec<GridNumber> {
    let mut numbers = Vec::new();
    
    for y in 0..grid.height() {
        let mut x = 0;
        while x < grid.width() {
            let coord = Coord::new(x, y);
            let ch = grid[coord];
            
            if ch.is_ascii_digit() {
                let x_start = x;
                let mut value_str = String::new();
                
                while x < grid.width() && grid[Coord::new(x, y)].is_ascii_digit() {
                    value_str.push(grid[Coord::new(x, y)]);
                    x += 1;
                }
                
                let x_end = x - 1;
                let value = value_str.parse::<u32>().unwrap();
                
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

fn main() {
    let input = std::fs::read_to_string("../advent_of_code/aoc2023/inputs/day03.txt")
        .expect("Failed to read input file");
    
    const ITERATIONS: usize = 100;
    
    println!("Benchmarking Day 3 Part 2 implementations...\n");
    
    // Warm up
    for _ in 0..10 {
        let _ = solve_part2_old(&input);
        let _ = solve_part2_new(&input);
    }
    
    // Benchmark old implementation
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = solve_part2_old(&input);
    }
    let old_time = start.elapsed() / ITERATIONS as u32;
    
    // Benchmark new implementation
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = solve_part2_new(&input);
    }
    let new_time = start.elapsed() / ITERATIONS as u32;
    
    // Verify both give same answer
    let old_result = solve_part2_old(&input);
    let new_result = solve_part2_new(&input);
    
    println!("Results:");
    println!("  Old: {}", old_result);
    println!("  New: {}", new_result);
    println!("  Match: {}", old_result == new_result);
    println!();
    
    println!("Performance ({} iterations):", ITERATIONS);
    println!("  Old implementation: {:?}", old_time);
    println!("  New implementation: {:?}", new_time);
    println!();
    
    let speedup = old_time.as_nanos() as f64 / new_time.as_nanos() as f64;
    println!("  Speedup: {:.1}x faster", speedup);
    
    if speedup >= 50.0 {
        println!("  ✅ Claim verified: ~100x speedup achieved!");
    } else if speedup >= 10.0 {
        println!("  ✅ Significant speedup: {}x faster", speedup as u32);
    } else {
        println!("  ⚠️  Speedup less than expected: {:.1}x", speedup);
    }
}
