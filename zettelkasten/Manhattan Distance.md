# 📏 Manhattan Distance

**4-connected grid distance metric for orthogonal movement**

## 🎯 Core Concept

**Manhattan Distance** (also called **taxicab distance** or **L1 distance**) measures the distance between two points by summing the absolute differences of their coordinates. It represents the shortest path when you can only move horizontally or vertically (4-connected movement).

**Formula:**

```
distance = |x₁ - x₂| + |y₁ - y₂|
```

**Why "Manhattan"?** Named after the grid layout of Manhattan streets where you can't cut diagonally through buildings - you must follow the street grid.

---

## 📐 Mathematical Definition

### **2D Grid (Most Common)**

```rust
fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> u32 {
    ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()) as u32
}

// Example:
let start = (0, 0);
let end = (3, 4);
let dist = manhattan_distance(start, end);  // 3 + 4 = 7
```

### **3D Space**

```rust
fn manhattan_distance_3d(p1: (i32, i32, i32), p2: (i32, i32, i32)) -> u32 {
    ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()) as u32
}
```

### **N-Dimensional**

```rust
fn manhattan_distance_nd(p1: &[i32], p2: &[i32]) -> u32 {
    p1.iter()
        .zip(p2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>() as u32
}
```

---

## 🎮 Visual Representation

### **4-Connected Movement Pattern**

```
Distance = 5 from S to E:

  0 1 2 3 4
0 S─→→→↓   
1       ↓   
2       E   

Moves: Right(3) + Down(2) = 5 steps
All paths have same length: 5
```

### **Manhattan Distance Circle**

```
Points at distance 3 from center (C):

    3
  2 X 2
1 X C X 1
  2 X 2
    3

Forms a diamond/rhombus shape!
```

### **vs [[Euclidean Distance]]**

```
Manhattan: |3-0| + |4-0| = 7
Euclidean: √(3² + 4²) = 5

     E
    /|
   / |
  /  | 4
 /   |
S----→
  3

Manhattan follows the grid (7 steps)
Euclidean cuts diagonally (5 units)
```

---

## 🚀 Implementation Patterns

### **Basic Distance Calculation**

```rust
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    fn manhattan_distance(&self, other: &Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

// Usage:
let p1 = Point::new(0, 0);
let p2 = Point::new(3, 4);
let dist = p1.manhattan_distance(&p2);  // 7
```

### **Grid-Based Distance Map**

```rust
use std::collections::VecDeque;

/// Calculate Manhattan distance from start to all reachable cells
fn distance_map(grid: &[Vec<bool>], start: Point) -> Vec<Vec<Option<u32>>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut distances = vec![vec![None; width]; height];
    let mut queue = VecDeque::new();
    
    distances[start.y as usize][start.x as usize] = Some(0);
    queue.push_back(start);
    
    while let Some(pos) = queue.pop_front() {
        let current_dist = distances[pos.y as usize][pos.x as usize].unwrap();
        
        // 4-connected neighbors (Manhattan movement)
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = pos.x + dx;
            let ny = pos.y + dy;
            
            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                let ux = nx as usize;
                let uy = ny as usize;
                
                if grid[uy][ux] && distances[uy][ux].is_none() {
                    distances[uy][ux] = Some(current_dist + 1);
                    queue.push_back(Point::new(nx, ny));
                }
            }
        }
    }
    
    distances
}
```

### **Finding All Points at Distance D**

```rust
fn points_at_manhattan_distance(center: Point, distance: u32) -> Vec<Point> {
    let mut points = Vec::new();
    let d = distance as i32;
    
    for dx in -d..=d {
        let remaining = d - dx.abs();
        // Two points: one above, one below (unless on axis)
        if remaining == 0 {
            points.push(Point::new(center.x + dx, center.y));
        } else {
            points.push(Point::new(center.x + dx, center.y + remaining));
            points.push(Point::new(center.x + dx, center.y - remaining));
        }
    }
    
    points
}

// Example: All points at distance 2 from origin
let pts = points_at_manhattan_distance(Point::new(0, 0), 2);
// Returns: [(2,0), (-2,0), (1,1), (1,-1), (-1,1), (-1,-1), (0,2), (0,-2)]
```

