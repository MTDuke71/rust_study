// HashSet Collision Handling - Deep Dive
// Understanding how Rust's HashSet deals with hash collisions

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn main() {
    println!("=== HashSet Collision Handling Deep Dive ===\n");
    
    demo_collision_basics();
    demo_forced_collisions();
    demo_collision_performance();
    demo_internal_structure_concept();
    demo_collision_resolution_strategies();
    demonstrate_hash_requirements();
}

fn demo_collision_basics() {
    println!("1. What Are Hash Collisions?");
    println!("   A collision occurs when different values produce the same hash");
    println!("   Example: hash(\"cat\") = 12345, hash(\"dog\") = 12345 ← COLLISION!\n");
    
    // Let's create a simple example where we can see hashes
    let items = ["hello", "world", "rust", "code"];
    
    println!("   Hash values for different strings:");
    for item in items {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let hash_value = hasher.finish();
        println!("   \"{}\" → hash: {}", item, hash_value);
    }
    println!();
}

fn demo_forced_collisions() {
    println!("2. Forcing Collisions with Bad Hash Function");
    println!("   Let's create a type that ALWAYS has the same hash\n");
    
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct BadHashItem {
        name: String,
        id: u32,
    }
    
    impl Hash for BadHashItem {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // BAD: Always hash to the same value - guaranteed collisions!
            999_u64.hash(state);
        }
    }
    
    let mut collision_set = HashSet::new();
    
    // All these items will have the SAME hash but are different values
    let items = [
        BadHashItem { name: "Alice".to_string(), id: 1 },
        BadHashItem { name: "Bob".to_string(), id: 2 },
        BadHashItem { name: "Charlie".to_string(), id: 3 },
        BadHashItem { name: "Diana".to_string(), id: 4 },
    ];
    
    println!("   Inserting items with identical hashes...");
    for item in items {
        let inserted = collision_set.insert(item.clone());
        println!("   {:?} → inserted: {}", item, inserted);
    }
    
    println!("\n   Final set size: {} (all items stored despite hash collisions!)", collision_set.len());
    println!("   HashSet correctly handles collisions by checking actual equality\n");
}

fn demo_collision_performance() {
    println!("3. Performance Impact of Collisions");
    println!("   Good hash: O(1) average lookup");
    println!("   Bad hash (many collisions): O(n) worst case lookup\n");
    
    // Good hash function (uses actual string content)
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct GoodHashString(String);
    
    impl Hash for GoodHashString {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state); // Use actual string content
        }
    }
    
    // Bad hash function (always same hash = all collisions)
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct BadHashString(String);
    
    impl Hash for BadHashString {
        fn hash<H: Hasher>(&self, state: &mut H) {
            42_u64.hash(state); // Always same hash!
        }
    }
    
    // Create sets with same data
    let mut good_set = HashSet::new();
    let mut bad_set = HashSet::new();
    
    let test_data = vec!["apple", "banana", "cherry", "date", "elderberry"];
    
    for item in &test_data {
        good_set.insert(GoodHashString(item.to_string()));
        bad_set.insert(BadHashString(item.to_string()));
    }
    
    println!("   Both sets contain {} items", test_data.len());
    println!("   Good hash: Each item likely in different bucket → O(1) lookup");
    println!("   Bad hash: All items in same bucket → O(n) lookup");
    println!("   Real HashSet implementations use techniques to minimize this impact\n");
}

fn demo_internal_structure_concept() {
    println!("4. How HashSet Handles Collisions Internally");
    println!("   Simplified conceptual model:\n");
    
    println!("   HashSet uses an array of 'buckets':");
    println!("   ┌─────────┬─────────┬─────────┬─────────┬─────────┐");
    println!("   │Bucket 0 │Bucket 1 │Bucket 2 │Bucket 3 │Bucket 4 │");
    println!("   └─────────┴─────────┴─────────┴─────────┴─────────┘");
    println!();
    
    println!("   Without collisions:");
    println!("   hash(\"cat\") % 5 = 2  →  Bucket 2: [\"cat\"]");
    println!("   hash(\"dog\") % 5 = 4  →  Bucket 4: [\"dog\"]");
    println!();
    
    println!("   With collision:");
    println!("   hash(\"cat\") % 5 = 2  →  Bucket 2: [\"cat\"]");
    println!("   hash(\"rat\") % 5 = 2  →  Bucket 2: [\"cat\", \"rat\"] ← Same bucket!");
    println!();
    
    println!("   Collision resolution strategies:");
    println!("   1. Chaining: Each bucket is a list/vector");
    println!("   2. Open Addressing: Find next empty slot");
    println!("   3. Robin Hood: Minimize variance in probe distances\n");
}

