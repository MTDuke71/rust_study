# 🚀 Memory Optimization in Rust

*Strategies for reducing memory footprint and improving cache efficiency in competitive programming and data structure implementations*

---

## 🎯 **Core Concept**

Memory optimization in Rust involves minimizing heap allocations, improving cache locality, and choosing appropriate data representations to maximize performance. Unlike garbage-collected languages, Rust gives you fine-grained control over memory layout, making these optimizations highly effective.

**Key principle**: The fastest code is code that doesn't run. The best memory is memory you don't allocate.

---

## 🧠 **Mental Models**

### **The Memory Hierarchy Pyramid**

```
         ┌──────────┐
         │ Registers│  ~1 cycle
         ├──────────┤
         │ L1 Cache │  ~4 cycles
         ├──────────┤
         │ L2 Cache │  ~10 cycles
         ├──────────┤
         │ L3 Cache │  ~40 cycles
         ├──────────┤
         │   RAM    │  ~100 cycles
         ├──────────┤
         │   Disk   │  ~10,000,000 cycles
         └──────────┘
```

**Implication**: Keeping data together in memory (cache-friendly) can be 25-100x faster than scattered allocations.

### **The Integrator Analogy**

Think of memory optimization like warehouse organization:

- **Stack**: Fast conveyor belt - items arrive and leave in order, no searching needed
- **Heap**: Large warehouse - flexible storage but requires tracking and retrieval time
- **Cache**: Workbench - keep frequently used tools within arm's reach

---

## 🔍 **Optimization Strategies**

### **1. Allocation Reduction**

#### **Pre-allocate Collections**

```rust
// ❌ Inefficient: Multiple reallocations as Vec grows
fn bad_collect() -> Vec<i32> {
    let mut results = Vec::new();
    for i in 0..10000 {
        results.push(i); // May trigger reallocation multiple times
    }
    results
}

// ✅ Efficient: Single allocation with known capacity
fn good_collect() -> Vec<i32> {
    let mut results = Vec::with_capacity(10000);
    for i in 0..10000 {
        results.push(i); // No reallocations
    }
    results
}

// ✅ Even better: Use iterator collection
fn best_collect() -> Vec<i32> {
    (0..10000).collect() // Rust optimizes this automatically
}
```

#### **Reuse Allocations**

```rust
// ❌ Allocates new Vec each iteration
fn process_rounds_bad(input: &[i32], rounds: usize) -> Vec<i32> {
    let mut data = input.to_vec();
    for _ in 0..rounds {
        data = data.iter().map(|x| x * 2).collect(); // New allocation!
    }
    data
}

// ✅ Reuse buffer with double-buffering pattern
fn process_rounds_good(input: &[i32], rounds: usize) -> Vec<i32> {
    let mut current = input.to_vec();
    let mut next = Vec::with_capacity(input.len());
    
    for _ in 0..rounds {
        next.clear(); // Reuse allocation
        next.extend(current.iter().map(|x| x * 2));
        std::mem::swap(&mut current, &mut next);
    }
    current
}
```

### **2. Data Structure Selection**

#### **Choose Appropriate Containers**

```rust
use std::collections::{HashMap, BTreeMap, HashSet};

// For small sets (< 50 elements), Vec can be faster than HashSet
fn small_set_membership(items: &[i32], target: i32) -> bool {
    items.contains(&target) // Linear but cache-friendly
}

// For large sets, HashSet is O(1) average
fn large_set_membership(items: &HashSet<i32>, target: i32) -> bool {
    items.contains(&target) // O(1) amortized
}

// BTreeMap when you need sorted iteration
fn range_queries(map: &BTreeMap<i32, String>, start: i32, end: i32) {
    for (k, v) in map.range(start..end) {
        println!("{}: {}", k, v);
    }
}
```

#### **Compact Representations**

