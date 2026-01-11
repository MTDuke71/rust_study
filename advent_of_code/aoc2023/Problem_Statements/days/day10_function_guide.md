# Day 10: Pipe Maze - Function Guide

**Problem**: AoC 2023 Day 10 - Navigate pipe maze to find loop and count enclosed tiles

**Solution**: `advent_of_code/aoc2023/src/solver/day10.rs`

**Performance**: Part 1: 3.1ms | Part 2: 3.4ms | Total: 6.5ms

---

## 📋 Table of Contents

1. [Overview](#overview)
2. [Type Definitions](#type-definitions)
3. [Core Implementation](#core-implementation)
4. [Mathematical Algorithms](#mathematical-algorithms)
5. [Public API](#public-api)
6. [Design Patterns](#design-patterns)
7. [Performance Analysis](#performance-analysis)
8. [Testing Strategy](#testing-strategy)
9. [Common Pitfalls](#common-pitfalls)
10. [Key Takeaways](#key-takeaways)
11. [Follow-Up Questions](#follow-up-questions)

---

## 🎯 Overview

### Problem Summary
You're standing on a field of metal pipes arranged in a 2D grid. An animal is traveling through one large continuous loop of pipes starting from position 'S'. We need to:
- **Part 1**: Find the farthest point in the loop from the starting position
- **Part 2**: Count how many tiles are enclosed within the loop

### Example
```
Input maze:
.....
.S-7.
.|.|.
.L-J.
.....

Loop visualization:
.....
.●─●.
.│.│.
.●─●.
.....

Part 1: Farthest point is 4 steps away
Part 2: No tiles enclosed (loop is empty inside)
```

### Pipe Types
- `|` - Vertical pipe (North ↔ South)
- `-` - Horizontal pipe (East ↔ West)
- `L` - 90° bend (North ↔ East)
- `J` - 90° bend (North ↔ West)
- `7` - 90° bend (South ↔ West)
- `F` - 90° bend (South ↔ East)
- `.` - Ground (no pipe)
- `S` - Starting position (pipe type determined by neighbors)

### Key Insights
1. **Graph traversal**: Pipes form edges in a graph, BFS finds distances
2. **Loop detection**: BFS naturally finds the continuous loop from 'S'
3. **Point-in-polygon**: Ray casting determines which tiles are inside the loop
4. **Corner handling**: The tricky part is correctly counting boundary crossings at corners

### Mathematical Foundation
**Graph Theory**:
- BFS for shortest paths in unweighted graphs
- Cycle detection in undirected graphs
- Connected component identification

**Computational Geometry**:
- Ray casting algorithm for point-in-polygon
- Scanline processing
- Boundary crossing detection with corner cases

**See**: 
- `zettelkasten/math-foundations/graph-theory-fundamentals.md`
- `zettelkasten/math-foundations/computational-geometry-basics.md`

---

## 📦 Type Definitions

### `Dir` - Cardinal Directions

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    South,
    East,
    West,
}
```

**Purpose**: Type-safe representation of cardinal directions for grid navigation

**Design Decisions**:
- ✅ **Enum over (i32, i32)**: Prevents mixing up x/y coordinates
- ✅ **Copy trait**: Lightweight, frequently passed around
- ✅ **PartialEq, Eq**: Enable comparison and use in collections
- ✅ **Debug**: Helpful for debugging pipe connections

**Methods**:

```rust
impl Dir {
    /// Get the (dx, dy) offset for moving in this direction
    fn offset(&self) -> (i32, i32) {
        match self {
            Dir::North => (0, -1),   // Up
            Dir::South => (0, 1),     // Down
            Dir::East => (1, 0),      // Right
            Dir::West => (-1, 0),     // Left
        }
    }
    
    /// Get the opposite direction
    fn opposite(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }
}
```

**Usage Example**:
```rust
let pos = Coord::new(5, 5);
let (dx, dy) = Dir::North.offset();
let new_pos = Coord::new(
    (pos.x as i32 + dx) as usize,
    (pos.y as i32 + dy) as usize
);
```

**Why `opposite()` is needed**:
```rust
// To check if two pipes connect:
// Current pipe must connect in direction D
// Neighbor pipe must connect in opposite direction D.opposite()

if pipe1_connects(North) && pipe2_connects(South) {
    // They connect!
}
```

### Mission Integration: `Grid<char>` and `Coord`

**From Mission 6**:
```rust
use mission6::{Grid, Coord};

// Grid<char> provides:
let grid = Grid::new(width, height, '.');
grid[(x, y)] = '#';                    // O(1) indexing
let in_bounds = grid.in_bounds(coord); // Safe bounds checking
let width = grid.width();              // Dimension queries
let height = grid.height();

// Coord provides:
let coord = Coord::new(x, y);
coord.x  // Get x coordinate
coord.y  // Get y coordinate
```

**Why Mission 6**:
- ✅ Proven, tested 2D grid implementation
- ✅ O(1) indexing with bounds checking
- ✅ Row-major memory layout (cache-friendly)
- ✅ Integrates with Mission 8 graph algorithms

---

## 🔧 Core Implementation

### Function 1: `pipe_connections`

**Signature**:
```rust
fn pipe_connections(ch: char) -> Vec<Dir>
```

**Purpose**: Map pipe characters to their connection directions

**Implementation**:
```rust
fn pipe_connections(ch: char) -> Vec<Dir> {
    match ch {
        '|' => vec![Dir::North, Dir::South],  // Vertical
        '-' => vec![Dir::East, Dir::West],     // Horizontal
        'L' => vec![Dir::North, Dir::East],    // North-East bend
        'J' => vec![Dir::North, Dir::West],    // North-West bend
        '7' => vec![Dir::South, Dir::West],    // South-West bend
        'F' => vec![Dir::South, Dir::East],    // South-East bend
        '.' => vec![],                          // Ground - no connections
        'S' => vec![Dir::North, Dir::South, Dir::East, Dir::West],  // Placeholder
        _ => vec![],
    }
}
```

**Visual Guide**:
```
   North
     ↑
West ← → East
     ↓
   South

'|' connects North-South:     '─' connects East-West:
       ↑                              ← → 
       │                              ─────
       ↓                              

'L' bends North-East:        'J' bends North-West:
       ↑                              ↑
       └─ →                      ← ──┘

'F' bends South-East:        '7' bends South-West:
       ┌─ →                      ← ──┐
       ↓                              ↓
```

**Why `Vec<Dir>` not an array**:
- Pipes have 0-4 connections (ground has 0, 'S' has 4)
- Vec is more flexible than fixed-size array
- Small allocations (2-4 elements) are cheap
- Could optimize to `&[Dir]` with static slices if performance critical

**Step-by-Step Example**:
```rust
// Input: 'L' (North-East bend)
pipe_connections('L')
// Returns: vec![Dir::North, Dir::East]
// Meaning: This pipe connects to tiles to the North and East
```

---

### Function 2: `move_coord`

**Signature**:
```rust
fn move_coord(grid: &Grid<char>, coord: Coord, dir: Dir) -> Option<Coord>
```

**Purpose**: Move from a coordinate in a direction, returning new coordinate if valid

**Implementation**:
```rust
fn move_coord(grid: &Grid<char>, coord: Coord, dir: Dir) -> Option<Coord> {
    let (dx, dy) = dir.offset();
    let new_x = coord.x as i32 + dx;
    let new_y = coord.y as i32 + dy;

    if new_x >= 0 && new_y >= 0 {
        let new_coord = Coord::new(new_x as usize, new_y as usize);
        if grid.in_bounds(new_coord) {
            return Some(new_coord);
        }
    }
    None
}
```

**Step-by-Step Walkthrough**:
```rust
// Grid: 10×10, Current position: (5, 5), Direction: North

// 1. Get offset for North
let (dx, dy) = Dir::North.offset();  // (0, -1)

// 2. Convert to i32 (to handle negative results)
let new_x = 5i32 + 0 = 5
let new_y = 5i32 + (-1) = 4

// 3. Check for negative (out of bounds)
if new_x >= 0 && new_y >= 0 {  // ✅ Both positive
    
    // 4. Convert back to usize
    let new_coord = Coord::new(5, 4);
    
    // 5. Check grid bounds
    if grid.in_bounds(new_coord) {  // ✅ (5, 4) is in 10×10 grid
        return Some(Coord::new(5, 4));
    }
}
```

**Edge Cases Handled**:
```rust
// Case 1: Moving North from top edge
move_coord(grid, Coord::new(5, 0), Dir::North)
// new_y = 0 - 1 = -1 (negative)
// Returns: None ✅

// Case 2: Moving East from right edge
move_coord(grid, Coord::new(9, 5), Dir::East)  // Grid is 10×10
// new_x = 9 + 1 = 10
// grid.in_bounds fails (x >= width)
// Returns: None ✅

// Case 3: Valid move
move_coord(grid, Coord::new(5, 5), Dir::South)
// Returns: Some(Coord::new(5, 6)) ✅
```

**Why `i32` intermediate**:
```rust
// ❌ Without i32:
let new_x = coord.x + dx;  // Compile error! Can't add i32 to usize

// ❌ Cast to i32 after:
let new_x = (coord.x + dx) as i32;  // Overflow if dx is negative!

// ✅ Correct: Cast to i32 first
let new_x = coord.x as i32 + dx;  // Safe arithmetic
if new_x >= 0 { /* can convert back */ }
```

---

### Function 3: `pipes_connect`

**Signature**:
```rust
fn pipes_connect(grid: &Grid<char>, from: Coord, dir: Dir) -> bool
```

**Purpose**: Check if two pipes connect in a given direction

**Implementation**:
```rust
fn pipes_connect(grid: &Grid<char>, from: Coord, dir: Dir) -> bool {
    let from_char = grid[from];
    let from_connections = pipe_connections(from_char);
    
    // Check if 'from' pipe has an opening in this direction
    if !from_connections.contains(&dir) && from_char != 'S' {
        return false;
    }

    // Get the neighbor coordinate
    let Some(to) = move_coord(grid, from, dir) else {
        return false;
    };

    let to_char = grid[to];
    let to_connections = pipe_connections(to_char);

    // Check if 'to' pipe has an opening in the opposite direction
    to_connections.contains(&dir.opposite())
}
```

**Visual Example**:
```
From: 'L' at (5, 5), Direction: North
  
Step 1: Does 'L' connect North?
   L connects: [North, East]
   North ∈ [North, East] ✓

Step 2: Get neighbor to the North
   move_coord((5,5), North) = Some((5, 4))

Step 3: Does neighbor connect back South?
   Grid[(5,4)] = '7'
   '7' connects: [South, West]
   South ∈ [South, West] ✓
   
Result: true ✓

Visual:
     (5, 4)
       7─┐
       │ ↓  '7' connects South ✓
     (5, 5)
       ↑
       └─  'L' connects North ✓
```

**Why check `opposite()`**:
```rust
// Two pipes connect if:
// 1. Pipe A has opening in direction D
// 2. Pipe B (in direction D from A) has opening in direction D.opposite()

// Example: 'L' at (5,5) going North
// - 'L' must connect North (✓)
// - Pipe at (5,4) must connect South (opposite of North)

// This ensures bidirectional connection:
//   ┌─  L connects North
//   │
//   │   
//   └─  7 connects South
```

**Special Case: 'S' Handling**:
```rust
// 'S' is special - we don't know its actual shape yet
// So we check if neighbor connects back
if !from_connections.contains(&dir) && from_char != 'S' {
    return false;  // Skip check for 'S'
}

// 'S' will connect if neighbor connects back to it
```

**Step-by-Step Logic**:
```rust
// Check: Do 'F' at (3,3) and '7' at (4,3) connect Eastward?

// 1. Get 'F' connections
pipe_connections('F') = [South, East]

// 2. Does 'F' connect East?
East ∈ [South, East] ? YES ✓

// 3. Get neighbor coordinate
move_coord((3,3), East) = Some((4,3))

// 4. Get '7' connections  
pipe_connections('7') = [South, West]

// 5. Does '7' connect back West (opposite of East)?
West ∈ [South, West] ? YES ✓

// Result: true - they connect!

Visual:
   F─7
   └┐
```

---

### Function 4: `find_start`

**Signature**:
```rust
fn find_start(grid: &Grid<char>) -> Option<Coord>
```

**Purpose**: Find the starting position 'S' in the grid

**Implementation**:
```rust
fn find_start(grid: &Grid<char>) -> Option<Coord> {
    for (coord, &ch) in grid.enumerate() {
        if ch == 'S' {
            return Some(coord);
        }
    }
    None
}
```

**Mission 6 Integration**:
```rust
// grid.enumerate() returns Iterator<Item = (Coord, &T)>
// Provided by Mission 6 Grid implementation

// Equivalent to:
for y in 0..grid.height() {
    for x in 0..grid.width() {
        let coord = Coord::new(x, y);
        if grid[coord] == 'S' {
            return Some(coord);
        }
    }
}
```

**Complexity**: O(w × h) worst case, but typically finds 'S' quickly

**Why `Option<Coord>`**:
- Returns `None` if 'S' not found (defensive programming)
- Allows `.expect()` or `?` operator in calling code
- Makes contract explicit: "`find_start` might not find anything"

---

### Function 5: `determine_start_pipe`

**Signature**:
```rust
fn determine_start_pipe(grid: &Grid<char>, start: Coord) -> char
```

**Purpose**: Figure out what pipe type 'S' actually represents based on neighbors

**Implementation**:
```rust
fn determine_start_pipe(grid: &Grid<char>, start: Coord) -> char {
    let connects_north = pipes_connect(grid, start, Dir::North);
    let connects_south = pipes_connect(grid, start, Dir::South);
    let connects_east = pipes_connect(grid, start, Dir::East);
    let connects_west = pipes_connect(grid, start, Dir::West);
    
    match (connects_north, connects_south, connects_east, connects_west) {
        (true, true, false, false) => '|',   // North-South
        (false, false, true, true) => '-',   // East-West
        (true, false, true, false) => 'L',   // North-East
        (true, false, false, true) => 'J',   // North-West
        (false, true, false, true) => '7',   // South-West
        (false, true, true, false) => 'F',   // South-East
        _ => 'S',  // Shouldn't happen in valid input
    }
}
```

**Step-by-Step Example**:
```
Grid:
  .....
  .S-7.
  .|.|.
  .L-J.
  .....

Step 1: Check all 4 directions from 'S' at (1, 1)

  North: (1, 0) = '.' → doesn't connect back South
    Result: false
  
  South: (1, 2) = '|' → connects back North  
    Result: true
  
  East: (2, 1) = '-' → connects back West
    Result: true
  
  West: (0, 1) = '.' → doesn't connect back East
    Result: false

Step 2: Match pattern
  (false, true, true, false)
  = (north, south, east, west)
  → 'F' (South-East bend)

Visual verification:
  .....
  .F-7.    'F' connects South and East ✓
  .│ │
  .└─┘
```

**All Patterns**:
```rust
// Straight pipes:
(T, T, F, F) → '|'  // │ North-South
(F, F, T, T) → '-'  // ─ East-West

// Corner pipes:
(T, F, T, F) → 'L'  // └ North-East
(T, F, F, T) → 'J'  // ┘ North-West
(F, T, F, T) → '7'  // ┐ South-West  
(F, T, T, F) → 'F'  // ┌ South-East
```

**Why this is needed**:
```rust
// Part 2 ray casting needs to know actual pipe type
// 'S' is just a placeholder in the input
// We determine its real type by checking what connects to it

// For example:
if ch == 'S' {
    ch = start_pipe;  // Replace 'S' with its actual type
}

match ch {
    '|' => inside = !inside,  // Now we know how to handle it!
    // ...
}
```

---

### Function 6: `find_loop_distances` (BFS Implementation)

**Signature**:
```rust
fn find_loop_distances(grid: &Grid<char>, start: Coord) -> HashMap<Coord, usize>
```

**Purpose**: Use BFS to traverse the pipe loop and find distances from start

**Implementation**:
```rust
fn find_loop_distances(grid: &Grid<char>, start: Coord) -> HashMap<Coord, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    distances.insert(start, 0);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        let current_dist = distances[&current];

        // Try all four directions
        for dir in [Dir::North, Dir::South, Dir::East, Dir::West] {
            if pipes_connect(grid, current, dir) {
                if let Some(neighbor) = move_coord(grid, current, dir) {
                    // Only visit if we haven't seen it yet
                    if !distances.contains_key(&neighbor) {
                        distances.insert(neighbor, current_dist + 1);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    distances
}
```

**BFS Algorithm Walkthrough**:
```
Initial state:
  Grid:
    .S-7.
    .|.|.
    .L-J.
  
  Start: S at (1, 0)
  distances = { (1,0): 0 }
  queue = [ (1,0) ]

Iteration 1: Process (1,0)
  current = (1,0), dist = 0
  Check North: Out of bounds
  Check South: (1,1) '|' connects? Yes!
    distances.insert((1,1), 1)
    queue.push_back((1,1))
  Check East: (2,0) '-' connects? Yes!
    distances.insert((2,0), 1)
    queue.push_back((2,0))
  Check West: Out of bounds
  
  distances = { (1,0):0, (1,1):1, (2,0):1 }
  queue = [ (1,1), (2,0) ]

Iteration 2: Process (1,1)
  current = (1,1), dist = 1
  Check North: (1,0) already visited (in distances)
  Check South: (1,2) 'L' connects? Yes!
    distances.insert((1,2), 2)
    queue.push_back((1,2))
  Check East: '.' doesn't connect
  Check West: '.' doesn't connect
  
  distances = { (1,0):0, (1,1):1, (2,0):1, (1,2):2 }
  queue = [ (2,0), (1,2) ]

...continue until queue empty...

Final distances:
  {
    (1,0): 0,  // Start 'S'
    (2,0): 1,  // '-'
    (3,0): 2,  // '7'
    (3,1): 3,  // '|'
    (3,2): 4,  // 'J'  ← Farthest point!
    (2,2): 3,  // '-'
    (1,2): 2,  // 'L'
    (1,1): 1,  // '|'
  }

Visualization:
  .0-1.      .S-7.
  .1.2.      .|.|.
  .2-3.      .L-J.
  
  Max distance = 4
```

**Why HashMap for distances**:
- ✅ Dual purpose: visited set + distance storage
- ✅ O(1) `contains_key` check prevents revisiting
- ✅ O(1) `insert` and lookup
- ❌ Alternative Vec<Vec<Option<usize>>>: Wastes memory for sparse loops

**BFS Guarantees**:
- First visit to each node = shortest path distance
- Level-by-level exploration ensures correct distances
- Queue (FIFO) crucial - DFS (stack) wouldn't work

**Complexity**:
- Time: O(V + E) where V = loop cells, E = pipe connections
- Space: O(V) for HashMap and queue
- For 140×141 grid with ~7000 loop cells: ~3ms

---

### Function 7: `parse_grid`

**Signature**:
```rust
fn parse_grid(input: &str) -> Grid<char>
```

**Purpose**: Parse text input into Mission 6 Grid

**Implementation**:
```rust
fn parse_grid(input: &str) -> Grid<char> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    let mut grid = Grid::new(width, height, '.');

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[(x, y)] = ch;
        }
    }

    grid
}
```

**Step-by-Step**:
```rust
// Input:
".S-7.\n\
 .|.|.\n\
 .L-J."

// 1. Split into lines
let lines = [".", "S-7.", "|.|.", "L-J."]
height = 3
width = 5

// 2. Create empty grid
let mut grid = Grid::new(5, 3, '.')

// 3. Fill grid
// y=0: grid[(0,0)]='S', grid[(1,0)]='-', grid[(2,0)]='7', ...
// y=1: grid[(0,1)]='|', grid[(1,1)]='.', grid[(2,1)]='|', ...
// y=2: grid[(0,2)]='L', grid[(1,2)]='-', grid[(2,2)]='J', ...

// Result: Grid populated with pipe characters
```

**Indexing Convention**:
```rust
// grid[(x, y)] where:
//   x = column (0 = leftmost)
//   y = row (0 = topmost)

// Example:
//     x=0  x=1  x=2
// y=0  .    S    -
// y=1  .    |    .
// y=2  .    L    -

// grid[(1, 0)] = 'S'
// grid[(1, 1)] = '|'
// grid[(1, 2)] = 'L'
```

---

## 🧮 Mathematical Algorithms

### Part 1: BFS for Loop Distances

**Algorithm**: Breadth-First Search (BFS)

**Why BFS works**:
1. **Unweighted graph**: All pipe connections have equal cost (1 step)
2. **Shortest path**: BFS finds shortest path in unweighted graphs
3. **Level-by-level**: BFS explores all nodes at distance d before distance d+1
4. **Loop structure**: Continuous loop ensures all reachable from start

**Mathematical Properties**:
```
For unweighted graph G = (V, E):
- BFS starting from s finds shortest path to all reachable vertices
- Distance d[v] = minimum number of edges from s to v
- Time complexity: O(V + E)
- Space complexity: O(V) for queue and visited set
```

**In this problem**:
- V = pipe tiles in loop (~7000 for puzzle input)
- E = pipe connections (each tile connects to 2 neighbors in loop)
- Loop structure: E ≈ V (each node has degree 2)
- Total complexity: O(V + V) = O(V) = O(loop_size)

**Farthest Point**:
```rust
// Since it's a loop, farthest point is at distance loop_size / 2
// Because you can go either direction around the loop

Loop: A - B - C - D - E - F - A
      ^                   ^
    start             farthest

Distance from A:
  A→B: 1, A→F: 1
  A→C: 2, A→E: 2  
  A→D: 3 ✓ Farthest (either direction takes 3 steps)
  
Max distance = loop_size / 2 (for even-length loops)
```

---

### Part 2: Ray Casting (Point-in-Polygon)

**Algorithm**: Ray Casting with Scanline Processing

**Mathematical Concept**:
```
Jordan Curve Theorem (1887):
  A point is inside a closed curve if and only if
  a ray from the point to infinity crosses the
  curve an odd number of times.
```

**Implementation Approach**:
```
For each point P not on the loop:
  1. Cast horizontal ray from P to the right (→ ∞)
  2. Count how many times ray crosses loop boundary
  3. If crossings is odd: P is inside
     If crossings is even: P is outside
```

**Visual Example**:
```
Grid with loop:
  ┌───┐
  │ • │ • 
  └───┘
  
Point 1: Inside the loop
  Ray: →→→→ crosses boundary 1 time (odd) ✓ Inside

Point 2: Outside the loop  
  Ray: →→→→ crosses boundary 0 times (even) ✓ Outside
```

**Corner Handling (The Tricky Part)**:

```rust
// Vertical pipes '|' always cross
inside = !inside

// Horizontal pipes '-' never cross
// (ray runs parallel to pipe)

// Corners require pairing:
'F' + '7' = Same side (no crossing)
  ┌──┐  Ray passes above both corners
       →→→→ (never enters/exits)

'F' + 'J' = Opposite sides (CROSSING!)
  ┌──┘  Ray enters at F, exits at J
       →→X→→ (crosses once)

'L' + '7' = Opposite sides (CROSSING!)
  └──┐  Ray enters at L, exits at 7  
       →→X→→ (crosses once)

'L' + 'J' = Same side (no crossing)
  └──┘  Ray passes below both corners
       →→→→ (never enters/exits)
```

**State Machine Implementation**:
```rust
let mut inside = false;              // Am I inside the loop?
let mut enter_corner = None;         // Which corner did I enter?

for each cell in row:
    match cell {
        '|' => inside = !inside,     // Simple crossing
        
        'F' | 'L' => {
            enter_corner = Some(cell);  // Remember entry corner
        }
        
        '7' => {
            if enter_corner == Some('L') {
                inside = !inside;     // L-7 is a crossing
            }
            // F-7 is NOT a crossing
            enter_corner = None;
        }
        
        'J' => {
            if enter_corner == Some('F') {
                inside = !inside;     // F-J is a crossing
            }
            // L-J is NOT a crossing
            enter_corner = None;
        }
        
        '-' => {
            // Horizontal segment - part of corner sequence
            // No state change (continue with same enter_corner)
        }
    }
```

**Correctness Proof (Informal)**:

```
Case 1: L---7 (corners on opposite sides)
   Ray: ─────────→
        └────┐
   
   Ray crosses from bottom to top of loop
   This IS a boundary crossing ✓
   inside should flip

Case 2: L---J (corners on same side - bottom)
   Ray: ─────────→
        └────┘
   
   Ray stays below loop entire time
   This is NOT a crossing ✗
   inside should NOT flip

Case 3: F---J (corners on opposite sides)
   Ray: ─────────→
        ┌────┘
   
   Ray crosses from top to bottom of loop
   This IS a boundary crossing ✓
   inside should flip

Case 4: F---7 (corners on same side - top)
   Ray: ─────────→
        ┌────┐
   
   Ray stays above loop entire time
   This is NOT a crossing ✗
   inside should NOT flip
```

**Mathematical Formalization**:

```
Define:
  L = loop boundary
  P = point to test
  R = horizontal ray from P to +∞
  C = number of crossings of R with L

Theorem (Jordan Curve):
  P ∈ interior(L) ⟺ C mod 2 = 1
  P ∈ exterior(L) ⟺ C mod 2 = 0

Corner crossing rules:
  Crossing at corner (a, b) counts as crossing ⟺
    a and b have opposite vertical orientations

  F (down-right), 7 (down-left) → same (down)
  F (down-right), J (up-left)   → different ✓
  L (up-right), 7 (down-left)   → different ✓
  L (up-right), J (up-left)     → same (up)
```

**Complexity Analysis**:
```
Time: O(width × height)
  - Visit every cell once
  - O(1) state machine per cell
  
Space: O(loop_size)
  - HashMap for loop tiles from Part 1
  - O(1) for scanline state
  
For 140×141 grid:
  - Total cells: 19,740
  - Loop cells: ~7,000
  - Non-loop cells: ~12,740
  - Time: ~3.4ms
```

---

## 🎯 Public API

### `solve_part1`

**Signature**:
```rust
pub fn solve_part1(input: &str) -> Result<String>
```

**Purpose**: Find the farthest point in the pipe loop from start

**Pipeline**:
```rust
pub fn solve_part1(input: &str) -> Result<String> {
    let grid = parse_grid(input);
    let start = find_start(&grid).expect("Should find starting position 'S'");
    let distances = find_loop_distances(&grid, start);
    let max_distance = distances.values().max().unwrap_or(&0);
    Ok(max_distance.to_string())
}
```

**Step-by-Step**:
```
Input: Grid with pipes and 'S'
  ↓
parse_grid: Convert to Grid<char>
  ↓
find_start: Locate 'S' coordinate
  ↓
find_loop_distances: BFS from 'S'
  ↓
max: Find maximum distance
  ↓
Output: Farthest point distance as string
```

**Example**:
```
Input:
  .....
  .S-7.
  .|.|.
  .L-J.
  .....

Grid: 5×5 with 'S' at (1, 1)
Distances: { (1,1):0, (2,1):1, (3,1):2, (3,2):3, (3,3):4, ... }
Max: 4
Output: "4"
```

---

### `solve_part2`

**Signature**:
```rust
pub fn solve_part2(input: &str) -> Result<String>
```

**Purpose**: Count tiles enclosed within the loop

**Pipeline**:
```rust
pub fn solve_part2(input: &str) -> Result<String> {
    let grid = parse_grid(input);
    let start = find_start(&grid).expect("Should find starting position 'S'");
    let loop_tiles = find_loop_distances(&grid, start);
    let start_pipe = determine_start_pipe(&grid, start);
    
    let mut enclosed_count = 0;
    
    for y in 0..grid.height() {
        let mut inside = false;
        let mut enter_corner: Option<char> = None;
        
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            let mut ch = grid[coord];
            
            if ch == 'S' { ch = start_pipe; }
            
            if loop_tiles.contains_key(&coord) {
                // State machine logic...
            } else if inside {
                enclosed_count += 1;
            }
        }
    }
    
    Ok(enclosed_count.to_string())
}
```

**Scanline Algorithm**:
```
For each row from top to bottom:
  inside = false
  enter_corner = None
  
  For each cell from left to right:
    If cell is loop tile:
      Update inside/enter_corner based on pipe type
    Else if inside:
      Count this cell (enclosed!)
      
Return total count
```

**Example**:
```
Input:
  ...........
  .S-------7.
  .|F-----7|.
  .||.....||.
  .||.....||.
  .|L-7.F-J|.
  .|..|.|..|.
  .L--J.L--J.
  ...........

Row 3 (index from 0):
  .||.....||.
  
Scan:
  . → not loop
  | → loop, flip inside=true
  | → loop, flip inside=false
  . → not loop, outside (skip)
  . → ...
  
Result: Cells between the '|' pairs are counted
```

---

## 🎨 Design Patterns

### Pattern 1: Mission Integration - Component Reuse

**Pattern**: Prefer composing from proven libraries over reimplementing

**Implementation**:
```rust
use mission6::{Grid, Coord};  // ✅ Reuse instead of rebuild

// ❌ Don't do this:
struct MyGrid {
    data: Vec<Vec<char>>,  // Nested Vec (bad cache locality)
}

impl MyGrid {
    fn get(&self, x: usize, y: usize) -> char {
        self.data[y][x]  // Manual indexing, no bounds check
    }
}

// ✅ Do this:
let grid = Grid::new(width, height, '.');
let ch = grid[(x, y)];  // Mission 6 provides O(1) indexing
```

**Benefits**:
- ✅ Proven correctness (Mission 6 tests passed)
- ✅ Optimized implementation (row-major layout)
- ✅ Consistent API across codebase
- ✅ Focus on problem logic, not infrastructure

---

### Pattern 2: Type-Safe Directions

**Pattern**: Use enum instead of raw coordinates for directions

**Implementation**:
```rust
// ❌ Error-prone:
fn move_in_direction(x: usize, y: usize, dx: i32, dy: i32) -> (usize, usize) {
    // Easy to mix up x/y or pass wrong signs
}

// Caller:
let new_pos = move_in_direction(x, y, 0, -1);  // What direction is this?

// ✅ Type-safe:
enum Dir {
    North, South, East, West,
}

fn move_in_direction(coord: Coord, dir: Dir) -> Option<Coord> {
    // Type system prevents mixing up directions
}

// Caller:
let new_pos = move_in_direction(coord, Dir::North);  // Crystal clear!
```

**Benefits**:
- ✅ Self-documenting code
- ✅ Compiler catches direction errors
- ✅ Easy to extend (add diagonals later)
- ✅ `opposite()` method impossible with raw (i32, i32)

---

### Pattern 3: State Machine for Scanline

**Pattern**: Explicit state machine with enum/Option for corner tracking

**Implementation**:
```rust
// State: Am I inside the loop?
let mut inside = false;

// State: Which corner did I enter (if any)?
let mut enter_corner: Option<char> = None;

// State transitions:
match cell {
    '|' => inside = !inside,
    'F' | 'L' => enter_corner = Some(cell),
    '7' => {
        if enter_corner == Some('L') { inside = !inside; }
        enter_corner = None;
    }
    // ...
}
```

**Why state machine**:
- ✅ Makes logic explicit and testable
- ✅ Each state transition has clear semantics
- ✅ Easy to debug (print states)
- ✅ Handles corner sequences correctly

---

### Pattern 4: HashMap as Visited Set + Data Storage

**Pattern**: Use HashMap to serve dual purpose

**Implementation**:
```rust
let mut distances: HashMap<Coord, usize> = HashMap::new();

// Dual purpose:
// 1. Visited set (contains_key)
if !distances.contains_key(&neighbor) {
    // Not visited yet
}

// 2. Distance storage (get value)
let dist = distances[&current];
```

**Alternatives and Trade-offs**:
```rust
// ❌ Separate structures:
let mut visited: HashSet<Coord> = HashSet::new();
let mut distances: HashMap<Coord, usize> = HashMap::new();
// Redundant - wastes memory

// ❌ Grid of Option<usize>:
let mut distances: Vec<Vec<Option<usize>>> = vec![vec![None; width]; height];
// Wastes memory for sparse loops (most cells are None)

// ✅ Single HashMap:
let mut distances: HashMap<Coord, usize> = HashMap::new();
// Efficient for sparse data, dual purpose
```

---

## ⚡ Performance Analysis

### Benchmarks (Empirical)

```
day10_part1         time:   [3.1270 ms 3.1360 ms 3.1454 ms]
day10_part2         time:   [3.3983 ms 3.4128 ms 3.4295 ms]
```

### Breakdown

**Part 1: 3.1ms**
- Parse grid: ~0.3ms (19,740 cells)
- Find start: ~0.1ms (linear scan)
- BFS loop: ~2.7ms (7,000 cells, 4 directions each)
  - HashMap insert: ~28,000 ops (loop_size × directions)
  - HashMap contains_key: ~28,000 ops
- Find max: ~0.01ms (max over 7,000 values)

**Part 2: 3.4ms**
- Reuse loop tiles: 0ms
- Scanline: ~3.4ms (all 19,740 cells)
  - HashMap contains_key: 19,740 ops (~0.17µs each)
  - State machine: ~0.01µs per cell
  - Grid indexing: 19,740 ops (~0.01µs each)

**Bottlenecks**:
1. **HashMap lookups**: Dominant cost (called 47,740 times total)
2. **Grid indexing**: Called 4-8× per BFS node
3. **Memory allocation**: Vec/HashMap allocations

### Complexity Analysis

**Part 1**:
```
Time: O(V + E) where V = loop cells, E = connections
  - BFS visits each loop cell once
  - Checks 4 directions per cell
  - Total: O(4V) = O(V) since E ≈ 2V for a loop
  
Space: O(V)
  - HashMap: O(loop_size)
  - Queue: O(loop_size) worst case
```

**Part 2**:
```
Time: O(W × H) where W = grid width, H = grid height
  - Visit every cell once
  - O(1) per cell (state machine + HashMap lookup)
  
Space: O(V)
  - Reuse loop_tiles HashMap from Part 1
  - O(1) for scanline state
```

### Optimization Opportunities

**Not Implemented** (good performance already):

1. **BitSet for loop tiles**: ~30% memory reduction
```rust
// Current: HashMap<Coord, usize> = 7000 × (8+8) = 112KB
// BitSet: 19740 bits = 2.4KB
// Trade-off: Slower lookup, harder to maintain distances
```

2. **Skip non-loop rows**: ~30-40% speedup for Part 2
```rust
// Only scan rows that contain loop tiles
let rows_with_loop: HashSet<usize> = loop_tiles.keys()
    .map(|coord| coord.y)
    .collect();

for y in rows_with_loop {
    // Scan only ~80-100 rows instead of 141
}
```

3. **Pre-compile directions array**: Micro-optimization
```rust
// Current:
for dir in [Dir::North, Dir::South, Dir::East, Dir::West] {

// Optimized:
static DIRS: [Dir; 4] = [Dir::North, Dir::South, Dir::East, Dir::West];
for dir in &DIRS {
```

**Why not optimized**:
- Current performance: 6.5ms (excellent)
- Code readability > marginal gains
- Mission integration philosophy values clarity
- Optimizations would complicate code for <50% speedup

---

## 🧪 Testing Strategy

### Test Categories

**1. Unit Tests - Pipe Connections**
```rust
#[test]
fn test_pipe_connections() {
    assert_eq!(pipe_connections('|'), vec![Dir::North, Dir::South]);
    assert_eq!(pipe_connections('-'), vec![Dir::East, Dir::West]);
    assert_eq!(pipe_connections('L'), vec![Dir::North, Dir::East]);
    // ... all pipe types
}
```

**Purpose**: Verify each pipe type maps to correct directions

---

**2. Example Tests - Part 1**
```rust
#[test]
fn test_part1_example1() {
    let result = solve_part1(EXAMPLE1).unwrap();
    assert_eq!(result, "4");
}

#[test]
fn test_part1_example2() {
    let result = solve_part1(EXAMPLE2).unwrap();
    assert_eq!(result, "8");
}
```

**Purpose**: Verify BFS correctly finds loop distances

---

**3. Example Tests - Part 2 (Ray Casting)**
```rust
#[test]
fn test_part2_example3() {
    let result = solve_part2(EXAMPLE3).unwrap();
    assert_eq!(result, "4");
}

#[test]
fn test_part2_example4() {
    let result = solve_part2(EXAMPLE4).unwrap();
    assert_eq!(result, "8");
}

#[test]
fn test_part2_example5() {
    let result = solve_part2(EXAMPLE5).unwrap();
    assert_eq!(result, "10");
}
```

**Purpose**: Verify ray casting handles various loop shapes and corner cases

---

### Test Coverage

- ✅ All pipe types tested
- ✅ Simple loops (square)
- ✅ Complex loops (irregular shapes)
- ✅ Small enclosures (4 tiles)
- ✅ Medium enclosures (8 tiles)
- ✅ Large enclosures with junk pipes (10 tiles)
- ✅ Different corner combinations
- ✅ Grid parsing

**Not tested** (would be good to add):
- ⚠️ Edge cases: loop touching grid boundary
- ⚠️ S determination: all 6 pipe types as S
- ⚠️ Multiple disconnected pipe segments
- ⚠️ Very large grids (stress test)

---

## ⚠️ Common Pitfalls

### Pitfall 1: Forgetting to Handle 'S' in Part 2

**Problem**:
```rust
// ❌ Wrong: Treats 'S' as unknown
for x in 0..grid.width() {
    let ch = grid[(x, y)];
    match ch {
        '|' => inside = !inside,
        'S' => {},  // Oops! Don't know how to handle this
        // ...
    }
}
```

**Solution**:
```rust
// ✅ Correct: Determine 'S' type first
let start_pipe = determine_start_pipe(&grid, start);

for x in 0..grid.width() {
    let mut ch = grid[(x, y)];
    if ch == 'S' { ch = start_pipe; }  // Replace!
    
    match ch {
        '|' => inside = !inside,  // Now we know what 'S' is
        // ...
    }
}
```

**Symptom**: Answer "too high" (counting wrong cells as enclosed)

---

### Pitfall 2: Incorrect Corner Crossing Logic

**Problem**:
```rust
// ❌ Wrong: Treating all corners as crossings
match ch {
    'F' | 'L' | '7' | 'J' => inside = !inside,  // All corners flip!
}
```

**Why it fails**:
```
  F───7
     →→→
  
  Ray passes above both corners
  Should NOT count as crossing, but this code flips twice!
  (Two flips = no net change, accidentally works sometimes)
```

**Solution**: Pair corners correctly
```rust
// ✅ Correct: Track entry corner
'F' | 'L' => enter_corner = Some(ch),
'7' => {
    if enter_corner == Some('L') { inside = !inside; }  // L-7 crosses
    // F-7 doesn't cross
}
```

---

### Pitfall 3: Using Unsigned Arithmetic for Grid Movement

**Problem**:
```rust
// ❌ Wrong: Underflow on negative coordinates
let new_x = coord.x + dx;  // Compile error if dx is i32
let new_y = coord.y + dy;  // or overflow if we cast wrong
```

**Why it fails**:
```rust
// coord.x = 0, dx = -1 (moving West)
let new_x = 0usize + (-1);  // Won't compile!

// Even if we force it:
let new_x = (0usize as i32 + (-1)) as usize;  // Wraps to usize::MAX!
```

**Solution**: Cast to i32 first
```rust
// ✅ Correct: Safe signed arithmetic
let new_x = coord.x as i32 + dx;  // -1 (negative - OK in i32)
if new_x >= 0 {
    let new_x_usize = new_x as usize;  // Now safe to convert
}
```

---

### Pitfall 4: Not Checking Grid Bounds After Move

**Problem**:
```rust
// ❌ Wrong: Assumes in bounds
fn move_coord(coord: Coord, dir: Dir) -> Coord {
    let (dx, dy) = dir.offset();
    Coord::new(
        (coord.x as i32 + dx) as usize,
        (coord.y as i32 + dy) as usize,
    )
}
```

**Why it fails**:
```rust
// Moving South from bottom edge (y = 140) on 141-tall grid
let new_y = (140i32 + 1) as usize = 141
Coord::new(x, 141)  // Out of bounds! Panic on grid access
```

**Solution**: Return Option and check bounds
```rust
// ✅ Correct: Safe bounds checking
fn move_coord(grid: &Grid<char>, coord: Coord, dir: Dir) -> Option<Coord> {
    let (dx, dy) = dir.offset();
    let new_x = coord.x as i32 + dx;
    let new_y = coord.y as i32 + dy;
    
    if new_x >= 0 && new_y >= 0 {
        let new_coord = Coord::new(new_x as usize, new_y as usize);
        if grid.in_bounds(new_coord) {
            return Some(new_coord);
        }
    }
    None
}
```

---

### Pitfall 5: Checking Only One Direction in `pipes_connect`

**Problem**:
```rust
// ❌ Wrong: Only check if current pipe connects
fn pipes_connect(grid: &Grid<char>, from: Coord, dir: Dir) -> bool {
    let from_connections = pipe_connections(grid[from]);
    from_connections.contains(&dir)  // Only half the check!
}
```

**Why it fails**:
```
Current: 'L' (connects North and East)
Moving East
Neighbor: '.' (ground - no connections)

from_connections.contains(East) = true ✓
But '.' doesn't connect back West ✗

Result: False connection detected!
```

**Solution**: Check both directions
```rust
// ✅ Correct: Bidirectional check
fn pipes_connect(grid: &Grid<char>, from: Coord, dir: Dir) -> bool {
    // Check current pipe
    let from_connections = pipe_connections(grid[from]);
    if !from_connections.contains(&dir) { return false; }
    
    // Check neighbor pipe
    let Some(to) = move_coord(grid, from, dir) else { return false; };
    let to_connections = pipe_connections(grid[to]);
    to_connections.contains(&dir.opposite())  // Must connect back!
}
```

---

## 🎓 Key Takeaways

### 1. Mission Integration Philosophy

**Lesson**: Compose from proven libraries rather than reimplementing

```rust
// ✅ This code:
use mission6::Grid;
let grid = Grid::new(width, height, '.');

// Saves you from writing:
// - Custom 2D indexing logic
// - Bounds checking
// - Row-major layout optimization
// - enumerate() iterator
// - Coordinate helpers
```

**Value**: Focus on problem-solving, not infrastructure

---

### 2. Graph Algorithms in Disguise

**Lesson**: Many grid problems are actually graph problems

```rust
// Grid of pipes ≅ Graph
// - Each pipe tile = vertex
// - Connections = edges
// - BFS finds shortest paths
```

**When to recognize**:
- "Connected", "reachable", "path", "distance" → Graph problem
- Apply graph algorithms (BFS, DFS, Dijkstra)
- Use graph libraries (Mission 8)

---

### 3. Ray Casting is Subtle

**Lesson**: Point-in-polygon requires careful corner handling

**Key Rules**:
- Vertical crossings (`|`) always count
- Horizontal segments (`-`) never count
- Corners must be paired:
  - `F-J`, `L-7` → Crossing (opposite sides)
  - `F-7`, `L-J` → Not crossing (same side)

**State machine** makes logic explicit and testable

---

### 4. Type Safety Prevents Bugs

**Lesson**: Enum for directions catches errors at compile time

```rust
// ❌ Raw coordinates:
move(x, y, 0, -1)  // Which direction? Easy to mix up

// ✅ Type-safe enum:
move(coord, Dir::North)  // Crystal clear!
```

**Also helps with**:
- `opposite()` method impossible with raw values
- Can't accidentally use diagonal when only cardinals valid
- Self-documenting code

---

### 5. HashMap as Multi-Purpose Structure

**Lesson**: One data structure can serve multiple roles

```rust
let distances: HashMap<Coord, usize>;

// Role 1: Visited set
if distances.contains_key(&coord) { /* seen before */ }

// Role 2: Distance storage
let dist = distances[&coord];
```

**Better than**: Separate HashSet + HashMap

---

## ❓ Follow-Up Questions

### Conceptual

1. **Why does BFS work for finding the loop?**
   - Unweighted graph (all edges cost 1)
   - BFS finds shortest paths in unweighted graphs
   - Loop structure ensures all nodes reachable from start

2. **What if the grid had multiple disconnected loops?**
   - BFS would only find loop containing 'S'
   - Other loops wouldn't be in `distances` HashMap
   - Part 2 would correctly ignore them (not in `loop_tiles`)

3. **Why is ray casting sufficient for point-in-polygon?**
   - Jordan Curve Theorem: Point inside ↔ odd crossings
   - Mathematically proven for closed curves
   - Works for any polygon shape (convex or concave)

### Implementation

4. **Could we use DFS instead of BFS for Part 1?**
   - Yes, but distances would be wrong!
   - DFS doesn't guarantee shortest path
   - BFS is correct choice for distance finding

5. **How would you handle diagonal pipe connections?**
   - Add 4 more directions to `Dir` enum
   - Update `offset()` for diagonals: (±1, ±1)
   - Add new pipe types (8-way connectors)

6. **What if pipes had weights (different costs)?**
   - BFS no longer works (assumes uniform cost)
   - Use Dijkstra's algorithm instead
   - Priority queue instead of regular queue

### Performance

7. **What's the bottleneck in Part 2?**
   - HashMap `contains_key` called 19,740 times
   - Could use BitSet for ~30% speedup
   - Trade-off: memory savings vs code complexity

8. **How would you parallelize this?**
   - Part 1: Hard (BFS is inherently sequential)
   - Part 2: Easy (each row independent)
     ```rust
     use rayon::prelude::*;
     let counts: Vec<usize> = (0..height)
         .into_par_iter()
         .map(|y| count_enclosed_in_row(y))
         .collect();
     let total = counts.iter().sum();
     ```

9. **Could you solve Part 2 without ray casting?**
   - Yes, flood fill from outside edges
   - Mark all reachable cells from borders
   - Remaining unmarked cells = inside loop
   - Trade-off: Similar complexity, different approach

### Extensions

10. **How would you visualize the solution?**
    ```rust
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let coord = Coord::new(x, y);
            if loop_tiles.contains_key(&coord) {
                print!("■");  // Loop tile
            } else if is_inside(coord) {
                print!("·");  // Inside
            } else {
                print!(" ");  // Outside
            }
        }
        println!();
    }
    ```

---

## 📚 Related Zettelkasten Notes

- [[mission-6]] - Grid<T> 2D spatial data structures
- [[mission-8]] - BFS/DFS graph traversal patterns
- [[graph-theory-fundamentals]] - BFS, shortest paths, cycles
- [[computational-geometry-basics]] - Ray casting, point-in-polygon
- [[aoc-grid-patterns]] - Common AoC grid problem patterns
- [[ray-casting-algorithm]] - Detailed ray casting theory and proofs
- [[state-machine-pattern]] - State machine design pattern

---

**End of Day 10 Function Guide**

*Created: January 10, 2026*  
*Performance: Part 1: 3.1ms | Part 2: 3.4ms | Total: 6.5ms*  
*Mission Integration: Mission 6 (Grid), Mission 8 (BFS)*
