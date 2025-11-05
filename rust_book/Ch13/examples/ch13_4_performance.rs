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
    for i in 0..data.len() {
        if data[i] == target {
            return Some(i);
        }
    }
    None
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

fn process_large_dataset_iterator(data: &[i32], threshold: i32) -> (usize, i32) {
    let filtered: Vec<_> = data.iter().copied().filter(|&x| x > threshold).collect();
    let count = filtered.len();
    let max = filtered.into_iter().max().unwrap_or(i32::MIN);
    (count, max)
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
    println!("Dataset size: {} elements\n", SIZE);
    
    // Search benchmarks
    let target = SIZE as i32 / 2;
    
    let start = Instant::now();
    let _result = search_with_loop(&data, target);
    let loop_search_time = start.elapsed();
    
    let start = Instant::now();
    let _result = search_with_iterator(&data, target);
    let iter_search_time = start.elapsed();
    
    println!("Search Performance:");
    println!("  Loop-based:     {:?}", loop_search_time);
    println!("  Iterator-based: {:?}", iter_search_time);
    println!("  Speedup: {:.2}x\n", 
             loop_search_time.as_nanos() as f64 / iter_search_time.as_nanos() as f64);
    
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
    
    // Large dataset processing
    let threshold = SIZE as i32 / 4;
    
    let start = Instant::now();
    let _result = process_large_dataset_loop(&data, threshold);
    let loop_large_time = start.elapsed();
    
    let start = Instant::now();
    let _result = process_large_dataset_iterator(&data, threshold);
    let iter_large_time = start.elapsed();
    
    let start = Instant::now();
    let _result = process_large_dataset_iterator_optimized(&data, threshold);
    let iter_opt_large_time = start.elapsed();
    
    println!("\nLarge Dataset Processing:");
    println!("  Loop-based:          {:?}", loop_large_time);
    println!("  Iterator (collect):  {:?}", iter_large_time);
    println!("  Iterator (fold):     {:?}", iter_opt_large_time);
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

/// Common performance pitfalls and solutions
fn performance_pitfalls() {
    println!("=== Performance Pitfalls and Solutions ===\n");
    
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
    
    // Common performance pitfalls
    performance_pitfalls();
    println!();
    
    // When loops might be preferable
    when_loops_are_better();
    
    println!("\n=== Key Takeaways ===");
    println!("✅ Iterators are zero-cost abstractions in Rust");
    println!("✅ The compiler often generates identical assembly for loops vs iterators");
    println!("✅ Iterators can be more performant due to better optimization opportunities");
    println!("✅ Functional style often leads to more readable and maintainable code");
    println!("✅ Lazy evaluation can provide significant performance benefits");
    println!("✅ Avoid unnecessary intermediate collections");
    println!("✅ Use the right iterator method for the job (fold vs collect + reduce)");
    println!("⚠️  Sometimes explicit loops are clearer for complex state management");
    println!("⚠️  Profile your specific use case - performance can vary by scenario");
    
    println!("\n🎯 Remember: In Rust, you can have both performance AND expressiveness!");
    println!("   The compiler is your friend - use high-level abstractions with confidence!");
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
        let iter_result = process_large_dataset_iterator(&data, threshold);
        let opt_result = process_large_dataset_iterator_optimized(&data, threshold);
        
        assert_eq!(loop_result, iter_result);
        assert_eq!(iter_result, opt_result);
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