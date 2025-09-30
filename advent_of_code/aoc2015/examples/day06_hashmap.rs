//! Day 6: Probably a Fire Hazard - HashMap Implementation
//! 
//! Alternative implementation using HashMap instead of Mission6 Grid
//! 
//! **Key Differences from Grid Version**:
//! - HashMap<(i32, i32), bool> for Part 1 (sparse storage)
//! - HashMap<(i32, i32), u32> for Part 2 (brightness levels)
//! - Only stores "interesting" cells (lit lights or brightness > 0)
//! - Memory efficient for sparse patterns
//! - Explicit coordinate-based operations
//! 
//! **Trade-offs**:
//! - ✅ Memory efficient for sparse grids
//! - ✅ No bounds checking needed
//! - ✅ Natural coordinate-based API
//! - ❌ Slightly slower for dense operations
//! - ❌ No spatial locality optimizations

use std::collections::HashMap;
use anyhow::Result;

/// Day 06: Probably a Fire Hazard - HashMap Implementation
/// Demonstrates HashMap usage for 2D coordinate-based problems
pub fn solve_part1_hashmap(input: &str) -> Result<String> {
    // Use HashMap to store only lit lights (sparse representation)
    let mut lights: HashMap<(i32, i32), bool> = HashMap::new();
    
    for line in input.lines().filter(|line| !line.is_empty()) {
        apply_light_command_hashmap(&mut lights, line)?;
    }
    
    // Count lights that are on (HashMap only stores true values)
    let count = lights.values().filter(|&&is_on| is_on).count();
    Ok(count.to_string())
}

pub fn solve_part2_hashmap(input: &str) -> Result<String> {
    // Use HashMap to store brightness levels (sparse representation)
    let mut lights: HashMap<(i32, i32), u32> = HashMap::new();
    
    for line in input.lines().filter(|line| !line.is_empty()) {
        apply_brightness_command_hashmap(&mut lights, line)?;
    }
    
    // Sum all brightness values
    let brightness: u32 = lights.values().sum();
    Ok(brightness.to_string())
}

