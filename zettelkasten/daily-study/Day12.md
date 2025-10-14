# Day 12 - BTreeMap & BTreeSet

**📍 This is a navigation page. The actual Day 12 content is located in the daily study directory.**

---

## 🔗 Access Full Content

**➡️ [[daily_study/rust_learning_week2_notes/Day12|Day 12 Complete Content]]**

Or navigate directly to: `daily_study/rust_learning_week2_notes/Day12.md`

---

## 📋 Quick Reference

**Day 12 Focus**: BTreeMap & BTreeSet - Ordered Collections

**Key Topics Covered**:
- BTreeMap creation and ordered key-value storage
- BTreeSet for sorted unique elements
- Automatic sorting and ordering properties
- Range queries and iteration in sorted order
- Performance characteristics: O(log n) operations
- When to use BTreeMap vs HashMap
- When to use BTreeSet vs HashSet
- Real-world applications (leaderboards, priority systems, sorted data)

---

## 🔗 Related Concepts

### Zettelkasten Deep Dives
- [[BTreeMap Usage]] - Deep dive into ordered map operations
- [[Mission5 Overview]] - HashMap vs BTreeMap comparison
- [[Collections MOC]] - All collection types overview

### Daily Study Progression
- [[Day 11 - HashSet Operations]] - Previous day (unique collections)
- **Current**: [[../daily_study/rust_learning_week2_notes/Day12|Day 12 - BTreeMap & BTreeSet]]
- [[Day 13 - Advanced Iterators]] - Next day (iterator patterns)

### Mission Applications
- [[Mission5 Overview]] - Collection performance comparison
- [[Mission5_tut Overview]] - Tutorial series for collections
- [[AoC Patterns MOC]] - BTreeMap usage in competitive programming

---

## 🚀 Quick Start

### BTreeMap Usage
```rust
use std::collections::BTreeMap;

let mut scores = BTreeMap::new();
scores.insert("Alice", 100);
scores.insert("Bob", 150);
scores.insert("Charlie", 75);

// Automatically sorted by key
for (name, score) in &scores {
    println!("{}: {}", name, score);
}
// Output: Alice: 100, Bob: 150, Charlie: 75
```

### Range Queries
```rust
let mut map = BTreeMap::new();
for i in 0..10 {
    map.insert(i, i * i);
}

// Range query: values between 3 and 7
for (key, value) in map.range(3..=7) {
    println!("{}: {}", key, value);
}
```

### BTreeSet Usage
```rust
use std::collections::BTreeSet;

let mut set = BTreeSet::new();
set.insert(3);
set.insert(1);
set.insert(4);
set.insert(1);  // Duplicate ignored

// Automatically sorted
for value in &set {
    println!("{}", value);
}
// Output: 1, 3, 4
```

---

## 📚 Full Content

**See the complete daily study file for:**
- Detailed performance comparisons
- Advanced range query patterns
- When to choose BTree vs Hash collections
- Real-world use cases and examples

**Direct Link**: [[../daily_study/rust_learning_week2_notes/Day12|📖 Day 12 - BTreeMap & BTreeSet (Full Content)]]

---

*Tags: #btreemap #btreeset #collections #week2 #quick-ref*