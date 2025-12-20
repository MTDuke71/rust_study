# Cycle Detection - Finding Circular Dependencies in Graphs

*Essential graph algorithm techniques for detecting cycles in both directed and undirected graphs*

---

## 🎯 **Core Concept**

**Cycle detection** is the process of identifying circular paths in a graph where you can start at a node and follow edges to return to that same node. This problem is fundamental in computer science with critical applications in dependency management, deadlock detection, and graph validation.

**Key distinction**: The approach differs significantly between:
- **Directed graphs**: Detect cycles using DFS with recursion stack tracking (back edges)
- **Undirected graphs**: Detect cycles using DFS with parent tracking or Union-Find

---

## 🧠 **Mental Models**

### **The Recursion Stack Model (Directed Graphs)**

Think of the recursion stack as **"ancestors you're currently visiting"**:
- **White** nodes: Never visited
- **Gray** nodes: Currently exploring (in recursion stack)
- **Black** nodes: Completely finished

Finding a **gray node** while exploring means you've found a path back to an ancestor → **cycle detected!**

```
Graph: A → B → C → A

Visit A (gray):
  Visit B (gray):
    Visit C (gray):
      Visit A → Found gray node! CYCLE!
```

### **The Parent Tracking Model (Undirected Graphs)**

In undirected graphs, every edge appears twice (u→v and v→u). To avoid false positives:
- Track the **immediate parent** of each node
- If you encounter a visited node that's **not your parent** → cycle detected!

```
Graph: 0-1-2-0

Visit 0, parent=None:
  Visit 1, parent=0:
    Visit 2, parent=1:
      Visit 0 → Found visited node (not parent 1)! CYCLE!
```

### **The Union-Find Model (Incremental Edge Addition)**

When building a graph edge by edge:
- If two nodes are **already connected** and you add an edge between them → **cycle created!**
- Union-Find efficiently tracks connectivity in near O(1) time

---

## 🔍 **Algorithm Approaches**

### **1. DFS-Based Cycle Detection (Directed Graphs)**

**Best for**: Static directed graphs, finding actual cycle paths

**Time Complexity**: O(V + E)  
**Space Complexity**: O(V)

```rust
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White, // Unvisited
    Gray,  // Currently visiting (in recursion stack)
    Black, // Completely visited
}

fn has_cycle<T>(graph: &HashMap<T, Vec<T>>) -> bool
where
    T: Clone + Eq + std::hash::Hash,
{
    let mut color: HashMap<T, Color> = HashMap::new();
    
    // Initialize all nodes as White
    for node in graph.keys() {
        color.insert(node.clone(), Color::White);
    }
    
    // Check each component
    for node in graph.keys() {
        if color.get(node) == Some(&Color::White) {
            if has_cycle_dfs(graph, node, &mut color) {
                return true;
            }
        }
    }
    
    false
}

fn has_cycle_dfs<T>(
    graph: &HashMap<T, Vec<T>>,
    node: &T,
    color: &mut HashMap<T, Color>,
) -> bool
where
    T: Clone + Eq + std::hash::Hash,
{
    color.insert(node.clone(), Color::Gray);
    
    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            match color.get(neighbor).unwrap_or(&Color::White) {
                Color::Gray => return true, // Back edge → cycle!
                Color::White => {
                    if has_cycle_dfs(graph, neighbor, color) {
                        return true;
                    }
                }
                Color::Black => continue, // Already processed
            }
        }
    }
    
    color.insert(node.clone(), Color::Black);
    false
}
```

**Characteristics**:
- ✅ Finds cycles efficiently in directed graphs
- ✅ Can be extended to return actual cycle path
- ✅ Handles disconnected graphs
- ⚠️ Requires careful recursion stack tracking

---

### **2. DFS-Based Cycle Detection (Undirected Graphs)**

**Best for**: Static undirected graphs, finding cycle paths

**Time Complexity**: O(V + E)  
**Space Complexity**: O(V)

