# Convex Hull Algorithm

**Tags:** #algorithm #computational-geometry #convex-hull #graham-scan #jarvis-march #monotone-chain #geometry #2d-geometry  
**Created:** 2025-12-08  
**Related:** [[computational-geometry]], [[union-find-algorithm]], [[glam-game-math-library]], [[sorting-algorithms]], [[angle-calculations]]

---

## 🎯 Core Concept

The **Convex Hull** of a set of points is the smallest convex polygon that contains all the points. Think of it as stretching a rubber band around the outermost points - the band naturally forms the convex hull.

**Convex**: A polygon where all interior angles are less than 180°. For any two points inside, the line segment connecting them lies entirely within the polygon.

## 🧠 Fundamental Properties

### **Mathematical Definition**

The convex hull of a set S of points is the smallest convex set containing S:

```
ConvexHull(S) = {∑ λᵢpᵢ : pᵢ ∈ S, λᵢ ≥ 0, ∑λᵢ = 1}
```

### **Key Characteristics**

- **Uniqueness**: Every point set has exactly one convex hull
- **Boundary**: Consists only of extreme points (cannot be expressed as convex combination of others)
- **Size**: Convex hull has at most `n` vertices for `n` input points
- **Worst case**: All `n` points on the hull (e.g., points on a circle)

### **Visual Intuition**

```
Input Points:              Convex Hull:
    •  •                      •--•
  •      •                   /    \
    • •  •        →         •      •
  •        •                 \    /
    •    •                    •--•
```

Points inside the hull are not part of the boundary.

## 🔄 Classic Algorithms

### **1. Graham Scan - O(n log n)**

**Strategy**: Sort points by polar angle, then scan to find hull

```rust
fn graham_scan(points: &mut Vec<Point>) -> Vec<Point> {
    if points.len() < 3 {
        return points.clone();
    }
    
    // Step 1: Find bottom-most point (or leftmost if tie)
    let start = points.iter()
        .min_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)))
        .unwrap()
        .clone();
    
    // Step 2: Sort by polar angle relative to start point
    points.sort_by(|a, b| {
        let angle_a = (a.y - start.y).atan2(a.x - start.x);
        let angle_b = (b.y - start.y).atan2(b.x - start.x);
        angle_a.partial_cmp(&angle_b).unwrap()
    });
    
    // Step 3: Build hull using stack and CCW test
    let mut hull = Vec::new();
    
    for point in points {
        // Remove points that create clockwise turn
        while hull.len() > 1 && ccw(&hull[hull.len()-2], &hull[hull.len()-1], point) <= 0 {
            hull.pop();
        }
        hull.push(point.clone());
    }
    
    hull
}

// Counter-clockwise test using cross product
fn ccw(a: &Point, b: &Point, c: &Point) -> i64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    // > 0: CCW turn, < 0: CW turn, = 0: collinear
}
```

**Key Steps:**
1. **Anchor**: Find lowest point (pivot)
2. **Sort**: By polar angle from anchor (O(n log n))
3. **Scan**: Build hull maintaining CCW turns (O(n))

**Time Complexity**: O(n log n) - dominated by sorting  
**Space Complexity**: O(n) - for hull storage

### **2. Jarvis March (Gift Wrapping) - O(nh)**

**Strategy**: Start from leftmost point, repeatedly find next point with smallest polar angle

```rust
fn jarvis_march(points: &[Point]) -> Vec<Point> {
    if points.len() < 3 {
        return points.to_vec();
    }
    
    let mut hull = Vec::new();
    
    // Start with leftmost point
    let mut current = points.iter()
        .min_by_key(|p| p.x)
        .unwrap();
    
    loop {
        hull.push(current.clone());
        let mut next = &points[0];
        
        // Find point with smallest CCW angle from current
        for point in points {
            if point == current {
                continue;
            }
            let cross = ccw(current, next, point);
            if next == current || cross > 0 || (cross == 0 && dist(current, point) > dist(current, next)) {
                next = point;
            }
        }
        
        current = next;
        if current == &hull[0] {
            break; // Wrapped back to start
        }
    }
    
    hull
}
```

**Key Insight**: "Gift wrapping" - imagine wrapping string around points

