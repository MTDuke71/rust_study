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
//The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa, to deliver presents with him.

//Santa and Robo-Santa start at the same location (delivering two presents to the same starting house),
//then take turns moving based on instructions from the elf, who is eggnoggedly reading from the same script as the previous year.

//This year, how many houses receive at least one present?

//^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
//^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
//^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.

pub fn solve_part2(input: &str) -> Result<String> {
    let mut santa_coordinate = (0, 0);
    let mut robot_coordinate = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(santa_coordinate);

    let even_chars = input.trim().chars().step_by(2); // Take every 2nd element starting from 0

    let odd_chars = input
        .trim()
        .chars()
        .skip(1) // Skip first element
        .step_by(2); // Then take every 2nd element

    for c in even_chars {
        match c {
            '^' => santa_coordinate.1 += 1,
            'v' => santa_coordinate.1 -= 1,
            '<' => santa_coordinate.0 -= 1,
            '>' => santa_coordinate.0 += 1,
            _ => (), // ignore invalid characters
        }
        visited.insert(santa_coordinate);
    }
    for c in odd_chars {
        match c {
            '^' => robot_coordinate.1 += 1,
            'v' => robot_coordinate.1 -= 1,
            '<' => robot_coordinate.0 -= 1,
            '>' => robot_coordinate.0 += 1,
            _ => (), // ignore invalid characters
        }
        visited.insert(robot_coordinate);
    }
    Ok(visited.len().to_string())
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

    // Part 2 tests based on examples in comments
    #[test]
    fn test_part2_example_1() {
        // ^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
        let result = solve_part2("^v").unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_part2_example_2() {
        // ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
        let result = solve_part2("^>v<").unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_part2_example_3() {
        // ^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.
        let result = solve_part2("^v^v^v^v^v").unwrap();
        assert_eq!(result, "11");
    }

    #[test]
    fn test_part2_starting_house() {
        // Empty input should count starting house once (both Santa and Robo-Santa start there)
        let result = solve_part2("").unwrap();
        assert_eq!(result, "1");
    }
}
