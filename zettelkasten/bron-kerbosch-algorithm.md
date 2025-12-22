# Bron-Kerbosch Algorithm - Maximum Clique Finding

**Tags:** #graph-algorithms #clique #backtracking #recursion #aoc-2024-day23 #np-complete #set-operations

**Related:** [[graph-algorithms]], [[backtracking-patterns]], [[aoc-day23-lan-party]], [[maximum-clique-problem]], [[graph-traversal]], [[bfs-graph-traversal]], [[dfs-pattern]]

---

## Core Concept

The **Bron-Kerbosch algorithm** is a recursive backtracking algorithm for finding all **maximal cliques** in an undirected graph. A **clique** is a subset of vertices where every two distinct vertices are adjacent (fully connected subgraph).

### **What is a Clique?**

```
Clique (size 4):          Not a Clique:
    A --- B                   A --- B
    |\   /|                   |     |
    | \ / |                   |     |
    | / \ |                   C     D
    |/   \|                   (A-D not connected)
    C --- D
    
All pairs connected:      Missing edge A-D
A-B, A-C, A-D, B-C, B-D, C-D
```

### **Maximum Clique Problem**

Finding the **largest clique** in a graph is an NP-complete problem (computationally hard). Bron-Kerbosch is one of the most efficient algorithms for this task, especially with optimizations.

---

## Algorithm Overview

### **Three Key Sets**

The algorithm maintains three sets during recursion:

| **Set** | **Name** | **Purpose** | **Initialization** |
|---------|----------|-------------|-------------------|
| **R** | Current clique | Nodes **definitely** in this clique | ∅ (empty) |
| **P** | Candidates | Nodes that **could** extend R | All nodes |
| **X** | Excluded | Nodes **already processed** | ∅ (empty) |

### **Basic Algorithm (No Pivoting)**

```rust
fn bron_kerbosch(R: Set, P: Set, X: Set) {
    // Base case: Found a maximal clique
    if P.is_empty() && X.is_empty() {
        report R as maximal clique
        return
    }
    
    // Recursive case: Try adding each candidate
    for each vertex v in P {
        // Recurse with:
        // - R ∪ {v}: Add v to current clique
        // - P ∩ N(v): Candidates that are neighbors of v
        // - X ∩ N(v): Excluded nodes that are neighbors of v
        bron_kerbosch(R ∪ {v}, P ∩ N(v), X ∩ N(v))
        
        // Move v from candidates to excluded
        P := P \ {v}
        X := X ∪ {v}
    }
}
```

**Initial call**: `bron_kerbosch(∅, V, ∅)` where V = all vertices

---

## Step-by-Step Walkthrough

### **Example Graph**

```
    A --- B
    |     |
    C --- D
```

Edges: A-B, A-C, B-D, C-D

### **Execution Trace**

```
Call 1: bron_kerbosch(R={}, P={A,B,C,D}, X={})
│
├─ Try A: bron_kerbosch(R={A}, P={B,C}, X={})
│  │  (P ∩ N(A) = {A,B,C,D} ∩ {B,C} = {B,C})
│  │
│  ├─ Try B: bron_kerbosch(R={A,B}, P={}, X={})
│  │  │  (P ∩ N(B) = {C} ∩ {A,D} = {})
│  │  └─ P=∅, X=∅ → Report {A,B} as maximal clique ✓
│  │
│  └─ Try C: bron_kerbosch(R={A,C}, P={}, X={B})
│     │  (P ∩ N(C) = {} ∩ {A,D} = {})
│     └─ P=∅, X≠∅ → Not maximal (already found {A,B,C,D})
│
├─ Try B: bron_kerbosch(R={B}, P={D}, X={A})
│  │  (P ∩ N(B) = {C,D} ∩ {A,D} = {D})
│  │
│  └─ Try D: bron_kerbosch(R={B,D}, P={}, X={})
│     └─ P=∅, X=∅ → Report {B,D} as maximal clique ✓
│
├─ Try C: bron_kerbosch(R={C}, P={D}, X={A,B})
│  │  (P ∩ N(C) = {D} ∩ {A,D} = {D})
│  │
│  └─ Try D: bron_kerbosch(R={C,D}, P={}, X={})
│     └─ P=∅, X=∅ → Report {C,D} as maximal clique ✓
│
└─ Try D: bron_kerbosch(R={D}, P={}, X={A,B,C})
   └─ P=∅, X≠∅ → Not maximal
```

**Maximal cliques found**: {A,B}, {B,D}, {C,D}

---

## Pivoting Optimization

### **Why Pivot?**

The basic algorithm explores many redundant branches. **Pivoting** dramatically reduces the search space.

### **Pivot Strategy**

Choose a **pivot node** `u` from `P ∪ X`. Only iterate over nodes in `P \ N(u)` (candidates NOT connected to pivot).

**Intuition**: If a node `v` is connected to pivot `u`, then any maximal clique containing `v` could also contain `u`. So we can safely skip `v` because we'll explore that clique when we process `u` itself.

### **Optimized Algorithm**

