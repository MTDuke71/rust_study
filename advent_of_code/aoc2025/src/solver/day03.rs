//! Day 3: Lobby - Battery Bank Maximum Joltage
//!
//! # Part 1
//! Find the maximum two-digit number that can be formed by selecting
//! exactly two digits from each row (in order), then sum all maximums.
//!
//! # Part 2
//! Find the maximum **12-digit** number from each bank (same rules, more digits).
//!
//! # Algorithm
//!
//! ## Part 1 (2 digits)
//! For each bank, find positions i < j that maximize `digits[i] * 10 + digits[j]`.
//!
//! ## Part 2 (k digits) - Greedy Selection
//! To select k digits from n digits (k <= n) to form the maximum number:
//! 1. For position 1: pick the largest digit from indices 0..=(n-k)
//!    (must leave k-1 digits for remaining positions)
//! 2. For position 2: pick the largest digit from (prev_idx+1)..=(n-k+1)
//! 3. Continue until all k positions filled
//!
//! **Key Insight**: At each step, greedily pick the largest available digit
//! that still leaves enough digits for the remaining positions.
//!
//! # Mission Integration
//! This problem is simple enough that no complex data structures are needed.
//! It's primarily a string parsing and greedy optimization problem.

use anyhow::{Context, Result};

/// Parse a single bank (line) into a vector of digit values
fn parse_bank(line: &str) -> Result<Vec<u8>> {
    line.trim()
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|d| d as u8)
                .ok_or_else(|| anyhow::anyhow!("Invalid digit '{}'", c))
        })
        .collect()
}

/// Find the maximum two-digit joltage from a single bank
///
/// Returns the maximum value of `digits[i] * 10 + digits[j]` where i < j
fn max_joltage_from_bank(digits: &[u8]) -> u32 {
    if digits.len() < 2 {
        return 0;
    }

    let mut max_joltage = 0;

    // For each possible first digit position
    for i in 0..digits.len() - 1 {
        let first_digit = digits[i] as u32;

        // Find the maximum digit in remaining positions
        let max_second = digits[i + 1..]
            .iter()
            .map(|&d| d as u32)
            .max()
            .unwrap_or(0);

        let joltage = first_digit * 10 + max_second;
        max_joltage = max_joltage.max(joltage);
    }

    max_joltage
}

/// Part 1: Sum of maximum joltage from each bank
///
/// For each bank, find the two batteries (digits) that form the largest
/// two-digit number when turned on (preserving order).
pub fn solve_part1(input: &str) -> Result<String> {
    let total: u32 = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(line_num, line)| {
            let digits = parse_bank(line)
                .with_context(|| format!("Failed to parse bank on line {}", line_num + 1))?;
            Ok(max_joltage_from_bank(&digits))
        })
        .collect::<Result<Vec<u32>>>()?
        .into_iter()
        .sum();

    Ok(total.to_string())
}

/// Find the maximum k-digit number by selecting k digits from the bank (preserving order)
///
/// Uses greedy algorithm: at each position, pick the largest digit that still
/// leaves enough digits for remaining positions.
///
/// **Tie-breaking**: When multiple positions have the same max digit,
/// pick the EARLIEST one to leave maximum flexibility for later positions.
fn max_k_digit_joltage(digits: &[u8], k: usize) -> u64 {
    if digits.len() < k || k == 0 {
        return 0;
    }

    let n = digits.len();
    let mut result: u64 = 0;
    let mut start_idx = 0;

    for positions_remaining in (1..=k).rev() {
        // We need to pick 1 digit now, and leave (positions_remaining - 1) for later
        // So we can search from start_idx to (n - positions_remaining) inclusive
        let end_idx = n - positions_remaining;

        // Find the maximum digit in this range
        let max_digit = (start_idx..=end_idx)
            .map(|i| digits[i])
            .max()
            .unwrap();

        // Find the FIRST index with this max digit (to leave more room for later)
        let best_idx = (start_idx..=end_idx)
            .find(|&i| digits[i] == max_digit)
            .unwrap();

        // Add this digit to result
        result = result * 10 + max_digit as u64;

        // Next search starts after this position
        start_idx = best_idx + 1;
    }

    result
}

