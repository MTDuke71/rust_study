# 🌊 BFS Patterns - Breadth-First Search

**Level-by-level exploration for shortest paths and reachability analysis**

## 🎯 Core Concept

**Breadth-First Search (BFS)** is a graph/grid traversal algorithm that explores nodes level by level, visiting all neighbors at distance `d` before moving to distance `d + 1`. It uses a **queue** (FIFO) data structure.

**Key Properties:**
- ✅ **Finds shortest path** in unweighted graphs
- ✅ **Explores level-by-level** (systematic layer exploration)
- ✅ **First visit = shortest distance** (optimal for unweighted)
- ✅ **Complete** (explores all reachable nodes)

**When to Use BFS:**
- Need shortest path in unweighted graph/grid
- Want to compute distances from source
- Exploring nodes by proximity/distance layers
- Finding connected components
- Level-order traversal needed

---

## 📐 BFS Algorithm Template

### **Basic BFS Structure**
```rust
use std::collections::VecDeque;

fn bfs(grid: &[Vec<bool>], start: Point) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(current) = queue.pop_front() {
        // Process current node
        println!("Visiting: {:?}", current);
        
        // Explore neighbors
        for neighbor in get_neighbors(current, grid) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
}
```

### **BFS with Distance Tracking**
```rust
fn bfs_with_distance(
    grid: &[Vec<bool>],
    start: Point,
) -> HashMap<Point, u32> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    
    queue.push_back((start, 0));
    distances.insert(start, 0);
    
    while let Some((current, dist)) = queue.pop_front() {
        for neighbor in get_neighbors(current, grid) {
            if !distances.contains_key(&neighbor) {
                distances.insert(neighbor, dist + 1);
                queue.push_back((neighbor, dist + 1));
            }
        }
    }
    
    distances
}
```

### **BFS Shortest Path**
```rust
fn bfs_shortest_path(
    grid: &[Vec<bool>],
    start: Point,
    goal: Point,
) -> Option<Vec<Point>> {
    let mut queue = VecDeque::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut visited = HashSet::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(current) = queue.pop_front() {
        if current == goal {
            return Some(reconstruct_path(&came_from, start, goal));
        }
        
        for neighbor in get_neighbors(current, grid) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                came_from.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }
    
    None  // No path found
}

fn reconstruct_path(
    came_from: &HashMap<Point, Point>,
    start: Point,
    goal: Point,
) -> Vec<Point> {
    let mut path = vec![goal];
    let mut current = goal;
    
    while current != start {
        current = came_from[&current];
        path.push(current);
    }
    
    path.reverse();
    path
}
```

---

## 🎨 Common BFS Patterns

### **Pattern 1: Multi-Source BFS**
*Start BFS from multiple sources simultaneously*

```rust
fn multi_source_bfs(
    grid: &[Vec<bool>],
    sources: &[Point],
) -> HashMap<Point, u32> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    
    // Initialize with all sources at distance 0
    for &source in sources {
        queue.push_back((source, 0));
        distances.insert(source, 0);
    }
    
    while let Some((current, dist)) = queue.pop_front() {
        for neighbor in get_neighbors(current, grid) {
            if !distances.contains_key(&neighbor) {
                distances.insert(neighbor, dist + 1);
                queue.push_back((neighbor, dist + 1));
            }
        }
    }
    
    distances
}

// Use case: Find distance to nearest hospital, fire station, etc.
```

### **Pattern 2: Level-Order Processing**
*Process all nodes at each distance level together*

```rust
fn bfs_by_levels(grid: &[Vec<bool>], start: Point) -> Vec<Vec<Point>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut levels = Vec::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while !queue.is_empty() {
        let level_size = queue.len();
        let mut current_level = Vec::new();
        
        for _ in 0..level_size {
            let current = queue.pop_front().unwrap();
            current_level.push(current);
            
            for neighbor in get_neighbors(current, grid) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
        
        levels.push(current_level);
    }
    
    levels
}

// Use case: Binary tree level-order traversal, wave simulation
```

### **Pattern 3: BFS with State**
*Track additional state alongside position*

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: Point,
    keys_collected: u32,  // Bitmask of collected keys
}

