# Ray Casting Algorithm

**Tags:** #computational-geometry #point-in-polygon #algorithms #geometric-primitives #spatial-queries #aoc  
**Created:** 2025-12-09  
**Related:** [[computational-geometry]], [[aabb-sampling-optimization]], [[bresenham-line-algorithm]], [[aoc-2025-day09]]

---

## 🎯 Core Concept

**Ray Casting** is a geometric algorithm that determines if a point lies inside or outside a polygon by casting a ray from the point to infinity and counting how many times it crosses the polygon's edges.

**Fundamental Rule**: 
- **Odd number of crossings** → Point is **inside** polygon
- **Even number of crossings** → Point is **outside** polygon

This works for any simple polygon (no self-intersections), including **concave polygons**.

## 🧠 The Algorithm

### **Basic Implementation**

```rust
/// Ray casting point-in-polygon test
/// Returns true if point is inside polygon
fn point_in_polygon(point: (i64, i64), polygon: &[(i64, i64)]) -> bool {
    let (px, py) = point;
    let mut inside = false;
    let n = polygon.len();
    
    // Cast ray from point to the right (increasing x direction)
    // Count how many edges it crosses
    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];  // Next vertex (wraps to first)
        
        // Check if horizontal ray crosses this edge
        // Edge must span the ray's y-coordinate
        if (y1 > py) != (y2 > py) {
            // Calculate x-coordinate of intersection point
            let x_intersect = (x2 - x1) * (py - y1) / (y2 - y1) + x1;
            
            // If intersection is to the right of point, ray crosses edge
            if px < x_intersect {
                inside = !inside;  // Toggle inside/outside
            }
        }
    }
    
    inside
}
```

### **Step-by-Step Visualization**

```
Polygon:     Point Tests:
  A----B      • P1 (outside): 0 crossings (even)
  |    |      • P2 (inside):  1 crossing (odd)
  |  P2|      • P3 (inside):  3 crossings (odd)
  D----C      • P4 (edge):    special case

P1 ----→     (no crossings)
   
   P2 ----→  (crosses right edge once)
   
      P3 ----→ (crosses 3 edges)
```

## 🔑 Key Implementation Details

### **1. Edge Condition Check**

```rust
if (y1 > py) != (y2 > py)
```

**What this does**: Ensures the edge **spans** the ray's y-coordinate
- One endpoint above ray (`y1 > py`)
- One endpoint below ray (`y2 > py`)
- XOR condition: `!=` ensures they're on **opposite sides**

**Why it matters**: Horizontal edges don't count (ray parallel to edge), edges entirely above/below don't count.

### **2. Intersection Calculation**

```rust
let x_intersect = (x2 - x1) * (py - y1) / (y2 - y1) + x1;
```

**Derivation**: Linear interpolation to find where edge crosses ray's y-coordinate

```
Edge line: y = y1 + (y2 - y1) * t  where t ∈ [0, 1]
Solve for t when y = py:
    py = y1 + (y2 - y1) * t
    t = (py - y1) / (y2 - y1)

x_intersect = x1 + (x2 - x1) * t
            = x1 + (x2 - x1) * (py - y1) / (y2 - y1)
```

### **3. Ray Direction**

```rust
if px < x_intersect {
    inside = !inside;
}
```

**Cast ray to the right**: Only count intersections where `x_intersect` is to the right of point
- Could also cast left, up, or down - just be consistent
- Right is conventional (increasing x direction)

### **4. Handling Vertices**

**Potential issue**: What if ray passes exactly through a vertex?

**Solution in algorithm**: The `(y1 > py) != (y2 > py)` condition handles this:
- If ray hits vertex with edges on **same side** (both above or both below): 0 edges counted ✅
- If ray hits vertex with edges on **opposite sides**: 1 edge counted ✅
- If ray hits vertex where edges form **horizontal segment**: Treated as single edge ✅

## 📊 Complexity Analysis

### **Time Complexity**

**Per point query**: `O(n)` where n = number of polygon edges
- Must check every edge once
- Can't skip edges without risking incorrect result

