# 🔄 Bellman-Ford Algorithm - Negative Edge Shortest Path

*Single-source shortest path algorithm that handles negative edge weights*

---

## 📋 Core Concept

**Purpose**: Find shortest paths from a source node to all other nodes, even with negative edge weights  
**Creator**: Richard Bellman (1958), Lester Ford Jr. (1956)  
**Key Feature**: Can detect negative cycles in graphs  
**Trade-off**: Slower than Dijkstra but more versatile

### **The Fundamental Guarantee**

> "After V-1 iterations of edge relaxation, all shortest paths have been found (if no negative cycles exist)"

Unlike [[Dijkstra Algorithm]], Bellman-Ford **cannot use early termination** because negative edges mean later paths might be cheaper.

---

## 🧮 Algorithm Description

### **Core Principle: Edge Relaxation**

**Relaxation**: Update distance if a shorter path is found through an edge

```rust
// Relax edge u → v with weight w
if distance[u] + w < distance[v] {
    distance[v] = distance[u] + w;
    previous[v] = u;
}
```

### **Why V-1 Iterations?**

```
In a graph with V vertices:
  - Longest possible shortest path uses at most V-1 edges
  - Example: 0→1→2→3→4 (5 vertices, 4 edges)

Each iteration extends shortest paths by 1 edge:
  - Iteration 1: Finds all 1-edge shortest paths
  - Iteration 2: Finds all 2-edge shortest paths
  - ...
  - Iteration V-1: Finds all (V-1)-edge shortest paths

After V-1 iterations: ALL shortest paths found! ✅
```

---

## 🔄 Algorithm Implementation

### **Basic Bellman-Ford**

```rust
fn bellman_ford(
    graph: &Graph, 
    start: NodeId
) -> Result<HashMap<NodeId, Weight>, NegativeCycleError> {
    let mut distances = HashMap::new();
    let mut previous = HashMap::new();
    
    // Initialize: start has distance 0, all others infinity
    for node in graph.nodes() {
        distances.insert(node, f64::INFINITY);
    }
    distances.insert(start, 0.0);
    
    // Relax all edges V-1 times
    for iteration in 0..(graph.node_count() - 1) {
        let mut changed = false;
        
        for edge in graph.all_edges() {
            let new_distance = distances[&edge.from] + edge.weight;
            
            if new_distance < distances[&edge.to] {
                distances.insert(edge.to, new_distance);
                previous.insert(edge.to, edge.from);
                changed = true;
            }
        }
        
        // Optimization: Stop early if no changes
        if !changed {
            break;
        }
    }
    
    // Check for negative cycles (Vth iteration)
    for edge in graph.all_edges() {
        if distances[&edge.from] + edge.weight < distances[&edge.to] {
            return Err(NegativeCycleError);
        }
    }
    
    Ok(distances)
}
```

### **Negative Cycle Detection**

```rust
// If distances still improve on Vth iteration, negative cycle exists!
for edge in graph.all_edges() {
    if distances[&edge.from] + edge.weight < distances[&edge.to] {
        // Negative cycle detected!
        return Err("Graph contains negative cycle reachable from source");
    }
}
```

**Why this works**: After V-1 iterations, all shortest paths found. If distances still improve, we're going around a cycle that reduces cost—a negative cycle!

---

## ⚡ Performance Analysis

### **Time Complexity**

- **Standard**: `O(V · E)`
  - Outer loop: V-1 iterations
  - Inner loop: Check all E edges
  - No shortcuts possible!

- **Early Termination Optimization**: Best case `O(E)` if graph is "easy"
  - Stop if no distances change in an iteration
  - Rare in practice with negative edges

### **Space Complexity**

- `O(V)` for distances HashMap
- `O(V)` for previous HashMap
- **Total**: `O(V)` (same as Dijkstra)

### **Performance Comparison**

```
Graph: 1,000 vertices, 5,000 edges

Dijkstra (binary heap):
  O(E log V) = 5,000 * 10 ≈ 50,000 operations
  Can stop early when goal found! 🚀

Bellman-Ford:
  O(V · E) = 1,000 * 5,000 = 5,000,000 operations
  Must complete ALL iterations! 🐌
  
Performance gap: ~100x slower
```

---

## 🚫 Why Bellman-Ford is Necessary

### **When Dijkstra Fails**