```rust
use std::collections::{HashMap, HashSet};

fn has_cycle_undirected<T>(graph: &HashMap<T, Vec<T>>) -> bool
where
    T: Clone + Eq + std::hash::Hash,
{
    let mut visited = HashSet::new();
    
    for start_node in graph.keys() {
        if !visited.contains(start_node) {
            if has_cycle_dfs_undirected(graph, start_node, None, &mut visited) {
                return true;
            }
        }
    }
    
    false
}

fn has_cycle_dfs_undirected<T>(
    graph: &HashMap<T, Vec<T>>,
    node: &T,
    parent: Option<&T>,
    visited: &mut HashSet<T>,
) -> bool
where
    T: Clone + Eq + std::hash::Hash,
{
    visited.insert(node.clone());
    
    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if Some(neighbor) == parent {
                continue; // Skip immediate parent edge
            }
            
            if visited.contains(neighbor) {
                return true; // Found visited non-parent → cycle!
            }
            
            if has_cycle_dfs_undirected(graph, neighbor, Some(node), visited) {
                return true;
            }
        }
    }
    
    false
}
```

**Characteristics**:
- ✅ Correctly handles undirected graph semantics
- ✅ Avoids false positives from bidirectional edges
- ✅ Can track cycle path with parent HashMap
- ⚠️ Must pass parent to avoid immediate backtrack

---

### **3. Union-Find Cycle Detection (Undirected Graphs)**

**Best for**: Incremental graph construction, online cycle detection

**Time Complexity**: O(E α(V)) where α is inverse Ackermann (near O(1))  
**Space Complexity**: O(V)

```rust
// Using Mission 10's UnionFind
use mission10::UnionFind;

fn detect_cycle_incremental(edges: &[(usize, usize)], num_vertices: usize) -> Option<(usize, usize)> {
    let mut uf = UnionFind::new(num_vertices);
    
    for &(u, v) in edges {
        // If u and v already connected, this edge creates cycle
        if uf.connected(u, v) {
            return Some((u, v)); // Cycle detected!
        }
        uf.union(u, v);
    }
    
    None
}
```

**Characteristics**:
- ✅ **Incremental**: Detect cycles as edges are added
- ✅ **Extremely fast**: Near constant time per edge
- ✅ **Online algorithm**: Perfect for dynamic graphs
- ❌ **Undirected only**: Cannot detect cycles in directed graphs
- ❌ **No cycle path**: Only detects existence, not actual cycle

---

### **4. Kahn's Algorithm (Directed Graphs - Topological Sort)**

**Best for**: Dependency resolution, explicit cycle detection

**Time Complexity**: O(V + E)  
**Space Complexity**: O(V)

```rust
use std::collections::{HashMap, VecDeque};

fn has_cycle_kahns<T>(graph: &HashMap<T, Vec<T>>) -> bool
where
    T: Clone + Eq + std::hash::Hash,
{
    let mut in_degree: HashMap<T, usize> = HashMap::new();
    let mut queue = VecDeque::new();
    let mut processed = 0;
    
    // Initialize in-degrees
    for node in graph.keys() {
        in_degree.insert(node.clone(), 0);
    }
    
    // Calculate in-degrees
    for neighbors in graph.values() {
        for neighbor in neighbors {
            *in_degree.entry(neighbor.clone()).or_insert(0) += 1;
        }
    }
    
    // Queue all nodes with in-degree 0
    for (node, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node.clone());
        }
    }
    
    // Process nodes
    while let Some(node) = queue.pop_front() {
        processed += 1;
        
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
    }
    
    // If processed < total nodes, cycle exists
    processed < graph.len()
}
```

**Characteristics**:
- ✅ **Explicit detection**: Result length < vertex count → cycle exists
- ✅ **No recursion**: Iterative, no stack overflow risk
- ✅ **Byproduct of topological sort**: Get valid ordering for free if acyclic
- ⚠️ **Directed only**: Designed for DAG validation

---

## ⚖️ **Algorithm Comparison**

| **Approach** | **Graph Type** | **Time** | **Space** | **Returns Path?** | **Incremental?** | **Best Use Case** |
|--------------|----------------|----------|-----------|-------------------|------------------|-------------------|
| **DFS (Directed)** | Directed | O(V+E) | O(V) | ✅ Yes | ❌ No | Static directed graphs, finding cycles |
| **DFS (Undirected)** | Undirected | O(V+E) | O(V) | ✅ Yes | ❌ No | Static undirected graphs, cycle paths |
| **Union-Find** | Undirected | O(E α(V)) | O(V) | ❌ No | ✅ Yes | Dynamic graphs, Kruskal's MST |
| **Kahn's Algorithm** | Directed | O(V+E) | O(V) | ❌ No | ❌ No | Dependency validation, topological sort |

