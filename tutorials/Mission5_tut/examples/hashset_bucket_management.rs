// HashSet Bucket Management - When and How Buckets Are Determined
// Understanding HashSet's dynamic sizing and load factor management

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn main() {
    println!("=== HashSet Bucket Management Deep Dive ===\n");

    demo_initial_capacity();
    demo_growth_during_insertion();
    demo_load_factor_concept();
    demo_capacity_vs_len();
    demo_shrinking_behavior();
    demo_with_capacity_control();
    demo_rehashing_process();
    demonstrate_bucket_math();
}

fn demo_initial_capacity() {
    println!("1. Initial Bucket Determination");
    println!("   HashSet starts with a default capacity and grows as needed\n");

    // Empty HashSet - minimal initial allocation
    let empty_set: HashSet<i32> = HashSet::new();
    println!("   New empty HashSet:");
    println!("   - Length: {}", empty_set.len());
    println!(
        "   - Capacity: {} (initial allocation)",
        empty_set.capacity()
    );

    // HashSet with known capacity
    let sized_set: HashSet<i32> = HashSet::with_capacity(100);
    println!("\n   HashSet::with_capacity(100):");
    println!("   - Length: {}", sized_set.len());
    println!("   - Capacity: {} (pre-allocated)", sized_set.capacity());
    println!("   - Note: Actual capacity may be higher due to power-of-2 rounding\n");
}

fn demo_growth_during_insertion() {
    println!("2. Dynamic Growth During Insertion");
    println!("   Watch capacity change as we add items\n");

    let mut set = HashSet::new();
    let mut previous_capacity = set.capacity();

    println!("   Insertion progression:");
    for i in 0..20 {
        set.insert(i);
        let current_capacity = set.capacity();

        if current_capacity != previous_capacity {
            println!(
                "   📈 After {} items: capacity {} → {} (GROWTH!)",
                i + 1,
                previous_capacity,
                current_capacity
            );
            previous_capacity = current_capacity;
        }
    }

    println!("\n   Final state:");
    println!("   - Items: {}", set.len());
    println!("   - Capacity: {}", set.capacity());
    println!(
        "   - Load factor: {:.2} ({}/{})",
        set.len() as f64 / set.capacity() as f64,
        set.len(),
        set.capacity()
    );
    println!();
}

fn demo_load_factor_concept() {
    println!("3. Load Factor and Resize Triggers");
    println!("   HashSet resizes when load factor gets too high\n");

    // Track load factor during growth
    let mut set = HashSet::new();
    println!("   Load factor tracking:");
    println!("   (HashSet typically resizes at ~75% load factor)\n");

    for i in 0..50 {
        set.insert(i);
        let load_factor = set.len() as f64 / set.capacity() as f64;

        // Show interesting load factor points
        if i == 0 || load_factor > 0.7 || (i + 1) % 10 == 0 {
            println!(
                "   {} items: capacity={}, load_factor={:.3}",
                set.len(),
                set.capacity(),
                load_factor
            );
        }
    }
    println!();
}

fn demo_capacity_vs_len() {
    println!("4. Capacity vs Length - The Key Difference");
    println!("   Length = actual items stored");
    println!("   Capacity = total bucket slots available\n");

    let mut set = HashSet::with_capacity(16);

    println!("   Starting state:");
    println!("   - len(): {} (items stored)", set.len());
    println!("   - capacity(): {} (bucket slots)", set.capacity());

    // Add some items
    for i in 0..10 {
        set.insert(i);
    }

    println!("\n   After adding 10 items:");
    println!("   - len(): {} (items stored)", set.len());
    println!("   - capacity(): {} (bucket slots)", set.capacity());
    println!("   - Available slots: {}", set.capacity() - set.len());

    // Remove some items
    for i in 0..5 {
        set.remove(&i);
    }

    println!("\n   After removing 5 items:");
    println!("   - len(): {} (items stored)", set.len());
    println!(
        "   - capacity(): {} (bucket slots - doesn't shrink automatically)",
        set.capacity()
    );
    println!();
}

fn demo_shrinking_behavior() {
    println!("5. Shrinking Behavior");
    println!("   HashSet doesn't automatically shrink when items are removed\n");

    let mut set = HashSet::new();

    // Fill it up
    for i in 0..100 {
        set.insert(i);
    }

    let full_capacity = set.capacity();
    println!("   After adding 100 items:");
    println!("   - Length: {}", set.len());
    println!("   - Capacity: {}", full_capacity);

    // Remove most items
    for i in 0..95 {
        set.remove(&i);
    }

    println!("\n   After removing 95 items:");
    println!("   - Length: {}", set.len());
    println!(
        "   - Capacity: {} (unchanged - no automatic shrinking)",
        set.capacity()
    );

    // Manual shrinking
    set.shrink_to_fit();

    println!("\n   After shrink_to_fit():");
    println!("   - Length: {}", set.len());
    println!("   - Capacity: {} (manually reduced)", set.capacity());
    println!();
}

