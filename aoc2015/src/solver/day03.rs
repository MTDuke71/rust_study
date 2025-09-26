
use anyhow::Result;
use std::collections::HashSet;
// use crate::parser::*;
// use crate::grid::*;


// Part 1: How many unique houses receive at least one present?
// valid chars: ^ v < >
// ^ = up, v = down, < = left, > = right
// start at (0,0) and track all visited locations in a HashSet
// Total characters: 8192
// coordinate: (x, y) where x=horizontal, y=vertical

// > delivers presents to 2 houses: one at the starting location, and one to the east.
// ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
// ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // > delivers presents to 2 houses: starting location + one to the east
        let result = solve_part1(">").unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_example_2() {
        // ^>v< delivers presents to 4 houses in a square
        let result = solve_part1("^>v<").unwrap();
        assert_eq!(result, "4");
    }

    #[test]
    fn test_example_3() {
        // ^v^v^v^v^v delivers presents to only 2 houses (back and forth)
        let result = solve_part1("^v^v^v^v^v").unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_starting_house_counted() {
        // Empty input should still count the starting house
        let result = solve_part1("").unwrap();
        assert_eq!(result, "1");
    }
}




