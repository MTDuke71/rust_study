# Chebyshev Distance

> **Navigation**: [[zettel-index]] | [[AoC Patterns MOC]] | [[Mission 6 Overview]]

**Also Known As**: Chessboard distance, Maximum metric, L∞ metric

## 📐 **Definition**

Chebyshev distance is the **maximum** of the absolute differences between coordinates:

```rust
fn chebyshev_distance(a: Coord, b: Coord) -> usize {
    let dx = if a.x > b.x { a.x - b.x } else { b.x - a.x };
    let dy = if a.y > b.y { a.y - b.y } else { b.y - a.y };
    dx.max(dy)  // Take the MAXIMUM
}
```

**Formula**: `d = max(|x₂ - x₁|, |y₂ - y₁|)`

---

## ♟️ **The Chess King Intuition**

Named "chessboard distance" because it represents **how many moves a chess king needs** to travel between two squares.

```
From (0,0) to (3,2):

. . . X     dx = 3
. . . ↗     dy = 2
. . ↗ .     Chebyshev = max(3, 2) = 3
K ↗ . .

King moves: (0,0) → (1,1) → (2,2) → (3,2)
Only 3 moves needed because diagonal moves count as 1 step!
```

---

## 📊 **Comparison with Other Distance Metrics**

| Metric | Formula | Movement Model | (0,0) → (3,2) | Shape at d=2 |
|--------|---------|----------------|---------------|--------------|
| **Manhattan** | `\|dx\| + \|dy\|` | 4-connected (N,E,S,W only) | `3 + 2 = 5` | Diamond ◆ |
| **Euclidean** | `√(dx² + dy²)` | Continuous (any angle) | `√13 ≈ 3.61` | Circle ● |
| **Chebyshev** | `max(\|dx\|, \|dy\|)` | 8-connected (with diagonals) | `max(3,2) = 3` | Square ■ |

---

## 🎨 **Visual Comparison**

All cells at distance 2 from center (*):

```
Manhattan (d=2):        Euclidean (d≈2):        Chebyshev (d=2):
. . 2 . .               . 2 2 2 .               2 2 2 2 2
. 2 1 2 .               2 1 1 1 2               2 1 1 1 2
2 1 * 1 2               2 1 * 1 2               2 1 * 1 2
. 2 1 2 .               2 1 1 1 2               2 1 1 1 2
. . 2 . .               . 2 2 2 .               2 2 2 2 2

Diamond shape           Circle shape            Square shape
4-directional only      Any direction           8-directional
```

**Key Observation**: Chebyshev distance creates **square neighborhoods** because diagonal moves are weighted equally with cardinal moves.

---

## 🎯 **When to Use Chebyshev Distance**

### ✅ **Ideal Use Cases:**

1. **8-Connected Grids**
   - Games where diagonal movement is allowed
   - King movement in chess
   - Strategy games (StarCraft, Age of Empires)

2. **A* Heuristic for 8-Connected Graphs**
   ```rust
   // Admissible for 8-connected grids with diagonal movement
   let h = chebyshev_distance(current, goal);
   let f_score = g_score + h;
   ```

3. **Image Processing**
   - 8-neighbor connectivity analysis
   - Morphological operations with 8-connectivity

4. **Board Games**
   - Chess king movement
   - Any game with diagonal moves at same cost

### ❌ **When NOT to Use:**

- **4-connected grids** (use Manhattan distance instead)
- **Weighted diagonal movement** (diagonals cost more than cardinals)
- **Continuous space** (use Euclidean distance)

---

## 🧮 **Mathematical Properties**

### **Metric Properties:**
1. **Non-negativity**: `d(a,b) ≥ 0`
2. **Identity**: `d(a,b) = 0` if and only if `a = b`
3. **Symmetry**: `d(a,b) = d(b,a)`
4. **Triangle inequality**: `d(a,c) ≤ d(a,b) + d(b,c)`

### **Relationship to Other Metrics:**
- **Lower bound**: `Chebyshev(a,b) ≤ Manhattan(a,b)`
- **Upper bound**: `Euclidean(a,b) ≤ Chebyshev(a,b) × √2`
- **Special case**: When `dx = dy`, Chebyshev = Euclidean = dx

---

## 💻 **Implementation Patterns**

### **Basic Implementation**
```rust
fn chebyshev_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    let dx = a.0.abs_diff(b.0);
    let dy = a.1.abs_diff(b.1);
    dx.max(dy)
}
```