```
Example: Negative edge breaks Dijkstra

    Start(0) --2.0--> Goal(1)
       |
       1.0
       ↓
    Node(2) --(-5.0)--> Goal(1)

Dijkstra's execution:
1. Pop Goal(1) at distance 2.0
2. STOP - found goal! ✋
3. ❌ WRONG! Actual shortest: 0→2→1 = -4.0

Bellman-Ford's execution:
1. Iteration 1: distances = {0: 0.0, 1: 2.0, 2: 1.0}
2. Iteration 2: distances = {0: 0.0, 1: -4.0, 2: 1.0}
3. ✅ CORRECT! Found path through negative edge
```

### **The "No Guarantee" Problem**

**[[Dijkstra Algorithm]]'s guarantee:**

```
When I pop node X at distance D:
  ✅ D is the shortest distance to X (if edges ≥ 0)
```

**Bellman-Ford's reality:**

```
After checking edge X→Y:
  ❌ Can't be sure this is shortest to Y yet
  ❌ A negative edge elsewhere might create cheaper path
  ❌ Must keep checking ALL edges, multiple times
```

**That's why it's O(V·E)!** No early termination possible—must explore everything.

---

## 🎯 Use Cases & Applications

### **Currency Arbitrage**

```
Currencies as nodes, exchange rates as edges
Negative edge: Profitable exchange cycle
Example: USD → EUR → GBP → USD with profit

Bellman-Ford detects: "Negative cycle exists!"
→ Arbitrage opportunity found!
```

### **Network Protocols**

- **Distance Vector Routing**: RIP protocol uses Bellman-Ford
- **BGP**: Internet backbone routing with policy costs
- **Network with Credits**: Edges can represent cost savings

### **Financial Modeling**

- **Transaction Networks**: Costs and rebates (negative costs)
- **Cash Flow Analysis**: Income (negative cost) and expenses
- **Optimization Problems**: Resource allocation with incentives

### **Constraint Networks**

- **Difference Constraints**: System of inequalities
- **Temporal Reasoning**: Time intervals with constraints
- **Resource Scheduling**: Tasks with dependencies and bonuses

---

## 🔄 Algorithm Variants

### **Queue-Based Bellman-Ford (SPFA)**

```rust
fn spfa(graph: &Graph, start: NodeId) -> HashMap<NodeId, Weight> {
    let mut distances = HashMap::new();
    let mut in_queue = HashSet::new();
    let mut queue = VecDeque::new();
    
    distances.insert(start, 0.0);
    queue.push_back(start);
    in_queue.insert(start);
    
    while let Some(u) = queue.pop_front() {
        in_queue.remove(&u);
        
        for (v, weight) in graph.neighbors(u) {
            let new_dist = distances[&u] + weight;
            
            if new_dist < *distances.get(&v).unwrap_or(&f64::INFINITY) {
                distances.insert(v, new_dist);
                
                if !in_queue.contains(&v) {
                    queue.push_back(v);
                    in_queue.insert(v);
                }
            }
        }
    }
    
    distances
}
```

**SPFA (Shortest Path Faster Algorithm)**:

- Only processes nodes whose distances changed
- Often faster in practice: average `O(E)`, worst still `O(V·E)`
- Popular in competitive programming

### **Yen's Improvement**

Tracks which edges caused changes, only checks those in next iteration. Reduces work when few edges matter.

---

## 🔗 Comparison with Other Algorithms

| Feature | Dijkstra | Bellman-Ford | BFS |
|---------|----------|--------------|-----|
| **Edge Weights** | Non-negative | Any (including negative) | Unweighted (1) |
| **Negative Cycles** | Cannot detect | ✅ Detects | N/A |
| **Time Complexity** | `O(E log V)` | `O(V · E)` | `O(V + E)` |
| **Early Termination** | ✅ Yes | ❌ No | ✅ Yes |
| **Space** | `O(V)` | `O(V)` | `O(V)` |
| **Use Case** | Maps, games | Finance, protocols | Simple paths |

### **Decision Tree**

```
Need shortest path?
  ├─ All edges weight 1? → Use BFS
  ├─ All edges ≥ 0?
  │   ├─ Single target? → Use A* (with Dijkstra as fallback)
  │   └─ All targets? → Use Dijkstra
  └─ Negative edges possible?
      ├─ Need cycle detection? → Use Bellman-Ford
      └─ Known no cycles? → Use Bellman-Ford anyway (only safe option)
```

