# Bresenham Line Algorithm

**Tags:** #algorithms #computational-geometry #line-drawing #rasterization #discrete-geometry #graphics  
**Created:** 2025-12-09  
**Related:** [[computational-geometry]], [[ray-casting-algorithm]], [[sparse-data-structures]], [[aoc-2025-day09]]

---

## 🎯 Core Concept

**Bresenham's Line Algorithm** generates all **integer grid points** along a line segment between two endpoints. It's the standard algorithm for drawing lines on discrete pixel grids without using floating-point arithmetic.

**Key Insight**: Use only integer addition, subtraction, and bit shifts to determine which pixels best approximate a continuous line.

## 🧠 The Problem

### **Rasterization Challenge**

Given two points in a discrete grid, which cells should be "filled" to represent the line between them?

```
From (1, 2) to (7, 5):

  6 |                    Continuous line: y = 0.5x + 1.5
  5 |           X        Need discrete approximation
  4 |         X          Which cells to mark?
  3 |       X
  2 | X   X              Bresenham provides the answer
  1 |
  0 +---+---+---+---+---+---+---+---
    0   1   2   3   4   5   6   7
```

**Naive approach**: Calculate y = mx + b for each x (uses floating-point, slow, rounding issues)

**Bresenham's approach**: Integer-only decisions using error accumulation

## 📐 The Algorithm

### **Basic Implementation**

```rust
/// Generate all grid points along line from (x0, y0) to (x1, y1)
fn bresenham_line(x0: i64, y0: i64, x1: i64, y1: i64) -> Vec<(i64, i64)> {
    let mut points = Vec::new();
    
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    
    let sx = if x0 < x1 { 1 } else { -1 };  // Step direction in x
    let sy = if y0 < y1 { 1 } else { -1 };  // Step direction in y
    
    let mut err = dx - dy;  // Error accumulator
    
    let mut x = x0;
    let mut y = y0;
    
    loop {
        points.push((x, y));  // Add current point
        
        if x == x1 && y == y1 {
            break;  // Reached endpoint
        }
        
        let e2 = 2 * err;
        
        // Should we step in x direction?
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        
        // Should we step in y direction?
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
    
    points
}
```

### **How It Works**

**Error Accumulation Principle**:
- Maintain running "error" value representing deviation from ideal line
- When error grows too large, step in the minor axis to correct
- Always step in the major axis (the longer dimension)

**Example walkthrough** for line from (1, 2) to (7, 5):
```
dx = 6, dy = 3
Initial err = 6 - 3 = 3

Step 1: (1,2), err=3, e2=6
  e2 > -dy (-3)? Yes → step x, err = 3-3 = 0, x=2
  e2 < dx (6)? Yes → step y, err = 0+6 = 6, y=3
  Point: (2, 3)

Step 2: (2,3), err=6, e2=12
  e2 > -3? Yes → step x, err = 6-3 = 3, x=3
  e2 < 6? No → don't step y
  Point: (3, 3)

Step 3: (3,3), err=3, e2=6
  e2 > -3? Yes → step x, err = 3-3 = 0, x=4
  e2 < 6? Yes → step y, err = 0+6 = 6, y=4
  Point: (4, 4)

...continues until (7, 5)

Result: [(1,2), (2,3), (3,3), (4,4), (5,4), (6,5), (7,5)]
```

## 🔑 Key Properties

### **1. Integer-Only Arithmetic**

**No division, no floating-point**:
- Only uses: `+`, `-`, `*2` (can be bit shift `<< 1`), comparisons
- Fast on all hardware, even embedded systems without FPU
- No rounding errors or precision issues

### **2. Symmetric**

```rust
bresenham_line(x0, y0, x1, y1) 
    == bresenham_line(x1, y1, x0, y0).reversed()
```

Line from A→B produces same points as B→A (just reversed order)

### **3. Connected**

**8-connectivity guarantee**: Each point is adjacent to the next (horizontally, vertically, or diagonally)

```
Valid transitions:
  X→X  (horizontal)
  X    (vertical)
  ↓
  X
  X    (diagonal)
   ↘
    X
```

**No gaps** - important for polygon boundaries!

### **4. Minimal**

Generates the **minimum number of points** needed to represent the line with 8-connectivity.

## 📊 Complexity Analysis

### **Time Complexity**

**O(max(dx, dy))** where dx, dy are coordinate differences
- Essentially O(n) where n = Manhattan distance
- Each iteration produces one point
- Number of points = number of steps in major axis