**Batch queries**: `O(m × n)` for m points
- Each point requires full polygon traversal
- No preprocessing amortizes the cost

### **Space Complexity**

**Space**: `O(1)` auxiliary space
- Only stores boolean `inside` flag and loop variables
- Polygon itself is input, not counted

### **Optimizations**

For **many point queries** on the **same polygon**, consider:

1. **Bounding box pre-filter**: Check if point in AABB before ray casting
   ```rust
   if px < min_x || px > max_x || py < min_y || py > max_y {
       return false;  // Outside bounding box → outside polygon
   }
   ```

2. **Spatial hashing**: Divide polygon into grid, only check nearby edges
   - Build grid: `O(n)` preprocessing
   - Query time: `O(k)` where k = edges in relevant grid cells

3. **Winding number algorithm**: More robust for edge cases, similar complexity

## 🎓 AoC 2025 Day 9 Application

### **The Problem Context**

**Challenge**: Find largest rectangle within a circular polygon formed by 496 points
- Polygon boundary: 588,656 tiles (generated via [[bresenham-line-algorithm]])
- Rectangle candidates: 122,760 pairs to check
- Each rectangle: Up to 1.54 billion tiles to validate

### **Solution Architecture**

```rust
// 1. Build polygon boundary as sparse HashSet
let boundary = build_polygon_boundary(&points);

// 2. Ray casting validator
fn is_valid_point(x: i64, y: i64, boundary: &HashSet<(i64, i64)>, 
                  polygon: &[(i64, i64)]) -> bool {
    // Point on boundary is valid (red tile)
    if boundary.contains(&(x, y)) {
        return true;
    }
    
    // Point in interior is valid (green tile from flood fill)
    point_in_polygon((x, y), polygon)
}

// 3. AABB sampling for rectangle validation
fn rectangle_in_polygon(x1, y1, x2, y2) -> bool {
    // Sample ~2,000-10,000 points instead of billions
    aabb_sample_points()
        .all(|(x, y)| is_valid_point(x, y, &boundary, &polygon))
}
```

### **Performance Impact**

**Without AABB sampling**: 
- 1.54B tiles × 496 edge checks = **2.3 trillion operations**
- Result: **Timeout (>10s)**

**With AABB sampling**:
- ~2,000 sample points × 496 edge checks = **~1 million operations** per rectangle
- 122,760 rectangles × 1M = **633 million operations total**
- Result: **3-5 seconds** ✅
- **Speedup: 3,650x**

## 🔍 Edge Cases and Gotchas

### **1. Collinear Points**

```rust
// Polygon with 3 collinear points
let polygon = vec![(0, 0), (5, 0), (10, 0), (10, 5), (0, 5)];
//                  ^       ^       ^  (all on same line)

// Ray casting still works!
// Horizontal edge is skipped (doesn't span any y-coordinate)
```

### **2. Self-Intersecting Polygons**

```rust
// Bow-tie polygon (self-intersecting)
let polygon = vec![(0, 0), (10, 10), (10, 0), (0, 10)];
//                  A       B          C       D
// Ray casting gives **winding count**, not geometric inside/outside
// May not match intuitive expectation
```

**Solution**: Ensure input polygon is **simple** (no self-intersections)

### **3. Point Exactly on Edge**

**Current algorithm**: Point on edge may return false (depends on calculation precision)

**Robust solution**: Add explicit edge containment check:
```rust
fn point_on_edge(p: (i64, i64), v1: (i64, i64), v2: (i64, i64)) -> bool {
    let (px, py) = p;
    let (x1, y1) = v1;
    let (x2, y2) = v2;
    
    // Check if p is on line segment v1-v2
    let cross = (py - y1) * (x2 - x1) - (px - x1) * (y2 - y1);
    if cross != 0 { return false; }  // Not collinear
    
    // Check if p is between v1 and v2
    px >= x1.min(x2) && px <= x1.max(x2) &&
    py >= y1.min(y2) && py <= y1.max(y2)
}
```

### **4. Horizontal Edges**

