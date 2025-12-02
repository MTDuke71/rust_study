//! Benchmark: Day 2 full 32-bit range - OPTIMIZED
//!
//! Key insight: Instead of checking every number, GENERATE valid patterns directly.
//!
//! For a number with D digits, valid patterns have length P where:
//! - D % P == 0 (pattern length must divide digit count evenly)
//! - D / P >= 2 (must repeat at least twice)
//!
//! Example: 7-digit numbers can only have pattern length 1 (7/1=7 repetitions)
//!          since 7 is prime. So only 1111111, 2222222, ..., 9999999 are valid.
//!
//! Run with: cargo run --release --example day02_full_range_optimized

use std::time::Instant;

/// Generate all "doubled" numbers (exactly 2 repetitions) within a range
fn generate_doubled_in_range(start: u64, end: u64) -> Vec<u64> {
    let mut results = Vec::new();
    
    // Determine digit range we need to cover
    let min_digits = if start == 0 { 1 } else { start.to_string().len() };
    let max_digits = end.to_string().len();
    
    for total_digits in min_digits..=max_digits {
        // For "doubled", we need exactly 2 repetitions, so pattern_len = total_digits / 2
        if total_digits % 2 != 0 {
            continue; // Odd digit count can't be doubled
        }
        
        let pattern_len = total_digits / 2;
        
        // Generate all patterns of this length
        // Pattern must start with 1-9 (no leading zeros), rest can be 0-9
        let pattern_start = 10_u64.pow((pattern_len - 1) as u32); // e.g., 100 for 3 digits
        let pattern_end = 10_u64.pow(pattern_len as u32) - 1;     // e.g., 999 for 3 digits
        
        for pattern in pattern_start..=pattern_end {
            // Create doubled number: pattern repeated twice
            let doubled = pattern * 10_u64.pow(pattern_len as u32) + pattern;
            
            if doubled >= start && doubled <= end {
                results.push(doubled);
            }
        }
    }
    
    results
}

/// Generate all "repeated" numbers (at least 2 repetitions) within a range
fn generate_repeated_in_range(start: u64, end: u64) -> Vec<u64> {
    let mut results = std::collections::HashSet::new();
    
    let min_digits = if start == 0 { 1 } else { start.to_string().len() };
    let max_digits = end.to_string().len();
    
    for total_digits in min_digits..=max_digits {
        // Find all valid pattern lengths for this digit count
        for pattern_len in 1..=total_digits / 2 {
            if total_digits % pattern_len != 0 {
                continue;
            }
            
            let repetitions = total_digits / pattern_len;
            if repetitions < 2 {
                continue;
            }
            
            // Generate all patterns of this length
            let pattern_start = if pattern_len == 1 { 1 } else { 10_u64.pow((pattern_len - 1) as u32) };
            let pattern_end = 10_u64.pow(pattern_len as u32) - 1;
            
            for pattern in pattern_start..=pattern_end {
                // Create repeated number
                let mut repeated = 0_u64;
                let multiplier = 10_u64.pow(pattern_len as u32);
                
                for _ in 0..repetitions {
                    repeated = repeated * multiplier + pattern;
                }
                
                if repeated >= start && repeated <= end {
                    results.insert(repeated);
                }
            }
        }
    }
    
    results.into_iter().collect()
}

fn main() {
    const MAX: u64 = 4_294_967_296; // 2^32
    
    println!("=== Day 2 Full Range Benchmark (OPTIMIZED) ===");
    println!("Range: 1 to {} (2^32)", MAX);
    println!();
    println!("Strategy: Generate valid patterns directly instead of checking every number");
    println!();
    
    // Part 1: Doubled sequences
    println!("Part 1: Generating doubled sequences (55, 1010, 123123)...");
    let start = Instant::now();
    
    let doubled = generate_doubled_in_range(1, MAX);
    let sum_part1: u64 = doubled.iter().sum();
    let count_part1 = doubled.len();
    
    let elapsed_part1 = start.elapsed();
    println!("  Count of doubled IDs: {}", count_part1);
    println!("  Sum of doubled IDs: {}", sum_part1);
    println!("  Time: {:.2?}", elapsed_part1);
    println!();
    
    // Part 2: Repeated sequences (at least twice)
    println!("Part 2: Generating repeated sequences (111, 1212, 123123123)...");
    let start = Instant::now();
    
    let repeated = generate_repeated_in_range(1, MAX);
    let sum_part2: u64 = repeated.iter().sum();
    let count_part2 = repeated.len();
    
    let elapsed_part2 = start.elapsed();
    println!("  Count of repeated IDs: {}", count_part2);
    println!("  Sum of repeated IDs: {}", sum_part2);
    println!("  Time: {:.2?}", elapsed_part2);
    println!();
    
    println!("=== Summary ===");
    println!("Part 1: {} (count: {}) in {:.2?}", sum_part1, count_part1, elapsed_part1);
    println!("Part 2: {} (count: {}) in {:.2?}", sum_part2, count_part2, elapsed_part2);
    println!("Total time: {:.2?}", elapsed_part1 + elapsed_part2);
    
    // Show breakdown by digit count
    println!();
    println!("=== Pattern Analysis ===");
    for digits in 1..=10 {
        let divisors: Vec<usize> = (1..=digits/2)
            .filter(|&p| digits % p == 0)
            .collect();
        if !divisors.is_empty() {
            println!("{} digits: pattern lengths {:?}", digits, divisors);
        } else {
            println!("{} digits: NO valid patterns (prime digit count)", digits);
        }
    }
}
