# Memoization Patterns (AoC 2024)

**Created**: 2025-12-21  
**Context**: Advent of Code 2024 problem-solving patterns

## Overview

**Memoization** is an optimization technique that caches the results of expensive function calls and returns the cached result when the same inputs occur again. In AoC 2024, memoization transformed exponential-time algorithms into polynomial or linear-time solutions across multiple problems.

**Core Pattern**:
```rust
use std::collections::HashMap;

fn solve_with_memo(input: Input, memo: &mut HashMap<Key, Result>) -> Result {
    // 1. Check cache first
    if let Some(&cached) = memo.get(&key) {
        return cached;  // Cache hit - avoid recomputation
    }
    
    // 2. Compute result (may recurse with memo)
    let result = expensive_computation(input, memo);
    
    // 3. Store in cache before returning
    memo.insert(key, result);
    result
}
```

## AoC 2024 Applications

### Day 11: Plutonian Pebbles (Stone Evolution)

**Problem**: Stones transform according to rules, count stones after N blinks  
**Naive approach**: Simulate transformations → O(2^n) exponential growth  
**Memoized approach**: Cache (stone_value, remaining_blinks) → O(unique_stones × blinks)

**Key Pattern**: **Count-based memoization**
```rust
fn count_stones(stone: u64, blinks: usize, memo: &mut HashMap<(u64, usize), u64>) -> u64 {
    // Base case
    if blinks == 0 {
        return 1;
    }
    
    // Memoization check
    let key = (stone, blinks);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }
    
    // Recursive computation with transformation rules
    let count = match stone {
        0 => count_stones(1, blinks - 1, memo),
        n if has_even_digits(n) => {
            let (left, right) = split_digits(n);
            count_stones(left, blinks - 1, memo) + count_stones(right, blinks - 1, memo)
        }
        n => count_stones(n * 2024, blinks - 1, memo),
    };
    
    memo.insert(key, count);
    count
}
```

**Transformation**: O(2^75) → O(~1000 unique states)  
**Insight**: Don't materialize the list - track counts only!

---

### Day 19: Linen Layout (Towel Pattern Matching)

**Problem**: Count ways to construct target pattern from available towel patterns  
**Naive approach**: Recursive backtracking → O(patterns^length) exponential  
**Memoized approach**: Cache substring results → O(target_length × patterns)

**Key Pattern**: **Substring memoization**
```rust
fn count_ways(target: &str, patterns: &[String], memo: &mut HashMap<String, usize>) -> usize {
    // Base case
    if target.is_empty() {
        return 1;  // Successfully constructed
    }
    
    // Memoization check
    if let Some(&cached) = memo.get(target) {
        return cached;
    }
    
    // Try all patterns that could start this substring
    let ways = patterns
        .iter()
        .filter(|p| target.starts_with(p.as_str()))
        .map(|p| count_ways(&target[p.len()..], patterns, memo))
        .sum();
    
    memo.insert(target.to_string(), ways);
    ways
}
```

**Transformation**: O(patterns^n) → O(n × patterns) where n = target length  
**Insight**: Suffix-based recursion naturally creates overlapping subproblems

---

### Day 21: Keypad Conundrum (Recursive Sequence Expansion)

**Problem**: Find shortest sequence length through multiple layers of directional keypads  
**Naive approach**: Expand full sequences → O(k^26) where k = sequence growth per layer  
**Memoized approach**: Cache (from, to, depth) → O(keys² × depth)

**Key Pattern**: **Composite key memoization**
```rust
fn min_sequence_length(
    from: char,
    to: char,
    depth: usize,
    memo: &mut HashMap<(char, char, usize), usize>
) -> usize {
    // Base case
    if depth == 0 {
        return direct_path_length(from, to);
    }
    
    // Memoization check - composite key!
    let key = (from, to, depth);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }
    
    // Recursive computation through next layer
    let paths = find_all_paths(from, to);
    let min_len = paths
        .iter()
        .map(|path| {
            path.chars()
                .tuple_windows()
                .map(|(a, b)| min_sequence_length(a, b, depth - 1, memo))
                .sum::<usize>()
        })
        .min()
        .unwrap();
    
    memo.insert(key, min_len);
    min_len
}
```

**Transformation**: O(4^26) → O(keys² × 26) = O(~7000 states)  
**Insight**: Multi-parameter memoization enables layered recursion optimization

---

## Common Patterns

### 1. **Count-Only Recursion** (Day 11, Day 19)
**When**: Counting solutions, not generating them  
**Benefit**: Avoid memory explosion from storing actual sequences/states  
**Pattern**:
```rust
// BAD: Return Vec<State> - memory explosion
fn generate_all(state: State, depth: usize) -> Vec<State> { ... }

// GOOD: Return count - constant memory per call
fn count_all(state: State, depth: usize, memo: &mut HashMap<Key, usize>) -> usize { ... }
```

