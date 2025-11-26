# Performance Engineering - Systematic Optimization for Rust Applications

## Core Concept

**Performance Engineering** is the discipline of systematically measuring, analyzing, and optimizing software performance. In Rust, this encompasses understanding zero-cost abstractions, leveraging the ownership system for memory efficiency, applying algorithmic optimizations, and using profiling tools to identify bottlenecks.

## Mental Models

### The Optimization Hierarchy

Think of performance engineering as a pyramid with layers of increasing specificity:

```
                    ┌─────────────────┐
                    │   Micro-opts    │  ← CPU cache, SIMD, inline
                    ├─────────────────┤
                    │  Data Structures│  ← Right container for the job
                    ├─────────────────┤
                    │   Algorithms    │  ← O(n) vs O(n²) vs O(log n)
                    ├─────────────────┤
                    │   Architecture  │  ← System design, parallelism
                    └─────────────────┘
                         Foundation
```

**Key Insight**: Always optimize from the bottom up. Micro-optimizations on a poor algorithm waste effort.

### The Integrator's Performance Perspective

As an integrator, think of performance engineering as **component profiling and bottleneck identification**:

- **Profile first** - Measure where time is actually spent
- **Optimize the critical path** - Focus on the hot 10% of code
- **Validate improvements** - Benchmark before and after changes
- **Document trade-offs** - Performance vs. readability vs. maintainability

## Detailed Content

### 1. Algorithmic Complexity

The most impactful performance gains come from algorithmic improvements:

```rust
// O(n²) - Nested loops
fn find_duplicates_slow(items: &[i32]) -> Vec<i32> {
    let mut duplicates = Vec::new();
    for i in 0..items.len() {
        for j in (i + 1)..items.len() {
            if items[i] == items[j] && !duplicates.contains(&items[i]) {
                duplicates.push(items[i]);
            }
        }
    }
    duplicates
}

// O(n) - HashSet for O(1) lookups
fn find_duplicates_fast(items: &[i32]) -> Vec<i32> {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();
    
    for &item in items {
        if !seen.insert(item) {
            duplicates.insert(item);
        }
    }
    
    duplicates.into_iter().collect()
}
```

**Performance Impact**:

| Input Size | O(n²) | O(n) | Speedup |
|------------|-------|------|---------|
| 1,000 | 500ms | 0.5ms | 1,000× |
| 10,000 | 50s | 5ms | 10,000× |
| 100,000 | 83min | 50ms | 100,000× |

### 2. Data Structure Selection

Choosing the right data structure is critical for performance:

```rust
// Scenario: Frequent lookups by key
// ❌ Vec with linear search - O(n)
let items: Vec<(String, i32)> = vec![...];
let value = items.iter().find(|(k, _)| k == "target");

// ✅ HashMap with O(1) average lookup
use std::collections::HashMap;
let items: HashMap<String, i32> = HashMap::new();
let value = items.get("target");

// Scenario: Ordered iteration needed
// ✅ BTreeMap maintains sorted order - O(log n) ops
use std::collections::BTreeMap;
let ordered: BTreeMap<String, i32> = BTreeMap::new();
for (key, value) in &ordered {
    // Keys are in sorted order
}

// Scenario: Priority-based processing
// ✅ BinaryHeap for O(log n) push/pop of max element
use std::collections::BinaryHeap;
let mut heap = BinaryHeap::new();
heap.push(5);
let max = heap.pop(); // Always returns largest
```

### 3. Memory Optimization Patterns

Rust's ownership system enables powerful memory optimizations:

```rust
// Pre-allocation to avoid reallocations
fn process_large_dataset(size: usize) -> Vec<i32> {
    let mut results = Vec::with_capacity(size); // Single allocation
    for i in 0..size {
        results.push(i as i32);
    }
    results
}

// Buffer reuse pattern
fn process_multiple_inputs(inputs: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut buffer = Vec::new(); // Reusable buffer
    inputs.iter().map(|input| {
        buffer.clear(); // Reuse allocation
        buffer.extend(input.iter().map(|&x| x * 2));
        buffer.clone()
    }).collect()
}

// Avoid unnecessary cloning
fn transform_borrowed(data: &[String]) -> Vec<&str> {
    data.iter().map(|s| s.as_str()).collect() // Borrows, no clone
}
```

