# Tabulation Patterns - Bottom-Up Dynamic Programming in Rust

**Comprehensive guide to tabulation techniques, iterative DP strategies, and performance optimization in Rust**

---

## Overview

Tabulation is a bottom-up dynamic programming approach where solutions are built iteratively from base cases up to the target. Unlike memoization (top-down recursion with caching), tabulation fills a table systematically by solving smaller subproblems first.

**Key insight:** Tabulation is bottom-up dynamic programming - you solve all subproblems in order, storing results in a table (array/vector) for direct access.

---

## Core Concepts

### What is Tabulation?

Tabulation involves:

- **Table Construction**: Create array/vector to store subproblem solutions
- **Base Case Initialization**: Set initial values for smallest subproblems
- **Iterative Filling**: Solve subproblems in dependency order
- **Direct Access**: O(1) lookup by array indexing (no hash lookups)

```rust
// Memoization (Top-Down) - Recursive with cache
fn fib_memo(n: u32, cache: &mut HashMap<u32, u64>) -> u64 {
    if let Some(&result) = cache.get(&n) {
        return result;
    }
    let result = match n {
        0 | 1 => n as u64,
        _ => fib_memo(n - 1, cache) + fib_memo(n - 2, cache)
    };
    cache.insert(n, result);
    result
}

// Tabulation (Bottom-Up) - Iterative with table
fn fib_tabulation(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    
    let mut dp = vec![0u64; n + 1];
    dp[0] = 0;
    dp[1] = 1;
    
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    
    dp[n]
}
```

### Why Tabulation Matters

**Performance Comparison** (from Mission 11 coin change exercise):

- **Memoization**: 16.4µs for amount=11 (HashMap overhead)
- **Tabulation**: 2.8µs for amount=11 (direct array access)
- **Speedup**: ~6x faster despite same O(n) complexity!

**Key Advantages:**
- No function call overhead (iterative, not recursive)
- O(1) array indexing vs O(log n) hash lookup
- Better CPU cache locality (sequential memory access)
- Predictable performance (no recursion stack variance)

---

## Tabulation Patterns in Rust

### Pattern 1: 1D Table (Single Parameter DP)

**Best for:** Problems with single state dimension (amount, position, index)

```rust
/// Minimum coins needed to make amount (unbounded knapsack)
fn min_coins_tabulation(coins: &[u32], amount: u32) -> Option<u32> {
    let amount_usize = amount as usize;
    let mut dp = vec![None; amount_usize + 1];
    dp[0] = Some(0);  // Base case: 0 coins for amount 0
    
    // Fill table from amount 1 to target
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
```

**Iteration Order:** 0 → 1 → 2 → ... → target
- Each `dp[i]` depends only on `dp[j]` where `j < i`
- Safe to fill left-to-right sequentially

---

### Pattern 2: 2D Table (Two Parameter DP)

**Best for:** Problems with two state dimensions (items × capacity, string matching)

```rust
/// 0/1 Knapsack - can use each item at most once
fn knapsack_01(weights: &[u32], values: &[u32], capacity: u32) -> u32 {
    let n = weights.len();
    let cap = capacity as usize;
    
    // dp[i][w] = max value using first i items with weight limit w
    let mut dp = vec![vec![0u32; cap + 1]; n + 1];
    
    for i in 1..=n {
        for w in 0..=cap {
            // Don't take item i-1
            dp[i][w] = dp[i - 1][w];
            
            // Take item i-1 if it fits
            if weights[i - 1] as usize <= w {
                let with_item = dp[i - 1][w - weights[i - 1] as usize] + values[i - 1];
                dp[i][w] = dp[i][w].max(with_item);
            }
        }
    }
    
    dp[n][cap]
}
```

**Space Optimization:** Can reduce 2D to 1D if only previous row needed:

```rust
fn knapsack_01_optimized(weights: &[u32], values: &[u32], capacity: u32) -> u32 {
    let cap = capacity as usize;
    let mut dp = vec![0u32; cap + 1];
    
    for i in 0..weights.len() {
        // Iterate backwards to avoid overwriting needed values
        for w in (weights[i] as usize..=cap).rev() {
            dp[w] = dp[w].max(dp[w - weights[i] as usize] + values[i]);
        }
    }
    
    dp[cap]
}
```

