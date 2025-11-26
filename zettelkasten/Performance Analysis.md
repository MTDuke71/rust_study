# ⚡ Performance Analysis

**Systematic techniques for measuring, profiling, and analyzing code performance in Rust**

---

## 📖 Core Concept

**Performance analysis** is the process of measuring and understanding how your code behaves at runtime. It answers questions like:

- *"Where is my code spending most of its time?"*
- *"Which functions are performance bottlenecks?"*
- *"How does performance scale with input size?"*
- *"Is my optimization actually helping?"*

**Key Distinction:**

- **Performance Analysis** (this page) = **Measuring and diagnosing** performance
- **[[Performance Optimization]]** = **Improving** performance after analysis
- **[[Big-O Analysis]]** = **Theoretical** complexity analysis

**The Golden Rule:** Always measure before and after optimization. Without measurement, you're guessing.

---

## 🎯 **Types of Performance Analysis**

### **1. Benchmarking**

*Measuring execution time under controlled conditions*

### **2. Profiling**

*Identifying where time/memory is spent during execution*

### **3. Complexity Analysis**

*Understanding scaling behavior as input grows*

### **4. Resource Analysis**

*Measuring memory, CPU, I/O, and other resource usage*

---

## 🔬 **Benchmarking with Criterion**

### **Basic Benchmark Setup**

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci_recursive(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    a
}

fn benchmark_fibonacci(c: &mut Criterion) {
    c.bench_function("fib_recursive_20", |b| {
        b.iter(|| fibonacci_recursive(black_box(20)))
    });
    
    c.bench_function("fib_iterative_20", |b| {
        b.iter(|| fibonacci_iterative(black_box(20)))
    });
}

criterion_group!(benches, benchmark_fibonacci);
criterion_main!(benches);
```

**Run with:** `cargo bench`

**Output:**

```
fib_recursive_20    time:   [2.7831 ms 2.7945 ms 2.8067 ms]
fib_iterative_20    time:   [32.847 ns 32.987 ns 33.141 ns]
                    ^^^
                    84,000x faster!
```

### **Mission5 HashMap Benchmark Example**

```rust
// Mission5: Benchmarking hash map operations
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;

fn benchmark_hashmap_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap_insert");
    
    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut map = HashMap::with_capacity(size);
                for i in 0..size {
                    map.insert(black_box(i), black_box(i * 2));
                }
                map
            });
        });
    }
    group.finish();
}

fn benchmark_hashmap_lookup(c: &mut Criterion) {
    let sizes = [100, 1_000, 10_000, 100_000];
    
    for &size in &sizes {
        let mut map = HashMap::new();
        for i in 0..size {
            map.insert(i, i * 2);
        }
        
        c.bench_function(&format!("lookup_{}", size), |b| {
            b.iter(|| {
                // Measure random lookups
                for i in (0..100).map(|_| rand::random::<usize>() % size) {
                    black_box(map.get(&i));
                }
            });
        });
    }
}

criterion_group!(benches, benchmark_hashmap_insert, benchmark_hashmap_lookup);
criterion_main!(benches);
```

### **Using `black_box` to Prevent Optimization**

```rust
// ❌ Without black_box: Compiler may optimize away
fn bad_benchmark(c: &mut Criterion) {
    c.bench_function("compute", |b| {
        b.iter(|| {
            expensive_computation(42)  // Compiler might evaluate once!
        })
    });
}

// ✅ With black_box: Forces computation each iteration
fn good_benchmark(c: &mut Criterion) {
    c.bench_function("compute", |b| {
        b.iter(|| {
            black_box(expensive_computation(black_box(42)))
        })
    });
}
```

---

## 📊 **Profiling Tools**

### **1. `cargo flamegraph` - Visual Profiling**

```bash
# Install
cargo install flamegraph

# Run profiler
cargo flamegraph --bin my_program

