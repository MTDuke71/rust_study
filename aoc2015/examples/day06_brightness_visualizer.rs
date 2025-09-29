//! Day 6 Brightness Grid Visualizer (Part 2)
//! 
//! Creates a colorized bitmap visualization of the 1000x1000 brightness grid after applying
//! all the instructions. Each pixel's color represents the brightness level using a heat map.
//! 
//! Usage:
//!   cargo run --example day06_brightness_visualizer                    # Normal AoC Day 6 Part 2 visualization
//!   cargo run --example day06_brightness_visualizer -- --christmas-tree # 🎄 FESTIVE BRIGHTNESS TREE! 🎄
//! 
//! This will:
//! 1. Read the Day 6 input file (or create Christmas tree pattern)
//! 2. Apply all brightness commands using our Day 6 Part 2 solver
//! 3. Generate a colorized bitmap showing brightness as RGB heat map
//! 4. Save as "examples/day06_brightness.bmp" (or "examples/christmas_tree_brightness.bmp") in the examples folder

use std::fs;
use std::io::Write;
use aoc2015::solver::day06::solve_part2;
use mission6::{Grid, Coord};
use anyhow::Result;

fn main() -> Result<()> {
    println!("🌈 Day 6 Brightness Grid Visualizer (Part 2)");
    println!("============================================");
    
    // Check for Christmas tree easter egg
    let args: Vec<String> = std::env::args().collect();
    let input = if args.len() > 1 && args[1] == "--christmas-tree" {
        println!("🎄✨ SPECIAL CHRISTMAS TREE BRIGHTNESS MODE ACTIVATED! ✨🎄");
        println!("🎅 Ho ho ho! Creating festive brightness pattern...");
        create_christmas_tree_brightness_pattern()
    } else {
        // Read the Day 6 input
        match fs::read_to_string("inputs/day06.txt")
            .or_else(|_| fs::read_to_string("inputs/day06_example.txt")) {
            Ok(content) => content,
            Err(_) => {
                println!("⚠️  Warning: Could not read inputs/day06.txt or day06_example.txt");
                println!("📝 Using demo brightness pattern instead...");
                println!("💡 Tip: Try 'cargo run --example day06_brightness_visualizer -- --christmas-tree' for a festive surprise!");
                create_demo_brightness_pattern()
            }
        }
    };
    
    println!("📊 Processing {} lines of brightness instructions...", input.lines().count());
    
    // Get the total brightness from solver
    let total_brightness = solve_part2(&input)?;
    println!("🌟 Total brightness: {}", total_brightness);
    
    // Create the actual brightness grid to visualize
    let grid = create_final_brightness_grid(&input)?;
    
    // Keep args available for filename decision
    
    // Generate colorized bitmap
    println!("🎨 Generating colorized brightness visualization...");
    let filename = if args.len() > 1 && args[1] == "--christmas-tree" {
        "examples/christmas_tree_brightness.bmp"
    } else {
        "examples/day06_brightness.bmp"
    };
    create_brightness_bitmap(&grid, filename)?;
    
    println!("✅ Brightness bitmap saved as '{}'", filename);
    println!("📋 Image specs: 1000x1000 pixels, 24-bit RGB heat map");
    println!("🌈 Color coding: Black=0, Blue=low, Green=medium, Yellow=high, Red=max brightness");
    
    // Show brightness statistics
    show_brightness_statistics(&grid);
    
    Ok(())
}

/// Create the final brightness grid state by running all commands
fn create_final_brightness_grid(input: &str) -> Result<Grid<u32>> {
    use aoc2015::solver::day06::apply_brightness_command;
    
    let mut lights = Grid::new(1000, 1000, 0u32);
    
    for line in input.lines().filter(|line| !line.is_empty()) {
        apply_brightness_command(&mut lights, line)?;
    }
    
    Ok(lights)
}

/// Create a demo brightness pattern if no input file is available
fn create_demo_brightness_pattern() -> String {
    "turn on 0,0 through 999,999
toggle 0,0 through 499,499  
turn off 250,250 through 749,749
toggle 400,400 through 599,599".to_string()
}

