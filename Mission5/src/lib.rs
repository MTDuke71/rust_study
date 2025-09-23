//! # Mission 5: HashMaps & HashSets - Dictionary Data Structures
//!
//! This library implements hash-based data structures essential for competitive programming
//! and algorithm development. We focus on dictionary patterns, set operations, frequency
//! counting, and memoization - all critical for Advent of Code and similar challenges.
//!
//! ## Requirements Fulfilled
//!
//! - **REQ-1**: Enhanced dictionary wrapper around `HashMap<K, V>`
//! - **REQ-2**: Set operations using `HashSet<T>` for membership and deduplication
//! - **REQ-3**: Efficient counting patterns for frequency analysis
//! - **REQ-4**: Multi-value dictionaries for one-to-many relationships
//! - **REQ-5**: Caching and memoization patterns for dynamic programming
//! - **REQ-6**: AoC-specific utilities for competitive programming
//!
//! ## Quick Start
//!
//! ```rust
//! use mission5::{Dictionary, Counter, SetOperations, MemoCache};
//!
//! // Dictionary operations
//! let mut dict = Dictionary::new();
//! dict.insert("language", "Rust");
//! dict.insert("year", "2010");
//! assert_eq!(dict.get(&"language"), Some(&"Rust"));
//!
//! // Frequency counting
//! let mut counter = Counter::new();
//! let items = vec!["apple".to_string(), "banana".to_string(), "apple".to_string()];
//! counter.count_multiple(items);
//! assert_eq!(counter.get(&"apple".to_string()), 2);
//!
//! // Set operations
//! let set1 = SetOperations::from_iter([1, 2, 3]);
//! let set2 = SetOperations::from_iter([2, 3, 4]);
//! let intersection = set1.intersection(&set2);
//! assert_eq!(intersection.len(), 2); // {2, 3}
//! ```
//!
//! ## Why Hash Structures are Critical for AoC
//!
//! Hash-based data structures solve common competitive programming patterns:
//!
//! 1. **Fast Lookups**: O(1) membership testing and value retrieval
//! 2. **Frequency Analysis**: Count occurrences of elements efficiently
//! 3. **State Tracking**: Deduplicate visited states in search algorithms
//! 4. **Memoization**: Cache expensive computations in recursive algorithms
//! 5. **Coordinate Maps**: Store grid states with (x,y) coordinate keys
//!
//! ## Performance Characteristics
//!
//! | Operation | Time Complexity | Space Complexity |
//! |-----------|----------------|------------------|
//! | `insert` | O(1) average | O(n) |
//! | `get` | O(1) average | O(1) |
//! | `remove` | O(1) average | O(1) |
//! | `union` | O(n + m) | O(n + m) |
//! | `intersection` | O(min(n,m)) | O(min(n,m)) |
//!
//! ## Use Cases
//!
//! - **Competitive Programming**: Fast lookups and frequency counting
//! - **AoC Problems**: Grid navigation, state tracking, memoization
//! - **Algorithm Development**: Set operations and data deduplication
//! - **Performance Optimization**: Caching expensive computations

pub mod dictionary;
pub mod counter;
pub mod set_operations;
pub mod memo_cache;

