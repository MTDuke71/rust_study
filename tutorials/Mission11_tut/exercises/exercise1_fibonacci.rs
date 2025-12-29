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
    
    compare_approaches();
    test_large_numbers();
    verify_correctness();
    
    println!("\n✅ Exercise 1 completed!");
}

// ============================================================================
// TODO: Implement these 4 approaches
// ============================================================================

/// Approach 1: Naive Recursion (SLOW - exponential)
/// 
/// Try this with n=5, n=10, n=20, n=40
/// Notice how it becomes unusably slow very quickly!
#[allow(dead_code, unused_variables)]
fn fibonacci_naive(n: u64) -> u64 {
    // TODO: Implement naive recursion
    // Base case: F(0) = 0, F(1) = 1
    // Recursive case: F(n) = F(n-1) + F(n-2)
    todo!("Implement fibonacci_naive");
}

/// Approach 2: Top-Down with Memoization (FAST - linear)
///
/// This should handle n=100+ easily
fn fibonacci_memoized(n: u64) -> u64 {
    let mut memo = HashMap::new();
    fibonacci_memo_helper(n, &mut memo)
}

#[allow(dead_code, unused_variables)]
fn fibonacci_memo_helper(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    // TODO: Implement memoized recursion
    // 1. Check base cases
    // 2. Check memo cache
    // 3. Compute recursively if not cached
    // 4. Store in memo before returning
    todo!("Implement fibonacci_memo_helper");
}

/// Approach 3: Bottom-Up with Table (FAST - linear, no recursion)
///
/// Build table from F(0) up to F(n)
#[allow(dead_code, unused_variables)]
fn fibonacci_bottom_up(n: u64) -> u64 {
    // TODO: Implement bottom-up DP
    // 1. Create Vec to store all values from 0 to n
    // 2. Initialize F(0) = 0, F(1) = 1
    // 3. Fill table: dp[i] = dp[i-1] + dp[i-2]
    // 4. Return dp[n]
    todo!("Implement fibonacci_bottom_up");
}

/// Approach 4: Space-Optimized O(1) (FASTEST - no extra space)
///
/// Only need last two values, not entire table
#[allow(dead_code, unused_variables)]
fn fibonacci_optimized(n: u64) -> u64 {
    // TODO: Implement space-optimized version
    // Only track prev and curr, update in rolling fashion
    // Hint: Use tuple destructuring (prev, curr) = (curr, prev + curr)
    todo!("Implement fibonacci_optimized");
}

// ============================================================================
// CHALLENGE: Large Numbers with Modular Arithmetic
// ============================================================================

const MOD: u64 = 1_000_000_007;

/// Compute F(n) mod 1_000_000_007 to avoid overflow
#[allow(dead_code, unused_variables)]
fn fibonacci_modular(n: u64) -> u64 {
    // TODO: Modify fibonacci_optimized to use modular arithmetic
    // Use (a + b) % MOD to keep numbers bounded
    todo!("Implement fibonacci_modular");
}

// ============================================================================
// Tests and Demonstrations
// ============================================================================

fn compare_approaches() {
    println!("--- Performance Comparison ---\n");
    
    let test_cases = vec![10, 20, 30];
    
    for n in test_cases {
        println!("F({}):", n);
        
        // Naive (only for small n)
        if n <= 20 {
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
