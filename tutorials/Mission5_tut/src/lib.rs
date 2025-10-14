//! # Mission 5 Tutorial: HashMap & HashSet Mastery
//!
//! **A progressive learning experience in Rust hash-based data structures**
//!
//! This tutorial teaches you everything you need to know about HashMap and HashSet
//! through hands-on examples, exercises, and real-world applications.
//!
//! ## 🎯 Learning Progression
//!
//! ### Step 1: Basic HashMap Operations
//! - Creating and manipulating HashMaps
//! - Safe access patterns and error handling
//! - Working with different data types
//! - Basic iteration and updates
//!
//! ### Step 2: HashSet Operations  
//! - Membership testing and uniqueness
//! - Set theory operations (union, intersection, difference)
//! - Deduplication patterns
//! - Performance benefits over Vec
//!
//! ### Step 3: Frequency Counting
//! - Word and character frequency analysis
//! - Building reusable frequency counters
//! - Statistical analysis with HashMap
//! - N-gram analysis for text processing
//!
//! ### Step 4: Multi-Value Patterns
//! - One-to-many relationships with HashMap\<K, Vec\<V\>\>
//! - Graph adjacency lists
//! - Data grouping and categorization
//! - Reverse indexing for search
//!
//! ### Step 5: Memoization and Caching
//! - Function result caching for performance
//! - LRU cache implementation
//! - Dynamic programming with HashMap
//! - Cache hit/miss ratio optimization
//!
//! ### Final Project: Comprehensive System
//! - Text analysis engine with caching
//! - Game world coordinate system
//! - Real-world application patterns
//! - Performance optimization techniques
//!
//! ## 🚀 Getting Started
//!
//! Run the tutorial steps in order:
//!
//! ```bash
//! cargo run --example step1_basic_hashmap
//! cargo run --example step2_hashset_operations  
//! cargo run --example step3_frequency_counting
//! cargo run --example step4_multi_value_patterns
//! cargo run --example step5_memoization_cache
//! cargo run --example final_project
//! ```
//!
//! ## 📚 Key Patterns You'll Learn
//!
//! ### HashMap Usage Patterns
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! // Basic operations
//! let mut scores = HashMap::new();
//! scores.insert("Alice", 100);
//! let alice_score = scores.get("Alice").unwrap_or(&0);
//!
//! // Frequency counting
//! let text = "hello world hello rust";
//! let mut word_count = HashMap::new();
//! for word in text.split_whitespace() {
//!     *word_count.entry(word).or_insert(0) += 1;
//! }
//!
//! // Multi-value relationships  
//! let mut groups: HashMap<String, Vec<String>> = HashMap::new();
//! groups.entry("category".to_string()).or_insert(Vec::new()).push("item".to_string());
//!
//! // Memoization
//! let mut cache = HashMap::new();
//! let input = "test_input";
//! let result = cache.entry(input).or_insert_with(|| input.len());
//! ```
//!
//! ### HashSet Usage Patterns
//!
//! ```rust
//! use std::collections::HashSet;
//!
//! // Deduplication
//! let data = vec![1, 2, 2, 3, 3, 3];
//! let unique_items: HashSet<_> = data.into_iter().collect();
//!
//! // Set operations
//! let set1: HashSet<_> = [1, 2, 3].iter().cloned().collect();
//! let set2: HashSet<_> = [2, 3, 4].iter().cloned().collect();
//! let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
//! let union: HashSet<_> = set1.union(&set2).cloned().collect();
//! let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
//!
//! // Membership testing (O(1) average)
//! let blacklist: HashSet<_> = ["bad", "evil"].iter().cloned().collect();
//! let item = "bad";
//! if blacklist.contains(&item) {
//!     // Handle blacklisted item
//! }
//! ```
//!
//! ## 🎄 Advent of Code Applications
//!
//! These patterns are essential for competitive programming:
//!
//! - **Coordinate Systems**: `HashMap<(i32, i32), CellType>` for grid problems
//! - **State Tracking**: `HashSet<GameState>` for visited states  
//! - **Frequency Analysis**: `HashMap<Item, Count>` for occurrence counting
//! - **Graph Algorithms**: `HashMap<Node, Vec<Node>>` for adjacency lists
//! - **Memoization**: `HashMap<Input, Output>` for dynamic programming
//!
//! ## 🏗️ Production-Ready Patterns
//!
//! The tutorial emphasizes patterns you'll use in real applications:
//!
//! - Error handling with `Option` and proper unwrapping
//! - Performance optimization through caching
//! - Memory-efficient data structures
//! - Scalable architecture design
//!
//! ## ⚡ Performance Considerations
//!
//! Learn when and why to choose each data structure:
//!
//! - HashMap: O(1) average lookup, ideal for key-value relationships
//! - HashSet: O(1) average membership testing, ideal for uniqueness
//! - Caching: Trade memory for computation time
//! - Batch operations: Minimize hash table resizing
//!
//! ## 🎯 Success Metrics
//!
//! You'll know you've mastered this tutorial when you can:
//!
//! 1. **Build frequency analyzers** for any type of data
//! 2. **Implement memoization** to optimize recursive algorithms  
//! 3. **Design coordinate systems** for 2D and 3D problems
//! 4. **Create efficient search indices** for text and data
//! 5. **Optimize performance** using appropriate caching strategies
//!
//! Start your journey with Step 1 and work through each example and exercise.
//! Each step builds on the previous, creating a comprehensive understanding
//! of hash-based data structures in Rust.

