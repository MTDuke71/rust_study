//! Generic memoization cache for dynamic programming
//! 
//! # Requirements Satisfied
//! - REQ-1: Generic recursive memoization framework
//! - REQ-3: Zero-copy string slice caching (via generic K parameter)
//! - REQ-6: Overlapping subproblem detection (cache statistics)

use std::collections::HashMap;
use std::hash::Hash;

/// Generic memoization cache supporting arbitrary key/value types
/// 
/// # Requirements Satisfied
/// - REQ-1: Generic cache structure with HashMap backing
/// - REQ-6: Cache statistics for performance analysis
/// 
/// # Type Parameters
/// - `K`: Key type (must implement Hash + Eq)
/// - `V`: Value type (must implement Clone for retrieval)
/// 
/// # Examples
/// ```rust
/// use mission11::MemoCache;
/// 
/// let mut cache = MemoCache::<(usize, usize), u64>::new();
/// 
/// let result = cache.memoize((5, 3), || 42);
/// assert_eq!(result, 42);
/// 
/// // Second call uses cached value
/// let result2 = cache.memoize((5, 3), || panic!("Should not compute!"));
/// assert_eq!(result2, 42);
/// ```
pub struct MemoCache<K, V> 
where
    K: Hash + Eq,
    V: Clone,
{
    cache: HashMap<K, V>,
    hits: usize,
    misses: usize,
}

impl<K, V> MemoCache<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    /// Creates a new empty memoization cache
    /// 
    /// # Requirements Satisfied: REQ-1
    /// - Generic type support for any K, V
    /// - Safe initialization with no unsafe operations
    /// 
    /// # Performance: O(1)
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    /// Creates cache with pre-allocated capacity
    /// 
    /// # Requirements Satisfied: REQ-1
    /// - Performance optimization for known state space sizes
    /// 
    /// # Performance: O(capacity) allocation
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(capacity),
            hits: 0,
            misses: 0,
        }
    }

    /// Retrieves or computes a memoized value
    /// 
    /// # Requirements Satisfied
    /// - REQ-1: Generic memoization API
    /// - REQ-2: Top-down DP pattern (cache check before compute)
    /// - REQ-6: Cache hit/miss tracking
    /// 
    /// # Arguments
    /// - `key`: Cache key to check
    /// - `compute`: Closure to compute value if not cached
    /// 
    /// # Returns
    /// Cached or newly computed value (cloned from cache)
    /// 
    /// # Performance
    /// - Cache hit: O(1) average case
    /// - Cache miss: O(compute) + O(1) insertion
    /// 
    /// # Examples
    /// ```rust
    /// use mission11::MemoCache;
    /// 
    /// let mut cache = MemoCache::new();
    /// let fib5 = cache.memoize(5, || {
    ///     // Expensive Fibonacci computation
    ///     5
    /// });
    /// ```
    pub fn memoize<F>(&mut self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(cached) = self.cache.get(&key) {
            self.hits += 1;
            return cached.clone();
        }

        self.misses += 1;
        let value = compute();
        self.cache.insert(key, value.clone());
        value
    }

    /// Returns number of cache hits
    /// 
    /// # Requirements Satisfied: REQ-6
    /// - Overlapping subproblem detection via hit ratio
    pub fn hits(&self) -> usize {
        self.hits
    }

    /// Returns number of cache misses
    /// 
    /// # Requirements Satisfied: REQ-6
    /// - Overlapping subproblem detection via hit ratio
    pub fn misses(&self) -> usize {
        self.misses
    }

    /// Returns cache hit ratio (0.0 to 1.0)
    /// 
    /// # Requirements Satisfied: REQ-6
    /// - Memoization effectiveness metric
    /// 
    /// # Returns
    /// - Hit ratio: hits / (hits + misses)
    /// - 0.0 if no operations yet
    /// 
    /// # Examples
    /// ```rust
    /// use mission11::MemoCache;
    /// 
    /// let mut cache = MemoCache::new();
    /// cache.memoize(1, || 10);
    /// cache.memoize(1, || 20); // Cache hit
    /// 
    /// assert_eq!(cache.hit_ratio(), 0.5); // 1 hit, 1 miss
    /// ```
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Returns number of unique entries in cache
    /// 
    /// # Requirements Satisfied: REQ-6
    /// - State space size analysis
    pub fn size(&self) -> usize {
        self.cache.len()
    }

    /// Clears all cached values and resets statistics
    /// 
    /// # Requirements Satisfied: REQ-1
    /// - Cache management and cleanup
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }
}

impl<K, V> Default for MemoCache<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_creation() {
        let cache: MemoCache<i32, String> = MemoCache::new();
        assert_eq!(cache.size(), 0);
        assert_eq!(cache.hits(), 0);
        assert_eq!(cache.misses(), 0);
    }

    #[test]
    fn test_memoize_first_call() {
        let mut cache = MemoCache::new();
        let result = cache.memoize(42, || "computed".to_string());
        
        assert_eq!(result, "computed");
        assert_eq!(cache.size(), 1);
        assert_eq!(cache.misses(), 1);
        assert_eq!(cache.hits(), 0);
    }

    #[test]
    fn test_memoize_cached_call() {
        let mut cache = MemoCache::new();
        
        cache.memoize(42, || "first".to_string());
        let result = cache.memoize(42, || "second".to_string());
        
        assert_eq!(result, "first"); // Should return cached value
        assert_eq!(cache.hits(), 1);
        assert_eq!(cache.misses(), 1);
    }

    #[test]
    fn test_hit_ratio() {
        let mut cache = MemoCache::new();
        
        cache.memoize(1, || 10);  // Miss
        cache.memoize(2, || 20);  // Miss
        cache.memoize(1, || 30);  // Hit
        cache.memoize(1, || 40);  // Hit
        
        assert_eq!(cache.hit_ratio(), 0.5); // 2 hits, 2 misses
    }
}
