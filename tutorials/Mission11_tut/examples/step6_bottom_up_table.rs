//! Step 6: Bottom-Up Tabulation - Alternative to Top-Down Memoization
//!
//! **Learning Objective**: By the end of this step, you will understand bottom-up
//! dynamic programming and when to choose it over top-down memoization.
//!
//! **Prerequisites**: Step 5 (boolean/counting), iteration vs recursion
//!
//! **Next Step Preview**: Step 7 will apply all learned patterns to real AoC problems,
//! demonstrating production-ready implementations.

use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("=== Step 6: Bottom-Up Tabulation ===\n");
    
    demonstrate_core_concept();
    show_common_patterns();
    handle_edge_cases();
    validate_understanding();
    
    println!("\n✅ Step 6 completed! Ready for Step 7.");
    println!("💡 Key Insight: Choose approach based on problem structure - no universal best!");
}

/// Core concept demonstration
///
/// Shows the difference between top-down and bottom-up approaches.
/// Key learning: Build from base cases vs recurse from target.
fn demonstrate_core_concept() {
    println!("--- Core Concept: Top-Down vs Bottom-Up ---");
    
    println!("\n1. Top-Down (Memoization):");
    println!("   - Start from the problem (target)");
    println!("   - Recurse to smaller subproblems");
    println!("   - Cache results as you go");
    println!("   - Only compute reachable states");
    
    println!("\n2. Bottom-Up (Tabulation):");
    println!("   - Start from base cases");
    println!("   - Build up to the target");
    println!("   - Fill table iteratively");
    println!("   - Compute all states up to target");
    
    compare_approaches();
}

fn compare_approaches() {
    println!("\n--- Comparing Approaches: Fibonacci ---");
    
    let n = 20;
    
    // Top-down
    let mut memo_td = HashMap::new();
    let start = Instant::now();
    let result_td = fib_top_down(n, &mut memo_td);
    let time_td = start.elapsed();
    
    // Bottom-up
    let start = Instant::now();
    let result_bu = fib_bottom_up(n);
    let time_bu = start.elapsed();
    
    // Bottom-up optimized (O(1) space)
    let start = Instant::now();
    let result_opt = fib_bottom_up_optimized(n);
    let time_opt = start.elapsed();
    
    println!("\n  Computing fib({}):", n);
    println!("  Top-down    (recursive):  {} in {:?}", result_td, time_td);
    println!("  Bottom-up   (iterative):  {} in {:?}", result_bu, time_bu);
    println!("  Bottom-up   (optimized):  {} in {:?}", result_opt, time_opt);
    
    println!("\n  Observations:");
    println!("  - All produce same result");
    println!("  - Bottom-up often faster (no function call overhead)");
    println!("  - Bottom-up can be space-optimized easily");
}

/// Top-down fibonacci with memoization
fn fib_top_down(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 {
        return n;
    }
    
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    
    let result = fib_top_down(n - 1, memo) + fib_top_down(n - 2, memo);
    memo.insert(n, result);
    result
}

/// Bottom-up fibonacci with table
fn fib_bottom_up(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    
    let mut table = vec![0; (n + 1) as usize];
    table[0] = 0;
    table[1] = 1;
    
    for i in 2..=n as usize {
        table[i] = table[i - 1] + table[i - 2];
    }
    
    table[n as usize]
}

/// Bottom-up fibonacci with O(1) space
fn fib_bottom_up_optimized(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    
    let mut prev2 = 0;
    let mut prev1 = 1;
    
    for _ in 2..=n {
        let current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;
    }
    
    prev1
}

/// Common usage patterns
///
/// Shows when to use each approach.
fn show_common_patterns() {
    println!("\n--- Common Patterns ---");
    
    pattern_1_string_problems();
    pattern_2_grid_paths();
    pattern_3_trade_offs();
}

fn pattern_1_string_problems() {
    println!("\nPattern 1: String problems - Top-down is clearer");
    
    let patterns = vec!["r", "wr", "b"];
    let design = "wrr";
    
    println!("  Problem: Can we make '{}' from patterns?", design);
    
    // Top-down: natural recursive decomposition
    let mut memo = HashMap::new();
    let result_td = can_make_top_down(&patterns, design, &mut memo);
    println!("  Top-down: {} (natural - matches problem structure)", result_td);
    
    // Bottom-up: requires explicit state ordering
    let result_bu = can_make_bottom_up(&patterns, design);
    println!("  Bottom-up: {} (complex - need substring ordering)", result_bu);
    
    println!("\n  Verdict: Top-down preferred for string DP");
    println!("  Why? Recursive structure matches string decomposition");
}

