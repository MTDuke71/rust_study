# Advent of Code 2025 - Problem Summary

This document provides a categorized overview of all Advent of Code 2025 problems, organized by day with problem types for both parts.

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

### Day 1: Secret Entrance
**Title**: Secret Entrance  
**Part 1 Type**: Simulation + Mathematical  
**Part 1 Description**: Simulate safe dial rotations (L/R with distance) on circular 0-99 positions, count how many times dial ends at position 0  
**Part 2 Type**: Simulation + Mathematical  
**Part 2 Description**: Count every time dial points at 0 (both final positions AND during rotations), using password method 0x434C49434B  
**Key Concepts**: Circular arithmetic, modular mathematics, boundary crossing detection, signed integer arithmetic, edge case handling for position 0  

**🧩 Algorithm Analysis**:
- **Problem Pattern**: Simulation escalation (endpoint counting → comprehensive zero detection including intermediate states)
- **Data Structure**: Simple state tracking with signed integers, boundary crossing arithmetic
- **Complexity**: Part 1: O(n) single pass through rotations, Part 2: O(n) with boundary crossing calculations
- **AoC Theme**: "Safe dial manipulation" with classic Part 2 escalation (simple counting → complex state tracking)

**🦀 Rust Implementation Highlights**:
- **Circular arithmetic safety** → **Modular operations with overflow protection** using signed `i32` arithmetic
- **Edge case handling** → **Special position 0 logic** preventing overcounting when starting from zero
- **Boundary detection** → **Division-based crossing counts** `(dial / 100).abs()` for natural zero detection
- **Error resilience** → **Comprehensive parsing with `anyhow::Result`** and detailed error context

**Debugging Journey**: 
- **Initial overcounting (6037)** → Fixed edge cases when starting from position 0
- **Overcorrection (5015)** → Balanced approach using signed arithmetic and boundary crossing
- **Final solution (5941)** → Elegant division-based detection with special Left-from-0 handling

**🔧 Key Implementation Insights**:
- **Special case**: `if dial == 0 && rotation.direction == Direction::Left { dial = 100; }` prevents edge case issues
- **Boundary crossing**: `zero_count += (dial / 100).abs();` naturally detects zero passages
- **Wrap handling**: `if dial < 1 { zero_count += 1; }` catches wrap-around cases
- **Normalization**: Final `dial % 100` with negative correction ensures 0-99 range

**Performance**: Pure Rust implementation, no Python comparison available for AoC 2025

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
| Data Structures | 0 | 0 |
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
| Simulation | 1 | 1 |
| String Processing | 0 | 0 |

## Implementation Notes

### Common Patterns Observed

### Early 2025 Themes
- **Mathematical simulations**: Day 1 demonstrates circular arithmetic challenges with edge case complexity
- **Boundary detection**: Division-based algorithms for detecting state transitions (dial crossing zero)
- **Signed arithmetic benefits**: Using `i32` over `u32` simplifies wrap-around and negative value handling

### Rust-Specific Considerations

- **Day 1**: Demonstrates signed integer arithmetic advantages for circular problems, comprehensive error handling with `anyhow::Result`, and the importance of special case handling for boundary conditions (position 0)

---

## Adding New Days


To add a new day to this summary:

1. **Read the problem statement**
2. **Identify the core algorithm type** for each part
3. **Add entry following the format above**
4. **Update the distribution table**
5. **Note any new patterns or Rust learning opportunities**
6. **⚠️ CRITICAL: Verify Rust-specific claims against actual implementation code**

### Documentation Quality Lesson Learned

**Always inspect actual code before documenting patterns.**  This highlights the importance of **evidence-based documentation** over **assumption-based documentation**.

**Verification Checklist**:

- [ ] Read the actual Rust implementation file
- [ ] Document patterns that are **actually present** in the code
- [ ] Note deliberate trade-offs (e.g., performance vs functional style)
- [ ] Compare claimed patterns against `grep`/search results in codebase

### Template for New Days

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

*Last Updated: December 1, 2025*
*Days Implemented: 1*
*Days Available: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12*

---
*Tags: #aoc #2025 #problem-analysis #patterns #algorithm-learning

*Links: [[../../../zettelkasten/AoC Patterns MOC]] | [[../../../zettelkasten/AoC Integration]] *
