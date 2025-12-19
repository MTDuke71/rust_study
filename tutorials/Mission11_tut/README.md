# Tutorial 11: Dynamic Programming with Memoization

**Companion to**: [Mission 11](../../missions/Mission11/)  
**Focus**: Step-by-step mastery of memoization patterns and lifetime management  
**Duration**: 7 days (1 step per day)  
**Prerequisites**: Mission 1 (recursion), Mission 5 (HashMap)

---

## 🎯 Tutorial Overview

This tutorial builds dynamic programming expertise through **progressive complexity**:

1. **Naive recursion** - See exponential complexity firsthand
2. **Manual HashMap memoization** - Add caching, observe speedup
3. **Lifetime safety** - Solve borrowing and lifetime errors
4. **Generic abstraction** - Make patterns reusable
5. **Boolean → Counting** - Transform existence to enumeration
6. **Bottom-up alternative** - Compare approaches
7. **Real AoC problems** - Apply to competitive programming

### Learning Philosophy

Each step is **runnable and testable**. You'll:
- See problems **before** solutions (struggle → insight)
- Measure performance **empirically** (not theoretically)
- Fix **real compiler errors** (lifetime practice)
- Apply to **actual AoC problems** (validation)

---

## 📚 Tutorial Structure

```
tutorials/Mission11_tut/
├── README.md                      # This file - Tutorial roadmap
├── Cargo.toml                     # Workspace member configuration
├── examples/
│   ├── step1_naive_recursion.rs       # 🐢 Exponential without cache
│   ├── step2_manual_hashmap.rs        # ⚡ Add memoization manually
│   ├── step3_lifetime_safety.rs       # 🔧 Solve borrowing issues
│   ├── step4_generic_cache.rs         # 🎨 Make it reusable
│   ├── step5_boolean_to_counting.rs   # 🔢 Transform pattern
│   ├── step6_bottom_up_table.rs       # 📊 Alternative DP approach
│   └── step7_real_aoc_problems.rs     # 🎄 Apply to Days 11, 19
├── tests/
│   └── tutorial_tests.rs              # Validation tests
└── exercises/
    ├── exercise1_fibonacci.md         # Practice: Classic DP
    ├── exercise2_longest_subsequence.md  # Practice: String DP
    └── exercise3_coin_change.md       # Practice: Optimization DP
```

---

## 📖 Step-by-Step Guide

### Step 1: Naive Recursion (Day 1) 🐢

**File**: `examples/step1_naive_recursion.rs`

**Learning Goal**: Experience exponential complexity and understand why memoization is necessary.

**Problem**: Pattern matching (simplified AoC Day 19)
```rust
// Can we make "brwrr" from patterns ["r", "wr", "b", "br"]?
fn can_make(patterns: &[&str], design: &str) -> bool {
    if design.is_empty() { return true; }
    
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            if can_make(patterns, rest) {  // ← Recursive call
                return true;
            }
        }
    }
    false
}
```

**Complexity**:
- **Time**: O(P^L) where P = patterns, L = design length
- **Example**: 8 patterns, 20-char design → 8^20 = **quintillion operations**

**Exercise**: 
1. Run with small inputs (works fine)
2. Try long designs (hangs forever)
3. Add print statements to see redundant computation
4. Count how many times the same substring is checked

**Key Insight**: Without caching, the same subproblem is solved **exponentially many times**.

---

### Step 2: Manual HashMap Memoization (Day 2) ⚡

**File**: `examples/step2_manual_hashmap.rs`

**Learning Goal**: Add memoization cache and observe transformation from exponential to linear.

**Implementation**:
```rust
use std::collections::HashMap;

fn can_make_memo<'a>(
    patterns: &[&str],
    design: &'a str,
    memo: &mut HashMap<&'a str, bool>,  // ← Cache previous results
) -> bool {
    if design.is_empty() { return true; }
    
    // Check cache first
    if let Some(&result) = memo.get(design) {
        return result;  // ← Instant return if computed before
    }
    
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            if can_make_memo(patterns, rest, memo) {
                memo.insert(design, true);  // ← Cache before returning
                return true;
            }
        }
    }
    
    memo.insert(design, false);
    false
}
```

**Complexity**:
- **Time**: O(P × L) - each unique substring computed once
- **Space**: O(U) where U = unique substrings (≤ L²)

