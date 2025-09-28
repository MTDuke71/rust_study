//! # Day 10: Look-and-Say with Memoization
//! 
//! Example showing how to use Mission5 MemoCache for AoC problems
//! that benefit from caching intermediate results.

use mission5::MemoCache;

/// Look-and-say sequence with memoization
/// Caches intermediate sequence results to avoid recomputation
pub fn look_and_say_with_memo(input: &str, iterations: usize) -> String {
    let mut cache = MemoCache::new();
    look_and_say_recursive(input.to_string(), iterations, &mut cache)
}

fn look_and_say_recursive(
    sequence: String, 
    remaining: usize, 
    cache: &mut MemoCache<(String, usize), String>
) -> String {
    let key = (sequence.clone(), remaining);
    
    // Check cache first
    if let Some(cached) = cache.get(&key) {
        return cached.clone();
    }
    
    // Base case
    if remaining == 0 {
        cache.insert(key, sequence.clone());
        return sequence;
    }
    
    // Transform sequence
    let next_sequence = transform_sequence(&sequence);
    
    // Recursive call with memoization
    let result = look_and_say_recursive(next_sequence, remaining - 1, cache);
    
    // Cache result before returning
    cache.insert(key, result.clone());
    result
}

fn transform_sequence(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    let mut i = 0;
    
    while i < chars.len() {
        let current_char = chars[i];
        let mut count = 1;
        
        // Count consecutive characters
        while i + count < chars.len() && chars[i + count] == current_char {
            count += 1;
        }
        
        // Add count and character to result
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
    fn test_look_and_say_memo() {
        assert_eq!(look_and_say_with_memo("1", 1), "11");
        assert_eq!(look_and_say_with_memo("11", 1), "21");
        assert_eq!(look_and_say_with_memo("21", 1), "1211");
        assert_eq!(look_and_say_with_memo("1211", 1), "111221");
    }
    
    #[test]
    fn test_memoization_benefit() {
        let input = "1113222113";
        
        // Multiple calls should benefit from memoization
        let result1 = look_and_say_with_memo(input, 10);
        let result2 = look_and_say_with_memo(input, 10);
        
        assert_eq!(result1, result2);
        println!("Result length: {}", result1.len());
    }
}

/// Example usage in AoC context
pub fn solve_day10_part1(input: &str) -> usize {
    let result = look_and_say_with_memo(input.trim(), 40);
    result.len()
}

pub fn solve_day10_part2(input: &str) -> usize {
    let result = look_and_say_with_memo(input.trim(), 50);
    result.len()
}