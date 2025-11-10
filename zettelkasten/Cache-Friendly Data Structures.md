# Cache-Friendly Data Structures

*Memory layout patterns that optimize CPU cache utilization for high-performance data structures*

---

## 🧠 **Cache Hierarchy Fundamentals**

### **CPU Cache Levels**
```
L1 Cache: ~32KB,  ~1-2 cycles  (per core)
L2 Cache: ~256KB, ~10 cycles   (per core)  
L3 Cache: ~8MB,   ~40 cycles   (shared)
RAM:      ~16GB,  ~200 cycles  (system)
```

**Key Insight**: Accessing data in cache is **100-200x faster** than RAM access

### **Cache Line Size**
- Modern CPUs: **64 bytes** per cache line
- **Spatial locality**: Adjacent memory locations loaded together
- **Temporal locality**: Recently accessed data likely to be accessed again

## 🏗️ **Data Structure Layout Patterns**

### **1. Array of Structures (AoS) vs Structure of Arrays (SoA)**

#### **Array of Structures - Poor Cache Utilization**
```rust
#[derive(Debug)]
struct Particle {
    position: [f32; 3],    // 12 bytes
    velocity: [f32; 3],    // 12 bytes  
    mass: f32,             // 4 bytes
    // Total: 28 bytes per particle
}

fn update_positions_aos(particles: &mut [Particle], dt: f32) {
    for particle in particles {
        // Only need position and velocity, but load entire 28-byte struct
        particle.position[0] += particle.velocity[0] * dt;
        particle.position[1] += particle.velocity[1] * dt;  
        particle.position[2] += particle.velocity[2] * dt;
        // mass field loaded but unused - cache waste!
    }
}
```

**Cache Analysis**: Loading 28 bytes per particle, but only using 24 bytes (position + velocity)

#### **Structure of Arrays - Optimal Cache Utilization**
```rust
#[derive(Debug)]
struct ParticleSystem {
    positions: Vec<[f32; 3]>,    // Contiguous position data
    velocities: Vec<[f32; 3]>,   // Contiguous velocity data  
    masses: Vec<f32>,            // Separate mass data
}

impl ParticleSystem {
    fn update_positions(&mut self, dt: f32) {
        for (pos, vel) in self.positions.iter_mut().zip(&self.velocities) {
            // Perfect cache utilization - only load needed data
            pos[0] += vel[0] * dt;
            pos[1] += vel[1] * dt;
            pos[2] += vel[2] * dt;
        }
        // masses not touched - no cache pollution
    }
}
```

**Cache Analysis**: Only loads position (12 bytes) + velocity (12 bytes) = 24 bytes of relevant data

### **2. Hot/Cold Data Separation**

#### **Problematic Mixed Layout**
```rust
struct Node {
    // Hot data - accessed frequently
    value: i32,
    next: Option<Box<Node>>,
    
    // Cold data - accessed rarely
    debug_info: String,        // 24 bytes
    creation_time: SystemTime, // 12 bytes
    access_count: u64,         // 8 bytes
    // Cold data adds 44 bytes per node!
}
```

#### **Optimized Hot/Cold Separation**
```rust
struct Node {
    // Hot data only - fits in single cache line
    value: i32,              // 4 bytes
    next: Option<Box<Node>>, // 8 bytes  
    // Total: 12 bytes (52 bytes remaining in cache line)
}

struct NodeMetadata {
    // Cold data in separate structure
    debug_info: String,
    creation_time: SystemTime,
    access_count: u64,
}

// Optional: HashMap<NodeId, NodeMetadata> for cold data
```

**Benefit**: Hot path operations only load 12-byte nodes instead of 56-byte nodes

### **3. Padding and Alignment for Cache Lines**

#### **Cache Line Alignment**
```rust
#[repr(align(64))]  // Align to cache line boundary
struct CacheAlignedBuffer {
    data: [u8; 64],
}

// Ensures each buffer occupies exactly one cache line
// Prevents false sharing between CPU cores
```

#### **Strategic Padding**
```rust
struct OptimizedStruct {
    // Frequently accessed together - same cache line
    hot_field_a: u32,        // 4 bytes
    hot_field_b: u32,        // 4 bytes
    
    // Padding to separate hot and cold data
    _padding: [u8; 56],      // Fill rest of cache line
    
    // Cold data starts on new cache line
    cold_field: String,      // 24 bytes
}
```

## 🔄 **Cache-Friendly Algorithms**

### **1. Sequential Access Patterns**

