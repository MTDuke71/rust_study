//! Real AoC Problem Solutions Demo
//! 
//! This example demonstrates how to apply pattern recognition to solve
//! actual Advent of Code problems from past years.

use aoc_pattern_recognition::grid_patterns::*;
use aoc_pattern_recognition::parsing_patterns::*;
use std::collections::{HashMap, HashSet};

fn main() {
    println!("=== Real AoC Problem Solutions Demo ===\n");

    // Demo 1: AoC 2021 Day 15 - Chiton (Shortest Path)
    println!("🏔️ Demo 1: AoC 2021 Day 15 - Chiton (Shortest Path)");
    demo_aoc_2021_day15();
    println!();

    // Demo 2: AoC 2022 Day 6 - Tuning Trouble (Sliding Window)
    println!("📡 Demo 2: AoC 2022 Day 6 - Tuning Trouble (Sliding Window)");
    demo_aoc_2022_day6();
    println!();

    // Demo 3: AoC 2021 Day 6 - Lanternfish (Memoization)
    println!("🐟 Demo 3: AoC 2021 Day 6 - Lanternfish (Memoization)");
    demo_aoc_2021_day6();
    println!();

    // Demo 4: AoC 2020 Day 12 - Rain Risk (Instruction Parsing)
    println!("🧭 Demo 4: AoC 2020 Day 12 - Rain Risk (Instruction Parsing)");
    demo_aoc_2020_day12();
    println!();

    // Demo 5: AoC 2017 Day 14 - Disk Defragmentation (Flood Fill)
    println!("💾 Demo 5: AoC 2017 Day 14 - Disk Defragmentation (Flood Fill)");
    demo_aoc_2017_day14();
    println!();

    // Demo 6: Pattern Recognition Summary
    println!("🧠 Demo 6: Pattern Recognition Summary");
    demo_pattern_summary();
    println!();

    println!("Real AoC problem solutions demo completed! ✅");
}

fn demo_aoc_2021_day15() {
    println!("  📋 Problem: Find the lowest total risk path through a cave system");
    println!("  🎯 Pattern: Shortest Path (Dijkstra's Algorithm)");
    println!();

    let cave_map = "1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581";
    
    println!("  🗺️ Cave risk map:");
    for line in cave_map.lines() {
        println!("    {}", line);
    }
    
    // Parse into grid
    let grid = Grid::from_string(cave_map);
    let start = Coord::new(0, 0);
    let end = Coord::new(grid.width as i32 - 1, grid.height as i32 - 1);
    
    println!("  Start: {:?}, End: {:?}", start, end);
    
    // Use shortest path pattern with custom cost function
    println!("  🔍 Applying Shortest Path Pattern:");
    println!("    • Grid navigation with weighted edges");
    println!("    • Each cell has a risk value (movement cost)");
    println!("    • Need Dijkstra instead of BFS for weighted graph");
    
    // Implement Dijkstra for this specific problem
    let total_risk = solve_chiton(&grid, start, end);
    println!("  ✅ Lowest total risk path: {}", total_risk);
    
    // Part 2 scaling
    println!();
    println!("  📈 Part 2: 5x5 tiled cave (pattern recognition):");
    println!("    • Same pattern, larger input");
    println!("    • Grid transformation + shortest path");
    println!("    • Risk values increment with distance from original tile");
    
    let expanded_grid = expand_cave_grid(&grid, 5);
    let expanded_end = Coord::new(expanded_grid.width as i32 - 1, expanded_grid.height as i32 - 1);
    let expanded_risk = solve_chiton(&expanded_grid, start, expanded_end);
    println!("    ✅ Expanded cave lowest risk: {}", expanded_risk);
    
    println!();
    println!("  💡 Pattern Recognition Keys:");
    println!("    • 'path' + 'lowest' + '2D grid' → Shortest path");
    println!("    • 'risk/cost per cell' → Weighted graph (Dijkstra)");
    println!("    • 'corner to corner' → Standard grid navigation");
}