/// Create a festive Christmas tree brightness pattern! 🎄
fn create_christmas_tree_brightness_pattern() -> String {
    let mut commands = Vec::new();
    
    // Clear the entire grid first
    commands.push("turn off 0,0 through 999,999".to_string());
    
    // Create Christmas tree layers with increasing brightness towards center
    let center_x = 500;
    let tree_layers = [
        (100, 20, 3),  // Top: small triangle, medium brightness
        (150, 30, 4),  // Upper middle  
        (200, 40, 5),  // Middle
        (250, 50, 6),  // Lower middle
        (300, 60, 7),  // Bottom: widest part, brightest
    ];
    
    for (y_start, width, brightness_level) in tree_layers {
        for y_offset in 0..50 { // Each layer is 50 pixels tall
            let y = y_start + y_offset;
            let layer_width = width + (y_offset * 2); // Triangle shape
            let left_x = center_x - layer_width / 2;
            let right_x = center_x + layer_width / 2;
            
            // Build up brightness by turning on multiple times
            for _ in 0..brightness_level {
                commands.push(format!("turn on {},{} through {},{}", left_x, y, right_x, y));
            }
        }
    }
    
    // Add tree trunk with moderate brightness
    let trunk_width = 20;
    let trunk_height = 60;
    let trunk_start_y = 350;
    for _ in 0..4 {
        commands.push(format!(
            "turn on {},{} through {},{}",
            center_x - trunk_width / 2,
            trunk_start_y,
            center_x + trunk_width / 2,
            trunk_start_y + trunk_height
        ));
    }
    
    // Add a super bright star on top! ⭐
    let star_size = 15;
    for _ in 0..15 { // Very bright star
        commands.push(format!(
            "turn on {},{} through {},{}",
            center_x - star_size / 2,
            80,
            center_x + star_size / 2,
            95
        ));
    }
    
    // Add extra bright ornaments (using toggle for +2 brightness each time)
    let ornament_positions = [
        (480, 120), (520, 125), (470, 140), (530, 145),
        (450, 170), (550, 175), (460, 190), (540, 195),
        (430, 220), (570, 225), (440, 240), (560, 245),
        (410, 270), (590, 275), (420, 290), (580, 295),
        (390, 320), (610, 325), (400, 340), (600, 345),
    ];
    
    for (x, y) in ornament_positions {
        // Multiple toggles to create very bright ornament spots
        for _ in 0..5 {
            commands.push(format!("toggle {},{} through {},{}", x-2, y-2, x+2, y+2));
        }
    }
    
    // Add glowing ground/snow with gradient brightness
    for _ in 0..2 {
        commands.push("turn on 0,450 through 999,480".to_string());
    }
    
    // Add twinkling "stars" in sky (scattered high brightness spots)
    for i in 0..30 {
        let x = (i * 37) % 1000;  // Pseudo-random distribution
        let y = (i * 23) % 70;    // Keep in sky area
        for _ in 0..(i % 8 + 1) { // Variable brightness
            commands.push(format!("turn on {},{} through {},{}", x, y, x+1, y+1));
        }
    }
    
    commands.join("\n")
}

/// Create a colorized BMP bitmap from the brightness grid using RGB heat map
fn create_brightness_bitmap(grid: &Grid<u32>, filename: &str) -> Result<()> {
    let width = 1000u32;
    let height = 1000u32;
    let row_padding = (4 - (width * 3) % 4) % 4;
    let image_size = (width * 3 + row_padding) * height;
    let file_size = 54 + image_size; // BMP header = 54 bytes
    
    // Find max brightness for scaling
    let max_brightness = *grid.iter().max().unwrap_or(&1);
    println!("🔥 Max brightness found: {}", max_brightness);
    
    let mut file = fs::File::create(filename)?;
    
    // BMP File Header (14 bytes)
    file.write_all(b"BM")?;                           // Signature
    file.write_all(&file_size.to_le_bytes())?;       // File size
    file.write_all(&0u32.to_le_bytes())?;            // Reserved
    file.write_all(&54u32.to_le_bytes())?;           // Data offset
    
    // BMP Info Header (40 bytes)
    file.write_all(&40u32.to_le_bytes())?;           // Header size
    file.write_all(&width.to_le_bytes())?;           // Width
    file.write_all(&height.to_le_bytes())?;          // Height
    file.write_all(&1u16.to_le_bytes())?;            // Planes
    file.write_all(&24u16.to_le_bytes())?;           // Bits per pixel (RGB)
    file.write_all(&0u32.to_le_bytes())?;            // Compression
    file.write_all(&image_size.to_le_bytes())?;      // Image size
    file.write_all(&2835u32.to_le_bytes())?;         // X pixels per meter
    file.write_all(&2835u32.to_le_bytes())?;         // Y pixels per meter
    file.write_all(&0u32.to_le_bytes())?;            // Colors used
    file.write_all(&0u32.to_le_bytes())?;            // Important colors
    
    // Pixel data (bottom-to-top, left-to-right)
    for y in (0..1000).rev() { // BMP stores bottom-to-top
        let mut row_data = Vec::new();
        
        for x in 0..1000 {
            let coord = Coord::new(x, y);
            let brightness = grid.get(coord).copied().unwrap_or(0);
            
            // Convert brightness to RGB using heat map
            let (r, g, b) = brightness_to_rgb(brightness, max_brightness);
            
            row_data.extend([b, g, r]); // BGR format for BMP
        }
        
        // Add row padding
        for _ in 0..row_padding {
            row_data.push(0);
        }
        
        file.write_all(&row_data)?;
    }
    
    Ok(())
}

