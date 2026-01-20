//! # Cache Instrumentation and Performance Analysis
//!
//! **REQ-6**: Overlapping Subproblem Detection
//!
//! Provides instrumentation to identify when memoization provides benefit:
//! - Cache statistics (hits, misses, size)
//! - Performance comparison (naive vs memoized)
//! - Complexity transformation demonstration
//!
//! ## Requirements Satisfied
//! - Debug helper: `CacheStats` with tracking
//! - Naive vs memoized comparison utilities
//! - Concrete examples showing exponential → linear
//! - Documentation explaining when memoization matters

use std::collections::HashMap;
use std::hash::Hash;

/// Cache statistics for analyzing memoization effectiveness.
///
/// Tracks metrics to determine when memoization provides benefit:
/// - Hit/miss ratio (higher = more subproblem overlap)
/// - Cache size (actual unique states vs theoretical state space)
/// - Max depth (recursion depth indicator)
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Total cache lookups (hits + misses)
    pub lookups: usize,
    /// Cache hits (found existing entry)
    pub hits: usize,
    /// Cache misses (computed new entry)
    pub misses: usize,
    /// Maximum cache size reached
    pub max_size: usize,
    /// Maximum recursion depth
    pub max_depth: usize,
    /// Current recursion depth (for tracking)
    current_depth: usize,
}

impl CacheStats {
    /// Create new cache statistics tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a cache hit.
    pub fn record_hit(&mut self) {
        self.lookups += 1;
        self.hits += 1;
    }

    /// Record a cache miss.
    pub fn record_miss(&mut self) {
        self.lookups += 1;
        self.misses += 1;
    }

    /// Update cache size tracking.
    pub fn update_size(&mut self, size: usize) {
        if size > self.max_size {
            self.max_size = size;
        }
    }

    /// Enter a recursion level.
    pub fn enter_recursion(&mut self) {
        self.current_depth += 1;
        if self.current_depth > self.max_depth {
            self.max_depth = self.current_depth;
        }
    }

    /// Exit a recursion level.
    pub fn exit_recursion(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
    }

    /// Calculate hit ratio (0.0 to 1.0).
    ///
    /// High ratio (>0.5) indicates significant subproblem overlap.
    pub fn hit_ratio(&self) -> f64 {
        if self.lookups == 0 {
            0.0
        } else {
            self.hits as f64 / self.lookups as f64
        }
    }

    /// Calculate cache efficiency.
    ///
    /// Returns ratio of cache size to total computations.
    /// Low ratio = high reuse, good memoization candidate.
    pub fn efficiency(&self) -> f64 {
        if self.misses == 0 {
            0.0
        } else {
            self.max_size as f64 / self.misses as f64
        }
    }

    /// Pretty-print statistics.
    pub fn report(&self) -> String {
        format!(
            "Cache Statistics:\n\
             - Lookups: {}\n\
             - Hits: {} ({:.1}%)\n\
             - Misses: {}\n\
             - Max Cache Size: {}\n\
             - Max Depth: {}\n\
             - Hit Ratio: {:.3}\n\
             - Efficiency: {:.3}",
            self.lookups,
            self.hits,
            self.hit_ratio() * 100.0,
            self.misses,
            self.max_size,
            self.max_depth,
            self.hit_ratio(),
            self.efficiency()
        )
    }

    /// Determine if memoization is beneficial based on stats.
    ///
    /// Heuristic: memoization helps if:
    /// - Hit ratio > 0.3 (30% of lookups hit cache)
    /// - OR cache size significantly smaller than total computations
    pub fn is_memoization_beneficial(&self) -> bool {
        self.hit_ratio() > 0.3 || (self.misses > self.max_size * 2)
    }
}

/// Instrumented cache wrapper for performance analysis.
///
/// Wraps a HashMap with statistics tracking.
pub struct InstrumentedCache<K, V> {
    cache: HashMap<K, V>,
    stats: CacheStats,
}

