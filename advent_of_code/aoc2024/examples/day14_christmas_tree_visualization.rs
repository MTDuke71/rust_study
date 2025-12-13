/// Day 14: Christmas Tree Visualization
/// 
/// This example visualizes the robot positions when they form the Christmas tree pattern.
/// The tree appears at timestep 6516 when all robots occupy unique positions.

use mission6::{Grid, Coord};
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() {
            return None;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            return None;
        }
        
        let pos = parts[0].strip_prefix("p=")?;
        let vel = parts[1].strip_prefix("v=")?;
        
        let pos_parts: Vec<&str> = pos.split(',').collect();
        let vel_parts: Vec<&str> = vel.split(',').collect();
        
        if pos_parts.len() != 2 || vel_parts.len() != 2 {
            return None;
        }
        
        Some(Robot {
            px: pos_parts[0].parse().ok()?,
            py: pos_parts[1].parse().ok()?,
            vx: vel_parts[0].parse().ok()?,
            vy: vel_parts[1].parse().ok()?,
        })
    }
    
    fn position_at(&self, seconds: i32, width: i32, height: i32) -> (i32, i32) {
        let x = (self.px + self.vx * seconds).rem_euclid(width);
        let y = (self.py + self.vy * seconds).rem_euclid(height);
        (x, y)
    }
}

fn parse_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter_map(|line| Robot::parse(line.trim()))
        .collect()
}

fn build_grid_display(robots: &[Robot], seconds: i32, width: i32, height: i32) -> Grid<char> {
    let mut grid = Grid::new(height as usize, width as usize, '.');
    
    for robot in robots {
        let (x, y) = robot.position_at(seconds, width, height);
        let coord = Coord::new(y as usize, x as usize);
        if let Some(cell) = grid.get_mut(coord) {
            *cell = '#';
        }
    }
    
    grid
}

fn print_grid(grid: &Grid<char>) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = Coord::new(y, x);
            let ch = grid.get(coord).copied().unwrap_or('.');
            print!("{}", ch);
        }
        println!();
    }
}

fn check_all_unique(robots: &[Robot], seconds: i32, width: i32, height: i32) -> bool {
    let mut positions = HashSet::new();
    
    for robot in robots {
        let pos = robot.position_at(seconds, width, height);
        if !positions.insert(pos) {
            return false; // Duplicate found
        }
    }
    
    true
}

fn main() {
    println!("🎄 Day 14: Christmas Tree Visualization 🎄\n");
    
    // Try to load the input file
    let input_path = "inputs/day14_example.txt";
    let input = match fs::read_to_string(input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading {}: {}", input_path, e);
            eprintln!("   Please run from the aoc2024 directory or provide the correct path.");
            return;
        }
    };
    
    let robots = parse_robots(&input);
    let (width, height) = (101, 103);
    
    println!("📊 Simulation Parameters:");
    println!("   Robots: {}", robots.len());
    println!("   Grid: {}×{}", width, height);
    println!();
    
    // The Christmas tree appears at timestep 6516
    // Key insight: All robots are at unique positions (no overlaps)
    let tree_timestep = 6516;
    
    println!("🔍 Searching for Christmas tree pattern...");
    println!("   (Looking for timestep where all robots occupy unique positions)\n");
    
    // Verify this is indeed when the pattern appears
    let mut found_timestep = None;
    for seconds in 0..=tree_timestep + 100 {
        if check_all_unique(&robots, seconds, width, height) {
            found_timestep = Some(seconds);
            break;
        }
    }
    
    match found_timestep {
        Some(t) => {
            println!("✅ Found Christmas tree at timestep: {}\n", t);
            
            if t != tree_timestep {
                println!("⚠️  Note: Expected timestep {}, found at {}", tree_timestep, t);
                println!();
            }
            
            println!("🎄 Christmas Tree Pattern:");
            println!("{}", "─".repeat(width as usize));
            
            let grid = build_grid_display(&robots, t, width, height);
            print_grid(&grid);
            
            println!("{}", "─".repeat(width as usize));
            println!();
            
            // Show some statistics
            let robot_count = robots.len();
            let occupied_cells = (0..height)
                .flat_map(|y| (0..width).map(move |x| (x, y)))
                .filter(|&(x, y)| {
                    let coord = Coord::new(y as usize, x as usize);
                    grid.get(coord).copied().unwrap_or('.') == '#'
                })
                .count();
            
            println!("📈 Pattern Statistics:");
            println!("   Total robots: {}", robot_count);
            println!("   Occupied cells: {}", occupied_cells);
            println!("   Grid cells: {}", width * height);
            println!("   Density: {:.2}%", (occupied_cells as f64 / (width * height) as f64) * 100.0);
            println!();
            
            // Show a few frames before and after
            println!("🎬 Animation Preview (frames around the pattern):");
            println!();
            
            for offset in [-2, -1, 0, 1, 2] {
                let frame_time = t + offset;
                if frame_time < 0 {
                    continue;
                }
                
                let is_unique = check_all_unique(&robots, frame_time, width, height);
                let marker = if is_unique { "🎄" } else { "  " };
                
                println!("{} Timestep {}: {} unique positions", 
                    marker, frame_time, 
                    if is_unique { "ALL" } else { "NOT all" });
            }
            
            println!();
            println!("🎁 The Christmas tree appears when all {} robots occupy unique positions!", robot_count);
            println!("   This prevents any overlapping and creates the distinct tree pattern.");
            
        }
        None => {
            println!("❌ Christmas tree pattern not found in the expected range!");
            println!("   Searched timesteps 0 to {}", tree_timestep + 100);
            println!();
            println!("💡 The pattern detection looks for when all robots are at unique positions.");
            println!("   This is the key insight from the problem: no two robots overlap.");
        }
    }
}
