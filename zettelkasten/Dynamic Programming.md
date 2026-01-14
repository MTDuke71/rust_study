# Dynamic Programming

**Related:** [[Algorithms MOC]], [[Recursion]], [[memoization-comprehensive-guide]], [[HashMap]], [[Vec Type]], [[AoC Patterns MOC]], [[top-down-dp-pattern]], [[zero-copy-string-slicing]], [[mission-11]]

## Overview

**Dynamic Programming (DP)** is an algorithmic optimization technique that solves complex problems by breaking them down into simpler subproblems and storing their solutions to avoid redundant computation. It's particularly powerful for problems with **overlapping subproblems** and **optimal substructure** properties.

**Core Principle:** "Those who cannot remember the past are condemned to repeat it" - DP remembers previously computed results to achieve dramatic performance improvements (often exponential to polynomial time).

## Key Concepts

### 1. **Two Essential Properties**

**Optimal Substructure:**

- The optimal solution to a problem contains optimal solutions to its subproblems
- Example: Shortest path from A→C through B = Shortest(A→B) + Shortest(B→C)

**Overlapping Subproblems:**

- The same subproblems are solved multiple times in a naive recursive approach
- Example: Computing fibonacci(5) recalculates fibonacci(3) and fibonacci(2) multiple times

### 2. **Two Main Approaches**

**Top-Down (Memoization):**

```rust
use std::collections::HashMap;

fn fib_memo(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 {
        return n;
    }
    
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    
    let result = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);
    memo.insert(n, result);
    result
}
```

**Bottom-Up (Tabulation):**

```rust
fn fib_table(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    
    let mut dp = vec![0; (n + 1) as usize];
    dp[1] = 1;
    
    for i in 2..=n as usize {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    
    dp[n as usize]
}
```

**Space-Optimized:**

```rust
fn fib_optimized(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    
    let (mut prev, mut curr) = (0, 1);
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}
```

## Common DP Patterns

### 1. **Linear DP** (1D State)

- **Problems:** House Robber, Climbing Stairs, Maximum Subarray
- **State:** `dp[i]` = optimal solution up to index i
- **Transition:** `dp[i] = f(dp[i-1], dp[i-2], ...)`

### 2. **Grid DP** (2D State)

- **Problems:** Unique Paths, Minimum Path Sum, Edit Distance
- **State:** `dp[i][j]` = optimal solution at position (i, j)
- **Transition:** `dp[i][j] = f(dp[i-1][j], dp[i][j-1], ...)`

### 3. **Knapsack DP** (Subset Selection)

- **Problems:** 0/1 Knapsack, Subset Sum, Coin Change
- **State:** `dp[i][w]` = optimal using first i items with weight/capacity w
- **Transition:** Include or exclude each item

### 4. **Interval DP** (Range Problems)

- **Problems:** Longest Palindromic Substring, Matrix Chain Multiplication
- **State:** `dp[i][j]` = optimal solution for range [i, j]
- **Transition:** Split range at different points

### 5. **Tree DP** (Hierarchical Structures)

- **Problems:** Tree Distance, House Robber III
- **State:** `dp[node][state]` = optimal at node with given state
- **Transition:** Aggregate children's results

## Rust-Specific Considerations

### **Ownership & Lifetimes in Memoization**

```rust
// Problem: Memoization with borrowed data
fn solve_with_memo<'a>(
    s: &'a str,
    memo: &mut HashMap<&'a str, i32>
) -> i32 {
    if let Some(&result) = memo.get(s) {
        return result;
    }
    
    // Compute result...
    let result = 42; // placeholder
    memo.insert(s, result);
    result
}
```

**Challenge:** Rust's borrow checker requires careful lifetime management when caching references.

### **Memory-Efficient Patterns**

```rust
// Rolling array for space optimization
fn min_path_sum(grid: &[Vec<i32>]) -> i32 {
    let n = grid[0].len();
    let mut dp = vec![0; n];
    
    // Initialize first row
    dp[0] = grid[0][0];
    for j in 1..n {
        dp[j] = dp[j - 1] + grid[0][j];
    }
    
    // Process remaining rows
    for row in grid.iter().skip(1) {
        dp[0] += row[0];
        for j in 1..n {
            dp[j] = dp[j].min(dp[j - 1]) + row[j];
        }
    }
    
    dp[n - 1]
}
```

### **Type-Safe State Representation**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Included,
    Excluded,
}