**Exercise**:
1. Instrument cache hits/misses with counters
2. Compare runtime: naive vs memoized
3. Measure cache size vs theoretical state space
4. Test with real AoC Day 19 input

**Key Insight**: Memoization transforms **intractable → instant** with minimal code changes.

---

### Step 3: Lifetime Safety (Day 3) 🔧

**File**: `examples/step3_lifetime_safety.rs`

**Learning Goal**: Master lifetime parameters in recursive functions with borrowed keys.

**Common Errors**:
```rust
// ❌ This won't compile:
fn broken<'a>(design: &str, memo: &mut HashMap<&'a str, bool>) -> bool {
    // ERROR: 'design' doesn't live long enough to insert into 'memo'
    memo.insert(design, true);
}

// ✅ This works:
fn correct<'a>(design: &'a str, memo: &mut HashMap<&'a str, bool>) -> bool {
    // OK: 'design' lifetime matches 'memo' key lifetime
    memo.insert(design, true);
}
```

**Key Concepts**:
- **Lifetime parameter** `<'a>`: Connects borrowed input to cache keys
- **Why needed**: HashMap stores references, must prove they stay valid
- **Propagation**: Lifetime flows through recursive calls automatically

**Exercise**:
1. Try removing `<'a>` from `design` parameter (see compiler error)
2. Try using `String` keys instead of `&str` (works but allocates)
3. Measure memory difference: `&str` vs `String` keys
4. Benchmark performance: zero-copy vs allocating

**Key Insight**: Lifetimes enable **zero-copy caching** - cache entries borrow from input without allocation.

---

### Step 4: Generic Cache Abstraction (Day 4) 🎨

**File**: `examples/step4_generic_cache.rs`

**Learning Goal**: Create reusable memoization patterns across different problem types.

**Generic Implementation**:
```rust
use std::collections::HashMap;
use std::hash::Hash;

/// Generic memoization cache
struct MemoCache<K, V> {
    cache: HashMap<K, V>,
    hits: usize,
    misses: usize,
}

impl<K: Hash + Eq, V: Clone> MemoCache<K, V> {
    fn new() -> Self {
        Self { cache: HashMap::new(), hits: 0, misses: 0 }
    }
    
    fn get_or_compute<F>(&mut self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(value) = self.cache.get(&key) {
            self.hits += 1;
            return value.clone();
        }
        
        self.misses += 1;
        let value = compute();
        self.cache.insert(key, value.clone());
        value
    }
    
    fn stats(&self) -> (usize, usize, f64) {
        let hit_rate = self.hits as f64 / (self.hits + self.misses) as f64;
        (self.hits, self.misses, hit_rate)
    }
}
```

**Usage**:
```rust
let mut cache = MemoCache::new();

fn solve(n: u64, cache: &mut MemoCache<u64, u64>) -> u64 {
    cache.get_or_compute(n, || {
        // Expensive computation here
        fibonacci_naive(n)
    })
}
```

**Exercise**:
1. Adapt generic cache for string DP problems
2. Add lifetime parameters for borrowed keys: `MemoCache<&'a str, V>`
3. Implement thread-safe variant with `Mutex`
4. Create macro for automatic memoization

**Key Insight**: **Abstraction enables reuse** - same pattern across many problems.

---

### Step 5: Boolean → Counting Transformation (Day 5) 🔢

**File**: `examples/step5_boolean_to_counting.rs`

**Learning Goal**: Understand how existence checks transform to exhaustive counting.

**Pattern Comparison**:

```rust
// Part 1: Does ANY path exist? (Boolean)
fn can_make<'a>(
    patterns: &[&str],
    design: &'a str,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if design.is_empty() { return true; }  // ← Success: found a way
    if let Some(&result) = memo.get(design) { return result; }
    
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            if can_make(patterns, rest, memo) {
                memo.insert(design, true);
                return true;  // ← Early exit on first success
            }
        }
    }
    memo.insert(design, false);
    false
}

// Part 2: How MANY paths exist? (Counting)
fn count_ways<'a>(
    patterns: &[&str],
    design: &'a str,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if design.is_empty() { return 1; }  // ← One way: do nothing
    if let Some(&count) = memo.get(design) { return count; }
    
    let mut total = 0;
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            total += count_ways(patterns, rest, memo);  // ← Accumulate all paths
        }
    }
    memo.insert(design, total);
    total
}
```