/// Apply light commands to HashMap-based grid
fn apply_light_command_hashmap(lights: &mut HashMap<(i32, i32), bool>, line: &str) -> Result<()> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    let (command, coord1_str, coord2_str) = if parts[0] == "toggle" {
        ("toggle", parts[1], parts[3])
    } else {
        (parts[1], parts[2], parts[4])  // parts[1] = "on"/"off"
    };
    
    // Parse coordinates
    let coord1_parts: Vec<&str> = coord1_str.split(',').collect();
    let coord2_parts: Vec<&str> = coord2_str.split(',').collect();
    
    let x1: i32 = coord1_parts[0].parse()?;
    let y1: i32 = coord1_parts[1].parse()?;
    let x2: i32 = coord2_parts[0].parse()?;
    let y2: i32 = coord2_parts[1].parse()?;
    
    // Apply command to rectangular region
    for y in y1..=y2 {
        for x in x1..=x2 {
            let coord = (x, y);
            match command {
                "on" => {
                    lights.insert(coord, true);
                },
                "off" => {
                    lights.remove(&coord); // Remove from HashMap (sparse representation)
                },
                "toggle" => {
                    let current = lights.get(&coord).copied().unwrap_or(false);
                    if current {
                        lights.remove(&coord); // Turn off = remove from HashMap
                    } else {
                        lights.insert(coord, true); // Turn on = add to HashMap
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
        (parts[1], parts[2], parts[4])  // parts[1] = "on"/"off"
    };
    
    // Parse coordinates
    let coord1_parts: Vec<&str> = coord1_str.split(',').collect();
    let coord2_parts: Vec<&str> = coord2_str.split(',').collect();
    
    let x1: i32 = coord1_parts[0].parse()?;
    let y1: i32 = coord1_parts[1].parse()?;
    let x2: i32 = coord2_parts[0].parse()?;
    let y2: i32 = coord2_parts[1].parse()?;
    
    // Apply brightness changes to rectangular region
    for y in y1..=y2 {
        for x in x1..=x2 {
            let coord = (x, y);
            match command {
                "on" => {
                    *lights.entry(coord).or_insert(0) += 1;
                },
                "off" => {
                    let current = lights.get(&coord).copied().unwrap_or(0);
                    if current > 1 {
                        lights.insert(coord, current - 1);
                    } else if current == 1 {
                        lights.remove(&coord); // Brightness 0 = remove from HashMap
                    }
                    // If current == 0, do nothing (already not in HashMap)
                },
                "toggle" => {
                    *lights.entry(coord).or_insert(0) += 2;
                },
                _ => return Err(anyhow::anyhow!("Unknown brightness command: {}", command)),
            }
        }
    }
    
    Ok(())
}

/// Compare performance and memory usage between Grid and HashMap approaches
fn performance_comparison() {
    println!("🔍 GRID vs HASHMAP COMPARISON");
    println!("=============================\n");
    
    let sample_input = "turn on 0,0 through 999,999\nturn off 499,499 through 500,500\ntoggle 0,0 through 999,0";
    
    println!("📝 Sample commands:");
    for line in sample_input.lines() {
        println!("  {}", line);
    }
    println!();
    
    // Test HashMap implementation
    println!("🗺️ HashMap Implementation:");
    
    let start = std::time::Instant::now();
    let mut lights_hashmap: HashMap<(i32, i32), bool> = HashMap::new();
    for line in sample_input.lines() {
        let _ = apply_light_command_hashmap(&mut lights_hashmap, line);
    }
    let hashmap_time = start.elapsed();
    
    let hashmap_count = lights_hashmap.len();
    let hashmap_on = lights_hashmap.values().filter(|&&is_on| is_on).count();
    
    println!("  ⏱️ Processing time: {:?}", hashmap_time);
    println!("  💾 HashMap entries: {}", hashmap_count);
    println!("  💡 Lights on: {}", hashmap_on);
    println!("  📊 Memory efficiency: Only stores {} entries vs 1M grid cells", hashmap_count);
    
    println!("\n💡 HashMap Advantages:");
    println!("  ✅ Sparse representation - only stores lit lights");
    println!("  ✅ No memory waste for empty areas");
    println!("  ✅ Natural coordinate-based operations");
    println!("  ✅ No bounds checking required");
    println!("  ✅ Easy to extend beyond 1000x1000 bounds");
    
    println!("\n⚠️ HashMap Trade-offs:");
    println!("  ❌ Hash computation overhead per access");
    println!("  ❌ No spatial locality for cache efficiency");
    println!("  ❌ Iteration order is not predictable");
    println!("  ❌ Slightly more complex for dense operations");
}

/// Demonstrate HashMap-specific features for AoC Day 6
fn hashmap_features_demo() {
    println!("\n🎯 HASHMAP-SPECIFIC FEATURES");
    println!("===========================\n");
    
    let mut lights: HashMap<(i32, i32), u32> = HashMap::new();
    
    // Demonstrate sparse storage
    println!("🔸 Sparse Storage Demo:");
    let _ = apply_brightness_command_hashmap(&mut lights, "turn on 100,100 through 102,102");
    let _ = apply_brightness_command_hashmap(&mut lights, "toggle 500,500 through 501,501");
    
    println!("  Created lights at scattered positions:");
    for ((x, y), brightness) in &lights {
        println!("    ({}, {}) -> brightness {}", x, y, brightness);
    }
    println!("  HashMap size: {} (vs 1,000,000 for full grid)", lights.len());
    
    // Demonstrate coordinate-based operations
    println!("\n🔸 Coordinate-Based Operations:");
    
    // Find all lights in a specific region
    let region_lights: Vec<_> = lights.iter()
        .filter(|((x, y), _)| *x >= 100 && *x <= 200 && *y >= 100 && *y <= 200)
        .collect();
    
    println!("  Lights in region (100,100) to (200,200): {}", region_lights.len());
    
    // Find brightest lights
    if let Some(((x, y), brightness)) = lights.iter().max_by_key(|(_, &brightness)| brightness) {
        println!("  Brightest light: ({}, {}) with brightness {}", x, y, brightness);
    }
    
    // Demonstrate easy bounds extension
    println!("\n🔸 Unlimited Bounds Demo:");
    let _ = apply_brightness_command_hashmap(&mut lights, "turn on -1000,-1000 through -999,-999");
    println!("  Added lights at negative coordinates: (-1000,-1000) to (-999,-999)");
    println!("  HashMap naturally handles any coordinate range!");
    
    // Show final statistics
    println!("\n📊 Final Statistics:");
    println!("  Total lights stored: {}", lights.len());
    println!("  Total brightness: {}", lights.values().sum::<u32>());
    println!("  Memory usage: ~{} bytes (estimated)", lights.len() * (8 + 4 + 24)); // coordinate + brightness + HashMap overhead
}

/// Demonstrate both Part 1 and Part 2 with HashMap approach
fn comprehensive_demo() {
    println!("🎄 DAY 6 HASHMAP IMPLEMENTATION DEMO");
    println!("===================================\n");
    
    let sample_input = "turn on 0,0 through 999,999\nturn off 499,499 through 500,500\ntoggle 0,0 through 999,0\ntoggle 0,0 through 0,999";
    
    println!("📝 Test Commands:");
    for (i, line) in sample_input.lines().enumerate() {
        println!("  {}. {}", i + 1, line);
    }
    
    // Part 1 Demo
    println!("\n🔸 Part 1 (Boolean Lights) - HashMap Implementation:");
    match solve_part1_hashmap(sample_input) {
        Ok(result) => println!("  💡 Total lights on: {}", result),
        Err(e) => println!("  ❌ Error: {}", e),
    }
    
    // Part 2 Demo  
    println!("\n🔸 Part 2 (Brightness Levels) - HashMap Implementation:");
    match solve_part2_hashmap(sample_input) {
        Ok(result) => println!("  🌟 Total brightness: {}", result),
        Err(e) => println!("  ❌ Error: {}", e),
    }
    
    performance_comparison();
    hashmap_features_demo();
    
    println!("\n🎉 HashMap implementation complete!");
    println!("💡 Perfect for sparse grids and coordinate-based operations!");
}

fn main() {
    comprehensive_demo();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hashmap_part1_basic_operations() {
        let mut lights = HashMap::new();
        
        // Turn on a single light
        apply_light_command_hashmap(&mut lights, "turn on 0,0 through 0,0").unwrap();
        assert_eq!(lights.get(&(0, 0)), Some(&true));
        assert_eq!(lights.len(), 1);
        
        // Turn off the light
        apply_light_command_hashmap(&mut lights, "turn off 0,0 through 0,0").unwrap();
        assert_eq!(lights.get(&(0, 0)), None); // Should be removed from HashMap
        assert_eq!(lights.len(), 0);
        
        // Toggle a light on
        apply_light_command_hashmap(&mut lights, "toggle 1,1 through 1,1").unwrap();
        assert_eq!(lights.get(&(1, 1)), Some(&true));
        
        // Toggle it off
        apply_light_command_hashmap(&mut lights, "toggle 1,1 through 1,1").unwrap();
        assert_eq!(lights.get(&(1, 1)), None); // Should be removed
    }
    
    #[test]
    fn test_hashmap_part1_rectangular_region() {
        let mut lights = HashMap::new();
        
        // Turn on a 2x2 region
        apply_light_command_hashmap(&mut lights, "turn on 0,0 through 1,1").unwrap();
        assert_eq!(lights.len(), 4); // 4 lights in 2x2 region
        
        // Check all positions
        assert_eq!(lights.get(&(0, 0)), Some(&true));
        assert_eq!(lights.get(&(0, 1)), Some(&true));
        assert_eq!(lights.get(&(1, 0)), Some(&true));
        assert_eq!(lights.get(&(1, 1)), Some(&true));
    }
    
    #[test]
    fn test_hashmap_part2_brightness_operations() {
        let mut lights = HashMap::new();
        
        // Turn on increases brightness by 1
        apply_brightness_command_hashmap(&mut lights, "turn on 0,0 through 0,0").unwrap();
        assert_eq!(lights.get(&(0, 0)), Some(&1));
        
        // Toggle increases by 2
        apply_brightness_command_hashmap(&mut lights, "toggle 0,0 through 0,0").unwrap();
        assert_eq!(lights.get(&(0, 0)), Some(&3)); // 1 + 2 = 3
        
        // Turn off decreases by 1
        apply_brightness_command_hashmap(&mut lights, "turn off 0,0 through 0,0").unwrap();
        assert_eq!(lights.get(&(0, 0)), Some(&2)); // 3 - 1 = 2
        
        // Turn off again
        apply_brightness_command_hashmap(&mut lights, "turn off 0,0 through 0,0").unwrap();
        assert_eq!(lights.get(&(0, 0)), Some(&1)); // 2 - 1 = 1
        
        // Turn off once more should remove from HashMap
        apply_brightness_command_hashmap(&mut lights, "turn off 0,0 through 0,0").unwrap();
        assert_eq!(lights.get(&(0, 0)), None); // Should be removed when brightness = 0
    }
    
    #[test]
    fn test_hashmap_sparse_efficiency() {
        let mut lights = HashMap::new();
        
        // Turn on just a few scattered lights
        apply_light_command_hashmap(&mut lights, "turn on 0,0 through 0,0").unwrap();
        apply_light_command_hashmap(&mut lights, "turn on 500,500 through 500,500").unwrap();
        apply_light_command_hashmap(&mut lights, "turn on 999,999 through 999,999").unwrap();
        
        // HashMap should only store 3 entries, not 1,000,000
        assert_eq!(lights.len(), 3);
        
        // But the "virtual grid" would have 1,000,000 positions
        // HashMap is much more memory efficient for sparse patterns
    }
    
    #[test] 
    fn test_hashmap_solve_functions() {
        let input = "turn on 0,0 through 999,999\nturn off 499,499 through 500,500";
        
        // Part 1
        let part1_result = solve_part1_hashmap(input).unwrap();
        let lights_on: usize = part1_result.parse().unwrap();
        assert_eq!(lights_on, 1000000 - 4); // All on except 2x2 region turned off
        
        // Part 2
        let part2_result = solve_part2_hashmap(input).unwrap();
        let brightness: u32 = part2_result.parse().unwrap();
        assert_eq!(brightness, 1000000 - 4); // Each light has brightness 1, except 4 turned off
    }
}