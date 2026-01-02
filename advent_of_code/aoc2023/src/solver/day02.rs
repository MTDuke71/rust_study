//! # AoC 2023 Day 2: Cube Conundrum
//!
//! ## Part 1
//! Determine which games would have been possible if the bag contained
//! only 12 red cubes, 13 green cubes, and 14 blue cubes.
//! Sum the IDs of possible games.
//!
//! ## Part 2
//! (To be revealed after Part 1)
//!
//! ## Algorithm
//! - Parse game records: "Game X: subset1; subset2; ..."
//! - For each game, check if any reveal exceeds limits
//! - Sum IDs of valid games
//!
//! ## Complexity
//! - Time: O(n * m) where n = games, m = avg reveals per game
//! - Space: O(1) - process line by line

use anyhow::{Context, Result};

/// Represents a single cube reveal (e.g., "3 blue")
#[derive(Debug, Clone, Copy)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    /// Check if this cube set is possible given the limits
    fn is_possible(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        self.red <= max_red && self.green <= max_green && self.blue <= max_blue
    }

    /// Calculate the power of this cube set (red * green * blue)
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    /// Update this set to contain the maximum of each color
    fn update_max(&mut self, other: &CubeSet) {
        self.red = self.red.max(other.red);
        self.green = self.green.max(other.green);
        self.blue = self.blue.max(other.blue);
    }
}

/// Parse a single cube reveal like "3 blue, 4 red"
fn parse_cube_set(s: &str) -> Result<CubeSet> {
    let mut set = CubeSet::new();
    
    for cube_str in s.split(',') {
        let parts: Vec<&str> = cube_str.trim().split_whitespace().collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid cube format: {}", cube_str);
        }
        
        let count: u32 = parts[0].parse()
            .context("Failed to parse cube count")?;
        let color = parts[1];
        
        match color {
            "red" => set.red += count,
            "green" => set.green += count,
            "blue" => set.blue += count,
            _ => anyhow::bail!("Unknown color: {}", color),
        }
    }
    
    Ok(set)
}

/// Parse a game line like "Game 1: 3 blue, 4 red; 1 red, 2 green"
/// Returns (game_id, is_possible)
fn parse_game(line: &str, max_red: u32, max_green: u32, max_blue: u32) -> Result<(u32, bool)> {
    // Split "Game X: ..." into parts
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid game format: {}", line);
    }
    
    // Extract game ID from "Game X"
    let game_id: u32 = parts[0]
        .trim()
        .strip_prefix("Game ")
        .context("Missing 'Game ' prefix")?
        .parse()
        .context("Failed to parse game ID")?;
    
    // Parse each reveal separated by semicolons
    let reveals = parts[1].split(';');
    let mut is_possible = true;
    
    for reveal in reveals {
        let cube_set = parse_cube_set(reveal)?;
        if !cube_set.is_possible(max_red, max_green, max_blue) {
            is_possible = false;
            break;
        }
    }
    
    Ok((game_id, is_possible))
}

pub fn solve_part1(input: &str) -> Result<String> {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    
    let sum: u32 = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            match parse_game(line, MAX_RED, MAX_GREEN, MAX_BLUE) {
                Ok((id, is_possible)) if is_possible => Some(id),
                _ => None,
            }
        })
        .sum();
    
    Ok(sum.to_string())
}

/// Parse a game line and find the minimum cube set needed
/// Returns (game_id, minimum_cube_set)
fn parse_game_minimum(line: &str) -> Result<(u32, CubeSet)> {
    // Split "Game X: ..." into parts
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid game format: {}", line);
    }
    
    // Extract game ID from "Game X"
    let game_id: u32 = parts[0]
        .trim()
        .strip_prefix("Game ")
        .context("Missing 'Game ' prefix")?
        .parse()
        .context("Failed to parse game ID")?;
    
    // Find the minimum cubes needed by tracking max of each color
    let mut min_cubes = CubeSet::new();
    
    for reveal in parts[1].split(';') {
        let cube_set = parse_cube_set(reveal)?;
        min_cubes.update_max(&cube_set);
    }
    
    Ok((game_id, min_cubes))
}

pub fn solve_part2(input: &str) -> Result<String> {
    let sum: u32 = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            parse_game_minimum(line).ok().map(|(_, min_set)| min_set.power())
        })
        .sum();
    
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "8");
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result, "2286");
    }

    #[test]
    fn test_cube_power() {
        let set = CubeSet { red: 4, green: 2, blue: 6 };
        assert_eq!(set.power(), 48); // 4 * 2 * 6 = 48
    }

    #[test]
    fn test_minimum_cubes_game1() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (_, min_set) = parse_game_minimum(line).unwrap();
        assert_eq!(min_set.red, 4);
        assert_eq!(min_set.green, 2);
        assert_eq!(min_set.blue, 6);
        assert_eq!(min_set.power(), 48);
    }


    #[test]
    fn test_parse_cube_set() {
        let set = parse_cube_set("3 blue, 4 red").unwrap();
        assert_eq!(set.red, 4);
        assert_eq!(set.blue, 3);
        assert_eq!(set.green, 0);
    }

    #[test]
    fn test_cube_set_possible() {
        let set = CubeSet { red: 12, green: 13, blue: 14 };
        assert!(set.is_possible(12, 13, 14));
        
        let set = CubeSet { red: 20, green: 6, blue: 8 };
        assert!(!set.is_possible(12, 13, 14));
    }
}
