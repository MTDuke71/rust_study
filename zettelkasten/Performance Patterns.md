# Performance Patterns

*Created: 2025-11-08*
*Tags: #rust-optimization #performance-engineering #algorithmic-optimization #memory-patterns #cpu-optimization*

## Overview

Performance patterns in Rust encompass **systematic approaches** to optimization that leverage Rust's unique features: zero-cost abstractions, ownership system, and explicit memory management. These patterns balance **developer productivity** with **runtime efficiency**.

## Memory Access Patterns

### Cache-Friendly Data Layout
```rust
// Bad: Array of Structures (AoS) - poor cache locality
struct ParticleAoS {
    position: (f32, f32, f32),
    velocity: (f32, f32, f32),
    mass: f32,
}

fn update_positions_aos(particles: &mut [ParticleAoS]) {
    for particle in particles {
        particle.position.0 += particle.velocity.0; // Cache miss likely
        particle.position.1 += particle.velocity.1;
        particle.position.2 += particle.velocity.2;
    }
}

// Good: Structure of Arrays (SoA) - optimal cache usage
struct ParticlesSoA {
    positions_x: Vec<f32>,
    positions_y: Vec<f32>, 
    positions_z: Vec<f32>,
    velocities_x: Vec<f32>,
    velocities_y: Vec<f32>,
    velocities_z: Vec<f32>,
    masses: Vec<f32>,
}

fn update_positions_soa(particles: &mut ParticlesSoA) {
    // Vectorizable loop - excellent cache locality
    for i in 0..particles.positions_x.len() {
        particles.positions_x[i] += particles.velocities_x[i]; // Sequential access
        particles.positions_y[i] += particles.velocities_y[i];
        particles.positions_z[i] += particles.velocities_z[i];
    }
}
```

### Memory Pre-allocation
```rust
// Inefficient: Multiple allocations
fn build_result_inefficient(data: &[i32]) -> Vec<String> {
    let mut result = Vec::new(); // Starts with 0 capacity
    for &item in data {
        result.push(item.to_string()); // Reallocations as it grows
    }
    result
}

// Efficient: Pre-allocated capacity
fn build_result_efficient(data: &[i32]) -> Vec<String> {
    let mut result = Vec::with_capacity(data.len()); // Single allocation
    for &item in data {
        result.push(item.to_string()); // No reallocations
    }
    result
}

// Even better: Iterator + collect
fn build_result_optimal(data: &[i32]) -> Vec<String> {
    data.iter()
        .map(|&item| item.to_string())
        .collect() // Size hint allows optimal allocation
}
```

## Algorithm Optimization Patterns

### Iterator Chain Optimization
```rust
// Suboptimal: Multiple passes over data
fn process_data_multipass(numbers: &[i32]) -> Vec<i32> {
    let evens: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .cloned()
        .collect();
    
    let doubled: Vec<i32> = evens.iter()
        .map(|&x| x * 2)
        .collect();
    
    doubled.into_iter()
        .filter(|&x| x > 10)
        .collect()
}

// Optimal: Single pass with iterator fusion
fn process_data_singlepass(numbers: &[i32]) -> Vec<i32> {
    numbers.iter()
        .filter(|&&x| x % 2 == 0)    // Fused into single loop
        .map(|&x| x * 2)             // Compiler optimizes
        .filter(|&x| x > 10)         // to single iteration
        .collect()
}
```

### Branching Optimization
```rust
// Branch-heavy code - unpredictable for CPU
fn classify_numbers_branchy(numbers: &[i32]) -> (usize, usize, usize) {
    let mut small = 0;
    let mut medium = 0; 
    let mut large = 0;
    
    for &num in numbers {
        if num < 10 {
            small += 1;
        } else if num < 100 {
            medium += 1;
        } else {
            large += 1;
        }
    }
    (small, medium, large)
}

// Branch-free optimization using arithmetic
fn classify_numbers_branchless(numbers: &[i32]) -> (usize, usize, usize) {
    numbers.iter().fold((0, 0, 0), |(small, medium, large), &num| {
        let is_small = (num < 10) as usize;
        let is_medium = ((num >= 10) & (num < 100)) as usize;
        let is_large = (num >= 100) as usize;
        
        (small + is_small, medium + is_medium, large + is_large)
    })
}
```

## Mission-Specific Optimization Patterns

### Mission 1: Stack Optimization
```rust
// Basic implementation
struct Stack<T> {
    items: Vec<T>,
}

// Optimized with capacity management
struct OptimizedStack<T> {
    items: Vec<T>,
    max_capacity: usize,
}

impl<T> OptimizedStack<T> {
    fn new_with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            max_capacity: capacity,
        }
    }
    
    fn push(&mut self, item: T) -> Result<(), T> {
        if self.items.len() < self.max_capacity {
            self.items.push(item);
            Ok(())
        } else {
            Err(item) // Avoid reallocation by rejecting overflow
        }
    }
    
    // Optimized batch operations
    fn push_many(&mut self, items: impl IntoIterator<Item = T>) -> Result<(), Vec<T>> {
        let items: Vec<T> = items.into_iter().collect();
        
        if self.items.len() + items.len() <= self.max_capacity {
            self.items.extend(items);
            Ok(())
        } else {
            Err(items)
        }
    }
}
```

