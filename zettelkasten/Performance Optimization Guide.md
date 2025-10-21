# Performance Optimization Guide

**Comprehensive guide to optimizing Rust code for competitive programming and production systems**

---

## 🎯 **Optimization Philosophy**

### **Measurement-Driven Approach**
1. **Profile First** - Never optimize without measuring
2. **Target Bottlenecks** - Focus on the highest-impact improvements  
3. **Validate Improvements** - Measure before and after changes
4. **Maintain Correctness** - Optimization should never break functionality
5. **Consider Readability** - Balance performance with maintainability

### **Rust-Specific Advantages**
- **Zero-Cost Abstractions** - High-level code compiles to optimal machine code
- **Ownership System** - Eliminates garbage collection overhead
- **Trait System** - Monomorphization enables aggressive optimization
- **Memory Safety** - No runtime bounds checking overhead in release builds
- **LLVM Backend** - Sophisticated compiler optimizations

---

## 🏗️ **Algorithmic Optimization**

### **Algorithm Selection Priority**
```rust
// ❌ O(n²) - Nested loops
fn slow_search(haystack: &[i32], needles: &[i32]) -> Vec<bool> {
    needles.iter().map(|needle| haystack.contains(needle)).collect()
}

// ✅ O(n + m) - Hash set preprocessing
fn fast_search(haystack: &[i32], needles: &[i32]) -> Vec<bool> {
    let set: HashSet<&i32> = haystack.iter().collect();
    needles.iter().map(|needle| set.contains(needle)).collect()
}
```

### **Data Structure Impact**
| Operation | Vec | HashMap | BTreeMap | HashSet |
|-----------|-----|---------|----------|---------|
| **Insert** | O(1)* | O(1)* | O(log n) | O(1)* |
| **Lookup** | O(n) | O(1)* | O(log n) | O(1)* |
| **Iterate** | O(n) | O(n) | O(n) | O(n) |
| **Memory** | Compact | Sparse | Compact | Sparse |

*Amortized time complexity

### **Common Algorithm Improvements**
```rust
// Two-Sum: O(n²) → O(n) with HashMap
fn two_sum_optimized(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = seen.get(&complement) {
            return Some((j, i));
        }
        seen.insert(num, i);
    }
    None
}

// Frequency counting: Multiple passes → Single pass
fn count_frequencies(text: &str) -> HashMap<char, usize> {
    text.chars().fold(HashMap::new(), |mut acc, ch| {
        *acc.entry(ch).or_insert(0) += 1;
        acc
    })
}
```

---

## 🧠 **Memory Optimization**

### **Allocation Strategies**
```rust
// ❌ Repeated allocations
fn slow_string_building(words: &[&str]) -> String {
    let mut result = String::new();
    for word in words {
        result = result + word + " "; // New allocation each time!
    }
    result
}

// ✅ Pre-allocated capacity
fn fast_string_building(words: &[&str]) -> String {
    let total_len: usize = words.iter().map(|s| s.len() + 1).sum();
    let mut result = String::with_capacity(total_len);
    for word in words {
        result.push_str(word);
        result.push(' ');
    }
    result.trim_end().to_string()
}

// ✅ Collection pre-sizing
fn optimized_collection_building(input_size: usize) -> Vec<ProcessedItem> {
    let mut result = Vec::with_capacity(input_size);
    // Avoid reallocation during growth
    // ... populate result
    result
}
```

### **Memory Layout Considerations**
```rust
// ❌ Poor cache locality - pointer chasing
struct BadNode {
    value: i32,
    children: Vec<Box<BadNode>>,
}

// ✅ Better cache locality - flat storage
struct GoodNode {
    value: i32,
    children_indices: Vec<usize>,
}
struct Arena {
    nodes: Vec<GoodNode>,
}

// ✅ Zero-allocation iteration
fn process_large_dataset(data: &[i32]) -> i32 {
    data.iter()                    // No allocation - iterator
        .filter(|&&x| x % 2 == 0)  // Lazy evaluation
        .map(|&x| x * x)           // Still no allocation
        .sum()                     // Single result
}
```

### **Stack vs Heap Allocation**
```rust
// ❌ Heap allocation for small, known-size data
fn slow_small_buffer() -> Vec<u8> {
    vec![0; 64] // Heap allocated
}

// ✅ Stack allocation for small, fixed-size data  
fn fast_small_buffer() -> [u8; 64] {
    [0; 64] // Stack allocated
}

// ✅ Smallvec for size optimization
use smallvec::{SmallVec, smallvec};
fn optimal_small_buffer() -> SmallVec<[u8; 64]> {
    smallvec![0; 32] // Stack if ≤64 elements, heap otherwise
}
```

---

## ⚡ **Iterator Optimization**

### **Iterator vs Loop Performance**
```rust
// Both compile to identical optimized code
fn functional_sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum() // Often fastest - LLVM optimization
}

fn imperative_sum(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in numbers {
        sum += num;
    }
    sum
}

// Iterator chains - zero-cost abstractions
fn complex_processing(numbers: &[i32]) -> Vec<i32> {
    numbers.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .filter(|&x| x < 100)
        .collect()
}
```