# Opens flamegraph.svg showing where time is spent
```

**Mission9 Example:**

```bash
# Profile Dijkstra's algorithm
cd missions/Mission9
cargo flamegraph --bin pathfinding -- --algorithm dijkstra

# Result: Visual flame graph showing:
# - 60% time in priority queue operations
# - 25% time in edge iteration
# - 10% time in distance updates
# - 5% time in path reconstruction
```

### **2. `perf` - Linux Performance Counter**

```bash
# Record performance data
cargo build --release
perf record --call-graph=dwarf ./target/release/my_program

# Analyze results
perf report

# Hot functions:
#   45.23%  dijkstra_inner
#   23.45%  binary_heap_push
#   12.34%  hash_map_insert
```

### **3. `cargo-llvm-lines` - Code Size Analysis**

```bash
# Install
cargo install cargo-llvm-lines

# Analyze which functions generate most LLVM IR
cargo llvm-lines | head -20

# Output shows functions that may benefit from optimization
```

### **4. Valgrind/Cachegrind - Memory and Cache Analysis**

```bash
# Memory profiling
valgrind --tool=massif ./target/release/my_program

# Cache behavior analysis
valgrind --tool=cachegrind ./target/release/my_program

# Results show:
# - Cache hit/miss rates
# - Memory allocation patterns
# - Heap usage over time
```

---

## 📈 **Complexity Verification**

### **Empirical Complexity Analysis**

```rust
use std::time::Instant;

fn verify_complexity() {
    println!("Size\tTime (ms)\tRatio");
    println!("----\t---------\t-----");
    
    let mut prev_time = 0.0;
    
    for size in [100, 1000, 10_000, 100_000, 1_000_000] {
        let data: Vec<i32> = (0..size).collect();
        
        let start = Instant::now();
        binary_search(&data, &(size / 2));
        let duration = start.elapsed();
        
        let ms = duration.as_secs_f64() * 1000.0;
        let ratio = if prev_time > 0.0 { ms / prev_time } else { 0.0 };
        
        println!("{}\t{:.6}\t{:.2}x", size, ms, ratio);
        prev_time = ms;
    }
}

// Expected output for O(log n):
// Size     Time (ms)    Ratio
// ----     ---------    -----
// 100      0.000050     0.00x
// 1000     0.000065     1.30x  ← ~constant increase
// 10000    0.000080     1.23x
// 100000   0.000095     1.19x
// 1000000  0.000110     1.16x
```

### **Mission3 Binary Search Verification**

```rust
// Verify O(log n) complexity
fn verify_binary_search_complexity() {
    use std::time::Instant;
    
    println!("Verifying binary search is O(log n):");
    println!("n\t\tTime (μs)\tlog₂(n)\t\tTime/log₂(n)");
    
    for exp in 10..=24 {
        let n = 2_usize.pow(exp);
        let arr: Vec<i32> = (0..n as i32).collect();
        
        let iterations = 1000;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = arr.binary_search(&(n as i32 / 2));
        }
        let duration = start.elapsed();
        
        let time_us = duration.as_micros() as f64 / iterations as f64;
        let log_n = (n as f64).log2();
        let ratio = time_us / log_n;
        
        println!("{}\t\t{:.3}\t\t{:.1}\t\t{:.4}", n, time_us, log_n, ratio);
    }
}

// Expected: Time/log₂(n) ratio stays roughly constant
// Proves O(log n) complexity empirically
```

---

## 🧪 **Statistical Analysis**

### **Understanding Benchmark Statistics**

```rust
// Criterion output:
// time:   [2.7831 ms 2.7945 ms 2.8067 ms]
//          └─────┬─────┘ └────┬────┘ └────┬────┘
//          Lower bound  Mean    Upper bound
//          (95% confidence interval)

// change: [-2.1% +0.5% +3.2%] (p = 0.12 > 0.05)
//          └──────────┬──────────┘
//          Change range (not statistically significant)
```

### **Detecting Performance Regressions**

```rust
// Save baseline
cargo bench --save-baseline master

// Make changes...

