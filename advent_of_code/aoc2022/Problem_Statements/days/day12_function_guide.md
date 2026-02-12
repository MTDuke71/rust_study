# Day 12 Function Guide - Hill Climbing Algorithm

**Problem**: [Day 12](day12.md) | **Code**: [day12.rs](../../src/solver/day12.rs)

---

## 📋 Overview

**Concept**: Find the shortest path on a heightmap grid with elevation climbing constraints.

**Key Challenges**:
- Part 1: Shortest path from 'S' (elevation 'a') to 'E' (elevation 'z')
- Part 2: Shortest path from ANY 'a' elevation to 'E'
- **Constraint**: Can only climb UP by at most 1 elevation (but can descend any amount)
- **Critical insight**: Part 2 is embarrassingly parallel - each starting position is independent

**Performance**:
- Part 1 (sequential): ~0.2ms (single BFS)
- Part 2 (sequential): 29.09ms (multiple BFS from all 'a' positions)
- Part 2 (parallel): 3.07ms (rayon parallel BFS)
- **Combined (with parallel)**: 3.33ms (9.5× speedup! ✅)

---

## 🎯 Algorithm Analysis

### Part 1: Single-Source BFS

**Approach**: Breadth-First Search from start position to end
```rust
BFS from S to E:
    queue = [(start, distance=0)]
    visited = set()
    
    while queue not empty:
        (pos, dist) = dequeue()
        if pos == end: return dist
        
        for neighbor in valid_neighbors(pos):
            if not visited[neighbor]:
                visited[neighbor] = true
                enqueue((neighbor, dist + 1))
```

**Why BFS?**
- Guarantees shortest path in unweighted graphs
- First time we reach 'E' is the shortest path
- Each edge has weight 1 (one step)

**Complexity**: O(rows × cols)
- Visit each cell at most once
- Queue operations: O(1) with VecDeque
- Neighbor checks: O(4) per cell (up/down/left/right)
- Total: O(rows × cols × 4) = O(grid_size)

### Part 2: Multi-Source BFS

**Problem**: Find shortest path from ANY 'a' elevation to 'E'

**Naive Approach** (sequential):
```rust
min_steps = MAX
for each 'a' position:
    if path_exists = BFS(a_pos, end):
        min_steps = min(min_steps, path_length)
```

**Complexity**: O(num_a_positions × rows × cols)
- Typically ~20-50 'a' positions in input
- Each runs full BFS
- Sequential: 29.09ms

**Optimized Approach** (parallel with rayon):
```rust
// Collect all 'a' positions
start_positions = collect_all_a_positions()

// Run BFS in parallel, find minimum
min_steps = start_positions
    .par_iter()  // Parallel iterator!
    .filter_map(|start| BFS(start, end))
    .min()
```

**Complexity**: Same O(num_a × grid) but parallelized
- Each BFS is independent (no shared mutable state)
- Perfect for parallelization
- Rayon automatically distributes work across cores
- Result: 3.07ms (9.5× speedup on typical hardware)

---

## 🔧 Function Breakdown

### 1. `Pos` Struct

