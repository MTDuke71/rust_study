//! # HashSet Source Code Proof
//! 
//! This shows what the actual Rust HashSet implementation looks like
//! (Simplified version of the real std::collections::HashSet)

use std::collections::HashMap;
use std::hash::Hash;

// This is conceptually how HashSet is implemented in Rust's standard library
pub struct MyHashSet<T> {
    map: HashMap<T, ()>,  // The () is the key insight!
}

impl<T> MyHashSet<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Self {
        MyHashSet {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        self.map.insert(value, ()).is_none()
    }

    pub fn contains(&self, value: &T) -> bool {
        self.map.contains_key(value)
    }

    pub fn remove(&mut self, value: &T) -> bool {
        self.map.remove(value).is_some()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

fn main() {
    println!("🔬 HashSet Implementation Proof");
    println!("===============================\n");

    // Our implementation that mirrors the real HashSet
    let mut my_set = MyHashSet::new();
    my_set.insert("apple");
    my_set.insert("banana");
    my_set.insert("apple"); // Duplicate

    println!("MyHashSet contents:");
    println!("  Length: {}", my_set.len());
    println!("  Contains 'apple': {}", my_set.contains(&"apple"));
    println!("  Contains 'cherry': {}", my_set.contains(&"cherry"));

    // Compare with real HashSet
    let mut real_set = std::collections::HashSet::new();
    real_set.insert("apple");
    real_set.insert("banana");
    real_set.insert("apple"); // Duplicate

    println!("\nStd HashSet contents:");
    println!("  Length: {}", real_set.len());
    println!("  Contains 'apple': {}", real_set.contains(&"apple"));
    println!("  Contains 'cherry': {}", real_set.contains(&"cherry"));

    println!("\n🎯 Key Insights:");
    println!("  1. HashSet<T> = HashMap<T, ()>");
    println!("  2. () is the unit type - takes 0 bytes");
    println!("  3. All the logic is in the HashMap implementation");
    println!("  4. HashSet just provides a set-focused API");
    println!("  5. Same performance characteristics as HashMap");
    
    // Show the memory efficiency
    println!("\n🧠 Memory Efficiency:");
    println!("  Size of (): {} bytes", std::mem::size_of::<()>());
    println!("  → No memory wasted on 'empty' values!");
}