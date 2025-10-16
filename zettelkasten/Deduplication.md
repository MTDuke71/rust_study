# Deduplication in Rust

**Removing duplicate values from collections efficiently and idiomatically**

## 🎯 The Core Problem

When working with collections, duplicate values often need to be removed:

```rust
let numbers = vec![1, 2, 3, 2, 4, 1, 5, 3];
// Want: [1, 2, 3, 4, 5]
```

**Challenge**: Different use cases require different approaches based on:
- Whether order matters
- Performance requirements (time vs space)
- Whether you need to count occurrences
- Original collection mutability

## 📊 Deduplication Strategies Comparison

| Strategy | Time | Space | Order Preserved | Use Case |
|----------|------|-------|-----------------|----------|
| **HashSet** | O(n) | O(n) | ❌ No | Fast, order doesn't matter |
| **sort + dedup** | O(n log n) | O(1) | ❌ Changes to sorted | In-place, memory constrained |
| **Itertools::unique** | O(n) | O(n) | ✅ Yes | Order matters, use iterator |
| **Manual loop** | O(n²) | O(1) | ✅ Yes | Simple, small collections |
| **HashMap counting** | O(n) | O(n) | ✅ Yes | Need occurrence counts |

## 🦀 Rust Idiomatic Approaches

### **Strategy 1: HashSet Collection** (Fastest, No Order)

```rust
use std::collections::HashSet;

fn deduplicate_hashset<T: Eq + std::hash::Hash + Clone>(items: &[T]) -> Vec<T> {
    items.iter()
        .cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

// Usage
let numbers = vec![1, 2, 3, 2, 4, 1, 5];
let unique: Vec<_> = numbers.iter()
    .copied()
    .collect::<HashSet<_>>()
    .into_iter()
    .collect();
// Result: [1, 2, 3, 4, 5] (order not guaranteed)
```

**Characteristics**:
- ✅ O(n) time complexity
- ✅ Simple and readable
- ❌ Doesn't preserve insertion order
- ❌ Requires Hash + Eq traits
- ✅ Best for large collections where order doesn't matter

### **Strategy 2: Sort + Dedup** (In-Place, Memory Efficient)

```rust
fn deduplicate_sort<T: Ord>(items: &mut Vec<T>) {
    items.sort();
    items.dedup();
}

// Usage
let mut numbers = vec![1, 2, 3, 2, 4, 1, 5];
numbers.sort();
numbers.dedup();  // Removes consecutive duplicates
// Result: [1, 2, 3, 4, 5] (sorted)
```

**Characteristics**:
- ✅ O(1) space (in-place)
- ✅ Built-in, no external crates
- ❌ O(n log n) time for sort
- ❌ Changes order (sorts the collection)
- ✅ Best when memory is constrained and sorted output is acceptable

### **Strategy 3: Preserve Order with HashSet Tracking**

```rust
use std::collections::HashSet;

fn deduplicate_preserve_order<T>(items: Vec<T>) -> Vec<T>
where
    T: Eq + std::hash::Hash,
{
    let mut seen = HashSet::new();
    items.into_iter()
        .filter(|item| seen.insert(item.clone()))
        .collect()
}

// Usage
let numbers = vec![1, 2, 3, 2, 4, 1, 5, 3];
let unique = deduplicate_preserve_order(numbers);
// Result: [1, 2, 3, 4, 5] (original order preserved)
```

**How it works**:
- `HashSet::insert()` returns `true` if value was newly inserted
- Returns `false` if value already existed
- Filter keeps only items that were newly inserted (first occurrence)

**Characteristics**:
- ✅ O(n) time complexity
- ✅ Preserves insertion order
- ❌ O(n) space for HashSet
- ✅ **Most common choice for general deduplication**

### **Strategy 4: Itertools::unique** (Iterator-Based)

```rust
use itertools::Itertools;

// Lazy evaluation - processes as consumed
let numbers = vec![1, 2, 3, 2, 4, 1, 5];
let unique: Vec<_> = numbers.iter()
    .unique()
    .copied()
    .collect();
// Result: [1, 2, 3, 4, 5] (order preserved)
```

