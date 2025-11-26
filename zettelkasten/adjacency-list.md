# Adjacency List

*Tags: #graph-algorithms #data-structures #adjacency #graph-representation*

## Overview

An **adjacency list** is a fundamental graph representation that stores each vertex's connections as a list of neighboring vertices. This is the most widely used graph representation in computer science due to its space efficiency and excellent performance for sparse graphs.

## Core Concept

In an adjacency list representation:

- Each vertex maintains a list of vertices it's directly connected to
- For vertex `v`, the adjacency list `adj[v]` contains all vertices `u` such that edge `(v,u)` exists
- The entire graph is represented as an array/vector of these individual lists

```
Visual Example:
Graph:          Adjacency Lists:
   0---1        adj[0] = [1, 2]
   |\ /         adj[1] = [0, 2, 3]  
   | 2          adj[2] = [0, 1]
   |/           adj[3] = [1]
   3
```

## Implementation Approaches

### **Vector of Vectors (Most Common)**

```rust
struct Graph {
    adj: Vec<Vec<usize>>,  // adj[i] contains neighbors of vertex i
    vertex_count: usize,
}
```

**Advantages**: Simple, cache-friendly, good for iteration
**Disadvantages**: O(degree) edge existence checks

### **Vector of HashSets**

```rust
use std::collections::HashSet;
struct Graph {
    adj: Vec<HashSet<usize>>,
}
```

**Advantages**: O(1) edge existence checks
**Disadvantages**: Higher memory overhead, worse cache performance

### **Hybrid Approach**

```rust
enum NeighborList {
    Small(Vec<usize>),      // For low-degree vertices
    Large(HashSet<usize>),  // For high-degree vertices  
}
```

**Advantages**: Optimal performance across all degree ranges
**Disadvantages**: Implementation complexity

## Space and Time Complexity

### **Space Complexity**

- **Total Space**: O(V + E) where V = vertices, E = edges
- **Best Case**: O(V) for graphs with no edges
- **Worst Case**: O(V²) for complete graphs (but still more efficient than adjacency matrix)

### **Time Complexity**

| Operation | Time | Notes |
|-----------|------|--------|
| Add Vertex | O(1) amortized | Vector reallocation |
| Add Edge | O(1) | Direct append |
| Remove Edge | O(degree) | Linear search |
| Edge Exists | O(degree) | Linear search |
| Get Neighbors | O(1) | Direct access |
| Iterate Edges | O(V + E) | Visit all lists |

## Comparison with Other Representations

### **vs Adjacency Matrix**

| Aspect | Adjacency List | Adjacency Matrix |
|--------|----------------|------------------|
| **Space** | O(V + E) | O(V²) |
| **Edge Check** | O(degree) | O(1) |
| **Add Edge** | O(1) | O(1) |  
| **Iterate Neighbors** | O(degree) | O(V) |
| **Best For** | Sparse graphs | Dense graphs, frequent edge queries |

### **vs Edge List**

| Aspect | Adjacency List | Edge List |
|--------|----------------|-----------|
| **Space** | O(V + E) | O(E) |
| **Find Neighbors** | O(1) + O(degree) | O(E) |
| **Add Edge** | O(1) | O(1) |
| **Best For** | Neighbor queries | Simple storage, algorithms that process all edges |

### **vs Union-Find** (for connectivity)

From [[mission-10]] analysis:

| Aspect | Adjacency List + DFS | Union-Find |
|--------|---------------------|------------|
| **Space** | O(V + E) | O(V) |
| **Connectivity Query** | O(V + E) | O(α(V)) ≈ O(1) |
| **Add Edge** | O(1) | O(α(V)) |
| **Best For** | Graph algorithms, general purpose | Connectivity queries only |

## Algorithm Performance

### **Graph Traversal Algorithms**

- **DFS/BFS**: O(V + E) - optimal performance
- **Connected Components**: O(V + E) using DFS/BFS
- **Topological Sort**: O(V + E) with DFS-based approach

