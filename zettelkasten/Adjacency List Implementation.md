# Adjacency List Implementation

*Tags: #graph-algorithms #implementation #rust #mission-7 #adjacency-list*

## Core Implementation Structure

Mission 7's adjacency list implementation follows Rust best practices for graph data structures:

```rust
pub struct Graph<T> {
    nodes: Vec<T>,                    // Node data storage
    adjacency_list: Vec<Vec<usize>>,  // Neighbor indices
    is_directed: bool,                // Graph type flag
}
```

## Key Implementation Details

### **Node Management**

```rust
impl<T> Graph<T> {
    pub fn add_node(&mut self, data: T) -> usize {
        let node_id = self.nodes.len();
        self.nodes.push(data);
        self.adjacency_list.push(Vec::new());
        node_id
    }
}
```

**Design Decision**: Using indices instead of pointers avoids Rust ownership complexity while maintaining O(1) node access.

### **Edge Operations**

```rust
pub fn add_edge(&mut self, from: usize, to: usize) -> Result<(), GraphError> {
    // Bounds checking
    if from >= self.nodes.len() || to >= self.nodes.len() {
        return Err(GraphError::InvalidNode);
    }
    
    // Add directed edge
    self.adjacency_list[from].push(to);
    
    // Add reverse edge for undirected graphs
    if !self.is_directed {
        self.adjacency_list[to].push(from);
    }
    
    Ok(())
}
```

**Safety First**: All operations include bounds checking to prevent panics.

### **Neighbor Access**

```rust
pub fn neighbors(&self, node: usize) -> Option<&Vec<usize>> {
    if node < self.adjacency_list.len() {
        Some(&self.adjacency_list[node])
    } else {
        None
    }
}
```

**Borrowing Strategy**: Return references to neighbor lists for efficient iteration without cloning.

## Advanced Features

### **Edge Existence Check**

```rust
pub fn has_edge(&self, from: usize, to: usize) -> bool {
    self.adjacency_list
        .get(from)
        .map(|neighbors| neighbors.contains(&to))
        .unwrap_or(false)
}
```

**Performance Note**: O(degree) complexity - consider `HashSet<usize>` for frequent edge queries.

### **Graph Statistics**

```rust
pub fn edge_count(&self) -> usize {
    let total_edges: usize = self.adjacency_list
        .iter()
        .map(|neighbors| neighbors.len())
        .sum();
    
    if self.is_directed {
        total_edges
    } else {
        total_edges / 2  // Each undirected edge counted twice
    }
}

pub fn vertex_count(&self) -> usize {
    self.nodes.len()
}
```

## Error Handling Strategy

```rust
#[derive(Debug, PartialEq)]
pub enum GraphError {
    InvalidNode,
    EdgeAlreadyExists,
    EdgeNotFound,
    EmptyGraph,
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GraphError::InvalidNode => write!(f, "Node index out of bounds"),
            GraphError::EdgeAlreadyExists => write!(f, "Edge already exists"),
            GraphError::EdgeNotFound => write!(f, "Edge not found"),
            GraphError::EmptyGraph => write!(f, "Operation invalid on empty graph"),
        }
    }
}

impl std::error::Error for GraphError {}
```

**Philosophy**: Explicit error types make debugging easier than generic errors.

## Memory Layout Optimization

### **Capacity Pre-allocation**

```rust
pub fn with_capacity(node_capacity: usize, is_directed: bool) -> Self {
    Graph {
        nodes: Vec::with_capacity(node_capacity),
        adjacency_list: Vec::with_capacity(node_capacity),
        is_directed,
    }
}
```

### **Neighbor List Optimization**

```rust
// For vertices expected to have many neighbors
pub fn reserve_neighbors(&mut self, node: usize, additional: usize) -> Result<(), GraphError> {
    if let Some(neighbors) = self.adjacency_list.get_mut(node) {
        neighbors.reserve(additional);
        Ok(())
    } else {
        Err(GraphError::InvalidNode)
    }
}
```

## Iterator Support

### **Node Iterator**

```rust
pub fn nodes(&self) -> impl Iterator<Item = (usize, &T)> {
    self.nodes.iter().enumerate()
}
```

### **Edge Iterator**

```rust
pub fn edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
    self.adjacency_list
        .iter()
        .enumerate()
        .flat_map(|(from, neighbors)| {
            neighbors.iter().map(move |&to| (from, to))
        })
}
```

**Iterator Philosophy**: Provide efficient, composable access patterns following Rust conventions.

## Performance Characteristics

### **Time Complexity Summary**

| Operation | Best Case | Average Case | Worst Case |
|-----------|-----------|--------------|------------|
| Add Node | O(1) | O(1) amortized | O(n) reallocation |
| Add Edge | O(1) | O(1) | O(1) |
| Has Edge | O(1) | O(degree) | O(degree) |
| Remove Edge | O(1) | O(degree) | O(degree) |
| Get Neighbors | O(1) | O(1) | O(1) |

### **Space Complexity**

- **Best Case**: O(V) for graphs with no edges
- **Average Case**: O(V + E) for typical sparse graphs  
- **Worst Case**: O(V²) for complete graphs (still better than adjacency matrix)

## Comparison with Alternative Approaches

### **vs HashSet-based Adjacency**

```rust
// HashMap approach (not used in Mission 7)
struct GraphWithHashSet<T> {
    nodes: Vec<T>,
    adjacency_list: Vec<HashSet<usize>>,
}
```

**Trade-offs**:

- ✅ HashSet: O(1) edge existence checks
- ❌ HashSet: Higher memory overhead, worse cache locality
- ✅ Vec: Better cache performance, lower memory usage
- ❌ Vec: O(degree) edge existence checks

