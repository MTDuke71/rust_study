# Memoization Patterns - Top-Down Dynamic Programming in Rust

**Comprehensive guide to memoization techniques, caching strategies, and performance optimization in Rust**

---

## Overview

Memoization is an optimization technique where the results of expensive function calls are cached and returned when the same inputs occur again. It transforms recursive algorithms from exponential time complexity to polynomial by eliminating redundant computations.

**Key insight:** Memoization is top-down dynamic programming - you solve problems recursively but cache intermediate results.

---

## Core Concepts

### What is Memoization?

Memoization involves:

- **Caching Results**: Store computed values for reuse
- **Checking Cache First**: Before computation, check if we have a cached result
- **Avoiding Redundant Work**: Eliminate exponential blowup in recursive algorithms
- **Trade-off**: Memory usage for execution speed

```rust
// Without memoization - exponential time O(2^n)
fn fib_naive(n: u32) -> u64 {
    match n {
        0 | 1 => n as u64,
        _ => fib_naive(n - 1) + fib_naive(n - 2)
    }
}

// With memoization - polynomial time O(n)
fn fib_memo(n: u32, cache: &mut std::collections::HashMap<u32, u64>) -> u64 {
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
```

### Why Memoization Matters

**Scenario: Fibonacci sequence**

- `fib(5)` without memoization: 15 function calls
- `fib(10)` without memoization: 177 function calls
- `fib(30)` without memoization: 2,178,308 function calls ❌ Unreasonable!
- `fib(30)` with memoization: 30 function calls ✅

---

## Memoization Patterns in Rust

### Pattern 1: HashMap-Based Memoization (Most Common)

**Best for:** Sparse state spaces, non-sequential access patterns, complex state types

```rust
use std::collections::HashMap;

type Cache<K, V> = HashMap<K, V>;

fn compute_with_memo<K, V, F>(
    state: K,
    cache: &mut Cache<K, V>,
    compute: F,
) -> V
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
    F: Fn(K, &mut Cache<K, V>) -> V,
{
    if let Some(result) = cache.get(&state) {
        return result.clone();
    }

    let result = compute(state.clone(), cache);
    cache.insert(state, result.clone());
    result
}

// Example: Fibonacci with clean separation
fn fib_with_pattern(n: u32) -> u64 {
    let mut cache = HashMap::new();

    fn fib_impl(n: u32, cache: &mut HashMap<u32, u64>) -> u64 {
        if let Some(&result) = cache.get(&n) {
            return result;
        }

        let result = match n {
            0 | 1 => n as u64,
            _ => fib_impl(n - 1, cache) + fib_impl(n - 2, cache)
        };

        cache.insert(n, result);
        result
    }

    fib_impl(n, &mut cache)
}
```

**Advantages:**

- Handles sparse computation graphs efficiently
- Works with any hashable key type
- No pre-allocation needed
- Cleaner for complex state representations

**Disadvantages:**

- HashMap lookups have O(log n) average overhead
- More memory overhead per entry

---

### Pattern 2: Vec-Based Memoization (Bottom-Up Alternative)

**Best for:** Sequential state spaces, dense computation, known bounds

```rust
// For problems where state is 0..n
fn fib_bottom_up(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let mut dp = vec![0u64; n + 1];
    dp[1] = 1;

    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[n]
}

// Space-optimized version (when you only need last two values)
fn fib_bottom_up_optimized(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let (mut prev, mut curr) = (0u64, 1u64);

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}
```

**Advantages:**

- O(1) cache lookup time (array indexing)
- Minimal memory overhead
- Cache-friendly (contiguous memory)
- Works great for sequential indices

**Disadvantages:**

- Requires knowing state space bounds
- Wasteful for sparse problems
- Less flexible state representation

---

### Pattern 3: Closure-Based Memoization

**Best for:** Simple recursive functions, functional style, embedding in other code

```rust
fn memoized_closure<K, V, F>(mut f: F) -> impl FnMut(K) -> V
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
    F: FnMut(K, &mut std::collections::HashMap<K, V>) -> V,
{
    let mut cache = std::collections::HashMap::new();

    move |input: K| -> V {
        if let Some(result) = cache.get(&input) {
            return result.clone();
        }

        let result = f(input.clone(), &mut cache);
        cache.insert(input, result.clone());
        result
    }
}

// Usage
let fib = memoized_closure(|n: u32, cache: &mut HashMap<u32, u64>| -> u64 {
    if let Some(&result) = cache.get(&n) {
        return result;
    }

    let result = match n {
        0 | 1 => n as u64,
        _ => fib(n - 1) + fib(n - 2)
    };

    cache.insert(n, result);
    result
});

// Note: This pattern has limitations in Rust due to borrow checker
```

