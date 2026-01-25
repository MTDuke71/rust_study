# Day 25: Snowverload - Function Guide

## Problem Overview
Find 3 edges in an undirected graph that, when removed, split the graph into exactly 2 connected components. This is a minimum cut problem where we need to disconnect the graph with minimal edge removals.

**Answer**: 558376 (product of component sizes after cut)

## Performance Benchmarks
- **Part 1**: ~689.67 ms (edge betweenness + combination testing)
- **Part 2**: ~584 ps (string literal)

## Core Algorithm: Edge Betweenness with Combination Search

The solution uses a two-phase approach:
1. **Edge Betweenness**: Identify candidate edges that carry high "traffic" (used in many shortest paths)
2. **Combination Testing**: Try combinations of top candidates to find the exact 3-cut

---

## Function Reference

### Main Entry Points

#### `solve_part1(input: &str) -> usize`
**Purpose**: Find the 3-edge minimum cut and return product of component sizes

**Algorithm**:
1. Parse input into undirected graph
2. Find top 20 candidate edges with highest betweenness
3. Try all C(20,3) = 1140 combinations of 3 edges
4. For each combination, check if it creates exactly 2 components
5. Return size1 × size2 when valid cut found

**Time Complexity**: O(V² + C(k,3) × V) where k=20 candidates
- V² for betweenness calculation
- C(20,3) = 1140 combinations
- V per component check

**Example**:
```rust
let input = "jqt: rhn xhk nvd\nrsh: frs pzl lsr\n...";
let result = solve_part1(input);
// Returns 54 (9 × 6 for example)
```

---

#### `solve_part2(_input: &str) -> &'static str`
**Purpose**: Part 2 completion message

**Returns**: "Merry Christmas! 🎄"

---

### Graph Construction

#### `parse_input(input: &str) -> Graph`
**Purpose**: Build undirected adjacency list from input

**Input Format**:
```
node1: neighbor1 neighbor2 neighbor3
node2: neighbor4 neighbor5
```

**Returns**: `HashMap<String, HashSet<String>>` - bidirectional edges

**Implementation Details**:
- Each edge stored in both directions (a→b and b→a)
- Nodes auto-created when referenced
- Empty lines skipped

**Example**:
```rust
let input = "jqt: rhn xhk\nrhn: xhk";
let graph = parse_input(input);
// graph["jqt"] = {rhn, xhk}
// graph["rhn"] = {jqt, xhk}
// graph["xhk"] = {jqt, rhn}
```

---

### Edge Betweenness Algorithm

#### `find_candidate_edges(graph: &Graph, top_n: usize) -> Vec<Edge>`
**Purpose**: Identify edges with highest betweenness (used in many shortest paths)

**Algorithm**:
1. For each node in graph:
   - Run BFS to find shortest paths
   - Count which edges are used in BFS tree
2. Sum edge usage across all BFS runs
3. Sort edges by total usage count (descending)
4. Return top N edges

**Parameters**:
- `graph`: The full graph
- `top_n`: Number of candidate edges to return (typically 20)

**Time Complexity**: O(V² × E) where V = nodes, E = edges
- V BFS runs (one per node)
- Each BFS is O(V + E)

**Why It Works**:
- Edges that bridge two large components appear in MANY shortest paths
- The 3 cut edges are bottlenecks between components
- High betweenness → high probability of being a cut edge

**Example**:
```rust
let candidates = find_candidate_edges(&graph, 20);
// Returns top 20 most frequently-used edges in BFS traversals
// e.g., [("hfx", "pzl"), ("bvb", "cmg"), ("jqt", "nvd"), ...]
```

---

#### `bfs_count_paths(graph: &Graph, start: &str) -> HashMap<Edge, usize>`
**Purpose**: Run BFS from start node and count edge usage in shortest-path tree

**Algorithm**:
1. Initialize visited set and queue with start node
2. For each node in BFS order:
   - Visit unvisited neighbors
   - Record edge used to reach neighbor
   - Increment edge usage count
3. Return map of (edge → usage count)

**Time Complexity**: O(V + E)

**Returns**: Edges used in THIS BFS tree (not all shortest paths, just the tree)

**Limitation**: Only counts edges in ONE shortest-path tree per source
- Doesn't find ALL shortest paths (too expensive)
- Approximation is sufficient for betweenness heuristic

**Example**:
```rust
let paths = bfs_count_paths(&graph, "jqt");
// Returns edges used when doing BFS from "jqt"
// Each edge counted once per BFS
```

---

### Cut Testing

#### `try_cut(graph: &Graph, cut_edges: &[Edge]) -> Option<(usize, usize)>`
**Purpose**: Test if removing these 3 edges creates exactly 2 components

**Algorithm**:
1. Clone graph
2. Remove all 3 edges (bidirectional)
3. Find connected components via BFS
4. If exactly 2 components: return their sizes
5. Otherwise: return None

**Time Complexity**: O(V + E)

**Returns**: 
- `Some((size1, size2))` if valid 2-cut
- `None` if not exactly 2 components

**Example**:
```rust
let cut = vec![
    ("hfx".to_string(), "pzl".to_string()),
    ("bvb".to_string(), "cmg".to_string()),
    ("jqt".to_string(), "nvd".to_string()),
];
let result = try_cut(&graph, &cut);
// Returns Some((9, 6)) for the example
```

---

### Utility Functions

#### `normalize_edge(edge: Edge) -> Edge`
**Purpose**: Canonical edge representation for undirected graphs

