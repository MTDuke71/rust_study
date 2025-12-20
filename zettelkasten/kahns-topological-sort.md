# 🔄 Kahn's Topological Sort Algorithm

*Queue-based approach to dependency resolution and cycle detection*

---

## 📋 Core Concept

**Definition**: Kahn's algorithm is an iterative method for topological sorting that uses **in-degree counting** and a **queue-based approach** to produce a linear ordering of vertices in a Directed Acyclic Graph (DAG).

**Key Insight**: Vertices with in-degree 0 have no dependencies and can be processed first. Removing them reduces the in-degree of their neighbors, potentially creating new vertices with in-degree 0.

**Time Complexity**: `O(V + E)` where V = vertices, E = edges  
**Space Complexity**: `O(V)` for in-degree tracking and queue storage

---

## 🧠 Algorithm Intuition

### **Mental Model: Course Prerequisites**

```
Prerequisites: [Math101 → Stats201], [Math101 → CS202], [Stats201 → ML301]

Step 1: Take Math101 (no prerequisites)
Step 2: Take Stats201 and CS202 (Math101 completed)  
Step 3: Take ML301 (Stats201 completed)

Result: Math101 → Stats201 → CS202 → ML301 (valid topological order)
```

### **Queue-Based Processing**

1. **Initialize**: Count in-degrees for all vertices
2. **Seed Queue**: Add all vertices with in-degree 0
3. **Process Loop**:
   - Dequeue vertex → add to result
   - For each neighbor: decrease in-degree by 1
   - If neighbor's in-degree becomes 0 → enqueue it
4. **Validation**: If result contains all vertices → no cycles, else cycle exists

---

## 🏗️ Implementation Architecture

### **Core Data Structures**

```rust
use std::collections::{HashMap, VecDeque, HashSet};

struct KahnsTopologicalSort {
    // In-degree count for each vertex
    in_degree: HashMap<NodeId, usize>,
    
    // Adjacency list representation
    adj_list: HashMap<NodeId, Vec<NodeId>>,
    
    // Processing queue for vertices with in-degree 0
    queue: VecDeque<NodeId>,
    
    // Result ordering
    result: Vec<NodeId>,
}
```

### **Algorithm Implementation**

```rust
impl KahnsTopologicalSort {
    pub fn topological_sort(
        adj_list: &HashMap<NodeId, Vec<NodeId>>
    ) -> Result<Vec<NodeId>, CycleError> {
        let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        // Step 1: Initialize in-degrees to 0
        for &node in adj_list.keys() {
            in_degree.insert(node, 0);
        }
        
        // Step 2: Calculate actual in-degrees
        for neighbors in adj_list.values() {
            for &neighbor in neighbors {
                *in_degree.entry(neighbor).or_insert(0) += 1;
            }
        }
        
        // Step 3: Find all vertices with in-degree 0
        for (&node, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node);
            }
        }
        
        // Step 4: Process vertices in topological order
        while let Some(node) = queue.pop_front() {
            result.push(node);
            
            // Reduce in-degree of all neighbors
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
        
        // Step 5: Cycle detection
        if result.len() != adj_list.len() {
            Err(CycleError::new("Graph contains cycles"))
        } else {
            Ok(result)
        }
    }
}
```

---

## 🔍 Algorithm Walkthrough

### **Example: Dependency Graph**

```
Graph: A → B, A → C, B → D, C → D
Adjacency List: {A: [B,C], B: [D], C: [D], D: []}
```

### **Step-by-Step Execution**

```
Initial State:
  in_degree: {A: 0, B: 1, C: 1, D: 2}
  queue: [A]
  result: []

Iteration 1: Process A
  result: [A]
  Update neighbors: B(1→0), C(1→0)  
  queue: [B, C]

Iteration 2: Process B  
  result: [A, B]
  Update neighbors: D(2→1)
  queue: [C]

Iteration 3: Process C
  result: [A, B, C]  
  Update neighbors: D(1→0)
  queue: [D]

Iteration 4: Process D
  result: [A, B, C, D]
  queue: []

Final: All 4 vertices processed → Valid topological order
```

---

## 🎯 Key Characteristics

### **Advantages over DFS-based Topological Sort**

1. **Explicit Cycle Detection**: Algorithm naturally detects cycles during execution
2. **Iterative Implementation**: No recursion stack depth concerns
3. **In-Degree Insight**: Provides dependency count information
4. **Incremental Processing**: Can handle dynamic graph updates more naturally

### **Disadvantages**

1. **Extra Memory**: Requires in-degree tracking (O(V) additional space)
2. **Initialization Overhead**: Two-pass algorithm (calculate in-degrees, then process)
3. **Less Intuitive**: Queue-based approach less intuitive than DFS traversal