**Purpose**: Represent a grid position

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize,
}
```

**Why `Copy`?**: Small struct, frequently passed around in BFS
**Why `PartialEq`?**: Need to compare positions for goal check

### 2. `HeightMap` Struct

**Purpose**: Store the grid and metadata

```rust
pub struct HeightMap {
    grid: Vec<Vec<u8>>,  // Heights 0-25 ('a'-'z')
    start: Pos,          // 'S' position
    end: Pos,            // 'E' position
    rows: usize,
    cols: usize,
}
```

**Design choices**:
- `Vec<Vec<u8>>`: Simple 2D representation, cache-friendly for row access
- `u8` for heights: 'a'=0 to 'z'=25 fits in one byte
- Pre-compute rows/cols: Avoid repeated `.len()` calls in hot loops

### 3. `HeightMap::neighbors()`

**Purpose**: Generate valid neighbors respecting elevation constraint

```rust
fn neighbors(&self, pos: Pos) -> Vec<Pos> {
    let mut result = Vec::new();
    let current_height = self.height_at(pos);

    // Check all 4 directions
    if pos.row > 0 {
        let next = Pos { row: pos.row - 1, col: pos.col };
        // Can move if next height <= current + 1
        if self.height_at(next) <= current_height + 1 {
            result.push(next);
        }
    }
    // ... repeat for down, left, right
    
    result
}
```

**Key insight**: Elevation constraint
- **Can climb UP by at most 1**: `next_height <= current_height + 1`
- **Can descend any amount**: No lower bound check
- Examples:
  - 'a' (0) → 'b' (1): ✅ (climb 1)
  - 'a' (0) → 'c' (2): ❌ (climb 2, too steep)
  - 'z' (25) → 'a' (0): ✅ (descend 25, allowed)

**Performance**:
- Bounds checks prevent out-of-bounds access
- Returns `Vec<Pos>`: Typically 2-4 neighbors
- Could be optimized with array/iterator, but not bottleneck

### 4. `bfs_shortest_path()`

**Purpose**: Find shortest path between two positions

```rust
fn bfs_shortest_path(map: &HeightMap, start: Pos, end: Pos) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; map.cols]; map.rows];
    
    queue.push_back((start, 0));
    visited[start.row][start.col] = true;

    while let Some((pos, dist)) = queue.pop_front() {
        if pos == end {
            return Some(dist);
        }

        for next in map.neighbors(pos) {
            if !visited[next.row][next.col] {
                visited[next.row][next.col] = true;
                queue.push_back((next, dist + 1));
            }
        }
    }

    None  // No path found
}
```

**BFS Pattern**:
1. **Queue**: VecDeque for O(1) push_back/pop_front
2. **Visited tracking**: 2D boolean array (faster than HashSet for grids)
3. **Distance tracking**: Store with each position in queue
4. **Early termination**: Return immediately when goal reached

**Why VecDeque?**
- FIFO queue required for BFS
- O(1) operations on both ends
- Cache-friendly contiguous storage

**Why 2D visited array vs HashSet?**
- Grid coordinates: Array indexing is faster than hashing
- Memory: Similar (rows × cols booleans)
- Access pattern: O(1) vs O(1) average, but array has no hash overhead

### 5. `solve_part1_with_data()`

**Purpose**: Part 1 - single BFS from 'S' to 'E'

```rust
fn solve_part1_with_data(map: &HeightMap) -> usize {
    bfs_shortest_path(map, map.start, map.end)
        .expect("Path should exist from S to E")
}
```

**Simple wrapper**: Just runs BFS once

### 6. `solve_part2_with_data()` - Sequential

**Purpose**: Part 2 - try all 'a' positions (sequential version)

```rust
pub fn solve_part2_with_data(map: &HeightMap) -> usize {
    let mut min_steps = usize::MAX;

    for row in 0..map.rows {
        for col in 0..map.cols {
            if map.grid[row][col] == 0 {  // elevation 'a'
                let start = Pos { row, col };
                if let Some(steps) = bfs_shortest_path(map, start, map.end) {
                    min_steps = min_steps.min(steps);
                }
            }
        }
    }

    min_steps
}
```

**Approach**:
- Scan entire grid for 'a' positions
- Run BFS from each
- Track minimum path length
- **Performance**: 29.09ms (baseline)

### 7. `solve_part2_parallel()` - Parallel ⚡

**Purpose**: Part 2 - try all 'a' positions (parallel version)

```rust
pub fn solve_part2_parallel(map: &HeightMap) -> usize {
    // Step 1: Collect all 'a' elevation positions
    let start_positions: Vec<Pos> = (0..map.rows)
        .flat_map(|row| {
            (0..map.cols).filter_map(move |col| {
                if map.grid[row][col] == 0 {
                    Some(Pos { row, col })
                } else {
                    None
                }
            })
        })
        .collect();

    // Step 2: Run BFS from each 'a' in parallel using rayon
    start_positions
        .par_iter()                                    // Parallel iterator
        .filter_map(|&start| bfs_shortest_path(map, start, map.end))
        .min()
        .expect("At least one path should exist")
}
```

**Optimization breakdown**:

**Step 1: Collection phase**
- Use `flat_map` to collect all 'a' positions
- Creates `Vec<Pos>` of starting points
- Typically 20-50 positions in puzzle input

**Step 2: Parallel execution**
- `.par_iter()`: Rayon parallel iterator
- Each BFS runs on separate thread
- No shared mutable state (HeightMap is read-only)
- `.filter_map()`: Skip positions with no path
- `.min()`: Find minimum across all results

**Why it's so effective**:
1. **Independent work**: Each BFS is completely separate
2. **Balanced workload**: Each BFS takes similar time (~0.5-2ms)
3. **No synchronization**: Read-only HeightMap, no locks needed
4. **Automatic scheduling**: Rayon handles thread pool and work distribution

**Performance**: 3.07ms (9.5× speedup from 29.09ms)

### 8. `parse_input()`

**Purpose**: Parse height map from text

```rust
pub fn parse_input(input: &str) -> HeightMap {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();
    
    let mut grid = vec![vec![0u8; cols]; rows];
    let mut start = Pos { row: 0, col: 0 };
    let mut end = Pos { row: 0, col: 0 };

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let height = match ch {
                'S' => { start = Pos { row, col }; 0 }      // 'S' = 'a'
                'E' => { end = Pos { row, col }; 25 }       // 'E' = 'z'
                'a'..='z' => ch as u8 - b'a',               // Convert to 0-25
                _ => panic!("Invalid character: {}", ch),
            };
            grid[row][col] = height;
        }
    }

    HeightMap { grid, start, end, rows, cols }
}
```

**Pattern matching**:
- 'S' marker: Record position, treat as elevation 'a' (0)
- 'E' marker: Record position, treat as elevation 'z' (25)
- Regular letters: Convert 'a'=0, 'b'=1, ..., 'z'=25

---

## 🎨 Key Patterns & Techniques

### Pattern 1: BFS for Shortest Path

**When to use**: Unweighted graphs, find shortest path

**Template**:
```rust
fn bfs(start, goal) -> Option<usize> {
    queue = [(start, 0)]
    visited = set()
    
    while queue not empty:
        (node, dist) = dequeue()
        if node == goal: return Some(dist)
        
        for neighbor in neighbors(node):
            if not visited(neighbor):
                mark_visited(neighbor)
                enqueue((neighbor, dist + 1))
    
    None
}
```

**Advantages**:
- Guarantees shortest path
- Simple to implement
- Works for any graph structure

### Pattern 2: Grid Neighbor Generation

**Template**:
```rust
fn neighbors(&self, pos: Pos) -> Vec<Pos> {
    let mut result = Vec::new();
    
    // Up
    if pos.row > 0 {
        let next = Pos { row: pos.row - 1, col: pos.col };
        if is_valid_move(pos, next) {
            result.push(next);
        }
    }
    // Down
    if pos.row < self.rows - 1 { ... }
    // Left
    if pos.col > 0 { ... }
    // Right
    if pos.col < self.cols - 1 { ... }
    
    result
}
```

**Key points**:
- Bounds checking prevents panics
- Apply movement constraints here
- Can use arrays of directions for cleaner code

### Pattern 3: Multi-Source BFS with Rayon

**When to use**: Need to run same algorithm from multiple starting points independently

**Template**:
```rust
let starting_points: Vec<Start> = collect_starts();

