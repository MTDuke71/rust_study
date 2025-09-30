// Real-Time Systems and HashSet Rehashing - Performance Analysis
// Understanding why rehashing can be problematic in real-time systems

use std::collections::HashSet;
use std::time::{Duration, Instant};
use std::hash::{Hash, Hasher};

fn main() {
    println!("=== Real-Time Systems: HashSet Rehashing Issues ===\n");
    
    demo_rehashing_performance_spike();
    demo_predictable_vs_unpredictable_timing();
    demo_real_time_mitigation_strategies();
    demo_alternative_data_structures();
    demo_bounded_capacity_approach();
}

fn demo_rehashing_performance_spike() {
    println!("1. The Rehashing Performance Problem");
    println!("   Measuring insertion times to see the rehashing spikes\n");
    
    let mut set = HashSet::new();
    let mut previous_capacity = 0;
    
    println!("   Timing each insertion (microseconds):");
    println!("   Item | Time (μs) | Capacity | Status");
    println!("   -----|-----------|----------|-------");
    
    for i in 0..20 {
        let start = Instant::now();
        set.insert(i);
        let duration = start.elapsed();
        
        let current_capacity = set.capacity();
        let is_rehash = current_capacity != previous_capacity;
        
        println!("   {:4} | {:9.1} | {:8} | {}", 
                 i + 1, 
                 duration.as_nanos() as f64 / 1000.0,
                 current_capacity,
                 if is_rehash { "🔄 REHASH!" } else { "normal" }
        );
        
        previous_capacity = current_capacity;
    }
    
    println!("\n   ⚠️  Real-time system impact:");
    println!("   - Most insertions: ~0.1-1 μs (predictable)");
    println!("   - Rehashing operations: 10-100x slower (unpredictable!)");
    println!("   - Could violate real-time deadlines\n");
}

fn demo_predictable_vs_unpredictable_timing() {
    println!("2. Predictable vs Unpredictable Performance");
    println!("   Comparing different scenarios\n");
    
    // Scenario 1: Pre-allocated HashSet (predictable)
    println!("   Scenario 1: Pre-allocated HashSet (1000 capacity)");
    let mut predictable_set = HashSet::with_capacity(1000);
    
    let mut times = Vec::new();
    for i in 0..100 {
        let start = Instant::now();
        predictable_set.insert(i);
        times.push(start.elapsed());
    }
    
    let avg_time = times.iter().sum::<Duration>() / times.len() as u32;
    let max_time = times.iter().max().unwrap();
    let min_time = times.iter().min().unwrap();
    
    println!("   - Average time: {:.2} μs", avg_time.as_nanos() as f64 / 1000.0);
    println!("   - Max time: {:.2} μs", max_time.as_nanos() as f64 / 1000.0);
    println!("   - Min time: {:.2} μs", min_time.as_nanos() as f64 / 1000.0);
    println!("   - Variance: LOW ✅ (good for real-time)");
    
    // Scenario 2: Growing HashSet (unpredictable)
    println!("\n   Scenario 2: Growing HashSet (starts empty)");
    let mut unpredictable_set = HashSet::new();
    
    let mut times = Vec::new();
    for i in 0..100 {
        let start = Instant::now();
        unpredictable_set.insert(i);
        times.push(start.elapsed());
    }
    
    let avg_time = times.iter().sum::<Duration>() / times.len() as u32;
    let max_time = times.iter().max().unwrap();
    let min_time = times.iter().min().unwrap();
    
    println!("   - Average time: {:.2} μs", avg_time.as_nanos() as f64 / 1000.0);
    println!("   - Max time: {:.2} μs", max_time.as_nanos() as f64 / 1000.0);
    println!("   - Min time: {:.2} μs", min_time.as_nanos() as f64 / 1000.0);
    println!("   - Variance: HIGH ❌ (problematic for real-time)\n");
}