#### **Cache-Friendly Linear Search**
```rust
fn linear_search_optimized<T: PartialEq>(slice: &[T], target: &T) -> Option<usize> {
    // Sequential access - perfect cache utilization
    slice.iter().position(|x| x == target)
}
```

#### **Cache-Hostile Random Access**
```rust
fn binary_search_cache_analysis<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    // Random access pattern - poor cache utilization for small searches
    // But still faster for large arrays due to O(log n) complexity
    slice.binary_search(target).ok()
}
```

**Trade-off**: Sequential algorithms can outperform logarithmic algorithms for small datasets

### **2. Block-Based Processing**

#### **Cache-Aware Matrix Multiplication**
```rust
fn matrix_multiply_blocked(a: &[Vec<f32>], b: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let n = a.len();
    let mut c = vec![vec![0.0; n]; n];
    const BLOCK_SIZE: usize = 64; // Tune for cache size
    
    // Process in blocks to maximize cache reuse
    for i_block in (0..n).step_by(BLOCK_SIZE) {
        for j_block in (0..n).step_by(BLOCK_SIZE) {
            for k_block in (0..n).step_by(BLOCK_SIZE) {
                
                // Inner loops work on cache-friendly blocks
                for i in i_block..std::cmp::min(i_block + BLOCK_SIZE, n) {
                    for j in j_block..std::cmp::min(j_block + BLOCK_SIZE, n) {
                        for k in k_block..std::cmp::min(k_block + BLOCK_SIZE, n) {
                            c[i][j] += a[i][k] * b[k][j];
                        }
                    }
                }
            }
        }
    }
    c
}
```

### **3. Prefetching Patterns**

#### **Manual Prefetching**
```rust
use std::arch::x86_64::_mm_prefetch;

fn process_with_prefetch(data: &[DataStruct]) {
    const PREFETCH_DISTANCE: usize = 8;
    
    for (i, item) in data.iter().enumerate() {
        // Prefetch future data while processing current
        if i + PREFETCH_DISTANCE < data.len() {
            unsafe {
                _mm_prefetch(
                    data.as_ptr().add(i + PREFETCH_DISTANCE) as *const i8,
                    _MM_HINT_T0  // Prefetch to L1 cache
                );
            }
        }
        
        // Process current item
        process_item(item);
    }
}
```

## 📊 **Ring Buffer Cache Optimization**

### **Cache-Aligned Ring Buffer**
```rust
#[repr(align(64))]  // Cache line alignment
pub struct CacheOptimizedRingBuffer<T> {
    // Separate cache lines for reader and writer to avoid false sharing
    buf: Box<[T]>,
    capacity: usize,
    
    // Writer-only fields (cache line 1)
    _cache_pad1: [u8; 64 - 16], // Padding calculation
    head: usize,        // Write position
    
    // Reader-only fields (cache line 2)  
    _cache_pad2: [u8; 64 - 8],
    tail: usize,        // Read position
    len: usize,         // Current count
}
```

### **SIMD-Friendly Buffer Operations**
```rust
impl<T: Copy> CacheOptimizedRingBuffer<T> {
    /// Bulk copy optimized for SIMD
    pub fn enqueue_bulk(&mut self, items: &[T]) -> Result<(), &[T]> {
        if items.len() > self.remaining_capacity() {
            return Err(items);
        }
        
        let head = self.head;
        let capacity = self.capacity;
        
        if head + items.len() <= capacity {
            // Contiguous copy - SIMD optimized
            unsafe {
                std::ptr::copy_nonoverlapping(
                    items.as_ptr(),
                    self.buf.as_mut_ptr().add(head),
                    items.len()
                );
            }
            self.head = head + items.len();
        } else {
            // Wrapped copy - two SIMD operations
            let first_chunk = capacity - head;
            let second_chunk = items.len() - first_chunk;
            
            unsafe {
                // First part to end of buffer
                std::ptr::copy_nonoverlapping(
                    items.as_ptr(),
                    self.buf.as_mut_ptr().add(head),
                    first_chunk
                );
                
                // Second part to beginning of buffer
                std::ptr::copy_nonoverlapping(
                    items.as_ptr().add(first_chunk),
                    self.buf.as_mut_ptr(),
                    second_chunk
                );
            }
            self.head = second_chunk;
        }
        
        self.len += items.len();
        Ok(())
    }
}
```

## 🔬 **Cache Performance Measurement**