### **Lazy vs Eager Evaluation**
```rust
// ❌ Eager - creates intermediate collections
fn eager_processing(data: &[String]) -> Vec<String> {
    let filtered: Vec<_> = data.iter()
        .filter(|s| s.len() > 5)
        .collect(); // Intermediate allocation
    
    filtered.iter()
        .map(|s| s.to_uppercase())
        .collect() // Final allocation
}

// ✅ Lazy - single allocation at the end
fn lazy_processing(data: &[String]) -> Vec<String> {
    data.iter()
        .filter(|s| s.len() > 5)
        .map(|s| s.to_uppercase())
        .collect() // Only allocation
}
```

### **Collect vs Extend Performance**
```rust
// ❌ Repeated collections and concatenation
fn slow_aggregation(groups: &[Vec<i32>]) -> Vec<i32> {
    let mut result = Vec::new();
    for group in groups {
        let processed: Vec<i32> = group.iter().map(|&x| x * 2).collect();
        result.extend(processed); // Potential reallocation each time
    }
    result
}

// ✅ Single collection with size hint
fn fast_aggregation(groups: &[Vec<i32>]) -> Vec<i32> {
    let total_len: usize = groups.iter().map(|g| g.len()).sum();
    let mut result = Vec::with_capacity(total_len);
    
    for group in groups {
        result.extend(group.iter().map(|&x| x * 2));
    }
    result
}
```

---

## 📊 **Collection-Specific Optimizations**

### **HashMap Optimization**
```rust
use std::collections::HashMap;
use rustc_hash::FxHashMap; // Faster hasher for integers

// ✅ Capacity pre-allocation
fn optimized_frequency_count(items: &[String]) -> HashMap<String, usize> {
    let mut counts = HashMap::with_capacity(items.len() / 2); // Estimate
    for item in items {
        *counts.entry(item.clone()).or_insert(0) += 1;
    }
    counts
}

// ✅ FxHashMap for integer keys (faster than default)
fn fast_coordinate_tracking(moves: &[(i32, i32)]) -> FxHashMap<(i32, i32), usize> {
    let mut visited = FxHashMap::default();
    for (i, &pos) in moves.iter().enumerate() {
        *visited.entry(pos).or_insert(0) += 1;
    }
    visited
}

// ✅ Entry API for complex operations
fn efficient_grouping(items: &[(String, i32)]) -> HashMap<String, Vec<i32>> {
    let mut groups = HashMap::new();
    for (key, value) in items {
        groups.entry(key.clone()).or_insert_with(Vec::new).push(*value);
    }
    groups
}
```

### **Vec Optimization**
```rust
// ✅ Reserve vs with_capacity
fn build_large_vec(input: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    result.reserve(input.len() * 2); // If you know approximate size
    
    for &item in input {
        result.push(item);
        if item > 100 {
            result.push(item * 2); // Might exceed capacity
        }
    }
    result
}

// ✅ Drain for efficient element removal
fn remove_negatives(vec: &mut Vec<i32>) {
    vec.retain(|&x| x >= 0); // More efficient than manual removal
}

// ✅ Swap removal for unordered data
fn fast_removal_unordered(vec: &mut Vec<i32>, index: usize) {
    vec.swap_remove(index); // O(1) vs O(n) for remove()
}
```

### **String Optimization**
```rust
// ✅ String capacity management
fn build_config_string(pairs: &[(&str, &str)]) -> String {
    let estimated_len: usize = pairs.iter()
        .map(|(k, v)| k.len() + v.len() + 3) // "key=value\n"
        .sum();
    
    let mut config = String::with_capacity(estimated_len);
    for (key, value) in pairs {
        config.push_str(key);
        config.push('=');
        config.push_str(value);
        config.push('\n');
    }
    config
}

// ✅ Avoid string allocation for temporary comparisons
fn efficient_string_processing(input: &str) -> Vec<&str> {
    input.lines()
        .filter(|line| line.starts_with("prefix_")) // No allocation
        .filter(|line| line.len() > 10)
        .collect()
}
```

---

## 🎲 **AoC-Specific Optimizations**

### **Input Parsing Performance**
```rust
// ✅ Efficient line-by-line processing
fn parse_aoc_input(input: &str) -> Vec<(i32, i32)> {
    input.lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            Some((
                parts.next()?.parse().ok()?,
                parts.next()?.parse().ok()?,
            ))
        })
        .collect()
}

// ✅ Batch parsing with pre-allocation
fn batch_parse_numbers(input: &str) -> Vec<Vec<i32>> {
    let line_count = input.lines().count();
    let mut result = Vec::with_capacity(line_count);
    
    for line in input.lines() {
        if line.trim().is_empty() { continue; }
        
        let numbers: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        result.push(numbers);
    }
    result
}
```

