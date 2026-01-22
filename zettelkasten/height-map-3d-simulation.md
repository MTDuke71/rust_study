# Height Map 3D Simulation Pattern

**Summary:** Reduce 3D spatial problems to 2D by tracking only the top surface (height map), enabling efficient gravity simulation and collision detection without storing full 3D occupancy grids.

*Tags: #3d-simulation #height-map #spatial-algorithms #optimization #aoc #hashmap #data-structures #physics*

---

## Core Concept

When simulating 3D objects falling under gravity, you don't need a full 3D occupancy grid. Instead, maintain a **height map** that stores only the maximum z-coordinate at each (x, y) position. This projection technique transforms O(x × y × z) space complexity into O(occupied_columns) with sparse storage.

### Mental Model

**Dense 3D Grid (Expensive):**
```rust
// Stores EVERY voxel, even empty space
let grid: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; z_max]; y_max]; x_max];
// Space: O(x_max × y_max × z_max)
// Lookup: O(1) but wastes memory
```

**Height Map (Efficient):**
```rust
// Only stores TOP SURFACE of occupied columns
let height_map: HashMap<(i32, i32), (i32, EntityId)> = HashMap::new();
//                      (x, y)        (max_z, who)
// Space: O(occupied_columns) - sparse, only what's used
// Lookup: O(1) average for "what's below me?"
```

**Key Insight:** For gravity simulation, only the **top surface** matters. Objects fall until they hit the highest obstacle below them. Internal structure doesn't affect falling behavior.

---

## Pattern Structure

### 1. Height Map Representation

```rust
// Key: (x, y) horizontal position
// Value: (max_z at this column, entity_id occupying that z)
type HeightMap = HashMap<(i32, i32), (i32, usize)>;

// Alternative for fixed grids:
type HeightMapArray = Vec<Vec<Option<(i32, usize)>>>;  // Dense if grid known
```

**Design Choices:**

| **Structure** | **Use When** | **Performance** |
|---------------|--------------|-----------------|
| `HashMap<(x,y), (z, id)>` | Sparse, unbounded coordinates | O(1) avg, memory efficient |
| `Vec<Vec<Option<_>>>` | Dense, fixed size grid known | O(1) guaranteed, cache friendly |
| `BTreeMap<(x,y), _>` | Need sorted access by position | O(log n), ordered iteration |

### 2. Falling Simulation Algorithm

```rust
fn simulate_falling(objects: &mut [Object], height_map: &mut HeightMap) {
    // CRITICAL: Sort by starting z (lowest first)
    // Higher objects can't fall until lower ones settle
    objects.sort_by_key(|obj| obj.min_z());
    
    for object in objects.iter_mut() {
        // Step 1: Find highest obstacle below object
        let mut max_z_below = 0;  // Ground level
        
        for (x, y) in object.get_footprint() {
            if let Some(&(z, _)) = height_map.get(&(x, y)) {
                max_z_below = max_z_below.max(z);
            }
        }
        
        // Step 2: Calculate fall distance
        // Object rests ON TOP of obstacle, not inside it
        let new_min_z = max_z_below + 1;
        let fall_distance = object.min_z() - new_min_z;
        
        // Step 3: Drop object
        object.move_down(fall_distance);
        
        // Step 4: Update height map with object's new top surface
        let new_max_z = object.max_z();
        for (x, y) in object.get_footprint() {
            height_map.insert((x, y), (new_max_z, object.id));
        }
    }
}
```

**Algorithm Complexity:**
- Sort: O(n log n) where n = number of objects
- Per object: O(f) where f = footprint size
- Total: **O(n log n + n × f)** = O(n log n) typically (f is constant)

**Why This Works:**
1. **Lowest-first processing** ensures objects below are already settled
2. **Height map** represents settled state (monotonically increasing)
3. **Footprint lookup** checks all (x, y) positions object occupies
4. **Update after settling** maintains invariant for next object

### 3. Support Detection (What's Touching?)