### **Cache Miss Profiling**
```rust
use std::time::Instant;

fn benchmark_cache_patterns() {
    const SIZE: usize = 1_000_000;
    let data: Vec<u64> = (0..SIZE as u64).collect();
    
    // Sequential access - cache friendly
    let start = Instant::now();
    let sum_sequential: u64 = data.iter().sum();
    let sequential_time = start.elapsed();
    
    // Random access - cache hostile  
    use rand::prelude::*;
    let mut rng = thread_rng();
    let indices: Vec<usize> = (0..SIZE).map(|_| rng.gen_range(0..SIZE)).collect();
    
    let start = Instant::now();
    let sum_random: u64 = indices.iter().map(|&i| data[i]).sum();
    let random_time = start.elapsed();
    
    println!("Sequential: {:?}, Random: {:?}", sequential_time, random_time);
    println!("Cache penalty: {:.2}x", random_time.as_nanos() as f64 / sequential_time.as_nanos() as f64);
}
```

### **Cache Line Utilization Analysis**
```rust
fn analyze_cache_utilization<T>(data: &[T]) {
    let element_size = std::mem::size_of::<T>();
    let cache_line_size = 64; // bytes
    let elements_per_cache_line = cache_line_size / element_size;
    
    println!("Element size: {} bytes", element_size);
    println!("Elements per cache line: {}", elements_per_cache_line);
    println!("Cache line utilization: {:.1}%", 
        (elements_per_cache_line * element_size) as f64 / cache_line_size as f64 * 100.0);
}
```

## 🎯 **Optimization Guidelines**

### **1. Data Structure Design**
- ✅ **Group frequently accessed fields** together
- ✅ **Separate hot and cold data** into different structures
- ✅ **Align to cache line boundaries** for shared data
- ✅ **Use SoA over AoS** when processing subsets of fields
- ❌ **Avoid unnecessary padding** that wastes cache space

### **2. Algorithm Design**
- ✅ **Prefer sequential access** patterns when possible
- ✅ **Process data in blocks** that fit in cache
- ✅ **Minimize pointer chasing** (linked lists, trees)
- ✅ **Use bulk operations** for SIMD optimization
- ❌ **Avoid random memory access** patterns

### **3. Memory Layout**
- ✅ **Pack related data** into single cache lines
- ✅ **Align critical structures** to cache boundaries
- ✅ **Consider false sharing** in multi-threaded code
- ✅ **Size working sets** to fit in cache levels
- ❌ **Don't over-optimize** at expense of code clarity

## 🏆 **Performance Impact Examples**

### **Cache-Friendly vs Cache-Hostile Comparison**
```rust
// Cache-hostile: 50ms for 1M elements
struct BadLayout {
    frequently_used: u32,     // 4 bytes
    padding: [u8; 60],        // Waste cache space
    rarely_used: String,      // 24 bytes
}

// Cache-friendly: 15ms for 1M elements  
struct GoodLayout {
    frequently_used: u32,     // 4 bytes - fits 16 per cache line
}
```

**Result**: 3.3x performance improvement through better cache utilization

### **Ring Buffer Performance Comparison**
```rust
// Standard implementation: 100 MB/s throughput
struct StandardRingBuffer<T> {
    buf: Vec<T>,
    head: usize,
    tail: usize,
}

// Cache-optimized: 350 MB/s throughput
#[repr(align(64))]
struct OptimizedRingBuffer<T> {
    buf: Box<[T]>,
    head: usize,
    _pad1: [u8; 56],
    tail: usize, 
    _pad2: [u8; 56],
}
```

**Result**: 3.5x throughput improvement through cache line alignment

## 🔧 **Tools and Profiling**

### **Performance Monitoring**
```bash
# Linux - Monitor cache misses
perf stat -e cache-misses,cache-references ./program

# Profile cache behavior
perf record -e cache-misses ./program
perf report
```

### **Rust Profiling Tools**
```rust
// Criterion benchmarks with cache analysis
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_cache_patterns(c: &mut Criterion) {
    c.bench_function("sequential_access", |b| {
        let data = vec![0u64; 1000000];
        b.iter(|| {
            black_box(data.iter().sum::<u64>())
        });
    });
}
```

---

*Tags: #cache-optimization #memory-layout #performance #cpu-cache #data-structures #simd #alignment*

*Links: [[zettel-index]] | [[Ring Buffer Implementation Patterns]] | [[Performance Analysis Patterns]] | [[Memory Layout Optimization]] | [[SIMD Optimization Patterns]] | [[mission-2]]*