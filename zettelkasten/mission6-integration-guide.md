# Mission 6 Integration Guide - Grid<T> + Coord

**Created**: 2026-02-14
**Tags**: #mission6 #grid #integration #pattern #aoc
**Links**: [[mission-6]], [[integrator-philosophy]], [[aoc2022-day14]], [[aoc2023-day16]]

---

## Overview

Mission 6 provides production-quality `Grid<T>` and `Coord` types for 2D spatial problems. This guide helps you recognize when to use these validated components versus implementing custom solutions.

**Core Philosophy**: Compose validated components when possible. Custom implementations should only exist when Mission 6 doesn't fit the problem constraints.

---

## When to Use Mission 6 Grid<T>

### ✅ Use Grid<T> When:

1. **2D Spatial Layout**
   - Problem involves rows and columns
   - Coordinates are (x, y) or (row, col)
   - Need to track state at each position

2. **Bounded Coordinates**
   - Grid has defined width and height
   - Can calculate or parse maximum dimensions
   - OK if dimensions are large (normal array-backed storage)

3. **Dense Data**
   - Most cells contain meaningful values
   - Not sparse (where most cells are empty/default)
   - Storage efficiency: O(width × height) is acceptable

4. **Need Indexing Operations**
   - Direct access by coordinate: `grid[coord]`
   - Iteration over neighbors
   - Boundary checking built-in

5. **Type Safety Desired**
   - Want compile-time prevention of row/col confusion
   - `Coord::new(x, y)` clearer than `(usize, usize)` tuples
   - Index operations return `&T` safely

### ❌ Don't Use Grid<T> When:

1. **Sparse Data**
   - Most cells are empty/unused
   - Only tracking specific positions
   - Better: `HashMap<(i32, i32), T>` or `HashSet<(i32, i32)>`

2. **Unbounded/Infinite Space**
   - Grid can grow arbitrarily in any direction
   - Can't predict maximum dimensions
   - Better: Hash-based positioning with i32/i64 coordinates

3. **Irregular Topology**
   - Non-rectangular layout (hexagonal, graph-based)
   - Connections don't follow grid neighbor pattern
   - Better: Custom graph structure or adjacency list

4. **Extreme Memory Constraints**
   - Grid dimensions would cause OOM
   - Need compressed representation
   - Better: Sparse data structures or bit packing

---

## Integration Checklist

### 1. Add Dependency

In your crate's `Cargo.toml`:

```toml
[dependencies]
mission6 = { path = "../../../missions/Mission6" }
```

**Note**: Adjust path based on your location relative to missions directory.

### 2. Import Types

```rust
use mission6::{Coord, Grid};
```

**Common mistake**: Don't use `missions::mission6` - the workspace member is just `mission6`.

### 3. Define Tile Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,    // Or Air, Open, etc.
    Wall,     // Or Rock, Obstacle, etc.
    // ... other states
}
```

**Best practices**:
- `Clone + Copy` for efficient grid operations
- `PartialEq + Eq` for comparison in logic
- `Debug` for debugging and display
- Use semantic names (Empty vs Air depends on problem domain)

### 4. Create Grid

```rust
// With initial value
let grid = Grid::new(width, height, Tile::Empty);

// From parsing (typical AoC pattern)
let mut grid = Grid::new(width, height, Tile::Empty);
// ... parse and populate
grid[Coord::new(x, y)] = Tile::Wall;
```

### 5. Use Coord for Positions

```rust
// Create coordinate
let pos = Coord::new(x, y);

// Access grid
let tile = grid[pos];
grid[pos] = Tile::Wall;