fn demo_aoc_2022_day6() {
    println!("  📋 Problem: Find first position where 4 consecutive characters are unique");
    println!("  🎯 Pattern: Sliding Window");
    println!();
    
    let datastreams = vec![
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
    ];
    
    for (i, datastream) in datastreams.iter().enumerate() {
        println!("  📡 Datastream {}: {}", i + 1, datastream);
        
        // Part 1: Start-of-packet marker (4 unique characters)
        let packet_pos = find_unique_window(datastream, 4);
        println!("    Start-of-packet marker at position: {}", packet_pos);
        
        // Part 2: Start-of-message marker (14 unique characters)  
        let message_pos = find_unique_window(datastream, 14);
        println!("    Start-of-message marker at position: {}", message_pos);
        println!();
    }
    
    println!("  💡 Pattern Recognition Keys:");
    println!("    • 'first position where N consecutive...' → Sliding window");
    println!("    • 'all different/unique' → Set-based validation");
    println!("    • 'marker detection' → Stream processing pattern");
    println!();
    
    println!("  ⚡ Implementation Strategy:");
    println!("    • Maintain window of size N");
    println!("    • Check uniqueness with HashSet");
    println!("    • Slide window one character at a time");
    println!("    • Return position when unique condition met");
}

fn demo_aoc_2021_day6() {
    println!("  📋 Problem: Simulate exponentially growing lanternfish population");
    println!("  🎯 Pattern: Memoization / State Tracking");
    println!();
    
    let initial_fish = "3,4,3,1,2";
    println!("  🐟 Initial fish timers: {}", initial_fish);
    
    // Parse initial state
    let fish_timers: Vec<u8> = initial_fish.split(',').map(|s| s.parse().unwrap()).collect();
    
    println!("  📊 Simulation Results:");
    
    // Naive approach (works for small days)
    let small_days = 18;
    let naive_count = simulate_fish_naive(fish_timers.clone(), small_days);
    println!("    Day {}: {} fish (naive simulation)", small_days, naive_count);
    
    // Optimized approach with state counting
    let medium_days = 80;
    let optimized_count = simulate_fish_optimized(fish_timers.clone(), medium_days);
    println!("    Day {}: {} fish (optimized)", medium_days, optimized_count);
    
    // Large simulation (impossible without optimization)
    let large_days = 256;
    let large_count = simulate_fish_optimized(fish_timers.clone(), large_days);
    println!("    Day {}: {} fish (optimized)", large_days, large_count);
    
    println!();
    println!("  💡 Pattern Recognition Keys:");
    println!("    • 'exponential growth' + 'large numbers' → Memoization needed");
    println!("    • 'simulate for N days' → State tracking over time");
    println!("    • 'too many to track individually' → Count states, not entities");
    println!();
    
    println!("  🧠 Optimization Insight:");
    println!("    • Don't track individual fish - track counts by timer value");
    println!("    • Use array[9] where index = timer, value = count");
    println!("    • Each day: rotate array and add births");
    println!("    • Reduces O(2ⁿ) to O(n) complexity");
}

fn demo_aoc_2020_day12() {
    println!("  📋 Problem: Navigate ship using sequence of movement instructions");
    println!("  🎯 Pattern: Instruction Parsing + State Tracking");
    println!();
    
    let navigation_input = "F10\nN3\nF7\nR90\nF11";
    println!("  🧭 Navigation instructions:");
    for line in navigation_input.lines() {
        println!("    {}", line);
    }
    
    // Parse instructions - for navigation, we need to handle format like "F10", "N3"
    let instructions: Vec<Instruction> = navigation_input.lines()
        .map(|line| {
            let line = line.trim();
            let command = line.chars().next().unwrap_or(' ').to_string();
            let value = line[1..].to_string();
            Instruction {
                command,
                parameters: vec![value],
            }
        })
        .collect();
    
    println!("  🚢 Part 1: Ship navigation (direct movement)");
    let (x1, y1) = navigate_ship_direct(&instructions);
    println!("    Final position: ({}, {})", x1, y1);
    println!("    Manhattan distance: {}", x1.abs() + y1.abs());
    
    println!();
    println!("  🧭 Part 2: Waypoint navigation (relative movement)");
    let (x2, y2) = navigate_ship_waypoint(&instructions);
    println!("    Final position: ({}, {})", x2, y2);
    println!("    Manhattan distance: {}", x2.abs() + y2.abs());
    
    println!();
    println!("  💡 Pattern Recognition Keys:");
    println!("    • 'sequence of instructions' → Instruction parsing");
    println!("    • 'F10', 'N3', 'R90' format → Command + value pattern");
    println!("    • 'navigation' + 'position tracking' → State management");
    println!("    • 'Manhattan distance' → Grid coordinate math");
    println!();
    
    println!("  🔧 Implementation Strategy:");
    println!("    • Parse each line into (command, value) pairs");
    println!("    • Track ship state: position + direction/waypoint");
    println!("    • Apply transformation rules for each command");
    println!("    • Handle coordinate rotation for turns");
}

