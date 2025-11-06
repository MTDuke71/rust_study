// Chapter 13.4 - Comparing Performance: Loops vs Iterators
// 
// This example demonstrates the performance characteristics of loops vs iterators
// in Rust, showing that iterators are zero-cost abstractions that often compile
// to the same or better assembly code than hand-written loops.

use std::time::Instant;

/// Audio buffer processing example from the Rust Book
/// This demonstrates a real-world scenario where performance matters
#[derive(Debug, Clone)]
struct AudioSample {
    frequency: f64,
    amplitude: f64,
}

impl AudioSample {
    fn new(frequency: f64, amplitude: f64) -> Self {
        Self { frequency, amplitude }
    }
    
    fn apply_gain(&mut self, gain: f64) {
        self.amplitude *= gain;
    }
    
    #[allow(dead_code)]
    fn to_i16(&self) -> i16 {
        (self.amplitude * 32767.0) as i16
    }
}

/// Performance comparison: Loop-based audio processing
fn process_audio_with_loop(samples: &mut [AudioSample], gain: f64) {
    for i in 0..samples.len() {
        samples[i].apply_gain(gain);
    }
}

/// Performance comparison: Iterator-based audio processing
fn process_audio_with_iterator(samples: &mut [AudioSample], gain: f64) {
    samples.iter_mut().for_each(|sample| sample.apply_gain(gain));
}

/// Functional style with chaining
#[allow(dead_code)]
fn process_audio_functional(samples: &[AudioSample], gain: f64) -> Vec<i16> {
    samples
        .iter()
        .map(|sample| AudioSample::new(sample.frequency, sample.amplitude * gain))
        .map(|sample| sample.to_i16())
        .collect()
}

/// Search performance comparison
fn search_with_loop(data: &[i32], target: i32) -> Option<usize> {
    (0..data.len()).find(|&i| data[i] == target)
}

fn search_with_iterator(data: &[i32], target: i32) -> Option<usize> {
    data.iter().position(|&x| x == target)
}

/// Complex data transformation comparisons
fn transform_with_loop(data: &[i32]) -> Vec<i64> {
    let mut result = Vec::new();
    for &item in data {
        if item % 2 == 0 {
            result.push((item as i64) * (item as i64));
        }
    }
    result
}

fn transform_with_iterator(data: &[i32]) -> Vec<i64> {
    data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| (x as i64) * (x as i64))
        .collect()
}

fn transform_with_iterator_collect(data: &[i32]) -> Vec<i64> {
    data.iter()
        .copied()
        .filter(|&x| x % 2 == 0)
        .map(|x| (x as i64) * (x as i64))
        .collect()
}

/// Numerical computation example
fn sum_of_squares_loop(data: &[f64]) -> f64 {
    let mut sum = 0.0;
    for &value in data {
        sum += value * value;
    }
    sum
}

fn sum_of_squares_iterator(data: &[f64]) -> f64 {
    data.iter().map(|&x| x * x).sum()
}

/// Chain multiple operations
fn complex_pipeline_loop(data: &[i32]) -> i64 {
    let mut sum: i64 = 0;
    for &item in data {
        if item > 10 {
            let doubled = (item as i64) * 2;
            if doubled % 3 == 0 {
                sum += doubled;
            }
        }
    }
    sum
}

fn complex_pipeline_iterator(data: &[i32]) -> i64 {
    data.iter()
        .copied()
        .filter(|&x| x > 10)
        .map(|x| (x as i64) * 2)
        .filter(|&x| x % 3 == 0)
        .sum()
}

/// Memory-efficient iterator processing
fn process_large_dataset_loop(data: &[i32], threshold: i32) -> (usize, i32) {
    let mut count = 0;
    let mut max = i32::MIN;
    
    for &value in data {
        if value > threshold {
            count += 1;
            if value > max {
                max = value;
            }
        }
    }
    
    (count, max)
}

fn process_large_dataset_iterator_bad(data: &[i32], threshold: i32) -> (usize, i32) {
    // BAD EXAMPLE: This is NOT zero-cost - it allocates and does two passes!
    let filtered: Vec<_> = data.iter().copied().filter(|&x| x > threshold).collect();
    let count = filtered.len();
    let max = filtered.into_iter().max().unwrap_or(i32::MIN);
    (count, max)
}

