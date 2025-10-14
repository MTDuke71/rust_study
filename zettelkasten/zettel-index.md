# 🧠 Zettelkasten Index - Rust Study Workspace

*Master entry point for knowledge navigation and discovery*

---

## 📘 **Link Naming Convention Guide**

**CRITICAL**: To prevent naming collisions, always use these formats:

| Content Type | Full Format | Short Format | Example |
|--------------|-------------|--------------|---------|
| **Daily Study** | `[[daily-study/DayXX]]` | `[[ds-dayXX]]` | `[[daily-study/Day24]]` or `[[ds-day24]]` |
| **AoC Problems** | `[[aoc-YYYY-dayDD]]` | `[[aocYY-DD]]` | `[[aoc-2023-day12]]` or `[[aoc23-12]]` |
| **Missions** | `[[mission-X]]` | `[[mX]]` | `[[mission-6]]` or `[[m6]]` |
| **Rust Book** | `[[rust-book-chX]]` | `[[rb-chX]]` | `[[rust-book-ch8]]` or `[[rb-ch8]]` |
| **Concepts** | `[[lowercase-with-dashes]]` | - | `[[find-all-components]]`, `[[flood-fill]]` |

**❌ AVOID**: Ambiguous forms like `[[Day24]]`, `[[Mission6]]`, `[[Ch8]]`

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
- **Current Day**: See [[../MONTHLY_CALENDAR]] Day activities
- **Latest Study Notes**: [[Daily Study MOC]] → Current Week
- **Recent Work**: Check most recently modified files

### **Mission Track** - Engineering Projects
- [[Mission1 Overview]] or [[mission-1]] - Stack Implementation  
  - **Quick Start**: [[../missions/Mission1/QUICK_REFERENCE|Quick Reference]] - 2-minute ownership essentials
  - **Beginner Guide**: [[../missions/Mission1/SIMPLE_GUIDE|Simple Guide]] - Mental models for ownership
  - **Application**: [[../advanced_examples/Brackets_Ext/README_EXTENDED|Brackets Extended Validator]] - Real-world stack usage
- [[Mission2 Overview]] or [[mission-2]] - Queue & Ring Buffer
- [[Mission3 Overview]] or [[mission-3]] - Binary Search Trees
- [[Mission4 Overview]] or [[mission-4]] - Linked Lists
  - **Tutorial**: [[mission-4-tutorial]] or [[m4-tut]] - Step-by-step linked list construction
- [[Mission5 Overview]] or [[mission-5]] - HashMap & HashSet
  - **Tutorial**: [[mission-5-tutorial]] or [[m5-tut]] - HashMap from scratch guide
- [[Mission6 Overview]] or [[mission-6]] - 2D Grids & Navigation
  - **Coverage**: [[../missions/Mission6/COVERAGE_IMPROVEMENT_LOG]] - Test coverage strategies and improvements
- [[Mission7 Overview]] or [[mission-7]] - Graph Algorithms & Traversal (DFS/BFS)
  - **Tutorial**: [[../tutorials/Mission7_tut/README|Mission7 Tutorial]] - 7-step graph learning progression

### **Daily Study Track** - Systematic Learning
- [[Week 1 Overview]] - Collections Fundamentals
  - [[daily-study/Day10]] or [[ds-day10]] - HashMap basics
  - [[daily-study/Day11]] or [[ds-day11]] - HashSet operations  
  - [[daily-study/Day12]] or [[ds-day12]] - BTreeMap usage
  - [[daily-study/Day13]] or [[ds-day13]] - Advanced iterators
  - [[daily-study/Day14]] or [[ds-day14]] - Error handling patterns
- [[Week 2 Overview]] - Advanced Collections
  - [[daily-study/Day15]] or [[ds-day15]] - Trait basics
  - [[daily-study/Day16]] or [[ds-day16]] - Generics introduction
  - [[daily-study/Day17]] or [[ds-day17]] - Lifetimes fundamentals
  - [[daily-study/Day18]] or [[ds-day18]] - Advanced traits