**Characteristics**:
- ✅ Lazy evaluation (doesn't allocate until collected)
- ✅ Preserves order
- ✅ Works with iterators (composable)
- ❌ Requires external crate (itertools)
- ✅ Best for iterator chains and pipelines

### **Strategy 5: HashMap with Occurrence Counting**

```rust
use std::collections::HashMap;

fn deduplicate_with_counts<T>(items: &[T]) -> Vec<(T, usize)>
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut counts = HashMap::new();
    
    for item in items {
        *counts.entry(item.clone()).or_insert(0) += 1;
    }
    
    counts.into_iter().collect()
}

// Usage
let numbers = vec![1, 2, 3, 2, 4, 1, 5, 3];
let unique_with_counts = deduplicate_with_counts(&numbers);
// Result: [(1, 2), (2, 2), (3, 2), (4, 1), (5, 1)]
```

**Characteristics**:
- ✅ Provides occurrence counts
- ✅ O(n) time complexity
- ❌ Doesn't preserve original order
- ✅ Best when you need frequency information

## 🎄 AoC Applications

### **Problem Pattern: Finding Unique Elements**

Many AoC problems require identifying unique items:

```rust
// AoC Pattern: Count unique locations visited
use std::collections::HashSet;

fn count_unique_positions(moves: &[(i32, i32)]) -> usize {
    let mut visited = HashSet::new();
    let mut pos = (0, 0);
    
    visited.insert(pos);
    
    for &(dx, dy) in moves {
        pos.0 += dx;
        pos.1 += dy;
        visited.insert(pos);
    }
    
    visited.len()  // Number of unique positions
}
```

### **Problem Pattern: Removing Duplicate Inputs**

```rust
// AoC Pattern: Process unique ingredients/components
fn process_unique_items(items: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    
    items.into_iter()
        .filter(|item| seen.insert(item.clone()))
        .collect()
}
```

### **Problem Pattern: Frequency Analysis**

```rust
// AoC Pattern: Find most/least common element
use std::collections::HashMap;

fn find_most_common<T: Eq + std::hash::Hash>(items: &[T]) -> Option<&T> {
    let mut counts = HashMap::new();
    
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    
    counts.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(item, _)| item)
}
```

## 🔧 Advanced Patterns

### **Dedup by Custom Key**

```rust
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

fn deduplicate_by_name(people: Vec<Person>) -> Vec<Person> {
    let mut seen_names = HashSet::new();
    
    people.into_iter()
        .filter(|person| seen_names.insert(person.name.clone()))
        .collect()
}
```

### **Dedup with Custom Equality**

```rust
// Keep only first occurrence based on custom comparison
fn deduplicate_custom<T, F>(items: Vec<T>, mut eq: F) -> Vec<T>
where
    F: FnMut(&T, &T) -> bool,
{
    let mut result = Vec::new();
    
    'outer: for item in items {
        for existing in &result {
            if eq(&item, existing) {
                continue 'outer;
            }
        }
        result.push(item);
    }
    
    result
}

// Usage: case-insensitive string deduplication
let strings = vec!["Hello", "hello", "World", "HELLO"];
let unique = deduplicate_custom(strings, |a, b| {
    a.eq_ignore_ascii_case(b)
});
// Result: ["Hello", "World"]
```

### **Dedup Consecutive Only (Built-in)**

```rust
// Remove only consecutive duplicates (no sorting)
let mut numbers = vec![1, 2, 2, 3, 2, 2, 4, 4, 5];
numbers.dedup();
// Result: [1, 2, 3, 2, 4, 5] (non-consecutive duplicates remain)
```

## 📈 Performance Considerations

### **Benchmark Comparison** (1000 elements with 50% duplicates)

```
HashSet collection:           ~15 µs  ✅ Fastest
Sort + dedup:                 ~45 µs  (sorting overhead)
Preserve order (HashSet):     ~18 µs  ✅ Fast + order preserved
Manual O(n²) loop:           ~500 µs  ❌ Slow for large collections
```

### **Memory Usage**

```rust
// Original: 1000 elements
let data = vec![1; 1000];

// HashSet approach: ~2000 elements allocated (original + set)
let unique: HashSet<_> = data.iter().copied().collect();

// Sort + dedup: ~1000 elements (in-place)
let mut data = vec![1; 1000];
data.sort();
data.dedup();

// Itertools unique: Lazy (allocates HashSet for tracking only)
```

## 🎯 Decision Guide

**Choose HashSet when:**
- ✅ Order doesn't matter
- ✅ Need fastest performance
- ✅ Working with large collections

**Choose sort + dedup when:**
- ✅ Memory is constrained
- ✅ Sorted output is acceptable
- ✅ No external dependencies needed

**Choose preserve-order with HashSet when:**
- ✅ Order matters (most common case)
- ✅ Good performance needed
- ✅ Can afford O(n) space

**Choose itertools::unique when:**
- ✅ Already using itertools
- ✅ Working with iterator chains
- ✅ Want lazy evaluation

**Choose HashMap counting when:**
- ✅ Need frequency information
- ✅ Finding most/least common elements
- ✅ Statistical analysis needed

## 🧪 Common Patterns

### **Dedup Vec<String>**

```rust
use std::collections::HashSet;

fn deduplicate_strings(strings: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    strings.into_iter()
        .filter(|s| seen.insert(s.clone()))
        .collect()
}
```

### **Dedup Vec<&str> (References)**

```rust
fn deduplicate_str_refs<'a>(strings: Vec<&'a str>) -> Vec<&'a str> {
    let mut seen = HashSet::new();
    strings.into_iter()
        .filter(|&s| seen.insert(s))
        .collect()
}
```

### **Dedup with Transformation**

```rust
// Keep unique items after applying transformation
fn deduplicate_normalized(strings: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    
    strings.into_iter()
        .filter(|s| seen.insert(s.to_lowercase()))
        .collect()
}
```

## 💡 Key Insights

1. **HashSet is your friend**: For most deduplication needs, HashSet provides the best balance
2. **Order matters most**: If you need to preserve order, use the "seen set" filter pattern
3. **In-place when possible**: sort + dedup uses no extra memory
4. **Count when needed**: Use HashMap when frequency matters
5. **Traits required**: Your type needs `Eq + Hash` for HashSet-based approaches

## 🔗 Related Concepts

- **[[Collections MOC]]** - Complete data structures overview
- **[[HashMap Internals]]** - How HashSet is implemented (HashSet = HashMap with unit values)
- **[[Iterator Traits]]** - Understanding filter and collect patterns
- **[[daily-study/Day10]]** - HashMap operations for frequency counting
- **[[Vec Patterns]]** - Working with dynamic arrays

---

## 🔗 Navigation

### 📚 Zettelkasten
- **[[zettel-index]]** - Main knowledge base entry point
- **[[Collections MOC]]** - Data structures overview
- **[[Rust Concepts MOC]]** - Core language features

### 🎯 Related Data Structures
- **[[HashMap Internals]]** - Hash-based collections (HashSet = HashMap<K, ()>)
- **[[Vec Patterns]]** - Dynamic array operations
- **[[Iterator Traits]]** - Iterator-based transformations

### 📖 Learning Path
- **[[daily-study/Day10]]** - Hash-based collections introduction
- **[[Day 09 - Iterators]]** - Iterator patterns for deduplication
- **[[Performance Optimization]]** - Choosing the right strategy

### 🎄 AoC Applications
- **[[AoC Patterns MOC]]** - Competitive programming patterns
- **[[Frequency Analysis]]** - Counting occurrences
- **[[Set Operations]]** - Unique element operations

### 🏗️ Mission Integration
- **[[Mission5 Overview]]** - HashMap applications
- **[Mission5 README](../missions/Mission5/README.md)** - Hash-based collection implementation

---

*Tags: #deduplication #collections #hashset #hashmap #performance #iterators #aoc-patterns #data-structures #optimization #frequency-counting*
