# Benchmarking - Measuring Real-World Performance

*Systematic measurement and analysis of code performance to validate optimizations and identify bottlenecks*

---

## 🎯 **Core Concept**

**Benchmarking** is the practice of measuring code execution time, memory usage, and other performance metrics under controlled conditions. It bridges the gap between theoretical complexity analysis ([[Big-O Analysis]]) and actual runtime behavior.

**Key Insight**: Big-O tells you how algorithms *scale*, but benchmarking tells you how fast they actually *run*.

```
O(n) algorithm with constant factor 1000 is slower than
O(n²) algorithm with constant factor 0.1 for n < 10,000
```

---

## 🧠 **Mental Models**

### **The Scientific Method**

Benchmarking is an experiment:

1. **Hypothesis**: "Optimization X will improve performance by Y%"
2. **Control**: Baseline measurement before changes
3. **Variable**: Apply single optimization
4. **Measurement**: Run benchmarks with statistical rigor
5. **Analysis**: Compare results, account for variance

### **The Optimization Pyramid**

Benchmark in order of impact:

```
         ╱╲
        ╱  ╲ Algorithm Choice (10x-1000x impact)
       ╱────╲
      ╱      ╲ Data Structure Selection (2x-100x)
     ╱────────╲
    ╱          ╲ Memory Layout/Cache (1.5x-10x)
   ╱────────────╲
  ╱              ╲ Micro-optimizations (1.1x-2x)
 ╱────────────────╲
```

---

## 🔍 **Benchmarking Tools in Rust**

### **1. Criterion.rs - Statistical Benchmarking**

The gold standard for Rust benchmarking:

```rust
// Cargo.toml
// [dev-dependencies]
// criterion = { version = "0.5", features = ["html_reports"] }
// 
// [[bench]]
// name = "my_benchmark"
// harness = false

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    b
}

fn fibonacci_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    
    for n in [10, 15, 20].iter() {
        group.bench_with_input(BenchmarkId::new("recursive", n), n, |b, &n| {
            b.iter(|| fibonacci_recursive(black_box(n)))
        });
        
        group.bench_with_input(BenchmarkId::new("iterative", n), n, |b, &n| {
            b.iter(|| fibonacci_iterative(black_box(n)))
        });
    }
    
    group.finish();
}

criterion_group!(benches, fibonacci_benchmark);
criterion_main!(benches);
```

**Key Criterion Features**:

- Statistical analysis (mean, median, std dev)
- Outlier detection and removal
- Comparison against baselines
- HTML reports with graphs
- Warm-up iterations to prime caches

### **2. Simple Timing with std::time**

For quick measurements:

```rust
use std::time::{Duration, Instant};

fn time_operation<F, R>(name: &str, iterations: u32, f: F) -> Duration
where
    F: Fn() -> R,
{
    // Warm-up
    for _ in 0..10 {
        let _ = f();
    }
    
    // Timed runs
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = std::hint::black_box(f());
    }
    let total = start.elapsed();
    
    let per_op = total / iterations;
    println!("{}: {:?} per operation ({} iterations)", name, per_op, iterations);
    
    per_op
}

fn main() {
    let data: Vec<i32> = (0..10_000).collect();
    
    time_operation("linear_search", 1000, || {
        data.iter().find(|&&x| x == 9999)
    });
    
    time_operation("binary_search", 1000, || {
        data.binary_search(&9999)
    });
}
```

### **3. Benchmarking Memory with dhat**

```rust
// Cargo.toml
// [dev-dependencies]
// dhat = "0.3"

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    
    // Your code here - will track all allocations
    let mut vec = Vec::new();
    for i in 0..1000 {
        vec.push(i);
    }
}
```

---

## 📊 **Benchmarking Best Practices**

### **1. Use black_box to Prevent Optimization**

```rust
use std::hint::black_box;

fn benchmark_bad() {
    let result = expensive_computation();
    // Compiler might optimize away if result unused!
}

fn benchmark_good() {
    let result = black_box(expensive_computation());
    // black_box tells compiler: "don't optimize this away"
}
```

### **2. Warm Up Before Measuring**

