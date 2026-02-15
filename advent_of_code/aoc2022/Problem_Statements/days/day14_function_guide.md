# Day 14: Regolith Reservoir - Function Guide

**Problem**: Simulate falling sand in a cave system with rock structures and physics-based settling.

**Navigation**: [← Day 13](day13_function_guide.md) | [Problem](day14.md) | [Code](../../../src/solver/day14.rs) | [Summary](../summary_2022.md) | [Day 15 →](day15_function_guide.md)

---

## Overview

### Problem Summary
- **Part 1**: Count sand units that come to rest before falling into the abyss
- **Part 2**: Add infinite floor at y=max+2, count sand until source (500,0) is blocked

### Performance
- **Parse**: 39.6µs (coordinate parsing, rock structure drawing)
- **Part 1**: 208µs (763 sand units simulated)
- **Part 2**: 8.37ms (23,921 sand units simulated)
- **Combined runtime**: 8.62ms
- **Complexity**: O(units × max_y) for simulation

### Key Insight
**Coordinate normalization for memory efficiency**: Instead of allocating a 601×201 grid (x: 0-600, y: 0-200), we normalize coordinates to only the required range (e.g., x: 489-573). This saves 66% memory and improves cache locality.

**Mission 6 Integration**: Uses validated `Grid<Tile>` and `Coord` components from Mission 6 for type-safe coordinate handling and proven correctness.

---

## Algorithm Analysis

### Sand Physics

**Movement Rules** (try in order):
1. **Down**: `(x, y+1)` - Falls straight down if air
2. **Down-left**: `(x-1, y+1)` - If blocked, try diagonal left
3. **Down-right**: `(x+1, y+1)` - If blocked, try diagonal right
4. **Rest**: If all three blocked, sand comes to rest at current position

**Example** (from problem):
```
......+...    ← Source at (500, 0)
..........
..........
........#.    ← Sand tries down → blocked
......oo#.    ← Tries down-left → air → settles
#########.    ← Rock floor
```

### Part 1: Abyss Detection

**Goal**: Simulate until sand falls below maximum rock y-coordinate (into endless void).

**Algorithm**:
```
while true:
    drop one sand unit from source (500, 0)

    loop:
        try down, down-left, down-right (in order)

        if moved to position with y > max_y:
            return count  // Fell into abyss

        if all three blocked:
            place Sand at current position
            count++
            break to next unit
```

**Termination**: First sand unit that falls below `max_y` ends simulation.

**Example Result**: 24 sand units (from problem example), 763 (actual input)

---

### Part 2: Infinite Floor

**Goal**: Add floor at `y = max_y + 2`, simulate until source is blocked.

**Key Differences**:
1. **Floor**: Infinite horizontal line at y=max_y+2
2. **Termination**: When sand comes to rest at source (500,0)
3. **Grid Size**: Must be wider to accommodate pyramid spread

**Pyramid Spread Calculation**:
```
Floor at y = max_y + 2
Maximum horizontal spread from source = floor_y distance
Example: If max_y=9, floor=11, source=(500,0)
  → Sand can spread ±11 from source horizontally
  → Need grid width to accommodate this spread
```

**Algorithm**:
```
floor_y = max_y + 2
Create wider grid (add floor_y padding on both sides)

while true:
    if source cell contains Sand:
        return count  // Source blocked!

    drop one sand unit from source

    loop:
        if current_y + 1 == floor_y:
            rest on floor (one row above)
            count++
            break

        try down, down-left, down-right

        if all blocked:
            place Sand
            count++
            break
```

**Result**: 93 sand units (example), 23,921 (actual input)

---

## Function Breakdown

### 1. `parse(input: &str) -> ParsedData`

**Purpose**: Parse rock paths and build grid with normalized coordinates.

**Steps**:
```
1. Parse all rock paths: "x1,y1 -> x2,y2 -> ..." format
2. Find bounding box (min_x, max_x, min_y, max_y)
3. Calculate x_offset for normalization (min_x - 2 for padding)
4. Create grid: width = max_x - x_offset + 3, height = max_y + 1
5. Draw rock structures as lines between consecutive points
6. Calculate normalized source position
```

