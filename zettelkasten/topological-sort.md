# 📊 Topological Sort

*Linear ordering of vertices in Directed Acyclic Graphs (DAGs)*

---

## 📋 Core Concept

**Definition**: Topological sorting is the process of creating a linear ordering of vertices in a **Directed Acyclic Graph (DAG)** such that for every directed edge `u → v`, vertex `u` comes before vertex `v` in the ordering.

**Critical Requirement**: The graph **must be a DAG** - topological sorting is only possible when there are no cycles.

**Key Applications**:
- 🎓 **Course prerequisites** - Determine valid course sequences
- 📦 **Build systems** - Resolve compilation dependencies
- 📋 **Task scheduling** - Order tasks respecting dependencies
- 🔄 **Dependency resolution** - Package managers, module loading

---

## 🧠 Fundamental Intuition

### **Mental Model: Getting Dressed**

```
Dependencies:
- Socks must come before shoes
- Underwear before pants
- Shirt before jacket

Valid topological orders:
1. Underwear → Shirt → Pants → Socks → Jacket → Shoes ✅
2. Socks → Underwear → Pants → Shirt → Jacket → Shoes ✅
3. Shirt → Underwear → Jacket → Socks → Pants → Shoes ✅

Invalid order:
- Shoes → Socks (violates sock → shoe dependency) ❌
```

**Key Insight**: Multiple valid orderings can exist, but all must respect the dependency edges.

---

## 🔧 Two Major Approaches

### **1. DFS-Based Topological Sort**

**Strategy**: Use depth-first search with post-order traversal

**Algorithm**:
1. Perform DFS from each unvisited vertex
2. Push vertices to stack **after** visiting all descendants
3. Reverse the stack to get topological order

**Characteristics**:
- ✅ Naturally recursive and elegant
- ✅ Detects cycles during traversal
- ✅ Works well with implicit graphs
- ⚠️ Stack-based - can overflow on deep graphs
- ⏱️ **Time**: `O(V + E)` | **Space**: `O(V)` for recursion stack

```rust
fn topological_sort_dfs(graph: &HashMap<NodeId, Vec<NodeId>>) -> Vec<NodeId> {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    
    fn dfs(node: NodeId, graph: &HashMap<NodeId, Vec<NodeId>>,
           visited: &mut HashSet<NodeId>, stack: &mut Vec<NodeId>) {
        visited.insert(node);
        
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    dfs(neighbor, graph, visited, stack);
                }
            }
        }
        
        stack.push(node); // Post-order: add after descendants
    }
    
    for &node in graph.keys() {
        if !visited.contains(&node) {
            dfs(node, graph, &mut visited, &mut stack);
        }
    }
    
    stack.reverse(); // Reverse for topological order
    stack
}
```

**See**: [[DFS Patterns]] for more recursive graph traversal patterns

---

### **2. Kahn's Algorithm (BFS-Based)**

**Strategy**: Iteratively process vertices with zero in-degree (no dependencies)

**Algorithm**:
1. Calculate in-degree for all vertices
2. Add all vertices with in-degree 0 to queue
3. Process queue: remove vertex, reduce neighbors' in-degrees
4. If result contains all vertices → valid topological order
5. Otherwise → cycle detected

**Characteristics**:
- ✅ Explicit cycle detection (result length < vertex count)
- ✅ Iterative - no stack overflow risk
- ✅ Natural for dependency resolution systems
- ✅ Easy to parallelize (process multiple zero-degree vertices)
- ⏱️ **Time**: `O(V + E)` | **Space**: `O(V)` for in-degree tracking

```rust
fn topological_sort_kahns(
    adj_list: &HashMap<NodeId, Vec<NodeId>>
) -> Result<Vec<NodeId>, CycleError> {
    let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    
    // Initialize in-degrees
    for &node in adj_list.keys() {
        in_degree.insert(node, 0);
    }
    
    // Calculate in-degrees
    for neighbors in adj_list.values() {
        for &neighbor in neighbors {
            *in_degree.entry(neighbor).or_insert(0) += 1;
        }
    }
    
    // Find vertices with no incoming edges
    for (&node, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node);
        }
    }
    
    // Process vertices in topological order
    while let Some(node) = queue.pop_front() {
        result.push(node);
        
        if let Some(neighbors) = adj_list.get(&node) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }
    
    // Cycle detection
    if result.len() != adj_list.len() {
        Err(CycleError::new("Graph contains cycles"))
    } else {
        Ok(result)
    }
}
```

**See**: [[kahns-topological-sort]] for detailed implementation and examples

---

## ⚖️ Algorithm Comparison

