# 🎯 Dijkstra's Algorithm - Optimal Shortest Path

*Single-source shortest path algorithm for graphs with non-negative edge weights*

---

## 📋 Core Concept

**Purpose**: Find the shortest path from a source node to all other nodes in a weighted graph  
**Creator**: Edsger W. Dijkstra (1956)  
**Key Insight**: Greedy approach with priority queue guarantees optimal shortest paths  
**Constraint**: **Requires non-negative edge weights** for correctness

### **The Fundamental Guarantee**

> "When a node is popped from the priority queue, the shortest path to that node has been found"

This guarantee enables **early termination** and makes Dijkstra efficient, but only works with non-negative edges.

---

## 🧮 Algorithm Description

### **High-Level Strategy**

1. Start at source node with distance 0
2. Maintain priority queue of nodes to explore (ordered by distance)
3. Always explore the closest unvisited node next
4. Update distances to neighbors if a shorter path is found
5. Stop when goal is reached (or all reachable nodes explored)

### **Core Data Structures**

```rust
// Priority Queue: Nodes to explore, ordered by distance
let mut queue: BinaryHeap<QueueNode> = BinaryHeap::new();

// Distances: Best known distance to each node
let mut distances: HashMap<NodeId, Weight> = HashMap::new();

// Previous: Track parent nodes for path reconstruction
let mut previous: HashMap<NodeId, NodeId> = HashMap::new();
```

### **The Magic Line: Min-Heap from Max-Heap**

```rust
impl Ord for QueueNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // REVERSED comparison - this is the key!
        // Transforms BinaryHeap (max-heap) into min-heap
        other.cost.partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}
```

By reversing the comparison (`other.cost` vs `self.cost`), the max-heap pops **lowest cost** nodes first.

---

## 🔄 Algorithm Steps

### **Detailed Execution Flow**

```rust
fn dijkstra(graph: &Graph, start: NodeId, goal: NodeId) -> Option<(f64, Vec<NodeId>)> {
    let mut queue = BinaryHeap::new();
    let mut distances = HashMap::new();
    let mut previous = HashMap::new();
    
    // Initialize: start node has distance 0
    distances.insert(start, 0.0);
    queue.push(QueueNode::new(start, 0.0));
    
    while let Some(current) = queue.pop() {
        // Early termination: found shortest path to goal!
        if current.node == goal {
            return Some((current.cost, reconstruct_path(&previous, goal)));
        }
        
        // Skip if we've found a better path already
        if current.cost > *distances.get(&current.node).unwrap_or(&f64::INFINITY) {
            continue;
        }
        
        // Explore neighbors
        for (neighbor, edge_weight) in graph.neighbors(current.node) {
            let new_distance = current.cost + edge_weight;
            
            // Found a better path to neighbor?
            if new_distance < *distances.get(&neighbor).unwrap_or(&f64::INFINITY) {
                distances.insert(neighbor, new_distance);
                previous.insert(neighbor, current.node);
                queue.push(QueueNode::new(neighbor, new_distance));
            }
        }
    }
    
    None // Goal unreachable
}
```

### **Path Reconstruction**

```rust
fn reconstruct_path(previous: &HashMap<NodeId, NodeId>, goal: NodeId) -> Vec<NodeId> {
    let mut path = vec![goal];
    let mut current = goal;
    
    // Walk backwards through previous nodes
    while let Some(&prev) = previous.get(&current) {
        path.push(prev);
        current = prev;
    }
    
    path.reverse(); // Built backwards, so reverse
    path
}
```

---

## ⚡ Performance Analysis

### **Time Complexity**

- **With Binary Heap**: `O(E log V)`
  - Each edge relaxed once: `O(E)`
  - Priority queue operations: `O(log V)` per operation
  - Total: `O(E log V)`

- **With Fibonacci Heap**: `O(E + V log V)` (theoretical optimum)

### **Space Complexity**

- `O(V)` for distances HashMap
- `O(V)` for previous HashMap  
- `O(V)` for priority queue (worst case)
- **Total**: `O(V)`

### **Practical Performance**

```
Graph: 1,000 vertices, 5,000 edges
Dijkstra: ~50,000 operations (E log V)
Can stop early when goal found! 🚀
```

---

## 🚫 The Non-Negative Edge Requirement

### **Why Non-Negative Edges Are Required**

Dijkstra's early termination guarantee breaks down with negative edges:

