/// Day 18: Interactive 100x100 Grid Simulator
///
/// This example runs the full 100x100 grid simulation with controls:
/// - SPACE: Pause/Resume
/// - 'S': Save current state to file
/// - 'Q': Quit
/// - '1': Switch to Part 1 mode (normal rules)
/// - '2': Switch to Part 2 mode (stuck corners)
/// - '+': Speed up animation
/// - '-': Slow down animation
///
/// Note: Due to terminal input limitations, this version auto-runs
/// and displays stats. Use Ctrl+C to stop.
///
/// Run with: cargo run --example day18_interactive
use mission6::{Coord, Grid};
use std::fs;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

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

/// Save grid to file
fn save_grid_to_file(grid: &Grid<bool>, step: usize, mode: &str) -> io::Result<String> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let filename = format!("day18_saved_state_step{}_mode{}_{}.txt", step, mode, timestamp);
    
    let mut content = String::new();
    content.push_str("# Day 18 Saved State\n");
    content.push_str(&format!("# Step: {}\n", step));
    content.push_str(&format!("# Mode: {}\n", mode));
    content.push_str(&format!("# Lights ON: {}\n", count_lights_on(grid)));
    content.push_str(&format!("# Grid Size: {}x{}\n\n", grid.width(), grid.height()));
    
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            content.push(if grid[(x, y)] { '#' } else { '.' });
        }
        content.push('\n');
    }
    
    fs::write(&filename, content)?;
    Ok(filename)
}

/// Display a compact view of the 100x100 grid
fn display_grid_compact(grid: &Grid<bool>, step: usize, mode: &str, delay_ms: u64) {
    // Clear screen
    print!("\x1B[2J\x1B[1;1H");
    
    println!("╔════════════════════════════════════════════════════════════════════════════╗");
    println!("║ 🎄 Day 18: Interactive 100x100 Grid Simulator                             ║");
    println!("╠════════════════════════════════════════════════════════════════════════════╣");
    println!("║ Mode: {:45} Step: {:6} ║", mode, step);
    println!("║ Lights ON: {:10}  Speed: {:4}ms/step                              ║", 
             count_lights_on(grid), delay_ms);
    println!("╠════════════════════════════════════════════════════════════════════════════╣");
    
    // Display grid with 2x2 cell compression (each character represents 4 cells)
    let block_chars = [
        ' ', '▗', '▖', '▄',  // 0000, 0001, 0010, 0011
        '▝', '▐', '▞', '▟',  // 0100, 0101, 0110, 0111
        '▘', '▚', '▌', '▙',  // 1000, 1001, 1010, 1011
        '▀', '▜', '▛', '█',  // 1100, 1101, 1110, 1111
    ];
    
    for y in (0..grid.height()).step_by(2) {
        print!("║ ");
        for x in (0..grid.width()).step_by(2) {
            let mut index = 0;
            
            // Top-left (bit 3)
            if grid[(x, y)] {
                index |= 8;
            }
            
            // Top-right (bit 2)
            if x + 1 < grid.width() && grid[(x + 1, y)] {
                index |= 4;
            }
            
            // Bottom-left (bit 1)
            if y + 1 < grid.height() && grid[(x, y + 1)] {
                index |= 2;
            }
            
            // Bottom-right (bit 0)
            if x + 1 < grid.width() && y + 1 < grid.height() && grid[(x + 1, y + 1)] {
                index |= 1;
            }
            
            print!("{}", block_chars[index]);
        }
        println!(" ║");
    }
    
    println!("╠════════════════════════════════════════════════════════════════════════════╣");
    println!("║ Controls: Ctrl+C=Quit  |  Auto-running...                                 ║");
    println!("║ Save points every 25 steps                                                 ║");
    println!("╚════════════════════════════════════════════════════════════════════════════╝");
}