fn can_make_top_down<'a>(
    patterns: &[&str],
    design: &'a str,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if design.is_empty() {
        return true;
    }
    
    if let Some(&result) = memo.get(design) {
        return result;
    }
    
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            if can_make_top_down(patterns, rest, memo) {
                memo.insert(design, true);
                return true;
            }
        }
    }
    
    memo.insert(design, false);
    false
}

fn can_make_bottom_up(patterns: &[&str], design: &str) -> bool {
    let n = design.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;  // Empty string is reachable
    
    // Build from position 0 to n
    for i in 0..=n {
        if !dp[i] {
            continue;  // Can't reach position i
        }
        
        // Try extending from position i
        for pattern in patterns {
            if design[i..].starts_with(pattern) {
                dp[i + pattern.len()] = true;
            }
        }
    }
    
    dp[n]
}

fn pattern_2_grid_paths() {
    println!("\nPattern 2: Grid paths - Bottom-up is natural");
    
    let rows = 3;
    let cols = 3;
    
    println!("  Problem: Count paths from (0,0) to ({},{}) (only right/down)", rows-1, cols-1);
    
    // Bottom-up: natural iteration over grid
    let result_bu = count_paths_bottom_up(rows, cols);
    println!("  Bottom-up: {} paths (natural - fill grid left-to-right, top-to-bottom)", result_bu);
    
    // Top-down: works but less intuitive
    let mut memo = HashMap::new();
    let result_td = count_paths_top_down(rows - 1, cols - 1, &mut memo);
    println!("  Top-down: {} paths (works but more complex)", result_td);
    
    println!("\n  Verdict: Bottom-up preferred for grid DP");
    println!("  Why? Natural left-to-right, top-to-bottom iteration");
}

