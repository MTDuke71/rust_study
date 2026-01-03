# Memoization MOC - Map of Content

**Map of Content for memoization patterns, techniques, and applications across Rust learning**

---

## Overview

Memoization is a fundamental optimization technique that caches expensive function call results to avoid redundant computation. This MOC organizes all memoization-related notes, examples, and patterns in the knowledge base.

**Core Principle**: Trade memory for speed by caching results of pure functions.

---

## Primary Resources

### 📚 Comprehensive Guides
- **[[memoization-comprehensive-guide.md]]** - Comprehensive Rust implementation guide
  - HashMap, Vec, and Closure-based patterns
  - Ownership and thread safety considerations
  - Testing strategies and common pitfalls
  - AoC 2015 examples (Day 7, Day 10)
  - 709 lines of detailed implementation guidance

### 🎯 Problem-Solving Patterns
- **[[memoization-aoc2024-patterns.md]]** - AoC 2024 tactical patterns
  - Count-Only Recursion (Day 11, Day 19)
  - Composite Keys (Day 21)
  - Suffix/Prefix Recursion (Day 19)
  - Depth-Based Recursion (Day 11, Day 21)
  - Performance impact tables with empirical results

### 🔍 Specialized Applications
- **[[state-based-memoization]]** - Composite state representation
  - Path counting with constraints (AoC Day 7, Day 11)
  - Multi-dimensional state spaces
  - Complex key design patterns

---

## Implementation Patterns

### Core Patterns