// Compare against baseline
cargo bench --baseline master

// Output shows if performance changed significantly
// time:   [2.7945 ms 2.8012 ms 2.8089 ms]
// change: [-0.4% +0.2% +0.8%] (p = 0.15 > 0.05)
//         ^^^^^^^^^ No significant change
```

---

## 💾 **Memory Analysis**

### **Heap Profiling with `cargo-instruments`** (macOS)

```bash
# Install
cargo install cargo-instruments

# Profile allocations
cargo instruments --template Allocations --bin my_program

# Shows:
# - Total allocations
# - Peak memory usage
# - Allocation backtraces
```

### **Custom Memory Tracking**

```rust
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

fn main() {
    println!("Initial allocation: {} bytes", ALLOCATED.load(Ordering::SeqCst));
    
    let data: Vec<i32> = vec![1; 1_000_000];
    println!("After Vec allocation: {} bytes", ALLOCATED.load(Ordering::SeqCst));
    // Output: ~4,000,000 bytes (4 bytes per i32)
    
    drop(data);
    println!("After Vec drop: {} bytes", ALLOCATED.load(Ordering::SeqCst));
}
```

### **Mission5 HashMap Memory Analysis**

```rust
fn analyze_hashmap_memory() {
    use std::mem::size_of;
    
    let mut map = HashMap::new();
    
    println!("Empty HashMap size: {} bytes", size_of::<HashMap<i32, i32>>());
    
    // Measure actual heap usage
    let start_mem = ALLOCATED.load(Ordering::SeqCst);
    
    for i in 0..10_000 {
        map.insert(i, i * 2);
    }
    
    let end_mem = ALLOCATED.load(Ordering::SeqCst);
    let used = end_mem - start_mem;
    
    println!("10,000 entries:");
    println!("  Total heap: {} bytes", used);
    println!("  Per entry: {:.1} bytes", used as f64 / 10_000.0);
    println!("  Capacity: {}", map.capacity());
    println!("  Load factor: {:.2}", map.len() as f64 / map.capacity() as f64);
}
```

---

## 🎓 **Mission-Specific Analysis Techniques**

### **Mission1: Stack Operations**

```rust
fn benchmark_stack_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack");
    
    // Verify O(1) push
    for size in [100, 1000, 10_000] {
        group.bench_function(&format!("push_{}", size), |b| {
            b.iter(|| {
                let mut stack = Vec::with_capacity(size);
                for i in 0..size {
                    stack.push(black_box(i));
                }
            });
        });
    }
    
    // All sizes should take roughly same time per operation
    // Confirms amortized O(1)
}
```

### **Mission3: Binary Search**

```rust
fn analyze_binary_search_vs_linear() {
    println!("Size\tBinary (μs)\tLinear (μs)\tSpeedup");
    
    for size in [100, 1000, 10_000, 100_000] {
        let data: Vec<i32> = (0..size).collect();
        let target = size / 2;
        
        // Binary search timing
        let start = Instant::now();
        for _ in 0..1000 {
            let _ = data.binary_search(&target);
        }
        let binary_time = start.elapsed().as_micros() as f64 / 1000.0;
        
        // Linear search timing
        let start = Instant::now();
        for _ in 0..1000 {
            let _ = data.iter().position(|&x| x == target);
        }
        let linear_time = start.elapsed().as_micros() as f64 / 1000.0;
        
        let speedup = linear_time / binary_time;
        println!("{}\t{:.3}\t\t{:.3}\t\t{:.1}x", size, binary_time, linear_time, speedup);
    }
    
    // Expected: Speedup grows with size (log n vs n)
}
```

### **Mission9: Graph Algorithm Comparison**

```rust
fn compare_pathfinding_algorithms(c: &mut Criterion) {
    let graph = generate_test_graph(1000, 5000);  // 1000 nodes, 5000 edges
    let start = NodeId(0);
    let goal = NodeId(999);
    
    let mut group = c.benchmark_group("pathfinding");
    
    group.bench_function("dijkstra", |b| {
        b.iter(|| {
            dijkstra(black_box(&graph), black_box(start), black_box(goal))
        })
    });
    
    group.bench_function("astar_euclidean", |b| {
        b.iter(|| {
            astar(
                black_box(&graph), 
                black_box(start), 
                black_box(goal),
                euclidean_heuristic
            )
        })
    });
    
    group.bench_function("astar_manhattan", |b| {
        b.iter(|| {
            astar(
                black_box(&graph), 
                black_box(start), 
                black_box(goal),
                manhattan_heuristic
            )
        })
    });
    
    group.finish();
    
    // Expected results:
    // - Dijkstra: baseline performance
    // - A* with good heuristic: 2-10x faster
    // - A* with poor heuristic: similar to Dijkstra
}
```

---

## 📊 **Visualization and Reporting**

### **Criterion HTML Reports**

```bash
# Generate detailed HTML report
cargo bench