fn bfs_with_state(
    grid: &[Vec<char>],
    start: Point,
) -> Option<u32> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    let initial = State {
        position: start,
        keys_collected: 0,
    };
    
    queue.push_back((initial, 0));
    visited.insert(initial);
    
    while let Some((state, steps)) = queue.pop_front() {
        // Check win condition
        if all_keys_collected(state.keys_collected) {
            return Some(steps);
        }
        
        for neighbor in get_neighbors(state.position, grid) {
            let mut new_state = state;
            new_state.position = neighbor;
            
            // Update state based on cell content
            if is_key(grid, neighbor) {
                new_state.keys_collected |= key_bit(grid, neighbor);
            }
            
            if !visited.contains(&new_state) {
                visited.insert(new_state);
                queue.push_back((new_state, steps + 1));
            }
        }
    }
    
    None
}

// Use case: Puzzles with collectibles, keys and doors
```

### **Pattern 4: 0-1 BFS**
*BFS variant for graphs with weights 0 and 1*

```rust
fn zero_one_bfs(
    graph: &HashMap<Point, Vec<(Point, u32)>>,  // (neighbor, cost)
    start: Point,
) -> HashMap<Point, u32> {
    let mut deque = VecDeque::new();
    let mut distances = HashMap::new();
    
    deque.push_back(start);
    distances.insert(start, 0);
    
    while let Some(current) = deque.pop_front() {
        let current_dist = distances[&current];
        
        for &(neighbor, cost) in &graph[&current] {
            let new_dist = current_dist + cost;
            
            if !distances.contains_key(&neighbor) || new_dist < distances[&neighbor] {
                distances.insert(neighbor, new_dist);
                
                if cost == 0 {
                    deque.push_front(neighbor);  // Free edge: front of queue
                } else {
                    deque.push_back(neighbor);   // Cost 1: back of queue
                }
            }
        }
    }
    
    distances
}

// Use case: Maze with portals (free teleport = cost 0, walk = cost 1)
```

### **Pattern 5: Bidirectional BFS**
*Search from both start and goal simultaneously*

```rust
fn bidirectional_bfs(
    grid: &[Vec<bool>],
    start: Point,
    goal: Point,
) -> Option<u32> {
    let mut queue_start = VecDeque::new();
    let mut queue_goal = VecDeque::new();
    
    let mut visited_start: HashMap<Point, u32> = HashMap::new();
    let mut visited_goal: HashMap<Point, u32> = HashMap::new();
    
    queue_start.push_back((start, 0));
    queue_goal.push_back((goal, 0));
    visited_start.insert(start, 0);
    visited_goal.insert(goal, 0);
    
    while !queue_start.is_empty() || !queue_goal.is_empty() {
        // Expand from start
        if let Some((current, dist)) = queue_start.pop_front() {
            if let Some(&goal_dist) = visited_goal.get(&current) {
                return Some(dist + goal_dist);  // Paths met!
            }
            
            for neighbor in get_neighbors(current, grid) {
                if !visited_start.contains_key(&neighbor) {
                    visited_start.insert(neighbor, dist + 1);
                    queue_start.push_back((neighbor, dist + 1));
                }
            }
        }
        
        // Expand from goal
        if let Some((current, dist)) = queue_goal.pop_front() {
            if let Some(&start_dist) = visited_start.get(&current) {
                return Some(dist + start_dist);  // Paths met!
            }
            
            for neighbor in get_neighbors(current, grid) {
                if !visited_goal.contains_key(&neighbor) {
                    visited_goal.insert(neighbor, dist + 1);
                    queue_goal.push_back((neighbor, dist + 1));
                }
            }
        }
    }
    
    None
}

// Use case: Long paths in large graphs - can be ~2x faster
```

---

## 🎮 AoC BFS Patterns

### **AoC Pattern 1: Maze Shortest Path**
```rust
// Classic maze solving
fn solve_maze(maze: &[Vec<char>]) -> u32 {
    let start = find_char(maze, 'S');
    let goal = find_char(maze, 'E');
    
    bfs_shortest_path_length(maze, start, goal).unwrap()
}
```

### **AoC Pattern 2: Flood Fill with Distance**
```rust
// Water spreading from sources
fn flood_simulation(grid: &mut [Vec<char>], water_sources: &[Point]) {
    let distances = multi_source_bfs(grid, water_sources);
    
    for (point, dist) in distances {
        grid[point.y][point.x] = char::from_digit(dist % 10, 10).unwrap();
    }
}
```

### **AoC Pattern 3: Collect All Items**
```rust
// Collect all keys/coins in minimum steps
fn collect_all_keys(dungeon: &[Vec<char>], start: Point) -> u32 {
    bfs_with_state(dungeon, start).unwrap()
}
```

### **AoC Pattern 4: Level Counting**
```rust
// Count nodes at each distance
fn count_by_distance(grid: &[Vec<bool>], start: Point) -> Vec<usize> {
    let levels = bfs_by_levels(grid, start);
    levels.iter().map(|level| level.len()).collect()
}
```

---

## 📊 BFS vs DFS Comparison

| Aspect | BFS | [[DFS Patterns\|DFS]] |
|--------|-----|-----|
| **Data Structure** | Queue (FIFO) | Stack (LIFO) or recursion |
| **Exploration** | Layer by layer | Deep first, backtrack |
| **Shortest Path** | ✅ Yes (unweighted) | ❌ No |
| **Memory** | O(width) - high | O(depth) - low |
| **Complete** | ✅ Yes | ✅ Yes (with cycle detection) |
| **Use Case** | Shortest path, levels | Exhaustive search, backtracking |

---

## ⚡ Performance Optimization

### **1. Use VecDeque Instead of Vec**
```rust
// ❌ Slow: O(n) pop from front
let mut queue: Vec<Point> = Vec::new();
let current = queue.remove(0);  // Expensive!

