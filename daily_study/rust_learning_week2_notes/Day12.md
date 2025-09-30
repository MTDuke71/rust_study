# Day 12 · BTreeMap & BTreeSet (ordered collections)

> **Learning Context**: Day 12 of Week 2 focuses on ordered collections, building toward Mission5's HashMap implementation by understanding the alternative tree-based approach to key-value storage.

**Cross-Track Integration:**
- **Mission5 Connection**: Contrasts with HashMap's hash-table approach - see [[Mission5 Overview]]
- **Daily Study**: Part of Week 2's collections mastery progression  
- **Rust Book**: Complements Chapter 8 Collections with ordered alternatives

**Related Zettelkasten Notes:**
- [[Collections MOC]] - Complete map of data structure learning
- [[HashMap Internals]] - Hash table vs B-tree comparison
- [[zettel-index]] - Main learning hub

## Core Concepts

### BTreeMap<K,V> Fundamentals
- **Ordering**: Maintains keys in sorted order automatically
- **Structure**: Self-balancing B-tree (not binary tree!)
- **Performance**: O(log n) for insert, remove, and lookup operations
- **Trait Requirements**: `K: Ord` (keys must be orderable)

### BTreeSet<T> Implementation
- **Zero-Cost Abstraction**: `BTreeSet<T> = BTreeMap<T, ()>` internally
- **Same Benefits**: Ordered iteration, range queries, sorted operations
- **Requirements**: `T: Ord`

## Key Differences from HashMap/HashSet

### Performance Comparison
```rust
use std::collections::{HashMap, BTreeMap};

// HashMap: O(1) average, O(n) worst case
let mut hash_map = HashMap::new();

// BTreeMap: O(log n) guaranteed
let mut btree_map = BTreeMap::new();

// When to choose which:
// HashMap: Raw speed, no ordering needed
// BTreeMap: Need sorted order, guaranteed performance
```

### Trait Requirements
```rust
use std::collections::BTreeMap;

// ✅ BTreeMap only needs Ord
let mut scores: BTreeMap<String, i32> = BTreeMap::new();
scores.insert("Alice".to_string(), 100);
scores.insert("Bob".to_string(), 85);

// Keys are automatically sorted!
for (name, score) in &scores {
    println!("{}: {}", name, score); // Alice: 100, Bob: 85
}

// ❌ This won't work - f64 doesn't implement Ord
// let mut float_map: BTreeMap<f64, String> = BTreeMap::new();
```

## Essential Operations

### Creation and Basic Operations
```rust
use std::collections::BTreeMap;

// Empty map
let mut map = BTreeMap::new();

// Insert maintains order
map.insert(3, "three");
map.insert(1, "one");
map.insert(2, "two");

// Iteration is always sorted by key
for (k, v) in &map {
    println!("{}: {}", k, v); // 1: one, 2: two, 3: three
}

// Standard operations
assert_eq!(map.get(&2), Some(&"two"));
assert_eq!(map.remove(&1), Some("one"));
```

### Range Operations (Unique to BTreeMap)
```rust
use std::collections::BTreeMap;
use std::ops::Bound::*;

let mut scores = BTreeMap::new();
scores.insert("Alice", 95);
scores.insert("Bob", 82);
scores.insert("Charlie", 78);
scores.insert("Diana", 91);
scores.insert("Eve", 88);

// Range queries - HashMap can't do this!
println!("Scores from Bob to Diana:");
for (name, score) in scores.range("Bob"..="Diana") {
    println!("{}: {}", name, score);
}

// Advanced range operations
let high_scores: Vec<_> = scores
    .range((Included("Charlie"), Unbounded))
    .filter(|(_, &score)| score > 85)
    .collect();
```

### Split Operations
```rust
use std::collections::BTreeMap;

let mut data = BTreeMap::new();
for i in 1..=10 {
    data.insert(i, i * i);
}

// Split at key 5
let upper = data.split_off(&5);
// data now contains {1: 1, 2: 4, 3: 9, 4: 16}
// upper contains {5: 25, 6: 36, 7: 49, 8: 64, 9: 81, 10: 100}
```

## BTreeSet Operations