```rust
fn bron_kerbosch_pivot(R: Set, P: Set, X: Set) {
    if P.is_empty() && X.is_empty() {
        report R as maximal clique
        return
    }
    
    // Choose pivot: any node from P ∪ X
    let u = choose_pivot(P ∪ X)
    
    // Only iterate over P \ N(u)
    for each vertex v in P \ N(u) {
        bron_kerbosch_pivot(R ∪ {v}, P ∩ N(v), X ∩ N(v))
        P := P \ {v}
        X := X ∪ {v}
    }
}
```

### **Pivot Selection Strategies**

| **Strategy** | **Description** | **Performance** |
|--------------|-----------------|-----------------|
| **First node** | Just pick any node | Simple, decent |
| **Max degree** | Node with most neighbors in P ∪ X | Better pruning |
| **Random** | Random selection | Similar to first |

**AoC Day 23 implementation uses**: First node from `P.union(&X)` (simple and effective)

---

## Complexity Analysis

### **Time Complexity**

- **Worst case**: O(3^(n/3)) where n = number of vertices
  - Exponential, but this is optimal for maximum clique (NP-complete)
- **With pivoting**: Significantly better in practice (reduces branching factor)

### **Space Complexity**

- **Recursion depth**: O(n) (stack frames)
- **Set storage**: O(n) per recursive call
- **Total**: O(n²) in practice

### **Graph Structure Impact**

| **Graph Type** | **Performance** |
|----------------|-----------------|
| **Dense graphs** | Slower (many cliques) |
| **Sparse graphs** | Faster (few connections to explore) |
| **Real-world networks** | Usually very fast (graphs are often sparse) |

---

## Rust Implementation Details

### **Set Operations**

```rust
use std::collections::HashSet;

// Union: nodes in P OR X
let pivot_candidates = p.union(&x);

// Intersection: nodes in BOTH P AND neighbors
let new_p: HashSet<String> = p.intersection(&neighbors).cloned().collect();

// Difference: nodes in P but NOT in pivot_neighbors
let candidates: Vec<String> = p.difference(&pivot_neighbors).cloned().collect();
```

### **Key Rust Patterns**

```rust
// Clone sets for recursion (ownership transfer)
let mut new_r = r.clone();
new_r.insert(v.clone());

// Mutable iteration pattern
for v in candidates {
    // ... recursive call ...
    
    // Update P and X for next iteration
    p.remove(&v);
    x.insert(v);
}
```

---

## AoC 2024 Day 23 Application

### **Problem Context**

Finding the **maximum clique** in a computer network graph:
- **Nodes**: Computer names (2-letter IDs)
- **Edges**: Network connections (undirected)
- **Goal**: Find largest group where all computers are interconnected

### **Solution Pattern**

```rust
fn find_largest_clique(graph: &Graph) -> String {
    let nodes: Vec<String> = graph.keys().cloned().collect();
    let mut largest_clique: Vec<String> = Vec::new();
    
    let r = HashSet::new();
    let p: HashSet<String> = nodes.into_iter().collect();
    let x = HashSet::new();
    
    bron_kerbosch(graph, r, p, x, &mut largest_clique);
    
    // Return sorted, comma-separated list (password format)
    largest_clique.sort();
    largest_clique.join(",")
}
```

### **Performance on AoC Input**

- **Nodes**: 520 computers
- **Edges**: 3380 connections
- **Maximum clique size**: 13 computers
- **Runtime**: < 100ms (with pivoting)

---

## Common Variations

### **1. All Maximal Cliques**

Instead of tracking just the largest, collect all maximal cliques:

```rust
fn bron_kerbosch_all(r: Set, p: Set, x: Set, cliques: &mut Vec<Set>) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());  // Collect all maximal cliques
        return;
    }
    // ... rest of algorithm
}
```

### **2. Degeneracy Ordering**

Order vertices by **degeneracy** (smallest last vertex ordering) before starting. This can improve performance significantly.

### **3. Parallel Bron-Kerbosch**

Top-level candidates can be processed in parallel:

```rust
use rayon::prelude::*;

let results: Vec<_> = candidates.par_iter()
    .map(|v| {
        // Run Bron-Kerbosch independently for each top-level candidate
    })
    .collect();
```

---

## Comparison with Other Algorithms

### **Maximum Clique Approaches**

| **Algorithm** | **Approach** | **When to Use** |
|---------------|--------------|-----------------|
| **Bron-Kerbosch** | Backtracking with pruning | General case, all sizes |
| **Branch and Bound** | Explicit upper bound pruning | Dense graphs |
| **Greedy** | Heuristic approximation | When approximate solution OK |
| **Dynamic Programming** | Special graph structures | Tree-like graphs |

### **Why Bron-Kerbosch?**

✅ **Advantages**:
- Finds exact solution (not approximate)
- Efficient with pivoting
- Easy to implement
- Handles sparse graphs well

❌ **Disadvantages**:
- Still exponential worst-case
- Not suitable for very large dense graphs
- No early termination if you only need "a large" clique (not maximum)

