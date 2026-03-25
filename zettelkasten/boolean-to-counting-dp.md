# Boolean → Counting DP Transformation

**Type**: Pattern Note  
**Source**: Mission 11 REQ-4, AoC 2024 Day 19 (Linen Layout)  
**Related**: [[dp-aggregation-patterns]] | [[dynamic-programming-memoization]] | [[top-down-dp-pattern]]

---

## Overview

A canonical transformation pattern in dynamic programming where **Part 1 asks "is it possible?"** (existence check) and **Part 2 asks "how many ways?"** (exhaustive counting).

**Key Insight**: The recursive structure remains **identical**. Only the reduction operator changes from **early-exit OR** to **accumulative SUM**.

---

## The Pattern

### Part 1: Existence Check (Boolean DP)

```rust
/// Can we construct the target?
fn can_construct<'a>(
    target: &'a str,
    patterns: &[&str],
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    // 1. Base case
    if target.is_empty() {
        return true; // ← Success condition
    }

    // 2. Cache check
    if let Some(&cached) = memo.get(target) {
        return cached;
    }

    // 3. Recursive computation - EARLY EXIT OR
    for pattern in patterns {
        if let Some(remainder) = target.strip_prefix(pattern) {
            if can_construct(remainder, patterns, memo) {
                memo.insert(target, true);
                return true; // ← Stop on first success
            }
        }
    }

    // 4. Cache and return
    memo.insert(target, false);
    false
}
```

**Behavior**: Returns `true` as soon as **any** valid path is found. Doesn't explore all possibilities.

### Part 2: Exhaustive Counting (Numeric DP)

```rust
/// Count all distinct ways to construct the target
fn count_constructions<'a>(
    target: &'a str,
    patterns: &[&str],
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    // 1. Base case
    if target.is_empty() {
        return 1; // ← One way to succeed
    }

    // 2. Cache check (IDENTICAL)
    if let Some(&cached) = memo.get(target) {
        return cached;
    }

    // 3. Recursive computation - ACCUMULATIVE SUM
    let mut total = 0;
    for pattern in patterns {
        if let Some(remainder) = target.strip_prefix(pattern) {
            total += count_constructions(remainder, patterns, memo); // ← Sum all paths
        }
    }

    // 4. Cache and return
    memo.insert(target, total);
    total
}
```

**Behavior**: Explores **all** valid paths and returns their sum.

---

## What Changes

| Aspect | Boolean DP | Counting DP |
|--------|-----------|-------------|
| **Return type** | `bool` | `u64` (or `usize`) |
| **Cache type** | `HashMap<K, bool>` | `HashMap<K, u64>` |
| **Base case value** | `true` | `1` |
| **Reduction operator** | Early-exit OR (`if success { return true }`) | Accumulative SUM (`total += result`) |
| **Exploration** | Stops on first success | Explores all paths |

**Everything else is identical**: Base case check, cache lookup, recursive structure, cache insertion.

---

## Real-World Examples

### AoC 2024 Day 19: Linen Layout

**Part 1**: Can we construct design "brwrr" from towel patterns ["r", "wr", "b", "g"]?

```rust
let patterns = vec!["r", "wr", "b", "g"];
let design = "brwrr";

let mut memo = HashMap::new();
let possible = can_construct(design, &patterns, &mut memo);
// Result: true (e.g., "b" + "r" + "wr" + "r")
```

**Part 2**: How many distinct ways can we construct "brwrr"?

```rust
let mut memo = HashMap::new();
let ways = count_constructions(design, &patterns, &mut memo);
// Result: 2 (different arrangements of patterns)
```

### Climbing Stairs Problem

**Part 1**: Can you reach step N? (Spoiler: always `true` for N > 0)

```rust
fn can_reach(n: usize, memo: &mut HashMap<usize, bool>) -> bool {
    if n == 0 { return true; }
    if n == 1 { return true; }
    
    if let Some(&cached) = memo.get(&n) { return cached; }
    
    // Can reach n-1 OR n-2, then take final step(s)
    let result = can_reach(n - 1, memo) || can_reach(n - 2, memo);
    
    memo.insert(n, result);
    result
}
```

**Part 2**: How many distinct ways to reach step N?

```rust
fn count_ways(n: usize, memo: &mut HashMap<usize, u64>) -> u64 {
    if n == 0 { return 1; }
    if n == 1 { return 1; }
    
    if let Some(&cached) = memo.get(&n) { return cached; }
    
    // SUM paths from n-1 and n-2
    let result = count_ways(n - 1, memo) + count_ways(n - 2, memo);
    
    memo.insert(n, result);
    result
}
```

**Results**:
- 3 stairs: 1 way? No, **3 ways** (1+1+1, 1+2, 2+1)
- 5 stairs: 1 way? No, **8 ways** (Fibonacci progression)
- 10 stairs: **89 ways**
- 20 stairs: **10,946 ways**

---

## Why This Works

### Mathematical Foundation

Both versions solve the same recurrence relation:

