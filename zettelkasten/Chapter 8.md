# Chapter 8: Common Collections

**Redirect**: This page provides a summary and redirect to the comprehensive Chapter 8 documentation.

## 🎯 **Quick Overview**

Chapter 8 introduces Rust's **common collections** - essential data structures for managing dynamic data:

1. **Vector (`Vec<T>`)** - Growable arrays with contiguous memory
2. **String (`String`)** - UTF-8 encoded text with ownership
3. **Hash Map (`HashMap<K, V>`)** - Key-value associations with fast lookups

## 🔗 **Full Documentation**

For comprehensive Chapter 8 content including:
- Detailed explanations of all three collection types
- Code examples and usage patterns
- Performance characteristics and best practices
- Integration with missions and daily study
- Project structure and runnable examples

**→ [[../rust_book/Ch8/README.md|View Complete Chapter 8 Documentation]]**

## 📚 **Key Learning Points**

### **Vector (`Vec<T>`)**
- Dynamic arrays stored on the heap
- O(1) amortized append, O(1) indexed access
- Use `vec!` macro for initialization
- Safe indexing with `get()` vs panicking `[]`

### **String (`String`)**
- UTF-8 encoded, owned text
- No direct indexing (use `.chars()` or slicing)
- Concatenation with `+` operator (moves left operand)
- Use `format!` macro for complex string building

### **HashMap (`HashMap<K, V>`)**
- Key-value storage with hashing
- Average O(1) insert, get, remove operations
- Use `.entry()` API for conditional updates
- Ownership rules apply to keys and values

## 🎯 **Mission Applications**

Collections are fundamental to all missions:
- **[[Mission1 Overview]]** - Stack using Vec<T>
- **[[Mission2 Overview]]** - Queue implementations with Vec
- **[[Mission5 Overview]]** - Custom HashMap implementation
- **[[Mission6 Overview]]** - Grid storage with Vec<Vec<T>>

## 📖 **Daily Study Integration**

- **[[daily-study/Day10]]** - HashMap fundamentals
- **[[daily-study/Day11]]** - HashSet operations
- **[[daily_study/rust_learning_week2_notes/Day12]]** - BTreeMap ordered collections
- **[[Week 2 Overview]]** - Collections mastery week

## 🔗 **Related Concepts**

- **[[Collections MOC]]** - Overview of all collection types
- **[[Unicode, UTF-8, and Rust]]** - Deep dive into string encoding
- **[[Heap Allocation]]** - Dynamic memory management
- **[[Performance Optimization]]** - Collection performance tips

---

*Tags: #chapter8 #collections #redirect #rust-book #vector #string #hashmap*
*Links: [[zettel-index]] | [[../rust_book/Ch8/README.md|Complete Chapter 8]] | [[Collections MOC]] | [[Mission1 Overview]] | [[Mission2 Overview]] | [[Mission5 Overview]] | [[Mission6 Overview]] | [[daily-study/Day10]] | [[daily-study/Day11]] | [[daily_study/rust_learning_week2_notes/Day12]] | [[Week 2 Overview]] | [[Unicode, UTF-8, and Rust]] | [[Heap Allocation]] | [[Performance Optimization]]*