---

### Pattern 3: Counting Variants

**Best for:** Counting number of ways to achieve target (combinations, paths)

```rust
/// Count ways to make amount with coins (order doesn't matter)
fn count_ways_tabulation(coins: &[u32], amount: u32) -> u64 {
    let amount_usize = amount as usize;
    let mut dp = vec![0u64; amount_usize + 1];
    dp[0] = 1;  // Base case: 1 way to make 0 (use no coins)
    
    // CRITICAL: Process coins in outer loop to avoid duplicates!
    // This ensures we count [1,5] and [5,1] as the same combination
    for &coin in coins {
        let coin_usize = coin as usize;
        for a in coin_usize..=amount_usize {
            dp[a] += dp[a - coin_usize];
        }
    }
    
    dp[amount_usize]
}

// If order DOES matter (permutations):
fn count_permutations_tabulation(coins: &[u32], amount: u32) -> u64 {
    let amount_usize = amount as usize;
    let mut dp = vec![0u64; amount_usize + 1];
    dp[0] = 1;
    
    // Process amounts in outer loop for permutations
    for a in 1..=amount_usize {
        for &coin in coins {
            if coin as usize <= a {
                dp[a] += dp[a - coin as usize];
            }
        }
    }
    
    dp[amount_usize]
}
```

**Loop Order Matters!**
- **Combinations** (no duplicates): Coins outer, amounts inner
- **Permutations** (order matters): Amounts outer, coins inner

---

## Real-World Applications

### Mission 11 Tutorial: Coin Change Problem

From `tutorials/Mission11_tut/exercises/exercise3_coin_change.rs`:

```rust
/// Minimum coins with bottom-up DP - O(m × n) time, O(n) space
fn min_coins_bottom_up(coins: &[u32], amount: u32) -> Option<u32> {
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
```

**Performance Results** (coins=[1,5,10,25]):
- Amount 11: **2.8µs** (vs 16.4µs memoized, 1.5µs naive)
- Amount 50: **7.8µs** (vs 35.2µs memoized, 104ms naive!)
- Amount 400: **54.1µs** for counting (vs 1.15ms memoized, 7.5ms naive)

---

### AoC 2024 Day 19: Pattern Matching with Towels

Tabulation is ideal for string/pattern DP:

```rust
fn count_arrangements_tabulation(pattern: &str, towels: &[&str]) -> u64 {
    let n = pattern.len();
    let mut dp = vec![0u64; n + 1];
    dp[0] = 1;  // Empty pattern: 1 way
    
    for i in 0..n {
        if dp[i] == 0 {
            continue;  // Can't reach position i
        }
        
        for &towel in towels {
            if pattern[i..].starts_with(towel) {
                dp[i + towel.len()] += dp[i];
            }
        }
    }
    
    dp[n]
}
```

---

## When to Use Tabulation vs Memoization

### ✅ Use Tabulation When:

1. **Dense Solution Space**: Need most/all subproblems
   - Example: Coin change with coins=[1,5,10,25] → all amounts 0→n reachable
   
2. **Simple Iteration Order**: Clear dependency structure
   - Example: Fibonacci (i depends on i-1, i-2)
   - Example: Minimum coins (amount a depends on a-coin)

3. **Performance Critical**: Need maximum speed
   - No function call overhead
   - No HashMap lookup cost
   - Better cache locality

4. **Known Bounds**: State space size is predictable
   - Can pre-allocate exact-size array

5. **Space Optimization Possible**: Can reduce dimensions
   - Example: 2D DP → 1D with backward iteration

### ❌ Use Memoization Instead When:

1. **Sparse Solution Space**: Only need small subset of subproblems
   - Example: coins=[500, 1000], amount=1000 (only 3 subproblems!)
   
2. **Complex Dependencies**: Hard to determine iteration order
   - Example: DAG traversal with arbitrary dependencies
   - Example: Multi-dimensional DP with tricky constraints

3. **Early Termination**: Can stop before solving everything
   - Example: "Does solution exist?" (return true immediately)

4. **Recursion is Natural**: Problem structure maps to recursion clearly
   - Example: Tree traversal, divide-and-conquer patterns

---

## Advanced Tabulation Patterns