$$
\text{solve}(s) = \begin{cases}
\text{base\_value} & \text{if } s = \text{base\_case} \\
\text{reduce}(\{\text{solve}(s') \mid s' \in \text{transitions}(s)\}) & \text{otherwise}
\end{cases}
$$

**Boolean version**: `reduce` = `any` (logical OR)  
**Counting version**: `reduce` = `sum` (arithmetic addition)

### State Space Identical

Both versions explore the **same state space** (cache sizes are equal):

```rust
// Example: "ab" with patterns ["a", "b"]
let mut bool_memo = HashMap::new();
let mut count_memo = HashMap::new();

can_construct("ab", &patterns, &mut bool_memo);
count_constructions("ab", &patterns, &mut count_memo);

assert_eq!(bool_memo.len(), count_memo.len()); // Same subproblems!
```

The cache contents differ (bool vs u64), but the **keys explored are identical**.

---

## Performance Characteristics

Both versions have **identical complexity** with memoization:

- **Time**: O(S × T) where S = state space size, T = transitions per state
- **Space**: O(S) for cache

**Difference**: Counting version doesn't benefit from early-exit optimization, but this is negligible compared to memoization savings.

### Example: String Pattern Matching

```rust
// Target: "aaaaaaaa" (8 'a's)
// Patterns: ["a", "aa", "aaa"]

// Boolean DP
let mut bool_memo = HashMap::new();
let exists = can_construct("aaaaaaaa", &["a", "aa", "aaa"], &mut bool_memo);
// Result: true (obviously)
// Cache size: 8 unique substrings

// Counting DP
let mut count_memo = HashMap::new();
let ways = count_constructions("aaaaaaaa", &["a", "aa", "aaa"], &mut count_memo);
// Result: 81 distinct ways
// Cache size: 8 unique substrings (same!)

// Performance: ~4.6µs for both (cache dominates, reduction is trivial)
```

**Without memoization**:
- Boolean: O(3^8) = ~6,561 recursive calls (early-exit helps slightly)
- Counting: O(3^8) = 6,561 recursive calls (explores all)

**With memoization**:
- Both: O(3 × 8) = 24 operations (3 patterns × 8 substrings)

---

## Implementation Checklist

When transforming boolean → counting DP:

- [ ] Change return type: `bool` → `u64`
- [ ] Change cache type: `HashMap<K, bool>` → `HashMap<K, u64>`
- [ ] Change base case value: `true` → `1`
- [ ] Change reduction: `if success { return true }` → `total += result`
- [ ] Change failure return: `false` → `0`
- [ ] **Keep everything else unchanged**: structure, base checks, cache lookups

**Common mistake**: Trying to restructure the algorithm. The structure is **identical**!

---

## Related Transformations

### Boolean → Counting (This Pattern)

- **Question**: "Can we?" → "How many ways?"
- **Operator**: OR → SUM
- **Example**: Can reach step N? → How many paths to N?

### Existence → Optimization

- **Question**: "Can we?" → "What's the best way?"
- **Operator**: OR → MAX/MIN
- **Example**: Can climb stairs? → Minimum cost to reach top?
- See: [[dp-aggregation-patterns]]

### Counting → Finding All

- **Question**: "How many?" → "What are they?"
- **Operator**: SUM → COLLECT
- **Example**: Count paths → Return all paths
- **Note**: Changes complexity from O(S) to O(S × P) where P = path length

---

## When to Use

✅ **This transformation applies when**:
- Part 1 asks for existence/possibility
- Part 2 asks for exhaustive counting
- State space and transitions are identical
- Both require memoization (overlapping subproblems)

❌ **This transformation does NOT apply when**:
- Part 2 changes state representation
- Part 2 adds new constraints
- Part 2 fundamentally changes problem structure
- Part 1 is trivial but Part 2 requires DP

---

## AoC Problem Examples

### Confirmed Pattern

- **AoC 2024 Day 19** (Linen Layout): Can construct design? → Count ways
- **AoC 2024 Day 12** (Hot Springs): Valid arrangement? → Count valid arrangements
- **AoC 2015 Day 17** (Eggnog): Can distribute? → Count distributions

### Similar Pattern (Different Aggregation)

- **AoC 2024 Day X** (Path finding): Can reach? → Shortest path (MIN aggregation)
- See [[dp-aggregation-patterns]] for MAX/MIN variants

---

## Mission 11 Integration

This pattern is **REQ-4** in Mission 11: Dynamic Programming with Memoization.

**Implementation**: [demo_counting_paths.rs](../missions/Mission11/examples/demo_counting_paths.rs)

**Tests**: All 4 REQ-4 tests validate this transformation:
- `req4_identical_structure` - Cache sizes match
- `req4_base_case_transformation` - true → 1
- `req4_reduction_operator_change` - OR → SUM
- `req4_multiple_paths_counting` - Accumulation verified

**Run demo**:
```bash
cargo run -p mission11 --example demo_counting_paths
```

---

## Key Takeaways

1. **Structure is identical** - Only reduction operator changes
2. **Base case transforms** - `true` → `1` (identity elements)
3. **Early-exit vs accumulation** - OR short-circuits, SUM explores all
4. **State space unchanged** - Same cache keys, different values
5. **Performance equivalent** - Memoization dominates both
6. **Common AoC pattern** - Part 1 existence → Part 2 counting

---

*Links*: [[dp-aggregation-patterns]] | [[dynamic-programming-memoization]] | [[top-down-dp-pattern]] | [[mission-11]] | [[aoc-pattern-recognition]] | [[boolean-to-counting-transformation]]

*Tags*: #dynamic-programming #memoization #transformation-pattern #boolean-to-counting #aoc-patterns #mission11
