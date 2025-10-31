/// Day 18 Part 2: Animated Game of Life with Stuck Corners
///
/// This example shows the Conway's Game of Life simulation with
/// the four corner lights stuck in the ON position.
///
/// Run with: cargo run --example day18_animation_part2

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

/// Stick the corner lights ON
fn stick_corners_on(grid: &mut Grid<bool>) {
    let width = grid.width();
    let height = grid.height();

    if width > 0 && height > 0 {
        grid[(0, 0)] = true;                    // Top-left
        grid[(width - 1, 0)] = true;            // Top-right
        grid[(0, height - 1)] = true;           // Bottom-left
        grid[(width - 1, height - 1)] = true;   // Bottom-right
    }
}

/// Simulate one step with stuck corners
fn simulate_step_with_stuck_corners(grid: &Grid<bool>) -> Grid<bool> {
    let mut next_grid = simulate_step(grid);
    stick_corners_on(&mut next_grid);
    next_grid
}

/// Count the total number of lights that are ON
fn count_lights_on(grid: &Grid<bool>) -> usize {
    grid.iter().filter(|&&is_on| is_on).count()
}

/// Check if coordinate is a corner
fn is_corner(coord: Coord, width: usize, height: usize) -> bool {
    (coord.x == 0 || coord.x == width - 1) && (coord.y == 0 || coord.y == height - 1)
}

/// Display the grid with ANSI colors and special corner highlighting
fn display_grid(grid: &Grid<bool>, step: usize) {
    // Clear screen (ANSI escape code)
    print!("\x1B[2J\x1B[1;1H");

    println!("┌{}┐", "─".repeat(grid.width() + 2));
    println!("│ 🎄 Day 18: Like a GIF For Your Yard - Part 2 │");
    println!("├{}┤", "─".repeat(grid.width() + 2));
    println!("│ Step: {:3} │ Lights ON: {:4} │ 🔒 CORNERS STUCK │", step, count_lights_on(grid));
    println!("├{}┤", "─".repeat(grid.width() + 2));

    for y in 0..grid.height() {
        print!("│ ");
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            let is_stuck = is_corner(coord, grid.width(), grid.height());

            if is_stuck {
                // Corners are bright red to show they're stuck
                print!("\x1B[91m█\x1B[0m");
            } else if grid[(x, y)] {
                // Regular lights are bright yellow
                print!("\x1B[93m█\x1B[0m");
            } else {
                // Dim gray for OFF
                print!("\x1B[90m░\x1B[0m");
            }
        }
        println!(" │");
    }

    println!("└{}┘", "─".repeat(grid.width() + 2));
    println!("\nLegend: \x1B[91m█\x1B[0m = Stuck Corner  \x1B[93m█\x1B[0m = Light ON  \x1B[90m░\x1B[0m = Light OFF");
}

fn main() {
    let test_input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    println!("\n🎬 Starting Game of Life Animation with Stuck Corners...\n");
    println!("Rules:");
    println!("  • Light ON: stays ON with 2-3 neighbors ON");
    println!("  • Light OFF: turns ON with exactly 3 neighbors ON");
    println!("  • 🔒 CORNERS: Always stuck in ON position!\n");
    println!("Press Ctrl+C to stop...\n");

    thread::sleep(Duration::from_secs(2));

    let mut grid = parse_grid(test_input);
    stick_corners_on(&mut grid); // Ensure corners start ON

    let total_steps = 10; // Simulate 10 steps for demonstration

    // Show initial state
    display_grid(&grid, 0);
    thread::sleep(Duration::from_millis(800));

    // Simulate steps
    for step in 1..=total_steps {
        grid = simulate_step_with_stuck_corners(&grid);
        display_grid(&grid, step);
        thread::sleep(Duration::from_millis(600));
    }

    println!("\n✨ Animation complete!");
    println!("Final state: {} lights ON after {} steps (including 4 stuck corners)", 
             count_lights_on(&grid), total_steps);
    println!("\nFor the full 100x100 grid with 100 steps:");
    println!("  cargo run --release --bin aoc2015 -- 18");
}