| **Aspect** | **DFS-Based** | **Kahn's Algorithm** |
|------------|---------------|----------------------|
| **Traversal** | Depth-first, recursive | Breadth-first, iterative |
| **In-Degree Tracking** | Not required | Explicitly maintained |
| **Cycle Detection** | Implicit via recursion stack | Explicit via result length check |
| **Implementation** | Recursive, concise | Iterative, explicit |
| **Stack Overflow Risk** | Yes for deep graphs | No, uses queue |
| **Parallelization** | Difficult | Easier, process multiple zero-degree nodes |
| **Best For** | Compact code, implicit graphs | Production systems, explicit graphs |

**When to Choose**:
- **DFS**: Competitive programming, small/medium graphs, elegant recursion preferred
- **Kahn's**: Production code, large graphs, explicit cycle detection needed

---

## 🎯 Real-World Applications

### **1. Build Systems (Cargo, Maven, Make)**

```rust
// Crate dependency resolution
struct CrateGraph {
    deps: HashMap<String, Vec<String>>, // crate → dependencies
}

impl CrateGraph {
    fn build_order(&self) -> Result<Vec<String>, CircularDependencyError> {
        // Use Kahn's algorithm for explicit cycle detection
        topological_sort_kahns(&self.deps)
    }
}

// Example:
// serde → serde_derive
// tokio → mio, bytes
// hyper → tokio, http
//
// Build order: [serde_derive, serde, mio, bytes, tokio, http, hyper]
```

---

### **2. Course Prerequisites**

```rust
// University course scheduler
let courses = hashmap! {
    "MATH101" => vec![],
    "CS101" => vec![],
    "STATS201" => vec!["MATH101"],
    "CS202" => vec!["CS101", "MATH101"],
    "ML301" => vec!["STATS201", "CS202"],
};

let schedule = topological_sort_kahns(&courses)?;
// Result: [MATH101, CS101, STATS201, CS202, ML301] (one valid order)
```

---

### **3. Task Scheduling**

```rust
// Project task dependencies
struct Task {
    name: String,
    dependencies: Vec<String>,
}

fn project_plan(tasks: &[Task]) -> Vec<String> {
    let graph = build_task_graph(tasks);
    topological_sort_dfs(&graph)
}

// Example:
// Task A: Design → Implementation → Testing → Deployment
// Critical path = longest path through topologically sorted tasks
```

---

### **4. AoC 2024 Day 5: Print Queue**

```rust
// Page ordering rules: X|Y means X must come before Y
// Fix incorrectly ordered page sequences

fn fix_page_order(rules: &HashMap<i32, Vec<i32>>, pages: &[i32]) -> Vec<i32> {
    // Build subgraph with only relevant pages
    let subgraph = build_subgraph(rules, pages);
    
    // Apply Kahn's algorithm for dependency resolution
    topological_sort_kahns(&subgraph).unwrap_or_else(|_| {
        // Fallback for cycles (shouldn't happen in valid AoC input)
        pages.to_vec()
    })
}
```

**See**: [[aoc2024-day5-mission-integration]] for full implementation

---

## 🔍 Common Patterns & Techniques

### **Pattern 1: Subgraph Extraction**

When working with large dependency graphs, extract only relevant vertices:

```rust
fn extract_subgraph(
    full_graph: &HashMap<NodeId, Vec<NodeId>>,
    relevant_nodes: &HashSet<NodeId>
) -> HashMap<NodeId, Vec<NodeId>> {
    let mut subgraph = HashMap::new();
    
    for &node in relevant_nodes {
        if let Some(neighbors) = full_graph.get(&node) {
            let filtered: Vec<NodeId> = neighbors.iter()
                .filter(|n| relevant_nodes.contains(n))
                .copied()
                .collect();
            subgraph.insert(node, filtered);
        }
    }
    
    subgraph
}
```

**Use Case**: AoC Day 5 processes only the pages in each update, not all pages

---

### **Pattern 2: Longest Path in DAG**

Combine topological sort with dynamic programming:

```rust
fn longest_path(graph: &HashMap<NodeId, Vec<NodeId>>, 
                start: NodeId, end: NodeId) -> Option<Vec<NodeId>> {
    // Get topological order
    let topo_order = topological_sort_dfs(graph);
    
    // Dynamic programming on topologically sorted vertices
    let mut distances: HashMap<NodeId, i32> = HashMap::new();
    let mut parent: HashMap<NodeId, NodeId> = HashMap::new();
    
    for &node in graph.keys() {
        distances.insert(node, if node == start { 0 } else { i32::MIN });
    }
    
    for &node in &topo_order {
        if distances[&node] == i32::MIN { continue; }
        
        for &neighbor in graph.get(&node)? {
            let new_dist = distances[&node] + 1;
            if new_dist > distances[&neighbor] {
                distances.insert(neighbor, new_dist);
                parent.insert(neighbor, node);
            }
        }
    }
    
    // Reconstruct path from parent pointers
    reconstruct_path(&parent, start, end)
}
```