### Mission 5: HashMap Performance
```rust
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};

// Custom hasher for known data patterns
struct IdentityHasher(u64);

impl Hasher for IdentityHasher {
    fn write(&mut self, bytes: &[u8]) {
        // For sequential integer keys, identity hash is optimal
        for &byte in bytes {
            self.0 = self.0.wrapping_mul(31).wrapping_add(byte as u64);
        }
    }
    
    fn finish(&self) -> u64 {
        self.0
    }
}

// Optimized HashMap for specific use case
fn create_optimized_map(data: &[(u32, String)]) -> HashMap<u32, String> {
    let mut map = HashMap::with_capacity_and_hasher(
        data.len(),
        std::collections::hash_map::RandomState::new(),
    );
    
    // Batch insertion is more efficient
    map.extend(data.iter().map(|(k, v)| (*k, v.clone())));
    map
}
```

### Mission 6: Grid Access Optimization
```rust
// Cache-unfriendly: column-major access
fn sum_columns_slow(grid: &[Vec<i32>]) -> Vec<i32> {
    let cols = grid[0].len();
    let mut column_sums = vec![0; cols];
    
    for col in 0..cols {
        for row in 0..grid.len() {
            column_sums[col] += grid[row][col]; // Cache miss on each access
        }
    }
    column_sums
}

// Cache-friendly: row-major with transpose
fn sum_columns_fast(grid: &[Vec<i32>]) -> Vec<i32> {
    let cols = grid[0].len();
    let mut column_sums = vec![0; cols];
    
    // Single pass through data in memory order
    for row in grid {
        for (col_idx, &value) in row.iter().enumerate() {
            column_sums[col_idx] += value; // Sequential memory access
        }
    }
    column_sums
}
```

## AoC Performance Patterns

### Day 15: Combinatorial Optimization
```rust
// Naive approach: full enumeration
fn find_optimal_naive(ingredients: &[Ingredient]) -> i32 {
    let mut max_score = 0;
    
    // Inefficient: O(n^4) nested loops
    for a in 0..=100 {
        for b in 0..=100-a {
            for c in 0..=100-a-b {
                let d = 100 - a - b - c;
                max_score = max_score.max(calculate_score(ingredients, [a, b, c, d]));
            }
        }
    }
    max_score
}

// Optimized: constraint-based generation  
fn find_optimal_constrained(ingredients: &[Ingredient]) -> i32 {
    (0..=100)
        .flat_map(|a| (0..=100-a).map(move |b| (a, b)))
        .flat_map(|(a, b)| (0..=100-a-b).map(move |c| (a, b, c, 100-a-b-c)))
        .map(|(a, b, c, d)| calculate_score(ingredients, [a, b, c, d]))
        .max()
        .unwrap_or(0)
}
```

### Day 11: String Generation Optimization
```rust
// Slow: String manipulation
fn increment_password_string(password: &str) -> String {
    let mut chars: Vec<char> = password.chars().collect();
    
    for i in (0..chars.len()).rev() {
        if chars[i] == 'z' {
            chars[i] = 'a';
        } else {
            chars[i] = ((chars[i] as u8) + 1) as char;
            break;
        }
    }
    
    chars.into_iter().collect()
}

// Fast: Byte manipulation
fn increment_password_bytes(password: &mut [u8]) {
    for i in (0..password.len()).rev() {
        if password[i] == b'z' {
            password[i] = b'a';
        } else {
            password[i] += 1;
            break;
        }
    }
}
```

## Concurrent Performance Patterns

### Parallel Processing with Rayon
```rust
use rayon::prelude::*;

// Sequential processing
fn process_sequential(data: &[i32]) -> Vec<i32> {
    data.iter()
        .map(|&x| expensive_computation(x))
        .collect()
}

// Parallel processing - automatic work stealing
fn process_parallel(data: &[i32]) -> Vec<i32> {
    data.par_iter()
        .map(|&x| expensive_computation(x))
        .collect()
}

// Chunk-based processing for better cache locality
fn process_chunked_parallel(data: &[i32]) -> Vec<i32> {
    data.par_chunks(1000) // Process in cache-friendly chunks
        .flat_map(|chunk| {
            chunk.iter().map(|&x| expensive_computation(x))
        })
        .collect()
}
```

### Lock-Free Data Structures
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// Traditional mutex-based counter (lock contention)
use std::sync::Mutex;

struct LockedCounter {
    value: Mutex<usize>,
}

// Lock-free atomic counter (no contention)
struct AtomicCounter {
    value: AtomicUsize,
}

impl AtomicCounter {
    fn increment(&self) -> usize {
        self.value.fetch_add(1, Ordering::Relaxed)
    }
    