```rust
fn find_supports(
    objects: &[Object],
    height_map: &HeightMap
) -> Vec<HashSet<usize>> {
    // Build spatial index for "what's at (x, y, z)?"
    let mut space: HashMap<(i32, i32, i32), usize> = HashMap::new();
    for obj in objects {
        for (x, y, z) in obj.get_all_cubes() {
            space.insert((x, y, z), obj.id);
        }
    }
    
    let mut supports = vec![HashSet::new(); objects.len()];
    
    for obj in objects {
        let top_z = obj.max_z();
        
        // Check positions ONE LEVEL ABOVE top surface
        for (x, y, z) in obj.get_all_cubes() {
            if z == top_z {  // Only check top surface
                if let Some(&above_id) = space.get(&(x, y, z + 1)) {
                    if above_id != obj.id {
                        supports[obj.id].insert(above_id);
                    }
                }
            }
        }
    }
    
    supports
}
```

**Why Check z+1?**
- Objects are touching if one's top surface (max_z) is adjacent to another's bottom (min_z)
- Adjacent in z means `top_z + 1 == bottom_z`
- Only need to check top surface cubes (internal cubes don't support anything)

---

## Real-World Example: AoC 2023 Day 22

**Problem:** 3D brick falling simulation ("Jenga tower")

**Bricks:**
```
Brick A: (1,0,10) to (1,2,10)  - horizontal line, z=10
Brick B: (0,0,12) to (2,0,12)  - horizontal line, z=12
Brick C: (1,1,15) to (1,1,16)  - vertical column, z=15-16
```

### Step-by-Step Simulation

**Initial State:**
```
Height Map: {} (empty - no settled bricks)
```

**Process Brick A (min_z=10):**
```rust
// Check footprint: (1,0), (1,1), (1,2)
// All return None → max_z_below = 0 (ground)
// Fall distance: 10 - (0 + 1) = 9
// New position: z=1
// Update height map:
height_map = {
    (1, 0) → (1, brick_A),
    (1, 1) → (1, brick_A),
    (1, 2) → (1, brick_A),
}
```

**Process Brick B (min_z=12):**
```rust
// Check footprint: (0,0), (1,0), (2,0)
// (0,0) → None (0)
// (1,0) → Some((1, brick_A)) ✓ Found obstacle!
// (2,0) → None (0)
// max_z_below = 1 (from brick A)
// Fall distance: 12 - (1 + 1) = 10
// New position: z=2
// Update height map:
height_map = {
    (1, 0) → (2, brick_B),  // Overwrites (B is higher)
    (1, 1) → (1, brick_A),
    (1, 2) → (1, brick_A),
    (0, 0) → (2, brick_B),
    (2, 0) → (2, brick_B),
}
```

**Process Brick C (min_z=15):**
```rust
// Check footprint: (1,1)  (single column)
// (1,1) → Some((1, brick_A))
// max_z_below = 1
// Fall distance: 15 - (1 + 1) = 13
// New position: z=2 to z=3 (2-cube tall)
// Update height map:
height_map = {
    (1, 0) → (2, brick_B),
    (1, 1) → (3, brick_C),  // Overwrites (C is higher)
    (1, 2) → (1, brick_A),
    (0, 0) → (2, brick_B),
    (2, 0) → (2, brick_B),
}
```

**Final Settled State:**
```
       [C]      z=3
       [C]      z=2
  [B][B][B]     z=2
  [A][A][A]     z=1
  -----------   z=0 (ground)
  0 1 2 (x)
```

### Performance Comparison

**Full 3D Grid Approach:**
```rust
// Allocate dense 3D array
let grid = vec![vec![vec![None; z_max]; y_max]; x_max];
// Space: 10 × 10 × 300 = 30,000 cells (mostly empty!)
// Update: O(f × z) to scan vertical column
```

**Height Map Approach:**
```rust
// Only store occupied columns
let height_map: HashMap<(i32, i32), (i32, usize)> = HashMap::new();
// Space: ~5-10 entries per brick × 1,360 bricks = ~6,800 entries
// Update: O(f) to check footprint, O(1) per lookup
```

**Actual Day 22 Results:**
- 1,360 bricks
- Average 5 cubes per brick
- Height map: ~400 unique (x, y) positions
- Memory: **8 KB** (height map) vs **30 MB** (dense grid)
- **3,750× memory savings!**

---

## Key Patterns and Variants

### Pattern 1: Multi-Level Tracking

Sometimes you need more than just max height:

```rust
// Track multiple layers (e.g., multiple floors in building)
type LayeredHeightMap = HashMap<(i32, i32), Vec<(i32, usize)>>;

// For each column, store ALL occupied z-levels
height_map.entry((x, y))
    .or_insert_with(Vec::new)
    .push((z, entity_id));

// Sort by z for binary search queries
height_map.get_mut(&(x, y)).unwrap().sort_by_key(|&(z, _)| z);
```

**Use Case:** Need to find "first obstacle below z=50" (not just highest).

### Pattern 2: 2D Projection with Metadata

Store additional info beyond just height:

```rust
#[derive(Clone)]
struct SurfaceInfo {
    height: i32,
    entity_id: usize,
    surface_type: Material,  // e.g., "ice", "rock", "water"
    normal: Vec3,            // Surface normal vector
}

type RichHeightMap = HashMap<(i32, i32), SurfaceInfo>;
```

**Use Case:** Physics simulations where surface properties matter (friction, bounce, etc.).

### Pattern 3: Dynamic Updates (Moving Objects)

```rust
fn move_object(
    obj: &mut Object,
    height_map: &mut HeightMap,
    dx: i32,
    dy: i32
) {
    // Step 1: Remove object from old position
    for (x, y) in obj.get_footprint() {
        height_map.remove(&(x, y));
    }
    
    // Step 2: Move object
    obj.translate(dx, dy, 0);
    
    // Step 3: Find new resting height
    let max_z_below = find_max_height(obj, height_map);
    obj.set_z(max_z_below + 1);
    
    // Step 4: Re-insert at new position
    let new_max_z = obj.max_z();
    for (x, y) in obj.get_footprint() {
        height_map.insert((x, y), (new_max_z, obj.id));
    }
}
```

**Invariant Maintenance:** Always remove before move, recalculate height, then re-insert.

---

## Common Pitfalls

### ❌ Pitfall 1: Processing Out of Order

```rust
// WRONG: Process bricks in arbitrary order
for brick in bricks {
    simulate_fall(brick, &mut height_map);
}
// Result: Higher bricks might fall first, creating impossible states!
```

```rust
// CORRECT: Sort by starting height first
bricks.sort_by_key(|b| b.min_z());
for brick in bricks {
    simulate_fall(brick, &mut height_map);
}
```

**Why:** Object at z=5 must settle before object at z=10 can know where to land.

### ❌ Pitfall 2: Forgetting to Rest "On Top"

```rust
// WRONG: Place object AT the obstacle's height
let new_z = max_z_below;  
brick.set_min_z(new_z);
// Result: Bricks overlap! (both at z=5)
```

```rust
// CORRECT: Place object ON TOP of obstacle
let new_z = max_z_below + 1;
brick.set_min_z(new_z);
// Result: Bricks stack properly (one at z=5, next at z=6)
```

**Think:** Objects rest on surfaces, not inside them.

### ❌ Pitfall 3: Updating Height Map Prematurely

```rust
// WRONG: Update height map BEFORE checking all footprint positions
for (x, y) in brick.get_footprint() {
    let max_z = height_map.get(&(x, y))...;
    height_map.insert((x, y), (brick.max_z(), brick.id));  // ❌
}
// Result: Later footprint checks see THIS brick's height!
```

```rust
// CORRECT: Read all heights FIRST, then update ALL at end
let mut max_z_below = 0;
for (x, y) in brick.get_footprint() {
    if let Some(&(z, _)) = height_map.get(&(x, y)) {
        max_z_below = max_z_below.max(z);
    }
}
// ... calculate fall ...
for (x, y) in brick.get_footprint() {
    height_map.insert((x, y), (brick.max_z(), brick.id));  // ✓
}
```

**Principle:** Query phase separate from update phase.

---

## When to Use This Pattern

### ✅ **Use Height Maps When:**

1. **Gravity/falling simulations** - Objects fall until collision
2. **Sparse 3D spaces** - Most (x, y) columns are empty
3. **Top-down queries** - "What's the highest thing at (x, y)?"
4. **2D projections sufficient** - Don't need full 3D occupancy queries
5. **Dynamic updates rare** - Mostly static after initial settling

### ❌ **Don't Use When:**

1. **Dense 3D volumes** - Most voxels occupied (use 3D array)
2. **Need arbitrary 3D queries** - "What's at (x, y, z=50)?" (use octree)
3. **Multiple objects per column** - Need to track internal layers (use layered variant)
4. **Frequent insertions/removals** - Height map invalidation expensive
5. **Non-axis-aligned objects** - Rotation makes footprint calculation hard

---

## Performance Characteristics

### Time Complexity

| **Operation** | **Height Map** | **Dense 3D Grid** |
|---------------|----------------|-------------------|
| **Initialize** | O(1) | O(x × y × z) |
| **Query height** | O(1) avg | O(z) scan column |
| **Insert object** | O(f) footprint | O(f × h) height |
| **Remove object** | O(f) | O(f × h) |
| **Simulate n objects** | O(n log n + n×f) | O(n × f × z) |

### Space Complexity

| **Structure** | **Worst Case** | **Typical (Sparse)** |
|---------------|----------------|----------------------|
| **Height Map** | O(x × y) | O(occupied) |
| **Dense 3D** | O(x × y × z) | O(x × y × z) |
| **Sparse 3D (HashSet)** | O(cubes) | O(cubes) |

**Day 22 Example:** 8 KB (height map) vs 30 MB (dense) = **3,750× savings**

---

## Mission Integration

### Mission 6: Grid Data Structures

Height maps are **2D grids** with specialized semantics:

```rust
// Mission 6 Grid stores cell contents
struct Grid<T> {
    data: Vec<Vec<T>>,
}

// Height map is Grid<Option<(i32, usize)>> conceptually
// But HashMap is more efficient for sparse data
type SparseHeightMap = HashMap<(i32, i32), (i32, usize)>;
```

**Connection:** Height maps = sparse grid optimization

### Mission 8: Graph Algorithms

Support relationships form a **directed acyclic graph (DAG)**:

```rust
// Nodes: Objects
// Edges: "A supports B" (A is below B, touching)
// Derived from height map + spatial overlap
let supports: Vec<HashSet<usize>> = build_support_graph(objects, height_map);
```

**Connection:** Height map enables efficient graph construction

---

## Related Patterns

- [[spatial-indexing-pattern]] - General coordinate → entity lookups
- [[Graph Theory MOC]] - Support graphs are DAGs
- [[Mission6_tut Overview]] - 2D grid fundamentals
- [[hashmap-ownership-patterns]] - HashMap storage patterns
- [[dependency-propagation-bfs]] - Using support graphs for chain reactions
- [[aoc-optimization-strategies]] - When to use sparse vs dense structures

---

## References

- **AoC 2023 Day 22**: Sand Slabs (3D brick falling)
  - Problem statement: `advent_of_code/aoc2023/Problem_Statements/days/day22.md`
  - Implementation: `advent_of_code/aoc2023/src/solver/day22.rs`
  - Function guide: `advent_of_code/aoc2023/Problem_Statements/days/day22_function_guide.md`
- **Patterns catalog**: `advent_of_code/aoc2023/Problem_Statements/patterns-catalog.md`
- **Algorithms reference**: `advent_of_code/aoc2023/Problem_Statements/algorithms-reference.md`

---

*Last Updated: 2026-01-22*  
*Related Missions: Mission 6 (Grid), Mission 8 (Graph)*  
*AoC Applications: 2023 Day 22 (Sand Slabs)*
