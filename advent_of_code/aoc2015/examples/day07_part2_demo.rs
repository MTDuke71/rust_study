use aoc2015::solver::day07::{solve_part1, solve_part2};

fn main() -> anyhow::Result<()> {
    // Simple test case to demonstrate Part 2 behavior
    let test_input = r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
19138 -> b
d AND e -> a"#;

    println!("=== Day 7 Part 2 Demo ===");
    println!("Input circuit has: b = 19138");
    println!("Part 1 result (wire a): {}", solve_part1(test_input)?);
    println!("Part 2 result (wire a with b overridden): {}", solve_part2(test_input)?);
    
    Ok(())
}