#[allow(clippy::needless_range_loop)]
fn count_paths_bottom_up(rows: usize, cols: usize) -> u64 {
    let mut dp = vec![vec![0u64; cols]; rows];
    
    // Initialize first row and column (only one way to reach)
    for i in 0..rows {
        dp[i][0] = 1;
    }
    for j in 0..cols {
        dp[0][j] = 1;
    }
    
    // Fill table: paths to (i,j) = paths to (i-1,j) + paths to (i,j-1)
    for i in 1..rows {
        for j in 1..cols {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }
    
    dp[rows - 1][cols - 1]
}

fn count_paths_top_down(row: usize, col: usize, memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    if row == 0 || col == 0 {
        return 1;
    }
    
    if let Some(&result) = memo.get(&(row, col)) {
        return result;
    }
    
    let result = count_paths_top_down(row - 1, col, memo) 
               + count_paths_top_down(row, col - 1, memo);
    memo.insert((row, col), result);
    result
}

fn pattern_3_trade_offs() {
    println!("\nPattern 3: Performance trade-offs");
    
    println!("\n  Aspect          | Top-Down          | Bottom-Up");
    println!("  --------------- | ----------------- | -----------------");
    println!("  Approach        | Recursive         | Iterative");
    println!("  Memory          | Reachable states  | All states ≤ n");
    println!("  Call overhead   | Yes (recursion)   | No (loops)");
    println!("  Code clarity    | Matches problem   | Needs ordering");
    println!("  Space optimize  | Harder            | Often easy");
    println!("  Debugging       | Call stack helps  | Step through");
    
    println!("\n  Choose top-down when:");
    println!("  ✓ Problem naturally recursive (string decomposition)");
    println!("  ✓ Not all states are reachable");
    println!("  ✓ State space is sparse");
    
    println!("\n  Choose bottom-up when:");
    println!("  ✓ Clear iteration order exists (arrays, grids)");
    println!("  ✓ All states needed anyway");
    println!("  ✓ Space optimization is important");
}

/// Edge case handling
///
/// Demonstrates edge cases in both approaches.
fn handle_edge_cases() {
    println!("\n--- Edge Cases ---");
    
    base_case_handling();
    space_optimization();
    stack_overflow_risk();
}

fn base_case_handling() {
    println!("\nEdge Case 1: Base case initialization");
    
    println!("\n  Top-down:");
    println!("    if n == 0: return 0  # Explicit check");
    println!("    if n == 1: return 1");
    
    println!("\n  Bottom-up:");
    println!("    dp[0] = 0  # Initialize table");
    println!("    dp[1] = 1");
    println!("    # Then iterate from 2 to n");
    
    println!("\n  Both need same base cases, different expression");
}

fn space_optimization() {
    println!("\nEdge Case 2: Space optimization");
    
    let n = 30;
    
    // Bottom-up full table
    let result1 = fib_bottom_up(n);
    println!("  Bottom-up (full table): O(n) space, result={}", result1);
    
    // Bottom-up optimized
    let result2 = fib_bottom_up_optimized(n);
    println!("  Bottom-up (optimized):  O(1) space, result={}", result2);
    
    println!("\n  Observation: Bottom-up easily optimized to O(1)");
    println!("  Top-down harder - would need custom eviction strategy");
}

fn stack_overflow_risk() {
    println!("\nEdge Case 3: Stack overflow with deep recursion");
    
    // Demonstrate with safe small example first
    println!("\n  Stack depth demonstration (safe):");
    let safe_n = 10;
    let depth = measure_recursion_depth(safe_n);
    println!("    fib({}) requires {} recursive calls", safe_n, depth);
    
    // Theoretical calculation
    println!("\n  Theoretical stack analysis:");
    println!("    - Each recursive call uses stack frame (~100-200 bytes)");
    println!("    - Rust default stack: ~2MB (2,097,152 bytes)");
    println!("    - Max recursion depth: ~10,000-20,000 calls");
    println!("    - fib(n) recursion depth ≈ n (memoized) or 2^n (unmemoized!)");
    
    println!("\n  Top-down memoization (safe range):");
    let mut memo = HashMap::new();
    let medium_n = 50;  // Safe for both computation and u64 range
    let result_td = fib_top_down(medium_n, &mut memo);
    println!("    fib({}) = {} ✓ (with memoization)", medium_n, result_td);
    println!("    Stack depth: ~{} calls (each subproblem computed once)", medium_n);
    println!("    Memo entries: {}", memo.len());
    
    println!("\n  What would happen with very large n (e.g., fib(100000)):");
    println!("    ❌ Stack depth: ~100,000 recursive calls");
    println!("    ❌ Stack usage: ~10-20 MB (exceeds default 2MB limit)");
    println!("    ❌ Result: thread 'main' has overflowed its stack");
    println!("    (Not actually running to avoid crash)");
    
    println!("\n  Bottom-up iterative (can handle larger n):");
    let large_n = 90;  // Still within u64 range, but safe to compute
    let result_bu = fib_bottom_up_optimized(large_n);
    println!("    fib({}) = {} ✓", large_n, result_bu);
    println!("    Stack depth: 1 (constant - just the function call)");
    println!("    Memory: O(1) - only two variables (prev1, prev2)");
    
    println!("\n  Real-world example - Counting paths:");
    println!("    Top-down with depth 10,000 → Risk of stack overflow");
    println!("    Bottom-up with depth 10,000 → No problem, just iteration");
    
    println!("\n  Key insight:");
    println!("    Top-down → Stack depth = O(n) → Can overflow with deep recursion");
    println!("    Bottom-up → Stack depth = O(1) → No overflow risk, any depth OK");
}

/// Counts recursive calls to demonstrate stack depth
fn measure_recursion_depth(n: u64) -> u64 {
    fn fib_count(n: u64, memo: &mut HashMap<u64, u64>, count: &mut u64) -> u64 {
        *count += 1;  // Track each call
        
        if n <= 1 {
            return n;
        }
        
        if let Some(&result) = memo.get(&n) {
            return result;
        }
        
        let result = fib_count(n - 1, memo, count) + fib_count(n - 2, memo, count);
        memo.insert(n, result);
        result
    }
    
    let mut memo = HashMap::new();
    let mut count = 0;
    fib_count(n, &mut memo, &mut count);
    count
}

/// Understanding validation
///
/// Exercises to validate understanding of both approaches.
fn validate_understanding() {
    println!("\n--- Validation Exercises ---");
    
    println!("\n🤔 Approach selection quiz:");
    
    // Exercise 1
    println!("\nExercise 1: Which approach for this problem?");
    println!("  Problem: Longest common subsequence of two strings");
    println!("  a) Top-down (recursive with memo)");
    println!("  b) Bottom-up (2D table)");
    println!("  Answer: Either works! Slightly favor bottom-up for 2D tables");
    
    // Exercise 2
    println!("\nExercise 2: Which approach for this problem?");
    println!("  Problem: Partition string into valid words");
    println!("  a) Top-down");
    println!("  b) Bottom-up");
    println!("  Answer: Top-down - natural string decomposition");
    
    // Exercise 3
    println!("\nExercise 3: Space optimization");
    println!("  Problem: Fibonacci with O(1) space");
    println!("  Which approach is easier?");
    println!("  Answer: Bottom-up - just keep last two values");
    
    println!("\n--- Practical Comparison ---");
    side_by_side_comparison();
}

fn side_by_side_comparison() {
    println!("\nCoin change problem: Minimum coins to make amount");
    println!("  Coins: [1, 5, 10, 25]");
    println!("  Amount: 41");
    
    let coins = vec![1, 5, 10, 25];
    let amount = 41;
    
    // Top-down
    let mut memo_td = HashMap::new();
    let start = Instant::now();
    let result_td = min_coins_top_down(&coins, amount, &mut memo_td);
    let time_td = start.elapsed();
    
    // Bottom-up
    let start = Instant::now();
    let result_bu = min_coins_bottom_up(&coins, amount);
    let time_bu = start.elapsed();
    
    println!("\n  Top-down:  {} coins in {:?}", result_td, time_td);
    println!("  Bottom-up: {} coins in {:?}", result_bu, time_bu);
    println!("\n  Both give same answer, choose based on clarity!");
}

fn min_coins_top_down(coins: &[usize], amount: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if amount == 0 {
        return 0;
    }
    
    if let Some(&result) = memo.get(&amount) {
        return result;
    }
    
    let mut min = usize::MAX;
    for &coin in coins {
        if coin <= amount {
            let sub_result = min_coins_top_down(coins, amount - coin, memo);
            if sub_result != usize::MAX {
                min = min.min(1 + sub_result);
            }
        }
    }
    
    memo.insert(amount, min);
    min
}

fn min_coins_bottom_up(coins: &[usize], amount: usize) -> usize {
    let mut dp = vec![usize::MAX; amount + 1];
    dp[0] = 0;
    
    for i in 1..=amount {
        for &coin in coins {
            if coin <= i && dp[i - coin] != usize::MAX {
                dp[i] = dp[i].min(dp[i - coin] + 1);
            }
        }
    }
    
    dp[amount]
}

#[cfg(test)]
mod step_tests {
    use super::*;
    
    #[test]
    fn test_fibonacci_equivalence() {
        for n in [0, 1, 5, 10, 20] {
            let mut memo = HashMap::new();
            let result_td = fib_top_down(n, &mut memo);
            let result_bu = fib_bottom_up(n);
            let result_opt = fib_bottom_up_optimized(n);
            
            assert_eq!(result_td, result_bu);
            assert_eq!(result_bu, result_opt);
        }
    }
    
    #[test]
    fn test_string_pattern_equivalence() {
        let patterns = vec!["r", "wr", "b"];
        let designs = vec!["r", "wr", "wrr", "xyz"];
        
        for design in designs {
            let mut memo = HashMap::new();
            let result_td = can_make_top_down(&patterns, design, &mut memo);
            let result_bu = can_make_bottom_up(&patterns, design);
            
            assert_eq!(result_td, result_bu);
        }
    }
    
    #[test]
    fn test_grid_paths_equivalence() {
        for size in [2, 3, 5] {
            let mut memo = HashMap::new();
            let result_td = count_paths_top_down(size - 1, size - 1, &mut memo);
            let result_bu = count_paths_bottom_up(size, size);
            
            assert_eq!(result_td, result_bu);
        }
    }
    
    #[test]
    fn test_coin_change_equivalence() {
        let coins = vec![1, 5, 10, 25];
        for amount in [0, 1, 5, 11, 41] {
            let mut memo = HashMap::new();
            let result_td = min_coins_top_down(&coins, amount, &mut memo);
            let result_bu = min_coins_bottom_up(&coins, amount);
            
            assert_eq!(result_td, result_bu);
        }
    }
}
