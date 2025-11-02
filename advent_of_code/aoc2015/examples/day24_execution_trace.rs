use aoc2015::solver::day24::*;
use std::include_str;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use include_str! to embed the example input at compile time
    let input = include_str!("../inputs/day24_example.txt");

    println!("🎄 AoC 2015 Day 24: It Hangs in the Balance - Execution Trace\n");

    println!("Input packages: {:?}", parse_weights(&input)?);

    // Compute total and target
    let weights = parse_weights(&input)?;
    let total: u64 = weights.iter().sum();
    println!("Total weight: {}", total);

    // Part 1: 3 groups
    let target_part1 = total / 3;
    println!("Part 1 - Target per group: {} (3 groups)", target_part1);

    match solve_part1(&input) {
        Ok(result) => println!("Part 1 Result: QE = {}", result),
        Err(e) => println!("Part 1 Error: {}", e),
    }

    // Part 2: 4 groups
    let target_part2 = total / 4;
    println!("\nPart 2 - Target per group: {} (4 groups)", target_part2);

    match solve_part2(&input) {
        Ok(result) => println!("Part 2 Result: QE = {}", result),
        Err(e) => println!("Part 2 Error: {}", e),
    }

    Ok(())
}