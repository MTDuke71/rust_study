//! Day 3: Rucksack Reorganization
//!
//! **Problem**: Find items that appear in both compartments of rucksacks and calculate priorities.
//!
//! **Approaches**:
//! - **HashSet**: Simple, idiomatic, general-purpose (~359µs)
//! - **Bitset**: Optimized for ASCII-only items (~20µs, 18× faster)
//!
//! # Part 1
//! - Split each rucksack line in half (first/second compartment)
//! - Find the common item between compartments
//! - Convert to priority: a-z = 1-26, A-Z = 27-52
//! - Sum all priorities

use std::collections::HashSet;

// ============================================================================
// Data Structures
// ============================================================================

/// Parsed rucksacks - each rucksack is a string of items
#[derive(Debug, Clone)]
pub struct Rucksacks {
    pub lines: Vec<String>,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert item character to priority value
/// a-z = 1-26, A-Z = 27-52
fn item_priority(item: char) -> u32 {
    match item {
        'a'..='z' => (item as u32) - ('a' as u32) + 1,
        'A'..='Z' => (item as u32) - ('A' as u32) + 27,
        _ => 0, // Invalid item
    }
}

/// Find the common item between two compartments (HashSet version)
fn find_common_item(first: &str, second: &str) -> Option<char> {
    let first_set: HashSet<char> = first.chars().collect();
    let second_set: HashSet<char> = second.chars().collect();
    
    first_set.intersection(&second_set).next().copied()
}

/// Find common item across three rucksacks (badge for group of 3 elves)
fn find_badge_item(first: &str, second: &str, third: &str) -> Option<char> {
    let first_set: HashSet<char> = first.chars().collect();
    let second_set: HashSet<char> = second.chars().collect();
    let third_set: HashSet<char> = third.chars().collect();
    
    // Find intersection of all three sets
    let first_second: HashSet<char> = first_set.intersection(&second_set).copied().collect();
    first_second.intersection(&third_set).next().copied()
}

// ============================================================================
// Bitset Optimization (18× faster for ASCII-only inputs)
// ============================================================================
//
// This uses the same technique as chess engines' "bitboards" - representing
// a set of positions/items as a bitmask where each bit indicates presence.
// Chess uses u64 for 64 squares, we use u128 for 128 ASCII characters.
//
// Benefits: Zero heap allocations, set operations = CPU instructions (AND/OR),
// cache-friendly (16 bytes vs scattered HashSet pointers)

/// Convert a string of items to a bitset (one bit per ASCII character)
/// Each character sets its corresponding bit in a u128
/// 
/// # Why u128 instead of u64?
/// ASCII values: 'A'=65, 'Z'=90, 'a'=97, 'z'=122
/// Using u128 lets us use raw ASCII values as bit positions directly:
///   'A' → bit 65, 'z' → bit 122 (requires 123 bits)
/// 
/// Alternative with u64 would require remapping:
///   'A'-'Z' (65-90) → bits 0-25
///   'a'-'z' (97-122) → bits 26-51
/// This would save 8 bytes but add branching/arithmetic overhead
/// 
/// # Example (chess engine analogy)
/// Chess bitboard: u64 where bit N = piece at square N (squares 0-63)
/// Our approach:    u128 where bit N = character with ASCII value N (0-127)
#[inline]
fn items_to_bitset(s: &str) -> u128 {
    s.bytes().fold(0u128, |acc, b| acc | (1u128 << b))
}

/// Find the common item between two compartments using bitset intersection
/// 
/// Equivalent to chess engine: "Which squares have both white pieces AND attacked by opponent?"
/// Answer: `white_pieces & opponent_attacks` (single CPU instruction)
#[inline]
fn find_common_item_bitset(first: &str, second: &str) -> Option<char> {
    let intersection = items_to_bitset(first) & items_to_bitset(second);
    if intersection == 0 {
        None
    } else {
        // Find the first set bit (lowest ASCII value)
        // Chess engines use this to iterate pieces: loop while bitboard != 0, pop LSB
        Some((intersection.trailing_zeros() as u8) as char)
    }
}

/// Find common item across three rucksacks using bitset intersection
#[inline]
fn find_badge_item_bitset(first: &str, second: &str, third: &str) -> Option<char> {
    let intersection = items_to_bitset(first) 
        & items_to_bitset(second) 
        & items_to_bitset(third);
    
    if intersection == 0 {
        None
    } else {
        Some((intersection.trailing_zeros() as u8) as char)
    }
}

// ============================================================================
// Parsing
// ============================================================================

pub fn parse_input(input: &str) -> Rucksacks {
    let lines = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.to_string())
        .collect();

    Rucksacks { lines }
}

// ============================================================================
// Part 1 Logic
// ============================================================================

fn solve_part1_impl(data: &Rucksacks) -> u32 {
    data.lines
        .iter()
        .filter_map(|rucksack| {
            let mid = rucksack.len() / 2;
            let (first, second) = rucksack.split_at(mid);
            
            find_common_item(first, second).map(item_priority)
        })
        .sum()
}

