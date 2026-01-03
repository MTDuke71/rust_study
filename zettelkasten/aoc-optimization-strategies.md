# AoC Optimization Strategies

*Navigation: [[zettel-index]] | [[AoC Pattern Library]] | [[rust-concurrency-moc]] | [[rayon-parallel-iterators]]*

---

## Overview

Advent of Code problems often have naive solutions that work for examples but timeout on real input. This guide covers **optimization strategies** from simple algorithmic improvements to advanced parallel processing techniques.

**Philosophy**: Start simple, profile, optimize targeted bottlenecks. Premature optimization wastes time.

---

## Optimization Decision Tree

```
                    ┌─────────────────────────┐
                    │   Solution too slow?    │
                    └───────────┬─────────────┘
                                │
              ┌─────────────────┼─────────────────┐
              ▼                 ▼                 ▼
     ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
     │  Algorithm   │  │  Data        │  │  Parallelism │
     │  Complexity  │  │  Structures  │  │              │
     └──────┬───────┘  └──────┬───────┘  └──────┬───────┘
            │                 │                 │
     ┌──────┴──────┐   ┌──────┴──────┐   ┌──────┴──────┐
     │ O(n²)→O(n)  │   │ Vec→HashMap │   │ Rayon       │
     │ O(n³)→O(n²) │   │ String→&str │   │ Threads     │
     │ BFS not DFS │   │ HashSet     │   │ SIMD        │
     └─────────────┘   └─────────────┘   └─────────────┘
```

---

## Level 1: Algorithm Improvements

### Replace Brute Force with Smart Algorithms

| Problem Type | Naive | Optimized |
|--------------|-------|-----------|
| Search | O(n) linear scan | O(log n) binary search |
| Lookup | O(n) list search | O(1) HashMap |
| Grid spatial queries | O(n×m) iterate all entities | O(1) spatial index |
| Shortest path | BFS all paths | Dijkstra / A* |
| Combinations | Generate all | Dynamic programming |
| Repeated work | Recalculate | Memoization |

### Example: Memoization

```rust
use std::collections::HashMap;

// Naive recursive Fibonacci - O(2^n)
fn fib_naive(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fib_naive(n - 1) + fib_naive(n - 2),
    }
}

// Memoized Fibonacci - O(n)
fn fib_memo(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&result) = cache.get(&n) {
        return result;
    }
    let result = match n {
        0 | 1 => n,
        _ => fib_memo(n - 1, cache) + fib_memo(n - 2, cache),
    };
    cache.insert(n, result);
    result
}
```

### Example: Binary Search vs Linear

```rust
// O(n) - Linear search
fn find_linear(data: &[i32], target: i32) -> Option<usize> {
    data.iter().position(|&x| x == target)
}

// O(log n) - Binary search (requires sorted data)
fn find_binary(data: &[i32], target: i32) -> Option<usize> {
    data.binary_search(&target).ok()
}

// Speedup on 1M elements: ~50,000x faster average case
```

---

## Level 2: Data Structure Selection

### Choose the Right Collection

| Need | Use | Why |
|------|-----|-----|
| Fast lookup by key | `HashMap<K, V>` | O(1) average |
| Membership testing | `HashSet<T>` | O(1) contains |
| Sorted iteration | `BTreeMap<K, V>` | O(log n) ops, sorted |
| Queue (FIFO) | `VecDeque<T>` | O(1) push/pop both ends |
| Stack (LIFO) | `Vec<T>` | O(1) push/pop |
| Priority queue | `BinaryHeap<T>` | O(log n) push, O(1) peek |

### Example: HashSet for Membership

```rust
use std::collections::HashSet;

// O(n²) - Check membership with Vec
fn has_duplicates_vec(data: &[i32]) -> bool {
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            if data[i] == data[j] {
                return true;
            }
        }
    }
    false
}

// O(n) - Check membership with HashSet
fn has_duplicates_set(data: &[i32]) -> bool {
    let mut seen = HashSet::new();
    for &x in data {
        if !seen.insert(x) {
            return true;
        }
    }
    false
}
```

