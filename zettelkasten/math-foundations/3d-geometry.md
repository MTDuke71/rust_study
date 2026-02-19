# 3D Geometry

**Field**: Computational Geometry / Discrete Mathematics

**Prerequisites**: [[set-theory-fundamentals]], [[computational-geometry-basics]], [[linear-algebra-fundamentals]]

---

## 📐 Definition

**3D geometry** in the AoC/programming context deals with objects positioned in three-dimensional space, typically on an integer grid. The key extension from 2D: a third axis (z) adds depth, increasing neighbor counts and making visualization harder.

**Coordinate System**:
$$\mathbf{P} = (x, y, z) \in \mathbb{Z}^3$$

```
    z ↑
      |  y
      | ↗
      |/
      +───→ x
```

**Intuition**: Everything you know about 2D grids extends naturally, but the constants change. 4 neighbors become 6 (faces), 8 become 26 (including diagonals). Volumes replace areas.

---

## 🔑 Key Concepts

### **Adjacency in 3D**

The number of neighbors depends on what you consider "adjacent":

| Adjacency Type | Count | Shares | Analog (2D) |
|---------------|-------|--------|-------------|
| **Face-adjacent** (Von Neumann) | 6 | A face | 4 (cardinal) |
| **Edge-adjacent** | 12 | An edge | — |
| **Vertex-adjacent** (Moore) | 26 | At least a vertex | 8 (including diagonals) |

**Face-adjacent** (most common in AoC):
```rust
const NEIGHBORS_6: [(i32, i32, i32); 6] = [
    ( 1,  0,  0), (-1,  0,  0),  // ±x
    ( 0,  1,  0), ( 0, -1,  0),  // ±y
    ( 0,  0,  1), ( 0,  0, -1),  // ±z
];
```

**All 26 neighbors** (including diagonals):
```rust
// Generate all combinations of {-1, 0, 1}³ except (0, 0, 0)
let neighbors_26: Vec<(i32, i32, i32)> = (-1..=1)
    .flat_map(|dx| (-1..=1)
        .flat_map(move |dy| (-1..=1)
            .filter_map(move |dz| {
                if dx == 0 && dy == 0 && dz == 0 { None }
                else { Some((dx, dy, dz)) }
            })))
    .collect();
assert_eq!(neighbors_26.len(), 26);  // 3³ - 1
```

### **Why 6 Neighbors for Flood Fill?**