let result = starting_points
    .par_iter()                    // Parallel!
    .filter_map(|start| {
        // Run independent computation
        compute(start)
    })
    .min()  // or .max(), .sum(), etc.
    .unwrap()
```

**Requirements for parallelization**:
- ✅ Independent computations (no shared mutable state)
- ✅ Balanced workload (each task takes similar time)
- ✅ Enough work to amortize thread overhead (not trivial tasks)

**When NOT to parallelize**:
- Small datasets (overhead > benefit)
- Sequential dependencies
- Shared mutable state requiring locks

### Pattern 4: Grid Visited Tracking

**Array vs HashSet**:

```rust
// Array: Fast for dense grids
let mut visited = vec![vec![false; cols]; rows];
visited[row][col] = true;

// HashSet: Better for sparse grids or large coordinates
let mut visited = HashSet::new();
visited.insert(Pos { row, col });
```

**Rule of thumb**:
- **Array**: Grid fits in memory, coordinates are bounded
- **HashSet**: Sparse grid, unknown bounds, or non-grid coordinates

---

## ⚠️ Common Mistakes

### 1. Wrong Elevation Constraint

❌ **Wrong**:
```rust
// Allowing any upward movement
if self.height_at(next) >= current_height { ... }
```

✅ **Correct**:
```rust
// Can climb UP by at most 1
if self.height_at(next) <= current_height + 1 { ... }
```

**Why**: Problem states "at most one higher" - this is critical constraint

### 2. Not Marking Visited Before Enqueue

❌ **Wrong**:
```rust
queue.push_back((next, dist + 1));
visited[next.row][next.col] = true;  // Too late!
```

✅ **Correct**:
```rust
visited[next.row][next.col] = true;  // Mark BEFORE enqueue
queue.push_back((next, dist + 1));
```

**Why**: If you mark after, the same cell can be added to queue multiple times before being processed, causing exponential blow-up

### 3. Using DFS Instead of BFS

❌ **Wrong**:
```rust
fn dfs(pos, visited) -> usize {
    if pos == end { return 0 }
    // Recursively try all paths...
}
```

**Why**: DFS doesn't guarantee shortest path in unweighted graphs. Need BFS for shortest path guarantee.

### 4. Not Checking Path Existence

❌ **Wrong**:
```rust
let min_steps = start_positions
    .par_iter()
    .map(|start| bfs_shortest_path(map, start, map.end).unwrap())  // Panic!
    .min()
    .unwrap()