```rust
// ❌ Wasteful: Using u64 when u8 suffices
struct WastefulCoord {
    x: u64, // 8 bytes, but range is 0-255
    y: u64, // 8 bytes
} // Total: 16 bytes

// ✅ Compact: Right-sized types
struct CompactCoord {
    x: u8, // 1 byte
    y: u8, // 1 byte
} // Total: 2 bytes (8x smaller!)

// ✅ Even more compact for grids: Pack into single integer
fn pack_coord(x: u8, y: u8) -> u16 {
    ((x as u16) << 8) | (y as u16)
}

fn unpack_coord(packed: u16) -> (u8, u8) {
    ((packed >> 8) as u8, (packed & 0xFF) as u8)
}
```

### **3. Cache-Friendly Patterns**

#### **Structure of Arrays vs Array of Structures**

```rust
// ❌ Array of Structures (AoS) - poor cache locality for single-field access
struct ParticleAoS {
    x: f64,
    y: f64,
    mass: f64,
    velocity: f64,
}
let particles_aos: Vec<ParticleAoS> = vec![/* ... */];

// Accessing all x values jumps through memory:
// [x1, y1, m1, v1, x2, y2, m2, v2, ...] <- scattered access

// ✅ Structure of Arrays (SoA) - excellent cache locality
struct ParticlesSoA {
    xs: Vec<f64>,
    ys: Vec<f64>,
    masses: Vec<f64>,
    velocities: Vec<f64>,
}

// Accessing all x values is contiguous:
// [x1, x2, x3, x4, ...] <- sequential access, cache-friendly
```

#### **Flat Storage for Grids**

```rust
// ❌ Vec<Vec<T>> - Each row is a separate allocation
struct ScatteredGrid<T> {
    data: Vec<Vec<T>>, // N+1 allocations, poor cache locality
}

// ✅ Flat Vec<T> with calculated indexing - Single allocation
struct FlatGrid<T> {
    data: Vec<T>,      // 1 allocation, excellent cache locality
    width: usize,
    height: usize,
}

impl<T> FlatGrid<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.data[y * self.width + x])
        } else {
            None
        }
    }
}
```

### **4. Zero-Copy Techniques**

#### **Borrowing Over Cloning**

```rust
// ❌ Unnecessary clone
fn process_string_bad(s: String) -> usize {
    s.len()
}
let data = String::from("hello");
let len = process_string_bad(data.clone()); // Wasteful allocation

// ✅ Borrow instead
fn process_string_good(s: &str) -> usize {
    s.len()
}
let len = process_string_good(&data); // Zero allocation
```

#### **Cow (Copy-on-Write)**

```rust
use std::borrow::Cow;

// Only allocates when modification is needed
fn maybe_modify(s: &str, should_modify: bool) -> Cow<str> {
    if should_modify {
        Cow::Owned(s.to_uppercase()) // Allocates only when needed
    } else {
        Cow::Borrowed(s) // Zero allocation
    }
}
```

---

## 📊 **AoC-Specific Optimizations**

### **Grid Problems**

```rust
// Optimized visited tracking for grid BFS/DFS
// Instead of HashSet<(usize, usize)> - heap allocated, hashing overhead
// Use flat Vec<bool> - stack-friendly, direct indexing

struct VisitedGrid {
    data: Vec<bool>,
    width: usize,
}

impl VisitedGrid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![false; width * height],
            width,
        }
    }
    
    fn mark(&mut self, x: usize, y: usize) {
        self.data[y * self.width + x] = true;
    }
    
    fn is_visited(&self, x: usize, y: usize) -> bool {
        self.data[y * self.width + x]
    }
}
```

### **String Processing**

```rust
// For look-and-say or similar string growth problems
// Pre-allocate based on expected growth

fn look_and_say_optimized(input: &str, iterations: usize) -> String {
    let mut current = input.to_string();
    // Each iteration roughly increases size by 30%
    let estimated_final_size = input.len() * (1.3_f64.powi(iterations as i32) as usize);
    
    let mut next = String::with_capacity(estimated_final_size);
    
    for _ in 0..iterations {
        next.clear();
        // ... process current into next
        std::mem::swap(&mut current, &mut next);
    }
    current
}
```

