# Performance Analysis Patterns

*Systematic approaches to measuring, profiling, and analyzing performance characteristics of data structures and algorithms*

---

## 🎯 **Performance Analysis Framework**

### **Analysis Dimensions**

```rust
pub struct PerformanceMetrics {
    // Time complexity
    pub time_complexity: ComplexityClass,
    pub wall_clock_time: Duration,
    pub cpu_time: Duration,
    
    // Space complexity  
    pub space_complexity: ComplexityClass,
    pub memory_usage: usize,
    pub peak_memory: usize,
    
    // Throughput metrics
    pub operations_per_second: f64,
    pub bytes_per_second: f64,
    
    // Cache performance
    pub cache_hit_rate: f64,
    pub cache_misses: u64,
    
    // System resources
    pub cpu_utilization: f64,
    pub memory_fragmentation: f64,
}

#[derive(Debug)]
pub enum ComplexityClass {
    Constant,      // O(1)
    Logarithmic,   // O(log n)
    Linear,        // O(n)  
    Linearithmic,  // O(n log n)
    Quadratic,     // O(n²)
    Exponential,   // O(2ⁿ)
}
```

## 📊 **Benchmarking Patterns**

### **1. Micro-Benchmarking with Criterion**

#### **Basic Operation Benchmarking**

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn benchmark_queue_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue_operations");
    
    // Test different queue sizes
    for size in [100, 1000, 10000, 100000].iter() {
        // Enqueue performance
        group.bench_with_input(
            BenchmarkId::new("enqueue", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || BoundedQueue::with_capacity(size),
                    |mut queue| {
                        for i in 0..size {
                            black_box(queue.enqueue(i));
                        }
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
        
        // Dequeue performance
        group.bench_with_input(
            BenchmarkId::new("dequeue", size),
            size,
            |b, &size| {
                b.iter_batched(
                    || {
                        let mut queue = BoundedQueue::with_capacity(size);
                        for i in 0..size { queue.enqueue(i).unwrap(); }
                        queue
                    },
                    |mut queue| {
                        for _ in 0..size {
                            black_box(queue.dequeue());
                        }
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    
    group.finish();
}
```

#### **Comparative Analysis Pattern**

```rust
fn benchmark_bounded_vs_unbounded(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue_comparison");
    
    let sizes = vec![1000, 10000, 100000];
    
    for size in sizes {
        // Bounded queue benchmark
        group.bench_with_input(
            BenchmarkId::new("bounded", size),
            &size,
            |b, &size| {
                b.iter_batched(
                    || BoundedQueue::with_capacity(size * 2), // Avoid full condition
                    |mut queue| {
                        for i in 0..size {
                            black_box(queue.enqueue(i));
                        }
                        for _ in 0..size {
                            black_box(queue.dequeue());
                        }
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
        
        // Unbounded queue benchmark
        group.bench_with_input(
            BenchmarkId::new("unbounded", size),
            &size,
            |b, &size| {
                b.iter_batched(
                    || std::collections::VecDeque::new(),
                    |mut queue| {
                        for i in 0..size {
                            black_box(queue.push_back(i));
                        }
                        for _ in 0..size {
                            black_box(queue.pop_front());
                        }
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }
    
    group.finish();
}
```

### **2. Memory Usage Analysis**

#### **Memory Profiling Pattern**

```rust
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

// Custom allocator to track memory usage
struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATED: AtomicUsize = AtomicUsize::new(0);

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
        DEALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

fn analyze_memory_usage<T, F>(operation: F) -> (T, usize, usize)
where
    F: FnOnce() -> T,
{
    // Reset counters
    ALLOCATED.store(0, Ordering::SeqCst);
    DEALLOCATED.store(0, Ordering::SeqCst);
    
    let result = operation();
    
    let allocated = ALLOCATED.load(Ordering::SeqCst);
    let deallocated = DEALLOCATED.load(Ordering::SeqCst);
    
    (result, allocated, deallocated)
}

// Usage example
fn test_queue_memory_usage() {
    let (queue, allocated, deallocated) = analyze_memory_usage(|| {
        let mut queue = BoundedQueue::with_capacity(1000);
        for i in 0..1000 {
            queue.enqueue(i).unwrap();
        }
        queue
    });
    
    println!("Allocated: {} bytes", allocated);
    println!("Deallocated: {} bytes", deallocated);
    println!("Net usage: {} bytes", allocated - deallocated);
}
```

### **3. Complexity Analysis Through Scaling**

#### **Empirical Complexity Detection**

```rust
use std::time::Instant;

fn analyze_complexity_scaling() {
    let sizes = vec![100, 200, 400, 800, 1600, 3200];
    let mut results = Vec::new();
    
    for &size in &sizes {
        let start = Instant::now();
        
        // Test operation scaling
        let mut queue = BoundedQueue::with_capacity(size);
        for i in 0..size {
            queue.enqueue(i).unwrap();
        }
        for _ in 0..size {
            queue.dequeue();
        }
        
        let duration = start.elapsed();
        results.push((size, duration));
        
        println!("Size: {}, Time: {:?}", size, duration);
    }
    
    // Analyze scaling relationship
    analyze_scaling_relationship(&results);
}

fn analyze_scaling_relationship(results: &[(usize, std::time::Duration)]) {
    println!("\nScaling Analysis:");
    
    for window in results.windows(2) {
        let (size1, time1) = window[0];
        let (size2, time2) = window[1];
        
        let size_ratio = size2 as f64 / size1 as f64;
        let time_ratio = time2.as_nanos() as f64 / time1.as_nanos() as f64;
        
        println!("Size {}→{} ({}x): Time ratio {:.2}x", 
                size1, size2, size_ratio, time_ratio);
        
        // Classify complexity
        let complexity = match time_ratio / size_ratio {
            r if r < 1.2 => "O(1) - Constant",
            r if r < 1.5 => "O(log n) - Logarithmic", 
            r if r < 2.2 => "O(n) - Linear",
            r if r < 4.5 => "O(n log n) - Linearithmic",
            _ => "O(n²) or worse - Polynomial/Exponential",
        };
        
        println!("  → Estimated complexity: {}", complexity);
    }
}
```

## 🔬 **Cache Performance Analysis**

### **Cache Miss Measurement**

```rust
use std::time::Instant;

struct CacheAnalyzer {
    cache_line_size: usize,
    l1_cache_size: usize,
    l2_cache_size: usize,
    l3_cache_size: usize,
}

impl CacheAnalyzer {
    fn new() -> Self {
        Self {
            cache_line_size: 64,
            l1_cache_size: 32 * 1024,      // 32KB
            l2_cache_size: 256 * 1024,     // 256KB  
            l3_cache_size: 8 * 1024 * 1024, // 8MB
        }
    }
    
    fn analyze_access_pattern<T>(&self, data: &[T], access_pattern: &[usize]) {
        let element_size = std::mem::size_of::<T>();
        
        // Predict cache behavior
        let working_set_size = access_pattern.len() * element_size;
        let cache_level = self.predict_cache_level(working_set_size);
        
        println!("Working set size: {} bytes", working_set_size);
        println!("Predicted cache level: {}", cache_level);
        
        // Measure actual performance
        let sequential_time = self.measure_sequential_access(data);
        let random_time = self.measure_random_access(data, access_pattern);
        
        let cache_penalty = random_time.as_nanos() as f64 / sequential_time.as_nanos() as f64;
        println!("Cache penalty: {:.2}x", cache_penalty);
        
        // Analyze results
        self.interpret_cache_penalty(cache_penalty);
    }
    
    fn predict_cache_level(&self, working_set_size: usize) -> &'static str {
        if working_set_size <= self.l1_cache_size {
            "L1 Cache"
        } else if working_set_size <= self.l2_cache_size {
            "L2 Cache"
        } else if working_set_size <= self.l3_cache_size {
            "L3 Cache"
        } else {
            "Main Memory"
        }
    }
    
    fn measure_sequential_access<T>(&self, data: &[T]) -> std::time::Duration {
        let start = Instant::now();
        let mut sum = 0usize;
        
        for (i, _) in data.iter().enumerate() {
            sum += i; // Prevent optimization
        }
        
        std::hint::black_box(sum);
        start.elapsed()
    }
    
    fn measure_random_access<T>(&self, data: &[T], indices: &[usize]) -> std::time::Duration {
        let start = Instant::now();
        let mut sum = 0usize;
        
        for &idx in indices {
            if idx < data.len() {
                sum += idx;
            }
        }
        
        std::hint::black_box(sum);
        start.elapsed()
    }
    
    fn interpret_cache_penalty(&self, penalty: f64) {
        let interpretation = match penalty {
            p if p < 1.5 => "Excellent cache utilization",
            p if p < 2.0 => "Good cache utilization", 
            p if p < 3.0 => "Moderate cache misses",
            p if p < 5.0 => "Poor cache utilization",
            _ => "Very poor cache utilization - mostly main memory access",
        };
        
        println!("Cache analysis: {}", interpretation);
    }
}
```

## 📈 **Throughput Analysis Patterns**

### **Operations Per Second Measurement**

```rust
use std::time::{Duration, Instant};

struct ThroughputAnalyzer;

impl ThroughputAnalyzer {
    fn measure_ops_per_second<F, T>(&self, operation: F, duration: Duration) -> (u64, f64)
    where
        F: Fn() -> T,
        T: Clone,
    {
        let start = Instant::now();
        let mut operations = 0u64;
        
        while start.elapsed() < duration {
            std::hint::black_box(operation());
            operations += 1;
        }
        
        let actual_duration = start.elapsed();
        let ops_per_second = operations as f64 / actual_duration.as_secs_f64();
        
        (operations, ops_per_second)
    }
    
    fn analyze_queue_throughput(&self) {
        println!("Queue Throughput Analysis");
        println!("========================");
        
        // Bounded queue analysis
        let (bounded_ops, bounded_ops_per_sec) = self.measure_ops_per_second(|| {
            let mut queue = BoundedQueue::with_capacity(1000);
            for i in 0..500 {
                queue.enqueue(i).ok();
            }
            for _ in 0..500 {
                queue.dequeue();
            }
        }, Duration::from_secs(1));
        
        println!("Bounded Queue: {} ops/sec ({} total ops)", 
                bounded_ops_per_sec as u64, bounded_ops);
        
        // Unbounded queue analysis  
        let (unbounded_ops, unbounded_ops_per_sec) = self.measure_ops_per_second(|| {
            let mut queue = std::collections::VecDeque::new();
            for i in 0..500 {
                queue.push_back(i);
            }
            for _ in 0..500 {
                queue.pop_front();
            }
        }, Duration::from_secs(1));
        
        println!("Unbounded Queue: {} ops/sec ({} total ops)", 
                unbounded_ops_per_sec as u64, unbounded_ops);
        
        // Performance comparison
        let ratio = bounded_ops_per_sec / unbounded_ops_per_sec;
        println!("Performance ratio: {:.2}x", ratio);
    }
}
```

### **Latency Distribution Analysis**

```rust
use std::collections::BTreeMap;

struct LatencyAnalyzer {
    measurements: Vec<Duration>,
}

impl LatencyAnalyzer {
    fn new() -> Self {
        Self {
            measurements: Vec::new(),
        }
    }
    
    fn measure_operation<F, T>(&mut self, operation: F) -> T
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let result = operation();
        let latency = start.elapsed();
        
        self.measurements.push(latency);
        result
    }
    
    fn analyze_distribution(&self) -> LatencyStats {
        let mut sorted = self.measurements.clone();
        sorted.sort();
        
        let len = sorted.len();
        let mean = sorted.iter().sum::<Duration>() / len as u32;
        
        let median = sorted[len / 2];
        let p95 = sorted[len * 95 / 100];
        let p99 = sorted[len * 99 / 100];
        let min = sorted[0];
        let max = sorted[len - 1];
        
        LatencyStats {
            count: len,
            mean,
            median,
            p95,
            p99,
            min,
            max,
        }
    }
    
    fn print_analysis(&self) {
        let stats = self.analyze_distribution();
        
        println!("Latency Analysis ({} samples)", stats.count);
        println!("================================");
        println!("Mean:     {:?}", stats.mean);
        println!("Median:   {:?}", stats.median);
        println!("95th %:   {:?}", stats.p95);
        println!("99th %:   {:?}", stats.p99);
        println!("Min:      {:?}", stats.min);
        println!("Max:      {:?}", stats.max);
        
        // Detect outliers
        let outlier_threshold = stats.median.as_nanos() * 3;
        let outliers = self.measurements.iter()
            .filter(|&&d| d.as_nanos() > outlier_threshold)
            .count();
            
        if outliers > 0 {
            println!("Outliers: {} ({:.1}%)", outliers, 
                    outliers as f64 / stats.count as f64 * 100.0);
        }
    }
}

#[derive(Debug)]
struct LatencyStats {
    count: usize,
    mean: Duration,
    median: Duration,
    p95: Duration,
    p99: Duration,
    min: Duration,
    max: Duration,
}
```

## 🎯 **Performance Regression Detection**

### **Automated Performance Testing**

```rust
#[derive(Debug)]
struct PerformanceBaseline {
    operation: String,
    expected_ops_per_sec: f64,
    tolerance: f64, // Allowed deviation percentage
}

struct RegressionDetector {
    baselines: Vec<PerformanceBaseline>,
}

impl RegressionDetector {
    fn new() -> Self {
        Self {
            baselines: vec![
                PerformanceBaseline {
                    operation: "bounded_queue_enqueue".to_string(),
                    expected_ops_per_sec: 10_000_000.0,
                    tolerance: 0.10, // 10% tolerance
                },
                PerformanceBaseline {
                    operation: "bounded_queue_dequeue".to_string(),
                    expected_ops_per_sec: 12_000_000.0,
                    tolerance: 0.10,
                },
            ],
        }
    }
    
    fn check_regression(&self, operation: &str, actual_ops_per_sec: f64) -> RegressionResult {
        if let Some(baseline) = self.baselines.iter().find(|b| b.operation == operation) {
            let deviation = (actual_ops_per_sec - baseline.expected_ops_per_sec) / baseline.expected_ops_per_sec;
            
            if deviation < -baseline.tolerance {
                RegressionResult::Regression {
                    operation: operation.to_string(),
                    expected: baseline.expected_ops_per_sec,
                    actual: actual_ops_per_sec,
                    deviation_percent: deviation * 100.0,
                }
            } else if deviation > baseline.tolerance {
                RegressionResult::Improvement {
                    operation: operation.to_string(),
                    expected: baseline.expected_ops_per_sec,
                    actual: actual_ops_per_sec,
                    improvement_percent: deviation * 100.0,
                }
            } else {
                RegressionResult::WithinTolerance
            }
        } else {
            RegressionResult::NoBaseline(operation.to_string())
        }
    }
}

#[derive(Debug)]
enum RegressionResult {
    Regression {
        operation: String,
        expected: f64,
        actual: f64,
        deviation_percent: f64,
    },
    Improvement {
        operation: String,
        expected: f64,
        actual: f64,
        improvement_percent: f64,
    },
    WithinTolerance,
    NoBaseline(String),
}
```

## 🛠️ **Analysis Tools Integration**

### **Flamegraph Generation**

```rust
// Requires flamegraph crate
#[cfg(feature = "profiling")]
fn profile_with_flamegraph<F>(name: &str, operation: F) 
where
    F: FnOnce(),
{
    let guard = pprof::ProfilerGuard::new(100).unwrap();
    
    operation();
    
    if let Ok(report) = guard.report().build() {
        let file = std::fs::File::create(format!("{}.svg", name)).unwrap();
        report.flamegraph(file).unwrap();
        println!("Flamegraph saved to {}.svg", name);
    }
}
```

### **Custom Performance Metrics**

```rust
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct MetricsCollector {
    counters: HashMap<String, u64>,
    timers: HashMap<String, Vec<Duration>>,
    gauges: HashMap<String, f64>,
}

impl MetricsCollector {
    pub fn increment_counter(&mut self, name: &str) {
        *self.counters.entry(name.to_string()).or_insert(0) += 1;
    }
    
    pub fn record_time(&mut self, name: &str, duration: Duration) {
        self.timers.entry(name.to_string()).or_default().push(duration);
    }
    
    pub fn set_gauge(&mut self, name: &str, value: f64) {
        self.gauges.insert(name.to_string(), value);
    }
    
    pub fn report(&self) {
        println!("Performance Metrics Report");
        println!("==========================");
        
        for (name, count) in &self.counters {
            println!("Counter {}: {}", name, count);
        }
        
        for (name, times) in &self.timers {
            if !times.is_empty() {
                let avg = times.iter().sum::<Duration>() / times.len() as u32;
                println!("Timer {} (avg): {:?}", name, avg);
            }
        }
        
        for (name, value) in &self.gauges {
            println!("Gauge {}: {:.2}", name, value);
        }
    }
}
```

---

*Tags: #performance-analysis #benchmarking #profiling #metrics #regression-testing #cache-analysis #throughput #latency*

*Links: [[zettel-index]] | [[Bounded vs Unbounded Collections]] | [[Cache-Friendly Data Structures]] | [[Performance Optimization]] | [[Benchmarking Strategies]] | [[mission-2]]*
