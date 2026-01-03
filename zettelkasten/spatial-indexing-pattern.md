# Spatial Indexing Pattern

**Summary:** Optimization technique for grid-based problems that reverses the search direction by building a coordinate-to-entity lookup table, transforming expensive O(n×m) iterations into O(1) lookups.

*Tags: #optimization #spatial-indexing #grid #hashmap #aoc #performance #data-structures*

---

## Core Concept

When working with grid-based problems that require repeated queries like "which entity is at coordinate (x, y)?", the naive approach of iterating all entities for each query becomes prohibitively expensive. **Spatial indexing** inverts this relationship by pre-building a `HashMap<Coord, EntityId>` that enables constant-time lookups.

### Mental Model

**Before (Expensive):**
```
For each point of interest:
    For each entity:
        Calculate entity's occupied coordinates
        Check if point is in those coordinates
        → O(points × entities × coords_per_entity)
```

**After (Optimized):**
```
Once: Build coordinate → entity index
    → O(entities × coords_per_entity)

For each point of interest:
    Look up entity at coordinate in HashMap
        → O(1) per lookup
        
Total: O(entities × coords + points)
```

---

## Real-World Example: AoC 2023 Day 3

**Problem:** Find gears (★ symbols) adjacent to exactly two numbers in a 140×140 grid.

### Naive Implementation

```rust
// For each gear, iterate ALL numbers and check adjacency
for y in 0..grid.height() {
    for x in 0..grid.width() {
        if grid[(x, y)] == '*' {
            let adjacent_numbers: Vec<&GridNumber> = numbers
                .iter()
                .filter(|num| {
                    // EXPENSIVE: Recalculates Vec of adjacent coords every time!
                    num.adjacent_coords(&grid).contains(&Coord::new(x, y))
                })
                .collect();
            
            if adjacent_numbers.len() == 2 {
                // Process gear
            }
        }
    }
}
```

**Complexity:** O(gears × numbers × adjacency_calc)
- ~100 gears × ~1000 numbers = 100,000+ adjacency calculations
- Each adjacency calc allocates a Vec and checks 8 neighbors
- **Runtime:** 88.92ms

### Optimized Implementation with Spatial Index

```rust
// Step 1: Build spatial index (one-time cost)
let mut coord_to_number: HashMap<Coord, usize> = HashMap::new();
for (idx, num) in numbers.iter().enumerate() {
    for coord in num.digit_coords() {
        coord_to_number.insert(coord, idx);
    }
}
// Cost: ~1000 numbers × ~3 digits = ~3000 insertions

// Step 2: Query with O(1) lookups
for y in 0..grid.height() {
    for x in 0..grid.width() {
        let gear_coord = Coord::new(x, y);
        if grid[gear_coord] == '*' {
            let mut adjacent_number_indices = HashSet::new();
            
            for neighbor in gear_coord.neighbors_8() {
                if grid.in_bounds(neighbor) {
                    if let Some(&num_idx) = coord_to_number.get(&neighbor) {
                        adjacent_number_indices.insert(num_idx);
                    }
                }
            }
            
            if adjacent_number_indices.len() == 2 {
                // Process gear
            }
        }
    }
}
```

**Complexity:** O(numbers × digits + gears × 8)
- Build index: ~3000 insertions
- Query: ~100 gears × 8 neighbors = 800 lookups
- **Runtime:** 360.25µs

**Measured Speedup:** **246.8x faster** (empirically validated with 100 iterations)

---

## When to Apply

✅ **Good fit:**
- Grid problems with repeated "what's at this coordinate?" queries
- Number of queries >> number of entities
- Entities occupy multiple coordinates
- Memory available for O(total_coordinates) storage

❌ **Not suitable:**
- Single query (overhead not worth it)
- Sparse grids with few queries
- Entities frequently move (rebuilding index is expensive)
- Memory-constrained environments

### Break-Even Analysis

The pattern is worth it when:
```
queries × entities > entities × coords_per_entity
```

For Day 3:
- Queries: 100 gears × 8 neighbors = 800
- Without index: 800 × 1000 = 800,000 operations
- With index: 1000 × 3 + 800 = 3,800 operations
- **Ratio:** ~210x fewer operations (close to measured 246.8x)

---

## Implementation Pattern

### Generic Template

```rust
use std::collections::HashMap;

// 1. Define coordinate and entity types
type Coord = (usize, usize);  // Or custom Coord struct
type EntityId = usize;

// 2. Build spatial index
fn build_spatial_index<T>(entities: &[Entity<T>]) -> HashMap<Coord, EntityId> {
    let mut index = HashMap::new();
    
    for (id, entity) in entities.iter().enumerate() {
        for coord in entity.occupied_coords() {
            index.insert(coord, id);
        }
    }
    
    index
}

// 3. Query efficiently
fn find_entities_near(point: Coord, index: &HashMap<Coord, EntityId>) -> Vec<EntityId> {
    let neighbors = get_neighbors_8(point);  // Or grid.neighbors_8()
    
    neighbors
        .into_iter()
        .filter_map(|coord| index.get(&coord).copied())
        .collect()
}
```

### Key Decisions

**What to store in the index?**
- **Entity ID:** When you need to look up the full entity afterward
- **Entity reference:** If you need immediate access (lifetime constraints)
- **Derived data:** If you only need specific properties

**Handling overlaps:**
- If multiple entities can occupy same coordinate, use `HashMap<Coord, Vec<EntityId>>`
- If only one entity per coordinate, use `HashMap<Coord, EntityId>`

**Coordinate representation:**
- Use Mission 6 `Coord` struct for hashability and neighbor iteration
- Ensure `Hash` and `Eq` traits are implemented

---

## Trade-offs