| **Pattern** | **Best For** | **Example** | **Guide** |
|-------------|-------------|-------------|-----------|
| **HashMap-Based** | Sparse state spaces, complex keys | Fibonacci, Circuit simulation | [[memoization-comprehensive-guide.md#Pattern 1]] |
| **Vec-Based** | Sequential state, known bounds | Bottom-up DP | [[memoization-comprehensive-guide.md#Pattern 2]] |
| **Closure-Based** | Functional style, embedded logic | Simple recursive functions | [[memoization-comprehensive-guide.md#Pattern 3]] |
| **Count-Only** | Counting solutions, not generating | Stone evolution (Day 11) | [[memoization-aoc2024-patterns.md#Count-Only Recursion]] |
| **Composite Keys** | Multi-parameter recursion | Keypad sequences (Day 21) | [[memoization-aoc2024-patterns.md#Composite Keys]] |

### When to Use Memoization

✅ **Use when:**
- Overlapping subproblems (Fibonacci, DP)
- Expensive computation with repeated inputs
- Pure functions (no side effects)
- Bounded state space (won't exhaust memory)

❌ **Don't use when:**
- All subproblems unique (no overlaps)
- Cache overhead > computation cost
- Simple iterative solution exists
- Unbounded state space

---

## Advent of Code Applications

### AoC 2024 (Memoization-Heavy Year)
- **[[aoc-2024-day11]]** - Plutonian Pebbles (Stone Evolution)
  - Pattern: Count-based memoization with composite keys `(stone, blinks)`
  - Transformation: O(2^75) → O(~1000 states) = **10^19× speedup**
  
- **[[aoc-2024-day19]]** - Linen Layout (Towel Pattern Matching)
  - Pattern: Suffix-based recursion with string keys
  - Transformation: O(patterns^n) → O(n × patterns) = **10^52× speedup**
  
- **[[aoc-2024-day21]]** - Keypad Conundrum (Recursive Sequence Expansion)
  - Pattern: Composite keys `(from, to, depth)` for layered recursion
  - Transformation: O(4^26) → O(keys² × 26) = **10^12× speedup**

### AoC 2015 (Learning Foundation)
- **AoC 2015 Day 7** - Circuit Simulation
  - Dependency graph evaluation
  - Wire value caching with cache invalidation
  - Example: [[memoization-comprehensive-guide.md#AoC 2015 Day 7]]
  
- **AoC 2015 Day 10** - Look-and-Say Sequence
  - **Anti-pattern example**: When memoization doesn't help!
  - String concatenation overhead dominates
  - See: [[../../advent_of_code/aoc2015/examples/DAY10_BENCHMARK_ANALYSIS]]

---

## Performance Considerations

### Complexity Transformations

| **Problem** | **Naive** | **Memoized** | **Speedup** | **Source** |
|-------------|-----------|--------------|-------------|------------|
| Fibonacci(30) | O(2^30) = 1B calls | O(30) = 30 calls | 35M× | [[memoization-comprehensive-guide.md#Why Memoization Matters]] |
| Day 11 (75 blinks) | O(2^75) = 3.8×10^22 | O(1000 states) | 10^19× | [[memoization-aoc2024-patterns.md#Performance Impact]] |
| Day 19 (length 60) | O(8^60) = 10^54 | O(60 × 8 = 480) | 10^52× | [[memoization-aoc2024-patterns.md#Performance Impact]] |
| Day 21 (26 layers) | O(4^26) = 4.5×10^15 | O(26 × 25 = 650) | 10^12× | [[memoization-aoc2024-patterns.md#Performance Impact]] |

### Cache Strategy Selection

```rust
// Sparse state space → HashMap
use std::collections::HashMap;
let cache: HashMap<(u64, usize), u64> = HashMap::new();

// Sequential indices → Vec
let cache: Vec<u64> = vec![0; n + 1];

// Integer keys (faster hashing) → FxHashMap
use rustc_hash::FxHashMap;
let cache: FxHashMap<u64, u64> = FxHashMap::default();
```

---

## Common Pitfalls & Solutions

### Pitfall Categories
1. **Excessive Cloning** - [[memoization-comprehensive-guide.md#Pitfall 1]]
   - Return references when possible
   - Use `Rc`/`Arc` for shared ownership

2. **Unbounded Cache Growth** - [[memoization-comprehensive-guide.md#Pitfall 2]]
   - Use LRU cache for bounded memory
   - Clear cache periodically

3. **Forgetting Cache Check** - [[memoization-comprehensive-guide.md#Pitfall 3]]
   - Always check before computing
   - Use pattern templates

---

## Rust-Specific Considerations

### Ownership Patterns
- **Owned Values**: Clone on return, move into cache
- **Borrowed Values**: Complex lifetimes, consider `Rc`
- **Thread Safety**: `Arc<Mutex<HashMap>>` or `DashMap`

### Related Rust Concepts
- **[[HashMap Deep Dive]]** - Cache implementation internals
- **[[rust-hashmap-performance]]** - HashMap vs FxHashMap benchmarks
- **[[Rust Collections MOC]]** - Collection choice impacts performance

---

## Mission Integration

### Mission 11 - Memoization Patterns (Formal Implementation)
- **Mission Overview**: [[missions/mission-11]] - Progress tracking and learning insights
  - Tutorial Progress: 7/7 steps complete ✅
  - Step 1 (Naive): 68ms at length 25 (exponential pain point)
  - Step 2 (Memoized): Expected 6,800× speedup
  - Step 7: Real AoC 2024 Day 19 application (279.8× speedup)
- **Tutorial**: [[../../tutorials/Mission11_tut/README.md]] - 7-step progressive learning
- **Formal V-Cycle**: Planned for next week
  - Will include memoization optimization requirements

---

## Related Concepts

### Dynamic Programming
- **[[Dynamic Programming Patterns]]** - General DP strategies
- **[[Tabulation Patterns]]** - Bottom-up DP (iterative alternative)
- **Memoization**: Top-down DP (recursive with cache)

### Algorithm Analysis
- **[[Algorithm Complexity Analysis]]** - Big-O impact of memoization
- **[[Performance Optimization Guide]]** - When to use various techniques
- **[[recursion-optimization]]** - Tail recursion, trampolining

---

## Code Locations

### Implementation Examples
- **AoC 2024 Solutions**:
  - `advent_of_code/aoc2024/src/solver/day11.rs` - Count-based memoization
  - `advent_of_code/aoc2024/src/solver/day19.rs` - Suffix recursion
  - `advent_of_code/aoc2024/src/solver/day21.rs` - Composite keys

- **AoC 2015 Examples**:
  - `advent_of_code/aoc2015/examples/DAY10_MEMOIZATION_WALKTHROUGH` - When memo works
  - `advent_of_code/aoc2015/examples/DAY10_BENCHMARK_ANALYSIS` - When memo fails

- **Mission 11 Tutorial**:
  - `tutorials/Mission11_tut/examples/step7_real_aoc_problems.rs` - Fibonacci, factorial memoization

---

## Learning Progression

### Beginner (Understanding the Concept)
1. Read: [[memoization-comprehensive-guide.md#What is Memoization?]]
2. Study: Fibonacci example (with/without memo)
3. Practice: Implement simple HashMap-based cache

### Intermediate (Problem-Solving Patterns)
1. Read: [[memoization-aoc2024-patterns.md#Common Patterns]]
2. Study: AoC 2024 examples (Day 11, 19, 21)
3. Practice: Identify when memo applies to AoC problems

### Advanced (Optimization & Edge Cases)
1. Read: [[memoization-comprehensive-guide.md#Performance Considerations]]
2. Study: Cache invalidation, thread safety patterns
3. Practice: Benchmark memo vs non-memo implementations

---

## External Resources

### Books & References
- *Rust for Rustaceans* - Advanced ownership patterns
- *Introduction to Algorithms (CLRS)* - DP chapter
- *Competitive Programming Handbook* - Memoization strategies

### Community Examples
- AoC subreddit discussions on memoization wins
- Rust forums: memoization with lifetimes

---

## Tags

*Tags*: #memoization #dynamic-programming #optimization #performance #algorithms #rust #aoc #map-of-content #moc

---

## Navigation

**Parent MOCs**:
- **[[AoC Patterns MOC]]** - Problem-solving patterns
- **[[Algorithms MOC]]** - General algorithm techniques
- **[[Performance Optimization Guide]]** - Optimization strategies

**Related MOCs**:
- **[[Rust Collections MOC]]** - HashMap/Vec implementation details
- **[[Algorithm Analysis]]** - Complexity analysis

**Daily Notes Using Memoization**:
- [[Daily Notes/2025-12-21]] - Created memoization-patterns.md for AoC 2024
- [[Daily Notes/2026-01-03]] - Mission 11 Step 7 completion (memoization examples)

---

*Last Updated*: 2026-01-03  
*Maintainer*: Learning system knowledge base
