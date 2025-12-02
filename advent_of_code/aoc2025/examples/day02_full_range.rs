//! Benchmark: Day 2 full 32-bit range challenge
//!
//! Reddit challenge: How long to calculate for range 1 to 4,294,967,296 (2^32)?
//!
//! Run with: cargo run --release --example day02_full_range

use std::time::Instant;

/// Check if a number is a "doubled" pattern (e.g., 55, 1010, 123123)
fn is_doubled(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    
    if len % 2 != 0 {
        return false;
    }
    
    let half = len / 2;
    let (first, second) = s.split_at(half);
    first == second
}

/// Check if a number is made of a pattern repeated at least twice.
fn is_repeated(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    
    for pattern_len in 1..=len / 2 {
        if len % pattern_len != 0 {
            continue;
        }
        
        let pattern = &s[..pattern_len];
        let repetitions = len / pattern_len;
        
        if repetitions < 2 {
            continue;
        }
        
        let reconstructed: String = pattern.repeat(repetitions);
        if reconstructed == s {
            return true;
        }
    }
    
    false
}

fn main() {
    const MAX: u64 = 4_294_967_296; // 2^32
    
    println!("=== Day 2 Full Range Benchmark ===");
    println!("Range: 1 to {} (2^32)", MAX);
    println!();
    
    // Part 1: Doubled sequences
    println!("Part 1: Finding doubled sequences (55, 1010, 123123)...");
    let start = Instant::now();
    
    let sum_part1: u64 = (1..=MAX)
        .filter(|&n| is_doubled(n))
        .sum();
    
    let elapsed_part1 = start.elapsed();
    println!("  Sum of doubled IDs: {}", sum_part1);
    println!("  Time: {:.2?}", elapsed_part1);
    println!();
    
    // Part 2: Repeated sequences (at least twice)
    println!("Part 2: Finding repeated sequences (111, 1212, 123123123)...");
    let start = Instant::now();
    
    let sum_part2: u64 = (1..=MAX)
        .filter(|&n| is_repeated(n))
        .sum();
    
    let elapsed_part2 = start.elapsed();
    println!("  Sum of repeated IDs: {}", sum_part2);
    println!("  Time: {:.2?}", elapsed_part2);
    println!();
    
    println!("=== Summary ===");
    println!("Part 1: {} in {:.2?}", sum_part1, elapsed_part1);
    println!("Part 2: {} in {:.2?}", sum_part2, elapsed_part2);
    println!("Total time: {:.2?}", elapsed_part1 + elapsed_part2);
}
