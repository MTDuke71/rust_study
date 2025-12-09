# Computational Geometry

**Tags:** #computational-geometry #algorithms #mathematics #geometry #spatial-algorithms #geometric-primitives #2d-geometry #3d-geometry  
**Created:** 2025-12-08  
**Related:** [[convex-hull-algorithm]], [[glam-game-math-library]], [[union-find-algorithm]], [[graph-algorithms]]

---

## 🎯 Core Concept

**Computational Geometry** is the study of algorithms for solving geometric problems. It combines geometry, algorithms, and data structures to efficiently process spatial data.

**Key Focus**: Finding efficient algorithms for problems involving points, lines, polygons, and higher-dimensional objects.

## 🧠 Fundamental Building Blocks

### **Geometric Primitives**

The basic operations that form the foundation of geometric algorithms:

#### **1. Orientation Test (CCW/CW)**

Determines if three points make a left turn (counter-clockwise), right turn (clockwise), or are collinear:

```rust
/// Cross product determines orientation
/// Returns: > 0 (CCW), < 0 (CW), = 0 (collinear)
fn orientation(p: Point, q: Point, r: Point) -> i64 {
    (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x)
}

// Equivalent using cross product
fn ccw(a: Point, b: Point, c: Point) -> i64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}
```

**Applications**:
- Convex hull construction
- Polygon orientation
- Point in polygon tests
- Line segment intersection

#### **2. Distance Calculations**

```rust
// Euclidean distance (exact, uses sqrt)
fn euclidean_distance(p1: Point, p2: Point) -> f64 {
    let dx = (p1.x - p2.x) as f64;
    let dy = (p1.y - p2.y) as f64;
    (dx * dx + dy * dy).sqrt()
}

// Squared distance (faster, avoids sqrt)
fn distance_squared(p1: Point, p2: Point) -> i64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    dx * dx + dy * dy
}

// Manhattan distance (grid-based)
fn manhattan_distance(p1: Point, p2: Point) -> i64 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}
```

**Optimization Tip**: Use squared distances when only comparing (same as AoC Day 8!), avoid `sqrt()` when possible.

#### **3. Line Segment Intersection**

```rust
/// Check if two line segments intersect
fn segments_intersect(p1: Point, q1: Point, p2: Point, q2: Point) -> bool {
    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);
    
    // General case: different orientations
    if o1 * o2 < 0 && o3 * o4 < 0 {
        return true;
    }
    
    // Special cases: collinear points
    if o1 == 0 && on_segment(p1, p2, q1) { return true; }
    if o2 == 0 && on_segment(p1, q2, q1) { return true; }
    if o3 == 0 && on_segment(p2, p1, q2) { return true; }
    if o4 == 0 && on_segment(p2, q1, q2) { return true; }
    
    false
}

fn on_segment(p: Point, q: Point, r: Point) -> bool {
    q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) &&
    q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
}
```

#### **4. Point in Polygon**

```rust
/// Ray casting algorithm - count intersections with edges
fn point_in_polygon(point: Point, polygon: &[Point]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    
    for i in 0..n {
        let j = (i + 1) % n;
        let (pi, pj) = (polygon[i], polygon[j]);
        
        // Check if ray crosses edge
        if ((pi.y > point.y) != (pj.y > point.y)) &&
           (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x) {
            inside = !inside;
        }
    }
    
    inside
}
```

## 📚 Classic Problems and Algorithms

### **1. Convex Hull** - O(n log n)

Find the smallest convex polygon containing all points.

