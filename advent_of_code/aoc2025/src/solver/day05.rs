//! Day 5: Cafeteria - Fresh Ingredient Detection
//!
//! # Part 1
//! Count how many ingredient IDs are "fresh" (fall within any given range).
//!
//! # Part 2
//! Count how many UNIQUE IDs are considered fresh across all ranges.
//! Ranges can overlap, so we use a HashSet to deduplicate.
//!
//! # Algorithm
//! 1. Parse input into two sections: ranges and ingredient IDs
//! 2. Part 1: For each ingredient, check if it falls within ANY range
//! 3. Part 2: Expand all ranges into a HashSet, count unique IDs
//!
//! # Parsing
//! - Section 1: Lines like "3-5" → inclusive range (3, 5)
//! - Section 2: Lines like "11" → ingredient ID

use anyhow::Result;

/// Represents an inclusive range of fresh ingredient IDs
#[derive(Debug, Clone, Copy)]
struct FreshRange {
    start: u64,
    end: u64,
}

impl FreshRange {
    /// Check if an ingredient ID falls within this range (inclusive)
    fn contains(&self, id: u64) -> bool {
        id >= self.start && id <= self.end
    }
}

/// Parse the input into ranges and ingredient IDs
fn parse_input(input: &str) -> (Vec<FreshRange>, Vec<u64>) {
    let mut sections = input.trim().split("\n\n");
    
    // Parse ranges section
    let ranges: Vec<FreshRange> = sections
        .next()
        .unwrap_or("")
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('-');
            let start = parts.next()?.parse().ok()?;
            let end = parts.next()?.parse().ok()?;
            Some(FreshRange { start, end })
        })
        .collect();
    
    // Parse ingredients section
    let ingredients: Vec<u64> = sections
        .next()
        .unwrap_or("")
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();
    
    (ranges, ingredients)
}

/// Check if an ingredient is fresh (falls within any range)
fn is_fresh(id: u64, ranges: &[FreshRange]) -> bool {
    ranges.iter().any(|range| range.contains(id))
}

/// Count how many ingredients are fresh
fn count_fresh(ranges: &[FreshRange], ingredients: &[u64]) -> usize {
    ingredients.iter().filter(|&&id| is_fresh(id, ranges)).count()
}

pub fn solve_part1(input: &str) -> Result<String> {
    let (ranges, ingredients) = parse_input(input);
    let fresh_count = count_fresh(&ranges, &ingredients);
    Ok(fresh_count.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let (ranges, _ingredients) = parse_input(input);
    
    // Convert to (start, end) tuples and sort by start
    let mut intervals: Vec<(u64, u64)> = ranges.iter().map(|r| (r.start, r.end)).collect();
    intervals.sort_by_key(|&(start, _)| start);
    
    // Merge overlapping ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in intervals {
        if let Some(last) = merged.last_mut() {
            // Check if current range overlaps or is adjacent to last merged range
            if start <= last.1 + 1 {
                // Extend the last range if current end is larger
                last.1 = last.1.max(end);
            } else {
                // No overlap - add new range
                merged.push((start, end));
            }
        } else {
            // First range
            merged.push((start, end));
        }
    }
    
    // Sum the lengths of all merged ranges (inclusive, so end - start + 1)
    let total: u64 = merged.iter().map(|(start, end)| end - start + 1).sum();
    
    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_parse_input() {
        let (ranges, ingredients) = parse_input(EXAMPLE);
        
        assert_eq!(ranges.len(), 4);
        assert_eq!(ranges[0].start, 3);
        assert_eq!(ranges[0].end, 5);
        assert_eq!(ranges[3].start, 12);
        assert_eq!(ranges[3].end, 18);
        
        assert_eq!(ingredients, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_range_contains() {
        let range = FreshRange { start: 3, end: 5 };
        assert!(!range.contains(2));
        assert!(range.contains(3));
        assert!(range.contains(4));
        assert!(range.contains(5));
        assert!(!range.contains(6));
    }

    #[test]
    fn test_is_fresh() {
        let (ranges, _) = parse_input(EXAMPLE);
        
        assert!(!is_fresh(1, &ranges));  // spoiled - no range
        assert!(is_fresh(5, &ranges));   // fresh - in 3-5
        assert!(!is_fresh(8, &ranges));  // spoiled
        assert!(is_fresh(11, &ranges));  // fresh - in 10-14
        assert!(is_fresh(17, &ranges));  // fresh - in 16-20 AND 12-18
        assert!(!is_fresh(32, &ranges)); // spoiled
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE).unwrap(), "3");
    }

    #[test]
    fn test_part2_example() {
        // Fresh IDs: 3,4,5, 10,11,12,13,14, 15,16,17,18,19,20 = 14 unique
        assert_eq!(solve_part2(EXAMPLE).unwrap(), "14");
    }
}