---

## 💡 **Key Takeaways**

1. **Choose algorithm based on graph type and requirements**:
   - Directed → DFS with color tracking or Kahn's
   - Undirected → DFS with parent tracking or Union-Find
   - Dynamic/incremental → Union-Find

2. **Recursion stack vs visited set**: In directed graphs, tracking **currently visiting** (gray) vs **completely visited** (black) is critical for correct cycle detection

3. **Parent tracking prevents false positives**: In undirected graphs, must avoid counting immediate parent as a cycle

4. **Union-Find for incremental construction**: When adding edges one by one, Union-Find detects cycles in near O(1) per edge

5. **Topological sort implies acyclicity**: Kahn's algorithm provides cycle detection as a byproduct of topological ordering

---

## 🔗 **Integration Points**

### **Builds On**
- [[DFS Patterns]] - Depth-first traversal fundamentals
- [[Graph Theory]] - Basic graph concepts and terminology
- [[union-find-algorithm]] - Disjoint set data structure

### **Enables**
- [[topological-sort]] - DAG validation prerequisite
- [[kahns-topological-sort]] - Explicit cycle detection in dependency graphs
- [[union-find-patterns]] - Kruskal's MST and dynamic connectivity

### **Related Concepts**
- [[BFS Patterns]] - Alternative traversal for undirected cycle detection
- [[graph-algorithms]] - Broader graph algorithm ecosystem
- [[mission-10]] - Union-Find implementation
- [[missions/mission-8]] - DFS/BFS algorithm implementations

---

## 🚀 **Mission Applications**

### **Mission 7**: Graph Data Structure Foundation
- DFS-based cycle detection in directed graphs
- Recursion stack tracking implementation
- See: [Mission7/src/lib.rs](d:\repos\rust_study\missions\Mission7\src\lib.rs)

### **Mission 8**: Advanced Graph Algorithms
- `has_cycle<G: Graph>()` - Generic cycle detection
- `find_cycle<G: Graph>()` - Returns actual cycle path
- Color-based DFS implementation (White/Gray/Black)
- See: [[missions/mission-8]]

### **Mission 10**: Union-Find Applications
- Incremental cycle detection during edge addition
- Kruskal's MST with cycle avoidance
- Performance comparison: Union-Find vs DFS
- Complete example: [cycle_detection.rs](d:\repos\rust_study\missions\Mission10\examples\cycle_detection.rs)

---

## 📚 **Real-World Applications**

### **1. Build Systems & Dependency Management**
```rust
// Detect circular dependencies in build systems
fn validate_build_order(dependencies: &HashMap<String, Vec<String>>) -> Result<(), String> {
    if has_cycle(dependencies) {
        Err("Circular dependency detected!".to_string())
    } else {
        Ok(())
    }
}
```

### **2. Kruskal's Minimum Spanning Tree**
```rust
// Use Union-Find to avoid cycles when building MST
fn kruskal_mst(edges: &[(usize, usize, f64)], num_vertices: usize) -> Vec<(usize, usize)> {
    let mut uf = UnionFind::new(num_vertices);
    let mut mst = Vec::new();
    
    for &(u, v, weight) in edges {
        // Only add edge if it doesn't create cycle
        if !uf.connected(u, v) {
            uf.union(u, v);
            mst.push((u, v));
        }
    }
    
    mst
}
```

### **3. Deadlock Detection in Concurrent Systems**
```rust
// Model resource allocation as directed graph
// Edge A → B means "A waits for B"
// Cycle → deadlock exists
fn detect_deadlock(wait_graph: &HashMap<ThreadId, Vec<ThreadId>>) -> bool {
    has_cycle(wait_graph)
}
```

### **4. Version Control & Git DAG Validation**
```rust
// Ensure commit history remains acyclic
fn validate_git_dag(commits: &HashMap<CommitId, Vec<CommitId>>) -> Result<(), GitError> {
    if has_cycle(commits) {
        Err(GitError::InvalidDAG("Commit cycle detected".into()))
    } else {
        Ok(())
    }
}
```

---

## 🧪 **Testing Patterns**