**Time Complexity**: O(nh) where h = hull size
- **Best case**: O(n) when h is constant (few hull points)
- **Worst case**: O(n²) when h = n (all points on hull)

**When to Use**: Better than Graham when h << n (sparse hull)

### **3. Andrew's Monotone Chain - O(n log n)**

**Strategy**: Build upper and lower hulls separately by sorted x-coordinate

```rust
fn monotone_chain(points: &mut Vec<Point>) -> Vec<Point> {
    if points.len() < 3 {
        return points.clone();
    }
    
    // Sort by x, then y
    points.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
    
    let mut lower = Vec::new();
    let mut upper = Vec::new();
    
    // Build lower hull (left to right)
    for point in points.iter() {
        while lower.len() >= 2 && ccw(&lower[lower.len()-2], &lower[lower.len()-1], point) <= 0 {
            lower.pop();
        }
        lower.push(point.clone());
    }
    
    // Build upper hull (right to left)
    for point in points.iter().rev() {
        while upper.len() >= 2 && ccw(&upper[upper.len()-2], &upper[upper.len()-1], point) <= 0 {
            upper.pop();
        }
        upper.push(point.clone());
    }
    
    // Remove last point of each half (duplicated)
    lower.pop();
    upper.pop();
    
    lower.extend(upper);
    lower
}
```

**Advantages over Graham Scan:**
- No polar angle computation (avoids floating-point)
- More numerically stable
- Easier to implement correctly
- Often preferred in competitive programming

**Time Complexity**: O(n log n)  
**Space Complexity**: O(n)

### **4. QuickHull - O(n²) worst, O(n log n) average**

**Strategy**: Divide-and-conquer approach similar to QuickSort

```rust
fn quickhull(points: &[Point]) -> Vec<Point> {
    if points.len() < 3 {
        return points.to_vec();
    }
    
    // Find leftmost and rightmost points
    let left = points.iter().min_by_key(|p| p.x).unwrap();
    let right = points.iter().max_by_key(|p| p.x).unwrap();
    
    let mut hull = Vec::new();
    
    // Find points on both sides of left-right line
    quickhull_helper(points, left, right, &mut hull);
    quickhull_helper(points, right, left, &mut hull);
    
    hull
}

fn quickhull_helper(points: &[Point], p1: &Point, p2: &Point, hull: &mut Vec<Point>) {
    // Find point furthest from line p1-p2
    let furthest = points.iter()
        .filter(|p| ccw(p1, p2, p) > 0) // Only points on correct side
        .max_by_key(|p| distance_to_line(p1, p2, p));
    
    match furthest {
        None => {
            hull.push(p1.clone()); // No points outside, p1 is on hull
        }
        Some(f) => {
            // Recursively process triangles
            quickhull_helper(points, p1, f, hull);
            quickhull_helper(points, f, p2, hull);
        }
    }
}
```

**Time Complexity**: 
- **Average**: O(n log n) - similar to QuickSort
- **Worst**: O(n²) - when points form specific patterns

**Practical Performance**: Often faster than Graham in practice due to cache locality

## 🎯 Algorithm Comparison

| Algorithm | Time Complexity | Space | Stability | Notes |
|-----------|----------------|-------|-----------|-------|
| **Graham Scan** | O(n log n) | O(n) | Deterministic | Classic, uses polar angles |
| **Jarvis March** | O(nh) | O(h) | Deterministic | Best when h << n |
| **Monotone Chain** | O(n log n) | O(n) | Deterministic | Most robust, no angles |
| **QuickHull** | O(n log n) avg | O(n) | Randomized | Fast in practice |
| **Chan's Algorithm** | O(n log h) | O(n) | Deterministic | Optimal output-sensitive |

### **Which to Choose?**

