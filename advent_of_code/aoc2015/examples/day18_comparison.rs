/// Day 18: Side-by-Side Comparison of Part 1 vs Part 2
///
/// This example shows both simulations running in parallel to highlight
/// the difference that stuck corners make to the final outcome.
///
/// Run with: cargo run --example day18_comparison
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
            neighbors_on == 2 || neighbors_on == 3
        } else {
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
        grid[(0, 0)] = true;
        grid[(width - 1, 0)] = true;
        grid[(0, height - 1)] = true;
        grid[(width - 1, height - 1)] = true;
    }
}

/// Count the total number of lights that are ON
fn count_lights_on(grid: &Grid<bool>) -> usize {
    grid.iter().filter(|&&is_on| is_on).count()
}

/// Check if coordinate is a corner
fn is_corner(coord: Coord, width: usize, height: usize) -> bool {
    (coord.x == 0 || coord.x == width - 1) && (coord.y == 0 || coord.y == height - 1)
}

/// Display both grids side by side
fn display_comparison(grid1: &Grid<bool>, grid2: &Grid<bool>, step: usize) {
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");

    let width = grid1.width();

    println!("┌{}┬{}┐", "─".repeat(width + 13), "─".repeat(width + 21));
    println!("│ 🎄 Day 18: Like a GIF For Your Yard - Comparison View                     │");
    println!("├{}┼{}┤", "─".repeat(width + 13), "─".repeat(width + 21));
    println!("│  PART 1: Normal     │  PART 2: Stuck Corners (🔒)       │");
    println!("├{}┼{}┤", "─".repeat(width + 13), "─".repeat(width + 21));
    println!(
        "│  Step: {:3}          │  Step: {:3}                        │",
        step, step
    );
    println!(
        "│  Lights: {:3}        │  Lights: {:3}                      │",
        count_lights_on(grid1),
        count_lights_on(grid2)
    );
    println!("├{}┼{}┤", "─".repeat(width + 13), "─".repeat(width + 21));

    for y in 0..grid1.height() {
        print!("│  ");

        // Part 1 grid
        for x in 0..width {
            if grid1[(x, y)] {
                print!("\x1B[93m█\x1B[0m");
            } else {
                print!("\x1B[90m░\x1B[0m");
            }
        }

        print!("          │  ");

        // Part 2 grid with highlighted corners
        for x in 0..width {
            let coord = Coord::new(x, y);
            let is_stuck = is_corner(coord, grid2.width(), grid2.height());

            if is_stuck {
                print!("\x1B[91m█\x1B[0m"); // Red for stuck corners
            } else if grid2[(x, y)] {
                print!("\x1B[93m█\x1B[0m"); // Yellow for regular ON
            } else {
                print!("\x1B[90m░\x1B[0m"); // Gray for OFF
            }
        }

        println!("                        │");
    }

    println!("└{}┴{}┘", "─".repeat(width + 13), "─".repeat(width + 21));
    println!("\nLegend: \x1B[93m█\x1B[0m = Light ON  \x1B[90m░\x1B[0m = Light OFF  \x1B[91m█\x1B[0m = Stuck Corner (Part 2 only)");
}

fn main() {
    let test_input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    println!("\n🎬 Starting Side-by-Side Comparison...\n");
    println!("Watch how the stuck corners in Part 2 keep the grid more active!");
    println!("Press Ctrl+C to stop...\n");

    thread::sleep(Duration::from_secs(2));

    // Part 1: Normal simulation
    let mut grid1 = parse_grid(test_input);

    // Part 2: With stuck corners
    let mut grid2 = parse_grid(test_input);
    stick_corners_on(&mut grid2);

    let total_steps = 10;

    // Show initial state
    display_comparison(&grid1, &grid2, 0);
    thread::sleep(Duration::from_millis(1000));

    // Simulate steps for both
    for step in 1..=total_steps {
        // Part 1: Normal step
        grid1 = simulate_step(&grid1);

        // Part 2: Step with stuck corners
        grid2 = simulate_step(&grid2);
        stick_corners_on(&mut grid2);

        display_comparison(&grid1, &grid2, step);
        thread::sleep(Duration::from_millis(700));
    }

    println!("\n✨ Comparison complete!\n");
    println!(
        "Part 1: {} lights ON (converged to stable pattern)",
        count_lights_on(&grid1)
    );
    println!(
        "Part 2: {} lights ON (stuck corners keep activity higher)",
        count_lights_on(&grid2)
    );
    println!("\nNotice how Part 1 converges to a 2x2 stable block,");
    println!("while Part 2 maintains more lights due to corner influence.\n");
    println!("For the full 100x100 puzzle:");
    println!("  Part 1: 1061 lights ON after 100 steps");
    println!("  Part 2: 1006 lights ON after 100 steps");
}
