# Performance Benchmarking & Grid Optimization

## 🎯 Core Concept

Performance benchmarking is the systematic measurement of code execution time and resource usage to identify bottlenecks and validate optimizations. For grid data structures, the key trade-offs involve memory layout, cache utilization, and access patterns.

---

## 🔑 Key Principles

### **1. Memory Layout Matters More Than Algorithm**

The physical arrangement of data in memory often has greater performance impact than algorithmic complexity for grid operations:

- **Flat Vec<T>**: Single contiguous allocation, O(1) calculated indexing
- **Vec<Vec<T>>**: Multiple allocations, pointer chasing overhead
- **Cache locality**: Sequential data enables CPU prefetching

### **2. Pointer Chasing vs Direct Access**

**Pointer chasing** occurs when accessing data requires following multiple memory addresses:

```rust
// ❌ Vec<Vec<T>> - Pointer chasing (2 memory accesses)
let value = nested_grid[y][x];
//          ^^^^^^^^^^^  ← Access 1: Get row vector pointer
//                    ^  ← Access 2: Get element from row

// ✅ Vec<T> flat - Direct access (1 memory access)
let index = y * width + x;  // Pure math (CPU registers)
let value = flat_grid[index];  // Single memory access
```

**Why it matters**:
- Each pointer dereference is a memory access (slow)
- Index calculation is CPU arithmetic (fast)
- Sequential data enables cache prefetching
- Benchmark result: **1.5x faster** for flat layout

### **3. Access Patterns & Cache Effects**

CPU cache performance depends heavily on memory access patterns:

| Pattern | Description | Performance Impact |
|---------|-------------|-------------------|
| **Row-major** | Sequential `(x,y) → (x+1,y)` | ✅ Optimal - cache lines fully utilized |
| **Column-major** | Strided `(x,y) → (x,y+1)` | ⚠️ Slower - cache line misses |
| **Random** | Unpredictable jumps | ❌ Worst - cache thrashing |

**Release mode benchmarks** (1000x1000 grid):
- Row-major: ~100µs (baseline)
- Column-major: ~327µs (3.26x slower)
- Random access: ~771µs (7.67x slower)

---

## ⚠️ Common Benchmarking Pitfalls

### **1. Zero-Value Optimization**
```rust
// ❌ BAD: OS may use zero-page optimization
let grid = TutorialGrid::new(1000, 1000, 0i32);

// ✅ GOOD: Use non-zero values
let grid = TutorialGrid::new(1000, 1000, 1i32);
```

**Why**: Operating systems optimize zero-page allocation through:
- Lazy allocation (memory not allocated until written)
- Copy-on-write (all zero pages point to single physical page)
- Result: Artificially fast creation times

### **2. Dead Code Elimination**
```rust
// ❌ BAD: Compiler may eliminate unused computation
let sum = grid.iter().sum::<i32>();
// If sum is never used, entire loop may be optimized away

// ✅ GOOD: Prevent optimization with black_box
let sum = std::hint::black_box(grid.iter().sum::<i32>());
```

### **3. Single-Sample Timing**
```rust
// ❌ BAD: Single measurement affected by system noise
let start = Instant::now();
let grid = create_grid();
let duration = start.elapsed();

// ✅ GOOD: Multiple iterations with median
let mut times = vec![];
for _ in 0..10 {
    let start = Instant::now();
    let grid = create_grid();
    times.push(start.elapsed());
}
times.sort();
let median = times[times.len() / 2];
```

### **4. Suspiciously Fast Times**
Watch for physically impossible results:
- 1000x1000 grid faster than 500x500 → optimization artifact
- Sub-nanosecond operations → dead code elimination
- Identical times across size variations → lazy allocation

---

## 🛠️ Proper Benchmarking Techniques

### **Framework Structure**
```rust
use std::time::Instant;
use std::hint::black_box;

fn benchmark_operation<F>(iterations: usize, operation: F) -> Duration 
where
    F: Fn() -> ()
{
    let mut times = vec![];
    
    for _ in 0..iterations {
        let start = Instant::now();
        black_box(operation());  // Prevent optimization
        times.push(start.elapsed());
    }
    
    times.sort();
    times[times.len() / 2]  // Return median
}
```

