# AoC Binary Search Application Patterns

*Practical binary search patterns for Advent of Code competitive programming, with real-world examples and complexity analysis.*

---

## 🎯 **Why Binary Search in AoC?**

Binary search appears in AoC problems more often than you'd expect:

- **Direct**: Find element in sorted array
- **Monotonic predicates**: Find first/last element matching condition
- **Range queries**: Find all elements in a range
- **Optimization**: Binary search on answer space

**Key Advantage:** O(log n) instead of O(n) - critical when n > 10^6

## 📐 **Core AoC Patterns**

### **Pattern 1: Classic Lookup**

```rust
// Find if element exists in sorted data
let coordinates = vec![10, 25, 47, 53, 89, 102];
match coordinates.binary_search(&47) {
    Ok(index) => println!("Found at position {}", index),
    Err(_) => println!("Not found"),
}
```

**AoC Use Cases:**

- Checking if coordinate is in valid set
- Verifying ID exists in sorted list
- Fast membership testing

**Complexity:** O(log n)

### **Pattern 2: Find All Duplicates**

```rust
// Mission3 pattern: Find all elements equal to target
let data = [1, 2, 2, 2, 3, 4, 5];
let twos: Vec<_> = data.find_all_equal(&2).collect();
// Returns [&2, &2, &2]
```

**Implementation:**

```rust
pub fn find_all_equal<'a, T: Ord>(slice: &'a [T], target: &T) -> RangeIter<'a, T> {
    let left = search_left_bound(slice, target);   // O(log n)
    let right = search_right_bound(slice, target);  // O(log n)
    RangeIter::new(slice, left, right)              // Returns iterator
}
```

**AoC Use Cases:**

- Finding all timestamps at specific time
- Grouping identical values
- Range sum queries with duplicates

**Complexity:** O(log n) to find range, O(k) to iterate results

### **Pattern 3: Range Queries**

```rust
// Find all elements in a range [min, max]
let data = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
let range: Vec<_> = data.find_range(&7, &13).collect();
// Returns [&7, &9, &11, &13]
```

**AoC Use Cases:**

- Finding coordinates in bounding box
- Time-based event filtering
- Energy level thresholds
- Temperature ranges

**Example - Coordinate Filtering:**

```rust
// Find all points in rectangular region
struct Point { x: i32, y: i32 }

let sorted_by_x: Vec<Point> = /* ... */;
let in_range: Vec<_> = sorted_by_x
    .find_range(&Point { x: 10, y: 0 }, &Point { x: 20, y: 0 })
    .filter(|p| p.y >= 10 && p.y <= 20)
    .collect();
```

**Complexity:** O(log n) to find bounds, O(k) to iterate results

### **Pattern 4: Predicate-Based Search**

```rust
// Find first element matching a condition
// Condition must be monotonic: once true, stays true
let data = [1, 3, 5, 7, 9, 11, 13, 15];

let first_big = data.find_first_matching(|&x| x >= 10);
// Returns Some(&11)
```

**AoC Use Cases:**

- First timestamp after event
- Minimum energy level to succeed
- First valid configuration
- Threshold crossing

**Example - Energy Threshold:**

```rust
let energy_levels = [5, 10, 15, 20, 25, 30];
let required = 18;

let first_valid = energy_levels
    .find_first_matching(|&e| e >= required);
// Returns Some(&20)
```

**Complexity:** O(log n)

### **Pattern 5: Binary Search on Answer Space**

```rust
// Search for optimal answer by binary search on possible values
fn can_achieve(target: i32, resources: &[i32]) -> bool {
    // Returns true if target is achievable
    // Monotonic: if target X works, all smaller values work too
}

fn find_minimum(resources: &[i32]) -> i32 {
    let mut low = 0;
    let mut high = 1_000_000;
    
    while low < high {
        let mid = low + (high - low) / 2;
        if can_achieve(mid, resources) {
            high = mid;  // Try smaller value
        } else {
            low = mid + 1;  // Need larger value
        }
    }
    low
}
```

**AoC Use Cases:**

- Minimum time to complete task
- Maximum capacity under constraints
- Optimal resource allocation
- Finding equilibrium points

**Real AoC Example - Ship Fuel (AoC 2019 Day 1):**

```rust
// Find minimum fuel to reach position
fn fuel_cost(pos: i32, target: i32) -> i32 {
    (pos - target).abs()
}

fn min_position(positions: &[i32]) -> i32 {
    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();
    
    (min_pos..=max_pos)
        .min_by_key(|&target| {
            positions.iter().map(|&p| fuel_cost(p, target)).sum::<i32>()
        })
        .unwrap()
}
```

