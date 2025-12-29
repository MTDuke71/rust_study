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

use std::collections::HashMap;

fn main() {
    println!("=== Exercise 3: Coin Change Problem ===\n");
    
    demonstrate_minimum_coins();
    demonstrate_count_ways();
    compare_approaches();
    test_edge_cases();
    
    println!("\n✅ Exercise 3 completed!");
}

// ============================================================================
// Part 1: MINIMUM COINS NEEDED (Optimization Problem)
// ============================================================================

/// Naive recursion for minimum coins (SLOW)
#[allow(dead_code, unused_variables)]
fn min_coins_naive(coins: &[u32], amount: u32) -> Option<u32> {
    // TODO: Implement naive recursion
    // 1. Base case: amount == 0 → return Some(0)
    // 2. Base case: amount < 0 → return None (impossible)
    // 3. Try each coin, recursively solve for (amount - coin)
    // 4. Return minimum + 1, or None if all paths impossible
    todo!("Implement min_coins_naive");
}

/// Top-down with memoization for minimum coins
fn min_coins_memoized(coins: &[u32], amount: u32) -> Option<u32> {
    let mut memo = HashMap::new();
    min_coins_memo_helper(coins, amount, &mut memo)
}

#[allow(dead_code, unused_variables)]
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
    todo!("Implement min_coins_memo_helper");
}

/// Bottom-up DP for minimum coins
#[allow(dead_code, unused_variables)]
fn min_coins_bottom_up(coins: &[u32], amount: u32) -> Option<u32> {
    // TODO: Implement bottom-up DP
    // 1. Create array dp[0..=amount]
    // 2. Initialize dp[0] = 0, others = infinity
    // 3. For each amount a from 1 to target:
    //    For each coin c:
    //      if c <= a: dp[a] = min(dp[a], 1 + dp[a - c])
    // 4. Return dp[amount] if not infinity
    todo!("Implement min_coins_bottom_up");
}

// ============================================================================
// Part 2: COUNT WAYS (Counting Problem)
// ============================================================================

/// Naive recursion for counting ways (SLOW)
#[allow(dead_code)]
fn count_ways_naive(coins: &[u32], amount: u32) -> u64 {
    count_ways_naive_helper(coins, amount, 0)
}

#[allow(dead_code, unused_variables)]
fn count_ways_naive_helper(coins: &[u32], amount: u32, coin_idx: usize) -> u64 {
    // TODO: Implement naive counting
    // Note: Need coin_idx to avoid counting duplicates (1+5 vs 5+1)
    // 1. Base case: amount == 0 → return 1 (found a way)
    // 2. Base case: amount < 0 or no more coins → return 0
    // 3. Include current coin: count_ways(amount - coin, coin_idx)
    // 4. Exclude current coin: count_ways(amount, coin_idx + 1)
    // 5. Return sum of both
    todo!("Implement count_ways_naive_helper");
}

/// Top-down with memoization for counting ways
#[allow(dead_code)]
fn count_ways_memoized(coins: &[u32], amount: u32) -> u64 {
    let mut memo = HashMap::new();
    count_ways_memo_helper(coins, amount, 0, &mut memo)
}

#[allow(dead_code, unused_variables)]
fn count_ways_memo_helper(
    coins: &[u32],
    amount: u32,
    coin_idx: usize,
    memo: &mut HashMap<(u32, usize), u64>,
) -> u64 {
    // TODO: Implement memoized counting
    // Use (amount, coin_idx) as cache key
    todo!("Implement count_ways_memo_helper");
}

/// Bottom-up DP for counting ways
#[allow(dead_code, unused_variables)]
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
    todo!("Implement count_ways_bottom_up");
}

// ============================================================================
// BONUS: Bounded Coin Change
// ============================================================================

/// Minimum coins with limited quantities
/// coins_with_count: [(denomination, quantity)]
#[allow(dead_code, unused_variables)]
fn min_coins_bounded(coins_with_count: &[(u32, u32)], amount: u32) -> Option<u32> {
    // TODO: CHALLENGE - Implement bounded knapsack variant
    // Need to track how many of each coin we've used
    // State becomes (amount, remaining_coins)
    todo!("BONUS: Implement min_coins_bounded");
}

// ============================================================================
// Tests and Demonstrations
// ============================================================================

fn demonstrate_minimum_coins() {
    println!("--- Part 1: Minimum Coins Needed ---\n");
    
    let test_cases = vec![
        (vec![1, 5, 10, 25], 41, "US coins"),
        (vec![1, 3, 4], 6, "Greedy fails"),
        (vec![2, 5], 3, "Impossible"),
        (vec![1, 2, 5], 11, "Standard example"),
    ];
    
    for (coins, amount, description) in test_cases {
        let result = min_coins_bottom_up(&coins, amount);
        match result {
            Some(count) => println!("{:20} | Amount: {:2} | Min coins: {}", 
                                    description, amount, count),
            None => println!("{:20} | Amount: {:2} | Impossible", 
                            description, amount),
        }
    }
}

fn demonstrate_count_ways() {
    println!("\n--- Part 2: Count Ways to Make Amount ---\n");
    
    let test_cases = vec![
        (vec![1, 2, 5], 5, "Standard coins"),
        (vec![2, 3, 5], 7, "No pennies"),
        (vec![1, 5, 10, 25], 10, "Exactly one coin"),
        (vec![1, 5, 10, 25], 100, "One dollar"),
    ];
    
    for (coins, amount, description) in test_cases {
        let count = count_ways_bottom_up(&coins, amount);
        println!("{:20} | Amount: {:3} | Ways: {}", 
                 description, amount, count);
    }
}