### 2. **Composite Keys** (Day 21)
**When**: Multiple parameters define the subproblem  
**Benefit**: Enables caching of multi-dimensional recursion  
**Pattern**:
```rust
type MemoKey = (Param1, Param2, Param3);
let memo: HashMap<MemoKey, Result> = HashMap::new();
```

### 3. **Suffix/Prefix Recursion** (Day 19)
**When**: Processing sequences with overlapping subproblems  
**Benefit**: Natural memoization structure for string/sequence problems  
**Pattern**:
```rust
fn process(remaining: &str, memo: &mut HashMap<String, Result>) -> Result {
    if remaining.is_empty() { return base_case; }
    if let Some(&cached) = memo.get(remaining) { return cached; }
    // Process prefix, recurse on suffix
    let result = compute_for_prefix() + process(&remaining[prefix_len..], memo);
    memo.insert(remaining.to_string(), result);
    result
}
```

### 4. **Depth-Based Recursion** (Day 11, Day 21)
**When**: Recursive descent with limited depth (blinks, layers)  
**Benefit**: Natural termination condition + memoization key  
**Pattern**:
```rust
fn recurse(state: State, depth: usize, memo: &mut HashMap<(State, usize), Result>) -> Result {
    if depth == 0 { return base_case(state); }
    let key = (state, depth);
    // ... memo check, recurse with depth - 1, store result
}
```

---

## Performance Impact

| **Problem** | **Naive Complexity** | **Memoized Complexity** | **Speedup** |
|-------------|---------------------|------------------------|-------------|
| **Day 11** (75 blinks) | O(2^75) = 3.8×10^22 | O(~1000 states) | 10^19× faster |
| **Day 19** (length 60) | O(8^60) = 10^54 | O(60 × 8 = 480) | 10^52× faster |
| **Day 21** (26 layers) | O(4^26) = 4.5×10^15 | O(26 × 25 = 650) | 10^12× faster |

**Key Insight**: Memoization transforms **exponential → polynomial/linear** by eliminating redundant computation.

---

## Implementation Considerations

### HashMap Choice
```rust
use std::collections::HashMap;  // General-purpose
use rustc_hash::FxHashMap;      // Faster for integer keys (Days 11, 21)
```

### Key Type Design
```rust
// Simple key
type Key = usize;

// Composite key (must implement Hash + Eq)
type Key = (u64, usize);

// String key (Day 19) - consider interning for large datasets
type Key = String;
```

### Ownership Patterns
```rust
// Borrow memo throughout recursion
fn solve(&self, memo: &mut HashMap<K, V>) -> V { ... }

// Or wrap in RefCell for interior mutability
struct Solver {
    memo: RefCell<HashMap<K, V>>,
}
```

---

## When to Use Memoization

**Use when**:
✅ **Overlapping subproblems**: Same inputs computed multiple times  
✅ **Pure functions**: Output depends only on inputs (no side effects)  
✅ **Expensive computation**: Cost of cache lookup < cost of recomputation  
✅ **Bounded state space**: Won't exhaust memory with cache

**Don't use when**:
❌ **No overlap**: Each subproblem unique (use iteration instead)  
❌ **Impure functions**: Side effects or external state dependencies  
❌ **Cheap computation**: HashMap overhead > direct computation  
❌ **Unbounded state space**: Risk of memory exhaustion

---

## Related Concepts

- **Dynamic Programming**: Memoization is top-down DP (recursion + cache)
- **Tabulation**: Bottom-up DP (iterative array filling)
- **Caching**: General technique, memoization is function result caching
- **Recursion Optimization**: Tail recursion, memoization, trampolining

---

## Code Locations

- **Day 11 Solution**: `advent_of_code/aoc2024/src/solver/day11.rs`
- **Day 19 Solution**: `advent_of_code/aoc2024/src/solver/day19.rs`
- **Day 21 Solution**: `advent_of_code/aoc2024/src/solver/day21.rs`

---

## Learning Resources

- [[memoization-comprehensive-guide]] - Comprehensive Rust implementation guide (HashMap, Vec, Closure patterns)
- [[dynamic-programming]] - General DP patterns
- [[recursion-optimization]] - Tail recursion, memoization, trampolining
- [[rust-hashmap-performance]] - HashMap vs FxHashMap benchmarks
- [[aoc-2024-retrospective]] - Overall AoC 2024 patterns and learnings

---

*Tags*: #memoization #dynamic-programming #recursion #optimization #aoc-2024 #performance

*Links*:
- **MOC**: [[Memoization MOC]] - Complete memoization knowledge map
- **Daily Notes**: [[2025-12-21]] (created this page)
- **AoC Problems**: [[aoc-2024-day11]], [[aoc-2024-day19]], [[aoc-2024-day21]]
- **Concepts**: [[dynamic-programming]], [[recursion-optimization]], [[hash-maps]]
- **Weekly Plan**: [[2025-W52]]
