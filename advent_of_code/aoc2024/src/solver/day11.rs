use anyhow::{Context, Result};
use std::collections::HashMap;

/// Parse the initial arrangement of stones.
/// Input is a single line with space-separated numbers.
fn parse_stones(input: &str) -> Result<Vec<u64>> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u64>().with_context(|| format!("Invalid number: {}", s)))
        .collect()
}

/// Count the number of digits in a number using logarithm.
/// More efficient than string conversion.
fn count_digits(n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    (n as f64).log10().floor() as u32 + 1
}

/// Apply the transformation rules to a single stone.
/// Returns a vector of new stones (1 or 2 stones).
///
/// Rules:
/// 1. If stone is 0 → becomes 1
/// 2. If stone has even number of digits → split into left half and right half
/// 3. Otherwise → multiply by 2024
fn transform_stone(stone: u64) -> Vec<u64> {
    if stone == 0 {
        // Rule 1: 0 becomes 1
        vec![1]
    } else {
        // Math-based approach: use logarithm instead of string conversion
        let num_digits = count_digits(stone);
        
        if num_digits.is_multiple_of(2) {
            // Rule 2: Even number of digits - split using integer math
            let divisor = 10_u64.pow(num_digits / 2);
            let left = stone / divisor;
            let right = stone % divisor;
            vec![left, right]
        } else {
            // Rule 3: Multiply by 2024
            vec![stone * 2024]
        }
    }
}

/* Original string-based approach (slower due to allocations):
fn transform_stone(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else {
        let num_str = stone.to_string();
        let num_digits = num_str.len();
        
        if num_digits % 2 == 0 {
            let mid = num_digits / 2;
            let left = num_str[..mid].parse::<u64>().unwrap();
            let right = num_str[mid..].parse::<u64>().unwrap();
            vec![left, right]
        } else {
            vec![stone * 2024]
        }
    }
}
*/

/// Simulate blinks by applying transformation rules.
/// Returns the stones after the specified number of blinks.
fn simulate_blinks(initial_stones: Vec<u64>, blinks: usize) -> Vec<u64> {
    let mut stones = initial_stones;
    
    for _ in 0..blinks {
        stones = stones
            .into_iter()
            .flat_map(transform_stone)
            .collect();
    }
    
    stones
}

/// Count stones after blinks using memoization for efficiency.
/// Uses dynamic programming: cache (stone_value, blinks_remaining) -> count
fn count_stones_memoized(
    stone: u64,
    blinks_remaining: usize,
    cache: &mut HashMap<(u64, usize), usize>,
) -> usize {
    // Base case: no more blinks, this stone counts as 1
    if blinks_remaining == 0 {
        return 1;
    }

    // Check cache
    if let Some(&cached) = cache.get(&(stone, blinks_remaining)) {
        return cached;
    }

    // Apply transformation and recursively count
    let transformed = transform_stone(stone);
    let count = transformed
        .iter()
        .map(|&s| count_stones_memoized(s, blinks_remaining - 1, cache))
        .sum();

    // Cache the result
    cache.insert((stone, blinks_remaining), count);
    count
}

/// Count stones with detailed tracing for educational purposes.
#[allow(dead_code)]
fn count_stones_with_trace(
    stone: u64,
    blinks_remaining: usize,
    cache: &mut HashMap<(u64, usize), usize>,
    depth: usize,
) -> usize {
    let indent = "  ".repeat(depth);
    
    // Base case: no more blinks, this stone counts as 1
    if blinks_remaining == 0 {
        println!("{}Stone {} with 0 blinks → count = 1 (base case)", indent, stone);
        return 1;
    }

    // Check cache
    if let Some(&cached) = cache.get(&(stone, blinks_remaining)) {
        println!("{}Stone {} with {} blinks → count = {} (CACHED ✓)", 
                 indent, stone, blinks_remaining, cached);
        return cached;
    }

    // Apply transformation and recursively count
    let transformed = transform_stone(stone);
    println!("{}Stone {} with {} blinks → transforms to {:?}", 
             indent, stone, blinks_remaining, transformed);
    
    let count = transformed
        .iter()
        .map(|&s| count_stones_with_trace(s, blinks_remaining - 1, cache, depth + 1))
        .sum();

    // Cache the result
    println!("{}→ CACHE: ({}, {}) = {}", indent, stone, blinks_remaining, count);
    cache.insert((stone, blinks_remaining), count);
    count
}

