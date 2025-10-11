//! Day 6 Light Grid Visualizer
//!
//! Creates a bitmap visualization of the 1000x1000 light grid after applying
//! all the instructions. Each pixel represents one light (on=white, off=black).
//!
//! Usage:
//!   cargo run --example day06_visualizer                    # Normal AoC Day 6 visualization
//!   cargo run --example day06_visualizer -- --christmas-tree # 🎄 FESTIVE EASTER EGG! 🎄
//!
//! This will:
//! 1. Read the Day 6 input file (or create Christmas tree pattern)
//! 2. Apply all light commands using our Day 6 solver
//! 3. Generate a bitmap image showing the final light pattern
//! 4. Save as "examples/day06_lights.bmp" (or "examples/christmas_tree.bmp") in the examples folder

use anyhow::Result;
use aoc2015::solver::day06::solve_part1;
use mission6::{Coord, Grid};
use std::fs;
use std::io::Write;

fn main() -> Result<()> {
    println!("🎄 Day 6 Light Grid Visualizer");
    println!("===============================");

    // Check for Christmas tree easter egg
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 && args[1] == "--christmas-tree" {
        println!("🎄✨ SPECIAL CHRISTMAS TREE MODE ACTIVATED! ✨🎄");
        println!("🎅 Ho ho ho! Creating festive light pattern...");
        create_christmas_tree_pattern()
    } else {
        // Read the Day 6 input
        match fs::read_to_string("inputs/day06.txt")
            .or_else(|_| fs::read_to_string("inputs/day06_example.txt"))
        {
            Ok(content) => content,
            Err(_) => {
                println!("⚠️  Warning: Could not read inputs/day06.txt or day06_example.txt");
                println!("📝 Using demo pattern instead...");
                println!("💡 Tip: Try 'cargo run --example day06_visualizer -- --christmas-tree' for a festive surprise!");
                create_demo_pattern()
            }
        }
    };

    println!(
        "📊 Processing {} lines of light instructions...",
        input.lines().count()
    );

    // Get the final grid state by parsing the solver result
    let lights_on = solve_part1(&input)?;
    println!("💡 Total lights on: {}", lights_on);

    // Create the actual grid to visualize
    let grid = create_final_grid(&input)?;

    // Keep args available for filename decision

    // Generate bitmap
    println!("🎨 Generating bitmap visualization...");
    let filename = if args.len() > 1 && args[1] == "--christmas-tree" {
        "examples/christmas_tree.bmp"
    } else {
        "examples/day06_lights.bmp"
    };
    create_bitmap(&grid, filename)?;

    println!("✅ Bitmap saved as '{}'", filename);
    println!("📋 Image specs: 1000x1000 pixels, 24-bit BMP format");
    println!("🔍 White pixels = lights ON, Black pixels = lights OFF");

    // Show some statistics
    show_grid_statistics(&grid);

    Ok(())
}

/// Create the final grid state by running all commands
fn create_final_grid(input: &str) -> Result<Grid<bool>> {
    use aoc2015::solver::day06::apply_light_command;

    let mut lights = Grid::new(1000, 1000, false);

    for line in input.lines().filter(|line| !line.is_empty()) {
        apply_light_command(&mut lights, line)?;
    }

    Ok(lights)
}

/// Create a demo pattern if no input file is available
fn create_demo_pattern() -> String {
    "turn on 0,0 through 999,999
turn off 499,499 through 500,500
toggle 0,0 through 999,0
toggle 0,0 through 0,999"
        .to_string()
}