### Pattern 4: Multiple Targets

Solve for multiple targets efficiently by filling table once:

```rust
fn min_coins_for_multiple_amounts(coins: &[u32], max_amount: u32) -> Vec<Option<u32>> {
    let max = max_amount as usize;
    let mut dp = vec![None; max + 1];
    dp[0] = Some(0);
    
    for a in 1..=max {
        for &coin in coins {
            if coin as usize <= a {
                if let Some(prev) = dp[a - coin as usize] {
                    dp[a] = Some(dp[a].map_or(prev + 1, |curr| curr.min(prev + 1)));
                }
            }
        }
    }
    
    dp  // Return entire table - O(1) lookup for any amount ≤ max
}
```

### Pattern 5: Path Reconstruction

Track choices to reconstruct solution:

```rust
fn min_coins_with_path(coins: &[u32], amount: u32) -> (Option<u32>, Vec<u32>) {
    let amount_usize = amount as usize;
    let mut dp = vec![None; amount_usize + 1];
    let mut choice = vec![None; amount_usize + 1];  // Track which coin used
    dp[0] = Some(0);
    
    for a in 1..=amount_usize {
        for &coin in coins {
            if coin as usize <= a {
                if let Some(prev) = dp[a - coin as usize] {
                    let candidate = prev + 1;
                    if dp[a].map_or(true, |curr| candidate < curr) {
                        dp[a] = Some(candidate);
                        choice[a] = Some(coin);  // Remember this choice
                    }
                }
            }
        }
    }
    
    // Reconstruct path
    let mut path = Vec::new();
    let mut current = amount_usize;
    while current > 0 {
        if let Some(coin) = choice[current] {
            path.push(coin);
            current -= coin as usize;
        } else {
            break;  // Impossible
        }
    }
    
    (dp[amount_usize], path)
}
```

### Pattern 6: Bounded Knapsack with Binary Grouping

From Mission 11 bounded coin change - O(n × Σlog(k)) optimization:

```rust
fn min_coins_bounded(coins_with_count: &[(u32, u32)], amount: u32) -> Option<u32> {
    let amount_usize = amount as usize;
    let inf = u32::MAX / 4;
    let mut dp = vec![inf; amount_usize + 1];
    dp[0] = 0;

    for &(coin, quantity) in coins_with_count {
        if coin == 0 || quantity == 0 {
            continue;
        }

        // Binary representation: group quantities as 1, 2, 4, 8, ..., remainder
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

            // Process this group (iterate backwards for 0/1 knapsack variant)
            for a in (weight..=amount_usize).rev() {
                let prev = dp[a - weight];
                if prev != inf {
                    dp[a] = dp[a].min(prev + take);
                }
            }
        }
    }

    if dp[amount_usize] == inf {
        None
    } else {
        Some(dp[amount_usize])
    }
}
```

**Why Binary Grouping?**
- Quantity k=13 → groups [1, 2, 4, 6] instead of 13 individual items
- Any value 0-13 achievable by selecting subset of groups
- Reduces O(n×k) to O(n×log(k))

---

## Performance Characteristics

### Complexity Analysis

| **Aspect** | **Tabulation** | **Memoization** |
|------------|----------------|-----------------|
| **Time** | O(subproblems × work_per_subproblem) | O(subproblems × work_per_subproblem) |
| **Space** | O(state_dimensions) | O(state_dimensions) + O(recursion_depth) |
| **Lookup** | O(1) array indexing | O(1) avg HashMap lookup |
| **Overhead** | Minimal (no function calls) | Recursion stack + hash operations |
| **Cache Locality** | Excellent (sequential access) | Poor (random access via hash) |
| **Predictability** | Deterministic iteration | Depends on recursion pattern |

**Measured Performance** (from exercise3_coin_change.rs):

```
Minimum coins for amount 50:
  Memoized:   Some(2) (took 35.2µs)
  Bottom-up:  Some(2) (took 7.8µs)   ← 4.5x faster!

Count ways for amount 400:
  Memoized:   10045 (took 1.1514ms)
  Bottom-up:  10045 (took 54.1µs)    ← 21x faster!
```

---

## Common Pitfalls

### Pitfall 1: Wrong Iteration Order

