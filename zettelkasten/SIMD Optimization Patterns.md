# SIMD Optimization Patterns

*Single Instruction, Multiple Data (SIMD) vectorization techniques for high-performance data structure operations*

---

## 🎯 **SIMD Fundamentals**

### **What is SIMD?**
SIMD allows a single CPU instruction to operate on multiple data elements simultaneously:

```rust
// Scalar operation (one at a time)
let a = [1, 2, 3, 4];
let b = [5, 6, 7, 8];
let mut result = [0; 4];
for i in 0..4 {
    result[i] = a[i] + b[i];  // 4 separate add instructions
}

// SIMD operation (all at once)
// One instruction adds all 4 pairs simultaneously
// result = [6, 8, 10, 12]
```

### **SIMD Capabilities by Architecture**
```rust
// x86/x86_64 SIMD extensions
// SSE:    128-bit vectors (4×f32, 2×f64, 16×i8, 8×i16, 4×i32)
// AVX:    256-bit vectors (8×f32, 4×f64, 32×i8, 16×i16, 8×i32)
// AVX-512: 512-bit vectors (16×f32, 8×f64, 64×i8, 32×i16, 16×i32)

// ARM NEON: 128-bit vectors
// Similar to SSE capabilities

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
```

## 🏗️ **Rust SIMD Approaches**

### **1. std::simd (Portable SIMD) - Stable in 1.72+**
```rust
#![feature(portable_simd)]
use std::simd::{f32x4, f32x8, Simd};

fn simd_add_vectors(a: &[f32], b: &[f32], result: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), result.len());
    
    // Process 8 elements at a time with AVX
    let chunks = a.len() / 8;
    for i in 0..chunks {
        let start = i * 8;
        let a_vec = f32x8::from_slice(&a[start..start + 8]);
        let b_vec = f32x8::from_slice(&b[start..start + 8]);
        let sum = a_vec + b_vec;
        sum.copy_to_slice(&mut result[start..start + 8]);
    }
    
    // Handle remaining elements scalar
    for i in (chunks * 8)..a.len() {
        result[i] = a[i] + b[i];
    }
}
```

### **2. Platform-Specific Intrinsics**
```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[target_feature(enable = "avx2")]
unsafe fn avx2_sum_i32(data: &[i32]) -> i32 {
    assert!(data.len() >= 8);
    
    let mut sum_vec = _mm256_setzero_si256();
    
    // Process 8 i32 values at once
    let chunks = data.len() / 8;
    for i in 0..chunks {
        let ptr = data.as_ptr().add(i * 8);
        let values = _mm256_loadu_si256(ptr as *const __m256i);
        sum_vec = _mm256_add_epi32(sum_vec, values);
    }
    
    // Horizontal sum of vector elements
    let sum_vec = _mm256_hadd_epi32(sum_vec, sum_vec);
    let sum_vec = _mm256_hadd_epi32(sum_vec, sum_vec);
    
    let result = _mm256_extract_epi32(sum_vec, 0) + 
                 _mm256_extract_epi32(sum_vec, 4);
    
    // Add remaining elements scalar
    let mut scalar_sum = result;
    for i in (chunks * 8)..data.len() {
        scalar_sum += data[i];
    }
    
    scalar_sum
}
```

### **3. Auto-Vectorization**
```rust
// Compiler can often auto-vectorize simple loops
fn auto_vectorized_sum(data: &[f32]) -> f32 {
    // Compiler likely vectorizes this with -O2/-O3
    data.iter().sum()
}

// Help the compiler with explicit patterns
fn explicit_vectorizable(a: &mut [f32], b: &[f32]) {
    // Iterator approach - easily vectorized
    for (dst, &src) in a.iter_mut().zip(b.iter()) {
        *dst += src;
    }
}

// Check generated assembly with:
// cargo rustc --release -- --emit asm
```

## 🚀 **SIMD Data Structure Patterns**