### 4. Iterator Optimization

Rust iterators are zero-cost abstractions:

```rust
// Lazy evaluation - work happens only when needed
let processed: Vec<i32> = (0..1_000_000)
    .filter(|&x| x % 2 == 0)    // No intermediate collection
    .map(|x| x * x)              // Chained transformations
    .take(100)                   // Early termination
    .collect();                  // Single allocation

// Parallel iteration with Rayon
use rayon::prelude::*;
let sum: i64 = (0..1_000_000i64)
    .into_par_iter()             // Parallel execution
    .map(|x| x * x)
    .sum();
```

### 5. Cache-Friendly Patterns

Modern CPUs rely heavily on cache efficiency:

```rust
// Structure of Arrays (SoA) - better cache locality
struct ParticlesSoA {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
    mass: Vec<f32>,
}

// Update all x positions - sequential memory access
fn update_x_positions(particles: &mut ParticlesSoA, dt: f32) {
    for x in &mut particles.x {
        *x += dt;
    }
}

// vs Array of Structures (AoS) - cache misses when accessing single field
struct Particle { x: f32, y: f32, z: f32, mass: f32 }
struct ParticlesAoS(Vec<Particle>);
```

### 6. Symmetry Exploitation

Mathematical symmetries can dramatically reduce search spaces:

```rust
// TSP with rotational symmetry - fix first element
fn tsp_rotation_optimized<T: Clone>(items: &[T]) -> Vec<T> {
    let fixed = items[0].clone();
    let remaining = &items[1..];
    
    // Search (n-1)! instead of n! permutations
    // For n=8: 5,040 vs 40,320 (8× speedup)
    generate_permutations(remaining)
        .map(|perm| {
            let mut full = vec![fixed.clone()];
            full.extend(perm);
            full
        })
        .min_by_key(|route| calculate_cost(route))
        .unwrap()
}

// Full circular symmetry optimization
fn tsp_full_optimization<T: Clone>(items: &[T]) -> Vec<T> {
    // Fix first two elements to eliminate rotational AND reflectional symmetry
    // For n=8: 720 vs 40,320 (56× speedup)
    let fixed1 = items[0].clone();
    let fixed2 = items[1].clone();
    let remaining = &items[2..];
    // ... permute only remaining
}
```

### 7. Profiling and Measurement

Always profile before optimizing:

```rust
// Criterion benchmarking
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_algorithms(c: &mut Criterion) {
    let data = generate_test_data(10_000);
    
    c.bench_function("algorithm_v1", |b| {
        b.iter(|| algorithm_v1(black_box(&data)))
    });
    
    c.bench_function("algorithm_v2", |b| {
        b.iter(|| algorithm_v2(black_box(&data)))
    });
}

criterion_group!(benches, benchmark_algorithms);
criterion_main!(benches);
```

**Profiling Tools**:

- `cargo flamegraph` - Visual profiling with flame graphs
- `perf` (Linux) - System-level performance analysis
- `cargo bench` - Criterion-based benchmarking
- `#[bench]` - Built-in benchmarking (nightly)

### 8. Concurrency for Performance

Leverage multiple cores for CPU-bound work:

```rust
use std::thread;
use std::sync::Arc;

// Parallel processing with threads
fn parallel_sum(data: &[i64]) -> i64 {
    let chunk_size = data.len() / num_cpus::get();
    let data = Arc::new(data.to_vec());
    
    let handles: Vec<_> = (0..num_cpus::get())
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let start = i * chunk_size;
                let end = if i == num_cpus::get() - 1 { data.len() } else { start + chunk_size };
                data[start..end].iter().sum::<i64>()
            })
        })
        .collect();
    
    handles.into_iter()
        .map(|h| h.join().unwrap())
        .sum()
}

// Or use Rayon for simpler parallel iteration
use rayon::prelude::*;
fn rayon_sum(data: &[i64]) -> i64 {
    data.par_iter().sum()
}
```