fn demo_aoc_2017_day14() {
    println!("  📋 Problem: Count connected regions in a 2D grid of used/free disk blocks");
    println!("  🎯 Pattern: Grid Generation + Flood Fill");
    println!();
    
    // Simplified example (actual problem uses knot hash)
    let example_grid = "##.#....#.\n.#.#.....#\n....#.#...\n#.#......#\n.##.#.....\n##...#....\n....#.....\n..#.#....#\n.#...#.##.\n.#..##.#..";
    
    println!("  💾 Disk fragmentation map (simplified):");
    for line in example_grid.lines() {
        println!("    {}", line);
    }
    
    let grid = Grid::from_string(example_grid);
    
    // Count used squares
    let used_count = grid.find_all(&'#').len();
    println!("  📊 Used squares: {}", used_count);
    
    // Count connected regions using flood fill
    let regions = count_connected_regions(&grid);
    println!("  🔗 Connected regions: {}", regions);
    
    println!();
    println!("  💡 Pattern Recognition Keys:");
    println!("    • '2D grid' + 'connected' → Flood fill / connected components");
    println!("    • 'count regions' → Start flood fill from each unvisited used cell");
    println!("    • 'hash generation' → Grid creation pattern");
    println!();
    
    println!("  🧠 Algorithm Insight:");
    println!("    • For each used cell not yet in a region:");
    println!("      1. Start flood fill (DFS/BFS)");
    println!("      2. Mark all connected used cells as visited");
    println!("      3. Increment region count");
    println!("    • Time: O(width × height), Space: O(width × height)");
}

fn demo_pattern_summary() {
    println!("  🧠 AoC Pattern Recognition Summary");
    println!();
    
    let pattern_applications = vec![
        (
            "Shortest Path",
            vec!["2021 Day 15 (Chiton)", "2019 Day 18 (Keys/Doors)", "2016 Day 13 (Maze)"],
            "Grid navigation with costs, pathfinding through obstacles"
        ),
        (
            "Sliding Window", 
            vec!["2022 Day 6 (Packet Detection)", "2020 Day 9 (Encoding Error)", "2017 Day 1 (Captcha)"],
            "Finding patterns in sequences, validation with fixed-size buffers"
        ),
        (
            "Memoization",
            vec!["2021 Day 6 (Lanternfish)", "2019 Day 14 (Reactions)", "2015 Day 20 (Presents)"],
            "Exponential growth problems, recursive calculations with overlapping subproblems"
        ),
        (
            "Instruction Parsing",
            vec!["2020 Day 12 (Navigation)", "2019 Day 2 (Intcode)", "2016 Day 8 (Screen)"],
            "Command sequences, state machines, robot/turtle graphics"
        ),
        (
            "Flood Fill",
            vec!["2017 Day 14 (Defrag)", "2018 Day 6 (Areas)", "2021 Day 9 (Basins)"],
            "Connected components, region counting, boundary detection"
        ),
        (
            "Cycle Detection",
            vec!["2017 Day 6 (Memory)", "2020 Day 14 (Docking)", "2018 Day 12 (Plants)"],
            "State machines that repeat, infinite sequences with patterns"
        ),
    ];
    
    for (pattern_name, examples, description) in pattern_applications {
        println!("  🎯 {}:", pattern_name);
        println!("    Description: {}", description);
        println!("    AoC Examples:");
        for example in examples {
            println!("      • {}", example);
        }
        println!();
    }
    
    println!("  🏆 Master Pattern Recognition Strategy:");
    println!("    1. 📖 Read problem statement carefully");
    println!("    2. 🔍 Identify key phrases and constraints");
    println!("    3. 🎯 Match to known patterns");
    println!("    4. 🚀 Choose optimal implementation approach");
    println!("    5. 🧪 Test with provided examples");
    println!("    6. ⚡ Optimize for time/space if needed");
    println!();
    
    println!("  💡 Pro Tips for AoC 2025:");
    println!("    • Practice pattern recognition daily");
    println!("    • Build a personal pattern library");
    println!("    • Time yourself implementing each pattern");
    println!("    • Study solutions from previous years");
    println!("    • Focus on clean, readable implementations first");
    println!("    • Optimize only when necessary");
}

