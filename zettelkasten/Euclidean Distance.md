# 📏 Euclidean Distance

**Straight-line distance in continuous space - the "crow flies" metric**

## 🎯 Core Concept

**Euclidean Distance** measures the straight-line distance between two points in Euclidean space. It's the distance "as the crow flies" - the shortest possible path when unrestricted movement is allowed.

**Formula:**

```
distance = √[(x₁ - x₂)² + (y₁ - y₂)²]
```

**Named after Euclid**, the ancient Greek mathematician who founded geometry. This is the "ordinary" distance we intuitively understand in the real world.

---

## 📐 Mathematical Definition

### **2D Space (Most Common)**

```rust
fn euclidean_distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    (dx * dx + dy * dy).sqrt()
}

// Example:
let start = (0.0, 0.0);
let end = (3.0, 4.0);
let dist = euclidean_distance(start, end);  // 5.0 (Pythagorean triple!)
```

### **3D Space**

```rust
fn euclidean_distance_3d(p1: (f64, f64, f64), p2: (f64, f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    let dz = p1.2 - p2.2;
    (dx * dx + dy * dy + dz * dz).sqrt()
}
```

### **N-Dimensional**

```rust
fn euclidean_distance_nd(p1: &[f64], p2: &[f64]) -> f64 {
    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}
```

### **Squared Distance (Optimization)**

```rust
// When only comparing distances, avoid sqrt() for performance
fn euclidean_distance_squared(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    dx * dx + dy * dy
}

// Usage for comparison:
fn is_closer(p1: Point, p2: Point, target: Point) -> bool {
    p1.distance_squared(target) < p2.distance_squared(target)
    // No sqrt needed!
}
```

---

## 🎮 Visual Representation

### **The Right Triangle**

```
     B (3, 4)
    /|
   / |
  /  | 4
 /   |
A----→
 (0,0) 3

Euclidean: √(3² + 4²) = √25 = 5
This is the Pythagorean theorem!
```

### **Euclidean Distance Circle**

```
Points at distance 2 from center (C):

    X
  X   X
 X  C  X
  X   X
    X

Forms a perfect circle!
```

### **Comparison with Grid Metrics**

```
Point A (0,0) to Point B (3,4):

Manhattan:  |3| + |4| = 7 (grid path)
Chebyshev:  max(|3|, |4|) = 4 (diagonal + straight)
Euclidean:  √(3² + 4²) = 5 (straight line)

Euclidean is always ≤ other metrics!
```

---

## 🚀 Implementation Patterns

### **Point Struct with Euclidean Distance**

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    /// Calculate Euclidean distance to another point
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    
    /// Squared distance (faster for comparisons)
    fn distance_squared(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
    
    /// Check if point is within radius
    fn within_radius(&self, center: &Point, radius: f64) -> bool {
        self.distance_squared(center) <= radius * radius
    }
}
```

### **Nearest Neighbor Search**

```rust
fn find_nearest(points: &[Point], target: Point) -> Option<Point> {
    points.iter()
        .min_by(|a, b| {
            a.distance_squared(&target)
                .partial_cmp(&b.distance_squared(&target))
                .unwrap()
        })
        .copied()
}

// K-nearest neighbors
fn find_k_nearest(points: &[Point], target: Point, k: usize) -> Vec<Point> {
    let mut distances: Vec<_> = points.iter()
        .map(|p| (*p, p.distance_squared(&target)))
        .collect();
    
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    distances.into_iter().take(k).map(|(p, _)| p).collect()
}
```

### **Circle Intersection / Range Query**

```rust
/// Find all points within a circular range
fn points_in_circle(points: &[Point], center: Point, radius: f64) -> Vec<Point> {
    let radius_squared = radius * radius;
    points.iter()
        .filter(|p| p.distance_squared(&center) <= radius_squared)
        .copied()
        .collect()
}