/// Convert brightness value to RGB using a heat map scale
fn brightness_to_rgb(brightness: u32, max_brightness: u32) -> (u8, u8, u8) {
    if brightness == 0 {
        return (0, 0, 0); // Black for zero brightness
    }
    
    // Normalize brightness to 0.0-1.0 range
    let normalized = brightness as f32 / max_brightness as f32;
    
    // Create heat map: Black -> Blue -> Cyan -> Green -> Yellow -> Red -> White
    let (r, g, b) = if normalized < 0.2 {
        // Black to Blue
        let t = normalized / 0.2;
        (0, 0, (255.0 * t) as u8)
    } else if normalized < 0.4 {
        // Blue to Cyan  
        let t = (normalized - 0.2) / 0.2;
        (0, (255.0 * t) as u8, 255)
    } else if normalized < 0.6 {
        // Cyan to Green
        let t = (normalized - 0.4) / 0.2;
        (0, 255, (255.0 * (1.0 - t)) as u8)
    } else if normalized < 0.8 {
        // Green to Yellow
        let t = (normalized - 0.6) / 0.2;
        ((255.0 * t) as u8, 255, 0)
    } else if normalized < 1.0 {
        // Yellow to Red
        let t = (normalized - 0.8) / 0.2;
        (255, (255.0 * (1.0 - t)) as u8, 0)
    } else {
        // Maximum brightness -> White
        (255, 255, 255)
    };
    
    (r, g, b)
}

/// Show interesting brightness statistics
fn show_brightness_statistics(grid: &Grid<u32>) {
    let total_cells = 1000 * 1000;
    let total_brightness: u64 = grid.iter().map(|&b| b as u64).sum();
    let non_zero_cells: usize = grid.iter().filter(|&&b| b > 0).count();
    let zero_cells = total_cells - non_zero_cells;
    let max_brightness = *grid.iter().max().unwrap_or(&0);
    let min_brightness = *grid.iter().filter(|&&b| b > 0).min().unwrap_or(&0);
    let avg_brightness = if non_zero_cells > 0 { 
        total_brightness as f64 / non_zero_cells as f64 
    } else { 
        0.0 
    };
    
    println!("\n📈 Brightness Statistics:");
    println!("   🌟 Total Brightness: {}", total_brightness);
    println!("   💡 Cells with Light: {} ({:.1}%)", non_zero_cells, (non_zero_cells as f64 / total_cells as f64) * 100.0);
    println!("   🌑 Dark Cells:      {} ({:.1}%)", zero_cells, (zero_cells as f64 / total_cells as f64) * 100.0);
    println!("   🔥 Max Brightness:  {}", max_brightness);
    println!("   💫 Min Brightness:  {}", min_brightness);
    println!("   📊 Avg Brightness:  {:.2}", avg_brightness);
    
    // Brightness distribution
    analyze_brightness_distribution(grid);
}

/// Analyze brightness distribution across different ranges
fn analyze_brightness_distribution(grid: &Grid<u32>) {
    println!("\n🌈 Brightness Distribution:");
    
    let ranges = [
        (1, 5, "Dim"),
        (6, 15, "Low"), 
        (16, 30, "Medium"),
        (31, 50, "High"),
        (51, 100, "Very High"),
        (101, u32::MAX, "Maximum"),
    ];
    
    for (min_b, max_b, label) in ranges {
        let count = grid.iter().filter(|&&b| b >= min_b && (max_b == u32::MAX || b <= max_b)).count();
        if count > 0 {
            println!("   {} ({}-{}): {} cells", label, min_b, if max_b == u32::MAX { "∞".to_string() } else { max_b.to_string() }, count);
        }
    }
    
    // Check some interesting spots
    let center = Coord::new(500, 500);
    let corners = [
        Coord::new(0, 0),
        Coord::new(999, 0), 
        Coord::new(0, 999),
        Coord::new(999, 999),
    ];
    
    println!("\n🔍 Sample Locations:");
    println!("   🎯 Center (500,500): brightness {}", grid.get(center).copied().unwrap_or(0));
    
    for (i, &coord) in corners.iter().enumerate() {
        let labels = ["TL", "TR", "BL", "BR"];
        println!("   🔲 Corner {} ({},{}): brightness {}", 
            labels[i], coord.x, coord.y, grid.get(coord).copied().unwrap_or(0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_brightness_to_rgb_mapping() {
        // Test key points in the heat map
        assert_eq!(brightness_to_rgb(0, 100), (0, 0, 0));       // Black
        assert_eq!(brightness_to_rgb(100, 100), (255, 255, 255)); // White (max)
        
        // Test heat map progression
        let (r1, g1, b1) = brightness_to_rgb(10, 100); // Should be blue-ish
        let (r2, g2, b2) = brightness_to_rgb(50, 100); // Should be green-ish
        let (r3, g3, b3) = brightness_to_rgb(90, 100); // Should be red-ish
        
        assert!(b1 > r1 && b1 > g1); // Blue dominates at low brightness
        assert!(g2 > 100);            // Green present at medium brightness
        assert!(r3 > 200);            // Red dominates at high brightness
    }
    
    #[test]
    fn test_demo_brightness_pattern() {
        let demo_input = create_demo_brightness_pattern();
        let grid = create_final_brightness_grid(&demo_input).unwrap();
        
        // Should have some brightness
        let total_brightness: u64 = grid.iter().map(|&b| b as u64).sum();
        assert!(total_brightness > 0);
        
        // Check that different areas have different brightness levels
        let center_brightness = grid.get(Coord::new(500, 500)).copied().unwrap_or(0);
        assert!(center_brightness > 0);
    }
}