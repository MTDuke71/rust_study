# 🔄 Top-Down DP Pattern (Memoization Template)

**The canonical recursive dynamic programming structure**

**Tags:** #dynamic-programming #memoization #recursion #algorithm-pattern #mission-11

**Related:** [[Dynamic Programming]], [[memoization-comprehensive-guide]], [[Recursion]], [[HashMap]], [[zero-copy-string-slicing]], [[mission-11]]

---

## 🎯 Core Concept

The **Top-Down DP Pattern** is a standardized 4-step recursive template that transforms naive exponential recursion into efficient polynomial-time algorithms through memoization. This pattern is the foundation for solving overlapping subproblems.

**Key Insight:** Every top-down DP solution follows the same structure — only the base cases, state transitions, and result types change.

---

## 📋 The Canonical Pattern

```rust
fn solve<'a>(
    state: &'a State,
    memo: &mut HashMap<&'a State, Result>,
) -> Result {
    // ✅ STEP 1: Base case check
    if is_base_case(state) {
        return base_value();
    }
    
    // ✅ STEP 2: Memoization check
    if let Some(&cached) = memo.get(state) {
        return cached;
    }
    
    // ✅ STEP 3: Recursive computation
    let result = compute_from_subproblems(state, memo);
    
    // ✅ STEP 4: Cache and return
    memo.insert(state, result);
    result
}
```

**Critical Order:** Steps must execute in this exact sequence for correctness and performance.

---

## 🔍 Step-by-Step Breakdown

### **Step 1: Base Case Check**

```rust
// Base case: empty string can always be constructed
if target.is_empty() {
    return true;  // Or 1, or "", depending on problem
}
```

**Purpose:**
- Stops infinite recursion
- Defines trivial problem solutions
- NO caching needed (immediate return)

**Common Base Cases:**
- Empty state: `str.is_empty()`, `n == 0`, `pos >= len`
- Boundary reached: `i == j`, `left > right`
- Success/failure state: `target_found`, `impossible_state`

### **Step 2: Memoization Check**

```rust
if let Some(&cached) = memo.get(state) {
    return cached;  // Avoid recomputation
}
```

**Purpose:**
- Transforms exponential → polynomial complexity
- Returns **reference** to cached value (no clone if possible)
- **Massive performance gain** on overlapping subproblems

**Cache Hit Example:**
```
fib(5) calls fib(3) twice
  First call: Cache miss → compute
  Second call: Cache HIT → instant return
```

### **Step 3: Recursive Computation**

```rust
// Example: String pattern matching
let mut found = false;
for pattern in patterns {
    if let Some(remainder) = target.strip_prefix(pattern) {
        if solve(remainder, patterns, memo) {
            found = true;
            break;  // Early exit for boolean DP
        }
    }
}
```

**Purpose:**
- Breaks problem into smaller subproblems
- Recursively solves subproblems (which may hit cache)
- Combines subproblem results via reduction operator

**Reduction Operators:**
- Boolean: Early-exit **OR** (`if any { return true }`)
- Counting: Accumulative **SUM** (`total += subproblem`)
- Optimization: **MIN/MAX** (`result.min(subproblem)`)

### **Step 4: Cache and Return**

```rust
memo.insert(state, result);
result
```

**Purpose:**
- Stores computed result for future cache hits
- **Must happen BEFORE return** (not after!)
- Future calls to same state will hit cache (Step 2)

**Critical:** Insert before return ensures all states are cached, even in early-exit scenarios.

---

## 🎓 Pattern Variations

### **Variation 1: Boolean DP (Existence Check)**

```rust
/// Can we construct target from patterns?
fn can_construct<'a>(
    target: &'a str,
    patterns: &[&str],
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    // Step 1: Base case
    if target.is_empty() { return true; }
    
    // Step 2: Cache check
    if let Some(&cached) = memo.get(target) { return cached; }
    
    // Step 3: Try all patterns (early-exit OR)
    for pattern in patterns {
        if let Some(remainder) = target.strip_prefix(pattern) {
            if can_construct(remainder, patterns, memo) {
                memo.insert(target, true);  // Step 4: Cache
                return true;  // Early exit on first success
            }
        }
    }
    
    // Step 4: Cache negative result
    memo.insert(target, false);
    false
}
```

**Use Case:** "Is there ANY way to solve this?"

### **Variation 2: Counting DP (Exhaustive Enumeration)**