### Grid Representation Matters

```rust
// Sparse grid (few active cells) - use HashMap
let sparse: HashMap<(i32, i32), Cell> = HashMap::new();

// Dense grid (most cells active) - use Vec<Vec<T>> or flat Vec
let dense: Vec<Vec<Cell>> = vec![vec![Cell::default(); width]; height];

// Flat Vec (cache-friendly, fastest for iteration)
let flat: Vec<Cell> = vec![Cell::default(); width * height];
let cell = flat[y * width + x];
```

---

## Level 3: String & Memory Optimization

### Avoid Unnecessary Allocations

```rust
// BAD: Creates new String every iteration
fn process_bad(lines: &[String]) -> Vec<String> {
    lines.iter()
        .map(|s| s.to_uppercase())  // Allocates new String
        .collect()
}

// BETTER: Work with references when possible
fn count_uppercase(lines: &[&str]) -> usize {
    lines.iter()
        .filter(|s| s.chars().all(|c| c.is_uppercase()))
        .count()
}
```

### Use &str Instead of String When Possible

```rust
// Parsing with zero-copy
fn parse_line(line: &str) -> (&str, &str) {
    let mut parts = line.split(':');
    (parts.next().unwrap(), parts.next().unwrap().trim())
}

// vs allocating
fn parse_line_alloc(line: &str) -> (String, String) {
    let mut parts = line.split(':');
    (parts.next().unwrap().to_string(), parts.next().unwrap().trim().to_string())
}
```

### Pre-allocate Vectors

```rust
// BAD: Vec grows multiple times
let mut results = Vec::new();
for i in 0..10000 {
    results.push(expensive_calc(i));
}

// GOOD: Pre-allocate known capacity
let mut results = Vec::with_capacity(10000);
for i in 0..10000 {
    results.push(expensive_calc(i));
}
```

---

## Level 4: Parallelism with Rayon

See [[rayon-parallel-iterators]] for detailed coverage.

### Quick Rayon Patterns

```rust
use rayon::prelude::*;

// Parallel map + collect
let results: Vec<_> = data.par_iter().map(|x| expensive(x)).collect();

// Parallel sum
let total: i64 = numbers.par_iter().sum();

// Parallel find (any match)
let found = data.par_iter().find_any(|x| is_solution(x));

// Parallel find (first match in order)
let first = data.par_iter().find_first(|x| is_solution(x));
```

### When to Parallelize

✅ **Good candidates**:
- Processing large grids (>100x100)
- Search spaces (>10,000 candidates)
- Independent calculations per item
- Total work >10ms

❌ **Poor candidates**:
- Small collections (<1000 items)
- Cheap operations per item
- Heavy synchronization needed
- Sequential dependencies

---

## Level 5: Bit Manipulation

### Bitsets for State

```rust
// Instead of HashSet<u8> for tracking used digits
let mut used: u16 = 0;  // 16 bits, one per digit 0-15

fn mark_used(used: &mut u16, digit: u8) {
    *used |= 1 << digit;
}

fn is_used(used: u16, digit: u8) -> bool {
    (used & (1 << digit)) != 0
}

fn count_used(used: u16) -> u32 {
    used.count_ones()
}
```

### Bit Operations Cheat Sheet

```rust
// Set bit i
x |= 1 << i;

// Clear bit i
x &= !(1 << i);

// Toggle bit i
x ^= 1 << i;

// Check bit i
(x >> i) & 1 == 1

// Count set bits
x.count_ones()

// Lowest set bit
x & (!x + 1)  // or x & x.wrapping_neg()

// Clear lowest set bit
x & (x - 1)
```

---

## Level 6: Caching & Precomputation

### Precompute Expensive Values

```rust
// Instead of calculating primes on demand
fn is_prime_naive(n: u64) -> bool {
    if n < 2 { return false; }
    (2..=(n as f64).sqrt() as u64).all(|i| n % i != 0)
}

// Precompute prime sieve
fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
}

// O(1) lookup vs O(√n) per check
```

