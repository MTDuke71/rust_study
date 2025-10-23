use std::collections::HashMap;
use std::hash::Hash;

/// Enhanced dictionary wrapper around `HashMap<K, V>` with convenient operations
///
/// Provides additional functionality beyond standard HashMap including:
/// - Default value handling
/// - Bulk insertion operations  
/// - Multi-value dictionary support
/// - AoC-friendly coordinate mapping
///
/// # Requirements Satisfied: REQ-1, REQ-4, REQ-6
///
/// # Examples
///
/// ```rust
/// use mission5::Dictionary;
///
/// let mut dict = Dictionary::new();
/// dict.insert("language", "Rust");
/// dict.insert("year", "2010");
///
/// assert_eq!(dict.get(&"language"), Some(&"Rust"));
/// assert_eq!(dict.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct Dictionary<K, V>
where
    K: Eq + Hash,
{
    inner: HashMap<K, V>,
}

impl<K, V> Dictionary<K, V>
where
    K: Eq + Hash,
{
    /// Create a new empty dictionary
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::Dictionary;
    ///
    /// let dict: Dictionary<String, i32> = Dictionary::new();
    /// assert!(dict.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Create a dictionary with specified capacity
    ///
    /// Pre-allocates space for at least `capacity` elements
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
        }
    }

    /// Insert a key-value pair
    ///
    /// Returns the previous value if key existed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::Dictionary;
    ///
    /// let mut dict = Dictionary::new();
    /// assert_eq!(dict.insert("key", "value1"), None);
    /// assert_eq!(dict.insert("key", "value2"), Some("value1"));
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    /// Get a reference to the value for a key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::Dictionary;
    ///
    /// let mut dict = Dictionary::new();
    /// dict.insert("key", "value");
    ///
    /// assert_eq!(dict.get(&"key"), Some(&"value"));
    /// assert_eq!(dict.get(&"missing"), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    /// Get a mutable reference to the value for a key
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.inner.get_mut(key)
    }

    /// Remove a key-value pair
    ///
    /// Returns the value if key existed
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }

    /// Check if dictionary contains a key
    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    /// Get number of key-value pairs
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if dictionary is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clear all key-value pairs
    pub fn clear(&mut self) {
        self.inner.clear()
    }

    /// Get an iterator over key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.inner.iter()
    }

    /// Get an iterator over keys
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.inner.keys()
    }

    /// Get an iterator over values
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.inner.values()
    }
}

impl<K, V> Dictionary<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Get value or return default if key doesn't exist
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::Dictionary;
    ///
    /// let mut dict = Dictionary::new();
    /// dict.insert("count", 5);
    ///
    /// assert_eq!(dict.get_or_default(&"count", 0), 5);
    /// assert_eq!(dict.get_or_default(&"missing", 0), 0);
    /// ```
    pub fn get_or_default(&self, key: &K, default: V) -> V {
        self.inner.get(key).cloned().unwrap_or(default)
    }

    /// Insert multiple key-value pairs from an iterator
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::Dictionary;
    ///
    /// let mut dict = Dictionary::new();
    /// dict.insert_many([("a", 1), ("b", 2), ("c", 3)]);
    ///
    /// assert_eq!(dict.len(), 3);
    /// assert_eq!(dict.get(&"b"), Some(&2));
    /// ```
    pub fn insert_many<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        for (key, value) in items {
            self.insert(key, value);
        }
    }
}

// Multi-value dictionary support (REQ-4)
impl<K> Dictionary<K, Vec<String>>
where
    K: Eq + Hash + Clone,
{
    /// Add a value to a list stored at the given key
    ///
    /// Creates the list if it doesn't exist
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mission5::Dictionary;
    ///
    /// let mut dict: Dictionary<&str, Vec<String>> = Dictionary::new();
    /// dict.add_to_list("fruits", "apple");
    /// dict.add_to_list("fruits", "banana");
    ///
    /// let fruits = dict.get_list(&"fruits");
    /// assert_eq!(fruits.len(), 2);
    /// assert!(fruits.contains(&"apple".to_string()));
    /// ```
    pub fn add_to_list(&mut self, key: K, value: &str) {
        self.inner.entry(key).or_default().push(value.to_string());
    }

    /// Get the list of values for a key
    ///
    /// Returns empty list if key doesn't exist
    pub fn get_list(&self, key: &K) -> Vec<String> {
        self.inner.get(key).cloned().unwrap_or_default()
    }

    /// Get the number of values in the list for a key
    pub fn list_len(&self, key: &K) -> usize {
        self.inner.get(key).map_or(0, |v| v.len())
    }
}

