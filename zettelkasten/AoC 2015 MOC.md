# 🎄 Advent of Code 2015 MOC

**Map of Content for AoC 2015 problem implementations and learnings**

## 📋 Overview

Advent of Code 2015 solutions with deep dives into Rust concepts, implementation patterns, and competitive programming techniques.

- **Location**: `advent_of_code/aoc2015/`
- **Summary Document**: [[../advent_of_code/aoc2015/Problem_Statements/summary]]
- **Main README**: [[../advent_of_code/aoc2015/README]]

---

## 🗓️ Completed Days

### **Day 1: Not Quite Lisp**
- String iteration and character counting
- Running sum pattern
- Early termination conditions
- **Implementation Discussion**: [[../../advent_of_code/aoc2015/discussions/day1]] - Complete walkthrough of scaffold usage and Rust vs Python comparison

### **Day 2: I Was Told There Would Be No Math**
- Parsing structured input (`split()`, `parse()`)
- Tuple destructuring
- Mathematical formulas and min/max operations

### **Day 3: Perfectly Spherical Houses in a Vacuum**
- 2D coordinate systems
- HashSet for uniqueness tracking
- Even/odd index splitting with iterators

### **Day 4: The Ideal Stocking Stuffer**
- External crate integration (`md5`)
- Computational optimization
- Loop efficiency patterns

### **Day 5: Doesn't He Have Intern-Elves For This?**
- Regex pattern matching
- Multiple validation rules
- String pattern detection
- **Reference**: [[../../tutorials/Mission5_tut/REGEX_QUICK_REFERENCE|Regex Quick Reference]] - Walkthrough of Day 5 solution

### **Day 6: Probably a Fire Hazard**
- 2D grid data structures
- Coordinate range iteration
- State management (on/off/toggle)
- `saturating_sub()` for safe arithmetic

