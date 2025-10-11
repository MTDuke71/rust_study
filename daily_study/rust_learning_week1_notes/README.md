# Week 1: Foundations (Days 1-7)

**Focus**: Ownership, Borrowing, Lifetimes, and Pattern Matching

---

## 📚 **Week Overview**

Week 1 establishes the foundational concepts of Rust's ownership system, which makes memory safety guarantees without a garbage collector. These concepts are essential for all Rust programming.

### **Core Themes**
- 🔐 **Ownership** - Understanding move semantics and value ownership
- 📦 **Borrowing** - Immutable and mutable references
- ⏱️ **Lifetimes** - Ensuring references remain valid
- 🎯 **Pattern Matching** - Destructuring and control flow

---

## 🗓️ **Daily Breakdown**

### [[daily-study/Day01]] or [[ds-day01]] - Vec and Basic Collections
**Topics**: Vector basics, push/pop operations, iteration patterns
- Introduction to owned collections
- Basic CRUD operations
- Memory layout and capacity management

### [[daily-study/Day02]] or [[ds-day02]] - HashMap Fundamentals  
**Topics**: Key-value storage, hash functions, lookup operations
- Creating and populating HashMaps
- Entry API patterns
- Ownership with HashMap keys and values
- 📘 **Expanded**: [[Day02_expanded]] - Additional examples and edge cases

### [[daily-study/Day03]] or [[ds-day03]] - HashSet Operations
**Topics**: Unique value storage, set operations, deduplication
- Set creation and membership testing
- Union, intersection, difference operations
- Practical deduplication patterns
- 📘 **Expanded**: [[Day03_expanded]] - Advanced set algorithms

### [[daily-study/Day04]] or [[ds-day04]] - BTreeMap Sorted Storage
**Topics**: Ordered maps, range queries, sorted iteration
- Differences from HashMap (O(log n) vs O(1))
- Range operations and ordered traversal
- Use cases for sorted data

### [[daily-study/Day05]] or [[ds-day05]] - Iterator Patterns
**Topics**: Lazy evaluation, iterator adapters, collection pipelines
- `map()`, `filter()`, `fold()` patterns
- Iterator chaining for data transformation
- Performance benefits of lazy evaluation

### [[daily-study/Day06]] or [[ds-day06]] - Error Handling Basics
**Topics**: `Result<T, E>`, `Option<T>`, `?` operator
- Representing fallible operations
- Error propagation patterns
- Converting between Option and Result

### [[daily-study/Day07]] or [[ds-day07]] - Pattern Matching Deep Dive
**Topics**: `match` expressions, destructuring, guard clauses
- Exhaustive pattern matching
- Destructuring structs and enums
- Pattern guards and advanced matching

---

## 🎯 **Learning Objectives**

By the end of Week 1, you should be able to:
- ✅ Explain Rust's ownership rules and why they prevent memory bugs
- ✅ Use borrowing correctly (immutable and mutable references)
- ✅ Choose appropriate collection types (Vec, HashMap, HashSet, BTreeMap)
- ✅ Write iterator chains for data transformation
- ✅ Handle errors idiomatically with Result and Option
- ✅ Use pattern matching for control flow and destructuring

---

## 🔗 **Related Missions**

### **Mission 1: Stack Implementation**
**Connection**: Ownership and move semantics in practice
- [[../../missions/Mission1/README|Mission1 README]] - Stack using Vec
- [[../../missions/Mission1/SIMPLE_GUIDE|Mission1 SIMPLE_GUIDE]] - Ownership tutorial

### **Mission 4: Linked List**
**Connection**: Advanced borrowing with mutable references
- [[../../missions/Mission4/README|Mission4 README]] - Linked structures

---

## 📖 **Rust Book Integration**

Week 1 concepts align with:
- **[[../../rust_book/Ch4/README|Chapter 4]]** - Understanding Ownership
- **[[../../rust_book/Ch6/README|Chapter 6]]** - Enums and Pattern Matching
- **[[../../rust_book/Ch8/README|Chapter 8]]** - Common Collections
- **[[../../rust_book/Ch9/README|Chapter 9]]** - Error Handling

---

## 🚀 **Running Week 1 Examples**

All Day files contain complete runnable examples. Execute them with:

```powershell
# Run individual day
.\scripts\run_md.bat daily_study\rust_learning_week1_notes\Day01.md

# Or use PowerShell directly
.\run_markdown_code.ps1 daily_study\rust_learning_week1_notes\Day05.md
```

---

## 🔗 **Navigation**

- **📚 [[../README|Daily Study Home]]** - All weeks overview
- **➡️ [[../rust_learning_week2_notes/README|Week 2: Collections]]** - Next week
- **🗺️ [[../../zettelkasten/Daily Study MOC]]** - Complete study navigation
- **📅 [[../../MONTHLY_CALENDAR]]** - 30-day learning plan

---

## 🎓 **Key Takeaways**

Week 1 establishes **the foundation** for all Rust programming:

> **"Ownership enables memory safety without garbage collection"**

The concepts learned this week—ownership, borrowing, and lifetimes—are what make Rust unique. They prevent entire classes of bugs at compile time:
- ❌ Use-after-free
- ❌ Double-free
- ❌ Data races
- ❌ Null pointer dereferences

Master these fundamentals, and the rest of Rust becomes significantly easier! 🚀

---

*Tags: #week1 #foundations #ownership #borrowing #collections #daily-study*
