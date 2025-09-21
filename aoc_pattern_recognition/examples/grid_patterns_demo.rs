//! Grid Pattern Recognition Demo
//! 
//! This example demonstrates how to identify and solve grid-based AoC problems
//! using the grid pattern recognition system.

use aoc_pattern_recognition::grid_patterns::*;
use aoc_pattern_recognition::{AocPattern, PatternComplexity};

fn main() {
    println!("=== Grid Pattern Recognition Demo ===\n");

    // Demo 1: Basic Grid Operations
    println!("🔲 Demo 1: Basic Grid Operations");
    demo_basic_grid_operations();
    println!();

    // Demo 2: Shortest Path Finding (AoC 2021 Day 15 style)
    println!("🎯 Demo 2: Shortest Path Finding");
    demo_shortest_path();
    println!();

    // Demo 3: Flood Fill Connected Regions (AoC 2017 Day 14 style)  
    println!("🌊 Demo 3: Flood Fill Connected Regions");
    demo_flood_fill();
    println!();

    // Demo 4: Grid Transformations (AoC 2020 Day 20 style)
    println!("🔄 Demo 4: Grid Transformations");
    demo_grid_transformations();
    println!();

    // Demo 5: Pattern Recognition in Action
    println!("🧠 Demo 5: Pattern Recognition in Action");
    demo_pattern_recognition();
    println!();

    println!("All grid pattern demos completed! ✅");
}

fn demo_basic_grid_operations() {
    // Create grid from string (common AoC input format)
    let input = "..#.\n.##.\n..#.";
    let grid = Grid::from_string(input);
    
    println!("  Grid from input:");
    println!("  {}", input);
    println!("  Dimensions: {}x{}", grid.width, grid.height);
    
    // Basic grid queries
    let obstacles = grid.find_all(&'#');
    println!("  Found {} obstacles at: {:?}", obstacles.len(), obstacles);
    
    // Test coordinate operations
    let center = Coord::new(1, 1);
    let corner = Coord::new(3, 2);
    let distance = center.manhattan_distance(&corner);
    println!("  Manhattan distance from {:?} to {:?}: {}", center, corner, distance);
    
    // Test neighbors
    let neighbors = center.neighbors_4();
    println!("  4-neighbors of {:?}: {:?}", center, neighbors);
    
    // Check bounds
    let in_bounds = neighbors.iter().filter(|&n| grid.in_bounds(n)).count();
    println!("  Neighbors in bounds: {}/{}", in_bounds, neighbors.len());
}

fn demo_shortest_path() {
    // Create a maze-like grid
    let maze = "S.#.E\n..#..\n###..\n.....\n##...";
    let grid = Grid::from_string(maze);
    
    println!("  Maze:");
    for line in maze.lines() {
        println!("    {}", line);
    }
    
    // Find start and end positions
    let start = grid.find_all(&'S')[0];
    let end = grid.find_all(&'E')[0];
    println!("  Start: {:?}, End: {:?}", start, end);
    
    // Define walkable function
    let walkable = |ch: char| ch != '#';
    
    // Use shortest path pattern
    let path_pattern = ShortestPathPattern;
    println!("  Pattern: {}", path_pattern.pattern_name());
    println!("  Complexity: {}", path_pattern.complexity());
    
    match path_pattern.solve((grid.clone(), start, end, walkable)) {
        Ok(Some(path)) => {
            println!("  ✅ Path found with {} steps:", path.len() - 1);
            for (i, coord) in path.iter().enumerate() {
                println!("    Step {}: {:?}", i, coord);
            }
        }
        Ok(None) => println!("  ❌ No path found"),
        Err(e) => println!("  ❌ Error: {}", e),
    }
}

