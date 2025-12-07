//! Day 6: Trash Compactor - Cephalopod Math Worksheet
//!
//! # Problem
//! Parse a vertical math worksheet where:
//! - Numbers are arranged vertically in columns
//! - Operators (+, *) are at the bottom of each column
//! - Columns are separated by full columns of spaces
//!
//! # Example
//! ```text
//! 123 328
//!  45 64
//!   6 98
//! *   +
//! ```
//! Column 1: 123 * 45 * 6 = 33210
//! Column 2: 328 + 64 + 98 = 490
//!
//! # Algorithm
//! 1. Find separator columns (all spaces)
//! 2. Group columns into problems
//! 3. Parse numbers and operator from each problem
//! 4. Compute and sum all results

use anyhow::Result;

// ============================================================================
// STEP 1: Find Separator Columns
// ============================================================================

/// Find column indices where ALL rows contain a space character.
/// These columns separate individual math problems.
///
/// # How it works
/// We scan each column index (0 to width-1) and check if EVERY line
/// has a space character at that position. If so, it's a separator.
///
/// # Arguments
/// * `lines` - All 5 lines of the worksheet (4 number rows + 1 operator row)
///
/// # Returns
/// A Vec of column indices that are separators
///
/// # Example
/// ```text
/// "123 45"   <- col 3 is space
/// " 67 89"   <- col 3 is space  
/// "*   + "   <- col 3 is space
/// ```
/// Column 3 is a separator (space in all rows) → returns vec![3]
fn find_separator_columns(lines: &[&str]) -> Vec<usize> {
    // Find the maximum width across all lines
    // (lines might have trailing spaces trimmed differently)
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    
    let mut separators = Vec::new();
    
    // Check each column index from 0 to width-1
    for col in 0..width {
        // A column is a separator if EVERY line has a space at this position
        let is_separator = lines.iter().all(|line| {
            // Get the character at this column
            // If the line is shorter than col, treat it as a space
            line.chars().nth(col).unwrap_or(' ') == ' '
        });
        
        if is_separator {
            separators.push(col);
        }
    }
    
    separators
}

// ============================================================================
// STEP 2: Split into Problem Ranges
// ============================================================================

/// Convert separator columns into ranges for each problem.
/// 
/// # How it works
/// Given separators at columns [3, 7, 11] and width 15:
/// - Problem 1: columns 0..3   (before first separator)
/// - Problem 2: columns 4..7   (between separators, skip the separator itself)
/// - Problem 3: columns 8..11  
/// - Problem 4: columns 12..15 (after last separator to end)
///
/// We need to handle consecutive separators (like cols 7,8 both being spaces)
/// by treating them as one gap.
///
/// # Arguments
/// * `separators` - Column indices that are separators
/// * `width` - Total width of the worksheet
///
/// # Returns
/// Vec of (start, end) ranges for each problem (exclusive end)
fn get_problem_ranges(separators: &[usize], width: usize) -> Vec<(usize, usize)> {
    if separators.is_empty() {
        // No separators = one big problem spanning entire width
        return vec![(0, width)];
    }
    
    let mut ranges = Vec::new();
    
    // First problem: from 0 to first separator
    if separators[0] > 0 {
        ranges.push((0, separators[0]));
    }
    
    // Middle problems: between consecutive non-adjacent separators
    // We need to find "gaps" between separator groups
    let mut i = 0;
    while i < separators.len() {
        // Skip consecutive separators (they're part of same gap)
        let _gap_start = separators[i];
        while i < separators.len() - 1 && separators[i + 1] == separators[i] + 1 {
            i += 1;
        }
        let gap_end = separators[i];
        
        // The next problem starts after this separator group
        let problem_start = gap_end + 1;
        
        // Find where the next problem ends (next separator or end of width)
        let problem_end = if i + 1 < separators.len() {
            separators[i + 1]
        } else {
            width
        };
        
        // Only add if there's actual content (problem_start < problem_end)
        if problem_start < problem_end {
            ranges.push((problem_start, problem_end));
        }
        
        i += 1;
    }
    
    ranges
}

