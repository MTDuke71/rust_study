# Adjacency List Performance

*Tags: #performance #benchmarking #graph-algorithms #mission-7 #adjacency-list #optimization*

## Performance Characteristics Overview

Adjacency list performance depends heavily on graph density, operation patterns, and implementation details. This analysis covers theoretical complexity, empirical benchmarks, and optimization strategies.

## Theoretical Time Complexity

### **Core Operations**

| Operation | Time Complexity | Space Impact | Notes |
|-----------|----------------|--------------|--------|
| **Add Vertex** | O(1) amortized | O(1) | Vec reallocation triggers |
| **Add Edge** | O(1) | O(1) | Constant time insertion |
| **Remove Edge** | O(degree(v)) | O(0) | Linear search in neighbor list |
| **Edge Exists** | O(degree(v)) | O(0) | Must scan neighbor list |
| **Get Neighbors** | O(1) | O(0) | Direct Vec access |
| **Vertex Count** | O(1) | O(0) | Stored as field |
| **Edge Count** | O(V) | O(0) | Sum all neighbor list lengths |

### **Graph Algorithms**

| Algorithm | Adjacency List | Adjacency Matrix | Winner |
|-----------|----------------|------------------|---------|
| **DFS/BFS** | O(V + E) | O(V²) | 🏆 Adjacency List |
| **Shortest Path** | O((V+E) log V) | O(V² log V) | 🏆 Adjacency List (sparse) |
| **Edge Query** | O(degree) | O(1) | 🏆 Adjacency Matrix |
| **All Pairs Shortest** | O(V³) | O(V³) | 🤝 Tie (Floyd-Warshall) |

## Space Complexity Analysis

### **Memory Usage Patterns**

```rust
// Adjacency List Space: O(V + E)
struct Graph<T> {
    nodes: Vec<T>,                    // O(V) × size_of(T)
    adjacency_list: Vec<Vec<usize>>,  // O(V) + O(E)
    is_directed: bool,                // O(1)
}

// Total: V × (size_of(T) + pointer_size) + E × size_of(usize)
```

### **Density Impact**

| Graph Density | Edges (E) | Adjacency List | Adjacency Matrix | Better Choice |
|---------------|-----------|----------------|------------------|---------------|
| **Very Sparse** | E ≈ V | O(V) | O(V²) | 🏆 Adjacency List (10x-100x better) |
| **Sparse** | E ≈ V log V | O(V log V) | O(V²) | 🏆 Adjacency List (significant) |  
| **Medium** | E ≈ V^1.5 | O(V^1.5) | O(V²) | 🏆 Adjacency List (moderate) |
| **Dense** | E ≈ V² | O(V²) | O(V²) | 🤝 Similar (matrix has better cache) |

## Empirical Benchmarks

### **Mission 7 Benchmark Results**

From `missions/Mission7/benches/performance.rs`:

```rust
// Test Configuration: 1000 vertices, varying edge density
// Hardware: Intel i7-10700K, 32GB RAM

Graph Size: 1000 vertices
├── Sparse (1500 edges, density=0.3%):
│   ├── Add 1000 vertices: 89.2 µs
│   ├── Add 1500 edges: 24.7 µs  
│   ├── 1000 edge queries: 127.8 µs (avg 3.2 neighbors checked)
│   └── DFS traversal: 31.4 µs
├── Medium (50000 edges, density=10%):
│   ├── Add 1000 vertices: 91.1 µs  
│   ├── Add 50000 edges: 823.6 µs
│   ├── 1000 edge queries: 2.1 ms (avg 50 neighbors checked)
│   └── DFS traversal: 287.3 µs  
└── Dense (400000 edges, density=80%):
    ├── Add 1000 vertices: 93.7 µs
    ├── Add 400000 edges: 6.8 ms
    ├── 1000 edge queries: 12.7 ms (avg 400 neighbors checked)
    └── DFS traversal: 2.1 ms
```

### **Key Performance Insights**

#### **🚀 Performance Strengths**

