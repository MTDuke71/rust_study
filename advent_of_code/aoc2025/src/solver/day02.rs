//! Day 2: Gift Shop - Find invalid product IDs (repeated digit sequences)
//!
//! Part 1: ID is invalid if it's a sequence repeated exactly twice (55, 1010, 123123)
//! Part 2: ID is invalid if it's a sequence repeated at least twice (111, 123123123, etc.)

use anyhow::Result;

/// Check if a number is a "doubled" pattern (e.g., 55, 1010, 123123)
/// The string must be exactly two repetitions of a pattern.
fn is_doubled(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Must have even length to be doubled
    if !len.is_multiple_of(2) {
        return false;
    }

    let half = len / 2;
    let (first, second) = s.split_at(half);

    first == second
}

/// Check if a number is made of a pattern repeated at least twice.
/// For example: 111 (1 x3), 1212 (12 x2), 123123123 (123 x3)
fn is_repeated(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=len / 2 {
        // The total length must be divisible by the pattern length
        if !len.is_multiple_of(pattern_len) {
            continue;
        }

        let pattern = &s[..pattern_len];
        let repetitions = len / pattern_len;

        // Must repeat at least twice
        if repetitions < 2 {
            continue;
        }

        // Check if the entire string is just this pattern repeated
        let reconstructed: String = pattern.repeat(repetitions);
        if reconstructed == s {
            return true;
        }
    }

    false
}

/// Find all invalid IDs in a range using Part 2 rules and return their sum
fn sum_repeated_in_range(start: u64, end: u64) -> u64 {
    (start..=end).filter(|&n| is_repeated(n)).sum()
}

/// Parse input: "start-end,start-end,..." into Vec<(u64, u64)>
fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            let start: u64 = parts[0].parse().expect("valid start");
            let end: u64 = parts[1].parse().expect("valid end");
            (start, end)
        })
        .collect()
}

/// Find all invalid IDs in a range and return their sum
fn sum_invalid_in_range(start: u64, end: u64) -> u64 {
    (start..=end).filter(|&n| is_doubled(n)).sum()
}

pub fn solve_part1(input: &str) -> Result<String> {
    let ranges = parse_ranges(input);
    let sum: u64 = ranges
        .iter()
        .map(|&(start, end)| sum_invalid_in_range(start, end))
        .sum();
    Ok(sum.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let ranges = parse_ranges(input);
    let sum: u64 = ranges
        .iter()
        .map(|&(start, end)| sum_repeated_in_range(start, end))
        .sum();
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_doubled_simple() {
        // Single digit doubled
        assert!(is_doubled(11));
        assert!(is_doubled(22));
        assert!(is_doubled(55));
        assert!(is_doubled(99));
    }

    #[test]
    fn test_is_doubled_two_digit() {
        // Two digit doubled
        assert!(is_doubled(1010));
        assert!(is_doubled(6464));
        assert!(is_doubled(1212));
    }

    #[test]
    fn test_is_doubled_three_digit() {
        // Three digit doubled
        assert!(is_doubled(123123));
        assert!(is_doubled(999999));
    }

    #[test]
    fn test_is_doubled_large() {
        // Large number from example
        assert!(is_doubled(1188511885));
        assert!(is_doubled(222222));
        assert!(is_doubled(446446));
        assert!(is_doubled(38593859));
    }

    #[test]
    fn test_not_doubled() {
        // Not doubled - odd length
        assert!(!is_doubled(1));
        assert!(!is_doubled(123));
        assert!(!is_doubled(12345));

        // Not doubled - even length but different halves
        assert!(!is_doubled(12));
        assert!(!is_doubled(1234));
        assert!(!is_doubled(1011));
    }

    #[test]
    fn test_parse_ranges() {
        let input = "11-22,95-115,998-1012";
        let ranges = parse_ranges(input);
        assert_eq!(ranges, vec![(11, 22), (95, 115), (998, 1012)]);
    }

    #[test]
    fn test_sum_invalid_in_range() {
        // 11-22 has 11 and 22
        assert_eq!(sum_invalid_in_range(11, 22), 11 + 22);

        // 95-115 has 99
        assert_eq!(sum_invalid_in_range(95, 115), 99);

        // 998-1012 has 1010
        assert_eq!(sum_invalid_in_range(998, 1012), 1010);
    }

    #[test]
    fn test_example_from_problem() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = solve_part1(input).unwrap();
        assert_eq!(result, "1227775554");
    }

    // Part 2 tests
    #[test]
    fn test_is_repeated_simple() {
        // Single digit repeated (at least twice)
        assert!(is_repeated(11)); // 1 x2
        assert!(is_repeated(111)); // 1 x3
        assert!(is_repeated(1111)); // 1 x4
        assert!(is_repeated(99)); // 9 x2
        assert!(is_repeated(999)); // 9 x3
    }

    #[test]
    fn test_is_repeated_multi_digit() {
        // Two digit patterns
        assert!(is_repeated(1212)); // 12 x2
        assert!(is_repeated(121212)); // 12 x3
        assert!(is_repeated(1212121212)); // 12 x5

        // Three digit patterns
        assert!(is_repeated(123123)); // 123 x2
        assert!(is_repeated(123123123)); // 123 x3
    }

    #[test]
    fn test_is_repeated_from_example() {
        // From Part 2 example
        assert!(is_repeated(12341234)); // 1234 x2
        assert!(is_repeated(123123123)); // 123 x3
        assert!(is_repeated(1212121212)); // 12 x5
        assert!(is_repeated(1111111)); // 1 x7
    }

    #[test]
    fn test_not_repeated() {
        // Single digits can't repeat
        assert!(!is_repeated(1));
        assert!(!is_repeated(9));

        // Not a repeating pattern
        assert!(!is_repeated(12));
        assert!(!is_repeated(123));
        assert!(!is_repeated(1234));
        assert!(!is_repeated(12345));

        // Almost repeated but not quite
        assert!(!is_repeated(1213)); // 12, 13 - not same pattern
    }

    #[test]
    fn test_part2_example_ranges() {
        // 95-115 now has 99 and 111
        assert_eq!(sum_repeated_in_range(95, 115), 99 + 111);

        // 998-1012 now has 999 and 1010
        assert_eq!(sum_repeated_in_range(998, 1012), 999 + 1010);

        // 565653-565659 now has 565656
        assert_eq!(sum_repeated_in_range(565653, 565659), 565656);

        // 824824821-824824827 now has 824824824
        assert_eq!(sum_repeated_in_range(824824821, 824824827), 824824824);

        // 2121212118-2121212124 now has 2121212121
        assert_eq!(sum_repeated_in_range(2121212118, 2121212124), 2121212121);
    }

    #[test]
    fn test_part2_example_from_problem() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = solve_part2(input).unwrap();
        assert_eq!(result, "4174379265");
    }
}
