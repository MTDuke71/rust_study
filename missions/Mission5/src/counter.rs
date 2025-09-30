use std::collections::HashMap;
use std::hash::Hash;

/// Frequency counter for tracking occurrences of items
/// 
/// Specialized HashMap wrapper optimized for counting operations.
/// Provides statistical analysis methods and efficient increment operations.
/// 
/// # Requirements Satisfied: REQ-3
/// 
/// # Examples
/// 
/// ```rust
/// use mission5::Counter;
/// 
/// let mut counter = Counter::new();
/// let items = vec!["apple".to_string(), "banana".to_string(), "apple".to_string(), "cherry".to_string()];
/// counter.count_multiple(items);
/// 
/// assert_eq!(counter.get(&"apple".to_string()), 2);
/// assert_eq!(counter.get(&"banana".to_string()), 1);
/// assert_eq!(counter.total_count(), 4);
/// ```
#[derive(Debug, Clone)]
pub struct Counter<K>
where
    K: Eq + Hash,
{
    counts: HashMap<K, usize>,
    total: usize,
}

impl<K> Counter<K>
where
    K: Eq + Hash,
{
    /// Create a new empty counter
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::Counter;
    /// 
    /// let counter: Counter<String> = Counter::new();
    /// assert_eq!(counter.total_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
            total: 0,
        }
    }

    /// Create a counter with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            counts: HashMap::with_capacity(capacity),
            total: 0,
        }
    }

    /// Increment count for a key by 1
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::Counter;
    /// 
    /// let mut counter = Counter::new();
    /// counter.increment("apple");
    /// counter.increment("apple");
    /// 
    /// assert_eq!(counter.get(&"apple"), 2);
    /// ```
    pub fn increment(&mut self, key: K) {
        self.increment_by(key, 1);
    }

    /// Increment count for a key by specified amount
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::Counter;
    /// 
    /// let mut counter = Counter::new();
    /// counter.increment_by("score", 10);
    /// counter.increment_by("score", 5);
    /// 
    /// assert_eq!(counter.get(&"score"), 15);
    /// ```
    pub fn increment_by(&mut self, key: K, amount: usize) {
        *self.counts.entry(key).or_insert(0) += amount;
        self.total += amount;
    }

    /// Get count for a key (returns 0 if key doesn't exist)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::Counter;
    /// 
    /// let mut counter = Counter::new();
    /// counter.increment("existing");
    /// 
    /// assert_eq!(counter.get(&"existing"), 1);
    /// assert_eq!(counter.get(&"missing"), 0);
    /// ```
    pub fn get(&self, key: &K) -> usize {
        self.counts.get(key).copied().unwrap_or(0)
    }

    /// Set count for a key to specific value
    pub fn set(&mut self, key: K, count: usize) {
        let old_count = self.counts.insert(key, count).unwrap_or(0);
        self.total = self.total - old_count + count;
    }

    /// Remove a key and its count
    pub fn remove(&mut self, key: &K) -> usize {
        if let Some(count) = self.counts.remove(key) {
            self.total -= count;
            count
        } else {
            0
        }
    }

    /// Get total count across all keys
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::Counter;
    /// 
    /// let mut counter = Counter::new();
    /// counter.increment_by("a", 5);
    /// counter.increment_by("b", 3);
    /// 
    /// assert_eq!(counter.total_count(), 8);
    /// ```
    pub fn total_count(&self) -> usize {
        self.total
    }

    /// Get number of unique keys
    pub fn unique_count(&self) -> usize {
        self.counts.len()
    }

    /// Check if counter is empty
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    /// Clear all counts
    pub fn clear(&mut self) {
        self.counts.clear();
        self.total = 0;
    }

    /// Check if counter contains a key
    pub fn contains(&self, key: &K) -> bool {
        self.counts.contains_key(key)
    }

    /// Get an iterator over (key, count) pairs
    pub fn iter(&self) -> impl Iterator<Item = (&K, &usize)> {
        self.counts.iter()
    }

    /// Get an iterator over keys
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.counts.keys()
    }

    /// Get an iterator over counts
    pub fn values(&self) -> impl Iterator<Item = &usize> {
        self.counts.values()
    }
}

impl<K> Counter<K>
where
    K: Eq + Hash + Clone,
{
    /// Count multiple items from an iterator
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::Counter;
    /// 
    /// let mut counter = Counter::new();
    /// let items = vec!["apple".to_string(), "banana".to_string(), "apple".to_string()];
    /// counter.count_multiple(items);
    /// 
    /// assert_eq!(counter.get(&"apple".to_string()), 2);
    /// assert_eq!(counter.get(&"banana".to_string()), 1);
    /// ```
    pub fn count_multiple<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = K>,
    {
        for item in items {
            self.increment(item);
        }
    }

    /// Get the most common items with their counts
    /// 
    /// Returns items sorted by count (descending)
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::Counter;
    /// 
    /// let mut counter = Counter::new();
    /// counter.count_multiple(&["a", "b", "a", "c", "a", "b"]);
    /// 
    /// let most_common = counter.most_common(2);
    /// assert_eq!(most_common[0].1, 3); // "a" appears 3 times
    /// assert_eq!(most_common[1].1, 2); // "b" appears 2 times
    /// ```
    pub fn most_common(&self, n: usize) -> Vec<(K, usize)> {
        let mut items: Vec<_> = self.counts.iter()
            .map(|(k, &v)| (k.clone(), v))
            .collect();
        
        items.sort_by(|a, b| b.1.cmp(&a.1));
        items.into_iter().take(n).collect()
    }

    /// Get the least common items with their counts
    pub fn least_common(&self, n: usize) -> Vec<(K, usize)> {
        let mut items: Vec<_> = self.counts.iter()
            .map(|(k, &v)| (k.clone(), v))
            .collect();
        
        items.sort_by(|a, b| a.1.cmp(&b.1));
        items.into_iter().take(n).collect()
    }
}