fn process_large_dataset_iterator_proper(data: &[i32], threshold: i32) -> (usize, i32) {
    // GOOD EXAMPLE: This should be zero-cost - single pass, no allocation
    data.iter()
        .copied()
        .filter(|&x| x > threshold)
        .fold((0, i32::MIN), |(count, max), value| {
            (count + 1, max.max(value))
        })
}

fn process_large_dataset_iterator_optimized(data: &[i32], threshold: i32) -> (usize, i32) {
    data.iter()
        .copied()
        .filter(|&x| x > threshold)
        .fold((0, i32::MIN), |(count, max), value| {
            (count + 1, max.max(value))
        })
}

/// Demonstrate iterator adaptor performance
fn demonstrate_lazy_evaluation() {
    println!("=== Lazy Evaluation Demo ===");
    
    let data: Vec<i32> = (0..1_000_000).collect();
    
    // This creates an iterator but doesn't process anything yet
    let start = Instant::now();
    let _lazy_iter = data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .filter(|&x| x > 1000);
    let lazy_time = start.elapsed();
    
    println!("Creating lazy iterator: {:?}", lazy_time);
    
    // This actually consumes and processes the data
    let start = Instant::now();
    let result: Vec<_> = data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .filter(|&x| x > 1000)
        .take(100)  // Only take first 100 to avoid processing everything
        .collect();
    let processing_time = start.elapsed();
    
    println!("Processing first 100 results: {:?}", processing_time);
    println!("First few results: {:?}", &result[..5.min(result.len())]);
}