```rust
/// How many ways can we construct target?
fn count_constructions<'a>(
    target: &'a str,
    patterns: &[&str],
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    // Step 1: Base case (one way to complete)
    if target.is_empty() { return 1; }
    
    // Step 2: Cache check
    if let Some(&cached) = memo.get(target) { return cached; }
    
    // Step 3: Accumulate ALL paths (SUM)
    let mut total = 0;
    for pattern in patterns {
        if let Some(remainder) = target.strip_prefix(pattern) {
            total += count_constructions(remainder, patterns, memo);
        }
    }
    
    // Step 4: Cache total
    memo.insert(target, total);
    total
}
```

**Use Case:** "How many DIFFERENT ways to solve this?"

**Transformation:** Only Step 3 changes (OR → SUM), structure identical!

### **Variation 3: Optimization DP (Best Solution)**

```rust
/// Minimum coins needed to make amount
fn min_coins(
    amount: u32,
    coins: &[u32],
    memo: &mut HashMap<u32, Option<u32>>,
) -> Option<u32> {
    // Step 1: Base case
    if amount == 0 { return Some(0); }
    
    // Step 2: Cache check
    if let Some(&cached) = memo.get(&amount) { return cached; }
    
    // Step 3: Find minimum across all choices (MIN)
    let mut best = None;
    for &coin in coins {
        if coin <= amount {
            if let Some(sub) = min_coins(amount - coin, coins, memo) {
                best = Some(best.map_or(sub + 1, |b| b.min(sub + 1)));
            }
        }
    }
    
    // Step 4: Cache best
    memo.insert(amount, best);
    best
}
```

**Use Case:** "What's the OPTIMAL solution?"

---

## ⚡ Performance Transformation

### **Without Memoization (Naive Recursion)**

```rust
fn fib_naive(n: u64) -> u64 {
    if n <= 1 { return n; }
    fib_naive(n - 1) + fib_naive(n - 2)  // Exponential recomputation!
}
```

**Complexity:** O(2^n) — **Exponential explosion**

**Example:** `fib(20)` computes `fib(5)` **10,946 times**!

### **With Memoization (Top-Down DP)**

```rust
fn fib_memo(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 { return n; }
    if let Some(&cached) = memo.get(&n) { return cached; }
    
    let result = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);
    memo.insert(n, result);
    result
}
```

**Complexity:** O(n) — **Linear performance**

**Example:** `fib(20)` computes each value **exactly once** (20 cache entries)

**Speedup:** ~1 million× faster for `fib(30)`

---

## 🎯 When to Use This Pattern

### **✅ Use Top-Down DP When:**

1. **Overlapping Subproblems** - Same subproblems computed multiple times
2. **Optimal Substructure** - Solution contains optimal subsolutions
3. **Natural Recursion** - Problem is naturally expressed recursively
4. **Unknown State Space** - Don't know which states are reachable
5. **Easier Reasoning** - Top-down matches problem decomposition

**Ideal For:** Pathfinding, string matching, combinatorial counting, optimization

### **❌ Consider Bottom-Up Instead When:**

1. **Predictable State Space** - Know all states upfront
2. **Stack Overflow Risk** - Deep recursion (>1000 levels)
3. **Memory Constraints** - Want space optimization (rolling arrays)
4. **Iterative Preference** - Team prefers loops over recursion

---

## 🔧 Rust-Specific Considerations

### **Lifetime Management**

```rust
// ✅ Correct: Lifetime 'a ties cache keys to input
fn solve<'a>(
    input: &'a str,
    memo: &mut HashMap<&'a str, Result>,
) -> Result
```

**Why:** Cache keys (`&'a str`) must remain valid as long as memo exists. Lifetime parameter `'a` enforces this at compile time.

### **Clone vs Reference Returns**

```rust
// If V: Copy (e.g., bool, u64, i32)
if let Some(&cached) = memo.get(state) {
    return cached;  // ✅ Copy value (cheap)
}

// If V: Clone but not Copy (e.g., String, Vec)
if let Some(cached) = memo.get(state) {
    return cached.clone();  // ⚠️ Allocates (may be expensive)
}
```

### **State Type Requirements**

```rust
// State must implement Hash + Eq
#[derive(Hash, Eq, PartialEq)]
struct State {
    pos: usize,
    count: u32,
}

// Or use primitive tuples
type State = (usize, usize, bool);  // Automatically Hash + Eq
```

---

## 📊 Comparison: Top-Down vs Bottom-Up