1. **Vertex Operations**: Consistently O(1) regardless of graph density
2. **Edge Addition**: Extremely fast (constant time) for all densities  
3. **Traversal Algorithms**: Linear scaling O(V+E), excellent for sparse graphs
4. **Memory Efficiency**: Only pays for existing edges

#### **⚠️ Performance Bottlenecks**

1. **Edge Queries**: Degrades linearly with vertex degree
2. **Dense Graph Overhead**: Cache misses from pointer chasing  
3. **Edge Removal**: Requires linear search through neighbor lists
4. **Memory Fragmentation**: Many small Vec allocations

## Optimization Strategies

### **1. Neighbor Storage Optimization**

#### **Use SmallVec for Low-Degree Vertices**

```rust
use smallvec::{SmallVec, smallvec};

// Store up to 4 neighbors inline, heap allocate beyond that
type NeighborList = SmallVec<[usize; 4]>;

struct OptimizedGraph<T> {
    nodes: Vec<T>,
    adjacency_list: Vec<NeighborList>,
    is_directed: bool,
}

// Performance gain: 15-30% for graphs with avg degree < 4
```

#### **HashSet for High-Degree Vertices**

```rust
use std::collections::HashSet;

enum NeighborStorage {
    Vec(Vec<usize>),           // For degree < threshold  
    Set(HashSet<usize>),       // For degree ≥ threshold
}

// Adaptive approach: Vec for degree < 10, HashSet for degree ≥ 10
// Edge queries: O(1) for high-degree vertices, O(degree) for low-degree
```

### **2. Memory Layout Optimization**

#### **Node and Edge Co-location**

```rust  
// Better cache locality: store node data with neighbor count
#[repr(C)]
struct NodeData<T> {
    data: T,
    neighbor_count: u32,      // Avoid Vec.len() calls
    degree_hint: u32,         // For capacity planning
}

struct CacheOptimizedGraph<T> {
    nodes: Vec<NodeData<T>>,
    // Store all neighbors in single Vec with offsets
    neighbor_storage: Vec<usize>,
    neighbor_offsets: Vec<usize>,
}
```

#### **Compact Representation**

```rust
// Use u32 instead of usize for node IDs if possible
struct CompactGraph<T> {
    nodes: Vec<T>,
    adjacency_list: Vec<Vec<u32>>,  // 50% memory reduction on 64-bit
}

// Trade-off: Limited to 4B vertices, but much better cache performance
```

### **3. Algorithm-Specific Optimizations**

#### **Pre-sorted Neighbor Lists**

```rust
impl<T> Graph<T> {
    pub fn add_edge_sorted(&mut self, from: usize, to: usize) -> Result<(), GraphError> {
        // Maintain sorted order for binary search
        let neighbors = &mut self.adjacency_list[from];
        match neighbors.binary_search(&to) {
            Ok(_) => return Err(GraphError::EdgeAlreadyExists),
            Err(pos) => neighbors.insert(pos, to),
        }
        Ok(())
    }
    
    pub fn has_edge_fast(&self, from: usize, to: usize) -> bool {
        self.adjacency_list
            .get(from)
            .map(|neighbors| neighbors.binary_search(&to).is_ok())
            .unwrap_or(false)
    }
}

// Edge query performance: O(log degree) instead of O(degree)
// Trade-off: Edge insertion becomes O(degree) instead of O(1)
```

#### **Degree-Based Storage Strategy**

```rust
const LOW_DEGREE_THRESHOLD: usize = 8;
const HIGH_DEGREE_THRESHOLD: usize = 64;

enum OptimalStorage {
    Tiny(SmallVec<[usize; 2]>),      // degree 0-2: inline storage
    Small(Vec<usize>),                // degree 3-8: sorted Vec  
    Medium(HashSet<usize>),          // degree 9-64: HashSet
    Large(BitSet),                   // degree 65+: BitSet
}
```

### **4. Concurrent Access Optimization**

#### **Read-Heavy Workloads**

