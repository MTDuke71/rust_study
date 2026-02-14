//! Day 13: Distress Signal
//!
//! Compare pairs of nested list packets to determine which are in the right order.
//! Uses recursive comparison with type coercion rules.

use serde_json::Value;
use std::cmp::Ordering;

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone)]
pub struct PacketPair {
    left: Value,
    right: Value,
}

#[derive(Debug, Clone)]
pub struct ParsedData {
    pairs: Vec<PacketPair>,
}

// ============================================================================
// Comparison Logic
// ============================================================================

/// Compare two packet values recursively according to the rules:
/// - Both integers: compare numerically
/// - Both lists: compare element by element, shorter wins on tie
/// - Mixed: convert integer to [integer] and retry
fn compare_packets(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        // Both are numbers - compare directly
        (Value::Number(l), Value::Number(r)) => {
            let l_num = l.as_i64().unwrap();
            let r_num = r.as_i64().unwrap();
            l_num.cmp(&r_num)
        }

        // Both are arrays - compare element by element
        (Value::Array(l_arr), Value::Array(r_arr)) => {
            for i in 0..l_arr.len().min(r_arr.len()) {
                match compare_packets(&l_arr[i], &r_arr[i]) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
            // If all elements equal, shorter array comes first
            l_arr.len().cmp(&r_arr.len())
        }

        // Left is number, right is array - convert left to [left]
        (Value::Number(_), Value::Array(_)) => {
            let l_arr = Value::Array(vec![left.clone()]);
            compare_packets(&l_arr, right)
        }

        // Left is array, right is number - convert right to [right]
        (Value::Array(_), Value::Number(_)) => {
            let r_arr = Value::Array(vec![right.clone()]);
            compare_packets(left, &r_arr)
        }

        // Should not happen with valid input
        _ => Ordering::Equal,
    }
}

// ============================================================================
// Parsing
// ============================================================================

pub fn parse_input(input: &str) -> ParsedData {
    let pairs = input
        .split("\n\n")
        .filter(|section| !section.trim().is_empty())
        .map(|section| {
            let mut lines = section.lines();
            let left_str = lines.next().unwrap().trim();
            let right_str = lines.next().unwrap().trim();

            let left: Value = serde_json::from_str(left_str)
                .unwrap_or_else(|_| panic!("Failed to parse left: {}", left_str));
            let right: Value = serde_json::from_str(right_str)
                .unwrap_or_else(|_| panic!("Failed to parse right: {}", right_str));

            PacketPair { left, right }
        })
        .collect();

    ParsedData { pairs }
}

// ============================================================================
// Part 1 Logic (Internal - accepts parsed data)
// ============================================================================

pub fn solve_part1_with_data(data: &ParsedData) -> usize {
    data.pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, pair)| {
            if compare_packets(&pair.left, &pair.right) == Ordering::Less {
                Some(idx + 1) // 1-based indexing
            } else {
                None
            }
        })
        .sum()
}

// ============================================================================
// Part 2 Logic (Internal - accepts parsed data)
// ============================================================================

pub fn solve_part2_with_data(data: &ParsedData) -> usize {
    // Divider packets
    let divider1: Value = serde_json::from_str("[[2]]").unwrap();
    let divider2: Value = serde_json::from_str("[[6]]").unwrap();
    
    // Start with divider positions (1-based):
    // [[2]] starts at position 1
    // [[6]] starts at position 2 (right after [[2]])
    let mut divider1_idx = 1;
    let mut divider2_idx = 2;
    
    // For each packet, count if it's less than dividers
    // This shifts the divider positions without actually sorting
    for pair in &data.pairs {
        for packet in [&pair.left, &pair.right] {
            if compare_packets(packet, &divider1) == Ordering::Less {
                // Packet comes before [[2]], so both dividers shift right
                divider1_idx += 1;
                divider2_idx += 1;
            } else if compare_packets(packet, &divider2) == Ordering::Less {
                // Packet is between [[2]] and [[6]], so only [[6]] shifts right
                divider2_idx += 1;
            }
            // else: packet >= [[6]], no shifts needed
        }
    }
    
    divider1_idx * divider2_idx
}

// ============================================================================
// Public API
// ============================================================================

/// Solve both parts efficiently (PARSE ONCE!)
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}

/// Solve Part 1 only (for testing)
pub fn solve_part1(input: &str) -> usize {
    let data = parse_input(input);
    solve_part1_with_data(&data)
}

/// Solve Part 2 only (for testing)
pub fn solve_part2(input: &str) -> usize {
    let data = parse_input(input);
    solve_part2_with_data(&data)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_parse() {
        let data = parse_input(EXAMPLE);
        assert_eq!(data.pairs.len(), 8);
        
        // Check first pair
        let first_left = data.pairs[0].left.as_array().unwrap();
        assert_eq!(first_left.len(), 5);
        assert_eq!(first_left[0].as_i64().unwrap(), 1);
    }

    #[test]
    fn test_compare_simple() {
        let left: Value = serde_json::from_str("[1,1,3,1,1]").unwrap();
        let right: Value = serde_json::from_str("[1,1,5,1,1]").unwrap();
        assert_eq!(compare_packets(&left, &right), Ordering::Less);
    }

    #[test]
    fn test_compare_mixed() {
        let left: Value = serde_json::from_str("[[1],[2,3,4]]").unwrap();
        let right: Value = serde_json::from_str("[[1],4]").unwrap();
        assert_eq!(compare_packets(&left, &right), Ordering::Less);
    }

    #[test]
    fn test_compare_wrong_order() {
        let left: Value = serde_json::from_str("[9]").unwrap();
        let right: Value = serde_json::from_str("[[8,7,6]]").unwrap();
        assert_eq!(compare_packets(&left, &right), Ordering::Greater);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 13);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day13.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, 5852);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 140);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day13.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, 24190);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day13.txt");
        assert_eq!(solve(input), (5852, 24190));
    }
}
