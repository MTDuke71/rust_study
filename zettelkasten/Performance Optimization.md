# Performance Optimization

## 🎯 Overview

**Performance optimization** in Rust involves systematically improving the speed, memory usage, and efficiency of your code while maintaining correctness and safety. Rust's zero-cost abstractions provide a solid foundation, but understanding when and how to optimize is crucial for building high-performance applications.

**Key Philosophy:** Measure first, optimize second. Rust's compiler is excellent at optimization, but understanding the bottlenecks and applying targeted improvements can yield significant performance gains.

---

## 🔍 Performance Analysis Fundamentals

### 1. **Profiling Before Optimizing**

```rust
// ❌ BAD: Optimizing without measuring
fn process_data_naive(data: &[i32]) -> i32 {
    data.iter().map(|x| x * 2).sum()
}

// ✅ GOOD: Profile first, then optimize
fn process_data_optimized(data: &[i32]) -> i32 {
    // After profiling showed this was the bottleneck
    let mut sum = 0;
    for &x in data {
        sum += x * 2;
    }
    sum
}
```

### 2. **Benchmarking Tools**

```rust
// Cargo bench setup
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_sum_operations(c: &mut Criterion) {
    let data: Vec<i32> = (1..1000).collect();
    
    c.bench_function("iter_sum", |b| {
        b.iter(|| {
            black_box(data.iter().map(|x| x * 2).sum::<i32>())
        })
    });
    
    c.bench_function("for_loop", |b| {
        b.iter(|| {
            let mut sum = 0;
            for &x in &data {
                sum += x * 2;
            }
            black_box(sum)
        })
    });
}

criterion_group!(benches, bench_sum_operations);
criterion_main!(benches);
```

**Run with:** `cargo bench`

---

## 🚀 Algorithm Complexity Optimization

### **Big-O Analysis and Real-World Impact**

| Algorithm | Time Complexity | Space | When to Use |
|-----------|----------------|-------|-------------|
| **HashMap lookup** | O(1) average | O(n) | Fast key-value access |
| **BTreeMap lookup** | O(log n) | O(n) | Ordered data needed |
| **Vec linear search** | O(n) | O(1) | Small datasets |
| **Binary search** | O(log n) | O(1) | Sorted data |
| **Sorting** | O(n log n) | O(1) | One-time sort for multiple queries |

### **Choosing the Right Data Structure**

```rust
// ❌ BAD: Linear search on large dataset
fn find_user_linear(users: &[User], id: u32) -> Option<&User> {
    users.iter().find(|u| u.id == id)  // O(n)
}

// ✅ GOOD: HashMap for O(1) lookup
use std::collections::HashMap;

fn find_user_hashmap(users: &HashMap<u32, User>, id: u32) -> Option<&User> {
    users.get(&id)  // O(1)
}

// ✅ GOOD: BTreeMap for ordered data
use std::collections::BTreeMap;

fn find_user_btree(users: &BTreeMap<u32, User>, id: u32) -> Option<&User> {
    users.get(&id)  // O(log n)
}
```

---

## 💾 Memory Optimization Strategies

### 1. **Avoiding Unnecessary Allocations**

```rust
// ❌ BAD: Multiple allocations
fn process_strings_bad(input: &str) -> Vec<String> {
    input.split_whitespace()
        .map(|s| s.to_uppercase())  // New String allocation
        .collect()
}

// ✅ GOOD: Reuse allocations
fn process_strings_good(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    for word in input.split_whitespace() {
        result.push(word.to_uppercase());
    }
    result
}

// ✅ BETTER: Use string slices when possible
fn process_strings_best(input: &str) -> Vec<&str> {
    input.split_whitespace()
        .filter(|s| s.len() > 3)
        .collect()  // No allocations!
}
```

### 2. **Pre-allocating Collections**

```rust
// ❌ BAD: Dynamic growth
fn build_list_naive(items: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();  // Starts with capacity 0
    for &item in items {
        result.push(item * 2);  // May reallocate multiple times
    }
    result
}

// ✅ GOOD: Pre-allocate capacity
fn build_list_optimized(items: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(items.len());
    for &item in items {
        result.push(item * 2);  // No reallocations
    }
    result
}

// ✅ BEST: Iterator chain (often optimized by compiler)
fn build_list_best(items: &[i32]) -> Vec<i32> {
    items.iter().map(|&x| x * 2).collect()
}
```

