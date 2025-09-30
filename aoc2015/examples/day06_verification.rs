//! Verification test to ensure HashMap and Grid implementations produce identical results
//!
//! This test compares the HashMap implementation against the Mission6 Grid implementation
//! to verify they produce exactly the same answers for the same input.

use std::collections::HashMap;
use std::fs;
use anyhow::Result;

// Import the Grid-based solver functions
use aoc2015::solver::day06::{solve_part1 as solve_part1_grid, solve_part2 as solve_part2_grid};

/// HashMap-based Part 1 solver (copied from day06_hashmap.rs)
fn solve_part1_hashmap(input: &str) -> Result<String> {
    let mut lights: HashMap<(i32, i32), bool> = HashMap::new();
    
    for line in input.lines().filter(|line| !line.is_empty()) {
        apply_light_command_hashmap(&mut lights, line)?;
    }
    
    let count = lights.values().filter(|&&is_on| is_on).count();
    Ok(count.to_string())
}

/// HashMap-based Part 2 solver (copied from day06_hashmap.rs)
fn solve_part2_hashmap(input: &str) -> Result<String> {
    let mut lights: HashMap<(i32, i32), u32> = HashMap::new();
    
    for line in input.lines().filter(|line| !line.is_empty()) {
        apply_brightness_command_hashmap(&mut lights, line)?;
    }
    
    let brightness: u32 = lights.values().sum();
    Ok(brightness.to_string())
}

/// Apply light commands to HashMap-based grid
fn apply_light_command_hashmap(lights: &mut HashMap<(i32, i32), bool>, line: &str) -> Result<()> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    let (command, coord1_str, coord2_str) = if parts[0] == "toggle" {
        ("toggle", parts[1], parts[3])
    } else {
        (parts[1], parts[2], parts[4])
    };
    
    let coord1_parts: Vec<&str> = coord1_str.split(',').collect();
    let coord2_parts: Vec<&str> = coord2_str.split(',').collect();
    
    let x1: i32 = coord1_parts[0].parse()?;
    let y1: i32 = coord1_parts[1].parse()?;
    let x2: i32 = coord2_parts[0].parse()?;
    let y2: i32 = coord2_parts[1].parse()?;
    
    for y in y1..=y2 {
        for x in x1..=x2 {
            let coord = (x, y);
            match command {
                "on" => { lights.insert(coord, true); },
                "off" => { lights.remove(&coord); },
                "toggle" => {
                    let current = lights.get(&coord).copied().unwrap_or(false);
                    if current {
                        lights.remove(&coord);
                    } else {
                        lights.insert(coord, true);
                    }
                },
                _ => return Err(anyhow::anyhow!("Unknown command: {}", command)),
            }
        }
    }
    
    Ok(())
}

/// Apply brightness commands to HashMap-based grid
fn apply_brightness_command_hashmap(lights: &mut HashMap<(i32, i32), u32>, line: &str) -> Result<()> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    let (command, coord1_str, coord2_str) = if parts[0] == "toggle" {
        ("toggle", parts[1], parts[3])
    } else {
        (parts[1], parts[2], parts[4])
    };
    
    let coord1_parts: Vec<&str> = coord1_str.split(',').collect();
    let coord2_parts: Vec<&str> = coord2_str.split(',').collect();
    
    let x1: i32 = coord1_parts[0].parse()?;
    let y1: i32 = coord1_parts[1].parse()?;
    let x2: i32 = coord2_parts[0].parse()?;
    let y2: i32 = coord2_parts[1].parse()?;
    
    for y in y1..=y2 {
        for x in x1..=x2 {
            let coord = (x, y);
            match command {
                "on" => { *lights.entry(coord).or_insert(0) += 1; },
                "off" => {
                    let current = lights.get(&coord).copied().unwrap_or(0);
                    if current > 1 {
                        lights.insert(coord, current - 1);
                    } else if current == 1 {
                        lights.remove(&coord);
                    }
                },
                "toggle" => { *lights.entry(coord).or_insert(0) += 2; },
                _ => return Err(anyhow::anyhow!("Unknown brightness command: {}", command)),
            }
        }
    }
    
    Ok(())
}

