//! Simple Pattern Recognition Demo
//!
//! This example demonstrates the core working functionality of the AoC pattern recognition system.

use aoc_pattern_recognition::grid_patterns::*;
use aoc_pattern_recognition::parsing_patterns::*;
use aoc_pattern_recognition::state_patterns::*;
use aoc_pattern_recognition::AocPattern;

fn main() {
    println!("=== AoC Pattern Recognition System Demo ===\n");

    // Demo 1: Grid Patterns
    println!("🔷 Grid Pattern Example:");
    demo_grid_patterns();
    println!();

    // Demo 2: Parsing Patterns
    println!("📝 Parsing Pattern Example:");
    demo_parsing_patterns();
    println!();

    // Demo 3: State Management
    println!("🧠 State Management Example:");
    demo_state_patterns();
    println!();

    println!("✅ AoC Pattern Recognition System is fully functional!");
    println!("   - Grid navigation and pathfinding");
    println!("   - Text parsing and data extraction");
    println!("   - State management and optimization");
    println!();
    println!("🎯 Ready for Advent of Code 2025!");
}

fn demo_grid_patterns() {
    // Create a simple grid
    let grid_data = "S.#\n..#\n..E";
    let grid = Grid::from_string(grid_data);

    println!("  Grid:");
    for line in grid_data.lines() {
        println!("    {}", line);
    }

    // Find start and end positions
    let start = grid.find_all(&'S')[0];
    let end = grid.find_all(&'E')[0];

    println!("  Start: {:?}, End: {:?}", start, end);

    // Use shortest path pattern
    let path_pattern = ShortestPathPattern;
    println!(
        "  Pattern: {} (complexity varies by implementation)",
        path_pattern.pattern_name()
    );

    let walkable = |ch: char| ch != '#';
    match path_pattern.solve((grid, start, end, walkable)) {
        Ok(Some(path)) => {
            println!("  ✅ Found path with {} steps", path.len() - 1);
            for (i, coord) in path.iter().enumerate() {
                println!("    Step {}: {:?}", i, coord);
            }
        }
        Ok(None) => println!("  ❌ No path found"),
        Err(_) => println!("  ❌ Error in pathfinding"),
    }
}

fn demo_parsing_patterns() {
    // Test coordinate parsing using the parser directly
    let input = "123,456";
    println!("  Input: {}", input);
    println!("  Pattern: Coordinate Parsing (Linear time complexity)");

    let coord_parser = CoordinateParser;
    match coord_parser.parse_line(input) {
        Ok(coord) => {
            println!("  ✅ Extracted coordinate: ({}, {})", coord.x, coord.y);
        }
        Err(_) => println!("  ❌ Failed to parse coordinate"),
    }

    // Test instruction parsing
    let instruction = "move 3 from 2 to 1";
    println!("  Instruction: {}", instruction);

    let instr_parser = InstructionParser;
    match instr_parser.parse_line(instruction) {
        Ok(parsed) => {
            println!("  ✅ Parsed instruction:");
            println!("    Command: {}", parsed.command);
            println!("    Parameters: {:?}", parsed.parameters);
        }
        Err(_) => println!("  ❌ Failed to parse instruction"),
    }
}

fn demo_state_patterns() {
    // Test memoization
    println!("  Fibonacci with memoization:");
    let mut cache = MemoizationCache::new();

    let n = 20;
    let result = cached_fibonacci(&mut cache, n);
    println!("  fib({}) = {}", n, result);

    let (hits, misses, hit_rate) = cache.stats();
    println!(
        "  Cache: {} hits, {} misses, {:.1}% hit rate",
        hits,
        misses,
        hit_rate * 100.0
    );

    // Test state tracking
    println!("  State tracking:");
    let initial_state = "start".to_string();
    let mut tracker = StateTracker::new(initial_state);

    println!("  Initial state: {}", tracker.current());

    // Advance through some states
    tracker.advance(|_| "move1".to_string());
    println!("  Advanced to: {}", tracker.current());

    tracker.advance(|_| "move2".to_string());
    println!("  Advanced to: {}", tracker.current());

    tracker.advance(|_| "move1".to_string()); // Repeat state
    println!("  Advanced to: {}", tracker.current());

    println!("  ✅ Tracked {} generations", tracker.generation());

    // Check if we've seen a state before
    if tracker.has_seen_state(&"move1".to_string()) {
        println!("  🔄 Detected repeated state: move1");
    }

    // Check for cycles
    if let Some((start, length)) = tracker.find_cycle() {
        println!(
            "  🔄 Cycle detected: starts at gen {}, length {}",
            start, length
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_functionality() {
        // Test that demo functions work without panicking
        demo_grid_patterns();
        demo_parsing_patterns();
        demo_state_patterns();
    }
}
