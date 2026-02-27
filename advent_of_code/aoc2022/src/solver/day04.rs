//! Day 4: Camp Cleanup
//!
//! **Problem**: Elves assigned section ranges. Find pairs where ranges fully contain or overlap.
//!
//! **Approach**:
//! - Part 1: Check if one range fully contains the other
//! - Part 2: Check if ranges have any overlap

// ============================================================================
// Data Structures
// ============================================================================

/// A range of section IDs [start, end] (inclusive)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

impl Range {
    /// Create a new range
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    /// Check if this range fully contains another range
    pub fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    /// Check if this range overlaps with another range at all
    pub fn overlaps(&self, other: &Range) -> bool {
        // Ranges overlap if start of one is <= end of the other AND vice versa
        self.start <= other.end && other.start <= self.end
    }
}

/// A pair of section assignment ranges
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RangePair {
    pub first: Range,
    pub second: Range,
}

impl RangePair {
    /// Check if one range fully contains the other
    pub fn has_full_containment(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }

    /// Check if the ranges have any overlap
    pub fn has_overlap(&self) -> bool {
        self.first.overlaps(&self.second)
    }
}

/// Parsed assignment pairs
#[derive(Debug, Clone)]
pub struct AssignmentPairs {
    pub pairs: Vec<RangePair>,
}

// ============================================================================
// Parsing
// ============================================================================

/// Parse a single range like "2-4" into Range { start: 2, end: 4 }
fn parse_range(s: &str) -> Option<Range> {
    let (start, end) = s.split_once('-')?;
    Some(Range::new(start.parse().ok()?, end.parse().ok()?))
}

/// Parse a line like "2-4,6-8" into a RangePair
fn parse_line(line: &str) -> Option<RangePair> {
    let (first, second) = line.split_once(',')?;
    Some(RangePair {
        first: parse_range(first)?,
        second: parse_range(second)?,
    })
}

pub fn parse_input(input: &str) -> AssignmentPairs {
    let pairs = input
        .lines()
        .filter_map(|line| parse_line(line.trim()))
        .collect();

    AssignmentPairs { pairs }
}

// ============================================================================
// Part 1 Logic
// ============================================================================

fn solve_part1_impl(data: &AssignmentPairs) -> usize {
    // Count pairs where one range fully contains the other
    data.pairs
        .iter()
        .filter(|pair| pair.has_full_containment())
        .count()
}

// ============================================================================
// Part 2 Logic
// ============================================================================

fn solve_part2_impl(data: &AssignmentPairs) -> usize {
    // Count pairs where ranges have any overlap
    data.pairs.iter().filter(|pair| pair.has_overlap()).count()
}

// ============================================================================
// Public API
// ============================================================================

/// Solve both parts efficiently (parse once)
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    (solve_part1_impl(&data), solve_part2_impl(&data))
}

/// Solve Part 1 only (for testing)
pub fn solve_part1(input: &str) -> usize {
    solve_part1_impl(&parse_input(input))
}

/// Solve Part 2 only (for testing)
pub fn solve_part2(input: &str) -> usize {
    solve_part2_impl(&parse_input(input))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_range_contains() {
        let r1 = Range::new(2, 8);
        let r2 = Range::new(3, 7);
        assert!(r1.contains(&r2));
        assert!(!r2.contains(&r1));

        let r3 = Range::new(6, 6);
        let r4 = Range::new(4, 6);
        assert!(r4.contains(&r3));
        assert!(!r3.contains(&r4));
    }

    #[test]
    fn test_range_overlaps() {
        // No overlap
        assert!(!Range::new(2, 4).overlaps(&Range::new(6, 8)));
        assert!(!Range::new(2, 3).overlaps(&Range::new(4, 5)));

        // Overlap
        assert!(Range::new(5, 7).overlaps(&Range::new(7, 9)));
        assert!(Range::new(2, 8).overlaps(&Range::new(3, 7)));
        assert!(Range::new(6, 6).overlaps(&Range::new(4, 6)));
        assert!(Range::new(2, 6).overlaps(&Range::new(4, 8)));
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("2-4"), Some(Range::new(2, 4)));
        assert_eq!(parse_range("10-99"), Some(Range::new(10, 99)));
    }

    #[test]
    fn test_parse_line() {
        let pair = parse_line("2-4,6-8").unwrap();
        assert_eq!(pair.first, Range::new(2, 4));
        assert_eq!(pair.second, Range::new(6, 8));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 2);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 4);
    }
}
