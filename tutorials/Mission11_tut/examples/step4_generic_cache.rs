//! Step 4: Generic Cache Abstraction - Make Patterns Reusable
//!
//! **Learning Objective**: By the end of this step, you will be able to create
//! reusable memoization patterns that work across different problem types.
//!
//! **Prerequisites**: Step 3 (lifetime safety), generics understanding
//!
//! **Next Step Preview**: Step 5 will show how to transform boolean existence
//! checks into exhaustive counting using the same cache pattern.

use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    println!("=== Step 4: Generic Cache Abstraction ===\n");
    
    demonstrate_core_concept();
    show_common_patterns();
    handle_edge_cases();
    validate_understanding();
    
    println!("\n✅ Step 4 completed! Ready for Step 5.");
    println!("💡 Key Insight: Abstraction enables pattern reuse across many problems!");
}

/// Core concept demonstration
///
/// Shows how to create a generic memoization cache with statistics.
/// Key learning: Generic types enable reuse while maintaining type safety.
fn demonstrate_core_concept() {
    println!("--- Core Concept: Generic Memoization Cache ---");
    
    println!("\n1. The Goal: Reusable cache for any key/value types");
    println!("   Generic over K (key type) and V (value type)");
    println!("   Provides get_or_compute pattern for easy usage");
    println!("   Tracks statistics (hits, misses, hit rate)");
    
    println!("\n2. Implementation:");
    println!("   struct MemoCache<K, V> where K: Hash + Eq, V: Clone");
    println!("   - K must be hashable and comparable");
    println!("   - V must be cloneable (to return from cache)");
    
    demonstrate_pattern_matching_with_cache();
}

/// Generic memoization cache with statistics
struct MemoCache<K, V> {
    cache: HashMap<K, V>,
    hits: usize,
    misses: usize,
}

impl<K: Hash + Eq, V: Clone> MemoCache<K, V> {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    
    /// Get value from cache
    fn get(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.cache.get(key) {
            self.hits += 1;
            Some(value.clone())
        } else {
            None
        }
    }
    
    /// Insert value into cache
    fn insert(&mut self, key: K, value: V) {
        self.misses += 1;
        self.cache.insert(key, value);
    }
    
    /// Get value from cache or compute it (convenience method for non-recursive cases)
    fn get_or_compute<F>(&mut self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(value) = self.cache.get(&key) {
            self.hits += 1;
            return value.clone();
        }
        
        self.misses += 1;
        let value = compute();
        self.cache.insert(key, value.clone());
        value
    }
    
    /// Get cache statistics
    fn stats(&self) -> CacheStats {
        let hit_rate = if self.hits + self.misses > 0 {
            self.hits as f64 / (self.hits + self.misses) as f64
        } else {
            0.0
        };
        
        CacheStats {
            hits: self.hits,
            misses: self.misses,
            hit_rate,
            size: self.cache.len(),
        }
    }
    
    fn clear_stats(&mut self) {
        self.hits = 0;
        self.misses = 0;
    }
}

#[derive(Debug)]
struct CacheStats {
    hits: usize,
    misses: usize,
    hit_rate: f64,
    size: usize,
}

fn demonstrate_pattern_matching_with_cache() {
    println!("\n--- Using Generic Cache for Pattern Matching ---");
    
    let patterns = vec!["r", "wr", "b", "br"];
    let design = "brwrrbrwrr";
    
    let mut cache = MemoCache::new();
    
    let result = can_make_with_cache(&patterns, design, &mut cache);
    let stats = cache.stats();
    
    println!("Design: '{}'", design);
    println!("Result: {}", result);
    println!("Statistics:");
    println!("  Cache hits: {}", stats.hits);
    println!("  Cache misses: {}", stats.misses);
    println!("  Hit rate: {:.1}%", stats.hit_rate * 100.0);
    println!("  Cache size: {}", stats.size);
}

/// Pattern matching using generic cache
/// Note: Manual check-compute-insert pattern required for recursive functions
fn can_make_with_cache<'a>(
    patterns: &[&str],
    design: &'a str,
    cache: &mut MemoCache<&'a str, bool>,
) -> bool {
    if design.is_empty() {
        return true;
    }
    
    // Manual pattern: check cache first
    if let Some(result) = cache.get(&design) {
        return result;
    }
    
    // Compute if not cached
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            if can_make_with_cache(patterns, rest, cache) {
                cache.insert(design, true);
                return true;
            }
        }
    }
    
    cache.insert(design, false);
    false
}

/// Common usage patterns
///
/// Shows generic cache applied to different problems.
fn show_common_patterns() {
    println!("\n--- Common Patterns ---");
    
    pattern_1_fibonacci_with_cache();
    pattern_2_string_problems();
    pattern_3_cache_statistics_analysis();
}