### Advantages
- ✅ Transforms O(n) iteration → O(1) lookup
- ✅ Predictable performance regardless of entity count
- ✅ Enables complex spatial queries (range searches, nearest neighbors)
- ✅ Reusable across multiple query passes

### Disadvantages
- ❌ Memory overhead: O(total_occupied_coordinates)
- ❌ One-time build cost: O(entities × coords_per_entity)
- ❌ Mutation complexity: Index must be updated when entities move
- ❌ Not cache-friendly for sparse grids

### Performance Characteristics

| Metric | Without Index | With Index |
|--------|---------------|------------|
| Build time | 0 | O(entities × coords) |
| Query time | O(entities) | O(1) |
| Memory | O(entities) | O(total_coords) |
| Update time | 0 | O(coords_per_entity) |

---

## Extensions and Variations

### 1. Multi-Entity Index

For coordinates with multiple entities:

```rust
let mut index: HashMap<Coord, Vec<EntityId>> = HashMap::new();

for (id, entity) in entities.iter().enumerate() {
    for coord in entity.occupied_coords() {
        index.entry(coord).or_insert_with(Vec::new).push(id);
    }
}
```

### 2. Spatial Hash with Metadata

Store additional data per coordinate:

```rust
struct SpatialData {
    entity_id: usize,
    entity_type: EntityType,
    priority: u8,
}

let index: HashMap<Coord, SpatialData> = /* build */;
```

### 3. Quadtree/R-tree for Large Grids

For very large or infinite grids, consider hierarchical spatial data structures:
- Quadtree: Recursive subdivision of 2D space
- R-tree: Bounding box hierarchy for range queries
- Grid partitioning: Divide into chunks with separate indexes

### 4. Incremental Updates

For dynamic entities:

```rust
fn move_entity(
    entity_id: EntityId,
    old_coords: &[Coord],
    new_coords: &[Coord],
    index: &mut HashMap<Coord, EntityId>
) {
    // Remove old positions
    for coord in old_coords {
        index.remove(coord);
    }
    
    // Add new positions
    for coord in new_coords {
        index.insert(*coord, entity_id);
    }
}
```

---

## Related Patterns

### Spatial Hash vs. Other Approaches

| Approach | Query Time | Build Time | Use Case |
|----------|------------|------------|----------|
| **Spatial Hash** | O(1) | O(n×k) | Exact coordinate lookups |
| **Sweep Line** | O(n log n) | O(1) | Interval/range queries |
| **BFS/DFS** | O(V+E) | O(1) | Connectivity/pathfinding |
| **Quadtree** | O(log n) | O(n log n) | Large/sparse grids |

### Integration with Mission 6

Mission 6's `Grid<T>` provides the foundation:
- `Coord` struct with `neighbors_8()` iterator
- Bounds checking with `in_bounds()`
- Indexing with `grid[coord]`

Spatial indexing complements Grid by adding entity-to-coordinate reverse lookup.

---

## Empirical Results

**AoC 2023 Day 3 Benchmark** (100 iterations, 140×140 grid):
- **Old implementation:** 88.92ms
- **Spatial index:** 360.25µs
- **Speedup:** 246.8x faster
- **Validation:** Both produce identical result (87263515)

### Why 246.8x Instead of Theoretical 210x?

Additional speedup factors:
1. **Allocation overhead:** Old version allocates Vec for every adjacency check
2. **Cache misses:** Iterating all numbers repeatedly thrashes CPU cache
3. **Redundant work:** Same coordinates calculated multiple times
4. **HashMap efficiency:** Modern HashMap is highly optimized with good hash distribution

---

## Implementation Checklist

When implementing spatial indexing:

- [ ] Profile to confirm query bottleneck (not build cost)
- [ ] Calculate memory requirements (total coords × 24 bytes for HashMap entry)
- [ ] Choose appropriate Coord representation (tuple vs struct)
- [ ] Implement proper Hash/Eq for custom Coord types
- [ ] Handle edge cases (out of bounds, empty cells)
- [ ] Decide on single vs multi-entity per coordinate
- [ ] Benchmark before/after to validate speedup
- [ ] Document complexity assumptions in code comments

---

## Code References

**Implementation:** `advent_of_code/aoc2023/src/solver/day03.rs`
- Function: `solve_part2()` (optimized version)
- Spatial index build: Lines ~80-85
- Query usage: Lines ~90-105

**Benchmark:** `bench_day03/src/main.rs`
- Compares old vs new implementations
- Run: `cargo run --release -p bench_day03`

**Mission 6:** `missions/Mission6/src/lib.rs`
- `Grid<T>` implementation
- `Coord` struct with `neighbors_8()`

---

## Further Reading

- [[mission-6]] - Grid and Coord foundations
- [[hashmap-optimization]] - HashMap performance characteristics
- [[bfs-dfs-traversal]] - Alternative graph traversal approaches
- [[aoc2023-day3]] - Full problem context and solution
- [[optimization-patterns]] - General optimization techniques

---

## Backlinks

**Referenced by:**
- [[aoc2023-day3]] - Spatial indexing enables 246.8x speedup
- [[optimization-case-studies]] - Real-world optimization example
- [[mission-6]] - Practical application of Grid component

**References:**
- [[mission-6]] - `Grid<T>` and Coord infrastructure
- [[hashmap-performance]] - HashMap as spatial index
- [[big-o-notation]] - Complexity analysis framework

---

*Created: 2026-01-03*  
*Last Updated: 2026-01-03*  
*Status: Complete - Validated with empirical benchmark*

*Links: [[optimization-patterns]] | [[aoc2023]] | [[mission-6]] | [[hashmap-optimization]] | [[grid-algorithms]]*