### **With TutorialCoord**
```rust
fn chebyshev_distance(a: TutorialCoord, b: TutorialCoord) -> usize {
    let dx = if a.x > b.x { a.x - b.x } else { b.x - a.x };
    let dy = if a.y > b.y { a.y - b.y } else { b.y - a.y };
    dx.max(dy)
}
```

### **3D Extension**
```rust
fn chebyshev_distance_3d(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32 {
    let dx = (a.0 - b.0).abs();
    let dy = (a.1 - b.1).abs();
    let dz = (a.2 - b.2).abs();
    dx.max(dy).max(dz)
}
```

---

## 🎮 **Practical Examples**

### **Example 1: Range Detection in Strategy Games**
```rust
// Check if enemy is in attack range (8-directional)
fn is_in_range(unit_pos: Coord, target_pos: Coord, range: usize) -> bool {
    chebyshev_distance(unit_pos, target_pos) <= range
}

// All cells within range 2 form a 5x5 square around the unit
```

### **Example 2: A* Heuristic for 8-Connected Grid**
```rust
fn astar_8connected(start: Coord, goal: Coord, grid: &Grid) -> Option<Path> {
    // Use Chebyshev as admissible heuristic
    let h = |pos: Coord| chebyshev_distance(pos, goal) as f64;
    
    // ... A* implementation with 8-neighbor expansion
}
```

### **Example 3: Chess King Mobility**
```rust
fn king_can_reach(from: Square, to: Square, moves: usize) -> bool {
    chebyshev_distance(from, to) <= moves
}

// King at (4,4) can reach (6,6) in 2 moves
assert!(king_can_reach((4,4), (6,6), 2));
```

---

## 🔗 **Related Concepts**

### **Distance Metrics:**
- [[Manhattan Distance]] - For 4-connected grids (L₁ metric)
- [[Euclidean Distance]] - For continuous space (L₂ metric)
- [[Distance Metrics Comparison]] - Complete comparison guide

### **Pathfinding:**
- [[A-Star Algorithm]] - Uses distance as heuristic
- [[BFS Pathfinding]] - Optimal for unweighted graphs
- [[Heuristic Functions]] - Admissibility and consistency

### **Applications:**
- [[Grid-Based Pathfinding]] - Tutorial implementation
- [[Mission 6 Overview]] - Step 4 pathfinding algorithms
- [[AoC Grid Patterns]] - Common competitive programming patterns

---

## 🎓 **From Tutorial Implementation**

See practical usage in:
- **File**: `tutorials/Mission6_tut/examples/step4_pathfinding.rs`
- **Section 6**: "Heuristic Functions Deep Dive"
- **Example output**:
  ```
  Position    Manhattan    Euclidean    Chebyshev
  (1, 1)             10         7.21            6
  (3, 3)              6         4.47            4
  (6, 2)              4         3.16            3
  ```

---

## 🧠 **Key Takeaways**

1. **Chebyshev = max(dx, dy)** - Takes the maximum coordinate difference
2. **Chessboard distance** - Number of king moves needed
3. **Square neighborhoods** - All cells within distance d form a square
4. **8-connected grids** - Ideal for games/grids with diagonal movement
5. **Admissible heuristic** - Can be used in A* for 8-connected graphs
6. **Named after mathematician** - Pafnuty Chebyshev (also known for Chebyshev filters in signal processing)

---

## 📚 **Further Reading**

- [Wikipedia: Chebyshev distance](https://en.wikipedia.org/wiki/Chebyshev_distance)
- The Rust Book Ch7 - Module organization for grid utilities
- [[Mission 6 Tutorial]] - Complete pathfinding implementation
- [[Performance Optimization Guide]] - Efficient distance calculations

---

## 🔗 **Related Documentation**

### **Core Concepts:**
- [[Distance Metrics Comparison]]
- [[Heuristic Functions]]
- [[A-Star Algorithm]]

### **Tutorial Steps:**
- [[Mission6_tut/README.md]] - Complete tutorial overview
- Step 4: Pathfinding Algorithms

### **Applications:**
- [[AoC Grid Patterns]]
- **Strategy Game Patterns**
- [[Image Processing Patterns]]

---

*Tags: #distance-metrics #chebyshev #pathfinding #astar #heuristics #mission6 #8-connected #grid-algorithms #chess #competitive-programming*

*Links: [[zettel-index]] | [[AoC Patterns MOC]] | [[Mission 6 Overview]] | [[Manhattan Distance]] | [[Euclidean Distance]] | [[A-Star Algorithm]]*