fn demo_collision_resolution_strategies() {
    println!("5. Rust's HashSet Collision Strategy");
    println!("   Rust uses Robin Hood hashing with quadratic probing\n");
    
    // Let's demonstrate by showing what happens with hash collisions
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    struct TrackedHashItem {
        value: String,
        expected_bucket: usize,
    }
    
    impl Hash for TrackedHashItem {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }
    
    impl PartialEq for TrackedHashItem {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }
    
    impl Eq for TrackedHashItem {}
    
    // Create items and show their hash distribution
    let items = ["hash", "map", "set", "rust", "code"];
    let mut tracked_set = HashSet::new();
    
    println!("   Conceptual bucket assignment (hash % bucket_count):");
    for item in items {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        let hash_value = hasher.finish();
        let conceptual_bucket = hash_value % 8; // Assuming 8 buckets
        
        let tracked_item = TrackedHashItem {
            value: item.to_string(),
            expected_bucket: conceptual_bucket as usize,
        };
        
        tracked_set.insert(tracked_item);
        println!("   \"{}\" → hash: {} → bucket: {}", item, hash_value, conceptual_bucket);
    }
    
    println!("\n   Key insights:");
    println!("   - HashSet automatically grows to minimize collisions");
    println!("   - Uses sophisticated algorithms to handle unavoidable collisions");
    println!("   - Robin Hood hashing keeps probe distances small and consistent");
    println!("   - You usually don't need to worry about collisions!");
    println!();
    
    println!("   Final set size: {} (all unique items stored)", tracked_set.len());
}

// Bonus: Function to demonstrate the hash function requirements
fn demonstrate_hash_requirements() {
    println!("6. Hash Function Requirements");
    println!("   For HashSet to work correctly, hash functions must satisfy:");
    println!("   1. Deterministic: Same input → Same hash (always)");
    println!("   2. Uniform distribution: Spread values evenly across hash space");
    println!("   3. Equality consistency: If a == b, then hash(a) == hash(b)");
    println!();
    
    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
    }
    
    impl Hash for Person {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // Good: Hash all fields that matter for equality
            self.name.hash(state);
            self.age.hash(state);
        }
    }
    
    impl PartialEq for Person {
        fn eq(&self, other: &Self) -> bool {
            // Must match the Hash implementation!
            self.name == other.name && self.age == other.age
        }
    }
    
    impl Eq for Person {}
    
    let alice1 = Person { name: "Alice".to_string(), age: 30 };
    let alice2 = Person { name: "Alice".to_string(), age: 30 };
    
    // Verify hash consistency
    let mut hasher1 = DefaultHasher::new();
    let mut hasher2 = DefaultHasher::new();
    
    alice1.hash(&mut hasher1);
    alice2.hash(&mut hasher2);
    
    println!("   alice1 == alice2: {}", alice1 == alice2);
    println!("   hash(alice1) == hash(alice2): {}", hasher1.finish() == hasher2.finish());
    println!("   ✅ Hash consistency maintained!\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_collision_handling() {
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct CollidingItem(String);
        
        impl Hash for CollidingItem {
            fn hash<H: Hasher>(&self, state: &mut H) {
                // Force all items to have same hash
                0_u64.hash(state);
            }
        }
        
        let mut set = HashSet::new();
        set.insert(CollidingItem("first".to_string()));
        set.insert(CollidingItem("second".to_string()));
        set.insert(CollidingItem("third".to_string()));
        
        // All have same hash but different values - should all be stored
        assert_eq!(set.len(), 3);
        
        // Contains should work despite collisions
        assert!(set.contains(&CollidingItem("first".to_string())));
        assert!(set.contains(&CollidingItem("second".to_string())));
        assert!(set.contains(&CollidingItem("third".to_string())));
    }
    
    #[test]
    fn test_hash_consistency_requirement() {
        use std::collections::hash_map::DefaultHasher;
        
        #[derive(Debug, PartialEq, Eq)]
        struct TestItem(String);
        
        impl Hash for TestItem {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
        
        let item1 = TestItem("test".to_string());
        let item2 = TestItem("test".to_string());
        
        // Equal items must have equal hashes
        assert_eq!(item1, item2);
        
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        
        item1.hash(&mut hasher1);
        item2.hash(&mut hasher2);
        
        assert_eq!(hasher1.finish(), hasher2.finish());
    }
}