---

## 🎯 Real-World Applications

### **1. Grid-Based Pathfinding**

Manhattan distance is the **perfect heuristic for A\*** when:

- Movement is restricted to 4 directions (up, down, left, right)
- Diagonal movement is not allowed
- Each step has equal cost

```rust
// A* heuristic for 4-connected grid
fn heuristic(current: Point, goal: Point) -> u32 {
    current.manhattan_distance(&goal)
}

// This is admissible (never overestimates) for 4-connected grids!
```

### **2. Taxi Routing / City Navigation**

```rust
// Estimate taxi fare based on grid distance
fn estimate_fare(pickup: Point, dropoff: Point, rate_per_block: f64) -> f64 {
    let blocks = pickup.manhattan_distance(&dropoff) as f64;
    blocks * rate_per_block
}
```

### **3. Circuit Board Layout**

```rust
// Estimate wire length in Manhattan routing
fn wire_length(start: Point, end: Point) -> u32 {
    start.manhattan_distance(&end)
}
```

### **4. Sensor Coverage**

```rust
// Check if a point is within sensor range
fn in_sensor_range(sensor: Point, target: Point, range: u32) -> bool {
    sensor.manhattan_distance(&target) <= range
}
```

### **5. AoC Pattern: Beacon Exclusion Zone**

```rust
// Day 15 (2022): Beacons and sensors with Manhattan distance ranges
struct Sensor {
    position: Point,
    closest_beacon: Point,
    range: u32,
}

impl Sensor {
    fn new(position: Point, closest_beacon: Point) -> Self {
        let range = position.manhattan_distance(&closest_beacon);
        Self { position, closest_beacon, range }
    }
    
    fn covers(&self, point: Point) -> bool {
        self.position.manhattan_distance(&point) <= self.range
    }
}
```

---

## 📊 Properties & Characteristics

### **Mathematical Properties**

1. **Non-negative**: `d(p, q) ≥ 0`
2. **Identity**: `d(p, p) = 0`
3. **Symmetry**: `d(p, q) = d(q, p)`
4. **Triangle Inequality**: `d(p, r) ≤ d(p, q) + d(q, r)`

### **Computational Properties**

- **Time Complexity**: O(n) where n is number of dimensions
- **Space Complexity**: O(1) - no extra memory needed
- **Integer Arithmetic**: No floating-point errors
- **Overflow Safe**: Use checked arithmetic for large coordinates

### **Comparison with Other Metrics**

| Metric | Movement | Formula | Use Case |
|--------|----------|---------|----------|
| **Manhattan** | 4-connected | \|Δx\| + \|Δy\| | Grid pathfinding (no diagonals) |
| [[Chebyshev Distance\|Chebyshev]] | 8-connected | max(\|Δx\|, \|Δy\|) | Chess king movement |
| [[Euclidean Distance\|Euclidean]] | Continuous | √(Δx² + Δy²) | Real-world straight-line |

### **When Manhattan Distance is Optimal**

✅ 4-connected grid movement (most roguelikes, puzzles)
✅ City street navigation (grid-based roads)
✅ Circuit board routing (Manhattan routing)
✅ Warehouse robot navigation (orthogonal aisles)

### **When to Use Other Metrics**

❌ Diagonal movement allowed → Use [[Chebyshev Distance]]
❌ Continuous space → Use [[Euclidean Distance]]
❌ Weighted terrain → Use actual path cost

---

## 🎮 AoC Applications

### **Common AoC Patterns Using Manhattan Distance**

**Pattern 1: Sensor Coverage (2022 Day 15)**

```rust
// Check if point is covered by any sensor
fn is_covered(sensors: &[Sensor], point: Point) -> bool {
    sensors.iter().any(|s| s.covers(point))
}
```

**Pattern 2: Shortest Path in Grid**

```rust
// BFS with Manhattan distance tracking
fn shortest_path(grid: &[Vec<bool>], start: Point, goal: Point) -> Option<u32> {
    // BFS naturally finds shortest Manhattan path in 4-connected grid
    bfs_search(grid, start, goal)
}
```