/// Check if two circles intersect
fn circles_intersect(
    center1: Point, 
    radius1: f64, 
    center2: Point, 
    radius2: f64
) -> bool {
    let dist = center1.distance(&center2);
    dist <= radius1 + radius2
}
```

### **Line Distance Calculation**

```rust
/// Distance from point to line segment
fn point_to_segment_distance(p: Point, a: Point, b: Point) -> f64 {
    let ab = Point::new(b.x - a.x, b.y - a.y);
    let ap = Point::new(p.x - a.x, p.y - a.y);
    
    let ab_len_sq = ab.x * ab.x + ab.y * ab.y;
    
    if ab_len_sq == 0.0 {
        return p.distance(&a);  // Line segment is a point
    }
    
    // Project point onto line, clamped to segment
    let t = ((ap.x * ab.x + ap.y * ab.y) / ab_len_sq).clamp(0.0, 1.0);
    
    let projection = Point::new(a.x + t * ab.x, a.y + t * ab.y);
    p.distance(&projection)
}
```

---

## 🎯 Real-World Applications

### **1. GPS and Navigation**

```rust
// Simplified distance between GPS coordinates (flat Earth approximation)
fn gps_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let earth_radius = 6371.0; // km
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    
    // Euclidean approximation for short distances
    let dx = earth_radius * dlon * lat1.to_radians().cos();
    let dy = earth_radius * dlat;
    
    (dx * dx + dy * dy).sqrt()
}
```

### **2. Collision Detection**

```rust
struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    fn collides_with(&self, other: &Circle) -> bool {
        let dist = self.center.distance(&other.center);
        dist < self.radius + other.radius
    }
}
```

### **3. Clustering (K-Means)**

```rust
fn assign_to_clusters(points: &[Point], centroids: &[Point]) -> Vec<usize> {
    points.iter()
        .map(|p| {
            centroids.iter()
                .enumerate()
                .min_by(|(_, c1), (_, c2)| {
                    p.distance_squared(c1)
                        .partial_cmp(&p.distance_squared(c2))
                        .unwrap()
                })
                .map(|(idx, _)| idx)
                .unwrap()
        })
        .collect()
}
```

### **4. Computer Vision / Image Processing**

```rust
// Calculate color distance in RGB space
fn color_distance(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> f64 {
    let dr = (rgb1.0 as f64 - rgb2.0 as f64);
    let dg = (rgb1.1 as f64 - rgb2.1 as f64);
    let db = (rgb1.2 as f64 - rgb2.2 as f64);
    (dr * dr + dg * dg + db * db).sqrt()
}
```

### **5. Physics Simulations**

```rust
// Gravitational force depends on Euclidean distance
fn gravitational_force(m1: f64, m2: f64, p1: Point, p2: Point) -> f64 {
    const G: f64 = 6.674e-11;
    let distance = p1.distance(&p2);
    G * m1 * m2 / (distance * distance)
}
```

---

## 📊 Properties & Characteristics

### **Mathematical Properties**

1. **Non-negative**: `d(p, q) ≥ 0`
2. **Identity**: `d(p, p) = 0`
3. **Symmetry**: `d(p, q) = d(q, p)`
4. **Triangle Inequality**: `d(p, r) ≤ d(p, q) + d(q, r)`
5. **Pythagorean Theorem**: `c² = a² + b²` where c is Euclidean distance

### **Computational Properties**

- **Time Complexity**: O(n) for n dimensions
- **Space Complexity**: O(1)
- **Requires**: Floating-point arithmetic (`f32` or `f64`)
- **sqrt() Cost**: Expensive operation - avoid when possible

### **Performance Optimization: Squared Distance**

```rust
// ❌ Slow: Computing actual distance for comparison
if p1.distance(&target) < p2.distance(&target) {
    // Two sqrt() calls!
}

// ✅ Fast: Compare squared distances
if p1.distance_squared(&target) < p2.distance_squared(&target) {
    // No sqrt() needed!
}
```

### **Comparison with Other Metrics**

| Metric | Movement | Formula | Relative Size |
|--------|----------|---------|---------------|
| [[Manhattan Distance\|Manhattan]] | 4-connected | \|Δx\| + \|Δy\| | Largest |
| [[Chebyshev Distance\|Chebyshev]] | 8-connected | max(\|Δx\|, \|Δy\|) | Medium |
| **Euclidean** | Continuous | √(Δx² + Δy²) | Smallest |

**Ordering Property:**

```
Euclidean ≤ Chebyshev ≤ Manhattan
(Shortest possible path)
```

---

## 🎮 AoC Applications

### **Pattern 1: Closest Point to Target**

```rust
// Find asteroid closest to monitoring station
fn find_closest_asteroid(
    asteroids: &[Point], 
    station: Point
) -> Option<Point> {
    asteroids.iter()
        .min_by(|a, b| {
            a.distance_squared(&station)
                .partial_cmp(&b.distance_squared(&station))
                .unwrap()
        })
        .copied()
}
```

### **Pattern 2: Range Queries**

```rust
// Count points within sensor range
fn count_in_range(points: &[Point], sensor: Point, range: f64) -> usize {
    let range_squared = range * range;
    points.iter()
        .filter(|p| p.distance_squared(&sensor) <= range_squared)
        .count()
}
```

### **Pattern 3: Continuous Space Movement**

```rust
// Simulate projectile motion with Euclidean distance
struct Projectile {
    position: Point,
    velocity: Point,
}

impl Projectile {
    fn step(&mut self, dt: f64) {
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
    }
    
    fn distance_to_target(&self, target: Point) -> f64 {
        self.position.distance(&target)
    }
}
```

### **Pattern 4: Clustering / Grouping**

```rust
// Group points by proximity (simple clustering)
fn simple_cluster(points: &[Point], max_distance: f64) -> Vec<Vec<Point>> {
    let mut clusters = Vec::new();
    let mut remaining: Vec<_> = points.iter().copied().collect();
    
    while let Some(seed) = remaining.pop() {
        let mut cluster = vec![seed];
        remaining.retain(|p| {
            if seed.distance(p) <= max_distance {
                cluster.push(*p);
                false
            } else {
                true
            }
        });
        clusters.push(cluster);
    }
    
    clusters
}
```

---

## ⚡ Performance Optimizations

### **1. Avoid sqrt() When Possible**

```rust
// ❌ Slow: Unnecessary sqrt()
fn is_within_radius(p1: Point, p2: Point, radius: f64) -> bool {
    p1.distance(&p2) < radius  // Computes sqrt!
}

// ✅ Fast: Compare squared values
fn is_within_radius_fast(p1: Point, p2: Point, radius: f64) -> bool {
    p1.distance_squared(&p2) < radius * radius  // No sqrt!
}
```

### **2. Fast Inverse Square Root (Advanced)**

```rust
// Approximate 1/sqrt(x) quickly (Quake III algorithm)
// Use only when approximation is acceptable
fn fast_inv_sqrt(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);
    y * (1.5 - 0.5 * x * y * y)  // Newton-Raphson iteration
}
```

### **3. Spatial Data Structures**

```rust
// Use KD-tree or quadtree for efficient nearest neighbor queries
// O(n) → O(log n) for large datasets
```

### **4. SIMD Optimizations**

```rust
// Use SIMD for parallel distance calculations
// Process multiple distances simultaneously on modern CPUs
```

---

## 🚫 Common Pitfalls

### **Pitfall 1: Floating-Point Precision**

```rust
// ❌ Dangerous: Exact equality check
if p1.distance(&p2) == 5.0 {
    // May fail due to floating-point error!
}

