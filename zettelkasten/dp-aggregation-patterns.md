# DP Aggregation Patterns - Reduction Operators in Dynamic Programming

**Type**: Pattern Note  
**Source**: Mission 11, AoC problem analysis  
**Related**: [[boolean-to-counting-dp]] | [[dynamic-programming-memoization]] | [[top-down-dp-pattern]]

---

## Overview

Dynamic programming solutions differ primarily in how they **aggregate results from subproblems**. The recursive structure is often identical - only the **reduction operator** changes.

**Core Insight**: Same state space, same transitions, different aggregation = different problem variant.

---

## The Generic DP Pattern

```rust
fn solve<'a>(
    state: &'a State,
    memo: &mut HashMap<&'a State, Result>,
) -> Result {
    // 1. Base case
    if is_base_case(state) {
        return base_value(); // ← Depends on aggregation
    }

    // 2. Cache check
    if let Some(&cached) = memo.get(state) {
        return cached;
    }

    // 3. Recursive computation with aggregation
    let result = AGGREGATE {  // ← This is what varies!
        solve(next_state, memo) for next_state in transitions(state)
    };

    // 4. Cache and return
    memo.insert(state, result);
    result
}
```

---

## Common Aggregation Operators

| Operator | Type | Base Value | Use Case | Example |
|----------|------|------------|----------|---------|
| **OR** (any) | `bool` | `true` | Existence check | Can reach destination? |
| **AND** (all) | `bool` | `true` | Validation | All paths valid? |
| **SUM** | `u64` | `1` | Counting | How many ways? |
| **MAX** | `i64` | `i64::MIN` | Optimization (maximize) | Longest path? |
| **MIN** | `u64` | `u64::MAX` | Optimization (minimize) | Shortest path? |
| **COLLECT** | `Vec<T>` | `vec![]` | Enumeration | Find all solutions |
| **PRODUCT** | `u64` | `1` | Combinations | Total configurations |

---

## Pattern 1: OR (Existence Check)

**Question**: "Is it possible?"

```rust
fn can_reach<'a>(
    state: &'a State,
    goal: &State,
    memo: &mut HashMap<&'a State, bool>,
) -> bool {
    // Base case
    if state == goal {
        return true; // ← Found solution
    }

    // Cache check
    if let Some(&cached) = memo.get(state) {
        return cached;
    }

    // Recursive OR aggregation
    for next_state in transitions(state) {
        if can_reach(next_state, goal, memo) {
            memo.insert(state, true);
            return true; // ← Early exit on first success
        }
    }

    // No path found
    memo.insert(state, false);
    false
}
```

**Characteristics**:
- **Early exit**: Returns immediately on first success
- **Base value**: `true` (success) or `false` (failure)
- **Short-circuits**: Doesn't explore all paths

**Example**: Can construct "brwrr" from patterns ["r", "wr", "b"]?

---

## Pattern 2: SUM (Counting)

**Question**: "How many ways?"

```rust
fn count_ways<'a>(
    state: &'a State,
    goal: &State,
    memo: &mut HashMap<&'a State, u64>,
) -> u64 {
    // Base case
    if state == goal {
        return 1; // ← One way to reach goal
    }

    // Cache check
    if let Some(&cached) = memo.get(state) {
        return cached;
    }

    // Recursive SUM aggregation
    let mut total = 0;
    for next_state in transitions(state) {
        total += count_ways(next_state, goal, memo); // ← Accumulate all paths
    }

    // Return total
    memo.insert(state, total);
    total
}
```

**Characteristics**:
- **No early exit**: Explores all paths
- **Base value**: `1` (one successful completion)
- **Accumulation**: Sums all valid paths

**Example**: How many ways to construct "ab" from ["a", "b", "ab"]? (Answer: 2)

See: [[boolean-to-counting-dp]] for detailed transformation.

---

## Pattern 3: MAX (Optimization - Maximize)

**Question**: "What's the best/longest/maximum?"

```rust
fn longest_path<'a>(
    state: &'a State,
    memo: &mut HashMap<&'a State, i64>,
) -> i64 {
    // Base case
    if is_terminal(state) {
        return 0; // ← No more length to add
    }

    // Cache check
    if let Some(&cached) = memo.get(state) {
        return cached;
    }

    // Recursive MAX aggregation
    let mut best = i64::MIN; // ← Start with impossible value
    for next_state in transitions(state) {
        let path_len = 1 + longest_path(next_state, memo); // ← Current edge + subproblem
        best = best.max(path_len); // ← Keep maximum
    }

    memo.insert(state, best);
    best
}
```

**Characteristics**:
- **Base value**: `i64::MIN` (impossible), or `0` at terminal state
- **Comparison**: `best.max(candidate)`
- **Explores all**: Must check every path to find maximum