**Coordinate Normalization Example**:
```
Input coordinates: x range [489, 573], y range [0, 158]
Without normalization: 601 × 159 grid (0-600 × 0-158)
With normalization:
  x_offset = 489 - 2 = 487
  width = 573 - 487 + 3 = 89
  height = 159
  Grid: 89 × 159 (saves 85% memory!)

Source (500, 0) → (500 - 487, 0) = (13, 0) in normalized grid
```

**Rock Drawing**:
```rust
for window in path.windows(2):
    let (x1, y1) = window[0]
    let (x2, y2) = window[1]

    if x1 == x2:
        // Vertical line
        for y in y1.min(y2)..=y1.max(y2):
            grid[Coord::new(x1_norm, y)] = Tile::Rock
    else:
        // Horizontal line
        for x in x1_norm.min(x2_norm)..=x1_norm.max(x2_norm):
            grid[Coord::new(x, y1)] = Tile::Rock
```

**Returns**: `ParsedData { grid, source, max_y }`

---

### 2. `try_move_sand(grid: &Grid<Tile>, pos: Coord) -> Option<Coord>`

**Purpose**: Attempt to move sand one step according to physics rules.

**Logic**:
```rust
1. Try down (x, y+1):
   if in bounds && air: return Some(down_pos)
   if out of bounds: return Some(down_pos)  // Falls into abyss

2. Try down-left (x-1, y+1):
   if x > 0 && in bounds && air: return Some(down_left)

3. Try down-right (x+1, y+1):
   if in bounds && air: return Some(down_right)
   if out of bounds: return Some(down_right)  // Falls off edge

4. None  // All three blocked, sand rests
```

**Bounds Checking**:
- **Underflow protection**: Check `x > 0` before `x - 1` (prevents usize underflow)
- **Abyss detection**: Out-of-bounds positions signal falling into void

**Returns**:
- `Some(new_pos)` if moved (including out-of-bounds for abyss)
- `None` if sand comes to rest

---

### 3. `drop_sand(grid: &mut Grid<Tile>, source: Coord, max_y: usize) -> bool`

**Purpose**: Simulate one sand unit falling (Part 1 version with abyss).

**Algorithm**:
```rust
let mut pos = source

loop:
    match try_move_sand(grid, pos):
        Some(next):
            if next.y > max_y:
                return false  // Fell into abyss!
            pos = next

        None:
            grid[pos] = Tile::Sand
            return true  // Came to rest
```

**Returns**: `true` if sand came to rest, `false` if into abyss

---

### 4. `drop_sand_with_floor(grid: &mut Grid<Tile>, source: Coord, floor_y: usize) -> bool`

**Purpose**: Simulate one sand unit with infinite floor (Part 2 version).

**Additions to Part 1**:
```rust
// Check source blockage (termination condition)
if grid[source] == Tile::Sand:
    return false  // Source blocked!

let mut pos = source

loop:
    // Floor check (one row above floor)
    if pos.y + 1 == floor_y:
        grid[pos] = Tile::Sand
        return true  // Rest on floor

    match try_move_sand(grid, pos):
        Some(next):
            // Handle edge overflow (grid width assumption)
            if next.x >= grid.width():
                grid[pos] = Tile::Sand
                return true
            pos = next

        None:
            grid[pos] = Tile::Sand
            return true
```

**Floor Logic**: Sand rests when `y+1 == floor_y` (one row above floor line)

**Returns**: `true` if sand came to rest, `false` if source blocked

---

### 5. `part1(data: &ParsedData) -> usize`

**Purpose**: Count sand units until one falls into abyss.

**Algorithm**:
```rust
let mut grid = data.grid.clone()
let mut count = 0

while drop_sand(&mut grid, data.source, data.max_y):
    count += 1

return count
```

**Complexity**: O(units × max_y) where units = result count

