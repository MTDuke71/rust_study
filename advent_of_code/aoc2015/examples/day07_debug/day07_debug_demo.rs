use aoc2015::solver::day07::{solve_part1_with_debug, solve_part2_with_debug};

fn main() -> anyhow::Result<()> {
    // Simple test case to demonstrate debug functionality
    let test_input = r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
1 -> b
d AND e -> a"#;

    println!("=== Day 7 DEBUG Demo ===");
    println!("Input circuit:");
    for line in test_input.lines() {
        println!("  {}", line);
    }
    println!();

    println!("=== Part 1 with DEBUG ===");
    let part1_result = solve_part1_with_debug(test_input, true)?;
    println!();
    
    println!("=== Part 2 with DEBUG ===");
    let part2_result = solve_part2_with_debug(test_input, true)?;
    
    println!();
    println!("Summary:");
    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
    
    Ok(())
}