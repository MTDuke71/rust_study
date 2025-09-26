
use anyhow::Result;
use std::collections::HashSet;
// use crate::parser::*;
// use crate::grid::*;


// Part 1: How many unique houses receive at least one present?
// valid chars: ^ v < >
// ^ = up, v = down, < = left, > = right
// start at (0,0) and track all visited locations in a HashSet
// Total characters: 8192

pub fn solve_part1(input: &str) -> Result<String> {
    let mut coordinate = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(coordinate);
    let houses = input.trim().chars();
    for c in houses {
        match c {
            '^' => coordinate.1 += 1,
            'v' => coordinate.1 -= 1,
            '<' => coordinate.0 -= 1,
            '>' => coordinate.0 += 1,
            _ => (), // ignore invalid characters
        }
        visited.insert(coordinate);
    }
    Ok(visited.len().to_string())
}

pub fn solve_part2(_input: &str) -> Result<String> {
    Ok(String::from("UNIMPLEMENTED"))
}




