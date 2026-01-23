# Longest Path Problem (NP-Hard)

**Type**: Algorithm / Computational Complexity  
**Domain**: Graph Theory, NP-Completeness  
**Created**: 2026-01-23  
**Context**: AoC 2023 Day 23 - Understanding why longest path is exponentially hard

---

## Core Concept

The **longest path problem** asks: Given a graph $G = (V, E)$, what is the **maximum-length** simple path from vertex $s$ to vertex $t$?

Unlike shortest path (polynomial with Dijkstra), longest path is **NP-hard** - no known polynomial-time algorithm exists.

---

## Mathematical Definition

**Input**: 
- Graph $G = (V, E)$ (directed or undirected)
- Start vertex $s \in V$
- Goal vertex $t \in V$
- Optional: Edge weights $w: E \to \mathbb{R}$

**Output**:
- Simple path $P = \langle v_0, v_1, \ldots, v_k \rangle$ where:
  - $v_0 = s$, $v_k = t$
  - All vertices distinct (simple path)
  - Maximizes $|P|$ (vertex count) or $\sum_{i=0}^{k-1} w(v_i, v_{i+1})$ (total weight)

**Decision Version** (NP-complete):
Does there exist a simple path from $s$ to $t$ with length $\geq k$?

---

## Why NP-Hard?

### 1. No Optimal Substructure

**Shortest path** has optimal substructure:
- If path $P = s \to \ldots \to u \to \ldots \to t$ is optimal
- Then subpath $s \to \ldots \to u$ is also optimal to $u$
- ⇒ Can use dynamic programming (Dijkstra, Bellman-Ford)

**Longest path** does NOT:
```
Example graph:
s → a → b → t
s → c → b → t

Longest s→b: s→a→b (length 2)
Longest s→t: s→c→b→t (length 3)

But subpath s→c→b (length 2) ≠ longest s→b!
```

Cannot break into independent subproblems.

### 2. Reduction from Hamiltonian Path

**Hamiltonian Path Problem** (known NP-complete):
- Does graph have simple path visiting every vertex exactly once?

**Reduction**: Hamiltonian Path ≤ Longest Path
- Hamiltonian path exists ⟺ Longest path has length $|V| - 1$
- Since Hamiltonian Path is NP-complete, Longest Path is NP-hard

### 3. Exponential Search Space

For graph with $n$ vertices:
- Number of simple paths ≈ $n!$ in worst case (complete graph)
- Backtracking explores $O(b^d)$ states where:
  - $b$ = branching factor (avg neighbors)
  - $d$ = path depth (can be $n$)
- Example: 4-neighbor grid, 2000-tile path → $O(4^{2000})$ states!

---

## Contrast: Shortest vs Longest Path

| Property | Shortest Path | Longest Path |
|----------|---------------|--------------|
| **Optimal Substructure** | YES ✓ | NO ✗ |
| **Greedy Works** | Sometimes (Dijkstra) | Never |
| **DP Works** | YES (Bellman-Ford) | NO |
| **Polynomial Time** | YES (O(V + E log V)) | NO (NP-hard) |
| **Complexity Class** | P | NP-hard |
| **Negative Cycles** | Problem if exist | Not relevant |

**Key Insight**: Adding edges can make shortest path longer (good for DP), but makes longest path harder (more choices).

---

## Solution Approaches

### 1. Backtracking DFS (Exact)

```rust
fn longest_path_dfs(current: Vertex, goal: Vertex, 
                    visited: &mut HashSet<Vertex>) -> usize {
    if current == goal {
        return 0;
    }
    
    let mut max_length = 0;
    
    for neighbor in graph.neighbors(current) {
        if visited.contains(&neighbor) {
            continue;  // Avoid cycles
        }
        
        visited.insert(neighbor);              // Make choice
        let length = longest_path_dfs(neighbor, goal, visited);
        max_length = max_length.max(length + 1);
        visited.remove(&neighbor);             // Backtrack
    }
    
    max_length
}
```