### **Vectorized Array Operations**
```rust
use std::simd::{f32x8, u32x8, Simd};

struct SimdArray<T, const N: usize> {
    data: [T; N],
}

impl SimdArray<f32, 1024> {
    fn add_scalar(&mut self, scalar: f32) {
        const LANES: usize = 8;
        let scalar_vec = f32x8::splat(scalar);
        
        // Vectorized portion
        let chunks = self.data.len() / LANES;
        for i in 0..chunks {
            let start = i * LANES;
            let mut vec = f32x8::from_slice(&self.data[start..start + LANES]);
            vec += scalar_vec;
            vec.copy_to_slice(&mut self.data[start..start + LANES]);
        }
        
        // Scalar remainder
        for i in (chunks * LANES)..self.data.len() {
            self.data[i] += scalar;
        }
    }
    
    fn dot_product(&self, other: &Self) -> f32 {
        const LANES: usize = 8;
        let mut sum_vec = f32x8::splat(0.0);
        
        let chunks = self.data.len() / LANES;
        for i in 0..chunks {
            let start = i * LANES;
            let a = f32x8::from_slice(&self.data[start..start + LANES]);
            let b = f32x8::from_slice(&other.data[start..start + LANES]);
            sum_vec += a * b;
        }
        
        // Horizontal sum
        let sum_array = sum_vec.to_array();
        let mut result: f32 = sum_array.iter().sum();
        
        // Add scalar remainder
        for i in (chunks * LANES)..self.data.len() {
            result += self.data[i] * other.data[i];
        }
        
        result
    }
}
```

### **SIMD-Friendly Matrix Storage**
```rust
// Row-major storage for cache-friendly SIMD access
struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl Matrix<f32> {
    fn new(rows: usize, cols: usize) -> Self {
        // Pad columns to SIMD width for alignment
        let padded_cols = (cols + 7) & !7; // Round up to multiple of 8
        Self {
            data: vec![0.0; rows * padded_cols],
            rows,
            cols: padded_cols,
        }
    }
    
    fn row_multiply_scalar(&mut self, row: usize, scalar: f32) {
        let start = row * self.cols;
        let row_data = &mut self.data[start..start + self.cols];
        
        let scalar_vec = f32x8::splat(scalar);
        let chunks = self.cols / 8;
        
        for i in 0..chunks {
            let start = i * 8;
            let mut vec = f32x8::from_slice(&row_data[start..start + 8]);
            vec *= scalar_vec;
            vec.copy_to_slice(&mut row_data[start..start + 8]);
        }
    }
    
    fn matrix_vector_multiply(&self, vector: &[f32]) -> Vec<f32> {
        let mut result = vec![0.0; self.rows];
        
        for row in 0..self.rows {
            result[row] = self.row_dot_product(row, vector);
        }
        
        result
    }
    
    fn row_dot_product(&self, row: usize, vector: &[f32]) -> f32 {
        let start = row * self.cols;
        let row_data = &self.data[start..start + self.cols.min(vector.len())];
        
        let mut sum_vec = f32x8::splat(0.0);
        let chunks = row_data.len() / 8;
        
        for i in 0..chunks {
            let start = i * 8;
            let a = f32x8::from_slice(&row_data[start..start + 8]);
            let b = f32x8::from_slice(&vector[start..start + 8]);
            sum_vec += a * b;
        }
        
        let sum_array = sum_vec.to_array();
        sum_array.iter().sum()
    }
}
```

## 🔍 **Search and Filter Operations**

