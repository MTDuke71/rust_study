// Step 6: Performance Optimization
// Tutorial Day 6 - Aligns with Mission 6 Performance Optimization & REQ-6
//
// Learning Objectives:
// - Learn grid performance benchmarking techniques
// - Understand memory layout optimization strategies  
// - Practice cache-friendly access patterns
// - Measure and optimize grid operations
//
// This tutorial step builds toward Mission6 REQ-6:
// "The system shall provide optimized performance for large grids with efficient
//  memory usage and cache-friendly access patterns for common operations."

use mission6_tut::tutorial_helpers::{print_section, print_step_complete, TutorialGrid, TutorialCoord};
use std::time::{Instant, Duration};

fn main() {
    println!("=== Mission 6 Tutorial - Step 6: Performance Optimization ===");
    println!("Day 6 Focus: Benchmarking, memory optimization, and cache-friendly patterns\n");

    // Section 1: Understanding Grid Performance
    print_section("1. Understanding Grid Performance");
    
    println!("Grid performance considerations:");
    println!("• Memory layout: Row-major storage for cache efficiency");
    println!("• Access patterns: Sequential access vs random access");
    println!("• Memory locality: Minimize cache misses");
    println!("• Data structure overhead: Vec<T> vs Vec<Vec<T>>");
    println!("• Algorithm complexity: O(1) indexing, O(n) iteration");
    
    // Section 2: Basic Performance Measurements
    print_section("2. Basic Performance Measurements");
    
    println!("Testing different grid sizes and operations:");
    
    let sizes = vec![10, 100, 500, 1000];
    
    for size in sizes {
        println!("\n📊 Grid size: {}x{} ({} cells)", size, size, size * size);
        
        // Grid creation time
        let start = Instant::now();
        let grid = TutorialGrid::new(size, size, 0i32);
        let creation_time = start.elapsed();
        
        // Sequential read time (row-major)
        let start = Instant::now();
        let mut sum = 0i64;
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                if let Some(value) = grid.get(TutorialCoord::new(x, y)) {
                    sum += *value as i64;
                }
            }
        }
        let seq_read_time = start.elapsed();
        
        // Random access time
        let start = Instant::now();
        let mut random_sum = 0i64;
        for i in 0..1000.min(size * size) { // Sample up to 1000 accesses
            let x = (i * 17) % size; // Pseudo-random pattern
            let y = (i * 31) % size;
            if let Some(value) = grid.get(TutorialCoord::new(x, y)) {
                random_sum += *value as i64;
            }
        }
        let random_access_time = start.elapsed();
        
        println!("  Creation: {:?}", creation_time);
        println!("  Sequential read: {:?} (sum: {})", seq_read_time, sum);
        println!("  Random access (1k samples): {:?} (sum: {})", random_access_time, random_sum);
        
        // Memory estimation
        let memory_bytes = size * size * std::mem::size_of::<i32>();
        println!("  Estimated memory: {} bytes ({:.2} MB)", 
                memory_bytes, memory_bytes as f64 / (1024.0 * 1024.0));
    }
    
    // Section 3: Cache-Friendly vs Cache-Unfriendly Patterns
    print_section("3. Cache-Friendly vs Cache-Unfriendly Patterns");
    
    let test_size = 500;
    let test_grid = TutorialGrid::new(test_size, test_size, 1i32);
    
    println!("Testing access patterns on {}x{} grid:\n", test_size, test_size);
    
    // Pattern 1: Row-major traversal (cache-friendly)
    let start = Instant::now();
    let mut sum1 = 0i64;
    for y in 0..test_grid.height() {
        for x in 0..test_grid.width() {
            if let Some(value) = test_grid.get(TutorialCoord::new(x, y)) {
                sum1 += *value as i64;
            }
        }
    }
    let row_major_time = start.elapsed();
    
    // Pattern 2: Column-major traversal (less cache-friendly)
    let start = Instant::now();
    let mut sum2 = 0i64;
    for x in 0..test_grid.width() {
        for y in 0..test_grid.height() {
            if let Some(value) = test_grid.get(TutorialCoord::new(x, y)) {
                sum2 += *value as i64;
            }
        }
    }
    let col_major_time = start.elapsed();
    
    // Pattern 3: Diagonal traversal (worst for cache)
    let start = Instant::now();
    let mut sum3 = 0i64;
    let min_dim = test_grid.width().min(test_grid.height());
    for offset in 0..min_dim {
        for i in 0..(min_dim - offset) {
            if let Some(value) = test_grid.get(TutorialCoord::new(i + offset, i)) {
                sum3 += *value as i64;
            }
            if offset > 0 {
                if let Some(value) = test_grid.get(TutorialCoord::new(i, i + offset)) {
                    sum3 += *value as i64;
                }
            }
        }
    }
    let diagonal_time = start.elapsed();
    
    println!("Row-major (cache-friendly): {:?} (sum: {})", row_major_time, sum1);
    println!("Column-major (less friendly): {:?} (sum: {})", col_major_time, sum2);
    println!("Diagonal (cache-unfriendly): {:?} (sum: {})", diagonal_time, sum3);
    
    let row_vs_col_ratio = col_major_time.as_nanos() as f64 / row_major_time.as_nanos() as f64;
    let row_vs_diag_ratio = diagonal_time.as_nanos() as f64 / row_major_time.as_nanos() as f64;
    
    println!("\nPerformance ratios:");
    println!("Column-major vs row-major: {:.2}x slower", row_vs_col_ratio);
    println!("Diagonal vs row-major: {:.2}x slower", row_vs_diag_ratio);
    
    // Section 4: Memory Layout Optimization
    print_section("4. Memory Layout Optimization");
    
    println!("Comparing different data layouts:");
    
    // Test Vec<Vec<T>> vs Vec<T> (flattened)
    let grid_size = 100;
    
    // Vec<Vec<T>> - not optimal
    let start = Instant::now();
    let mut vec_of_vecs: Vec<Vec<i32>> = Vec::new();
    for _y in 0..grid_size {
        vec_of_vecs.push(vec![0; grid_size]);
    }
    let vec_vec_creation = start.elapsed();
    
    // Flatten Vec<T> - optimal
    let start = Instant::now();
    let flattened: Vec<i32> = vec![0; grid_size * grid_size];
    let flat_creation = start.elapsed();
    
    println!("Creation times:");
    println!("  Vec<Vec<T>>: {:?}", vec_vec_creation);
    println!("  Vec<T> (flat): {:?}", flat_creation);
    
    // Access time comparison
    let start = Instant::now();
    let mut sum_vec_vec = 0i64;
    for y in 0..grid_size {
        for x in 0..grid_size {
            sum_vec_vec += vec_of_vecs[y][x] as i64;
        }
    }
    let vec_vec_access = start.elapsed();
    
    let start = Instant::now();
    let mut sum_flat = 0i64;
    for y in 0..grid_size {
        for x in 0..grid_size {
            sum_flat += flattened[y * grid_size + x] as i64;
        }
    }
    let flat_access = start.elapsed();
    
    println!("\nAccess times:");
    println!("  Vec<Vec<T>>: {:?} (sum: {})", vec_vec_access, sum_vec_vec);
    println!("  Vec<T> (flat): {:?} (sum: {})", flat_access, sum_flat);
    
    let access_ratio = vec_vec_access.as_nanos() as f64 / flat_access.as_nanos() as f64;
    println!("  Flattened is {:.2}x faster for access", access_ratio);
    
    // Section 5: Bulk Operations Optimization
    print_section("5. Bulk Operations Optimization");
    
    let bulk_size = 200;
    let mut bulk_grid = TutorialGrid::new(bulk_size, bulk_size, 1i32);
    
    // Bulk fill operation
    let start = Instant::now();
    for y in 50..150 {
        for x in 50..150 {
            bulk_grid.set(TutorialCoord::new(x, y), 42);
        }
    }
    let bulk_fill_time = start.elapsed();
    
    println!("Bulk operations on {}x{} grid:", bulk_size, bulk_size);
    println!("  Filled 100x100 region in {:?}", bulk_fill_time);
    
    // Count non-default values
    let start = Instant::now();
    let mut non_default_count = 0;
    for y in 0..bulk_grid.height() {
        for x in 0..bulk_grid.width() {
            if let Some(&value) = bulk_grid.get(TutorialCoord::new(x, y)) {
                if value != 1 {
                    non_default_count += 1;
                }
            }
        }
    }
    let count_time = start.elapsed();
    
    println!("  Counted {} non-default values in {:?}", non_default_count, count_time);
    
    // Section 6: Iterator Performance
    print_section("6. Iterator Performance");
    
    println!("Comparing iteration methods:");
    
    let iter_grid = TutorialGrid::new(300, 300, 5i32);
    
    // Method 1: Coordinate-based iteration
    let start = Instant::now();
    let mut coord_sum = 0i64;
    for y in 0..iter_grid.height() {
        for x in 0..iter_grid.width() {
            if let Some(value) = iter_grid.get(TutorialCoord::new(x, y)) {
                coord_sum += *value as i64;
            }
        }
    }
    let coord_iter_time = start.elapsed();
    
    println!("Coordinate iteration: {:?} (sum: {})", coord_iter_time, coord_sum);
    
    // Section 7: Memory Usage Profiling
    print_section("7. Memory Usage Profiling");
    
    println!("Memory usage analysis:");
    
    let memory_sizes = vec![50, 100, 200, 500];
    
    for size in memory_sizes {
        let _grid = TutorialGrid::new(size, size, 0u8);
        let element_size = std::mem::size_of::<u8>();
        let total_elements = size * size;
        let data_size = total_elements * element_size;
        let overhead = std::mem::size_of::<TutorialGrid<u8>>() - std::mem::size_of::<Vec<u8>>();
        
        println!("\n{}x{} grid (u8 elements):", size, size);
        println!("  Elements: {}", total_elements);
        println!("  Data size: {} bytes", data_size);
        println!("  Struct overhead: {} bytes", overhead);
        println!("  Total estimated: {} bytes ({:.2} KB)", 
                data_size + overhead, (data_size + overhead) as f64 / 1024.0);
        
        // Memory efficiency
        let efficiency = data_size as f64 / (data_size + overhead) as f64 * 100.0;
        println!("  Memory efficiency: {:.1}%", efficiency);
    }
    
    // Section 8: Performance Optimization Tips
    print_section("8. Performance Optimization Tips");
    
    println!("Grid performance optimization strategies:");
    println!("\n🚀 Memory Layout:");
    println!("  ✓ Use Vec<T> instead of Vec<Vec<T>> for better cache locality");
    println!("  ✓ Choose appropriate data types (u8 vs u32 vs u64)");
    println!("  ✓ Consider memory alignment for SIMD operations");
    
    println!("\n🔄 Access Patterns:");
    println!("  ✓ Prefer row-major traversal (y outer loop, x inner loop)");
    println!("  ✓ Process data in blocks/tiles for better cache usage");
    println!("  ✓ Avoid random access patterns when possible");
    
    println!("\n⚡ Algorithm Choice:");
    println!("  ✓ Use appropriate algorithms for problem size");
    println!("  ✓ Consider spatial data structures for sparse grids");
    println!("  ✓ Batch operations to reduce function call overhead");
    
    println!("\n📊 Measurement:");
    println!("  ✓ Always measure performance with realistic data");
    println!("  ✓ Test on target hardware configurations");
    println!("  ✓ Profile memory allocation patterns");
    
    // Section 9: Benchmarking Framework
    print_section("9. Benchmarking Framework");
    
    println!("Simple benchmarking framework for grid operations:");
    
    fn benchmark_operation<F>(name: &str, iterations: usize, operation: F) 
    where F: Fn() -> Duration {
        let mut total_time = Duration::from_nanos(0);
        let mut times = Vec::new();
        
        for _ in 0..iterations {
            let time = operation();
            total_time += time;
            times.push(time);
        }
        
        times.sort();
        let avg_time = total_time / iterations as u32;
        let median_time = times[times.len() / 2];
        let min_time = times[0];
        let max_time = times[times.len() - 1];
        
        println!("\n{} ({} iterations):", name, iterations);
        println!("  Average: {:?}", avg_time);
        println!("  Median:  {:?}", median_time);
        println!("  Min:     {:?}", min_time);
        println!("  Max:     {:?}", max_time);
    }
    
    // Benchmark grid creation
    benchmark_operation("Grid Creation (100x100)", 100, || {
        let start = Instant::now();
        let _grid = TutorialGrid::new(100, 100, 0i32);
        start.elapsed()
    });
    
    // Benchmark grid traversal
    let bench_grid = TutorialGrid::new(100, 100, 1i32);
    benchmark_operation("Grid Traversal (100x100)", 50, || {
        let start = Instant::now();
        let mut sum = 0i64;
        for y in 0..bench_grid.height() {
            for x in 0..bench_grid.width() {
                if let Some(value) = bench_grid.get(TutorialCoord::new(x, y)) {
                    sum += *value as i64;
                }
            }
        }
        let _ = sum; // Prevent optimization
        start.elapsed()
    });
    
    print_step_complete("Step 6: Performance Optimization");
    
    // Next Steps Preview
    println!("\n🔄 Next: Step 7 - Documentation & Integration");
    println!("   Learn comprehensive documentation, integration patterns, and best practices");
    println!("   Command: cargo run --example step7_documentation");
    
    // Key Takeaways
    println!("\n📝 Key Takeaways from Step 6:");
    println!("   ✓ Row-major access is significantly more cache-friendly");
    println!("   ✓ Flattened Vec<T> outperforms Vec<Vec<T>> for grid storage");
    println!("   ✓ Memory layout affects performance more than algorithmic complexity");
    println!("   ✓ Always benchmark with realistic data sizes and patterns");
    println!("   ✓ Choose data types based on memory usage vs performance trade-offs");
}

// Exercise for the Reader:
// 1. Implement a grid that uses different storage strategies (sparse vs dense)
// 2. Create a benchmarking suite for different pathfinding algorithms
// 3. Measure memory allocation patterns for different grid operations
// 4. Implement SIMD-optimized bulk operations for grid processing

// Design Questions to Consider:
// - How do different data types (u8, u16, u32, u64) affect performance?
// - When should you choose Vec<Vec<T>> over Vec<T> despite performance costs?
// - How do you optimize for both memory usage and access speed?
// - What profiling tools help identify performance bottlenecks in grid operations?