// Re-export main types for convenient access
pub use dictionary::Dictionary;
pub use counter::Counter;
pub use set_operations::SetOperations;
pub use memo_cache::MemoCache;

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // REQ-1: Enhanced dictionary wrapper around HashMap<K, V>
    fn req1_dictionary_basic_operations() {
        let mut dict = Dictionary::new();
        
        // Basic insertion and retrieval with String keys
        dict.insert("name".to_string(), "Rust".to_string());
        dict.insert("year".to_string(), "2010".to_string());
        
        assert_eq!(dict.get(&"name".to_string()), Some(&"Rust".to_string()));
        assert_eq!(dict.get(&"year".to_string()), Some(&"2010".to_string()));
        assert_eq!(dict.get(&"missing".to_string()), None);
        
        // Update existing key
        dict.insert("year".to_string(), "2015".to_string());
        assert_eq!(dict.get(&"year".to_string()), Some(&"2015".to_string()));
        
        // Size and emptiness
        assert_eq!(dict.len(), 2);
        assert!(!dict.is_empty());
    }

    #[test] // REQ-2: Set operations using HashSet<T>
    fn req2_set_operations_basic() {
        let set1 = SetOperations::from_iter([1, 2, 3, 4]);
        let set2 = SetOperations::from_iter([3, 4, 5, 6]);
        
        // Union: all elements from both sets
        let union = set1.union(&set2);
        assert_eq!(union.len(), 6);
        assert!(union.contains(&1));
        assert!(union.contains(&6));
        
        // Intersection: common elements
        let intersection = set1.intersection(&set2);
        assert_eq!(intersection.len(), 2);
        assert!(intersection.contains(&3));
        assert!(intersection.contains(&4));
        
        // Difference: elements in first but not second
        let difference = set1.difference(&set2);
        assert_eq!(difference.len(), 2);
        assert!(difference.contains(&1));
        assert!(difference.contains(&2));
    }

    #[test] // REQ-3: Efficient counting patterns
    fn req3_frequency_counting() {
        let mut counter = Counter::new();
        
        // Count individual items using integers for simplicity
        counter.increment(1);
        counter.increment(2);
        counter.increment(1);
        
        assert_eq!(counter.get(&1), 2);
        assert_eq!(counter.get(&2), 1);
        assert_eq!(counter.get(&3), 0);
        
        // Batch counting
        let items = vec![1, 3, 1];
        for item in items {
            counter.increment(item);
        }
        assert_eq!(counter.get(&1), 4);
        assert_eq!(counter.get(&3), 1);
        
        // Total count
        assert_eq!(counter.total_count(), 6);
    }

    #[test] // REQ-4: Multi-value dictionaries
    fn req4_multi_value_dictionary() {
        let mut dict: Dictionary<String, Vec<String>> = Dictionary::new();
        
        // Add multiple values to same key
        dict.add_to_list("fruits".to_string(), "apple");
        dict.add_to_list("fruits".to_string(), "banana");
        dict.add_to_list("fruits".to_string(), "cherry");
        
        dict.add_to_list("colors".to_string(), "red");
        dict.add_to_list("colors".to_string(), "blue");
        
        let fruits = dict.get_list(&"fruits".to_string());
        assert_eq!(fruits.len(), 3);
        assert!(fruits.contains(&"apple".to_string()));
        assert!(fruits.contains(&"banana".to_string()));
        assert!(fruits.contains(&"cherry".to_string()));
        
        let colors = dict.get_list(&"colors".to_string());
        assert_eq!(colors.len(), 2);
        assert!(colors.contains(&"red".to_string()));
        assert!(colors.contains(&"blue".to_string()));
        
        // Non-existent key returns empty list
        let empty = dict.get_list(&"vegetables".to_string());
        assert_eq!(empty.len(), 0);
    }

    #[test] // REQ-5: Caching and memoization patterns
    fn req5_memoization_cache() {
        let mut cache = MemoCache::new();
        
        // Cache miss and insertion
        assert_eq!(cache.get(&"key1"), None);
        cache.insert("key1", "value1");
        assert_eq!(cache.get(&"key1"), Some(&"value1"));
        
        // Cache hit
        assert_eq!(cache.get(&"key1"), Some(&"value1"));
        
        // Multiple keys
        cache.insert("key2", "value2");
        cache.insert("key3", "value3");
        
        assert_eq!(cache.len(), 3);
        
        // Cache invalidation
        cache.remove(&"key2");
        assert_eq!(cache.get(&"key2"), None);
        assert_eq!(cache.len(), 2);
    }

    #[test] // REQ-6: AoC-specific utilities
    fn req6_coordinate_mapping() {
        let mut grid = Dictionary::new();
        
        // Store grid cell states using coordinate keys
        grid.insert((0, 0), '#');  // Wall
        grid.insert((0, 1), '.');  // Empty
        grid.insert((1, 0), 'S');  // Start
        grid.insert((1, 1), 'E');  // End
        
        // Query grid state
        assert_eq!(grid.get(&(0, 0)), Some(&'#'));
        assert_eq!(grid.get(&(1, 1)), Some(&'E'));
        assert_eq!(grid.get(&(2, 2)), None);
        
        // Count specific cell types
        let wall_positions: Vec<_> = grid.iter()
            .filter(|(_, &cell)| cell == '#')
            .map(|(pos, _)| *pos)
            .collect();
        
        assert_eq!(wall_positions.len(), 1);
        assert_eq!(wall_positions[0], (0, 0));
    }
}