### **Best Practices Checklist**
- ✅ Use non-zero test data
- ✅ Multiple iterations (10+ for creation, 5+ for operations)
- ✅ Calculate median (not average) to filter outliers
- ✅ Apply `black_box()` to prevent dead code elimination
- ✅ Verify results (ensure computation actually occurred)
- ✅ Test with realistic data sizes
- ✅ Measure on target hardware
- ✅ Use `--release` mode for production-realistic results

---

## 📊 Performance Characteristics

### **Grid Creation Scaling**
| Size | Elements | Time | Per 100 cells |
|------|----------|------|---------------|
| 10x10 | 100 | 500ns | 500ns |
| 100x100 | 10K | 5µs | 50ns |
| 500x500 | 250K | 150µs | 60ns |
| 1000x1000 | 1M | 600µs | 60ns |

**Analysis**: O(n) linear scaling, consistent per-cell time demonstrates no hidden overhead.

### **Memory Layout Comparison** (100x100 grid)
| Layout | Creation | Access | Speedup |
|--------|----------|--------|---------|
| Vec<Vec<T>> | ~15µs | ~22µs | Baseline |
| Vec<T> flat | ~3µs | ~13µs | **1.5-2x faster** |

---

## 🎯 Optimization Strategies

### **Memory Layout**
1. **Use flat Vec<T>** instead of Vec<Vec<T>>
   - Single allocation vs multiple allocations
   - Direct indexing vs pointer chasing
   - Better cache locality

2. **Choose appropriate data types**
   - `u8` for small values (4x more cache-efficient than `u32`)
   - `u32` for general purpose
   - Consider alignment for SIMD operations

### **Access Patterns**
1. **Prefer row-major traversal**
   ```rust
   // ✅ GOOD: Row-major (y outer, x inner)
   for y in 0..height {
       for x in 0..width {
           process(grid.get(Coord { x, y }));
       }
   }
   
   // ❌ BAD: Column-major (x outer, y inner)
   for x in 0..width {
       for y in 0..height {
           process(grid.get(Coord { x, y }));
       }
   }
   ```

2. **Process data in blocks/tiles** for cache efficiency
3. **Batch operations** to reduce function call overhead

### **Algorithm Choice**
- Consider spatial data structures for sparse grids (quadtree, R-tree)
- Use appropriate algorithms for problem size
- Profile memory allocation patterns

---

## 🔗 Related Concepts

- **[[Cache Locality]]** - How memory access patterns affect CPU cache utilization
- **[[SIMD Optimization]]** - Vectorized operations for grid processing
- **[[Memory Alignment]]** - Data structure layout for performance
- **[[Big-O Complexity]]** - Asymptotic analysis vs practical performance
- **[[Rust Zero-Cost Abstractions]]** - High-level code compiling to efficient machine code

---

## 📚 Learning Resources

