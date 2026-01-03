# AoC 2023 - Algorithms Reference

Links to zettelkasten deep dives and implementation details for complex algorithms encountered in AoC 2023.

---

## 📊 Algorithm Usage Summary

| Algorithm | Day(s) | Complexity | Zettelkasten |
|-----------|--------|------------|--------------|
| Linear Scan | Day 1, Day 2 | O(n) | - |
| Delimiter Parsing | Day 2 | O(n × m) | - |
| Running Maximum | Day 2 | O(n) | - |
| Spatial Indexing | Day 3 | O(n) build, O(1) lookup | [[spatial-hash]] |
| Grid Scanning | Day 3 | O(w × h) | - |

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

### Grid Scanning with Adjacency Checks (Day 3)
**Implementation**: `src/solver/day03.rs`  
**Complexity**: O(w × h) for grid scan, O(8) for neighbor checks  
**Key Concept**: Scan grid for patterns, check 8-directional neighbors  
**Mission**: Mission 6 (Grid, Coord)

**When to use**: 
- 2D grid problems with adjacency requirements
- Symbol/pattern detection in grids
- Local neighborhood analysis

**Pattern**:
```rust
for y in 0..grid.height() {
    for x in 0..grid.width() {
        let coord = Coord::new(x, y);
        for neighbor in coord.neighbors_8() {
            // Check adjacent cells
        }
    }
}
```

**Zettelkasten**: None (standard grid traversal)

### Spatial Indexing for Grid Lookups (Day 3 Optimization)
**Implementation**: `src/solver/day03.rs::solve_part2()`  
**Complexity**: O(n × d) to build index, O(1) per lookup  
**Key Concept**: HashMap mapping coordinates to entities for instant lookups  

**When to use**: 
- Need to find "what's at this coordinate" repeatedly
- Reverse lookups (coordinate → entity instead of entity → coordinates)
- O(1) spatial queries instead of O(n) linear search

**Pattern**:
```rust
// Build spatial index: coord → entity_id
let mut coord_to_entity: HashMap<Coord, usize> = HashMap::new();
for (id, entity) in entities.iter().enumerate() {
    for coord in entity.occupied_coords() {
        coord_to_entity.insert(coord, id);
    }
}

// O(1) lookup instead of O(n) linear search
if let Some(&entity_id) = coord_to_entity.get(&target_coord) {
    // Found entity at coordinate
}
```

**Optimization Impact**: Day 3 Part 2 speedup: ~100x (millions of ops → 160K ops)

**Zettelkasten**: [[spatial-hash]] (if pattern repeats)

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
| Mission 6 (Grid) | Grid traversal, 8-directional neighbors, spatial indexing | Day 3 |
## 📝 Notes

- Algorithm entries are created when first encountered
- Deep dives go to zettelkasten (linked from here)
- Implementation details stay in solver files
- This file serves as navigation hub
