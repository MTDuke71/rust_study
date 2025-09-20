
use anyhow::Result;
// use crate::parser::*;
// use crate::grid::*;

/// Day 04: template demonstrating reuse of parser/grid helpers.
pub fn solve_part1(input: &str) -> Result<String> {
    // Example template: count non-empty lines
    let count = input.lines().filter(|s| !s.trim().is_empty()).count();
    Ok(count.to_string())
}

pub fn solve_part2(_input: &str) -> Result<String> {
    Ok(String::from("UNIMPLEMENTED"))
}