**Pattern 3: Area Coverage**

```rust
// Count cells within Manhattan distance
fn count_cells_in_range(center: Point, range: u32, grid_size: u32) -> u32 {
    let mut count = 0;
    for y in 0..grid_size {
        for x in 0..grid_size {
            let p = Point::new(x as i32, y as i32);
            if center.manhattan_distance(&p) <= range {
                count += 1;
            }
        }
    }
    count
}
```

**Pattern 4: Closest Point Finding**

```rust
fn find_closest(points: &[Point], target: Point) -> Option<Point> {
    points.iter()
        .min_by_key(|p| p.manhattan_distance(&target))
        .copied()
}
```

---

## 🚀 Performance Optimizations

### **Avoid Repeated Calculations**

```rust
// Cache distance if used multiple times
let dist = p1.manhattan_distance(&p2);
if dist < threshold && expensive_check(p1, p2) {
    // Use cached dist
}
```

### **Early Exit in Range Checks**

```rust
fn in_range_optimized(p1: Point, p2: Point, max_dist: u32) -> bool {
    let dx = (p1.x - p2.x).abs();
    if dx > max_dist as i32 {
        return false;  // Early exit - no need to check y
    }
    let dy = (p1.y - p2.y).abs();
    (dx + dy) as u32 <= max_dist
}
```

### **Squared Distance for Comparisons**

```rust
// When only comparing distances, can avoid abs() sometimes
fn closer_manhattan(p1: Point, target: Point, p2: Point) -> bool {
    p1.manhattan_distance(&target) < p2.manhattan_distance(&target)
}
```

---

## 🔗 Connected Concepts

### **Related Zettelkasten Pages**

- [[Chebyshev Distance]] - 8-connected grid distance (king moves)
- [[Euclidean Distance]] - Straight-line distance in continuous space
- [[A-Star-Algorithm-Deep-Dive]] - Uses Manhattan distance as heuristic
- [[BFS Patterns]] - Naturally finds shortest Manhattan paths
- [[Week 4 Overview]] - Day 22-23 cover grid navigation
- [[mission-6]] - Grid navigation and pathfinding

### **Mission Integration**

- **Mission6**: 4-connected grid navigation using Manhattan distance
- **Mission7**: Graph algorithms with Manhattan distance heuristics
- **AoC Solutions**: Many grid-based puzzles use Manhattan distance

### **Related Algorithms**

- **BFS**: Finds shortest path in unweighted grids (Manhattan distance)
- **A\* Search**: Uses Manhattan distance as heuristic function
- **Flood Fill**: Explores cells in Manhattan distance order

---

## 💡 Key Takeaways

1. **Manhattan = Grid Movement**: Measures distance when limited to orthogonal moves
2. **Perfect for 4-Connected Grids**: Natural metric for non-diagonal pathfinding
3. **Simple & Fast**: Just addition and absolute values - no sqrt() needed
4. **Integer Arithmetic**: No floating-point errors or precision issues
5. **Admissible Heuristic**: Perfect for A\* on 4-connected grids
6. **Forms Diamond Shape**: Points equidistant form rhombus, not circle

**When to Use Manhattan Distance:**

```
✅ 4-connected grid pathfinding
✅ City block navigation
✅ Warehouse routing
✅ Circuit board layout
✅ Sensor range checks (discrete grid)
✅ A* heuristic (no diagonal movement)

❌ Diagonal movement allowed
❌ Continuous space
❌ Chess king movement
❌ Flying/swimming (unrestricted)
```

**Manhattan Distance Philosophy:**
> "In a grid world where you can't cut corners, the shortest path is the sum of your perpendicular steps. Manhattan distance captures this perfectly." 🌆

---

*Tags: #manhattan-distance #grid-distance #pathfinding #metrics #algorithms #aoc-patterns #4-connected #heuristic*

*Links: [[zettel-index]] | [[Chebyshev Distance]] | [[Euclidean Distance]] | [[A-Star-Algorithm-Deep-Dive]] | [[BFS Patterns]] | [[Week 4 Overview]] | [[mission-6]]*
