use aoc2015::solver::day07::{solve_part1_with_debug, solve_part2_with_debug};

fn main() -> anyhow::Result<()> {
    // Smaller test case to demonstrate debug functionality without overwhelming output
    let test_input = r#"123 -> x
456 -> y
x AND y -> d
NOT x -> h
1 -> b
d OR h -> a"#;

    println!("=== Day 7 DEBUG Demo (Condensed) ===");
    println!("Input circuit:");
    for line in test_input.lines() {
        println!("  {}", line);
    }
    println!();

    println!("=== Part 1 with DEBUG (shows command parsing & hash tables) ===");
    let part1_result = solve_part1_with_debug(test_input, true)?;
    println!();
    
    println!("=== Part 2 with DEBUG (shows b override & recalculation) ===");
    let part2_result = solve_part2_with_debug(test_input, true)?;
    
    println!();
    println!("🎯 Summary:");
    println!("Part 1: {} (wire a with b=1)", part1_result);
    println!("Part 2: {} (wire a with b={})", part2_result, part1_result);
    
    Ok(())
}