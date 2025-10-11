# Day 25 · Queue Applications (BFS & Level Traversal)

> **Learning Context**: Day 25 explores practical queue applications with breadth-first search (BFS) - the foundation for shortest path algorithms, level-order traversal, and distance-based problems essential for Mission 6's pathfinding and countless AoC challenges.

**Cross-Track Integration:**
- **Mission Focus**: BFS powers Mission 6's shortest path utilities and Mission 2's ring buffer applications
- **Daily Study**: Week 4, Day 4 - Applied queue algorithms building on data structure foundations
- **Rust Book**: Applies Chapter 8 (VecDeque) and Chapter 4 (Ownership) with queue patterns

**Related Zettelkasten Notes:**
- [[../missions/Mission2/README|Mission2 - Ring Buffer Queue]]
- [[../missions/Mission6/README|Mission6 - BFS Pathfinding]]
- [[BFS Patterns]] - Breadth-first search applications
- [[zettel-index]] - Main learning hub

---

## 🎯 Core Concepts

### What is BFS (Breadth-First Search)?

**BFS** is a graph/grid traversal algorithm that explores nodes level by level, visiting all neighbors at distance `d` before moving to distance `d + 1`.

**Key Characteristics:**
- **FIFO Order**: Uses a queue (first-in, first-out)
- **Level-by-level**: Explores by increasing distance from start
- **Shortest Path**: First time reaching a node = shortest path
- **Complete**: Explores all reachable nodes systematically

**Why BFS Matters:**

1. **Shortest Path Finding**: In unweighted graphs/grids, BFS finds the shortest path
2. **Distance Computation**: Calculates distances from source to all reachable nodes
3. **Level Detection**: Groups nodes by their distance layer
4. **Reachability Analysis**: Determines what's accessible from a starting point

**Real-World Applications:**
- **GPS Navigation**: Shortest route with equal-cost edges
- **Social Networks**: Degrees of separation, friend recommendations
- **Web Crawlers**: Breadth-first page exploration
- **Game AI**: Movement planning, visibility checks
- **AoC Problems**: Maze solving, shortest path, flood fill with distances

---

## 🌊 BFS Algorithm Fundamentals

### Basic BFS Template

The **core BFS pattern** is surprisingly simple and powerful:

```rust
use std::collections::{VecDeque, HashSet};

/// Core BFS template for grid traversal
/// 
/// This is the fundamental pattern that all BFS variants build upon:
/// 1. Start with initial position in queue
/// 2. Mark it as visited
/// 3. While queue not empty:
///    a. Dequeue front position
///    b. Process it (print, store, check condition)
///    c. Add unvisited neighbors to queue
/// 
/// # Time Complexity
/// O(V + E) where V = cells, E = edges (typically 4V for grids)
/// 
/// # Space Complexity
/// O(V) for queue and visited set
pub fn bfs_basic(grid: &Grid<char>, start: Coord) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    // Step 1: Initialize with starting position
    queue.push_back(start);
    visited.insert(start);
    
    // Step 2: Process queue until empty
    while let Some(current) = queue.pop_front() {
        // Step 3: Process current node
        println!("Visiting: {:?}", current);
        
        // Step 4: Add unvisited neighbors
        for neighbor in grid.neighbors_4(current) {
            // Only process unvisited, passable cells
            if !visited.contains(&neighbor) && grid.is_passable(neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
}
```

**BFS Execution Visualization:**

```
Initial: Start at 'S'         Level 1: Distance 1          Level 2: Distance 2
#####                          #####                        #####
#S..#                          #1..#                        #12.#
#...#                          #1..#                        #12.#
#####                          #####                        #####

Level 3: Distance 3            Final: All reachable
#####                          #####
#123#                          #1234#
#123#                          #1234#
#####                          #####
```

### Why Use VecDeque Instead of Vec?

**Performance Comparison:**

