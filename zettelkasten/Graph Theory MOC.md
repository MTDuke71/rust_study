# Graph Theory MOC (Map of Content)

## Core Concepts

### Graph Types

- **[[Graph Network Density]]** - Measuring connectivity in graph structures
- **Adjacency Graphs** - HashMap-based adjacency list implementations
- **Weighted Graphs** - Graphs with edge weights (distances, costs, happiness)
- **Directed vs Undirected** - Asymmetric vs symmetric relationships
- **Complete Graphs** - Every vertex connected to every other vertex

### Graph Representations

- **Adjacency Matrix** - 2D array representation (memory intensive)
- **Adjacency List** - HashMap/Vec-based (memory efficient, used in Day 13)
- **Edge List** - Simple list of (vertex1, vertex2, weight) tuples

### Graph Algorithms

- **[[TSP Algorithms]]** - Traveling Salesman Problem variants and solutions
- **BFS/DFS** - Breadth-first and depth-first search patterns
- **Shortest Path** - Dijkstra's, Floyd-Warshall algorithms
- **Minimum Spanning Tree** - Kruskal's and Prim's algorithms

### Graph Applications

- **Social Networks** - Modeling relationships and influence
- **Transportation** - Route optimization and logistics
- **Circuit Design** - Electronic component connectivity
- **Dinner Parties** - Seating optimization (Day 13 implementation)

## Rust Implementation Patterns

### Data Structures

```rust
// Adjacency list with HashMap (Day 13 pattern)
struct Graph<T> {
    vertices: HashSet<T>,
    edges: HashMap<(T, T), Weight>,
}

// Alternative vector-based adjacency list
struct GraphVec {
    adj_list: Vec<Vec<(NodeId, Weight)>>,
}
```

### Common Operations

- **Add Vertex**: `graph.vertices.insert(vertex)`
- **Add Edge**: `graph.edges.insert((from, to), weight)`
- **Get Neighbors**: `graph.edges.iter().filter(|((from, _), _)| from == vertex)`
- **Graph Traversal**: BFS/DFS with `VecDeque`/`Vec<T>` respectively

### Performance Characteristics

- **Space**: O(V + E) for adjacency list vs O(V²) for matrix
- **Edge Lookup**: O(1) for HashMap, O(degree) for Vec
- **Memory Access**: HashMap has cache misses, Vec has locality

## Advent of Code Applications

### Day 13: Knights of the Dinner Table

- **[[day13_analysis]]** - Complete TSP implementation with adjacency graph
- **Problem**: Circular seating optimization (TSP variant)
- **Graph**: Weighted, directed, complete (happiness relationships)
- **Algorithm**: Brute force with symmetry optimization
- **Key Insight**: Rotational symmetry reduces search space by 8×

### Day 9: All in a Single Night

- **Problem**: Classic TSP (shortest/longest route through cities)
- **Graph**: Weighted, undirected, complete (distance matrix)
- **Algorithm**: Heap's algorithm for permutation generation
- **Optimization**: DRY principle with single TSP solver for min/max

### Day 7: Some Assembly Required (Circuit Graph)

- **Problem**: Dependency resolution in logic circuit
- **Graph**: Directed Acyclic Graph (DAG) with 208 levels
- **Algorithm**: Recursive memoization for topological evaluation
- **Key Feature**: Wire dependencies form natural DAG structure

## Advanced Topics

### Symmetry Exploitation

- **Rotational Symmetry**: Fix one vertex to eliminate equivalent rotations
- **Reflectional Symmetry**: Account for clockwise ≡ counter-clockwise equivalence
- **Performance Impact**: Can reduce search space by 16× (8 rotations × 2 reflections)

### TSP Optimization Strategies

- **[[Symmetry in Algorithms]]** - Mathematical symmetries for speedup
- **Branch and Bound** - Prune search space using bounds
- **Dynamic Programming** - Held-Karp algorithm O(n²2ⁿ)
- **Heuristics** - Nearest neighbor, 2-opt, simulated annealing

### Graph Theory in Competitive Programming

- **Pattern Recognition**: Identifying when problems are graph problems
- **Algorithm Selection**: Choosing right algorithm for graph properties
- **Implementation Speed**: Template patterns for contest programming
- **Complexity Analysis**: Understanding when brute force vs optimization needed

## Mission Integration

### Mission 5: Dictionary/HashMap

- **Connection**: Adjacency graphs use Mission 5 Dictionary for edge storage
- **Pattern**: `Dictionary<(String, String), i32>` for weighted edges
- **Benefits**: O(1) edge lookup, natural key-value mapping

### Mission 7: Graph Algorithms

- **Connection**: Direct application of graph theory concepts
- **Implementation**: BFS/DFS traversal patterns
- **Data Structures**: Adjacency list representations (see [[Adjacency List Performance]])

## Related Concepts

### Mathematical Foundations

- **[[Combinatorial Optimization]]** - Finding optimal solutions in discrete spaces
- **Permutation Theory** - Systematic generation of arrangements
- **Group Theory** - Mathematical structure of symmetries

### Algorithm Analysis

- **[[Performance Engineering]]** - Systematic measurement, profiling, and optimization of graph algorithms
- **[[Benchmarking]]** - Comparing different graph representations
- **Complexity Theory** - Understanding tractable vs intractable graph problems

## External Resources

### Books

- "Introduction to Algorithms" (CLRS) - Comprehensive graph algorithm coverage
- "Graph Theory" (Diestel) - Mathematical foundations
- "Competitive Programming" (Halim) - Contest programming patterns

### Online References

- Wikipedia Graph Theory Portal
- GeeksforGeeks Graph Algorithms
- Rust std::collections documentation (HashMap, HashSet)

---

*Created: 2025-10-19*
*Last Updated: 2025-10-19*

*Tags: #graph-theory #adjacency-graph #algorithms #data-structures #moc #rust-implementation #competitive-programming #optimization #symmetry #tsp*

*Links: [[AoC Patterns MOC]] | [[Collections MOC]] | [[TSP Algorithms]] | [[Symmetry in Algorithms]] | [[topological-sort]] | [[kahns-topological-sort]] | [[day13_analysis]] | [[../missions/Mission5/README]] | [[../missions/Mission7/README]]*