/// Benchmark different approaches
fn benchmark_operations() {
    const SIZE: usize = 1_000_000;
    let data: Vec<i32> = (0..SIZE as i32).collect();
    let mut audio_samples: Vec<AudioSample> = (0..SIZE)
        .map(|i| AudioSample::new(440.0 + i as f64, 0.5))
        .collect();
    let float_data: Vec<f64> = (0..SIZE).map(|i| i as f64 / 1000.0).collect();
    
    println!("=== Performance Benchmarks ===");
    println!("Dataset size: {} elements", SIZE);
    println!("NOTE: In release mode, compiler optimizations may make some operations appear");
    println!("      much faster than expected. This demonstrates the power of LLVM optimization!\n");
    
    // Search benchmarks - test both early and late positions
    let target_early = 1000; // Early in array
    let target_late = SIZE as i32 - 1000; // Late in array
    
    // Verify both methods find the same results
    let loop_result_early = search_with_loop(&data, target_early);
    let iter_result_early = search_with_iterator(&data, target_early);
    let loop_result_late = search_with_loop(&data, target_late);
    let iter_result_late = search_with_iterator(&data, target_late);
    
    assert_eq!(loop_result_early, iter_result_early);
    assert_eq!(loop_result_late, iter_result_late);
    
    // Benchmark with multiple iterations for statistical significance
    let mut loop_total = std::time::Duration::ZERO;
    let mut iter_total = std::time::Duration::ZERO;
    let iterations = 100;
    
    // Test early position (should be fast for both)
    for _ in 0..iterations {
        let start = Instant::now();
        std::hint::black_box(search_with_loop(&data, target_early));
        loop_total += start.elapsed();
        
        let start = Instant::now();
        std::hint::black_box(search_with_iterator(&data, target_early));
        iter_total += start.elapsed();
    }
    
    println!("Search Performance (early position, {} iterations):", iterations);
    println!("  Loop-based:     {:?} (found at {:?})", 
             loop_total / iterations, loop_result_early);
    println!("  Iterator-based: {:?} (found at {:?})", 
             iter_total / iterations, iter_result_early);
    
    if iter_total.as_nanos() > 0 && loop_total.as_nanos() > 0 {
        println!("  Ratio: {:.2}x\n", 
                 loop_total.as_nanos() as f64 / iter_total.as_nanos() as f64);
    } else {
        println!("  Both optimized to near-zero time by compiler\n");
    }
    
    // Transformation benchmarks (use smaller dataset to avoid overflow)
    let small_data: Vec<i32> = (0..10000).collect();
    let start = Instant::now();
    let _result = transform_with_loop(&small_data);
    let loop_transform_time = start.elapsed();
    
    let start = Instant::now();
    let _result = transform_with_iterator(&small_data);
    let iter_transform_time = start.elapsed();
    
    let start = Instant::now();
    let _result = transform_with_iterator_collect(&small_data);
    let iter_collect_transform_time = start.elapsed();
    
    println!("Transformation Performance:");
    println!("  Loop-based:          {:?}", loop_transform_time);
    println!("  Iterator-based:      {:?}", iter_transform_time);
    println!("  Iterator + collect:  {:?}", iter_collect_transform_time);
    
    // Sum of squares benchmarks
    let start = Instant::now();
    let _result = sum_of_squares_loop(&float_data);
    let loop_sum_time = start.elapsed();
    
    let start = Instant::now();
    let _result = sum_of_squares_iterator(&float_data);
    let iter_sum_time = start.elapsed();
    
    println!("\nSum of Squares Performance:");
    println!("  Loop-based:     {:?}", loop_sum_time);
    println!("  Iterator-based: {:?}", iter_sum_time);
    
    // Complex pipeline benchmarks (use smaller dataset)
    let pipeline_data: Vec<i32> = (0..10000).collect();
    let start = Instant::now();
    let _result = complex_pipeline_loop(&pipeline_data);
    let loop_pipeline_time = start.elapsed();
    
    let start = Instant::now();
    let _result = complex_pipeline_iterator(&pipeline_data);
    let iter_pipeline_time = start.elapsed();
    
    println!("\nComplex Pipeline Performance:");
    println!("  Loop-based:     {:?}", loop_pipeline_time);
    println!("  Iterator-based: {:?}", iter_pipeline_time);
    
    // Audio processing benchmarks
    let mut samples_copy = audio_samples.clone();
    let start = Instant::now();
    process_audio_with_loop(&mut samples_copy, 0.8);
    let loop_audio_time = start.elapsed();
    
    let start = Instant::now();
    process_audio_with_iterator(&mut audio_samples, 0.8);
    let iter_audio_time = start.elapsed();
    
    println!("\nAudio Processing Performance:");
    println!("  Loop-based:     {:?}", loop_audio_time);
    println!("  Iterator-based: {:?}", iter_audio_time);
    
    // Large dataset processing - demonstrating zero-cost vs non-zero-cost
    let threshold = SIZE as i32 / 4;
    
    let start = Instant::now();
    let loop_result = process_large_dataset_loop(&data, threshold);
    let loop_large_time = start.elapsed();
    
    let start = Instant::now();
    let bad_result = process_large_dataset_iterator_bad(&data, threshold);
    let iter_bad_time = start.elapsed();
    
    let start = Instant::now();
    let good_result = process_large_dataset_iterator_proper(&data, threshold);
    let iter_good_time = start.elapsed();
    
    let start = Instant::now();
    let fold_result = process_large_dataset_iterator_optimized(&data, threshold);
    let iter_fold_time = start.elapsed();
    
    println!("\nLarge Dataset Processing (Zero-Cost Analysis):");
    println!("  Loop-based:              {:?} -> {:?}", loop_large_time, loop_result);
    println!("  Iterator (BAD - collect): {:?} -> {:?} ❌ NOT zero-cost!", iter_bad_time, bad_result);
    println!("  Iterator (GOOD - fold):  {:?} -> {:?} ✅ Should be zero-cost!", iter_good_time, good_result);
    println!("  Iterator (fold alt):     {:?} -> {:?}", iter_fold_time, fold_result);
    
    // Performance analysis
    let loop_vs_good = loop_large_time.as_nanos() as f64 / iter_good_time.as_nanos() as f64;
    let bad_vs_loop = iter_bad_time.as_nanos() as f64 / loop_large_time.as_nanos() as f64;
    
    println!("\n📊 Performance Analysis:");
    println!("  Good Iterator vs Loop: {:.2}x (should be ~1.0x for zero-cost)", loop_vs_good);
    println!("  Bad Iterator vs Loop:  {:.2}x (shows cost of unnecessary allocation)", bad_vs_loop);
}

