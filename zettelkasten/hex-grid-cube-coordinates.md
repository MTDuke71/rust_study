# Hex Grid Cube Coordinates - The Right Way to Think About Hex Grids

*Cube coordinates (q, r, s) with the constraint q + r + s = 0 make hex grid math trivial — distance, neighbors, rotation, and reflection all become simple formulas.*

---

## 🎯 **Core Concept**

Hex grids are tricky because squares have obvious (x, y) coordinates but hexagons don't tile that way. The breakthrough is **cube coordinates**: treat each hex as a point in 3D space where q + r + s = 0, which constrains all points to a single plane.

This turns every hex operation into simple arithmetic — no conditionals, no even/odd row special cases.

## 🧠 **Mental Models**

### **Why Three Axes?**

Think of slicing a 3D cube along the plane where x + y + z = 0. The cross-section is a hex grid:

```
        +s
        |
   nw   |   ne
     \  |  /
      \ | /
  -q ---+--- +q
      / | \
     /  |  \
   sw   |   se
        |
       -s
```

Each hex has three coordinates that always sum to zero. Moving in any direction changes exactly two coordinates (one up, one down).

### **AUTOSAR Analogy**

Think of cube coordinates like a three-axis sensor fusion system. Any two axes give you full information (the third is redundant via q + r + s = 0), but keeping all three makes every calculation symmetric and branchless — like having redundant sensors that simplify the fusion algorithm.

## 🔍 **Detailed Content**

### **The Six Directions**

| Direction | dq | dr | ds | Visual |
|-----------|----|----|-----|--------|
| **n**     | 0  | -1 | +1  | straight up |
| **ne**    | +1 | -1 | 0   | upper-right |
| **se**    | +1 | 0  | -1  | lower-right |
| **s**     | 0  | +1 | -1  | straight down |
| **sw**    | -1 | +1 | 0   | lower-left |
| **nw**    | -1 | 0  | +1  | upper-left |

Every offset sums to 0, preserving the invariant.

**Opposite directions**: n/s, ne/sw, se/nw — each pair sums to (0, 0, 0), which is why they cancel perfectly.

### **Distance Formula**

```rust
fn hex_distance(q: i32, r: i32, s: i32) -> i32 {
    q.abs().max(r.abs()).max(s.abs())
}
```

**Why this works**: Each step changes the largest-magnitude coordinate by 1 (moving diagonally reduces two coordinates simultaneously). So the minimum number of steps equals the largest absolute coordinate.

**Equivalent formula**: `(|q| + |r| + |s|) / 2` — since q + r + s = 0, the sum of absolutes is always even.

### **Common Operations**

#### Neighbors
```rust
const HEX_DIRS: [(i32, i32, i32); 6] = [
    (0, -1, 1),  // n
    (1, -1, 0),  // ne
    (1, 0, -1),  // se
    (0, 1, -1),  // s
    (-1, 1, 0),  // sw
    (-1, 0, 1),  // nw
];
```

#### Rotation (60 degrees clockwise)
```rust
fn rotate_cw(q: i32, r: i32, s: i32) -> (i32, i32, i32) {
    (-r, -s, -q)
}
```

Rotate 6 times to get back to start. Each rotation cycles and negates: (q,r,s) → (-r,-s,-q) → (s,q,r) → (-q,-r,-s) → (r,s,q) → (-s,-q,-r) → (q,r,s).

#### Reflection (across q axis)
```rust
fn reflect_q(q: i32, r: i32, s: i32) -> (i32, i32, i32) {
    (q, s, r)  // swap r and s
}
```

#### Ring at distance N
```rust
fn hex_ring(center: (i32, i32, i32), radius: i32) -> Vec<(i32, i32, i32)> {
    // Start at center + radius * direction[4] (sw), walk each edge
    // Each edge has `radius` hexes, 6 edges total = 6*radius hexes
    todo!()
}
```

### **Coordinate System Comparison**

| System | Distance Formula | Neighbors | Even/Odd Cases | Storage |
|--------|-----------------|-----------|----------------|---------|
| **Offset (row, col)** | Ugly conditional formula | Depends on row parity | Yes, everywhere | 2 values |
| **Axial (q, r)** | `max(|q|, |r|, |q+r|)` | Simple | None | 2 values |
| **Cube (q, r, s)** | `max(|q|, |r|, |s|)` | Simplest | None | 3 values (1 redundant) |

**Recommendation**: Use cube coordinates for algorithms (clean math), convert to axial for storage (drop s, compute as -q-r when needed).

### **Flat-Top vs Pointy-Top**

The direction table above is for **pointy-top** hexagons (north = straight up). Flat-top hexagons rotate everything 30 degrees — the directions become E, NE, NW, W, SW, SE instead of N, NE, SE, S, SW, NW. The cube coordinate math is identical; only the direction-to-offset mapping changes.

AoC 2017 Day 11 uses pointy-top (n/ne/se/s/sw/nw).

## 🔬 **AoC Implementation (Day 11)**

```rust
fn solve_both(input: &str) -> (i32, i32) {
    let mut q = 0i32;
    let mut r = 0i32;
    let mut s = 0i32;
    let mut max_dist = 0;

    for step in input.trim().split(',') {
        match step {
            "n"  => { r -= 1; s += 1; }
            "ne" => { q += 1; r -= 1; }
            "se" => { q += 1; s -= 1; }
            "s"  => { r += 1; s -= 1; }
            "sw" => { q -= 1; r += 1; }
            "nw" => { q -= 1; s += 1; }
            _ => unreachable!(),
        }
        max_dist = max_dist.max(hex_distance(q, r, s));
    }

    (hex_distance(q, r, s), max_dist)
}
```

No HashMap, no grid storage, no pathfinding — just accumulate offsets and compute distance. O(n) time, O(1) space.

## 💡 **Key Takeaways**

- **Always use cube coordinates** for hex grid problems — every other system leads to ugly conditional math
- **Distance = max(|q|, |r|, |s|)** — this one formula replaces complex pathfinding
- **Cancellation is free** — opposite directions sum to (0,0,0), handled implicitly by accumulation
- **Rotation = negate and cycle** — (q,r,s) → (-r,-s,-q) for 60-degree CW rotation
- **The Red Blob Games hex guide** is the definitive resource: redblobgames.com/grids/hexagons/

## 🔗 **Integration Points**

### **Builds On**
- [[Coordinate Systems]] - General coordinate system concepts
- [[Manhattan Distance]] - Analogous distance metric for square grids

### **Enables**
- [[Grid Data Structures]] - Hex grid storage strategies
- Future AoC hex grid problems (they recur every few years)

### **Related Concepts**
- [[mission-6]] - Grid<T> for square grids; hex grids need different storage
- [[BFS Pathfinding]] - When hex distance isn't enough (obstacles, weighted edges)
- [[Chebyshev Distance]] - The square grid equivalent of hex distance (8-connected)

---

*Tags: #coordinate-systems #hex-grid #aoc #algorithms #geometry #cube-coordinates*

*Links: [[zettel-index]] | [[AoC Patterns MOC]] | [[Coordinate Systems]] | [[Manhattan Distance]] | [[Chebyshev Distance]] | [[aoc-2017-day11]]*
