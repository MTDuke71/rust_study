//! Day 17 Path Visualization Example
//!
//! This example demonstrates how to visualize the optimal paths for Day 17
//! using both text-based (terminal) and HTML (browser) outputs.
//!
//! Run with:
//! ```bash
//! cargo run --example day17_visualization
//! ```
//!
//! This will generate two HTML files:
//! - day17_part1_visualization.html
//! - day17_part2_visualization.html

use mission6::{Coord, Direction, Grid};
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pos: Coord,
    dir: Direction,
    consecutive: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    cost: usize,
    state: State,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_grid(input: &str) -> Grid<u8> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    let mut vec2d = vec![vec![0u8; width]; height];
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            vec2d[y][x] = ch.to_digit(10).unwrap() as u8;
        }
    }

    Grid::from_vec2d(vec2d)
}

fn find_path_with_tracking(
    grid: &Grid<u8>,
    min_straight: u8,
    max_straight: u8,
) -> (usize, Vec<Coord>) {
    let start = Coord::new(0, 0);
    let goal = Coord::new(grid.width() - 1, grid.height() - 1);

    let mut heap = BinaryHeap::new();
    let mut visited: HashMap<State, usize> = HashMap::new();
    let mut predecessors: HashMap<State, State> = HashMap::new();

    // Start facing east or south (can't go backwards from start)
    for dir in [Direction::East, Direction::South] {
        let state = State {
            pos: start,
            dir,
            consecutive: 0,
        };
        heap.push(Node { cost: 0, state });
    }

    while let Some(Node { cost, state }) = heap.pop() {
        // Reached goal - but must have moved at least min_straight blocks
        if state.pos == goal && state.consecutive >= min_straight {
            let path = reconstruct_path(&predecessors, state, start);
            return (cost, path);
        }

        // Already visited with lower cost
        if let Some(&prev_cost) = visited.get(&state) {
            if cost >= prev_cost {
                continue;
            }
        }
        visited.insert(state, cost);

        // Generate next states
        let mut next_dirs = Vec::new();

        // Can continue straight if < max_straight consecutive
        if state.consecutive < max_straight {
            next_dirs.push((state.dir, state.consecutive + 1));
        }

        // Can turn left or right only if moved at least min_straight blocks
        if state.consecutive >= min_straight {
            next_dirs.push((state.dir.rotate_90_counterclockwise(), 1));
            next_dirs.push((state.dir.rotate_90_clockwise(), 1));
        }

        for (next_dir, next_consecutive) in next_dirs {
            let (dx, dy) = next_dir.delta();
            let nx = state.pos.x as isize + dx;
            let ny = state.pos.y as isize + dy;

            if nx >= 0 && ny >= 0 {
                let next_pos = Coord::new(nx as usize, ny as usize);

                if grid.in_bounds(next_pos) {
                    let next_state = State {
                        pos: next_pos,
                        dir: next_dir,
                        consecutive: next_consecutive,
                    };

                    let heat_loss = grid.get(next_pos).unwrap();
                    let next_cost = cost + *heat_loss as usize;

                    // Only add if not visited or found cheaper path
                    if !visited.contains_key(&next_state)
                        || visited.get(&next_state).unwrap() > &next_cost
                    {
                        predecessors.insert(next_state, state);
                        heap.push(Node {
                            cost: next_cost,
                            state: next_state,
                        });
                    }
                }
            }
        }
    }

    unreachable!("No path found to goal")
}

fn reconstruct_path(
    predecessors: &HashMap<State, State>,
    mut current: State,
    start: Coord,
) -> Vec<Coord> {
    let mut path = vec![current.pos];

    while current.pos != start {
        if let Some(&prev) = predecessors.get(&current) {
            current = prev;
            path.push(current.pos);
        } else {
            break;
        }
    }

    path.reverse();
    path
}

fn visualize_path(grid: &Grid<u8>, path: &[Coord], title: &str) {
    println!("\n{}", title);
    println!("{}", "=".repeat(title.len()));

    // Create a map of path positions to their index
    let path_map: HashMap<Coord, usize> = path
        .iter()
        .enumerate()
        .map(|(i, &coord)| (coord, i))
        .collect();

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);

            if let Some(&idx) = path_map.get(&coord) {
                // Determine direction to next coord
                if idx < path.len() - 1 {
                    let next = path[idx + 1];
                    let symbol = if next.x > coord.x {
                        '>' // Moving right
                    } else if next.x < coord.x {
                        '<' // Moving left
                    } else if next.y > coord.y {
                        'v' // Moving down
                    } else if next.y < coord.y {
                        '^' // Moving up
                    } else {
                        '*'
                    };
                    print!("{}", symbol);
                } else {
                    print!("E"); // End
                }
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("\nLegend: > = East, v = South, < = West, ^ = North, E = End");
    println!("Path length: {} steps", path.len());

    // Calculate total heat loss
    let total_heat: usize = path
        .iter()
        .skip(1)
        .map(|&pos| *grid.get(pos).unwrap() as usize)
        .sum();
    println!("Total heat loss: {}", total_heat);
}