### **Memoization with Bounded Cache**

```rust
use std::collections::HashMap;

// When memoization cache grows too large, consider:
// 1. LRU eviction
// 2. Bounded capacity
// 3. State compression

fn memoized_with_limit<K, V>(
    cache: &mut HashMap<K, V>,
    key: K,
    compute: impl FnOnce() -> V,
    max_size: usize,
) -> V
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    if let Some(v) = cache.get(&key) {
        return v.clone();
    }
    
    // Simple eviction: clear when full (production code would use LRU)
    if cache.len() >= max_size {
        cache.clear();
    }
    
    let result = compute();
    cache.insert(key, result.clone());
    result
}
```

---

## 🔬 **Profiling and Measurement**

### **Memory Profiling Tools**

```bash
# Use cargo's built-in allocation tracking
RUSTFLAGS="-C instrument-coverage" cargo build

# Heap profiling with heaptrack (Linux)
heaptrack ./target/release/my_program

# Memory usage in benchmarks
cargo bench -- --profile-time 10
```

### **Quick Memory Measurement**

```rust
// Simple allocation counter for debugging
fn measure_vec_allocations() {
    let initial = std::mem::size_of::<Vec<i32>>();
    println!("Empty Vec size: {} bytes", initial);
    
    let v: Vec<i32> = Vec::with_capacity(1000);
    let capacity_bytes = v.capacity() * std::mem::size_of::<i32>();
    println!("Vec with 1000 capacity: {} bytes", capacity_bytes);
}
```

---

## 💡 **Key Takeaways**

1. **Pre-allocate** when size is known or estimable - avoids reallocation overhead
2. **Reuse buffers** in hot loops - swap instead of allocate
3. **Choose right-sized types** - u8 vs u64, i16 vs i32
4. **Prefer flat storage** - Vec<T> over Vec<Vec<T>> for cache locality
5. **Borrow don't clone** - use references and Cow for flexibility
6. **Profile before optimizing** - measure to find actual bottlenecks

---

## 🔗 **Integration Points**

### **Builds On**

- [[Memory Management]] - Fundamental ownership and allocation concepts
- [[Memory Safety]] - Rust's safety guarantees that enable fearless optimization
- [[Stack vs Heap]] - Understanding where data lives

### **Enables**

- [[Performance Optimization]] - General performance improvement strategies
- [[Parallel Processing]] - Memory-efficient concurrent algorithms
- [[Profiling Techniques]] - Measuring optimization impact

### **Related Concepts**

- [[BFS Patterns]] - Memory-efficient visited tracking
- [[DFS Patterns]] - Stack-based traversal optimization
- [[Memoization Patterns]] - Cache sizing and eviction
- [[Grid Data Structures]] - Flat storage implementations

### **Mission Applications**

- [[mission-5]] - HashMap memory layout optimization
- [[mission-6]] - Grid storage patterns and cache-friendly access
- [[mission-8]] - BFS/DFS visited set optimization
- [[mission-9]] - Priority queue and pathfinding memory patterns

### **AoC Applications**

- [[AoC Patterns MOC]] - Performance optimization section
- [[Run-Length Encoding]] - String growth and buffer management
- [[Connected Components]] - Efficient visited tracking
- [[Shortest Path]] - Memory-efficient graph representation

---

*Tags: #memory-optimization #performance #cache-efficiency #data-structures #competitive-programming #aoc #pattern #intermediate*

*Links: [[zettel-index]] | [[Memory Management]] | [[AoC Patterns MOC]] | [[graph-algorithms]] | [[Performance Optimization]] | [[mission-6]] | [[BFS Patterns]]*
