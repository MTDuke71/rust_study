use anyhow::Result;

/// AoC 2015 Day 10: Elves Look, Elves Say
/// Implement the look-and-say sequence (run-length encoding iteration)
/// 
/// ## Examples
/// - "1" becomes "11" (one 1)
/// - "11" becomes "21" (two 1s)
/// - "21" becomes "1211" (one 2, one 1)
/// - "1211" becomes "111221" (one 1, one 2, two 1s)
/// - "111221" becomes "312211" (three 1s, two 2s, one 1)

pub fn solve_part1(input: &str) -> Result<String> {
    println!("🎄 Day 10 Part 1: Apply look-and-say 40 times");
    
    let start_sequence = input.trim();
    let mut current = start_sequence.to_string();
    
    for _ in 0..40 {
        current = look_and_say(&current);
    }
    
    Ok(current.len().to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    println!("🎄 Day 10 Part 2: Apply look-and-say 50 times");
    
    let start_sequence = input.trim();
    let mut current = start_sequence.to_string();
    
    for _ in 0..50 {
        current = look_and_say(&current);
    }
    
    Ok(current.len().to_string())
}

/// Applies one iteration of the look-and-say sequence
/// 
/// # Examples
/// 
/// ```
/// # use aoc2015::solver::day10_sol::look_and_say;
/// assert_eq!(look_and_say("1"), "11");
/// assert_eq!(look_and_say("11"), "21");
/// assert_eq!(look_and_say("21"), "1211");
/// assert_eq!(look_and_say("1211"), "111221");
/// assert_eq!(look_and_say("111221"), "312211");
/// ```
pub fn look_and_say(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    
    if chars.is_empty() {
        return result;
    }
    
    let mut i = 0;
    while i < chars.len() {
        let current_char = chars[i];
        let mut count = 1;
        
        // Count consecutive occurrences
        while i + count < chars.len() && chars[i + count] == current_char {
            count += 1;
        }
        
        // Append count and character
        result.push_str(&count.to_string());
        result.push(current_char);
        
        i += count;
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say_examples() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
    
    #[test]
    fn test_look_and_say_empty() {
        assert_eq!(look_and_say(""), "");
    }
    
    #[test]
    fn test_look_and_say_single() {
        assert_eq!(look_and_say("5"), "15");
    }
    
    #[test]
    fn test_look_and_say_multiple_groups() {
        assert_eq!(look_and_say("1223"), "112213");
    }
}