**Only 4 Changes**:
1. `HashMap<_, bool>` → `HashMap<_, u64>`
2. `return true` → `return 1`
3. Early-exit `if found { return true }` → Accumulate `total += count()`
4. `return false` → `return total`

**Exercise**:
1. Apply transformation to Fibonacci (count ways to reach N with steps 1 or 2)
2. Apply to grid paths (count routes from (0,0) to (m,n))
3. Apply to subset sum (count ways to make target)
4. Measure: does memoization help counting more than existence?

**Key Insight**: **Identical structure, different semantics** - DP patterns are composable.

---

### Step 6: Bottom-Up Tabulation (Day 6) 📊

**File**: `examples/step6_bottom_up_table.rs`

**Learning Goal**: Compare top-down (memoization) vs bottom-up (tabulation) approaches.

**Top-Down (Recursive)**:
```rust
// Start from target, recurse to base cases
fn fib_top_down(n: usize, memo: &mut HashMap<usize, u64>) -> u64 {
    if n <= 1 { return n as u64; }
    if let Some(&result) = memo.get(&n) { return result; }
    
    let result = fib_top_down(n - 1, memo) + fib_top_down(n - 2, memo);
    memo.insert(n, result);
    result
}
```

**Bottom-Up (Iterative)**:
```rust
// Build from base cases to target
fn fib_bottom_up(n: usize) -> u64 {
    if n <= 1 { return n as u64; }
    
    let mut table = vec![0; n + 1];
    table[0] = 0;
    table[1] = 1;
    
    for i in 2..=n {
        table[i] = table[i - 1] + table[i - 2];
    }
    
    table[n]
}
```

**Trade-offs**:

| Aspect | Top-Down (Memoization) | Bottom-Up (Tabulation) |
|--------|----------------------|----------------------|
| **Recursion** | Yes (call stack) | No (iteration) |
| **Memory** | Only reachable states | All states up to n |
| **Clarity** | Matches problem statement | Requires ordering |
| **Performance** | Function call overhead | Pure iteration |
| **Optimization** | Space = O(reachable) | Can reduce to O(1) often |

**Exercise**:
1. Implement both for coin change problem
2. Benchmark: which is faster for large inputs?
3. Optimize bottom-up to O(1) space (rolling array)
4. Identify problems where top-down is clearer

**Key Insight**: **Choose approach based on problem structure** - no universal best.

---

### Step 7: Real AoC Problems (Day 7) 🎄

**File**: `examples/step7_real_aoc_problems.rs`

**Learning Goal**: Apply all patterns to actual competitive programming problems.

**Problems Covered**:
1. **AoC 2024 Day 19** - Linen Layout (string pattern DP)
2. **AoC 2024 Day 11** - Stone Multiplication (numeric state DP)
3. **AoC 2015 Day 7** - Circuit Simulation (DAG with memo)

**Implementation Strategy**:
```rust
// Standard DP template for AoC
fn solve_aoc_problem(input: &str) -> (usize, u64) {
    let (patterns, designs) = parse_input(input);
    
    // Part 1: Boolean existence (count how many possible)
    let part1 = designs.iter()
        .filter(|d| can_make(&patterns, d))
        .count();
    
    // Part 2: Count all arrangements
    let part2 = designs.iter()
        .map(|d| count_ways(&patterns, d))
        .sum();
    
    (part1, part2)
}
```

**Exercise**:
1. Solve Day 19 using Mission 11 patterns
2. Measure cache statistics (hit rate, size)
3. Compare your solution to tutorial implementation
4. Identify 3 more AoC problems solvable with DP

**Key Insight**: **Pattern recognition is the skill** - implementation is mechanical once you see the DP structure.

---

## 🎯 Learning Milestones

### After Step 3 (Lifetime Safety)
✅ Understand lifetime parameters in recursive functions  
✅ Can explain why `<'a>` is needed for borrowed cache keys  
✅ Comfortable with compiler lifetime errors  

### After Step 5 (Boolean → Counting)
✅ Recognize when DP applies to a problem  
✅ Can transform existence to counting mechanically  
✅ Understand overlapping subproblems  

### After Step 7 (Real Problems)
✅ Solve AoC DP problems independently  
✅ Choose appropriate state representation  
✅ Implement memoization from scratch  
✅ Debug lifetime and borrowing issues  

---

## 📊 Practice Exercises

### Exercise 1: Fibonacci Variants
**File**: `exercises/exercise1_fibonacci.md`