```rust
fn benchmark_with_warmup<F, R>(f: F, warmup: u32, measured: u32) -> Duration
where
    F: Fn() -> R,
{
    // Warm-up: prime CPU caches, trigger JIT (if any), stabilize clocks
    for _ in 0..warmup {
        black_box(f());
    }
    
    // Measured runs
    let start = Instant::now();
    for _ in 0..measured {
        black_box(f());
    }
    start.elapsed() / measured
}
```

### **3. Control for External Factors**

```rust
/// Benchmarking checklist:
/// - Close other applications
/// - Disable CPU frequency scaling if possible
/// - Run multiple times and check variance
/// - Test on representative data sizes
/// - Account for memory allocation patterns
fn production_benchmark() {
    // Set thread priority (platform-specific)
    // Pin to specific CPU core if available
    // Disable garbage collection (N/A for Rust, relevant for FFI)
    
    // Multiple runs with statistics
    let mut times = Vec::new();
    for _ in 0..100 {
        let start = Instant::now();
        // ... operation ...
        times.push(start.elapsed());
    }
    
    // Calculate statistics
    times.sort();
    let median = times[times.len() / 2];
    let mean = times.iter().sum::<Duration>() / times.len() as u32;
    let min = times[0];
    let max = times[times.len() - 1];
    
    println!("Median: {:?}, Mean: {:?}, Min: {:?}, Max: {:?}", 
             median, mean, min, max);
}
```

### **4. Test Multiple Input Sizes**

```rust
fn scalability_benchmark() {
    println!("Size\tTime\t\tTime/N");
    
    for size in [100, 1000, 10_000, 100_000, 1_000_000] {
        let data: Vec<i32> = (0..size as i32).collect();
        
        let start = Instant::now();
        let _sum: i32 = data.iter().sum();
        let duration = start.elapsed();
        
        let time_per_element = duration.as_nanos() as f64 / size as f64;
        
        println!("{}\t{:?}\t{:.2} ns/element", size, duration, time_per_element);
    }
}

// Output helps verify Big-O:
// If O(n): time_per_element should be roughly constant
// If O(n²): time_per_element should grow linearly with n
// If O(log n): time_per_element should decrease
```

---

## 💡 **Interpreting Benchmark Results**

### **Understanding Variance**

```rust
struct BenchmarkStats {
    mean: Duration,
    median: Duration,
    std_dev: Duration,
    min: Duration,
    max: Duration,
    samples: usize,
}

impl BenchmarkStats {
    fn from_samples(mut samples: Vec<Duration>) -> Self {
        samples.sort();
        let n = samples.len();
        
        let sum: Duration = samples.iter().sum();
        let mean = sum / n as u32;
        let median = samples[n / 2];
        
        let variance: f64 = samples
            .iter()
            .map(|d| {
                let diff = d.as_nanos() as f64 - mean.as_nanos() as f64;
                diff * diff
            })
            .sum::<f64>() / n as f64;
        
        let std_dev = Duration::from_nanos(variance.sqrt() as u64);
        
        Self {
            mean,
            median,
            std_dev,
            min: samples[0],
            max: samples[n - 1],
            samples: n,
        }
    }
    
    fn coefficient_of_variation(&self) -> f64 {
        self.std_dev.as_nanos() as f64 / self.mean.as_nanos() as f64 * 100.0
    }
    
    fn print_report(&self) {
        println!("Benchmark Results ({} samples):", self.samples);
        println!("  Mean:   {:?}", self.mean);
        println!("  Median: {:?}", self.median);
        println!("  Std Dev: {:?}", self.std_dev);
        println!("  CV:     {:.1}%", self.coefficient_of_variation());
        println!("  Range:  {:?} - {:?}", self.min, self.max);
        
        if self.coefficient_of_variation() > 10.0 {
            println!("  ⚠️  High variance! Results may be unreliable.");
        }
    }
}
```

### **Comparing Two Implementations**

