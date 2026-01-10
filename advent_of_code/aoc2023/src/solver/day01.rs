//! # Day 1: Trebuchet?!
//!
//! ## Part 1
//! Extract first and last digits from each line, combine into two-digit numbers,
//! and sum all calibration values.
//!
//! ## Part 2
//! Same as Part 1, but digits can also be spelled out (one, two, three, etc.)
//!
//! ## Algorithm
//! - Part 1: Iterate chars, find first/last numeric digits - O(n)
//! - Part 2: Scan for digit words at each position - O(n*m) where m = max word length

use anyhow::Result;

/// Extract first and last numeric digits from a line (Part 1)
/// Searches forward for first digit, backward for last - avoids collecting all digits
fn extract_digits_part1(line: &str) -> Option<(u32, u32)> {
    let first = line.chars().find_map(|c| c.to_digit(10))?;
    let last = line.chars().rev().find_map(|c| c.to_digit(10))?;
    Some((first, last))
}

/// Word-to-digit mapping for Part 2
const DIGIT_WORDS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

/// Extract digit at position - handles both numeric chars and spelled-out words
fn digit_at_position(s: &str, pos: usize) -> Option<u32> {
    let remainder = &s[pos..];

    // Check for numeric digit first
    if let Some(digit) = remainder.chars().next()?.to_digit(10) {
        return Some(digit);
    }

    // Check for spelled-out digits
    for (word, value) in DIGIT_WORDS {
        if remainder.starts_with(word) {
            return Some(value);
        }
    }

    None
}

/// Extract first and last digits including spelled-out words (Part 2)
/// Forward search for first, backward search for last - with early termination
fn extract_digits_part2(line: &str) -> Option<(u32, u32)> {
    // Forward: find first position with a digit
    let first = (0..line.len()).find_map(|pos| digit_at_position(line, pos))?;

    // Backward: find last position with a digit (iterate positions in reverse)
    let last = (0..line.len())
        .rev()
        .find_map(|pos| digit_at_position(line, pos))?;

    Some((first, last))
}

pub fn solve_part1(input: &str) -> Result<String> {
    let sum: u32 = input
        .lines()
        .filter_map(|line| {
            let (first, last) = extract_digits_part1(line)?;
            Some(first * 10 + last)
        })
        .sum();

    Ok(sum.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let sum: u32 = input
        .lines()
        .filter_map(|line| {
            let (first, last) = extract_digits_part2(line)?;
            Some(first * 10 + last)
        })
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_PART2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_extract_digits() {
        assert_eq!(extract_digits_part1("1abc2"), Some((1, 2)));
        assert_eq!(extract_digits_part1("pqr3stu8vwx"), Some((3, 8)));
        assert_eq!(extract_digits_part1("a1b2c3d4e5f"), Some((1, 5)));
        assert_eq!(extract_digits_part1("treb7uchet"), Some((7, 7)));
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE_PART1).unwrap();
        assert_eq!(result, "142");
    }

    #[test]
    fn test_extract_digits_part2() {
        // Test individual lines from Part 2 example
        assert_eq!(extract_digits_part2("two1nine"), Some((2, 9)));
        assert_eq!(extract_digits_part2("eightwothree"), Some((8, 3)));
        assert_eq!(extract_digits_part2("abcone2threexyz"), Some((1, 3)));
        assert_eq!(extract_digits_part2("xtwone3four"), Some((2, 4)));
        assert_eq!(extract_digits_part2("4nineeightseven2"), Some((4, 2)));
        assert_eq!(extract_digits_part2("zoneight234"), Some((1, 4)));
        assert_eq!(extract_digits_part2("7pqrstsixteen"), Some((7, 6)));
    }

    #[test]
    fn test_part2_example() {
        // Sum: 29 + 83 + 13 + 24 + 42 + 14 + 76 = 281
        let result = solve_part2(EXAMPLE_PART2).unwrap();
        assert_eq!(result, "281");
    }
}
