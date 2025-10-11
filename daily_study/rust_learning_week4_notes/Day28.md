# Day 28 · Week 4 Review & Integration

> **Learning Context**: Day 28 synthesizes Week 4's complete journey through grids, algorithms, queues, and parsing - bringing together all concepts into integrated problem-solving patterns essential for Mission 6 and advanced AoC challenges.

**Cross-Track Integration:**
- **Mission Focus**: Complete toolkit for Mission 6 (grids + BFS + Dijkstra) and Mission 2 (advanced queues)
- **Daily Study**: Week 4 Capstone - Integration of Days 22-27
- **Rust Book**: Chapters 8-10 applied in complete solutions

**Related Zettelkasten Notes:**
- [[../missions/Mission6/README|Mission6 - Complete Pathfinding]]
- [[Week 4 Concepts MOC]] - All Week 4 topics
- [[zettel-index]] - Main learning hub

---

## 🎯 Week 4 Journey Overview

### What We Accomplished

**Day 22: Grid Fundamentals**
- ✅ 2D array representations
- ✅ Coordinate systems (array-style, Cartesian, linear)
- ✅ Grid storage strategies (flat vector vs nested)
- ✅ `isize` vs `usize` for coordinates

**Day 23: Grid Navigation**
- ✅ Direction enums (Cardinal 4-connected, Intercardinal 8-connected)
- ✅ Neighbor generation algorithms
- ✅ Three bounds checking strategies
- ✅ Movement and rotation patterns

**Day 24: Grid Algorithms**
- ✅ Flood fill (recursive, iterative, BFS)
- ✅ Connected components detection
- ✅ Component analysis (size, perimeter, bounds)
- ✅ Island counting patterns

**Day 25: Queue Applications**
- ✅ BFS shortest path
- ✅ Distance computation
- ✅ Level-order traversal
- ✅ Multi-source BFS
- ✅ VecDeque fundamentals

**Day 26: Advanced Queues**
- ✅ Priority queues (BinaryHeap)
- ✅ Dijkstra's algorithm for weighted graphs
- ✅ VecDeque advanced patterns (sliding window, monotonic queue)
- ✅ Min-heap with `Reverse<T>`

**Day 27: String Parsing**
- ✅ Split methods family
- ✅ Number extraction
- ✅ Regex patterns (with `regex` crate)
- ✅ Custom parser implementations
- ✅ AoC input handling

---

## 🧩 Integrated Problem Solving

### Complete AoC-Style Problem: Dungeon Pathfinding

Let's solve a comprehensive problem that uses ALL Week 4 concepts:

**Problem Statement:**
```
You're in a dungeon represented as a grid with:
- '#' = walls (impassable)
- '.' = open floor (cost 1 to traverse)
- '~' = water (cost 3 to traverse)
- 'S' = start position
- 'G' = goal position

Tasks:
1. Parse the dungeon from text input
2. Find shortest path (considering costs)
3. Count all reachable rooms
4. Find the largest room
5. Calculate water coverage percentage
```

### Complete Integrated Solution