// ============================================================================
// STEP 3: Parse a Single Problem (Numbers + Operator)
// ============================================================================

/// Represents a parsed math problem from one column group
#[derive(Debug)]
struct Problem {
    numbers: Vec<i64>,
    operator: char, // '+' or '*'
}

/// Extract a problem from a specific column range.
///
/// # How it works
/// Given a range (start, end), we:
/// 1. Extract the substring from each line
/// 2. The LAST line contains the operator
/// 3. All lines BEFORE the last contain numbers (trimming whitespace)
///
/// # Arguments
/// * `lines` - All lines of the worksheet (N-1 number rows + 1 operator row)
/// * `start` - Start column (inclusive)
/// * `end` - End column (exclusive)
///
/// # Returns
/// A Problem with the numbers and operator, or None if invalid
fn parse_problem(lines: &[&str], start: usize, end: usize) -> Option<Problem> {
    if lines.is_empty() {
        return None;
    }
    
    let mut numbers = Vec::new();
    
    // All lines except the last contain numbers
    let number_lines = &lines[..lines.len() - 1];
    for line in number_lines {
        // Safely extract the substring for this column range
        let slice = if start < line.len() {
            let actual_end = end.min(line.len());
            &line[start..actual_end]
        } else {
            ""
        };
        
        // Trim whitespace and try to parse as number
        let trimmed = slice.trim();
        if !trimmed.is_empty() {
            if let Ok(num) = trimmed.parse::<i64>() {
                numbers.push(num);
            }
            // If it doesn't parse, skip it (might be malformed)
        }
    }
    
    // Last line contains the operator
    let operator_line = lines.last()?;
    let op_slice = if start < operator_line.len() {
        let actual_end = end.min(operator_line.len());
        &operator_line[start..actual_end]
    } else {
        return None;
    };
    
    // Find the operator character (should be '+' or '*')
    let operator = op_slice.chars().find(|&c| c == '+' || c == '*')?;
    
    // Must have at least one number
    if numbers.is_empty() {
        return None;
    }
    
    Some(Problem { numbers, operator })
}

// ============================================================================
// PART 2: Parse Problem Column-by-Column (Right-to-Left)
// ============================================================================

/// Parse a problem for Part 2: read columns right-to-left, digits top-to-bottom.
///
/// # How it works
/// In cephalopod math, numbers are written vertically in columns:
/// - Each column (top to bottom) contains the digits of ONE number
/// - Most significant digit at top, least significant at bottom
/// - Columns are read right-to-left within each problem
///
/// # Example
/// ```text
/// 64      <- row 0
/// 23      <- row 1  
/// 314     <- row 2
/// +       <- operator row
/// ```
/// Reading right-to-left by column:
/// - Col 2 (rightmost): '4', '3', '4' → but wait, that's chars at positions...
///
/// Actually, for range cols 12-14 in the example:
/// - Col 14: chars ' ', ' ', '4' → "4"
/// - Col 13: chars '4', '3', '1' → "431"  
/// - Col 12: chars '6', '2', '3' → "623"
///
/// Result: 4 + 431 + 623 = 1058
///
/// # Arguments
/// * `lines` - All lines of the worksheet
/// * `start` - Start column (inclusive)
/// * `end` - End column (exclusive)
///
/// # Returns
/// A Problem with numbers read column-wise right-to-left
fn parse_problem_part2(lines: &[&str], start: usize, end: usize) -> Option<Problem> {
    if lines.is_empty() || start >= end {
        return None;
    }
    
    let mut numbers = Vec::new();
    let number_rows = lines.len() - 1; // All rows except the last (operator)
    
    // Read columns right-to-left (from end-1 down to start)
    for col in (start..end).rev() {
        // Collect digits from this column, top to bottom
        let mut digit_chars = String::new();
        
        for row in 0..number_rows {
            if let Some(line) = lines.get(row) {
                if let Some(ch) = line.chars().nth(col) {
                    if ch.is_ascii_digit() {
                        digit_chars.push(ch);
                    }
                    // Skip spaces - they're just padding
                }
            }
        }
        
        // If we got any digits, parse as a number
        if !digit_chars.is_empty() {
            if let Ok(num) = digit_chars.parse::<i64>() {
                numbers.push(num);
            }
        }
    }
    
    // Get operator from last row (same as Part 1)
    let operator_line = lines.last()?;
    let op_slice = if start < operator_line.len() {
        let actual_end = end.min(operator_line.len());
        &operator_line[start..actual_end]
    } else {
        return None;
    };
    
    let operator = op_slice.chars().find(|&c| c == '+' || c == '*')?;
    
    if numbers.is_empty() {
        return None;
    }
    
    Some(Problem { numbers, operator })
}

