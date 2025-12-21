# Rayon Parallel Iterators

*Navigation: [[zettel-index]] | [[rust-concurrency-moc]] | [[rust-threading-basics]] | [[async-vs-threads-decision]]*

---

## Overview

**Rayon** is Rust's premier data parallelism library, providing effortless parallel iteration with minimal code changes. Convert sequential iterators to parallel with a single method call while Rayon handles work-stealing, load balancing, and thread pool management.

**Key Insight**: Rayon is the **go-to solution for CPU-bound parallel work** in Rust. It's simpler than manual threading and often faster due to sophisticated work-stealing algorithms.

---

## The One-Line Parallelization

```rust
use rayon::prelude::*;

// Sequential
let sum: i64 = (0..1_000_000).map(|n| expensive_calc(n)).sum();

// Parallel - just add .par_iter() or use into_par_iter()
let sum: i64 = (0..1_000_000).into_par_iter().map(|n| expensive_calc(n)).sum();
```

That's it. Rayon automatically:
- Creates a thread pool (one thread per CPU core)
- Divides work into chunks
- Steals work between threads for load balancing
- Joins results efficiently

---

## Core API

### Converting to Parallel Iterators

| Method | Use Case | Ownership |
|--------|----------|-----------|
| `.par_iter()` | Parallel iteration over `&T` | Borrows |
| `.par_iter_mut()` | Parallel iteration over `&mut T` | Mutable borrow |
| `.into_par_iter()` | Parallel iteration consuming collection | Takes ownership |

```rust
use rayon::prelude::*;

let data = vec![1, 2, 3, 4, 5];

// Borrow - read-only access
let sum: i32 = data.par_iter().map(|x| x * 2).sum();

// Mutable borrow - modify in place
let mut data = vec![1, 2, 3, 4, 5];
data.par_iter_mut().for_each(|x| *x *= 2);

// Consume - take ownership
let doubled: Vec<i32> = data.into_par_iter().map(|x| x * 2).collect();
```

### Common Parallel Operations

```rust
use rayon::prelude::*;

let numbers: Vec<i64> = (0..1_000_000).collect();

// Map + Collect
let squares: Vec<i64> = numbers.par_iter().map(|n| n * n).collect();

// Filter + Collect
let evens: Vec<&i64> = numbers.par_iter().filter(|n| *n % 2 == 0).collect();

// Reduce (fold with combining)
let sum: i64 = numbers.par_iter().sum();
let product: i64 = numbers.par_iter().product();

// Find (returns first match found by ANY thread)
let found: Option<&i64> = numbers.par_iter().find_any(|n| is_prime(**n));

// For Each (side effects)
numbers.par_iter().for_each(|n| println!("{}", n));

// All / Any
let all_positive = numbers.par_iter().all(|n| *n >= 0);
let any_negative = numbers.par_iter().any(|n| *n < 0);
```

---

## Work-Stealing Algorithm

Rayon uses **work-stealing** for load balancing:

```
┌─────────────────────────────────────────────────────────┐
│                    Initial State                         │
│  Thread 1: [████████████████████]  (lots of work)       │
│  Thread 2: [████████████████████]                        │
│  Thread 3: [████████████████████]                        │
│  Thread 4: [████████████████████]                        │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│                    After Some Time                       │
│  Thread 1: [████░░░░░░░░░░░░░░░░]  (finished early)     │
│  Thread 2: [████████████████████]  (still working)       │
│  Thread 3: [██████████░░░░░░░░░░]                        │
│  Thread 4: [████████████████████]                        │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│                    Work Stealing                         │
│  Thread 1: [████████]  ← Stole from Thread 2            │
│  Thread 2: [████████████]  (gave away some work)        │
│  Thread 3: [████████████]                                │
│  Thread 4: [████████████████]                            │
└─────────────────────────────────────────────────────────┘
```

**Benefits**:
- Automatic load balancing
- No manual chunk sizing needed
- Handles uneven work distributions
- Maximizes CPU utilization

---

## AoC Applications

Rayon excels at many AoC problem patterns:

### Pattern 1: Parallel Search

```rust
use rayon::prelude::*;

// Find first solution in a large search space
fn find_md5_with_prefix(key: &str, prefix: &str) -> u64 {
    (0..u64::MAX)
        .into_par_iter()
        .find_first(|&n| {
            let hash = format!("{:x}", md5::compute(format!("{}{}", key, n)));
            hash.starts_with(prefix)
        })
        .unwrap()
}
```

### Pattern 2: Grid Processing

```rust
use rayon::prelude::*;

// Process each row of a grid in parallel
fn process_grid(grid: &mut Vec<Vec<Cell>>) {
    grid.par_iter_mut().for_each(|row| {
        for cell in row.iter_mut() {
            cell.update();
        }
    });
}

// Or process all cells if independent
fn count_active(grid: &[Vec<Cell>]) -> usize {
    grid.par_iter()
        .map(|row| row.iter().filter(|c| c.is_active()).count())
        .sum()
}
```