// Get bounds
if pos.x < grid.width() && pos.y < grid.height() {
    // In bounds
}
```

---

## Common Patterns

### Pattern 1: Parsing to Grid

**Typical AoC input**: Multi-line text map with characters representing tiles.

```rust
pub fn parse(input: &str) -> ParsedData {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    let mut grid = Grid::new(width, height, Tile::Empty);

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let tile = match ch {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                _ => panic!("Unknown tile: {}", ch),
            };
            grid[Coord::new(x, y)] = tile;
        }
    }

    ParsedData { grid }
}
```

**Key points**:
- Parse once, use many times
- Height = number of lines
- Width = length of first line (assume rectangular)
- Use pattern matching for character → Tile conversion

### Pattern 2: Coordinate Normalization

**Problem**: Input coordinates don't start at (0, 0), wasting memory.

**Example**: AoC 2022 Day 14 - x range [489, 573] instead of [0, 84].

```rust
fn parse_with_normalization(input: &str) -> ParsedData {
    // Parse all coordinates first
    let coords = parse_all_coordinates(input);

    // Find bounds
    let min_x = coords.iter().map(|c| c.0).min().unwrap();
    let max_x = coords.iter().map(|c| c.0).max().unwrap();
    let max_y = coords.iter().map(|c| c.1).max().unwrap();

    // Calculate offset for normalization
    let x_offset = min_x.saturating_sub(padding);
    let width = max_x - x_offset + padding;
    let height = max_y + 1;

    let mut grid = Grid::new(width, height, Tile::Empty);

    // Apply normalization when placing tiles
    for (orig_x, y) in coords {
        let norm_x = orig_x - x_offset;
        grid[Coord::new(norm_x, y)] = Tile::Wall;
    }

    ParsedData {
        grid,
        x_offset, // Save for later coordinate conversions
    }
}
```

**Benefits**:
- Reduces memory by 85% (Day 14: 600 width → 90 width)
- Still O(n) time for parsing
- Need to track offset for converting back if needed

**See**: [[coordinate-normalization-pattern]] for detailed guide.

### Pattern 3: Neighbor Iteration

**Four directions (cardinal)**:

```rust
const DIRECTIONS: [(i32, i32); 4] = [
    (0, -1),  // Up
    (0, 1),   // Down
    (-1, 0),  // Left
    (1, 0),   // Right
];

fn get_neighbors(grid: &Grid<Tile>, pos: Coord) -> Vec<Coord> {
    let mut neighbors = Vec::new();

    for (dx, dy) in DIRECTIONS {
        let new_x = pos.x as i32 + dx;
        let new_y = pos.y as i32 + dy;

        if new_x >= 0 && new_y >= 0 {
            let new_coord = Coord::new(new_x as usize, new_y as usize);
            if new_coord.x < grid.width() && new_coord.y < grid.height() {
                neighbors.push(new_coord);
            }
        }
    }

    neighbors
}
```

**Eight directions (including diagonals)**:

```rust
const DIRECTIONS_8: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),  // Top row
    (-1,  0),          (1,  0),  // Middle row (skip self)
    (-1,  1), (0,  1), (1,  1),  // Bottom row
];
```

**Key considerations**:
- Use `i32` for delta to handle negative offsets
- Check for negative results before casting to `usize`
- Bounds check against `grid.width()` and `grid.height()`
- Return `Vec<Coord>` for flexible use

### Pattern 4: Boundary Checking with usize

**Problem**: `usize` underflow when checking `pos.x - 1` near edge.

**Solution**: Check bounds BEFORE subtraction.

```rust
// ❌ WRONG - can panic if pos.x == 0
let left = Coord::new(pos.x - 1, pos.y);
if left.x >= 0 && grid[left] == Tile::Empty {
    // ...
}