```
Example with negative edge:

    Start(0) --2.0--> Goal(1)
       |
       1.0
       ↓
    Node(2) --(-5.0)--> Goal(1)  ← Negative edge!

Dijkstra's execution:
1. Pop Goal(1) at distance 2.0
2. STOP - found goal! ✋
3. ❌ WRONG! Actual shortest: 0→2→1 = -4.0
```

**The Problem**: With negative edges, a longer path (more hops) might have lower total cost. Dijkstra stops too early and misses it.

### **Alternative: Bellman-Ford Algorithm**

**[[Bellman-Ford Algorithm]]** handles negative edges correctly:

```rust
// Bellman-Ford: Relax ALL edges V-1 times
for _ in 0..(num_vertices - 1) {
    for edge in all_edges {
        relax(edge); // Update distance if shorter path found
    }
}
```

**Trade-offs**:

| Feature | Dijkstra | Bellman-Ford |
|---------|----------|--------------|
| **Edge Weights** | Non-negative only | Any (including negative) |
| **Time Complexity** | `O(E log V)` | `O(V · E)` |
| **Early Termination** | ✅ Yes | ❌ No |
| **Negative Cycle Detection** | ❌ No | ✅ Yes |
| **Use Case** | Maps, games, networks | Currency arbitrage, network analysis |

**Key Insight**: Bellman-Ford must explore **everything** because there's no guarantee that the first path found is optimal. That's why it's `O(V · E)` - no shortcuts possible!

---

## 🎮 Real-World Applications

### **Navigation & Mapping**
- **GPS Systems**: Road network shortest path calculation
- **[[Network Routing]]**: Internet packet routing (OSPF protocol)
- **Public Transit**: Optimal route planning with transfer costs

### **Game Development**
- **[[Game AI]]**: NPC pathfinding in game worlds
- **Strategy Games**: Unit movement and tactical positioning
- **Procedural Generation**: Connected world layout optimization

### **Infrastructure**
- **Transportation Networks**: Traffic flow optimization
- **Utility Networks**: Pipeline and cable routing
- **Supply Chain**: Distribution and logistics optimization

### **Why These Work with Dijkstra**
All these domains use **non-negative costs**:
- Distance is always ≥ 0
- Time is always ≥ 0  
- Energy/fuel consumption is always ≥ 0
- This makes Dijkstra's efficient early termination possible!

---

## 🔧 Implementation Patterns

### **Priority Queue Optimization**

```rust
// Common pattern: Skip outdated queue entries
while let Some(current) = queue.pop() {
    // If we've found a better path since this was queued, skip it
    if current.cost > distances[&current.node] {
        continue; // Don't explore outdated entries
    }
    // ... process current node
}
```

This handles duplicate queue entries efficiently without expensive decrease-key operations.

### **Visited Set vs Distance Check**

**Option 1: Visited Set**
```rust
let mut visited = HashSet::new();
if visited.contains(&current.node) {
    continue;
}
visited.insert(current.node);
```

**Option 2: Distance Comparison** (preferred)
```rust
if current.cost > distances[&current.node] {
    continue;
}
```

Distance comparison is more flexible and handles priority queue duplicates naturally.

---

## 🧪 Testing & Validation

### **Critical Test Cases**

1. **Single Path**: Only one route to goal
2. **Multiple Paths**: Several routes with different costs
3. **Dead Ends**: Nodes with no outgoing edges
4. **Unreachable Goal**: Disconnected components
5. **Equal Costs**: Tie-breaking behavior validation
6. **Large Graphs**: Performance and scaling verification

### **Edge Cases**

```rust
#[test]
fn test_unreachable_goal() {
    // Goal in disconnected component
    assert_eq!(dijkstra(&graph, start, unreachable_goal), None);
}

#[test]
fn test_zero_cost_edges() {
    // Zero-weight edges should work (non-negative)
    let path = dijkstra(&graph, start, goal);
    assert_eq!(path.unwrap().0, 0.0);
}

#[test]
fn test_tie_breaking() {
    // When costs equal, lower node ID should break tie
    // (depends on Ord implementation)
}
```

---

## 🔗 Relationship to Other Algorithms

### **Simpler Alternatives**
- **[[BFS Patterns]]**: For unweighted graphs (all edges weight 1)
  - BFS is simpler and faster when weights don't matter
  - Use case: Social networks, maze solving