fn save_path_as_html(
    grid: &Grid<u8>,
    path: &[Coord],
    filename: &str,
    title: &str,
    cost: usize,
) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    // Create a set of path positions for quick lookup
    let path_set: std::collections::HashSet<Coord> = path.iter().copied().collect();

    // HTML header with styling
    writeln!(file, "<!DOCTYPE html>")?;
    writeln!(file, "<html><head><meta charset='UTF-8'>")?;
    writeln!(file, "<title>{}</title>", title)?;
    writeln!(file, "<style>")?;
    writeln!(file, "body {{ font-family: 'Courier New', monospace; background: #1e1e1e; color: #fff; padding: 20px; }}")?;
    writeln!(file, "h1 {{ color: #4ec9b0; }}")?;
    writeln!(file, "table {{ border-collapse: collapse; margin: 20px 0; box-shadow: 0 4px 6px rgba(0,0,0,0.3); }}")?;
    writeln!(file, "td {{ width: 30px; height: 30px; text-align: center; font-weight: bold; font-size: 14px; border: 1px solid #444; }}")?;
    writeln!(file, ".normal {{ background: #2d2d30; color: #888; }}")?;
    writeln!(file, ".path {{ background: #ffd700; color: #000; box-shadow: inset 0 0 5px rgba(255,215,0,0.5); }}")?;
    writeln!(
        file,
        ".start {{ background: #4ec9b0; color: #000; font-weight: bold; }}"
    )?;
    writeln!(
        file,
        ".end {{ background: #ff6b6b; color: #fff; font-weight: bold; }}"
    )?;
    writeln!(file, ".info {{ background: #252526; padding: 15px; border-left: 4px solid #4ec9b0; margin: 20px 0; }}")?;
    writeln!(file, ".info p {{ margin: 5px 0; }}")?;
    writeln!(file, "</style></head><body>")?;

    writeln!(file, "<h1>{}</h1>", title)?;
    writeln!(file, "<div class='info'>")?;
    writeln!(file, "<p><strong>Total Heat Loss:</strong> {}</p>", cost)?;
    writeln!(
        file,
        "<p><strong>Path Length:</strong> {} steps</p>",
        path.len()
    )?;
    writeln!(
        file,
        "<p><strong>Grid Size:</strong> {}x{}</p>",
        grid.width(),
        grid.height()
    )?;
    writeln!(file, "</div>")?;

    // Grid table
    writeln!(file, "<table>")?;
    for y in 0..grid.height() {
        writeln!(file, "<tr>")?;
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            let heat = grid.get(coord).unwrap();

            let class = if coord == Coord::new(0, 0) {
                "start"
            } else if coord == Coord::new(grid.width() - 1, grid.height() - 1) {
                "end"
            } else if path_set.contains(&coord) {
                "path"
            } else {
                "normal"
            };

            writeln!(file, "<td class='{}'>{}</td>", class, heat)?;
        }
        writeln!(file, "</tr>")?;
    }
    writeln!(file, "</table>")?;

    // Legend
    writeln!(file, "<div class='info'>")?;
    writeln!(file, "<p><strong>Legend:</strong></p>")?;
    writeln!(file, "<p><span style='background:#4ec9b0;color:#000;padding:2px 8px;border-radius:3px;'>■</span> Start (0,0)</p>")?;
    writeln!(file, "<p><span style='background:#ff6b6b;color:#fff;padding:2px 8px;border-radius:3px;'>■</span> End (bottom-right)</p>")?;
    writeln!(file, "<p><span style='background:#ffd700;color:#000;padding:2px 8px;border-radius:3px;'>■</span> Optimal Path</p>")?;
    writeln!(file, "<p><span style='background:#2d2d30;color:#888;padding:2px 8px;border-radius:3px;'>■</span> Not on path</p>")?;
    writeln!(file, "</div>")?;

    writeln!(file, "</body></html>")?;

    Ok(())
}

fn main() {
    // Read the puzzle input
    let input = std::fs::read_to_string("inputs/day17.txt")
        .expect("Failed to read input file. Make sure to run from the aoc2023 directory.");

    let grid = parse_grid(&input);

    println!("Day 17: Clumsy Crucible - Path Visualization");
    println!("===========================================\n");

    // Part 1: Max 3 consecutive
    println!("Computing Part 1 path (Max 3 consecutive blocks)...");
    let (cost1, path1) = find_path_with_tracking(&grid, 0, 3);
    visualize_path(&grid, &path1, &format!("Part 1 Path (Cost: {})", cost1));

    save_path_as_html(
        &grid,
        &path1,
        "day17_part1_visualization.html",
        "Day 17 Part 1: Clumsy Crucible (Max 3 Consecutive)",
        cost1,
    )
    .expect("Failed to save Part 1 HTML");
    println!("✓ Part 1 HTML saved to: day17_part1_visualization.html");

    // Part 2: Min 4, Max 10 consecutive
    println!("\nComputing Part 2 path (Min 4, Max 10 consecutive blocks)...");
    let (cost2, path2) = find_path_with_tracking(&grid, 4, 10);
    visualize_path(&grid, &path2, &format!("Part 2 Path (Cost: {})", cost2));

    save_path_as_html(
        &grid,
        &path2,
        "day17_part2_visualization.html",
        "Day 17 Part 2: Ultra Crucible (Min 4, Max 10 Consecutive)",
        cost2,
    )
    .expect("Failed to save Part 2 HTML");
    println!("✓ Part 2 HTML saved to: day17_part2_visualization.html");

    println!("\n✓ Visualization complete! Open the HTML files in your browser to view the paths.");
}
