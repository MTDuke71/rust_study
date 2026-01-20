//! Day 20 Circuit Visualization
//! 
//! Run with: cargo run -p aoc2023 --example day20_visualize

use aoc2023::solver::day20;

fn main() {
    // Read puzzle input
    let input = include_str!("../inputs/day20.txt");
    
    // Visualize the circuit topology
    day20::visualize_circuit(input);
}
