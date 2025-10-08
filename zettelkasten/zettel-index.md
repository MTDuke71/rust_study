# 🧠 Zettelkasten Index - Rust Study Workspace

*Master entry point for knowledge navigation and discovery*

---

## 🌟 Foundational Documents

- [[PROJECT_ORIGIN]] - **The Genesis** - How this learning system was born (founding conversation, philosophy, "The Pact")

---

## 🗺️ Maps of Content (MOCs)

- [[Rust Concepts MOC]] - Core language features and patterns
- [[Missions MOC]] - V-Cycle project implementations  
- [[Collections MOC]] - Data structures and algorithms
- [[Daily Study MOC]] - Progressive learning track
- [[AoC Patterns MOC]] - Competitive programming techniques
- [[AoC 2015 MOC]] - Advent of Code 2015 solutions and learnings

## 🗺️ Navigation Hub

### **Active Learning Tracks**
- [[Collections MOC]] - HashMap, HashSet, BTreeMap, Iterators
- [[Missions MOC]] - V-Cycle engineering projects
- [[Daily Study MOC]] - Week 1-4 systematic progression  
- [[AoC Patterns MOC]] - Competitive programming techniques

### **Find Today's Focus**
- **Current Day**: See [[MONTHLY_CALENDAR]] Day activities
- **Mission Progress**: [[Mission Progress Tracker]]
- **Latest Study Notes**: [[Daily Study MOC]] → Current Week
- **Recent Work**: Check most recently modified files

### **Mission Track** - Engineering Projects
- [[Mission1 Overview]] - Stack Implementation  
  - **Application**: [[../advanced_examples/Brackets_Ext/README_EXTENDED|Brackets Extended Validator]] - Real-world stack usage
- [[Mission2 Overview]] - Queue & Ring Buffer
- [[Mission3 Overview]] - Binary Search Trees
- [[Mission4 Overview]] - Linked Lists
- [[Mission5 Overview]] - HashMap & HashSet
- [[Mission6 Overview]] - 2D Grids & Navigation
  - **Coverage**: [[COVERAGE_IMPROVEMENT_LOG]] - Test coverage strategies and improvements
- [[../missions/Mission7/README|Mission7]] - Graph Algorithms & Traversal (DFS/BFS)
  - **Tutorial**: [[../tutorials/Mission7_tut/README|Mission7 Tutorial]] - 7-step graph learning progression

### **Daily Study Track** - Systematic Learning
- [[Week 1 Notes]] - Collections Fundamentals
- [[Week 2 Notes]] - Advanced Collections  
- [[Week 3 Notes]] - Traits & Type System
- [[Week 4 Notes]] - Applied Problem Solving (AoC)

### **Key Concept Areas**

**Data Structures & Collections:**
- [[HashMap Internals]] - Hash table implementation details
- [[Ring Buffer Overwriting Semantics]] - Circular buffer capacity management strategies

**Language Features:**
- [[While Let Pattern Deep Dive]] - Idiomatic consuming collection patterns
- [[Closures in Rust]] - Anonymous functions with environment capture
- [[Deref Coercion and Automatic Dereferencing]] - Smart pointer unwrapping mechanics

**Advanced Topics:**
- [[Error Handling Deep Dive]] - Comprehensive error handling
- [[Generic Programming]] - Type parameterization techniques
- [[Trait Objects]] - Dynamic dispatch patterns
- [[Unsafe Rust - Raw Pointers and Safety Contracts]] - Building safe abstractions from unsafe code
- [[PhantomData Type Safety Patterns]] - Zero-cost type safety

**Performance & Optimization:**
- [[../missions/Mission6/SIZE_HINT_EXPLAINED]] - Iterator size optimization
- [[../missions/Mission6/WHEN_SIZE_HINT_CALLED]] - When optimizations trigger



## 🔄 Cross-Track Integration

### **Mission ↔ Daily Study Alignment**
- Ownership fundamentals connect to [[../daily_study/rust_learning_week1_notes/Day02_expanded|Day 2 Expanded - Variables & Ownership Deep Dive]]
- Borrowing and references connect to [[../daily_study/rust_learning_week1_notes/Day03_expanded|Day 3 Expanded - Functions & References Deep Dive]]
- HashMap implementation connects to [[Day 10 - HashMap Basics]]
- HashSet wrapper connects to [[Day 11 - HashSet Operations]] 
- Memory patterns connect to [[Day 01 - Ownership Basics]]
- Iterator patterns connect to [[Day 13 - Advanced Iterators]]

### **Mission ↔ Rust Book Connections**  
- Struct design connects to [[Chapter 5 - Structs]]
- Generic implementation connects to [[Chapter 10 - Generics]]
- Error handling connects to [[Chapter 9 - Error Handling]]
- Module organization connects to [[../rust_book/Ch7/crates/README|Chapter 7 - Packages and Crates]]

### **Progress Tracking**
- **Live Progress**: See [[MONTHLY_CALENDAR]] for today's activities
- **Mission Status**: [[Mission Progress Tracker]]
- **Tutorial Alignment**: [[Mission5_tut Step Mapping]]

## 🧪 Knowledge Testing

- [[HashMap Quiz Questions]] - Self-assessment
- [[Implementation Challenges]] - Coding exercises
- [[Connection Verification]] - Link validation

## 📦 Smart Pointers & Memory Management

- [[Box Smart Pointer Patterns]] - Heap allocation and ownership management
- [[../tutorials/Mission4_tut/compilation_stages/VISUAL_COMPILATION_PROCESS]] - Visual compilation: Rust → LLVM → Assembly → Machine Code with memory layouts

## 🏗️ Type System Fundamentals

- [[Rust Trinity - Struct Trait Impl]] - The three fundamental building blocks of Rust's type system

## 🎯 Algorithms & Problem Solving

- [[Chebyshev Distance]] - Chessboard/8-connected distance metric for pathfinding
- [[Manhattan Distance]] - 4-connected grid distance metric
- [[Euclidean Distance]] - Continuous space distance metric
- [[A* Search]] - Heuristic-based optimal pathfinding
- [[BFS Patterns]] - Breadth-first search for shortest paths
- [[DFS Patterns]] - Depth-first search and backtracking

## ⚡ Performance & Optimization

- [[performance-benchmarking-grid-optimization]] - Comprehensive guide to performance benchmarking
  - Pointer chasing vs direct access
  - Cache-friendly access patterns
  - Memory layout optimization (Vec<Vec<T>> vs Vec<T> flat)
  - Common benchmarking pitfalls and solutions
  - Reference: [Algorithmica HPC Guide](https://en.algorithmica.org/hpc/)

---
*Tags: #index #overview #navigation #cross-track #zettelkasten #moc*
*Links: [[MONTHLY_CALENDAR]] | [[Collections MOC]] | [[Missions MOC]] | [[Daily Study MOC]] | [[README]]*