### **Enhanced Versions**
- **[[A* Algorithm]]**: Dijkstra + heuristic for faster goal-directed search
  - Uses estimated distance to goal to prioritize exploration
  - Perfect for single-target pathfinding (games, robotics)
  - Falls back to Dijkstra when heuristic is always 0

### **Alternative Approaches**
- **Bellman-Ford**: Handles negative edges, detects negative cycles
  - Slower but more versatile: `O(V · E)` vs `O(E log V)`
  - Use when edge weights can be negative

### **Specialized Variants**
- **Bidirectional Dijkstra**: Search from both start and goal
- **Contraction Hierarchies**: Preprocessing for repeated queries
- **Highway Hierarchies**: Road network optimization

---

## 📚 Learning Resources

### **Tutorial Progression**
- **[[../tutorials/Mission9_tut/examples/step1_priority_queue_foundation]]**: Priority queue foundations
- **[[../tutorials/Mission9_tut/examples/step2_dijkstra_basics]]**: Core algorithm implementation
- **[[../missions/Mission9/README]]**: Production pathfinding system

### **Supporting Concepts**
- **[[Priority Queue Patterns]]**: Binary heap and min-heap conversion
- **[[Graph Representation]]**: Adjacency list for efficient neighbor access
- **[[Pathfinding Strategies]]**: When to use different algorithms

### **Advanced Topics**
- **[[Heuristic Design]]**: Transitioning from Dijkstra to A*
- **[[Algorithm Composition]]**: Combining pathfinding with other algorithms
- **[[Performance Analysis]]**: Profiling and optimization techniques

---

## 💡 Key Insights & Gotchas

### **Critical Understanding**

1. **The Reversed Comparison Trick**
   - `other.cost.cmp(&self.cost)` inverts BinaryHeap behavior
   - Transforms max-heap into min-heap without changing heap algorithms
   - This single line is the "magic" that makes it work

2. **Early Termination Power**
   - Can stop as soon as goal is popped from queue
   - Doesn't need to explore entire graph
   - Only works because of non-negative edge guarantee

3. **No Explicit Path Storage**
   - Only store parent relationships, not full paths
   - Reconstruct path at end by walking backwards
   - Memory efficient: O(V) instead of exponential

4. **Multiple Paths Stay Active**
   - Queue maintains ALL promising paths simultaneously
   - Algorithm explores cheapest option first
   - Other paths remain available if primary path fails

### **Common Pitfalls**

❌ **Forgetting to reverse comparison** → Gets max-heap behavior, pops expensive nodes first  
❌ **Using with negative edges** → Wrong results due to early termination  
❌ **Not handling unreachable nodes** → Need to check for None/infinity results  
❌ **Inefficient visited tracking** → Use distance comparison, not separate HashSet  

---

## 🎯 Mission Integration

### **Mission 9 Implementation**
- **Location**: `missions/Mission9/src/`
- **Core Files**:
  - `priority_queue.rs`: Min-heap priority queue implementation
  - `dijkstra.rs`: Production Dijkstra pathfinder
  - `graph.rs`: Weighted graph representation
- **Status**: ✅ Complete with comprehensive tests

### **Tutorial Support**
- **Step 1**: [[../tutorials/Mission9_tut/examples/step1_priority_queue_foundation]]
- **Step 2**: [[../tutorials/Mission9_tut/examples/step2_dijkstra_basics]]
- **Progress**: Steps 1-5 complete, building toward A* integration

---

## 📊 Complexity Summary

| Aspect | Complexity | Notes |
|--------|-----------|-------|
| **Time (Binary Heap)** | `O(E log V)` | Standard implementation |
| **Time (Fibonacci Heap)** | `O(E + V log V)` | Theoretical optimum |
| **Space** | `O(V)` | Linear in vertices |
| **Best Case** | `O(V log V)` | Goal is direct neighbor |
| **Worst Case** | `O(E log V)` | Must explore entire graph |

---

*Tags: #dijkstra #pathfinding #graph-algorithms #shortest-path #priority-queue #greedy-algorithm #optimization #mission9*

*Backlinks: [[Graph Algorithms]] | [[Priority Queue Patterns]] | [[A* Algorithm]] | [[Bellman-Ford Algorithm]] | [[BFS Patterns]] | [[Pathfinding Strategies]] | [[Mission9 Overview]] | [[Network Routing]] | [[Game AI]] | [[Graph Representation]]*