| Aspect | Top-Down (Memoization) | Bottom-Up (Tabulation) |
|--------|------------------------|------------------------|
| **Direction** | Recursive (problem → subproblems) | Iterative (base → goal) |
| **Cache** | HashMap (on-demand) | Array/Vec (pre-allocated) |
| **States Computed** | Only reachable states | All possible states |
| **Stack Usage** | O(depth) recursion | O(1) iteration |
| **Implementation** | Matches problem decomposition | Requires dependency ordering |
| **Debugging** | Harder (recursion traces) | Easier (step-through loops) |
| **Space Optimization** | Difficult | Easy (rolling arrays) |

**Rule of Thumb:** Start with top-down (easier), optimize to bottom-up if needed.

---

## 🎓 Learning Resources

### **Mission 11 Implementation**

See: [missions/Mission11/src/string_dp.rs](../../../missions/Mission11/src/string_dp.rs)

**Examples:**
- `can_construct()` - Boolean DP pattern
- `count_constructions()` - Counting DP pattern  
- `can_construct_multi()` - Composite state pattern

**Tests:** 31 comprehensive tests validating pattern correctness

### **Real-World Applications (AoC)**

- **AoC 2024 Day 19** (Linen Layout) - String pattern matching
- **AoC 2023 Day 12** (Hot Springs) - Constraint satisfaction DP
- **AoC 2024 Day 11** (Stone Multiplication) - Numeric state caching

### **Practice Problems**

1. **Fibonacci** - Classic intro (1D state)
2. **Climbing Stairs** - Similar to Fibonacci
3. **Coin Change** - Unbounded knapsack variant
4. **Word Break** - String pattern matching (Mission 11 pattern)
5. **Longest Palindromic Substring** - Interval DP

---

## ⚠️ Common Pitfalls

### **❌ Pitfall 1: Caching BEFORE Computation**

```rust
// ❌ WRONG ORDER
memo.insert(state, result);  // Caching uninitialized value!
let result = compute_subproblems(state, memo);
```

**Fix:** Always compute first, then cache.

### **❌ Pitfall 2: Forgetting to Cache Negative Results**

```rust
// ❌ Only caches successes
if found_solution {
    memo.insert(state, true);
    return true;
}
return false;  // Not cached! Will recompute on next call
```

**Fix:** Cache **both** positive and negative results.

### **❌ Pitfall 3: Not Checking Cache Before Recursion**

```rust
// ❌ Computes before checking cache
let result = expensive_computation(state, memo);
if let Some(&cached) = memo.get(state) { return cached; }  // Too late!
```

**Fix:** Always check cache **immediately** after base case.

### **❌ Pitfall 4: Mutating State After Caching**

```rust
// ❌ State changes after being used as key
memo.insert(state, result);
state.count += 1;  // Now cache lookup will fail!
```

**Fix:** Don't mutate states that are cache keys.

---

## 🔗 Related Concepts

- [[Dynamic Programming]] - General DP overview
- [[memoization-comprehensive-guide]] - Memoization theory and patterns
- [[Recursion]] - Foundation for top-down approach
- [[HashMap]] - Cache data structure
- [[zero-copy-string-slicing]] - Performance optimization for string DP
- [[mission-11]] - Complete V-Cycle implementation

---

## 📝 Quick Reference Card

```rust
// ═══════════════════════════════════════════════════
// TOP-DOWN DP PATTERN - Copy-Paste Template
// ═══════════════════════════════════════════════════

fn solve<'a>(
    state: &'a State,                    // Problem state
    context: &Context,                   // Immutable data
    memo: &mut HashMap<&'a State, T>,    // Memoization cache
) -> T {
    // 1️⃣ BASE CASE - Trivial solution
    if is_base_case(state) {
        return base_value();
    }
    
    // 2️⃣ CACHE CHECK - Avoid recomputation
    if let Some(&cached) = memo.get(state) {
        return cached;
    }
    
    // 3️⃣ RECURSIVE COMPUTATION - Solve subproblems
    let result = match problem_type {
        Boolean => early_exit_or_logic(state, context, memo),
        Counting => accumulate_sum_logic(state, context, memo),
        Optimization => find_min_max_logic(state, context, memo),
    };
    
    // 4️⃣ CACHE & RETURN - Store for future use
    memo.insert(state, result);
    result
}
```

---

**Key Takeaway:** The top-down DP pattern is a **mechanical transformation** — write naive recursion, add 3 lines of memoization code, gain exponential speedup. Master this template and you can solve 80% of DP problems.

*Links:*
- **Outgoing:** [[Dynamic Programming]], [[memoization-comprehensive-guide]], [[Recursion]], [[HashMap]], [[zero-copy-string-slicing]], [[mission-11]]
- **Incoming:** (To be added by related notes)