### **vs Adjacency Matrix**

From [[mission-10]] tutorial analysis:

- **Adjacency List**: O(V+E) space, better for sparse graphs
- **Adjacency Matrix**: O(V²) space, O(1) edge queries, better for dense graphs

## Testing Strategy

### **Unit Test Structure**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_graph() {
        let graph: Graph<i32> = Graph::new(false);
        assert_eq!(graph.vertex_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_add_nodes() {
        let mut graph = Graph::new(false);
        let id1 = graph.add_node(42);
        let id2 = graph.add_node(43);
        
        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
        assert_eq!(graph.vertex_count(), 2);
    }

    #[test]
    fn test_directed_vs_undirected() {
        // Test both graph types to ensure correctness
    }
}
```

## Real-World Usage Patterns

### **Social Network Analysis**

```rust
let mut social_network = Graph::new(false);  // Undirected friendships
let alice = social_network.add_node("Alice");
let bob = social_network.add_node("Bob");
social_network.add_edge(alice, bob).unwrap();

// Find mutual friends
let alice_friends = social_network.neighbors(alice).unwrap();
let bob_friends = social_network.neighbors(bob).unwrap();
```

### **Dependency Graph**

```rust
let mut dependencies = Graph::new(true);  // Directed dependencies
let lib_a = dependencies.add_node("Library A");
let lib_b = dependencies.add_node("Library B");
dependencies.add_edge(lib_b, lib_a).unwrap();  // B depends on A
```

### **Route Planning**

```rust
let mut road_network = Graph::new(false);  // Bidirectional roads
let intersection1 = road_network.add_node("Main & 1st");
let intersection2 = road_network.add_node("Main & 2nd");
road_network.add_edge(intersection1, intersection2).unwrap();
```

## Integration with Graph Algorithms

### **DFS Implementation**

```rust
pub fn dfs_visit<F>(&self, start: usize, mut visitor: F) -> Result<(), GraphError>
where
    F: FnMut(usize, &T),
{
    let mut visited = vec![false; self.nodes.len()];
    let mut stack = vec![start];
    
    while let Some(node) = stack.pop() {
        if !visited[node] {
            visited[node] = true;
            visitor(node, &self.nodes[node]);
            
            // Add neighbors to stack
            if let Some(neighbors) = self.neighbors(node) {
                for &neighbor in neighbors.iter().rev() {  // Reverse for consistent ordering
                    if !visited[neighbor] {
                        stack.push(neighbor);
                    }
                }
            }
        }
    }
    
    Ok(())
}
```

### **BFS Implementation**  

```rust
use std::collections::VecDeque;

pub fn bfs_visit<F>(&self, start: usize, mut visitor: F) -> Result<(), GraphError>
where
    F: FnMut(usize, &T),
{
    let mut visited = vec![false; self.nodes.len()];
    let mut queue = VecDeque::new();
    
    queue.push_back(start);
    visited[start] = true;
    
    while let Some(node) = queue.pop_front() {
        visitor(node, &self.nodes[node]);
        
        if let Some(neighbors) = self.neighbors(node) {
            for &neighbor in neighbors {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }
    }
    
    Ok(())
}
```

## Production Quality Considerations

### **Thread Safety**

```rust
use std::sync::{Arc, RwLock};

type ThreadSafeGraph<T> = Arc<RwLock<Graph<T>>>;

// Usage example
let shared_graph: ThreadSafeGraph<String> = Arc::new(RwLock::new(Graph::new(false)));
```

### **Serialization Support**

```rust
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
pub struct Graph<T> {
    nodes: Vec<T>,
    adjacency_list: Vec<Vec<usize>>,
    is_directed: bool,
}
```

### **Custom Debug Implementation**

```rust
impl<T: std::fmt::Debug> std::fmt::Debug for Graph<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Graph")
            .field("nodes", &self.nodes.len())
            .field("edges", &self.edge_count())
            .field("is_directed", &self.is_directed)
            .finish()
    }
}
```

## Related Concepts

- [[adjacency-list-representation]] - Theoretical foundation and concepts
- [[adjacency-list-performance]] - Benchmarks and optimization techniques  
- [[mission-7]] - Complete Mission 7 implementation requirements
- [[graph-traversal]] - DFS/BFS algorithms on adjacency lists
- [[graph-algorithms]] - Advanced algorithms using this representation
- [[union-find-vs-adjacency]] - Performance comparison analysis

## Common Implementation Pitfalls

### **❌ Index Out of Bounds**

```rust
// DON'T: No bounds checking
self.adjacency_list[node].push(neighbor);

// DO: Always validate indices  
if node < self.adjacency_list.len() {
    self.adjacency_list[node].push(neighbor);
} else {
    return Err(GraphError::InvalidNode);
}
```

### **❌ Forgetting Bidirectional Edges**

```rust
// DON'T: Only add one direction for undirected graph
self.adjacency_list[from].push(to);

// DO: Handle both directions for undirected graphs
if !self.is_directed {
    self.adjacency_list[to].push(from);
}
```

### **❌ Memory Leaks in Edge Removal**

```rust
// DON'T: Leave dangling references
self.adjacency_list[from].retain(|&x| x != to);
// Forgot to remove reverse edge in undirected graph!

// DO: Clean up all references
self.adjacency_list[from].retain(|&x| x != to);
if !self.is_directed {
    self.adjacency_list[to].retain(|&x| x != from);
}
```

*Links: [[adjacency-list-representation]] | [[mission-7]] | [[graph-performance-analysis]]*
