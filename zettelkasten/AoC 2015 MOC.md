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
- While loop with manual index control
- Benchmarking with Criterion (340ms vs 394ms)
- Understanding when memoization hurts performance (0% cache hit rate)

Day 11:
[[Day11_README]]

---

## 🎓 Key Learnings by Concept

### **String Processing**
- [[../advent_of_code/aoc2015/examples/day08_rust_string_handling_explained]] - Compile-time vs runtime string handling
- Day 8: Escape sequence parsing (`\\`, `\"`, `\xHH`)
- Day 8: UTF-8 encoding vs byte arrays (C-style strings)

### **Data Structures**
- Day 3: HashSet for coordinate tracking
- Day 6: 2D grids and array indexing
- Day 7: HashMap for memoization

### **Performance Optimization**
- Day 4: Computational efficiency
- Day 7: Memoization patterns
- Day 8: Character counting algorithms
- Day 10: [[../advent_of_code/aoc2015/examples/DAY10_BENCHMARK_ANALYSIS]] - Benchmarking iterative vs memoized approaches
- Day 10: Understanding when caching hurts performance

### **Parsing & Validation**
- Day 2: Structured input parsing
- Day 5: Regex patterns
- Day 8: Custom escape sequence parser
- Day 10: Run-length encoding

### **Graph & Dependencies**
- Day 7: DAG traversal
- Day 7: Dependency resolution
- Day 7: Cycle detection
- [[../advent_of_code/aoc2015/examples/day07_debug/DEPENDENCY_TREE_README]] - Complete dependency tree analysis for circuit gates
- Day 9: TSP with permutation generation

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
- Day 8 → [[Day 09 - String Patterns]] - String vs &str
- Day 7 → [[Day 10 - HashMap Basics]] - Key-value storage
- Day 3 → [[Day 11 - HashSet Operations]] - Set operations
- Day 10 → [[../daily_study/rust_learning_week2_notes/Day10]] - HashMap and iteration patterns

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

## 🔗 **Related MOCs**

- [[AoC Patterns MOC]] - Cross-year competitive programming patterns
- [[Collections MOC]] - Data structure implementations
- [[Rust Concepts MOC]] - Core language features

---

*Tags: #aoc #aoc2015 #competitive-programming #advent-of-code #problem-solving #rust-learning*

*Links: [[zettel-index]] | [[AoC Patterns MOC]] | [[Collections MOC]] | [[Rust Concepts MOC]]*