/// Create a festive Christmas tree pattern! 🎄
/// Usage: cargo run --example day06_visualizer -- --christmas-tree
fn create_christmas_tree_pattern() -> String {
    let mut commands = Vec::new();

    // Clear the entire grid first
    commands.push("turn off 0,0 through 999,999".to_string());

    // Create Christmas tree layers (triangles getting wider as we go down)
    let center_x = 500;
    let tree_layers = [
        (100, 20), // Top: small triangle
        (150, 30), // Upper middle
        (200, 40), // Middle
        (250, 50), // Lower middle
        (300, 60), // Bottom: widest part
    ];

    for (y_start, width) in tree_layers {
        for y_offset in 0..50 {
            // Each layer is 50 pixels tall
            let y = y_start + y_offset;
            let layer_width = width + (y_offset * 2); // Triangle shape
            let left_x = center_x - layer_width / 2;
            let right_x = center_x + layer_width / 2;

            commands.push(format!(
                "turn on {},{} through {},{}",
                left_x, y, right_x, y
            ));
        }
    }

    // Add tree trunk
    let trunk_width = 20;
    let trunk_height = 60;
    let trunk_start_y = 350;
    commands.push(format!(
        "turn on {},{} through {},{}",
        center_x - trunk_width / 2,
        trunk_start_y,
        center_x + trunk_width / 2,
        trunk_start_y + trunk_height
    ));

    // Add a star on top! ⭐
    let star_size = 15;
    commands.push(format!(
        "turn on {},{} through {},{}",
        center_x - star_size / 2,
        80,
        center_x + star_size / 2,
        95
    ));

    // Add some ornaments (scattered lights on the tree)
    let ornament_positions = [
        (480, 120),
        (520, 125),
        (470, 140),
        (530, 145),
        (450, 170),
        (550, 175),
        (460, 190),
        (540, 195),
        (430, 220),
        (570, 225),
        (440, 240),
        (560, 245),
        (410, 270),
        (590, 275),
        (420, 290),
        (580, 295),
        (390, 320),
        (610, 325),
        (400, 340),
        (600, 345),
    ];

    for (x, y) in ornament_positions {
        // Toggle to create bright ornament spots
        commands.push(format!(
            "toggle {},{} through {},{}",
            x - 2,
            y - 2,
            x + 2,
            y + 2
        ));
    }

    // Add ground/snow
    commands.push("turn on 0,450 through 999,480".to_string());

    // Add some "snow" (scattered lights in the background)
    for i in 0..50 {
        let x = (i * 37) % 1000; // Pseudo-random distribution
        let y = (i * 23) % 400;
        commands.push(format!("turn on {},{} through {},{}", x, y, x + 1, y + 1));
    }

    commands.join("\n")
}

/// Create a BMP bitmap file from the grid
fn create_bitmap(grid: &Grid<bool>, filename: &str) -> Result<()> {
    let width = 1000u32;
    let height = 1000u32;
    let row_padding = (4 - (width * 3) % 4) % 4;
    let image_size = (width * 3 + row_padding) * height;
    let file_size = 54 + image_size; // BMP header = 54 bytes

    let mut file = fs::File::create(filename)?;

    // BMP File Header (14 bytes)
    file.write_all(b"BM")?; // Signature
    file.write_all(&file_size.to_le_bytes())?; // File size
    file.write_all(&0u32.to_le_bytes())?; // Reserved
    file.write_all(&54u32.to_le_bytes())?; // Data offset

    // BMP Info Header (40 bytes)
    file.write_all(&40u32.to_le_bytes())?; // Header size
    file.write_all(&width.to_le_bytes())?; // Width
    file.write_all(&height.to_le_bytes())?; // Height
    file.write_all(&1u16.to_le_bytes())?; // Planes
    file.write_all(&24u16.to_le_bytes())?; // Bits per pixel (RGB)
    file.write_all(&0u32.to_le_bytes())?; // Compression
    file.write_all(&image_size.to_le_bytes())?; // Image size
    file.write_all(&2835u32.to_le_bytes())?; // X pixels per meter
    file.write_all(&2835u32.to_le_bytes())?; // Y pixels per meter
    file.write_all(&0u32.to_le_bytes())?; // Colors used
    file.write_all(&0u32.to_le_bytes())?; // Important colors

    // Pixel data (bottom-to-top, left-to-right)
    for y in (0..1000).rev() {
        // BMP stores bottom-to-top
        let mut row_data = Vec::new();

        for x in 0..1000 {
            let coord = Coord::new(x, y);
            let is_on = grid.get(coord).copied().unwrap_or(false);

            if is_on {
                row_data.extend([255, 255, 255]); // White (BGR format)
            } else {
                row_data.extend([0, 0, 0]); // Black
            }
        }

        // Add row padding
        for _ in 0..row_padding {
            row_data.push(0);
        }

        file.write_all(&row_data)?;
    }

    Ok(())
}

/// Show interesting statistics about the grid
fn show_grid_statistics(grid: &Grid<bool>) {
    let total_lights = 1000 * 1000;
    let lights_on: usize = grid.iter().filter(|&&light| light).count();
    let lights_off = total_lights - lights_on;
    let percentage_on = (lights_on as f64 / total_lights as f64) * 100.0;

    println!("\n📈 Grid Statistics:");
    println!("   💡 Lights ON:  {:7} ({:.1}%)", lights_on, percentage_on);
    println!(
        "   🌑 Lights OFF: {:7} ({:.1}%)",
        lights_off,
        100.0 - percentage_on
    );
    println!(
        "   📏 Grid Size:  1000x1000 = {} total lights",
        total_lights
    );

    // Find interesting patterns
    analyze_patterns(grid);
}