/// Part 2: Sum of maximum 12-digit joltage from each bank
///
/// For each bank, find the 12 batteries (digits) that form the largest
/// 12-digit number when turned on (preserving order).
pub fn solve_part2(input: &str) -> Result<String> {
    const K: usize = 12;

    let total: u64 = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(line_num, line)| {
            let digits = parse_bank(line)
                .with_context(|| format!("Failed to parse bank on line {}", line_num + 1))?;
            Ok(max_k_digit_joltage(&digits, K))
        })
        .collect::<Result<Vec<u64>>>()?
        .into_iter()
        .sum();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn test_parse_bank() {
        let digits = parse_bank("12345").unwrap();
        assert_eq!(digits, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_max_joltage_first_two() {
        // 987654321111111 -> max is 98 (first two digits)
        let digits = parse_bank("987654321111111").unwrap();
        assert_eq!(max_joltage_from_bank(&digits), 98);
    }

    #[test]
    fn test_max_joltage_first_and_last() {
        // 811111111111119 -> max is 89 (8 from first, 9 from last)
        let digits = parse_bank("811111111111119").unwrap();
        assert_eq!(max_joltage_from_bank(&digits), 89);
    }

    #[test]
    fn test_max_joltage_last_two() {
        // 234234234234278 -> max is 78 (last two)
        let digits = parse_bank("234234234234278").unwrap();
        assert_eq!(max_joltage_from_bank(&digits), 78);
    }

    #[test]
    fn test_max_joltage_scattered() {
        // 818181911112111 -> max is 92
        // 9 appears at position 6, then we need max after position 6
        // After position 6: 11112111 -> max is 2
        // So 9*10 + 2 = 92
        let digits = parse_bank("818181911112111").unwrap();
        assert_eq!(max_joltage_from_bank(&digits), 92);
    }

    #[test]
    fn test_example_part1() {
        // 98 + 89 + 78 + 92 = 357
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "357");
    }

    #[test]
    fn test_simple_two_digits() {
        assert_eq!(max_joltage_from_bank(&[1, 2]), 12);
        assert_eq!(max_joltage_from_bank(&[2, 1]), 21);
        assert_eq!(max_joltage_from_bank(&[5, 5]), 55);
    }

    #[test]
    fn test_three_digits() {
        // 123 -> choices: 12, 13, 23 -> max is 23
        assert_eq!(max_joltage_from_bank(&[1, 2, 3]), 23);
        // 321 -> choices: 32, 31, 21 -> max is 32
        assert_eq!(max_joltage_from_bank(&[3, 2, 1]), 32);
        // 192 -> choices: 19, 12, 92 -> max is 92
        assert_eq!(max_joltage_from_bank(&[1, 9, 2]), 92);
    }

    // Part 2 tests

    #[test]
    fn test_max_k_digit_simple() {
        // Select 3 digits from "12345" -> max is 345 (skip 1,2)
        let digits = parse_bank("12345").unwrap();
        assert_eq!(max_k_digit_joltage(&digits, 3), 345);

        // Select 4 digits from "12345" -> max is 2345 (skip 1)
        assert_eq!(max_k_digit_joltage(&digits, 4), 2345);

        // Select all 5 digits -> 12345
        assert_eq!(max_k_digit_joltage(&digits, 5), 12345);
    }

    #[test]
    fn test_max_12_digit_example1() {
        // 987654321111111 (15 digits) -> select 12 -> 987654321111
        // Skip 3 digits from the end (the 1s)
        let digits = parse_bank("987654321111111").unwrap();
        assert_eq!(max_k_digit_joltage(&digits, 12), 987654321111);
    }

    #[test]
    fn test_max_12_digit_example2() {
        // 811111111111119 (15 digits) -> select 12 -> 811111111119
        // Keep the 8 at start, skip three 1s, keep the 9 at end
        let digits = parse_bank("811111111111119").unwrap();
        assert_eq!(max_k_digit_joltage(&digits, 12), 811111111119);
    }

    #[test]
    fn test_max_12_digit_example3() {
        // 234234234234278 (15 digits) -> select 12 -> 434234234278
        // Skip a 2, 3, and another 2 near the start
        let digits = parse_bank("234234234234278").unwrap();
        assert_eq!(max_k_digit_joltage(&digits, 12), 434234234278);
    }

    #[test]
    fn test_max_12_digit_example4() {
        // 818181911112111 (15 digits) -> select 12 -> 888911112111
        // Keep the 8s and 9, skip some 1s near the front
        let digits = parse_bank("818181911112111").unwrap();
        assert_eq!(max_k_digit_joltage(&digits, 12), 888911112111);
    }

    #[test]
    fn test_example_part2() {
        // 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result, "3121910778619");
    }
}