    fn get(&self) -> usize {
        self.value.load(Ordering::Relaxed)
    }
}
```

## Memory Layout Optimization

### Enum Optimization
```rust
// Suboptimal: Large enum variants
enum Message {
    Small(u8),
    Large([u8; 1000]), // Forces entire enum to be 1000+ bytes
}

// Optimized: Box large variants
enum OptimizedMessage {
    Small(u8),
    Large(Box<[u8; 1000]>), // Enum now only size of largest unboxed variant + discriminant
}

// Even better: Separate types
enum MessageType {
    Small(u8),
    Large(Box<LargeData>),
}

struct LargeData {
    data: [u8; 1000],
}
```

### Bit Packing
```rust
// Inefficient: Multiple bools take 1 byte each
struct Flags {
    flag1: bool, // 1 byte
    flag2: bool, // 1 byte 
    flag3: bool, // 1 byte
    flag4: bool, // 1 byte
    // Total: 4 bytes
}

// Efficient: Bit field in single byte
struct PackedFlags {
    flags: u8, // All flags in 1 byte
}

impl PackedFlags {
    const FLAG1: u8 = 0b0001;
    const FLAG2: u8 = 0b0010;
    const FLAG3: u8 = 0b0100;
    const FLAG4: u8 = 0b1000;
    
    fn set_flag1(&mut self, value: bool) {
        if value {
            self.flags |= Self::FLAG1;
        } else {
            self.flags &= !Self::FLAG1;
        }
    }
    
    fn get_flag1(&self) -> bool {
        (self.flags & Self::FLAG1) != 0
    }
}
```

## Profile-Guided Optimization

### Hot Path Identification
```rust
// Use profiling tools to identify hot paths
fn hot_path_example(data: &[i32]) -> i32 {
    let mut sum = 0;
    
    // This loop might be identified as hot path by profiler
    for &value in data {
        sum += expensive_operation(value); // Optimize this function
    }
    
    sum
}

// Optimized version based on profiling
fn optimized_hot_path(data: &[i32]) -> i32 {
    data.iter()
        .map(|&value| {
            // Inline expensive operations identified by profiler
            #[inline]
            fn fast_operation(x: i32) -> i32 {
                // Optimized implementation
                x * x + x // Instead of complex calculation
            }
            fast_operation(value)
        })
        .sum()
}
```

### Branch Prediction Optimization
```rust
// Help branch predictor with likely/unlikely hints
fn search_with_hints(data: &[i32], target: i32) -> Option<usize> {
    for (i, &value) in data.iter().enumerate() {
        if value == target {
            return Some(i); // Likely case for search hit
        }
    }
    None // Unlikely case - not found
}

// Organize code to optimize for common case
fn optimized_search(data: &[i32], target: i32) -> Option<usize> {
    // Common case first - improves branch prediction
    data.iter()
        .position(|&value| value == target)
}
```

## Integration with Other Concepts

- **[[zero-cost-abstractions]]**: High-level patterns without runtime cost
- **[[Performance Benchmarking]]**: Measuring optimization effectiveness
- **[[interior-mutability]]**: Managing shared state performance
- **[[Memory Safety]]**: Safe optimization techniques
- **[[Big-O Notation]]**: Algorithmic complexity optimization

## Daily Study Applications

### Week 2: Collection Optimization
- Vector capacity management
- Iterator fusion patterns
- HashMap sizing strategies

### Week 5: Error Handling Performance  
- Result<T, E> optimization patterns
- Error propagation efficiency
- Zero-cost error handling

### Week 6: Advanced Optimization
- SIMD operation patterns
- Async runtime optimization
- Memory pool patterns

## Mission Integration Examples

### Mission Performance Reports
- **Mission 1**: Stack operation optimization (O(1) amortized)
- **Mission 4**: LinkedList memory layout optimization
- **Mission 5**: HashMap load factor and rehashing patterns
- **Mission 6**: Grid traversal cache optimization
- **Mission 10**: Union-Find path compression efficiency

### Optimization Methodology
1. **Profile First**: Identify actual bottlenecks
2. **Measure Everything**: Before and after benchmarks
3. **Validate Correctness**: Ensure optimizations don't break functionality
4. **Document Trade-offs**: Performance vs readability vs safety

## Further Reading

- **[[Performance Benchmarking]]**: Measuring optimization effectiveness
- **[[zero-cost-abstractions]]**: Rust's performance philosophy
- **[[Big-O Notation]]**: Algorithmic complexity analysis
- **[[interior-mutability]]**: Performance costs of shared mutability

---

*Performance Patterns Links:*
- [[zero-cost-abstractions]] - High-level performance patterns
- [[Performance Benchmarking]] - Measuring optimization results
- [[interior-mutability]] - Managing shared state costs
- [[Memory Safety]] - Safe optimization techniques
- [[Big-O Notation]] - Algorithmic optimization
- [[mission-1]] - Stack optimization examples
- [[mission-5]] - HashMap performance patterns
- [[mission-6]] - Grid access optimization