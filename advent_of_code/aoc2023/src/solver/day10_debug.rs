//! Debug version of day10 to visualize the solution

use super::day10::*;
use mission6::{Coord, Grid};
use std::collections::HashMap;

pub fn visualize_part2(input: &str) {
    let grid = parse_grid(input);
    let start = find_start(&grid).expect("Should find starting position 'S'");
    let loop_tiles = find_loop_distances(&grid, start);
    let start_pipe = determine_start_pipe(&grid, start);
    
    println!("Grid: {}x{}", grid.width(), grid.height());
    println!("Start: {:?} -> '{}'", start, start_pipe);
    println!("Loop tiles: {}", loop_tiles.len());
    
    // Create visualization
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            if loop_tiles.contains_key(&coord) {
                print!("{}", grid[coord]);
            } else {
                print!(".");
            }
        }
        println!();
    }
}