### **SIMD String Search**
```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// Find first occurrence of byte in slice using SIMD
#[target_feature(enable = "sse2")]
unsafe fn simd_find_byte(haystack: &[u8], needle: u8) -> Option<usize> {
    let needle_vec = _mm_set1_epi8(needle as i8);
    let chunks = haystack.len() / 16;
    
    for i in 0..chunks {
        let start = i * 16;
        let data = _mm_loadu_si128(haystack.as_ptr().add(start) as *const __m128i);
        let comparison = _mm_cmpeq_epi8(data, needle_vec);
        let mask = _mm_movemask_epi8(comparison);
        
        if mask != 0 {
            // Found match, find first bit set
            let offset = mask.trailing_zeros() as usize;
            return Some(start + offset);
        }
    }
    
    // Check remaining bytes
    for i in (chunks * 16)..haystack.len() {
        if haystack[i] == needle {
            return Some(i);
        }
    }
    
    None
}
```

### **Parallel Comparison Operations**
```rust
use std::simd::{u8x32, mask32};

fn simd_count_matches(data: &[u8], target: u8) -> usize {
    let target_vec = u8x32::splat(target);
    let mut count = 0;
    
    let chunks = data.len() / 32;
    for i in 0..chunks {
        let start = i * 32;
        let data_vec = u8x32::from_slice(&data[start..start + 32]);
        let matches = data_vec.simd_eq(target_vec);
        count += matches.to_bitmask().count_ones() as usize;
    }
    
    // Handle remainder
    for i in (chunks * 32)..data.len() {
        if data[i] == target {
            count += 1;
        }
    }
    
    count
}

fn simd_filter_range(data: &[i32], min: i32, max: i32) -> Vec<i32> {
    use std::simd::{i32x8, Simd};
    
    let min_vec = i32x8::splat(min);
    let max_vec = i32x8::splat(max);
    let mut result = Vec::new();
    
    let chunks = data.len() / 8;
    for i in 0..chunks {
        let start = i * 8;
        let data_vec = i32x8::from_slice(&data[start..start + 8]);
        
        let ge_min = data_vec.simd_ge(min_vec);
        let le_max = data_vec.simd_le(max_vec);
        let in_range = ge_min & le_max;
        
        // Extract matching elements
        let mask = in_range.to_bitmask();
        for j in 0..8 {
            if (mask >> j) & 1 != 0 {
                result.push(data[start + j]);
            }
        }
    }
    
    // Handle remainder
    for &value in &data[(chunks * 8)..] {
        if value >= min && value <= max {
            result.push(value);
        }
    }
    
    result
}
```

## 📊 **Aggregation Operations**

### **SIMD Reductions**
```rust
use std::simd::{f64x4, i32x8};

fn simd_sum_f64(data: &[f64]) -> f64 {
    let mut sum_vec = f64x4::splat(0.0);
    
    let chunks = data.len() / 4;
    for i in 0..chunks {
        let start = i * 4;
        let data_vec = f64x4::from_slice(&data[start..start + 4]);
        sum_vec += data_vec;
    }
    
    // Horizontal sum
    let sum_array = sum_vec.to_array();
    let mut result: f64 = sum_array.iter().sum();
    
    // Add remainder
    for &value in &data[(chunks * 4)..] {
        result += value;
    }
    
    result
}

fn simd_min_max_i32(data: &[i32]) -> (i32, i32) {
    if data.is_empty() {
        return (i32::MAX, i32::MIN);
    }
    
    let mut min_vec = i32x8::splat(data[0]);
    let mut max_vec = i32x8::splat(data[0]);
    
    let chunks = data.len() / 8;
    for i in 0..chunks {
        let start = i * 8;
        let data_vec = i32x8::from_slice(&data[start..start + 8]);
        min_vec = min_vec.simd_min(data_vec);
        max_vec = max_vec.simd_max(data_vec);
    }
    
    // Horizontal min/max
    let min_array = min_vec.to_array();
    let max_array = max_vec.to_array();
    
    let mut min_result = *min_array.iter().min().unwrap();
    let mut max_result = *max_array.iter().max().unwrap();
    
    // Handle remainder
    for &value in &data[(chunks * 8)..] {
        min_result = min_result.min(value);
        max_result = max_result.max(value);
    }
    
    (min_result, max_result)
}
```

