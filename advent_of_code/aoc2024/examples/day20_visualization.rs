use aoc2024::solver::day20::{parse_input, bfs_distances, Pos};
use mission6::Grid;
use std::collections::{HashMap, HashSet};

// ANSI color codes
const RESET: &str = "\x1b[0m";
const WALL: &str = "\x1b[90m";        // Dark gray
const PATH: &str = "\x1b[92m";        // Bright green
const AVOIDED_PATH: &str = "\x1b[91m"; // Bright red - path skipped by cheat
const CHEAT_START: &str = "\x1b[93m"; // Bright yellow
const CHEAT_END: &str = "\x1b[96m";   // Bright cyan
const CHEAT_PATH: &str = "\x1b[95m";  // Bright magenta
const START_END_MARKER: &str = "\x1b[97m"; // Bright white for S and E
const EMPTY: &str = "\x1b[37m";       // White

fn reconstruct_path(distances: &HashMap<Pos, usize>, start: Pos, end: Pos) -> Vec<Pos> {
    let mut path = vec![end];
    let mut current = end;
    
    while current != start {
        let current_dist = distances[&current];
        // Find neighbor with distance = current - 1
        for next in [
            Pos { row: current.row.wrapping_sub(1), col: current.col },
            Pos { row: current.row + 1, col: current.col },
            Pos { row: current.row, col: current.col.wrapping_sub(1) },
            Pos { row: current.row, col: current.col + 1 },
        ] {
            if let Some(&dist) = distances.get(&next) {
                if dist == current_dist - 1 {
                    path.push(next);
                    current = next;
                    break;
                }
            }
        }
    }
    
    path.reverse();
    path
}

fn visualize_grid(grid: &Grid<char>, path: &[Pos], cheat: Option<(Pos, Pos, Vec<Pos>)>, title: &str) {
    println!("\n{}{}{}", CHEAT_START, title, RESET);
    println!("{}", "=".repeat(title.len()));
    
    // Build set of avoided positions if cheat exists
    let avoided_set: HashSet<Pos> = if let Some((_, _, ref avoided)) = cheat {
        avoided.iter().copied().collect()
    } else {
        HashSet::new()
    };
    
    let start_pos = path.first();
    let end_pos = path.last();
    
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            let pos = Pos { row, col };
            let coord = pos.to_coord();
            
            // Check if this is start or end
            if Some(&pos) == start_pos {
                print!("{}S{}", START_END_MARKER, RESET);
                continue;
            } else if Some(&pos) == end_pos {
                print!("{}E{}", START_END_MARKER, RESET);
                continue;
            }
            
            if let Some((cheat_start, cheat_end, _)) = cheat {
                if pos == cheat_start {
                    print!("{}█{}", CHEAT_START, RESET);
                    continue;
                } else if pos == cheat_end {
                    print!("{}█{}", CHEAT_END, RESET);
                    continue;
                }
                
                // Show cheat path (positions between start and end within Manhattan distance)
                if cheat_start.manhattan_distance(&pos) <= cheat_start.manhattan_distance(&cheat_end)
                    && cheat_end.manhattan_distance(&pos) <= cheat_start.manhattan_distance(&cheat_end)
                {
                    let dist_sum = cheat_start.manhattan_distance(&pos) + pos.manhattan_distance(&cheat_end);
                    if dist_sum == cheat_start.manhattan_distance(&cheat_end) {
                        print!("{}▒{}", CHEAT_PATH, RESET);
                        continue;
                    }
                }
            }
            
            // Check if this is part of avoided path
            if avoided_set.contains(&pos) {
                print!("{}×{}", AVOIDED_PATH, RESET);
            } else if path.contains(&pos) {
                print!("{}·{}", PATH, RESET);
            } else {
                let ch = grid[coord];
                if ch == '#' {
                    print!("{}█{}", WALL, RESET);
                } else {
                    print!("{} {}", EMPTY, RESET);
                }
            }
        }
        println!();
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day20_example.txt")
        .expect("Failed to read input file");
    
    let (grid, start, end) = parse_input(&input);
    let dist_from_start = bfs_distances(&grid, start);
    let dist_from_end = bfs_distances(&grid, end);
    
    let normal_time = dist_from_start[&end];
    let path = reconstruct_path(&dist_from_start, start, end);
    
    // 1. Normal path (no cheat)
    visualize_grid(&grid, &path, None, &format!("Normal Path (Time: {} ps)", normal_time));
    
    // 2. Find best 2ps cheat
    let mut best_2ps_cheat = None;
    let mut best_2ps_savings = 0;
    
    for (idx, &pos) in path.iter().enumerate() {
        let dist_to_pos = dist_from_start[&pos];
        
        for (other, dist) in pos.positions_within_distance(2, grid.height(), grid.width()) {
            if grid[other.to_coord()] != '#' {
                if let Some(&dist_from_other) = dist_from_end.get(&other) {
                    let cheat_time = dist_to_pos + dist + dist_from_other;
                    let savings = normal_time.saturating_sub(cheat_time);
                    
                    if savings > best_2ps_savings && savings > 0 {
                        if let Some(end_idx) = path.iter().position(|&p| p == other) {
                            let avoided = path[idx + 1..end_idx].to_vec();
                            best_2ps_savings = savings;
                            best_2ps_cheat = Some((pos, other, avoided));
                        }
                    }
                }
            }
        }
    }
    
    if let Some(cheat) = best_2ps_cheat {
        visualize_grid(
            &grid, 
            &path, 
            Some(cheat),
            &format!("Best 2ps Cheat (Saves: {} ps)", best_2ps_savings)
        );
    }
    
    // 3. Find best 20ps cheat
    let mut best_20ps_cheat = None;
    let mut best_20ps_savings = 0;
    
    for (idx, &pos) in path.iter().enumerate() {
        let dist_to_pos = dist_from_start[&pos];
        
        for (other, manhattan) in pos.positions_within_distance(20, grid.height(), grid.width()) {
            if grid[other.to_coord()] != '#' {
                if let Some(&dist_from_other) = dist_from_end.get(&other) {
                    let cheat_time = dist_to_pos + manhattan + dist_from_other;
                    let savings = normal_time.saturating_sub(cheat_time);
                    
                    if savings > best_20ps_savings {
                        if let Some(end_idx) = path.iter().position(|&p| p == other) {
                            let avoided = path[idx + 1..end_idx].to_vec();
                            best_20ps_savings = savings;
                            best_20ps_cheat = Some((pos, other, avoided));
                        }
                    }
                }
            }
        }
    }
    
    if let Some(cheat) = best_20ps_cheat {
        visualize_grid(
            &grid, 
            &path, 
            Some(cheat),
            &format!("Best 20ps Cheat (Saves: {} ps)", best_20ps_savings)
        );
    }
    
    println!("\nLegend:");
    println!("  {}S{} = Start position", START_END_MARKER, RESET);
    println!("  {}E{} = End position", START_END_MARKER, RESET);
    println!("  {}█{} = Wall", WALL, RESET);
    println!("  {}·{} = Normal path", PATH, RESET);
    println!("  {}×{} = Avoided path (skipped by cheat)", AVOIDED_PATH, RESET);
    println!("  {}█{} = Cheat start", CHEAT_START, RESET);
    println!("  {}█{} = Cheat end", CHEAT_END, RESET);
    println!("  {}▒{} = Cheat tunnel", CHEAT_PATH, RESET);
}