fn pattern_1_fibonacci_with_cache() {
    println!("\nPattern 1: Generic cache for numeric problems (Fibonacci)");
    
    let mut cache = MemoCache::new();
    
    let n = 20;
    let result = fibonacci_cached(n, &mut cache);
    let stats = cache.stats();
    
    println!("  fib({}) = {}", n, result);
    println!("  Cache hits: {}, misses: {}", stats.hits, stats.misses);
    println!("  Hit rate: {:.1}%", stats.hit_rate * 100.0);
    
    // Call again to see cache reuse
    cache.clear_stats();
    let result2 = fibonacci_cached(n, &mut cache);
    let stats2 = cache.stats();
    
    println!("\n  Calling fib({}) again:", n);
    println!("  Result: {} (same)", result2);
    println!("  Cache hits: {} (all hits!)", stats2.hits);
    println!("  Cache misses: {} (no new computation)", stats2.misses);
}

fn fibonacci_cached(n: u64, cache: &mut MemoCache<u64, u64>) -> u64 {
    if n <= 1 {
        return n;
    }
    
    // Manual pattern for recursive memoization
    if let Some(result) = cache.get(&n) {
        return result;
    }
    
    let result = fibonacci_cached(n - 1, cache) + fibonacci_cached(n - 2, cache);
    cache.insert(n, result);
    result
}

fn pattern_2_string_problems() {
    println!("\nPattern 2: Generic cache for multiple string problems");
    
    // Problem 1: Pattern matching
    let patterns = vec!["r", "wr"];
    let mut cache1 = MemoCache::new();
    let result1 = can_make_with_cache(&patterns, "wrr", &mut cache1);
    println!("  Pattern matching: {} (cache size: {})", result1, cache1.stats().size);
    
    // Problem 2: Could use same cache type for different string problem
    let mut cache2: MemoCache<String, usize> = MemoCache::new();
    let text = "hello";
    let count = cache2.get_or_compute(text.to_string(), || text.len());
    println!("  String length caching: {} (demonstrating flexibility)", count);
}

fn pattern_3_cache_statistics_analysis() {
    println!("\nPattern 3: Analyzing cache performance");
    
    let patterns = vec!["r", "wr", "b", "br"];
    
    let test_cases = vec![
        ("r", 1),
        ("wrr", 3),
        ("brwrr", 5),
        ("brwrrbrwrr", 10),
    ];
    
    println!("\n  Design Length | Hits | Misses | Hit Rate | Cache Size");
    println!("  ------------- | ---- | ------ | -------- | ----------");
    
    for (design, len) in test_cases {
        let mut cache = MemoCache::new();
        can_make_with_cache(&patterns, design, &mut cache);
        let stats = cache.stats();
        
        println!("  {:13} | {:4} | {:6} | {:7.1}% | {:10}",
                 len, stats.hits, stats.misses, stats.hit_rate * 100.0, stats.size);
    }
    
    println!("\n  Observation: Hit rate increases with design length");
    println!("  Why? Longer strings have more repeated substrings");
}

/// Edge case handling
///
/// Demonstrates generic cache behavior in special cases.
fn handle_edge_cases() {
    println!("\n--- Edge Cases ---");
    
    different_value_types();
    cache_with_complex_keys();
    empty_cache_behavior();
}

fn different_value_types() {
    println!("\nEdge Case 1: Cache with different value types");
    
    // Boolean cache (what we've been using)
    let mut bool_cache: MemoCache<&str, bool> = MemoCache::new();
    bool_cache.get_or_compute("test", || true);
    println!("  Boolean cache: works for existence checks");
    
    // Numeric cache (for counting)
    let mut num_cache: MemoCache<&str, u64> = MemoCache::new();
    num_cache.get_or_compute("test", || 42);
    println!("  Numeric cache: works for counting problems");
    
    // String cache (for transformations)
    let mut str_cache: MemoCache<String, String> = MemoCache::new();
    str_cache.get_or_compute("hello".to_string(), || "HELLO".to_string());
    println!("  String cache: works for memoizing transformations");
    
    println!("  Generic implementation works for all types!");
}

fn cache_with_complex_keys() {
    println!("\nEdge Case 2: Cache with tuple keys (2D problems)");
    
    // Grid paths: cache (row, col) positions
    let mut grid_cache: MemoCache<(usize, usize), u64> = MemoCache::new();
    
    grid_cache.get_or_compute((0, 0), || 1);
    grid_cache.get_or_compute((0, 1), || 1);
    grid_cache.get_or_compute((1, 0), || 1);
    grid_cache.get_or_compute((1, 1), || 2);  // (0,1) + (1,0)
    
    println!("  Tuple keys: {} entries", grid_cache.stats().size);
    println!("  Useful for 2D DP problems (grid paths, LCS, edit distance)");
}

