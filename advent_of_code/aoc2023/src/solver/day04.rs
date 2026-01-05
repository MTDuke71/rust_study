//! # Day 4: Scratchcards
//!
//! ## Part 1
//! Calculate total points from scratchcards.
//! - First matching number = 1 point
//! - Each subsequent match doubles the points
//! - Formula: points = 2^(matches - 1) for matches > 0
//!
//! ## Part 2
//! Calculate total scratchcards after processing copies.
//! - Each card with N matches wins copies of the next N cards
//! - Copies also win more copies (cascading effect)
//! - Count total cards after all copies are processed
//!
//! ## Algorithm (Part 2)
//! - Use a Vec to track card counts (starts at 1 for each original)
//! - For each card, add its count to the next N cards
//! - This is a forward-propagation DP problem
//!
//! ## Algorithm (Part 1)
//! - Parse each card to extract winning numbers and our numbers
//! - Use HashSet for O(1) lookup of winning numbers
//! - Count matches and calculate points using bit shifting (2^n = 1 << n)
//!
//! ## Mathematical Foundation
//!
//! Uses **set theory** for efficient membership testing:
//! - Build set W (winning numbers): O(m) time
//! - Test membership for each number: O(1) per test (hash-based)
//! - Total complexity: O(m + n) vs O(m × n) for nested loops
//!
//! See `zettelkasten/math-foundations/set-theory-fundamentals.md` for:
//! - Set operations (union, intersection, membership)
//! - Mathematical properties and theorems
//! - Performance analysis: HashSet 3× faster than linear search
//!
//! ## Optimizations
//! - Mission integration: HashSet from Mission 5 concepts
//! - Rust-specific: Iterator combinators, bit shifting for powers of 2

use anyhow::Result;
use std::collections::HashSet;

/// Parse a scratchcard line and return the count of matching numbers
fn count_matches(line: &str) -> usize {
    // Format: "Card X: 1 2 3 | 4 5 6"
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 2 {
        return 0;
    }

    // Extract winning numbers (after the colon)
    let winning_part = parts[0].split(':').nth(1).unwrap_or("");
    let winning: HashSet<u32> = winning_part
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Extract our numbers
    let our_numbers: Vec<u32> = parts[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Count matches
    our_numbers.iter().filter(|n| winning.contains(n)).count()
}

/// Calculate points for a given number of matches
/// First match = 1 point, then doubles for each additional match
fn calculate_points(matches: usize) -> u32 {
    if matches == 0 {
        0
    } else {
        // 2^(matches - 1) = 1 << (matches - 1)
        1 << (matches - 1)
    }
}

pub fn solve_part1(input: &str) -> Result<String> {
    let total: u32 = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let matches = count_matches(line);
            calculate_points(matches)
        })
        .sum();

    Ok(total.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    
    let num_cards = lines.len();
    
    // Track how many of each card we have (start with 1 original of each)
    let mut card_counts = vec![1u32; num_cards];
    
    // Process each card
    for (i, line) in lines.iter().enumerate() {
        let matches = count_matches(line);
        let current_count = card_counts[i];
        
        // This card wins copies of the next 'matches' cards
        // Add current_count copies to each of those cards
        for j in 1..=matches {
            if i + j < num_cards {
                card_counts[i + j] += current_count;
            }
        }
    }
    
    // Sum up total cards
    let total: u32 = card_counts.iter().sum();
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_count_matches() {
        assert_eq!(count_matches("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 4);
        assert_eq!(count_matches("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), 2);
        assert_eq!(count_matches("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), 0);
    }

    #[test]
    fn test_calculate_points() {
        assert_eq!(calculate_points(0), 0);
        assert_eq!(calculate_points(1), 1);
        assert_eq!(calculate_points(2), 2);
        assert_eq!(calculate_points(3), 4);
        assert_eq!(calculate_points(4), 8);
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "13"); // 8 + 2 + 2 + 1 + 0 + 0 = 13
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE).unwrap();
        // Card 1: 1 instance
        // Card 2: 2 instances (1 original + 1 from card 1)
        // Card 3: 4 instances (1 original + 1 from card 1 + 2 from card 2)
        // Card 4: 8 instances (1 original + 1 from card 1 + 2 from card 2 + 4 from card 3)
        // Card 5: 14 instances (1 original + 1 from card 1 + 4 from card 3 + 8 from card 4)
        // Card 6: 1 instance (original only)
        // Total: 1 + 2 + 4 + 8 + 14 + 1 = 30
        assert_eq!(result, "30");
    }
}