---

## Mental Model

### **The Invitation Strategy**

Think of finding a clique like planning a party:

1. **R**: People already invited (definitely coming)
2. **P**: People you could still invite (know everyone already invited)
3. **X**: People you already considered (to avoid duplicate guest lists)

**Pivot optimization**: "If Bob is friends with everyone Alice is friends with, we don't need to separately check Alice's friends—Bob's party will cover that."

### **Pruning Insight**

```
At each step:
- Add a candidate to R
- Filter P to only mutual friends (intersection with neighbors)
- This naturally prunes impossible branches:
  - If new P is empty → Can't extend further
  - If new X has elements → Already explored this clique
```

---

## Common Pitfalls

### **1. Forgetting to Clone Sets**

```rust
// ❌ Wrong - mutates original sets
let new_r = r;
new_r.insert(v);

// ✅ Correct - clones before mutation
let mut new_r = r.clone();
new_r.insert(v);
```

### **2. Not Updating P and X**

```rust
// After recursion, MUST move v from P to X
bron_kerbosch(...);

p.remove(&v);  // ← Don't forget this!
x.insert(v);   // ← Or this!
```

### **3. Incorrect Intersection**

```rust
// ❌ Wrong - not limiting to neighbors
let new_p = p.clone();

// ✅ Correct - only neighbors of v
let new_p: HashSet<_> = p.intersection(&neighbors).cloned().collect();
```

---

## Extensions and Applications

### **Real-World Uses**

1. **Social Networks**: Find tightly-knit friend groups
2. **Bioinformatics**: Identify protein interaction clusters
3. **Telecommunications**: Network reliability analysis
4. **Code Analysis**: Find tightly coupled code modules
5. **Chemistry**: Molecular structure analysis

### **Related Problems**

- **Independent Set**: Complement of clique (nodes with NO edges)
- **Vertex Cover**: Minimum nodes to cover all edges
- **Graph Coloring**: Related to clique number (chromatic number ≥ clique number)

---

## Key Takeaways

1. ✅ **Bron-Kerbosch finds maximum cliques** in undirected graphs
2. ✅ **Three sets (R, P, X)** track current clique, candidates, and processed nodes
3. ✅ **Pivoting optimization** dramatically reduces search space
4. ✅ **NP-complete problem** - exponential worst case, but practical with optimizations
5. ✅ **Recursive backtracking** explores all possible cliques systematically
6. ✅ **Set intersections** naturally prune impossible branches
7. ✅ **Base case**: P=∅ and X=∅ means R is maximal clique
8. ✅ **Rust ownership**: Clone sets when recursing to avoid mutation issues

---

## Further Reading

- **Original paper**: Bron, C.; Kerbosch, J. (1973). "Algorithm 457: finding all cliques of an undirected graph"
- **Optimizations**: Tomita, E.; Tanaka, A.; Takahashi, H. (2006). "The worst-case time complexity for generating all maximal cliques and computational experiments"
- **Survey**: Wu, Q.; Hao, J. K. (2015). "A review on algorithms for maximum clique problems"

---

## Code Example (Complete)

```rust
use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, HashSet<String>>;

fn find_largest_clique(graph: &Graph) -> Vec<String> {
    let nodes: Vec<String> = graph.keys().cloned().collect();
    let mut largest_clique: Vec<String> = Vec::new();
    
    let r = HashSet::new();
    let p: HashSet<String> = nodes.into_iter().collect();
    let x = HashSet::new();
    
    bron_kerbosch(graph, r, p, x, &mut largest_clique);
    
    largest_clique.sort();
    largest_clique
}

fn bron_kerbosch(
    graph: &Graph,
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    max_clique: &mut Vec<String>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            *max_clique = r.iter().cloned().collect();
        }
        return;
    }
    
    // Pivoting: choose node from P ∪ X
    let pivot = p.union(&x).next().cloned();
    
    let candidates: Vec<String> = if let Some(pivot_node) = pivot {
        let pivot_neighbors = graph.get(&pivot_node).cloned().unwrap_or_default();
        p.difference(&pivot_neighbors).cloned().collect()
    } else {
        p.iter().cloned().collect()
    };
    
    for v in candidates {
        let neighbors = graph.get(&v).cloned().unwrap_or_default();
        
        let mut new_r = r.clone();
        new_r.insert(v.clone());
        
        let new_p: HashSet<String> = p.intersection(&neighbors).cloned().collect();
        let new_x: HashSet<String> = x.intersection(&neighbors).cloned().collect();
        
        bron_kerbosch(graph, new_r, new_p, new_x, max_clique);
        
        p.remove(&v);
        x.insert(v);
    }
}
```

---

*Created*: 2025-12-22 (AoC 2024 Day 23 - LAN Party maximum clique)  
*Source*: [[advent_of_code/aoc2024/day23]], Bron & Kerbosch (1973)  
*Applications*: Network analysis, social graphs, protein interactions, AoC graph problems  
*Complexity*: O(3^(n/3)) worst case, much better with pivoting in practice