// ✅ Safe: Epsilon comparison
const EPSILON: f64 = 1e-10;
if (p1.distance(&p2) - 5.0).abs() < EPSILON {
    // Handles floating-point precision
}
```

### **Pitfall 2: Integer Coordinates**

```rust
// When working with integer grids but need Euclidean distance
fn euclidean_int_coords(p1: (i32, i32), p2: (i32, i32)) -> f64 {
    let dx = (p1.0 - p2.0) as f64;
    let dy = (p1.1 - p2.1) as f64;
    (dx * dx + dy * dy).sqrt()
}
```

### **Pitfall 3: Overflow with Large Coordinates**

```rust
// ❌ Can overflow with large integers
let dist_squared = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);

// ✅ Convert to float first
let dx = (x1 - x2) as f64;
let dy = (y1 - y2) as f64;
let dist_squared = dx * dx + dy * dy;
```

---

## 🔗 Connected Concepts

### **Related Zettelkasten Pages**

- [[Manhattan Distance]] - Grid-based 4-connected distance
- [[Chebyshev Distance]] - Grid-based 8-connected distance
- [[A-Star-Algorithm-Deep-Dive]] - Can use Euclidean distance as heuristic
- [[Week 4 Overview]] - Spatial algorithms and distance metrics
- [[mission-6]] - Grid navigation (usually uses Manhattan/Chebyshev)

### **When to Use Each Metric**

- **Euclidean**: Continuous space, physics, real-world straight-line
- **Manhattan**: Grid movement (4-connected), no diagonals
- **Chebyshev**: Grid movement (8-connected), chess king

### **Related Algorithms**

- **K-Nearest Neighbors**: Uses Euclidean distance
- **K-Means Clustering**: Uses Euclidean distance
- **A\* Search**: Can use Euclidean as heuristic (continuous space)
- **Collision Detection**: Circle/sphere collision uses Euclidean

---

## 💡 Key Takeaways

1. **Euclidean = Straight Line**: Shortest distance in continuous space
2. **Pythagorean Theorem**: Foundation of 2D Euclidean distance
3. **sqrt() is Expensive**: Use squared distance for comparisons
4. **Floating-Point Required**: Need `f32` or `f64` for accuracy
5. **Smallest Metric**: Always ≤ Manhattan and Chebyshev distances
6. **Real-World Default**: Natural choice for physical simulations

**When to Use Euclidean Distance:**

```
✅ Continuous space (not grid)
✅ Physics simulations
✅ Real-world navigation (GPS)
✅ Collision detection (circles/spheres)
✅ Clustering algorithms
✅ Computer vision / image processing

❌ Discrete grid movement
❌ Integer-only arithmetic needed
❌ 4-connected or 8-connected grids
```

**Performance Rule:**

```
If comparing distances: Use distance_squared()
If need actual distance: Use distance() with sqrt()
```

**Euclidean Distance Philosophy:**
> "The shortest distance between two points is a straight line. Euclidean distance captures this fundamental geometric truth." 📐

---

*Tags: #euclidean-distance #continuous-space #pythagorean-theorem #metrics #algorithms #physics #clustering*

*Links: [[zettel-index]] | [[Manhattan Distance]] | [[Chebyshev Distance]] | [[A-Star-Algorithm-Deep-Dive]] | [[Week 4 Overview]] | [[mission-6]]*
