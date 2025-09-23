use std::collections::HashMap;
use std::hash::Hash;

/// Memoization cache for dynamic programming and expensive computations
/// 
/// Optimized HashMap wrapper for caching function results.
/// Provides convenient methods for memoization patterns common in
/// recursive algorithms and dynamic programming solutions.
/// 
/// # Requirements Satisfied: REQ-5
/// 
/// # Examples
/// 
/// ```rust
/// use mission5::MemoCache;
/// 
/// let mut cache = MemoCache::new();
/// 
/// // Cache expensive computation
/// cache.insert("fibonacci_10", 55);
/// 
/// // Retrieve cached result
/// assert_eq!(cache.get(&"fibonacci_10"), Some(&55));
/// ```
#[derive(Debug, Clone)]
pub struct MemoCache<K, V>
where
    K: Eq + Hash,
{
    cache: HashMap<K, V>,
    hits: usize,
    misses: usize,
}

impl<K, V> MemoCache<K, V>
where
    K: Eq + Hash,
{
    /// Create a new empty cache
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::MemoCache;
    /// 
    /// let cache: MemoCache<String, i32> = MemoCache::new();
    /// assert_eq!(cache.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    /// Create a cache with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(capacity),
            hits: 0,
            misses: 0,
        }
    }

    /// Insert a key-value pair into the cache
    /// 
    /// Returns the previous value if key existed
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::MemoCache;
    /// 
    /// let mut cache = MemoCache::new();
    /// cache.insert("key", "value");
    /// 
    /// assert_eq!(cache.get(&"key"), Some(&"value"));
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.cache.insert(key, value)
    }

    /// Get a reference to the cached value
    /// 
    /// Updates hit/miss statistics for performance monitoring
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::MemoCache;
    /// 
    /// let mut cache = MemoCache::new();
    /// cache.insert("existing", 42);
    /// 
    /// assert_eq!(cache.get(&"existing"), Some(&42));
    /// assert_eq!(cache.get(&"missing"), None);
    /// 
    /// // Check cache statistics
    /// assert_eq!(cache.hit_count(), 1);
    /// assert_eq!(cache.miss_count(), 1);
    /// ```
    pub fn get(&mut self, key: &K) -> Option<&V> {
        match self.cache.get(key) {
            Some(value) => {
                self.hits += 1;
                Some(value)
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }

    /// Get a reference without updating statistics
    /// 
    /// Useful for queries that shouldn't affect cache metrics
    pub fn peek(&self, key: &K) -> Option<&V> {
        self.cache.get(key)
    }

    /// Get a mutable reference to the cached value
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.cache.get_mut(key)
    }

    /// Remove a key-value pair from the cache
    /// 
    /// Returns the value if key existed
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.cache.remove(key)
    }

    /// Check if cache contains a key
    pub fn contains_key(&self, key: &K) -> bool {
        self.cache.contains_key(key)
    }

    /// Get number of cached items
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Clear all cached items and reset statistics
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }

    /// Get cache hit count
    pub fn hit_count(&self) -> usize {
        self.hits
    }

    /// Get cache miss count
    pub fn miss_count(&self) -> usize {
        self.misses
    }

    /// Calculate cache hit ratio (hits / total_accesses)
    /// 
    /// Returns 0.0 if no accesses have been made
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::MemoCache;
    /// 
    /// let mut cache = MemoCache::new();
    /// cache.insert("key", "value");
    /// 
    /// cache.get(&"key");    // hit
    /// cache.get(&"missing"); // miss
    /// cache.get(&"key");    // hit
    /// 
    /// assert_eq!(cache.hit_ratio(), 2.0 / 3.0); // 2 hits out of 3 accesses
    /// ```
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Get an iterator over key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.cache.iter()
    }

    /// Get an iterator over keys
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.cache.keys()
    }

    /// Get an iterator over values
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.cache.values()
    }
}

impl<K, V> MemoCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Get value or compute and cache it if missing
    /// 
    /// This is the core memoization method - it returns cached values
    /// or computes new ones using the provided function
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::MemoCache;
    /// 
    /// let mut cache = MemoCache::new();
    /// 
    /// // First call computes and caches
    /// let result1 = cache.get_or_compute(5, |&n| n * n);
    /// assert_eq!(result1, 25);
    /// 
    /// // Second call returns cached value
    /// let result2 = cache.get_or_compute(5, |&n| panic!("Should not compute!"));
    /// assert_eq!(result2, 25);
    /// 
    /// assert_eq!(cache.hit_count(), 1);
    /// assert_eq!(cache.miss_count(), 1);
    /// ```
    pub fn get_or_compute<F>(&mut self, key: K, compute_fn: F) -> V
    where
        F: FnOnce(&K) -> V,
    {
        if let Some(value) = self.cache.get(&key) {
            self.hits += 1;
            return value.clone();
        }
        
        // This is a miss - we need to compute
        self.misses += 1;
        
        let value = compute_fn(&key);
        self.insert(key, value.clone());
        value
    }

    /// Get value or insert default if missing
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::MemoCache;
    /// 
    /// let mut cache = MemoCache::new();
    /// cache.insert("existing", 42);
    /// 
    /// assert_eq!(cache.get_or_default("existing", 0), 42);
    /// assert_eq!(cache.get_or_default("missing", 0), 0);
    /// assert!(cache.contains_key(&"missing")); // Default was inserted
    /// ```
    pub fn get_or_default(&mut self, key: K, default: V) -> V {
        self.get_or_compute(key, |_| default)
    }
}

