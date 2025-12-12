# 🗺️ Pathfinding Strategies: Choosing the Right Algorithm

*A decision guide for selecting optimal pathfinding algorithms based on graph characteristics and requirements*

*Tags: #pathfinding #algorithms #graph-theory #decision-guide #performance #mission-9*  
*Links: [[zettel-index]] | [[graph-algorithms]] | [[Dijkstra Algorithm]] | [[Bellman-Ford Algorithm]] | [[A-Star-Algorithm-Deep-Dive|A* Algorithm]] | [[BFS Patterns]] | [[missions/mission-9]]*

---

## 🎯 Overview

Choosing the right pathfinding algorithm is critical for performance and correctness. The wrong choice can mean:
- **Incorrect results** (e.g., Dijkstra with negative edges)
- **Timeout** (e.g., BFS on weighted graphs with large weights)
- **Wasted computation** (e.g., A* without a good heuristic)

This guide helps you select the optimal algorithm for your specific problem.

---

## 📊 Quick Decision Matrix

| Scenario | Best Algorithm | Why |
|----------|---------------|-----|
| Unweighted graph | **BFS** | O(V+E), guaranteed shortest |
| Non-negative weights | **Dijkstra** | O(E log V), optimal |
| Negative edges (no neg cycles) | **Bellman-Ford** | O(VE), handles negatives |
| Known goal + good heuristic | **A*** | Often faster than Dijkstra |
| Grid with obstacles | **A*** (Manhattan) | Heuristic exploits structure |
| Need all-pairs shortest | **Floyd-Warshall** | O(V³), complete solution |
| Sparse graph, single query | **Dijkstra/A*** | Better than Floyd-Warshall |
| Detect negative cycles | **Bellman-Ford** | Built-in detection |

---

## 🔍 Decision Flowchart

```
Start: Need shortest path?
    │
    ▼
Are edge weights present?
    │
    ├── NO → Use BFS (O(V+E))
    │
    └── YES → Can weights be negative?
                    │
                    ├── NO → Do you have a goal node with admissible heuristic?
                    │           │
                    │           ├── YES → Use A* (often faster)
                    │           │
                    │           └── NO → Use Dijkstra (O(E log V))
                    │
                    └── YES → Need to detect negative cycles?
                                │
                                ├── YES → Use Bellman-Ford (O(VE))
                                │
                                └── NO → Use Bellman-Ford anyway
                                         (Dijkstra gives WRONG results!)
```

---

## 📚 Algorithm Deep Dives

### **BFS - Breadth-First Search**

**When to use**: Unweighted graphs or unit-weight edges

```rust
// BFS: Perfect for unweighted shortest paths
fn bfs_shortest_path(graph: &Graph, start: NodeId, goal: NodeId) -> Option<Vec<NodeId>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = HashMap::new();
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(node) = queue.pop_front() {
        if node == goal {
            return Some(reconstruct_path(&parent, goal));
        }
        
        for neighbor in graph.neighbors(node) {
            if visited.insert(neighbor) {
                parent.insert(neighbor, node);
                queue.push_back(neighbor);
            }
        }
    }
    None
}
```

**Complexity**: O(V + E)  
**Space**: O(V)  
**Guarantees**: Shortest path in unweighted graphs

### **Dijkstra's Algorithm**

**When to use**: Weighted graphs with **non-negative** edges

```rust
// Dijkstra: Greedy with priority queue
fn dijkstra(graph: &Graph, start: NodeId, goal: NodeId) -> Option<(Weight, Vec<NodeId>)> {
    let mut distances: HashMap<NodeId, Weight> = HashMap::new();
    let mut heap = BinaryHeap::new();
    
    distances.insert(start, 0);
    heap.push(Reverse((0, start)));  // Min-heap via Reverse
    
    while let Some(Reverse((dist, node))) = heap.pop() {
        if node == goal {
            return Some((dist, reconstruct_path(...)));
        }
        
        if dist > *distances.get(&node).unwrap_or(&Weight::MAX) {
            continue;  // Already found better path
        }
        
        for (neighbor, weight) in graph.edges(node) {
            let new_dist = dist + weight;
            if new_dist < *distances.get(&neighbor).unwrap_or(&Weight::MAX) {
                distances.insert(neighbor, new_dist);
                heap.push(Reverse((new_dist, neighbor)));
            }
        }
    }
    None
}
```

**Complexity**: O((V + E) log V) with binary heap  
**Space**: O(V)  
**⚠️ WARNING**: Produces **incorrect results** with negative edges!

### **Bellman-Ford Algorithm**

**When to use**: Graphs with negative edges OR need cycle detection

```rust
// Bellman-Ford: Handles negative edges
fn bellman_ford(graph: &Graph, start: NodeId) -> Result<HashMap<NodeId, Weight>, &'static str> {
    let mut distances: HashMap<NodeId, Weight> = HashMap::new();
    distances.insert(start, 0);
    
    // Relax all edges V-1 times
    for _ in 0..graph.node_count() - 1 {
        for (u, v, weight) in graph.all_edges() {
            if let Some(&dist_u) = distances.get(&u) {
                let new_dist = dist_u + weight;
                if new_dist < *distances.get(&v).unwrap_or(&Weight::MAX) {
                    distances.insert(v, new_dist);
                }
            }
        }
    }
    
    // Check for negative cycles (one more iteration)
    for (u, v, weight) in graph.all_edges() {
        if let Some(&dist_u) = distances.get(&u) {
            if dist_u + weight < *distances.get(&v).unwrap_or(&Weight::MAX) {
                return Err("Negative cycle detected!");
            }
        }
    }
    
    Ok(distances)
}
```

**Complexity**: O(V × E)  
**Space**: O(V)  
**Unique feature**: Detects negative cycles