// Helper functions for the demos

fn solve_chiton(grid: &Grid<char>, start: Coord, end: Coord) -> i32 {
    let mut distances = HashMap::new();
    let mut queue = std::collections::BinaryHeap::new();
    
    distances.insert(start, 0);
    queue.push(std::cmp::Reverse((0, start)));
    
    while let Some(std::cmp::Reverse((dist, pos))) = queue.pop() {
        if pos == end {
            return dist;
        }
        
        if distances.get(&pos).unwrap_or(&i32::MAX) < &dist {
            continue;
        }
        
        for neighbor in pos.neighbors_4() {
            if !grid.in_bounds(&neighbor) {
                continue;
            }
            
            let risk = grid.get(&neighbor).unwrap().to_digit(10).unwrap() as i32;
            let new_dist = dist + risk;
            
            if new_dist < *distances.get(&neighbor).unwrap_or(&i32::MAX) {
                distances.insert(neighbor, new_dist);
                queue.push(std::cmp::Reverse((new_dist, neighbor)));
            }
        }
    }
    
    -1 // No path found
}

fn expand_cave_grid(grid: &Grid<char>, factor: usize) -> Grid<char> {
    let new_width = grid.width * factor;
    let new_height = grid.height * factor;
    let mut expanded = Grid::new(new_width, new_height);
    
    for tile_y in 0..factor {
        for tile_x in 0..factor {
            let risk_increase = (tile_x + tile_y) as u32;
            
            for y in 0..grid.height {
                for x in 0..grid.width {
                    let coord = Coord::new(x as i32, y as i32);
                    if let Some(original_char) = grid.get(&coord) {
                        let original_risk = original_char.to_digit(10).unwrap();
                        let new_risk = ((original_risk - 1 + risk_increase) % 9) + 1;
                        
                        let new_coord = Coord::new(
                            (tile_x * grid.width + x) as i32,
                            (tile_y * grid.height + y) as i32
                        );
                        expanded.set(new_coord, char::from_digit(new_risk, 10).unwrap());
                    }
                }
            }
        }
    }
    
    expanded
}

fn find_unique_window(data: &str, window_size: usize) -> usize {
    let chars: Vec<char> = data.chars().collect();
    
    for i in window_size..=chars.len() {
        let window = &chars[i-window_size..i];
        let mut unique = HashSet::new();
        
        for &ch in window {
            unique.insert(ch);
        }
        
        if unique.len() == window_size {
            return i; // Return 1-indexed position after the window (AoC standard)
        }
    }
    
    0 // Not found
}

fn simulate_fish_naive(mut fish: Vec<u8>, days: usize) -> usize {
    for _ in 0..days {
        let mut new_fish = Vec::new();
        
        for timer in fish.iter_mut() {
            if *timer == 0 {
                *timer = 6;
                new_fish.push(8);
            } else {
                *timer -= 1;
            }
        }
        
        fish.extend(new_fish);
    }
    
    fish.len()
}