Implement and compare:
1. Naive recursion (exponential)
2. Top-down with HashMap
3. Bottom-up with Vec
4. Space-optimized O(1) version

**Challenge**: Compute F(100) without overflow (use modular arithmetic).

---

### Exercise 2: Longest Common Subsequence
**File**: `exercises/exercise2_longest_subsequence.md`

**Problem**: Given two strings, find length of longest common subsequence.

**Example**: 
- `"ABCDEF"` and `"ADBEF"` → LCS = `"ABEF"` (length 4)

**State Design**:
- `(i, j)` = position in both strings
- Recurrence: `lcs(i, j) = max(lcs(i+1, j), lcs(i, j+1))` or `1 + lcs(i+1, j+1)` if match

**Implement**:
1. Top-down with HashMap<(usize, usize), usize>
2. Bottom-up with 2D table
3. Space-optimized O(min(m, n))

---

### Exercise 3: Coin Change
**File**: `exercises/exercise3_coin_change.md`

**Problem**: Minimum coins needed to make target amount.

**Example**: 
- Coins: `[1, 5, 10, 25]`, Target: `41`
- Answer: 4 coins (25 + 10 + 5 + 1)

**Two Variants**:
1. **Optimization**: Find minimum coins needed (unbounded knapsack)
2. **Counting**: Count number of ways to make target

**Challenge**: What if coins have limited quantity? (bounded knapsack)

---

## 🔗 Integration with Mission 11

This tutorial prepares you for **Mission 11 implementation**:

| Tutorial Step | Mission Requirement |
|---------------|-------------------|
| Step 1-2 | REQ-2: Top-Down DP Pattern |
| Step 3 | REQ-3: Zero-Copy String Caching |
| Step 4 | REQ-1: Generic Memoization Framework |
| Step 5 | REQ-4: Boolean → Counting Transformation |
| Step 6 | REQ-7: Bottom-Up Alternative |
| Step 7 | REQ-8: AoC Integration |

After completing this tutorial:
1. Return to **Mission 11** and implement production-grade versions
2. Add comprehensive test suite (50+ tests)
3. Create instrumentation (REQ-6)
4. Publish as reusable library

---

## 📚 Additional Resources

### Rust-Specific
- [Rust Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) - Official docs
- [HashMap Internals](https://doc.rust-lang.org/std/collections/struct.HashMap.html) - Performance characteristics
- [Recursion Limits](https://doc.rust-lang.org/reference/attributes/limits.html) - Stack overflow prevention

### Algorithm Theory
- [CLRS Chapter 15](https://mitpress.mit.edu/9780262046305/) - Dynamic Programming fundamentals
- [Competitive Programmer's Handbook](https://cses.fi/book/book.pdf) - DP section (Chapter 7)

### AoC Community
- [AoC Subreddit](https://www.reddit.com/r/adventofcode/) - Solution discussions
- [AoC Solutions Megathreads](https://adventofcode.com/) - Community approaches

---

## 🚀 Running the Tutorial

```bash
# Run each step individually
cargo run --example step1_naive_recursion
cargo run --example step2_manual_hashmap
cargo run --example step3_lifetime_safety
cargo run --example step4_generic_cache
cargo run --example step5_boolean_to_counting
cargo run --example step6_bottom_up_table
cargo run --example step7_real_aoc_problems

# Run all tests
cargo test -p mission11_tut

# Check progress
cargo clippy -p mission11_tut -- -D warnings
```

---

## 📈 Success Criteria

**Tutorial Complete When**:
- ✅ All 7 examples run successfully
- ✅ Can explain why memoization transforms complexity
- ✅ Comfortable with lifetime parameters in recursion
- ✅ Can implement both top-down and bottom-up DP
- ✅ Solved all 3 practice exercises
- ✅ Applied patterns to real AoC problem

**Ready for Mission 11 When**:
- ✅ Tutorial complete
- ✅ Understand generic abstractions (Step 4)
- ✅ Can debug lifetime errors independently
- ✅ Recognize DP patterns in new problems

---

**Next Steps**: After completing tutorial, proceed to **[Mission 11](../../missions/Mission11/)** for production implementation.

*Tags: #tutorial #dynamic-programming #memoization #lifetimes #step-by-step*

*Links: [[Mission 11]] | [[AoC 2024 Day 19]] | [[dynamic-programming-memoization]]*
