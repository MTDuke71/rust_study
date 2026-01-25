# Graph Minimum Cut

**Field**: Graph Theory / Network Analysis

**Prerequisites**: [[graph-theory-fundamentals]], [[graph-traversal-bfs]]

---

## 📐 Definition

A **minimum cut** (or min-cut) in a graph is the smallest set of edges whose removal disconnects the graph into two or more components.

**Formal**: Given an undirected graph $G = (V, E)$, a cut is a partition of vertices into two disjoint sets $S$ and $T = V \setminus S$. The cut-set is the set of edges with one endpoint in $S$ and one in $T$. A minimum cut is a cut with the minimum number of edges.

**Intuition**: Imagine a network of connections - the minimum cut finds the weakest point where removing the fewest connections breaks the network into separate pieces.

---

## 🔑 Key Properties/Theorems

### **Property 1**: Uniqueness
- **Statement**: A graph may have multiple minimum cuts with the same cardinality
- **Significance**: Different cuts may have equal minimum size, need to find any one

### **Property 2**: Connected Components
- **Statement**: Removing a minimum cut partitions the graph into exactly 2 connected components (for 2-way cuts)
- **Applications**: Network reliability, clustering, community detection

### **Theorem 1**: Max-Flow Min-Cut Theorem
- **Statement**: In a flow network, the maximum value of a feasible flow equals the minimum capacity of a cut
- **Applications**: Network capacity analysis, resource allocation
- **Note**: Can solve min-cut using max-flow algorithms (Ford-Fulkerson, Edmonds-Karp)

### **Theorem 2**: Karger's Algorithm
- **Statement**: Randomized contraction algorithm finds min-cut with probability $\geq \frac{2}{n(n-1)}$ in one run
- **Complexity**: $O(n^2)$ per run, need $O(n^2 \log n)$ runs for high-probability success
- **Significance**: Simple probabilistic approach, useful for small graphs

---

## 🎯 Common Algorithms

### **1. Edge Betweenness (Heuristic)**
- **Idea**: Edges that appear in many shortest paths are likely cut candidates
- **Complexity**: $O(V^2 \times E)$ for complete betweenness calculation
- **Limitation**: Not guaranteed to find global minimum, but works well in practice
- **Usage**: AoC 2023 Day 25 (our implementation)