### Pattern 3: Simulation Steps

```rust
use rayon::prelude::*;

// Parallel game of life step
fn step(current: &Grid) -> Grid {
    let mut next = current.clone();
    
    next.cells.par_iter_mut().enumerate().for_each(|(i, row)| {
        for (j, cell) in row.iter_mut().enumerate() {
            let neighbors = count_neighbors(current, i, j);
            *cell = match (current.cells[i][j], neighbors) {
                (true, 2..=3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    });
    
    next
}
```

### Pattern 3.5: Parallel Reduction with Tuples (AoC 2024 Day 14)

```rust
use rayon::prelude::*;
use mission6::Grid;

// Parallel quadrant counting with tuple reduction
fn calculate_safety_factor_parallel(robots: &[Robot], width: i32, height: i32) -> usize {
    let mid_x = width / 2;
    let mid_y = height / 2;
    
    // Map each robot to (q1_count, q2_count, q3_count, q4_count)
    let (q1, q2, q3, q4) = robots
        .par_iter()
        .map(|robot| {
            let (x, y) = robot.position_at(100, width, height);
            match (x.cmp(&mid_x), y.cmp(&mid_y)) {
                (Less, Less) => (1, 0, 0, 0),
                (Greater, Less) => (0, 1, 0, 0),
                (Less, Greater) => (0, 0, 1, 0),
                (Greater, Greater) => (0, 0, 0, 1),
                _ => (0, 0, 0, 0),  // On midline, skip
            }
        })
        .reduce(
            || (0, 0, 0, 0),
            |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3),
        );
    
    q1 * q2 * q3 * q4
}
```

**Pattern**: Map items to tuple components, reduce by adding corresponding fields. Elegantly handles multiple counters in parallel.

### Pattern 4: Multiple Independent Parts

```rust
use rayon::prelude::*;

// Solve multiple puzzle parts in parallel
fn solve_all_parts(input: &str) -> (u64, u64, u64) {
    let parts: Vec<_> = vec![
        || solve_part1(input),
        || solve_part2(input),
        || solve_part3(input),
    ];
    
    let results: Vec<u64> = parts.into_par_iter()
        .map(|f| f())
        .collect();
    
    (results[0], results[1], results[2])
}
```

---

## find_first vs find_any

Important distinction for AoC:

```rust
use rayon::prelude::*;

let numbers: Vec<i64> = (0..1_000_000).collect();

// find_any - Returns ANY match (faster, non-deterministic order)
// Good for: "Does any solution exist?"
let any_prime: Option<&i64> = numbers.par_iter().find_any(|n| is_prime(**n));

// find_first - Returns FIRST match in original order (slower, deterministic)
// Good for: "What's the smallest/first solution?"
let first_prime: Option<&i64> = numbers.par_iter().find_first(|n| is_prime(**n));
```

**AoC Rule of Thumb**:
- Use `find_first` when answer must be the smallest/earliest
- Use `find_any` when you just need any valid answer

---

## Thread Pool Configuration

```rust
use rayon::ThreadPoolBuilder;

// Default: One thread per CPU core
// Usually optimal - don't change unless you have a reason

// Custom thread pool (rare)
let pool = ThreadPoolBuilder::new()
    .num_threads(4)
    .build()
    .unwrap();

pool.install(|| {
    // All rayon operations here use this pool
    let result: i64 = (0..1000).into_par_iter().sum();
});
```

---

## When to Use Rayon vs Threads vs Async

| Scenario | Best Choice |
|----------|-------------|
| CPU-bound data parallelism | **Rayon** ✅ |
| Independent tasks, same operation | **Rayon** ✅ |
| Complex thread communication | Manual threads |
| I/O-bound concurrent work | Async |
| Simple fire-and-forget threads | `std::thread::spawn` |
| Need specific thread control | Manual threads |

**Rayon Sweet Spot**: "I have a collection and want to process items in parallel"

---

## Performance Tips

### ✅ DO:

```rust
// Process large collections
let result: Vec<_> = large_vec.par_iter().map(expensive_fn).collect();

// Use when work per item is substantial
numbers.par_iter().map(|n| heavy_computation(n)).sum()
```

### ❌ DON'T:

```rust
// Don't parallelize tiny collections (overhead > benefit)
let small = vec![1, 2, 3];
small.par_iter().sum();  // Sequential is faster!

// Don't parallelize cheap operations
numbers.par_iter().map(|n| n + 1).sum();  // Sequential is faster!
```

### Rule of Thumb

Rayon benefits when:
- Collection size > ~1000 elements
- Work per element > ~1µs
- Total work > ~10ms

### Empirical Data (AoC 2024 Day 14)

Concrete performance measurements from robot simulation:

| **Dataset Size** | **Serial** | **Parallel** | **Speedup** | **Winner** |
|------------------|------------|--------------|-------------|-----------|
| 5 items          | 400 ns     | 96.6 µs      | 0.004x      | Serial (241x faster) |
| 1,000 items      | 37.8 µs    | 733 µs       | 0.05x       | Serial (19x faster) |
| 1,000,000 items  | 5.01 ms    | 1.41 ms      | **3.54x**   | **Parallel** |

