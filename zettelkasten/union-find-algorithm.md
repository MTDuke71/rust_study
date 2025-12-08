# Union-Find Algorithm (Disjoint Set Data Structure)

**Tags:** #algorithm #union-find #disjoint-sets #graph-algorithms #optimization #data-structures #path-compression #union-by-rank  
**Created:** 2025-11-11  
**Related:** [[mission-10]], [[graph-algorithms]], [[Tree Algorithms]], [[Performance Optimization]], [[Kruskal's Algorithm]], [[Connected Components]], [[union-find-patterns]]

---

## 🎯 Core Concept

The **Union-Find** (also called **Disjoint Set Union** or **DSU**) is a data structure that efficiently tracks a partition of elements into disjoint (non-overlapping) sets. It supports two primary operations:

- **Find**: Determine which set an element belongs to
- **Union**: Merge two sets into one

## 🧠 Fundamental Insight

**The Core Invariant**: Elements `x` and `y` are in the same connected component **if and only if** `find(x) == find(y)`.

This works because:

1. Each set forms a **tree** with a unique **root**
2. **Parent pointers** create paths from elements to roots
3. **Following parent chains** always leads to the same root for elements in the same set
4. **Different roots** mean different sets

```
Set 1: {0,1,2}    Set 2: {3,4}      Set 3: {5}
   0 (root)         3 (root)        5 (root)
  / \               |
 1   2              4
```

## ⚡ Basic Implementation

```rust
struct UnionFind {
    parent: Vec<usize>,  // parent[i] = parent of element i
    count: usize,        // number of disjoint sets
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(), // each element is its own parent initially
            count: n,                 // n separate sets
        }
    }
    
    // Basic find - follow parent pointers to root
    fn find(&self, mut x: usize) -> usize {
        while self.parent[x] != x {
            x = self.parent[x]; // traverse up the tree
        }
        x // return root
    }
    
    // Basic union - attach one root to another
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x == root_y {
            return false; // already connected
        }
        
        self.parent[root_x] = root_y; // attach root_x under root_y
        self.count -= 1;
        true
    }
}
```

**Time Complexity (Basic):**

- Find: O(n) worst case (degenerate tree)
- Union: O(n) worst case (requires find)

## 🚀 Optimization 1: Path Compression

**Key Idea**: During `find`, flatten the tree by pointing all nodes directly to the root.

```rust
fn find(&mut self, x: usize) -> usize {
    if self.parent[x] != x {
        self.parent[x] = self.find(self.parent[x]); // path compression!
    }
    self.parent[x]
}
```

**Before Path Compression:**

```
    0
    |
    1
    |
    2
    |
    3
```

**After Path Compression:**

```
    0
   /|\
  1 2 3
```

**Time Complexity with Path Compression:**

- Find: O(log n) amortized
- Union: O(log n) amortized

## ⚖️ Optimization 2: Union by Rank

**Key Idea**: Always attach the smaller tree under the root of the larger tree to keep trees balanced.

```rust
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,   // approximate tree height
    count: usize,
}

fn union(&mut self, x: usize, y: usize) -> bool {
    let root_x = self.find(x);
    let root_y = self.find(y);
    
    if root_x == root_y {
        return false;
    }
    
    // Union by rank - attach smaller tree under larger
    if self.rank[root_x] < self.rank[root_y] {
        self.parent[root_x] = root_y;
    } else if self.rank[root_x] > self.rank[root_y] {
        self.parent[root_y] = root_x;
    } else {
        self.parent[root_y] = root_x;
        self.rank[root_x] += 1; // height increases only when equal ranks merge
    }
    
    self.count -= 1;
    true
}
```

## 🌟 Combined Optimizations: O(α(n)) Performance

When **both optimizations** are used together:

- **Path Compression** + **Union by Rank** = **O(α(n))** amortized time
- **α(n)** is the **inverse Ackermann function** - practically constant!

```rust
// For all practical purposes, α(n) ≤ 4
α(1) = 1
α(3) = 2  
α(2047) = 3
α(2^65536 - 1) = 4
```

**Performance Comparison:**

| Input Size | Operations | Basic | Path Compression | Union by Rank | Both Combined |
|------------|------------|-------|------------------|---------------|---------------|
| 1,000 | 10,000 | ~5s | ~0.1s | ~0.2s | **~0.01s** |
| 100,000 | 1,000,000 | timeout | ~10s | ~20s | **~0.5s** |

## 🎯 Core Operations API

```rust
impl UnionFind {
    // Create n disjoint sets {0}, {1}, ..., {n-1}
    fn new(n: usize) -> Self
    
    // Find root of set containing x
    fn find(&mut self, x: usize) -> usize
    
    // Union sets containing x and y
    fn union(&mut self, x: usize, y: usize) -> bool
    
    // Check if x and y are in same set
    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    
    // Number of disjoint sets
    fn count(&self) -> usize
    
    // Size of set containing x  
    fn size(&mut self, x: usize) -> usize
}
```

## 🌍 Real-World Applications

### 1. **Minimum Spanning Tree (Kruskal's Algorithm)**

```rust
// Kruskal's MST using Union-Find for cycle detection
fn kruskal_mst(edges: &mut [Edge], n: usize) -> Vec<Edge> {
    edges.sort_by_key(|e| e.weight); // sort by weight
    let mut uf = UnionFind::new(n);
    let mut mst = Vec::new();
    
    for edge in edges {
        if uf.union(edge.u, edge.v) { // if not creates cycle
            mst.push(*edge);
            if mst.len() == n - 1 { break; } // MST complete
        }
    }
    mst
}
```

### 2. **Dynamic Connectivity**

```rust
// Handle queries: "Are cities A and B connected by roads?"
let mut cities = UnionFind::new(1000);

// Add road between cities 5 and 23
cities.union(5, 23);

// Query: Can you travel from city 5 to city 87?
if cities.connected(5, 87) {
    println!("Cities are connected!");
}
```

### 3. **Connected Components in Graphs**

```rust
fn find_components(edges: &[Edge], n: usize) -> Vec<Vec<usize>> {
    let mut uf = UnionFind::new(n);
    
    // Process all edges
    for edge in edges {
        uf.union(edge.u, edge.v);
    }
    
    // Group elements by component
    let mut components = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        components.entry(root).or_insert(Vec::new()).push(i);
    }
    
    components.into_values().collect()
}
```

### 4. **Cycle Detection**

```rust
// Detect if adding edge creates cycle in undirected graph
fn has_cycle_after_adding(uf: &mut UnionFind, u: usize, v: usize) -> bool {
    if uf.connected(u, v) {
        return true; // already connected = would create cycle
    }
    uf.union(u, v); // safe to add
    false
}
```

### 5. **Social Networks (Friend Circles)**

```rust
// Find friend circles: groups where everyone is connected through friendships
let mut people = UnionFind::new(8);

// Friendship connections
people.union(alice, bob);     // Alice and Bob are friends
people.union(bob, carol);     // Bob and Carol are friends
people.union(dave, eve);      // Dave and Eve are friends

// Query: Are Alice and Carol in the same friend circle?
if people.connected(alice, carol) {
    println!("Alice and Carol are in the same friend group!");
}
```

## ⚡ Performance Characteristics

### **Time Complexity**

| Operation | Without Optimizations | Path Compression Only | Union by Rank Only | Both Combined |
|-----------|----------------------|---------------------|-------------------|---------------|
| `new(n)` | O(n) | O(n) | O(n) | O(n) |
| `find(x)` | O(n) worst | O(log n) amortized | O(log n) worst | **O(α(n))** amortized |
| `union(x,y)` | O(n) worst | O(log n) amortized | O(log n) worst | **O(α(n))** amortized |
| `connected(x,y)` | O(n) worst | O(log n) amortized | O(log n) worst | **O(α(n))** amortized |

### **Space Complexity**

- **Basic**: O(n) for parent array
- **With Union by Rank**: O(n) for parent + rank arrays  
- **With Size Tracking**: O(n) for parent + size arrays
- **Overall**: O(n) regardless of optimizations

### **Real-World Performance**

For practical input sizes (n < 10^6), α(n) ≤ 4, making operations effectively **constant time**!

## 🔄 Advanced Variants

### **Weighted Union-Find**

Track distances/weights between connected elements:

```rust
struct WeightedUnionFind {
    parent: Vec<usize>,
    weight: Vec<i64>, // weight from element to parent
    rank: Vec<usize>,
}

// Can query distance between connected elements
fn distance(&mut self, x: usize, y: usize) -> Option<i64>
```

### **Union-Find with Undo**

Support operation rollback for backtracking algorithms:

```rust
struct UndoableUnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>, 
    history: Vec<Operation>, // operation stack
}

fn undo(&mut self) -> bool // undo last union operation
```

### **Persistent Union-Find**

Immutable versions with structural sharing:

```rust
struct PersistentUnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    version: usize,
}

fn union(&self, x: usize, y: usize) -> Self // returns new version
```

## 🎓 Learning Path & Mastery

### **Beginner Level**

1. ✅ Understand the **core problem**: tracking disjoint sets
2. ✅ Implement **basic Union-Find** with simple find/union
3. ✅ Learn the **fundamental invariant**: same root = same component
4. ✅ Practice with simple **connectivity queries**

### **Intermediate Level**  

1. ✅ Add **path compression** optimization
2. ✅ Add **union by rank** optimization  
3. ✅ Understand **amortized analysis** and α(n) complexity
4. ✅ Implement **Kruskal's MST algorithm**
5. ✅ Solve **connected components** problems

### **Advanced Level**

1. ✅ Master **real-world applications** (5+ different domains)
2. ✅ Implement **advanced variants** (weighted, undoable, persistent)
3. ✅ **Performance benchmarking** and complexity verification
4. ✅ **Problem-solving patterns** for competitive programming
5. ✅ **System design** applications at scale

### **Expert Level**

1. 🔄 **Distributed Union-Find** across multiple machines
2. 🔄 **Lock-free concurrent** Union-Find for parallel processing
3. 🔄 **Cache-optimized** implementations for HPC applications
4. 🔄 **Custom variants** for domain-specific requirements

## 🧪 Testing & Validation Strategies

### **Correctness Testing**

```rust
#[test]
fn test_union_find_correctness() {
    let mut uf = UnionFind::new(5);
    
    // Initial state: each element in its own set
    assert_eq!(uf.count(), 5);
    assert!(!uf.connected(0, 1));
    
    // Union operations
    assert!(uf.union(0, 1));   // successful union
    assert!(!uf.union(0, 1));  // already connected
    assert_eq!(uf.count(), 4);
    assert!(uf.connected(0, 1));
    
    // Transitive connectivity
    uf.union(1, 2);
    assert!(uf.connected(0, 2)); // 0-1-2 all connected
}
```

### **Performance Testing**

```rust
#[test]  
fn test_path_compression_performance() {
    let mut uf = UnionFind::new(1000);
    
    // Create long chain: 0-1-2-3-...-999
    for i in 0..999 {
        uf.union(i, i + 1);
    }
    
    // First find is expensive (traverses long path)
    let start = Instant::now();
    let root1 = uf.find(999);
    let time1 = start.elapsed();
    
    // Second find should be fast (path compressed)
    let start = Instant::now();
    let root2 = uf.find(999);
    let time2 = start.elapsed();
    
    assert_eq!(root1, root2);
    assert!(time2 < time1 / 2); // path compression effect
}
```

### **Application Testing**

```rust
#[test]
fn test_kruskals_algorithm() {
    let mut edges = vec![
        Edge { u: 0, v: 1, weight: 4 },
        Edge { u: 0, v: 2, weight: 2 },
        Edge { u: 1, v: 2, weight: 3 },
        Edge { u: 1, v: 3, weight: 5 },
        Edge { u: 2, v: 3, weight: 1 },
    ];
    
    let mst = kruskal_mst(&mut edges, 4);
    
    assert_eq!(mst.len(), 3); // n-1 edges in MST
    let total_weight: i32 = mst.iter().map(|e| e.weight).sum();
    assert_eq!(total_weight, 6); // minimal weight
}
```

## 🚀 Production Considerations

### **Error Handling**

```rust
#[derive(Debug)]
enum UnionFindError {
    IndexOutOfBounds(usize, usize), // index, max_size
    EmptyStructure,
}

type Result<T> = std::result::Result<T, UnionFindError>;

impl UnionFind {
    fn find(&mut self, x: usize) -> Result<usize> {
        if x >= self.parent.len() {
            return Err(UnionFindError::IndexOutOfBounds(x, self.parent.len()));
        }
        // ... implementation
    }
}
```

### **Thread Safety**

```rust
use std::sync::{Arc, RwLock};

type ConcurrentUnionFind = Arc<RwLock<UnionFind>>;

fn parallel_union_operations(uf: ConcurrentUnionFind, operations: Vec<(usize, usize)>) {
    operations.into_par_iter().for_each(|(x, y)| {
        let mut uf = uf.write().unwrap();
        uf.union(x, y);
    });
}
```

### **Memory Optimization**

```rust
// For sparse graphs, use HashMap-based implementation
struct SparseUnionFind {
    parent: HashMap<usize, usize>,
    rank: HashMap<usize, usize>,
}

// For very large datasets, consider compression
struct CompressedUnionFind {
    parent: Vec<u32>,  // Use u32 instead of usize for large datasets
    rank: Vec<u8>,     // Rank rarely exceeds 255
}
```

## 🔗 Integration with Other Algorithms

### **Graph Algorithms**

- **[[Kruskal's Algorithm]]**: MST construction with O(E log E) complexity
- **[[Connected Components]]**: O(V + E) graph traversal alternative  
- **[[Cycle Detection]]**: Online cycle detection during edge insertion
- **[[Lowest Common Ancestor]]**: Offline LCA queries using Union-Find

### **Network Algorithms**

- **[[Network Flow]]**: Augmenting path construction in residual graphs
- **[[Clustering]]**: Community detection in social networks
- **[[Percolation]]**: Connectivity threshold analysis in random graphs

### **Computational Geometry**

- **[[convex-hull-algorithm]]**: Dynamic maintenance of convex hull updates
- **[[Voronoi Diagrams]]**: Region connectivity in spatial partitioning
- **[[Mesh Generation]]**: Triangle connectivity in finite element methods

## 🐛 Common Pitfalls & Debug Strategies

### **Path Compression Implementation Bugs**

```rust
// ❌ WRONG: Infinite recursion possible
fn find_buggy(&mut self, x: usize) -> usize {
    if self.parent[x] != x {
        self.parent[x] = self.find_buggy(x); // BUG: should be self.parent[x]
    }
    self.parent[x]
}

// ✅ CORRECT: Proper path compression
fn find(&mut self, x: usize) -> usize {
    if self.parent[x] != x {
        self.parent[x] = self.find(self.parent[x]); // Correct recursive call
    }
    self.parent[x]
}
```

### **Union by Rank Bugs**

```rust
// ❌ WRONG: Rank not updated correctly  
fn union_buggy(&mut self, x: usize, y: usize) -> bool {
    let root_x = self.find(x);
    let root_y = self.find(y);
    
    if root_x == root_y { return false; }
    
    if self.rank[root_x] <= self.rank[root_y] { // BUG: should handle equality
        self.parent[root_x] = root_y;
        // BUG: Missing rank increment when equal
    } else {
        self.parent[root_y] = root_x;
    }
    true
}

// ✅ CORRECT: Handle equal ranks properly
fn union(&mut self, x: usize, y: usize) -> bool {
    // ... find roots ...
    
    if self.rank[root_x] < self.rank[root_y] {
        self.parent[root_x] = root_y;
    } else if self.rank[root_x] > self.rank[root_y] {
        self.parent[root_y] = root_x;
    } else {
        self.parent[root_y] = root_x;
        self.rank[root_x] += 1; // CRITICAL: increment rank when equal
    }
    true
}
```

### **Debugging Techniques**

1. **Visualize Tree Structure**: Print parent arrays and draw trees
2. **Trace Operations**: Log each find/union with before/after states
3. **Invariant Checking**: Verify tree properties after each operation
4. **Deterministic Testing**: Use fixed seeds for reproducible test cases

## 📚 Research & Advanced Topics

### **Theoretical Foundations**

- **Inverse Ackermann Function**: Mathematical analysis of α(n) complexity
- **Amortized Analysis**: Potential method proof of O(α(n)) bound
- **Lower Bounds**: Theoretical limits on disjoint set operations

### **Advanced Optimizations**

- **Path Halving**: Alternate path compression strategy
- **Path Splitting**: Another compression variant with different trade-offs
- **Link-Cut Trees**: Dynamic tree operations extending Union-Find

### **Applications in Research**

- **Computational Biology**: Phylogenetic tree construction
- **Machine Learning**: Clustering and community detection
- **Distributed Systems**: Consensus protocols and partition tolerance
- **Computer Graphics**: Mesh processing and connectivity queries

## 📖 References & Further Reading

### **Classic Papers**

- Tarjan, R. E. (1975). "Efficiency of a Good But Not Linear Set Union Algorithm"
- Tarjan, R. E., & van Leeuwen, J. (1984). "Worst-case Analysis of Set Union Algorithms"

### **Textbook Coverage**

- **CLRS**: Introduction to Algorithms, Chapter 21 (Data Structures for Disjoint Sets)
- **Sedgewick**: Algorithms, Chapter 1.5 (Union-Find)
- **Skiena**: Algorithm Design Manual, Chapter 6 (Weighted Graph Algorithms)

### **Online Resources**

- **CP-Algorithms**: Comprehensive competitive programming reference
- **Visualizations**: VisuAlgo Union-Find interactive demonstrations
- **Leetcode**: Practice problems tagged with Union-Find

---

## *Links:*

**Core Concepts:** [[mission-10]] | [[graph-algorithms]] | [[Tree Algorithms]] | [[Performance Optimization]]

**Applications:** [[Kruskal's Algorithm]] | [[Connected Components]] | [[Cycle Detection]] | [[Minimum Spanning Tree]] | [[union-find-patterns]]

**Advanced Topics:** [[Path Compression]] | [[Union by Rank]] | [[Inverse Ackermann Function]] | [[Amortized Analysis]]

**Implementation:** [[deterministic-debugging]] | [[Testing Strategies]] | [[V-Cycle Methodology]] | [[Algorithm Design Patterns]]

**Mission System:** [[Missions Overview]] | [[mission-9]] | [[Advanced Examples]] | [[competitive-programming]]

**AoC Examples:** [[aoc-day-08]] | [[AoC Patterns MOC]]

---

*The Union-Find algorithm represents one of the most elegant examples of how simple optimizations can transform an algorithm from impractical O(n) to practically constant O(α(n)) performance. Its widespread applications across graph theory, network analysis, and computational geometry make it an essential algorithm for any serious programmer's toolkit.*