fn demo_flood_fill() {
    // Create a grid with different regions
    let input = "AAABBB\nAAA..B\nA....B\nXXX..B\nXXXBBB";
    let grid = Grid::from_string(input);
    
    println!("  Grid with regions:");
    for line in input.lines() {
        println!("    {}", line);
    }
    
    let fill_pattern = FloodFillPattern;
    println!("  Pattern: {}", fill_pattern.pattern_name());
    println!("  Complexity: {}", fill_pattern.complexity());
    
    // Fill from different starting points
    let start_points = vec![
        (Coord::new(0, 0), 'A'),
        (Coord::new(5, 0), 'B'), 
        (Coord::new(0, 3), 'X'),
        (Coord::new(3, 2), '.'),
    ];
    
    for (start, target_char) in start_points {
        match fill_pattern.solve((grid.clone(), start, target_char)) {
            Ok(filled_region) => {
                println!("  ✅ Filled {} '{}' cells from {:?}",
                    filled_region.len(), target_char, start);
                
                // Show first few coordinates
                let coords: Vec<_> = filled_region.iter().take(5).collect();
                println!("    First few: {:?}{}", coords, 
                    if filled_region.len() > 5 { "..." } else { "" });
            }
            Err(e) => println!("  ❌ Error filling from {:?}: {}", start, e),
        }
    }
}

fn demo_grid_transformations() {
    // Create a small grid to transform
    let input = "AB\nCD";
    let original = Grid::from_string(input);
    
    println!("  Original grid:");
    println!("    {}", original.to_string());
    
    let rotation_pattern = GridRotationPattern;
    
    // Rotate clockwise
    let rotated_cw = rotation_pattern.rotate_clockwise(&original);
    println!("  Rotated clockwise:");
    println!("    {}", rotated_cw.to_string());
    
    // Rotate counter-clockwise
    let rotated_ccw = rotation_pattern.rotate_counter_clockwise(&original);
    println!("  Rotated counter-clockwise:");
    println!("    {}", rotated_ccw.to_string());
    
    // Flip horizontally
    let flipped = rotation_pattern.flip_horizontal(&original);
    println!("  Flipped horizontally:");
    println!("    {}", flipped.to_string());
    
    // Demonstrate that 4 clockwise rotations = original
    let mut current = original.clone();
    for i in 1..=4 {
        current = rotation_pattern.rotate_clockwise(&current);
        println!("  After {} rotations: {}", i, current.to_string());
    }
    
    // Verify we're back to original
    assert_eq!(current.to_string(), original.to_string());
    println!("  ✅ Four rotations returned to original!");
}

fn demo_pattern_recognition() {
    println!("  🧠 How to recognize grid patterns in AoC problems:");
    println!();
    
    let examples = vec![
        (
            "Find shortest path through cave system avoiding high-risk areas",
            vec!["Grid parsing", "Shortest path (BFS/Dijkstra)", "Pathfinding with obstacles"]
        ),
        (
            "Count number of separate islands in ocean map",
            vec!["Grid parsing", "Flood fill", "Connected components"]
        ),
        (
            "Rotate and flip image tiles to create larger picture",
            vec!["Grid parsing", "Grid transformations", "Pattern matching"]
        ),
        (
            "Simulate Conway's Game of Life for 1000 generations",
            vec!["Grid parsing", "Cellular automaton", "State evolution", "Cycle detection"]
        ),
        (
            "Find all positions reachable within N steps",
            vec!["Grid parsing", "BFS with distance", "Reachability analysis"]
        )
    ];
    
    for (problem_desc, expected_patterns) in examples {
        println!("  Problem: \"{}\"", problem_desc);
        println!("  Patterns to consider:");
        for pattern in expected_patterns {
            println!("    • {}", pattern);
        }
        println!();
    }
    
    println!("  🎯 Key Recognition Signals:");
    println!("    • '2D map/grid' → Grid parsing + navigation");
    println!("    • 'shortest path' → BFS or Dijkstra"); 
    println!("    • 'connected regions' → Flood fill");
    println!("    • 'rotate/flip' → Grid transformations");
    println!("    • 'simulate' → State evolution, possibly with cycles");
    println!("    • 'reachable positions' → BFS with constraints");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_demo_examples() {
        // Test that demo examples work correctly
        let input = "..#.\n.##.\n..#.";
        let grid = Grid::from_string(input);
        assert_eq!(grid.width, 4);
        assert_eq!(grid.height, 3);
        
        let obstacles = grid.find_all(&'#');
        assert_eq!(obstacles.len(), 4);
        
        // Test shortest path
        let maze = "S...E\n.....\n.....";
        let maze_grid = Grid::from_string(maze);
        let start = maze_grid.find_all(&'S')[0];
        let end = maze_grid.find_all(&'E')[0];
        
        let pattern = ShortestPathPattern;
        let result = pattern.solve((maze_grid, start, end, |ch| ch != '#')).unwrap();
        assert!(result.is_some());
    }
}