```rust
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::{Ordering, Reverse};
use std::fmt;

fn main() {
    println!("=== Day 28: Week 4 Integration - Dungeon Explorer ===\n");
    
    // Step 1: Parse input (Day 27 - String Parsing)
    let input = "\
####################
#S.................#
#.####.###~~###....#
#.#..#.#.~~~~#.....#
#.#..#.#.~~~~#.###.#
#.####.###~~###.#G.#
#..................#
####################";
    
    println!("🔷 Task 1: Parse Dungeon");
    println!("========================");
    println!("{}", input);
    
    let dungeon = DungeonGrid::parse(input);
    println!("Parsed: {}x{} grid", dungeon.height(), dungeon.width());
    println!("Start: {:?}", dungeon.start);
    println!("Goal: {:?}\n", dungeon.goal);
    
    // Step 2: Find shortest path (Day 26 - Dijkstra)
    println!("🔷 Task 2: Find Shortest Path (Dijkstra)");
    println!("=========================================");
    
    if let Some((cost, path)) = dungeon.shortest_path() {
        println!("Path found!");
        println!("  Total cost: {}", cost);
        println!("  Steps: {}", path.len() - 1);
        
        // Visualize path
        println!("\nPath visualization:");
        dungeon.visualize_path(&path);
    } else {
        println!("No path exists!");
    }
    println!();
    
    // Step 3: Count rooms (Day 24 - Connected Components)
    println!("🔷 Task 3: Analyze Rooms");
    println!("========================");
    
    let rooms = dungeon.find_rooms();
    println!("Found {} rooms", rooms.len());
    
    for (i, room) in rooms.iter().enumerate() {
        let info = dungeon.analyze_component(room);
        println!("  Room {}: {} cells, perimeter {}",
            i + 1, info.size, info.perimeter);
    }
    println!();
    
    // Step 4: Find largest room
    println!("🔷 Task 4: Largest Room");
    println!("=======================");
    
    if let Some(largest) = rooms.iter().max_by_key(|r| r.len()) {
        let info = dungeon.analyze_component(largest);
        println!("Largest room:");
        println!("  Size: {} cells", info.size);
        println!("  Perimeter: {}", info.perimeter);
        println!("  Bounds: {:?}", info.bounds);
    }
    println!();
    
    // Step 5: Water coverage
    println!("🔷 Task 5: Terrain Analysis");
    println!("===========================");
    
    let terrain_stats = dungeon.analyze_terrain();
    println!("Terrain distribution:");
    println!("  Walls: {} ({:.1}%)",
        terrain_stats.walls,
        100.0 * terrain_stats.walls as f64 / terrain_stats.total as f64);
    println!("  Floor: {} ({:.1}%)",
        terrain_stats.floor,
        100.0 * terrain_stats.floor as f64 / terrain_stats.total as f64);
    println!("  Water: {} ({:.1}%)",
        terrain_stats.water,
        100.0 * terrain_stats.water as f64 / terrain_stats.total as f64);
    println!();
    
    // Bonus: BFS distance map
    println!("🔷 Bonus: Distance Map from Start (BFS)");
    println!("========================================");
    
    let distances = dungeon.bfs_distances(dungeon.start);
    println!("Reachable cells: {}", distances.len());
    
    if let Some(&max_dist) = distances.values().max() {
        println!("Farthest reachable point: {} steps away", max_dist);
    }
    
    // Show distance map (first few rows)
    println!("\nDistance visualization (excerpt):");
    for row in 0..8.min(dungeon.height()) {
        for col in 0..dungeon.width() {
            let pos = Coord::new(row as isize, col as isize);
            if dungeon[(row, col)] == '#' {
                print!(" ## ");
            } else if let Some(&dist) = distances.get(&pos) {
                print!("{:3} ", dist);
            } else {
                print!(" -- ");
            }
        }
        println!();
    }
}

// === Complete Integrated Data Structures ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord {
    pub row: isize,
    pub col: isize,
}

impl Coord {
    pub fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

pub struct DungeonGrid {
    data: Vec<char>,
    width: usize,
    height: usize,
    start: Coord,
    goal: Coord,
}

impl DungeonGrid {
    /// Day 27: String Parsing - Parse grid from text
    pub fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let mut data = Vec::with_capacity(width * height);
        let mut start = Coord::new(0, 0);
        let mut goal = Coord::new(0, 0);
        
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                data.push(ch);
                if ch == 'S' {
                    start = Coord::new(row as isize, col as isize);
                } else if ch == 'G' {
                    goal = Coord::new(row as isize, col as isize);
                }
            }
        }
        
        Self { data, width, height, start, goal }
    }
    
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    
    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
    
    /// Day 22: Grid Fundamentals - Bounds checking
    pub fn contains(&self, coord: Coord) -> bool {
        coord.row >= 0 && coord.row < self.height as isize &&
        coord.col >= 0 && coord.col < self.width as isize
    }
    
    pub fn get_coord(&self, coord: Coord) -> Option<char> {
        if self.contains(coord) {
            Some(self[(coord.row as usize, coord.col as usize)])
        } else {
            None
        }
    }
    
    /// Day 23: Grid Navigation - 4-connected neighbors
    pub fn neighbors_4(&self, coord: Coord) -> Vec<Coord> {
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        offsets.iter()
            .map(|(dr, dc)| Coord::new(coord.row + dr, coord.col + dc))
            .filter(|&pos| self.contains(pos))
            .collect()
    }
    
    fn get_cost(&self, coord: Coord) -> usize {
        match self.get_coord(coord) {
            Some('#') => usize::MAX, // Impassable
            Some('~') => 3,           // Water
            Some('.') | Some('S') | Some('G') => 1, // Floor
            _ => usize::MAX,
        }
    }
    
    /// Day 26: Dijkstra's Algorithm - Shortest path with costs
    pub fn shortest_path(&self) -> Option<(usize, Vec<Coord>)> {
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State {
            cost: usize,
            position: Coord,
        }
        
        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                other.cost.cmp(&self.cost)
                    .then_with(|| self.position.cmp(&other.position))
            }
        }
        
        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        
        let mut dist: HashMap<Coord, usize> = HashMap::new();
        let mut parent: HashMap<Coord, Coord> = HashMap::new();
        let mut heap = BinaryHeap::new();
        
        dist.insert(self.start, 0);
        parent.insert(self.start, self.start);
        heap.push(State { cost: 0, position: self.start });
        
        while let Some(State { cost, position }) = heap.pop() {
            if position == self.goal {
                let path = self.reconstruct_path(&parent, self.start, self.goal);
                return Some((cost, path));
            }
            
            if let Some(&best) = dist.get(&position) {
                if cost > best {
                    continue;
                }
            }
            
            for neighbor in self.neighbors_4(position) {
                let move_cost = self.get_cost(neighbor);
                if move_cost == usize::MAX {
                    continue;
                }
                
                let next_cost = cost + move_cost;
                let is_shorter = dist.get(&neighbor)
                    .map_or(true, |&current| next_cost < current);
                
                if is_shorter {
                    dist.insert(neighbor, next_cost);
                    parent.insert(neighbor, position);
                    heap.push(State { cost: next_cost, position: neighbor });
                }
            }
        }
        
        None
    }
    
    fn reconstruct_path(
        &self,
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
    
    /// Day 25: BFS - Distance computation
    pub fn bfs_distances(&self, start: Coord) -> HashMap<Coord, usize> {
        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();
        
        queue.push_back((start, 0));
        distances.insert(start, 0);
        
        while let Some((current, dist)) = queue.pop_front() {
            for neighbor in self.neighbors_4(current) {
                if distances.contains_key(&neighbor) {
                    continue;
                }
                if self.get_cost(neighbor) == usize::MAX {
                    continue;
                }
                
                distances.insert(neighbor, dist + 1);
                queue.push_back((neighbor, dist + 1));
            }
        }
        
        distances
    }
    
    /// Day 24: Connected Components - Find all rooms
    pub fn find_rooms(&self) -> Vec<Vec<Coord>> {
        let mut visited = HashSet::new();
        let mut rooms = Vec::new();
        
        for row in 0..self.height {
            for col in 0..self.width {
                let pos = Coord::new(row as isize, col as isize);
                
                if visited.contains(&pos) {
                    continue;
                }
                
                let cell = self[(row, col)];
                if cell == '#' {
                    continue;
                }
                
                let room = self.explore_room(pos, &mut visited);
                if !room.is_empty() {
                    rooms.push(room);
                }
            }
        }
        
        rooms
    }
    
    fn explore_room(&self, start: Coord, visited: &mut HashSet<Coord>) -> Vec<Coord> {
        let mut stack = vec![start];
        let mut room = Vec::new();
        
        while let Some(pos) = stack.pop() {
            if visited.contains(&pos) {
                continue;
            }
            if !self.contains(pos) {
                continue;
            }
            if self.get_coord(pos) == Some('#') {
                continue;
            }
            
            visited.insert(pos);
            room.push(pos);
            
            for neighbor in self.neighbors_4(pos) {
                if !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
        
        room
    }
    
    /// Day 24: Component analysis
    pub fn analyze_component(&self, component: &[Coord]) -> ComponentInfo {
        let mut min_row = isize::MAX;
        let mut max_row = isize::MIN;
        let mut min_col = isize::MAX;
        let mut max_col = isize::MIN;
        let mut perimeter = 0;
        
        let component_set: HashSet<_> = component.iter().copied().collect();
        
        for &pos in component {
            min_row = min_row.min(pos.row);
            max_row = max_row.max(pos.row);
            min_col = min_col.min(pos.col);
            max_col = max_col.max(pos.col);
            
            for neighbor in self.neighbors_4(pos) {
                if !component_set.contains(&neighbor) {
                    perimeter += 1;
                }
            }
        }
        
        ComponentInfo {
            size: component.len(),
            perimeter,
            bounds: (
                Coord::new(min_row, min_col),
                Coord::new(max_row, max_col),
            ),
        }
    }
    
    /// Day 27: Terrain analysis
    pub fn analyze_terrain(&self) -> TerrainStats {
        let mut walls = 0;
        let mut floor = 0;
        let mut water = 0;
        
        for &cell in &self.data {
            match cell {
                '#' => walls += 1,
                '.' | 'S' | 'G' => floor += 1,
                '~' => water += 1,
                _ => {}
            }
        }
        
        TerrainStats {
            total: self.data.len(),
            walls,
            floor,
            water,
        }
    }
    
    pub fn visualize_path(&self, path: &[Coord]) {
        let path_set: HashSet<_> = path.iter().copied().collect();
        
        for row in 0..self.height {
            for col in 0..self.width {
                let pos = Coord::new(row as isize, col as isize);
                let cell = self[(row, col)];
                
                if path_set.contains(&pos) && cell != 'S' && cell != 'G' {
                    print!("*");
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }
    }
}

impl std::ops::Index<(usize, usize)> for DungeonGrid {
    type Output = char;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[self.index(row, col)]
    }
}

#[derive(Debug)]
pub struct ComponentInfo {
    pub size: usize,
    pub perimeter: usize,
    pub bounds: (Coord, Coord),
}

#[derive(Debug)]
pub struct TerrainStats {
    pub total: usize,
    pub walls: usize,
    pub floor: usize,
    pub water: usize,
}
```