- **Default choice**: **Monotone Chain** (Andrew's) - simplest, most stable
- **Sparse hull** (h << n): **Jarvis March** or **Chan's Algorithm**
- **Random data**: **QuickHull** - often fastest in practice
- **Learning/teaching**: **Graham Scan** - clearest geometric intuition

## 🔢 Key Geometric Primitives

### **Cross Product (CCW Test)**

```rust
fn ccw(a: &Point, b: &Point, c: &Point) -> i64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

// Returns:
//   > 0: c is to the left of line a→b (counter-clockwise)
//   < 0: c is to the right of line a→b (clockwise)
//   = 0: a, b, c are collinear
```

**Geometric Interpretation**:
- Magnitude = 2 × triangle area
- Sign = orientation (CCW/CW)

### **Distance to Line**

```rust
fn distance_to_line(p1: &Point, p2: &Point, p: &Point) -> i64 {
    // Perpendicular distance from p to line p1-p2
    ((p2.y - p1.y) * p.x - (p2.x - p1.x) * p.y + p2.x * p1.y - p2.y * p1.x).abs()
}
```

### **Point Distance**

```rust
fn dist_squared(a: &Point, b: &Point) -> i64 {
    (a.x - b.x).pow(2) + (a.y - b.y).pow(2)
}

// Use squared distance to avoid sqrt (same as AoC Day 8 optimization!)
```

## 🌍 Applications

### **1. Collision Detection**

```rust
// Check if two convex polygons intersect
fn polygons_intersect(hull1: &[Point], hull2: &[Point]) -> bool {
    // Separating Axis Theorem (SAT) using hull edges
    for i in 0..hull1.len() {
        let edge = (hull1[i], hull1[(i+1) % hull1.len()]);
        if separates(edge, hull2) {
            return false;
        }
    }
    true // No separating axis found
}
```

**Use cases**: Game engines, robotics path planning

### **2. Computational Geometry**

```rust
// Diameter of point set (furthest pair)
fn diameter(points: &[Point]) -> f64 {
    let hull = monotone_chain(points);
    
    // Rotating calipers algorithm on convex hull
    let mut max_dist = 0.0;
    for i in 0..hull.len() {
        for j in i+1..hull.len() {
            max_dist = max_dist.max(distance(&hull[i], &hull[j]));
        }
    }
    max_dist
}
```

### **3. Pattern Recognition**

```rust
// Minimum enclosing circle center often lies within convex hull
fn approximate_center(points: &[Point]) -> Point {
    let hull = monotone_chain(points);
    
    // Centroid of convex hull
    let sum_x: i64 = hull.iter().map(|p| p.x).sum();
    let sum_y: i64 = hull.iter().map(|p| p.y).sum();
    
    Point {
        x: sum_x / hull.len() as i64,
        y: sum_y / hull.len() as i64,
    }
}
```

### **4. Advent of Code Applications**

**Rare but valuable**: Convex hull appears occasionally in AoC

- **2018 Day 10**: Some solvers used hull area to detect point convergence
- **2024 Day 12**: Hull perimeter for region boundary calculations
- **General**: Any "find outer boundary" or "minimum enclosing shape" problem

**Pattern recognition**: Look for:
- "Outermost points"
- "Enclosing boundary"
- "Maximum perimeter/area"
- "Points on edge"

### **5. Graphics & Visualization**

- **Skyline rendering**: Building silhouettes
- **Mesh simplification**: Reduce polygon complexity
- **Terrain analysis**: Horizon visibility

## ⚡ Performance Optimizations

### **Integer-Only Operations**

```rust
// Avoid floating-point entirely for exact computation
#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

// Cross product stays integer (no sqrt, no atan2)
fn ccw_int(a: &Point, b: &Point, c: &Point) -> i64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}
```

**Benefits**: 
- Exact arithmetic (no floating-point error)
- Faster on modern CPUs
- Same pattern as AoC Day 8 squared distance optimization!

### **Collinear Point Handling**

```rust
// Decide whether to include collinear points on hull
fn ccw_with_collinear(a: &Point, b: &Point, c: &Point, include_collinear: bool) -> i64 {
    let cross = ccw(a, b, c);
    if cross == 0 && !include_collinear {
        // Collinear - check if c is between a and b
        return if is_between(a, b, c) { 0 } else { 1 };
    }
    cross
}
```

### **Early Termination**

```rust
// For applications needing only hull size, not points
fn hull_size_only(points: &[Point]) -> usize {
    // Build hull but only count, don't store
    let mut count = 0;
    // ... monotone chain algorithm ...
    count
}
```

## 🧪 Testing Strategies

### **Edge Cases**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_and_small() {
        assert_eq!(convex_hull(&[]).len(), 0);
        assert_eq!(convex_hull(&[Point{x:0,y:0}]).len(), 1);
        assert_eq!(convex_hull(&[Point{x:0,y:0}, Point{x:1,y:1}]).len(), 2);
    }
    
    #[test]
    fn test_triangle() {
        let triangle = vec![
            Point{x:0, y:0},
            Point{x:4, y:0},
            Point{x:2, y:3},
        ];
        assert_eq!(convex_hull(&triangle).len(), 3);
    }
    
    #[test]
    fn test_with_interior_points() {
        let points = vec![
            Point{x:0, y:0}, Point{x:4, y:0}, Point{x:4, y:4}, Point{x:0, y:4}, // Square
            Point{x:2, y:2}, // Interior point
        ];
        let hull = convex_hull(&points);
        assert_eq!(hull.len(), 4); // Only square vertices
    }
    
    #[test]
    fn test_collinear() {
        let collinear = vec![
            Point{x:0, y:0},
            Point{x:1, y:1},
            Point{x:2, y:2},
            Point{x:3, y:3},
        ];
        let hull = convex_hull(&collinear);
        assert_eq!(hull.len(), 2); // Only endpoints
    }
    
    #[test]
    fn test_circle() {
        // All points on circle = all on hull
        let n = 20;
        let points: Vec<Point> = (0..n)
            .map(|i| {
                let angle = 2.0 * PI * i as f64 / n as f64;
                Point {
                    x: (100.0 * angle.cos()) as i64,
                    y: (100.0 * angle.sin()) as i64,
                }
            })
            .collect();
        
        let hull = convex_hull(&points);
        assert!(hull.len() >= n - 2); // Allow for rounding/duplicates
    }
}
```

### **Invariant Checking**

```rust
fn verify_hull(points: &[Point], hull: &[Point]) -> bool {
    // 1. All hull points from original set
    for h in hull {
        assert!(points.contains(h), "Hull point not in input");
    }
    
    // 2. All points either on hull or inside
    for p in points {
        assert!(is_on_or_inside_hull(p, hull), "Point outside hull");
    }
    
    // 3. All hull vertices make CCW turns
    for i in 0..hull.len() {
        let a = &hull[i];
        let b = &hull[(i+1) % hull.len()];
        let c = &hull[(i+2) % hull.len()];
        assert!(ccw(a, b, c) > 0, "Non-CCW turn in hull");
    }
    
    true
}
```

## 🔗 Related Algorithms

### **3D Convex Hull**

Extend to 3D using **Gift Wrapping** or **QuickHull in 3D**:

```rust
struct Point3D { x: i64, y: i64, z: i64 }