/// Demonstrate compiler optimizations
fn demonstrate_compiler_optimizations() {
    println!("=== Compiler Optimization Examples ===\n");
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // These should compile to nearly identical assembly:
    
    // Traditional loop
    let mut sum1 = 0;
    for &item in &data {
        if item % 2 == 0 {
            sum1 += item * item;
        }
    }
    
    // Iterator chain
    let sum2: i32 = data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();
    
    // Functional with explicit collect (potentially less optimal)
    let sum3: i32 = data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect::<Vec<_>>()
        .iter()
        .sum();
    
    println!("Sum calculations (should be identical):");
    println!("  Loop result:      {}", sum1);
    println!("  Iterator result:  {}", sum2);
    println!("  Collect result:   {}", sum3);
    
    assert_eq!(sum1, sum2);
    assert_eq!(sum2, sum3);
    
    // Demonstrate zero-cost abstraction
    let start = Instant::now();
    for _ in 0..1000 {
        let _: i32 = data.iter().map(|&x| x * 2).sum();
    }
    let iter_time = start.elapsed();
    
    let start = Instant::now();
    for _ in 0..1000 {
        let mut _sum = 0;
        for &item in &data {
            _sum += item * 2;
        }
    }
    let loop_time = start.elapsed();
    
    println!("\nZero-cost abstraction demonstration:");
    println!("  Iterator approach: {:?}", iter_time);
    println!("  Loop approach:     {:?}", loop_time);
    println!("  Ratio: {:.2}", iter_time.as_nanos() as f64 / loop_time.as_nanos() as f64);
}

/// When iterators are NOT zero-cost
fn when_iterators_are_not_zero_cost() {
    println!("=== When Iterators Are NOT Zero-Cost ===\n");
    
    let data: Vec<i32> = (0..100000).collect();
    
    println!("🚨 MYTH BUSTING: Iterators are not always zero-cost!");
    println!("The 'zero-cost abstraction' claim applies when iterators compile");
    println!("to the same code as hand-written loops. But some patterns add cost:\n");
    
    // 1. Unnecessary intermediate collections
    println!("1. Unnecessary intermediate collections:");
    
    let start = Instant::now();
    let _result: Vec<_> = data.iter()
        .take(1000)  // Use smaller subset to avoid overflow
        .filter(|&&x| x % 2 == 0)
        .collect::<Vec<_>>()  // ❌ Creates intermediate Vec
        .iter()
        .map(|&x| x * x)
        .collect();
    let with_collect = start.elapsed();
    
    let start = Instant::now();
    let _result: Vec<_> = data.iter()
        .take(1000)  // Use smaller subset to avoid overflow
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)      // ✅ Direct chain, no intermediate allocation
        .collect();
    let without_collect = start.elapsed();
    
    println!("  With intermediate collect(): {:?} ❌", with_collect);
    println!("  Direct chain:                {:?} ✅", without_collect);
    println!("  Cost of intermediate collection: {:.2}x slower\n", 
             with_collect.as_nanos() as f64 / without_collect.as_nanos() as f64);
    
    // 2. Wrong abstraction level
    println!("2. Using wrong abstraction level:");
    
    let start = Instant::now();
    let _sum: i32 = data.iter()
        .take(1000) // Limit to avoid overflow
        .filter(|&&x| x > 500)
        .collect::<Vec<_>>()  // ❌ Collect then aggregate
        .iter()
        .copied()
        .sum();
    let collect_then_sum = start.elapsed();
    
    let start = Instant::now();
    let _sum: i32 = data.iter()
        .take(1000) // Limit to avoid overflow
        .copied()
        .filter(|&x| x > 500)
        .sum();               // ✅ Direct aggregation
    let direct_sum = start.elapsed();
    
    println!("  Collect then sum: {:?} ❌", collect_then_sum);
    println!("  Direct sum:       {:?} ✅", direct_sum);
    
    // 3. Multiple passes when one would suffice
    println!("\n3. Multiple passes vs single pass:");
    
    let start = Instant::now();
    let count = data.iter().take(1000).filter(|&&x| x > 500).count();
    let sum: i32 = data.iter().take(1000).copied().filter(|&x| x > 500).sum();
    let _avg = if count > 0 { sum as f64 / count as f64 } else { 0.0 };
    let multi_pass = start.elapsed();
    
    let start = Instant::now();
    let (count, sum) = data.iter()
        .take(1000)
        .filter(|&&x| x > 500)
        .fold((0, 0), |(count, sum), &x| (count + 1, sum + x));
    let _avg = if count > 0 { sum as f64 / count as f64 } else { 0.0 };
    let single_pass = start.elapsed();
    
    println!("  Multiple passes: {:?} ❌", multi_pass);
    println!("  Single pass:     {:?} ✅", single_pass);
    
    println!("\n💡 Key Insight: Zero-cost means 'no abstraction overhead',");
    println!("   not 'magically faster than any possible implementation'!");
    println!("   Bad algorithm + iterators = Bad performance + iterators");
}