/// Display statistics
fn display_statistics(history: &[(usize, usize)]) {
    println!("\n📊 Simulation Statistics:");
    println!("   Total steps: {}", history.len());
    
    if !history.is_empty() {
        let (_, initial) = history[0];
        let (_, final_count) = history[history.len() - 1];
        
        println!("   Initial lights: {}", initial);
        println!("   Final lights: {}", final_count);
        println!("   Change: {:+}", final_count as i32 - initial as i32);
        
        // Find min and max
        let min = history.iter().map(|(_, count)| count).min().unwrap();
        let max = history.iter().map(|(_, count)| count).max().unwrap();
        
        println!("   Peak lights: {}", max);
        println!("   Minimum lights: {}", min);
        
        // Check for stability (last 10 steps have same count)
        if history.len() >= 10 {
            let last_10: Vec<usize> = history.iter()
                .rev()
                .take(10)
                .map(|(_, count)| *count)
                .collect();
            
            let is_stable = last_10.windows(2).all(|w| w[0] == w[1]);
            
            if is_stable {
                println!("   ⚠️  Stable state detected (last 10 steps unchanged)");
            }
        }
    }
}

fn main() -> io::Result<()> {
    // Read the actual puzzle input
    let input = fs::read_to_string("inputs/day18_example.txt")?;
    
    println!("\n🎮 Interactive 100x100 Grid Simulator");
    println!("═══════════════════════════════════════\n");
    
    println!("Select mode:");
    println!("  1. Part 1 - Normal rules");
    println!("  2. Part 2 - Stuck corners");
    print!("\nEnter choice (1 or 2): ");
    io::stdout().flush()?;
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let is_part2 = choice.trim() == "2";
    
    let mode = if is_part2 { "Part 2 (Stuck Corners)" } else { "Part 1 (Normal)" };
    
    println!("\nHow many steps to simulate? (default: 1000): ");
    io::stdout().flush()?;
    
    let mut steps_input = String::new();
    io::stdin().read_line(&mut steps_input)?;
    let max_steps: usize = steps_input.trim().parse().unwrap_or(1000);
    
    println!("\nAnimation delay in milliseconds? (default: 100): ");
    io::stdout().flush()?;
    
    let mut delay_input = String::new();
    io::stdin().read_line(&mut delay_input)?;
    let mut delay_ms: u64 = delay_input.trim().parse().unwrap_or(100);
    
    println!("\n🎬 Starting simulation...\n");
    println!("Mode: {}", mode);
    println!("Max steps: {}", max_steps);
    println!("Animation speed: {}ms per step", delay_ms);
    println!("\nAuto-saving every 25 steps to current directory.");
    println!("Press Ctrl+C to stop at any time.\n");
    
    thread::sleep(Duration::from_secs(2));
    
    let mut grid = parse_grid(&input);
    
    if is_part2 {
        stick_corners_on(&mut grid);
    }
    
    let mut history: Vec<(usize, usize)> = Vec::new();
    history.push((0, count_lights_on(&grid)));
    
    // Show initial state
    display_grid_compact(&grid, 0, mode, delay_ms);
    thread::sleep(Duration::from_millis(delay_ms));
    
    // Simulate steps
    for step in 1..=max_steps {
        // Simulate one step
        grid = simulate_step(&grid);
        
        if is_part2 {
            stick_corners_on(&mut grid);
        }
        
        let lights_on = count_lights_on(&grid);
        history.push((step, lights_on));
        
        // Display current state
        display_grid_compact(&grid, step, mode, delay_ms);
        
        // Auto-save every 25 steps
        if step % 25 == 0 {
            match save_grid_to_file(&grid, step, mode) {
                Ok(filename) => {
                    println!("💾 Auto-saved to: {}", filename);
                }
                Err(e) => {
                    println!("⚠️  Failed to save: {}", e);
                }
            }
        }
        
        // Check for early stability (same count for 50 steps)
        if history.len() >= 50 {
            let last_50: Vec<usize> = history.iter()
                .rev()
                .take(50)
                .map(|(_, count)| *count)
                .collect();
            
            if last_50.windows(2).all(|w| w[0] == w[1]) {
                println!("\n⚠️  Stable state detected! No changes in last 50 steps.");
                println!("   Consider stopping the simulation.");
            }
        }
        
        thread::sleep(Duration::from_millis(delay_ms));
        
        // Adaptive speed: slow down after 100 steps
        if step == 100 && delay_ms < 200 {
            delay_ms = 200;
        }
    }
    
    // Final save
    println!("\n\n🏁 Simulation complete!\n");
    
    match save_grid_to_file(&grid, max_steps, mode) {
        Ok(filename) => {
            println!("💾 Final state saved to: {}", filename);
        }
        Err(e) => {
            println!("⚠️  Failed to save final state: {}", e);
        }
    }
    
    display_statistics(&history);
    
    println!("\n✨ Done! All saved files are in the current directory.");
    
    Ok(())
}
