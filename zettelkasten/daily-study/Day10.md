# Day 10 - HashMap Basics

---

## 🎯 Quick Reference

**Topic**: HashMap Basics - Key-value storage, borrowing, and hash table fundamentals

**Location**: `daily_study/rust_learning_week2_notes/Day10.md`

**Learning Context**: Day 10 introduces HashMap<K, V> for efficient key-value lookups, building toward Mission5's custom HashMap implementation.

---

## 📋 Key Concepts Covered

From the daily study file, Day 10 covers:

1. **HashMap Basics** - Creating and using HashMap<K, V>
2. **Insert and Access** - Adding and retrieving key-value pairs
3. **Ownership in HashMap** - How HashMap takes ownership of keys and values
4. **Borrowing Keys** - Using references for lookups without taking ownership
5. **Entry API** - or_insert and other entry methods
6. **Iteration** - Iterating over keys, values, and entries
7. **Practical Patterns** - Frequency counting and data aggregation

---

## 🔗 Related Concepts

### Zettelkasten Deep Dives
- [[HashMap Internals]] - Deep dive into hash table implementation
- [[Mission5 Overview]] - Custom HashMap implementation project
- [[Generic Programming]] - HashMap<K, V> generic design

### Daily Study Progression
- [[Day 09 - String Patterns]] - Previous day (string manipulation)
- **Current**: [[../daily_study/rust_learning_week2_notes/Day10|Day 10 - HashMap Basics]]
- [[Day 11 - HashSet Operations]] - Next day (unique collections)

### Mission Applications
- [[Mission5 Overview]] - Custom HashMap<K, V> implementation
- [[Mission5_tut Overview]] - Tutorial series for HashMap
- [[AoC Patterns MOC]] - HashMap usage in competitive programming

---

## 🚀 Quick Start

### Basic HashMap Usage
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Alice".to_string(), 100);
scores.insert("Bob".to_string(), 150);

// Access with borrowing
let alice_score = scores.get("Alice");  // Returns Option<&i32>

// Entry API for conditional insertion
scores.entry("Charlie".to_string())
    .or_insert(75);  // Insert if not exists
```

### Ownership Patterns
```rust
// HashMap takes ownership of String keys/values
let mut map = HashMap::new();
let key = String::from("key");
let value = String::from("value");
map.insert(key, value);  // key and value moved into map

// Use references for lookups
let result = map.get("key");  // Borrows &str for lookup
```

---

## 📚 Full Content

**See the complete daily study file for:**
- Detailed explanations of HashMap internals
- Advanced Entry API patterns
- Iteration techniques
- Performance considerations
- Practical examples and exercises

**Direct Link**: [[../daily_study/rust_learning_week2_notes/Day10|📖 Day 10 - HashMap Basics (Full Content)]]

---

*Tags: #hashmap #collections #week2 #quick-ref*