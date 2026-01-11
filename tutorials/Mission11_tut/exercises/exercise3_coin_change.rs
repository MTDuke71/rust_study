//! Exercise 3: Coin Change Problem
//!
//! **Learning Goals**:
//! - Practice unbounded knapsack pattern
//! - Understand optimization vs counting variants
//! - Compare top-down vs bottom-up for different variants
//!
//! **Problem**: Given coin denominations and a target amount:
//! 1. **Minimum Coins**: What's the minimum number of coins needed?
//! 2. **Count Ways**: How many different ways can we make the amount?
//!
//! **Example**:
//! - Coins: [1, 5, 10, 25], Target: 41
//! - Minimum: 4 coins (25 + 10 + 5 + 1)
//! - Ways: Many (25+10+5+1, 25+10+1+1+1+1+1+1, etc.)
//!
//! **Recurrence (Minimum)**:
//! ```text
//! min_coins(amount) = {
//!     0                                    if amount == 0
//!     ∞                                    if amount < 0
//!     1 + min(min_coins(amount - coin))    for each coin
//! }
//! ```
//!
//! **Recurrence (Counting)**:
//! ```text
//! count_ways(amount) = {
//!     1                                    if amount == 0
//!     0                                    if amount < 0
//!     sum(count_ways(amount - coin))       for each coin
//! }
//! ```
//!
//! **Complexity Analysis** (m = coins, n = amount):
//! ```text
//! ┌─────────────┬────────────────────────┬────────────────────────┐
//! │  Approach   │   Minimum Coins        │    Count Ways          │
//! ├─────────────┼────────────────────────┼────────────────────────┤
//! │ Naive       │ Time:  O(m^n)          │ Time:  O(m^n)          │
//! │ Recursion   │ Space: O(n) stack      │ Space: O(n) stack      │
//! │             │ (exponential explosion)│ (exponential explosion)│
//! ├─────────────┼────────────────────────┼────────────────────────┤
//! │ Memoized    │ Time:  O(m × n)        │ Time:  O(m × n)        │
//! │ (Top-Down)  │ Space: O(n) memo+stack │ Space: O(n) memo+stack │
//! │             │ HashMap overhead       │ HashMap overhead       │
//! ├─────────────┼────────────────────────┼────────────────────────┤
//! │ Bottom-Up   │ Time:  O(m × n)        │ Time:  O(m × n)        │
//! │ (DP Array)  │ Space: O(n) array only │ Space: O(n) array only │
//! │             │ ✅ FASTEST in practice │ ✅ FASTEST in practice │
//! └─────────────┴────────────────────────┴────────────────────────┘
//! ```
//!
//! **Why Bottom-Up is Fastest Despite Same O(m×n)**:
//! - No function call overhead (iterative vs recursive)
//! - Direct array access (O(1)) vs HashMap lookup (O(1) average)
//! - Better CPU cache locality (sequential memory access)
//! - No recursion stack management

use std::collections::HashMap;

fn main() {
    println!("=== Exercise 3: Coin Change Problem ===\n");

    demonstrate_minimum_coins();
    demonstrate_count_ways();
    demonstrate_bounded_coins();
    compare_approaches();
    test_edge_cases();

    println!("\n✅ Exercise 3 completed!");
}

// ============================================================================
// Part 1: MINIMUM COINS NEEDED (Optimization Problem)
// ============================================================================

/// Naive recursion for minimum coins (SLOW)

fn min_coins_naive(coins: &[u32], amount: u32) -> Option<u32> {
    // TODO: Implement naive recursion
    // 1. Base case: amount == 0 → return Some(0)
    // 2. Base case: amount < 0 → return None (impossible)
    // 3. Try each coin, recursively solve for (amount - coin)
    // 4. Return minimum + 1, or None if all paths impossible
    if amount == 0 {
        return Some(0);
    }

    let mut min = None;

    for &coin in coins {
        if coin <= amount {
            if let Some(sub_result) = min_coins_naive(coins, amount - coin) {
                min = Some(min.map_or(sub_result + 1, |m: u32| m.min(sub_result + 1)));
            }
        }
    }

    min
}

