//! # Parser Utilities
//!
//! Common parsing patterns for Advent of Code input formats.

use anyhow::{Context, Result};

/// Parse lines into a `Vec<T>` where T: FromStr
pub fn parse_lines<T>(input: &str) -> Result<Vec<T>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<T>().context("Failed to parse line"))
        .collect()
}

/// Parse a grid of characters
pub fn parse_char_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

/// Parse numbers from a line (whitespace or comma separated)
pub fn parse_numbers<T>(line: &str) -> Result<Vec<T>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    line.split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<T>().context("Failed to parse number"))
        .collect()
}

/// Split input into sections separated by blank lines
pub fn parse_sections(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = "1\n2\n3\n";
        let result: Vec<i32> = parse_lines(input).unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_char_grid() {
        let input = "abc\ndef\n";
        let grid = parse_char_grid(input);
        assert_eq!(grid, vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
    }

    #[test]
    fn test_parse_numbers() {
        let result: Vec<i32> = parse_numbers("1 2 3").unwrap();
        assert_eq!(result, vec![1, 2, 3]);

        let result: Vec<i32> = parse_numbers("1,2,3").unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }
}