// String-specific methods for text analysis
impl Counter<String> {
    /// Count words in a text string
    /// 
    /// Splits on whitespace and counts each word
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::Counter;
    /// 
    /// let mut counter = Counter::new();
    /// counter.count_words("hello world hello rust");
    /// 
    /// assert_eq!(counter.get(&"hello".to_string()), 2);
    /// assert_eq!(counter.get(&"world".to_string()), 1);
    /// assert_eq!(counter.get(&"rust".to_string()), 1);
    /// ```
    pub fn count_words(&mut self, text: &str) {
        for word in text.split_whitespace() {
            self.increment(word.to_string());
        }
    }

    /// Count characters in a text string
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mission5::Counter;
    /// 
    /// let mut counter = Counter::new();
    /// counter.count_chars("hello");
    /// 
    /// assert_eq!(counter.get(&"l".to_string()), 2);
    /// assert_eq!(counter.get(&"e".to_string()), 1);
    /// ```
    pub fn count_chars(&mut self, text: &str) {
        for ch in text.chars() {
            self.increment(ch.to_string());
        }
    }
}

impl<K> Default for Counter<K>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K> FromIterator<K> for Counter<K>
where
    K: Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut counter = Counter::new();
        for item in iter {
            counter.increment(item);
        }
        counter
    }
}

impl<K> Extend<K> for Counter<K>
where
    K: Eq + Hash,
{
    fn extend<T: IntoIterator<Item = K>>(&mut self, iter: T) {
        for item in iter {
            self.increment(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_counting() {
        let mut counter = Counter::new();
        
        counter.increment("apple");
        counter.increment("banana");
        counter.increment("apple");
        
        assert_eq!(counter.get(&"apple"), 2);
        assert_eq!(counter.get(&"banana"), 1);
        assert_eq!(counter.get(&"missing"), 0);
        assert_eq!(counter.total_count(), 3);
        assert_eq!(counter.unique_count(), 2);
    }

    #[test]
    fn test_increment_by() {
        let mut counter = Counter::new();
        
        counter.increment_by("score", 10);
        counter.increment_by("score", 5);
        counter.increment_by("lives", 3);
        
        assert_eq!(counter.get(&"score"), 15);
        assert_eq!(counter.get(&"lives"), 3);
        assert_eq!(counter.total_count(), 18);
    }

    #[test]
    fn test_batch_counting() {
        let mut counter = Counter::new();
        let items = vec!["a".to_string(), "b".to_string(), "a".to_string(), "c".to_string(), "a".to_string()];
        counter.count_multiple(items);
        
        assert_eq!(counter.get(&"a".to_string()), 3);
        assert_eq!(counter.get(&"b".to_string()), 1);
        assert_eq!(counter.get(&"c".to_string()), 1);
        assert_eq!(counter.total_count(), 5);
    }

    #[test]
    fn test_most_common() {
        let mut counter = Counter::new();
        let items = vec!["a".to_string(), "b".to_string(), "a".to_string(), "c".to_string(), "a".to_string(), "b".to_string()];
        counter.count_multiple(items);
        
        let most_common = counter.most_common(3);
        assert_eq!(most_common.len(), 3);
        assert_eq!(most_common[0], ("a".to_string(), 3));
        assert_eq!(most_common[1], ("b".to_string(), 2));
        assert_eq!(most_common[2], ("c".to_string(), 1));
    }

    #[test]
    fn test_word_counting() {
        let mut counter = Counter::new();
        counter.count_words("hello world hello rust world");
        
        assert_eq!(counter.get(&"hello".to_string()), 2);
        assert_eq!(counter.get(&"world".to_string()), 2);
        assert_eq!(counter.get(&"rust".to_string()), 1);
        assert_eq!(counter.total_count(), 5);
    }

    #[test]
    fn test_char_counting() {
        let mut counter = Counter::new();
        counter.count_chars("hello");
        
        assert_eq!(counter.get(&"h".to_string()), 1);
        assert_eq!(counter.get(&"e".to_string()), 1);
        assert_eq!(counter.get(&"l".to_string()), 2);
        assert_eq!(counter.get(&"o".to_string()), 1);
        assert_eq!(counter.total_count(), 5);
    }

    #[test]
    fn test_set_and_remove() {
        let mut counter = Counter::new();
        counter.increment("a");
        counter.increment("b");
        
        assert_eq!(counter.total_count(), 2);
        
        counter.set("a", 5);
        assert_eq!(counter.get(&"a"), 5);
        assert_eq!(counter.total_count(), 6);
        
        let removed = counter.remove(&"b");
        assert_eq!(removed, 1);
        assert_eq!(counter.total_count(), 5);
        assert_eq!(counter.get(&"b"), 0);
    }

    #[test]
    fn test_from_iterator() {
        let items = vec!["a", "b", "a", "c", "a"];
        let counter: Counter<&str> = items.into_iter().collect();
        
        assert_eq!(counter.get(&"a"), 3);
        assert_eq!(counter.get(&"b"), 1);
        assert_eq!(counter.get(&"c"), 1);
    }
}