/// Top-down with memoization for minimum coins
fn min_coins_memoized(coins: &[u32], amount: u32) -> Option<u32> {
    let mut memo = HashMap::new();
    min_coins_memo_helper(coins, amount, &mut memo)
}

fn min_coins_memo_helper(
    coins: &[u32],
    amount: u32,
    memo: &mut HashMap<u32, Option<u32>>,
) -> Option<u32> {
    // TODO: Implement memoized version
    // 1. Check base cases
    // 2. Check memo cache
    // 3. Try each coin recursively
    // 4. Store minimum in memo
    if amount == 0 {
        return Some(0);
    }

    if let Some(&result) = memo.get(&amount) {
        return result;
    }

    let mut min = None;

    for &coin in coins {
        if coin <= amount {
            if let Some(sub_result) = min_coins_memo_helper(coins, amount - coin, memo) {
                min = Some(min.map_or(sub_result + 1, |m: u32| m.min(sub_result + 1)));
            }
        }
    }

    memo.insert(amount, min);
    min
}

/// Bottom-up DP for minimum coins
fn min_coins_bottom_up(coins: &[u32], amount: u32) -> Option<u32> {
    // TODO: Implement bottom-up DP
    // 1. Create array dp[0..=amount]
    // 2. Initialize dp[0] = 0, others = infinity
    // 3. For each amount a from 1 to target:
    //    For each coin c:
    //      if c <= a: dp[a] = min(dp[a], 1 + dp[a - c])
    // 4. Return dp[amount] if not infinity
    let amount_usize = amount as usize;
    let mut dp = vec![None; amount_usize + 1];
    dp[0] = Some(0);

    for a in 1..=amount_usize {
        for &coin in coins {
            if coin as usize <= a {
                if let Some(prev) = dp[a - coin as usize] {
                    dp[a] = Some(dp[a].map_or(prev + 1, |curr| curr.min(prev + 1)));
                }
            }
        }
    }

    dp[amount_usize]
}

// ============================================================================
// Part 2: COUNT WAYS (Counting Problem)
// ============================================================================

/// Naive recursion for counting ways (SLOW)
fn count_ways_naive(coins: &[u32], amount: u32) -> u64 {
    count_ways_naive_helper(coins, amount, 0)
}

fn count_ways_naive_helper(coins: &[u32], amount: u32, coin_idx: usize) -> u64 {
    // TODO: Implement naive counting
    // Note: Need coin_idx to avoid counting duplicates (1+5 vs 5+1)
    // 1. Base case: amount == 0 → return 1 (found a way)
    // 2. Base case: amount < 0 or no more coins → return 0
    // 3. Include current coin: count_ways(amount - coin, coin_idx)
    // 4. Exclude current coin: count_ways(amount, coin_idx + 1)
    // 5. Return sum of both
    if amount == 0 {
        return 1;
    }

    if coin_idx >= coins.len() {
        return 0;
    }

    let coin = coins[coin_idx];

    // Include current coin (if possible) - stay at same coin_idx for unbounded
    let include = if coin <= amount {
        count_ways_naive_helper(coins, amount - coin, coin_idx)
    } else {
        0
    };

    // Exclude current coin - move to next coin
    let exclude = count_ways_naive_helper(coins, amount, coin_idx + 1);

    include + exclude
}

/// Top-down with memoization for counting ways
fn count_ways_memoized(coins: &[u32], amount: u32) -> u64 {
    let mut memo = HashMap::new();
    count_ways_memo_helper(coins, amount, 0, &mut memo)
}

fn count_ways_memo_helper(
    coins: &[u32],
    amount: u32,
    coin_idx: usize,
    memo: &mut HashMap<(u32, usize), u64>,
) -> u64 {
    // TODO: Implement memoized counting
    // Use (amount, coin_idx) as cache key
    if amount == 0 {
        return 1;
    }

    if coin_idx >= coins.len() {
        return 0;
    }

    let key = (amount, coin_idx);
    if let Some(&result) = memo.get(&key) {
        return result;
    }

    let coin = coins[coin_idx];

    // Include current coin (if possible)
    let include = if coin <= amount {
        count_ways_memo_helper(coins, amount - coin, coin_idx, memo)
    } else {
        0
    };

    // Exclude current coin
    let exclude = count_ways_memo_helper(coins, amount, coin_idx + 1, memo);

    let result = include + exclude;
    memo.insert(key, result);
    result
}