// ============================================================================
// STEP 4: Compute Problem Result and Solve
// ============================================================================

impl Problem {
    /// Compute the result of this problem by applying the operator.
    ///
    /// # How it works
    /// - If operator is '+': sum all numbers
    /// - If operator is '*': multiply all numbers
    ///
    /// # Returns
    /// The computed result as i64
    fn compute(&self) -> i64 {
        match self.operator {
            '+' => self.numbers.iter().sum(),
            '*' => self.numbers.iter().product(),
            _ => 0, // Should never happen if parsing is correct
        }
    }
}

// Placeholder for Part 1 and Part 2 (we'll build these up)
pub fn solve_part1(input: &str) -> Result<String> {
    let lines: Vec<&str> = input.lines().collect();
    
    if lines.is_empty() {
        anyhow::bail!("Empty input");
    }
    
    // Step 1: Find separator columns
    let separators = find_separator_columns(&lines);
    
    // Step 2: Get problem ranges
    let width = lines[0].len();
    let ranges = get_problem_ranges(&separators, width);
    
    // Step 3 & 4: Parse each problem and compute results
    let mut total: i64 = 0;
    for (start, end) in ranges {
        if let Some(problem) = parse_problem(&lines, start, end) {
            let result = problem.compute();
            total += result;
        }
    }
    
    Ok(total.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let lines: Vec<&str> = input.lines().collect();
    
    if lines.is_empty() {
        anyhow::bail!("Empty input");
    }
    
    // Step 1: Find separator columns (same as Part 1)
    let separators = find_separator_columns(&lines);
    
    // Step 2: Get problem ranges (same as Part 1)
    let width = lines[0].len();
    let ranges = get_problem_ranges(&separators, width);
    
    // Step 3 & 4: Parse each problem with Part 2 logic and compute results
    let mut total: i64 = 0;
    for (start, end) in ranges {
        if let Some(problem) = parse_problem_part2(&lines, start, end) {
            let result = problem.compute();
            total += result;
        }
    }
    
    Ok(total.to_string())
}

// ============================================================================
// TESTS - We build these alongside each function
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Test data from problem statement
    const EXAMPLE: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    /// Test Step 1: find_separator_columns
    /// 
    /// In the example, let's trace the columns:
    /// ```
    /// Col: 0123456789...
    ///      123 328  51 64 
    ///       45 64  387 23 
    ///        6 98  215 314
    ///      *   +   *   +  
    /// ```
    /// - Col 0: '1', ' ', ' ', '*' → NOT all spaces
    /// - Col 3: ' ', ' ', ' ', ' ' → ALL spaces → SEPARATOR
    /// - Col 7: ' ', ' ', ' ', ' ' → ALL spaces → SEPARATOR
    /// - Col 8: ' ', ' ', ' ', ' ' → ALL spaces → SEPARATOR
    #[test]
    fn test_find_separator_columns() {
        let lines: Vec<&str> = EXAMPLE.lines().collect();
        let separators = find_separator_columns(&lines);
        
        println!("Lines:");
        for (i, line) in lines.iter().enumerate() {
            println!("  {}: '{}' (len={})", i, line, line.len());
        }
        println!("Separators found: {:?}", separators);
        
        // Column 3 (between "123" and "328") should be a separator
        assert!(separators.contains(&3), "Column 3 should be separator");
        
        // Columns within numbers should NOT be separators
        assert!(!separators.contains(&0), "Column 0 is part of '123', not separator");
        assert!(!separators.contains(&1), "Column 1 is part of '123'/'45'/'6', not separator");
        assert!(!separators.contains(&4), "Column 4 is part of '328', not separator");
    }
    
    #[test]
    fn test_simple_two_column() {
        // Simpler test case: two single-digit columns
        let input = "1 2\n3 4\n* +";
        let lines: Vec<&str> = input.lines().collect();
        let separators = find_separator_columns(&lines);
        
        println!("Simple test separators: {:?}", separators);
        
        // Column 1 should be the only separator
        assert_eq!(separators, vec![1], "Only column 1 should be separator");
    }
    
    // ========================================================================
    // STEP 2 TESTS: get_problem_ranges
    // ========================================================================
    
    #[test]
    fn test_get_problem_ranges_simple() {
        // Two columns separated by single space at col 1
        // "1 2" -> width 3, separator at col 1
        let separators = vec![1];
        let ranges = get_problem_ranges(&separators, 3);
        
        println!("Simple ranges: {:?}", ranges);
        
        // Should have 2 problems: (0,1) and (2,3)
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0], (0, 1), "First problem: col 0");
        assert_eq!(ranges[1], (2, 3), "Second problem: col 2");
    }
    
    #[test]
    fn test_get_problem_ranges_example() {
        // From our example: separators at [3, 7, 11], width 15
        // But wait - looking at the actual data:
        // '123 328  51 64 ' 
        // The separators should handle the double-space between 328 and 51
        
        let lines: Vec<&str> = EXAMPLE.lines().collect();
        let separators = find_separator_columns(&lines);
        let width = lines[0].len();
        
        println!("Example separators: {:?}, width: {}", separators, width);
        
        let ranges = get_problem_ranges(&separators, width);
        println!("Example ranges: {:?}", ranges);
        
        // Should have 4 problems (matching the 4 operations in example)
        assert_eq!(ranges.len(), 4, "Should have 4 problems");
    }
    
    #[test]
    fn test_get_problem_ranges_consecutive_separators() {
        // Test case with consecutive separators (double space)
        // "12  34" has separators at cols 2 and 3
        let separators = vec![2, 3];
        let ranges = get_problem_ranges(&separators, 6);
        
        println!("Consecutive separator ranges: {:?}", ranges);
        
        // Should have 2 problems: (0,2) and (4,6)
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0], (0, 2), "First problem: cols 0-1");
        assert_eq!(ranges[1], (4, 6), "Second problem: cols 4-5");
    }
    
    // ========================================================================
    // STEP 3 TESTS: parse_problem
    // ========================================================================
    
    #[test]
    fn test_parse_problem_simple() {
        // Simple case: single column with numbers and operator
        // Note: last line is operator, all others are numbers
        let input = "1\n2\n3\n*";
        let lines: Vec<&str> = input.lines().collect();
        
        let problem = parse_problem(&lines, 0, 1).unwrap();
        
        println!("Simple problem: {:?}", problem);
        
        assert_eq!(problem.numbers, vec![1, 2, 3]);
        assert_eq!(problem.operator, '*');
    }
    
    #[test]
    fn test_parse_problem_from_example() {
        // Test parsing first problem from example (cols 0-3)
        // Lines:
        //   "123 328  51 64 "
        //   " 45 64  387 23 "
        //   "  6 98  215 314"
        //   "*   +   *   +  "
        let lines: Vec<&str> = EXAMPLE.lines().collect();
        
        // First problem: cols 0-3 should give [123, 45, 6] with '*'
        let problem1 = parse_problem(&lines, 0, 3).unwrap();
        println!("Problem 1: {:?}", problem1);
        assert_eq!(problem1.numbers, vec![123, 45, 6]);
        assert_eq!(problem1.operator, '*');
        
        // Second problem: cols 4-7 should give [328, 64, 98] with '+'
        let problem2 = parse_problem(&lines, 4, 7).unwrap();
        println!("Problem 2: {:?}", problem2);
        assert_eq!(problem2.numbers, vec![328, 64, 98]);
        assert_eq!(problem2.operator, '+');
    }
    
    #[test]
    fn test_parse_all_example_problems() {
        let lines: Vec<&str> = EXAMPLE.lines().collect();
        let separators = find_separator_columns(&lines);
        let ranges = get_problem_ranges(&separators, lines[0].len());
        
        println!("Parsing all problems from example:");
        for (i, (start, end)) in ranges.iter().enumerate() {
            let problem = parse_problem(&lines, *start, *end).unwrap();
            println!("  Problem {}: cols {}..{} -> {:?}", i + 1, start, end, problem);
        }
        
        // Verify we got 4 problems
        assert_eq!(ranges.len(), 4);
    }
    
    // ========================================================================
    // STEP 4 TESTS: compute and solve_part1
    // ========================================================================
    
    #[test]
    fn test_problem_compute() {
        // Test multiplication
        let mul_problem = Problem {
            numbers: vec![123, 45, 6],
            operator: '*',
        };
        assert_eq!(mul_problem.compute(), 33210);
        
        // Test addition
        let add_problem = Problem {
            numbers: vec![328, 64, 98],
            operator: '+',
        };
        assert_eq!(add_problem.compute(), 490);
    }
    
    #[test]
    fn test_part1_example() {
        // From problem statement:
        // 123 * 45 * 6 = 33210
        // 328 + 64 + 98 = 490
        // 51 * 387 * 215 = 4243455
        // 64 + 23 + 314 = 401
        // Total: 33210 + 490 + 4243455 + 401 = 4277556
        
        let result = solve_part1(EXAMPLE).unwrap();
        println!("Part 1 example result: {}", result);
        assert_eq!(result, "4277556");
    }
    
    // ========================================================================
    // PART 2 TESTS: parse_problem_part2 and solve_part2
    // ========================================================================
    
    #[test]
    fn test_parse_problem_part2_rightmost() {
        // Test the rightmost problem from the example
        // Columns 12-14 (area "64 " / "23 " / "314" / "+  ")
        // Reading columns right-to-left:
        //   Col 14: ' ', ' ', '4' → "4"
        //   Col 13: '4', '3', '1' → "431"
        //   Col 12: '6', '2', '3' → "623"
        // Result: 4 + 431 + 623 = 1058
        
        let lines: Vec<&str> = EXAMPLE.lines().collect();
        let problem = parse_problem_part2(&lines, 12, 15).unwrap();
        
        println!("Part 2 rightmost problem: {:?}", problem);
        
        // Numbers read right-to-left by column
        assert_eq!(problem.numbers, vec![4, 431, 623]);
        assert_eq!(problem.operator, '+');
        assert_eq!(problem.compute(), 1058);
    }
    
    #[test]
    fn test_parse_all_part2_problems() {
        let lines: Vec<&str> = EXAMPLE.lines().collect();
        let separators = find_separator_columns(&lines);
        let ranges = get_problem_ranges(&separators, lines[0].len());
        
        println!("Part 2 - Parsing all problems:");
        for (i, (start, end)) in ranges.iter().enumerate() {
            let problem = parse_problem_part2(&lines, *start, *end).unwrap();
            let result = problem.compute();
            println!("  Problem {}: cols {}..{} -> {:?} = {}", i + 1, start, end, problem, result);
        }
    }
    
    #[test]
    fn test_part2_example() {
        // From problem statement Part 2:
        // Rightmost: 4 + 431 + 623 = 1058
        // Second: 175 * 581 * 32 = 3253600
        // Third: 8 + 248 + 369 = 625
        // Leftmost: 356 * 24 * 1 = 8544
        // Total: 1058 + 3253600 + 625 + 8544 = 3263827
        
        let result = solve_part2(EXAMPLE).unwrap();
        println!("Part 2 example result: {}", result);
        assert_eq!(result, "3263827");
    }
}