| Operation | Vec (as queue) | VecDeque |
|-----------|----------------|----------|
| `push_back()` | O(1) amortized | O(1) amortized |
| `pop_front()` | **O(n)** ❌ | **O(1)** ✅ |
| Memory | Contiguous | Ring buffer |

**Vec as Queue Problem:**
```rust
// DON'T DO THIS - O(n) per removal!
let mut queue = Vec::new();
queue.push(start);              // O(1) - good
let current = queue.remove(0);  // O(n) - BAD! Shifts entire array
```

**VecDeque as Queue:**
```rust
// DO THIS - O(1) for both operations
let mut queue = VecDeque::new();
queue.push_back(start);         // O(1)
let current = queue.pop_front(); // O(1) - GOOD!
```

**Why VecDeque is Efficient:**
- Implemented as a **ring buffer** (circular array)
- Both ends can grow/shrink in O(1)
- No shifting elements on removal
- Cache-friendly for sequential access

---

## 🗺️ BFS with Distance Tracking

### Recording Distances from Source

One of BFS's superpowers is computing shortest distances:

```rust
use std::collections::{VecDeque, HashMap};

/// BFS that tracks distance from start to each reachable cell
/// 
/// Returns a HashMap mapping coordinates to their distance from start.
/// This is the foundation for shortest path algorithms.
/// 
/// # Returns
/// HashMap where key = coordinate, value = shortest distance from start
/// 
/// # Example
/// ```
/// let distances = bfs_distances(&grid, start);
/// println!("Distance to goal: {}", distances[&goal]);
/// ```
pub fn bfs_distances(
    grid: &Grid<char>,
    start: Coord,
) -> HashMap<Coord, usize> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    
    // Start at distance 0
    queue.push_back((start, 0));
    distances.insert(start, 0);
    
    while let Some((current, dist)) = queue.pop_front() {
        // Explore neighbors at distance + 1
        for neighbor in grid.neighbors_4(current) {
            // Skip if already visited or impassable
            if distances.contains_key(&neighbor) {
                continue;
            }
            if !grid.is_passable(neighbor) {
                continue;
            }
            
            // Record distance and enqueue
            distances.insert(neighbor, dist + 1);
            queue.push_back((neighbor, dist + 1));
        }
    }
    
    distances
}
```

**Distance Map Visualization:**

```
Grid:         Distances from S:
#####         #####
#S..#         #0123#
#.#.#         #1#2#
#..G#         #234G#
#####         #####

Shortest path from S to G: 4 steps
```

### Reconstructing the Shortest Path

Computing distances is great, but often you need the **actual path**:

```rust
use std::collections::{VecDeque, HashMap};

/// BFS that finds shortest path from start to goal
/// 
/// Returns the path as a vector of coordinates, or None if no path exists.
/// Uses a "parent" map to reconstruct the path backwards from goal.
/// 
/// # Algorithm
/// 1. Run BFS, recording each node's parent
/// 2. If goal reached, backtrack from goal to start using parents
/// 3. Reverse the path to get start → goal order
pub fn bfs_shortest_path(
    grid: &Grid<char>,
    start: Coord,
    goal: Coord,
) -> Option<Vec<Coord>> {
    let mut queue = VecDeque::new();
    let mut parent: HashMap<Coord, Coord> = HashMap::new();
    
    queue.push_back(start);
    parent.insert(start, start); // Start is its own parent
    
    // Run BFS until we find the goal
    while let Some(current) = queue.pop_front() {
        // Found the goal!
        if current == goal {
            return Some(reconstruct_path(&parent, start, goal));
        }
        
        for neighbor in grid.neighbors_4(current) {
            if parent.contains_key(&neighbor) {
                continue;
            }
            if !grid.is_passable(neighbor) {
                continue;
            }
            
            parent.insert(neighbor, current); // Record how we got here
            queue.push_back(neighbor);
        }
    }
    
    None // No path exists
}