**Complexity:** O(log(max - min) * cost_of_check)

## 🎮 **Practical AoC Scenarios**

### **Scenario 1: Coordinate System Navigation**

```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Coord { x: i32, y: i32 }

// Sort by x first, then y
let mut coords = vec![
    Coord { x: 5, y: 10 },
    Coord { x: 3, y: 7 },
    Coord { x: 8, y: 2 },
    // ... many more
];
coords.sort();

// Fast lookup: Is coordinate in our set?
let target = Coord { x: 5, y: 10 };
if coords.binary_search(&target).is_ok() {
    println!("Coordinate found!");
}

// Range query: All coords with x in [3, 6]
let in_range: Vec<_> = coords
    .find_range(&Coord { x: 3, y: 0 }, &Coord { x: 6, y: i32::MAX })
    .collect();
```

### **Scenario 2: Event Timeline Processing**

```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    timestamp: i32,
    event_type: String,
}

let mut events = vec![/* sorted by timestamp */];

// Find all events in time window [100, 200]
let window_events: Vec<_> = events
    .find_range(
        &Event { timestamp: 100, event_type: String::new() },
        &Event { timestamp: 200, event_type: String::from("\u{10FFFF}") }
    )
    .collect();

// Find first event after timestamp 150
let next_event = events
    .find_first_matching(|e| e.timestamp > 150);
```

### **Scenario 3: Resource Allocation**

```rust
// Sorted list of available resources
let resources = [10, 20, 30, 40, 50, 60, 70];

// Need at least 45 units
let min_needed = 45;

let viable: Vec<_> = resources
    .iter()
    .filter(|&&r| r >= min_needed)
    .collect();

// Or using binary search:
let first_viable = resources
    .find_first_matching(|&r| r >= min_needed);
```

## ⚡ **Performance Considerations**

### **When to Use Binary Search:**

✅ **Good Cases:**

- Data is sorted (or can be sorted once)
- Many queries on same dataset
- Large dataset (n > 1000)
- Need O(log n) guarantee

❌ **Bad Cases:**

- Unsorted data with infrequent queries (sorting overhead)
- Very small datasets (linear search faster)
- Need to find multiple elements scattered throughout array

### **Sorting Cost:**

```rust
// If you need multiple searches, sort once:
let mut data = vec![/* unsorted data */];
data.sort();  // O(n log n)

// Then each search is O(log n)
data.binary_search(&target1);  // O(log n)
data.binary_search(&target2);  // O(log n)
// ...

// Breakeven: If queries > log n, binary search wins
```

### **Memory Usage:**

- **Slice binary search**: O(1) - no extra memory
- **Iterator pattern**: O(1) - just stores indices
- **Collecting results**: O(k) - only for results

## 🧪 **Testing Strategy**

```rust
#[test]
fn test_binary_search_patterns() {
    let data = vec![1, 2, 2, 3, 4, 5, 5, 5, 6];
    
    // Test exact match
    assert!(data.binary_search(&3).is_ok());
    
    // Test find all duplicates
    let fives: Vec<_> = data.find_all_equal(&5).collect();
    assert_eq!(fives.len(), 3);
    
    // Test range query
    let range: Vec<_> = data.find_range(&2, &5).collect();
    assert!(range.contains(&&2));
    assert!(range.contains(&&5));
    
    // Test predicate search
    let first_big = data.find_first_matching(|&&x| x > 4);
    assert_eq!(first_big, Some(&5));
}
```

## 🎯 **AoC Success Checklist**

When facing an AoC problem:

- [ ] Can the data be sorted?
- [ ] Is there a monotonic property? (once true, stays true)
- [ ] Are there multiple queries on same data?
- [ ] Is the dataset large? (n > 1000)
- [ ] Do I need range queries?
- [ ] Am I searching for optimal value in range?

If you answered "yes" to 2+ questions, consider binary search!

## 🔗 **Related Patterns**

- [[Binary Search]] - Foundational algorithm and complexity analysis
- [[Binary Search Iterator Patterns]] - Iterator integration techniques
- [[mission-3]] - Complete implementation with tests
- [[Trait Design Patterns - Mission3 Lessons]] - API design insights
- [[AoC Pattern Recognition]] - Other common AoC patterns

---

*Tags: #binary-search #aoc #competitive-programming #algorithms #mission3 #performance #optimization*

*Links: [[zettel-index]] | [[mission-3]] | [[Algorithm Complexity Analysis]]*