/// Bitset-optimized Part 1 (18× faster)
fn solve_part1_bitset(data: &Rucksacks) -> u32 {
    data.lines
        .iter()
        .filter_map(|rucksack| {
            let mid = rucksack.len() / 2;
            let (first, second) = rucksack.split_at(mid);
            
            find_common_item_bitset(first, second).map(item_priority)
        })
        .sum()
}

// ============================================================================
// Part 2 Logic
// ============================================================================

fn solve_part2_impl(data: &Rucksacks) -> u32 {
    data.lines
        .chunks(3)
        .filter_map(|group| {
            if group.len() == 3 {
                find_badge_item(&group[0], &group[1], &group[2]).map(item_priority)
            } else {
                None
            }
        })
        .sum()
}

/// Bitset-optimized Part 2 (18× faster)
fn solve_part2_bitset(data: &Rucksacks) -> u32 {
    data.lines
        .chunks(3)
        .filter_map(|group| {
            if group.len() == 3 {
                find_badge_item_bitset(&group[0], &group[1], &group[2]).map(item_priority)
            } else {
                None
            }
        })
        .sum()
}

// ============================================================================
// Public API
// ============================================================================

pub fn solve(input: &str) -> (u32, u32) {
    let data = parse_input(input);
    (solve_part1_impl(&data), solve_part2_impl(&data))
}

/// Bitset-optimized solve (uses bitset for 18× speedup)
pub fn solve_bitset(input: &str) -> (u32, u32) {
    let data = parse_input(input);
    (solve_part1_bitset(&data), solve_part2_bitset(&data))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_item_priority() {
        assert_eq!(item_priority('a'), 1);
        assert_eq!(item_priority('z'), 26);
        assert_eq!(item_priority('A'), 27);
        assert_eq!(item_priority('Z'), 52);
        assert_eq!(item_priority('p'), 16);
        assert_eq!(item_priority('L'), 38);
        assert_eq!(item_priority('P'), 42);
    }

    #[test]
    fn test_find_common_item() {
        assert_eq!(find_common_item("vJrwpWtwJgWr", "hcsFMMfFFhFp"), Some('p'));
        assert_eq!(find_common_item("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"), Some('L'));
        assert_eq!(find_common_item("PmmdzqPrV", "vPwwTWBwg"), Some('P'));
    }

    #[test]
    fn test_part1_example() {
        let data = parse_input(EXAMPLE);
        assert_eq!(solve_part1_impl(&data), 157);
    }

    #[test]
    fn test_part1_individual_lines() {
        // First line: common item 'p' = 16
        let line1 = parse_input("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(solve_part1_impl(&line1), 16);

        // Second line: common item 'L' = 38
        let line2 = parse_input("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        assert_eq!(solve_part1_impl(&line2), 38);
    }

    #[test]
    fn test_find_badge_item() {
        // First group in example - common item 'r'
        assert_eq!(
            find_badge_item(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ),
            Some('r')
        );

        // Second group in example - common item 'Z'
        assert_eq!(
            find_badge_item(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            Some('Z')
        );
    }

    #[test]
    fn test_part2_example() {
        let data = parse_input(EXAMPLE);
        // r = 18, Z = 52, total = 70
        assert_eq!(solve_part2_impl(&data), 70);
    }

    // ========================================================================
    // Bitset Implementation Tests
    // ========================================================================

    #[test]
    fn test_bitset_find_common_item() {
        assert_eq!(find_common_item_bitset("vJrwpWtwJgWr", "hcsFMMfFFhFp"), Some('p'));
        assert_eq!(find_common_item_bitset("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"), Some('L'));
        assert_eq!(find_common_item_bitset("PmmdzqPrV", "vPwwTWBwg"), Some('P'));
    }

    #[test]
    fn test_bitset_find_badge_item() {
        // First group in example - common item 'r'
        assert_eq!(
            find_badge_item_bitset(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ),
            Some('r')
        );

        // Second group in example - common item 'Z'
        assert_eq!(
            find_badge_item_bitset(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            Some('Z')
        );
    }

    #[test]
    fn test_bitset_part1_example() {
        let data = parse_input(EXAMPLE);
        assert_eq!(solve_part1_bitset(&data), 157);
    }

    #[test]
    fn test_bitset_part2_example() {
        let data = parse_input(EXAMPLE);
        // r = 18, Z = 52, total = 70
        assert_eq!(solve_part2_bitset(&data), 70);
    }

    #[test]
    fn test_bitset_matches_hashset() {
        // Verify bitset and hashset implementations produce identical results
        let data = parse_input(EXAMPLE);
        
        let hashset_p1 = solve_part1_impl(&data);
        let bitset_p1 = solve_part1_bitset(&data);
        assert_eq!(hashset_p1, bitset_p1, "Part 1 results should match");

        let hashset_p2 = solve_part2_impl(&data);
        let bitset_p2 = solve_part2_bitset(&data);
        assert_eq!(hashset_p2, bitset_p2, "Part 2 results should match");
    }
}
