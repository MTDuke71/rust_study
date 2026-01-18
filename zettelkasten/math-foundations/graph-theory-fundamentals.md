# Graph Theory Fundamentals

**Field**: Discrete Mathematics / Graph Theory

**Prerequisites**: [[set-theory-fundamentals]]

---

## 📐 Definition

A **graph** G = (V, E) consists of:
- **V**: A set of vertices (nodes)
- **E**: A set of edges (connections between vertices)

**Intuition**: A graph models relationships between objects - think social networks, road maps, dependencies.

---

## 🔑 Key Concepts

### **Graph Types**

#### **Directed Graph (Digraph)**
- Edges have direction: edge (u, v) goes from u to v
- Example: One-way streets, dependencies, state transitions

#### **Undirected Graph**
- Edges have no direction: edge {u, v} connects both ways
- Example: Friendship networks, bidirectional roads

#### **Weighted Graph**
- Edges have associated costs/weights
- Example: Road networks with distances

### **Graph Properties**

#### **Path**
- **Definition**: Sequence of vertices where each adjacent pair is connected by an edge
- **Simple path**: No repeated vertices
- **Cycle**: Path that starts and ends at the same vertex

#### **Connectivity**
- **Connected graph**: Path exists between any two vertices
- **Strongly connected** (directed): Path exists in both directions between any vertices
- **Weakly connected** (directed): Underlying undirected graph is connected

#### **Degree**
- **In-degree**: Number of incoming edges to a vertex (directed graphs)
- **Out-degree**: Number of outgoing edges from a vertex (directed graphs)
- **Degree**: Number of edges connected to a vertex (undirected graphs)

---

## 🔍 Graph Traversal Algorithms

### **Breadth-First Search (BFS)**
- **Strategy**: Explore neighbors level by level
- **Data structure**: Queue (FIFO)
- **Complexity**: O(V + E)
- **Applications**: 
  - Shortest path in unweighted graphs
  - Level-order traversal
  - Connected components
- **Used in**: Mission 8 (Graph trait), AoC grid pathfinding

### **Depth-First Search (DFS)**
- **Strategy**: Explore as far as possible before backtracking
- **Data structure**: Stack (LIFO) or recursion
- **Complexity**: O(V + E)
- **Applications**:
  - Cycle detection
  - Topological sorting
  - Connected components
  - Maze solving

### **Cycle Detection**
- **Definition**: Determining if a graph contains a cycle
- **Methods**:
  - DFS with color marking (white/gray/black)
  - Track visited nodes during traversal
  - Back edge detection
- **Applications**: Deadlock detection, dependency checking

---

## 🛤️ Shortest Path Algorithms

### **Dijkstra's Algorithm**
- **Strategy**: Find shortest path in weighted graph with non-negative edge weights
- **Data structure**: Priority queue (min-heap)
- **Complexity**: O((V + E) log V) with binary heap
- **Key idea**: 
  - Greedily explore nodes by increasing distance from source
  - Once a node is processed, its shortest distance is final
  - Only works with non-negative weights
- **Applications**:
  - GPS routing
  - Network packet routing
  - Pathfinding in games with cost constraints

### **State-Space Dijkstra**
- **Extension**: When simple vertex tracking insufficient
- **State**: Tuple of (position, additional_constraints)
- **Key insight**: Same location can be visited multiple times with different states
- **Example states**:
  - (position, direction, consecutive_moves) - movement constraints
  - (position, keys_collected) - dependency tracking
  - (position, remaining_fuel) - resource constraints
- **Complexity**: O((V × S) log(V × S)) where S = state space size
- **Trade-off**: Exponentially larger state space, but handles constraints exactly

---

## 💻 Rust Implementations

### **Mission 8**: Graph Trait and Algorithms
- **What**: Generic graph trait with BFS/DFS implementations
- **How it uses this concept**: 
  - Abstract graph interface (`neighbors()` method)
  - BFS for shortest path finding
  - Generic over vertex types
- **Link**: [[mission-8]]

### **AoC 2023 Day 8**: Haunted Wasteland
- **What**: Navigate through directed graph following L/R instructions
- **How it uses this concept**:
  - Network represented as directed graph
  - Each node has exactly 2 outgoing edges (left, right)
  - Path traversal following instruction sequence
  - Cycle detection for ghost navigation