### **Statistical Operations**
```rust
fn simd_mean_variance(data: &[f32]) -> (f32, f32) {
    if data.is_empty() {
        return (0.0, 0.0);
    }
    
    // First pass: calculate mean
    let sum = simd_sum_f32(data);
    let mean = sum / data.len() as f32;
    
    // Second pass: calculate variance
    let mean_vec = f32x8::splat(mean);
    let mut sum_squares_vec = f32x8::splat(0.0);
    
    let chunks = data.len() / 8;
    for i in 0..chunks {
        let start = i * 8;
        let data_vec = f32x8::from_slice(&data[start..start + 8]);
        let diff = data_vec - mean_vec;
        sum_squares_vec += diff * diff;
    }
    
    let sum_squares_array = sum_squares_vec.to_array();
    let mut variance: f32 = sum_squares_array.iter().sum();
    
    // Handle remainder
    for &value in &data[(chunks * 8)..] {
        let diff = value - mean;
        variance += diff * diff;
    }
    
    variance /= data.len() as f32;
    (mean, variance)
}

fn simd_sum_f32(data: &[f32]) -> f32 {
    let mut sum_vec = f32x8::splat(0.0);
    
    let chunks = data.len() / 8;
    for i in 0..chunks {
        let start = i * 8;
        let data_vec = f32x8::from_slice(&data[start..start + 8]);
        sum_vec += data_vec;
    }
    
    let sum_array = sum_vec.to_array();
    let mut result: f32 = sum_array.iter().sum();
    
    for &value in &data[(chunks * 8)..] {
        result += value;
    }
    
    result
}
```

## 🧮 **Specialized Data Structure Operations**

### **SIMD Hash Table Probing**
```rust
use std::simd::{u64x4, mask64};

struct SimdHashMap<K, V> {
    // Align capacity to SIMD width for efficient probing
    buckets: Vec<Option<(K, V)>>,
    hashes: Vec<u64>, // Store hashes separately for SIMD comparison
    capacity: usize,
}

impl<K: PartialEq, V> SimdHashMap<K, V> {
    fn find_slot_simd(&self, hash: u64) -> Option<usize> {
        let hash_vec = u64x4::splat(hash);
        let chunks = self.capacity / 4;
        
        for i in 0..chunks {
            let start = i * 4;
            let hashes_vec = u64x4::from_slice(&self.hashes[start..start + 4]);
            let matches = hashes_vec.simd_eq(hash_vec);
            let mask = matches.to_bitmask();
            
            if mask != 0 {
                // Found potential match(es)
                for j in 0..4 {
                    if (mask >> j) & 1 != 0 {
                        let idx = start + j;
                        if self.buckets[idx].is_some() {
                            return Some(idx);
                        }
                    }
                }
            }
        }
        
        // Fallback to scalar search for remainder
        for i in (chunks * 4)..self.capacity {
            if self.hashes[i] == hash && self.buckets[i].is_some() {
                return Some(i);
            }
        }
        
        None
    }
}
```

### **SIMD Sorting Networks**
```rust
use std::simd::{i32x8, Simd};

// Sort 8 elements using SIMD operations
fn simd_sort_8(data: &mut [i32; 8]) {
    let mut vec = i32x8::from_slice(data);
    
    // Bitonic sorting network for 8 elements
    vec = simd_compare_swap(vec, [1, 0, 3, 2, 5, 4, 7, 6]);
    vec = simd_compare_swap(vec, [2, 3, 0, 1, 6, 7, 4, 5]);
    vec = simd_compare_swap(vec, [0, 2, 1, 3, 4, 6, 5, 7]);
    vec = simd_compare_swap(vec, [4, 0, 5, 1, 6, 2, 7, 3]);
    vec = simd_compare_swap(vec, [2, 0, 3, 1, 6, 4, 7, 5]);
    vec = simd_compare_swap(vec, [1, 0, 3, 2, 5, 4, 7, 6]);
    
    vec.copy_to_slice(data);
}

fn simd_compare_swap(vec: i32x8, indices: [usize; 8]) -> i32x8 {
    let shuffled = shuffle_lanes(vec, indices);
    vec.simd_min(shuffled).simd_max(shuffled)
}

fn shuffle_lanes(vec: i32x8, indices: [usize; 8]) -> i32x8 {
    let array = vec.to_array();
    let shuffled = [
        array[indices[0]], array[indices[1]], array[indices[2]], array[indices[3]],
        array[indices[4]], array[indices[5]], array[indices[6]], array[indices[7]],
    ];
    i32x8::from_array(shuffled)
}
```