### **Essential Reading**
- **[Algorithmica HPC](https://en.algorithmica.org/hpc/)** - Comprehensive resource on high-performance computing, covering:
  - Computer architecture fundamentals
  - Cache optimization techniques
  - SIMD programming
  - Compiler optimizations
  - Memory layout strategies
  - Practical benchmarking methods

### **Hands-On Examples**
- **Mission6_tut/examples/step6_performance.rs** - Complete benchmarking framework
  - Grid creation benchmarks
  - Cache pattern testing
  - Memory layout comparisons
  - Documented pitfalls and solutions

### **Key Concepts from Algorithmica**
- **Memory hierarchy**: L1/L2/L3 cache, RAM, storage
- **Cache lines**: 64-byte blocks, spatial locality
- **Prefetching**: Hardware and software techniques
- **Branch prediction**: Control flow optimization
- **SIMD instructions**: Processing multiple data elements simultaneously

---

## 💡 Practical Applications

### **Advent of Code Scenarios**
1. **Grid traversal** (Day 3, Day 11, Day 15)
   - Flat Vec layout for cache efficiency
   - Row-major iteration for optimal access

2. **Flood fill algorithms** (BFS/DFS)
   - Memory layout affects queue/stack performance
   - Coordinate iteration patterns matter

3. **Image processing**
   - Tile-based processing for cache optimization
   - SIMD for parallel pixel operations

### **Real-World Use Cases**
- Game development (tile maps, collision detection)
- Scientific computing (matrix operations, simulations)
- Computer vision (image filtering, convolution)
- Geographic information systems (spatial queries)

---

## 🧪 Verification Methods

### **Sanity Checks**
```rust
// Verify monotonic scaling
assert!(time_1000x1000 > time_500x500);
assert!(time_500x500 > time_100x100);

// Verify linear scaling (within tolerance)
let ratio = time_1000x1000.as_secs_f64() / time_100x100.as_secs_f64();
assert!(ratio > 90.0 && ratio < 110.0);  // ~100x for 100x size increase

// Verify computation actually occurred
assert!(sum != 0 || all_elements_are_zero());
```

### **Debug vs Release Mode**
- **Debug mode**: Unoptimized, smaller performance gaps (5-10%)
- **Release mode**: Full optimizations, dramatic cache effects (3-7x)
- Always benchmark in `--release` for production-realistic results

---

## 📝 Implementation Example

Complete example from Mission6 tutorial:

```rust
use mission6_tut::grid::{TutorialGrid, Coord};
use std::time::Instant;
use std::hint::black_box;

fn benchmark_grid_creation(size: usize) -> Duration {
    let mut times = vec![];
    
    for _ in 0..10 {
        let start = Instant::now();
        // Use non-zero value to prevent OS optimization
        let grid = black_box(TutorialGrid::new(size, size, 1i32));
        times.push(start.elapsed());
    }
    
    times.sort();
    times[times.len() / 2]  // Median
}

fn benchmark_cache_patterns(grid: &TutorialGrid<i32>) -> (Duration, Duration, Duration) {
    // Row-major (cache-friendly)
    let row_major_time = median_of(5, || {
        let mut sum = 0i64;
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                sum += *grid.get(Coord { x, y }).unwrap() as i64;
            }
        }
        black_box(sum);
    });
    
    // Column-major (strided)
    let col_major_time = median_of(5, || {
        let mut sum = 0i64;
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                sum += *grid.get(Coord { x, y }).unwrap() as i64;
            }
        }
        black_box(sum);
    });
    
    // Random access (cache-hostile)
    let random_time = median_of(5, || {
        let mut sum = 0i64;
        let mut rng = simple_rng();
        for _ in 0..grid.width() * grid.height() {
            let x = rng.next() % grid.width();
            let y = rng.next() % grid.height();
            sum += *grid.get(Coord { x, y }).unwrap() as i64;
        }
        black_box(sum);
    });
    
    (row_major_time, col_major_time, random_time)
}
```

---

## 🎓 Key Takeaways

1. **Memory layout beats algorithm complexity** for grid operations
   - Flat Vec<T> with calculated indexing > Vec<Vec<T>> with pointer chasing
   - 1.5-2x performance improvement from layout alone

2. **Cache effects are dramatic in release mode**
   - Row-major vs random access: 7.67x difference
   - Debug mode masks these effects with other overhead

3. **Benchmarking requires discipline**
   - Use non-zero values, multiple iterations, median calculation
   - Apply `black_box()` to prevent compiler optimizations (see [[black-box-benchmarking]])
   - Always verify results and watch for anomalies

4. **Access patterns matter immensely**
   - Row-major traversal for sequential cache utilization
   - Column-major creates cache misses every stride
   - Random access causes complete cache thrashing

5. **Test on target hardware with realistic data**
   - Different CPUs have different cache characteristics
   - Production workloads may behave differently than toy examples

---

*Tags: #performance #benchmarking #grid-optimization #cache #mission6 #tutorial #concept #daily-study #implementation*

*Links: [[zettel-index]] | [[Daily Study MOC]] | [[Mission6 Tutorial MOC]] | [[Data Structures Overview]] | [[black-box-benchmarking]]*

---

**Created**: 2025-10-07  
**Source**: Mission6 Tutorial Step 6, Performance Optimization Workshop  
**Related Code**: `tutorials/Mission6_tut/examples/step6_performance.rs`  
**Reference**: [Algorithmica HPC - High Performance Computing Guide](https://en.algorithmica.org/hpc/)