### **Cycle Detection Capability**

```rust
// Kahn's algorithm automatically detects cycles
if result.len() != total_vertices {
    // Some vertices never reached in-degree 0
    // → Circular dependencies exist
    return Err("Cycle detected");
}
```

---

## 🚀 Real-World Applications

### **1. Build System Dependency Resolution**

```rust
// Example: Cargo build order determination
dependencies: {
    "serde": [],
    "tokio": ["serde"], 
    "myapp": ["tokio", "serde"]
}

// Kahn's result: ["serde", "tokio", "myapp"]
// Build order: compile serde → compile tokio → compile myapp
```

### **2. Course Scheduling**

```rust
// University course prerequisites
prerequisites: {
    "Math101": [],
    "Stats201": ["Math101"],
    "CS202": ["Math101"], 
    "ML301": ["Stats201", "CS202"]
}

// Valid semester ordering for graduation
```

### **3. Task Scheduling with Dependencies**

```rust
// Project management: task dependency resolution
tasks: {
    "Design": [],
    "Frontend": ["Design"],
    "Backend": ["Design"],
    "Testing": ["Frontend", "Backend"],
    "Deploy": ["Testing"]
}
```

---

## 🔬 Mission Integration Examples

### **AoC 2024 Day 5 Integration**

*Real-world application from completed implementation*

```rust
// From day05_real_missions_clean.rs
pub fn fix_sequence(&self, update: &[i32]) -> Result<Vec<i32>> {
    // Step 1: Build subgraph for this specific sequence
    let pages_set: HashSet<i32> = update.iter().copied().collect();
    let mut adj_list: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
    
    // Step 2: Extract relevant edges from full graph
    for &page in update {
        if let Some(&node_id) = self.page_to_node.get(&page) {
            let mut neighbors = Vec::new();
            for &neighbor_id in self.graph.neighbors(node_id) {
                if let Some(&neighbor_page) = self.node_to_page.get(&neighbor_id) {
                    if pages_set.contains(&neighbor_page) {
                        neighbors.push(neighbor_id);
                    }
                }
            }
            adj_list.insert(node_id, neighbors);
        }
    }
    
    // Step 3: Apply Kahn's algorithm for dependency resolution
    let has_cycles = m8::has_cycle(&adj_list);
    if has_cycles {
        return self.fix_sequence_with_rules(update); // Fallback strategy
    }
    
    // Step 4: Standard Kahn's topological sort
    let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    
    // ... (standard Kahn's implementation)
    
    Ok(result)
}
```

**Key Integration Insights**:

- **Subgraph Extraction**: Only process vertices relevant to current problem
- **Cycle Handling**: Detect cycles and gracefully fallback to alternative algorithms  
- **Bidirectional Mapping**: Convert between problem space (page numbers) and graph space (NodeIds)
- **Mission 8 Integration**: Use `m8::has_cycle()` for explicit cycle detection before sorting

---

## 📊 Performance Analysis

### **Complexity Breakdown**

```
Phase 1 - In-degree Calculation: O(E)
Phase 2 - Queue Initialization: O(V)  
Phase 3 - Main Processing Loop: O(V + E)
Phase 4 - Cycle Detection: O(1)

Total: O(V + E) - Linear in graph size
```

### **Memory Usage**

```
In-degree map: O(V)
Processing queue: O(V) worst case
Result vector: O(V)
Adjacency list: O(V + E) (usually provided)

Total Additional: O(V) beyond input graph
```

### **Benchmark Comparison**

*From AoC Day 5 real dataset*

```
Graph Size: 49 nodes, 1,176 edges
Kahn's Runtime: ~0.2ms
DFS Topological: ~0.15ms  
Bubble Sort Fallback: ~1.2ms

Winner: DFS for small graphs, Kahn's for explicit cycle detection
```

---

## 🧪 Testing Strategies