// 3D hull consists of triangular faces
fn convex_hull_3d(points: &[Point3D]) -> Vec<Triangle> {
    // QuickHull extends naturally to 3D
    // Graham scan does NOT extend to 3D
}
```

### **Dynamic Convex Hull**

Support insertions/deletions:

```rust
struct DynamicHull {
    upper_hull: BTreeSet<Point>,
    lower_hull: BTreeSet<Point>,
}

impl DynamicHull {
    fn insert(&mut self, p: Point) -> bool {
        // Update hull structure
    }
    
    fn remove(&mut self, p: Point) -> bool {
        // Rebuild affected portion
    }
}
```

**Complexity**: O(log n) per operation with proper data structures

### **Related Geometry Algorithms**

- **[[Voronoi Diagrams]]**: Dual of Delaunay triangulation
- **[[Closest Pair]]**: Often solved on convex hull
- **[[Line Intersection]]**: Used in sweep-line hull algorithms
- **[[Rotating Calipers]]**: Diameter, width, minimum bounding box

## 🐛 Common Pitfalls

### **1. Floating-Point Errors**

```rust
// ❌ WRONG: Using f64 for angles can cause errors
fn bad_angle_sort(points: &mut [Point], center: &Point) {
    points.sort_by(|a, b| {
        let angle_a = ((a.y - center.y) as f64).atan2((a.x - center.x) as f64);
        let angle_b = ((b.y - center.y) as f64).atan2((b.x - center.x) as f64);
        angle_a.partial_cmp(&angle_b).unwrap() // Can fail on NaN!
    });
}

