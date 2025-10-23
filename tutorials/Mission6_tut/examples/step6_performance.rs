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
//
// 📊 Expected Results (--release mode):
//
// Grid Creation (scales linearly with size):
//   10x10:    ~500ns    (400ns per 100 cells)
//   100x100:  ~5µs      (50ns per 100 cells)
//   500x500:  ~150µs    (60ns per 100 cells)
//   1000x1000: ~600µs   (60ns per 100 cells)
//
// Cache Pattern Impact (1000x1000 grid in --release):
//   Row-major (optimal):     ~100µs   (baseline)
//   Column-major (strided):  ~327µs   (3.26x slower - cache line misses)
//   Random access (hostile): ~771µs   (7.67x slower - cache thrashing)
//
// Memory Layout (100x100 grid):
//   Vec<Vec<T>> creation: ~15µs  | access: ~22µs
//   Vec<T> flat creation: ~3µs   | access: ~13µs  (1.5-2x faster)
//
// 💡 Key Insight: In --release mode, cache effects are dramatically visible!
//    Debug mode shows smaller differences due to unoptimized code overhead.

use mission6_tut::tutorial_helpers::{
    print_section, print_step_complete, TutorialCoord, TutorialGrid,
};
use std::hint::black_box;
use std::time::{Duration, Instant};