### **Shortest Path Algorithms**  

- **Dijkstra's Algorithm**: O((V + E) log V) with priority queue
- **Bellman-Ford**: O(VE) - good performance for sparse graphs
- **A* Search**: O((V + E) log V) with efficient neighbor access

### **Minimum Spanning Tree**

- **Kruskal's**: O(E log E) - needs all edges, adjacency list helps with iteration
- **Prim's**: O((V + E) log V) - excellent with adjacency list neighbor access

## Real-World Applications

### **Social Networks**

```rust
// Friend connections - highly sparse
let friends: Vec<Vec<UserId>> = vec![
    vec![alice, bob, charlie],    // My friends
    vec![me, dave],               // Alice's friends
    vec![me, eve],                // Bob's friends
];

// Typical properties:
// - Average degree: 100-500 (out of millions of users)
// - Power-law distribution: few users have many friends
// - Locality: friends often share friends
```

### **Web Graph**  

```rust
// Page links - very sparse but with some high-degree pages
let web_links: Vec<Vec<PageId>> = vec![
    vec![page1, page2, page3],    // Homepage links
    vec![homepage],               // Most pages link back to homepage
    vec![],                       // Dead-end page
];

// Typical properties:
// - Billions of pages, trillions of links
// - Extremely sparse (avg degree ~7)
// - Small-world property: short paths between pages
```

### **Transportation Networks**

```rust
// Road intersections and connections
let road_network: Vec<Vec<IntersectionId>> = vec![
    vec![north, south, east, west], // Major intersection (4-way)  
    vec![main_st],                  // Minor intersection (2-way)
    vec![highway_onramp],           // Highway connection
];

// Typical properties:
// - Planar graph (edges don't cross in 2D)
// - Low average degree (2-6 connections per intersection)
// - Geographic locality: nearby intersections more likely connected
```

### **Dependency Graphs**

```rust
// Software package dependencies
let dependencies: Vec<Vec<PackageId>> = vec![
    vec![libc, std],              // My package depends on libc, std
    vec![],                       // libc has no dependencies  
    vec![libc],                   // std depends on libc
];

// Typical properties:
// - Directed acyclic graph (DAG)
// - Sparse (packages depend on few others)  
// - Hierarchical structure with foundational packages
```

## Optimization Strategies

### **Memory Optimization**

```rust
use smallvec::SmallVec;

// Use SmallVec for vertices with few neighbors (common case)
type NeighborList = SmallVec<[usize; 4]>;  // 4 neighbors stored inline

struct OptimizedGraph {
    adj: Vec<NeighborList>,
}

// Benefits:
// - Reduced heap allocations for small neighbor lists
// - Better cache locality
// - 15-30% performance improvement for sparse graphs
```

### **Cache Optimization**

```rust
// Store vertices and adjacency data together for better locality
#[repr(C)]
struct Vertex<T> {
    data: T,
    first_neighbor_index: usize,
    neighbor_count: usize,
}

struct CacheOptimizedGraph<T> {
    vertices: Vec<Vertex<T>>,
    neighbors: Vec<usize>,  // All neighbors in single array
}

// Benefits:
// - Single memory allocation
// - Sequential access patterns
// - Better prefetching behavior
```

### **Concurrent Access**

```rust
use std::sync::Arc;
use parking_lot::RwLock;

// Fine-grained locking: one lock per vertex
struct ConcurrentGraph {
    adj: Vec<Arc<RwLock<Vec<usize>>>>,
}

// Benefits:  
// - Multiple threads can modify different vertices simultaneously
// - Readers don't block other readers on same vertex
// - Scales better than single global lock
```

## Advanced Variations

### **Weighted Adjacency Lists**

```rust
struct WeightedEdge {
    target: usize,
    weight: f64,
}

struct WeightedGraph {
    adj: Vec<Vec<WeightedEdge>>,
}

// Applications: shortest path, network flow, social influence
```