### 3. **String Optimization**

```rust
// ❌ BAD: String concatenation in loop
fn build_message_bad(parts: &[&str]) -> String {
    let mut result = String::new();
    for part in parts {
        result.push_str(part);  // May reallocate
    }
    result
}

// ✅ GOOD: Pre-calculate capacity
fn build_message_good(parts: &[&str]) -> String {
    let total_len = parts.iter().map(|s| s.len()).sum();
    let mut result = String::with_capacity(total_len);
    for part in parts {
        result.push_str(part);
    }
    result
}

// ✅ BEST: Use join for concatenation
fn build_message_best(parts: &[&str]) -> String {
    parts.join("")
}
```

---

## ⚡ CPU Optimization Techniques

### 1. **Bounds Checking Elimination**

```rust
// From bounds-checking-performance.md context
// ❌ BAD: Bounds check in hot loop
fn process_grid_naive(grid: &[[i32; 8]; 8], x: usize, y: usize) -> i32 {
    if x < 8 && y < 8 {  // Runtime bounds check
        grid[y][x]
    } else {
        0
    }
}

// ✅ GOOD: Trust the type system
fn process_grid_optimized(grid: &[[i32; 8]; 8], pos: (usize, usize)) -> i32 {
    let (x, y) = pos;
    debug_assert!(x < 8 && y < 8);  // Debug-only check
    grid[y][x]
}

// ✅ BEST: Use unsafe when you can prove safety
fn process_grid_unsafe(grid: &[[i32; 8]; 8], x: usize, y: usize) -> i32 {
    unsafe { *grid.get_unchecked(y).get_unchecked(x) }
}
```

### 2. **Branch Prediction Optimization**

```rust
// ❌ BAD: Unpredictable branches
fn process_data_unpredictable(data: &[i32]) -> i32 {
    let mut sum = 0;
    for &x in data {
        if x % 2 == 0 {  // Unpredictable pattern
            sum += x;
        }
    }
    sum
}

// ✅ GOOD: Sort data by branch condition
fn process_data_predictable(mut data: Vec<i32>) -> i32 {
    data.sort_by_key(|&x| x % 2);  // Group by even/odd
    let mut sum = 0;
    for &x in data {
        if x % 2 == 0 {  // Predictable pattern
            sum += x;
        }
    }
    sum
}

// ✅ BEST: Separate processing paths
fn process_data_optimal(data: &[i32]) -> i32 {
    data.iter()
        .filter(|&&x| x % 2 == 0)
        .sum()
}
```

### 3. **Cache-Friendly Data Layout**

```rust
// ❌ BAD: Array of structs (AoS) - poor cache locality
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

fn process_points_aos(points: &[Point3D]) -> f32 {
    points.iter().map(|p| p.x + p.y + p.z).sum()
}

// ✅ GOOD: Structure of arrays (SoA) - better cache locality
struct Points3D {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
}

fn process_points_soa(points: &Points3D) -> f32 {
    let mut sum = 0.0;
    for i in 0..points.x.len() {
        sum += points.x[i] + points.y[i] + points.z[i];
    }
    sum
}
```

---

## 🔧 Compiler Optimization

### 1. **Release Build Optimization**

```toml
# Cargo.toml
[profile.release]
opt-level = 3          # Maximum optimization
lto = true            # Link-time optimization
codegen-units = 1     # Better optimization (slower compile)
panic = "abort"       # Smaller binaries
strip = true          # Remove debug symbols
```

### 2. **Function Inlining**

```rust
// ✅ GOOD: Mark hot functions for inlining
#[inline(always)]
fn hot_function(x: i32) -> i32 {
    x * x + x
}

// ✅ GOOD: Conditional inlining
#[inline(never)]
fn cold_function(x: i32) -> i32 {
    // Complex function that's rarely called
    expensive_computation(x)
}

// ✅ GOOD: Let compiler decide (default)
fn normal_function(x: i32) -> i32 {
    x + 1
}
```

### 3. **SIMD Optimization**