### **Day 7: Some Assembly Required**
- **Advanced**: Circuit simulation with DAG
- HashMap memoization
- Recursive dependency resolution
- Enum-based instruction modeling
- Professional debug tooling
- 36 comprehensive tests
- **Deep Dive**: [[../advent_of_code/aoc2015/Problem_Statements/summary#Day 7 Deep Dive]]
- **Dependency Tree Analysis**: [[../advent_of_code/aoc2015/examples/day07_debug/DEPENDENCY_TREE_README]] - Visual gate-level dependency analysis for wire 'a'

### **Day 8: Matchsticks**
- **String parsing & escape sequences**
- **Critical Rust Concept**: [[../advent_of_code/aoc2015/examples/day08_rust_string_handling_explained]]
- Character vs byte counting
- UTF-8 encoding challenges
- Part 1: Code length vs memory length
- Part 2: String encoding/escaping

### **Day 9: All in a Single Night**
- **Traveling Salesman Problem (TSP)**
- Permutation generation with Heap's algorithm
- Mission 5 Dictionary integration
- Brute force optimization
- Lifetime management with string slices

### **Day 10: Elves Look, Elves Say**
- **Run-length encoding** and look-and-say sequences
- **Performance Analysis**: [[../advent_of_code/aoc2015/examples/DAY10_BENCHMARK_ANALYSIS]] - Iterative vs Memoized comparison
- **Deep Dive**: [[../advent_of_code/aoc2015/examples/DAY10_MEMOIZATION_WALKTHROUGH]] - Why memoization fails for Day 10
- **Execution Trace**: [[../advent_of_code/aoc2015/examples/DAY10_EXECUTION_TRACE]] - Side-by-side visualization
- **Learning Guide**: [[../advent_of_code/aoc2015/examples/DAY10_LEARNING_GUIDE]] - Step-by-step implementation
- **Quick Reference**: [[../advent_of_code/aoc2015/examples/DAY10_README]] - Commands and summary
- **Problem Statement**: [[../advent_of_code/aoc2015/Problem_Statements/day10]]
- **Integration Summary**: [[../advent_of_code/aoc2015/examples/DAY10_INTEGRATION_SUMMARY]] - Complete knowledge graph integration documentation
- While loop with manual index control
- Benchmarking with Criterion (340ms vs 394ms)
- Understanding when memoization hurts performance (0% cache hit rate)

### **Day 11: Corporate Policy**
- **Documentation Hub**: [[../advent_of_code/aoc2015/examples/Day11_README]] - Complete Day 11 guide and file links
- **Quick Start**: [[../advent_of_code/aoc2015/examples/DAY11_QUICK_REFERENCE]] - Rules summary and implementation order
- **Implementation Guide**: [[../advent_of_code/aoc2015/examples/DAY11_IMPLEMENTATION_GUIDE]] - Step-by-step coding walkthrough
- **Setup Summary**: [[../advent_of_code/aoc2015/examples/DAY11_SETUP_COMPLETE]] - Files created and structure overview
- **Problem Statement**: [[../advent_of_code/aoc2015/Problem_Statements/day11]]
- **Source Code**: [[../advent_of_code/aoc2015/src/solver/day11.rs]]
- **Core Algorithm**: Base-26 counting system for string incrementing ("xx" → "xy" → "xz" → "ya")
- **Validation Rules**: Multi-rule composition (straight sequences, forbidden chars, duplicate pairs)
- **Performance**: Smart range skipping optimization to avoid generating invalid passwords

### **Day 12: JSAbacusFramework.io**
- **Deep Dive**: [[JSON Processing with serde_json]] - Complete guide to JSON parsing and traversal
- JSON parsing with `serde_json::Value`
- Recursive data structure traversal
- Pattern matching on JSON types
- Conditional filtering (Part 2: "red" object exclusion)
- Dynamic typing in statically-typed language

### **Day 13: Knights of the Dinner Table**
- **Traveling Salesman Problem** with circular seating constraints
- **Complete Analysis**: [[../advent_of_code/aoc2015/examples/day13_analysis]] - TSP algorithms, graph theory, and mathematical proofs
- Weighted directed complete adjacency graph
- Heap's algorithm for permutation generation
- Mathematical symmetry exploitation (9× performance optimization)
- Global vs. local optimization strategies

### **Day 14: Reindeer Olympics**
- **Cyclic Behavior Simulation** and mathematical optimization
- **Complete Analysis**: [[../advent_of_code/aoc2015/examples/day14_analysis]] - Comprehensive implementation guide for cyclic systems
- **Complete Summary**: [[../advent_of_code/aoc2015/examples/DAY14_COMPLETE_SUMMARY]] - Full problem walkthrough
- **Documentation Guide**: [[../advent_of_code/aoc2015/examples/DOCUMENTATION_ENHANCEMENTS]] - Best practices for AoC documentation
- **Graphics Guide**: [[../advent_of_code/aoc2015/examples/GRAPHICS_GUIDE]] - Visualization techniques for algorithm analysis
- State machine implementation for flight/rest cycles
- Algorithmic complexity comparison (O(n×c) vs O(n×m))
- Real-time leader tracking with different scoring systems
- Performance optimization through cycle mathematics

---

## 🎓 Key Learnings by Concept

### **String Processing**
- [[../advent_of_code/aoc2015/examples/day08_rust_string_handling_explained]] - Compile-time vs runtime string handling
- Day 8: Escape sequence parsing (`\\`, `\"`, `\xHH`)
- Day 8: UTF-8 encoding vs byte arrays (C-style strings)
- Day 11: [[../advent_of_code/aoc2015/examples/DAY11_IMPLEMENTATION_GUIDE]] - Base-26 counting and string incrementing with wrap-around logic
- Day 11: Character arithmetic (`'a' as u8 + 1`) for consecutive sequence detection

### **Data Structures**
- Day 3: HashSet for coordinate tracking
- Day 6: 2D grids and array indexing
- Day 7: HashMap for memoization
- Day 12: JSON recursive traversal with `serde_json::Value`

### **Performance Optimization**
- Day 4: Computational efficiency
- Day 7: Memoization patterns
- Day 8: Character counting algorithms
- Day 10: [[../advent_of_code/aoc2015/examples/DAY10_BENCHMARK_ANALYSIS]] - Benchmarking iterative vs memoized approaches
- Day 10: Understanding when caching hurts performance
- Day 11: [[../advent_of_code/aoc2015/examples/DAY11_QUICK_REFERENCE]] - Smart range skipping and rule ordering optimization

### **Parsing & Validation**
- Day 2: Structured input parsing
- Day 5: Regex patterns
- Day 8: Custom escape sequence parser
- Day 10: Run-length encoding
- Day 11: Multi-rule validation composition - combining 3 independent password rules
- Day 12: [[JSON Processing with serde_json]] - Dynamic JSON parsing and conditional filtering

### **Graph & Dependencies**
- Day 7: DAG traversal
- Day 7: Dependency resolution
- Day 7: Cycle detection
- [[../advent_of_code/aoc2015/examples/day07_debug/DEPENDENCY_TREE_README]] - Complete dependency tree analysis for circuit gates

### **Algorithm Patterns**
- Day 11: Base-N counting systems (base-26 for alphabetic incrementing)
- Day 11: Multi-constraint satisfaction with early termination
- Day 11: Generation with validation loop - increment until valid pattern found
- Day 12: Recursive tree traversal with conditional filtering
- Day 9: TSP with permutation generation
- Day 13: TSP variant with circular constraints and symmetry optimization
- Day 14: Cyclic state machines with mathematical optimization vs simulation

### **Algorithms & Problem Solving**
- Day 9: Heap's algorithm for permutations
- Day 10: [[../advent_of_code/aoc2015/examples/DAY10_MEMOIZATION_WALKTHROUGH]] - Run-length encoding and sequence transformation
- Day 10: [[../advent_of_code/aoc2015/examples/DAY10_EXECUTION_TRACE]] - Iterative vs recursive execution patterns

---

## 🔗 Integration with Learning Tracks

### **Mission Connections**
- Day 7 → [[Mission5 Overview]] - HashMap memoization patterns
- Day 8 → String parsing techniques
- Day 3 → [[Collections MOC]] - HashSet usage
- Day 9 → [[Mission5 Overview]] - Dictionary for distance matrix
- Day 10 → [[Mission5 Overview]] - MemoCache usage (educational example of when NOT to cache)

### **Daily Study Connections**
- Day 8 → [[daily-study/Day09]] - String vs &str
- Day 7 → [[daily-study/Day10]] - Key-value storage
- Day 3 → [[daily-study/Day11]] - Set operations
- Day 10 → [[../daily_study/rust_learning_week2_notes/Day10]] - HashMap and iteration patterns
- Day 11 → [[../daily_study/rust_learning_week2_notes/Day09]] - Password validation patterns and string rule composition

### **Rust Concepts**
- [[Error Handling Deep Dive]] - AoC input validation
- [[Collections MOC]] - Data structure selection
- [[../advent_of_code/aoc2015/examples/day08_rust_string_handling_explained]] - String internals

---

## 📊 Problem Categories

Based on [[../advent_of_code/aoc2015/Problem_Statements/summary]]:

- **String Processing**: Days 1, 8, 10
- **Mathematical**: Days 2
- **Simulation**: Days 3, 6, 10
- **Cryptographic**: Day 4
- **Pattern Matching**: Day 5
- **Graph Algorithms**: Day 7, 9
- **Parsing**: Day 8
- **Encoding**: Day 8 (Part 2), Day 10 (Run-length encoding)
- **Optimization**: Day 9 (TSP)
- **Brute Force**: Day 4, 9

---

## 🎯 Competitive Programming Techniques

### **AoC-Specific Patterns**
- Reading and parsing input files
- Handling edge cases (empty strings, zero values)
- Part 1 → Part 2 evolution patterns
- Testing with example data

### **Performance Considerations**
- Pre-allocation strategies
- Memoization for expensive computations
- Iterator efficiency
- Avoiding unnecessary allocations
- **Benchmarking**: [[../advent_of_code/aoc2015/examples/DAY10_README]] - Day 10 demonstrates when NOT to memoize
- Cache hit rate analysis
- Simple solutions often outperform complex ones

---

## 🧪 Testing Strategies

### **Day 7 Excellence** (36 tests)
- Requirement-based test naming (`req1_*`, `req2_*`)
- Integration tests with real data
- Edge case coverage
- Performance benchmarking

### **Day 8 Comprehensive** (18 tests)
- Escape sequence validation
- UTF-8 encoding edge cases
- Part 1 and Part 2 coverage
- Example-driven development

### **Day 10 Performance Testing**
- **Criterion Benchmarking**: [[../advent_of_code/aoc2015/examples/DAY10_BENCHMARK_ANALYSIS]]
- Release mode performance comparison
- Iterative vs recursive with memoization
- Cache effectiveness analysis
- 4 unit tests for correctness validation

---

## 📈 Progress Tracking

**Days Completed**: 10 / 25
**Test Coverage**: Excellent (Day 7: 36 tests, Day 8: 18 tests, Day 10: 4 tests + benchmarks)
**Documentation**: Comprehensive with problem summaries and deep dives
**Performance Analysis**: Day 10 benchmarking suite with Criterion

---

---

## 🎯 **AoC 2024 Problem References**

*Problem statements for cross-year pattern analysis and learning*

### **Early December 2024 Problems**
- [[../advent_of_code/aoc2024/Problem_Statements/day01]] - Historian Hysteria (2024) - List processing and distance calculations
- [[../advent_of_code/aoc2024/Problem_Statements/day02]] - Red-Nosed Reports (2024) - Sequence validation and safety analysis
- [[../advent_of_code/aoc2024/Problem_Statements/day03]] - Mull It Over (2024) - String parsing and instruction processing

**Note**: Python implementations exist in excluded `2024py/` directory. These problem statements serve as:
- Cross-year pattern comparison with AoC 2015
- Future Rust implementation reference
- Problem-solving technique analysis

---

## 🔗 **Related MOCs**

- [[AoC Patterns MOC]] - Cross-year competitive programming patterns
- [[Collections MOC]] - Data structure implementations  
- [[Rust Concepts MOC]] - Core language features

---

*Tags: #aoc #aoc2015 #aoc2024 #competitive-programming #advent-of-code #problem-solving #rust-learning*

*Links: [[zettel-index]] | [[AoC Patterns MOC]] | [[Collections MOC]] | [[Rust Concepts MOC]]*