// Use enums instead of boolean flags for clarity
fn solve(items: &[Item], state: State, memo: &mut HashMap<(usize, State), i32>) -> i32 {
    // Type-safe state transitions
    match state {
        State::Included => { /* ... */ }
        State::Excluded => { /* ... */ }
    }
    0 // placeholder
}
```

## Advent of Code Applications

### **Common AoC DP Problems:**

1. **Pathfinding with Costs** (Many AoC grid problems)
   - Minimum cost path through grid
   - Dijkstra's algorithm with DP state

2. **Counting Ways** (Combinatorial problems)
   - Number of ways to reach goal
   - Number of valid sequences

3. **Optimization Problems** (Maximize/minimize)
   - Maximum profit/score
   - Minimum operations/cost

4. **Sequence Problems** (String/array processing)
   - Longest Common Subsequence
   - Edit distance between strings

### **Example: Coin Change (AoC-style)**

```rust
/// Given coins and amount, find minimum coins needed
/// Returns None if impossible
fn coin_change(coins: &[u32], amount: u32) -> Option<u32> {
    let mut dp = vec![u32::MAX; (amount + 1) as usize];
    dp[0] = 0;
    
    for amt in 1..=amount as usize {
        for &coin in coins {
            if coin as usize <= amt && dp[amt - coin as usize] != u32::MAX {
                dp[amt] = dp[amt].min(dp[amt - coin as usize] + 1);
            }
        }
    }
    
    if dp[amount as usize] == u32::MAX {
        None
    } else {
        Some(dp[amount as usize])
    }
}
```

## Decision Framework: When to Use DP

### **✅ Use DP When:**

- Problem asks for optimal solution (min/max)
- Problem asks for counting (number of ways)
- Recursive solution has repeated subproblems
- Problem has overlapping subproblems
- Small input size allows O(n²) or O(n³) complexity

### **❌ Don't Use DP When:**

- Greedy algorithm works (simpler)
- No overlapping subproblems (use divide & conquer)
- Subproblems not independent (use other techniques)
- State space too large (exponential states)

## Performance Characteristics

| Approach | Time Complexity | Space Complexity | Use Case |
|----------|----------------|------------------|----------|
| **Naive Recursion** | O(2ⁿ) typical | O(n) stack | Small inputs only |
| **Top-Down Memo** | O(n × states) | O(n × states) | Natural recursion |
| **Bottom-Up Table** | O(n × states) | O(n × states) | Iterative preference |
| **Space-Optimized** | O(n × states) | O(states) | Memory constraints |

## Common Pitfalls in Rust

### **1. Lifetime Issues with Memoization**

```rust
// ❌ Won't compile - lifetime conflict
fn bad_memo(s: String, memo: &mut HashMap<String, i32>) -> i32 {
    if let Some(&result) = memo.get(&s) {
        return result;
    }
    // s is moved here, can't insert into memo
    let result = process(s); 
    // memo.insert(s, result); // Error: s was moved
    result
}

// ✅ Use references or clone strategically
fn good_memo(s: &str, memo: &mut HashMap<String, i32>) -> i32 {
    if let Some(&result) = memo.get(s) {
        return result;
    }
    let result = process(s);
    memo.insert(s.to_string(), result);
    result
}
```

### **2. Integer Overflow**

```rust
// ❌ Can overflow
fn fib_bad(n: u32) -> u32 {
    // ...
}

// ✅ Use appropriate types or checked arithmetic
fn fib_good(n: u32) -> Option<u64> {
    // Use u64 or checked_add
}
```

### **3. Off-by-One Errors**

```rust
// ❌ Index confusion
let dp = vec![0; n]; // Size n
let result = dp[n]; // Panic! Index out of bounds

// ✅ Careful size calculation
let dp = vec![0; n + 1]; // Size n+1 for indices 0..=n
let result = dp[n]; // OK
```

## Learning Progression

### **Phase 1: Basic DP** (Mission 11 Foundation)

1. Fibonacci variations (1D DP)
2. Climbing stairs problems
3. House robber (non-adjacent selection)

### **Phase 2: 2D DP** (Grid Problems)

1. Unique paths in grid
2. Minimum path sum
3. Longest common subsequence

### **Phase 3: Advanced Patterns** (AoC Preparation)

1. Knapsack variations
2. State machine DP
3. Bitmask DP
4. Tree DP

## Related Missions & Resources

- **🎯 Mission 11** (Planned): Dynamic Programming - Formal implementation with V-Cycle methodology
- **🎯 Mission 5**: [[mission-5]] - HashMap for memoization
- **🎯 Mission 3**: [[mission-3]] - Binary search (greedy vs DP decision)
- **📚 Rust Book Ch8**: [[zettelkasten/rust_book/rust-book-ch8]] - Collections for DP state storage

## Practice Resources

### **LeetCode DP Patterns:**

- Easy: Climbing Stairs (70), House Robber (198)
- Medium: Coin Change (322), Longest Increasing Subsequence (300)
- Hard: Edit Distance (72), Regular Expression Matching (10)

### **AoC DP Problems:**

- 2015 Day 17: Container combinations (subset sum)
- 2018 Day 9: Marble game (simulation with optimization)
- 2020 Day 10: Adapter chains (counting paths)
- 2021 Day 21: Dirac Dice (game states DP)

## References & Further Reading

- **Theory:** Introduction to Algorithms (CLRS) - Chapter 15
- **Practice:** Dynamic Programming Patterns (Grokking DP)
- **Rust-Specific:** Rust Algorithm Club - DP implementations
- **Competitive Programming:** Competitive Programmer's Handbook - Chapter 7

---

## Links & Navigation

**Core Concepts:**

- [[Recursion]] - Foundation for top-down DP
- [[Memoization]] - Caching technique for DP optimization
- [[top-down-dp-pattern]] - Canonical 4-step recursive DP template
- [[HashMap]] - Rust's hash table for memoization
- [[Vec Type]] - Array storage for tabulation
- [[zero-copy-string-slicing]] - Performance optimization for string DP

**Related Topics:**

- [[Algorithms MOC]] - Algorithmic patterns overview
- [[Greedy Algorithms]] - When DP is overkill
- [[Divide and Conquer]] - Related but distinct paradigm
- [[Complexity Analysis]] - Understanding DP performance

**Applications:**

- [[AoC Patterns MOC]] - DP in competitive programming
- [[Performance Optimization Guide]] - When and how to optimize DP

**Navigation:**

- [[zettel-index]] - Main knowledge base
- [[rust-concepts-MOC]] - Language features for DP
- [[Daily Study MOC]] - Progressive learning path

---

*Tags: #dynamic-programming #algorithms #optimization #memoization #recursion #aoc-pattern #mission-11 #rust-algorithms*

*Created: 2025-10-26 | Status: 🎯 Reference Ready | Mission: Planned for Mission 11*