// ✅ CORRECT: Use cross product for comparisons
fn good_angle_sort(points: &mut [Point], center: &Point) {
    points.sort_by(|a, b| {
        let cross = ccw(center, a, b);
        if cross == 0 {
            dist_squared(center, a).cmp(&dist_squared(center, b))
        } else {
            cross.cmp(&0).reverse()
        }
    });
}
```

### **2. Collinear Point Handling**

```rust
// Different applications need different behavior:

// Include all collinear points (for boundary tracing)
while hull.len() > 1 && ccw(&hull[hull.len()-2], &hull[hull.len()-1], point) < 0 {
    hull.pop(); // Only remove CW, keep collinear
}

// Exclude collinear points (for minimal hull)
while hull.len() > 1 && ccw(&hull[hull.len()-2], &hull[hull.len()-1], point) <= 0 {
    hull.pop(); // Remove both CW and collinear
}
```

### **3. Degeneracy Handling**

```rust
// Handle duplicate points
fn remove_duplicates(points: &mut Vec<Point>) {
    points.sort();
    points.dedup();
}

// Handle all-collinear case
fn convex_hull_robust(points: &mut Vec<Point>) -> Vec<Point> {
    remove_duplicates(points);
    
    if points.len() < 3 {
        return points.clone();
    }
    
    let hull = monotone_chain(points);
    
    if hull.len() == 2 {
        // All points collinear - return endpoints only
        return hull;
    }
    
    hull
}
```

## 📚 Learning Path

### **Beginner**
1. ✅ Understand convex vs concave polygons
2. ✅ Implement CCW test (cross product)
3. ✅ Implement **Monotone Chain** (Andrew's)
4. ✅ Test on small examples by hand

### **Intermediate**
1. 🔄 Implement **Graham Scan** with polar angles
2. 🔄 Implement **Jarvis March** gift wrapping
3. 🔄 Compare performance on different datasets
4. 🔄 Handle edge cases (collinear, duplicates)

### **Advanced**
1. 🔄 Implement **QuickHull** divide-and-conquer
2. 🔄 Study **Chan's Algorithm** (O(n log h))
3. 🔄 Extend to **3D Convex Hull**
4. 🔄 Implement **Dynamic Convex Hull**

### **Expert**
1. 🔄 **Rotating Calipers** for diameter/width
2. 🔄 **Half-plane intersection**
3. 🔄 **Numerical stability** analysis
4. 🔄 **Parallel convex hull** algorithms

## 📖 References

### **Classic Papers**
- Graham, R. L. (1972). "An Efficient Algorithm for Determining the Convex Hull of a Finite Planar Set"
- Jarvis, R. A. (1973). "On the Identification of the Convex Hull of a Finite Set of Points in the Plane"
- Andrew, A. M. (1979). "Another Efficient Algorithm for Convex Hulls in Two Dimensions"

### **Textbooks**
- **CLRS**: Introduction to Algorithms, Chapter 33 (Computational Geometry)
- **de Berg**: Computational Geometry: Algorithms and Applications, Chapter 1
- **O'Rourke**: Computational Geometry in C (Second Edition)

### **Online Resources**
- **CP-Algorithms**: Convex Hull construction
- **GeeksforGeeks**: Multiple algorithm implementations
- **Visualizations**: Algorithm visualizers for convex hull

---

## *Links:*

**Core Concepts:** [[computational-geometry]] | [[sorting-algorithms]] | [[divide-and-conquer]] | [[geometric-primitives]]

**Related Algorithms:** [[graham-scan]] | [[monotone-chain]] | [[jarvis-march]] | [[quickhull]] | [[chans-algorithm]]

**Geometric Primitives:** [[cross-product]] | [[ccw-test]] | [[point-line-distance]] | [[angle-calculations]]

**Applications:** [[collision-detection]] | [[pattern-recognition]] | [[rotating-calipers]] | [[voronoi-diagrams]]

**Advanced Topics:** [[3d-convex-hull]] | [[dynamic-convex-hull]] | [[half-plane-intersection]] | [[numerical-stability]]

**Integration:** [[union-find-algorithm]] | [[graph-algorithms]] | [[aoc-patterns]]

---

*Convex hull algorithms represent fundamental computational geometry techniques with applications spanning computer graphics, robotics, GIS, and competitive programming. While less common than graph algorithms in AoC, understanding convex hull provides essential geometric intuition and introduces key primitives (CCW test, cross product) used throughout computational geometry.*