### **Test Case Categories**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_cycle() {
        // Triangle: 0 → 1 → 2 → 0
        let mut graph = HashMap::new();
        graph.insert(0, vec![1]);
        graph.insert(1, vec![2]);
        graph.insert(2, vec![0]);
        
        assert!(has_cycle(&graph));
    }
    
    #[test]
    fn test_dag_no_cycle() {
        // Tree: 0 → 1, 0 → 2, 1 → 3
        let mut graph = HashMap::new();
        graph.insert(0, vec![1, 2]);
        graph.insert(1, vec![3]);
        graph.insert(2, vec![]);
        graph.insert(3, vec![]);
        
        assert!(!has_cycle(&graph));
    }
    
    #[test]
    fn test_self_loop() {
        // Self-loop: 0 → 0
        let mut graph = HashMap::new();
        graph.insert(0, vec![0]);
        
        assert!(has_cycle(&graph));
    }
    
    #[test]
    fn test_disconnected_components() {
        // Two separate components, one has cycle
        let mut graph = HashMap::new();
        graph.insert(0, vec![1]);
        graph.insert(1, vec![2]);
        graph.insert(2, vec![0]); // Cycle here
        graph.insert(3, vec![4]);
        graph.insert(4, vec![]); // No cycle
        
        assert!(has_cycle(&graph)); // Detects cycle in first component
    }
}
```

---

## 🎯 **Common Pitfalls**

### **1. Forgetting to Track Recursion Stack (Directed Graphs)**
```rust
// ❌ WRONG: Only using visited set
fn has_cycle_wrong(graph: &HashMap<u32, Vec<u32>>, node: u32, visited: &mut HashSet<u32>) -> bool {
    visited.insert(node);
    for &neighbor in graph.get(&node).unwrap_or(&vec![]) {
        if !visited.contains(&neighbor) {
            if has_cycle_wrong(graph, neighbor, visited) {
                return true;
            }
        }
    }
    false // Missing back edge detection!
}

// ✅ CORRECT: Track gray (visiting) vs black (visited)
// See DFS-Based Cycle Detection section above
```

### **2. Not Skipping Parent in Undirected Graphs**
```rust
// ❌ WRONG: Every edge appears as cycle
fn has_cycle_wrong_undirected(graph: &HashMap<u32, Vec<u32>>, node: u32, visited: &mut HashSet<u32>) -> bool {
    visited.insert(node);
    for &neighbor in graph.get(&node).unwrap_or(&vec![]) {
        if visited.contains(&neighbor) {
            return true; // Wrong! This is just the parent edge
        }
        if has_cycle_wrong_undirected(graph, neighbor, visited) {
            return true;
        }
    }
    false
}

// ✅ CORRECT: Track and skip immediate parent
// See DFS-Based Cycle Detection (Undirected) section above
```

### **3. Using Union-Find for Directed Graphs**
```rust
// ❌ WRONG: Union-Find doesn't understand edge direction
fn detect_cycle_directed_wrong(edges: &[(usize, usize)]) -> bool {
    let mut uf = UnionFind::new(100);
    for &(u, v) in edges {
        if uf.connected(u, v) {
            return true; // Wrong for directed graphs!
        }
        uf.union(u, v);
    }
    false
}

// ✅ CORRECT: Use DFS or Kahn's for directed graphs
```

---

## 📖 **Learning Progression**

### **Introduction**: [[daily-study/Day24]] & [[Mission7_tut Overview]]
- Basic DFS traversal patterns
- Visited set tracking
- Introduction to graph cycles

### **Application**: [[Mission8_tut Overview]]
- DFS-based cycle detection implementation
- Color tracking (White/Gray/Black)
- Finding actual cycle paths

### **Advanced**: [[Mission10]]
- Union-Find for incremental cycle detection
- Performance comparison: Union-Find vs DFS
- Real-world applications (Kruskal's MST)

---

*Tags: #algorithm #graph-algorithms #dfs #union-find #mission-7 #mission-8 #mission-10 #intermediate*

*Links: [[zettel-index]] | [[Graph Theory MOC]] | [[Algorithms MOC]] | [[AoC Patterns MOC]] | [[DFS Patterns]] | [[union-find-algorithm]] | [[union-find-patterns]] | [[topological-sort]] | [[kahns-topological-sort]] | [[missions/mission-8]] | [[mission-10]] | [[Mission7_tut Overview]] | [[Mission8_tut Overview]] | [[graph-algorithms]] | [[BFS Patterns]]*