### 🛠️ How to Run This Code:

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day28_demo.rs` and run `rustc day28_demo.rs && ./day28_demo`
3. **In this workspace**: Create a new cargo project and paste into `src/main.rs`

---

## 🎓 Concept Integration Map

### How Week 4 Concepts Connect

```
┌─────────────────────────────────────────────────────────┐
│                   Week 4 Integration                     │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  Day 27: String Parsing                                 │
│     ↓ Converts text to...                               │
│                                                          │
│  Day 22: Grid Data Structure                            │
│     ↓ Stored and accessed via...                        │
│                                                          │
│  Day 23: Navigation & Direction                         │
│     ↓ Enables traversal for...                          │
│                                                          │
│  Day 24: Flood Fill & Components                        │
│     ↓ Finds regions, feeds into...                      │
│                                                          │
│  Day 25: BFS (Unweighted Shortest Path)                 │
│     ↓ Special case of...                                │
│                                                          │
│  Day 26: Dijkstra (Weighted Shortest Path)              │
│     ↓ Uses priority queue from...                       │
│                                                          │
│  Day 26: Advanced Queue Patterns                        │
│     ↓ Supports all traversal algorithms                 │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### Data Flow in Complete Solution

```
Input String
    ↓
[Day 27 Parsing]
    ↓
Grid<char> + Metadata
    ↓
[Day 23 Navigation] → Neighbors
    ↓
[Day 24 Components] → Room Detection
    ↓
[Day 25 BFS] → Distance Map (unweighted)
    ↓
[Day 26 Dijkstra] → Shortest Path (weighted)
    ↓
Complete Solution
```