/// Compare HashMap vs Grid implementations on example input
fn main() -> Result<()> {
    println!("🔍 HASHMAP vs GRID VERIFICATION");
    println!("===============================\n");
    
    // Read the example input
    let input_path = "aoc2015/inputs/day06_example.txt";
    let input = fs::read_to_string(input_path)
        .map_err(|e| anyhow::anyhow!("Failed to read {}: {}", input_path, e))?;
    
    println!("📄 Input file: {}", input_path);
    println!("📏 Input lines: {}", input.lines().count());
    
    // Test Part 1: Boolean lights
    println!("\n🔸 Part 1 (Boolean Lights) Comparison:");
    
    let grid_part1 = solve_part1_grid(&input)?;
    let hashmap_part1 = solve_part1_hashmap(&input)?;
    
    println!("  🗂️ Grid implementation:    {}", grid_part1);
    println!("  🗺️ HashMap implementation: {}", hashmap_part1);
    
    if grid_part1 == hashmap_part1 {
        println!("  ✅ MATCH! Both implementations agree");
    } else {
        println!("  ❌ MISMATCH! Implementations differ");
        return Err(anyhow::anyhow!("Part 1 results don't match"));
    }
    
    // Test Part 2: Brightness levels
    println!("\n🔸 Part 2 (Brightness Levels) Comparison:");
    
    let grid_part2 = solve_part2_grid(&input)?;
    let hashmap_part2 = solve_part2_hashmap(&input)?;
    
    println!("  🗂️ Grid implementation:    {}", grid_part2);
    println!("  🗺️ HashMap implementation: {}", hashmap_part2);
    
    if grid_part2 == hashmap_part2 {
        println!("  ✅ MATCH! Both implementations agree");
    } else {
        println!("  ❌ MISMATCH! Implementations differ");
        return Err(anyhow::anyhow!("Part 2 results don't match"));
    }
    
    println!("\n🎉 VERIFICATION COMPLETE!");
    println!("📊 Both HashMap and Grid implementations produce identical results");
    
    // Performance comparison on smaller test cases
    performance_comparison(&input);
    
    Ok(())
}

/// Compare performance characteristics of HashMap vs Grid approaches
fn performance_comparison(input: &str) {
    println!("\n⚡ PERFORMANCE COMPARISON");
    println!("========================\n");
    
    let iterations = 10;
    
    // Time Grid implementation
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = solve_part1_grid(input);
        let _ = solve_part2_grid(input);
    }
    let grid_time = start.elapsed();
    
    // Time HashMap implementation
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = solve_part1_hashmap(input);
        let _ = solve_part2_hashmap(input);
    }
    let hashmap_time = start.elapsed();
    
    println!("📊 Performance Results ({} iterations):", iterations);
    println!("  🗂️ Grid:    {:?} ({:?} per solve)", grid_time, grid_time / (iterations * 2));
    println!("  🗺️ HashMap: {:?} ({:?} per solve)", hashmap_time, hashmap_time / (iterations * 2));
    
    let ratio = hashmap_time.as_nanos() as f64 / grid_time.as_nanos() as f64;
    if ratio > 1.0 {
        println!("  📈 Grid is {:.2}x faster than HashMap", ratio);
    } else {
        println!("  📈 HashMap is {:.2}x faster than Grid", 1.0 / ratio);
    }
    
    println!("\n🧠 Implementation Analysis:");
    println!("  🗂️ Grid Characteristics:");
    println!("    • Fixed 1000x1000 array allocation");
    println!("    • Constant-time access O(1)");
    println!("    • Cache-friendly memory layout");  
    println!("    • Memory usage: ~1MB (1M bool/u32 values)");
    
    println!("  🗺️ HashMap Characteristics:");
    println!("    • Sparse storage (only lit/bright lights)");
    println!("    • Hash computation overhead");
    println!("    • Memory scales with number of lit lights");
    println!("    • Unlimited coordinate range");
    
    println!("\n🎯 Use Case Recommendations:");
    println!("  🗂️ Use Grid when:");
    println!("    • Dense light patterns (many lights on)");
    println!("    • Fixed coordinate bounds");
    println!("    • Maximum performance needed");
    println!("    • Spatial locality is important");
    
    println!("  🗺️ Use HashMap when:");
    println!("    • Sparse light patterns (few lights on)");
    println!("    • Unlimited coordinate range needed");
    println!("    • Memory efficiency is priority");
    println!("    • Coordinate-based queries are common");
}