// Specialized methods for numeric caches (common in DP)
impl<K> MemoCache<K, i64>
where
    K: Eq + Hash + Clone,
{
    /// Get cached integer or compute using function
    /// 
    /// Convenience method for integer-based dynamic programming
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::MemoCache;
    /// 
    /// let mut fib_cache = MemoCache::new();
    /// 
    /// // Base cases
    /// fib_cache.insert(0, 0);
    /// fib_cache.insert(1, 1);
    /// 
    /// // Compute fibonacci(5) with memoization
    /// let fib5 = fib_cache.get_or_compute_int(5, |&n| {
    ///     // This would be called recursively in real implementation
    ///     if n <= 1 { n } else { 5 } // Simplified for example
    /// });
    /// ```
    pub fn get_or_compute_int<F>(&mut self, key: K, compute_fn: F) -> i64
    where
        F: FnOnce(&K) -> i64,
    {
        self.get_or_compute(key, compute_fn)
    }
}

impl<K, V> Default for MemoCache<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> FromIterator<(K, V)> for MemoCache<K, V>
where
    K: Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut cache = MemoCache::new();
        for (key, value) in iter {
            cache.insert(key, value);
        }
        cache
    }
}

impl<K, V> Extend<(K, V)> for MemoCache<K, V>
where
    K: Eq + Hash,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut cache = MemoCache::new();
        
        cache.insert("key1", "value1");
        assert_eq!(cache.get(&"key1"), Some(&"value1"));
        assert_eq!(cache.get(&"missing"), None);
        
        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());
        assert!(cache.contains_key(&"key1"));
    }

    #[test]
    fn test_cache_statistics() {
        let mut cache = MemoCache::new();
        cache.insert("key", 42);
        
        // Test hits and misses
        cache.get(&"key");        // hit
        cache.get(&"missing1");   // miss
        cache.get(&"key");        // hit
        cache.get(&"missing2");   // miss
        
        assert_eq!(cache.hit_count(), 2);
        assert_eq!(cache.miss_count(), 2);
        assert_eq!(cache.hit_ratio(), 0.5);
    }

    #[test]
    fn test_get_or_compute() {
        let mut cache = MemoCache::new();
        let computation_count = std::cell::RefCell::new(0);
        
        // First call should compute
        let result1 = cache.get_or_compute(5, |&n| {
            *computation_count.borrow_mut() += 1;
            n * n
        });
        assert_eq!(result1, 25);
        assert_eq!(*computation_count.borrow(), 1);
        
        // Second call should use cache
        let result2 = cache.get_or_compute(5, |&n| {
            *computation_count.borrow_mut() += 1;
            n * n
        });
        assert_eq!(result2, 25);
        assert_eq!(*computation_count.borrow(), 1); // Shouldn't increment
        
        assert_eq!(cache.hit_count(), 1);
        assert_eq!(cache.miss_count(), 1);
    }

    #[test]
    fn test_get_or_default() {
        let mut cache = MemoCache::new();
        cache.insert("existing", 42);
        
        assert_eq!(cache.get_or_default("existing", 0), 42);
        assert_eq!(cache.get_or_default("missing", 99), 99);
        
        // Default should be inserted
        assert!(cache.contains_key(&"missing"));
        assert_eq!(cache.peek(&"missing"), Some(&99));
    }

    #[test]
    fn test_fibonacci_memoization() {
        let mut cache = MemoCache::new();
        
        // Base cases
        cache.insert(0, 0);
        cache.insert(1, 1);
        
        // Compute fibonacci numbers with memoization
        for n in 2..=10 {
            let fib_n = cache.get_or_compute_int(n, |&num| {
                // In real implementation, this would be recursive
                // For testing, we'll use the mathematical formula
                if num <= 1 { 
                    num 
                } else {
                    // Simplified - in reality would use cache.get() for previous values
                    match num {
                        2 => 1, 3 => 2, 4 => 3, 5 => 5,
                        6 => 8, 7 => 13, 8 => 21, 9 => 34, 10 => 55,
                        _ => 0,
                    }
                }
            });
            
            // Verify some known fibonacci numbers
            match n {
                5 => assert_eq!(fib_n, 5),
                10 => assert_eq!(fib_n, 55),
                _ => {}
            }
        }
        
        assert_eq!(cache.len(), 11); // 0 through 10
    }

    #[test]
    fn test_clear_and_reset() {
        let mut cache = MemoCache::new();
        cache.insert("key", "value");
        
        // Generate some statistics
        cache.get(&"key");
        cache.get(&"missing");
        
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.hit_count(), 1);
        assert_eq!(cache.miss_count(), 1);
        
        cache.clear();
        
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.hit_count(), 0);
        assert_eq!(cache.miss_count(), 0);
        assert_eq!(cache.hit_ratio(), 0.0);
    }

    #[test]
    fn test_peek_no_stats() {
        let mut cache = MemoCache::new();
        cache.insert("key", "value");
        
        // Peek shouldn't affect statistics
        assert_eq!(cache.peek(&"key"), Some(&"value"));
        assert_eq!(cache.peek(&"missing"), None);
        
        assert_eq!(cache.hit_count(), 0);
        assert_eq!(cache.miss_count(), 0);
    }

    #[test]
    fn test_from_iterator() {
        let items = vec![("a", 1), ("b", 2), ("c", 3)];
        let cache: MemoCache<&str, i32> = items.into_iter().collect();
        
        assert_eq!(cache.len(), 3);
        assert_eq!(cache.peek(&"b"), Some(&2));
    }
}