```rust
// Enable SIMD for vector operations
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn simd_sum(data: &[f32]) -> f32 {
    #[cfg(target_arch = "x86_64")]
    {
        unsafe {
            let mut sum = _mm256_setzero_ps();
            for chunk in data.chunks(8) {
                if chunk.len() == 8 {
                    let values = _mm256_load_ps(chunk.as_ptr());
                    sum = _mm256_add_ps(sum, values);
                }
            }
            // Extract sum from SIMD register
            let mut result = [0.0f32; 8];
            _mm256_store_ps(result.as_mut_ptr(), sum);
            result.iter().sum()
        }
    }
    
    #[cfg(not(target_arch = "x86_64"))]
    {
        data.iter().sum()
    }
}
```

---

## 📊 Collection-Specific Optimizations

### **HashMap Performance**

```rust
use std::collections::HashMap;

// ✅ GOOD: Pre-allocate HashMap with expected capacity
fn build_map_optimized(items: &[(String, i32)]) -> HashMap<String, i32> {
    let mut map = HashMap::with_capacity(items.len());
    for (key, value) in items {
        map.insert(key.clone(), *value);
    }
    map
}

// ✅ GOOD: Use entry API to avoid double lookup
fn update_or_insert(map: &mut HashMap<String, i32>, key: String, value: i32) {
    map.entry(key)
        .and_modify(|v| *v += value)
        .or_insert(value);
}
```

### **Vec Performance**

```rust
// ✅ GOOD: Reserve capacity for known size
fn process_vectors(vectors: &[Vec<i32>]) -> Vec<i32> {
    let total_capacity: usize = vectors.iter().map(|v| v.len()).sum();
    let mut result = Vec::with_capacity(total_capacity);
    
    for v in vectors {
        result.extend(v);  // No reallocations
    }
    result
}

// ✅ GOOD: Use slice operations when possible
fn find_max_slice(data: &[i32]) -> Option<i32> {
    data.iter().max().copied()
}
```

### **String Performance**

```rust
// ✅ GOOD: Use String::with_capacity for known size
fn build_large_string(parts: &[&str]) -> String {
    let total_len: usize = parts.iter().map(|s| s.len()).sum();
    let mut result = String::with_capacity(total_len);
    
    for part in parts {
        result.push_str(part);
    }
    result
}

// ✅ GOOD: Use Cow<str> for conditional ownership
use std::borrow::Cow;

fn process_text(text: &str) -> Cow<str> {
    if text.contains("special") {
        Cow::Owned(text.replace("special", "unique"))
    } else {
        Cow::Borrowed(text)  // No allocation
    }
}
```

---

## 🎯 Mission-Specific Optimizations

### **Mission1: Stack Implementation**

```rust
// ✅ GOOD: Generic Stack with capacity
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
    
    // ✅ GOOD: Inline hot path methods
    #[inline]
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }
    
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}
```

### **Mission3: Iterator Patterns**

```rust
// ✅ GOOD: Optimized iterator chain
fn process_data_efficient(data: &[i32]) -> Vec<i32> {
    data.iter()
        .filter(|&&x| x > 0)      // Early filtering
        .map(|&x| x * 2)          // Transform
        .take(100)                // Limit early
        .collect()                // Single allocation
}

// ✅ GOOD: Parallel processing for large datasets
use rayon::prelude::*;

fn process_data_parallel(data: &[i32]) -> Vec<i32> {
    data.par_iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * 2)
        .collect()
}
```

### **Mission6: Pathfinding Optimization**

```rust
// ✅ GOOD: Pre-computed neighbor tables
static NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

fn get_neighbors_fast(pos: (i32, i32), grid: &Grid) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::with_capacity(8);
    
    for &(dx, dy) in &NEIGHBOR_OFFSETS {
        let new_pos = (pos.0 + dx, pos.1 + dy);
        if grid.is_valid(new_pos) {
            neighbors.push(new_pos);
        }
    }
    
    neighbors
}
```

---

## 🛠️ Profiling and Measurement Tools

### 1. **Cargo Flamegraph**

```bash
# Install flamegraph
cargo install flamegraph

# Profile your application
cargo flamegraph --bin your_app
```

### 2. **Perf (Linux)**