/// Bottom-up DP for counting ways
fn count_ways_bottom_up(coins: &[u32], amount: u32) -> u64 {
    // TODO: Implement bottom-up counting
    // 1. Create array dp[0..=amount], initialize dp[0] = 1
    // 2. For each coin c:
    //    For each amount a from c to target:
    //      dp[a] += dp[a - c]
    // 3. Return dp[amount]
    //
    // Note: Order matters to avoid duplicates!
    // Process coins in outer loop, amounts in inner loop
    let amount_usize = amount as usize;
    let mut dp = vec![0u64; amount_usize + 1];
    dp[0] = 1;

    // Process each coin type
    for &coin in coins {
        let coin_usize = coin as usize;
        // Update all amounts that can use this coin
        for a in coin_usize..=amount_usize {
            dp[a] += dp[a - coin_usize];
        }
    }

    dp[amount_usize]
}

// ============================================================================
// BONUS: Bounded Coin Change
// ============================================================================

/// Minimum coins with limited quantities
/// coins_with_count: [(denomination, quantity)]
fn min_coins_bounded(coins_with_count: &[(u32, u32)], amount: u32) -> Option<u32> {
    if amount == 0 {
        return Some(0);
    }

    let amount_usize = amount as usize;
    let inf = u32::MAX / 4;
    let mut dp = vec![inf; amount_usize + 1];
    dp[0] = 0;

    for &(coin, quantity) in coins_with_count {
        if coin == 0 || quantity == 0 {
            continue;
        }

        let mut remaining = quantity;
        let mut group = 1u32;
        while remaining > 0 {
            let take = remaining.min(group);
            remaining -= take;
            group = group.saturating_mul(2);

            let weight_u64 = coin as u64 * take as u64;
            if weight_u64 > amount as u64 {
                continue;
            }
            let weight = weight_u64 as usize;

            for a in (weight..=amount_usize).rev() {
                let prev = dp[a - weight];
                if prev == inf {
                    continue;
                }
                let candidate = prev + take;
                if candidate < dp[a] {
                    dp[a] = candidate;
                }
            }
        }
    }

    (dp[amount_usize] != inf).then_some(dp[amount_usize])
}

// ============================================================================
// Tests and Demonstrations
// ============================================================================

fn demonstrate_minimum_coins() {
    println!("--- Part 1: Minimum Coins Needed ---\n");

    let test_cases = vec![
        (vec![1, 5, 10, 25], 41, "US coins"),
        // Greedy fails: Greedy takes largest coin first (4+1+1 = 3 coins)
        // But optimal DP finds 3+3 = 2 coins
        // This proves greedy doesn't always work for arbitrary coin sets
        (vec![1, 3, 4], 6, "Greedy fails"),
        (vec![2, 5], 3, "Impossible"),
        (vec![1, 2, 5], 11, "Standard example"),
        (vec![1, 5, 10, 25], 10, "Exactly one coin"),
    ];

    for (coins, amount, description) in test_cases {
        let result = min_coins_bottom_up(&coins, amount);
        match result {
            Some(count) => println!(
                "{:20} | Amount: {:2} | Min coins: {}",
                description, amount, count
            ),
            None => println!("{:20} | Amount: {:2} | Impossible", description, amount),
        }
    }
}

fn demonstrate_count_ways() {
    println!("\n--- Part 2: Count Ways to Make Amount ---\n");

    let test_cases = vec![
        (vec![1, 2, 5], 5, "Standard coins"),
        (vec![2, 3, 5], 7, "No pennies"),
        (vec![1, 5, 10, 25], 10, "Multiple combinations"),
        (vec![1, 5, 10, 25], 100, "One dollar"),
    ];

    for (coins, amount, description) in test_cases {
        let count = count_ways_bottom_up(&coins, amount);
        println!(
            "{:20} | Amount: {:3} | Ways: {}",
            description, amount, count
        );
    }
}