**Why Topological Sort?**: Ensures we process each vertex only after all its dependencies are processed.

---

### **Pattern 3: Cycle Detection as Precondition**

```rust
fn safe_topological_sort(graph: &HashMap<NodeId, Vec<NodeId>>) 
    -> Result<Vec<NodeId>, GraphError> {
    // Check for cycles first
    if has_cycle(graph) {
        return Err(GraphError::CycleDetected);
    }
    
    // Proceed with sorting (DFS won't detect cycles now)
    Ok(topological_sort_dfs(graph))
}
```

**See**: [[mission-8]] for `has_cycle()` implementation

---

## 🧪 Testing Strategies

### **Core Test Cases**

```rust
#[test]
fn test_simple_chain() {
    // A → B → C
    let graph = hashmap! { 0 => vec![1], 1 => vec![2], 2 => vec![] };
    let result = topological_sort(&graph);
    assert_eq!(result, vec![0, 1, 2]);
}

#[test]
fn test_multiple_valid_orders() {
    // A → C, B → C (A and B have no ordering constraint)
    let graph = hashmap! { 0 => vec![2], 1 => vec![2], 2 => vec![] };
    let result = topological_sort(&graph);
    
    // Both [0,1,2] and [1,0,2] are valid
    assert!(result[2] == 2); // C must be last
    assert!(result[..2].contains(&0) && result[..2].contains(&1));
}

#[test]
fn test_cycle_detection() {
    // A → B → C → A (cycle)
    let graph = hashmap! { 0 => vec![1], 1 => vec![2], 2 => vec![0] };
    
    // DFS: Will eventually encounter visited vertex in recursion stack
    // Kahn's: Result length < graph.len()
    assert!(topological_sort(&graph).is_err());
}

#[test]
fn test_disconnected_components() {
    // A → B (component 1), C → D (component 2)
    let graph = hashmap! {
        0 => vec![1], 1 => vec![],
        2 => vec![3], 3 => vec![]
    };
    let result = topological_sort(&graph);
    
    // Valid orders: [0,1,2,3], [0,2,1,3], [0,2,3,1], [2,3,0,1], etc.
    assert_eq!(result.len(), 4);
    assert!(result.iter().position(|&x| x == 0) < result.iter().position(|&x| x == 1));
    assert!(result.iter().position(|&x| x == 2) < result.iter().position(|&x| x == 3));
}
```

---

## ⚠️ Common Pitfalls

### **1. Forgetting to Check for Cycles**

```rust
// ❌ WRONG: Assumes graph is always a DAG
fn bad_topological_sort(graph: &HashMap<NodeId, Vec<NodeId>>) -> Vec<NodeId> {
    topological_sort_dfs(graph) // Infinite recursion if cycles exist!
}

// ✅ CORRECT: Explicit cycle detection
fn safe_topological_sort(graph: &HashMap<NodeId, Vec<NodeId>>) 
    -> Result<Vec<NodeId>, GraphError> {
    if has_cycle(graph) {
        return Err(GraphError::CycleDetected);
    }
    Ok(topological_sort_dfs(graph))
}
```

---

### **2. Not Initializing All Vertices in Kahn's Algorithm**

```rust
// ❌ WRONG: Misses vertices with no outgoing edges
for neighbors in adj_list.values() {
    for &neighbor in neighbors {
        *in_degree.entry(neighbor).or_insert(0) += 1;
    }
}

// ✅ CORRECT: Initialize all vertices first
for &node in adj_list.keys() {
    in_degree.insert(node, 0);
}
for neighbors in adj_list.values() {
    for &neighbor in neighbors {
        *in_degree.get_mut(&neighbor).unwrap() += 1;
    }
}
```

---

### **3. Forgetting to Reverse Stack in DFS Approach**

```rust
// ❌ WRONG: Reverse topological order (dependencies appear AFTER dependents)
fn incorrect_dfs_sort(graph: &HashMap<NodeId, Vec<NodeId>>) -> Vec<NodeId> {
    let mut stack = Vec::new();
    // ... DFS traversal ...
    stack // WRONG: Should reverse!
}

// ✅ CORRECT: Reverse for correct topological order
fn correct_dfs_sort(graph: &HashMap<NodeId, Vec<NodeId>>) -> Vec<NodeId> {
    let mut stack = Vec::new();
    // ... DFS traversal ...
    stack.reverse();
    stack
}
```