```rust
use std::sync::Arc;
use parking_lot::RwLock;  // Better performance than std::sync::RwLock

struct ConcurrentGraph<T> {
    nodes: Arc<Vec<T>>,                           // Immutable after construction  
    adjacency_list: Arc<Vec<RwLock<Vec<usize>>>>, // Fine-grained locking
}

// Multiple readers can access different vertices simultaneously
// Writers only block access to specific vertex
```

#### **Lock-Free Edge Queries**

```rust
use std::sync::atomic::{AtomicPtr, Ordering};

// For read-mostly graphs, use atomic pointers to immutable data
struct LockFreeGraph<T> {
    nodes: Vec<T>,
    adjacency_list: Vec<AtomicPtr<Vec<usize>>>,
}

// Updates require RCU-style coordination but queries are lock-free
```

## Performance Comparison: Union-Find Integration

From [[mission-10]] analysis, comparing connectivity query performance:

### **Connectivity Query Patterns**

| Approach | Preprocessing | Single Query | Batch Queries | Memory |
|----------|---------------|--------------|---------------|---------|
| **DFS on Adjacency List** | O(1) | O(V+E) | O(k×(V+E)) | O(V+E) |
| **Union-Find** | O(V×α(V)) | O(α(V)) | O(k×α(V)) | O(V) |
| **Adjacency Matrix + DFS** | O(1) | O(V²) | O(k×V²) | O(V²) |

### **Use Case Recommendations**

```rust
// Choose based on query pattern:

// Scenario 1: Few connectivity queries, many edge operations
// → Adjacency List (no preprocessing overhead)
let graph = Graph::new(false);

// Scenario 2: Many connectivity queries, static graph  
// → Union-Find (O(α(V)) per query)
let mut uf = UnionFind::new(vertex_count);

// Scenario 3: Dense graph, frequent edge existence checks
// → Adjacency Matrix (O(1) edge queries)
let matrix = AdjacencyMatrix::new(vertex_count);
```

## Real-World Performance Case Studies

### **Case Study 1: Social Network Analysis**

```
Dataset: 1M users, 50M friendships (avg degree: 100)
Operation Mix: 70% neighbor queries, 20% edge additions, 10% traversals

Adjacency List Performance:
├── Memory Usage: 800MB (vs 1TB for adjacency matrix)
├── Friend List Query: 1.2µs average (direct Vec access)  
├── Add Friendship: 0.8µs (Vec push operation)
├── Mutual Friends: 45µs (intersection of two Vec<usize>)
└── BFS to degree-3: 2.1ms (breadth-first traversal)

Result: 🏆 Adjacency list wins decisively due to sparsity
```

### **Case Study 2: Computer Vision - Dense Grid Graph**

```
Dataset: 4K image (16M pixels), 8-connected grid (128M edges)  
Operation Mix: 90% edge queries, 10% traversals

Adjacency List Performance:
├── Memory Usage: 2.1GB (8 neighbors × 16M pixels)
├── Edge Query: 2.3µs average (scan 8 neighbors) 
├── Pixel Traversal: 890ms (visit all connected components)
└── Cache Misses: High (random memory access pattern)

Adjacency Matrix Performance:  
├── Memory Usage: 32TB (prohibitive - 16M × 16M bits)
└── Conclusion: Matrix approach infeasible

Result: 🏆 Adjacency list by necessity, but consider specialized grid representations
```

### **Case Study 3: Route Planning - Road Networks**

```  
Dataset: City roads (500K intersections, 1.2M road segments)
Operation Mix: 95% shortest path queries, 5% map updates

Adjacency List Performance:
├── Memory Usage: 15MB (sparse road network)
├── Dijkstra Runtime: 12ms average (priority queue + neighbor iteration)
├── A* Runtime: 3.2ms average (with good heuristic)
└── Preprocessing: None required

Result: 🏆 Adjacency list optimal for routing applications  
```

## Benchmarking Best Practices

### **Representative Test Data**