- [[Week 3 Overview]] - Traits & Type System
- [[Week 4 Overview]] - Applied Problem Solving (AoC)
  - [[daily-study/Day19]] or [[ds-day19]] - Grid navigation
  - [[daily-study/Day20]] or [[ds-day20]] - Coordinate systems
  - [[daily-study/Day21]] or [[ds-day21]] - BFS algorithms
  - [[daily-study/Day22]] or [[ds-day22]] - DFS algorithms
  - [[daily-study/Day23]] or [[ds-day23]] - Grid parsing
  - [[daily-study/Day24]] or [[ds-day24]] - Flood fill & connected components
  - [[daily-study/Day25]] or [[ds-day25]] - Queue applications
- [[Week 5 Overview]] - Error Handling Mastery ✅ **COMPLETED**
  - [[daily-study/Day29]] or [[ds-day29]] - Custom error types
  - [[daily-study/Day30]] or [[ds-day30]] - Error propagation
  - [[daily-study/Day31]] or [[ds-day31]] - anyhow & thiserror crates
  - [[daily-study/Day32]] or [[ds-day32]] - Result combinators
  - [[daily-study/Day33]] or [[ds-day33]] - Panic recovery
  - [[daily-study/Day34]] or [[ds-day34]] - Error handling patterns
  - [[daily-study/Day35]] or [[ds-day35]] - Robust parsing

### **Advent of Code Track**
- [[AoC Patterns MOC]] - Common competitive programming patterns
- **2023 Solutions:**
  - [[aoc-2023-day10]] or [[aoc23-10]] - Pipe maze navigation
  - [[aoc-2023-day12]] or [[aoc23-12]] - Spring arrangement patterns
- **2022 Solutions:**
  - [[aoc-2022-day12]] or [[aoc22-12]] - Hill climbing algorithm
- **2021 Solutions:**
  - [[aoc-2021-day09]] or [[aoc21-09]] - Basin detection (flood fill)
- **2015 Solutions:**
  - [[AoC 2015 MOC]] - Complete 2015 challenge overview
  - [[aoc-2015-day01]] or [[aoc15-01]] - Floor navigation
  - [[aoc-2015-day03]] or [[aoc15-03]] - Grid visited tracking

### **Rust Book Integration**
- [[rust-book-ch1-4-review]] - **Comprehensive Review: Chapters 1-4** (Getting Started, Ownership Foundation)
- [[rust-book-ch5-8-review]] - **Comprehensive Review: Chapters 5-8** (Structs, Enums, Modules, Collections)
- [[rust-book-ch4]] or [[rb-ch4]] - Ownership chapter notes
- [[rust-book-ch5]] or [[rb-ch5]] - Structs chapter notes
- [[rust-book-ch6]] or [[rb-ch6]] - Enums and pattern matching
- [[rust-book-ch7]] or [[rb-ch7]] - Modules and packages
- [[rust-book-ch8]] or [[rb-ch8]] - Common collections
- [[rust-book-ch9]] or [[rb-ch9]] - Error handling
- [[rust-book-ch10]] or [[rb-ch10]] - Generics, traits, lifetimes

### **Key Concept Areas**

**Grid Algorithms:**
- [[find-all-components]] - Connected component detection algorithm (NEW)
- [[flood-fill]] - Single-region modification (recursive, iterative, BFS)
- [[explore-component]] - DFS helper for component exploration
- [[4-connectivity]] - Grid neighbor patterns and connectivity rules

**Data Structures & Collections:**
- [[entry-api-hashmap]] - **Entry API** - Efficient single-lookup HashMap patterns
- [[hashmap-ownership-patterns]] - **HashMap Ownership Patterns** - Owned vs Reference Storage (NEW)
- [[HashMap Internals]] or [[hashmap-internals]] - Hash table implementation details
- [[hashset-operations]] - HashSet usage and patterns
- [[btreemap-vs-hashmap]] - Ordered vs unordered maps comparison
- [[Ring Buffer Overwriting Semantics]] - Circular buffer capacity management strategies

**Language Features:**
- [[While Let Pattern Deep Dive]] - Idiomatic consuming collection patterns
- [[Closures in Rust]] - Anonymous functions with environment capture
- [[Deref Coercion and Automatic Dereferencing]] - Smart pointer unwrapping mechanics
- [[JSON Processing with serde_json]] - JSON parsing, traversal, and type-safe processing

