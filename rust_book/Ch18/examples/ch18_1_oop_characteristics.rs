//! Chapter 18.1: Characteristics of Object-Oriented Languages
//!
//! This example demonstrates:
//! - Encapsulation with structs and pub keyword
//! - Objects containing data and methods
//! - Private implementation details
//! - Public interface design

/// AveragedCollection demonstrates encapsulation in Rust
/// 
/// The internal list and cached average are private, ensuring
/// the average is always kept in sync with the list contents.
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    /// Create a new empty AveragedCollection
    pub fn new() -> Self {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }

    /// Add a value to the collection
    /// 
    /// Automatically updates the cached average
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    /// Remove and return the last value
    /// 
    /// Returns None if the collection is empty
    /// Automatically updates the cached average
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    /// Get the current average (O(1) due to caching)
    pub fn average(&self) -> f64 {
        self.average
    }

    /// Get the number of elements
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Private method - implementation detail
    /// 
    /// This method is hidden from users of the API,
    /// ensuring internal consistency is maintained
    fn update_average(&mut self) {
        if self.list.is_empty() {
            self.average = 0.0;
        } else {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
}

impl Default for AveragedCollection {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    println!("=== Chapter 18.1: OOP Characteristics ===");

    // Demonstrating encapsulation
    println!("1. Encapsulation Example:");
    let mut numbers = AveragedCollection::new();
    
    println!("   Adding numbers: 5, 10, 15");
    numbers.add(5);
    numbers.add(10);
    numbers.add(15);
    
    println!("   Average: {}", numbers.average());
    println!("   Length: {}", numbers.len());
    
    println!("\n   Removing last number");
    numbers.remove();
    println!("   Average: {}", numbers.average());
    println!("   Length: {}", numbers.len());

    // The internal state is protected
    println!("\n2. Encapsulation Benefits:");
    println!("   ✓ Can't access 'list' directly");
    println!("   ✓ Can't access 'average' directly");
    println!("   ✓ Can't call 'update_average()' directly");
    println!("   ✓ Average is always consistent with list contents");
    
    // Demonstrating empty collection edge case
    println!("\n3. Edge Cases:");
    let mut empty = AveragedCollection::new();
    println!("   Empty collection average: {}", empty.average());
    println!("   Is empty: {}", empty.is_empty());
    println!("   Remove from empty: {:?}", empty.remove());
    
    empty.add(42);
    println!("   After adding 42, average: {}", empty.average());

    // Demonstrating the object-like nature
    println!("\n4. Object Properties:");
    println!("   ✓ Data (fields): list, average");
    println!("   ✓ Methods (behavior): add, remove, average, len");
    println!("   ✓ Encapsulation (privacy): private fields and methods");
    println!("   ✓ Abstraction: users don't need to know about caching");

    // Comparison with traditional OOP
    println!("\n5. OOP Comparison:");
    println!("   Rust                  | Traditional OOP");
    println!("   ---------------------|-------------------");
    println!("   struct + impl        | class");
    println!("   pub fn               | public method");
    println!("   fn (no pub)          | private method");
    println!("   pub struct           | public class");
    println!("   struct (no pub)      | private class");
    println!("   mod                  | package/namespace");

    // Demonstrating why encapsulation matters
    println!("\n6. Why Encapsulation Matters:");
    let mut scores = AveragedCollection::new();
    scores.add(85);
    scores.add(90);
    scores.add(78);
    
    println!("   Test scores: {}, {}, {}", 85, 90, 78);
    println!("   Class average: {:.1}", scores.average());
    println!("   Number of students: {}", scores.len());
    
    println!("\n   If fields were public, users could:");
    println!("   ✗ Modify list without updating average");
    println!("   ✗ Set average to wrong value");
    println!("   ✗ Break internal invariants");
    println!("\n   With encapsulation:");
    println!("   ✓ Average always matches list contents");
    println!("   ✓ Can change internal implementation");
    println!("   ✓ Guaranteed consistency");

    println!("\n=== End of Chapter 18.1 ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_collection_is_empty() {
        let collection = AveragedCollection::new();
        assert_eq!(collection.len(), 0);
        assert!(collection.is_empty());
        assert_eq!(collection.average(), 0.0);
    }

    #[test]
    fn test_add_updates_average() {
        let mut collection = AveragedCollection::new();
        collection.add(10);
        assert_eq!(collection.average(), 10.0);
        
        collection.add(20);
        assert_eq!(collection.average(), 15.0);
        
        collection.add(30);
        assert_eq!(collection.average(), 20.0);
    }

    #[test]
    fn test_remove_updates_average() {
        let mut collection = AveragedCollection::new();
        collection.add(10);
        collection.add(20);
        collection.add(30);
        assert_eq!(collection.average(), 20.0);
        
        collection.remove();
        assert_eq!(collection.average(), 15.0);
        
        collection.remove();
        assert_eq!(collection.average(), 10.0);
    }

    #[test]
    fn test_remove_from_empty() {
        let mut collection = AveragedCollection::new();
        assert_eq!(collection.remove(), None);
        assert_eq!(collection.average(), 0.0);
    }

    #[test]
    fn test_encapsulation_maintains_invariant() {
        let mut collection = AveragedCollection::new();
        
        // Add several values and track running sum
        let values = [5, 10, 15, 20, 25];
        let mut running_sum = 0;
        
        for (i, &value) in values.iter().enumerate() {
            collection.add(value);
            running_sum += value;
            
            // Verify average matches expected
            let expected: f64 = running_sum as f64 / (i + 1) as f64;
            
            assert!((collection.average() - expected).abs() < 0.001);
        }
    }
}
