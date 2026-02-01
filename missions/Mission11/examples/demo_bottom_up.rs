//! Bottom-Up vs Top-Down DP Comparison
//!
//! **REQ-7: Bottom-Up DP Alternative (Comparison)**
//!
//! Demonstrates both DP approaches on the same problems to illustrate trade-offs:
//! - **Top-down (Memoization)**: Recursive with cache, only computes reachable states
//! - **Bottom-up (Tabulation)**: Iterative with pre-allocated table, computes all states
//!
//! # Examples Covered
//! 1. Fibonacci numbers
//! 2. Coin change (minimum coins)
//! 3. Longest common subsequence (LCS)

use std::collections::HashMap;
use std::time::Instant;

// ============================================================================
// EXAMPLE 1: Fibonacci - Simplest DP problem
// ============================================================================

/// Top-down Fibonacci with memoization (from demo_fibonacci.rs)
fn fib_top_down(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 {
        return n;
    }
    if let Some(&cached) = memo.get(&n) {
        return cached;
    }
    let result = fib_top_down(n - 1, memo) + fib_top_down(n - 2, memo);
    memo.insert(n, result);
    result
}

/// Bottom-up Fibonacci with tabulation
///
/// **Trade-offs**:
/// - ✅ No recursion overhead (iterative)
/// - ✅ Predictable memory usage (array size = n+1)
/// - ✅ Better cache locality (sequential access)
/// - ❌ Computes all states from 0 to n (even if some unused)
fn fib_bottom_up(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    // Pre-allocate table for all states [0..n]
    let mut dp = vec![0u64; (n + 1) as usize];

    // Base cases
    dp[0] = 0;
    dp[1] = 1;

    // Fill table in dependency order (bottom-up)
    for i in 2..=n as usize {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[n as usize]
}

/// Space-optimized bottom-up Fibonacci
///
/// **Optimization**: Since fib(n) only depends on fib(n-1) and fib(n-2),
/// we can use O(1) space instead of O(n).
fn fib_bottom_up_optimized(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let mut prev2 = 0u64;
    let mut prev1 = 1u64;

    for _ in 2..=n {
        let current = prev1 + prev2;
        prev2 = prev1;
        prev1 = current;
    }

    prev1
}

// ============================================================================
// EXAMPLE 2: Coin Change - Classic DP optimization problem
// ============================================================================

/// Top-down coin change with memoization
///
/// **Problem**: Find minimum coins needed to make amount using given coin denominations.
/// Returns None if amount cannot be made.
///
/// **State**: Remaining amount to make
/// **Recurrence**: min_coins(amount) = 1 + min(min_coins(amount - coin) for coin in coins)
fn coin_change_top_down(
    amount: u32,
    coins: &[u32],
    memo: &mut HashMap<u32, Option<u32>>,
) -> Option<u32> {
    // Base case: no amount needed
    if amount == 0 {
        return Some(0);
    }

    // Memoization check
    if let Some(&cached) = memo.get(&amount) {
        return cached;
    }

    // Try each coin and find minimum
    let mut min_coins: Option<u32> = None;
    for &coin in coins {
        if coin <= amount {
            if let Some(sub_result) = coin_change_top_down(amount - coin, coins, memo) {
                let current = 1 + sub_result;
                min_coins = Some(min_coins.map_or(current, |m| m.min(current)));
            }
        }
    }

    // Cache and return
    memo.insert(amount, min_coins);
    min_coins
}

/// Bottom-up coin change with tabulation
///
/// **Trade-offs**:
/// - ✅ No recursion depth limits
/// - ✅ Predictable iteration pattern
/// - ❌ Must compute all amounts [0..amount] even if some unreachable
fn coin_change_bottom_up(amount: u32, coins: &[u32]) -> Option<u32> {
    // Pre-allocate table: dp[i] = min coins to make amount i
    let mut dp = vec![None; (amount + 1) as usize];

    // Base case: 0 amount requires 0 coins
    dp[0] = Some(0);

    // Fill table in dependency order
    for current_amount in 1..=amount as usize {
        let mut min_coins: Option<u32> = None;

        // Try each coin
        for &coin in coins {
            let coin = coin as usize;
            if coin <= current_amount {
                if let Some(sub_result) = dp[current_amount - coin] {
                    let current = 1 + sub_result;
                    min_coins = Some(min_coins.map_or(current, |m| m.min(current)));
                }
            }
        }

        dp[current_amount] = min_coins;
    }

    dp[amount as usize]
}

// ============================================================================
// EXAMPLE 3: Longest Common Subsequence (LCS) - 2D DP
// ============================================================================

/// Top-down LCS with memoization
///
/// **Problem**: Find length of longest common subsequence of two strings.
///
/// **State**: (i, j) = positions in both strings
/// **Recurrence**:
/// - If chars match: 1 + lcs(i+1, j+1)
/// - Else: max(lcs(i+1, j), lcs(i, j+1))
fn lcs_top_down(
    s1: &[u8],
    s2: &[u8],
    i: usize,
    j: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // Base case: reached end of either string
    if i >= s1.len() || j >= s2.len() {
        return 0;
    }

    // Memoization check
    if let Some(&cached) = memo.get(&(i, j)) {
        return cached;
    }

    // Recursive computation
    let result = if s1[i] == s2[j] {
        // Characters match - include in LCS
        1 + lcs_top_down(s1, s2, i + 1, j + 1, memo)
    } else {
        // Characters don't match - try skipping each
        let skip_s1 = lcs_top_down(s1, s2, i + 1, j, memo);
        let skip_s2 = lcs_top_down(s1, s2, i, j + 1, memo);
        skip_s1.max(skip_s2)
    };

    // Cache and return
    memo.insert((i, j), result);
    result
}

/// Bottom-up LCS with 2D tabulation
///
/// **Trade-offs**:
/// - ✅ No recursion (handles long strings without stack overflow)
/// - ✅ Can reconstruct actual LCS string (not just length)
/// - ✅ Cache-friendly sequential memory access
/// - ❌ Always uses O(m×n) space (top-down may use less if sparse)
fn lcs_bottom_up(s1: &[u8], s2: &[u8]) -> usize {
    let m = s1.len();
    let n = s2.len();

    // Pre-allocate 2D table: dp[i][j] = LCS length of s1[i..] and s2[j..]
    // We build from end to start, so index 0 is the answer
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Base cases: dp[m][*] = 0 and dp[*][n] = 0 (already initialized)

    // Fill table in reverse order (bottom-up from end of strings)
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            dp[i][j] = if s1[i] == s2[j] {
                1 + dp[i + 1][j + 1]
            } else {
                dp[i + 1][j].max(dp[i][j + 1])
            };
        }
    }

    dp[0][0]
}

