# Day 18: Boiling Boulders - Function Guide

**Problem**: Calculate the surface area of a 3D lava droplet made of 1x1x1 cubes.

**Navigation**: [← Day 17](day17_function_guide.md) | [Problem](day18.md) | [Code](../../src/solver/day18.rs) | [Summary](../summary_2022.md) | [Day 19 →](day19_function_guide.md)

---

## Overview

### Problem Summary
- **Part 1**: Total surface area — count faces not adjacent to another cube
- **Part 2**: Exterior surface area only — exclude faces touching trapped air pockets

### Performance
- **Parse**: 67.5µs
- **Part 1**: 185µs (HashSet neighbor lookup)
- **Part 2**: 1.10ms (BFS flood fill from outside)
- **Combined**: 1.38ms ✅ Parse-once verified (1.38ms < 67.5µs + 185µs + 1100µs)

### Key Insight
**Part 2 is a flood fill, not a subtraction.** Instead of finding air pockets and subtracting their faces, flood from outside the bounding box and count every lava face the water touches. Internal pockets are never reached.

---

## Algorithm Analysis

### Part 1: Neighbor Counting

Each cube has 6 faces. A face is exposed if no adjacent cube exists in that direction.

```rust
const NEIGHBORS: [(i32, i32, i32); 6] = [
    (1,0,0), (-1,0,0),  // ±x
    (0,1,0), (0,-1,0),  // ±y
    (0,0,1), (0,0,-1),  // ±z
];

for &(x, y, z) in cubes {
    for (dx, dy, dz) in NEIGHBORS {
        if !set.contains(&(x + dx, y + dy, z + dz)) {
            surface += 1;  // This face is exposed
        }
    }
}
```

**Complexity**: O(n) — 6 HashSet lookups per cube, each O(1) average.

**Why not count shared faces?** Equivalent approach: start with 6n faces, subtract 2 for each adjacent pair (both cubes lose one face). The neighbor-counting approach is simpler — just check and count.

### Part 2: Exterior Flood Fill (BFS)

The key insight: instead of finding internal voids, fill from outside and count what the water touches.

```
Step 1: Compute bounding box with 1 unit of padding
        (ensures water can flow around all surfaces)

Step 2: BFS from corner (min_x-1, min_y-1, min_z-1)
        - Guaranteed to be outside the droplet

Step 3: For each BFS position, check 6 neighbors:
        - If neighbor is LAVA → count that face (exterior surface)
        - If neighbor is EMPTY and unvisited → add to BFS queue
        - If neighbor is out of bounds → skip

Step 4: Total count = exterior surface area
```

```
2D cross-section visualization:

  . . . . . . .     W W W W W W W
  . . # # # . .     W W # # # W W
  . # . . . # .     W # . . . # W    ← internal air (.) never reached
  . # . . . # .     W # . . . # W       by water (W)
  . . # # # . .     W W # # # W W
  . . . . . . .     W W W W W W W

  Before flood          After flood
```

**Why padding?** Without 1 unit of extra space, the BFS can't flow around corners of the droplet. The padding ensures complete exterior coverage.

**Complexity**: O(V) where V = bounding box volume. For this input: ~22 × 22 × 22 ≈ 10,648 cells. Each cell visited at most once.

---

## Implementation Details

### Data Flow

```
Input: "2,2,2\n1,2,2\n..."
  ↓ parse_input()
Vec<(i32, i32, i32)>     ← 2,043 cubes
  ↓ solve_part1_with_data()
HashSet → 6 neighbor checks per cube → count exposed faces
  ↓ solve_part2_with_data()
Bounding box → BFS flood → count lava faces hit by water
```

### Parse-Once Pattern

Both parts build their own HashSet from the shared `Vec<(i32, i32, i32)>`. The parsing (67.5µs) is done once, while HashSet construction (~100µs) happens twice. A further optimization could share the HashSet, but at 1.38ms total it's not worth the added complexity.

### BFS Implementation