**Example**: Longest increasing subsequence, maximum profit path

---

## Pattern 4: MIN (Optimization - Minimize)

**Question**: "What's the cheapest/shortest/minimum?"

```rust
fn shortest_path<'a>(
    state: &'a State,
    goal: &State,
    memo: &mut HashMap<&'a State, u64>,
) -> u64 {
    // Base case
    if state == goal {
        return 0; // ← No cost to stay at goal
    }

    // Cache check
    if let Some(&cached) = memo.get(state) {
        return cached;
    }

    // Recursive MIN aggregation
    let mut best = u64::MAX; // ← Start with impossible value
    for (next_state, cost) in transitions_with_cost(state) {
        let total_cost = cost + shortest_path(next_state, goal, memo);
        best = best.min(total_cost); // ← Keep minimum
    }

    memo.insert(state, best);
    best
}
```

**Characteristics**:
- **Base value**: `u64::MAX` (impossible), or `0` at goal
- **Comparison**: `best.min(candidate)`
- **Alternative**: Dijkstra's algorithm for weighted graphs

**Example**: Minimum edit distance, cheapest flight route

---

## Pattern 5: AND (Universal Validation)

**Question**: "Are all paths valid?"

```rust
fn all_paths_valid<'a>(
    state: &'a State,
    memo: &mut HashMap<&'a State, bool>,
) -> bool {
    // Base case
    if is_terminal(state) {
        return validate(state); // ← Check terminal validity
    }

    // Cache check
    if let Some(&cached) = memo.get(state) {
        return cached;
    }

    // Recursive AND aggregation
    for next_state in transitions(state) {
        if !all_paths_valid(next_state, memo) {
            memo.insert(state, false);
            return false; // ← Early exit on first failure
        }
    }

    // All paths valid
    memo.insert(state, true);
    true
}
```

**Characteristics**:
- **Early exit**: Returns immediately on first failure
- **Base value**: `true` (valid) at terminal states
- **Dual of OR**: OR finds any success, AND requires all successes

**Example**: Validate all branches of decision tree, check game winning states

---

## Pattern 6: COLLECT (Enumeration)

**Question**: "What are all the solutions?"

```rust
fn find_all_paths<'a>(
    state: &'a State,
    goal: &State,
    current_path: &[&'a State],
    memo: &mut HashMap<&'a State, Vec<Vec<&'a State>>>,
) -> Vec<Vec<&'a State>> {
    // Base case
    if state == goal {
        let mut path = current_path.to_vec();
        path.push(state);
        return vec![path]; // ← One complete path
    }

    // Cache (less effective for path enumeration!)
    if let Some(cached) = memo.get(state) {
        return cached.clone(); // ⚠️ Expensive!
    }

    // Recursive COLLECT aggregation
    let mut all_paths = Vec::new();
    let mut new_path = current_path.to_vec();
    new_path.push(state);

    for next_state in transitions(state) {
        let paths_from_here = find_all_paths(next_state, goal, &new_path, memo);
        all_paths.extend(paths_from_here); // ← Collect all paths
    }

    memo.insert(state, all_paths.clone());
    all_paths
}
```

**Characteristics**:
- **Memory intensive**: O(S × P) where P = path length
- **Less effective caching**: Paths are large, cloning is expensive
- **Alternative**: Generate paths lazily with iterators

**Example**: Find all valid permutations, enumerate all configurations

⚠️ **Warning**: COLLECT pattern has much higher complexity than other aggregations. Consider if you really need all solutions or just the count.

---

## Choosing the Right Aggregation

### Decision Tree

```
What's your question?
├─ "Is it possible?" → OR
├─ "How many ways?" → SUM
├─ "What's the best?"
│  ├─ "Maximum/Longest?" → MAX
│  └─ "Minimum/Shortest?" → MIN
├─ "Are all valid?" → AND
└─ "What are they?" → COLLECT (expensive!)
```

### Complexity Comparison

| Pattern | Time | Space | Cache Effectiveness |
|---------|------|-------|---------------------|
| OR | O(S × T) | O(S) | ⭐⭐⭐ Excellent (early exit) |
| SUM | O(S × T) | O(S) | ⭐⭐⭐ Excellent |
| MAX | O(S × T) | O(S) | ⭐⭐⭐ Excellent |
| MIN | O(S × T) | O(S) | ⭐⭐⭐ Excellent |
| AND | O(S × T) | O(S) | ⭐⭐⭐ Excellent (early exit) |
| COLLECT | O(S × T × P) | O(S × P) | ⭐ Poor (path storage) |

S = state space, T = transitions per state, P = average path length

---

## Real-World Examples

### Climbing Stairs (All Aggregations)