impl<K, V> InstrumentedCache<K, V>
where
    K: Hash + Eq,
{
    /// Create new instrumented cache.
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            stats: CacheStats::new(),
        }
    }

    /// Get value from cache, tracking hit/miss.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(value) = self.cache.get(key) {
            self.stats.record_hit();
            Some(value)
        } else {
            self.stats.record_miss();
            None
        }
    }

    /// Insert value into cache.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let result = self.cache.insert(key, value);
        self.stats.update_size(self.cache.len());
        result
    }

    /// Get cache statistics.
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Get mutable statistics (for recursion tracking).
    pub fn stats_mut(&mut self) -> &mut CacheStats {
        &mut self.stats
    }

    /// Number of entries in cache.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

impl<K, V> Default for InstrumentedCache<K, V>
where
    K: Hash + Eq,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Performance comparison result.
#[derive(Debug)]
pub struct PerformanceComparison {
    pub naive_time_ns: Option<u128>,
    pub memoized_time_ns: u128,
    pub speedup: Option<f64>,
    pub cache_stats: CacheStats,
}

impl PerformanceComparison {
    /// Pretty-print comparison.
    pub fn report(&self) -> String {
        let mut result = String::new();
        
        if let Some(naive) = self.naive_time_ns {
            result.push_str(&format!("Naive execution: {:.2}ms\n", naive as f64 / 1_000_000.0));
        } else {
            result.push_str("Naive execution: TIMEOUT (infeasible)\n");
        }
        
        result.push_str(&format!(
            "Memoized execution: {:.2}ms\n",
            self.memoized_time_ns as f64 / 1_000_000.0
        ));
        
        if let Some(speedup) = self.speedup {
            result.push_str(&format!("Speedup: {:.2}x\n\n", speedup));
        } else {
            result.push_str("Speedup: ∞ (exponential → linear)\n\n");
        }
        
        result.push_str(&self.cache_stats.report());
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_stats_tracking() {
        let mut stats = CacheStats::new();
        
        // Record some hits and misses
        stats.record_miss();
        stats.record_miss();
        stats.record_hit();
        stats.record_hit();
        stats.record_hit();
        
        assert_eq!(stats.lookups, 5);
        assert_eq!(stats.hits, 3);
        assert_eq!(stats.misses, 2);
        assert_eq!(stats.hit_ratio(), 0.6);
    }

    #[test]
    fn test_instrumented_cache() {
        let mut cache = InstrumentedCache::new();
        
        // First access - miss
        assert!(cache.get(&"key1").is_none());
        assert_eq!(cache.stats().misses, 1);
        
        // Insert
        cache.insert("key1", 42);
        
        // Second access - hit
        assert_eq!(cache.get(&"key1"), Some(&42));
        assert_eq!(cache.stats().hits, 1);
        
        // Different key - miss
        assert!(cache.get(&"key2").is_none());
        assert_eq!(cache.stats().misses, 2);
        
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_recursion_depth_tracking() {
        let mut stats = CacheStats::new();
        
        stats.enter_recursion();
        stats.enter_recursion();
        stats.enter_recursion();
        assert_eq!(stats.max_depth, 3);
        
        stats.exit_recursion();
        stats.enter_recursion();
        assert_eq!(stats.max_depth, 3); // Max stays 3
        
        stats.exit_recursion();
        stats.exit_recursion();
        stats.exit_recursion();
        assert_eq!(stats.current_depth, 0);
    }

    #[test]
    fn test_memoization_benefit_detection() {
        let mut stats = CacheStats::new();
        
        // Low hit ratio - need misses to establish baseline
        stats.record_miss();
        stats.record_miss();
        stats.record_miss();
        stats.record_hit();
        stats.update_size(3);  // 3 unique entries, 4 total lookups
        
        // Hit ratio = 1/4 = 0.25 < 0.3, cache reuse check:
        // misses (3) > max_size (3) * 2? No, so not beneficial yet
        assert!(!stats.is_memoization_beneficial());
        
        // Add more hits - high hit ratio, beneficial
        for _ in 0..10 {
            stats.record_hit();
        }
        // Now hit ratio = 11/14 = 0.79 > 0.3
        assert!(stats.is_memoization_beneficial());
    }

    #[test]
    fn test_cache_size_tracking() {
        let mut cache = InstrumentedCache::new();
        
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);
        
        assert_eq!(cache.stats().max_size, 3);
        assert_eq!(cache.len(), 3);
    }
}