### **A* Search Algorithm**

**When to use**: Known goal + admissible heuristic available

```rust
// A*: Heuristic-guided search
fn a_star(
    graph: &Graph, 
    start: NodeId, 
    goal: NodeId,
    heuristic: impl Fn(NodeId) -> Weight
) -> Option<(Weight, Vec<NodeId>)> {
    let mut g_score: HashMap<NodeId, Weight> = HashMap::new();
    let mut heap = BinaryHeap::new();
    
    g_score.insert(start, 0);
    let f_start = heuristic(start);
    heap.push(Reverse((f_start, start)));
    
    while let Some(Reverse((_, node))) = heap.pop() {
        if node == goal {
            return Some((g_score[&goal], reconstruct_path(...)));
        }
        
        let current_g = g_score[&node];
        
        for (neighbor, weight) in graph.edges(node) {
            let tentative_g = current_g + weight;
            
            if tentative_g < *g_score.get(&neighbor).unwrap_or(&Weight::MAX) {
                g_score.insert(neighbor, tentative_g);
                let f = tentative_g + heuristic(neighbor);
                heap.push(Reverse((f, neighbor)));
            }
        }
    }
    None
}
```

**Complexity**: O(E log V) best case, O(b^d) worst case  
**Space**: O(V)  
**Key**: Heuristic must be **admissible** (never overestimate)

---

## 🎮 Common Heuristics for A*

### **Grid-Based Problems**

| Heuristic | Formula | Best For |
|-----------|---------|----------|
| Manhattan | `abs(dx) + abs(dy)` | 4-directional movement |
| Euclidean | `sqrt(dx² + dy²)` | Any-angle movement |
| Chebyshev | `max(abs(dx), abs(dy))` | 8-directional movement |
| Octile | `max(dx,dy) + (√2-1)*min(dx,dy)` | 8-dir with diagonals |

```rust
// Manhattan distance - perfect for grid problems like AoC
fn manhattan(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

// Euclidean - for continuous space or any-angle
fn euclidean(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}
```

---

## 🏆 Advent of Code Applications

| AoC Pattern | Recommended Algorithm |
|-------------|----------------------|
| Grid maze, find shortest path | BFS (unweighted) or Dijkstra (weighted) |
| Grid with varying terrain costs | Dijkstra or A* with Manhattan |
| State-space search | BFS or Dijkstra (states as nodes) |
| Find all reachable positions | BFS flood fill |
| Minimum steps with constraints | BFS with state encoding |

### **Example: AoC Grid Navigation**

```rust
// Common AoC pattern: Grid with movement costs
use mission6::Grid;  // Reuse validated Grid component!

fn solve_grid_pathfinding(grid: &Grid<char>, start: Pos, goal: Pos) -> Option<usize> {
    // If all moves cost 1, use BFS
    if uniform_costs(grid) {
        return bfs(grid, start, goal);
    }
    
    // If we have varying costs and a clear goal, use A*
    let heuristic = |pos: Pos| manhattan(pos, goal);
    a_star(grid, start, goal, heuristic)
}
```

---

## ⚠️ Common Pitfalls

### **1. Using Dijkstra with Negative Edges**
```rust
// ❌ WRONG - Dijkstra fails with negative edges
let graph = vec![
    (A, B, 1),
    (B, C, -5),  // Negative edge!
    (A, C, 2),
];
// Dijkstra might return A→C (cost 2) instead of A→B→C (cost -4)

// ✅ CORRECT - Use Bellman-Ford
let result = bellman_ford(&graph, A);
```

### **2. Non-Admissible Heuristic in A***
```rust
// ❌ WRONG - Overestimating heuristic
fn bad_heuristic(pos: Pos, goal: Pos) -> i32 {
    manhattan(pos, goal) * 2  // Overestimates!
}
// A* may not find optimal path

// ✅ CORRECT - Admissible heuristic
fn good_heuristic(pos: Pos, goal: Pos) -> i32 {
    manhattan(pos, goal)  // Never overestimates
}
```

### **3. BFS on Weighted Graphs**
```rust
// ❌ WRONG - BFS ignores weights
// Graph: A--1-->B--1-->C, A--10-->C
// BFS finds A→C (1 edge) but misses that A→B→C costs 2 vs 10

// ✅ CORRECT - Use Dijkstra for weighted graphs
```

---

## 🔗 Related Concepts

### **Algorithm Implementations**
- [[Dijkstra Algorithm]] - Complete implementation guide
- [[Bellman-Ford Algorithm]] - Negative edge handling
- [[A-Star-Algorithm-Deep-Dive|A* Algorithm]] - Heuristic-guided search
- [[BFS Patterns]] - Breadth-first traversal patterns

### **Data Structures**
- [[Priority Queue Patterns]] - Efficient frontier management
- [[Binary Heap Data Structure]] - Min/max heap for Dijkstra
- [[graph-representation]] - Adjacency list vs matrix

### **Mission Integration**
- [[missions/mission-9]] - Pathfinding algorithms implementation
- [[mission-8]] - Graph traversal foundation (BFS/DFS)
- [[mission-6]] - Grid infrastructure for 2D pathfinding

### **Applications**
- [[AoC Optimization Patterns]] - Advent of Code pathfinding problems
- [[Game AI]] - NPC navigation and movement
- [[Network Routing]] - Packet routing algorithms

---

*Links: [[zettel-index]] | [[graph-algorithms]] | [[Dijkstra Algorithm]] | [[Bellman-Ford Algorithm]] | [[A-Star-Algorithm-Deep-Dive|A* Algorithm]] | [[BFS Patterns]] | [[missions/mission-9]] | [[mission-8]]*

*Tags: #pathfinding #algorithms #graph-theory #dijkstra #bellman-ford #a-star #bfs #decision-guide #aoc*