```

✅ **Correct**:
```rust
let min_steps = start_positions
    .par_iter()
    .filter_map(|start| bfs_shortest_path(map, start, map.end))  // Skip None
    .min()
    .expect("At least one path should exist")
```

**Why**: Some 'a' positions might not have a path to 'E' (surrounded by high elevations)

### 5. Parallel Overhead for Small Grids

For very small grids (e.g., example input 5×8), parallel overhead might exceed benefit:
- Example: 5 starting positions, each BFS takes 10µs
- Sequential: 50µs
- Parallel: Thread spawn overhead + 10µs ≈ might be slower!

**Lesson**: Always benchmark! Parallel isn't always faster.

---

## 📊 Performance Analysis

### Benchmark Results

```
day12_part2_sequential:  29.09 ms
day12_part2_parallel:     3.07 ms
Speedup:                  9.5×
```

### Why 9.5× and not 8× (on 8-core machine)?

**Factors**:
1. **Workload variation**: Some 'a' positions have no path (fast), some have long paths
2. **Rayon efficiency**: Work-stealing scheduler balances well
3. **Cache effects**: Parallel execution may have better cache locality per thread
4. **Not perfectly parallelizable**: Some overhead from thread coordination

### Scalability Analysis

**Input characteristics** (typical puzzle input):
- Grid: ~41 rows × ~161 cols ≈ 6,600 cells
- Starting positions: ~50 'a' elevations
- Average BFS: ~500-1000 cells visited

**Sequential complexity**: O(num_starts × grid_size)
- 50 starts × 6,600 cells = ~330,000 operations

**Parallel complexity**: O(grid_size) with multi-core
- Same work, divided by cores
- Actual speedup: 9.5× (excellent scaling)

### Memory Usage

**Sequential**:
- Grid: ~6,600 bytes (u8 per cell)
- Visited array per BFS: ~6,600 bytes (bool per cell)
- Queue: ~100-1,000 entries during BFS
- **Total per BFS**: ~15 KB

**Parallel**:
- Shared grid: ~6,600 bytes (read-only)
- Per-thread visited + queue: ~15 KB × num_threads
- **Total**: ~6.6 KB + (15 KB × 8) ≈ 126 KB

**Conclusion**: Memory cost is negligible for the 9.5× speedup

---

## 🔗 Related Problems

**Similar AoC problems**:
- **2021 Day 15** - Grid pathfinding with Dijkstra's algorithm
- **2019 Day 18** - Multi-key maze solving with BFS
- **2022 Day 24** - Grid pathfinding with time-varying obstacles

**Graph algorithms**:
- **Dijkstra's**: For weighted shortest path
- **A***: For heuristic-guided search (if we had goal estimate)
- **Floyd-Warshall**: For all-pairs shortest paths

**Optimization techniques**:
- **Bidirectional BFS**: Search from both start and end (if end is known)
- **Multi-source BFS**: Instead of running BFS from each 'a', start BFS from ALL 'a' positions simultaneously
  - Would give same answer but might be faster!
  - Trade-off: More complex visited tracking

---

## 💡 Key Takeaways

1. **BFS guarantees shortest path** in unweighted graphs - use it!

2. **Grid neighbor generation** needs careful bounds checking and constraint validation

3. **Parallel optimization** can give massive speedups (9.5×) when:
   - Work is independent (no shared mutable state)
   - Workload is balanced
   - Enough computation to amortize overhead

4. **Rayon makes parallelization trivial**:
   - `.par_iter()` instead of `.iter()`
   - Automatic thread pooling and work distribution
   - No manual thread management

5. **Always benchmark** - parallel isn't always faster (overhead vs benefit)

6. **Visited tracking strategy** matters:
   - Grid arrays: Fast for bounded coordinates
   - HashSet: Better for sparse or unbounded spaces

---

**Navigation**: [← Day 11](day11.md) | [All Days](README.md) | [Day 13 →](day13.md)