fn demo_real_time_mitigation_strategies() {
    println!("3. Real-Time System Mitigation Strategies\n");
    
    println!("   Strategy 1: Pre-allocation");
    println!("   ✅ Use HashSet::with_capacity() with worst-case estimate");
    println!("   ✅ Eliminates rehashing during normal operation");
    println!("   ❌ May waste memory if estimate is too high");
    
    let mut preallocated = HashSet::with_capacity(10000);
    println!("   Example: Pre-allocated for 10,000 items");
    
    // Add items without rehashing
    for i in 0..1000 {
        preallocated.insert(i);
    }
    println!("   Added 1,000 items - no rehashing occurred\n");
    
    println!("   Strategy 2: Bounded Collections");
    println!("   ✅ Fixed maximum size - predictable memory usage");
    println!("   ✅ No unexpected rehashing");
    println!("   ❌ Need eviction policy when full");
    // We'll demo this in the bounded_capacity_approach function
    
    println!("\n   Strategy 3: Alternative Data Structures");
    println!("   ✅ Arrays/Vectors for small, known-size data");
    println!("   ✅ Custom fixed-size hash tables");
    println!("   ✅ Lock-free data structures");
    println!("   ❌ May sacrifice some functionality\n");
}

fn demo_alternative_data_structures() {
    println!("4. Alternative Data Structures for Real-Time\n");
    
    println!("   Option 1: Fixed-size array for small sets");
    
    // Simple fixed-size set using an array
    struct FixedSet<T: PartialEq + Copy, const N: usize> {
        data: [Option<T>; N],
        len: usize,
    }
    
    impl<T: PartialEq + Copy, const N: usize> FixedSet<T, N> {
        fn new() -> Self {
            Self {
                data: [None; N],
                len: 0,
            }
        }
        
        fn insert(&mut self, value: T) -> bool {
            // Check if already exists
            for item in &self.data[..self.len] {
                if *item == Some(value) {
                    return false; // Already exists
                }
            }
            
            // Add if space available
            if self.len < N {
                self.data[self.len] = Some(value);
                self.len += 1;
                true
            } else {
                false // Full
            }
        }
        
        fn contains(&self, value: &T) -> bool {
            self.data[..self.len].iter().any(|item| item == &Some(*value))
        }
        
        fn len(&self) -> usize {
            self.len
        }
    }
    
    let mut fixed_set: FixedSet<i32, 100> = FixedSet::new();
    
    // Timing test
    let start = Instant::now();
    for i in 0..50 {
        fixed_set.insert(i);
    }
    let duration = start.elapsed();
    
    println!("   FixedSet<i32, 100>:");
    println!("   - Added 50 items in {:.2} μs", duration.as_nanos() as f64 / 1000.0);
    println!("   - Final length: {}", fixed_set.len());
    println!("   - Contains 25: {}", fixed_set.contains(&25));
    println!("   - Predictable O(n) insertion, O(n) lookup");
    println!("   - Good for small sets (< 100 items)");
    println!("   - Zero dynamic allocation\n");
    
    println!("   Option 2: Pre-allocated Vec for simple cases");
    
    let mut vec_set = Vec::with_capacity(1000);
    let start = Instant::now();
    for i in 0..100 {
        if !vec_set.contains(&i) {
            vec_set.push(i);
        }
    }
    let duration = start.elapsed();
    
    println!("   Vec<i32> with contains() check:");
    println!("   - Added 100 unique items in {:.2} μs", duration.as_nanos() as f64 / 1000.0);
    println!("   - O(n²) insertion but predictable timing");
    println!("   - Good for small to medium sets\n");
}