## 🎯 **Performance Optimization Techniques**

### **Memory Alignment**
```rust
use std::alloc::{alloc, dealloc, Layout};

struct AlignedVec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
    layout: Layout,
}

impl<T> AlignedVec<T> {
    fn new_aligned(capacity: usize, alignment: usize) -> Self {
        let layout = Layout::array::<T>(capacity)
            .unwrap()
            .align_to(alignment)
            .unwrap();
        
        let ptr = unsafe { alloc(layout) as *mut T };
        
        Self {
            ptr,
            len: 0,
            capacity,
            layout,
        }
    }
    
    fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
    
    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<T> Drop for AlignedVec<T> {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr as *mut u8, self.layout);
        }
    }
}

// Usage for SIMD operations
let mut aligned_data = AlignedVec::<f32>::new_aligned(1024, 32); // 32-byte aligned for AVX
```

### **Loop Unrolling and Prefetching**
```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn simd_transform_with_prefetch(input: &[f32], output: &mut [f32], factor: f32) {
    const PREFETCH_DISTANCE: isize = 256; // Prefetch 256 bytes ahead
    
    let factor_vec = f32x8::splat(factor);
    let chunks = input.len() / 8;
    
    for i in 0..chunks {
        let start = i * 8;
        
        // Prefetch future data
        if start + PREFETCH_DISTANCE as usize < input.len() {
            unsafe {
                _mm_prefetch(
                    input.as_ptr().add(start + PREFETCH_DISTANCE as usize) as *const i8,
                    _MM_HINT_T0,
                );
            }
        }
        
        // Process current chunk
        let input_vec = f32x8::from_slice(&input[start..start + 8]);
        let result = input_vec * factor_vec;
        result.copy_to_slice(&mut output[start..start + 8]);
    }
    
    // Handle remainder
    for i in (chunks * 8)..input.len() {
        output[i] = input[i] * factor;
    }
}
```

### **Runtime SIMD Feature Detection**
```rust
use std::sync::Once;

static INIT: Once = Once::new();
static mut HAS_AVX2: bool = false;
static mut HAS_SSE41: bool = false;

fn init_simd_features() {
    INIT.call_once(|| {
        unsafe {
            HAS_AVX2 = is_x86_feature_detected!("avx2");
            HAS_SSE41 = is_x86_feature_detected!("sse4.1");
        }
    });
}

fn optimized_sum(data: &[i32]) -> i32 {
    init_simd_features();
    
    unsafe {
        if HAS_AVX2 {
            avx2_sum(data)
        } else if HAS_SSE41 {
            sse41_sum(data)
        } else {
            scalar_sum(data)
        }
    }
}

#[target_feature(enable = "avx2")]
unsafe fn avx2_sum(data: &[i32]) -> i32 {
    // AVX2 implementation (8 i32 at once)
    // ... implementation
    0
}

#[target_feature(enable = "sse4.1")]
unsafe fn sse41_sum(data: &[i32]) -> i32 {
    // SSE4.1 implementation (4 i32 at once)
    // ... implementation
    0
}

fn scalar_sum(data: &[i32]) -> i32 {
    data.iter().sum()
}
```

## 🧪 **Testing and Benchmarking**