---

## 🧪 Testing & Validation

### **Critical Test Cases**

```rust
#[test]
fn test_negative_edge_path() {
    // Verify handles negative edges correctly
    let path_cost = bellman_ford(&graph, start);
    assert_eq!(path_cost[&goal], -4.0); // Through negative edge
}

#[test]
fn test_negative_cycle_detection() {
    // Graph: 0 → 1 → 2 → 1 (with total weight < 0)
    let result = bellman_ford(&graph, start);
    assert!(result.is_err()); // Should detect cycle
}

#[test]
fn test_early_termination() {
    // Simple graph should terminate before V-1 iterations
    // (Check iteration counter in instrumented version)
}

#[test]
fn test_disconnected_nodes() {
    // Unreachable nodes should remain at infinity
    assert_eq!(path_cost[&unreachable], f64::INFINITY);
}
```

---

## 💡 Key Insights

### **Why V-1 Iterations?**

Visual explanation:

```
Path from 0 to 4 in worst case:
  0 → 1 → 2 → 3 → 4  (4 edges for 5 vertices)

Iteration 1: Finds 0→1
Iteration 2: Finds 0→1→2  
Iteration 3: Finds 0→1→2→3
Iteration 4: Finds 0→1→2→3→4

General: V-1 iterations finds paths of length up to V-1 edges
```

### **The Power of Relaxation**

```
Relaxation is idempotent:
  - Relaxing same edge multiple times is safe
  - Distance only improves (decreases), never worsens
  - Eventually converges to shortest path
```

### **Negative Cycles**

```
Negative cycle: Sum of edge weights < 0
Example: A → B (-1) → C (-2) → A (1) = -2

Problem: Can go around cycle infinitely, making path shorter
Solution: Bellman-Ford detects this on Vth iteration!
```

---

## 📚 Learning Integration

### **Prerequisite Knowledge**

- **[[graph-representation]]**: Understanding graph data structures
- **[[BFS Patterns]]**: Basic graph traversal concepts
- **[[Dijkstra Algorithm]]**: Context for why Bellman-Ford is needed

### **Related Concepts**

- **[[graph-algorithms]]**: Overall algorithm landscape
- **[[Pathfinding Strategies]]**: When to use which algorithm
- **[[Dynamic Programming]]**: Bellman-Ford as DP on graphs

### **Advanced Topics**

- **[[Floyd-Warshall Algorithm]]**: All-pairs shortest paths
- **[[Network Flow Algorithms]]**: Related optimization problems
- **[[Constraint Satisfaction]]**: Application to scheduling problems

---

## 🎯 Practical Recommendations

### **When to Use Bellman-Ford**

✅ **Use Bellman-Ford when:**

- Edge weights can be negative
- Need to detect negative cycles
- Graph is small (< 1000 vertices)
- Running offline/batch processing

❌ **Avoid Bellman-Ford when:**

- All edges are non-negative → Use [[Dijkstra Algorithm]]
- Need real-time performance on large graphs
- Only care about unweighted paths → Use [[BFS Patterns]]

### **Optimization Tips**

1. **Early Termination**: Track if any changes in iteration, stop if none
2. **SPFA Variant**: Use queue-based version for average-case speedup
3. **Negative Cycle Check**: Can skip if domain guarantees no cycles
4. **Sparse Graphs**: Consider adjacency list over edge list

---

## 📊 Complexity Summary

| Aspect | Complexity | Notes |
|--------|-----------|-------|
| **Time (Standard)** | `O(V · E)` | Must do V-1 iterations |
| **Time (Best Case)** | `O(E)` | With early termination |
| **Time (Worst Case)** | `O(V · E)` | Dense graphs or many iterations needed |
| **Space** | `O(V)` | Same as Dijkstra |
| **Cycle Detection** | `O(E)` | One additional pass through edges |

---

*Tags: #bellman-ford #shortest-path #negative-edges #graph-algorithms #dynamic-programming #cycle-detection #pathfinding*

*Backlinks: [[graph-algorithms]] | [[Dijkstra Algorithm]] | [[BFS Patterns]] | [[Pathfinding Strategies]] | [[Dynamic Programming]] | [[Network Flow Algorithms]] | [[graph-representation]]*