### **Grid Processing Optimization**
```rust
// ✅ Flat array for 2D grid (better cache performance)
struct OptimizedGrid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> OptimizedGrid<T> {
    fn new(width: usize, height: usize, default: T) -> Self 
    where T: Clone {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }
    
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            self.data.get(y * self.width + x)
        } else {
            None
        }
    }
    
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            self.data.get_mut(y * self.width + x)
        } else {
            None
        }
    }
}

// ✅ Neighbor iteration without allocation
fn get_neighbors(x: usize, y: usize, width: usize, height: usize) 
    -> impl Iterator<Item = (usize, usize)> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter_map(move |(dx, dy)| {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx < width && ny < height { Some((nx, ny)) } else { None }
        })
}
```

### **State Space Search Optimization**
```rust
use std::collections::{VecDeque, HashSet};

// ✅ BFS with visited set and capacity hints
fn optimized_bfs<State>(start: State, is_goal: impl Fn(&State) -> bool,
                       get_neighbors: impl Fn(&State) -> Vec<State>) -> Option<usize>
where State: Clone + Eq + std::hash::Hash {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back((start.clone(), 0));
    visited.insert(start);
    
    while let Some((state, depth)) = queue.pop_front() {
        if is_goal(&state) { return Some(depth); }
        
        for neighbor in get_neighbors(&state) {
            if visited.insert(neighbor.clone()) {
                queue.push_back((neighbor, depth + 1));
            }
        }
    }
    None
}
```

---

## 🔧 **Profiling and Measurement**

### **Benchmarking with Criterion**
```rust
// Cargo.toml
// [dev-dependencies]
// criterion = "0.5"

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn benchmark_collections(c: &mut Criterion) {
    let mut group = c.benchmark_group("collection_comparison");
    
    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::new("HashMap", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut map = HashMap::new();
                    for i in 0..size {
                        map.insert(i, i * 2);
                    }
                    map
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("Vec", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let mut vec = Vec::with_capacity(size);
                    for i in 0..size {
                        vec.push((i, i * 2));
                    }
                    vec
                });
            },
        );
    }
}

criterion_group!(benches, benchmark_collections);
criterion_main!(benches);
```

### **Memory Usage Analysis**
```rust
// Use heaptrack, valgrind, or built-in allocation tracking

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn profile_memory_usage() {
    let _profiler = dhat::Profiler::new_heap();
    
    // Your code here - memory allocations will be tracked
    let mut data = Vec::with_capacity(1000);
    for i in 0..1000 {
        data.push(i * i);
    }
    
    // Profile report generated when _profiler drops
}
```

### **Compile-Time Optimization**
```rust
// Cargo.toml release profile
[profile.release]
opt-level = 3          # Maximum optimization
lto = true             # Link-time optimization  
codegen-units = 1      # Better optimization, slower compile
panic = "abort"        # Smaller binary, no unwinding
overflow-checks = false # Disable integer overflow checks

// Target-specific optimization
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "target-cpu=native"] # Use all CPU features
```

---

## 🚀 **Quick Optimization Checklist**

### **Algorithm Level**
- [ ] Choose optimal algorithm for problem size and constraints
- [ ] Consider time vs space tradeoffs  
- [ ] Use appropriate data structures (HashMap vs Vec vs BTreeMap)
- [ ] Minimize algorithmic complexity (nested loops → hash lookups)

### **Implementation Level**
- [ ] Pre-allocate collections with known or estimated sizes
- [ ] Use iterator chains instead of intermediate collections
- [ ] Prefer `&str` over `String` when possible
- [ ] Use entry API for complex HashMap operations
- [ ] Consider `smallvec` for small, variable-size data

### **Memory Level**  
- [ ] Minimize allocations in hot loops
- [ ] Reuse buffers with `.clear()` instead of creating new ones
- [ ] Use stack allocation for small, fixed-size data
- [ ] Consider flat data structures over pointer-based ones
- [ ] Profile memory usage for large datasets

### **Compilation Level**
- [ ] Use release profile for benchmarking
- [ ] Enable LTO for final optimization
- [ ] Consider target-specific optimization flags
- [ ] Profile with realistic input sizes and patterns

**See Also**: [[../../tutorials/Mission4_tut/compilation_stages/VISUAL_COMPILATION_PROCESS]] - Visual guide showing Rust source → LLVM IR → Assembly → Machine code transformation with memory layout diagrams

---

*Tags: #performance #optimization #algorithms #memory #benchmarking #profiling #competitive-programming #rust-specific*
*Links: [[zettel-index]] | [[AoC Collection Problems]] | [[Rust Collections MOC]] | [[AoC Patterns MOC]] | [[../missions/Mission8/PERFORMANCE_REPORT]] | [[../tutorials/Mission8_tut/DAY4_EXERCISE_SOLUTIONS]] | [[mission8_overview]] | [[Algorithm Complexity Analysis]] | [[../advent_of_code/aoc2015/examples/day14_analysis]] | [[../advent_of_code/aoc2015/examples/GRAPHICS_GUIDE]]*