// ✅ CORRECT - check bounds first
if pos.x > 0 {
    let left = Coord::new(pos.x - 1, pos.y);
    if grid[left] == Tile::Empty {
        // ...
    }
}
```

**Alternative**: Use `saturating_sub` for safe arithmetic.

```rust
let left_x = pos.x.saturating_sub(1);
if left_x != pos.x {  // Actually moved (wasn't at edge)
    let left = Coord::new(left_x, pos.y);
    // ...
}
```

### Pattern 5: Grid Iteration

**Iterate all positions**:

```rust
for y in 0..grid.height() {
    for x in 0..grid.width() {
        let coord = Coord::new(x, y);
        let tile = grid[coord];
        // Process tile...
    }
}
```

**Count specific tiles**:

```rust
let mut count = 0;
for y in 0..grid.height() {
    for x in 0..grid.width() {
        if grid[Coord::new(x, y)] == Tile::Wall {
            count += 1;
        }
    }
}
```

**Find positions matching predicate**:

```rust
fn find_positions<F>(grid: &Grid<Tile>, predicate: F) -> Vec<Coord>
where
    F: Fn(Tile) -> bool,
{
    let mut positions = Vec::new();
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            if predicate(grid[coord]) {
                positions.push(coord);
            }
        }
    }
    positions
}
```

---

## Real-World Examples

### Example 1: AoC 2022 Day 14 - Regolith Reservoir

**Problem**: Simulate falling sand in cave with rock structures.

**Why Mission 6**:
- 2D cave layout (width × height grid)
- Bounded space (can calculate max dimensions from input)
- Dense data (rocks, air, sand all meaningful)
- Need direct access by coordinate for physics simulation

**Integration approach**:

```rust
use mission6::{Coord, Grid};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

pub struct ParsedData {
    grid: Grid<Tile>,
    source: Coord,
    max_y: usize,
}

pub fn parse(input: &str) -> ParsedData {
    // Parse rock paths: "498,4 -> 498,6 -> 496,6"
    let paths = parse_rock_paths(input);
    let (min_x, max_x, _min_y, max_y) = find_bounds(&paths);

    // Coordinate normalization
    let x_offset = min_x.saturating_sub(2);
    let width = max_x - x_offset + 3;
    let height = max_y + 1;

    let mut grid = Grid::new(width, height, Tile::Air);

    // Draw rock structures
    for path in paths {
        for window in path.windows(2) {
            let (x1, y1) = window[0];
            let (x2, y2) = window[1];
            // Normalize and draw line...
        }
    }

    let source = Coord::new(500 - x_offset, 0);
    ParsedData { grid, source, max_y }
}
```

**Key benefits**:
- Type-safe coordinates (prevented row/col confusion)
- Built-in bounds checking
- 42 lines eliminated vs custom grid implementation
- Consistent with other AoC days using Mission 6

**Performance**: 8.62ms total (no overhead from Mission 6 abstraction)

**See**: [day14.rs](../advent_of_code/aoc2022/src/solver/day14.rs) for full implementation.

### Example 2: AoC 2023 Day 16 - The Floor Will Be Lava

**Problem**: Trace light beams through grid of mirrors and splitters.

**Why Mission 6**:
- 2D grid of mirror tiles
- Fixed dimensions from input
- Need to track beam positions and check tile types
- Frequent coordinate-based lookups during beam tracing

**Integration approach**:

```rust
use mission6::{Coord, Grid};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    MirrorForward,   // '/'
    MirrorBackward,  // '\'
    SplitterVert,    // '|'
    SplitterHoriz,   // '-'
}

fn trace_beam(grid: &Grid<Tile>, start: Coord, dir: Direction) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, dir));

    while let Some((pos, dir)) = queue.pop_front() {
        if !visited.insert((pos, dir)) {
            continue; // Already visited from this direction
        }

        // Check tile type and determine next moves
        match grid[pos] {
            Tile::MirrorForward => { /* reflect */ },
            Tile::SplitterVert if dir.is_horizontal() => { /* split */ },
            // ... other cases
        }
    }

    // Count unique positions (ignoring direction)
    visited.iter().map(|(pos, _)| pos).collect::<HashSet<_>>().len()
}
```

**Key benefits**:
- Eliminated 42 lines of custom grid code
- Type-safe Coord prevented coordinate swap bugs
- Consistent API with Day 14 and other grid problems

**Performance**: 23.08ms total with Rayon parallelization

**See**: [[aoc2023-day16]] for full details.

---

## Migration from Custom Grid

### Before (Custom Implementation)

```rust
struct CustomGrid {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl CustomGrid {
    fn new(width: usize, height: usize, default: Tile) -> Self {
        let tiles = vec![vec![default; width]; height];
        CustomGrid { tiles, width, height }
    }

