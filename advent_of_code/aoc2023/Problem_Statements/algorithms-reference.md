# AoC 2023 - Algorithms Reference

Links to zettelkasten deep dives and implementation details for complex algorithms encountered in AoC 2023.

---

## 📊 Algorithm Usage Summary

| Algorithm | Day(s) | Complexity | Zettelkasten |
|-----------|--------|------------|--------------|
| Linear Scan | Day 1, Day 2 | O(n) | - |
| Delimiter Parsing | Day 2 | O(n × m) | - |
| Running Maximum | Day 2 | O(n) | - |

---

## 🔤 String Algorithms

### Position-Based Pattern Matching (Day 1)
**Implementation**: `src/solver/day01.rs`  
**Complexity**: O(n × m) where n = string length, m = pattern count  
**Key Concept**: Scan each position for all possible patterns  

**When to use**: 
- Overlapping patterns need detection
- Simple regex would miss overlaps
- Pattern set is small and fixed

**Zettelkasten**: None (too simple for deep dive)

### Delimiter-Based Parsing (Day 2)
**Implementation**: `src/solver/day02.rs`  
**Complexity**: O(n × m) where n = lines, m = avg delimited segments  
**Key Concept**: Hierarchical split on multiple delimiters (`:`, `;`, `,`)  

**When to use**: 
- Input has structured format with consistent separators
- Need to parse nested data (records → fields → values)
- Simple regex would be overkill

**Pattern**:
```rust
line.split(':')      // Primary structure
    .split(';')      // Secondary sequences  
    .split(',')      // Tertiary elements
    .split_whitespace() // Final values
```

**Zettelkasten**: None (common parsing pattern)

### Running Maximum Tracking (Day 2)
**Implementation**: `src/solver/day02.rs::update_max()`  
**Complexity**: O(n) single pass  
**Key Concept**: Track maximum value seen so far across stream  

**When to use**:
- Need minimum resources to satisfy all observations
- Finding envelope/bounds of variable data
- One-pass streaming aggregation

**Pattern**:
```rust
fn update_max(&mut self, other: &T) {
    self.field = self.field.max(other.field);
}
```

**Zettelkasten**: None (standard accumulation pattern)

---

## 🗺️ Graph Algorithms

*To be populated as graph problems are solved.*

### Breadth-First Search (BFS)
**Day(s)**: TBD  
**Zettelkasten**: [[bfs-patterns]]  
**Mission**: Mission 8 (Graph)

### Dijkstra's Algorithm
**Day(s)**: TBD  
**Zettelkasten**: [[dijkstra-algorithm]]  
**Mission**: Mission 8 (Graph)

### A* Search
**Day(s)**: TBD  
**Zettelkasten**: [[a-star-algorithm]]

---

## 🔢 Mathematical Algorithms

*To be populated as math problems are solved.*

### Greatest Common Divisor (GCD) / Least Common Multiple (LCM)
**Day(s)**: TBD  
**Zettelkasten**: [[number-theory-basics]]

### Modular Arithmetic
**Day(s)**: TBD  
**Zettelkasten**: [[modular-arithmetic]]

---

## 🧩 Dynamic Programming

*To be populated as DP problems are solved.*

### Memoization Patterns
**Day(s)**: TBD  
**Zettelkasten**: [[dynamic-programming-patterns]]

---

## 🔍 Search Algorithms

*To be populated as search problems are solved.*

### Binary Search Variants
**Day(s)**: TBD  
**Zettelkasten**: [[binary-search-patterns]]  
**Mission**: Mission 3 (Binary Search)

### Backtracking
**Day(s)**: TBD  
**Zettelkasten**: [[backtracking-patterns]]

---

## 📐 Geometric Algorithms

*To be populated as geometry problems are solved.*

### Flood Fill
**Day(s)**: TBD  
**Mission**: Mission 6 (Grid) + Mission 8 (Graph)

### Shoelace Formula / Pick's Theorem
**Day(s)**: TBD  
**Zettelkasten**: [[computational-geometry]]

---

## 🔗 Mission Integration Map

| Mission | Algorithms | Days Used |
|---------|------------|-----------|
| Mission 3 (Binary Search) | Binary search variants | TBD |
| Mission 6 (Grid) | Grid traversal, flood fill | TBD |
| Mission 8 (Graph) | BFS, DFS, Dijkstra, A* | TBD |
| Mission 10 (Union-Find) | Connected components | TBD |

---

## 📝 Notes

- Algorithm entries are created when first encountered
- Deep dives go to zettelkasten (linked from here)
- Implementation details stay in solver files
- This file serves as navigation hub
