# Day 12 - BTreeMap & BTreeSet (Ordered Collections)

*Ordered collections using balanced B-trees for sorted data and range queries.*

---

## 📅 **Session Context**

- **Week**: Week 2 - Collections & Data Structures
- **Focus**: Tree-based ordered collections as alternatives to hash-based maps/sets
- **Learning Path**: Building toward Mission 5 (HashMap) by understanding the ordered alternative

---

## 🎯 **Core Learning Objectives**

1. **BTreeMap<K, V>**: Self-balancing B-tree for ordered key-value storage
2. **BTreeSet<T>**: Ordered set built on BTreeMap (zero-cost abstraction)
3. **Range Operations**: Powerful queries unavailable in HashMap/HashSet
4. **Performance Trade-offs**: O(log n) guaranteed vs HashMap's O(1) average
5. **Custom Ordering**: Implementing `Ord` trait for custom sort logic

---

## 🔑 **Key Concepts Covered**

### **BTreeMap Fundamentals**
- Maintains keys in **sorted order** automatically
- Requires `K: Ord` (orderability, not hashability)
- O(log n) for insert, lookup, remove (guaranteed, no worst-case O(n))

### **Range Queries**
```rust
use std::collections::BTreeMap;

let mut scores = BTreeMap::new();
scores.insert("Alice", 95);
scores.insert("Charlie", 78);
scores.insert("Eve", 88);

// Range query - HashMap can't do this!
for (name, score) in scores.range("Bob"..="Diana") {
    println!("{}: {}", name, score);
}
```

### **When to Choose BTreeMap over HashMap**
- ✅ Need sorted iteration
- ✅ Range queries (time windows, coordinate bounds)
- ✅ Guaranteed O(log n) performance (no hash collision worst-case)
- ✅ Predictable memory usage (no rehashing)
- ❌ Don't need maximum speed for single operations

---

## 💡 **Mental Models**

### **Hash Table vs B-Tree**
```
HashMap:          BTreeMap:
┌──┬──┬──┬──┐    ┌─────5─────┐
│  │  │██│  │    │   ╱  ╲    │
└──┴──┴──┴──┘    2──3  7──9
Unordered        Sorted by key
O(1) average     O(log n) guaranteed
```

### **The Library Catalog Analogy**
- **HashMap**: Like a library's fast lookup system (call number → location)
  - Fast retrieval, but no order
- **BTreeMap**: Like books on a sorted shelf
  - Slower to find specific book, but easy to browse ranges
  - "Show me all books published between 2010-2020"

---

## 🔗 **Integration Points**

### **Builds On**
- [[daily-study/Day10]] - HashMap basics (hash-based collections)
- [[daily-study/Day11]] - HashSet operations
- [[ownership-fundamentals]] - Ord trait requires full ownership semantics

### **Enables**
- [[mission-5]] - Performance comparison: hash vs tree trade-offs
- [[graph-data-structures]] - Ordered adjacency lists for deterministic iteration
- [[sorting-algorithms]] - Pre-sorted data structures

### **Related Concepts**
- [[Collections MOC]] - Complete overview of Rust collections
- [[HashMap Internals]] - Hash collision vs tree rebalancing
- [[Big-O Notation]] - Comparing O(1) vs O(log n) performance

---

## 🚀 **Mission Applications**

### **Mission 5 - HashMap Implementation**
BTreeMap provides instructive contrast:
- **REQ-4 Performance**: O(log n) guaranteed vs HashMap's O(1) average, O(n) worst-case
- **REQ-5 Collision Handling**: Tree rebalancing vs hash collision resolution
- Both solve REQ-1 (generic storage), different trade-offs

### **AoC Pattern Recognition**
Common Advent of Code scenarios where BTreeMap excels:
- **Time series processing**: Events sorted by timestamp
- **Coordinate systems**: Ordered points for grid traversal
- **Priority queues**: Simple task scheduling with natural ordering

---

## 📊 **Performance Characteristics**

| Operation | HashMap | BTreeMap |
|-----------|---------|----------|
| Insert | O(1) avg | O(log n) |
| Lookup | O(1) avg | O(log n) |
| Remove | O(1) avg | O(log n) |
| Iteration | Unordered | **Sorted** |
| Range Query | ❌ Not available | ✅ O(log n + k) |
| Memory | Higher overhead | Predictable |

---

## 📚 **Complete Daily Study Notes**

For detailed examples, code demonstrations, and practice exercises:

**[[../../daily_study/rust_learning_week2_notes/Day12]]**

The full day's content includes:
- Complete BTreeMap/BTreeSet API reference
- Custom `Ord` implementation examples
- Range query patterns
- Time series and coordinate system applications
- Performance benchmarking comparisons
- Complete runnable examples

---

## 🏆 **Key Takeaways**

1. **Automatic Ordering**: BTreeMap maintains sorted keys with no extra work
2. **Range Superpowers**: Query data ranges efficiently (timestamps, scores, coordinates)
3. **Guaranteed Performance**: O(log n) always - no hash collision degradation
4. **Trait Requirements**: `Ord` (orderability) vs HashMap's `Eq + Hash`
5. **Use Case Clarity**: Ordered data, range queries, predictable performance → BTreeMap

---

## 🔄 **Learning Progression**

- **Previous**: [[daily-study/Day11]] - HashSet set operations
- **Current**: Day 12 - BTreeMap ordered collections
- **Next**: [[daily-study/Day13]] - Advanced iterators for collection transformation

---

## 🎓 **Practice Exercises**

1. Implement word frequency counter with alphabetical output using BTreeMap
2. Create time series processor with range query support
3. Build custom priority queue using BTreeSet with `Reverse` ordering
4. Compare HashMap vs BTreeMap performance for different workloads

---

*Tags: #daily-study #week2 #btreemap #btreeset #ordered-collections #data-structures #collections #performance #intermediate*

*Links: [[daily-study/Day10]] | [[daily-study/Day11]] | [[daily-study/Day13]] | [[Collections MOC]] | [[mission-5]] | [[HashMap Internals]] | [[zettel-index]]*
