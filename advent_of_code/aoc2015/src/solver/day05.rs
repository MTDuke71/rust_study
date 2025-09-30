use anyhow::Result;
use regex::Regex;

/// Day 05: Doesn't He Have Intern-Elves For This?
pub fn solve_part1(input: &str) -> Result<String> {
    // Process each line of input
    let nice_count = input
        .lines()                          // Split input into lines
        .filter(|line| !line.is_empty())  // Skip empty lines
        .filter(|line| is_nice(line))     // Check if line is "nice"
        .count();                         // Count the nice ones
    
    Ok(nice_count.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let nice_count = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| is_nice_part2(line))
        .count();
    
    Ok(nice_count.to_string())
}

//  Part 1 logic
fn is_nice(s: &str) -> bool {
    let vowels = Regex::new(r"[aeiou]").unwrap();
    let forbidden = Regex::new(r"ab|cd|pq|xy").unwrap();
    
    let vowel_count = vowels.find_iter(s).count();
    let has_double = s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b);
    let has_forbidden = forbidden.is_match(s);
    
    vowel_count >= 3 && has_double && !has_forbidden
}

// Part2 logic
fn is_nice_part2(s: &str) -> bool {
    has_non_overlapping_pair(s) && has_letter_with_one_between(s)
}

/// Rule 1: Contains a pair of letters that appears at least twice without overlapping
/// Examples: 
/// - "xyxy" -> true (xy appears twice, no overlap)
/// - "aabcdefgaa" -> true (aa appears twice, no overlap) 
/// - "aaa" -> false (aa overlaps with itself)
fn has_non_overlapping_pair(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    
    // Check every possible 2-character pair
    for i in 0..chars.len().saturating_sub(1) {
        let pair = (chars[i], chars[i + 1]);
        
        // Look for the same pair later in the string (with no overlap)
        for j in (i + 2)..chars.len().saturating_sub(1) {
            if chars[j] == pair.0 && chars[j + 1] == pair.1 {
                return true;
            }
        }
    }
    
    false
}

/// Rule 2: Contains at least one letter which repeats with exactly one letter between them
/// Examples:
/// - "xyx" -> true (x repeats with y between)
/// - "abcdefeghi" -> true (e repeats with f between)
/// - "aaa" -> true (a repeats with a between)
fn has_letter_with_one_between(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    
    // Check every character with the character 2 positions ahead
    for i in 0..chars.len().saturating_sub(2) {
        if chars[i] == chars[i + 2] {
            return true;
        }
    }
    
    false
}