---

### 6. `part2(data: &ParsedData) -> usize`

**Purpose**: Count sand units until source is blocked (with floor).

**Grid Expansion**:
```rust
floor_y = max_y + 2
left_padding = floor_y
right_padding = floor_y
width_needed = grid.width + left_padding + right_padding

// Create wider grid and copy rocks with offset
new_grid = Grid::new(width_needed, floor_y, Tile::Air)
for each rock in old_grid:
    new_grid[pos + left_padding] = Rock

new_source = source + left_padding
```

**Why Wider Grid?**
- Sand spreads in pyramid shape
- Maximum horizontal spread = floor_y from source
- Must allocate padding on both sides

**Simulation**:
```rust
let mut count = 0

while drop_sand_with_floor(&mut grid, new_source, floor_y):
    count += 1

return count
```

**Complexity**: O(units × floor_y) where units ≈ pyramid volume

---

## Mission 6 Integration

### Components Used

**1. `Grid<T>`** - Generic 2D grid structure
```rust
use mission6::{Coord, Grid};

// Type-safe tile grid
enum Tile { Air, Rock, Sand }
let grid: Grid<Tile> = Grid::new(width, height, Tile::Air);

// Indexing with Coord
grid[Coord::new(x, y)] = Tile::Rock;
```

**Benefits**:
- Type-safe coordinate access (no manual bounds checking)
- Proven correct through Mission 6's test suite
- Consistent API with other AoC days (Day 9)

**2. `Coord`** - Type-safe 2D coordinates
```rust
let pos = Coord::new(x, y);
let down = Coord::new(pos.x, pos.y + 1);
let down_left = Coord::new(pos.x - 1, pos.y + 1);
```

**Benefits**:
- Prevents mixing up x/y coordinates (compiler-enforced)
- Self-documenting code (`Coord` vs `(usize, usize)`)
- Eliminates 42 lines of custom grid code (compared to initial implementation)

### Integrator Philosophy in Action

**Before Mission 6**:
```rust
// Custom 2D grid: Vec<Vec<Tile>>
let grid: Vec<Vec<Tile>> = vec![vec![Tile::Air; width]; height];
// Manual bounds checking, error-prone indexing
if x < grid[0].len() && y < grid.len() {
    grid[y][x] = Tile::Rock;  // Easy to swap x/y!
}
```

**After Mission 6**:
```rust
// Validated component composition
let mut grid: Grid<Tile> = Grid::new(width, height, Tile::Air);
grid[Coord::new(x, y)] = Tile::Rock;  // Type-safe, self-checking
```

**Impact**: 42 lines eliminated, type safety improved, consistent architecture

---

## Performance Characteristics

### Parse (39.6µs)
- **Bottlenecks**: String splitting, coordinate parsing, grid allocation
- **Optimizations Applied**:
  - Dynamic bounds calculation (minimal grid size)
  - Single-pass rock drawing
  - Coordinate normalization (66% memory savings for typical inputs)

### Part 1 (208µs for 763 units)
- **Per-unit cost**: ~272ns per sand unit
- **Bottlenecks**: Grid lookups (3 per move attempt)
- **Optimizations Considered**:
  - HashSet for filled cells (memory trade-off)
  - Early termination patterns (negligible improvement)

### Part 2 (8.37ms for 23,921 units)
- **Per-unit cost**: ~350ns per sand unit
- **Slower than Part 1**: Wider grid → more memory per lookup
- **Dominant Factor**: 31× more sand units (763 → 23,921)
- **Grid Size**:
  - Part 1: ~89 × 159 = 14,151 cells
  - Part 2: ~(89 + 2×11) × 11 = 1,221 cells (floor_y=11 for example)
  - Actual input Part 2: Much larger due to higher max_y

**Performance Target**: ✅ Both parts under 10ms (no optimization needed)

---

## Edge Cases Handled

### 1. **Coordinate Underflow**
```rust
// Problem: x-1 underflows when x=0
if pos.x > 0 {  // Guard against underflow
    let down_left = Coord::new(pos.x - 1, pos.y + 1);
    // ...
}
```