### **2. Max-Flow Based (Optimal)**
- **Idea**: Use max-flow min-cut theorem with flow algorithms
- **Complexity**: $O(V \times E^2)$ (Edmonds-Karp) or $O(V^2 \times E)$ (Dinic's)
- **Advantage**: Finds exact minimum cut

### **3. Karger's Randomized Contraction**
- **Idea**: Randomly contract edges until 2 vertices remain
- **Complexity**: $O(n^2)$ per attempt, repeat $O(n^2 \log n)$ times
- **Advantage**: Simple to implement, probabilistic guarantee

### **4. Stoer-Wagner Algorithm**
- **Idea**: Deterministic min-cut for undirected graphs
- **Complexity**: $O(V \times E + V^2 \log V)$
- **Advantage**: Always finds exact min-cut, no randomization

---

## 💻 Rust Implementations

### **AoC 2023 Day 25**: Snowverload
- **Problem**: Disconnect a network of wires into two groups by cutting exactly 3 wires
- **Approach**: Edge betweenness centrality to identify high-traffic edges, test all $\binom{k}{3}$ combinations
- **Algorithm**:
  1. Calculate edge betweenness (count shortest paths using each edge)
  2. Select top $k=20$ high-betweenness edges as candidates
  3. Test all $\binom{20}{3} = 1140$ combinations of 3 edges
  4. For each triple, remove edges and check if graph splits into 2 components
- **Complexity**: $O(V^2 \times E)$ for betweenness + $O(k^3 \times V)$ for combination testing
- **Runtime**: 689.67ms (edge betweenness dominates)
- **Link**: [advent_of_code/aoc2023/src/solver/day25.rs](../../advent_of_code/aoc2023/src/solver/day25.rs)
- **Function Guide**: [Problem_Statements/days/day25_function_guide.md](../../advent_of_code/aoc2023/Problem_Statements/days/day25_function_guide.md)

---

## 📚 Code Example - Edge Betweenness Approach

```rust
use std::collections::{HashMap, HashSet, VecDeque};

type Graph = HashMap<String, HashSet<String>>;
type Edge = (String, String);

/// Calculate edge betweenness centrality via BFS tree counting
fn edge_betweenness(graph: &Graph) -> HashMap<Edge, usize> {
    let mut betweenness = HashMap::new();
    
    // For each vertex as source, count paths through each edge
    for source in graph.keys() {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(source.clone());
        visited.insert(source.clone());
        
        while let Some(current) = queue.pop_front() {
            for neighbor in &graph[&current] {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                    
                    // Increment betweenness for this edge
                    let edge = normalize_edge((&current, neighbor));
                    *betweenness.entry(edge).or_insert(0) += 1;
                }
            }
        }
    }
    
    betweenness
}

/// Normalize edge to canonical form (smaller, larger)
fn normalize_edge((a, b): (&str, &str)) -> Edge {
    if a < b {
        (a.to_string(), b.to_string())
    } else {
        (b.to_string(), a.to_string())
    }
}

/// Test if removing edges splits graph into 2 components
fn try_cut(graph: &Graph, edges: &[Edge]) -> Option<(usize, usize)> {
    // Create modified graph without cut edges
    let mut modified = graph.clone();
    for (a, b) in edges {
        modified.get_mut(a).unwrap().remove(b);
        modified.get_mut(b).unwrap().remove(a);
    }
    
    // BFS to find size of first component
    let start = modified.keys().next()?;
    let component1 = bfs_component_size(&modified, start);
    let total = modified.len();
    
    // Check if split into exactly 2 components
    if component1 != total {
        let component2 = total - component1;
        Some((component1, component2))
    } else {
        None
    }
}
```

**Explanation**: 
- Edge betweenness counts how many BFS trees include each edge
- High-betweenness edges are "bridges" connecting different parts of the graph
- For AoC Day 25 (known min-cut = 3), test top candidates exhaustively
- More sophisticated: Use Girvan-Newman algorithm (iterative edge removal)

---

## 🎯 Applications

### **Network Reliability**
- Find critical connections whose failure isolates parts of network
- Examples: Internet backbone, power grids, transportation networks

### **Community Detection**
- Identify natural clusters in social networks
- Cut edges separate communities with weak connections

### **Image Segmentation**
- Partition image pixels into foreground/background
- Edges represent pixel similarity, min-cut finds optimal boundary

### **VLSI Circuit Design**
- Partition circuit components to minimize wire crossings
- Minimize communication between chip regions

### **Load Balancing**
- Split computational graph to balance workload
- Minimize communication between partitions

---

## 🌳 Related Concepts

- **Prerequisites**: 
  - [[graph-theory-fundamentals]] - Basic graph concepts
  - [[graph-traversal-bfs]] - Breadth-first search
  
- **Related**: 
  - [[connected-components]] - Graph connectivity
  - [[network-flow]] - Max-flow algorithms
  - [[graph-centrality]] - Betweenness centrality measures
  
- **Advanced**:
  - [[spectral-graph-theory]] - Eigenvalue-based clustering
  - [[community-detection]] - Modularity optimization
  - [[graph-partitioning]] - k-way cuts, balanced partitions

---

## 📖 Resources

- **Wikipedia**: [Minimum cut](https://en.wikipedia.org/wiki/Minimum_cut)
- **Wikipedia**: [Max-flow min-cut theorem](https://en.wikipedia.org/wiki/Max-flow_min-cut_theorem)
- **Paper**: Girvan, M. & Newman, M. E. J. (2002). "Community structure in social and biological networks"
- **Book**: *Introduction to Algorithms* (CLRS) - Chapter 26 (Network Flow)
- **Video**: MIT 6.046J - Lecture on Network Flow and Min-Cut

---

*Tags: #mathematics #graph-theory #network-analysis #algorithms #connectivity*

*Created*: 2026-01-25  
*Last Updated*: 2026-01-25  
*Implementations*: 1 (AoC 2023 Day 25)