fn empty_cache_behavior() {
    println!("\nEdge Case 3: Empty cache behavior");
    
    let mut cache: MemoCache<i32, i32> = MemoCache::new();
    let stats = cache.stats();
    
    println!("  Empty cache hit rate: {:.1}%", stats.hit_rate * 100.0);
    println!("  (Correctly shows 0.0%, no division by zero)");
    
    cache.get_or_compute(1, || 10);
    let stats2 = cache.stats();
    println!("  After one miss: hit rate = {:.1}%", stats2.hit_rate * 100.0);
}

/// Understanding validation
///
/// Exercises to validate generic abstraction understanding.
fn validate_understanding() {
    println!("\n--- Validation Exercises ---");
    
    println!("\n🤔 Generic abstraction comprehension:");
    
    // Exercise 1
    println!("\nExercise 1: What constraints are needed on K?");
    println!("  MemoCache<K, V> where K: ??? + ???");
    println!("  Prediction: _____ and _____");
    println!("  Actual: Hash + Eq (for HashMap keys)");
    
    // Exercise 2
    println!("\nExercise 2: Why does V need Clone?");
    println!("  a) To insert into HashMap");
    println!("  b) To return cached values without moving them");
    println!("  c) For better performance");
    println!("  Actual: b - We clone to return value while keeping it in cache");
    
    // Exercise 3
    println!("\nExercise 3: Design a cache for this problem:");
    println!("  Problem: Memoize function f(row, col) -> count");
    println!("  Key type: _____");
    println!("  Value type: _____");
    println!("  Actual: MemoCache<(usize, usize), u64>");
    
    println!("\n✅ If you can design caches for new problems, you grasp generics!");
    println!("🔄 If uncertain, review the demonstrate_core_concept() section.");
    
    // Practical exercise
    println!("\n--- Practical Exercise ---");
    demonstrate_custom_problem();
}

fn demonstrate_custom_problem() {
    println!("\nApplying generic cache to factorials:");
    
    let mut cache = MemoCache::new();
    
    for n in [5, 10, 5, 12, 10] {
        cache.clear_stats();
        let result = factorial_cached(n, &mut cache);
        let stats = cache.stats();
        
        println!("  factorial({:2}) = {:10} | hits: {:2}, misses: {:2}",
                 n, result, stats.hits, stats.misses);
    }
    
    println!("\n  Notice: Second call to factorial(5) and factorial(10) hits cache!");
}

fn factorial_cached(n: u64, cache: &mut MemoCache<u64, u64>) -> u64 {
    if n <= 1 {
        return 1;
    }
    
    // Manual pattern for recursive memoization
    if let Some(result) = cache.get(&n) {
        return result;
    }
    
    let result = n * factorial_cached(n - 1, cache);
    cache.insert(n, result);
    result
}

#[cfg(test)]
mod step_tests {
    use super::*;
    
    #[test]
    fn test_generic_cache_boolean() {
        let mut cache = MemoCache::new();
        
        let result = cache.get_or_compute("key1", || true);
        assert!(result);
        
        let stats = cache.stats();
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hits, 0);
        
        // Second call should hit cache
        let result2 = cache.get_or_compute("key1", || false);
        assert!(result2);  // Returns cached true, not computed false
        
        let stats2 = cache.stats();
        assert_eq!(stats2.hits, 1);
    }
    
    #[test]
    fn test_generic_cache_numeric() {
        let mut cache = MemoCache::new();
        
        let fib5 = fibonacci_cached(5, &mut cache);
        assert_eq!(fib5, 5);
        
        let stats = cache.stats();
        assert!(stats.size > 0);
    }
    
    #[test]
    fn test_cache_statistics() {
        let mut cache: MemoCache<i32, i32> = MemoCache::new();
        
        cache.get_or_compute(1, || 10);  // miss
        cache.get_or_compute(1, || 20);  // hit
        cache.get_or_compute(1, || 30);  // hit
        
        let stats = cache.stats();
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hits, 2);
        assert!((stats.hit_rate - 2.0/3.0).abs() < 0.01);
    }
    
    #[test]
    fn test_pattern_matching_with_cache() {
        let patterns = vec!["r", "wr", "b"];
        let mut cache = MemoCache::new();
        
        assert!(can_make_with_cache(&patterns, "wrr", &mut cache));
        assert!(!can_make_with_cache(&patterns, "g", &mut cache));
    }
}