```rust
// ❌ BAD: Overwrites values still needed
fn knapsack_wrong(weights: &[u32], values: &[u32], capacity: u32) -> u32 {
    let mut dp = vec![0u32; capacity as usize + 1];
    
    for i in 0..weights.len() {
        for w in weights[i] as usize..=capacity as usize {  // Forward iteration!
            dp[w] = dp[w].max(dp[w - weights[i] as usize] + values[i]);
            // ❌ dp[w] might use already-updated dp[w-weight] from SAME iteration
        }
    }
    dp[capacity as usize]
}

// ✅ GOOD: Backward iteration preserves previous row values
fn knapsack_correct(weights: &[u32], values: &[u32], capacity: u32) -> u32 {
    let mut dp = vec![0u32; capacity as usize + 1];
    
    for i in 0..weights.len() {
        for w in (weights[i] as usize..=capacity as usize).rev() {  // Backward!
            dp[w] = dp[w].max(dp[w - weights[i] as usize] + values[i]);
            // ✅ dp[w-weight] still has previous row value
        }
    }
    dp[capacity as usize]
}
```

### Pitfall 2: Insufficient Table Size

```rust
// ❌ BAD: Off-by-one error
let mut dp = vec![0; amount];  // Needs amount+1 to index dp[amount]!

// ✅ GOOD: Correct size
let mut dp = vec![0; amount as usize + 1];
```

### Pitfall 3: Forgetting Base Case

```rust
// ❌ BAD: No base case
let mut dp = vec![0; n + 1];
for i in 1..=n {
    dp[i] = dp[i - 1] + dp[i - 2];  // ❌ dp[1] uses uninitialized dp[-1]!
}

// ✅ GOOD: Initialize base cases
let mut dp = vec![0; n + 1];
dp[0] = 0;
dp[1] = 1;  // Base cases set
for i in 2..=n {
    dp[i] = dp[i - 1] + dp[i - 2];
}
```

---

## Integration with Rust Concepts

### Ownership and Borrowing

```rust
// Tabulation naturally works with owned data
fn solve_tabulation(data: Vec<u32>) -> u32 {
    let mut dp = vec![0; data.len()];
    // No borrowing issues - table is local, owned, mutable
    for i in 0..data.len() {
        dp[i] = data[i] + dp.get(i.saturating_sub(1)).unwrap_or(&0);
    }
    dp[data.len() - 1]
}
```

### Iterator Patterns

```rust
// Functional style with iterators
fn fib_iterator(n: usize) -> u64 {
    (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
}

// Collect into table
fn solve_with_collect(items: &[Item]) -> Vec<u32> {
    items.iter()
        .scan(0, |state, item| {
            *state += item.value;
            Some(*state)
        })
        .collect()
}
```

---

## Testing Tabulation

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correctness() {
        let coins = vec![1, 5, 10, 25];
        assert_eq!(min_coins_tabulation(&coins, 11), Some(2));
        assert_eq!(min_coins_tabulation(&coins, 0), Some(0));
        assert_eq!(min_coins_tabulation(&[5, 10], 3), None);
    }

    #[test]
    fn test_vs_memoization() {
        let coins = vec![1, 5, 10, 25];
        let amount = 100;
        
        let tab_result = min_coins_tabulation(&coins, amount);
        
        let mut memo = std::collections::HashMap::new();
        let memo_result = min_coins_memoized(&coins, amount, &mut memo);
        
        assert_eq!(tab_result, memo_result);  // Same answer
    }

    #[test]
    fn test_counting_vs_permutations() {
        let coins = vec![1, 2];
        let amount = 3;
        
        // Combinations: [1,1,1], [1,2], [2,1] but [1,2]==[2,1] → 2 ways
        assert_eq!(count_ways_tabulation(&coins, amount), 2);
        
        // Permutations: [1,1,1], [1,2], [2,1] → 3 ways
        assert_eq!(count_permutations_tabulation(&coins, amount), 3);
    }
}
```

---

## Space Optimization Techniques

### Technique 1: Rolling Array (Reduce Dimensions)

```rust
// 2D DP: O(n × m) space
fn lcs_2d(s1: &str, s2: &str) -> usize {
    let (n, m) = (s1.len(), s2.len());
    let mut dp = vec![vec![0; m + 1]; n + 1];
    
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if s1.as_bytes()[i-1] == s2.as_bytes()[j-1] {
                dp[i-1][j-1] + 1
            } else {
                dp[i-1][j].max(dp[i][j-1])
            };
        }
    }
    dp[n][m]
}

