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
- Part 1 (sequential): ~260µs (single BFS)
- Part 2 (sequential): 28.74ms (multiple BFS from all 'a' positions)
- Part 2 (parallel): 3.08ms (rayon parallel BFS - 9.3× speedup)
- Part 2 (backward BFS): 174.68µs (reverse search from E - **164× speedup!** 🚀)
- **Combined (with backward BFS)**: 435.56µs (best approach! ✅)

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

**Optimized Approach 1** (parallel with rayon):
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
- Result: 3.08ms (9.3× speedup)

**Optimized Approach 2** (backward BFS - BEST!):
```rust
// Reverse the problem: Start from E, find closest 'a'
BFS backward from E:
    queue = [(end, distance=0)]
    visited = set()
    
    while queue not empty:
        (pos, dist) = dequeue()
        if height[pos] == 'a': return dist  // First 'a' is closest!
        
        for neighbor in valid_neighbors_reverse(pos):
            if not visited[neighbor]:
                visited[neighbor] = true
                enqueue((neighbor, dist + 1))
```

**Complexity**: O(rows × cols) - SINGLE BFS!
- Only ONE search instead of 50+
- Early termination at first 'a' found
- Constraint reversal: `current_height ≤ next_height + 1`
- Result: **174.68µs (164× speedup!)**

**Why backward BFS wins**:
- Instead of "which 'a' reaches E fastest?" (50+ questions)
- Ask "what's the nearest 'a' from E?" (1 question!)
- BFS guarantees first 'a' found is the closest

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

**Performance**: 3.08ms (9.3× speedup from 28.74ms)

### 8. `bfs_backward_to_any_a()` - Backward BFS 🚀 (BEST!)

**Purpose**: Part 2 - reverse search from E to nearest 'a'

```rust
pub fn bfs_backward_to_any_a(map: &HeightMap) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; map.cols]; map.rows];
    
    queue.push_back((map.end, 0));
    visited[map.end.row][map.end.col] = true;

    while let Some((pos, dist)) = queue.pop_front() {
        // Check if we reached any 'a' elevation
        if map.height_at(pos) == 0 {
            return dist;  // First 'a' found is the closest!
        }

        for next in map.neighbors_reverse(pos) {
            if !visited[next.row][next.col] {
                visited[next.row][next.col] = true;
                queue.push_back((next, dist + 1));
            }
        }
    }

    panic!("No path from E to any 'a' position")
}
```

**Key innovation**: Reverse the search direction!

**Instead of**: "Run BFS from each of 50 'a' positions to find which reaches E fastest"
**Do this**: "Run ONE BFS from E backward to find the first (closest) 'a'"

**Critical insight - Constraint reversal**:
- **Forward**: Can climb UP by at most 1
  - Rule: `next_height ≤ current_height + 1`
- **Backward**: Can descend FROM by at most 1 (equivalent!)
  - Rule: `current_height ≤ next_height + 1`
  - Implementation in `neighbors_reverse()`

**neighbors_reverse() explained**:
```rust
fn neighbors_reverse(&self, pos: Pos) -> Vec<Pos> {
    // When going backward from E:
    // We want neighbors that COULD climb UP to current position
    // Which means: current can descend TO neighbor by at most 1
    
    if current_height <= neighbor_height + 1 {
        // Valid: neighbor can climb to current
        result.push(neighbor);
    }
}
```

**Why it's so fast**:
1. **Single search**: Only ONE BFS instead of 50+
2. **Early termination**: Stop at FIRST 'a' found
3. **No parallelization overhead**: Single-threaded but faster than 8-core parallel!
4. **Optimal by definition**: BFS guarantees first 'a' is nearest

**Performance**: **174.68µs (164× faster than sequential, 17.7× faster than parallel!)**

### 9. `parse_input()`

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

### Benchmark Results - All Three Approaches

```
day12_part2_sequential:   28.74 ms  (baseline)
day12_part2_parallel:      3.08 ms  (9.3× speedup)
day12_part2_backward:      0.175 ms (164× speedup!) 🚀

day12_combined_backward:   0.436 ms (parse + Part 1 + Part 2 backward)
```

### The Optimization Journey

**Evolution of Part 2 solutions**:
1. **Sequential**: Run BFS from each 'a' → 28.74ms
2. **Parallel (rayon)**: Same algorithm, use all CPU cores → 3.08ms (9.3×)
3. **Backward BFS**: Reverse the search, ONE BFS from E → **174.68µs (164×)**

**Why backward BFS is the clear winner**:
- Simpler code (no parallelization complexity)
- Faster than parallel on 8 cores!
- Lower memory usage (single BFS state vs multiple threads)
- Elegant algorithm - the "aha!" moment solution

### Why Backward BFS is 164× Faster

**The math**:
- Sequential: 50 'a' positions × ~575µs per BFS = 28.74ms
- Backward: 1 BFS from E × ~175µs = 0.175ms
- Speedup: 28.74 / 0.175 = **164×**

**The insight**:
- Don't solve 50 problems when you can solve 1!
- BFS from E stops at FIRST 'a' (which is closest)
- Same answer, massively less work

### Why Parallel is "Only" 9.3× (on 8-core machine)

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
- **Backward BFS** (IMPLEMENTED!): Reverse the search when goal is known but starts are many
  - Day 12 Part 2: ONE search from E backward beats 50+ searches from 'a' forward
  - 164× speedup with simpler code!
- **Bidirectional BFS**: Search from both start and end simultaneously, meet in middle
  - Good for single start/end when path is very long
  - More complex than backward BFS
- **Multi-source BFS**: Start BFS from ALL starting points simultaneously in one queue
  - Alternative to backward BFS
  - Requires tracking which source each path came from

---

## 💡 Key Takeaways

1. **BFS guarantees shortest path** in unweighted graphs - use it!

2. **Reverse the problem!** When you have many starts and one goal:
   - Run BFS backward from goal to find nearest start
   - Day 12 Part 2: 164× speedup from this insight alone!
   - Simpler AND faster than parallelization

3. **Constraint reversal**: When searching backward, flip the movement rules
   - Forward: "can climb UP by ≤1" → `next ≤ current + 1`
   - Backward: "can descend FROM by ≤1" → `current ≤ next + 1`
   - Same constraint, different direction!

4. **Grid neighbor generation** needs careful bounds checking and constraint validation

5. **Parallel optimization** can give speedups (9.3×) but:
   - Algorithm choice matters MORE than parallelization!
   - Backward BFS (175µs) beats 8-core parallel (3.08ms)
   - Always question if there's a better algorithm first

6. **Rayon makes parallelization trivial** when you need it:
   - `.par_iter()` instead of `.iter()`
   - Automatic thread pooling and work distribution
   - But don't reach for it before considering algorithm improvements

7. **Always benchmark** multiple approaches:
   - Sequential baseline: 28.74ms
   - Parallel optimization: 3.08ms (good!)
   - Algorithm change: 0.175ms (amazing!)

8. **Visited tracking strategy** matters:
   - Grid arrays: Fast for bounded coordinates
   - HashSet: Better for sparse or unbounded spaces

---

**Navigation**: [← Day 11](day11.md) | [All Days](README.md) | [Day 13 →](day13.md)