**Key Finding**: Parallel overhead is ~100µs. Only worth it when total work > 1ms.

### Empirical Data (AoC 2024 Day 22)

Concrete performance measurements from PRNG simulation (2020 buyers × 2000 secrets):

**Part 1: Pure Computation (Embarrassingly Parallel)**

| **Metric** | **Serial** | **Parallel** | **Result** |
|------------|------------|--------------|------------|
| Execution Time | 18.6 ms | 1.1 ms | **16.58x speedup** ✅ |
| Problem Size | 2020 buyers | 2020 buyers | 12.1M PRNG operations |
| Scaling | Linear (1 core) | Near-linear (multi-core) | Excellent parallelization |

**Part 2: HashMap Aggregation (Shared State Challenge)**

| **Approach** | **Time** | **Speedup** | **Analysis** |
|--------------|----------|-------------|--------------|
| Serial | 183.4 ms | 1.0x (baseline) | Single-threaded HashMap updates |
| Parallel (Mutex) | 93.0 ms | 1.97x | Lock contention limits scaling |
| **Parallel (Thread-local)** | **84.6 ms** | **2.17x** ✅ | Reduced contention, best performance |

**Key Findings**:
- **Part 1**: Near-perfect scaling (16.58x on multi-core CPU) - each buyer's PRNG is completely independent
- **Part 2**: Thread-local HashMaps significantly outperform shared Mutex approach
- **Pattern**: Even with shared state (HashMap), 2x+ speedup achievable with proper contention management
- **Lesson**: Always measure Mutex vs thread-local patterns for parallel aggregation

**Implementation Pattern (Thread-Local)**:
```rust
// Each thread builds its own HashMap, serial merge at end
let thread_maps: Vec<HashMap<K, V>> = data
    .par_iter()
    .map(|item| process_to_hashmap(item))  // Independent per thread
    .collect();

// Serial merge (relatively cheap compared to parallel work)
let mut result = HashMap::new();
for map in thread_maps {
    for (k, v) in map {
        *result.entry(k).or_insert(0) += v;
    }
}
```

See `advent_of_code/aoc2024/examples/day22_rayon_benchmark.rs` for complete benchmark code.

---

## Adding Rayon to Your Project

```toml
# Cargo.toml
[dependencies]
rayon = "1.10"
```

```rust
// Import the prelude for all parallel iterator methods
use rayon::prelude::*;
```

---

## Integrator Perspective

Think of Rayon like **AUTOSAR parallel runnables**:

| Rayon Concept | AUTOSAR Analogy |
|---------------|-----------------|
| Thread pool | RTE with multiple cores |
| `.par_iter()` | Distributing runnables across cores |
| Work stealing | Dynamic load balancing |
| `collect()` | Barrier synchronization |

**Key insight**: You don't manually manage threads—you declare *what* should be parallel, and the runtime handles *how*.
### Mission Library Composition

Rayon composes beautifully with mission data structures:

```rust
use mission6::{Grid, Coord};
use rayon::prelude::*;

// Parallel grid processing with Mission 6
fn process_grid_parallel(grid: &Grid<i32>) -> i32 {
    (0..grid.height())
        .into_par_iter()
        .map(|row| {
            (0..grid.width())
                .map(|col| {
                    let coord = Coord::new(row, col);
                    grid.get(coord).copied().unwrap_or(0)
                })
                .sum::<i32>()
        })
        .sum()
}
```

**Integrator approach**: Compose validated mission components (Grid, Graph, UnionFind) with Rayon's parallel patterns.
---

## Related Concepts

### Concurrency Foundations
- [[rust-threading-basics]] - Manual thread management (when Rayon isn't enough)
- [[async-vs-threads-decision]] - Choosing the right concurrency model
- [[rust-concurrency-moc]] - Map of all concurrency topics

### Thread Safety
- [[sync-send-traits]] - Why Rayon requires `Send` + `Sync`
- [[shared-state-concurrency]] - When parallel tasks need shared data

### AoC Patterns
- [[aoc-optimization-strategies]] - Performance optimization techniques
- [[AoC Pattern Library]] - Reusable solution patterns
- **AoC 2024 Day 14** - Robot simulation with parallel safety factor calculation (see `advent_of_code/aoc2024/examples/day14_rayon_learning.rs`)
- **AoC 2024 Day 22** - PRNG simulation with 16.58x speedup + HashMap aggregation patterns (see `advent_of_code/aoc2024/examples/day22_rayon_benchmark.rs`)

---

*Tags: #rayon #parallelism #iterators #data-parallelism #performance #aoc-optimization #concurrency*

*Links: [[zettel-index]] | [[rust-concurrency-moc]] | [[rust-threading-basics]] | [[async-vs-threads-decision]] | [[sync-send-traits]] | [[aoc-optimization-strategies]]*