// Optimized: O(min(n, m)) space
fn lcs_optimized(s1: &str, s2: &str) -> usize {
    let (n, m) = (s1.len(), s2.len());
    let mut prev = vec![0; m + 1];
    let mut curr = vec![0; m + 1];
    
    for i in 1..=n {
        for j in 1..=m {
            curr[j] = if s1.as_bytes()[i-1] == s2.as_bytes()[j-1] {
                prev[j-1] + 1
            } else {
                prev[j].max(curr[j-1])
            };
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[m]
}
```

### Technique 2: In-Place Updates

```rust
// Fibonacci: O(n) space
fn fib_array(n: usize) -> u64 {
    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i-1] + dp[i-2];
    }
    dp[n]
}

// Optimized: O(1) space
fn fib_constant(n: usize) -> u64 {
    if n <= 1 { return n as u64; }
    let (mut a, mut b) = (0, 1);
    for _ in 2..=n {
        (a, b) = (b, a + b);
    }
    b
}
```

---

## Complexity Comparison Table

**From Mission 11 Exercise 3** (m=coins, n=amount, k=quantity):

```text
┌─────────────┬────────────────────────┬────────────────────────┐
│  Approach   │   Minimum Coins        │    Count Ways          │
├─────────────┼────────────────────────┼────────────────────────┤
│ Naive       │ Time:  O(m^n)          │ Time:  O(m^n)          │
│ Recursion   │ Space: O(n) stack      │ Space: O(n) stack      │
│             │ (exponential explosion)│ (exponential explosion)│
├─────────────┼────────────────────────┼────────────────────────┤
│ Memoized    │ Time:  O(m × n)        │ Time:  O(m × n)        │
│ (Top-Down)  │ Space: O(n) memo+stack │ Space: O(n) memo+stack │
│             │ HashMap overhead       │ HashMap overhead       │
├─────────────┼────────────────────────┼────────────────────────┤
│ Tabulation  │ Time:  O(m × n)        │ Time:  O(m × n)        │
│ (Bottom-Up) │ Space: O(n) array only │ Space: O(n) array only │
│             │ ✅ FASTEST in practice │ ✅ FASTEST in practice │
├─────────────┼────────────────────────┼────────────────────────┤
│ Bounded     │ Time:  O(n × Σlog(k))  │ N/A (different problem)│
│ Binary Opt  │ Space: O(n)            │                        │
│             │ For quantity limits    │                        │
└─────────────┴────────────────────────┴────────────────────────┘
```

---

## Related Concepts

- **[[memoization-comprehensive-guide]]** - Top-down DP alternative (recursive with caching)
- **[[Dynamic Programming Patterns]]** - General DP strategies and problem classification
- **[[Mission 11 Knapsack Tutorial]]** - Complete unbounded/bounded knapsack implementations
- **[[Complexity Analysis]]** - Big-O analysis for tabulation vs memoization
- **[[Space Optimization Techniques]]** - Reducing memory usage in DP
- **[[../../tutorials/Mission11_tut/exercises/exercise3_coin_change.rs]]** - Complete working examples

---

## References

- *Tabulation is often faster than memoization in practice despite same asymptotic complexity*
- *See [[AoC Patterns MOC]] for real-world applications*
- *[[Performance Optimization Guide]] covers when to use which DP approach*
- *Mission 11 Tutorial demonstrates both approaches with benchmarks*

---

*Tags: #dynamic-programming #tabulation #bottom-up-dp #optimization #algorithms #rust #performance #knapsack*

*Related MOCs: [[Dynamic Programming MOC]] | [[AoC Patterns MOC]] | [[Algorithm Analysis]] | [[Algorithms MOC]] | [[Performance Optimization Guide]]*

---

**Key Takeaway:** Tabulation (bottom-up) is typically faster than memoization (top-down) when you need most subproblems. Use tabulation for dense solution spaces with clear iteration order. Use memoization for sparse spaces or complex dependencies.