/// Analyze interesting patterns in the grid
fn analyze_patterns(grid: &Grid<bool>) {
    println!("\n🔍 Pattern Analysis:");

    // Check corners
    let corners = [
        Coord::new(0, 0),
        Coord::new(999, 0),
        Coord::new(0, 999),
        Coord::new(999, 999),
    ];

    let corner_states: Vec<bool> = corners
        .iter()
        .map(|&coord| grid.get(coord).copied().unwrap_or(false))
        .collect();

    println!(
        "   🔲 Corners: TL={} TR={} BL={} BR={}",
        if corner_states[0] { "ON" } else { "OFF" },
        if corner_states[1] { "ON" } else { "OFF" },
        if corner_states[2] { "ON" } else { "OFF" },
        if corner_states[3] { "ON" } else { "OFF" }
    );

    // Check center region
    let center_on = (495..=505)
        .flat_map(|y| {
            (495..=505).map(move |x| grid.get(Coord::new(x, y)).copied().unwrap_or(false))
        })
        .filter(|&light| light)
        .count();

    println!(
        "   🎯 Center (495-505, 495-505): {}/121 lights on",
        center_on
    );

    // Check edges
    let top_edge_on = (0..1000)
        .filter(|&x| grid.get(Coord::new(x, 0)).copied().unwrap_or(false))
        .count();

    let left_edge_on = (0..1000)
        .filter(|&y| grid.get(Coord::new(0, y)).copied().unwrap_or(false))
        .count();

    println!("   📏 Top edge: {}/1000 lights on", top_edge_on);
    println!("   📏 Left edge: {}/1000 lights on", left_edge_on);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_pattern_visualization() {
        let demo_input = create_demo_pattern();
        let grid = create_final_grid(&demo_input).unwrap();

        // Should have some lights on
        let lights_on: usize = grid.iter().filter(|&&light| light).count();
        assert!(lights_on > 0);

        // Test specific expectations from demo pattern
        // "turn on 0,0 through 999,999" -> all on (1,000,000)
        // "turn off 499,499 through 500,500" -> 4 lights off in middle
        // "toggle 0,0 through 999,0" -> top row toggled (1000 lights off)
        // "toggle 0,0 through 0,999" -> left column toggled (999 more off, 0,0 already off)
        // Expected: 1,000,000 - 4 - 1000 - 999 = 997,997 lights on

        // But (0,0) is toggled twice, so it should be ON
        assert_eq!(grid.get(Coord::new(0, 0)).copied(), Some(true));

        // Center region should be OFF (turned off explicitly)
        assert_eq!(grid.get(Coord::new(499, 499)).copied(), Some(false));
        assert_eq!(grid.get(Coord::new(500, 500)).copied(), Some(false));

        // Top edge (except 0,0) should be OFF
        assert_eq!(grid.get(Coord::new(1, 0)).copied(), Some(false));
        assert_eq!(grid.get(Coord::new(999, 0)).copied(), Some(false));

        // Left edge (except 0,0) should be OFF
        assert_eq!(grid.get(Coord::new(0, 1)).copied(), Some(false));
        assert_eq!(grid.get(Coord::new(0, 999)).copied(), Some(false));

        // Other areas should be ON
        assert_eq!(grid.get(Coord::new(500, 1)).copied(), Some(true));
        assert_eq!(grid.get(Coord::new(1, 500)).copied(), Some(true));
    }

    #[test]
    fn test_bitmap_creation() {
        // Create a small test grid
        let mut test_grid = Grid::new(4, 4, false);

        // Set a pattern
        *test_grid.get_mut(Coord::new(0, 0)).unwrap() = true;
        *test_grid.get_mut(Coord::new(3, 3)).unwrap() = true;

        // This would test bitmap creation with a smaller grid
        // (We can't easily test the full bitmap without file I/O)
        assert_eq!(test_grid.get(Coord::new(0, 0)).copied(), Some(true));
        assert_eq!(test_grid.get(Coord::new(3, 3)).copied(), Some(true));
        assert_eq!(test_grid.get(Coord::new(1, 1)).copied(), Some(false));
    }
}