**Advantages:**

- Functional programming style
- State encapsulated in closure
- Elegant for simple problems

**Disadvantages:**

- Borrow checker challenges with recursive closures
- Requires boxing or advanced techniques
- May require `RefCell` or `Rc`

---

### Pattern 4: Generic MemoCache Wrapper (Mission 11 Pattern)

**Best for:** Reusable memoization, production code, cache statistics, API abstraction

**Context:** Developed in [[missions/mission-11|Mission 11]] and validated by AoC 2023 Day 12 (Hot Springs).

```rust
use std::collections::HashMap;
use std::hash::Hash;

/// Generic memoization cache with statistics tracking
/// 
/// # Examples
/// ```
/// use std::collections::HashMap;
/// 
/// let mut cache = MemoCache::<(usize, usize), u64>::new();
/// 
/// // First call computes
/// let result = cache.memoize((5, 3), || expensive_computation(5, 3));
/// 
/// // Second call uses cache (instant)
/// let result2 = cache.memoize((5, 3), || panic!("Should not compute!"));
/// ```
pub struct MemoCache<K, V> 
where
    K: Hash + Eq,
    V: Clone,
{
    cache: HashMap<K, V>,
    hits: usize,
    misses: usize,
}

impl<K, V> MemoCache<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    /// Retrieves or computes a memoized value
    pub fn memoize<F>(&mut self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(cached) = self.cache.get(&key) {
            self.hits += 1;
            return cached.clone();
        }

        self.misses += 1;
        let value = compute();
        self.cache.insert(key, value.clone());
        value
    }

    /// Returns cache hit ratio (0.0 to 1.0)
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    pub fn size(&self) -> usize {
        self.cache.len()
    }
}

// Example: AoC 2023 Day 12 Pattern (3D State Tuple)
fn count_arrangements(
    springs: &[u8],
    groups: &[usize],
    pos: usize,
    group_idx: usize,
    current_run: usize,
    memo: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    // Base case
    if pos == springs.len() {
        if group_idx == groups.len() && current_run == 0 {
            return 1; // Valid arrangement
        }
        return 0; // Invalid
    }

    // Memoization check
    let key = (pos, group_idx, current_run);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let mut count = 0;
    let ch = springs[pos];

    // Try placing '.' (operational spring)
    if ch == b'.' || ch == b'?' {
        if current_run == 0 {
            count += count_arrangements(springs, groups, pos + 1, group_idx, 0, memo);
        } else if group_idx < groups.len() && current_run == groups[group_idx] {
            count += count_arrangements(springs, groups, pos + 1, group_idx + 1, 0, memo);
        }
    }

    // Try placing '#' (damaged spring)
    if (ch == b'#' || ch == b'?') && group_idx < groups.len() && current_run < groups[group_idx] {
        count += count_arrangements(springs, groups, pos + 1, group_idx, current_run + 1, memo);
    }

    memo.insert(key, count);
    count
}
```

**Key Features:**

- **Generic over state types:** Works with tuples, structs, any `K: Hash + Eq`
- **Statistics tracking:** Hit ratio reveals memoization effectiveness
  - AoC 2023 Day 12 Part 2: 95% hit rate (massive overlap!)
  - Low hit ratio suggests memoization may not be beneficial
- **API abstraction:** `memoize(key, compute)` pattern cleaner than manual cache management
- **Type safety:** Compile-time guarantees on key/value types

**Composite State Keys:**

For complex DP problems, use tuple keys to represent multi-dimensional state:

```rust
// 3D State: (position, group_index, current_run_length)
type State3D = (usize, usize, usize);
let mut memo: HashMap<State3D, usize> = HashMap::new();