- **Link**: `advent_of_code/aoc2023/src/solver/day08.rs`
- **Performance**: ~1.5ms (Part 1), ~6.7ms (Part 2)

### **AoC 2023 Day 10**: Pipe Maze
- **What**: Navigate through pipe loop to find farthest point
- **How it uses this concept**:
  - Grid of pipes represented as undirected graph
  - Each pipe tile = vertex, connections = edges
  - BFS for loop traversal and distance calculation
  - Cycle detection (continuous loop structure)
  - Each node has degree 2 (exactly two connections in loop)
- **Link**: `advent_of_code/aoc2023/src/solver/day10.rs`
- **Performance**: 3.1ms (BFS on ~7000-node loop)
- **Integration**: Mission 6 Grid + Mission 8 BFS pattern

### **AoC 2024 Day 10**: Topographic Map (Example)
- **What**: Find hiking trails through elevation grid
- **How it uses this concept**:
  - Grid represented as graph (Mission 6 Grid + Mission 8 Graph trait)
  - BFS for pathfinding from trailheads
  - Connected component analysis
- **Integration**: Demonstrates mission composition (Grid + Graph)

### **AoC 2023 Day 17**: Clumsy Crucible
- **What**: Minimum heat loss pathfinding with movement constraints
- **How it uses this concept**:
  - State-space Dijkstra: state = (position, direction, consecutive_moves)
  - Weighted graph where edges = grid cells with heat loss values
  - Movement constraints: max 3 consecutive (Part 1), min 4/max 10 (Part 2)
  - Cannot reverse direction, must respect straight-line limits
- **Link**: `advent_of_code/aoc2023/src/solver/day17.rs`
- **Performance**: 64.3ms (Part 1), 182.4ms (Part 2)
- **Complexity**: O(V × D × C × log(V × D × C)) where V=cells, D=4 directions, C=max_consecutive
- **State space**: Part 1 ~239k states, Part 2 ~795k states (explains 2.8× slowdown)
- **Integration**: Mission 6 Grid for heat loss map representation
- **Key learning**: When position-only tracking insufficient, extend to full state tuples

---

## 📚 Code Examples

### Graph Representation
```rust
use std::collections::HashMap;

/// Network as directed graph with labeled edges
struct Network {
    /// Adjacency list: node -> (left_node, right_node)
    nodes: HashMap<String, (String, String)>,
}
```

**Mathematical foundation**:
- Adjacency list representation: O(V + E) space
- Each vertex maps to its neighbors
- Efficient for sparse graphs

### Graph Traversal
```rust
/// Navigate through graph following instruction sequence
fn navigate(&self, start: &str, end: &str) -> Result<usize> {
    let mut current = start.to_string();
    let mut steps = 0;
    
    while current != end {
        // Follow edge based on instruction
        let (left, right) = self.nodes.get(&current)?;
        current = match instruction {
            'L' => left.clone(),
            'R' => right.clone(),
        };
        steps += 1;
    }
    
    Ok(steps)
}
```

**Mathematical foundation**:
- Path finding in directed graph
- Deterministic edge selection (no search needed)
- Cycle possible (instructions repeat)

---

## 🌳 Related Concepts

- **Prerequisites**: 
  - [[set-theory-fundamentals]] - Graphs are sets of vertices and edges
- **Related**: 
  - [[number-theory-basics]] - Cycle length calculation (LCM)
  - [[complexity-theory]] - Algorithm analysis (BFS/DFS complexity)
- **Applications**:
  - [[mission-8]] - Graph trait implementation
  - Social network analysis
  - Routing algorithms (Dijkstra, A*)
  - Dependency resolution

---

## 📖 Resources

- [Wikipedia: Graph Theory](https://en.wikipedia.org/wiki/Graph_theory)
- [Wikipedia: Graph Traversal](https://en.wikipedia.org/wiki/Graph_traversal)
- [Introduction to Algorithms (CLRS), Chapter 22: Elementary Graph Algorithms](https://mitpress.mit.edu/9780262046305/introduction-to-algorithms/)

---

*Tags: #graph-theory #bfs #dfs #cycle-detection #directed-graph #traversal #math-foundations*

**Related Zettelkasten Links**:
- [[mission-8]] - Graph trait and algorithm implementations
- [[set-theory-fundamentals]] - Mathematical foundation for graphs
- [[number-theory-basics]] - Cycle analysis uses LCM