**Algorithm**: Return `(a, b)` where `a < b` lexicographically

**Why Needed**: Treat (a,b) and (b,a) as same edge
- BFS might record edge as (a,b) or (b,a) depending on traversal order
- Normalization ensures both map to same HashMap key

**Example**:
```rust
let e1 = ("xyz".to_string(), "abc".to_string());
let e2 = ("abc".to_string(), "xyz".to_string());
assert_eq!(normalize_edge(e1), normalize_edge(e2));
// Both become ("abc", "xyz")
```

---

#### `component_bfs(graph: &Graph, start: &str, visited: &mut HashSet<String>) -> usize`
**Purpose**: Find size of connected component containing start node

**Algorithm**:
1. BFS from start node
2. Mark all reachable nodes as visited
3. Return count of nodes reached

**Time Complexity**: O(V + E) for component

**Side Effect**: Updates `visited` set with all nodes in this component

**Usage**: Called repeatedly to find all components:
```rust
let mut visited = HashSet::new();
let mut sizes = Vec::new();
for node in graph.keys() {
    if !visited.contains(node) {
        sizes.push(component_bfs(&graph, node, &mut visited));
    }
}
// sizes = [component1_size, component2_size, ...]
```

---

## Type Definitions

```rust
type Graph = HashMap<String, HashSet<String>>;  // Adjacency list
type Edge = (String, String);                    // Undirected edge
```

---

## Algorithm Deep Dive

### Why Edge Betweenness?

**Intuition**: The 3 edges connecting two large components are "bottlenecks"
- All shortest paths between components must use these edges
- High edge betweenness → high probability of being a cut edge

**Formal Definition**:
- Edge betweenness = number of shortest paths that use this edge
- We approximate with BFS tree counting (simpler than all-pairs shortest paths)

**Trade-off**:
- Exact betweenness: O(V³) using Floyd-Warshall
- BFS approximation: O(V² × E) but good enough for this problem

---

### Why Combination Testing?

**Problem**: Edge betweenness is a heuristic, not guaranteed exact
- Top 3 edges by betweenness *might* not be the correct cut
- Need to verify by testing combinations

**Solution**: 
- Find top 20 candidates (k=20)
- Test all C(20,3) = 1140 combinations
- First combination that creates exactly 2 components wins

**Complexity**:
- C(20,3) = 1140 combinations
- Each test: O(V + E) to check components
- Total: ~1140 × (V + E) ≈ 1140 × 3000 ≈ 3.4M operations
- Fast enough (~690ms)

**Why k=20?**:
- Empirically sufficient to contain the 3 cut edges
- Could use k=10 (faster) or k=30 (safer) if needed

---

### Alternative Approaches

#### 1. Karger's Algorithm (Randomized Min-Cut)
**Idea**: Randomly contract edges until 2 nodes remain
- Probability of success: ≥ 1/C(n,2) per run
- Need O(n² log n) runs for high success probability
- Our approach is deterministic and faster

#### 2. Stoer-Wagner Algorithm (Deterministic Min-Cut)
**Idea**: Find global minimum cut via max-flow
- Guaranteed to find minimum cut
- O(V³) complexity
- Overkill when we know cut size = 3

#### 3. All-Pairs Shortest Paths Betweenness
**Idea**: Exact betweenness calculation
- O(V³) via Floyd-Warshall
- More accurate but slower
- Our BFS approximation works fine

---

## Test Cases

### Example Input
```
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
```

**Cut Edges**: (hfx,pzl), (bvb,cmg), (jqt,nvd)

**Components**:
- Component 1: cmg, frs, lhk, lsr, nvd, pzl, qnr, rsh, rzs (9 nodes)
- Component 2: bvb, hfx, jqt, ntq, rhn, xhk (6 nodes)

**Answer**: 9 × 6 = 54

---

### Real Input
**Graph Size**: ~1500 nodes, ~3000 edges

**Cut Edges**: Found via betweenness (specific edges depend on input)

**Answer**: 558376

**Performance**: ~690ms (edge betweenness is bottleneck)

---

## Key Insights

1. **Graph Problem Recognition**:
   - "Disconnect components" → minimum cut
   - "Three wires" → looking for 3-edge cut
   - Undirected graph with bidirectional edges

2. **Edge Betweenness Intuition**:
   - Bottleneck edges appear in many shortest paths
   - BFS tree approximation is fast and effective
   - Top-k candidates contain the answer

3. **Combination Search**:
   - Heuristic isn't perfect → test combinations
   - C(20,3) = 1140 is tractable
   - Early exit when valid cut found

4. **Performance Optimization**:
   - BFS instead of Dijkstra (unweighted graph)
   - HashSet for O(1) neighbor lookup
   - Clone graph once per combination (cheaper than rebuilding)

5. **Rust Patterns**:
   - `HashMap<String, HashSet<String>>` for adjacency list
   - Tuple edge representation with normalization
   - Option return type for try_cut (Some/None)

---

## Related Problems
- **Mission 8**: Graph trait, BFS/DFS implementations
- **Mission 10**: Union-Find (alternative component detection)
- **Day 10 (2023)**: Graph traversal with pipes
- **Day 23 (2023)**: Longest path (graph algorithms)

---

## Zettelkasten Links
- [[graph-minimum-cut]]
- [[edge-betweenness-centrality]]
- [[bfs-shortest-paths]]
- [[connected-components]]
- [[aoc2023-day25]]
