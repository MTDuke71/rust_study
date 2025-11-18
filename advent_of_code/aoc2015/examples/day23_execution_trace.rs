/// Execution trace demonstration for Day 23: Opening the Turing Lock
/// Shows register values (a, b) throughout program execution
use aoc2015::solver::day23::get_execution_trace;

fn main() {
    println!("🎯 Day 23: Opening the Turing Lock - Execution Traces\n");

    // Read the example program
    let input = include_str!("../inputs/day23_example.txt");

    println!("Program to execute:");
    for (i, line) in input.lines().enumerate() {
        println!("{:2}: {}", i + 1, line);
    }
    println!();

    // Part 1: Register a starts at 0
    println!("=== PART 1: Register a = 0 ===");
    match get_execution_trace(input, 0) {
        Ok(trace) => println!("{}", trace),
        Err(e) => println!("Error: {}", e),
    }

    println!();

    // Part 2: Register a starts at 1
    println!("=== PART 2: Register a = 1 ===");
    match get_execution_trace(input, 1) {
        Ok(trace) => println!("{}", trace),
        Err(e) => println!("Error: {}", e),
    }
}
