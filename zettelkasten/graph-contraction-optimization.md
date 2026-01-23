# Graph Contraction Optimization

**Type**: Algorithm Technique  
**Domain**: Graph Theory, Optimization  
**Created**: 2026-01-23  
**Context**: AoC 2023 Day 23 - Solved stack overflow via state space reduction

---

## Core Concept

**Graph contraction** simplifies graphs by collapsing **degree-2 vertices** (corridors with no branching) into **weighted edges** between junctions.

### Formal Definition

Given graph $G = (V, E)$, a **path contraction** produces $G' = (V', E')$ where:

$$V' = \{v \in V \mid \text{degree}(v) \neq 2\} \cup \{\text{start}, \text{goal}\}$$

$$E' = \{(u, v, w) \mid \exists \text{ path } u \to v \text{ of length } w \text{ in } G \text{ with only degree-2 vertices}\}$$

In plain terms:
- **Keep**: Junctions (degree ≠ 2), start, goal
- **Collapse**: Corridors (degree-2 chains) into single weighted edges
- **Preserve**: All path lengths remain identical

---

## When to Use

✅ **Good for**:
- Grid mazes with long corridors
- Graphs with many degree-2 "bottleneck" vertices
- Reducing state space before DFS/BFS
- Preprocessing for repeated pathfinding queries

❌ **Not suitable for**:
- Dense graphs (few corridors to collapse)
- Directed graphs with asymmetric edges (needs special handling)
- When exact graph structure must be preserved

---

## Algorithm

### Phase 1: Identify Junctions

```rust
let mut junctions = HashSet::new();
junctions.insert(start);
junctions.insert(goal);

for vertex in graph.vertices() {
    let degree = graph.neighbors(vertex).count();
    if degree != 2 {
        junctions.insert(vertex);  // Branch point or dead end
    }
}
```

**Complexity**: O(V + E) to count degrees

### Phase 2: Trace Corridors

```rust
let mut contracted_graph = HashMap::new();

for &junction in &junctions {
    let mut edges = Vec::new();
    
    for initial_neighbor in graph.neighbors(junction) {
        let mut current = initial_neighbor;
        let mut distance = 1;
        let mut visited = HashSet::from([junction, current]);
        
        loop {
            if junctions.contains(&current) {
                edges.push((current, distance));  // Found next junction
                break;
            }
            
            // Follow corridor (should have exactly 1 unvisited neighbor)
            let next_unvisited: Vec<_> = graph.neighbors(current)
                .filter(|n| !visited.contains(n))
                .collect();
            
            match next_unvisited.len() {
                0 => break,  // Dead end
                1 => {
                    current = next_unvisited[0];
                    distance += 1;
                    visited.insert(current);
                }
                _ => break,  // Should be junction (error case)
            }
        }
    }
    
    contracted_graph.insert(junction, edges);
}
```

**Complexity**: O(V + E) - each vertex visited once during tracing

---

## Correctness Proof

**Theorem**: Graph contraction preserves all path lengths.

**Proof**:
1. Let $P$ be any path from $s$ to $t$ in original graph $G$
2. $P$ can be partitioned into junction-to-junction segments
3. Each segment traverses a corridor (chain of degree-2 vertices)
4. Corridor has no branches ⇒ only one possible traversal
5. Distance through corridor = sum of edge weights (fixed)
6. Replacing corridor with single edge of same weight preserves length
7. ∴ $\text{length}_G(P) = \text{length}_{G'}(P)$ for all paths $P$. QED □

---

## AoC 2023 Day 23 Case Study

### The Problem

Longest path through 141×141 hiking trail grid:
- Part 1: With slope restrictions (directed edges) → 2,182 steps
- Part 2: Ignoring slopes (bidirectional) → 6,670 steps

### Naive Approach (Part 2)

```rust
fn dfs(&self, current: Coord, visited: &mut HashSet<Coord>) -> usize {
    if current == goal { return 0; }
    
    let mut max_length = 0;
    for next in current.neighbors() {
        if !visited.contains(&next) {
            visited.insert(next);
            let length = self.dfs(next, visited);
            max_length = max_length.max(length + 1);
            visited.remove(&next);
        }
    }
    max_length
}
```

**Result**: Stack overflow at ~7,000 recursion depth! 💥

### Optimized Approach

**Step 1**: Contract graph
- Original: ~20,000 walkable tiles
- Contracted: ~35 junctions
- **Reduction**: 570× fewer vertices!

**Step 2**: DFS on contracted graph
```rust
fn dfs_graph(&self, current: Coord, visited: &mut HashSet<Coord>,
             graph: &HashMap<Coord, Vec<(Coord, usize)>>) -> usize {
    if current == goal { return 0; }
    
    let mut max_length = 0;
    for &(next_junction, corridor_distance) in &graph[&current] {
        if !visited.contains(&next_junction) {
            visited.insert(next_junction);
            let length = self.dfs_graph(next_junction, visited, graph);
            max_length = max_length.max(length + corridor_distance);
            visited.remove(&next_junction);
        }
    }
    max_length
}
```

**Result**: ✓ Completes in 2.38s with only 35 recursion depth

### Performance Comparison

| Metric | Naive DFS | Graph Contraction |
|--------|-----------|-------------------|
| **Vertices** | 20,000 | 35 |
| **Max Recursion** | 7,000 | 35 |
| **Stack Usage** | **OVERFLOW** | 3.5 KB |
| **Runtime** | N/A (crash) | 2.38s |
| **Speedup** | - | ∞ (impossible → working) |

---

## Implementation Notes

### Grid-Specific Optimizations

For 2D grids, degree = number of walkable neighbors:
```rust
fn is_junction(&self, coord: Coord) -> bool {
    let neighbor_count = coord.neighbors(self.rows, self.cols)
        .into_iter()
        .filter(|(next, dir)| self.can_move(coord, *next, *dir))
        .count();
    
    neighbor_count != 2  // Junction if ≠ 2 neighbors
}
```

### Directed vs Undirected

- **Undirected** (Day 23 Part 2): Simple contraction works
- **Directed** (Day 23 Part 1 with slopes): Need directional corridor tracing
  - Check if corridor is bidirectional or one-way
  - Store directed edges in contracted graph

---

## Related Concepts

- [[longest-path-np-hard]] - Why we needed optimization
- [[backtracking-patterns]] - DFS with visited set
- [[graph-theory-fundamentals]] - Basic graph concepts
- [[state-space-reduction]] - General optimization technique
- [[mission-6]] - Grid navigation primitives
- [[mission-8]] - Graph traversal algorithms

---

## References

- **AoC 2023 Day 23**: [day23.rs](../advent_of_code/aoc2023/src/solver/day23.rs)
- **Function Guide**: [day23_function_guide.md](../advent_of_code/aoc2023/Problem_Statements/days/day23_function_guide.md)
- **Algorithms Reference**: [algorithms-reference.md](../advent_of_code/aoc2023/Problem_Statements/algorithms-reference.md)

---

*Tags: #graph-theory #optimization #state-space-reduction #aoc2023 #mission6 #mission8*
