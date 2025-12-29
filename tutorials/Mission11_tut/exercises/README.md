# Mission 11 Tutorial: Practice Exercises
**Zettelkasten**: [[mission-11]] - Mission 11 knowledge hub
This directory contains 3 comprehensive practice exercises to reinforce dynamic programming concepts from the tutorial steps.

## 📋 Exercise Overview

| Exercise | Topic | Difficulty | Concepts |
|----------|-------|-----------|----------|
| **Exercise 1** | Fibonacci Variants | ⭐ Easy | Naive recursion, memoization, bottom-up, space optimization |
| **Exercise 2** | Longest Common Subsequence | ⭐⭐ Medium | 2D state space, HashMap<(i,j)>, reconstruction |
| **Exercise 3** | Coin Change | ⭐⭐⭐ Hard | Optimization vs counting, unbounded knapsack |

## 🎯 How to Use These Exercises

### 1. **Try Before Looking at Solutions**

Each exercise file has:
- `TODO` sections for you to implement
- Solutions in `#[cfg(test)] mod solutions` at the bottom
- Don't peek until you've tried!

### 2. **Run Each Exercise**

```bash
# Exercise 1: Fibonacci (start here)
cargo run -p mission11_tut --example exercise1_fibonacci

# Exercise 2: LCS
cargo run -p mission11_tut --example exercise2_longest_subsequence

# Exercise 3: Coin Change
cargo run -p mission11_tut --example exercise3_coin_change
```

The `main()` function in each will call TODO functions (which will panic). As you implement each function, remove the `todo!()` macro and add your solution.

### 3. **Test Your Solutions**

```bash
# Test exercise 1
cargo test -p mission11_tut --example exercise1_fibonacci

# Test exercise 2
cargo test -p mission11_tut --example exercise2_longest_subsequence

# Test exercise 3
cargo test -p mission11_tut --example exercise3_coin_change
```

All tests should pass once you've correctly implemented the TODO functions.

### 4. **Compare Approaches**

Each exercise includes performance comparison demonstrations:
- Naive recursion (exponential - only works for small inputs)
- Top-down memoization (linear - fast)
- Bottom-up DP (linear - faster, no recursion overhead)
- Space-optimized (when applicable)

---

## 📖 Exercise 1: Fibonacci Variants

**File**: `exercise1_fibonacci.rs`

### What You'll Implement

1. `fibonacci_naive(n)` - Simple recursion (exponential)
2. `fibonacci_memo_helper(n, memo)` - With HashMap caching
3. `fibonacci_bottom_up(n)` - Build table from F(0) to F(n)
4. `fibonacci_optimized(n)` - Only track last 2 values
5. **BONUS**: `fibonacci_modular(n)` - Handle F(100+) with modular arithmetic

### Key Learning Points

- See exponential complexity explosion with naive approach
- Observe dramatic speedup with memoization
- Understand space-time tradeoffs
- Practice modular arithmetic to avoid overflow

### Example Output

```
F(10):
  Naive:      55 (took 2.3µs)
  Memoized:   55 (took 1.1µs)
  Bottom-up:  55 (took 0.8µs)
  Optimized:  55 (took 0.5µs)

F(100) mod 1000000007 = 687995182 (took 2.1µs)
```

---

## 📖 Exercise 2: Longest Common Subsequence

**File**: `exercise2_longest_subsequence.rs`

### What You'll Implement

1. `lcs_naive_helper(s1, s2, i, j)` - Recursive with 2 indices
2. `lcs_memo_helper(...)` - Cache with HashMap<(usize, usize), usize>
3. `lcs_bottom_up(s1, s2)` - Build 2D DP table
4. `lcs_space_optimized(s1, s2)` - Only need O(min(m,n)) space
5. **BONUS**: `lcs_reconstruct(s1, s2)` - Return actual LCS string, not just length

### Key Learning Points

- Practice 2D state space (two indices)
- Understand when characters match vs don't match
- See how to reconstruct solution by backtracking
- Learn space optimization with rolling arrays

### Example Output

```
LCS('ABCDEF', 'ADBEF') = 4 ('ABEF')
LCS('AGGTAB', 'GXTXAYB') = 4 ('GTAB')
LCS('ABC', 'DEF') = 0 ('')
```

---

## 📖 Exercise 3: Coin Change Problem

**File**: `exercise3_coin_change.rs`

### What You'll Implement

#### Part 1: Minimum Coins (Optimization)

1. `min_coins_naive(coins, amount)` - Try all combinations
2. `min_coins_memo_helper(...)` - Cache with HashMap<u32, Option<u32>>
3. `min_coins_bottom_up(coins, amount)` - Build DP table

#### Part 2: Count Ways (Counting)

4. `count_ways_naive_helper(coins, amount, coin_idx)` - Avoid duplicates
5. `count_ways_memo_helper(...)` - Cache with (amount, coin_idx) key
6. `count_ways_bottom_up(coins, amount)` - Order matters for uniqueness!

### Key Learning Points