fn demonstrate_bounded_coins() {
    println!("\n--- BONUS: Bounded Coin Change ---\n");
    println!("(Each coin type has limited quantity)\n");

    let test_cases = vec![
        (
            vec![(1, 3), (5, 2), (10, 1)],
            17,
            "Limited pennies/nickels/dimes",
        ),
        (vec![(1, 2), (5, 1), (10, 1)], 11, "Need exact coins"),
        (vec![(2, 2), (5, 1)], 3, "Impossible with limits"),
        (
            vec![(1, 5), (5, 3), (10, 2), (25, 1)],
            41,
            "US coins with limits",
        ),
        (vec![(3, 2), (4, 3), (5, 1)], 15, "Suboptimal (only 1×5)"),
        (
            vec![(3, 2), (4, 3), (5, 3)],
            15,
            "Optimal with enough nickels",
        ),
    ];

    for (coins_with_count, amount, description) in test_cases {
        let result = min_coins_bounded(&coins_with_count, amount);

        // Format the coin inventory for display
        let inventory: Vec<String> = coins_with_count
            .iter()
            .map(|(denom, qty)| format!("{}×{}", qty, denom))
            .collect();
        let inventory_str = inventory.join(", ");

        match result {
            Some(count) => println!(
                "{:30} | Amount: {:2} | Coins: [{:25}] | Min: {}",
                description, amount, inventory_str, count
            ),
            None => println!(
                "{:30} | Amount: {:2} | Coins: [{:25}] | Impossible",
                description, amount, inventory_str
            ),
        }
    }

    println!("\nNote: Bounded coin change uses binary representation optimization");
    println!("      to reduce O(n*k) to O(n*log(k)) where k is coin quantity.");
}

fn compare_approaches() {
    println!("\n--- Performance Comparison ---\n");

    let coins = vec![1, 5, 10, 25];
    let amounts = vec![11, 25, 50];

    println!("=== Minimum Coins (Optimization) ===\n");
    for amount in amounts {
        println!("Minimum coins for amount {}:", amount);

        // Naive
        let start = std::time::Instant::now();
        let result = min_coins_naive(&coins, amount);
        let duration = start.elapsed();
        println!("  Naive:      {:?} (took {:?})", result, duration);

        // Memoized
        let start = std::time::Instant::now();
        let result = min_coins_memoized(&coins, amount);
        let duration = start.elapsed();
        println!("  Memoized:   {:?} (took {:?})", result, duration);

        // Bottom-up
        let start = std::time::Instant::now();
        let result = min_coins_bottom_up(&coins, amount);
        let duration = start.elapsed();
        println!("  Bottom-up:  {:?} (took {:?})", result, duration);

        println!();
    }

    println!("=== Count Ways (Counting) ===\n");
    let count_amounts = vec![10, 25, 50, 100, 200, 400];
    for amount in count_amounts {
        println!("Count ways for amount {}:", amount);

        // Naive (only for smaller amounts - exponential growth!)
        if amount <= 500 {
            let start = std::time::Instant::now();
            let result = count_ways_naive(&coins, amount);
            let duration = start.elapsed();
            println!("  Naive:      {} (took {:?})", result, duration);
        } else {
            println!("  Naive:      <skipped - exponential growth too slow>");
        }

        // Memoized
        let start = std::time::Instant::now();
        let result = count_ways_memoized(&coins, amount);
        let duration = start.elapsed();
        println!("  Memoized:   {} (took {:?})", result, duration);

        // Bottom-up
        let start = std::time::Instant::now();
        let result = count_ways_bottom_up(&coins, amount);
        let duration = start.elapsed();
        println!("  Bottom-up:  {} (took {:?})", result, duration);

        println!();
    }

    println!("=== Bounded Coin Change (with limited quantities) ===\n");
    
    // Test 1: Small quantities - O(n*log(k)) should be fast
    let coins_small = vec![(1, 5), (5, 3), (10, 2), (25, 1)];
    let amount = 41;
    println!("Small quantities: {:?}, Amount: {}", coins_small, amount);
    let start = std::time::Instant::now();
    let result = min_coins_bounded(&coins_small, amount);
    let duration = start.elapsed();
    println!("  Bounded:    {:?} (took {:?})", result, duration);
    println!("  Complexity: O(n × Σlog(k)) where k = quantity per coin");
    println!();

    // Test 2: Large quantities - demonstrate log(k) efficiency
    let coins_large = vec![(1, 1000), (5, 500), (10, 200), (25, 100)];
    let amount = 999;
    println!("Large quantities: {:?}, Amount: {}", coins_large, amount);
    let start = std::time::Instant::now();
    let result = min_coins_bounded(&coins_large, amount);
    let duration = start.elapsed();
    println!("  Bounded:    {:?} (took {:?})", result, duration);
    println!("  Note: Despite k=1000, uses only log₂(1000)≈10 groups");
    println!();

    // Test 3: Compare bounded vs unbounded (when quantities are unlimited)
    let coins_unlimited = vec![(1, 1000), (5, 1000), (10, 1000), (25, 1000)];
    let coins_vec = vec![1, 5, 10, 25];
    let amount = 100;
    println!("Bounded vs Unbounded comparison for amount {}:", amount);
    
    let start = std::time::Instant::now();
    let result_bounded = min_coins_bounded(&coins_unlimited, amount);
    let duration_bounded = start.elapsed();
    println!("  Bounded:    {:?} (took {:?}) - O(n × Σlog(k))", result_bounded, duration_bounded);
    
    let start = std::time::Instant::now();
    let result_unbounded = min_coins_bottom_up(&coins_vec, amount);
    let duration_unbounded = start.elapsed();
    println!("  Unbounded:  {:?} (took {:?}) - O(m × n)", result_unbounded, duration_unbounded);
    
    println!("  Note: Unbounded is faster when quantities don't matter");
    println!("        Bounded is necessary when enforcing quantity limits");
    println!();
}

