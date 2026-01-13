//! Classic Fibonacci example demonstrating memoization benefits
//! 
//! Compares naive recursive vs memoized implementations to show
//! exponential → linear complexity transformation.

use std::collections::HashMap;

/// Naive recursive Fibonacci - O(2^n) exponential time
/// 
/// # Performance: EXPONENTIAL - DO NOT USE FOR n > 40
fn fib_naive(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib_naive(n - 1) + fib_naive(n - 2)
    }
}

/// Memoized Fibonacci - O(n) linear time
/// 
/// # Requirements Satisfied: REQ-1, REQ-2, REQ-6
/// - REQ-1: Generic HashMap memoization
/// - REQ-2: Top-down DP pattern (base case, cache check, compute, insert)
/// - REQ-6: Overlapping subproblem detection via cache reuse
fn fib_memoized(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    // Base case
    if n <= 1 {
        return n;
    }
    
    // Memoization check
    if let Some(&cached) = memo.get(&n) {
        return cached;
    }
    
    // Recursive computation
    let result = fib_memoized(n - 1, memo) + fib_memoized(n - 2, memo);
    
    // Cache and return
    memo.insert(n, result);
    result
}

fn main() {
    println!("=== Fibonacci Memoization Demo ===\n");
    
    // Small value - both work fine
    println!("Small value (n=10):");
    println!("  Naive:     {}", fib_naive(10));
    
    let mut memo = HashMap::new();
    println!("  Memoized:  {}", fib_memoized(10, &mut memo));
    println!("  Cache size: {} entries\n", memo.len());
    
    // Medium value - naive becomes slow
    println!("Medium value (n=30):");
    
    use std::time::Instant;
    
    let start = Instant::now();
    let result_naive = fib_naive(30);
    let time_naive = start.elapsed();
    println!("  Naive:     {} ({:?})", result_naive, time_naive);
    
    let mut memo = HashMap::new();
    let start = Instant::now();
    let result_memoized = fib_memoized(30, &mut memo);
    let time_memoized = start.elapsed();
    println!("  Memoized:  {} ({:?})", result_memoized, time_memoized);
    println!("  Cache size: {} entries", memo.len());
    println!("  Speedup:   {}x\n", time_naive.as_nanos() / time_memoized.as_nanos());
    
    // Large value - only memoized is tractable
    println!("Large value (n=90):");
    println!("  Naive:     SKIPPED (would take hours)");
    
    let mut memo = HashMap::new();
    let start = Instant::now();
    let result = fib_memoized(90, &mut memo);
    let time = start.elapsed();
    println!("  Memoized:  {} ({:?})", result, time);
    println!("  Cache size: {} entries", memo.len());
    
    println!("\n=== Key Insights ===");
    println!("- Naive: O(2^n) - exponential growth makes n>40 impractical");
    println!("- Memoized: O(n) - linear cache fills once, all lookups are O(1)");
    println!("- Cache size equals n (each Fibonacci number computed once)");
    println!("- Memoization transforms intractable → instant");
}
