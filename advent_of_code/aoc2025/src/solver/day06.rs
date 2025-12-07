//! # Day 6: Cephalopod Math Worksheet (Simplified)
//!
//! A parsing puzzle where math problems are arranged in vertical columns.
//!
//! ## Key Insight
//! Using `split_whitespace()` on each row, tokens at the same position
//! belong to the same problem. This is much simpler than column-index parsing.
//!
//! ## Part 1 vs Part 2
//! - **Part 1**: Numbers read row-wise (parse each token as-is)
//! - **Part 2**: Read columns right-to-left, digits top-to-bottom within columns

use anyhow::Result;

/// A parsed math problem: numbers and an operator.
#[derive(Debug, Clone)]
struct Problem {
    numbers: Vec<i64>,
    operator: char,
}

impl Problem {
    /// Compute the result by applying the operator to all numbers.
    fn compute(&self) -> i64 {
        match self.operator {
            '+' => self.numbers.iter().sum(),
            '*' => self.numbers.iter().product(),
            _ => 0,
        }
    }
}

/// Solve Part 1: Standard row-wise number reading.
///
/// Each row's whitespace-separated tokens map to problems by position index.
/// Problem i gets the i-th token from each row.
pub fn solve_part1(input: &str) -> Result<String> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        anyhow::bail!("Empty input");
    }

    // Split each row by whitespace
    let tokens: Vec<Vec<&str>> = lines.iter().map(|l| l.split_whitespace().collect()).collect();

    // Last row contains operators, all others contain numbers
    let op_row = tokens.last().unwrap();
    let num_rows = &tokens[..tokens.len() - 1];

    let mut total: i64 = 0;

    for (i, op_token) in op_row.iter().enumerate() {
        // Collect numbers at position i from each number row
        let numbers: Vec<i64> = num_rows
            .iter()
            .filter_map(|row| row.get(i).and_then(|s| s.parse().ok()))
            .collect();

        // Get operator from this token
        let operator = op_token.chars().next().unwrap_or('+');

        let problem = Problem { numbers, operator };
        total += problem.compute();
    }

    Ok(total.to_string())
}

/// Solve Part 2: Column-wise reading (right-to-left, digits top-to-bottom).
///
/// For Part 2, we still need column-based parsing because each vertical column
/// of characters (not tokens) forms a single number's digits.
pub fn solve_part2(input: &str) -> Result<String> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        anyhow::bail!("Empty input");
    }

    // For Part 2, we need column-based parsing.
    // Find separator columns (all spaces) to identify problem boundaries.
    let separators = find_separator_columns(&lines);
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let ranges = get_problem_ranges(&separators, width);

    let mut total: i64 = 0;

    for (start, end) in ranges {
        if let Some(problem) = parse_problem_part2(&lines, start, end) {
            total += problem.compute();
        }
    }

    Ok(total.to_string())
}

// ============================================================================
// Part 2 Helper Functions (column-based parsing still needed)
// ============================================================================

/// Find columns that are all spaces (separators between problems).
fn find_separator_columns(lines: &[&str]) -> Vec<usize> {
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    (0..width)
        .filter(|&col| {
            lines
                .iter()
                .all(|line| line.chars().nth(col).unwrap_or(' ') == ' ')
        })
        .collect()
}

/// Convert separator columns into (start, end) ranges for each problem.
fn get_problem_ranges(separators: &[usize], width: usize) -> Vec<(usize, usize)> {
    if separators.is_empty() {
        return vec![(0, width)];
    }

    let mut ranges = Vec::new();
    let mut pos = 0;
    let mut i = 0;

    while i < separators.len() {
        let sep_start = separators[i];

        // Problem before this separator group
        if sep_start > pos {
            ranges.push((pos, sep_start));
        }

        // Skip consecutive separators
        while i < separators.len() - 1 && separators[i + 1] == separators[i] + 1 {
            i += 1;
        }
        pos = separators[i] + 1;
        i += 1;
    }

    // Problem after last separator
    if pos < width {
        ranges.push((pos, width));
    }

    ranges
}

/// Parse a problem for Part 2: columns right-to-left, digits top-to-bottom.
fn parse_problem_part2(lines: &[&str], start: usize, end: usize) -> Option<Problem> {
    if lines.is_empty() || start >= end {
        return None;
    }

    let number_rows = lines.len() - 1;
    let mut numbers = Vec::new();

    // Read columns right-to-left
    for col in (start..end).rev() {
        let digit_str: String = (0..number_rows)
            .filter_map(|row| {
                lines
                    .get(row)
                    .and_then(|l| l.chars().nth(col))
                    .filter(|c| c.is_ascii_digit())
            })
            .collect();

        if !digit_str.is_empty() {
            if let Ok(num) = digit_str.parse::<i64>() {
                numbers.push(num);
            }
        }
    }

    // Operator is always at the leftmost column of each problem range
    let operator = lines.last()?.chars().nth(start)?;

    if numbers.is_empty() {
        return None;
    }

    Some(Problem { numbers, operator })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1_example() {
        // 123 * 45 * 6 = 33210
        // 328 + 64 + 98 = 490
        // 51 * 387 * 215 = 4243455
        // 64 + 23 + 314 = 401
        // Total: 4277556
        assert_eq!(solve_part1(EXAMPLE).unwrap(), "4277556");
    }

    #[test]
    fn test_part2_example() {
        // From problem statement: 3263827
        assert_eq!(solve_part2(EXAMPLE).unwrap(), "3263827");
    }

    #[test]
    fn test_simple_whitespace_parsing() {
        // Verify the whitespace approach works
        let input = "1 2\n3 4\n* +";
        // Problem 1: 1 * 3 = 3
        // Problem 2: 2 + 4 = 6
        // Total: 9
        assert_eq!(solve_part1(input).unwrap(), "9");
    }

    #[test]
    fn test_with_real_input() {
        let input = include_str!("../../inputs/day06.txt");
        // Part 1 answer from original solution
        assert_eq!(solve_part1(input).unwrap(), "5873191732773");
        // Part 2 answer from original solution
        assert_eq!(solve_part2(input).unwrap(), "11386445308378");
    }
}