/// Reconstructs path by following parent pointers backwards
fn reconstruct_path(
    parent: &HashMap<Coord, Coord>,
    start: Coord,
    goal: Coord,
) -> Vec<Coord> {
    let mut path = Vec::new();
    let mut current = goal;
    
    // Walk backwards from goal to start
    while current != start {
        path.push(current);
        current = parent[&current];
    }
    path.push(start);
    
    // Reverse to get start → goal order
    path.reverse();
    path
}
```

**Path Reconstruction Example:**

```
Parent Map:              Reconstructed Path:
#####                    #####
#S→→↓#                   #S123#
#↑#↓#                    #4#5#
#↑←←G#                   #678G#
#####                    #####

Path: [S, (0,1), (0,2), (1,2), (2,2), (3,2), (3,1), (3,0), G]
```

---

## 📊 Level-Order Traversal

### Processing Nodes by Distance Layers

Sometimes you need to process all nodes at distance `d` before moving to `d + 1`:

```rust
/// BFS that processes nodes in distinct levels
/// 
/// Returns a vector of levels, where each level is a vector of coordinates
/// at that distance from the start.
/// 
/// # Use Cases
/// - Multi-agent expansion (all agents move simultaneously)
/// - Flood simulation (water spreads evenly)
/// - Turn-based game mechanics
/// 
/// # Example Output
/// ```
/// Level 0: [start]
/// Level 1: [neighbors of start]
/// Level 2: [neighbors of level 1]
/// ...
/// ```
pub fn bfs_by_levels(
    grid: &Grid<char>,
    start: Coord,
) -> Vec<Vec<Coord>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut levels = Vec::new();
    
    queue.push_back((start, 0));
    visited.insert(start);
    
    let mut current_level = 0;
    let mut current_level_nodes = Vec::new();
    
    while let Some((pos, level)) = queue.pop_front() {
        // New level started?
        if level > current_level {
            levels.push(current_level_nodes);
            current_level_nodes = Vec::new();
            current_level = level;
        }
        
        current_level_nodes.push(pos);
        
        for neighbor in grid.neighbors_4(pos) {
            if !visited.contains(&neighbor) && grid.is_passable(neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, level + 1));
            }
        }
    }
    
    // Don't forget last level
    if !current_level_nodes.is_empty() {
        levels.push(current_level_nodes);
    }
    
    levels
}
```

**Alternative: Size-Based Level Processing**

```rust
/// Process levels using queue size instead of tracking level numbers
/// 
/// This is a common interview pattern that's more elegant in some cases.
pub fn bfs_by_levels_size_based(
    grid: &Grid<char>,
    start: Coord,
) -> Vec<Vec<Coord>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut levels = Vec::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while !queue.is_empty() {
        let level_size = queue.len(); // All nodes at current level
        let mut current_level = Vec::new();
        
        // Process exactly level_size nodes
        for _ in 0..level_size {
            let pos = queue.pop_front().unwrap();
            current_level.push(pos);
            
            for neighbor in grid.neighbors_4(pos) {
                if !visited.contains(&neighbor) && grid.is_passable(neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
        
        levels.push(current_level);
    }
    
    levels
}
```

---

## 🎮 Advanced BFS Patterns

### Multi-Source BFS

**Problem**: Find shortest distance from ANY of multiple starting points.

```rust
/// BFS from multiple sources simultaneously
/// 
/// Useful for:
/// - Nearest facility problems (hospitals, fire stations)
/// - Multi-player game start positions
/// - Expanding multiple regions at once
/// 
/// # Example
/// Find distance to nearest fire station from any cell
pub fn bfs_multi_source(
    grid: &Grid<char>,
    sources: &[Coord],
) -> HashMap<Coord, (usize, Coord)> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    
    // Initialize queue with ALL sources at distance 0
    for &source in sources {
        queue.push_back((source, 0, source));
        distances.insert(source, (0, source));
    }
    
    while let Some((current, dist, origin)) = queue.pop_front() {
        for neighbor in grid.neighbors_4(current) {
            if distances.contains_key(&neighbor) {
                continue;
            }
            if !grid.is_passable(neighbor) {
                continue;
            }
            
            // Record distance and which source it came from
            distances.insert(neighbor, (dist + 1, origin));
            queue.push_back((neighbor, dist + 1, origin));
        }
    }
    
    distances // Returns (distance, nearest_source)
}
```

### BFS with Obstacles and Keys

**Problem**: Grid with locked doors that require keys.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State {
    pub pos: Coord,
    pub keys: u32, // Bitmask of collected keys
}

/// BFS in a dungeon with keys and doors
/// 
/// State space: (position, keys_collected)
/// Doors can only be passed if you have the matching key.
pub fn bfs_with_keys(
    grid: &Grid<char>,
    start: Coord,
    goal: Coord,
) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    let initial_state = State { pos: start, keys: 0 };
    queue.push_back((initial_state, 0));
    visited.insert(initial_state);
    
    while let Some((state, dist)) = queue.pop_front() {
        if state.pos == goal {
            return Some(dist);
        }
        
        for neighbor_pos in grid.neighbors_4(state.pos) {
            let cell = grid.get_coord(neighbor_pos)?;
            
            // Determine if we can pass this cell
            let can_pass = match cell {
                '#' => false, // Wall
                '.' | 'S' | 'G' => true, // Open space
                'a'..='z' => true, // Key (can always pick up)
                'A'..='Z' => {
                    // Door (need matching key)
                    let key_bit = (*cell as u8 - b'A') as u32;
                    (state.keys & (1 << key_bit)) != 0
                }
                _ => false,
            };
            
            if !can_pass {
                continue;
            }
            
            // Update keys if we picked one up
            let mut new_keys = state.keys;
            if cell.is_ascii_lowercase() {
                let key_bit = (*cell as u8 - b'a') as u32;
                new_keys |= 1 << key_bit;
            }
            
            let new_state = State { pos: neighbor_pos, keys: new_keys };
            
            if !visited.contains(&new_state) {
                visited.insert(new_state);
                queue.push_back((new_state, dist + 1));
            }
        }
    }
    
    None
}
```

---

## 🚀 Complete Runnable Example

```rust
use std::collections::{VecDeque, HashSet, HashMap};
use std::fmt;

fn main() {
    println!("=== Day 25: Queue Applications (BFS) Demo ===\n");
    
    // 1. Basic BFS Traversal
    println!("🔷 1. Basic BFS Traversal");
    println!("=========================");
    
    let grid1 = Grid::from_pattern(vec![
        "#########",
        "#S......#",
        "#.####..#",
        "#....#..#",
        "####.#..#",
        "#.......#",
        "#########",
    ]);
    
    println!("Grid:");
    println!("{}", grid1);
    
    let start = Coord::new(1, 1);
    println!("BFS traversal from S:");
    bfs_basic(&grid1, start);
    
    // 2. Distance Calculation
    println!("\n🔷 2. Computing Distances");
    println!("=========================");
    
    let distances = bfs_distances(&grid1, start);
    
    println!("Distance map:");
    for row in 0..grid1.height() {
        for col in 0..grid1.width() {
            let pos = Coord::new(row as isize, col as isize);
            if let Some(&dist) = distances.get(&pos) {
                print!("{:2} ", dist);
            } else {
                print!(" # ");
            }
        }
        println!();
    }
    
    // 3. Shortest Path Finding
    println!("\n🔷 3. Shortest Path");
    println!("===================");
    
    let grid2 = Grid::from_pattern(vec![
        "##########",
        "#S.......#",
        "#.####.#.#",
        "#....#.#.#",
        "####.#.#.#",
        "#....#...#",
        "#.####.#.#",
        "#......#G#",
        "##########",
    ]);
    
    println!("Maze:");
    println!("{}", grid2);
    
    let start = Coord::new(1, 1);
    let goal = Coord::new(7, 8);
    
    if let Some(path) = bfs_shortest_path(&grid2, start, goal) {
        println!("\nShortest path found! Length: {}", path.len());
        println!("Path: {:?}", path);
        
        // Visualize path
        println!("\nPath visualization:");
        for row in 0..grid2.height() {
            for col in 0..grid2.width() {
                let pos = Coord::new(row as isize, col as isize);
                if path.contains(&pos) {
                    print!("*");
                } else {
                    print!("{}", grid2[(row, col)]);
                }
            }
            println!();
        }
    }
    
    // 4. Level-Order Traversal
    println!("\n🔷 4. Level-Order Traversal");
    println!("===========================");
    
    let grid3 = Grid::from_pattern(vec![
        "#######",
        "#.....#",
        "#.###.#",
        "#..S..#",
        "#.###.#",
        "#.....#",
        "#######",
    ]);
    
    println!("Grid:");
    println!("{}", grid3);
    
    let start = Coord::new(3, 3);
    let levels = bfs_by_levels(&grid3, start);
    
    println!("\nNodes by distance level:");
    for (level, nodes) in levels.iter().enumerate() {
        println!("  Level {}: {} nodes", level, nodes.len());
        if level < 3 {
            println!("    {:?}", nodes);
        }
    }
    
    // 5. Multi-Source BFS
    println!("\n🔷 5. Multi-Source BFS (Nearest Facility)");
    println!("==========================================");
    
    let grid4 = Grid::from_pattern(vec![
        "###########",
        "#.........#",
        "#....H....#",
        "#.........#",
        "#.........#",
        "#....H....#",
        "#.........#",
        "###########",
    ]);
    
    println!("City map (H = hospital):");
    println!("{}", grid4);
    
    let hospitals = vec![
        Coord::new(2, 5),
        Coord::new(5, 5),
    ];
    
    let nearest = bfs_multi_source(&grid4, &hospitals);
    
    println!("\nDistance to nearest hospital:");
    for row in 0..grid4.height() {
        for col in 0..grid4.width() {
            let pos = Coord::new(row as isize, col as isize);
            if let Some(&(dist, _)) = nearest.get(&pos) {
                print!("{}", dist);
            } else {
                print!("#");
            }
        }
        println!();
    }
    
    // 6. Practical Application: Escape Room
    println!("\n🔷 6. Escape Room Problem");
    println!("=========================");
    
    let escape_grid = Grid::from_pattern(vec![
        "###########",
        "#S........#",
        "#.###.###.#",
        "#...#.#...#",
        "###.#.#.###",
        "#.........#",
        "#.###.###.#",
        "#........G#",
        "###########",
    ]);
    
    println!("Escape room layout:");
    println!("{}", escape_grid);
    
    let start = Coord::new(1, 1);
    let exit = Coord::new(7, 9);
    
    if let Some(path) = bfs_shortest_path(&escape_grid, start, exit) {
        println!("\nEscape route found!");
        println!("Steps to exit: {}", path.len() - 1);
        println!("Efficiency: {}%",
            ((path.len() - 1) as f64 / 20.0 * 100.0) as usize);
    }
    
    // 7. BFS Performance Comparison
    println!("\n🔷 7. Algorithm Comparison");
    println!("==========================");
    
    let large_grid = Grid::from_pattern(vec![
        "#".repeat(50),
        format!("#S{}#", ".".repeat(48)),
        format!("#{}.#", ".".repeat(48)),
        // ... (truncated for brevity)
        format!("#{}G#", ".".repeat(48)),
        "#".repeat(50),
    ].iter().map(|s| s.as_str()).collect::<Vec<_>>());
    
    use std::time::Instant;
    
    let start = Coord::new(1, 1);
    let goal = Coord::new(48, 48);
    
    let timer = Instant::now();
    let path = bfs_shortest_path(&large_grid, start, goal);
    let duration = timer.elapsed();
    
    println!("Large grid (50x50):");
    println!("  Path length: {}", path.map_or(0, |p| p.len()));
    println!("  Time: {:?}", duration);
    println!("  Performance: Excellent for unweighted graphs! ✅");
}

// === Supporting Code ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: isize,
    pub col: isize,
}

impl Coord {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl Grid<char> {
    pub fn from_pattern(lines: Vec<&str>) -> Self {
        let height = lines.len();
        let width = lines[0].len();
        let mut data = Vec::with_capacity(width * height);
        
        for line in lines {
            for ch in line.chars() {
                data.push(ch);
            }
        }
        
        Self { data, width, height }
    }
    
    pub fn is_passable(&self, coord: Coord) -> bool {
        if let Some(&ch) = self.get_coord(coord) {
            ch != '#'
        } else {
            false
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    
    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
    
    pub fn contains(&self, coord: Coord) -> bool {
        coord.row >= 0 && coord.row < self.height as isize &&
        coord.col >= 0 && coord.col < self.width as isize
    }
    
    pub fn get_coord(&self, coord: Coord) -> Option<&T> {
        if self.contains(coord) {
            Some(&self[(coord.row as usize, coord.col as usize)])
        } else {
            None
        }
    }
    
    pub fn neighbors_4(&self, coord: Coord) -> Vec<Coord> {
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        offsets.iter()
            .map(|(dr, dc)| Coord::new(coord.row + dr, coord.col + dc))
            .filter(|&pos| self.contains(pos))
            .collect()
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[self.index(row, col)]
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self[(row, col)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// BFS implementations
fn bfs_basic(grid: &Grid<char>, start: Coord) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    let mut count = 0;
    while let Some(current) = queue.pop_front() {
        count += 1;
        if count <= 10 {
            println!("  Visiting: {:?}", current);
        }
        
        for neighbor in grid.neighbors_4(current) {
            if !visited.contains(&neighbor) && grid.is_passable(neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
    println!("  ... (visited {} cells total)", count);
}

fn bfs_distances(grid: &Grid<char>, start: Coord) -> HashMap<Coord, usize> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    
    queue.push_back((start, 0));
    distances.insert(start, 0);
    
    while let Some((current, dist)) = queue.pop_front() {
        for neighbor in grid.neighbors_4(current) {
            if distances.contains_key(&neighbor) {
                continue;
            }
            if !grid.is_passable(neighbor) {
                continue;
            }
            
            distances.insert(neighbor, dist + 1);
            queue.push_back((neighbor, dist + 1));
        }
    }
    
    distances
}

fn bfs_shortest_path(
    grid: &Grid<char>,
    start: Coord,
    goal: Coord,
) -> Option<Vec<Coord>> {
    let mut queue = VecDeque::new();
    let mut parent: HashMap<Coord, Coord> = HashMap::new();
    
    queue.push_back(start);
    parent.insert(start, start);
    
    while let Some(current) = queue.pop_front() {
        if current == goal {
            return Some(reconstruct_path(&parent, start, goal));
        }
        
        for neighbor in grid.neighbors_4(current) {
            if parent.contains_key(&neighbor) {
                continue;
            }
            if !grid.is_passable(neighbor) {
                continue;
            }
            
            parent.insert(neighbor, current);
            queue.push_back(neighbor);
        }
    }
    
    None
}

fn reconstruct_path(
    parent: &HashMap<Coord, Coord>,
    start: Coord,
    goal: Coord,
) -> Vec<Coord> {
    let mut path = Vec::new();
    let mut current = goal;
    
    while current != start {
        path.push(current);
        current = parent[&current];
    }
    path.push(start);
    
    path.reverse();
    path
}

fn bfs_by_levels(grid: &Grid<char>, start: Coord) -> Vec<Vec<Coord>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut levels = Vec::new();
    
    queue.push_back((start, 0));
    visited.insert(start);
    
    let mut current_level = 0;
    let mut current_level_nodes = Vec::new();
    
    while let Some((pos, level)) = queue.pop_front() {
        if level > current_level {
            levels.push(current_level_nodes);
            current_level_nodes = Vec::new();
            current_level = level;
        }
        
        current_level_nodes.push(pos);
        
        for neighbor in grid.neighbors_4(pos) {
            if !visited.contains(&neighbor) && grid.is_passable(neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, level + 1));
            }
        }
    }
    
    if !current_level_nodes.is_empty() {
        levels.push(current_level_nodes);
    }
    
    levels
}

fn bfs_multi_source(
    grid: &Grid<char>,
    sources: &[Coord],
) -> HashMap<Coord, (usize, Coord)> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    
    for &source in sources {
        queue.push_back((source, 0, source));
        distances.insert(source, (0, source));
    }
    
    while let Some((current, dist, origin)) = queue.pop_front() {
        for neighbor in grid.neighbors_4(current) {
            if distances.contains_key(&neighbor) {
                continue;
            }
            if !grid.is_passable(neighbor) {
                continue;
            }
            
            distances.insert(neighbor, (dist + 1, origin));
            queue.push_back((neighbor, dist + 1, origin));
        }
    }
    
    distances
}
```

### 🛠️ How to Run This Code:

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day25_demo.rs` and run `rustc day25_demo.rs && ./day25_demo`
3. **In this workspace**: Create a new cargo project and paste into `src/main.rs`

---

## 💡 Key Takeaways

### When to Use BFS

**Use BFS when:**
- ✅ Need shortest path in unweighted graphs
- ✅ Computing distances from a source
- ✅ Level-order processing required
- ✅ Finding nearest/closest entities
- ✅ Exploring reachability

**Don't use BFS when:**
- ❌ Graph has weighted edges (use Dijkstra instead)
- ❌ Only need to check reachability (DFS is simpler)
- ❌ Memory is extremely limited (DFS uses less)

### Performance Characteristics

| Aspect | Complexity | Notes |
|--------|-----------|-------|
| Time | O(V + E) | V = vertices, E = edges |
| Space | O(V) | Queue + visited set |
| Shortest Path | ✅ Optimal | For unweighted graphs |
| Distance Computation | ✅ Exact | First visit = shortest |

### Common BFS Patterns

**Pattern 1: Distance Map**
```rust
let distances = bfs_distances(&grid, start);
let max_distance = distances.values().max().unwrap();
```

**Pattern 2: Reachability Check**
```rust
let visited = bfs_basic(&grid, start);
let is_reachable = visited.contains(&goal);
```

**Pattern 3: Multi-Goal Search**
```rust
// Find shortest path to ANY goal
for current in bfs_iterator(&grid, start) {
    if goals.contains(&current) {
        return Some(current);
    }
}
```

---

## 🔗 Related Topics

### Tomorrow's Preview: Day 26 - Advanced Queues
- Priority queues with BinaryHeap
- Dijkstra's algorithm for weighted graphs
- VecDeque advanced patterns
- Queue-based state machines

### Mission Integration
- **Mission 2**: Ring buffer queue implementation
- **Mission 6**: Complete BFS pathfinding utilities

### AoC Applications
- **Maze Solving**: Shortest path problems
- **Island Counting**: With distance calculation
- **Flood Fill**: BFS variant for region analysis
- **Multi-Agent**: Simultaneous movement simulation

---

*Tags: #bfs #queues #shortest-path #level-traversal #vecdeque #mission2 #mission6 #graph-algorithms #aoc-patterns*
*Links: [[daily-study/Day24]] ← | [[../missions/Mission6/README|Mission6]] | [[zettel-index]] | [[daily-study/Day26]] →*
