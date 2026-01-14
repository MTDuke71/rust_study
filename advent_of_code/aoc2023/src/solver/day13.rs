use anyhow::Result;

/// Day 13: Point of Incidence - Find mirror reflections in ash/rock patterns
///
/// # Problem
/// Find lines of reflection (horizontal or vertical) in 2D patterns.
/// Each pattern must mirror perfectly across the reflection line.
///
/// # Approach
/// 1. Parse input into separate pattern grids
/// 2. For each pattern, check all potential reflection lines (both horizontal and vertical)
/// 3. Score: columns_left_of_vertical_reflection + (100 × rows_above_horizontal_reflection)
/// 4. Sum all pattern scores
///
/// # Mathematical Foundation
///
/// Uses **Hamming distance** (discrete metric) to count mismatches between reflected pairs.
/// See `zettelkasten/math-foundations/hamming-distance-discrete-metrics.md` for theory.
///
/// - Part 1: Perfect reflection requires Hamming distance = 0
/// - Part 2: Smudge reflection requires Hamming distance = 1 (exactly one character differs)
///
/// **Key insight**: Generalizing from boolean "exact match" to integer "mismatch count"
/// unifies both parts with a single algorithm.
///
/// # Mission Reuse
/// Uses Mission 6 `Grid<T>` for 2D pattern storage and access
pub fn solve_part1(input: &str) -> Result<usize> {
    let patterns = parse_patterns(input)?;
    let total: usize = patterns
        .iter()
        .map(|pattern| find_reflection_score(pattern, 0))
        .sum();
    Ok(total)
}

pub fn solve_part2(input: &str) -> Result<usize> {
    let patterns = parse_patterns(input)?;
    let total: usize = patterns
        .iter()
        .map(|pattern| find_reflection_score(pattern, 1))
        .sum();
    Ok(total)
}

/// Parse input into separate pattern grids
fn parse_patterns(input: &str) -> Result<Vec<Vec<Vec<char>>>> {
    let mut patterns = Vec::new();
    let mut current_pattern = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            if !current_pattern.is_empty() {
                patterns.push(current_pattern);
                current_pattern = Vec::new();
            }
        } else {
            current_pattern.push(line.chars().collect());
        }
    }

    // Don't forget last pattern if input doesn't end with blank line
    if !current_pattern.is_empty() {
        patterns.push(current_pattern);
    }

    Ok(patterns)
}

/// Find the reflection score for a single pattern with specified target Hamming distance
/// Returns columns_left + (100 × rows_above)
/// 
/// # Parameters
/// - `target_distance`: 0 for perfect reflection, 1 for smudge reflection
///
/// # Implementation Note
/// **Refactored for simplicity**: Originally implemented as three separate functions
/// (`find_reflection_score`, `find_smudge_reflection_score`, and `find_reflection_score_with_target`).
/// Consolidated into this single parameterized function following the YAGNI principle.
/// External documentation (e.g., `day13_function_guide.md`) may reference the old structure.
fn find_reflection_score(pattern: &[Vec<char>], target_distance: usize) -> usize {
    // Try horizontal reflection first (between rows)
    if let Some(row) = find_horizontal_reflection(pattern, target_distance) {
        return row * 100;
    }

    // Try vertical reflection (between columns)
    if let Some(col) = find_vertical_reflection(pattern, target_distance) {
        return col;
    }

    // Should always find a reflection per problem statement
    panic!("No reflection found in pattern with Hamming distance {}", target_distance);
}

/// Find horizontal reflection line (returns rows ABOVE the reflection line)
/// target_distance: 0 for perfect reflection, 1 for smudge reflection
fn find_horizontal_reflection(pattern: &[Vec<char>], target_distance: usize) -> Option<usize> {
    let rows = pattern.len();

    // Try each potential reflection line between row i and row i+1
    for i in 0..rows - 1 {
        if count_horizontal_mismatches(pattern, i) == target_distance {
            return Some(i + 1); // Return rows ABOVE (1-indexed)
        }
    }

    None
}

/// Count mismatches for horizontal reflection between row `above_idx` and row `above_idx + 1`
fn count_horizontal_mismatches(pattern: &[Vec<char>], above_idx: usize) -> usize {
    let rows = pattern.len();
    let mut mismatches = 0;
    let mut distance = 0;

    loop {
        let upper = above_idx.checked_sub(distance);
        let lower = above_idx + 1 + distance;

        // If either side goes out of bounds, we've validated all possible pairs
        if upper.is_none() || lower >= rows {
            return mismatches;
        }

        // Count mismatches between the two rows
        let upper_row = upper.unwrap();
        for (a, b) in pattern[upper_row].iter().zip(pattern[lower].iter()) {
            if a != b {
                mismatches += 1;
            }
        }

        distance += 1;
    }
}

/// Find vertical reflection line (returns columns to the LEFT of the reflection line)
/// target_distance: 0 for perfect reflection, 1 for smudge reflection
fn find_vertical_reflection(pattern: &[Vec<char>], target_distance: usize) -> Option<usize> {
    if pattern.is_empty() {
        return None;
    }

    let cols = pattern[0].len();

    // Try each potential reflection line between column i and column i+1
    for i in 0..cols - 1 {
        if count_vertical_mismatches(pattern, i) == target_distance {
            return Some(i + 1); // Return columns to LEFT (1-indexed)
        }
    }

    None
}

/// Count mismatches for vertical reflection between column `left_idx` and column `left_idx + 1`
fn count_vertical_mismatches(pattern: &[Vec<char>], left_idx: usize) -> usize {
    let cols = pattern[0].len();
    let mut mismatches = 0;
    let mut distance = 0;

    loop {
        let left = left_idx.checked_sub(distance);
        let right = left_idx + 1 + distance;

        // If either side goes out of bounds, we've validated all possible pairs
        if left.is_none() || right >= cols {
            return mismatches;
        }

        // Count mismatches between the two columns across all rows
        let left_col = left.unwrap();
        for row in pattern {
            if row[left_col] != row[right] {
                mismatches += 1;
            }
        }

        distance += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_parse_patterns() {
        let patterns = parse_patterns(EXAMPLE).unwrap();
        assert_eq!(patterns.len(), 2);
        assert_eq!(patterns[0].len(), 7); // 7 rows
        assert_eq!(patterns[0][0].len(), 9); // 9 columns
        assert_eq!(patterns[1].len(), 7); // 7 rows
    }

    #[test]
    fn test_vertical_reflection() {
        let patterns = parse_patterns(EXAMPLE).unwrap();
        let score = find_reflection_score(&patterns[0], 0);
        assert_eq!(score, 5); // Vertical reflection, 5 columns to left
    }

    #[test]
    fn test_horizontal_reflection() {
        let patterns = parse_patterns(EXAMPLE).unwrap();
        let score = find_reflection_score(&patterns[1], 0);
        assert_eq!(score, 400); // Horizontal reflection, 4 rows above × 100
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, 405); // 5 + 400
    }

    #[test]
    fn test_smudge_first_pattern() {
        let patterns = parse_patterns(EXAMPLE).unwrap();
        let score = find_reflection_score(&patterns[0], 1);
        assert_eq!(score, 300); // Horizontal reflection, 3 rows above × 100
    }

    #[test]
    fn test_smudge_second_pattern() {
        let patterns = parse_patterns(EXAMPLE).unwrap();
        let score = find_reflection_score(&patterns[1], 1);
        assert_eq!(score, 100); // Horizontal reflection, 1 row above × 100
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result, 400); // 300 + 100
    }
}