### **Core Test Cases**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_dag() {
        // A → B → C
        let adj_list = hashmap! {
            0 => vec![1],
            1 => vec![2], 
            2 => vec![]
        };
        
        let result = KahnsTopologicalSort::topological_sort(&adj_list).unwrap();
        assert_eq!(result, vec![0, 1, 2]);
    }
    
    #[test] 
    fn test_cycle_detection() {
        // A → B → C → A (cycle)
        let adj_list = hashmap! {
            0 => vec![1],
            1 => vec![2],
            2 => vec![0]  // Creates cycle
        };
        
        assert!(KahnsTopologicalSort::topological_sort(&adj_list).is_err());
    }
    
    #[test]
    fn test_multiple_valid_orders() {
        // A → C, B → C (A and B can be processed in any order)
        let adj_list = hashmap! {
            0 => vec![2],  // A → C
            1 => vec![2],  // B → C  
            2 => vec![]    // C
        };
        
        let result = KahnsTopologicalSort::topological_sort(&adj_list).unwrap();
        assert_eq!(result.len(), 3);
        assert!(result.iter().position(|&x| x == 2).unwrap() == 2); // C must be last
    }
    
    #[test]
    fn test_empty_graph() {
        let adj_list: HashMap<i32, Vec<i32>> = HashMap::new();
        let result = KahnsTopologicalSort::topological_sort(&adj_list).unwrap();
        assert_eq!(result, vec![]);
    }
    
    #[test]
    fn test_single_node() {
        let adj_list = hashmap! { 0 => vec![] };
        let result = KahnsTopologicalSort::topological_sort(&adj_list).unwrap();
        assert_eq!(result, vec![0]);
    }
}
```

### **Property-Based Testing**

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_all_nodes_included(
        nodes in prop::collection::vec(0..20i32, 1..10),
        edges in prop::collection::vec((0..20i32, 0..20i32), 0..30)
    ) {
        let adj_list = build_dag_from_edges(&nodes, &edges);
        
        match KahnsTopologicalSort::topological_sort(&adj_list) {
            Ok(result) => {
                // All nodes should be included exactly once
                assert_eq!(result.len(), nodes.len());
                for &node in &nodes {
                    assert_eq!(result.iter().filter(|&&x| x == node).count(), 1);
                }
            }
            Err(_) => {
                // Cycle detected - validate using DFS
                assert!(has_cycle_dfs(&adj_list));
            }
        }
    }
}
```

---

## 🔄 Comparison with Alternative Approaches

### **Kahn's vs DFS-based Topological Sort**

| **Aspect** | **Kahn's Algorithm** | **DFS-based** |
|------------|---------------------|---------------|
| **Approach** | Queue-based, iterative | Stack-based, recursive |
| **Cycle Detection** | Explicit during execution | Requires separate pass |
| **Memory** | O(V) additional | O(V) recursion stack |
| **Intuition** | Dependency counting | Graph traversal |
| **Implementation** | More complex setup | More straightforward |
| **Performance** | O(V + E) | O(V + E) |

### **When to Choose Kahn's**

- ✅ **Explicit cycle detection needed**
- ✅ **Avoiding deep recursion** (large graphs)
- ✅ **Understanding dependency counts** is valuable
- ✅ **Incremental processing** of dynamic graphs

### **When to Choose DFS**

- ✅ **Simpler implementation** preferred
- ✅ **Memory constraints** (no extra in-degree storage)
- ✅ **Small graphs** where recursion depth isn't a concern
- ✅ **Already have DFS infrastructure** in place

---

## 🌐 Advanced Applications & Extensions

### **1. Parallel Kahn's Algorithm**

```rust
// Process vertices with in-degree 0 in parallel
use rayon::prelude::*;

let zero_in_degree_nodes: Vec<NodeId> = in_degree
    .par_iter()
    .filter_map(|(&node, &degree)| if degree == 0 { Some(node) } else { None })
    .collect();

// Process level by level in parallel
for level_nodes in zero_in_degree_nodes.chunks(thread_count) {
    level_nodes.par_iter().for_each(|&node| {
        // Process node and update neighbors atomically
    });
}
```

### **2. Lexicographic Topological Ordering**

```rust
// Use BinaryHeap instead of VecDeque for smallest-first ordering
use std::collections::BinaryHeap;
use std::cmp::Reverse;

let mut priority_queue = BinaryHeap::new();

// Ensure lexicographically smallest valid topological order
for (&node, &degree) in &in_degree {
    if degree == 0 {
        priority_queue.push(Reverse(node)); // Min-heap behavior
    }
}

while let Some(Reverse(node)) = priority_queue.pop() {
    result.push(node);
    // ... continue with standard algorithm
}
```

### **3. Incremental Topological Sort**

```rust
// Handle dynamic edge insertion/deletion
impl IncrementalKahns {
    pub fn add_edge(&mut self, from: NodeId, to: NodeId) -> Result<(), CycleError> {
        // Check if edge creates cycle before adding
        if self.would_create_cycle(from, to) {
            return Err(CycleError::new("Edge would create cycle"));
        }
        
        self.adj_list.entry(from).or_default().push(to);
        *self.in_degree.entry(to).or_insert(0) += 1;
        
        // Recompute only affected portion of topological order
        self.incremental_recompute(to);
        Ok(())
    }
}
```

---

