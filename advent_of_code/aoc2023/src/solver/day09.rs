//! # Day 9: Mirage Maintenance
//!
//! Extrapolate sequences by computing differences until reaching all zeros.
//!
//! ## Part 1
//! For each sequence, compute successive difference sequences until all zeros,
//! then extrapolate the next value by working bottom-up.
//!
//! ## Part 2
//! Same algorithm but extrapolate backwards (previous value) instead of forwards.
//! Use first value - extrapolated difference instead of last value + difference.
//!
//! ## Algorithm
//! - Parse each line as a sequence of integers
//! - Build difference pyramid recursively
//! - Base case: all zeros → extrapolated value is 0
//! - Forward: extrapolate = last_value + extrapolate(differences)
//! - Backward: extrapolate = first_value - extrapolate(differences)
//! - Complexity: O(n²) per sequence where n is sequence length (depth × width)
//!
//! ## Mathematical Foundation
//!
//! **Finite Differences**: Polynomial interpolation and extrapolation.
//! This algorithm computes forward differences to detect polynomial patterns.
//! The method works because polynomial sequences have constant nth differences.
//!
//! **See**: `zettelkasten/math-foundations/finite-differences.md` for complete theory,
//! examples, and mathematical proofs of why this algorithm works.
//!
//! ## Performance
//! - Parsing: O(n) where n is total numbers in input
//! - Extrapolation: O(k × m²) where k is number of sequences, m is avg sequence length
//! - Total: O(k × m²)
//! - Part 1: ~132 µs (200 sequences, avg ~21 values each)
//! - Part 2: ~191 µs (200 sequences, avg ~21 values each)

use anyhow::Result;

/// Parse a line into a sequence of integers
fn parse_sequence(line: &str) -> Result<Vec<i64>> {
    line.split_whitespace()
        .map(|s| {
            s.parse::<i64>()
                .map_err(|e| anyhow::anyhow!("Parse error: {}", e))
        })
        .collect()
}

/// Compute the difference sequence (each element is next - current)
fn compute_differences(sequence: &[i64]) -> Vec<i64> {
    sequence.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

/// Check if all elements in a sequence are zero
fn all_zeros(sequence: &[i64]) -> bool {
    sequence.iter().all(|&x| x == 0)
}

/// Extrapolate the next value in a sequence
fn extrapolate_next(sequence: &[i64]) -> i64 {
    // Base case: if all zeros, next value is also 0
    if all_zeros(sequence) {
        return 0;
    }

    // Recursive case: compute differences and extrapolate
    let differences = compute_differences(sequence);
    let diff_extrapolated = extrapolate_next(&differences);

    // Next value = last value + extrapolated difference
    sequence.last().unwrap() + diff_extrapolated
}

/// Extrapolate the previous value in a sequence
fn extrapolate_prev(sequence: &[i64]) -> i64 {
    // Base case: if all zeros, previous value is also 0
    if all_zeros(sequence) {
        return 0;
    }

    // Recursive case: compute differences and extrapolate
    let differences = compute_differences(sequence);
    let diff_extrapolated = extrapolate_prev(&differences);

    // Previous value = first value - extrapolated difference
    sequence.first().unwrap() - diff_extrapolated
}

pub fn solve_part1(input: &str) -> Result<String> {
    let sum: i64 = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let sequence = parse_sequence(line)?;
            Ok(extrapolate_next(&sequence))
        })
        .collect::<Result<Vec<i64>>>()?
        .iter()
        .sum();

    Ok(sum.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    // Part 2: Extrapolate backwards (previous values)
    let sum: i64 = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let sequence = parse_sequence(line)?;
            Ok(extrapolate_prev(&sequence))
        })
        .collect::<Result<Vec<i64>>>()?
        .iter()
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_parse_sequence() {
        let result = parse_sequence("0 3 6 9 12 15").unwrap();
        assert_eq!(result, vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn test_compute_differences() {
        let sequence = vec![0, 3, 6, 9, 12, 15];
        let diffs = compute_differences(&sequence);
        assert_eq!(diffs, vec![3, 3, 3, 3, 3]);
    }

    #[test]
    fn test_all_zeros() {
        assert!(all_zeros(&[0, 0, 0]));
        assert!(!all_zeros(&[0, 1, 0]));
        assert!(!all_zeros(&[1, 2, 3]));
    }

    #[test]
    fn test_extrapolate_next_simple() {
        // Sequence: 0 3 6 9 12 15
        // Diffs:      3 3 3  3  3
        // Next:                 18
        let sequence = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(extrapolate_next(&sequence), 18);
    }

    #[test]
    fn test_extrapolate_next_example2() {
        // Sequence: 1 3 6 10 15 21
        // Expected: 28
        let sequence = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(extrapolate_next(&sequence), 28);
    }

    #[test]
    fn test_extrapolate_next_example3() {
        // Sequence: 10 13 16 21 30 45
        // Expected: 68
        let sequence = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(extrapolate_next(&sequence), 68);
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "114");
    }

    #[test]
    fn test_extrapolate_prev_simple() {
        // Sequence: 0 3 6 9 12 15
        // Diffs:      3 3 3  3  3
        // Prev:    -3
        let sequence = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(extrapolate_prev(&sequence), -3);
    }

    #[test]
    fn test_extrapolate_prev_example() {
        // Sequence: 10 13 16 21 30 45
        // Expected: 5 (working backwards from the example)
        let sequence = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(extrapolate_prev(&sequence), 5);
    }

    #[test]
    fn test_part2_example() {
        // Example should give 2 for part 2 (based on typical AoC patterns)
        let result = solve_part2(EXAMPLE).unwrap();
        // The sum of backward extrapolations: -3 + 0 + 5 = 2
        assert_eq!(result, "2");
    }
}