// ============================================================================
// MAIN - Performance and Trade-off Comparison
// ============================================================================

fn main() {
    println!("=== Bottom-Up vs Top-Down DP Comparison ===");
    println!("REQ-7 Validation\n");

    // ========================================================================
    // Example 1: Fibonacci
    // ========================================================================
    println!("━━━ Example 1: Fibonacci ━━━");
    let n = 40u64;

    let mut memo = HashMap::new();
    let start = Instant::now();
    let result_td = fib_top_down(n, &mut memo);
    let time_td = start.elapsed();

    let start = Instant::now();
    let result_bu = fib_bottom_up(n);
    let time_bu = start.elapsed();

    let start = Instant::now();
    let result_opt = fib_bottom_up_optimized(n);
    let time_opt = start.elapsed();

    println!("fib({}) = {}", n, result_td);
    println!(
        "  Top-down (memoization):     {:?} | Cache entries: {}",
        time_td,
        memo.len()
    );
    println!(
        "  Bottom-up (tabulation):     {:?} | Table size: {}",
        time_bu,
        n + 1
    );
    println!(
        "  Bottom-up (space optimized): {:?} | Space: O(1)",
        time_opt
    );
    assert_eq!(result_td, result_bu);
    assert_eq!(result_td, result_opt);

    println!("\n📊 Trade-offs:");
    println!("  • Top-down: Only computes reachable states (cache size = n)");
    println!("  • Bottom-up: Computes all states 0..n (table size = n+1)");
    println!("  • Optimized: Uses constant space (only need last 2 values)");

    // ========================================================================
    // Example 2: Coin Change
    // ========================================================================
    println!("\n━━━ Example 2: Coin Change ━━━");
    let amount = 63;
    let coins = vec![1, 5, 10, 21, 25];

    let mut memo = HashMap::new();
    let start = Instant::now();
    let result_td = coin_change_top_down(amount, &coins, &mut memo);
    let time_td = start.elapsed();

    let start = Instant::now();
    let result_bu = coin_change_bottom_up(amount, &coins);
    let time_bu = start.elapsed();

    println!("Minimum coins for amount {} with coins {:?}", amount, coins);
    println!(
        "  Top-down:  {:?} coins ({:?}) | Cache entries: {}",
        result_td.unwrap(),
        time_td,
        memo.len()
    );
    println!(
        "  Bottom-up: {:?} coins ({:?}) | Table size: {}",
        result_bu.unwrap(),
        time_bu,
        amount + 1
    );
    assert_eq!(result_td, result_bu);

    println!("\n📊 Trade-offs:");
    println!("  • Top-down: May skip unreachable amounts (cache size <= amount)");
    println!("  • Bottom-up: Always computes all amounts 0..amount");
    println!("  • Both have same time complexity: O(amount × coins)");

    // ========================================================================
    // Example 3: Longest Common Subsequence
    // ========================================================================
    println!("\n━━━ Example 3: Longest Common Subsequence ━━━");
    let s1 = b"ABCDGH";
    let s2 = b"AEDFHR";

    let mut memo = HashMap::new();
    let start = Instant::now();
    let result_td = lcs_top_down(s1, s2, 0, 0, &mut memo);
    let time_td = start.elapsed();

    let start = Instant::now();
    let result_bu = lcs_bottom_up(s1, s2);
    let time_bu = start.elapsed();

    println!(
        "LCS of {:?} and {:?}",
        String::from_utf8_lossy(s1),
        String::from_utf8_lossy(s2)
    );
    println!(
        "  Top-down:  {} ({:?}) | Cache entries: {}",
        result_td,
        time_td,
        memo.len()
    );
    println!(
        "  Bottom-up: {} ({:?}) | Table size: {} ({}×{})",
        result_bu,
        time_bu,
        (s1.len() + 1) * (s2.len() + 1),
        s1.len() + 1,
        s2.len() + 1
    );
    assert_eq!(result_td, result_bu);

    println!("\n📊 Trade-offs:");
    println!("  • Top-down: Only fills reachable (i,j) states (may be sparse)");
    println!("  • Bottom-up: Fills entire m×n table (dense, predictable)");
    println!("  • Bottom-up better for: Reconstructing solution, avoiding recursion");
    println!("  • Top-down better for: Sparse state spaces, easier to reason about");

    // ========================================================================
    // Summary
    // ========================================================================
    println!("\n━━━ Summary: When to Use Each Approach ━━━");
    println!("\n🔄 Top-Down (Memoization):");
    println!("  ✅ Natural translation from recursive problem definition");
    println!("  ✅ Only computes reachable states (good for sparse problems)");
    println!("  ✅ Easier to understand and prove correctness");
    println!("  ❌ Recursion overhead (stack depth limits)");
    println!("  ❌ HashMap overhead for cache (vs array indexing)");

    println!("\n📊 Bottom-Up (Tabulation):");
    println!("  ✅ No recursion (handles deep problems, predictable performance)");
    println!("  ✅ Better cache locality (sequential array access)");
    println!("  ✅ Can reconstruct solution path easily");
    println!("  ✅ Easier to optimize space (rolling arrays, etc.)");
    println!("  ❌ Must determine correct iteration order (not always obvious)");
    println!("  ❌ May compute unreachable states");

    println!("\n💡 Best Practices:");
    println!("  • Start with top-down if problem naturally recursive");
    println!("  • Convert to bottom-up if recursion depth is an issue");
    println!("  • Use bottom-up for production (predictable, optimizable)");
    println!("  • Both have same time/space complexity for most problems");
}