    fn get(&self, x: usize, y: usize) -> Option<Tile> {
        if y < self.height && x < self.width {
            Some(self.tiles[y][x])
        } else {
            None
        }
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        if y < self.height && x < self.width {
            self.tiles[y][x] = tile;
        }
    }
}

// Usage
let mut grid = CustomGrid::new(width, height, Tile::Empty);
if let Some(tile) = grid.get(x, y) {
    // Process tile
}
grid.set(x, y, Tile::Wall);
```

**Issues**:
- 15+ lines of boilerplate code
- Separate bounds checking for get/set
- No indexing operator (must use get/set methods)
- Easy to confuse x/y order (row vs col)
- Need to implement Debug, Clone, etc. manually

### After (Mission 6 Integration)

```rust
use mission6::{Coord, Grid};

let mut grid = Grid::new(width, height, Tile::Empty);
let pos = Coord::new(x, y);

// Direct indexing with bounds checking
let tile = grid[pos];
grid[pos] = Tile::Wall;

// Bounds checking when needed
if pos.x < grid.width() && pos.y < grid.height() {
    // In bounds
}
```

**Benefits**:
- Zero boilerplate (1 line creation)
- Index operator with automatic bounds checking
- Type-safe Coord prevents x/y confusion
- Debug, Clone, etc. already implemented
- Consistent API across all code using Mission 6

**Migration steps**:
1. Add mission6 dependency to Cargo.toml
2. Replace `CustomGrid::new` with `Grid::new`
3. Replace all `(x, y)` tuples with `Coord::new(x, y)`
4. Replace `.get(x, y)` with `[coord]` indexing
5. Replace `.set(x, y, tile)` with `[coord] = tile` assignment
6. Delete CustomGrid implementation (typically 15-30 lines)

**Estimated time**: 5-10 minutes for typical AoC solution

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: Reinventing Grid

**Symptom**: Writing `Vec<Vec<T>>` wrapper with bounds checking.

```rust
// ❌ DON'T DO THIS
struct MyGrid {
    data: Vec<Vec<Tile>>,
}

impl MyGrid {
    fn get(&self, x: usize, y: usize) -> Option<Tile> { /* ... */ }
    fn set(&mut self, x: usize, y: usize, tile: Tile) { /* ... */ }
}
```

**Fix**: Use Mission 6 Grid<T> directly.

**When exception is OK**: If you need fundamentally different semantics (e.g., sparse grid, infinite grid, hexagonal topology).

### Anti-Pattern 2: Using Tuples for Coordinates

**Symptom**: Passing `(usize, usize)` everywhere instead of Coord.

```rust
// ❌ Unclear which is row vs column
fn process(grid: &Grid<Tile>, pos: (usize, usize)) {
    let tile = grid[Coord::new(pos.0, pos.1)]; // Is pos.0 x or y?
}

// ✅ Self-documenting with Coord
fn process(grid: &Grid<Tile>, pos: Coord) {
    let tile = grid[pos]; // Clear that pos is a coordinate
}
```

**Fix**: Use Coord throughout your API. Only convert to/from tuples at parsing boundaries.

### Anti-Pattern 3: Not Checking Bounds Before usize Math

**Symptom**: Panic on `pos.x - 1` when `pos.x == 0`.

```rust
// ❌ Can panic
let left = Coord::new(pos.x - 1, pos.y);

// ✅ Check first
if pos.x > 0 {
    let left = Coord::new(pos.x - 1, pos.y);
}

// ✅ Or use saturating arithmetic
let left_x = pos.x.saturating_sub(1);
if left_x != pos.x {
    let left = Coord::new(left_x, pos.y);
}
```

**Fix**: Always check bounds before subtraction with usize, or use saturating operations.

### Anti-Pattern 4: Using Mission 6 for Sparse Data

**Symptom**: Grid is 1000×1000 but only 50 cells have data.

```rust
// ❌ Wasteful - 1M cells for 50 data points
let grid = Grid::new(1000, 1000, Tile::Empty);
```

**Fix**: Use HashMap for sparse data.

```rust
// ✅ Efficient - only stores non-empty cells
let mut tiles: HashMap<(i32, i32), Tile> = HashMap::new();
tiles.insert((500, 750), Tile::Wall);
```

**Threshold**: If < 10% of cells have non-default values, consider sparse representation.

---

## Decision Tree

```
Is it a 2D spatial problem?
├─ No → Don't use Mission 6
└─ Yes → Can you bound the dimensions?
    ├─ No (infinite/unbounded) → Use HashMap<(i32,i32), T>
    └─ Yes → Is the data dense (>10% non-default)?
        ├─ No (sparse) → Use HashMap<(usize,usize), T>
        └─ Yes → Is the topology rectangular?
            ├─ No (hex, graph) → Custom structure
            └─ Yes → ✅ USE MISSION 6 GRID<T>
```

---

## Integration Checklist Summary

- [ ] Add `mission6` dependency to Cargo.toml
- [ ] Import `use mission6::{Coord, Grid};`
- [ ] Define Tile enum with `Clone, Copy, PartialEq, Eq, Debug`
- [ ] Create grid with `Grid::new(width, height, default)`
- [ ] Use `Coord::new(x, y)` for all positions (not tuples)
- [ ] Access with index operator: `grid[coord]`
- [ ] Check bounds before usize subtraction
- [ ] Consider coordinate normalization if input has offset
- [ ] Test with actual grid dimensions from problem

---

## Performance Characteristics

**Time Complexity**:
- Creation: O(width × height) - must initialize all cells
- Access: O(1) - direct array indexing
- Iteration: O(width × height) - must visit all cells

**Space Complexity**:
- O(width × height) - stores all cells regardless of content
- Not suitable for sparse data (use HashMap instead)

**Benchmarks from Real AoC Solutions**:
- Day 14: 8.62ms total (width=90, height=159, ~14K cells)
- Day 16: 23.08ms total (width=110, height=110, ~12K cells)
- No measurable overhead vs custom Vec<Vec<T>>

**When performance matters**:
- Mission 6 Grid is zero-cost abstraction over Vec<Vec<T>>
- Bounds checking is same as manual checks
- Type safety has zero runtime cost
- Bottleneck is always algorithm, not data structure

---

## Related Patterns

- [[coordinate-normalization-pattern]] - Reduce memory by shifting coordinates
- [[physics-simulation-pattern]] - Separating rules from termination (pairs well with Grid)
- [[mission-8]] - Graph traversal (for when topology isn't grid-based)
- [[integrator-philosophy]] - Compose validated components vs custom implementation

---

## References

- Mission 6 source: `missions/Mission6/src/lib.rs`
- AoC 2022 Day 14: `advent_of_code/aoc2022/src/solver/day14.rs`
- AoC 2023 Day 16: `advent_of_code/aoc2023/src/solver/day16.rs`
- Daily Note: [[2026-02-14]] - Mission 6 integration insights

---

## Key Takeaways

1. **Default to Mission 6** for 2D rectangular grids with bounded dimensions
2. **Type safety matters** - Coord prevents x/y confusion at zero runtime cost
3. **Integrator mindset** - Compose validated components vs reinventing
4. **Memory optimization** - Use coordinate normalization when input has offset
5. **Know when not to use** - Sparse data, unbounded space, irregular topology
6. **Zero overhead** - Mission 6 is abstraction without cost

**Remember**: Even when custom code "works", Mission 6 integration often improves architecture, consistency, and maintainability. The 42 lines you don't write are the easiest to maintain.
