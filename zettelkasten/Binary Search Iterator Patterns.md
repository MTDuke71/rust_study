# Binary Search + Iterator Integration Patterns

*A deep dive into combining O(log n) search algorithms with Rust's zero-cost iterator abstractions for competitive programming.*

---

## 🎯 **Core Concept**

Binary search provides O(log n) lookup, but returning multiple results requires iteration. This note explores how to elegantly combine these patterns using Rust's iterator system for maximum performance and ergonomics.

## 📐 **The Pattern: Search + Lazy Iteration**

### **1. Custom Iterator Implementation**

```rust
pub struct RangeIter<'a, T> {
    slice: &'a [T],    // Borrowed data (lifetime 'a)
    end: usize,        // Range boundary
    current: usize,    // Current position
}

impl<'a, T> Iterator for RangeIter<'a, T> {
    type Item = &'a T;  // Yields borrowed references
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end && self.current < self.slice.len() {
            let item = &self.slice[self.current];
            self.current += 1;
            Some(item)
        } else {
            None
        }
    }
}
```

**Key Design Points:**
- Stores a **borrowed slice** (`&'a [T]`) - no copying data
- **Lifetime annotation** ensures iterator can't outlive the data
- Returns **borrowed references** (`&'a T`) - zero allocation
- **Lazy evaluation** - only advances when `next()` is called

### **2. Size Hints for Optimization**

```rust
fn size_hint(&self) -> (usize, Option<usize>) {
    let remaining = if self.current < self.end {
        self.end - self.current
    } else {
        0
    };
    (remaining, Some(remaining))  // Exact size known
}
```

**Why This Matters:**
- `collect()` can **pre-allocate** the exact Vec capacity
- No reallocation during collection
- Other iterators can optimize based on size

### **3. Binary Search Integration**

```rust
pub fn find_all_equal<'a, T: Ord>(slice: &'a [T], target: &T) -> RangeIter<'a, T> {
    let left = search_left_bound(slice, target);   // O(log n)
    let right = search_right_bound(slice, target);  // O(log n)
    RangeIter::new(slice, left, right)              // O(1)
}

// Usage:
let data = [1, 2, 2, 2, 3, 4, 5];
let twos: Vec<_> = find_all_equal(&data, &2).collect(); // [&2, &2, &2]
```

**Performance Characteristics:**
- **Find range**: O(log n) for both bounds
- **Create iterator**: O(1) - just stores indices
- **Iterate results**: O(k) where k = number of matches
- **Total**: O(log n + k) - optimal for this problem

## 🔧 **Extension Trait Pattern**

Make binary search feel like a native slice method:

```rust
pub trait SearchExt<T> {
    fn find_all_equal(&self, target: &T) -> RangeIter<'_, T>;
    fn find_range(&self, min: &T, max: &T) -> RangeIter<'_, T>;
}

impl<T: Ord> SearchExt<T> for [T] {
    fn find_all_equal(&self, target: &T) -> RangeIter<'_, T> {
        find_all_equal(self, target)
    }
    
    fn find_range(&self, min: &T, max: &T) -> RangeIter<'_, T> {
        find_range(self, min, max)
    }
}

// Now ANY slice can use these methods:
let data = [1, 2, 2, 3, 4, 5];
let results: Vec<_> = data.find_all_equal(&2).collect();
```

**Benefits:**
- **Ergonomic API** - feels like standard library
- **Method chaining** - `data.find_range(&3, &8).filter(...)`
- **Discoverable** - IDE autocomplete shows custom methods
- **Generic** - works with any `Ord` type

## ⚡ **Iterator Composition Patterns**

### **1. Chaining Operations (Zero Intermediate Allocations)**

```rust
let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let result: Vec<_> = data
    .find_range(&3, &8)        // Iterator over [3,4,5,6,7,8]
    .filter(|&&x| x % 2 == 0)  // Filter even numbers
    .map(|&x| x * x)           // Square them
    .collect();                // [16, 36, 64]
```

**Why This Is Fast:**
- No intermediate `Vec` allocations
- Single pass through data
- Compiler optimizations apply across the entire chain

### **2. Early Termination**

```rust
let found = data
    .find_range(&1, &100)
    .find(|&&x| x > 50)        // Stops at first match
    .copied();
```

**Iterator Short-Circuits:**
- Doesn't iterate entire range
- Returns as soon as condition is met
- O(k) where k = position of first match

### **3. Collecting into Different Types**

```rust
use std::collections::HashSet;

let unique_range: HashSet<i32> = data
    .find_range(&3, &8)
    .copied()
    .collect();  // HashSet instead of Vec
```

**FromIterator Trait:**
- Many types implement `FromIterator`
- Same iterator works with Vec, HashSet, BTreeSet, etc.
- No code changes needed

## 🎮 **AoC-Style Application Patterns**

### **1. Range Queries**

```rust
// Find all coordinates in bounding box
let coordinates: Vec<_> = sorted_points
    .find_range(&Point::new(10, 10), &Point::new(20, 20))
    .collect();
```

### **2. Threshold Finding**

```rust
// Find first element meeting criteria
let first_valid = sorted_energy_levels
    .find_first_matching(|&energy| energy >= required_threshold);
```

### **3. Batch Processing**

```rust
// Process data in chunks efficiently
for chunk in data.find_range(&start, &end).collect::<Vec<_>>().chunks(100) {
    process_batch(chunk);
}
```

## 💡 **Performance Benefits**

### **1. No Intermediate Collections**

```rust
// ❌ BAD: Creates intermediate Vec
let temp: Vec<_> = data.iter().filter(|&&x| x > 5).collect();
let result: Vec<_> = temp.iter().map(|&x| x * 2).collect();

// ✅ GOOD: Single pass, no allocations until collect()
let result: Vec<_> = data.iter()
    .filter(|&&x| x > 5)
    .map(|&x| x * 2)
    .collect();
```

### **2. Lifetime-Safe Borrowing**

```rust
fn process_range<'a>(data: &'a [i32]) -> impl Iterator<Item = &'a i32> {
    data.find_range(&5, &15)  // Iterator has same lifetime as data
}
```

**Safety Guarantees:**
- Iterator can't outlive the data it references
- Compiler enforces lifetime relationships
- No runtime overhead

### **3. Stack-Allocated State**

```rust
// These are tiny - no heap allocation:
let iter = data.iter().filter(|&&x| x > 5).map(|&x| x * 2);
```

**Iterator Adaptor Sizes:**
- Many iterators are **zero-sized types** (ZSTs)
- Filter/Map store closure + base iterator (usually small)
- Entire chain lives on the stack

## 🎯 **Why This Pattern Matters**

1. **Performance**: Zero-cost abstractions - high-level code runs as fast as hand-optimized loops
2. **Composability**: Mix and match operations without performance penalty
3. **Safety**: Borrowing rules prevent use-after-free and data races
4. **Readability**: Declarative style describes *what* you want, not *how* to get it
5. **Lazy Evaluation**: Work only happens when results are consumed

## 🔗 **Related Concepts**

- [[Mission3]] - Binary search implementation with this pattern
- [[Zero-Cost Abstractions]] - How Rust achieves performance + ergonomics
- [[Iterator Trait Deep Dive]] - Understanding the Iterator trait system
- [[Lifetime Annotations in Practice]] - Practical lifetime management patterns

---

*Tags: #binary-search #iterators #mission3 #performance #aoc #zero-cost-abstractions #competitive-programming*

*Links: [[zettel-index]] | [[Mission3]] | [[Trait-Based Design Patterns]]*