---

## 💡 Key Patterns for AoC Success

### Pattern 1: Grid Problem Template

```rust
// 1. Parse input
let grid = Grid::parse(input);

// 2. Find starting positions
let start = grid.find('S');

// 3. Apply algorithm
let result = match problem_type {
    "shortest_path" => grid.bfs(start, goal),
    "regions" => grid.find_components(),
    "weighted_path" => grid.dijkstra(start, goal),
};

// 4. Process result
analyze_and_output(result);
```

### Pattern 2: Multi-Phase Processing

```rust
// Phase 1: Parse and validate
let grid = Grid::parse(input)?;

// Phase 2: Precompute structures
let distances = grid.bfs_distances(start);
let rooms = grid.find_rooms();

// Phase 3: Query efficiently
for query in queries {
    let result = distances.get(&query);
    // O(1) lookup thanks to precomputation
}
```

### Pattern 3: Performance Optimization

```rust
// ❌ Inefficient: Repeated work
for query in queries {
    let path = grid.bfs(start, query); // O(V+E) each
}

// ✅ Efficient: Precompute once
let all_paths = grid.bfs_distances(start); // O(V+E) once
for query in queries {
    let distance = all_paths.get(&query); // O(1) each
}
```

---

## 📊 Week 4 Performance Summary

