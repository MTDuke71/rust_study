# Day 23: A Long Walk - Complete Function Guide

**Problem**: Find the longest path through a hiking trail maze with directional slope restrictions (Part 1) and without restrictions (Part 2).

**Solution Location**: `advent_of_code/aoc2023/src/solver/day23.rs`

---

## Table of Contents

1. [Problem Summary](#problem-summary)
2. [Algorithm Overview](#algorithm-overview)
3. [Data Structures](#data-structures)
4. [Core Functions](#core-functions)
5. [Part 1: Longest Path with Slopes](#part-1-longest-path-with-slopes)
6. [Part 2: Longest Path Without Slopes](#part-2-longest-path-without-slopes)
7. [Graph Contraction Optimization](#graph-contraction-optimization)
8. [Mathematical Foundations](#mathematical-foundations)
9. [Performance Analysis](#performance-analysis)
10. [Testing Strategy](#testing-strategy)
11. [Complete Code Walkthrough](#complete-code-walkthrough)

---

## Problem Summary

### Part 1: Longest Path Respecting Slope Directions

Given:
- Hiking trail map as 2D grid
- `.` = path (can move in any cardinal direction)
- `#` = forest (impassable)
- `^`, `>`, `v`, `<` = slopes (MUST move in indicated direction when on slope)
- Start at top row's single path
- Goal at bottom row's single path
- Cannot revisit tiles

Find the **longest hike** (maximum number of steps) from start to goal.

**Example Tile Types**:
```
#.#####################    # = forest (impassable)
#.......#########...###    . = path (free movement)
###.....#.>.>.###.#.###    > = slope east (must go right)
###v#####.#v#.###.#.###    v = slope south (must go down)
```

**Answer (Example)**: 94 steps

**Key Constraint**: Slopes are **one-way**. If you step onto a `>` slope, your next move MUST be East.

### Part 2: Longest Path Treating Slopes as Paths

**Change**: Ignore slope restrictions - treat all slope tiles as normal paths (`.`).

**Result**: Many more possible paths since movement is no longer forced.

**Answer (Example)**: 154 steps

**Challenge**: Exponentially larger search space → requires optimization.

---

## Algorithm Overview

### High-Level Strategy

```
Part 1: DFS with Slope Restrictions
├─ Parse grid into HikingMap
├─ Find start (top row) and goal (bottom row)
├─ DFS with backtracking from start
│  ├─ For each unvisited neighbor:
│  │  ├─ Check if move is valid (respect slopes)
│  │  ├─ Mark as visited
│  │  ├─ Recursively explore
│  │  └─ Backtrack (unmark visited)
│  └─ Return maximum path length found
└─ Return longest path to goal

Part 2: Graph Contraction + DFS
├─ Identify junction points (nodes with >2 neighbors)
├─ Build contracted graph:
│  ├─ Junctions = vertices
│  ├─ Corridors = weighted edges (distance between junctions)
│  └─ Reduce ~10,000 tiles to ~35 junctions
├─ DFS on contracted graph
│  ├─ Much smaller state space
│  └─ Avoids stack overflow from deep recursion
└─ Return longest path to goal
```

### Complexity Analysis

| **Operation** | **Time Complexity** | **Space Complexity** |
|---------------|---------------------|----------------------|
| **Parse Grid** | O(r × c) | O(r × c) |
| **DFS (Part 1)** | O(4^n) worst case | O(n) recursion stack |
| **Find Junctions** | O(r × c) | O(j) junctions |
| **Build Graph** | O(r × c) | O(j + e) graph |
| **DFS on Graph (Part 2)** | O(4^j) | O(j) stack |

Where:
- `r` = number of rows (~141 in real input)
- `c` = number of columns (~141 in real input)
- `n` = path length (~2,000-7,000 steps)
- `j` = number of junctions (~35 in real input)
- `e` = edges between junctions (~100-150)

**Why Graph Contraction Works**:
- Reduces search space from ~20,000 tiles to ~35 junctions
- Reduces branching factor by collapsing long corridors
- Changes O(4^2000) to O(4^35) - massive improvement

---

## Data Structures

### Tile - Grid Cell Type

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,        // '.' - normal walkable path
    Forest,      // '#' - impassable wall
    SlopeNorth,  // '^' - forces movement north
    SlopeEast,   // '>' - forces movement east
    SlopeSouth,  // 'v' - forces movement south
    SlopeWest,   // '<' - forces movement west
}
```

**Purpose**: Represent each type of terrain in the hiking trail.

**Why enum**: Type-safe representation with pattern matching for movement rules.

**Traits**:
- `Copy`: Lightweight (1 byte)
- `Eq`: Required for comparisons

**Mission Integration**: Similar to Mission 6's grid cell types, but with directional constraints.

### Coord - 2D Position

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,  // Y coordinate (0 = top)
    col: usize,  // X coordinate (0 = left)
}
```

**Purpose**: Represent a position in the 2D grid.

**Why separate from Mission 6**: Simplified for this problem (no need for full Grid<T> generic).

**Key Methods**:
```rust
impl Coord {
    fn new(row: usize, col: usize) -> Self;
    
    fn neighbors(&self, rows: usize, cols: usize) 
        -> Vec<(Coord, Direction)>;
    // Returns valid cardinal neighbors with direction moved
}
```

**Traits**:
- `Hash`: Used in HashSet for visited tracking
- `Copy`: Efficient to pass around

### Direction - Cardinal Movement

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,  // row - 1
    East,   // col + 1
    South,  // row + 1
    West,   // col - 1
}
```

**Purpose**: Represent the four cardinal directions for movement.

**Why needed**: Slope validation requires knowing which direction we're attempting to move.

**Usage**: Paired with Coord in `neighbors()` to validate slope movement rules.

### HikingMap - Complete Trail System

```rust
struct HikingMap {
    grid: Vec<Vec<Tile>>,  // 2D array of tiles
    rows: usize,           // Grid height
    cols: usize,           // Grid width
    start: Coord,          // Starting position (top row)
    goal: Coord,           // Goal position (bottom row)
}
```

**Purpose**: Encapsulate the entire hiking trail with metadata.

**Why Vec<Vec<Tile>>**: Simple 2D array (not using Mission 6 Grid<T> for this problem).

**Key Invariants**:
- `grid.len() == rows`
- `grid[0].len() == cols` (rectangular)
- `start.row == 0`
- `goal.row == rows - 1`

---

## Core Functions

### 1. Parse Input → HikingMap

```rust
impl HikingMap {
    fn parse(input: &str) -> Result<Self>
}
```

**Purpose**: Convert text input into structured HikingMap.

**Algorithm**:
```
1. Split input into lines
2. For each line:
   - Map each char to Tile:
     - '.' → Tile::Path
     - '#' → Tile::Forest
     - '^' → Tile::SlopeNorth
     - '>' → Tile::SlopeEast
     - 'v' → Tile::SlopeSouth
     - '<' → Tile::SlopeWest
3. Find start: first Path in row 0
4. Find goal: first Path in row (rows-1)
5. Return HikingMap
```

**Complexity**: O(r × c) - linear scan of input

**Error Handling**: Panics on unknown characters (validated input).

**Example**:
```rust
let input = "#.###\n#...#\n###.#";
let map = HikingMap::parse(input)?;
// map.start = Coord { row: 0, col: 1 }
// map.goal = Coord { row: 2, col: 3 }
```

### 2. Get Tile at Coordinate

```rust
impl HikingMap {
    fn get(&self, coord: Coord) -> Tile
}
```

**Purpose**: Safe coordinate access.

**Algorithm**: Direct array indexing `grid[coord.row][coord.col]`.

**Complexity**: O(1)

**Precondition**: Coord must be within bounds (enforced by `neighbors()`).

### 3. Find Valid Neighbors

```rust
impl Coord {
    fn neighbors(&self, rows: usize, cols: usize) 
        -> Vec<(Coord, Direction)>
}
```

**Purpose**: Get all valid cardinal neighbors with their directions.

**Algorithm**:
```
result = []
if row > 0:
    result.push((Coord(row-1, col), North))
if row < rows - 1:
    result.push((Coord(row+1, col), South))
if col > 0:
    result.push((Coord(row, col-1), West))
if col < cols - 1:
    result.push((Coord(row, col+1), East))
return result
```

**Complexity**: O(1) - at most 4 neighbors

**Bounds Checking**: Built into the method (no out-of-bounds coordinates returned).

**Example**:
```rust
let coord = Coord::new(5, 5);
let neighbors = coord.neighbors(10, 10);
// Returns 4 neighbors: North, South, West, East
```

### 4. Validate Movement (Slope Check)

```rust
impl HikingMap {
    fn can_move(&self, _current: Coord, next: Coord, 
                direction: Direction, ignore_slopes: bool) -> bool
}
```

**Purpose**: Determine if moving from current to next in given direction is legal.

**Parameters**:
- `_current`: Where we're moving from (unused but kept for API consistency)
- `next`: Destination coordinate
- `direction`: Direction of movement
- `ignore_slopes`: If true, slopes act as normal paths (Part 2)

**Algorithm**:
```
tile = grid[next]
match tile:
    Forest → return false (impassable)
    Path → return true (always allowed)
    SlopeNorth → return ignore_slopes OR direction == North
    SlopeEast → return ignore_slopes OR direction == East
    SlopeSouth → return ignore_slopes OR direction == South
    SlopeWest → return ignore_slopes OR direction == West
```

**Complexity**: O(1)

**Part 1 vs Part 2**:
- Part 1: `ignore_slopes = false` → slopes enforce direction
- Part 2: `ignore_slopes = true` → slopes treated as Path

**Example**:
```rust
// Part 1: Standing at (5,5), slope East at (5,6)
can_move(coord, east_coord, East, false)  // true - going WITH slope
can_move(coord, east_coord, West, false)  // false - can't go against slope

// Part 2: Same situation
can_move(coord, east_coord, East, true)   // true
can_move(coord, east_coord, West, true)   // true - slopes ignored
```

---

## Part 1: Longest Path with Slopes

### Simple DFS Solution

```rust
impl HikingMap {
    fn find_longest_path(&self, ignore_slopes: bool) -> usize;
    fn dfs(&self, current: Coord, visited: &mut HashSet<Coord>, 
           ignore_slopes: bool) -> usize;
}
```

**Purpose**: Find longest path from start to goal using depth-first search with backtracking.

**Algorithm**:
```
function find_longest_path(ignore_slopes):
    visited = HashSet with [start]
    return dfs(start, visited, ignore_slopes)

function dfs(current, visited, ignore_slopes):
    // Base case: reached goal
    if current == goal:
        return 0
    
    max_length = 0
    
    // Try all neighbors
    for (next, direction) in current.neighbors():
        // Skip if visited or invalid move
        if visited.contains(next) OR !can_move(current, next, direction, ignore_slopes):
            continue
        
        // Explore this path
        visited.insert(next)
        length = dfs(next, visited, ignore_slopes)
        
        // Update max if valid path
        if length > 0 OR next == goal:
            max_length = max(max_length, length + 1)
        
        // Backtrack
        visited.remove(next)
    
    return max_length
```

**Complexity**: O(4^n) worst case, where n is the longest path length.

**Why This Works for Part 1**:
- Slope restrictions naturally prune the search space
- Most paths are forced by slopes → limited branching
- Recursion depth manageable (~2,000 steps)

**Visited Set**: Critical for preventing cycles and ensuring "never step on same tile twice".

**Backtracking**: Essential for exploring all possible paths - must unmark visited tiles when unwinding recursion.

**Example Trace** (simplified 5x5 grid):
```
Step 1: At start (0,1), visited = {(0,1)}
Step 2: Move to (1,1), visited = {(0,1), (1,1)}
Step 3: Move to (2,1), visited = {(0,1), (1,1), (2,1)}
...
Step N: Reached goal (4,3), return path length
Backtrack: Try alternative at (1,1)...
```

**Performance** (Actual Input):
- Part 1: ~22.9ms
- Path length: 2,182 steps
- Max recursion depth: ~2,200

---

## Part 2: Longest Path Without Slopes

### Problem: Stack Overflow with Naive DFS

**Issue**: Treating slopes as paths creates exponentially more possible routes.

**Symptoms**:
```
thread 'main' panicked: stack overflow
```

**Why**: 
- Without slope restrictions, grid has ~20,000 walkable tiles
- DFS explores all paths → recursion depth can reach 7,000+
- Rust default stack: ~1-2MB → overflows around 5,000-10,000 recursive calls

**Failed Approach**: Direct application of Part 1's DFS.

### Solution: Graph Contraction

**Key Insight**: Most of the grid is **corridors** (tiles with exactly 2 neighbors). These can be collapsed into weighted edges between **junctions** (tiles with >2 neighbors).

**Analogy**: Instead of representing "walk 100 steps down this hallway" as 100 nodes, represent it as a single edge with weight 100.

**Benefits**:
- Reduces state space from ~20,000 tiles to ~35 junctions
- Reduces recursion depth from ~7,000 to ~35
- Eliminates stack overflow
- Faster overall (less repeated work)

### Graph Contraction Algorithm

```rust
impl HikingMap {
    fn build_graph(&self, ignore_slopes: bool) 
        -> HashMap<Coord, Vec<(Coord, usize)>>;
}
```

**Purpose**: Build a contracted graph where vertices are junctions and edges are corridor distances.

**Algorithm**:
```
Phase 1: Identify Junctions
├─ junctions = HashSet
├─ junctions.insert(start)  // Always a junction
├─ junctions.insert(goal)   // Always a junction
├─ For each tile in grid:
│  ├─ Skip if Forest
│  ├─ Count accessible neighbors (using can_move)
│  └─ If count > 2: insert into junctions
└─ Return junctions set

Phase 2: Build Edges from Each Junction
├─ graph = HashMap<Coord, Vec<(Coord, distance)>>
├─ For each junction:
│  ├─ edges = []
│  ├─ For each immediate neighbor:
│  │  ├─ If invalid move: skip
│  │  ├─ visited = {junction, neighbor}
│  │  ├─ current = neighbor
│  │  ├─ distance = 1
│  │  │
│  │  ├─ Follow corridor until hitting another junction:
│  │  │  ├─ If current is junction: edges.push((current, distance))
│  │  │  ├─ Get unvisited neighbors
│  │  │  ├─ If 0 neighbors: dead end, break
│  │  │  ├─ If 1 neighbor: continue along corridor
│  │  │  │  ├─ visited.insert(neighbor)
│  │  │  │  ├─ current = neighbor
│  │  │  │  └─ distance += 1
│  │  │  └─ If >1 neighbors: should be junction, break
│  │  │
│  └─ graph.insert(junction, edges)
└─ Return graph
```

**Complexity**: O(r × c) to identify junctions + O(r × c) to trace corridors = O(r × c) total.

**Example**:
```
Original Grid (simplified):
  0 1 2 3 4
0 # . # # #
1 # . . . #
2 # # # . #
3 # # # . #
4 # # # . #

Junctions:
- (0, 1): start (1 neighbor, but special)
- (1, 1): 3 neighbors (junction)
- (4, 3): goal (1 neighbor, but special)

Graph:
(0, 1) → [(1, 1, distance=1)]
(1, 1) → [(0, 1, distance=1), (4, 3, distance=3)]
(4, 3) → [(1, 1, distance=3)]
```

**Space Complexity**: O(j + e) where j = junctions, e = edges.

### DFS on Contracted Graph

```rust
impl HikingMap {
    fn dfs_graph(&self, current: Coord, visited: &mut HashSet<Coord>,
                 graph: &HashMap<Coord, Vec<(Coord, usize)>>) -> usize;
}
```

**Purpose**: Find longest path on contracted graph (much smaller state space).

**Algorithm**:
```
function dfs_graph(current, visited, graph):
    // Base case
    if current == goal:
        return 0
    
    max_length = 0
    
    // Get edges from current junction
    if let Some(neighbors) = graph.get(current):
        for (next_junction, edge_distance) in neighbors:
            // Skip visited junctions
            if visited.contains(next_junction):
                continue
            
            // Explore path
            visited.insert(next_junction)
            length = dfs_graph(next_junction, visited, graph)
            
            // Update max
            if length > 0 OR next_junction == goal:
                max_length = max(max_length, length + edge_distance)
            
            // Backtrack
            visited.remove(next_junction)
    
    return max_length
```

**Complexity**: O(4^j) where j ≈ 35 junctions (much better than O(4^20000) tiles).

**Why This Avoids Stack Overflow**:
- Max recursion depth = number of junctions ≈ 35
- Stack usage: 35 × (frame size ~100 bytes) ≈ 3.5KB
- Well within stack limits

**Performance** (Actual Input):
- Part 2: 2.378 seconds (104x slower than Part 1 due to larger search space)
- Path length: 6,670 steps
- Max recursion depth: ~35 junctions

**Comparison**:
| Metric | Naive DFS | Graph Contraction |
|--------|-----------|-------------------|
| Nodes | ~20,000 | ~35 |
| Max depth | ~7,000 | ~35 |
| Stack usage | **OVERFLOW** | 3.5KB ✓ |
| Time | N/A (crashes) | 2.378s ✓ |

---

## Mathematical Foundations

### Longest Path Problem

**Definition**: Find the path between two vertices in a graph that visits the maximum number of vertices (or has maximum total edge weight).

**Complexity Class**: NP-Hard (no known polynomial-time algorithm).

**Why NP-Hard**: 
- Reduction from Hamiltonian Path Problem
- Requires exploring exponentially many paths in worst case
- No dynamic programming solution (subproblems overlap in complex ways)

**Contrast with Shortest Path**:
| Property | Shortest Path | Longest Path |
|----------|---------------|--------------|
| Optimal Substructure | YES ✓ | NO ✗ |
| DP Solution | YES (Dijkstra) | NO |
| Greedy Solution | YES (for some) | NO |
| Polynomial Time | YES (O(V+E)) | NO (NP-Hard) |

**Why No Optimal Substructure**: 
- Shortest path: If A→B→C is shortest, then A→B must be shortest to B
- Longest path: If A→B→C is longest, A→B might NOT be longest to B (might need to take detour that later connects)

### Graph Contraction

**Definition**: Process of simplifying a graph by merging vertices and edges while preserving important structural properties.

**Types**:
1. **Edge Contraction**: Merge two vertices connected by an edge
2. **Path Contraction**: Collapse degree-2 vertices (corridors) into weighted edges
3. **Tree Contraction**: Iteratively remove leaves

**Our Application**: Path contraction on grid graph.

**Correctness Proof**:

*Claim*: If path P from junction A to junction B passes through corridor C (degree-2 vertices), then any longest path using A→B uses the same corridor distance.

*Proof*: 
- Corridor C has no branches (degree-2 vertices only)
- Only one way to traverse from A to B through C
- Distance = number of corridor tiles (fixed)
- Therefore, replacing corridor with weighted edge preserves all path lengths
- QED ∎

**Application to Grid**:
- Original: ~20,000 vertices, ~40,000 edges
- Contracted: ~35 vertices, ~100 edges
- Reduction factor: ~570x fewer vertices

### Backtracking Algorithm

**Definition**: Algorithmic technique for finding all (or some) solutions by incrementally building candidates and abandoning ("backtracking") when a candidate cannot lead to a valid solution.

**Template**:
```
function backtrack(state, solution):
    if is_solution(state):
        record(solution)
        return
    
    for each choice in get_choices(state):
        if is_valid(choice):
            make_choice(choice)           // Modify state
            backtrack(new_state, solution)
            unmake_choice(choice)         // Restore state
```

**Our Application**:
- State: current position + visited set
- Choices: neighbors not yet visited
- Valid: not visited AND movement allowed
- Make choice: add to visited, recurse
- Unmake: remove from visited (backtrack)

**Time Complexity**: O(b^d) where:
- b = branching factor (avg 2-3 for Part 1, 3-4 for Part 2)
- d = maximum depth (path length)

### Directed Acyclic Graph (DAG)

**Part 1 Graph Structure**: Slopes create directed edges → DAG-like structure.

**Properties of Part 1**:
- Edges have direction (slopes force movement)
- Most vertices have 1-2 outgoing edges (limited branching)
- Natural topological ordering from top to bottom

**Why DFS Works Well**: 
- DAG structure naturally limits cycles
- Visited set prevents revisiting in current path
- Slopes create "channels" that prune search space

**Part 2 Graph Structure**: Undirected (bidirectional edges when slopes ignored).

**Properties of Part 2**:
- Each edge can be traversed both ways
- Higher branching factor (3-4 vs 1-2)
- Exponentially larger search space

**Why Graph Contraction Needed**:
- Undirected → more branching
- Must reduce state space to avoid explosion

---

## Performance Analysis

### Benchmark Results

**Part 1 Performance**:
```
day23_part1             time:   [22.795 ms 22.873 ms 22.958 ms]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
```

**Part 2 Performance**:
```
day23_part2             time:   [2.3738 s 2.3788 s 2.3844 s]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe
```

### Performance Breakdown

**Part 1 Timing** (~22.9ms total):
| Phase | Time (est) | % | Operations |
|-------|------------|---|------------|
| Parse | ~0.5ms | 2% | String to grid |
| DFS | ~22.4ms | 98% | Backtracking search |
| Total | 22.9ms | 100% | |

**Part 2 Timing** (~2.38s total):
| Phase | Time (est) | % | Operations |
|-------|------------|---|------------|
| Parse | ~0.5ms | 0.02% | String to grid |
| Build Graph | ~15ms | 0.6% | Junction finding + edge tracing |
| DFS | ~2,363ms | 99.3% | Search on contracted graph |
| Total | 2,378.8ms | 100% | |

### Optimization Impact

**Without Graph Contraction**:
- Stack overflow (crashes)
- Would take minutes to hours if stack were unlimited

**With Graph Contraction**:
- 2.378 seconds ✓
- 570x reduction in state space
- ~35 recursion depth (manageable)

**Speedup Factor**: ∞ (infinite speedup from crash to working solution!)

### Memory Usage

**Part 1**:
- Grid: 141 × 141 × 1 byte = ~20 KB
- Visited set: max 2,182 × 24 bytes = ~52 KB
- Stack: ~2,200 frames × 100 bytes = ~220 KB
- **Total**: ~292 KB

**Part 2**:
- Grid: ~20 KB
- Graph: 35 junctions × (Vec overhead + edges) ≈ 10 KB
- Visited set: 35 × 24 bytes = ~0.8 KB
- Stack: 35 frames × 100 bytes = ~3.5 KB
- **Total**: ~34 KB (less than Part 1 due to contraction!)

### Scaling Characteristics

**How would solution scale?**

| Grid Size | Part 1 (DFS) | Part 2 (Graph) |
|-----------|--------------|----------------|
| 50×50 | ~5ms | ~200ms |
| 100×100 | ~15ms | ~1s |
| 141×141 | **22.9ms** | **2.38s** |
| 200×200 | ~50ms | ~8s |
| 500×500 | ~200ms | Stack overflow likely |

**Bottleneck**: Part 2 is exponential in number of junctions. If junctions grow, time explodes.

**Solution for Larger Grids**: 
- Memoization (cache subproblem results)
- Heuristic pruning (A* with admissible heuristic)
- Parallel search (explore branches concurrently)

---

## Testing Strategy

### Test Hierarchy

```
Unit Tests
├─ Coord::neighbors() - boundary cases
├─ HikingMap::parse() - grid creation
├─ HikingMap::get() - tile access
└─ HikingMap::can_move() - slope validation

Integration Tests
├─ Example input Part 1 (94 steps)
├─ Example input Part 2 (154 steps)
└─ Tile parsing correctness

System Tests
├─ Real input Part 1 (2,182 steps)
└─ Real input Part 2 (6,670 steps)
```

### Example Test Cases

```rust
#[test]
fn test_part1_example() {
    let result = solve_part1(EXAMPLE).unwrap();
    assert_eq!(result, "94");
}

#[test]
fn test_part2_example() {
    let result = solve_part2(EXAMPLE).unwrap();
    assert_eq!(result, "154");
}

#[test]
fn test_parse_map() {
    let map = HikingMap::parse(EXAMPLE).unwrap();
    assert_eq!(map.rows, 23);
    assert_eq!(map.cols, 23);
    assert_eq!(map.start, Coord::new(0, 1));
    assert_eq!(map.goal, Coord::new(22, 21));
}

#[test]
fn test_tile_parsing() {
    let map = HikingMap::parse(EXAMPLE).unwrap();
    assert_eq!(map.get(Coord::new(0, 0)), Tile::Forest);
    assert_eq!(map.get(Coord::new(0, 1)), Tile::Path);
    assert_eq!(map.get(Coord::new(3, 10)), Tile::SlopeEast);
    assert_eq!(map.get(Coord::new(4, 3)), Tile::SlopeSouth);
}
```

### Edge Cases

**Grid Boundaries**:
- ✓ Start at row 0
- ✓ Goal at row (rows-1)
- ✓ No out-of-bounds access in neighbors()

**Slope Validation**:
- ✓ Can move WITH slope direction
- ✓ Cannot move AGAINST slope direction (Part 1)
- ✓ Can move any direction on slope (Part 2)

**Graph Contraction**:
- ✓ Start and goal always junctions
- ✓ Corridor tracing terminates
- ✓ Edge distances match corridor lengths

---

## Complete Code Walkthrough

### Entry Points

```rust
pub fn solve_part1(input: &str) -> Result<String> {
    let map = HikingMap::parse(input)?;
    let longest = map.find_longest_path(false); // Respect slopes
    Ok(longest.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let map = HikingMap::parse(input)?;
    let graph = map.build_graph(true); // Ignore slopes
    let mut visited = HashSet::new();
    visited.insert(map.start);
    let longest = map.dfs_graph(map.start, &mut visited, &graph);
    Ok(longest.to_string())
}
```

**Design Decisions**:
1. **Different strategies for Part 1 vs Part 2**: Part 1 uses simple DFS (works fine), Part 2 uses graph contraction (avoids stack overflow)
2. **Single parse**: Both parts share the same parsing logic
3. **Boolean flag**: `ignore_slopes` parameter controls slope behavior

### Full Implementation Flow

**Part 1 Execution**:
```
solve_part1(input)
  ↓
HikingMap::parse(input)
  ├─ Create grid: Vec<Vec<Tile>>
  ├─ Find start: Coord(0, 1)
  └─ Find goal: Coord(141, 139)
  ↓
find_longest_path(ignore_slopes=false)
  ├─ visited = {start}
  └─ dfs(start, visited, false)
      ├─ Base case: current == goal? No
      ├─ For each neighbor:
      │  ├─ Check can_move (respecting slopes)
      │  ├─ If valid: mark visited, recurse, backtrack
      │  └─ Track max length
      └─ Return 2182
```

**Part 2 Execution**:
```
solve_part2(input)
  ↓
HikingMap::parse(input)
  ↓
build_graph(ignore_slopes=true)
  ├─ Phase 1: Find junctions (~35)
  │  ├─ Start, goal always junctions
  │  └─ Any tile with >2 neighbors
  ├─ Phase 2: Trace corridors
  │  ├─ From each junction, follow each direction
  │  ├─ Count steps until next junction
  │  └─ Build edge: (junction_a, junction_b, distance)
  └─ Return HashMap<Coord, Vec<(Coord, usize)>>
  ↓
dfs_graph(start, visited={start}, graph)
  ├─ Base case: current == goal? No
  ├─ For each neighbor junction + distance:
  │  ├─ If visited: skip
  │  ├─ Mark visited, recurse, backtrack
  │  └─ Track max (length + edge_distance)
  └─ Return 6670
```

### Key Implementation Details

**Coord::neighbors() Boundary Handling**:
```rust
fn neighbors(&self, rows: usize, cols: usize) -> Vec<(Coord, Direction)> {
    let mut result = Vec::new();
    
    // North - check if not at top row
    if self.row > 0 {
        result.push((Coord::new(self.row - 1, self.col), Direction::North));
    }
    // ... similar for South, West, East
    
    result
}
```

**Why this works**: Prevents usize underflow by checking before subtracting.

**Slope Validation Logic**:
```rust
fn can_move(&self, _current: Coord, next: Coord, direction: Direction, 
            ignore_slopes: bool) -> bool {
    let tile = self.get(next);
    
    match tile {
        Tile::Forest => false,
        Tile::Path => true,
        Tile::SlopeNorth => ignore_slopes || direction == Direction::North,
        // ... similar for other slopes
    }
}
```

**Pattern**: Combines boolean short-circuit with pattern matching.

**Graph Contraction Corridor Following**:
```rust
// Follow corridor until hitting junction
loop {
    if junctions.contains(&current) {
        edges.push((current, distance));
        break;
    }
    
    let next_coords: Vec<_> = current.neighbors(self.rows, self.cols)
        .into_iter()
        .filter(|(next, dir)| {
            !visited.contains(next) && 
            self.can_move(current, *next, *dir, ignore_slopes)
        })
        .collect();
    
    if next_coords.is_empty() {
        break; // Dead end
    } else if next_coords.len() == 1 {
        // Continue along corridor
        let (next, _) = next_coords[0];
        visited.insert(next);
        current = next;
        distance += 1;
    } else {
        break; // Multiple paths (shouldn't happen if junctions correct)
    }
}
```

**Why collect first**: Need to know count before deciding how to proceed.

---

## Mission Integration

### Mission 6: Grid & 2D Arrays

**Concepts Used**:
- ✅ Coordinate representation (Coord struct)
- ✅ Neighbor finding (cardinal directions)
- ✅ Grid parsing from text
- ✅ Bounds checking

**Not Used** (could be integrated):
- Mission 6's generic `Grid<T>` (opted for simpler Vec<Vec<Tile>>)
- Direction enum from Mission 6 (created custom for this problem)
- Distance calculations (Manhattan, Euclidean)

**Why custom implementation**: Problem-specific needs (slope validation) made custom simpler than adapting Mission 6.

### Mission 8: Graph Traversal

**Concepts Used**:
- ✅ Depth-First Search (DFS)
- ✅ Visited set tracking
- ✅ Graph representation (adjacency list)
- ✅ Backtracking

**Advanced Application**:
- Graph contraction (not in Mission 8)
- Longest path problem (vs shortest path in Mission 8)
- Directional constraints (slopes)

**Integration Point**: Could implement Mission 8's `Graph` trait for `HikingMap`:
```rust
impl Graph for HikingMap {
    type Node = Coord;
    
    fn neighbors(&self, node: Coord) -> Vec<Coord> {
        node.neighbors(self.rows, self.cols)
            .into_iter()
            .filter(|(n, d)| self.can_move(node, *n, *d, false))
            .map(|(n, _)| n)
            .collect()
    }
}
```

---

## Mathematical Insights

### Why Longest Path is Hard

**Contrast with Shortest Path (easy)**:
```
Shortest Path (Dijkstra):
- Optimal substructure: shortest A→C uses shortest A→B (if path is A→B→C)
- Greedy choice: always pick closest unvisited vertex
- Polynomial time: O((V+E) log V)

Longest Path (this problem):
- NO optimal substructure: longest A→C might NOT use longest A→B
- NO greedy choice: picking longest edge might miss better path
- Exponential time: O(b^d) backtracking required
```

**Example Breaking Optimal Substructure**:
```
Graph:
A --1--> B --1--> C
 \             /
  ----3------

Longest A→C: A→C (3) - doesn't use any A→B path!
```

### Graph Contraction Theory

**Formal Definition**:

Given graph G = (V, E), a **path contraction** produces G' = (V', E') where:
- V' = junctions (degree ≠ 2) ∪ {start, goal}
- E' = {(u, v, w) | ∃ path u→v of length w in G with only degree-2 vertices}

**Theorem**: For longest path problems, path contraction preserves solution.

**Proof Sketch**:
1. Any path in G using corridor must traverse entire corridor (degree-2 vertices)
2. Corridor length is fixed (no choices within corridor)
3. Replacing corridor with weighted edge preserves total path weight
4. Solution in G' directly maps to solution in G
5. ∴ optimal solution preserved under contraction. QED ∎

**Practical Impact**: 
- G has |V| = 19,881 vertices
- G' has |V'| = 35 vertices
- Reduction ratio: 568:1

---

## Lessons Learned

### What Worked Well

✅ **Graph Contraction**: Transformed impossible problem (stack overflow) into solvable in 2.4s.

✅ **Incremental Development**: 
1. Solve Part 1 with simple DFS
2. Discover stack overflow on Part 2
3. Implement graph contraction
4. Success!

✅ **Mission Integration Philosophy**: Recognized when to use mission libraries (concepts) vs custom code (implementation).

✅ **Backtracking Pattern**: Standard template applied cleanly.

### Challenges Encountered

❌ **Initial Part 2 Attempt**: Naive DFS caused stack overflow.

**Solution**: Graph contraction optimization.

❌ **Graph Contraction for Part 1**: Tried to apply to Part 1, but slopes create directed edges → contraction doesn't work simply.

**Lesson**: Different parts may need different algorithms even for "similar" problems.

❌ **Benchmark Time**: Part 2 takes ~4 minutes to benchmark (100 samples × 2.4s).

**Mitigation**: Reduced sample count or increase timeout in Criterion config.

### Future Optimizations

**Memoization**: Cache (position, visited) → longest_path. But visited set makes key large.

**Bitset for Visited**: Replace HashSet<Coord> with u128 bitset (max 128 tiles). Much faster.

**Parallel Branch Exploration**: Use rayon to explore branches in parallel (challenging with mutable visited set).

**Better Heuristics**: A* with admissible heuristic (though finding good heuristic for longest path is hard).

**Alternative: Meet in the Middle**: 
- Search from start and goal simultaneously
- Meet in middle, combine results
- Reduces search space from O(b^d) to O(2 × b^(d/2))

---

## Conclusion

Day 23 demonstrates the power of **algorithmic optimization** to transform impossible problems into solvable ones:

- **Part 1**: Simple DFS with backtracking works perfectly due to slope restrictions pruning search space.

- **Part 2**: Graph contraction reduces state space by 568× to avoid stack overflow and enable solution in reasonable time.

**Key Takeaway**: When exponential algorithms become too expensive, **preprocess to reduce state space** before searching.

**Mission Connections**:
- Grid navigation (Mission 6)
- Graph traversal (Mission 8)
- Optimization through problem transformation (new technique)

**Performance**: 
- Part 1: 22.9ms
- Part 2: 2,378.8ms (104× slower due to larger search space)
- Memory: <300 KB

**Solution demonstrates**: Backtracking, graph theory, algorithm optimization, and Rust performance tuning.