// Re-export key types for convenience
pub use std::collections::{HashMap, HashSet};

/// Common type aliases for frequent patterns
pub type FrequencyMap<T> = HashMap<T, usize>;
pub type AdjacencyList<T> = HashMap<T, Vec<T>>;
pub type CoordinateMap<T> = HashMap<(i32, i32), T>;
pub type Cache<K, V> = HashMap<K, V>;

/// Utility functions for common operations
pub mod utils {
    use super::*;

    /// Create a frequency map from an iterator
    pub fn frequency_map<T, I>(items: I) -> FrequencyMap<T>
    where
        T: std::hash::Hash + Eq,
        I: IntoIterator<Item = T>,
    {
        let mut freq_map = HashMap::new();
        for item in items {
            *freq_map.entry(item).or_insert(0) += 1;
        }
        freq_map
    }

    /// Remove duplicates from a vector using HashSet
    pub fn deduplicate<T>(items: Vec<T>) -> Vec<T>
    where
        T: std::hash::Hash + Eq + Clone,
    {
        let unique_set: HashSet<_> = items.into_iter().collect();
        unique_set.into_iter().collect()
    }

    /// Find intersection of multiple HashSets
    pub fn intersect_all<T>(sets: Vec<&HashSet<T>>) -> HashSet<T>
    where
        T: std::hash::Hash + Eq + Clone,
    {
        if sets.is_empty() {
            return HashSet::new();
        }

        let mut result = sets[0].clone();
        for set in sets.into_iter().skip(1) {
            result = result.intersection(set).cloned().collect();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_map() {
        let items = vec!["a", "b", "a", "c", "b", "a"];
        let freq = utils::frequency_map(items);

        assert_eq!(freq.get("a"), Some(&3));
        assert_eq!(freq.get("b"), Some(&2));
        assert_eq!(freq.get("c"), Some(&1));
    }

    #[test]
    fn test_deduplicate() {
        let items = vec![1, 2, 2, 3, 1, 4];
        let unique = utils::deduplicate(items);

        assert_eq!(unique.len(), 4);
        assert!(unique.contains(&1));
        assert!(unique.contains(&2));
        assert!(unique.contains(&3));
        assert!(unique.contains(&4));
    }

    #[test]
    fn test_intersect_all() {
        let set1: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
        let set2: HashSet<i32> = vec![2, 3, 4, 5].into_iter().collect();
        let set3: HashSet<i32> = vec![3, 4, 5, 6].into_iter().collect();

        let intersection = utils::intersect_all(vec![&set1, &set2, &set3]);

        assert_eq!(intersection.len(), 2);
        assert!(intersection.contains(&3));
        assert!(intersection.contains(&4));
    }
}