**Why?**: DFS pushes vertices in post-order (after descendants), which is reverse topological order.

---

## 🎓 Learning Integration

### **Conceptual Prerequisites**

- [[Graph Theory MOC]] - Basic graph terminology and DAG properties
- [[DFS Patterns]] - Depth-first search and recursion patterns
- [[mission-8]] - Graph cycle detection and traversal algorithms
- [[Queue Data Structure]] - FIFO processing for Kahn's algorithm

---

### **Related Algorithms**

- [[kahns-topological-sort]] - Detailed Kahn's algorithm implementation
- [[cycle-detection]] - Identifying cycles in directed graphs
- [[DFS Patterns]] - Recursive DFS-based topological sort
- [[bfs-patterns]] - Breadth-first traversal (Kahn's uses BFS mindset)

---

### **Real Implementations**

- [[aoc2024-day5-mission-integration]] - AoC Day 5 dependency resolution
- [[mission-7]] - Graph data structure foundations
- [[mission-8]] - Graph traversal algorithms (BFS, DFS, cycle detection)

---

### **Advanced Topics**

- **Strongly Connected Components** - Decomposition for cyclic graphs
- **Critical Path Method (CPM)** - Project scheduling with task durations
- **Parallel Topological Sort** - Processing multiple zero-degree vertices concurrently
- **Online Topological Sort** - Incremental updates as edges are added

---

## 📊 Complexity Analysis

| **Operation** | **DFS-Based** | **Kahn's Algorithm** |
|--------------|---------------|----------------------|
| **Time** | O(V + E) | O(V + E) |
| **Space** | O(V) recursion stack | O(V) in-degree map + queue |
| **Cycle Detection** | During traversal | After sorting (length check) |
| **Best Case** | O(V + E) | O(V + E) |
| **Worst Case** | O(V + E) | O(V + E) |

**Both algorithms have identical asymptotic complexity**, but practical considerations:
- **DFS**: Lower constant factors, more cache-friendly
- **Kahn's**: More explicit state, easier to debug

---

## 🔗 Mission Integration

### **Mission 7: Graph Data Structures**

```rust
use mission7::Graph;

let mut graph: Graph<&str> = Graph::new_directed();
let a = graph.add_node("Task A");
let b = graph.add_node("Task B");
let c = graph.add_node("Task C");

graph.add_edge(a, b); // A → B
graph.add_edge(b, c); // B → C

// Topological sort would return: [A, B, C]
```

**See**: [[mission-7]] for graph construction and representation

---

### **Mission 8: Graph Algorithms**

```rust
use mission8::{has_cycle, bfs, dfs};

// Check for cycles before sorting
if has_cycle(&adj_list) {
    return Err("Cannot topologically sort cyclic graph");
}

// DFS-based sorting uses Mission 8's DFS infrastructure
let topo_order = dfs_topological_sort(&adj_list);
```

**See**: [[mission-8]] for cycle detection and graph traversal

---

## 📚 Further Reading

### **Academic Sources**

- **Kahn, A. B. (1962)** - "Topological sorting of large networks"
- **Cormen, et al.** - "Introduction to Algorithms" - Section 22.4
- **Knuth, D. E.** - "The Art of Computer Programming, Vol 1" - Topological sorting

---

### **Implementation References**

- **Rust Standard Library** - No built-in topological sort (use crates or implement)
- **petgraph crate** - `petgraph::algo::toposort()` implementation
- **AoC 2024 Day 5** - Real-world competitive programming application

---

### **Related Zettelkasten Pages**

- [[kahns-topological-sort]] - Detailed Kahn's algorithm with examples
- [[DFS Patterns]] - DFS-based topological sort implementation
- [[Graph Theory MOC]] - Comprehensive graph algorithm overview
- [[cycle-detection]] - Detecting cycles in directed graphs
- [[mission-7]] - Graph data structure implementations
- [[mission-8]] - Graph traversal and algorithm library

---

*Tags: #graph-algorithms #topological-sort #DAG #dependency-resolution #DFS #BFS #kahns-algorithm #mission7 #mission8 #aoc2024*

---

*Links:*

- **Core Algorithms**: [[kahns-topological-sort]], [[DFS Patterns]]
- **Graph Foundations**: [[Graph Theory MOC]], [[mission-7]], [[mission-8]]
- **Applications**: [[aoc2024-day5-mission-integration]]
- **Related Concepts**: [[cycle-detection]], [[bfs-patterns]], [[Queue Data Structure]]
- **Advanced**: [[strongly-connected-components]], [[critical-path-method]]