```rust
// Edge from (2, 5) to (8, 5) is horizontal
// Condition (y1 > py) != (y2 > py) when py = 5:
//   y1 > 5 is false
//   y2 > 5 is false
//   false != false is FALSE → edge skipped ✅

// This is correct! Horizontal edges don't affect inside/outside status
```

## 🏆 Alternative Algorithms

### **Winding Number Algorithm**

**Idea**: Count signed crossings (left = +1, right = -1)
- Winding number = 0 → outside
- Winding number ≠ 0 → inside

**Advantages**: More robust for edge cases, handles winding direction
**Disadvantages**: Slightly more complex implementation

### **Point in Convex Polygon**

If polygon is **convex**, use cross product test (much faster):

```rust
fn point_in_convex_polygon(p: Point, polygon: &[Point]) -> bool {
    let n = polygon.len();
    let sign = |a: Point, b: Point, c: Point| -> i64 {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    };
    
    let initial_sign = sign(polygon[0], polygon[1], p).signum();
    
    for i in 1..n {
        let s = sign(polygon[i], polygon[(i+1) % n], p).signum();
        if s != initial_sign {
            return false;  // Point on wrong side of an edge
        }
    }
    true
}
```

**Complexity**: Still O(n), but constant factor is smaller (no division)

### **Preprocessing for Many Queries**

For repeated queries on the same polygon:

1. **Triangulation**: Split polygon into triangles, use barycentric coordinates
   - Preprocessing: O(n log n)
   - Query: O(log n) via binary search

2. **Trapezoidal decomposition**: Partition polygon into trapezoids
   - Preprocessing: O(n log n)
   - Query: O(log n)

3. **Grid-based**: Rasterize polygon into grid
   - Preprocessing: O(W × H) where W, H are grid dimensions
   - Query: O(1) hash lookup

## 💡 When to Use Ray Casting

### ✅ **Good Fit**

- **Simple polygons** (no self-intersections)
- **Moderate query volume** (hundreds to thousands of points)
- **Arbitrary polygon shapes** (concave, convex, doesn't matter)
- **2D spatial queries** (game collision, GIS, robotics)

### ❌ **Poor Fit**

- **Millions of queries** on same polygon (use preprocessing)
- **Real-time performance** critical (use convex-only optimization or grid)
- **3D polyhedra** (use different algorithm - ray-triangle intersection)
- **Self-intersecting polygons** (undefined behavior)

## 📚 Related Concepts

**Geometric Foundations**:
- [[computational-geometry]] - Algorithm family overview
- [[orientation-test]] - CCW/CW detection using cross product
- [[line-segment-intersection]] - Edge crossing detection

**Performance Optimizations**:
- [[aabb-sampling-optimization]] - Strategic sampling for large regions
- [[sparse-data-structures]] - HashSet for polygon boundaries
- [[bresenham-line-algorithm]] - Discrete line generation for edges

**AoC Applications**:
- [[aoc-2025-day09]] - Rectangle validation within circular polygon
- [[aoc-2024-day08]] - Grid-based line of sight (simpler case)

---

## 🎯 Implementation Checklist

When implementing ray casting:

- [ ] Choose consistent ray direction (typically right/east)
- [ ] Handle vertex cases with `(y1 > py) != (y2 > py)` condition
- [ ] Use integer arithmetic when possible (avoid floating-point precision issues)
- [ ] Consider bounding box pre-filter for performance
- [ ] Add explicit boundary containment check if points may be on edges
- [ ] Test with concave polygons (not just convex)
- [ ] Test with horizontal/vertical edges
- [ ] Verify with known inside/outside test cases

---

*Ray casting is one of the most elegant algorithms in computational geometry - converting a complex spatial query (is point inside arbitrary polygon?) into a simple counting problem (count edge crossings). Its O(n) time complexity without preprocessing makes it practical for moderate query volumes, and its simplicity makes it robust and easy to implement correctly. Combined with modern optimizations like AABB sampling, ray casting scales to problems involving billions of spatial queries.*

**Links:** [[computational-geometry]] | [[aabb-sampling-optimization]] | [[point-in-polygon]] | [[aoc-2025-day09]] | [[bresenham-line-algorithm]] | [[sparse-data-structures]]
