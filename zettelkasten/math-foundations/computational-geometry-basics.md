# Computational Geometry Basics

**Field**: Computational Geometry / Algorithms

**Prerequisites**: [[set-theory-fundamentals]]

---

## 📐 Definition

**Computational Geometry** is the study of algorithms for solving geometric problems. It focuses on:
- Point-in-polygon detection
- Line intersection
- Convex hulls
- Geometric search
- Spatial indexing

**Intuition**: Applying algorithmic thinking to geometric shapes - think collision detection, map rendering, robot path planning.

---

## 🔑 Key Concepts

### **Point-in-Polygon Problem**

**Definition**: Determine if a point P lies inside, outside, or on the boundary of a polygon.

**Applications**:
- Geographic information systems (GIS)
- Computer graphics (hit testing)
- Game development (collision detection)
- Map enclosed regions

---

## 🎯 Ray Casting Algorithm

### **Jordan Curve Theorem (1887)**

**Mathematical Foundation**:
```
For a simple closed curve C and point P not on C:
  P is inside C ⟺ any ray from P to infinity crosses C an odd number of times
  P is outside C ⟺ any ray from P to infinity crosses C an even number of times
```

**Intuition**: Imagine walking from point P to infinity. Count how many times you cross the boundary. If odd, you started inside. If even, you started outside.

### **Algorithm Steps**

1. **Cast ray** from point P in any direction (commonly horizontal →)
2. **Count crossings** where ray intersects polygon boundary
3. **Determine position**:
   - Odd crossings → Inside
   - Even crossings → Outside

### **Complexity**
- **Time**: O(n) where n = number of boundary edges
- **Space**: O(1) for scanline approach

---

## 🔧 Implementation Strategies

### **Scanline Ray Casting**

Process each horizontal row from left to right, tracking inside/outside state:

```
Grid:
  ┌───┐
  │ • │ • 
  └───┘

Row scan:
  ┌───┐
  →→→→→→→→
  
State machine:
  outside → cross '│' → inside → cross '│' → outside
```

**Advantages**:
- Single pass per row
- O(1) space (just tracking state)
- Easy to parallelize (rows are independent)

### **Corner Handling**

**The Tricky Part**: Corners can be "tangent" to the ray

```
Case 1: L---7 (Crossing - opposite sides)
  Ray: →→X→→
       └────┐
  
  Ray crosses from bottom to top → Counts as 1 crossing

Case 2: L---J (Not crossing - same side)
  Ray: →→→→→
       └────┘
  
  Ray stays below loop → Counts as 0 crossings

Case 3: F---J (Crossing - opposite sides)
  Ray: →→X→→
       ┌────┘
  
  Ray crosses from top to bottom → Counts as 1 crossing

Case 4: F---7 (Not crossing - same side)
  Ray: →→→→→
       ┌────┐
  
  Ray stays above loop → Counts as 0 crossings
```

**Rule**: Corner pair (entry, exit) counts as crossing if corners have **opposite vertical orientations**

| Entry | Exit | Crossing? | Reason |
|-------|------|-----------|--------|
| `F` (down-right) | `J` (up-left) | ✓ | Opposite vertical |
| `F` (down-right) | `7` (down-left) | ✗ | Same vertical (down) |
| `L` (up-right) | `7` (down-left) | ✓ | Opposite vertical |
| `L` (up-right) | `J` (up-left) | ✗ | Same vertical (up) |

---

## 💻 Rust Implementations

### **AoC 2023 Day 10**: Pipe Maze
- **What**: Count tiles enclosed within pipe loop
- **How it uses this concept**:
  - Scanline ray casting on grid
  - State machine for inside/outside tracking
  - Corner pairing logic for crossing detection
  - Vertical pipes (`|`) always cross
  - Horizontal pipes (`-`) never cross
  - Corners (`F`, `L`, `7`, `J`) require pairing
- **Link**: `advent_of_code/aoc2023/src/solver/day10.rs`
- **Performance**: 3.4ms (19,740 cells, 7,000 loop tiles)

---

## 📚 Code Example

### State Machine for Scanline Ray Casting

