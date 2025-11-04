use anyhow::{Context, Result};
use std::collections::HashMap;

/// Parse input into two vectors of location IDs
/// 
/// Internal function used by solve_part1 and solve_part2.
/// Example usage is demonstrated in the public solver functions.
fn parse_location_lists(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let pairs: Result<Vec<(i32, i32)>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(line_num, line)| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            match parts.as_slice() {
                [left_str, right_str] => {
                    let left = left_str.parse::<i32>()
                        .with_context(|| format!("Failed to parse left number '{}' on line {}", left_str, line_num + 1))?;
                    let right = right_str.parse::<i32>()
                        .with_context(|| format!("Failed to parse right number '{}' on line {}", right_str, line_num + 1))?;
                    Ok((left, right))
                }
                _ => anyhow::bail!("Invalid format on line {}: expected exactly 2 numbers, got {}", 
                                 line_num + 1, parts.len()),
            }
        })
        .collect();

    let pairs = pairs?;
    let (left_list, right_list): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    
    Ok((left_list, right_list))
}

/// Part 1: Calculate total distance between sorted lists
/// 
/// Pairs up smallest numbers from each list and sums absolute differences.
/// This is much more idiomatic than manual indexing and mutation.
pub fn solve_part1(input: &str) -> Result<String> {
    let (mut left_list, mut right_list) = parse_location_lists(input)
        .context("Failed to parse location lists for Part 1")?;
    
    // Sort both lists in place - more efficient than creating new sorted vectors
    left_list.sort_unstable();
    right_list.sort_unstable();
    
    // Functional approach: zip sorted lists, calculate differences, sum them
    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();
    
    Ok(total_distance.to_string())
}

/// Part 2: Calculate similarity score using frequency counting
/// 
/// Uses HashMap for O(1) lookups instead of linear searches.
/// Much more efficient than Python's Counter for large inputs.
pub fn solve_part2(input: &str) -> Result<String> {
    let (left_list, right_list) = parse_location_lists(input)
        .context("Failed to parse location lists for Part 2")?;
    
    // Build frequency map of right list - O(n) time complexity
    let right_frequencies: HashMap<i32, i32> = right_list
        .iter()
        .fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });
    
    // Calculate similarity score using functional approach
    let similarity_score: i32 = left_list
        .iter()
        .map(|&left_num| {
            let frequency = right_frequencies.get(&left_num).copied().unwrap_or(0);
            left_num * frequency
        })
        .sum();
    
    Ok(similarity_score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn test_parse_location_lists() {
        let (left, right) = parse_location_lists(EXAMPLE_INPUT).unwrap();
        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "11");
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "31");
    }

    #[test] 
    fn test_empty_input() {
        let result = parse_location_lists("");
        assert!(result.is_ok());
        let (left, right) = result.unwrap();
        assert!(left.is_empty());
        assert!(right.is_empty());
    }

    #[test]
    fn test_invalid_format() {
        let invalid_input = "3 4 5"; // Three numbers instead of two
        let result = parse_location_lists(invalid_input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected exactly 2 numbers"));
    }

    #[test]
    fn test_non_numeric_input() {
        let invalid_input = "abc   def";
        let result = parse_location_lists(invalid_input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to parse"));
    }
}