### **Space Complexity**

**O(max(dx, dy))** for storing result
- Can be reduced to O(1) if points are processed as generated (iterator pattern)

### **Optimization: Iterator Version**

```rust
struct BresenhamIterator {
    x: i64, y: i64,
    x1: i64, y1: i64,
    dx: i64, dy: i64,
    sx: i64, sy: i64,
    err: i64,
    finished: bool,
}

impl Iterator for BresenhamIterator {
    type Item = (i64, i64);
    
    fn next(&mut self) -> Option<(i64, i64)> {
        if self.finished {
            return None;
        }
        
        let point = (self.x, self.y);
        
        if self.x == self.x1 && self.y == self.y1 {
            self.finished = true;
            return Some(point);
        }
        
        let e2 = 2 * self.err;
        if e2 > -self.dy {
            self.err -= self.dy;
            self.x += self.sx;
        }
        if e2 < self.dx {
            self.err += self.dx;
            self.y += self.sy;
        }
        
        Some(point)
    }
}
```

**Benefit**: Can process points without allocating Vec, useful for large lines

## 🎓 AoC 2025 Day 9 Application

### **Building Polygon Boundary**

**Problem**: Connect 496 points into a polygon boundary for ray casting validation

```rust
fn build_polygon_boundary(points: &[(i64, i64)]) -> HashSet<(i64, i64)> {
    let mut boundary = HashSet::new();
    
    for i in 0..points.len() {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % points.len()];  // Wrap to first
        
        // Generate all points along edge
        for point in bresenham_line(x1, y1, x2, y2) {
            boundary.insert(point);
        }
    }
    
    boundary
}
```

### **Results**

**Input**: 496 points forming approximate circle with radius ~48,386

**Theoretical perimeter**: `2πr ≈ 2 × 3.14159 × 48,386 ≈ 304,000 points`

**Actual boundary points**: **588,656 points** (via Bresenham)

**Why more than theoretical?**
1. **Discrete approximation**: Continuous circle becomes staircase pattern
2. **8-connectivity**: Diagonal steps count as 1 grid cell but √2 in distance
3. **Polygon vs circle**: 496-sided polygon has slightly longer perimeter than circle

**Visualization**:
```
Continuous circle: smooth curve
                    ___
                  /     \
                 |   •   |
                  \     /
                   ‾‾‾
                   
Bresenham polygon: staircase edges
                    _--_
                  _/    \_
                 |   •   |
                  \_    _/
                   ‾‾‾‾
```

Each "stair" adds extra discrete points compared to smooth arc.

## 🔍 Variations and Extensions

### **1. Thick Lines**

Generate points for lines wider than 1 pixel:

```rust
fn thick_bresenham_line(x0: i64, y0: i64, x1: i64, y1: i64, 
                        thickness: i64) -> HashSet<(i64, i64)> {
    let mut points = HashSet::new();
    let line = bresenham_line(x0, y0, x1, y1);
    
    for &(x, y) in &line {
        // Add perpendicular points for thickness
        for offset in -thickness/2..=thickness/2 {
            if dx > dy {
                points.insert((x, y + offset));  // Thicken vertically
            } else {
                points.insert((x + offset, y));  // Thicken horizontally
            }
        }
    }
    
    points
}
```

### **2. Antialiased Lines (Xiaolin Wu's Algorithm)**

For smoother appearance with grayscale/alpha:

```rust
// Each point gets opacity value based on distance from ideal line
fn wu_line(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<(i64, i64, f64)> {
    // Returns (x, y, opacity) tuples
    // More complex than Bresenham, uses floating-point
}
```

### **3. Circle Rasterization (Midpoint Circle)**

Similar integer-only approach for circles:

```rust
fn bresenham_circle(cx: i64, cy: i64, radius: i64) -> Vec<(i64, i64)> {
    let mut points = Vec::new();
    let mut x = 0;
    let mut y = radius;
    let mut d = 3 - 2 * radius;
    
    while y >= x {
        // Add 8 symmetric points
        for &(dx, dy) in &[(x,y), (y,x), (-x,y), (-y,x), 
                           (x,-y), (y,-x), (-x,-y), (-y,-x)] {
            points.push((cx + dx, cy + dy));
        }
        x += 1;
        if d > 0 {
            y -= 1;
            d += 4 * (x - y) + 10;
        } else {
            d += 4 * x + 6;
        }
    }
    
    points
}
```

