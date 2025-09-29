//--- Day 6: Probably a Fire Hazard ---
// You've decided to deploy one million lights in a 1000x1000 grid. Each light can be either on or off.
// The lights all start off (0). You receive a list of instructions (your puzzle input) that look like this:    

// turn on 0,0 through 999,999 would turn on (or leave on) every light.
// toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning off the ones that were on, and turning on the ones that were off.
// turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.
// After following the instructions, how many lights are lit?

use anyhow::Result;

// Mission6 imports for grid operations
use mission6::{Grid, Coord};

/// Day 06: Probably a Fire Hazard
/// TDD Implementation - Functions exist but not implemented yet (RED phase)
pub fn solve_part1(input: &str) -> Result<String> {
    // TODO: Implement light grid operations for Part 1
    // - Parse instructions (turn on/off/toggle)
    // - Apply to 1000x1000 grid using Mission6::Grid
    // - Count lights that are on
    
    // Step 1 Process each line of input
    let mut lights = Grid::new(1000, 1000, false);
    
    for line in input.lines().filter(|line| !line.is_empty()) {
        // Parse and apply each command directly to the grid
        apply_light_command(&mut lights, line)?;
    }
    
    let count = lights.iter().filter(|&&light| light).count();
    Ok(count.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    // TODO: Implement brightness control for Part 2  
    // - Parse same instructions but different meaning
    // - turn on = +1 brightness, turn off = -1 (min 0), toggle = +2
    // - Sum total brightness across grid
    let mut lights = Grid::new(1000, 1000, 0u32);
    
    for line in input.lines().filter(|line| !line.is_empty()) {
        // Parse and apply each command directly to the grid
        apply_brightness_command(&mut lights, line)?;
    }
    
    let brightness: u32 = lights.iter().sum();
    Ok(brightness.to_string())
}

pub fn apply_light_command(grid: &mut Grid<bool>, line: &str) -> Result<()> {
    // Parse: "turn on 0,0 through 999,999" or "toggle 0,0 through 999,0"
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    let (command, coord1_str, coord2_str) = if parts[0] == "toggle" {
        // "toggle 0,0 through 999,0" -> command at 0, coords at 1 and 3
        ("toggle", parts[1], parts[3])
    } else {
        // "turn on 0,0 through 999,999" -> parts[1] is "on" or "off"
        (parts[1], parts[2], parts[4])  // parts[1] = "on"/"off", coords at 2 and 4
    };
    
    // Parse coordinates "x,y"
    let coord1_parts: Vec<&str> = coord1_str.split(',').collect();
    let coord2_parts: Vec<&str> = coord2_str.split(',').collect();
    
    let x1: usize = coord1_parts[0].parse()?;
    let y1: usize = coord1_parts[1].parse()?;
    let x2: usize = coord2_parts[0].parse()?;
    let y2: usize = coord2_parts[1].parse()?;
    
    // Apply command to rectangular region
    for y in y1..=y2 {
        for x in x1..=x2 {
            let coord = Coord::new(x, y);
            if let Some(cell) = grid.get_mut(coord) {
                match command {
                    "on" => *cell = true,
                    "off" => *cell = false,
                    "toggle" => *cell = !*cell,
                    _ => return Err(anyhow::anyhow!("Unknown command: {}", command)),
                }
            }
        }
    }
    
    Ok(())
}

pub fn apply_brightness_command(grid: &mut Grid<u32>, line: &str) -> Result<()> {
    // Parse same format but apply brightness rules:
    // "turn on" = +1, "turn off" = -1 (min 0), "toggle" = +2
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    let (command, coord1_str, coord2_str) = if parts[0] == "toggle" {
        ("toggle", parts[1], parts[3])
    } else {
        (parts[1], parts[2], parts[4])  // parts[1] = "on"/"off"
    };
    
    // Parse coordinates
    let coord1_parts: Vec<&str> = coord1_str.split(',').collect();
    let coord2_parts: Vec<&str> = coord2_str.split(',').collect();
    
    let x1: usize = coord1_parts[0].parse()?;
    let y1: usize = coord1_parts[1].parse()?;
    let x2: usize = coord2_parts[0].parse()?;
    let y2: usize = coord2_parts[1].parse()?;
    
    // Apply brightness changes to rectangular region
    for y in y1..=y2 {
        for x in x1..=x2 {
            let coord = Coord::new(x, y);
            if let Some(cell) = grid.get_mut(coord) {
                match command {
                    "on" => *cell += 1,
                    "off" => *cell = cell.saturating_sub(1), // Never go below 0
                    "toggle" => *cell += 2,
                    _ => return Err(anyhow::anyhow!("Unknown brightness command: {}", command)),
                }
            }
        }
    }
    
    Ok(())
}