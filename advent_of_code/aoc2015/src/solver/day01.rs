use anyhow::Result;

/// AoC 2015 Day 1: Not Quite Lisp
/// Santa is trying to deliver presents in a large apartment building, but he can't find the right floor.
/// The directions specify which floor to go to, using parentheses:
/// - '(' = go up one floor
/// - ')' = go down one floor
///   Start at floor 0 (ground floor).
///
/// Part 1: What floor does Santa end up on?
/// Part 2: What is the position of the first character that causes Santa to enter the basement (floor -1)?
pub fn solve_part1(input: &str) -> Result<String> {
    let floor = input.trim().chars().fold(0, |floor, c| {
        match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor, // ignore non-parentheses characters
        }
    });

    Ok(floor.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let mut floor = 0;

    for (position, c) in input.trim().chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => continue, // ignore non-parentheses characters
        }

        // Check if Santa entered the basement (floor -1)
        if floor == -1 {
            // Position is 1-indexed in the problem
            return Ok((position + 1).to_string());
        }
    }

    // If Santa never enters the basement
    Ok("Never enters basement".to_string())
}
