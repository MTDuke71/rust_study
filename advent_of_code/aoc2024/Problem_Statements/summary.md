# Advent of Code 2024 - Problem Summary

This document provides a categorized overview of all Advent of Code 2024 problems, organized by day with problem types for both parts.

## Problem Categories

- **Advanced Pattern Matching**: Complex pattern constraints, non-overlapping patterns
- **Brute Force**: Exhaustive search through solution space
- **Cellular Automaton**: Conway's Game of Life, state evolution, neighbor counting, grid simulation
- **Combinatorial Optimization**: Subset sum, container packing, constrained combination enumeration
- **Conditional Logic**: Property-based filtering, range-based matching, rule-based comparisons
- **Cryptographic**: Hash functions, encryption, cryptographic puzzles
- **Data Structures**: Working with arrays, lists, sets, maps
- **Encoding**: String encoding, character escaping
- **Graph Algorithms**: Graph traversal, shortest path, connectivity analysis
- **Greedy Algorithms**: Optimal greedy strategies, reverse optimization, exploiting problem structure
- **Mathematical**: Arithmetic calculations, formulas, geometric problems
- **Number Theory**: Divisor sums, highly composite numbers, sieve algorithms, multiplicative functions
- **Optimization**: Finding minimum/maximum values
- **Parsing**: Escape sequence parsing, character-level analysis
- **Pattern Matching**: Regular expressions, string validation, substring detection
- **Real-time Analysis**: Temporal scoring, moment-by-moment leader tracking, time-dependent calculations
- **Search**: Informed search algorithms, A* search, heuristics, state space exploration
- **Search/Traversal**: Finding positions, tracking states
- **Simulation**: State tracking, following instructions step-by-step
- **String Processing**: Character manipulation, parsing, pattern matching

---

## Day-by-Day Summary

### Day 1: Historian Hysteria
**Title**: Historian Hysteria  
**Part 1 Type**: Data Structures + Mathematical  
**Part 1 Description**: Calculate total distance between two lists by pairing smallest elements after sorting  
**Part 2 Type**: Data Structures + Mathematical  
**Part 2 Description**: Calculate similarity score by multiplying each left number by its frequency in right list  
**Key Concepts**: List processing, sorting algorithms, frequency counting with HashMap, iterator chains  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Two-part escalation (distance metric → similarity metric)
- **Data Structure**: Vec for sorting, HashMap for frequency counting
- **Complexity**: Part 1: O(n log n) sorting, Part 2: O(n) frequency map construction + O(n) scoring
- **AoC Theme**: Classic "process pairs of lists" with different metrics

**🦀 Rust Conversion Highlights**:
- **From Python manual loops** → **Functional iterator chains** (`zip`, `map`, `fold`, `sum`)
- **From exception handling** → **Explicit `Result<T, E>` with detailed error context**
- **From dynamic typing** → **Compile-time type safety with pattern matching**
- **From multiple data copies** → **In-place sorting and zero-cost abstractions**

**Performance**: Python→Rust identical results, significantly improved safety and performance



---

## Problem Type Distribution (Available Days)

| Category | Part 1 Count | Part 2 Count |
|----------|--------------|--------------|
| Advanced Pattern Matching | 0 | 0 |
| Brute Force | 0 | 0 |
| Cellular Automaton | 0 | 0 |
| Combinatorial Optimization | 0 | 0 |
| Conditional Logic | 0 | 0 |
| Cryptographic | 0 | 0 |
| Data Structures | 1 | 1 |
| Encoding | 0 | 0 |
| Graph Algorithms | 0 | 0 |
| Greedy Algorithms | 0 | 0 |
| Mathematical | 1 | 1 |
| Number Theory | 0 | 0 |
| Optimization | 0 | 0 |
| Parsing | 0 | 0 |
| Pattern Matching | 0 | 0 |
| Real-time Analysis | 0 | 0 |
| Search | 0 | 0 |
| Search/Traversal | 0 | 0 |
| Simulation | 0 | 0 |
| String Processing | 0 | 0 |

## Implementation Notes

### Common Patterns Observed:
1. **Input Parsing**: Structured text parsing with error handling (Day 1: whitespace-separated integers)
2. **Two-Part Escalation**: Part 2 transforms Part 1's approach (Day 1: distance → similarity)
3. **List Processing**: Sort and pair operations (Day 1: smallest-to-smallest pairing)
4. **Frequency Analysis**: Count occurrences for scoring (Day 1: HashMap frequency counting)
5. **Functional Pipelines**: Iterator chains for data transformation (Day 1: parsing → unzipping → processing)

### Rust-Specific Considerations:
- **Day 1**: Excellent introduction to functional error handling with `Result<T, E>`, iterator combinators (`zip`, `fold`, `sum`), and pattern matching for safe parsing. Demonstrates HashMap construction with functional approach vs Python's Counter. 
---

## Adding New Days

To add a new day to this summary:

1. **Read the problem statement**
2. **Identify the core algorithm type** for each part
3. **Add entry following the format above**
4. **Update the distribution table**
5. **Note any new patterns or Rust learning opportunities**

### Template for New Days:
```markdown
### Day X: [Problem Title]
**Title**: [Problem Title]  
**Part 1 Type**: [Category]  
**Part 1 Description**: [Brief description]  
**Part 2 Type**: [Category]  
**Part 2 Description**: [Brief description]  
**Key Concepts**: [Relevant programming concepts]
```

---

*Last Updated: November 3, 2025*
*Days Implemented: 1*
*Days Available: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25*

---
*Tags: #aoc #2024 #problem-analysis #patterns #rust-conversion #algorithm-learning*
*Links: [[../../../zettelkasten/AoC Patterns MOC]] | [[../../../zettelkasten/AoC Integration]] | [[../../../zettelkasten/Priority Queue Patterns]]*