/// Common performance pitfalls and solutions
fn performance_pitfalls() {
    println!("\n=== Performance Pitfalls and Solutions ===\n");
    
    let data: Vec<i32> = (0..1000).collect();  // Smaller to avoid overflow
    
    // PITFALL 1: Unnecessary collect()
    println!("1. Avoiding unnecessary collect()");
    
    // Inefficient: collect() creates intermediate Vec
    let start = Instant::now();
    let _result: Vec<_> = data.iter()
        .copied()
        .filter(|&x| x % 2 == 0)
        .collect::<Vec<_>>()
        .iter()
        .map(|&x| (x as i64) * (x as i64))  // Prevent overflow
        .collect();
    let collect_time = start.elapsed();
    
    // Efficient: chain operations without intermediate collection
    let start = Instant::now();
    let _result: Vec<_> = data.iter()
        .copied()
        .filter(|&x| x % 2 == 0)
        .map(|x| (x as i64) * (x as i64))  // Prevent overflow
        .collect();
    let chain_time = start.elapsed();
    
    println!("  With intermediate collect(): {:?}", collect_time);
    println!("  Direct chaining:             {:?}", chain_time);
    println!("  Speedup: {:.2}x", collect_time.as_nanos() as f64 / chain_time.as_nanos() as f64);
    
    // PITFALL 2: Using collect() when you don't need to
    println!("\n2. Using fold instead of collect + reduce");
    
    let start = Instant::now();
    let _max = data.iter()
        .filter(|&&x| x > 1000)
        .collect::<Vec<_>>()
        .iter()
        .max();
    let collect_max_time = start.elapsed();
    
    let start = Instant::now();
    let _max = data.iter()
        .filter(|&&x| x > 1000)
        .max();
    let direct_max_time = start.elapsed();
    
    println!("  Collect then max: {:?}", collect_max_time);
    println!("  Direct max:       {:?}", direct_max_time);
    
    // PITFALL 3: Choosing the right iterator method
    println!("\n3. Choosing efficient iterator methods");
    
    // Less efficient: multiple passes
    let start = Instant::now();
    let count = data.iter().filter(|&&x| x > 500).count();
    let sum: i32 = data.iter().filter(|&&x| x > 500).sum();
    let _avg = if count > 0 { sum as f64 / count as f64 } else { 0.0 };
    let multi_pass_time = start.elapsed();
    
    // More efficient: single pass with fold
    let start = Instant::now();
    let (count, sum) = data.iter()
        .filter(|&&x| x > 500)
        .fold((0, 0), |(count, sum), &x| (count + 1, sum + x));
    let _avg = if count > 0 { sum as f64 / count as f64 } else { 0.0 };
    let single_pass_time = start.elapsed();
    
    println!("  Multiple passes: {:?}", multi_pass_time);
    println!("  Single pass:     {:?}", single_pass_time);
}

