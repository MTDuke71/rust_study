// Day 11 Tutorial: Custom Hash Types
// Understanding how to implement Hash and Eq for HashSet/HashMap usage

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn main() {
    println!("=== Day 11 Tutorial: Custom Hash Types ===\n");
    
    demo_basic_requirements();
    demo_case_insensitive_string();
    demo_custom_point_hash();
    demo_selective_field_hashing();
    demo_hash_collision_example();
}

/// Demonstrates the basic trait requirements for HashSet usage
fn demo_basic_requirements() {
    println!("1. Basic Trait Requirements (Hash + Eq + PartialEq)");
    println!("   For a type to work in HashSet/HashMap, it needs:");
    println!("   - Hash: How to compute hash value");
    println!("   - Eq: Reflexive equality (a == a is always true)"); 
    println!("   - PartialEq: How to compare for equality");
    println!("   ");
    println!("   🔍 Eq vs PartialEq difference:");
    println!("   - PartialEq: Can have cases where equality is undefined (like f64::NAN)");
    println!("   - Eq: Guarantees x == x is ALWAYS true (marker trait, no methods)");
    println!("   - HashMap/HashSet require Eq because reflexivity is essential for hashing\n");
    
    // Simple enum that derives all required traits
    #[derive(Debug, Hash, Eq, PartialEq)]
    #[allow(dead_code)]
    enum Color {
        Red, Green, Blue
    }
    
    let mut color_set = HashSet::new();
    color_set.insert(Color::Red);
    color_set.insert(Color::Blue);
    color_set.insert(Color::Red); // Duplicate - ignored
    
    println!("   Color set: {:?}", color_set);
    println!("   Length: {} (Red was deduplicated)\n", color_set.len());
}

/// Case-insensitive string hashing - practical example
fn demo_case_insensitive_string() {
    println!("2. Case-Insensitive String Hashing");
    println!("   Goal: \"RUST\" and \"rust\" should be treated as the same\n");
    
    #[derive(Debug, Clone)]
    struct CaseInsensitiveString(String);
    
    impl Hash for CaseInsensitiveString {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // Hash the lowercase version, so "RUST" and "rust" have same hash
            self.0.to_lowercase().hash(state);
        }
    }
    
    impl PartialEq for CaseInsensitiveString {
        fn eq(&self, other: &Self) -> bool {
            // Compare lowercase versions
            self.0.to_lowercase() == other.0.to_lowercase()
        }
    }
    
    impl Eq for CaseInsensitiveString {}
    
    let mut tags = HashSet::new();
    tags.insert(CaseInsensitiveString("RUST".to_string()));
    tags.insert(CaseInsensitiveString("Python".to_string()));
    tags.insert(CaseInsensitiveString("rust".to_string())); // Should be duplicate!
    tags.insert(CaseInsensitiveString("PYTHON".to_string())); // Should be duplicate!
    
    println!("   Tags inserted: RUST, Python, rust, PYTHON");
    println!("   Unique tags: {} (case-insensitive deduplication)", tags.len());
    for tag in &tags {
        println!("     - {:?}", tag.0);
    }
    println!();
}

/// Custom Point type with distance-based equality
fn demo_custom_point_hash() {
    println!("3. Custom Point with Coordinate Hashing");
    println!("   Grid points that hash based on coordinates\n");
    
    #[derive(Debug, Clone)]
    struct Point {
        x: i32,
        y: i32,
        label: String, // This won't affect hashing/equality
    }
    
    impl Hash for Point {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // Only hash coordinates, ignore label
            self.x.hash(state);
            self.y.hash(state);
        }
    }
    
    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            // Only compare coordinates, ignore label
            self.x == other.x && self.y == other.y
        }
    }
    
    impl Eq for Point {}
    
    let mut visited = HashSet::new();
    visited.insert(Point { x: 0, y: 0, label: "origin".to_string() });
    visited.insert(Point { x: 1, y: 1, label: "diagonal".to_string() });
    visited.insert(Point { x: 0, y: 0, label: "start".to_string() }); // Same coords, different label
    
    println!("   Points: (0,0,'origin'), (1,1,'diagonal'), (0,0,'start')");
    println!("   Unique positions: {} (labels don't matter)", visited.len());
    for point in &visited {
        println!("     - ({}, {}) '{}'", point.x, point.y, point.label);
    }
    println!();
}