## AoC-Specific Performance Patterns

### Grid Problems

```rust
// Flat array storage for cache efficiency
struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y * self.width + x]  // Single index calculation
    }
    
    // Pre-compute neighbor indices for repeated access
    fn neighbor_indices(&self, idx: usize) -> [Option<usize>; 4] {
        let x = idx % self.width;
        let y = idx / self.width;
        [
            if y > 0 { Some(idx - self.width) } else { None },           // Up
            if y < self.height - 1 { Some(idx + self.width) } else { None }, // Down
            if x > 0 { Some(idx - 1) } else { None },                    // Left
            if x < self.width - 1 { Some(idx + 1) } else { None },      // Right
        ]
    }
}
```

### Memoization

```rust
use std::collections::HashMap;

fn fibonacci_memoized(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    
    let result = match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_memoized(n - 1, memo) + fibonacci_memoized(n - 2, memo),
    };
    
    memo.insert(n, result);
    result
}
```

### Early Termination

```rust
// Stop searching when solution found
fn find_path_optimized(grid: &Grid, start: Coord, end: Coord) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back((start, 0));
    visited.insert(start);
    
    while let Some((pos, dist)) = queue.pop_front() {
        if pos == end {
            return Some(dist);  // Early return on success
        }
        
        for neighbor in grid.neighbors(pos) {
            if visited.insert(neighbor) {
                queue.push_back((neighbor, dist + 1));
            }
        }
    }
    
    None
}
```

## Key Takeaways

1. **Profile First**: Never optimize without measuring - intuition about bottlenecks is often wrong
2. **Algorithm > Implementation**: A better algorithm beats micro-optimizations every time
3. **Know Your Data Structures**: Right container = right performance characteristics
4. **Leverage Zero-Cost Abstractions**: Rust iterators and generics have no runtime overhead
5. **Think About Memory**: Pre-allocate, reuse buffers, avoid unnecessary cloning
6. **Exploit Problem Structure**: Symmetries, early termination, and memoization provide huge wins
7. **Parallelize CPU-Bound Work**: Modern CPUs have many cores - use them

## Integration Points

### Builds On

- [[Memory Optimization]] - Memory-efficient patterns and allocation strategies
- [[zero-cost-abstractions]] - Understanding Rust's compile-time optimizations
- [[Iteration Patterns]] - Iterator chains and lazy evaluation

### Enables

- [[Symmetry in Algorithms]] - Mathematical optimization techniques
- [[TSP Algorithms]] - Combinatorial optimization implementations
- [[Graph Algorithms]] - Efficient graph traversal and pathfinding
- [[BFS Patterns]] - Optimized breadth-first search implementations

### Related Concepts

- [[Big-O Analysis]] - Theoretical complexity foundations
- [[Amortized Analysis]] - Average cost over operation sequences
- [[Benchmarking]] - Systematic performance measurement
- [[Concurrency Patterns]] - Parallel and async optimization
- [[Cache-Friendly Data Structures]] - CPU cache optimization

### Mission Applications

- **[[mission-5]]** - HashMap performance characteristics
- **[[mission-6]]** - Grid storage and pathfinding optimization
- **[[mission-8]]** - Graph algorithm efficiency

### AoC Applications

- [[AoC Patterns MOC]] - Problem-solving performance patterns
- [[day13_analysis]] - TSP symmetry optimization case study
- [[Grid Navigation Patterns]] - Efficient 2D traversal

---

*Created: 2025-11-25*
*Last Updated: 2025-11-25*

*Tags: #performance-engineering #optimization #algorithms #benchmarking #profiling #zero-cost-abstractions #cache-efficiency #parallelism #competitive-programming #pattern #intermediate*

*Links: [[zettel-index]] | [[Memory Optimization]] | [[Symmetry in Algorithms]] | [[TSP Algorithms]] | [[Graph Algorithms]] | [[Iteration Patterns]] | [[AoC Patterns MOC]] | [[mission-5]] | [[mission-6]] | [[mission-8]]*
