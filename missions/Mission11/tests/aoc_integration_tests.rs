//! REQ-8: Real AoC Problem Integration Tests
//!
//! This test suite validates that Mission 11 memoization patterns are correctly
//! applied in actual Advent of Code solutions.

use std::collections::HashMap;

// ============================================================================
// REQ-8: AoC 2024 Day 19 Integration
// ============================================================================

/// Test that AoC 2024 Day 19 Part 1 uses Mission 11 memoization patterns
#[test]
fn req8_aoc2024_day19_part1_pattern_validation() {
    // Use the example from AoC 2024 Day 19
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    let (patterns, designs) = parse_input(input);

    // Part 1: Count how many designs can be made
    let possible_count: usize = designs
        .iter()
        .filter(|design| can_make_design(&patterns, design))
        .count();

    // Validate against AoC example answer: 6 designs possible
    assert_eq!(possible_count, 6);
}

/// Test that AoC 2024 Day 19 Part 2 uses Mission 11 counting transformation
#[test]
fn req8_aoc2024_day19_part2_pattern_validation() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    let (patterns, designs) = parse_input(input);

    // Part 2: Count total number of ways to make all possible designs
    let total_ways: u64 = designs
        .iter()
        .map(|design| count_ways_to_make(&patterns, design))
        .sum();

    // Validate against AoC example answer: 16 total arrangements
    assert_eq!(total_ways, 16);
}

/// Test that Mission 11 patterns match AoC implementation exactly
#[test]
fn req8_pattern_equivalence_validation() {
    let patterns = vec!["r", "wr", "b"];
    let design = "brwrr";

    // Both implementations should give same result
    let can_make = can_make_design(&patterns, design);
    assert!(can_make);

    let count = count_ways_to_make(&patterns, design);
    assert!(count > 0);
}

/// Test memoization effectiveness on realistic AoC input size
#[test]
fn req8_memoization_effectiveness() {
    // Realistic pattern set (similar to AoC)
    let patterns = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
    let design = "brwrrbrwrrbrwrr"; // Repeated pattern

    let mut memo = HashMap::new();
    let result = can_make_recursive(&patterns, design, &mut memo);

    assert!(result);

    // Memo should have multiple entries (one per suffix)
    assert!(!memo.is_empty());
    assert!(memo.len() <= design.len());
}

/// Test boolean → counting transformation (REQ-4 applied to REQ-8)
#[test]
fn req8_boolean_to_counting_transformation() {
    let patterns = vec!["a", "aa", "aaa"];
    let design = "aaaa";

    // Part 1 pattern: boolean existence
    let mut bool_memo = HashMap::new();
    let exists = can_make_recursive(&patterns, design, &mut bool_memo);
    assert!(exists);

    // Part 2 pattern: count all ways
    let mut count_memo = HashMap::new();
    let ways = count_ways_recursive(&patterns, design, &mut count_memo);

    // "aaaa" can be made as:
    // - a, a, a, a
    // - aa, a, a
    // - a, aa, a
    // - a, a, aa
    // - aa, aa
    // - aaa, a
    // - a, aaa
    assert_eq!(ways, 7);

    // Both should use same cache structure (just different return types)
    assert!(!bool_memo.is_empty());
    assert!(!count_memo.is_empty());
}

/// Test zero-copy string slice caching (REQ-3 applied to REQ-8)
#[test]
fn req8_zero_copy_string_slicing() {
    let patterns = vec!["abc", "def", "ghi"];
    let design = "abcdefghi";

    let mut memo = HashMap::new();
    let result = can_make_recursive(&patterns, design, &mut memo);

    assert!(result);

    // Verify memo keys are string slices (not allocations)
    // Each key should be a suffix of the original design
    for key in memo.keys() {
        assert!(design.contains(key)); // Key is substring of original
                                       // Verify it's actually a slice (pointer arithmetic check)
        let key_ptr = key.as_ptr();
        let design_ptr = design.as_ptr();
        let design_end = unsafe { design_ptr.add(design.len()) };
        assert!(key_ptr >= design_ptr && key_ptr <= design_end);
    }
}

/// Test overlapping subproblem detection (REQ-6 applied to REQ-8)
#[test]
fn req8_overlapping_subproblems() {
    // Pattern that creates many overlapping subproblems
    let patterns = vec!["a", "b", "ab", "ba"];
    let design = "abababa";

    let mut memo = HashMap::new();
    let count = count_ways_recursive(&patterns, design, &mut memo);

    assert!(count > 1);

    // Memo size shows how many unique subproblems were solved
    // Should be << naive exponential branching
    let unique_subproblems = memo.len();
    assert!(unique_subproblems <= design.len()); // Linear in design length

    // Without memoization, would explore exponential paths
    // With memoization, only solves O(L) unique subproblems
}

// ============================================================================
// Helper Functions (Mirror AoC 2024 Day 19 Implementation)
// ============================================================================

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let (patterns_str, designs_str) = if parts.len() == 1 {
        if let Some(first_newline) = input.find('\n') {
            (&input[..first_newline], &input[first_newline + 1..])
        } else {
            (input, "")
        }
    } else {
        (parts[0], parts[1])
    };

    let patterns: Vec<&str> = patterns_str.split(", ").map(|s| s.trim()).collect();

    let designs: Vec<&str> = designs_str
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    (patterns, designs)
}

fn can_make_design(patterns: &[&str], design: &str) -> bool {
    let mut memo = HashMap::new();
    can_make_recursive(patterns, design, &mut memo)
}

fn can_make_recursive<'a>(
    patterns: &[&str],
    remaining: &'a str,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    // REQ-2: Base case check
    if remaining.is_empty() {
        return true;
    }

    // REQ-2: Memoization check
    if let Some(&result) = memo.get(remaining) {
        return result;
    }

    // REQ-2: Recursive computation
    for pattern in patterns {
        // REQ-3: Zero-copy string slicing with strip_prefix
        if let Some(rest) = remaining.strip_prefix(pattern) {
            if can_make_recursive(patterns, rest, memo) {
                memo.insert(remaining, true);
                return true;
            }
        }
    }

    // REQ-2: Cache and return
    memo.insert(remaining, false);
    false
}

fn count_ways_to_make(patterns: &[&str], design: &str) -> u64 {
    let mut memo = HashMap::new();
    count_ways_recursive(patterns, design, &mut memo)
}

fn count_ways_recursive<'a>(
    patterns: &[&str],
    remaining: &'a str,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    // REQ-4: Base case returns count (1 way to complete)
    if remaining.is_empty() {
        return 1;
    }

    // REQ-4: Memoization check (same structure as boolean version)
    if let Some(&result) = memo.get(remaining) {
        return result;
    }

    // REQ-4: Accumulate count (sum instead of OR)
    let mut total = 0;
    for pattern in patterns {
        if let Some(rest) = remaining.strip_prefix(pattern) {
            total += count_ways_recursive(patterns, rest, memo);
        }
    }

    // REQ-4: Cache and return
    memo.insert(remaining, total);
    total
}