**Complexity**: $O(b^d)$ where $b$ = branching, $d$ = depth
- **Pros**: Finds exact answer
- **Cons**: Exponential time, can stack overflow

### 2. Graph Contraction (Preprocessing)

Reduce state space before searching:
- Collapse degree-2 vertices (corridors) into weighted edges
- Keep only junctions (degree ≠ 2)
- Example: 20,000 vertices → 35 junctions (570× reduction!)

See [[graph-contraction-optimization]] for details.

### 3. Branch and Bound

Prune branches that cannot improve current best:
```rust
fn longest_path_bnb(current: Vertex, goal: Vertex, 
                    visited: &mut HashSet<Vertex>,
                    current_length: usize,
                    best: &mut usize) -> usize {
    if current == goal {
        *best = (*best).max(current_length);
        return current_length;
    }
    
    // Upper bound: current + optimistic estimate
    let upper_bound = current_length + heuristic_max_remaining(current, goal);
    if upper_bound <= *best {
        return 0;  // Prune this branch
    }
    
    // ... continue search ...
}
```

### 4. Approximation Algorithms

For very large graphs where exact solution infeasible:
- **Greedy heuristic**: Always take longest available edge (not optimal)
- **Local search**: Start with random path, improve iteratively
- **Genetic algorithms**: Evolve population of paths

**Tradeoff**: Fast but no guarantee of optimality

---

## AoC 2023 Day 23 Application

### Part 1: Constrained Longest Path

**Setup**: 141×141 hiking trail with directional slopes
- Slopes force specific directions (one-way edges)
- Natural pruning reduces search space

**Solution**: Simple DFS backtracking
- Answer: 2,182 steps
- Runtime: 22.9ms
- Recursion depth: ~2,200 (manageable)

```rust
// Slopes constrain movement
fn can_move(&self, from: Coord, to: Coord, dir: Direction) -> bool {
    match self.grid[to] {
        Tile::SlopeNorth => dir == Direction::North,
        Tile::SlopeSouth => dir == Direction::South,
        // ... other slopes
        _ => true,
    }
}
```

### Part 2: Unconstrained Longest Path

**Setup**: Same grid, but treat slopes as normal paths (bidirectional)
- No constraints → exponential explosion!
- Naive DFS → stack overflow (7,000 depth)

**Solution**: Graph contraction + DFS
1. Identify 35 junctions (degree > 2 vertices)
2. Collapse 20,000 tiles → 35-vertex graph
3. DFS on contracted graph

**Result**:
- Answer: 6,670 steps
- Runtime: 2.38s
- Recursion depth: 35 (safe)

---

## When to Use Each Approach

| Graph Size | Constraints | Recommended Approach |
|------------|-------------|----------------------|
| Small (<100 vertices) | Any | DFS backtracking |
| Medium (<1000) | Heavy constraints | DFS with pruning |
| Medium | Light constraints | Graph contraction + DFS |
| Large (>1000) | Any | Approximation algorithms |
| Any | Many corridors | Graph contraction first |

---

## Related Concepts

- [[graph-theory-fundamentals]] - Basic graph concepts
- [[graph-contraction-optimization]] - State space reduction technique
- [[backtracking-patterns]] - DFS with visited tracking
- [[np-completeness]] - Complexity theory
- [[hamiltonian-path-problem]] - Related NP-complete problem
- [[state-space-reduction]] - General optimization principle

---

## References

- **AoC 2023 Day 23**: [day23.rs](../advent_of_code/aoc2023/src/solver/day23.rs)
- **Function Guide**: [day23_function_guide.md](../advent_of_code/aoc2023/Problem_Statements/days/day23_function_guide.md)
- **Classic Paper**: "The Longest Path Problem" (Garey & Johnson, Computers and Intractability, 1979)

---

*Tags: #graph-theory #np-hard #complexity-theory #algorithms #aoc2023 #backtracking*