fn compare_approaches() {
    println!("\n--- Performance Comparison ---\n");
    
    let coins = vec![1, 5, 10, 25];
    let amounts = vec![11, 25, 50];
    
    for amount in amounts {
        println!("Minimum coins for amount {}:", amount);
        
        // Naive (only small amounts)
        if amount <= 15 {
            let start = std::time::Instant::now();
            let result = min_coins_naive(&coins, amount);
            let duration = start.elapsed();
            println!("  Naive:      {:?} (took {:?})", result, duration);
        } else {
            println!("  Naive:      <skipped - too slow>");
        }
        
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
        println!("{:30} | Min: {:?}, Ways: {}", 
                 description, min_result, count_result);
    }
}

// ============================================================================
// SOLUTIONS (Don't peek until you've tried!)
// ============================================================================

#[cfg(test)]
mod solutions {
    use super::*;
    
    // Solution 1: Naive Minimum Coins
    #[allow(dead_code)]
    fn min_coins_naive_solution(coins: &[u32], amount: u32) -> Option<u32> {
        if amount == 0 {
            return Some(0);
        }
        
        let mut min = None;
        
        for &coin in coins {
            if coin <= amount {
                if let Some(sub_result) = min_coins_naive_solution(coins, amount - coin) {
                    min = Some(min.map_or(sub_result + 1, |m: u32| m.min(sub_result + 1)));
                }
            }
        }
        
        min
    }
    
    // Solution 2: Memoized Minimum Coins
    fn min_coins_memo_solution_helper(
        coins: &[u32],
        amount: u32,
        memo: &mut HashMap<u32, Option<u32>>,
    ) -> Option<u32> {
        if amount == 0 {
            return Some(0);
        }
        
        if let Some(&result) = memo.get(&amount) {
            return result;
        }
        
        let mut min = None;
        
        for &coin in coins {
            if coin <= amount {
                if let Some(sub_result) = min_coins_memo_solution_helper(coins, amount - coin, memo) {
                    min = Some(min.map_or(sub_result + 1, |m: u32| m.min(sub_result + 1)));
                }
            }
        }
        
        memo.insert(amount, min);
        min
    }
    
    // Solution 3: Bottom-Up Minimum Coins
    #[allow(dead_code)]
    fn min_coins_bottom_up_solution(coins: &[u32], amount: u32) -> Option<u32> {
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
    
    // Solution 4: Naive Count Ways
    fn count_ways_naive_solution_helper(coins: &[u32], amount: u32, coin_idx: usize) -> u64 {
        if amount == 0 {
            return 1;
        }
        
        if coin_idx >= coins.len() {
            return 0;
        }
        
        let coin = coins[coin_idx];
        let mut total = 0;
        
        // Try using this coin 0, 1, 2, ... times
        let mut remaining = amount;
        while remaining >= coin {
            total += count_ways_naive_solution_helper(coins, remaining - coin, coin_idx + 1);
            remaining -= coin;
        }
        // Don't use this coin at all
        total += count_ways_naive_solution_helper(coins, remaining, coin_idx + 1);
        
        total
    }
    
    // Solution 5: Memoized Count Ways
    fn count_ways_memo_solution_helper(
        coins: &[u32],
        amount: u32,
        coin_idx: usize,
        memo: &mut HashMap<(u32, usize), u64>,
    ) -> u64 {
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
            count_ways_memo_solution_helper(coins, amount - coin, coin_idx, memo)
        } else {
            0
        };
        
        // Exclude current coin
        let exclude = count_ways_memo_solution_helper(coins, amount, coin_idx + 1, memo);
        
        let result = include + exclude;
        memo.insert(key, result);
        result
    }
    
    // Solution 6: Bottom-Up Count Ways
    #[allow(dead_code)]
    fn count_ways_bottom_up_solution(coins: &[u32], amount: u32) -> u64 {
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
    
    #[test]
    fn test_minimum_coins() {
        let coins = vec![1, 5, 10, 25];
        
        assert_eq!(min_coins_bottom_up_solution(&coins, 0), Some(0));
        assert_eq!(min_coins_bottom_up_solution(&coins, 1), Some(1));
        assert_eq!(min_coins_bottom_up_solution(&coins, 6), Some(2));  // 5+1
        assert_eq!(min_coins_bottom_up_solution(&coins, 41), Some(4)); // 25+10+5+1
        
        // Impossible case
        let coins2 = vec![2, 5];
        assert_eq!(min_coins_bottom_up_solution(&coins2, 3), None);
    }
    
    #[test]
    fn test_count_ways() {
        let coins = vec![1, 2, 5];
        
        assert_eq!(count_ways_bottom_up_solution(&coins, 0), 1);
        assert_eq!(count_ways_bottom_up_solution(&coins, 1), 1);  // 1
        assert_eq!(count_ways_bottom_up_solution(&coins, 2), 2);  // 1+1, 2
        assert_eq!(count_ways_bottom_up_solution(&coins, 5), 4);  // 1+1+1+1+1, 1+1+1+2, 1+2+2, 5
    }
    
    #[test]
    fn test_greedy_fails() {
        // Greedy would choose 4+1+1 = 3 coins
        // Optimal is 3+3 = 2 coins
        let coins = vec![1, 3, 4];
        assert_eq!(min_coins_bottom_up_solution(&coins, 6), Some(2));
    }
}