## 💡 Common Use Cases

### **Graphics and Games**

1. **Line rendering**: Draw lines on pixel grids
2. **Line-of-sight**: Check visibility between two points
3. **Projectile paths**: Determine which cells a bullet crosses
4. **Tile-based movement**: Valid movement paths

### **Computational Geometry**

1. **Polygon rasterization**: Convert vector polygons to pixel representation
2. **Boundary generation**: Create discrete boundaries for flood fill
3. **Grid-based pathfinding**: Connect waypoints on discrete grids
4. **Voxel traversal**: Ray marching through 3D voxel grids

### **Advent of Code Patterns**

1. **Grid-based line drawing**: "Hydrothermal vents" (2021 Day 5)
2. **Visibility calculations**: "Asteroid monitoring" (2019 Day 10)
3. **Polygon boundaries**: "Rectangular polygon area" (2025 Day 9)
4. **Ray tracing**: Various grid-based puzzles

## 🚨 Edge Cases and Gotchas

### **1. Horizontal/Vertical Lines**

```rust
// Horizontal line (y constant)
bresenham_line(2, 5, 8, 5)
// → [(2,5), (3,5), (4,5), (5,5), (6,5), (7,5), (8,5)]
// Works correctly! dx > 0, dy = 0

// Vertical line (x constant)
bresenham_line(3, 2, 3, 7)
// → [(3,2), (3,3), (3,4), (3,5), (3,6), (3,7)]
// Works correctly! dx = 0, dy > 0
```

### **2. Single Point**

```rust
bresenham_line(5, 5, 5, 5)
// → [(5, 5)]
// Correctly returns single point
```

### **3. Diagonal Lines**

```rust
// Perfect diagonal (dx = dy)
bresenham_line(0, 0, 5, 5)
// → [(0,0), (1,1), (2,2), (3,3), (4,4), (5,5)]
// Steps both x and y every iteration
```

### **4. Negative Coordinates**

```rust
bresenham_line(-5, -3, 2, 4)
// Works correctly with signed integers
// Uses sx, sy to handle direction
```

## 📚 Historical Context

**Jack E. Bresenham** (1962) developed this algorithm at IBM for pen plotters.

**Original motivation**: Mechanical plotters needed efficient way to approximate lines without expensive floating-point hardware.

**Legacy**: Still used today in:
- Graphics libraries (SDL, DirectX, OpenGL rasterization)
- Game engines (Unity, Unreal collision detection)
- CAD software (vector-to-raster conversion)
- Embedded systems (low-power line drawing)

## 📖 Related Algorithms

**Geometric Rasterization**:
- [[midpoint-circle-algorithm]] - Bresenham for circles
- [[dda-line-algorithm]] - Digital Differential Analyzer (uses floating-point)
- [[xiaolin-wu-antialiasing]] - Smooth antialiased lines

**Spatial Algorithms**:
- [[ray-casting-algorithm]] - Uses Bresenham for grid traversal
- [[flood-fill-algorithm]] - Often uses Bresenham-generated boundaries
- [[computational-geometry]] - Broader geometric algorithm family

**AoC Applications**:
- [[aoc-2025-day09]] - Polygon boundary generation
- [[aoc-2021-day05]] - Hydrothermal vent lines
- [[aoc-2019-day10]] - Asteroid line-of-sight

---

## 🎯 Implementation Checklist

When implementing Bresenham:

- [ ] Handle all 8 octants (all combinations of dx/dy signs and magnitudes)
- [ ] Use signed integers for coordinates (support negative values)
- [ ] Test with horizontal, vertical, and diagonal lines
- [ ] Test with single-point lines
- [ ] Consider iterator version for memory efficiency
- [ ] Verify 8-connectivity (no gaps between consecutive points)
- [ ] Test symmetry: line(A→B) reversed equals line(B→A)

---

*Bresenham's line algorithm is a masterpiece of algorithmic efficiency - achieving pixel-perfect line rasterization using only integer arithmetic and simple comparisons. Its elegance lies in transforming a continuous geometric problem into discrete steps without sacrificing accuracy. Nearly 60 years after its invention, it remains the gold standard for line drawing, proving that sometimes the best algorithms are timeless.*

**Links:** [[computational-geometry]] | [[ray-casting-algorithm]] | [[sparse-data-structures]] | [[aoc-2025-day09]] | [[graphics-algorithms]] | [[discrete-geometry]]