fn simulate_fish_optimized(fish: Vec<u8>, days: usize) -> u64 {
    let mut counts = [0u64; 9];
    
    // Initialize counts
    for timer in fish {
        counts[timer as usize] += 1;
    }
    
    // Simulate days
    for _ in 0..days {
        let births = counts[0];
        
        // Shift all counts down
        for i in 0..8 {
            counts[i] = counts[i + 1];
        }
        
        // Reset reproducing fish and add births
        counts[6] += births;
        counts[8] = births;
    }
    
    counts.iter().sum()
}

fn navigate_ship_direct(instructions: &[Instruction]) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut direction = 90; // East
    
    for instr in instructions {
        // For navigation instructions like "F10", "N3", the first parameter contains the value
        let value = instr.parameters.get(0)
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);
            
        match instr.command.as_str() {
            "N" => y += value,
            "S" => y -= value,
            "E" => x += value,
            "W" => x -= value,
            "L" => direction = (direction - value + 360) % 360,
            "R" => direction = (direction + value) % 360,
            "F" => {
                match direction {
                    0 => y += value,
                    90 => x += value,
                    180 => y -= value,
                    270 => x -= value,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    
    (x, y)
}

fn navigate_ship_waypoint(instructions: &[Instruction]) -> (i32, i32) {
    let mut ship_x = 0;
    let mut ship_y = 0;
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;
    
    for instr in instructions {
        // For navigation instructions like "F10", "N3", the first parameter contains the value
        let value = instr.parameters.get(0)
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);
            
        match instr.command.as_str() {
            "N" => waypoint_y += value,
            "S" => waypoint_y -= value,
            "E" => waypoint_x += value,
            "W" => waypoint_x -= value,
            "L" => {
                for _ in 0..(value / 90) {
                    let temp = waypoint_x;
                    waypoint_x = -waypoint_y;
                    waypoint_y = temp;
                }
            }
            "R" => {
                for _ in 0..(value / 90) {
                    let temp = waypoint_x;
                    waypoint_x = waypoint_y;
                    waypoint_y = -temp;
                }
            }
            "F" => {
                ship_x += waypoint_x * value;
                ship_y += waypoint_y * value;
            }
            _ => {}
        }
    }
    
    (ship_x, ship_y)
}

fn count_connected_regions(grid: &Grid<char>) -> usize {
    let mut visited = HashSet::new();
    let mut regions = 0;
    
    for y in 0..grid.height {
        for x in 0..grid.width {
            let coord = Coord::new(x as i32, y as i32);
            
            if grid.get(&coord) == Some(&'#') && !visited.contains(&coord) {
                // Start new region
                regions += 1;
                
                // Flood fill this region
                let mut stack = vec![coord];
                
                while let Some(pos) = stack.pop() {
                    if visited.contains(&pos) {
                        continue;
                    }
                    
                    visited.insert(pos);
                    
                    for neighbor in pos.neighbors_4() {
                        if grid.get(&neighbor) == Some(&'#') && !visited.contains(&neighbor) {
                            stack.push(neighbor);
                        }
                    }
                }
            }
        }
    }
    
    regions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chiton_pathfinding() {
        let simple_grid = Grid::from_string("111\n191\n111");
        let start = Coord::new(0, 0);
        let end = Coord::new(2, 2);
        
        let risk = solve_chiton(&simple_grid, start, end);
        // Path: start(free) -> (0,1):1 -> (0,2):1 -> (1,2):1 -> (2,2):1 = 4
        assert_eq!(risk, 4);
    }
    
    #[test]
    fn test_unique_window_detection() {
        // "abcdefg": first 4 unique chars are at positions 0-3, marker ends at position 4 (1-indexed)
        assert_eq!(find_unique_window("abcdefg", 4), 4);
        // "abccdefg": first 4 unique chars are at positions 3-6, marker ends at position 7 (1-indexed)  
        assert_eq!(find_unique_window("abccdefg", 4), 7);
    }
    
    #[test]
    fn test_fish_simulation() {
        let fish = vec![3, 4, 3, 1, 2];
        assert_eq!(simulate_fish_optimized(fish.clone(), 18), 26);
        assert_eq!(simulate_fish_optimized(fish, 80), 5934);
    }
}