fn test_edge_cases() {
    println!("--- Edge Cases ---\n");

    let edge_cases = vec![
        (vec![1], 0, "Zero amount"),
        (vec![5], 0, "Zero with non-unit coin"),
        (vec![2], 3, "Impossible (odd amount, even coin)"),
        (vec![1, 5, 10], 11, "Multiple solutions"),
        (vec![3, 4, 5], 8, "Non-greedy optimal"),
    ];

    for (coins, amount, description) in edge_cases {
        let min_result = min_coins_bottom_up(&coins, amount);
        let count_result = count_ways_bottom_up(&coins, amount);
        println!(
            "{:30} | Min: {:?}, Ways: {}",
            description, min_result, count_result
        );
    }
}

// ============================================================================
// SOLUTIONS (Don't peek until you've tried!)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_coins() {
        let coins = vec![1, 5, 10, 25];

        assert_eq!(min_coins_bottom_up(&coins, 0), Some(0));
        assert_eq!(min_coins_bottom_up(&coins, 1), Some(1));
        assert_eq!(min_coins_bottom_up(&coins, 6), Some(2)); // 5+1
        assert_eq!(min_coins_bottom_up(&coins, 41), Some(4)); // 25+10+5+1

        // Impossible case
        let coins2 = vec![2, 5];
        assert_eq!(min_coins_bottom_up(&coins2, 3), None);
    }

    #[test]
    fn test_count_ways() {
        let coins = vec![1, 2, 5];

        assert_eq!(count_ways_bottom_up(&coins, 0), 1);
        assert_eq!(count_ways_bottom_up(&coins, 1), 1); // 1
        assert_eq!(count_ways_bottom_up(&coins, 2), 2); // 1+1, 2
        assert_eq!(count_ways_bottom_up(&coins, 5), 4); // 1+1+1+1+1, 1+1+1+2, 1+2+2, 5
    }

    #[test]
    fn test_greedy_fails() {
        // Greedy would choose 4+1+1 = 3 coins
        // Optimal is 3+3 = 2 coins
        let coins = vec![1, 3, 4];
        assert_eq!(min_coins_bottom_up(&coins, 6), Some(2));
    }
}