### Set Operations with Ordering
```rust
use std::collections::BTreeSet;

let set1: BTreeSet<i32> = [3, 1, 4, 1, 5].into_iter().collect();
let set2: BTreeSet<i32> = [2, 4, 6, 8].into_iter().collect();

// All operations maintain sorted order
let union: BTreeSet<_> = set1.union(&set2).cloned().collect();
println!("Union: {:?}", union); // {1, 2, 3, 4, 5, 6, 8}

let intersection: BTreeSet<_> = set1.intersection(&set2).cloned().collect();
println!("Intersection: {:?}", intersection); // {4}
```

### Range Queries on Sets
```rust
use std::collections::BTreeSet;

let numbers: BTreeSet<i32> = (1..=20).collect();

// Find all numbers between 5 and 15
let middle_range: Vec<_> = numbers.range(5..=15).collect();
println!("Range 5-15: {:?}", middle_range);

// First/last elements
if let Some(&first) = numbers.first() {
    println!("Smallest: {}", first);
}
if let Some(&last) = numbers.last() {
    println!("Largest: {}", last);
}
```

## Advanced Patterns

### Custom Ordering
```rust
use std::collections::BTreeMap;
use std::cmp::Reverse;

// Reverse ordering using Reverse wrapper
let mut scores: BTreeMap<Reverse<i32>, String> = BTreeMap::new();
scores.insert(Reverse(95), "Alice".to_string());
scores.insert(Reverse(82), "Bob".to_string());
scores.insert(Reverse(91), "Diana".to_string());

// Now iterates from highest to lowest score
for (Reverse(score), name) in &scores {
    println!("{}: {}", name, score); // Alice: 95, Diana: 91, Bob: 82
}
```

### Custom Ord Implementation
```rust
use std::collections::BTreeSet;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct Student {
    name: String,
    grade: u8,
}

impl Ord for Student {
    fn cmp(&self, other: &Self) -> Ordering {
        // First by grade (descending), then by name (ascending)
        other.grade.cmp(&self.grade)
            .then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

let mut honor_roll = BTreeSet::new();
honor_roll.insert(Student { name: "Alice".to_string(), grade: 95 });
honor_roll.insert(Student { name: "Bob".to_string(), grade: 87 });
honor_roll.insert(Student { name: "Charlie".to_string(), grade: 95 });

// Automatically sorted by grade (desc), then name (asc)
for student in &honor_roll {
    println!("{}: {}", student.name, student.grade);
}
// Output: Alice: 95, Charlie: 95, Bob: 87
```

## Mission5 Integration: Hash vs Tree Trade-offs

### When to Choose BTreeMap over HashMap (Mission5 REQ-4 Analysis)
```rust
use std::collections::{HashMap, BTreeMap};

// Mission5 HashMap: O(1) amortized, unordered
// Perfect for: Fast lookups, no ordering requirements
let mut user_sessions: HashMap<String, u64> = HashMap::new();

// BTreeMap: O(log n) guaranteed, ordered  
// Perfect for: Range queries, sorted iteration, predictable performance
let mut timestamp_events: BTreeMap<u64, String> = BTreeMap::new();

// Example: Processing events chronologically (common AoC pattern)
timestamp_events.insert(1001, "user_login".to_string());
timestamp_events.insert(1003, "data_access".to_string());  
timestamp_events.insert(1002, "permission_check".to_string());

// BTreeMap automatically maintains chronological order
println!("Event timeline:");
for (timestamp, event) in &timestamp_events {
    println!("  {}: {}", timestamp, event);
}
// Output is naturally chronological: 1001, 1002, 1003
```

### Performance Comparison (Mission5 REQ-5 Validation Context)
```rust
// Mission5 focuses on HashMap optimization, but BTreeMap provides:
// ✅ Guaranteed O(log n) - no worst-case O(n) hash collisions
// ✅ Predictable memory usage - no rehashing overhead
// ✅ Cache-friendly iteration - sequential tree traversal
// ❌ Slower single operations - O(log n) vs O(1) average

// Use BTreeMap when Mission5 HashMap isn't suitable:
// - Need sorted output (reports, debugging)
// - Range queries (time windows, coordinate bounds)
// - Predictable performance critical
```

## Use Cases and Applications