```rust
while let Some((x, y, z)) = queue.pop_front() {
    for (dx, dy, dz) in NEIGHBORS {
        let (nx, ny, nz) = (x + dx, y + dy, z + dz);

        // Stay within padded bounding box
        if out_of_bounds(nx, ny, nz) { continue; }

        if set.contains(&(nx, ny, nz)) {
            exterior_surface += 1;  // Hit a lava face
        } else if visited.insert((nx, ny, nz)) {
            queue.push_back((nx, ny, nz));  // Continue flooding
        }
    }
}
```

Note: `visited.insert()` returns `true` if the value was newly inserted — this serves as both the "not yet visited" check and the "mark visited" step in one call.

---

## Performance Analysis

### Benchmark Results

```
day18_parse:     67.5µs   ← Split lines, parse 3 integers each
day18_part1:     185µs    ← Build HashSet + 6 lookups × 2,043 cubes
day18_part2:     1.10ms   ← Build HashSet + BFS flood fill
day18_combined:  1.38ms   ← Parse once + both parts
```

### Where Time Is Spent

| Component | Estimated Time | Notes |
|-----------|---------------|-------|
| Parsing | 67.5µs | 2,043 lines × 3 integers |
| HashSet construction (×2) | ~200µs | 2,043 inserts, once per part |
| Part 1 lookups | ~85µs | 12,258 HashSet lookups (2,043 × 6) |
| Part 2 BFS | ~900µs | ~10K cells, 6 neighbors each, HashSet lookups |

Part 2 dominates because the BFS visits every empty cell in the bounding box (~8K empty cells + 2K cubes), each doing 6 HashSet lookups.

### Potential Optimizations (Not Implemented)

- **3D boolean grid instead of HashSet**: `Vec<Vec<Vec<bool>>>` with direct indexing — O(1) with no hashing. Would reduce Part 2 from ~1.1ms to ~200µs.
- **Share HashSet between parts**: Build once, pass to both. Saves ~100µs.
- **Combined BFS + counting**: Do Part 1 counting during the Part 2 flood — get both answers in one pass.

These aren't worth implementing at 1.38ms, but illustrate the trade-off between HashSet generality and array-based performance.

### Complexity Summary

| Component | Complexity | Notes |
|-----------|-----------|-------|
| Parsing | O(n) | n = 2,043 cubes |
| Part 1 | O(n) | 6 HashSet lookups per cube |
| Part 2 | O(V) | V = bounding box volume ≈ 10K |
| Space | O(n + V) | HashSet + BFS visited set |

---

## Edge Cases

### Trapped Air Pockets
The example has a single trapped air pocket at (2,2,5), which has 6 faces all touching lava. Part 1 counts these as exposed (6 × adjacent cubes see empty space), but Part 2 correctly excludes them (water never reaches this cell).

Part 1 − Part 2 = 3,522 − 2,074 = **1,448 interior faces** (faces touching trapped air).

### Diagonal Adjacency
Only face-adjacent neighbors count (6 directions). Diagonally adjacent cubes do NOT share a face — they have a visible edge but no hidden face between them.

### Single Cube
One cube has 6 exposed faces for both parts (no neighbors, no air pockets).

---

## Key Takeaways

1. **Flood fill from outside is simpler than void detection**: Finding all internal air pockets requires identifying connected components of empty space and testing which are enclosed. BFS from outside skips all of that — just flood and count.

2. **Bounding box padding enables complete coverage**: Adding 1 unit of padding around the bounding box ensures the BFS can flow around all exterior surfaces, even at the extremes.

3. **HashSet for 3D point lookup**: `HashSet<(i32, i32, i32)>` is the natural choice for sparse 3D point sets. O(1) average lookup, no need to allocate a full 3D grid.

4. **`visited.insert()` double-duty**: Rust's `HashSet::insert` returns whether the element was new — combining the "check" and "mark" steps into one operation. Clean and efficient.

5. **Interior vs exterior is a connectivity question**: A face is interior if the empty space it touches can't reach the outside. This is exactly what BFS determines — reachability from an external starting point.

---

**Answer**: Part 1: `3522` | Part 2: `2074`

**Interior faces**: 3,522 − 2,074 = 1,448 faces belong to trapped air pockets inside the droplet.

**Related patterns**: [[bfs-patterns]], [[flood-fill]], [[3d-geometry]], [[hashset-lookup]]