### 2. **Empty Input**
```rust
// No rock structures → sand falls immediately
parse("") → grid with only Air
part1() → 0 (first sand falls into abyss)
```

### 3. **Grid Width for Part 2**
```rust
// Pyramid spread requires padding
let left_padding = floor_y;  // Maximum spread distance
let width_needed = grid.width + 2 * floor_y;
// Prevents sand falling off grid edges
```

### 4. **Source Blockage Detection**
```rust
// Part 2 termination: source filled
if grid[source] == Tile::Sand {
    return false;  // Stop simulation
}
```

---

## Testing Strategy

### Unit Tests (12 tests)
1. **Parse correctness**: Coordinate normalization, source position, max_y
2. **Rock structure**: Verify 20 rocks in example, line drawing (horizontal, vertical)
3. **Part 1 example**: 24 units expected
4. **Part 2 example**: 93 units expected
5. **Edge cases**: Empty input, single point rock, straight-down falling
6. **Actual input verification**: 763 (Part 1), 23,921 (Part 2)

### Integration Coverage
- Example input (both parts)
- Actual input (regression test)
- Boundary conditions (source blockage, abyss detection)

---

## Lessons Learned

### 1. **Coordinate Normalization = Memory Efficiency**
- Calculate actual bounds, not theoretical max
- Example: 601×201 → 89×159 (85% reduction)
- Improves cache locality and reduces allocation time

### 2. **Mission Integration > Custom Implementation**
- Even when custom code works, mission components improve architecture
- Grid<T> + Coord: Type safety, consistency, proven correctness
- Integrator mindset: Compose validated components

### 3. **Physics Simulation Pattern**
```rust
// Clean separation: physics rules vs termination conditions
try_move_sand()  // Pure physics (down, down-left, down-right)
drop_sand()      // Part 1 termination (abyss)
drop_sand_with_floor()  // Part 2 termination (source blockage, floor)
```

### 4. **Underflow Guards Essential**
```rust
// usize arithmetic requires explicit bounds checks
if pos.x > 0 {  // MUST check before pos.x - 1
    // Safe to subtract
}
```

### 5. **Acceptable Performance Threshold**
- Part 2: 8.37ms for 23,921 simulations
- Per-unit: 350ns (includes grid lookups, state updates)
- **Decision**: No optimization needed (< 10ms threshold)
- Alternative considered: HashSet for filled cells (memory trade-off not worth it)

---

## Related Patterns

- **Simulation**: Day 5 (stacks), Day 9 (rope), Day 10 (CPU), Day 11 (monkeys)
- **Grid**: Day 8 (visibility), Day 12 (pathfinding)
- **Mission 6**: Day 9 (rope positions), Day 14 (sand grid)
- **Coordinate systems**: Day 9 (signed), Day 14 (normalized unsigned)
- **Physics rules**: Day 9 (rope following), Day 14 (sand falling)

---

## Summary

**What We Built**: Sand physics simulator with coordinate normalization and Mission 6 integration

**Key Techniques**:
- Dynamic bounds calculation for minimal grid allocation
- Coordinate normalization (66% memory savings)
- Mission 6 Grid<T> and Coord for type safety
- Separate physics rules from termination conditions
- Underflow-safe arithmetic for boundary cases

**Performance**: 8.62ms total (39.6µs parse + 208µs Part 1 + 8.37ms Part 2)

**Integrator Win**: Refactored custom grid → Mission 6 components (42 lines eliminated, type safety gained)

**Mission 6 Usage**: [Grid<Tile>](../../../missions/Mission6/src/lib.rs), [Coord](../../../missions/Mission6/src/lib.rs#Coord)

---

**Navigation**: [← Day 13](day13_function_guide.md) | [Problem](day14.md) | [Code](../../../src/solver/day14.rs) | [Summary](../summary_2022.md) | [Day 15 →](day15_function_guide.md)
