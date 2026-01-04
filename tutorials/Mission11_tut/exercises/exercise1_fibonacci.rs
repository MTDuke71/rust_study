//! Exercise 1: Fibonacci Variants
//!
//! **Learning Goals**:
//! - Compare naive recursion vs memoization performance
//! - Practice top-down and bottom-up DP approaches
//! - Understand space optimization techniques
//!
//! **Problem**: Compute Fibonacci numbers using 4 different approaches:
//! 1. Naive recursion (exponential)
//! 2. Top-down with HashMap memoization
//! 3. Bottom-up with Vec table
//! 4. Space-optimized O(1) version
//!
//! **Challenge**: Compute F(100) without overflow using modular arithmetic

use std::collections::HashMap;

fn main() {
    println!("=== Exercise 1: Fibonacci Variants ===\n");
    
    // Test your implementations
    test_implementations();
    
    // Show recursive limits
    demonstrate_recursive_limits();
    
    // Performance comparison
    compare_approaches();
    
    // Large number handling
    test_large_numbers();
    
    // Correctness verification
    verify_correctness();
    
    println!("\n✅ Exercise 1 completed!");
}

fn test_implementations() {
    println!("--- Testing Your Implementations ---\n");
    
    let test_values = vec![
        (0, 0),
        (1, 1),
        (5, 5),
        (10, 55),
        (15, 610),
    ];
    
    for (n, expected) in &test_values {
        let naive = if *n <= 20 { Some(fibonacci_naive(*n)) } else { None };
        let memoized = fibonacci_memoized(*n);
        let bottom_up = fibonacci_bottom_up(*n);
        let optimized = fibonacci_optimized(*n);
        
        println!("F({:2}) = {}", n, expected);
        
        if let Some(result) = naive {
            let status = if result == *expected { "✓" } else { "✗" };
            println!("  {} Naive:      {}", status, result);
        } else {
            println!("    Naive:      <skipped - too slow>");
        }
        
        let status = if memoized == *expected { "✓" } else { "✗" };
        println!("  {} Memoized:   {}", status, memoized);
        
        let status = if bottom_up == *expected { "✓" } else { "✗" };
        println!("  {} Bottom-up:  {}", status, bottom_up);
        
        let status = if optimized == *expected { "✓" } else { "✗" };
        println!("  {} Optimized:  {}", status, optimized);
        
        println!();
    }
}

// ============================================================================
// TODO: Implement these 4 approaches
// ============================================================================

/// Approach 1: Naive Recursion (SLOW - exponential)
/// 
/// Try this with n=5, n=10, n=20, n=40
/// Notice how it becomes unusably slow very quickly!

fn fibonacci_naive(n: u64) -> u64 {
    // TODO: Implement naive recursion
    // Base case: F(0) = 0, F(1) = 1
    // Recursive case: F(n) = F(n-1) + F(n-2)
    if n <= 1 {
        return n;
    }
    fibonacci_naive(n - 1) + fibonacci_naive(n - 2)
}

/// Approach 2: Top-Down with Memoization (FAST - linear)
///
/// This should handle n=100+ easily
fn fibonacci_memoized(n: u64) -> u64 {
    let mut memo = HashMap::new();
    fibonacci_memo_helper(n, &mut memo)
}

fn fibonacci_memo_helper(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    // Base cases: F(0) = 0, F(1) = 1
    if n <= 1 {
        return n;
    }

    // Check if already computed and cached
    if let Some(&cached) = memo.get(&n) {
        return cached;
    }

    // Compute recursively and cache the result
    let result = fibonacci_memo_helper(n - 1, memo) + fibonacci_memo_helper(n - 2, memo);
    memo.insert(n, result);
    result
}