/// Count total stones after blinks using memoization.
fn count_stones_after_blinks(initial_stones: &[u64], blinks: usize) -> usize {
    let mut cache = HashMap::new();
    initial_stones
        .iter()
        .map(|&stone| count_stones_memoized(stone, blinks, &mut cache))
        .sum()
}

/// Count total stones and return both count and cache size for analysis.
#[allow(dead_code)]
fn count_with_cache_stats(initial_stones: &[u64], blinks: usize) -> (usize, usize) {
    let mut cache = HashMap::new();
    let count: usize = initial_stones
        .iter()
        .map(|&stone| count_stones_memoized(stone, blinks, &mut cache))
        .sum();
    (count, cache.len())
}

pub fn solve_part1(input: &str) -> Result<String> {
    let stones = parse_stones(input)?;
    let final_stones = simulate_blinks(stones, 25);
    Ok(final_stones.len().to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let stones = parse_stones(input)?;
    let count = count_stones_after_blinks(&stones, 75);
    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let input = "125 17";
        let stones = parse_stones(input).unwrap();
        assert_eq!(stones, vec![125, 17]);
    }

    #[test]
    fn test_parse_single_stone() {
        let input = "0";
        let stones = parse_stones(input).unwrap();
        assert_eq!(stones, vec![0]);
    }

    #[test]
    fn test_parse_multiple_stones() {
        let input = "0 1 10 99 999";
        let stones = parse_stones(input).unwrap();
        assert_eq!(stones, vec![0, 1, 10, 99, 999]);
    }

    #[test]
    fn test_parse_with_leading_trailing_whitespace() {
        let input = "  125 17  \n";
        let stones = parse_stones(input).unwrap();
        assert_eq!(stones, vec![125, 17]);
    }

    #[test]
    fn test_parse_invalid_number() {
        let input = "125 abc";
        assert!(parse_stones(input).is_err());
    }

    #[test]
    fn test_transform_rule1_zero_becomes_one() {
        // Rule 1: 0 → 1
        assert_eq!(transform_stone(0), vec![1]);
    }

    #[test]
    fn test_transform_rule2_even_digits_split() {
        // Rule 2: Even number of digits splits in half
        assert_eq!(transform_stone(10), vec![1, 0]);
        assert_eq!(transform_stone(99), vec![9, 9]);
        assert_eq!(transform_stone(1000), vec![10, 0]);
        assert_eq!(transform_stone(253000), vec![253, 0]);
    }

    #[test]
    fn test_transform_rule3_multiply_2024() {
        // Rule 3: Odd number of digits → multiply by 2024
        assert_eq!(transform_stone(1), vec![2024]);
        assert_eq!(transform_stone(999), vec![999 * 2024]);
    }

    #[test]
    fn test_simulate_example_1_blink() {
        // Example from problem: "125 17" after 1 blink
        // 125 has 3 digits (odd) → 125 * 2024 = 253000
        // 17 has 2 digits (even) → splits to 1 and 7
        let stones = vec![125, 17];
        let result = simulate_blinks(stones, 1);
        assert_eq!(result, vec![253000, 1, 7]);
    }

    #[test]
    fn test_simulate_example_2_blinks() {
        // After 2 blinks from "125 17":
        // [253000, 1, 7]
        // 253000 (6 digits, even) → 253, 0
        // 1 (1 digit, odd) → 2024
        // 7 (1 digit, odd) → 14168
        let stones = vec![125, 17];
        let result = simulate_blinks(stones, 2);
        assert_eq!(result, vec![253, 0, 2024, 14168]);
    }

    #[test]
    fn test_simulate_example_6_blinks() {
        // After 6 blinks, the example shows 22 stones
        let stones = vec![125, 17];
        let result = simulate_blinks(stones, 6);
        assert_eq!(result.len(), 22);
    }

    #[test]
    fn test_simulate_example_25_blinks() {
        // After 25 blinks, the example shows 55312 stones
        let stones = vec![125, 17];
        let result = simulate_blinks(stones, 25);
        assert_eq!(result.len(), 55312);
    }

    #[test]
    fn test_memoized_matches_naive_25_blinks() {
        // Verify memoized version matches naive for 25 blinks
        let stones = vec![125, 17];
        let naive_count = simulate_blinks(stones.clone(), 25).len();
        let memoized_count = count_stones_after_blinks(&stones, 25);
        assert_eq!(memoized_count, naive_count);
        assert_eq!(memoized_count, 55312);
    }

    #[test]
    fn test_memoized_small_example() {
        // Test memoization with smaller examples
        let stones = vec![0];
        assert_eq!(count_stones_after_blinks(&stones, 1), 1); // 0 -> 1
        
        let stones = vec![1];
        assert_eq!(count_stones_after_blinks(&stones, 1), 1); // 1 -> 2024
        
        let stones = vec![10];
        assert_eq!(count_stones_after_blinks(&stones, 1), 2); // 10 -> 1, 0
    }

    #[test]
    #[ignore] // Expensive test - run manually
    fn test_naive_simulation_limits() {
        use std::time::Instant;
        
        // Test how far naive simulation can go before hitting limits
        let stones = vec![125, 17];
        
        for blinks in [25, 30, 35, 40] {
            let start = Instant::now();
            let result = simulate_blinks(stones.clone(), blinks);
            let elapsed = start.elapsed();
            
            println!("\nNaive simulation - {} blinks:", blinks);
            println!("  Stones: {}", result.len());
            println!("  Time: {:?}", elapsed);
            println!("  Memory estimate: ~{} bytes", result.len() * std::mem::size_of::<u64>());
        }
    }

    #[test]
    #[ignore] // Expensive test - run manually
    fn test_memoized_vs_naive_comparison() {
        use std::time::Instant;
        
        let stones = vec![125, 17];
        
        println!("\nComparing naive vs memoized:");
        
        // Test both approaches up to where naive is still feasible
        for blinks in [10, 15, 20, 25] {
            // Naive
            let start = Instant::now();
            let naive_result = simulate_blinks(stones.clone(), blinks);
            let naive_time = start.elapsed();
            
            // Memoized
            let start = Instant::now();
            let memo_result = count_stones_after_blinks(&stones, blinks);
            let memo_time = start.elapsed();
            
            println!("\n{} blinks:", blinks);
            println!("  Naive:    {} stones in {:?}", naive_result.len(), naive_time);
            println!("  Memoized: {} stones in {:?}", memo_result, memo_time);
            assert_eq!(naive_result.len(), memo_result);
        }
    }

    #[test]
    fn test_cache_size_analysis() {
        // Analyze how many unique (stone, blinks) states we actually cache
        let stones = vec![125, 17];
        
        println!("\nCache size analysis for example input [125, 17]:");
        for blinks in [10, 25, 50, 75] {
            let (count, cache_size) = count_with_cache_stats(&stones, blinks);
            println!("  {} blinks: {} stones, {} cache entries", blinks, count, cache_size);
        }
        
        // Test with actual puzzle input
        let puzzle_input = vec![77, 515, 6779622, 6, 91370, 959685, 0, 9861];
        println!("\nCache size analysis for puzzle input:");
        for blinks in [25, 75] {
            let (count, cache_size) = count_with_cache_stats(&puzzle_input, blinks);
            println!("  {} blinks: {} stones, {} cache entries", blinks, count, cache_size);
        }
    }

    #[test]
    fn test_trace_example_5_blinks() {
        // Trace execution with 3 initial stones and 5 blinks
        println!("\n=== TRACING: [0, 1, 2] with 5 blinks ===\n");
        
        let stones = vec![0, 1, 2];
        let mut cache = HashMap::new();
        
        let mut total = 0;
        for stone in stones {
            println!("Processing initial stone: {}", stone);
            let count = count_stones_with_trace(stone, 5, &mut cache, 1);
            println!("→ Stone {} produces {} stones after 5 blinks\n", stone, count);
            total += count;
        }
        
        println!("=== FINAL RESULTS ===");
        println!("Total stones: {}", total);
        println!("Cache entries: {}", cache.len());
        println!("\n=== CACHE CONTENTS ===");
        let mut cache_vec: Vec<_> = cache.iter().collect();
        cache_vec.sort_by_key(|((stone, blinks), _)| (blinks, stone));
        for ((stone, blinks), count) in cache_vec {
            println!("  ({}, {}) → {}", stone, blinks, count);
        }
    }
}