```rust
fn compare_implementations<F1, F2, R>(
    name1: &str, f1: F1,
    name2: &str, f2: F2,
    iterations: u32,
)
where
    F1: Fn() -> R,
    F2: Fn() -> R,
{
    let t1 = time_operation(name1, iterations, f1);
    let t2 = time_operation(name2, iterations, f2);
    
    let speedup = t1.as_nanos() as f64 / t2.as_nanos() as f64;
    
    if speedup > 1.0 {
        println!("✅ {} is {:.2}x faster than {}", name2, speedup, name1);
    } else {
        println!("✅ {} is {:.2}x faster than {}", name1, 1.0/speedup, name2);
    }
}
```

---

## 🚀 **Common Benchmarking Patterns**

### **AoC Performance Validation**

```rust
/// Standard AoC benchmark pattern
fn benchmark_aoc_day(day: u32, input: &str) {
    println!("=== Day {} Benchmarks ===", day);
    
    // Parse benchmark
    let parse_time = time_operation("parse", 100, || {
        parse_input(input)
    });
    
    let parsed = parse_input(input);
    
    // Part 1 benchmark
    let part1_time = time_operation("part1", 100, || {
        solve_part1(&parsed)
    });
    
    // Part 2 benchmark  
    let part2_time = time_operation("part2", 100, || {
        solve_part2(&parsed)
    });
    
    let total = parse_time + part1_time + part2_time;
    println!("Total: {:?}", total);
    
    // Performance targets
    if total < Duration::from_millis(100) {
        println!("✅ Under 100ms - Excellent!");
    } else if total < Duration::from_secs(1) {
        println!("⚠️  Under 1s - Good");
    } else {
        println!("❌ Over 1s - Needs optimization");
    }
}
```

### **Mission Performance Validation**

```rust
/// Mission benchmark pattern (REQ-X performance requirements)
fn benchmark_mission_operations() {
    use mission5::HashMap;
    
    // REQ-2: O(1) average insert
    let mut map = HashMap::new();
    time_operation("insert_10k", 1, || {
        for i in 0..10_000 {
            map.insert(i, i);
        }
    });
    
    // REQ-3: O(1) average lookup
    time_operation("get_10k", 1, || {
        for i in 0..10_000 {
            black_box(map.get(&i));
        }
    });
    
    // Verify O(1) by testing scaling
    for size in [1_000, 10_000, 100_000] {
        let mut map = HashMap::new();
        for i in 0..size {
            map.insert(i, i);
        }
        
        time_operation(&format!("get_from_{}", size), 10_000, || {
            map.get(&(size / 2))
        });
    }
    // If O(1): times should be similar regardless of size
}
```

### **Data Structure Comparison**

```rust
use std::collections::{HashMap, BTreeMap, HashSet, BTreeSet};

fn compare_map_implementations() {
    let sizes = [100, 1_000, 10_000, 100_000];
    
    println!("=== Insert Performance ===");
    for &size in &sizes {
        println!("\nSize: {}", size);
        
        time_operation("  HashMap", 10, || {
            let mut map = HashMap::new();
            for i in 0..size {
                map.insert(i, i);
            }
            map
        });
        
        time_operation("  BTreeMap", 10, || {
            let mut map = BTreeMap::new();
            for i in 0..size {
                map.insert(i, i);
            }
            map
        });
    }
    
    println!("\n=== Lookup Performance ===");
    for &size in &sizes {
        let hash_map: HashMap<i32, i32> = (0..size).map(|i| (i, i)).collect();
        let btree_map: BTreeMap<i32, i32> = (0..size).map(|i| (i, i)).collect();
        
        println!("\nSize: {}", size);
        
        time_operation("  HashMap", 10_000, || {
            hash_map.get(&(size / 2))
        });
        
        time_operation("  BTreeMap", 10_000, || {
            btree_map.get(&(size / 2))
        });
    }
}
```

---

## ⚠️ **Benchmarking Pitfalls**

### **1. Dead Code Elimination**

```rust
// ❌ BAD: Compiler might optimize away
fn benchmark_bad() {
    let start = Instant::now();
    let result = expensive_computation();  // Might be optimized out!
    println!("Time: {:?}", start.elapsed());
}

// ✅ GOOD: Use black_box
fn benchmark_good() {
    let start = Instant::now();
    let result = black_box(expensive_computation());
    println!("Time: {:?}", start.elapsed());
}
```

### **2. Measuring the Wrong Thing**

