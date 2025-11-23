# Week 2: Collections & Data Structures (Days 8-14)

**Focus**: Advanced Collection Usage, Iteration Patterns, and Data Processing

---

## 📚 **Week Overview**

Week 2 builds on Week 1 foundations with deeper exploration of Rust's collection types, advanced iteration techniques, and practical data processing patterns. This week emphasizes real-world usage and performance considerations.

### **Core Themes**
- 📊 **Advanced Collections** - Mastering Vec, HashMap, HashSet, BTreeMap
- 🔄 **Iteration Mastery** - Iterator adapters, combinators, and lazy evaluation
- 🔍 **Search Patterns** - Finding, filtering, and transforming data
- ⚠️ **Error Handling** - Robust error management in data pipelines

---

## 🗓️ **Daily Breakdown**

### [[daily-study/Day08]] or [[ds-day08]] - Vector Advanced Patterns
**Topics**: Capacity management, slicing, efficient insertions
- Pre-allocation strategies
- Slice operations and borrowing
- Zero-copy transformations

### [[daily-study/Day09]] or [[ds-day09]] - HashMap Deep Dive
**Topics**: Entry API, custom hashers, performance tuning
- Entry API for upsert patterns
- Avoiding redundant lookups
- Hash function selection

### [[daily-study/Day10]] or [[ds-day10]] - HashSet Advanced Operations
**Topics**: Set algebra, batch operations, custom equality
- Symmetric difference and Cartesian products
- Performance characteristics
- Custom Hash and Eq implementations

### [[daily-study/Day11]] or [[ds-day11]] - BTreeMap Range Operations
**Topics**: Sorted traversal, range queries, ordered iteration
- Split operations and partitioning
- Range-based queries
- Use cases for ordered data

### [[daily_study/rust_learning_week2_notes/Day12]] or [[ds-day12]] - Iterator Combinators
**Topics**: `chain()`, `zip()`, `enumerate()`, `scan()`
- Composing multiple iterators
- Stateful iteration with `scan()`
- Performance implications

### [[daily-study/Day13]] or [[ds-day13]] - Iterator Consumers
**Topics**: `collect()`, `fold()`, `for_each()`, `partition()`
- Terminal operations and consumption
- Building custom collections
- Efficient aggregation patterns

### [[daily-study/Day14]] or [[ds-day14]] - Error Handling with Collections
**Topics**: `Result<Vec<T>, E>`, `collect()` with fallible operations
- Propagating errors through pipelines
- Short-circuiting on first error
- Collecting all errors vs fail-fast

---

## 🎯 **Learning Objectives**

By the end of Week 2, you should be able to:
- ✅ Choose optimal collection types for specific use cases
- ✅ Write efficient iterator chains without intermediate allocations
- ✅ Use Entry API to avoid redundant HashMap lookups
- ✅ Apply range operations on BTreeMap for sorted queries
- ✅ Handle errors gracefully in collection pipelines
- ✅ Understand performance tradeoffs between collection types

---

## 🔗 **Related Missions**

### **Mission 5: HashMap from Scratch**
**Connection**: Deep understanding of hash-based collections
- [[../../missions/Mission5/README|Mission5 README]] - Custom HashMap implementation
- [[../../missions/Mission5_tut/README|Mission5 Tutorial]] - Step-by-step building

### **Mission 2: Ring Buffer Queue**
**Connection**: Efficient collection with fixed capacity
- [[../../missions/Mission2/README|Mission2 README]] - Circular buffer design

### **Mission 6: Grid Systems**
**Connection**: 2D collections and iteration patterns
- [[mission-6]] - Multi-dimensional data

---

## 📖 **Rust Book Integration**

Week 2 concepts align with:
- **[[../../rust_book/Ch8/README|Chapter 8]]** - Common Collections (expanded)
- **[[../../rust_book/Ch13/README|Chapter 13]]** - Iterators and Closures
- **[[../../rust_book/Ch9/README|Chapter 9]]** - Error Handling (advanced)

---

## 🎮 **AoC Applications**

Week 2 patterns are essential for Advent of Code problems:
- **Day 08-09**: Hash tables for frequency counting
- **Day 10-11**: Set operations for puzzle solving
- **Day 12-14**: Iterator chains for data transformation

See [[../../advent_of_code/aoc2015/README|AoC 2015]] for practical applications.

---

## 🚀 **Running Week 2 Examples**

All Day files contain complete runnable examples:

```powershell
# Run individual day
.\scripts\run_md.bat daily_study\rust_learning_week2_notes\Day10.md

# Run all Week 2 examples (PowerShell)
Get-ChildItem daily_study\rust_learning_week2_notes\Day*.md | 
    ForEach-Object { .\run_markdown_code.ps1 $_.FullName }
```

---

## 🔗 **Navigation**

- **⬅️ [[../rust_learning_week1_notes/README|Week 1: Foundations]]** - Previous week
- **📚 [[../README|Daily Study Home]]** - All weeks overview
- **➡️ [[../rust_learning_week3_notes/README|Week 3: Abstractions]]** - Next week
- **🗺️ [[../../zettelkasten/Daily Study MOC]]** - Complete study navigation
- **📅 [[../../MONTHLY_CALENDAR]]** - 30-day learning plan

---

## 🎓 **Key Takeaways**

Week 2 emphasizes **practical mastery** of Rust collections:

> **"Choose the right collection for the job, then use iterators to transform data efficiently"**

Performance Hierarchy:
- **O(1)**: HashMap/HashSet for lookups
- **O(log n)**: BTreeMap/BTreeSet for sorted data
- **O(n)**: Vec for sequential access

Iterator Philosophy:
- **Lazy evaluation** - No work until consumed
- **Zero-cost abstractions** - Same speed as hand-written loops
- **Composable** - Chain operations for readable pipelines

Master these patterns, and you'll write idiomatic, performant Rust code! 🚀

---

*Tags: #week2 #collections #iterators #hashmap #data-structures #daily-study*