## 🎓 Learning Integration & Mastery

### **Conceptual Prerequisites**

- **[[Graph Theory MOC]]** - Basic graph terminology and representation
- **[[Queue Data Structure]]** - FIFO processing for algorithm execution
- **[[HashMap Deep Dive]]** - Efficient key-value lookups for in-degree tracking
- **[[cycle-detection]]** - Understanding circular dependencies in graphs

### **Implementation Skills**

- **[[Iterator Patterns]]** - Efficient graph traversal and processing
- **[[Error Handling Patterns]]** - Robust cycle detection and error reporting
- **[[Testing Strategies]]** - Comprehensive validation of sorting correctness
- **[[Performance Analysis]]** - Understanding time/space complexity trade-offs

### **Application Domains**

- **[[Build System Design]]** - Dependency resolution in software compilation
- **[[Task Scheduling]]** - Project management and workflow optimization
- **[[Constraint Satisfaction]]** - Dependency-based problem solving
- **[[Graph Database Queries]]** - Dependency traversal in data relationships

---

## 🔧 Common Implementation Pitfalls

### **1. Incorrect In-degree Initialization**

```rust
// ❌ Wrong: Only counting nodes that appear as sources
for source in adj_list.keys() {
    in_degree.insert(*source, 0);
}

// ✅ Correct: Initialize ALL nodes, including isolated targets
for source in adj_list.keys() {
    in_degree.entry(*source).or_insert(0);
}
for targets in adj_list.values() {
    for &target in targets {
        in_degree.entry(target).or_insert(0);
    }
}
```

### **2. Forgetting Cycle Validation**

```rust
// ❌ Wrong: Assuming result is always valid
pub fn topological_sort(adj_list: &HashMap<NodeId, Vec<NodeId>>) -> Vec<NodeId> {
    // ... algorithm implementation
    result  // May be incomplete if cycles exist!
}

// ✅ Correct: Explicit cycle detection
pub fn topological_sort(
    adj_list: &HashMap<NodeId, Vec<NodeId>>
) -> Result<Vec<NodeId>, CycleError> {
    // ... algorithm implementation
    
    if result.len() != adj_list.len() {
        Err(CycleError::new("Graph contains cycles"))
    } else {
        Ok(result)
    }
}
```

### **3. Modifying In-degree During Iteration**

```rust
// ❌ Wrong: Modifying HashMap while iterating
for (&node, &degree) in &in_degree {
    if degree == 0 {
        // This modifies in_degree during iteration!
        *in_degree.get_mut(&neighbor).unwrap() -= 1;
    }
}

// ✅ Correct: Separate collection and processing phases
let mut to_process = Vec::new();
for (&node, &degree) in &in_degree {
    if degree == 0 {
        to_process.push(node);
    }
}

for node in to_process {
    // Now safe to modify in_degree
    // ... process neighbors
}
```

---

## 📚 Further Reading & Resources

### **Academic Sources**

- **Kahn, A. B. (1962)** - "Topological sorting of large networks" - Original algorithm paper
- **Cormen, et al.** - "Introduction to Algorithms" - Section 22.4: Topological Sort
- **Sedgewick & Wayne** - "Algorithms" - Directed graphs and topological ordering

### **Implementation References**

- **[[Mission 8]]** - `has_cycle()` function used in AoC Day 5 integration
- **[[topological-sort]]** - General topological sort overview and DFS comparison
- **[[DFS Patterns]]** - Alternative recursive approach to topological sorting
- **[[graph-algorithms]]** - Comprehensive graph algorithm implementations
- **[[aoc2024-day5-mission-integration]]** - Real-world competitive programming application

### **Related Algorithms**

- **[[strongly-connected-components]]** - Tarjan's and Kosaraju's algorithms
- **[[minimum-spanning-tree]]** - Kruskal's and Prim's algorithms
- **[[shortest-path-algorithms]]** - Dijkstra and Bellman-Ford implementations
- **[[network-flow-algorithms]]** - Max-flow and min-cut problems

---

*Tags: #graph-algorithms #topological-sort #kahn-algorithm #cycle-detection #dependency-resolution #queue-algorithms #dag #competitive-programming #mission-integration*

*Links: [[topological-sort]] | [[graph-algorithms]] | [[Graph Theory MOC]] | [[DFS Patterns]] | [[aoc2024-day5-mission-integration]] | [[Mission 8]] | [[Queue Data Structure]] | [[cycle-detection]] | [[HashMap Deep Dive]] | [[Algorithm Design Patterns]] | [[Build System Design]] | [[Task Scheduling]] | [[Performance Analysis]] | [[Testing Strategies]] | [[zettel-index]]*