```rust
// ❌ BAD: Measures allocation, not algorithm
fn benchmark_includes_allocation() {
    let start = Instant::now();
    let data = vec![0; 1_000_000];  // Allocation time included!
    let sum: i32 = data.iter().sum();
    println!("Time: {:?}", start.elapsed());
}

// ✅ GOOD: Separate setup from measurement
fn benchmark_algorithm_only() {
    let data = vec![0; 1_000_000];  // Setup outside timing
    
    let start = Instant::now();
    let sum: i32 = data.iter().sum();  // Only algorithm measured
    println!("Time: {:?}", start.elapsed());
}
```

### **3. Cache Effects**

```rust
// ❌ BAD: First run is cold cache, subsequent runs are warm
fn benchmark_ignores_cache() {
    for i in 0..5 {
        let start = Instant::now();
        // First iteration: cache cold (slower)
        // Later iterations: cache warm (faster)
        let _ = operation(&data);
        println!("Run {}: {:?}", i, start.elapsed());
    }
}

// ✅ GOOD: Explicit warm-up phase
fn benchmark_handles_cache() {
    // Warm-up: fill caches
    for _ in 0..5 {
        let _ = operation(&data);
    }
    
    // Measured runs: cache warm
    let start = Instant::now();
    for _ in 0..100 {
        let _ = operation(&data);
    }
    println!("Average: {:?}", start.elapsed() / 100);
}
```

### **4. Unrealistic Data**

```rust
// ❌ BAD: Sorted data might not reflect real usage
fn benchmark_sorted() {
    let data: Vec<i32> = (0..10_000).collect();  // Already sorted!
    // Quicksort on sorted data is O(n²) worst case
}

// ✅ GOOD: Test multiple data distributions
fn benchmark_realistic() {
    use rand::seq::SliceRandom;
    
    // Random data
    let mut random: Vec<i32> = (0..10_000).collect();
    random.shuffle(&mut rand::rng());
    
    // Nearly sorted
    let mut nearly_sorted: Vec<i32> = (0..10_000).collect();
    for i in (0..nearly_sorted.len()).step_by(100) {
        let j = (i + 50).min(nearly_sorted.len() - 1);
        nearly_sorted.swap(i, j);
    }
    
    // Reverse sorted (worst case for some algorithms)
    let reverse: Vec<i32> = (0..10_000).rev().collect();
    
    // Benchmark all distributions
}
```

---

## 🔗 **Integration Points**

### **Builds On**

- [[Big-O Analysis]] - Theoretical complexity to validate
- [[Algorithm Analysis]] - Empirical analysis techniques
- [[Performance Engineering]] - Systematic optimization

### **Enables**

- [[Cache Efficiency]] - Validating cache optimization
- [[Memory Optimization]] - Measuring allocation impact
- [[Performance Engineering]] - Data-driven optimization

### **Related Concepts**

- [[Amortized Analysis]] - Validating amortized claims
- [[Profiling]] - Finding bottlenecks (complementary to benchmarking)
- [[Property-Based Testing]] - Generating benchmark inputs

### **Workspace Applications**

- **Missions**: Validate REQ performance requirements
- **AoC**: Measure solution efficiency
- **Tutorials**: Compare implementation approaches

---

## 📚 **Tools Reference**

| Tool | Purpose | Use When |
|------|---------|----------|
| **Criterion** | Statistical benchmarking | Production performance testing |
| **std::time** | Quick measurements | Development iteration |
| **dhat** | Heap profiling | Memory usage analysis |
| **perf** (Linux) | CPU profiling | Finding hot spots |
| **cargo flamegraph** | Visualization | Understanding call patterns |

---

## 📖 **Further Reading**

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [[performance-benchmarking-grid-optimization]] - Workspace benchmarking guide
- [[Algorithm Analysis]] - Comprehensive analysis techniques

---

*Tags: #benchmarking #performance #profiling #optimization #criterion #testing #measurement #aoc #missions*

*Links: [[Algorithm Analysis]] | [[Big-O Analysis]] | [[Performance Engineering]] | [[Amortized Analysis]] | [[Cache Efficiency]] | [[Memory Optimization]] | [[Algorithms MOC]] | [[AoC Patterns MOC]] | [[zettel-index]]*