# Opens target/criterion/report/index.html
# Shows:
# - Violin plots of timing distributions
# - Time series showing changes over runs
# - Statistical analysis
# - Regression detection
```

### **Custom Performance Reports**

```rust
fn generate_performance_report() {
    use std::fs::File;
    use std::io::Write;
    
    let mut report = File::create("performance_report.md").unwrap();
    
    writeln!(report, "# Performance Analysis Report\n").unwrap();
    writeln!(report, "## Test Configuration").unwrap();
    writeln!(report, "- Date: {}", chrono::Local::now()).unwrap();
    writeln!(report, "- Rust version: {}", rustc_version()).unwrap();
    writeln!(report, "\n## Results\n").unwrap();
    
    // Run benchmarks and collect data
    let results = run_all_benchmarks();
    
    for (name, time_ns) in results {
        writeln!(
            report, 
            "- **{}**: {:.2} ms", 
            name, 
            time_ns as f64 / 1_000_000.0
        ).unwrap();
    }
}
```

---

## 🔍 **Advanced Analysis Techniques**

### **Cache Performance Analysis**

```rust
fn analyze_cache_behavior() {
    use perf_event::Builder;
    
    let size = 10_000_000;
    let data: Vec<i32> = (0..size).collect();
    
    // Measure cache misses
    let mut counter = Builder::new()
        .kind(perf_event::events::Hardware::CACHE_MISSES)
        .build()
        .unwrap();
    
    counter.enable().unwrap();
    
    // Sequential access (cache-friendly)
    let mut sum = 0;
    for &x in &data {
        sum += x;
    }
    
    let sequential_misses = counter.read().unwrap();
    counter.reset().unwrap();
    
    // Random access (cache-unfriendly)
    sum = 0;
    for _ in 0..size {
        let idx = rand::random::<usize>() % size;
        sum += data[idx];
    }
    
    let random_misses = counter.read().unwrap();
    
    println!("Cache Analysis:");
    println!("  Sequential access: {} misses", sequential_misses);
    println!("  Random access: {} misses", random_misses);
    println!("  Ratio: {:.2}x", random_misses as f64 / sequential_misses as f64);
}
```

### **Branch Prediction Analysis**

```rust
fn analyze_branch_prediction() {
    let size = 10_000_000;
    let data: Vec<i32> = (0..size).map(|_| rand::random()).collect();
    
    // Predictable branches (sorted data)
    let mut sorted = data.clone();
    sorted.sort();
    
    let start = Instant::now();
    let count = sorted.iter().filter(|&&x| x > 0).count();
    let sorted_time = start.elapsed();
    
    // Unpredictable branches (random data)
    let start = Instant::now();
    let count2 = data.iter().filter(|&&x| x > 0).count();
    let random_time = start.elapsed();
    
    println!("Branch Prediction Impact:");
    println!("  Sorted data: {:?}", sorted_time);
    println!("  Random data: {:?}", random_time);
    println!("  Slowdown: {:.2}x", random_time.as_secs_f64() / sorted_time.as_secs_f64());
    // Typical slowdown: 2-4x due to branch mispredictions
}
```

---

## 📋 **Performance Analysis Checklist**

### **Before Optimization**

- [ ] Establish baseline measurements
- [ ] Identify bottlenecks with profiling
- [ ] Verify Big-O complexity matches expectations
- [ ] Measure memory usage patterns
- [ ] Check cache behavior
- [ ] Document current performance

### **During Optimization**

- [ ] Benchmark each change in isolation
- [ ] Compare against baseline
- [ ] Check for statistical significance (p < 0.05)
- [ ] Verify correctness hasn't regressed
- [ ] Monitor memory usage changes

### **After Optimization**

- [ ] Generate performance report
- [ ] Update documentation with new benchmarks
- [ ] Add regression tests
- [ ] Compare with theoretical complexity
- [ ] Validate on different input sizes

---

## 🎯 **Common Analysis Patterns**

### **Pattern 1: Comparing Implementations**

```rust
fn compare_implementations(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison");
    let data: Vec<i32> = (0..1000).collect();
    
    group.bench_function("version_1", |b| b.iter(|| v1(&data)));
    group.bench_function("version_2", |b| b.iter(|| v2(&data)));
    group.bench_function("version_3", |b| b.iter(|| v3(&data)));
    
    group.finish();
}
```

### **Pattern 2: Scaling Analysis**

```rust
fn scaling_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling");
    
    for size in [10, 100, 1_000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, 
            |b, &size| {
                let data = generate_data(size);
                b.iter(|| process(black_box(&data)))
            }
        );
    }
    
    group.finish();
}
```

### **Pattern 3: Memory Profiling**

```rust
fn memory_profile() {
    let before = ALLOCATED.load(Ordering::SeqCst);
    
    {
        let structure = build_structure();
        let during = ALLOCATED.load(Ordering::SeqCst);
        println!("Structure memory: {} bytes", during - before);
        
        // Structure goes out of scope
    }
    
    let after = ALLOCATED.load(Ordering::SeqCst);
    println!("Leaked memory: {} bytes", after - before);
}
```

---

## 🔗 **Related Concepts**

- **[[Performance Optimization]]** - Applying improvements after analysis
- **[[Performance Optimization Guide]]** - Comprehensive optimization strategies
- **[[Big-O Analysis]]** - Theoretical complexity analysis
- **[[Algorithm Analysis]]** - Broader algorithm analysis techniques
- **[[Computational Complexity Classes]]** - Problem hardness analysis
- **[[mission-5]]** - HashMap performance case study
- **[[mission-9]]** - Graph algorithm performance comparison
- **[[Rust Collections MOC]]** - Collection performance characteristics
- **[[AoC Patterns MOC]]** - Performance patterns in competitive programming

---

## 📚 **Tools Reference**

### **Benchmarking**

- `criterion` - Statistical benchmarking
- `cargo bench` - Run benchmarks
- `iai` - Cachegrind-based benchmarking

### **Profiling**

- `cargo flamegraph` - Flame graph generation
- `perf` - Linux performance analysis
- `valgrind` - Memory profiling
- `cargo-instruments` - macOS profiling

### **Analysis**

- `cargo-llvm-lines` - Code size analysis
- `cargo-bloat` - Binary size analysis
- `cargo-asm` - Inspect generated assembly

### **Mission Examples**

- Mission1: Stack operation benchmarks
- Mission3: Binary search complexity verification
- Mission5: HashMap load factor analysis
- Mission9: Pathfinding algorithm comparison

---

*Tags: #performance-analysis #benchmarking #profiling #criterion #flamegraph #measurement #optimization #performance-testing #empirical-analysis #complexity-verification*

*Links: [[Performance Optimization]] | [[Performance Optimization Guide]] | [[Big-O Analysis]] | [[Algorithm Analysis]] | [[mission-5]] | [[mission-9]] | [[Rust Collections MOC]] | [[zettel-index]]*