### Sorted Data Processing
```rust
use std::collections::BTreeMap;

// Word frequency counter with alphabetical output
fn count_words(text: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }
    
    counts
}

let text = "rust is fast rust is safe rust is fun";
let word_counts = count_words(text);

// Automatically alphabetical output!
for (word, count) in word_counts {
    println!("{}: {}", word, count);
}
```

### Time Series Data
```rust
use std::collections::BTreeMap;

type Timestamp = u64;
type Temperature = f64;

let mut readings: BTreeMap<Timestamp, Temperature> = BTreeMap::new();
readings.insert(1000, 20.5);
readings.insert(2000, 21.2);
readings.insert(1500, 20.8);
readings.insert(2500, 22.1);

// Query temperature range between timestamps
let start_time = 1200;
let end_time = 2200;

let readings_in_range: Vec<_> = readings
    .range(start_time..=end_time)
    .collect();

println!("Readings from {} to {}:", start_time, end_time);
for (time, temp) in readings_in_range {
    println!("  {}: {:.1}°C", time, temp);
}
```

### Priority Queues (Simple Version)
```rust
use std::collections::BTreeSet;
use std::cmp::Reverse;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Task {
    priority: Reverse<u8>, // Higher number = higher priority
    id: u32,
}

let mut task_queue = BTreeSet::new();
task_queue.insert(Task { priority: Reverse(3), id: 101 });
task_queue.insert(Task { priority: Reverse(1), id: 102 });
task_queue.insert(Task { priority: Reverse(5), id: 103 });

// Process tasks in priority order
while let Some(task) = task_queue.pop_first() { // pop_first() available in newer Rust
    println!("Processing task {} with priority {}", task.id, task.priority.0);
}
```

## AoC Applications

### Coordinate Systems with Ordering
```rust
use std::collections::BTreeSet;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    y: i32,
    x: i32, // Note: y first for row-major ordering
}

let mut points = BTreeSet::new();
points.insert(Point { x: 1, y: 2 });
points.insert(Point { x: 0, y: 1 });
points.insert(Point { x: 2, y: 1 });

// Automatically sorted by y, then x
for point in points {
    println!("({}, {})", point.x, point.y);
}
// Output: (0, 1), (2, 1), (1, 2)
```

### Event Processing
```rust
use std::collections::BTreeMap;

#[derive(Debug)]
struct Event {
    timestamp: u64,
    event_type: String,
}

let mut timeline = BTreeMap::new();
timeline.insert(1500, Event { timestamp: 1500, event_type: "start".to_string() });
timeline.insert(1000, Event { timestamp: 1000, event_type: "init".to_string() });
timeline.insert(2000, Event { timestamp: 2000, event_type: "end".to_string() });

// Process events in chronological order
for (timestamp, event) in timeline {
    println!("Time {}: {}", timestamp, event.event_type);
}
```

## Performance Considerations

### When to Choose BTreeMap/BTreeSet
```rust
// ✅ Use BTreeMap/BTreeSet when:
// - Need sorted iteration
// - Performing range queries
// - Need guaranteed O(log n) performance
// - Working with ordered data (timestamps, coordinates)
// - Memory usage is more predictable than HashMap

// ✅ Use HashMap/HashSet when:
// - Only need basic operations (insert/lookup/remove)
// - Don't care about ordering
// - Need maximum speed for simple operations
// - Hash function quality is good
```

### Memory Usage
- **BTreeMap**: More predictable memory usage, better cache locality
- **HashMap**: Can have memory overhead from hash table sizing
- **Iteration**: BTreeMap iteration is cache-friendly (sequential access)

## Best Practices

### Choosing Ord vs PartialOrd
```rust
// ✅ Prefer deriving when possible
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct SimpleStruct {
    primary: i32,
    secondary: String,
}

// ✅ Custom implementation for complex ordering
impl Ord for ComplexStruct {
    fn cmp(&self, other: &Self) -> Ordering {
        // Custom logic here
        self.important_field.cmp(&other.important_field)
            .then_with(|| self.tiebreaker.cmp(&other.tiebreaker))
    }
}
```

## Learning Progression Summary

From Day 12, you should understand:
1. **Ordering**: Automatic key sorting vs hash-based unordered storage
2. **Performance**: O(log n) guaranteed vs O(1) average for HashMap
3. **Range Queries**: Powerful range operations unavailable in HashMap
4. **Trait Requirements**: `Ord` vs `Eq + Hash`
5. **Use Cases**: Time series, sorted output, range searches
6. **Trade-offs**: Predictable performance vs maximum speed