### **SIMD Correctness Testing**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    
    #[test]
    fn test_simd_vs_scalar() {
        let mut rng = rand::thread_rng();
        let data: Vec<f32> = (0..1000).map(|_| rng.gen_range(-100.0..100.0)).collect();
        
        // Compare SIMD and scalar implementations
        let simd_result = simd_sum_f32(&data);
        let scalar_result: f32 = data.iter().sum();
        
        // Use epsilon comparison for floating point
        assert!((simd_result - scalar_result).abs() < 1e-5);
    }
    
    #[test]
    fn test_simd_edge_cases() {
        // Test empty slice
        assert_eq!(simd_sum_f32(&[]), 0.0);
        
        // Test single element
        assert_eq!(simd_sum_f32(&[42.0]), 42.0);
        
        // Test non-multiple of SIMD width
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0]; // Length 5, not multiple of 8
        let expected: f32 = data.iter().sum();
        assert_eq!(simd_sum_f32(&data), expected);
    }
}
```

### **SIMD Performance Benchmarks**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_simd_operations(c: &mut Criterion) {
    let sizes = [100, 1000, 10000, 100000];
    let data: Vec<f32> = (0..100000).map(|i| i as f32).collect();
    
    let mut group = c.benchmark_group("simd_sum");
    
    for size in sizes.iter() {
        group.bench_with_input(
            BenchmarkId::new("scalar", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let sum: f32 = black_box(&data[..size]).iter().sum();
                    black_box(sum)
                });
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("simd", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let sum = simd_sum_f32(black_box(&data[..size]));
                    black_box(sum)
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_simd_operations);
criterion_main!(benches);
```

## 📝 **Best Practices**

### **When to Use SIMD**
✅ **Good candidates:**
- Mathematical operations on arrays
- Image/signal processing
- Cryptographic operations
- String searching and pattern matching
- Statistical computations

❌ **Poor candidates:**
- Irregular memory access patterns
- Complex branching logic
- Small data sets (overhead cost)
- Operations with data dependencies

### **Performance Considerations**
```rust
// Good: Data-parallel operations
fn good_simd_candidate(a: &[f32], b: &[f32]) -> Vec<f32> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x * y + 1.0).collect()
}

// Bad: Complex branching
fn bad_simd_candidate(data: &[i32]) -> Vec<i32> {
    data.iter().map(|&x| {
        if x > 0 {
            if x % 2 == 0 { x * 2 } else { x * 3 }
        } else {
            -x
        }
    }).collect()
}

// Bad: Irregular memory access
fn bad_memory_pattern(data: &[f32], indices: &[usize]) -> Vec<f32> {
    indices.iter().map(|&i| data[i]).collect() // Gather operation
}
```

### **Portability Guidelines**
```rust
// Use feature detection for runtime dispatch
fn portable_simd_function(data: &[f32]) -> f32 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { avx2_implementation(data) };
        }
        if is_x86_feature_detected!("sse2") {
            return unsafe { sse2_implementation(data) };
        }
    }
    
    #[cfg(target_arch = "aarch64")]
    {
        if std::arch::is_aarch64_feature_detected!("neon") {
            return unsafe { neon_implementation(data) };
        }
    }
    
    // Fallback to scalar implementation
    scalar_implementation(data)
}

// Or use std::simd for portable code
fn portable_with_std_simd(data: &[f32]) -> f32 {
    use std::simd::f32x8;
    // This will use the best available SIMD on the target platform
    // ... implementation using f32x8
    0.0
}
```

---

*Tags: #simd #vectorization #performance #optimization #parallel-processing #cpu-architecture #high-performance-computing*

*Links: [[zettel-index]] | [[Cache-Friendly Data Structures]] | [[Performance Analysis Patterns]] | [[Memory Layout Optimization]] | [[CPU Architecture Patterns]] | [[High-Performance Computing]] | [[Vector Processing]]* 