```bash
# Record performance data
perf record --call-graph dwarf ./target/release/your_app

# Analyze results
perf report
perf annotate
```

### 3. **Built-in Timing**

```rust
use std::time::Instant;

fn measure_performance() {
    let start = Instant::now();
    
    // Your code here
    let result = expensive_operation();
    
    let duration = start.elapsed();
    println!("Operation took: {:?}", duration);
}
```

---

## 📈 Performance Optimization Checklist

### **Before Optimizing**
- [ ] **Profile first** - Use `cargo bench` or profiling tools
- [ ] **Identify bottlenecks** - Focus on hot paths (80/20 rule)
- [ ] **Measure baseline** - Document current performance
- [ ] **Set performance goals** - Define success criteria

### **Algorithm Optimization**
- [ ] **Choose right data structure** - HashMap vs BTreeMap vs Vec
- [ ] **Optimize time complexity** - O(n²) → O(n log n) → O(n)
- [ ] **Reduce unnecessary work** - Early returns, filtering
- [ ] **Cache results** - Memoization, lookup tables

### **Memory Optimization**
- [ ] **Pre-allocate collections** - `Vec::with_capacity()`
- [ ] **Avoid unnecessary allocations** - Reuse buffers
- [ ] **Use references** - `&str` instead of `String` when possible
- [ ] **Optimize data layout** - Structure of arrays vs array of structures

### **CPU Optimization**
- [ ] **Enable release optimizations** - `opt-level = 3`
- [ ] **Inline hot functions** - `#[inline]` attributes
- [ ] **Eliminate bounds checks** - Use `get_unchecked()` in hot paths
- [ ] **Optimize branches** - Sort data by branch conditions

### **After Optimizing**
- [ ] **Verify correctness** - Tests still pass
- [ ] **Measure improvement** - Quantify performance gains
- [ ] **Document changes** - Explain why optimization was needed
- [ ] **Consider trade-offs** - Readability vs performance

---

## 🔗 Related Concepts

### **Core Performance Concepts**
- **[[Zero-Cost Abstractions]]** - How Rust achieves performance without runtime overhead
- **[[Generic Programming]]** - Monomorphization and compile-time optimization
- **[[Iterator Traits]]** - Efficient functional programming patterns
- **[[bounds-checking-performance]]** - Eliminating runtime checks in hot paths

### **Data Structure Performance**
- **[[HashMap Internals]]** - Hash table optimization strategies
- **[[Collections MOC]]** - Choosing the right collection for the job
- **[[Binary Search Iterator Patterns]]** - Efficient search algorithms
- **[[Deduplication]]** - Performance trade-offs in duplicate removal

### **Mission-Specific Optimization**
- **[[Mission1 Overview]]** - Stack implementation performance
- **[[Mission3 Overview]]** - Iterator and trait optimization
- **[[Mission6 Overview]]** - Pathfinding algorithm optimization
- **[[Chess Engine Architecture]]** - High-performance game engine patterns

### **Tools and Techniques**
- **[[Benchmarking Guide]]** - Measuring performance accurately
- **[[Profiling Techniques]]** - Finding bottlenecks in real applications
- **[[Unsafe Rust Guidelines]]** - When and how to use unsafe for performance
- **[[SIMD Optimization]]** - Vector processing for numerical code

---

## 🎓 Key Takeaways

1. **Measure First**: Always profile before optimizing - you might be optimizing the wrong thing
2. **Algorithm Matters Most**: O(n²) → O(n log n) beats micro-optimizations
3. **Cache is King**: Optimize for cache locality and memory access patterns
4. **Release Mode**: Debug builds hide the true performance characteristics
5. **Hot Paths**: Focus optimization efforts on the 20% of code that runs 80% of the time
6. **Trade-offs**: Every optimization has costs - readability, complexity, maintainability

---

*Tags: #performance #optimization #benchmarking #profiling #memory-optimization #cpu-optimization #algorithm-complexity #collections #zero-cost-abstractions*

*Links: [[zettel-index]] | [[Zero-Cost Abstractions]] | [[Generic Programming]] | [[Collections MOC]] | [[Bounds Checking Performance]] | [[Deduplication]] | [[Mission1 Overview]] | [[Mission3 Overview]] | [[Mission6 Overview]]*