```rust
/// Count enclosed tiles in a grid using ray casting
fn count_enclosed(grid: &Grid<char>, loop_tiles: &HashMap<Coord, usize>) -> usize {
    let mut count = 0;
    
    for y in 0..grid.height() {
        let mut inside = false;              // Am I inside the loop?
        let mut enter_corner: Option<char> = None;  // Which corner did I enter?
        
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            let ch = grid[coord];
            
            if loop_tiles.contains_key(&coord) {
                // On loop boundary - update state
                match ch {
                    '|' => inside = !inside,     // Vertical crossing
                    
                    'F' | 'L' => {
                        enter_corner = Some(ch);  // Remember entry
                    }
                    
                    '7' => {
                        if enter_corner == Some('L') {
                            inside = !inside;     // L-7 crosses
                        }
                        // F-7 doesn't cross
                        enter_corner = None;
                    }
                    
                    'J' => {
                        if enter_corner == Some('F') {
                            inside = !inside;     // F-J crosses
                        }
                        // L-J doesn't cross
                        enter_corner = None;
                    }
                    
                    '-' => {
                        // Horizontal segment - no state change
                    }
                    
                    _ => {}
                }
            } else if inside {
                // Not on loop, but inside - count it!
                count += 1;
            }
        }
    }
    
    count
}
```

**Mathematical Foundation**:
1. **Jordan Curve Theorem**: Odd crossings = inside
2. **State machine**: Track inside/outside as we scan
3. **Corner logic**: Pair corners to determine crossing
4. **Invariant**: `inside` correctly reflects position after processing each cell

---

## 🎓 Alternative Approaches

### **Flood Fill**
- **Strategy**: Fill from outside edges, mark all reachable cells
- **Remaining unmarked cells** = inside polygon
- **Complexity**: O(W × H) time, O(W × H) space
- **Trade-off**: More intuitive, but uses more memory

### **Winding Number**
- **Strategy**: Sum signed angles as you traverse polygon
- **Winding number = 0** → outside
- **Winding number ≠ 0** → inside
- **Works for**: Complex polygons with holes
- **Complexity**: O(n) per point query

### **Cross Product Method**
- **Strategy**: Test which side of each edge the point is on
- **All same side** → inside (convex polygons only)
- **Limitation**: Only works for convex polygons
- **Complexity**: O(n) per point

---

## 🌳 Related Concepts

- **Prerequisites**: 
  - [[set-theory-fundamentals]] - Points and regions as sets
- **Related**: 
  - [[graph-theory-fundamentals]] - Boundary as graph structure
  - [[state-machine-pattern]] - Implementation uses state machine
- **Applications**:
  - GIS (geographic information systems)
  - Computer graphics
  - Game development
  - Robotics (collision detection)
  - CAD/CAM systems

---

## 📊 Mathematical Proof (Informal)

**Theorem**: Ray casting correctly determines point-in-polygon

**Proof Sketch**:
1. **Base case**: Point clearly outside has 0 crossings (even)
2. **Crossing into polygon**: +1 crossing → now odd (inside)
3. **Crossing out of polygon**: +1 crossing → now even (outside)
4. **Multiple crossings**: Alternates inside/outside with each crossing
5. **Final state**: Parity of crossings = position (odd=inside, even=outside)

**Corner correctness**:
- Corners on **same side** of ray: Ray never enters/exits polygon → 0 crossings
- Corners on **opposite sides**: Ray enters at one, exits at other → 1 crossing

---

## 📖 Resources

- [Jordan Curve Theorem (Wikipedia)](https://en.wikipedia.org/wiki/Jordan_curve_theorem)
- [Point in Polygon (Wikipedia)](https://en.wikipedia.org/wiki/Point_in_polygon)
- [Computational Geometry: Algorithms and Applications (de Berg et al.)](https://www.springer.com/gp/book/9783540779735)
- [Geometric Tools Engine (David Eberly)](https://www.geometrictools.com/)

---

*Tags: #computational-geometry #ray-casting #point-in-polygon #jordan-curve-theorem #scanline #math-foundations*

**Related Zettelkasten Links**:
- [[graph-theory-fundamentals]] - Polygon boundary as graph
- [[set-theory-fundamentals]] - Points and regions as sets
- [[state-machine-pattern]] - Scanline implementation pattern
- [[mission-6]] - Grid structure for spatial data
