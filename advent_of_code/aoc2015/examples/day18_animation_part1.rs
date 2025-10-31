/// Day 18 Part 1: Animated Game of Life Simulation
///
/// This example shows the Conway's Game of Life simulation with
/// step-by-step visualization of the light grid.
///
/// Run with: cargo run --example day18_animation_part1

use mission6::{Coord, Grid};
use std::thread;
use std::time::Duration;

/// Parse the input grid from text format
fn parse_grid(input: &str) -> Grid<bool> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    let mut grid = Grid::new(width, height, false);

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = ch == '#';
        }
    }

    grid
}

/// Count the number of lights that are ON (8-connected neighbors)
fn count_neighbors_on(grid: &Grid<bool>, coord: Coord) -> usize {
    coord
        .neighbors_8_bounded(grid.width(), grid.height())
        .filter(|&neighbor| grid[neighbor])
        .count()
}

/// Simulate one step of the Game of Life
fn simulate_step(grid: &Grid<bool>) -> Grid<bool> {
    let mut next_grid = Grid::new(grid.width(), grid.height(), false);

    for (coord, &is_on) in grid.enumerate() {
        let neighbors_on = count_neighbors_on(grid, coord);

        let next_state = if is_on {
            // Light is currently ON
            neighbors_on == 2 || neighbors_on == 3
        } else {
            // Light is currently OFF
            neighbors_on == 3
        };

        next_grid[coord] = next_state;
    }

    next_grid
}

/// Count the total number of lights that are ON
fn count_lights_on(grid: &Grid<bool>) -> usize {
    grid.iter().filter(|&&is_on| is_on).count()
}

/// Display the grid with ANSI colors and box drawing characters
fn display_grid(grid: &Grid<bool>, step: usize) {
    // Clear screen (ANSI escape code)
    print!("\x1B[2J\x1B[1;1H");

    println!("┌{}┐", "─".repeat(grid.width() + 2));
    println!("│ 🎄 Day 18: Like a GIF For Your Yard - Part 1 │");
    println!("├{}┤", "─".repeat(grid.width() + 2));
    println!("│ Step: {:3} │ Lights ON: {:4} │", step, count_lights_on(grid));
    println!("├{}┤", "─".repeat(grid.width() + 2));

    for y in 0..grid.height() {
        print!("│ ");
        for x in 0..grid.width() {
            if grid[(x, y)] {
                // ANSI bright yellow for ON
                print!("\x1B[93m█\x1B[0m");
            } else {
                // Dim gray for OFF
                print!("\x1B[90m░\x1B[0m");
            }
        }
        println!(" │");
    }

    println!("└{}┘", "─".repeat(grid.width() + 2));
}

fn main() {
    let test_input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    println!("\n🎬 Starting Game of Life Animation...\n");
    println!("Rules:");
    println!("  • Light ON: stays ON with 2-3 neighbors ON");
    println!("  • Light OFF: turns ON with exactly 3 neighbors ON\n");
    println!("Press Ctrl+C to stop...\n");

    thread::sleep(Duration::from_secs(2));

    let mut grid = parse_grid(test_input);
    let total_steps = 10; // Simulate 10 steps for demonstration

    // Show initial state
    display_grid(&grid, 0);
    thread::sleep(Duration::from_millis(800));

    // Simulate steps
    for step in 1..=total_steps {
        grid = simulate_step(&grid);
        display_grid(&grid, step);
        thread::sleep(Duration::from_millis(600));
    }

    println!("\n✨ Animation complete!");
    println!("Final state: {} lights ON after {} steps", count_lights_on(&grid), total_steps);
    println!("\nFor the full 100x100 grid with 100 steps:");
    println!("  cargo run --release --bin aoc2015 -- 18");
}