- Understand optimization vs counting variants
- Handle impossible cases (return None or 0)
- Learn why coin order matters in counting
- Practice unbounded knapsack pattern (can use same coin multiple times)

### Example Output

```
Part 1: Minimum Coins
US coins            | Amount: 41 | Min coins: 4
Greedy fails        | Amount:  6 | Min coins: 2
Impossible          | Amount:  3 | Impossible

Part 2: Count Ways
Standard coins      | Amount:   5 | Ways: 4
No pennies          | Amount:   7 | Ways: 2
One dollar          | Amount: 100 | Ways: 242
```

---

## 🎓 Learning Progression

### Recommended Order

1. **Exercise 1 (Fibonacci)** - Start here
   - Simplest 1D state space
   - Clear performance differences
   - Builds confidence

2. **Exercise 2 (LCS)** - Next
   - Introduces 2D state
   - More complex recurrence
   - String manipulation

3. **Exercise 3 (Coin Change)** - Final
   - Two different variants (optimization + counting)
   - Handling impossible cases
   - Order-dependent logic

### Time Estimate

- **Exercise 1**: 30-45 minutes
- **Exercise 2**: 45-60 minutes
- **Exercise 3**: 60-90 minutes

**Total**: 2.5 - 3 hours for all 3 exercises

---

## 💡 Tips for Success

### General Strategy

1. **Start with base cases** - What are the simplest inputs?
2. **Write the recurrence** - On paper, what's the recursive formula?
3. **Implement naive first** - Get correctness before optimization
4. **Add memoization** - Cache lookup + store before return
5. **Convert to bottom-up** - Build table in correct order
6. **Test incrementally** - Don't wait to implement everything

### Common Mistakes to Avoid

❌ **Forgetting to cache before returning**
```rust
// WRONG
fn fib(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&v) = memo.get(&n) { return v; }
    let result = fib(n-1, memo) + fib(n-2, memo);
    result  // ← FORGOT memo.insert(n, result)!
}
```

✅ **Always cache before returning**
```rust
fn fib(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&v) = memo.get(&n) { return v; }
    let result = fib(n-1, memo) + fib(n-2, memo);
    memo.insert(n, result);  // ← Always cache!
    result
}
```

❌ **Wrong table iteration order**
```rust
// WRONG - accessing dp[i+1] before it's computed
for i in 0..n {
    dp[i] = dp[i+1] + dp[i+2];  // ← i+1 not yet filled!
}
```

✅ **Iterate in dependency order**
```rust
// CORRECT - compute from bottom to top
for i in (0..n).rev() {
    dp[i] = dp[i+1] + dp[i+2];  // ← i+1 already computed
}
```

---

## 🔍 How to Debug

### Add Print Statements

```rust
fn fib_debug(n: u64, memo: &mut HashMap<u64, u64>, depth: usize) -> u64 {
    let indent = "  ".repeat(depth);
    println!("{}fib({})", indent, n);
    
    if let Some(&v) = memo.get(&n) {
        println!("{}→ cache hit: {}", indent, v);
        return v;
    }
    
    // ... rest of implementation
}
```

### Check State Space Size

```rust
println!("Cache size: {}", memo.len());
println!("Expected max: O({} * {}) = {}", m, n, m * n);
```

### Verify with Known Values

```rust
assert_eq!(lcs("ABC", "AC"), 2);  // Known correct
assert_eq!(fibonacci(10), 55);    // F(10) = 55
```

---

## ✅ Completion Checklist

After completing all exercises, you should be able to:

- [ ] Recognize when a problem needs DP (overlapping subproblems + optimal substructure)
- [ ] Choose between top-down vs bottom-up approaches
- [ ] Design state space (1D, 2D, or custom keys)
- [ ] Write correct recurrence relations
- [ ] Implement memoization with HashMap
- [ ] Build bottom-up DP tables
- [ ] Optimize space complexity when possible
- [ ] Handle edge cases (empty inputs, impossible states)
- [ ] Debug DP solutions systematically

---

## 🚀 Next Steps

Once you've completed all 3 exercises:

1. **Review Solutions**: Compare your implementations with the provided solutions
2. **Optimize Further**: Can you reduce space/time complexity more?
3. **Try Variations**: What if inputs are larger? What if constraints change?
4. **Move to Mission 11**: Apply these patterns to production-grade implementation

---

## 📚 Additional Practice Problems

Want more practice? Try these:

### Easy
- Min Cost Climbing Stairs (LeetCode 746)
- House Robber (LeetCode 198)
- Maximum Subarray (LeetCode 53)

### Medium
- Unique Paths (LeetCode 62)
- Edit Distance (LeetCode 72)
- Decode Ways (LeetCode 91)

### Hard
- Longest Palindromic Subsequence (LeetCode 516)
- Regular Expression Matching (LeetCode 10)
- Burst Balloons (LeetCode 312)

---

**Happy coding!** 🎉

*Remember: The goal is not just to make tests pass, but to deeply understand the patterns and build intuition for recognizing DP problems in the wild.*