```rust
// OR: Can you reach step N? (Always true for N > 0)
fn can_climb(n: usize) -> bool { /* ... */ true }

// SUM: How many distinct ways?
fn count_ways(n: usize) -> u64 { /* Fibonacci */ }

// MIN: Minimum number of jumps? (If jumps have different costs)
fn min_cost(n: usize, costs: &[u64]) -> u64 { /* ... */ }

// MAX: Maximum points collecting coins on stairs?
fn max_points(n: usize, coins: &[u64]) -> u64 { /* ... */ }

// COLLECT: What are all the distinct paths?
fn all_paths(n: usize) -> Vec<Vec<usize>> { /* Expensive! */ }
```

### AoC 2024 Day 19: Linen Layout

**Part 1 (OR)**: Can we construct design from towel patterns?
```rust
can_construct("brwrr", &["r", "wr", "b"]) // → true
```

**Part 2 (SUM)**: How many ways to construct it?
```rust
count_constructions("brwrr", &["r", "wr", "b"]) // → 2
```

**Hypothetical Part 3 (MAX)**: Longest pattern sequence?
```rust
longest_pattern_sequence("brwrr", &["r", "wr", "b"]) // → 5 (b+r+w+r+r)
```

---

## Transformation Table

| From | To | Change | Example |
|------|-----|--------|---------|
| OR → SUM | `bool` → `u64`, `true` → `1`, early-exit → accumulate | [[boolean-to-counting-dp]] |
| OR → MAX | `bool` → `i64`, `true` → `0`, `if success` → `max(best, result)` | Possible → Longest |
| SUM → MAX | Keep `u64`, change `total +=` → `best = max(best, ...)` | Count ways → Best way |
| MIN → MAX | Flip comparison: `min` → `max`, `u64::MAX` → `i64::MIN` | Shortest → Longest |

---

## Implementation Tips

### 1. Choose Base Value Carefully

```rust
// OR: true = success, false = failure
if at_goal { return true; }

// SUM: 1 = one way to complete
if at_goal { return 1; }

// MAX: 0 = no additional value, or i64::MIN = impossible
if at_goal { return 0; }

// MIN: 0 = no cost, or u64::MAX = impossible
if at_goal { return 0; }
```

### 2. Initialize Aggregator Correctly

```rust
// OR: Start with false, update to true
let mut found = false;

// SUM: Start with 0, accumulate
let mut total = 0;

// MAX: Start with worst possible (negative infinity or 0)
let mut best = i64::MIN; // or 0 if values are non-negative

// MIN: Start with worst possible (positive infinity)
let mut best = u64::MAX;
```

### 3. Update Pattern

```rust
// OR: Early exit
if condition { return true; }

// SUM: Accumulate
total += recursive_result;

// MAX: Keep maximum
best = best.max(recursive_result);

// MIN: Keep minimum
best = best.min(recursive_result);
```

---

## Anti-Patterns

### ❌ Mixing Aggregations Incorrectly

```rust
// BAD: Trying to count AND optimize simultaneously
fn bad_dp(state: &State) -> (u64, i64) {
    // Returns (count, max_value)
    // This couples two concerns - separate them!
}

// GOOD: Separate functions
fn count_ways(state: &State) -> u64 { /* ... */ }
fn max_value(state: &State) -> i64 { /* ... */ }
```

### ❌ Wrong Base Value

```rust
// BAD: SUM with base value 0
if at_goal { return 0; } // Wrong! Counts nothing

// GOOD: SUM with base value 1
if at_goal { return 1; } // Correct! One way completed
```

### ❌ Forgetting to Cache in COLLECT

```rust
// BAD: No caching in COLLECT (still expensive, but at least correct)
fn find_all(state: &State) -> Vec<Path> {
    if at_goal { return vec![Path::new()]; }
    
    // No memo - recomputes everything!
    let mut paths = vec![];
    for next in transitions(state) {
        paths.extend(find_all(next));
    }
    paths
}
```

---

## Key Takeaways

1. **Same structure, different operators** - DP patterns differ mainly in aggregation
2. **Base values matter** - Must match aggregation (1 for SUM, true for OR, etc.)
3. **Complexity is similar** - All O(S×T) except COLLECT which is O(S×T×P)
4. **Early exit when possible** - OR and AND can short-circuit
5. **Transformation is straightforward** - Change operator, base value, and type

---

*Links*: [[boolean-to-counting-dp]] | [[dynamic-programming-memoization]] | [[top-down-dp-pattern]] | [[mission-11]] | [[aoc-pattern-recognition]]

*Tags*: #dynamic-programming #aggregation-patterns #reduction-operators #or-sum-max-min #optimization #counting #memoization #mission11