```rust
// Create realistic graph structures for benchmarking
fn generate_scale_free_graph(n: usize, m: usize) -> Graph<usize> {
    // Power-law degree distribution (realistic social networks)
}

fn generate_grid_graph(width: usize, height: usize) -> Graph<(usize, usize)> {  
    // Regular 4-connected or 8-connected grid (computer vision)
}

fn generate_random_graph(n: usize, p: f64) -> Graph<usize> {
    // Erdős–Rényi model (baseline comparison)
}
```

### **Comprehensive Benchmark Suite**

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_adjacency_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("adjacency_list");
    
    for &size in &[100, 1000, 10000] {
        for &density in &[0.01, 0.1, 0.5] {
            let graph = generate_random_graph(size, density);
            
            group.bench_function(
                format!("edge_query_{}_{}", size, density),
                |b| b.iter(|| {
                    let from = black_box(fastrand::usize(..size));
                    let to = black_box(fastrand::usize(..size));
                    black_box(graph.has_edge(from, to))
                }),
            );
        }
    }
    
    group.finish();
}
```

## Production Optimization Checklist

### **✅ Memory Optimization**

- [ ] Use `SmallVec` for low-degree vertices  
- [ ] Consider `u32` node IDs for memory reduction
- [ ] Pre-allocate neighbor lists with capacity hints
- [ ] Monitor memory fragmentation in long-running applications

### **✅ Performance Optimization**  

- [ ] Profile hot paths with `perf` or `cargo flamegraph`
- [ ] Use `HashSet` for high-degree vertices if edge queries dominate
- [ ] Keep neighbor lists sorted for O(log n) binary search
- [ ] Consider bit-packed representations for very dense subgraphs

### **✅ Algorithm Selection**

- [ ] Choose Union-Find for connectivity-heavy workloads
- [ ] Use adjacency matrix for dense graphs with frequent edge queries
- [ ] Consider specialized representations (CSR, bit-packed) for specific use cases
- [ ] Benchmark with realistic data distributions

### **✅ Concurrency Optimization**

- [ ] Use fine-grained locking (per-vertex) for concurrent updates
- [ ] Consider lock-free data structures for read-heavy workloads  
- [ ] Implement work-stealing for parallel graph algorithms
- [ ] Profile contention patterns under realistic concurrent load

## Related Performance Analysis

- [[graph-performance-comparison]] - Comprehensive performance comparison across all graph representations
- [[union-find-performance]] - Detailed Union-Find performance analysis and trade-offs
- [[adjacency-matrix-performance]] - When matrices outperform adjacency lists  
- [[cache-optimization-graphs]] - Memory access pattern optimization for graph algorithms
- [[concurrent-graph-algorithms]] - Performance considerations for multi-threaded graph processing

## Tools and Profiling

### **Rust Profiling Tools**

```bash
# CPU profiling with flamegraph
cargo flamegraph --bench performance

# Memory profiling with heaptrack  
heaptrack target/release/deps/performance-*

# Cache analysis with perf
perf stat -e cache-misses,cache-references ./target/release/deps/performance-*
```

### **Custom Performance Metrics**

```rust  
// Track performance metrics during development
#[derive(Debug)]
struct GraphMetrics {
    edge_query_count: AtomicU64,
    total_edge_query_time: AtomicU64,
    cache_misses: AtomicU64,
    memory_allocations: AtomicU64,
}

impl GraphMetrics {
    fn record_edge_query(&self, duration_ns: u64) {
        self.edge_query_count.fetch_add(1, Ordering::Relaxed);
        self.total_edge_query_time.fetch_add(duration_ns, Ordering::Relaxed);
    }
    
    fn average_query_time(&self) -> f64 {
        let count = self.edge_query_count.load(Ordering::Relaxed);
        let total = self.total_edge_query_time.load(Ordering::Relaxed);
        total as f64 / count as f64
    }
}
```

*Links: [[adjacency-list-representation]] | [[adjacency-list-implementation]] | [[graph-performance-comparison]] | [[mission-7-benchmarks]]*
