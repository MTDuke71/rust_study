//! Day 13 Maze Visualization
//!
//! Generates a PNG image of the procedural maze with the optimal BFS path
//! from (1,1) to (31,39), plus the flood-fill region reachable in ≤50 steps.
//!
//! Usage: cargo run --example day13_maze

use image::{Rgb, RgbImage};
use std::collections::{HashMap, HashSet, VecDeque};

const FAV: u64 = 1350;
const START: (u64, u64) = (1, 1);
const TARGET: (u64, u64) = (31, 39);
const MAX_STEPS: u64 = 50;

// Render dimensions — show enough of the maze for context
const WIDTH: u64 = 45;
const HEIGHT: u64 = 50;
const CELL_SIZE: u32 = 12;
const BORDER: u32 = 1;

// Colors
const COLOR_WALL: Rgb<u8> = Rgb([40, 40, 55]);
const COLOR_OPEN: Rgb<u8> = Rgb([220, 220, 230]);
const COLOR_FLOOD: Rgb<u8> = Rgb([180, 210, 255]);
const COLOR_PATH: Rgb<u8> = Rgb([255, 100, 50]);
const COLOR_PATH_BEYOND: Rgb<u8> = Rgb([50, 180, 80]);
const COLOR_START: Rgb<u8> = Rgb([50, 200, 50]);
const COLOR_TARGET: Rgb<u8> = Rgb([200, 50, 50]);
const COLOR_GRID: Rgb<u8> = Rgb([160, 160, 170]);

fn is_open(x: u64, y: u64) -> bool {
    let val = x * x + 3 * x + 2 * x * y + y + y * y + FAV;
    val.count_ones() & 1 == 0
}

/// BFS that tracks parent pointers for path reconstruction
fn bfs_with_path() -> (Vec<(u64, u64)>, HashSet<(u64, u64)>) {
    let mut visited: HashSet<(u64, u64)> = HashSet::new();
    let mut parent: HashMap<(u64, u64), (u64, u64)> = HashMap::new();
    let mut queue: VecDeque<(u64, u64, u64)> = VecDeque::new();
    let mut flood_region: HashSet<(u64, u64)> = HashSet::new();

    queue.push_back((START.0, START.1, 0));
    visited.insert(START);

    let mut found_target = false;

    while let Some((x, y, steps)) = queue.pop_front() {
        if steps <= MAX_STEPS {
            flood_region.insert((x, y));
        }

        if (x, y) == TARGET {
            found_target = true;
        }

        if steps >= MAX_STEPS && found_target {
            continue;
        }

        let neighbors = [
            (x + 1, y),
            (x.wrapping_sub(1), y),
            (x, y + 1),
            (x, y.wrapping_sub(1)),
        ];

        for (nx, ny) in neighbors {
            if nx < 1000 && ny < 1000 && is_open(nx, ny) && visited.insert((nx, ny)) {
                parent.insert((nx, ny), (x, y));
                queue.push_back((nx, ny, steps + 1));
            }
        }
    }

    // Reconstruct path from target back to start
    let mut path = Vec::new();
    let mut current = TARGET;
    while current != START {
        path.push(current);
        current = parent[&current];
    }
    path.push(START);
    path.reverse();

    (path, flood_region)
}

fn main() {
    let (path, flood_region) = bfs_with_path();

    // Split path into within-50 and beyond-50
    let path_in_flood: HashSet<(u64, u64)> = path.iter()
        .take(MAX_STEPS as usize + 1)  // steps 0..=50
        .copied()
        .collect();
    let path_beyond: HashSet<(u64, u64)> = path.iter()
        .skip(MAX_STEPS as usize + 1)  // steps 51+
        .copied()
        .collect();

    let img_w = WIDTH as u32 * CELL_SIZE;
    let img_h = HEIGHT as u32 * CELL_SIZE;
    let mut img = RgbImage::new(img_w, img_h);

    // Draw each cell
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = if (x, y) == START {
                COLOR_START
            } else if (x, y) == TARGET {
                COLOR_TARGET
            } else if path_in_flood.contains(&(x, y)) {
                COLOR_PATH          // orange = path within 50 steps
            } else if path_beyond.contains(&(x, y)) {
                COLOR_PATH_BEYOND   // green = path beyond 50 steps
            } else if !is_open(x, y) {
                COLOR_WALL
            } else if flood_region.contains(&(x, y)) {
                COLOR_FLOOD
            } else {
                COLOR_OPEN
            };

            let px = x as u32 * CELL_SIZE;
            let py = y as u32 * CELL_SIZE;

            for dy in 0..CELL_SIZE {
                for dx in 0..CELL_SIZE {
                    // Draw thin grid lines on right and bottom edges
                    let is_border = dx >= CELL_SIZE - BORDER || dy >= CELL_SIZE - BORDER;
                    let pixel_color = if is_border && is_open(x, y) {
                        COLOR_GRID
                    } else {
                        color
                    };
                    img.put_pixel(px + dx, py + dy, pixel_color);
                }
            }
        }
    }

    // Print path with step numbers, marking the 50-step boundary
    println!("Path coordinates (step: x,y):");
    for (step, &(x, y)) in path.iter().enumerate() {
        let in_flood = step as u64 <= MAX_STEPS;
        let marker = if step as u64 == MAX_STEPS { " <-- step 50 boundary" }
                     else if !in_flood { " (beyond 50)" }
                     else { "" };
        println!("  step {:3}: ({:2}, {:2}){}", step, x, y, marker);
    }
    println!();

    // Count path cells per row within flood region
    println!("Orange cells per row (within blue region, steps 0-50):");
    let mut total_in_flood = 0;
    for row in 0..HEIGHT {
        let count: usize = path.iter().enumerate()
            .filter(|(step, &(_, y))| y == row && (*step as u64) <= MAX_STEPS)
            .count();
        if count > 0 {
            total_in_flood += count;
            println!("  row {:2}: {} orange cells", row, count);
        }
    }
    println!("  TOTAL: {} orange cells in flood region", total_in_flood);
    println!();

    let output = "day13_maze.png";
    img.save(output).expect("Failed to save image");

    println!("Maze saved to {output}");
    println!("  Grid:   {WIDTH} x {HEIGHT} cells ({img_w} x {img_h} pixels)");
    println!("  Path:   {} steps (start to target)", path.len() - 1);
    println!("  Flood:  {} cells reachable in ≤{MAX_STEPS} steps", flood_region.len());
    println!();
    println!("Legend:");
    println!("  Green  = Start (1,1)");
    println!("  Red    = Target (31,39)");
    println!("  Orange = Path within ≤{MAX_STEPS} steps ({} cells)", path_in_flood.len());
    println!("  Green  = Path beyond {MAX_STEPS} steps ({} cells)", path_beyond.len() + 1); // +1 for target
    println!("  Blue   = Reachable in ≤{MAX_STEPS} steps ({} cells)", flood_region.len());
    println!("  Dark   = Walls");
    println!("  Light  = Open spaces");
}
