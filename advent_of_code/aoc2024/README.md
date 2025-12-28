# 🎄 Advent of Code 2024 Solutions

**Dual-focus AoC solutions** combining algorithmic learning with Python→Rust conversion, emphasizing both problem-solving patterns and idiomatic Rust programming.

This crate serves two learning objectives:
1. **Algorithm Mastery** - Understanding AoC 2024's unique problem patterns and solution strategies
2. **Rust Conversion** - Demonstrating the transition from Python's dynamic approach to Rust's type-safe, performance-oriented paradigm

---

## 📚 **What's Inside**

### **Solutions** (Currently: Day 1 implemented)
Each solution provides dual learning value:

**🧩 Algorithm Learning:**
- ✅ **Problem pattern analysis** - Identifying AoC 2024's unique algorithmic themes
- ✅ **Multiple approaches** - Exploring different solution strategies
- ✅ **Complexity analysis** - Time/space Big-O understanding
- ✅ **AoC-specific optimizations** - Competitive programming techniques

**🦀 Rust Conversion:**
- ✅ **Idiomatic Rust patterns** - Functional programming with iterators
- ✅ **Robust error handling** - `Result<T, E>` with detailed context
- ✅ **Type safety** - Compile-time guarantees and pattern matching
- ✅ **Performance** - Zero-cost abstractions and memory efficiency
- ✅ **Comprehensive testing** - Unit tests with edge cases

### **Core Infrastructure** (`src/`)
- **`solver/`** - Day-specific solution implementations
- **`main.rs`** - CLI runner with debug options
- **`lib.rs`** - Public API and module organization

---

## 🎯 **Implemented Solutions**

### **Day 1: Historian Hysteria** 📊
**Problem**: Calculate distance and similarity between two lists of location IDs

**🧩 Algorithm Analysis**:
- **Problem type**: List processing with sorting and frequency counting
- **Pattern**: Two-part escalation (distance → similarity)
- **Key insights**: Part 1 uses sorting for pairing, Part 2 uses frequency maps
- **Complexity**: O(n log n) for sorting, O(n) for frequency counting
- **AoC theme**: Classic "process pairs of lists" with different metrics

**🦀 Python → Rust Conversion**:
- **From manual loops** → **Functional iterator chains**
- **From exception handling** → **Explicit `Result<T, E>` error types**
- **From dynamic typing** → **Compile-time type safety**
- **From multiple data copies** → **In-place operations and zero-cost abstractions**

**Key Rust Features Demonstrated**:
- **Pattern matching** for safe parsing (`match parts.as_slice()`)
- **Error context** with `anyhow` for detailed error messages
- **Iterator combinators** (`zip`, `map`, `fold`, `sum`)
- **HashMap** with functional construction (`fold` instead of manual insertion)
- **Comprehensive unit testing** with edge cases and error conditions

**Learning Outcomes**: 
- ✅ **Algorithm**: Understanding list processing patterns in competitive programming
- ✅ **Rust**: Mastering functional error handling and iterator chains
- ✅ **Performance**: Same results as Python with significantly better safety and speed

---

## 🚀 **Quick Start**

### **Run Solutions**
```bash
# Run specific day
cargo run -p aoc2024 -- 1 inputs/day01_example.txt

# Run with debug output
cargo run -p aoc2024 -- 1 inputs/simple_test.txt --debug

# Test with example data
cargo run -p aoc2024 -- 1 inputs/simple_test.txt
# Expected: Part 1: 11, Part 2: 31
```

### **Run Tests**
```bash
# All tests for aoc2024
cargo test -p aoc2024

# Specific day tests
cargo test -p aoc2024 --lib day01

# All tests with output
cargo test -p aoc2024 -- --nocapture
```

---

## 📂 **Project Structure**

```
aoc2024/
├── src/
│   ├── lib.rs              # Public API and prelude
│   ├── main.rs             # CLI runner
│   ├── parser.rs           # Parsing utilities
│   ├── grid.rs             # 2D utilities 
│   └── solver/             # Day implementations
│       ├── mod.rs          # Solver module organization
│       ├── day01.rs        # Day 1: Historian Hysteria ✅
│       └── day01.py        # Original Python solution (reference)
├── inputs/                 # Input files
│   ├── day01_example.txt   # Real puzzle input
│   └── simple_test.txt     # Problem statement example
├── Problem_Statements/     # Problem descriptions
│   └── day01.md           # Day 1 problem statement
└── Cargo.toml
```

---

## 🎯 **Dual Learning Philosophy**

### **🧩 Algorithm Learning Approach**
- **Pattern Recognition** - Identify recurring AoC themes (grids, graphs, parsing, optimization)
- **Multiple Solutions** - Explore brute force, optimized, and mathematical approaches
- **Complexity Analysis** - Understand time/space trade-offs for competitive programming
- **Problem Evolution** - Study how Part 2 often transforms or scales Part 1's approach
- **AoC-Specific Techniques** - Learn competition-specific optimizations and shortcuts

### **🦀 Python to Rust Conversion Philosophy**
- **Type Safety** - Replace runtime errors with compile-time guarantees
- **Error Handling** - Explicit `Result<T, E>` instead of exceptions
- **Functional Style** - Iterator chains over manual loops
- **Zero-Cost Abstractions** - High-level code that compiles to efficient machine code
- **Memory Safety** - Ownership system prevents common bugs

### **Integrated Learning Process**
1. **Solve in mindset** - Understand the algorithmic challenge first
2. **Analyze Python solution** - Study existing approach and identify patterns
3. **Design algorithm** - Choose optimal approach for the problem type
4. **Model in Rust types** - Design appropriate data structures
5. **Implement with tests** - TDD approach with comprehensive coverage
6. **Optimize idiomatically** - Use Rust strengths (iterators, pattern matching, etc.)
7. **Document insights** - Capture both algorithmic and language-specific learnings
8. **Cross-validate** - Ensure Rust solution matches Python results

---

## � **Navigation**

For detailed analysis and additional solutions, see:
- **[SUMMARY.md](advent_of_code/aoc2024/Problem_Statements/summary.md)** - Comprehensive implementation details, algorithm analysis, benchmarks, and conversion insights
- **[AoC 2015](../aoc2015/README.md)** - Reference implementation with extensive algorithmic analysis
- **[AoC Integration](../../zettelkasten/AoC%20Integration.md)** - Integration with mission system and learning tracks

### **Algorithm Learning Resources**
- [[AoC Patterns MOC]] - Catalog of competitive programming patterns
- [[Priority Queue Patterns]] - Pathfinding and optimization techniques
- [[graph-algorithms]] - Traversal, shortest path, and connectivity algorithms
- [[Frequency Counting]] - Hash-based analysis techniques
- [[Grid Traversal]] - 2D problem solving strategies

### **Rust Learning Integration**
- [[Complete Runnable Examples]] - Documentation standard followed
- [[Missions Overview]] - Data structure implementations used in solutions
- [[Daily Study MOC]] - Progressive Rust concept learning
- [[Rust Collections MOC]] - HashMap, Vec, and iterator patterns

---

*Tags: #aoc2024 #advent-of-code #algorithms #competitive-programming #rust #python-conversion #functional-programming #type-safety #pattern-recognition #problem-solving*

---

**🎄 AoC 2024: Dual-focus learning combining algorithmic mastery with idiomatic Rust - from understanding competitive programming patterns to demonstrating the transition from Python's flexibility to Rust's performance and safety.**