impl<K, V> Default for Dictionary<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> FromIterator<(K, V)> for Dictionary<K, V>
where
    K: Eq + Hash,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut dict = Dictionary::new();
        for (key, value) in iter {
            dict.insert(key, value);
        }
        dict
    }
}

impl<K, V> IntoIterator for Dictionary<K, V>
where
    K: Eq + Hash,
{
    type Item = (K, V);
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<K, V> Extend<(K, V)> for Dictionary<K, V>
where
    K: Eq + Hash,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.inner.extend(iter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut dict = Dictionary::new();

        // Insert and get
        dict.insert("key1".to_string(), "value1".to_string());
        assert_eq!(dict.get(&"key1".to_string()), Some(&"value1".to_string()));
        assert_eq!(dict.get(&"missing".to_string()), None);

        // Update
        dict.insert("key1".to_string(), "updated".to_string());
        assert_eq!(dict.get(&"key1".to_string()), Some(&"updated".to_string()));

        // Size
        assert_eq!(dict.len(), 1);
        assert!(!dict.is_empty());
    }

    #[test]
    fn test_default_values() {
        let mut dict = Dictionary::new();
        dict.insert("existing", 42);

        assert_eq!(dict.get_or_default(&"existing", 0), 42);
        assert_eq!(dict.get_or_default(&"missing", 0), 0);
    }

    #[test]
    fn test_bulk_operations() {
        let mut dict = Dictionary::new();
        dict.insert_many([("a", 1), ("b", 2), ("c", 3)]);

        assert_eq!(dict.len(), 3);
        assert_eq!(dict.get(&"b"), Some(&2));
    }

    #[test]
    fn test_multi_value_lists() {
        let mut dict: Dictionary<&str, Vec<String>> = Dictionary::new();

        dict.add_to_list("group1", "item1");
        dict.add_to_list("group1", "item2");
        dict.add_to_list("group2", "item3");

        let group1 = dict.get_list(&"group1");
        assert_eq!(group1.len(), 2);
        assert!(group1.contains(&"item1".to_string()));
        assert!(group1.contains(&"item2".to_string()));

        let group2 = dict.get_list(&"group2");
        assert_eq!(group2.len(), 1);
        assert!(group2.contains(&"item3".to_string()));

        let empty = dict.get_list(&"missing");
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn test_iteration() {
        let dict: Dictionary<&str, i32> = [("a", 1), ("b", 2), ("c", 3)].into_iter().collect();

        let keys: Vec<_> = dict.keys().collect();
        assert_eq!(keys.len(), 3);

        let values: Vec<_> = dict.values().collect();
        assert_eq!(values.len(), 3);

        for (key, value) in dict.iter() {
            match *key {
                "a" => assert_eq!(*value, 1),
                "b" => assert_eq!(*value, 2),
                "c" => assert_eq!(*value, 3),
                _ => panic!("Unexpected key: {}", key),
            }
        }
    }

    #[test]
    fn test_coordinate_mapping() {
        let mut grid = Dictionary::new();

        // Store grid positions
        grid.insert((0, 0), '#');
        grid.insert((0, 1), '.');
        grid.insert((1, 0), 'S');
        grid.insert((1, 1), 'E');

        assert_eq!(grid.get(&(0, 0)), Some(&'#'));
        assert_eq!(grid.get(&(1, 1)), Some(&'E'));
        assert_eq!(grid.get(&(2, 2)), None);

        // Find all walls
        let walls: Vec<_> = grid
            .iter()
            .filter(|(_, &cell)| cell == '#')
            .map(|(pos, _)| *pos)
            .collect();

        assert_eq!(walls.len(), 1);
        assert_eq!(walls[0], (0, 0));
    }
}