/// Demonstrating selective field hashing for complex structs
fn demo_selective_field_hashing() {
    println!("4. Selective Field Hashing");
    println!("   User struct where only username matters for uniqueness\n");
    
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    struct User {
        id: u32,
        username: String,
        email: String,
        last_login: String,
    }
    
    impl Hash for User {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // Only hash username - users are unique by username
            self.username.hash(state);
        }
    }
    
    impl PartialEq for User {
        fn eq(&self, other: &Self) -> bool {
            // Only compare username
            self.username == other.username
        }
    }
    
    impl Eq for User {}
    
    let mut users = HashSet::new();
    users.insert(User {
        id: 1,
        username: "alice".to_string(),
        email: "alice@old.com".to_string(),
        last_login: "2023-01-01".to_string(),
    });
    users.insert(User {
        id: 2,
        username: "bob".to_string(), 
        email: "bob@example.com".to_string(),
        last_login: "2023-02-01".to_string(),
    });
    users.insert(User {
        id: 3,
        username: "alice".to_string(), // Same username!
        email: "alice@new.com".to_string(), // Different email
        last_login: "2023-03-01".to_string(), // Different login
    });
    
    println!("   Inserted: alice(id=1), bob(id=2), alice(id=3,different email)");
    println!("   Unique users: {} (username-based uniqueness)", users.len());
    for user in &users {
        println!("     - {} (id={}, {})", user.username, user.id, user.email);
    }
    println!();
}

/// Understanding hash collisions and quality
fn demo_hash_collision_example() {
    println!("5. Hash Quality and Collisions");
    println!("   Bad vs Good hash implementations\n");
    
    // Bad hash: always returns same value (terrible distribution)
    #[derive(Debug, Clone)]
    struct BadHashString(String);
    
    impl Hash for BadHashString {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // BAD: Always hash the same value
            42u64.hash(state); // Everything has same hash!
        }
    }
    
    impl PartialEq for BadHashString {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    
    impl Eq for BadHashString {}
    
    // Good hash: uses the actual string content
    #[derive(Debug, Clone)]
    struct GoodHashString(String);
    
    impl Hash for GoodHashString {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // GOOD: Hash the actual string content
            self.0.hash(state);
        }
    }
    
    impl PartialEq for GoodHashString {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    
    impl Eq for GoodHashString {}
    
    // Demonstrate with HashSet
    let mut bad_set = HashSet::new();
    bad_set.insert(BadHashString("hello".to_string()));
    bad_set.insert(BadHashString("world".to_string()));
    bad_set.insert(BadHashString("rust".to_string()));
    
    let mut good_set = HashSet::new();
    good_set.insert(GoodHashString("hello".to_string()));
    good_set.insert(GoodHashString("world".to_string()));
    good_set.insert(GoodHashString("rust".to_string()));
    
    println!("   Bad hash (all collide): {} items stored correctly", bad_set.len());
    println!("   Good hash (distributed): {} items stored correctly", good_set.len());
    println!("   Bad hashing causes O(n) performance instead of O(1)!\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_case_insensitive_string() {
        let s1 = CaseInsensitiveString("HELLO".to_string());
        let s2 = CaseInsensitiveString("hello".to_string());
        
        // Should be equal
        assert_eq!(s1, s2);
        
        // Should have same hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;
        
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        
        s1.hash(&mut hasher1);
        s2.hash(&mut hasher2);
        
        assert_eq!(hasher1.finish(), hasher2.finish());
    }
    
    #[test]
    fn test_point_equality() {
        let p1 = Point { x: 5, y: 10, label: "first".to_string() };
        let p2 = Point { x: 5, y: 10, label: "second".to_string() };
        
        // Same coordinates, different labels - should be equal
        assert_eq!(p1, p2);
        
        let mut set = HashSet::new();
        set.insert(p1);
        set.insert(p2); // Should be ignored (duplicate coordinates)
        
        assert_eq!(set.len(), 1);
    }
}

// Helper types for the demo functions
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CaseInsensitiveString(String);

impl Hash for CaseInsensitiveString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_lowercase().hash(state);
    }
}

impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Eq for CaseInsensitiveString {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
    label: String,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}