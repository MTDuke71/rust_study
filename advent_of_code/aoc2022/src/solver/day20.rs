//! Day 20: Grove Positioning System
//!
//! Circular list mixing: each number moves forward/backward by its own value.
//! Part 1: Mix once, sum values at positions 1000, 2000, 3000 after 0.
//! Part 2: Multiply by decryption key 811589153, mix 10 times.

// ============================================================================
// Data Types
// ============================================================================

pub type ParsedData = Vec<i64>;

// ============================================================================
// Parsing
// ============================================================================

pub fn parse_input(input: &str) -> ParsedData {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

// ============================================================================
// Core mixing logic
// ============================================================================

/// Mix the list `rounds` times. Each element is tagged with its original index
/// so duplicates are tracked correctly. Returns the mixed list of values.
fn mix(values: &[i64], rounds: usize) -> Vec<i64> {
    let n = values.len();
    // Track (original_index, value) pairs
    let mut seq: Vec<(usize, i64)> = values.iter().copied().enumerate().collect();

    for _ in 0..rounds {
        for orig_idx in 0..n {
            // Find current position of this element
            let pos = seq.iter().position(|&(i, _)| i == orig_idx).unwrap();
            let (_, val) = seq[pos];

            if val == 0 {
                continue;
            }

            // Remove from current position
            seq.remove(pos);

            // Compute new position (mod n-1 because element is temporarily removed)
            let modulus = (n - 1) as i64;
            let new_pos = ((pos as i64 + val) % modulus + modulus) % modulus;
            seq.insert(new_pos as usize, (orig_idx, val));
        }
    }

    seq.into_iter().map(|(_, v)| v).collect()
}

/// Extract grove coordinates: values at positions 1000, 2000, 3000 after 0.
fn grove_coordinates(mixed: &[i64]) -> i64 {
    let n = mixed.len();
    let zero_pos = mixed.iter().position(|&v| v == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|offset| mixed[(zero_pos + offset) % n])
        .sum()
}

// ============================================================================
// Part 1
// ============================================================================

pub fn solve_part1(values: &ParsedData) -> i64 {
    let mixed = mix(values, 1);
    grove_coordinates(&mixed)
}

// ============================================================================
// Part 2
// ============================================================================

const DECRYPTION_KEY: i64 = 811_589_153;

pub fn solve_part2(values: &ParsedData) -> i64 {
    let scaled: Vec<i64> = values.iter().map(|&v| v * DECRYPTION_KEY).collect();
    let mixed = mix(&scaled, 10);
    grove_coordinates(&mixed)
}

// ============================================================================
// Entry point
// ============================================================================

pub fn solve(input: &str) -> (i64, i64) {
    let values = parse_input(input);
    (solve_part1(&values), solve_part2(&values))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1
2
-3
3
-2
0
4";

    #[test]
    fn test_parse() {
        let values = parse_input(EXAMPLE);
        assert_eq!(values, vec![1, 2, -3, 3, -2, 0, 4]);
    }

    #[test]
    fn test_mix_example() {
        let values = parse_input(EXAMPLE);
        let mixed = mix(&values, 1);
        // Result is circular — verify the sequence relative to 0
        let zero_pos = mixed.iter().position(|&v| v == 0).unwrap();
        let n = mixed.len();
        let rotated: Vec<i64> = (0..n).map(|i| mixed[(zero_pos + i) % n]).collect();
        assert_eq!(rotated, vec![0, 3, -2, 1, 2, -3, 4]);
    }

    #[test]
    fn test_part1_example() {
        let values = parse_input(EXAMPLE);
        // 1000th after 0 = 4, 2000th = -3, 3000th = 2 → sum = 3
        assert_eq!(solve_part1(&values), 3);
    }

    #[test]
    fn test_part2_example() {
        let values = parse_input(EXAMPLE);
        assert_eq!(solve_part2(&values), 1623178306);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day20.txt");
        let values = parse_input(input);
        assert_eq!(solve_part1(&values), 8028);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day20.txt");
        let values = parse_input(input);
        assert_eq!(solve_part2(&values), 8_798_438_007_673);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day20.txt");
        assert_eq!(solve(input), (8028, 8_798_438_007_673));
    }
}