**3-Track Learning Integration:**
- **Mission5**: Compare BTreeMap's O(log n) with HashMap's O(1) amortized - both solve REQ-1 generic storage
- **Week 2 Progression**: Day 11 HashMap basics → Day 12 BTreeMap ordering → Day 13 Iterator mastery
- **AoC Applications**: Coordinate systems, event timelines, priority queues in competitive programming

**Cross-References:**
- [[Collections MOC]] - See "Tree-Based Collections" section
- [[Mission5 Overview]] - REQ-4 performance comparison opportunities  
- [[HashMap Internals]] - Hash collision vs tree rebalancing trade-offs

**Next**: Day 13 will cover **Advanced Iterators** - transforming and processing collections efficiently!

## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as a .rs file
use std::collections::{BTreeMap, BTreeSet, HashMap};

fn main() {
    println!("=== BTreeMap & BTreeSet Demo from Day 12 ===\n");
    
    // 1. Basic BTreeMap with automatic ordering
    println!("1. BTreeMap - Automatic Key Ordering:");
    let mut scores = BTreeMap::new();
    scores.insert("Charlie", 85);
    scores.insert("Alice", 95);
    scores.insert("Bob", 90);
    scores.insert("Diana", 88);
    
    println!("   Scores (sorted by name):");
    for (name, score) in &scores {
        println!("     {}: {}", name, score);
    }
    
    // 2. Range queries - the killer feature
    println!("\n2. Range Queries (impossible with HashMap):");
    let range_scores: BTreeMap<_, _> = scores.range("B".."D").collect();
    println!("   Names B-C: {:?}", range_scores);
    
    // 3. BTreeSet for ordered unique values
    println!("\n3. BTreeSet - Ordered Unique Collection:");
    let mut priorities = BTreeSet::new();
    priorities.insert(3);
    priorities.insert(1);
    priorities.insert(4);
    priorities.insert(1); // Duplicate ignored
    priorities.insert(5);
    
    println!("   Priorities (sorted): {:?}", priorities);
    
    // 4. Performance comparison demo
    println!("\n4. Performance Characteristics:");
    let mut hash_map = HashMap::new();
    let mut btree_map = BTreeMap::new();
    
    // Insert same data in both
    for i in 0..1000 {
        hash_map.insert(i, format!("value_{}", i));
        btree_map.insert(i, format!("value_{}", i));
    }
    
    println!("   HashMap: O(1) average lookup, unordered");
    println!("   BTreeMap: O(log n) guaranteed lookup, ordered");
    
    // 5. Time series example (practical AoC pattern)
    println!("\n5. Time Series Data (common AoC pattern):");
    let mut events = BTreeMap::new();
    events.insert(10, "Start process");
    events.insert(5, "Initialize");
    events.insert(15, "Complete");
    events.insert(8, "Load data");
    
    println!("   Chronological events:");
    for (time, event) in &events {
        println!("     T{}: {}", time, event);
    }
    
    // 6. Custom struct ordering
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Task {
        priority: i32,
        name: String,
    }
    
    let mut task_queue = BTreeSet::new();
    task_queue.insert(Task { priority: 2, name: "Medium task".to_string() });
    task_queue.insert(Task { priority: 1, name: "High priority".to_string() });
    task_queue.insert(Task { priority: 3, name: "Low priority".to_string() });
    
    println!("\n6. Priority Queue (sorted by priority then name):");
    for task in &task_queue {
        println!("     Priority {}: {}", task.priority, task.name);
    }
}
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day12_demo.rs` and run `rustc day12_demo.rs && ./day12_demo`  
3. **In this workspace**: `.\scripts\run_md.bat daily_study\rust_learning_week2_notes\Day12.md`
4. **As Cargo example**: `cargo run --example day12_btree_demo` (if you add it to Mission5_tut)

---
**Zettelkasten Integration:**
*Links: [[Collections MOC]] | [[Mission5 Overview]] | [[HashMap Internals]] | [[zettel-index]]*

*Tags: #btreemap #btreeset #ordered-collections #data-structures #daily-study #week2 #collections #performance-comparison*