### **Bidirectional Adjacency Lists**

```rust
struct BiGraph {
    out_adj: Vec<Vec<usize>>,  // Outgoing edges
    in_adj: Vec<Vec<usize>>,   // Incoming edges  
}

// Benefits:
// - O(in_degree) to find predecessors
// - O(out_degree) to find successors
// - Useful for algorithms needing both directions
```

### **Compressed Adjacency Lists**

```rust
// For very large graphs, use compressed representations
struct CompressedGraph {
    adj_offsets: Vec<usize>,      // Where each vertex's neighbors start
    adj_neighbors: Vec<u32>,      // All neighbors in single array
}

// Benefits:
// - Reduced memory overhead (no Vec headers)
// - Better cache performance
// - Immutable after construction (good for read-heavy workloads)
```

## Performance Considerations

### **When Adjacency Lists Excel**

- **Sparse Graphs**: E << V², which is most real-world graphs
- **Graph Traversal**: DFS, BFS, connected components
- **Neighbor Iteration**: When you need to visit all neighbors
- **Dynamic Graphs**: Frequent edge additions/removals
- **Memory Constraints**: Limited memory for large graphs

### **When Adjacency Lists Struggle**

- **Dense Graphs**: E ≈ V², adjacency matrix becomes competitive
- **Frequent Edge Queries**: Checking if specific edge exists
- **Random Access**: Accessing arbitrary vertex pairs
- **Cache-Sensitive**: Random memory access patterns

### **Optimization Decision Tree**

```
Graph Density?
├── Sparse (E < V²/10)
│   ├── Frequent edge queries? → Use HashSet-based adjacency
│   └── Memory critical? → Use Vec-based adjacency  
├── Medium (V²/10 ≤ E ≤ V²/2)
│   ├── Read-heavy? → Consider adjacency matrix
│   └── Dynamic? → Stick with adjacency list
└── Dense (E > V²/2)
    └── Use adjacency matrix (O(1) queries, similar memory)
```

## Mission 7 Integration

In [[mission-7]], adjacency lists are the primary focus:

- **REQ-1**: Generic graph structure with adjacency list representation
- **REQ-2**: Support both directed and undirected graphs  
- **REQ-3**: Efficient neighbor access and graph traversal
- **REQ-4**: Memory-efficient storage for sparse graphs

The Mission 7 implementation demonstrates production-quality adjacency list usage with:

- Proper error handling for invalid operations
- Generic type support for vertex data
- Comprehensive test coverage for edge cases
- Performance benchmarks comparing different approaches

## Educational Value

Adjacency lists teach fundamental concepts:

- **Trade-offs**: Space vs time complexity decisions
- **Algorithm Design**: How data structure choice affects algorithm efficiency  
- **Real-World Modeling**: Most networks are sparse, making adjacency lists practical
- **Optimization**: Cache locality, memory management, concurrent access patterns

Understanding adjacency lists provides foundation for:

- Advanced graph algorithms (shortest path, network flow)
- Distributed graph processing (GraphX, Pregel)
- Graph databases (Neo4j, Amazon Neptune)
- Social network analysis and recommendation systems

## Related Concepts

- [[adjacency-list-representation]] - Detailed theoretical foundation
- [[adjacency-list-implementation]] - Production-quality implementation guide  
- [[adjacency-list-performance]] - Comprehensive performance analysis and optimization
- [[mission-7]] - Complete graph implementation using adjacency lists
- [[graph-algorithms]] - Algorithms that work efficiently with adjacency lists
- [[graph-representations]] - Comparison of all graph representation methods
- [[union-find-vs-adjacency]] - When to use Union-Find vs adjacency-based approaches

*Links: [[graph-algorithms]] | [[data-structures]] | [[mission-7]] | [[performance-optimization]]*