// 4D State: (position, value, count, depth)  
type State4D = (usize, u64, usize, usize);
let mut memo: HashMap<State4D, u64> = HashMap::new();
```

**Advantages:**

- Production-ready pattern with statistics
- Reusable across different problems
- Clear separation of caching logic from algorithm
- Performance metrics guide optimization decisions
- Generic design supports any state type

**Disadvantages:**

- Slight overhead from wrapper abstraction
- Clone requirement for values (usually negligible)

**When to Use:**

✅ Production code requiring maintainability  
✅ Need cache statistics for analysis  
✅ Multiple problems with similar memoization needs  
✅ API abstraction over raw HashMap  

❌ Simple one-off scripts (raw HashMap is fine)  
❌ Performance-critical inner loops (measure first!)

**Real-World Impact:**

- **AoC 2023 Day 12**: 2.94ms (Part 1), 41.26ms (Part 2) with 300K cached states
- **Fibonacci n=30**: 262x speedup over naive recursion
- **State space analysis**: Cache size reveals problem complexity

See [[missions/mission-11|Mission 11]] for complete implementation with tests, benchmarks, and production-quality documentation.

---

## Real-World Applications

### AoC 2015 Day 10: Look-and-Say Sequence

A great example is the "Look-and-Say" sequence where each iteration processes the previous one:

```rust
use std::collections::HashMap;

fn look_and_say(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        let mut count = 1;

        while i + count < chars.len() && chars[i + count] == ch {
            count += 1;
        }

        result.push_str(&count.to_string());
        result.push(ch);
        i += count;
    }

    result
}

fn look_and_say_n_times(s: &str, n: u32) -> String {
    let mut current = s.to_string();

    for _ in 0..n {
        current = look_and_say(&current);
    }

    current
}

// Memoization helps when computing for different starting points
fn compute_results_with_memo(
    starts: &[&str],
    iterations: u32,
    cache: &mut HashMap<(String, u32), String>,
) -> Vec<String> {
    starts
        .iter()
        .map(|&s| {
            let key = (s.to_string(), iterations);
            if let Some(result) = cache.get(&key) {
                result.clone()
            } else {
                let result = look_and_say_n_times(s, iterations);
                cache.insert(key, result.clone());
                result
            }
        })
        .collect()
}
```

**Important Note:** As Day 10 teaches us, memoization isn't always the answer! For this problem:

- String concatenation overhead dominates
- Intermediate strings are unique, so cache hits are rare
- Simple iteration without memoization is faster
- See [[../../advent_of_code/aoc2015/examples/DAY10_BENCHMARK_ANALYSIS]] for performance comparisons

---

### AoC 2015 Day 7: Circuit Simulation

Memoization is ideal for dependency graphs:

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Wire {
    Signal(u16),
    Ref(String),
}

#[derive(Debug, Clone)]
enum Gate {
    Direct(Wire),
    And(Wire, Wire),
    Or(Wire, Wire),
    Lshift(Wire, u8),
    Rshift(Wire, u8),
    Not(Wire),
}

fn evaluate_wire(
    wire_name: &str,
    gates: &HashMap<String, Gate>,
    memo: &mut HashMap<String, u16>,
) -> u16 {
    // Check cache first
    if let Some(&value) = memo.get(wire_name) {
        return value;
    }

    let gate = &gates[wire_name];

    let value = match gate {
        Gate::Direct(Wire::Signal(s)) => *s,
        Gate::Direct(Wire::Ref(r)) => evaluate_wire(r, gates, memo),
        Gate::And(w1, w2) => {
            let v1 = resolve_wire(w1, gates, memo);
            let v2 = resolve_wire(w2, gates, memo);
            v1 & v2
        }
        Gate::Or(w1, w2) => {
            let v1 = resolve_wire(w1, gates, memo);
            let v2 = resolve_wire(w2, gates, memo);
            v1 | v2
        }
        Gate::Lshift(w, n) => resolve_wire(w, gates, memo) << n,
        Gate::Rshift(w, n) => resolve_wire(w, gates, memo) >> n,
        Gate::Not(w) => !resolve_wire(w, gates, memo),
    };

    memo.insert(wire_name.to_string(), value);
    value
}

fn resolve_wire(
    wire: &Wire,
    gates: &HashMap<String, Gate>,
    memo: &mut HashMap<String, u16>,
) -> u16 {
    match wire {
        Wire::Signal(s) => *s,
        Wire::Ref(r) => evaluate_wire(r, gates, memo),
    }
}
```

Here, memoization prevents re-evaluating dependent wires multiple times!

---

## Performance Considerations

### When to Use Memoization

✅ **Use memoization when:**

- Problem has overlapping subproblems (Fibonacci, DP problems)
- Recursive solution is natural and clear
- State space is moderate-sized
- Cache hit rate will be high
- Results are truly expensive to compute

❌ **Don't use memoization when:**