### LRU Cache for Repeated Lookups

```rust
use std::collections::HashMap;

struct LruCache<K, V> {
    map: HashMap<K, V>,
    capacity: usize,
}

// Or use the `lru` crate for production code
```

---

## AoC-Specific Patterns

### Pattern: Process Input Once

```rust
// BAD: Re-parse input in each function
fn part1(input: &str) -> u64 {
    let data = parse(input);  // Expensive
    solve_part1(&data)
}

fn part2(input: &str) -> u64 {
    let data = parse(input);  // Same expensive parse!
    solve_part2(&data)
}

// GOOD: Parse once, solve both
fn solve(input: &str) -> (u64, u64) {
    let data = parse(input);  // Parse once
    (solve_part1(&data), solve_part2(&data))
}
```

### Pattern: Early Termination

```rust
// Find first solution, don't continue searching
fn find_solution(range: Range<u64>) -> Option<u64> {
    range.into_par_iter().find_first(|&n| is_valid(n))
    // Stops as soon as ANY thread finds a match
}
```

### Pattern: Avoid Repeated String Parsing

```rust
// Parse to numbers once, not every time you need them
fn parse_numbers(input: &str) -> Vec<i64> {
    input.lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

// Then work with Vec<i64> instead of re-parsing strings
```

---

## Profiling Your Solutions

### Quick Timing

```rust
use std::time::Instant;

let start = Instant::now();
let result = solve(input);
let duration = start.elapsed();
println!("Solved in {:?}", duration);
```

### Criterion Benchmarking

```rust
// benches/benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_solve(c: &mut Criterion) {
    let input = include_str!("../input.txt");
    
    c.bench_function("part1", |b| {
        b.iter(|| solve_part1(black_box(input)))
    });
}

criterion_group!(benches, benchmark_solve);
criterion_main!(benches);
```

---

## Optimization Checklist

Before optimizing, ask:

- [ ] **Does it need optimizing?** (Is it actually slow?)
- [ ] **Where is the bottleneck?** (Profile first!)
- [ ] **Can the algorithm improve?** (O(n²) → O(n log n))
- [ ] **Are data structures optimal?** (Vec → HashMap?)
- [ ] **Are there unnecessary allocations?** (String → &str?)
- [ ] **Is the work parallelizable?** (Rayon?)
- [ ] **Can values be precomputed?** (Lookup table?)

---

## Common AoC Optimization Wins

| Problem Type | Typical Win |
|--------------|-------------|
| Path finding | HashMap for visited, BinaryHeap for priority |
| Grid processing | Flat Vec, parallel rows with Rayon |
| State exploration | Bitset representation, memoization |
| Large ranges | Mathematical insight, pattern detection |
| String matching | &str over String, avoid clone() |
| Counting | HashMap counters, mathematical formulas |

---

## Related Concepts

### Algorithms & Data Structures
- [[binary-search]] - O(log n) search in sorted data
- [[graph-algorithms-overview]] - BFS, DFS, Dijkstra
- [[dynamic-programming-basics]] - Memoization patterns

### Parallelism
- [[rayon-parallel-iterators]] - Data parallelism made easy
- [[rust-threading-basics]] - Manual thread control
- [[async-vs-threads-decision]] - Choosing concurrency model

### Collections
- [[hashmap-fundamentals]] - O(1) key-value lookups
- [[Collections MOC]] - All Rust collection types
- [[spatial-indexing-pattern]] - Coordinate-to-entity lookup (246.8x speedup on Day 3)

### AoC Specific
- [[AoC Pattern Library]] - Reusable solution patterns
- [[aoc-parsing-patterns]] - Efficient input parsing

---

*Tags: #aoc #optimization #performance #algorithms #data-structures #rayon #profiling*

*Links: [[zettel-index]] | [[AoC Pattern Library]] | [[rayon-parallel-iterators]] | [[rust-threading-basics]] | [[Collections MOC]] | [[rust-concurrency-moc]]*