/// Demonstrate when loops might be better
fn when_loops_are_better() {
    println!("=== When Loops Might Be Better ===\n");
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 1. Early termination with complex conditions
    println!("1. Early termination scenarios");
    
    fn find_complex_condition_loop(data: &[i32]) -> Option<usize> {
        for (i, &value) in data.iter().enumerate() {
            if value > 5 && value % 2 == 0 && value < 10 {
                return Some(i);
            }
        }
        None
    }
    
    fn find_complex_condition_iter(data: &[i32]) -> Option<usize> {
        data.iter()
            .enumerate()
            .find(|(_, &value)| value > 5 && value % 2 == 0 && value < 10)
            .map(|(i, _)| i)
    }
    
    let loop_result = find_complex_condition_loop(&data);
    let iter_result = find_complex_condition_iter(&data);
    
    println!("  Loop result: {:?}", loop_result);
    println!("  Iter result: {:?}", iter_result);
    assert_eq!(loop_result, iter_result);
    
    // 2. In-place modifications with complex logic
    println!("\n2. Complex in-place modifications");
    
    let mut data_loop = data.clone();
    let start = Instant::now();
    for i in 0..data_loop.len() {
        if i > 0 && data_loop[i-1] % 2 == 0 {
            data_loop[i] *= 2;
        }
    }
    let loop_modify_time = start.elapsed();
    
    // Iterator version is more complex for this case
    let mut data_iter = data.clone();
    let start = Instant::now();
    let indices_to_modify: Vec<_> = data_iter.iter()
        .enumerate()
        .skip(1)
        .filter(|(i, _)| data_iter[i-1] % 2 == 0)
        .map(|(i, _)| i)
        .collect();
    
    for i in indices_to_modify {
        data_iter[i] *= 2;
    }
    let iter_modify_time = start.elapsed();
    
    println!("  Loop modification:     {:?}", loop_modify_time);
    println!("  Iterator modification: {:?}", iter_modify_time);
    println!("  Results match: {}", data_loop == data_iter);
    
    // 3. Performance-critical tight loops
    println!("\n3. Sometimes explicit control is clearer");
    println!("   For very performance-critical code, explicit loops");
    println!("   can give you more control over memory access patterns");
    println!("   and make optimizations more predictable.");
}

