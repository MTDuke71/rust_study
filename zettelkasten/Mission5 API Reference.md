# Mission5 API Reference

**🔗 This is a redirect to the main Mission5 API documentation**

See: [[../missions/Mission5/README|Mission5 README.md]] - Scroll to "API Documentation" section

## Quick API Links

### **Core Types**

- `HashMap<K, V>` - Generic hash map implementation
- `HashSet<T>` - Set wrapper using HashMap
- `MemoCache<K, V>` - Memoization cache for dynamic programming

### **Key Operations**

- `insert(key, value)` - Add or update entry (O(1) average)
- `get(&key) -> Option<&V>` - Retrieve value (O(1) average)
- `remove(&key) -> Option<V>` - Delete entry (O(1) average)
- `contains_key(&key) -> bool` - Check existence
- `entry(key)` - Entry API for efficient updates

### **Iteration**

- `keys()` - Iterator over keys
- `values()` - Iterator over values
- `iter()` - Iterator over key-value pairs

## Related Documentation

**Mission5 Implementation:**

- [[Mission5 README|../missions/Mission5/README]] - Complete V-Cycle documentation
- [[mission-5]] - Learning integration and requirements

**Mission5 Tutorial:**

- [[Mission5_tut Overview]] - Step-by-step guided learning
- [[Mission5_tut Final Review]] - Mastery checklist

**Zettelkasten Concepts:**

- [[HashMap Deep Dive]] - Internal implementation details
- [[Hash Function Design]] - Creating effective hash functions
- [[Collision Resolution]] - Handling hash conflicts
- [[Generic Programming]] - Type parameters and constraints

**Collections Context:**

- [[Collections MOC]] - All data structure concepts
- [[HashMap Internals]] - Deep technical dive
- [[HashSet Wrapper Pattern]] - Set abstraction

**Daily Study Integration:**

- [[daily-study/Day10]] - Foundational concepts
- [[daily-study/Day11]] - Set operations
- [[daily-study/Day13]] - Iterator patterns

**Rust Book:**

- [[Chapter 8|../rust_book/Ch8/README]] - Collections chapter
- [[Chapter 10|../rust_book/Ch10/README]] - Generics and traits

**Learning Path:**

- [[Week 2 Overview]] - HashMap week overview
- [[learning-plan|../MONTHLY_CALENDAR]] - Daily integration schedule

---

*Tags: #mission5 #api-reference #hashmap #hashset #documentation #redirect*
*Links: [[zettel-index]] | [[mission-5]] | [[Collections MOC]]]*
