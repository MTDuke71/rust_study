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

## 🎯 Parametric Line Intersection

### **Mathematical Foundation**

See [[parametric-equations]] for complete treatment.

**Quick Reference**:
- **Parametric form**: $\mathbf{r}(t) = \mathbf{P}_0 + t \cdot \mathbf{V}$
- **2D intersection**: Solve 2×2 system using [[linear-algebra-fundamentals#Cramer's Rule]]
- **Applications**: Ray tracing, collision detection, trajectory analysis

**AoC 2023 Day 24 Example**:
```rust
// Test if two hailstone paths intersect in future
let det = self.vx * other.vy - self.vy * other.vx;
if det.abs() < 1e-10 { return false; }  // Parallel

let t = (dx * other.vy - dy * other.vx) / det;
let s = (dx * self.vy - dy * self.vx) / det;

// Future intersection: both t ≥ 0 and s ≥ 0
if t < 0.0 || s < 0.0 { return false; }
```

**Performance**: O(1) per test, ~6.4 ns per pair (Day 24: 44,850 pairs in 315 µs)

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
  - [[3d-geometry]] - Extension to 3D: voxels, flood fill, surface area
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

## 📏 Polygon Area Algorithms

### **Shoelace Formula (Gauss's Area Formula)**

**Also Known As**: Surveyor's Formula, Shoelace Theorem

**Mathematical Definition**:

For a polygon with vertices $(x_0, y_0), (x_1, y_1), \ldots, (x_{n-1}, y_{n-1})$ listed in order:

$$\text{Area} = \frac{1}{2} \left| \sum_{i=0}^{n-1} (x_i \cdot y_{i+1} - x_{i+1} \cdot y_i) \right|$$

where indices are taken modulo $n$ (so $x_n = x_0$, $y_n = y_0$).

**Intuition**: 
- The name "shoelace" comes from the cross-multiplication pattern resembling lacing shoes
- Each term $(x_i \cdot y_{i+1} - x_{i+1} \cdot y_i)$ is the **signed area** of a trapezoid under edge $(i, i+1)$
- Summing all trapezoids gives total signed area
- Absolute value handles clockwise vs counter-clockwise vertex ordering

**Why It Works**:

The formula is derived from Green's Theorem (a fundamental theorem of calculus):

$$\oint_C (L \, dx + M \, dy) = \iint_D \left( \frac{\partial M}{\partial x} - \frac{\partial L}{\partial y} \right) dA$$

Choosing $L = 0$ and $M = x$ gives:

$$\text{Area} = \iint_D 1 \, dA = \oint_C x \, dy = \sum_{i=0}^{n-1} \frac{x_i + x_{i+1}}{2} (y_{i+1} - y_i)$$

Which simplifies to the shoelace formula.

**Complexity**:
- **Time**: O(n) - one pass through vertices
- **Space**: O(1) - only accumulator needed

**Example**:

Rectangle with vertices $(0,0), (4,0), (4,3), (0,3)$:

```
sum = (0×0 - 4×0) + (4×3 - 4×0) + (4×3 - 0×3) + (0×0 - 0×3)
    = 0 + 12 + 12 + 0
    = 24

Area = |24| / 2 = 12  ✓
```

---

### **Pick's Theorem (1899)**

**Historical Context**: Discovered by Austrian mathematician Georg Alexander Pick. Provides a beautiful relationship between geometry and combinatorics.

**Mathematical Statement**:

For a simple polygon with vertices on a 2D integer lattice:

$$A = I + \frac{B}{2} - 1$$

Where:
- $A$ = Area of polygon
- $I$ = Number of interior lattice points
- $B$ = Number of boundary lattice points

**Intuition**:
- Each interior point contributes **1** to area
- Each boundary point contributes **1/2** to area (on average)
- The $-1$ is an Euler characteristic adjustment (related to topology)

**Rearranged Forms**:

Find interior points from area and boundary:
$$I = A - \frac{B}{2} + 1$$

Find total lattice points (interior + boundary):
$$I + B = A + \frac{B}{2} + 1$$

**Why This Matters for AoC**:

Many AoC problems give you:
- Instructions that trace a polygon on a grid
- Need to count **all grid cells** inside AND on the boundary

**Standard Approach**:
1. Trace polygon vertices from instructions
2. Calculate perimeter $B$ = sum of instruction distances
3. Use **Shoelace** to get polygon area $A$
4. Apply **Pick's rearranged** to get total cells: $I + B = A + \frac{B}{2} + 1$

**Example**:

Small rectangle on lattice:
```
Grid:
  0 1 2 3
0 +─+─+─+
  │ • • │
1 +─+─+─+
  │ • • │
2 +─+─+─+

Vertices: (0,0), (3,0), (3,2), (0,2)
```

- Shoelace area: $A = 6$
- Boundary points: $B = 12$ (trace perimeter)
- Interior points: $I = A - \frac{B}{2} + 1 = 6 - 6 + 1 = 1$ ❌ Wait...

Actually let me recount:
- Interior: 4 points (2×2 grid inside)
- Boundary: 10 points (perimeter)
- Shoelace: $A = 6$
- Pick's: $6 = 4 + \frac{10}{2} - 1 = 4 + 5 - 1 = 8$ ❌

Let me recalculate correctly:
- Rectangle from $(0,0)$ to $(2,2)$ has area $2 \times 2 = 4$
- Interior points: $(1,1)$ = 1 point
- Boundary points: 8 (corners + edges)
- Pick's: $4 = 1 + \frac{8}{2} - 1 = 1 + 4 - 1 = 4$ ✓

**Proof Sketch** (via triangulation):
1. Any lattice polygon can be triangulated into lattice triangles
2. Pick's theorem holds for minimal lattice triangle (area = 1/2, vertices only)
3. Induction: Adding triangles preserves the relationship
4. The $-1$ term comes from Euler's formula: $V - E + F = 2$

---

## 🦀 Rust Implementations

### **AoC 2023 Day 18: Lavaduct Lagoon**

**Problem**: Calculate area of lagoon traced by dig instructions. Part 2 has massive coordinates (trillions of cells).

**Solution**: Shoelace + Pick's Theorem (O(n) time, works regardless of coordinate scale)

See: [`advent_of_code/aoc2023/src/solver/day18.rs`](../../../advent_of_code/aoc2023/src/solver/day18.rs)

```rust
/// Calculate polygon area using Shoelace formula
fn shoelace_area(vertices: &[(i64, i64)]) -> i64 {
    let n = vertices.len();
    let mut sum = 0i64;
    
    for i in 0..n - 1 {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[i + 1];
        sum += x1 * y2 - x2 * y1;  // Cross product
    }
    
    // Close the polygon
    let (x1, y1) = vertices[n - 1];
    let (x2, y2) = vertices[0];
    sum += x1 * y2 - x2 * y1;
    
    sum.abs() / 2
}

// Apply Pick's Theorem rearranged: Total = Area + Perimeter/2 + 1
let total_cells = shoelace_area + perimeter / 2 + 1;
```

**Performance**: ~86µs for Part 1, ~107µs for Part 2 (52 trillion cells!)

**Key Insight**: Mathematical approach scales to ANY coordinate size - grid-based brute force would fail on Part 2.

---

## 📚 References

- [Jordan Curve Theorem (Wikipedia)](https://en.wikipedia.org/wiki/Jordan_curve_theorem)
- [Point in Polygon (Wikipedia)](https://en.wikipedia.org/wiki/Point_in_polygon)
- [Shoelace Formula (Wikipedia)](https://en.wikipedia.org/wiki/Shoelace_formula)
- [Pick's Theorem (Wikipedia)](https://en.wikipedia.org/wiki/Pick%27s_theorem)
- [Computational Geometry: Algorithms and Applications (de Berg et al.)](https://www.springer.com/gp/book/9783540779735)
- [Geometric Tools Engine (David Eberly)](https://www.geometrictools.com/)

---

*Tags: #computational-geometry #ray-casting #point-in-polygon #shoelace-formula #picks-theorem #polygon-area #lattice-points #jordan-curve-theorem #scanline #math-foundations*

**Related Zettelkasten Links**:
- [[parametric-equations]] - Line intersection using parametric form (AoC Day 24)
- [[linear-algebra-fundamentals]] - Cramer's rule for solving intersection systems
- [[cross-products-vector-algebra]] - 3D geometric operations
- [[graph-theory-fundamentals]] - Polygon boundary as graph
- [[set-theory-fundamentals]] - Points and regions as sets
- [[state-machine-pattern]] - Scanline implementation pattern
- [[mission-6]] - Grid structure for spatial data
- [[number-theory-basics]] - Lattice points and integer coordinates