// ✅ Fast: O(1) pop from front
let mut queue: VecDeque<Point> = VecDeque::new();
let current = queue.pop_front().unwrap();  // Efficient!
```

### **2. Pre-allocate Visited Set**
```rust
// For grid: use Vec<Vec<bool>> instead of HashSet
let mut visited = vec![vec![false; width]; height];
visited[y][x] = true;  // O(1) access
```

### **3. Early Exit**
```rust
// Stop as soon as goal is found
if current == goal {
    return Some(distance);  // Don't continue exploring
}
```

### **4. Avoid Redundant Checks**
```rust
// Mark visited when adding to queue, not when popping
if !visited.contains(&neighbor) {
    visited.insert(neighbor);
    queue.push_back(neighbor);  // Mark visited here
}
```

---

## 🎯 Real-World Applications

1. **Social Networks** - Degrees of separation ("6 degrees of Kevin Bacon")
2. **GPS Navigation** - Shortest path in road networks (unweighted)
3. **Web Crawling** - Breadth-first page exploration
4. **Network Broadcasting** - Packet routing to all nodes
5. **Game AI** - Pathfinding for NPCs
6. **Puzzle Solvers** - Rubik's cube, sliding puzzles
7. **Circuit Design** - Testing connectivity

---

## 🔗 Connected Concepts

### **Related Zettelkasten Pages**
- [[DFS Patterns]] - Depth-first alternative
- [[A-Star-Algorithm-Deep-Dive]] - Heuristic-guided BFS improvement
- [[Manhattan Distance]] - Distance metric for grids
- [[mission-2]] - Queue implementation for BFS
- [[mission-6]] - Grid pathfinding with BFS
- [[Week 4 Overview]] - Day 25 covers BFS applications
- [[directed-vs-undirected-graphs]] - Understanding graph types for BFS

### **Related Data Structures**
- [[mission-2]] - Queue/VecDeque for BFS
- Priority Queue for weighted BFS (Dijkstra)

### **Algorithm Family**
- **Uninformed Search**: BFS, DFS, Uniform Cost
- **Informed Search**: [[A-Star-Algorithm-Deep-Dive]], Greedy Best-First
- **Optimal**: BFS (unweighted), Dijkstra (weighted), A*

---

## 💡 Key Takeaways

1. **BFS = Level-by-Level**: Systematic exploration by distance
2. **Shortest Path Guarantee**: In unweighted graphs
3. **Queue is Essential**: FIFO ensures level-order
4. **First Visit = Shortest**: No need to revisit nodes
5. **Memory Trade-Off**: Uses more memory than DFS
6. **Perfect for Grids**: Natural fit for grid pathfinding

**When to Use BFS:**
```
✅ Need shortest path (unweighted)
✅ Want distance from source
✅ Level-order traversal needed
✅ Find connected components
✅ Grid pathfinding (4/8-connected)

❌ Weighted graphs → Use Dijkstra or A*
❌ Memory constrained → Use DFS or IDA*
❌ Need exhaustive search → Use DFS
❌ Backtracking needed → Use DFS
```

**BFS Philosophy:**
> "Explore near before far. In an unweighted world, the first path found is the shortest path. BFS embodies this wisdom." 🌊

---

*Tags: #bfs #breadth-first-search #shortest-path #level-order #graph-algorithms #grid-navigation #queue #aoc-patterns*

*Links: [[zettel-index]] | [[DFS Patterns]] | [[A-Star-Algorithm-Deep-Dive]] | [[Manhattan Distance]] | [[mission-2]] | [[mission-6]] | [[mission-8]] | [[Day 5 Exercise Solutions]] | [[Week 4 Overview]]*
