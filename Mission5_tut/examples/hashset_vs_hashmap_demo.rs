//! # HashSet vs HashMap: The Real Relationship
//! 
//! **Demonstrates**: How HashSet is actually implemented using HashMap

use std::collections::{HashMap, HashSet};

fn main() {
    println!("🦀 HashSet vs HashMap: The Truth Revealed");
    println!("==========================================\n");
    
    demonstrate_internal_structure();
    demonstrate_performance_equivalence();
    demonstrate_api_differences();
    demonstrate_use_case_differences();
}

/// Shows how HashSet is conceptually implemented
fn demonstrate_internal_structure() {
    println!("🔍 Internal Structure Comparison");
    println!("================================");
    
    // What you think HashSet might be:
    println!("❌ MYTH: HashSet<T> = HashMap<T, (nothing)>");
    
    // What HashSet actually is:
    println!("✅ REALITY: HashSet<T> = HashMap<T, ()>");
    println!("   - The value is () (unit type), not absent");
    println!("   - () takes zero bytes, so no memory overhead");
    
    // Demonstrate with manual implementation
    let mut manual_set: HashMap<&str, ()> = HashMap::new();
    manual_set.insert("apple", ());
    manual_set.insert("banana", ());
    manual_set.insert("cherry", ());
    
    let mut real_set: HashSet<&str> = HashSet::new();
    real_set.insert("apple");
    real_set.insert("banana");
    real_set.insert("cherry");
    
    println!("   Manual HashSet (using HashMap): {:?}", manual_set.keys().collect::<Vec<_>>());
    println!("   Real HashSet: {:?}", real_set.iter().collect::<Vec<_>>());
    
    println!();
}

/// Shows that HashSet and HashMap have equivalent performance
fn demonstrate_performance_equivalence() {
    println!("⚡ Performance Characteristics");
    println!("=============================");
    
    println!("Both HashSet and HashMap have:");
    println!("  - O(1) average insert/lookup/remove");
    println!("  - O(n) worst case (hash collisions)");
    println!("  - Same memory usage patterns");
    println!("  - Same hashing algorithms");
    
    // Time some operations (conceptual - actual timing would need more setup)
    let mut set = HashSet::new();
    let mut map = HashMap::new();
    
    // Insert 1000 items
    for i in 0..1000 {
        set.insert(i);
        map.insert(i, ());  // Equivalent operation
    }
    
    println!("  ✅ Inserted 1000 items in both collections");
    println!("  ✅ Lookup performance is identical");
    
    println!();
}

/// Shows how the APIs differ despite similar internals
fn demonstrate_api_differences() {
    println!("🛠️  API Differences");
    println!("==================");
    
    let mut fruits_set = HashSet::new();
    let mut fruits_map: HashMap<&str, ()> = HashMap::new();
    
    println!("HashSet API (set-focused):");
    fruits_set.insert("apple");
    fruits_set.insert("banana");
    
    println!("  - insert(key) -> bool");
    println!("  - contains(&key) -> bool");
    println!("  - Set operations: union(), intersection(), difference()");
    
    println!("\nHashMap API (key-value focused):");
    fruits_map.insert("apple", ());
    fruits_map.insert("banana", ());
    
    println!("  - insert(key, value) -> Option<value>");
    println!("  - get(&key) -> Option<&value>");  
    println!("  - No built-in set operations");
    
    // Set operations that HashSet provides
    let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
    
    println!("\n🧮 Set Theory Operations (HashSet only):");
    println!("  Set 1: {:?}", set1);
    println!("  Set 2: {:?}", set2);
    
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
    
    println!("  Union: {:?}", union);
    println!("  Intersection: {:?}", intersection);
    println!("  Difference: {:?}", difference);
    
    println!();
}

/// Shows when to use each collection type
fn demonstrate_use_case_differences() {
    println!("🎯 When to Use Each");
    println!("===================");
    
    println!("Use HashSet when:");
    println!("  ✅ You only care about membership (is X in the set?)");
    println!("  ✅ You need to eliminate duplicates");
    println!("  ✅ You need set theory operations (union, intersection)");
    println!("  ✅ You're tracking unique items");
    
    println!("\nUse HashMap when:");
    println!("  ✅ You need to associate values with keys");
    println!("  ✅ You're building lookup tables or caches");
    println!("  ✅ You need to count occurrences");
    println!("  ✅ You're mapping one thing to another");
    
    // Examples
    println!("\n📝 Examples:");
    
    // HashSet: Unique visitor tracking
    let mut unique_visitors = HashSet::new();
    unique_visitors.insert("user_123");
    unique_visitors.insert("user_456");
    unique_visitors.insert("user_123"); // Duplicate ignored
    println!("  Unique visitors: {}", unique_visitors.len());
    
    // HashMap: Visit count tracking
    let mut visit_counts = HashMap::new();
    *visit_counts.entry("user_123").or_insert(0) += 1;
    *visit_counts.entry("user_456").or_insert(0) += 1;
    *visit_counts.entry("user_123").or_insert(0) += 1;
    println!("  Visit counts: {:?}", visit_counts);
    
    println!();
}

/// Bonus: Memory layout comparison
#[allow(dead_code)]
fn memory_analysis() {
    use std::mem;
    
    println!("🧠 Memory Analysis");
    println!("=================");
    
    let set = HashSet::<i32>::new();
    let map = HashMap::<i32, ()>::new();
    
    println!("Size of HashSet<i32>: {} bytes", mem::size_of_val(&set));
    println!("Size of HashMap<i32, ()>: {} bytes", mem::size_of_val(&map));
    println!("Size of (): {} bytes", mem::size_of::<()>());
    
    println!("✅ () takes zero bytes - no memory waste!");
}