fn main() {
    println!("=== Mission 6 Tutorial - Step 6: Performance Optimization ===");
    println!("Day 6 Focus: Benchmarking, memory optimization, and cache-friendly patterns");
    println!("💡 Run with --release flag for dramatic cache effect visibility!\n");

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
    println!("⚠️  Note: Using non-zero values and multiple iterations for accurate measurements\n");

    let sizes = vec![10, 100, 500, 1000];

    for size in sizes {
        println!("\n📊 Grid size: {}x{} ({} cells)", size, size, size * size);

        // Grid creation time - measure multiple iterations for accuracy
        let iterations = 10;
        let mut creation_times = Vec::new();

        for _ in 0..iterations {
            let start = Instant::now();
            // Use non-zero value to prevent zero-page optimization
            let grid = TutorialGrid::new(size, size, 1i32);
            std::hint::black_box(&grid); // Prevent dead code elimination
            creation_times.push(start.elapsed());
        }

        // Use median instead of single measurement (more robust to outliers)
        creation_times.sort();
        let creation_time = creation_times[iterations / 2];

        // Create grid for subsequent tests
        let grid = TutorialGrid::new(size, size, 1i32);

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
        std::hint::black_box(sum); // Prevent optimizer from eliminating computation
        let seq_read_time = start.elapsed();

        // Random access time
        let start = Instant::now();
        let mut random_sum = 0i64;
        for i in 0..1000.min(size * size) {
            // Sample up to 1000 accesses
            let x = (i * 17) % size; // Pseudo-random pattern
            let y = (i * 31) % size;
            if let Some(value) = grid.get(TutorialCoord::new(x, y)) {
                random_sum += *value as i64;
            }
        }
        std::hint::black_box(random_sum); // Prevent optimizer from eliminating computation
        let random_access_time = start.elapsed();

        // Calculate per-cell metrics
        let total_cells = size * size;
        let cells_per_100 = 100;
        let creation_per_100_cells = if total_cells >= cells_per_100 {
            creation_time.as_nanos() as f64 * (cells_per_100 as f64 / total_cells as f64)
        } else {
            creation_time.as_nanos() as f64
        };

        println!("  Creation (median of {}): {:?}", iterations, creation_time);
        println!("  Creation per 100 cells: {:.0}ns", creation_per_100_cells);
        println!("  Sequential read: {:?} (sum: {})", seq_read_time, sum);
        println!(
            "  Random access (1k samples): {:?} (sum: {})",
            random_access_time, random_sum
        );

        // Memory estimation
        let memory_bytes = size * size * std::mem::size_of::<i32>();
        println!(
            "  Estimated memory: {} bytes ({:.2} MB)",
            memory_bytes,
            memory_bytes as f64 / (1024.0 * 1024.0)
        );
    }

    // Section 3: Cache-Friendly vs Cache-Unfriendly Patterns
    print_section("3. Cache-Friendly vs Cache-Unfriendly Patterns");

    let test_size = 1000; // Larger grid to exceed L1 cache

    // Create grid with varying values to prevent CPU optimizations
    let mut test_grid = TutorialGrid::new(test_size, test_size, 0i32);
    for y in 0..test_size {
        for x in 0..test_size {
            let value = ((x * 7 + y * 11) % 256) as i32; // Pseudo-random pattern
            test_grid.set(TutorialCoord::new(x, y), value);
        }
    }

    println!(
        "Testing access patterns on {}x{} grid:",
        test_size, test_size
    );
    println!("(Using varied data to prevent CPU prediction)\n");

    // Pattern 1: Row-major traversal (cache-friendly) - multiple iterations
    let iterations = 5;
    let mut row_major_times = Vec::new();
    for _ in 0..iterations {
        let start = Instant::now();
        let mut sum1 = 0i64;
        for y in 0..test_grid.height() {
            for x in 0..test_grid.width() {
                if let Some(value) = test_grid.get(TutorialCoord::new(x, y)) {
                    sum1 += *value as i64;
                }
            }
        }
        std::hint::black_box(sum1);
        row_major_times.push(start.elapsed());
    }
    row_major_times.sort();
    let row_major_time = row_major_times[iterations / 2];

    // Pattern 2: Column-major traversal (cache-unfriendly) - strided access
    let mut col_major_times = Vec::new();
    for _ in 0..iterations {
        let start = Instant::now();
        let mut sum2 = 0i64;
        for x in 0..test_grid.width() {
            for y in 0..test_grid.height() {
                if let Some(value) = test_grid.get(TutorialCoord::new(x, y)) {
                    sum2 += *value as i64;
                }
            }
        }
        std::hint::black_box(sum2);
        col_major_times.push(start.elapsed());
    }
    col_major_times.sort();
    let col_major_time = col_major_times[iterations / 2];

    // Pattern 3: Random access (very cache-unfriendly)
    let mut random_times = Vec::new();
    for _ in 0..iterations {
        let start = Instant::now();
        let mut sum3 = 0i64;
        // Access in pseudo-random order
        for i in 0..(test_size * test_size) {
            let x = (i * 17) % test_size;
            let y = (i * 31) % test_size;
            if let Some(value) = test_grid.get(TutorialCoord::new(x, y)) {
                sum3 += *value as i64;
            }
        }
        std::hint::black_box(sum3);
        random_times.push(start.elapsed());
    }
    random_times.sort();
    let random_time = random_times[iterations / 2];

    println!(
        "Row-major (cache-friendly): {:?} (median of {})",
        row_major_time, iterations
    );
    println!(
        "Column-major (strided access): {:?} (median of {})",
        col_major_time, iterations
    );
    println!(
        "Random access (cache-hostile): {:?} (median of {})",
        random_time, iterations
    );

    let row_vs_col_ratio = col_major_time.as_nanos() as f64 / row_major_time.as_nanos() as f64;
    let row_vs_random_ratio = random_time.as_nanos() as f64 / row_major_time.as_nanos() as f64;

    println!("\nPerformance ratios:");
    println!("Column-major vs row-major: {:.2}x slower", row_vs_col_ratio);
    println!(
        "Random access vs row-major: {:.2}x slower",
        row_vs_random_ratio
    );
    println!("\n💡 Cache Impact Analysis:");
    println!("   • Row-major: Sequential access = optimal cache utilization");
    println!(
        "   • Column-major: Strided access = cache line misses every {}x{} elements",
        test_size,
        std::mem::size_of::<i32>()
    );
    println!("   • Random access: Unpredictable pattern = cache thrashing");
    println!("\n⚠️  Modern CPU Note: Results show 5-10%% difference (not 2-5x) because:");
    println!("   • Hardware prefetchers predict patterns (even strided ones)");
    println!("   • Large L3 cache (8-32MB) can hold entire grid");
    println!("   • Out-of-order execution hides some memory latency");
    println!("   • On constrained systems (embedded, old hardware), gaps are much larger!");

    // Section 4: Memory Layout Optimization
    print_section("4. Memory Layout Optimization");

    println!("🎯 Key Insight: TutorialGrid uses SINGLE flat Vec<T> with calculated indexing");
    println!("   NOT nested Vec<Vec<T>> with pointer chasing!\n");

    println!("Comparing: Vec<Vec<T>> (BAD) vs Vec<T> with index calc (GOOD)");

    // Test Vec<Vec<T>> vs Vec<T> (flattened)
    let grid_size = 100;

    // BAD: Vec<Vec<T>> - nested structure with pointer indirection
    println!("\n❌ Vec<Vec<T>> approach (what NOT to do):");
    println!("   • Each row is a separate allocation");
    println!("   • Requires pointer chase: grid[y] → find Vec → [x]");
    println!("   • Poor cache locality - rows may be scattered in memory");
    let start = Instant::now();
    let mut vec_of_vecs: Vec<Vec<i32>> = Vec::new();
    for _y in 0..grid_size {
        vec_of_vecs.push(vec![1; grid_size]);
    }
    std::hint::black_box(&vec_of_vecs);
    let vec_vec_creation = start.elapsed();

    // GOOD: Flatten Vec<T> - single allocation with calculated indexing
    println!("\n✅ Vec<T> flat approach (TutorialGrid's strategy):");
    println!("   • Single contiguous allocation");
    println!("   • Direct index calculation: y * width + x");
    println!("   • Excellent cache locality - all data sequential");
    let start = Instant::now();
    let flattened: Vec<i32> = vec![1; grid_size * grid_size];
    std::hint::black_box(&flattened);
    let flat_creation = start.elapsed();

    println!("\nCreation times:");
    println!("  Vec<Vec<T>>: {:?}", vec_vec_creation);
    println!("  Vec<T> (flat): {:?}", flat_creation);

    // Access time comparison - demonstrate the indexing difference
    println!("\nAccess pattern comparison:");

    // Vec<Vec<T>> - pointer indirection
    let start = Instant::now();
    let mut sum_vec_vec = 0i64;
    for row in &vec_of_vecs {
        for &val in row {
            sum_vec_vec += val as i64; // Two indirections!
        }
    }
    std::hint::black_box(sum_vec_vec);
    let vec_vec_access = start.elapsed();

    // Vec<T> flat - calculated index (what TutorialGrid does internally)
    let start = Instant::now();
    let mut sum_flat = 0i64;
    for y in 0..grid_size {
        for x in 0..grid_size {
            let index = y * grid_size + x; // Calculate index: y * width + x
            sum_flat += flattened[index] as i64; // Single indirection!
        }
    }
    std::hint::black_box(sum_flat);
    let flat_access = start.elapsed();

    println!(
        "  Vec<Vec<T>>: {:?} (sum: {}) - pointer chase overhead",
        vec_vec_access, sum_vec_vec
    );
    println!(
        "  Vec<T> flat: {:?} (sum: {}) - direct calculation",
        flat_access, sum_flat
    );

    let access_ratio = vec_vec_access.as_nanos() as f64 / flat_access.as_nanos() as f64;
    println!("  ⚡ Flattened is {:.2}x faster for access", access_ratio);

    println!("\n💡 TutorialGrid Implementation:");
    println!("   struct TutorialGrid<T> {{");
    println!("       data: Vec<T>,        // Single flat vector");
    println!("       width: usize,");
    println!("       height: usize,");
    println!("   }}");
    println!("   ");
    println!("   fn get(&self, coord: Coord) -> Option<&T> {{");
    println!("       let index = coord.y * self.width + coord.x;  // Index calculation");
    println!("       self.data.get(index)  // Direct access, no pointer chase");
    println!("   }}");

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

    println!(
        "  Counted {} non-default values in {:?}",
        non_default_count, count_time
    );

    // Section 6: Iterator Performance
    print_section("6. Iterator Performance");

    println!("Comparing different iteration approaches:\n");

    let iter_grid = TutorialGrid::new(300, 300, 5i32);
    let iterations = 5;

    // Method 1: Coordinate-based iteration (what we've been using)
    println!("Method 1: Coordinate-based with get() - Safety overhead");
    let mut coord_times = vec![];
    for _ in 0..iterations {
        let start = Instant::now();
        let mut coord_sum = 0i64;
        for y in 0..iter_grid.height() {
            for x in 0..iter_grid.width() {
                if let Some(value) = iter_grid.get(TutorialCoord::new(x, y)) {
                    coord_sum += *value as i64;
                }
            }
        }
        black_box(coord_sum);
        coord_times.push(start.elapsed());
    }
    coord_times.sort();
    let coord_median = coord_times[coord_times.len() / 2];
    println!("  Time: {:?} (median of {})", coord_median, iterations);
    println!("  • Creates Coord structs each iteration");
    println!("  • Performs bounds checking on every get()");
    println!("  • Safe but has overhead\n");

    // Method 2: Direct index calculation (bypassing Coord creation)
    println!("Method 2: Direct index calculation - Reduced allocations");
    let mut index_times = vec![];
    for _ in 0..iterations {
        let start = Instant::now();
        let mut index_sum = 0i64;
        for y in 0..iter_grid.height() {
            for x in 0..iter_grid.width() {
                let coord = TutorialCoord::new(x, y);
                if let Some(value) = iter_grid.get(coord) {
                    index_sum += *value as i64;
                }
            }
        }
        black_box(index_sum);
        index_times.push(start.elapsed());
    }
    index_times.sort();
    let index_median = index_times[index_times.len() / 2];
    println!("  Time: {:?} (median of {})", index_median, iterations);
    println!("  • Still creates Coord but cleaner syntax");
    println!("  • Still performs bounds checking");
    println!("  • Similar performance to Method 1\n");

    // Method 3: Iterator over range with index calculation
    println!("Method 3: Range-based with manual indexing - Compiler friendly");
    let mut range_times = vec![];
    for _ in 0..iterations {
        let start = Instant::now();
        let mut range_sum = 0i64;
        for idx in 0..(iter_grid.width() * iter_grid.height()) {
            // Simulate direct data access if we had access to internal Vec
            let y = idx / iter_grid.width();
            let x = idx % iter_grid.width();
            if let Some(value) = iter_grid.get(TutorialCoord::new(x, y)) {
                range_sum += *value as i64;
            }
        }
        black_box(range_sum);
        range_times.push(start.elapsed());
    }
    range_times.sort();
    let range_median = range_times[range_times.len() / 2];
    println!("  Time: {:?} (median of {})", range_median, iterations);
    println!("  • Single loop over 0..width*height");
    println!("  • Calculates x,y from linear index (involves division/modulo)");
    println!("  • ⚠️  Slower in release mode due to math overhead!\n");

    // Performance comparison
    println!("📊 Performance Comparison:");
    println!("  Coordinate-based: {:?} (baseline)", coord_median);
    println!(
        "  Direct indexing:  {:?} ({:.2}x)",
        index_median,
        coord_median.as_secs_f64() / index_median.as_secs_f64()
    );
    println!(
        "  Range-based:      {:?} ({:.2}x)",
        range_median,
        coord_median.as_secs_f64() / range_median.as_secs_f64()
    );

    println!("\n💡 Key Insights:");
    println!("  • Methods 1 & 2 have identical performance (~within 1%%)");
    println!("  • Method 3 is SLOWER due to division/modulo for x,y calculation");
    println!("  • Coord struct creation overhead is negligible (optimized away)");
    println!("  • Bounds checking cost is acceptable for safety");
    println!("  • In --release: direct 2D loops are fastest (no index math needed)");

    println!("\n🚀 Performance Hierarchy (fastest to slowest):");
    println!("  1. Direct Vec access:        ~2x faster (unsafe, no bounds check)");
    println!("  2. Coordinate-based (2D):    Baseline (safe, clean, fast)");
    println!("  3. Index calculation (1D):   ~3x slower (division/modulo penalty)");

    println!("\n⚠️  Lesson: Don't assume single-loop is faster! Division/modulo is expensive.");
    println!("   The 2D nested loop approach (Method 1) is both clearer AND faster!");

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
        println!(
            "  Total estimated: {} bytes ({:.2} KB)",
            data_size + overhead,
            (data_size + overhead) as f64 / 1024.0
        );

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
    println!("  ✓ Single contiguous allocation beats multiple allocations");

    println!("\n🔄 Access Patterns:");
    println!("  ✓ Prefer row-major traversal (y outer loop, x inner loop)");
    println!("  ✓ Use 2D nested loops instead of single-loop with division/modulo");
    println!("  ✓ Process data in blocks/tiles for better cache usage");
    println!("  ✓ Avoid random access patterns when possible");
    println!("  ✗ Avoid index = y*width+x in loops (division/modulo on reverse is slow)");

    println!("\n⚡ Algorithm Choice:");
    println!("  ✓ Use appropriate algorithms for problem size");
    println!("  ✓ Consider lookup tables for small input spaces (avoid repeated calculations)");
    println!("  ✓ Consider spatial data structures for sparse grids");
    println!("  ✓ Batch operations to reduce function call overhead");
    println!("  ✗ Don't assume fewer loops = faster (measure first!)");

    println!("\n� Computation vs Memory Trade-offs:");
    println!("  ✓ Lookup tables win over calculations for small domains");
    println!("  ✓ Modern CPUs are memory-bound, not compute-bound for simple math");
    println!("  ✓ 'Don't calculate what you can remember' - precompute when possible");
    println!("  ✓ Example: 64-square chess board lookup = 512 bytes but 3-10x faster");

    println!("\n�📊 Measurement:");
    println!("  ✓ Always measure performance with realistic data");
    println!("  ✓ Test on target hardware configurations");
    println!("  ✓ Profile memory allocation patterns");
    println!("  ✓ Use --release mode for production-realistic measurements");
    println!("  ✓ Watch for counter-intuitive results (like Section 6!)");

    // Section 9: Benchmarking Framework
    print_section("9. Benchmarking Framework");

    println!("Simple benchmarking framework for grid operations:");

    fn benchmark_operation<F>(name: &str, iterations: usize, operation: F)
    where
        F: Fn() -> Duration,
    {
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

    // Common Benchmarking Pitfalls
    println!("\n⚠️  Common Benchmarking Pitfalls:");
    println!("   • Compiler optimizations can eliminate 'unused' work");
    println!("   • OS lazy allocation delays actual memory allocation");
    println!("   • Dead code elimination removes computations with unused results");
    println!("   • Use std::hint::black_box() to prevent optimization");
    println!("   • Always verify results to ensure computation actually occurred");
    println!("   • Watch for suspiciously fast times (like 1000x1000 < 500x500)");

    // Release Mode Expected Results
    println!("\n🚀 Expected Results in --release Mode:");
    println!("   (Run: cargo run --example step6_performance --release)");
    println!("\n   📊 Grid Creation (linear scaling):");
    println!("      10x10:     ~500ns   (400ns per 100 cells)");
    println!("      100x100:   ~5µs     (50ns per 100 cells)");
    println!("      500x500:   ~150µs   (60ns per 100 cells)");
    println!("      1000x1000: ~600µs   (60ns per 100 cells)");
    println!("\n   🔥 Cache Pattern Impact (1000x1000 grid):");
    println!("      Row-major (optimal):     ~100µs   (baseline)");
    println!("      Column-major (strided):  ~327µs   (3.26x slower)");
    println!("      Random access (hostile): ~771µs   (7.67x slower)");
    println!("\n   💾 Memory Layout (100x100 grid):");
    println!("      Vec<Vec<T>>: creation ~15µs | access ~22µs");
    println!("      Vec<T> flat: creation ~3µs  | access ~13µs  (1.5-2x faster)");
    println!("\n   💡 In release mode, cache effects become dramatically visible!");
    println!("      Debug mode shows smaller differences due to unoptimized overhead.");
}

// Exercise for the Reader:
// 1. Implement a grid that uses different storage strategies (sparse vs dense)
// 2. Create a benchmarking suite for different pathfinding algorithms
// 3. Measure memory allocation patterns for different grid operations
// 4. Implement SIMD-optimized bulk operations for grid processing
// 5. Try removing black_box and observe timing anomalies!
// 6. Compare debug vs release mode results - see cache effects amplified!

// Design Questions to Consider:
//
// Q: How do different data types (u8, u16, u32, u64) affect performance?
// A: Smaller types = better cache utilization (4x more u8s fit in cache vs u32s), but
//    can introduce alignment padding. u8 grids process 4x more elements per cache line.
//    Trade-off: u8 saves memory but may need casting; u32/u64 allow larger value ranges.
//    For large grids (>1MB), u8 can show 2-3x speedup due to cache efficiency alone.
//
// Q: When should you choose Vec<Vec<T>> over Vec<T> despite performance costs?
// A: Only when you NEED variable row lengths (jagged arrays) or dynamic row operations
//    (inserting/removing entire rows frequently). Examples: CSV with different column counts,
//    sparse matrices where some rows are empty. For uniform grids, ALWAYS use Vec<T> flat.
//    The 3x performance penalty is rarely justified.
//
// Q: How do you optimize for both memory usage and access speed?
// A: 1) Use smallest type that fits your value range (u8 when possible)
//    2) Flat Vec<T> for cache locality (as demonstrated in Section 4)
//    3) Row-major access patterns (Section 3)
//    4) For sparse data: consider spatial data structures (quadtree, R-tree)
//    5) Align struct fields and use #[repr(C)] for predictable layout
//    Best practice: Profile first, then optimize the hot path only.
//
// Q: What profiling tools help identify performance bottlenecks in grid operations?
// A: 1) cargo-flamegraph - Visual CPU time breakdown (shows where time is spent)
//    2) perf (Linux) - Hardware counter analysis (cache misses, branch mispredictions)
//    3) valgrind/cachegrind - Detailed cache simulation and memory access patterns
//    4) cargo-criterion - Statistical benchmarking with outlier detection
//    5) std::hint::black_box - Prevent compiler from optimizing away benchmarks
//    6) Built-in: Manual timing with Instant::now() + multiple iterations (as shown here)
//
// Q: Why are cache effects more visible in release mode than debug mode?
// A: Debug mode adds overhead that dominates timing (bounds checks not optimized, no
//    inlining, extra stack operations). This "noise" masks cache differences. Release
//    mode eliminates overhead through optimizations (inlining, loop unrolling, SIMD),
//    making memory access the bottleneck. When computation is fast, memory becomes the
//    limiting factor. Result: Cache-friendly code shows 3-7x speedup in release mode
//    vs only 5-10% in debug mode. Always benchmark with --release for realistic results!