**Algorithms**:
- **[[convex-hull-algorithm#Graham Scan]]**: Sort by angle, scan with stack
- **[[convex-hull-algorithm#Monotone Chain]]**: Andrew's algorithm (most robust)
- **[[convex-hull-algorithm#Jarvis March]]**: Gift wrapping, O(nh) for sparse hulls
- **[[convex-hull-algorithm#QuickHull]]**: Divide-and-conquer approach

**Applications**: Collision detection, outlier detection, diameter finding

### **2. Closest Pair of Points** - O(n log n)

Find the two closest points in a set.

```rust
/// Divide-and-conquer closest pair algorithm
fn closest_pair(points: &mut [Point]) -> f64 {
    points.sort_by_key(|p| p.x);
    closest_pair_recursive(points)
}

fn closest_pair_recursive(points: &[Point]) -> f64 {
    let n = points.len();
    
    // Base case: brute force for small sets
    if n <= 3 {
        return brute_force_closest(points);
    }
    
    // Divide
    let mid = n / 2;
    let (left, right) = points.split_at(mid);
    
    // Conquer
    let dl = closest_pair_recursive(left);
    let dr = closest_pair_recursive(right);
    let d = dl.min(dr);
    
    // Combine: check strip around midline
    let midx = points[mid].x;
    let strip: Vec<_> = points.iter()
        .filter(|p| (p.x - midx).abs() < d as i64)
        .copied()
        .collect();
    
    strip_closest(&strip, d)
}
```

**Applications**: Collision detection, clustering, spatial databases

### **3. Line Sweep Algorithms**

Process geometric objects by sweeping a line across the plane.

**Classic Applications**:
- **Segment intersection**: Find all pairs of intersecting segments
- **Rectangle union**: Compute area covered by overlapping rectangles
- **Voronoi diagrams**: Fortune's algorithm

```rust
/// Line sweep for segment intersections
struct Event {
    x: i64,
    event_type: EventType,
    segment_id: usize,
}

enum EventType {
    Start,    // Segment starts
    End,      // Segment ends
    Intersection, // Segments cross
}

fn find_intersections(segments: &[Segment]) -> Vec<Point> {
    let mut events = Vec::new();
    let mut active = BTreeSet::new();
    let mut intersections = Vec::new();
    
    // Create events for segment endpoints
    for (id, seg) in segments.iter().enumerate() {
        events.push(Event { x: seg.start.x, event_type: EventType::Start, segment_id: id });
        events.push(Event { x: seg.end.x, event_type: EventType::End, segment_id: id });
    }
    
    events.sort_by_key(|e| e.x);
    
    // Process events left to right
    for event in events {
        match event.event_type {
            EventType::Start => {
                active.insert(event.segment_id);
                // Check for intersections with neighbors
            }
            EventType::End => {
                active.remove(&event.segment_id);
            }
            EventType::Intersection => {
                intersections.push(/* intersection point */);
            }
        }
    }
    
    intersections
}
```

### **4. Polygon Triangulation** - O(n log n)

Decompose a polygon into triangles.

**Methods**:
- **Ear clipping**: Simple, O(n²) naive, O(n) with proper data structures
- **Monotone decomposition**: Divide into y-monotone polygons, then triangulate
- **Delaunay triangulation**: For point sets, maximizes minimum angle

**Applications**: Graphics rendering, finite element analysis, mesh generation

### **5. Range Searching**

Query points within a region.

**Data Structures**:
- **k-d trees**: O(√n + k) for k results in 2D
- **Range trees**: O(log^d n + k) for d dimensions
- **Quadtrees**: Spatial partitioning for 2D
- **R-trees**: For rectangles and spatial databases

```rust
/// Simple 2D range tree for orthogonal range queries
struct RangeTree2D {
    root: Option<Box<Node>>,
}

struct Node {
    point: Point,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    y_tree: Vec<Point>, // Secondary structure sorted by y
}

impl RangeTree2D {
    fn range_query(&self, x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> Vec<Point> {
        let mut result = Vec::new();
        self.query_x_range(&self.root, x_min, x_max, y_min, y_max, &mut result);
        result
    }
    
    fn query_x_range(
        &self,
        node: &Option<Box<Node>>,
        x_min: i64,
        x_max: i64,
        y_min: i64,
        y_max: i64,
        result: &mut Vec<Point>,
    ) {
        // Binary search on x, then scan y_tree for y range
        // Implementation details...
    }
}
```

### **6. Voronoi Diagrams and Delaunay Triangulation**

**Voronoi Diagram**: Partition space into regions based on closest point  
**Delaunay Triangulation**: Dual of Voronoi, connects points such that no point is inside circumcircle

**Applications**:
- Nearest neighbor queries
- Mesh generation
- Path planning
- Geographic analysis

## 🎮 Implementation Libraries

### **Rust Crates for Computational Geometry**

| Crate | Focus | Strengths |
|-------|-------|-----------|
| **[[glam-game-math-library]]** | Vectors, matrices, transforms | SIMD performance, game math |
| **geo** | Geographic operations | Polygons, lines, GeoJSON |
| **geo-types** | Geometric primitives | Point, Line, Polygon types |
| **robust** | Exact predicates | Numerical stability |
| **spade** | Delaunay triangulation | High-quality triangulation |
| **rstar** | R-tree spatial index | Fast spatial queries |

### **Using glam for Geometric Computations**

```rust
use glam::*;

// 2D geometric operations with glam
let p1 = Vec2::new(1.0, 2.0);
let p2 = Vec2::new(4.0, 6.0);

// Distance (both methods available)
let dist = p1.distance(p2);
let dist_sq = p1.distance_squared(p2); // Faster!

// Dot product for projections
let v = Vec2::new(3.0, 4.0);
let n = Vec2::new(1.0, 0.0);
let projection = v.dot(n) * n;

// Cross product in 3D
let v1 = Vec3::new(1.0, 0.0, 0.0);
let v2 = Vec3::new(0.0, 1.0, 0.0);
let normal = v1.cross(v2); // (0, 0, 1)

// Transformations
let rotation = Mat2::from_angle(std::f32::consts::FRAC_PI_2);
let rotated = rotation * p1;
```

## 🔢 Integer vs Floating-Point Geometry

### **Integer Geometry (Exact)**

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

// Cross product stays exact (no floating-point error)
fn cross_product(a: Point, b: Point, c: Point) -> i64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}
```

**Advantages**:
- No floating-point precision errors
- Exact comparisons
- Faster on modern CPUs
- Perfect for AoC and competitive programming

**Disadvantages**:
- Overflow risk for large coordinates
- No direct angle calculations
- Limited to rational coordinates

### **Floating-Point Geometry**

```rust
#[derive(Clone, Copy)]
struct PointF {
    x: f64,
    y: f64,
}

// Can compute angles, but beware precision
fn angle_between(a: PointF, b: PointF) -> f64 {
    (b.y - a.y).atan2(b.x - a.x)
}
```

**Advantages**:
- Natural for angles and rotations
- Handles irrational values
- Good for graphics and physics

**Disadvantages**:
- Floating-point precision errors
- Need epsilon comparisons
- Slower than integer operations

### **Best Practice: Hybrid Approach**

```rust
// Use integers for exact predicates
fn orientation(a: Point, b: Point, c: Point) -> i64 {
    cross_product(a, b, c) // Exact
}

// Convert to float only for measurements
fn distance(a: Point, b: Point) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    (dx * dx + dy * dy).sqrt()
}
```

## 🎯 Advent of Code Applications

Computational geometry appears regularly in AoC:

### **Common Patterns**

| Problem Type | Geometric Concept | Examples |
|--------------|-------------------|----------|
| Grid navigation | Manhattan distance | 2015 Day 3, 2019 Day 3 |
| Polygon areas | Shoelace formula | 2023 Day 18 |
| Line intersections | Ray casting | 2019 Day 10 |
| 3D rotations | Matrix transforms | 2021 Day 19 |
| Flood fill | Connected regions | 2024 Day 12 |
| Pathfinding | Euclidean distance | Multiple years |

### **Example: Shoelace Formula for Polygon Area**

```rust
/// Calculate area of polygon using shoelace formula
fn polygon_area(vertices: &[Point]) -> i64 {
    let n = vertices.len();
    let mut sum = 0i64;
    
    for i in 0..n {
        let j = (i + 1) % n;
        sum += vertices[i].x * vertices[j].y;
        sum -= vertices[j].x * vertices[i].y;
    }
    
    sum.abs() / 2
}

// Used in AoC 2023 Day 18 for trench area calculation
```

### **Example: Pick's Theorem**

```rust
/// Pick's theorem: A = i + b/2 - 1
/// Where A = area, i = interior points, b = boundary points
fn interior_points(area: i64, boundary_points: i64) -> i64 {
    area - boundary_points / 2 + 1
}

// Useful for grid problems where you know area and boundary
```

## 🚀 Performance Optimization Techniques

### **1. Avoid Floating-Point When Possible**

```rust
// ❌ SLOW: Uses sqrt and floating-point
fn compare_distances_slow(p1: Point, p2: Point, threshold: f64) -> bool {
    let dist = euclidean_distance(p1, p2);
    dist < threshold
}

// ✅ FAST: Compare squared distances (integer)
fn compare_distances_fast(p1: Point, p2: Point, threshold_sq: i64) -> bool {
    distance_squared(p1, p2) < threshold_sq
}
```

### **2. Use Bounding Boxes for Early Rejection**

```rust
struct BoundingBox {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl BoundingBox {
    fn intersects(&self, other: &BoundingBox) -> bool {
        !(self.max_x < other.min_x || other.max_x < self.min_x ||
          self.max_y < other.min_y || other.max_y < self.min_y)
    }
    
    fn contains(&self, point: Point) -> bool {
        point.x >= self.min_x && point.x <= self.max_x &&
        point.y >= self.min_y && point.y <= self.max_y
    }
}

// Check bounding boxes before expensive geometric tests
fn detailed_intersection(a: &Polygon, b: &Polygon) -> bool {
    if !a.bounding_box().intersects(&b.bounding_box()) {
        return false; // Quick rejection
    }
    // Expensive polygon-polygon intersection test...
}
```

### **3. Spatial Data Structures**

```rust
// Grid-based spatial hashing for 2D points
struct SpatialHash {
    grid: HashMap<(i64, i64), Vec<Point>>,
    cell_size: i64,
}

impl SpatialHash {
    fn insert(&mut self, point: Point) {
        let cell = self.get_cell(point);
        self.grid.entry(cell).or_insert(Vec::new()).push(point);
    }
    
    fn get_cell(&self, point: Point) -> (i64, i64) {
        (point.x / self.cell_size, point.y / self.cell_size)
    }
    
    fn nearby(&self, point: Point, radius: i64) -> Vec<Point> {
        let cell = self.get_cell(point);
        let mut results = Vec::new();
        
        // Check neighboring cells
        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(points) = self.grid.get(&(cell.0 + dx, cell.1 + dy)) {
                    results.extend(points.iter().filter(|p| {
                        distance_squared(**p, point) <= radius * radius
                    }));
                }
            }
        }
        
        results
    }
}
```

## 🧪 Testing Geometric Algorithms

### **Edge Cases to Consider**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_collinear_points() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 1, y: 1 };
        let p3 = Point { x: 2, y: 2 };
        assert_eq!(orientation(p1, p2, p3), 0);
    }
    
    #[test]
    fn test_degenerate_polygon() {
        // Polygon with < 3 vertices
        let empty: Vec<Point> = vec![];
        assert_eq!(polygon_area(&empty), 0);
        
        let line = vec![Point{x:0,y:0}, Point{x:1,y:1}];
        assert_eq!(polygon_area(&line), 0);
    }
    
    #[test]
    fn test_duplicate_points() {
        let points = vec![
            Point{x:0,y:0}, Point{x:1,y:1}, Point{x:1,y:1}, Point{x:2,y:2}
        ];
        // Algorithm should handle duplicates gracefully
    }
    
    #[test]
    fn test_integer_overflow() {
        let large = Point { x: i64::MAX / 2, y: i64::MAX / 2 };
        // distance_squared would overflow!
        // Use checked arithmetic or i128 for intermediate calculations
    }
}
```

## 📖 Learning Path

### **Beginner**
1. ✅ Master basic primitives (CCW test, distance)
2. ✅ Implement point in polygon
3. ✅ Solve 2D grid problems (AoC)
4. ✅ Understand integer vs floating-point trade-offs

### **Intermediate**
1. 🔄 Implement convex hull (Monotone Chain recommended)
2. 🔄 Line segment intersection
3. 🔄 Closest pair of points
4. 🔄 Polygon area (Shoelace formula)
5. 🔄 Basic spatial data structures

### **Advanced**
1. 🔄 Line sweep algorithms
2. 🔄 Voronoi diagrams and Delaunay triangulation
3. 🔄 3D geometry (using glam or nalgebra)
4. 🔄 Robust geometric predicates
5. 🔄 Dynamic geometric structures

### **Expert**
1. 🔄 Arrangement of lines and circles
2. 🔄 Motion planning algorithms
3. 🔄 Mesh generation and refinement
4. 🔄 Curved object intersection

## 📚 Reference Books

### **Classic Texts**
- **Computational Geometry: Algorithms and Applications** by de Berg et al. (the standard textbook)
- **Computational Geometry in C** by Joseph O'Rourke
- **Introduction to Algorithms (CLRS)** - Chapter 33: Computational Geometry

### **Online Resources**
- **CP-Algorithms**: Geometry section
- **Handbook of Discrete and Computational Geometry**
- **Jeff Erickson's Algorithms Notes**: Computational geometry chapters

## 🔗 Related Topics

### **Core Algorithms**
- **[[convex-hull-algorithm]]**: Finding outer boundaries
- **[[union-find-algorithm]]**: For connectivity in spatial problems
- **[[graph-algorithms]]**: Geometric graphs and visibility

### **Implementation Tools**
- **[[glam-game-math-library]]**: SIMD vectors/matrices for performance
- **Mission 6 Grid**: 2D grid navigation and flood fill
- **Mission 8 Graph**: Graph representation of spatial relationships

### **Mathematical Foundations**
- **[[linear-algebra]]**: Vector and matrix operations
- **[[cross-product]]**: Fundamental geometric primitive
- **[[affine-transformations]]**: Rotations, translations, scaling

### **Applications**
- **[[collision-detection]]**: Game physics
- **[[path-planning]]**: Robotics and navigation
- **[[computer-graphics]]**: Rendering and visualization
- **[[geographic-information-systems]]**: Spatial data analysis

---

## *Links:*

**Algorithms:** [[convex-hull-algorithm]] | [[closest-pair]] | [[line-sweep]] | [[polygon-triangulation]] | [[voronoi-diagrams]]

**Data Structures:** [[spatial-hash]] | [[kdtree]] | [[range-tree]] | [[quadtree]] | [[r-tree]]

**Primitives:** [[orientation-test]] | [[line-intersection]] | [[point-in-polygon]] | [[distance-functions]]

**Libraries:** [[glam-game-math-library]] | [[geo-crate]] | [[robust-predicates]] | [[spade]]

**Related:** [[graph-algorithms]] | [[union-find-algorithm]] | [[mission-6]] | [[mission-8]]

**Applications:** [[advent-of-code]] | [[competitive-programming]] | [[game-development]] | [[robotics]]

---

*Computational geometry bridges pure mathematics and practical algorithm design, providing the tools to efficiently solve spatial problems. From simple point-in-polygon tests to complex Voronoi diagrams, these algorithms power everything from video games to geographic information systems. Mastering computational geometry opens doors to graphics programming, robotics, competitive programming, and a deeper understanding of how algorithms interact with spatial data.*
