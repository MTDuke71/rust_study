
use anyhow::Result;
// use crate::parser::*;
// use crate::grid::*;

/// Day 03: template demonstrating reuse of parser/grid helpers.
pub fn solve_part1(input: &str) -> Result<String> {
    let _input = input.trim();
    let mut visited = HashSet::new();
    Ok(String::from("UNIMPLEMENTED"))
}

pub fn solve_part2(_input: &str) -> Result<String> {
    Ok(String::from("UNIMPLEMENTED"))
}


// valid chars: ^[<>v^]+$
// ^ = up, v = down, < = left, > = right
// start at (0,0) and track all visited locations in a HashSet
// Total characters: 8192