**Advanced Topics:**
- [[Error Handling Deep Dive]] - Comprehensive error handling
- [[Generic Programming]] - Type parameterization techniques
- [[Trait Objects]] - Dynamic dispatch patterns
- [[Unsafe Rust - Raw Pointers and Safety Contracts]] - Building safe abstractions from unsafe code
- [[PhantomData Type Safety Patterns]] - Zero-cost type safety

**Performance & Optimization:**
- [[performance-benchmarking-grid-optimization]] - Comprehensive guide to performance benchmarking
- [[../missions/Mission6/SIZE_HINT_EXPLAINED]] - Iterator size optimization
- [[../missions/Mission6/WHEN_SIZE_HINT_CALLED]] - When optimizations trigger
- Reference: [Algorithmica HPC Guide](https://en.algorithmica.org/hpc/)

## 🔄 Cross-Track Integration

### **Mission ↔ Daily Study Alignment**
- Ownership fundamentals connect to [[../daily_study/rust_learning_week1_notes/Day02_expanded|Day 2 Expanded - Variables & Ownership Deep Dive]]
- Borrowing and references connect to [[../daily_study/rust_learning_week1_notes/Day03_expanded|Day 3 Expanded - Functions & References Deep Dive]]
- HashMap implementation ([[mission-5]]) connects to [[daily-study/Day10]] or [[ds-day10]]
- HashSet wrapper connects to [[daily-study/Day11]] or [[ds-day11]]
- Memory patterns connect to [[daily-study/Day02]] or [[ds-day02]]
- Iterator patterns connect to [[daily-study/Day13]] or [[ds-day13]]
- Grid algorithms ([[mission-6]]) connect to [[daily-study/Day24]] or [[ds-day24]]

### **Mission ↔ Rust Book Connections**  
- Struct design connects to [[rust-book-ch5]] or [[rb-ch5]]
- Generic implementation connects to [[rust-book-ch10]] or [[rb-ch10]]
- Error handling connects to [[rust-book-ch9]] or [[rb-ch9]]
- Module organization connects to [[rust-book-ch7]] or [[rb-ch7]]
- Collections fundamentals connect to [[rust-book-ch8]] or [[rb-ch8]]

### **Progress Tracking**
- **Live Progress**: See [[../MONTHLY_CALENDAR]] for today's activities

## 📦 Smart Pointers & Memory Management

- [[Box Smart Pointer Patterns]] - Heap allocation and ownership management
- [[../tutorials/Mission4_tut/compilation_stages/VISUAL_COMPILATION_PROCESS]] - Visual compilation: Rust → LLVM → Assembly → Machine Code with memory layouts

## 🏗️ Type System Fundamentals

- [[Rust Trinity - Struct Trait Impl]] - The three fundamental building blocks of Rust's type system

## 🎯 Algorithms & Problem Solving

- [[find-all-components]] - Connected component detection with DFS (NEW)
- [[flood-fill]] - Region modification algorithms (recursive, iterative, BFS)
- [[Chebyshev Distance]] - Chessboard/8-connected distance metric for pathfinding
- [[Manhattan Distance]] - 4-connected grid distance metric
- [[Euclidean Distance]] - Continuous space distance metric
- [[A-Star-Algorithm-Deep-Dive]] - Heuristic-based optimal pathfinding
- [[BFS Patterns]] - Breadth-first search for shortest paths
- [[DFS Patterns]] - Depth-first search and backtracking
- [[Graph Network Density]] - Network connectivity metrics and graph analysis

## ⚡ Performance & Optimization

- [[performance-benchmarking-grid-optimization]] - Comprehensive guide to performance benchmarking
- [[zero-cost-abstractions]] - Rust's performance guarantees
- [[amortized-analysis]] - Complexity analysis patterns
- Reference: [Algorithmica HPC Guide](https://en.algorithmica.org/hpc/)

## 📊 System Reports & Analysis

- [[Reports/README|Reports Directory]] - Automated zettelkasten health reports
  - [[Reports/Orphans|Orphaned Files]] - Files needing better integration
  - [[Reports/Broken Links Report|Broken Links]] - Missing concepts to create
  - [[Reports/Link Validation Report|Link Validation]] - Overall link health

---
*Tags: #index #overview #navigation #cross-track #zettelkasten #moc*
*Links: [[../MONTHLY_CALENDAR]] | [[Collections MOC]] | [[Missions MOC]] | [[Daily Study MOC]] | [[README]]*