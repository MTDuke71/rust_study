
# Graph Representation - Adjacency Lists, Matrices, and Edge Lists

*How you store a graph determines which algorithms are easy, which are fast, and where the memory goes.*

---

## 🎯 Core Concept

A **graph representation** is the concrete data structure you choose to model:

- vertices (nodes)
- edges (connections)
- direction (directed vs undirected)
- weights/costs (weighted vs unweighted)

Most algorithm complexity statements assume a representation:

- Adjacency list: $O(V + E)$ storage, fast neighbor iteration.
- Adjacency matrix: $O(V^2)$ storage, constant-time edge existence checks.
- Edge list: $O(E)$ storage, convenient for “scan all edges” algorithms.

## 🧠 Mental Models

- **Adjacency list = "neighbors per node"** (great when you often ask “what can I reach from here?”).
- **Adjacency matrix = "lookup table"** (great when you often ask “is there an edge (u,v)?”).
- **Edge list = "facts"** (great when you often loop: “for every edge, relax/update”).

## 🔍 Detailed Content

### 1) Adjacency list

Best default for sparse graphs (typical in AoC inputs and most real-world networks).

- **Storage**: $O(V + E)$
- **Iterating neighbors**: $O(\deg(v))$
- **Edge existence check**: usually $O(\deg(v))$ (unless you add a set/map)

Rust-shaped representation examples:

- `Vec<Vec<usize>>` for unweighted graphs
- `Vec<Vec<(usize, i64)>>` for weighted graphs
- `HashMap<NodeId, Vec<Edge>>` when node IDs are not dense (often paired with a remapping step)

This is the representation implicitly suggested by:
- [[Dijkstra Algorithm]] (efficient neighbor access)
- [[Pathfinding Strategies]] (sparse graphs, traversal)

See also: [[adjacency-list]] and [[Adjacency List Implementation]].

### 2) Adjacency matrix

Best when $V$ is small and dense, or when you do lots of “does edge exist?” queries.

- **Storage**: $O(V^2)$
- **Edge existence check**: $O(1)$
- **Iterating neighbors**: $O(V)$ per node (scan the row)

In Rust, a common choice is a flat `Vec<bool>`/`Vec<u8>`/`Vec<i64>` with index math.

### 3) Edge list

Best for algorithms that naturally iterate over all edges.

- **Storage**: $O(E)$
- **Iterating all edges**: $O(E)$
- **Iterating neighbors**: expensive unless you build an index

This representation is a natural fit for:

- [[Bellman-Ford Algorithm]] (relax every edge repeatedly)

### 4) “Indexing strategy” is part of the representation

Even before you pick list/matrix, decide how nodes are identified:

- **Dense integer IDs (0..V)**: fastest, simplest, enables `Vec`-based storage.
- **External labels (strings, coordinates, etc.)**: often best handled by *remapping* into dense IDs via a `HashMap`.

This is the common integrator move: keep parsing/labeling at the boundary, and run algorithms on dense indices internally.

### 5) Visualization as a representation boundary

For learning/diagnostics, exporting a representation is often more valuable than rendering in-process.

- Emit DOT (Graphviz) or SVG-friendly data
- Keep it separate from the solver so it stays cross-platform and CI-friendly

See: [[cross-platform-graphics]].

## 💡 Key Takeaways

- Adjacency lists are the default for sparse graphs and shortest-path work.
- Matrices trade memory for constant-time edge lookup.
- Edge lists pair naturally with algorithms that "scan all edges" (e.g., Bellman-Ford).
- Node ID strategy (dense vs labeled) is a core design decision, not an implementation detail.

## 🔗 Integration Points

### Builds On
- [[directed-vs-undirected-graphs]] - Direction changes edge modeling
- [[Graph Network Density]] - Dense vs sparse drives the choice

### Enables
- [[graph-algorithms]] - Most algorithms assume a representation
- [[Dijkstra Algorithm]] - Neighbor iteration + weights
- [[Bellman-Ford Algorithm]] - Edge-relaxation over an edge list
- [[Pathfinding Strategies]] - Representation ↔ algorithm selection
- [[missions/mission-9]] - Weighted graphs + pathfinding implementation
- [[AoC Patterns MOC]] - Pattern-level guidance for AoC solutions

---

*Tags: #concept #graph-theory #data-structure #aoc #intermediate*

*Links: [[zettel-index]] | [[graph-algorithms]] | [[AoC Patterns MOC]] | [[missions/mission-9]] | [[cross-platform-graphics]]*