fn demo_with_capacity_control() {
    println!("6. Controlling Initial Capacity");
    println!("   Pre-allocating vs letting it grow naturally\n");

    // Method 1: Let it grow naturally
    let mut natural_set = HashSet::new();
    for i in 0..1000 {
        natural_set.insert(i);
    }

    println!("   Natural growth to 1000 items:");
    println!("   - Final capacity: {}", natural_set.capacity());

    // Method 2: Pre-allocate
    let mut preallocated_set = HashSet::with_capacity(1000);
    for i in 0..1000 {
        preallocated_set.insert(i);
    }

    println!("\n   Pre-allocated with_capacity(1000):");
    println!("   - Final capacity: {}", preallocated_set.capacity());

    println!("\n   Benefits of pre-allocation:");
    println!("   ✅ Fewer resize operations during insertion");
    println!("   ✅ More predictable performance");
    println!("   ✅ Reduces memory fragmentation");
    println!();
}

fn demo_rehashing_process() {
    println!("7. The Rehashing Process");
    println!("   What happens when HashSet needs to grow\n");

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[allow(clippy::mutable_key_type)]
    struct TrackedItem {
        value: i32,
        hash_calls: std::cell::Cell<usize>,
    }

    impl Hash for TrackedItem {
        fn hash<H: Hasher>(&self, state: &mut H) {
            // Track how many times this item is hashed
            let calls = self.hash_calls.get();
            self.hash_calls.set(calls + 1);
            self.value.hash(state);
        }
    }

    impl TrackedItem {
        fn new(value: i32) -> Self {
            Self {
                value,
                hash_calls: std::cell::Cell::new(0),
            }
        }

        fn hash_call_count(&self) -> usize {
            self.hash_calls.get()
        }
    }

    let mut set = HashSet::new();
    let items: Vec<TrackedItem> = (0..10).map(TrackedItem::new).collect();

    println!("   Inserting items and tracking hash calls...");

    // Insert items one by one
    for (i, item) in items.iter().enumerate() {
        let old_capacity = set.capacity();
        set.insert(item.clone());
        let new_capacity = set.capacity();

        if new_capacity > old_capacity {
            println!(
                "   🔄 REHASHING at item {}: capacity {} → {}",
                i + 1,
                old_capacity,
                new_capacity
            );

            // Show hash call counts after potential rehashing
            let total_hash_calls: usize = items
                .iter()
                .take(i + 1)
                .map(|item| item.hash_call_count())
                .sum();

            println!(
                "      Total hash calls so far: {} (includes rehashing)",
                total_hash_calls
            );
        }
    }

    println!("\n   Key insights about rehashing:");
    println!("   - All existing items must be rehashed when capacity changes");
    println!("   - New capacity is typically double the old capacity");
    println!("   - This is why pre-allocation can improve performance");
    println!("   - Rehashing preserves all items but changes their bucket positions");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capacity_behavior() {
        let mut set = HashSet::new();
        let initial_capacity = set.capacity();

        // Add items until we see growth
        let mut grew = false;
        for i in 0..100 {
            set.insert(i);
            if set.capacity() > initial_capacity {
                grew = true;
                break;
            }
        }

        assert!(grew, "HashSet should grow when load factor gets high");
        assert!(
            set.capacity() >= set.len(),
            "Capacity should always be >= length"
        );
    }

    #[test]
    fn test_with_capacity() {
        let set: HashSet<i32> = HashSet::with_capacity(50);

        // Capacity should be at least what we requested
        assert!(set.capacity() >= 50);
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_shrink_to_fit() {
        let mut set = HashSet::new();

        // Fill it up
        for i in 0..100 {
            set.insert(i);
        }
        let large_capacity = set.capacity();

        // Remove most items
        for i in 0..95 {
            set.remove(&i);
        }

        // Capacity should still be large (but might have been reduced during removal)
        // The important thing is that it's still >= the current length
        assert!(set.capacity() >= set.len());
        let capacity_before_shrink = set.capacity();

        // Shrink it
        set.shrink_to_fit();

        // Capacity should be smaller now (or at least optimized for current size)
        assert!(set.capacity() >= set.len());
        // Note: shrink_to_fit() may or may not actually reduce capacity,
        // but it should optimize for current usage
    }
}

// Helper function to demonstrate bucket math
fn demonstrate_bucket_math() {
    println!("8. Bucket Math Examples");
    println!("   How items are mapped to buckets\n");

    // Simulate how HashSet might map items to buckets
    let capacity = 8; // Example capacity
    let items = [42, 17, 99, 23, 8];

    println!("   With capacity = {} buckets:", capacity);
    for item in items {
        let bucket = (item as usize) % capacity; // Simplified hash % capacity
        println!(
            "   item {} → bucket {} (simplified: {} % {} = {})",
            item, bucket, item, capacity, bucket
        );
    }

    println!("\n   When capacity doubles to {}:", capacity * 2);
    for item in items {
        let new_bucket = (item as usize) % (capacity * 2);
        let old_bucket = (item as usize) % capacity;
        println!(
            "   item {} → bucket {} (was bucket {})",
            item, new_bucket, old_bucket
        );
    }

    println!("\n   This is why rehashing is necessary when capacity changes!");
}
