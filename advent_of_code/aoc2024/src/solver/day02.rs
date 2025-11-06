use anyhow::{Context, Result};

/// Parse input into a vector of reports, where each report is a vector of levels
/// 
/// Internal function used by solve_part1 and solve_part2.
/// Example usage is demonstrated in the public solver functions.
fn parse_reports(input: &str) -> Result<Vec<Vec<i32>>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(line_num, line)| {
            line.split_whitespace()
                .enumerate()
                .map(|(col_num, level_str)| {
                    level_str.parse::<i32>()
                        .with_context(|| format!("Failed to parse level '{}' at position {} on line {}", level_str, col_num + 1, line_num + 1))
                })
                .collect::<Result<Vec<i32>>>()
        })
        .collect()
}

/// Check if a report is safe according to the rules:
/// 1. The levels are either all increasing or all decreasing
/// 2. Any two adjacent levels differ by at least one and at most three
fn is_safe_report(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut direction: Option<bool> = None; // None = unknown, Some(true) = increasing, Some(false) = decreasing
    
    for window in levels.windows(2) {
        let diff = window[1] - window[0];
        let abs_diff = diff.abs();
        
        // Check if difference is within allowed range (1-3)
        if !(1..=3).contains(&abs_diff) {
            return false;
        }
        
        // Determine or verify direction
        let is_increasing = diff > 0;
        match direction {
            None => {
                // First pair - establish direction
                direction = Some(is_increasing);
            }
            Some(expected_increasing) => {
                // Check if direction is consistent
                if is_increasing != expected_increasing {
                    return false;
                }
            }
        }
    }
    
    true
}

/// Check if a report can be made safe by removing exactly one level
fn is_safe_with_dampener(levels: &[i32]) -> bool {
    // First check if it's already safe
    if is_safe_report(levels) {
        return true;
    }
    
    // Try removing each level one at a time
    for i in 0..levels.len() {
        let mut modified_levels = levels.to_vec();
        modified_levels.remove(i);
        
        if is_safe_report(&modified_levels) {
            return true;
        }
    }
    
    false
}

/// Solve Part 1: Count how many reports are safe
/// 
/// A report is safe if:
/// - The levels are either all increasing or all decreasing
/// - Any two adjacent levels differ by at least one and at most three
/// 
/// # Arguments
/// 
/// * `input` - The puzzle input as a string
/// 
/// # Returns
/// 
/// The number of safe reports
/// 
/// # Example
/// 
/// ```
/// use aoc2024::solver::day02::solve_part1;
/// 
/// // Small example with 6 reports
/// let input = "7 6 4 2 1
/// 1 2 7 8 9
/// 9 7 6 2 1
/// 1 3 2 4 5
/// 8 6 4 4 1
/// 1 3 6 7 9";
/// 
/// assert_eq!(solve_part1(input).unwrap(), 2);
/// ```
pub fn solve_part1(input: &str) -> Result<usize> {
    let reports = parse_reports(input)
        .context("Failed to parse reports from input")?;
    
    let safe_count = reports
        .iter()
        .filter(|report| is_safe_report(report))
        .count();
    
    Ok(safe_count)
}

/// Solve Part 2: Count how many reports are safe with the Problem Dampener
/// 
/// The Problem Dampener allows the reactor safety systems to tolerate a single
/// bad level in what would otherwise be a safe report. A report is safe if
/// removing exactly one level would make it safe according to Part 1 rules.
/// 
/// # Arguments
/// 
/// * `input` - The puzzle input as a string
/// 
/// # Returns
/// 
/// The number of safe reports with the Problem Dampener
/// 
/// # Example
/// 
/// ```
/// use aoc2024::solver::day02::solve_part2;
/// 
/// // Small example with 6 reports
/// let input = "7 6 4 2 1
/// 1 2 7 8 9
/// 9 7 6 2 1
/// 1 3 2 4 5
/// 8 6 4 4 1
/// 1 3 6 7 9";
/// 
/// assert_eq!(solve_part2(input).unwrap(), 4);
/// ```
pub fn solve_part2(input: &str) -> Result<usize> {
    let reports = parse_reports(input)
        .context("Failed to parse reports from input")?;
    
    let safe_count = reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count();
    
    Ok(safe_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_parse_reports() {
        let reports = parse_reports(SMALL_EXAMPLE).unwrap();
        assert_eq!(reports.len(), 6);
        assert_eq!(reports[0], vec![7, 6, 4, 2, 1]);
        assert_eq!(reports[1], vec![1, 2, 7, 8, 9]);
    }

    #[test]
    fn test_is_safe_report() {
        assert_eq!(is_safe_report(&[7, 6, 4, 2, 1]), true);  // All decreasing by 1 or 2
        assert_eq!(is_safe_report(&[1, 2, 7, 8, 9]), false); // Increase of 5
        assert_eq!(is_safe_report(&[9, 7, 6, 2, 1]), false); // Decrease of 4
        assert_eq!(is_safe_report(&[1, 3, 2, 4, 5]), false); // Increasing then decreasing
        assert_eq!(is_safe_report(&[8, 6, 4, 4, 1]), false); // No change (4 4)
        assert_eq!(is_safe_report(&[1, 3, 6, 7, 9]), true);  // All increasing by 1, 2, or 3
    }

    #[test]
    fn test_is_safe_with_dampener() {
        assert_eq!(is_safe_with_dampener(&[7, 6, 4, 2, 1]), true);  // Safe without removing any level
        assert_eq!(is_safe_with_dampener(&[1, 2, 7, 8, 9]), false); // Unsafe regardless of removal
        assert_eq!(is_safe_with_dampener(&[9, 7, 6, 2, 1]), false); // Unsafe regardless of removal  
        assert_eq!(is_safe_with_dampener(&[1, 3, 2, 4, 5]), true);  // Safe by removing second level (3)
        assert_eq!(is_safe_with_dampener(&[8, 6, 4, 4, 1]), true);  // Safe by removing third level (4)
        assert_eq!(is_safe_with_dampener(&[1, 3, 6, 7, 9]), true);  // Safe without removing any level
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(SMALL_EXAMPLE).unwrap(), 2);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(SMALL_EXAMPLE).unwrap(), 4);
    }

    #[test]
    fn test_edge_cases() {
        // Single level - should be safe
        assert_eq!(is_safe_report(&[5]), true);
        
        // Two levels - should be safe if diff is 1-3
        assert_eq!(is_safe_report(&[1, 2]), true);
        assert_eq!(is_safe_report(&[1, 4]), true);
        assert_eq!(is_safe_report(&[1, 5]), false); // diff > 3
        assert_eq!(is_safe_report(&[1, 1]), false); // no change
        
        // Empty report - should be safe
        assert_eq!(is_safe_report(&[]), true);
    }

    #[test]
    fn test_problem_dampener_edge_cases() {
        // Report with only one bad level at the beginning
        assert_eq!(is_safe_with_dampener(&[10, 1, 2, 3]), true);
        
        // Report with only one bad level at the end
        assert_eq!(is_safe_with_dampener(&[1, 2, 3, 10]), true);
        
        // Report with only one bad level in the middle
        assert_eq!(is_safe_with_dampener(&[1, 2, 10, 3, 4]), true);
    }
}