### Algorithm Complexity Chart

| Algorithm | Time | Space | Use Case |
|-----------|------|-------|----------|
| Grid Access | O(1) | O(W×H) | Any grid operation |
| Neighbor Gen | O(1) | O(1) | Navigation step |
| BFS | O(V+E) | O(V) | Unweighted shortest path |
| Dijkstra | O((V+E) log V) | O(V) | Weighted shortest path |
| Flood Fill | O(V+E) | O(V) | Region coloring |
| Components | O(V+E) | O(V) | Finding all regions |
| Parse Split | O(n) | O(n) | Simple delimiter |
| Regex | O(n) | O(m) | Complex patterns |

### When to Use What

**BFS vs Dijkstra:**
- All edges cost 1? → BFS (faster)
- Different edge costs? → Dijkstra (correct)

**Recursive vs Iterative Flood Fill:**
- Small regions (< 1000 cells)? → Recursive (simpler)
- Large regions? → Iterative (stack-safe)

**Split vs Regex:**
- Simple delimiter? → `split()` (faster)
- Complex pattern? → Regex (more powerful)

**VecDeque vs BinaryHeap:**
- FIFO queue? → VecDeque
- Priority-based? → BinaryHeap

---

## 🚀 Mission 6 Preview

### What You're Ready For

With Week 4 complete, you have everything needed for **Mission 6: Advanced Pathfinding**:

✅ **Grid fundamentals** - Store and access 2D data efficiently  
✅ **Navigation** - Generate neighbors and handle movement  
✅ **BFS** - Shortest paths in unweighted graphs  
✅ **Dijkstra** - Shortest paths in weighted graphs  
✅ **Flood Fill** - Region detection and analysis  
✅ **Input Parsing** - Handle real-world problem inputs  

**Mission 6 Requirements You Can Now Satisfy:**
- REQ-1: Grid data structure ✅ (Day 22)
- REQ-2: BFS pathfinding ✅ (Day 25)
- REQ-3: Dijkstra for weighted graphs ✅ (Day 26)
- REQ-4: Component detection ✅ (Day 24)
- REQ-5: Input parsing utilities ✅ (Day 27)