- All subproblems are unique (no overlaps)
- Cache overhead exceeds computation cost
- Simple iterative solution exists
- State space is enormous
- String concatenation dominates (like Day 10)

### Cache Invalidation

For problems requiring multiple solutions with modifications:

```rust
// Problem: Compute wire "a" before and after resetting it
fn solve_with_invalidation(
    gates: &HashMap<String, Gate>,
    target: &str,
) -> (u16, u16) {
    // First computation
    let mut memo = HashMap::new();
    let part1 = evaluate_wire(target, gates, &mut memo);

    // Invalidate cache (or create new one) for modified problem
    let mut memo2 = HashMap::new();
    let part2 = evaluate_wire(target, gates, &mut memo2);

    (part1, part2)
}

// Better: Parameterized computation
fn solve_with_override(
    gates: &HashMap<String, Gate>,
    target: &str,
    override_wire: &str,
    override_value: u16,
) -> u16 {
    let mut memo = HashMap::new();
    // Prime the cache with override
    memo.insert(override_wire.to_string(), override_value);
    evaluate_wire(target, gates, &mut memo)
}
```

---

## Common Pitfalls

### Pitfall 1: Excessive Cloning

```rust
// ❌ Bad: Clones on every access
fn bad_memo(n: u32, cache: &mut HashMap<u32, Vec<i32>>) -> Vec<i32> {
    let result = cache.get(&n).cloned()?;  // Unnecessary clone
    // ...
}

// ✅ Good: Return reference when possible
fn good_memo<'a>(
    n: u32,
    cache: &'a mut HashMap<u32, Vec<i32>>,
) -> Option<&'a Vec<i32>> {
    cache.get(&n)
}
```

### Pitfall 2: Unbounded Cache Growth

```rust
// ❌ Bad: Cache grows indefinitely
fn unbounded_cache(inputs: &[u32]) -> u64 {
    let mut cache = HashMap::new();  // Can grow to millions of entries
    for &input in inputs {
        fib_memo(input, &mut cache);
    }
    // ...
}

// ✅ Good: Clear cache periodically or use bounded structure
use lru::LruCache;
fn bounded_cache(inputs: &[u32]) -> u64 {
    let mut cache = LruCache::new(std::num::NonZeroUsize::new(1000).unwrap());
    for &input in inputs {
        // Access cache, oldest entries evicted when size exceeded
    }
}
```

### Pitfall 3: Forgetting to Check Cache

```rust
// ❌ Bad: Computes even if cached
fn forgetting_check(n: u32, cache: &mut HashMap<u32, u64>) -> u64 {
    let result = expensive_computation(n);  // No cache check!
    cache.insert(n, result);
    result
}

// ✅ Good: Always check first
fn proper_check(n: u32, cache: &mut HashMap<u32, u64>) -> u64 {
    if let Some(&result) = cache.get(&n) {
        return result;  // Return immediately
    }
    let result = expensive_computation(n);
    cache.insert(n, result);
    result
}
```

---

## Memoization vs. Other Optimization Techniques

| Technique | When to Use | Complexity | Memory |
|-----------|------------|-----------|--------|
| **Memoization** | Overlapping subproblems, recursive | Top-down DP | O(subproblems) |
| **Tabulation** | Sequential computation, known bounds | Bottom-up DP | O(subproblems) |
| **Greedy** | Optimal substructure, greedy choice | Usually O(n log n) | O(1) |
| **Divide & Conquer** | Non-overlapping subproblems | Depends on split | Depends |
| **Iteration** | Simple sequential problems | O(n) or O(n²) | O(1) |

---

## Integration with Rust Concepts

### Ownership and Memoization

```rust
// Memoization with owned values
fn compute_owned(n: u32, cache: &mut HashMap<u32, String>) -> String {
    if let Some(result) = cache.get(&n) {
        return result.clone();  // Clone for return
    }

    let result = format!("value_{}", n);
    cache.insert(n, result.clone());  // Move into cache
    result
}

// Memoization with references (more complex)
fn compute_ref<'a>(
    n: u32,
    cache: &'a mut HashMap<u32, String>,
) -> &'a str {
    if cache.contains_key(&n) {
        return &cache[&n];  // Borrow from cache
    }

    let result = format!("value_{}", n);
    cache.insert(n, result);
    &cache[&n]  // Return borrow
}
```

### Thread Safety