Face-adjacency means "shares a face" — the water can flow through shared faces but not through edges or corners. This matches physical fluid flow (Day 18's steam/water) and most AoC grid connectivity.

Diagonal adjacency (26 neighbors) would allow "squeezing through cracks," which is usually wrong for AoC problems.

---

### **Distance Metrics in 3D**

| Metric | Formula | Use Case |
|--------|---------|----------|
| **Manhattan** (L₁) | $\|x_1 - x_2\| + \|y_1 - y_2\| + \|z_1 - z_2\|$ | Grid movement, taxicab distance |
| **Chebyshev** (L∞) | $\max(\|x_1-x_2\|, \|y_1-y_2\|, \|z_1-z_2\|)$ | King movement in 3D |
| **Euclidean** (L₂) | $\sqrt{(x_1-x_2)^2 + (y_1-y_2)^2 + (z_1-z_2)^2}$ | True distance (rare in AoC) |

```rust
fn manhattan_3d(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}

fn chebyshev_3d(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32 {
    (a.0 - b.0).abs().max((a.1 - b.1).abs()).max((a.2 - b.2).abs())
}
```

**Manhattan distance in 3D** defines a regular octahedron (vs. a diamond/rhombus in 2D). All points at Manhattan distance ≤ r from the origin form a solid octahedron with $\frac{4r^3 + 6r^2 + 8r + 3}{3}$ integer lattice points.

---

### **Surface Area of Voxel Shapes**

For a set of unit cubes (voxels) on a grid:

$$\text{Surface Area} = \sum_{\text{cube } c} \sum_{\text{face } f} [f \text{ has no adjacent cube}]$$

Each cube contributes 6 faces. Each adjacent pair hides 2 faces (one from each cube).

**Equivalent formula**:
$$\text{Surface Area} = 6n - 2 \cdot (\text{number of adjacent pairs})$$

```rust
// Method 1: Check each face (AoC 2022 Day 18)
let set: HashSet<(i32, i32, i32)> = cubes.iter().copied().collect();
let surface: u64 = cubes.iter()
    .flat_map(|&(x, y, z)| NEIGHBORS_6.iter()
        .filter(move |&&(dx, dy, dz)| !set.contains(&(x+dx, y+dy, z+dz))))
    .count() as u64;

// Method 2: Count adjacent pairs
let adj_pairs = cubes.iter()
    .flat_map(|&(x, y, z)| NEIGHBORS_6.iter()
        .filter(move |&&(dx, dy, dz)| set.contains(&(x+dx, y+dy, z+dz))))
    .count() / 2;  // Each pair counted twice
let surface = 6 * cubes.len() - 2 * adj_pairs;
```

---

### **Volume and Bounding Box**

**Bounding box**: The smallest axis-aligned box containing all points.

$$\text{BBox} = [\min(x), \max(x)] \times [\min(y), \max(y)] \times [\min(z), \max(z)]$$

```rust
let min_x = cubes.iter().map(|c| c.0).min().unwrap();
let max_x = cubes.iter().map(|c| c.0).max().unwrap();
// ... same for y, z

let volume = (max_x - min_x + 1) * (max_y - min_y + 1) * (max_z - min_z + 1);
```

**Padded bounding box**: Extend by 1 in all directions to ensure BFS can wrap around all surfaces. Essential for exterior flood fill.

---

## 🎯 Algorithms

### **BFS/Flood Fill in 3D**

Identical to 2D flood fill, just with 6 neighbors instead of 4.

```rust
fn flood_fill_3d(
    start: (i32, i32, i32),
    occupied: &HashSet<(i32, i32, i32)>,
    bounds: ((i32, i32), (i32, i32), (i32, i32)),
) -> HashSet<(i32, i32, i32)> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(start);
    queue.push_back(start);

    while let Some((x, y, z)) = queue.pop_front() {
        for &(dx, dy, dz) in &NEIGHBORS_6 {
            let n = (x + dx, y + dy, z + dz);
            if in_bounds(n, bounds) && !occupied.contains(&n) && visited.insert(n) {
                queue.push_back(n);
            }
        }
    }
    visited
}
```

**Complexity**: O(V) where V = volume of bounded region.

**Applications**:
- AoC 2022 Day 18: Exterior surface area via flood from outside
- AoC 2020 Day 17: Conway Cubes (cellular automata in 3D/4D)

### **Gravity Simulation (Falling Bricks)**

For objects falling along the z-axis (AoC 2023 Day 22):

1. Sort bricks by minimum z coordinate (lowest first)
2. For each brick, find the highest occupied z below it
3. Drop the brick to rest on that surface
4. Build a support graph (which bricks support which)

```rust
// Sort by z (lowest first — they land first)
bricks.sort_by_key(|b| b.start.z.min(b.end.z));

// For each brick, find highest z occupied below its footprint
for brick in &mut bricks {
    let footprint: Vec<(i32, i32)> = brick.xy_cells();
    let max_z_below = footprint.iter()
        .map(|&(x, y)| height_map[x][y])
        .max().unwrap();
    let drop = brick.min_z() - max_z_below - 1;
    brick.start.z -= drop;
    brick.end.z -= drop;
}
```

---

## 🧮 Data Structure Choices for 3D

| Approach | Lookup | Memory | Best For |
|----------|--------|--------|----------|
| `HashSet<(i32, i32, i32)>` | O(1) avg | O(n) | Sparse (few cubes in large space) |
| `Vec<Vec<Vec<bool>>>` | O(1) exact | O(V) | Dense (most cells occupied), small bounds |
| `Vec<Vec<Vec<T>>>` | O(1) exact | O(V) | Dense with per-cell data |
| Height map `Vec<Vec<i32>>` | O(1) | O(xy) | Gravity problems (only care about max z per column) |

**AoC 2022 Day 18**: 2,043 cubes in ~22³ space → 47% fill rate. HashSet works fine; a dense grid would also be reasonable.

**When to prefer HashSet**: Coordinates might be negative, space is large and sparse, or you don't want to offset all coordinates to be non-negative.

**When to prefer dense grid**: Small bounding box, need to check many cells rapidly (no hashing overhead), or implementing cellular automata (every cell checked every step).

---

## 🔬 AoC Applications

### AoC 2022 Day 18: Boiling Boulders
- **Part 1**: Surface area via neighbor counting (6 × n - 2 × adjacent_pairs)
- **Part 2**: Exterior flood fill — BFS from outside padded bounding box
- **Key pattern**: Flood fill to classify interior vs exterior empty space

### AoC 2023 Day 22: Sand Slabs
- **3D falling bricks**: Sort by z, drop to rest, build support graph
- **Height map optimization**: Track max z per (x,y) column instead of full 3D grid
- **Support relationships**: BFS to count chain reactions when removing bricks

### AoC 2020 Day 17: Conway Cubes
- **3D/4D cellular automata**: Infinite grid, active/inactive cells
- **Neighbor counting**: 26 neighbors per cell (all adjacent including diagonals)
- **Optimization**: Only check cells adjacent to active cells (sparse representation)

### AoC 2021 Day 19: Beacon Scanner
- **3D rotation alignment**: Match overlapping beacons between scanners
- **24 rotation matrices**: All axis-aligned rotations in 3D
- **Point matching**: Find 12+ common beacons to confirm scanner overlap

### AoC 2023 Day 24: Hailstone Trajectories
- **3D parametric lines**: Position + velocity vectors in 3D
- **Intersection**: Solve linear systems for trajectory crossings
- **See**: [[parametric-equations]], [[linear-algebra-fundamentals]]

### AoC 2018 Day 23: Nanobot Teleportation
- **3D Manhattan distance**: Nanobot signal radius as Manhattan sphere (octahedron)
- **Optimization**: Find point in range of most nanobots
- **Advanced**: Coordinate compression or octree search

---

## 🔗 2D → 3D Extension Patterns

| 2D Concept | 3D Extension | Change |
|-----------|-------------|--------|
| 4 cardinal neighbors | 6 face neighbors | +2 (±z) |
| 8 all neighbors | 26 all neighbors | 3³-1 vs 2³-1 |
| Area = w × h | Volume = w × h × d | One more dimension |
| Perimeter | Surface area | Faces instead of edges |
| Flood fill (4-connected) | Flood fill (6-connected) | Same algorithm, more neighbors |
| Grid `Vec<Vec<T>>` | Grid `Vec<Vec<Vec<T>>>` | One more nesting level |
| Line intersection | Plane intersection | Cross product for normals |
| Rotation: 4 states (0°/90°/180°/270°) | Rotation: 24 states | All axis-aligned 3D rotations |

**Key insight**: Most 2D algorithms extend naturally to 3D by changing neighbor constants and adding a dimension to data structures. The algorithmic structure (BFS, flood fill, simulation) stays the same.

---

## 📊 Complexity Considerations

**Volume grows cubically**: A bounding box of side length L has L³ cells. Algorithms that visit every cell become expensive fast.

| Grid Size | 2D Cells | 3D Cells | Ratio |
|-----------|----------|----------|-------|
| 10 | 100 | 1,000 | 10× |
| 100 | 10,000 | 1,000,000 | 100× |
| 1,000 | 1,000,000 | 1,000,000,000 | 1000× |

This is why sparse representations (HashSet) are often preferred in 3D — you only pay for occupied cells, not the entire volume.

**AoC Day 18 example**: 2,043 cubes in a ~22³ bounding box. HashSet stores 2,043 entries; dense grid would allocate 10,648 cells. Both are fine. But if the bounding box were 1000³, HashSet would still be 2,043 entries while a dense grid would need 1 billion cells.

---

## 🔧 Implementation Tips

### Tuple vs Struct

```rust
// Tuple — quick and simple
let cubes: HashSet<(i32, i32, i32)> = HashSet::new();

// Struct — self-documenting, type-safe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3D { x: i32, y: i32, z: i32 }
```

For AoC, tuples are usually fine. For larger projects or when mixing 2D and 3D coordinates, a struct prevents accidental `(x, y)` vs `(x, y, z)` confusion.

### Extending to 4D

AoC 2020 Day 17 Part 2 extends Conway Cubes to 4D. The pattern scales:
- 4D neighbors: 3⁴ - 1 = **80** neighbors per cell
- Same sparse HashSet approach works
- Same BFS/cellular automata algorithm

```rust
const NEIGHBORS_4D: [(i32, i32, i32, i32); 80] = /* ... */;
// Or generate dynamically
```

---

*Tags: #3d-geometry #computational-geometry #bfs #flood-fill #voxel #math-foundations*
*Links: [[computational-geometry-basics]] | [[cross-products-vector-algebra]] | [[linear-algebra-fundamentals]] | [[bitmask-representation]] | [[set-theory-fundamentals]] | [[parametric-equations]]*