fn main() {
    println!("🚀 Chapter 13.4: Comparing Performance - Loops vs Iterators\n");
    
    // Demonstrate that iterators are zero-cost abstractions
    demonstrate_compiler_optimizations();
    println!();
    
    // Show lazy evaluation benefits
    demonstrate_lazy_evaluation();
    println!();
    
    // Comprehensive performance benchmarks
    benchmark_operations();
    println!();
    
    // When iterators are NOT zero-cost (myth busting)
    when_iterators_are_not_zero_cost();
    
    // Common performance pitfalls
    performance_pitfalls();
    println!();
    
    // When loops might be preferable
    when_loops_are_better();
    
    println!("\n=== Key Takeaways (CORRECTED) ===");
    println!("✅ Iterators CAN BE zero-cost abstractions when used properly");
    println!("✅ The compiler generates identical assembly for equivalent algorithms");
    println!("✅ Iterator chains without intermediate collections are usually optimal");
    println!("✅ Functional style often leads to more readable and maintainable code");
    println!("✅ Lazy evaluation can provide significant performance benefits");
    println!("❌ NOT all iterator patterns are zero-cost - avoid these pitfalls:");
    println!("   • Unnecessary .collect() creating intermediate Vec allocations");
    println!("   • Multiple passes when single pass with .fold() would work");
    println!("   • Wrong abstraction level (collect then aggregate vs direct aggregate)");
    println!("⚠️  'Zero-cost' means 'no abstraction overhead', not 'magically faster'");
    println!("⚠️  Bad algorithm + iterators still equals bad performance");
    println!("⚠️  Sometimes explicit loops are clearer for complex state management");
    println!("⚠️  Release mode optimizations can make benchmarks look weird (0ns times)");
    println!("⚠️  Single-run benchmarks are unreliable - use statistical analysis");
    println!("📊 Always profile your specific use case - measure, don't guess!");
    println!("🔬 Use proper benchmarking tools like Criterion for production analysis");
    
    println!("\n🎯 Remember: In Rust, you can have both performance AND expressiveness!");
    println!("   But you still need to choose good algorithms and avoid unnecessary allocations!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_search_equivalence() {
        let data = vec![1, 5, 3, 7, 2, 8, 4];
        let target = 7;
        
        assert_eq!(
            search_with_loop(&data, target),
            search_with_iterator(&data, target)
        );
    }
    
    #[test]
    fn test_transform_equivalence() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        
        let loop_result = transform_with_loop(&data);
        let iter_result = transform_with_iterator(&data);
        let collect_result = transform_with_iterator_collect(&data);
        
        assert_eq!(loop_result, iter_result);
        assert_eq!(iter_result, collect_result);
    }
    
    #[test]
    fn test_sum_of_squares_equivalence() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
        let loop_result = sum_of_squares_loop(&data);
        let iter_result = sum_of_squares_iterator(&data);
        
        assert!((loop_result - iter_result).abs() < f64::EPSILON);
    }
    
    #[test]
    fn test_pipeline_equivalence() {
        let data = vec![5, 12, 8, 21, 15, 3, 18, 9];
        
        assert_eq!(
            complex_pipeline_loop(&data),
            complex_pipeline_iterator(&data)
        );
    }
    
    #[test]
    fn test_audio_processing() {
        let mut samples1 = vec![
            AudioSample::new(440.0, 0.5),
            AudioSample::new(880.0, 0.3),
        ];
        let mut samples2 = samples1.clone();
        
        process_audio_with_loop(&mut samples1, 0.8);
        process_audio_with_iterator(&mut samples2, 0.8);
        
        for (s1, s2) in samples1.iter().zip(samples2.iter()) {
            assert!((s1.amplitude - s2.amplitude).abs() < f64::EPSILON);
        }
    }
    
    #[test]
    fn test_large_dataset_processing() {
        let data = vec![5, 12, 8, 21, 15, 3, 18, 9, 25, 7];
        let threshold = 10;
        
        let loop_result = process_large_dataset_loop(&data, threshold);
        let iter_bad_result = process_large_dataset_iterator_bad(&data, threshold);
        let iter_good_result = process_large_dataset_iterator_proper(&data, threshold);
        let opt_result = process_large_dataset_iterator_optimized(&data, threshold);
        
        // All methods should produce the same result
        assert_eq!(loop_result, iter_bad_result);
        assert_eq!(iter_bad_result, iter_good_result);
        assert_eq!(iter_good_result, opt_result);
        
        // Expected result: count=4 (12, 21, 15, 18, 25), max=25
        assert_eq!(loop_result, (5, 25)); // Note: 25 > 21 > 18 > 15 > 12, count = 5
    }
    
    #[test]
    fn test_functional_audio_processing() {
        let samples = vec![
            AudioSample::new(440.0, 0.5),
            AudioSample::new(880.0, 0.3),
        ];
        
        let result = process_audio_functional(&samples, 0.8);
        
        // Verify that the original samples are unchanged
        assert_eq!(samples[0].amplitude, 0.5);
        assert_eq!(samples[1].amplitude, 0.3);
        
        // Verify the processing worked
        assert_eq!(result.len(), 2);
    }
}

// Performance tips and best practices
#[allow(dead_code)]
fn performance_tips() {
    println!("=== Performance Tips ===");
    println!("1. Prefer iterator chains over intermediate collections");
    println!("2. Use `fold` instead of `collect` + `reduce` when possible");
    println!("3. Take advantage of lazy evaluation");
    println!("4. Use `copied()` or `cloned()` explicitly when needed");
    println!("5. Avoid unnecessary `collect()` calls");
    println!("6. Use specialized methods like `sum()`, `max()`, `min()`");
    println!("7. Consider `for_each()` instead of `map()` for side effects");
    println!("8. Profile your code to verify assumptions");
}

// Assembly analysis tips (conceptual)
#[allow(dead_code)]
fn assembly_analysis_tips() {
    println!("=== Assembly Analysis Tips ===");
    println!("To see generated assembly:");
    println!("1. Use `cargo rustc --release -- --emit asm`");
    println!("2. Use `objdump -d target/release/your_binary`");
    println!("3. Use online tools like Compiler Explorer (godbolt.org)");
    println!("4. Look for vectorization (SIMD instructions)");
    println!("5. Check for loop unrolling and inlining");
    println!("6. Verify that bounds checks are eliminated");
}