---

## 🎯 Practice Challenges

### Challenge 1: Island Hopping
```
Given a grid with islands ('L') and water ('W'), find:
1. Number of islands (Day 24)
2. Shortest path between two islands (Day 25)
3. If water costs 5 and land costs 1, cheapest path (Day 26)
```

### Challenge 2: Maze Runner
```
Parse a maze from input:
- Find all dead ends
- Calculate longest shortest path (diameter)
- Find the center point (equidistant from all walls)
```

### Challenge 3: Multi-Terrain Pathfinding
```
Grid with different terrain types:
- Plains (cost 1)
- Forest (cost 2)
- Mountain (cost 4)
- River (cost 10)

Find optimal path considering costs.
```

---

## 📚 Further Study

### Advanced Topics (Beyond Week 4)

**Week 5 Preview:**
- A* pathfinding (heuristic search)
- Graph representations (adjacency lists)
- Topological sorting
- Minimum spanning trees

**Mission 7 Preview:**
- Graph data structures
- DFS applications
- Cycle detection
- Strongly connected components

---

## 💡 Final Key Takeaways

### The Week 4 Toolkit

1. **Grids** - Universal 2D data structure for spatial problems
2. **Navigation** - Systematic neighbor generation and movement
3. **Traversal** - BFS for unweighted, Dijkstra for weighted
4. **Analysis** - Flood fill and components for region detection
5. **Parsing** - Transform text into structured data

### Success Patterns

**Pattern: Start Simple**
```rust
// 1. Get it working
let path = bfs_simple(grid, start, goal);

// 2. Make it correct
let path = bfs_with_validation(grid, start, goal)?;

// 3. Make it fast
let path = bfs_optimized(grid, start, goal)?;
```

**Pattern: Progressive Enhancement**
```rust
// Level 1: Basic grid
struct Grid { data: Vec<char> }

// Level 2: Add navigation
impl Grid { fn neighbors(&self) -> Vec<Coord> }

// Level 3: Add algorithms
impl Grid { fn bfs(&self) -> HashMap<Coord, usize> }

// Level 4: Add parsing
impl Grid { fn parse(input: &str) -> Self }
```

### Debugging Strategies

**Visualize Your Data:**
```rust
// Print grid with highlights
fn visualize_path(grid: &Grid, path: &[Coord]) {
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            let pos = Coord::new(row, col);
            if path.contains(&pos) {
                print!("*");
            } else {
                print!("{}", grid[(row, col)]);
            }
        }
        println!();
    }
}
```

**Test Edge Cases:**
```rust
#[test]
fn test_edge_cases() {
    // Empty grid
    let grid = Grid::new(0, 0);
    
    // Single cell
    let grid = Grid::new(1, 1);
    
    // All walls
    let grid = Grid::filled('#', 10, 10);
    
    // Start equals goal
    let path = grid.bfs(pos, pos);
}
```

---

## 🎓 Week 4 Complete!

**Congratulations!** You've built a complete algorithmic toolkit for spatial problem-solving. You can now:

✅ Parse complex inputs efficiently  
✅ Represent and navigate 2D spaces  
✅ Apply appropriate traversal algorithms  
✅ Analyze regions and components  
✅ Optimize paths with different costs  
✅ Integrate multiple concepts into complete solutions  

**Next Steps:**
1. Review any concepts that need reinforcement
2. Practice with real AoC problems from previous years
3. Start Mission 6 with confidence
4. Prepare for Week 5 advanced topics

**You're now equipped to tackle sophisticated grid-based problems!** 🚀

---

*Tags: #week4-review #integration #grids #bfs #dijkstra #flood-fill #parsing #mission6 #aoc-patterns #problem-solving*
*Links: [[daily-study/Day27]] ← | [[../missions/Mission6/README|Mission6]] | [[zettel-index]] | [[Week 5 Preview]] →*