```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

fn fib_threadsafe(n: u32, cache: Arc<Mutex<HashMap<u32, u64>>>) -> u64 {
    {
        let cached = cache.lock().unwrap();
        if let Some(&result) = cached.get(&n) {
            return result;
        }
    }

    let result = match n {
        0 | 1 => n as u64,
        _ => {
            let c1 = Arc::clone(&cache);
            let c2 = Arc::clone(&cache);
            fib_threadsafe(n - 1, c1) + fib_threadsafe(n - 2, c2)
        }
    };

    cache.lock().unwrap().insert(n, result);
    result
}
```

---

## Advanced Patterns

### Memoization with Multiple Parameters

```rust
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone)]
struct State {
    position: (i32, i32),
    resources: u32,
    turns_left: u32,
}

fn solve(state: State, cache: &mut HashMap<State, u32>) -> u32 {
    if let Some(&result) = cache.get(&state) {
        return result;
    }

    if state.turns_left == 0 {
        return state.resources;
    }

    let mut max_result = 0;
    for direction in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let new_state = State {
            position: (state.position.0 + direction.0, state.position.1 + direction.1),
            resources: state.resources,
            turns_left: state.turns_left - 1,
        };
        max_result = max_result.max(solve(new_state, cache));
    }

    cache.insert(state, max_result);
    max_result
}
```

### Memoization with Callbacks

```rust
fn compute_with_callback<F>(
    n: u32,
    cache: &mut HashMap<u32, u64>,
    on_compute: &mut F,
) -> u64
where
    F: FnMut(u32),
{
    if let Some(&result) = cache.get(&n) {
        return result;
    }

    on_compute(n);  // Track what we're computing
    let result = expensive_computation(n);
    cache.insert(n, result);
    result
}
```

---

## Testing Memoization

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_correctness() {
        let mut cache = HashMap::new();
        assert_eq!(fib_memo(5, &mut cache), 5);
        assert_eq!(fib_memo(10, &mut cache), 55);
    }

    #[test]
    fn test_cache_effectiveness() {
        let mut cache = HashMap::new();

        // First call
        let start = std::time::Instant::now();
        fib_memo(30, &mut cache);
        let first = start.elapsed();

        // Second call should be instant
        let start = std::time::Instant::now();
        fib_memo(30, &mut cache);
        let second = start.elapsed();

        assert!(second < first);  // Cache hit should be faster
    }

    #[test]
    fn test_cache_isolation() {
        let mut cache1 = HashMap::new();
        let mut cache2 = HashMap::new();

        fib_memo(10, &mut cache1);
        fib_memo(10, &mut cache2);

        assert_eq!(cache1.get(&10), cache2.get(&10));  // Same results
    }
}
```

---

## Related Concepts

- **[[memoization-aoc2024-patterns]]** - AoC 2024 problem-solving patterns (Day 11, 19, 21)
- **[[tabulation-patterns]]** - Bottom-up DP alternative
- **[[Dynamic Programming Patterns]]** - General DP strategies
- **[[state-based-memoization]]** - Composite state representation for path counting with constraints (Day 7 + Day 11 examples)
- **[[HashMap Deep Dive]]** - Cache implementation details
- **[[Performance Optimization Guide]]** - Broader optimization techniques
- **[[Algorithm Complexity Analysis]]** - Understanding when memo helps
- **[[../../advent_of_code/aoc2015/examples/DAY10_MEMOIZATION_WALKTHROUGH]]** - Real example walkthrough
- **[[../../advent_of_code/aoc2015/examples/DAY10_BENCHMARK_ANALYSIS]]** - When NOT to use memoization

## References

- *Competence with memoization is essential for competitive programming*
- *See [[AoC Patterns MOC]] for integration with other patterns*
- *[[Performance Optimization Guide]] covers cost-benefit analysis*
- *[[Rust Collections MOC]] for HashMap performance characteristics*
- *[[top-down-dp-pattern]] - Canonical 4-step recursive DP template*
- *[[zero-copy-string-slicing]] - Zero-allocation string optimization*
- *[[mission-11]] - Mission 11 implementation with memoization patterns*

---

*Tags: #dynamic-programming #memoization #caching #optimization #algorithms #rust #performance #aoc*

*Related MOCs: [[Memoization MOC]] | [[AoC Patterns MOC]] | [[Algorithm Analysis]] | [[Algorithms MOC]] | [[Performance Optimization Guide]] | [[top-down-dp-pattern]] | [[mission-11]]*