/// Approach 3: Bottom-Up with Table (FAST - linear, no recursion)
///
/// Build table from F(0) up to F(n)
fn fibonacci_bottom_up(n: u64) -> u64 {
    // Base cases: F(0) = 0, F(1) = 1
    if n <= 1 {
        return n;
    }
    
    // Create Vec to store all values from 0 to n
    let n_usize = n as usize;
    let mut dp = vec![0u64; n_usize + 1];
    
    // Initialize F(1) = 1 (F(0) = 0 already from vec initialization)
    dp[1] = 1;
    
    // Fill table: dp[i] = dp[i-1] + dp[i-2]
    for i in 2..=n_usize {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    
    // Return dp[n]
    dp[n_usize]
}

/// Approach 4: Space-Optimized O(1) (FASTEST - no extra space)
///
/// Only need last two values, not entire table

fn fibonacci_optimized(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let mut prev = 0u64;
    let mut curr = 1u64;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}

// ============================================================================
// CHALLENGE: Large Numbers with Modular Arithmetic
// ============================================================================

const MOD: u64 = 1_000_000_007;

/// Compute F(n) mod 1_000_000_007 to avoid overflow
fn fibonacci_modular(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let mut prev = 0u64;
    let mut curr = 1u64;

    for _ in 2..=n {
        // Use modulo to prevent potential overflow when prev + curr exceeds u64::MAX
        let next = (prev + curr) % MOD;
        prev = curr;
        curr = next;
    }

    curr
}

// ============================================================================
// Tests and Demonstrations
// ============================================================================

fn demonstrate_recursive_limits() {
    println!("\n--- Recursive Approach Limitations ---\n");
    println!("Testing naive recursion with increasing values...");
    println!("(Will stop before stack overflow or when too slow)\n");
    
    // NOTE: Times include ~10ms thread spawning overhead
    // For small n (10-30), thread overhead dominates actual computation time
    // For larger n (35+), exponential growth becomes visible
    // See "Performance Comparison" section for actual computation times without thread overhead
    let test_values = vec![10, 20, 30, 35, 40, 45];
    let timeout = std::time::Duration::from_secs(2);
    
    for n in test_values {
        let start = std::time::Instant::now();
        
        // Use a thread with limited stack to catch potential overflow
        let handle = std::thread::Builder::new()
            .stack_size(2 * 1024 * 1024) // 2MB stack (default is usually 2-8MB)
            .spawn(move || fibonacci_naive(n))
            .expect("Failed to spawn thread");
        
        // Wait for result with timeout
        let mut elapsed = std::time::Duration::ZERO;
        let mut result = None;
        
        while elapsed < timeout {
            if handle.is_finished() {
                result = Some(handle.join());
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
            elapsed = start.elapsed();
        }
        
        match result {
            Some(Ok(value)) => {
                let duration = start.elapsed();
                if duration > std::time::Duration::from_millis(500) {
                    println!("F({:2}): {} (took {:>8.1?}) ⚠️  Getting slow!", 
                             n, value, duration);
                } else {
                    println!("F({:2}): {} (took {:>8.1?})", n, value, duration);
                }
            }
            Some(Err(_)) => {
                println!("F({:2}): ❌ STACK OVERFLOW! Recursion depth too large.", n);
                println!("      Naive recursion makes 2^n function calls.");
                println!("      For n={}, that's ~{} million calls!", n, 1 << (n - 20));
                break;
            }
            None => {
                println!("F({:2}): ⏱️  TIMEOUT! Taking longer than {:?}", n, timeout);
                println!("      Naive recursion is O(2^n) - exponential time.");
                println!("      Each increase by 1 roughly DOUBLES the runtime!");
                break;
            }
        }
    }
    
    println!("\n💡 Key Insight:");
    println!("   - Naive recursion: Unusable for n > 40 (exponential time + stack risk)");
    println!("   - Memoization: Can handle n=10,000+ easily (linear time, stack limited)");
    println!("   - Bottom-up/Optimized: Can handle n=100,000+ (linear time, no recursion)\n");
}

fn compare_approaches() {
    println!("--- Performance Comparison ---\n");
    
    let test_cases = vec![10, 20, 30];
    
    for n in test_cases {
        println!("F({}):", n);
        
        // Naive (only for small n)
        if n <= 30 {
            let start = std::time::Instant::now();
            let result = fibonacci_naive(n);
            let duration = start.elapsed();
            println!("  Naive:      {} (took {:?})", result, duration);
        } else {
            println!("  Naive:      <skipped - too slow>");
        }
        
        // Memoized
        let start = std::time::Instant::now();
        let result = fibonacci_memoized(n);
        let duration = start.elapsed();
        println!("  Memoized:   {} (took {:?})", result, duration);
        
        // Bottom-up
        let start = std::time::Instant::now();
        let result = fibonacci_bottom_up(n);
        let duration = start.elapsed();
        println!("  Bottom-up:  {} (took {:?})", result, duration);
        
        // Optimized
        let start = std::time::Instant::now();
        let result = fibonacci_optimized(n);
        let duration = start.elapsed();
        println!("  Optimized:  {} (took {:?})", result, duration);
        
        println!();
    }
}

fn test_large_numbers() {
    println!("--- Large Numbers (with modular arithmetic) ---\n");
    
    let test_cases = vec![50, 100, 1000, 10000];
    
    for n in test_cases {
        let start = std::time::Instant::now();
        let result = fibonacci_modular(n);
        let duration = start.elapsed();
        println!("F({:5}) mod {} = {:10} (took {:?})", 
                 n, MOD, result, duration);
    }
}

fn verify_correctness() {
    println!("\n--- Correctness Verification ---\n");
    
    let known_values = vec![
        (0, 0),
        (1, 1),
        (2, 1),
        (3, 2),
        (4, 3),
        (5, 5),
        (6, 8),
        (7, 13),
        (8, 21),
        (9, 34),
        (10, 55),
    ];
    
    for (n, expected) in known_values {
        let result = fibonacci_optimized(n);
        let status = if result == expected { "✓" } else { "✗" };
        println!("{} F({:2}) = {:3} (expected {:3})", status, n, result, expected);
    }
}

// ============================================================================
// SOLUTIONS (Don't peek until you've tried!)
// ============================================================================

#[cfg(test)]
mod solutions {
    use super::*;
    
    // Solution 1: Naive Recursion
    #[allow(dead_code)]
    fn fibonacci_naive_solution(n: u64) -> u64 {
        if n <= 1 {
            return n;
        }
        fibonacci_naive_solution(n - 1) + fibonacci_naive_solution(n - 2)
    }
    
    // Solution 2: Memoized
    #[allow(dead_code)]
    fn fibonacci_memoized_solution(n: u64) -> u64 {
        let mut memo = HashMap::new();
        fibonacci_memo_solution_helper(n, &mut memo)
    }
    
    fn fibonacci_memo_solution_helper(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
        if n <= 1 {
            return n;
        }
        
        if let Some(&result) = memo.get(&n) {
            return result;
        }
        
        let result = fibonacci_memo_solution_helper(n - 1, memo) 
                   + fibonacci_memo_solution_helper(n - 2, memo);
        memo.insert(n, result);
        result
    }
    
    // Solution 3: Bottom-Up
    #[allow(dead_code)]
    fn fibonacci_bottom_up_solution(n: u64) -> u64 {
        if n <= 1 {
            return n;
        }
        
        let n_usize = n as usize;
        let mut dp = vec![0u64; n_usize + 1];
        dp[0] = 0;
        dp[1] = 1;
        
        for i in 2..=n_usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        
        dp[n_usize]
    }
    
    // Solution 4: Space-Optimized
    #[allow(dead_code)]
    fn fibonacci_optimized_solution(n: u64) -> u64 {
        if n <= 1 {
            return n;
        }
        
        let mut prev = 0u64;
        let mut curr = 1u64;
        
        for _ in 2..=n {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }
        
        curr
    }
    
    // Solution 5: Modular Arithmetic
    #[allow(dead_code)]
    fn fibonacci_modular_solution(n: u64) -> u64 {
        if n <= 1 {
            return n;
        }
        
        let mut prev = 0u64;
        let mut curr = 1u64;
        
        for _ in 2..=n {
            let next = (prev + curr) % MOD;
            prev = curr;
            curr = next;
        }
        
        curr
    }
    
    #[test]
    fn test_solutions() {
        // Test all solutions produce same results for small n
        for n in 0..15 {
            let naive = fibonacci_naive_solution(n);
            let memoized = fibonacci_memoized_solution(n);
            let bottom_up = fibonacci_bottom_up_solution(n);
            let optimized = fibonacci_optimized_solution(n);
            
            assert_eq!(naive, memoized, "Mismatch at n={}", n);
            assert_eq!(naive, bottom_up, "Mismatch at n={}", n);
            assert_eq!(naive, optimized, "Mismatch at n={}", n);
        }
        
        // Test modular arithmetic doesn't overflow
        let result = fibonacci_modular_solution(10000);
        assert!(result < MOD);
    }
    
    #[test]
    fn test_known_values() {
        assert_eq!(fibonacci_optimized_solution(0), 0);
        assert_eq!(fibonacci_optimized_solution(1), 1);
        assert_eq!(fibonacci_optimized_solution(10), 55);
        assert_eq!(fibonacci_optimized_solution(20), 6765);
    }
}