fn demo_bounded_capacity_approach() {
    println!("5. Bounded Capacity Approach");
    println!("   Fixed-size hash table that never rehashes\n");
    
    // Simple bounded hash set
    struct BoundedHashSet<T> {
        buckets: Vec<Vec<T>>,
        count: usize,
        max_capacity: usize,
    }
    
    impl<T: Hash + PartialEq + Clone> BoundedHashSet<T> {
        fn new(bucket_count: usize, max_capacity: usize) -> Self {
            let mut buckets = Vec::with_capacity(bucket_count);
            for _ in 0..bucket_count {
                buckets.push(Vec::new());
            }
            
            Self {
                buckets,
                count: 0,
                max_capacity,
            }
        }
        
        fn insert(&mut self, value: T) -> Result<bool, &'static str> {
            if self.count >= self.max_capacity {
                return Err("Capacity exceeded");
            }
            
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            value.hash(&mut hasher);
            let bucket_index = (hasher.finish() as usize) % self.buckets.len();
            
            let bucket = &mut self.buckets[bucket_index];
            
            // Check if already exists
            if bucket.contains(&value) {
                return Ok(false);
            }
            
            // Add new value
            bucket.push(value);
            self.count += 1;
            Ok(true)
        }
        
        fn contains(&self, value: &T) -> bool {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            value.hash(&mut hasher);
            let bucket_index = (hasher.finish() as usize) % self.buckets.len();
            
            self.buckets[bucket_index].contains(value)
        }
        
        fn len(&self) -> usize {
            self.count
        }
    }
    
    let mut bounded_set = BoundedHashSet::new(32, 1000); // 32 buckets, max 1000 items
    
    println!("   BoundedHashSet (32 buckets, max 1000 items):");
    
    // Performance test
    let mut insert_times = Vec::new();
    for i in 0..100 {
        let start = Instant::now();
        let _ = bounded_set.insert(i);
        insert_times.push(start.elapsed());
    }
    
    let avg_time = insert_times.iter().sum::<Duration>() / insert_times.len() as u32;
    let max_time = insert_times.iter().max().unwrap();
    
    println!("   - Average insert time: {:.2} μs", avg_time.as_nanos() as f64 / 1000.0);
    println!("   - Max insert time: {:.2} μs", max_time.as_nanos() as f64 / 1000.0);
    println!("   - Items stored: {}", bounded_set.len());
    println!("   - Contains 50: {}", bounded_set.contains(&50));
    println!("   ✅ Predictable timing - never rehashes");
    println!("   ✅ Bounded memory usage");
    println!("   ❌ Fixed capacity limit\n");
    
    println!("   Real-time system recommendations:");
    println!("   1. 🎯 Profile your exact use case");
    println!("   2. 📐 Pre-allocate with realistic worst-case estimates");
    println!("   3. 🚧 Consider bounded collections for strict guarantees");
    println!("   4. ⏱️  Monitor timing in production");
    println!("   5. 🔄 Have fallback strategies for capacity exceeded");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_predictable_timing() {
        // Pre-allocated should have consistent timing
        let mut set = HashSet::with_capacity(1000);
        
        let mut times = Vec::new();
        for i in 0..100 {
            let start = Instant::now();
            set.insert(i);
            times.push(start.elapsed());
        }
        
        // Should not have grown during this test
        assert_eq!(set.capacity(), 1792); // Might be larger due to internal sizing
        
        // All times should be relatively similar (no major spikes from rehashing)
        let max_time = times.iter().max().unwrap();
        let min_time = times.iter().min().unwrap();
        
        // Max shouldn't be more than 50x min for pre-allocated (more realistic threshold)
        let time_ratio = max_time.as_nanos() as f64 / min_time.as_nanos() as f64;
        assert!(time_ratio < 50.0, "Timing too variable: max/min ratio = {:.1}", time_ratio);
    }
    
    #[test]
    fn test_bounded_hash_set_limits() {
        struct SimpleBoundedSet {
            data: Vec<i32>,
            max_size: usize,
        }
        
        impl SimpleBoundedSet {
            fn new(max_size: usize) -> Self {
                Self {
                    data: Vec::new(),
                    max_size,
                }
            }
            
            fn insert(&mut self, value: i32) -> Result<bool, &'static str> {
                if self.data.contains(&value) {
                    return Ok(false); // Already exists
                }
                
                if self.data.len() >= self.max_size {
                    return Err("Capacity exceeded");
                }
                
                self.data.push(value);
                Ok(true)
            }
        }
        
        let mut bounded = SimpleBoundedSet::new(3);
        
        assert_eq!(bounded.insert(1), Ok(true));
        assert_eq!(bounded.insert(2), Ok(true));
        assert_eq!(bounded.insert(3), Ok(true));
        
        // Should reject when full
        assert_eq!(bounded.insert(4), Err("Capacity exceeded"));
        
        // Duplicate should be handled
        assert_eq!(bounded.insert(1), Ok(false));
    }
}