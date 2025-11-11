# Adjacency List Representation

*Tags: #graph-algorithms #data-structures #adjacency-list #mission-7*

## Core Concept

An **adjacency list** is a graph representation where each vertex stores a list of its adjacent (directly connected) vertices. This is the most common and efficient way to represent sparse graphs.

## Structure

```rust
// Basic adjacency list representation
struct Graph {
    adj_list: Vec<Vec<usize>>,  // Vector of neighbor lists
    vertex_count: usize,
}

// Each index represents a vertex, containing its neighbors
// Example: adj_list[0] = [1, 2] means vertex 0 connects to vertices 1 and 2
```

## Visual Example

```
Graph:     Adjacency List:
0---1      0: [1, 2]
|\ /       1: [0, 2] 
| 2        2: [0, 1]
```

## Implementation Patterns

### **Mission 7 Implementation**
- Uses `Vec<Vec<usize>>` for neighbor storage
- Supports both directed and undirected graphs
- O(1) vertex addition, O(degree) neighbor lookup
- Generic over node data types

### **Performance Characteristics**
| Operation | Time Complexity | Space |
|-----------|----------------|-------|
| Add Vertex | O(1) amortized | O(V) |
| Add Edge | O(1) | O(E) |
| Remove Edge | O(degree) | - |
| Check Adjacent | O(degree) | - |
| Get Neighbors | O(1) | - |

## Advantages vs Alternatives

### **✅ Adjacency List Strengths**
- **Space Efficient**: O(V + E) - only stores existing edges
- **Sparse Graph Optimal**: Perfect for graphs with few edges
- **Edge Iteration**: Fast to iterate through all edges
- **Dynamic**: Easy to add/remove vertices and edges

### **❌ Adjacency List Weaknesses**  
- **Edge Lookup**: O(degree) to check if specific edge exists
- **Dense Graph Overhead**: Less efficient than matrix for dense graphs
- **Cache Performance**: Pointer chasing can hurt cache locality

## Comparison with Adjacency Matrix

| Aspect | Adjacency List | Adjacency Matrix |
|--------|----------------|------------------|
| **Space** | O(V + E) | O(V²) |
| **Edge Check** | O(degree) | O(1) |
| **Add Edge** | O(1) | O(1) |
| **Neighbor Iteration** | O(degree) | O(V) |
| **Best For** | Sparse graphs | Dense graphs, frequent edge queries |

## Union-Find Connection

In [[mission-10]] Union-Find tutorial, we compared adjacency approaches:
- **Union-Find**: O(V) space, O(α(V)) connectivity queries
- **Adjacency List**: O(V+E) space, O(V+E) connectivity via DFS/BFS  
- **Adjacency Matrix**: O(V²) space, O(1) edge queries

The choice depends on query patterns and graph density.

## Real-World Applications

### **Social Networks**
```rust
// User connections - typically sparse
let friends: Vec<Vec<UserId>> = vec![
    vec![1, 2, 5],     // User 0 friends with 1, 2, 5
    vec![0, 3],        // User 1 friends with 0, 3
    vec![0, 4],        // User 2 friends with 0, 4
];
```

### **Web Graph**
```rust
// Web pages with outbound links
let web_graph: Vec<Vec<PageId>> = vec![
    vec![1, 2, 3],     // Page 0 links to pages 1, 2, 3
    vec![2],           // Page 1 links to page 2
    vec![],            // Page 2 has no outbound links
];
```

### **Road Networks**
```rust
// Intersections and connected roads
let road_network: Vec<Vec<IntersectionId>> = vec![
    vec![1, 3],        // Intersection 0 connects to 1, 3
    vec![0, 2, 4],     // Intersection 1 connects to 0, 2, 4
];
```

## Implementation Variants

### **Weighted Adjacency List**
```rust
struct WeightedGraph {
    adj_list: Vec<Vec<(usize, i32)>>,  // (neighbor, weight) pairs
}
```

### **Generic Adjacency List**
```rust  
struct GenericGraph<T> {
    nodes: Vec<T>,                    // Node data
    adj_list: Vec<Vec<usize>>,       // Connections by index
}
```

### **Bidirectional Links**
For undirected graphs, ensure symmetry:
```rust
fn add_undirected_edge(&mut self, u: usize, v: usize) {
    self.adj_list[u].push(v);
    self.adj_list[v].push(u);  // Add both directions
}
```

## Advanced Considerations

### **Memory Optimization**
- Use `SmallVec` for vertices with few neighbors
- Consider `HashMap<usize, Vec<usize>>` for very sparse graphs
- Pack edge weights efficiently for weighted graphs

### **Concurrent Access**
- Use `Arc<RwLock<Vec<Vec<usize>>>>` for thread-safe access
- Consider lock-free data structures for high-performance scenarios
- Separate read/write operations to minimize contention

## Related Concepts

- [[adjacency-list-performance]] - Performance analysis and benchmarks
- [[adjacency-list]] - General adjacency list concepts
- [[graph-representations]] - Comparison of all graph representations  
- [[mission-7]] - Complete graph implementation with adjacency lists
- [[graph-algorithms]] - Algorithms that work well with adjacency lists
- [[graph-traversal]] - DFS/BFS on adjacency list structures

## Code Examples

See complete implementations in:
- `missions/Mission7/` - Production-quality graph with adjacency lists
- `tutorials/Mission7_tut/examples/step2_adjacency_lists.rs` - Educational examples
- Advanced examples in `advanced_examples/` for specialized use cases

*Links: [[mission-7]] | [[graph-algorithms]] | [[data-structures-comparison]]*