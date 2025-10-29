# BFS Algorithms

*Core breadth-first search algorithm implementations and variants*

*Tags: #bfs #algorithms #graph-traversal #mission-7 #pathfinding*

## 🎯 Overview

This page covers the fundamental BFS algorithm implementations that form the foundation of graph traversal and pathfinding. While [[BFS Patterns]] focuses on common usage patterns, this page details the core algorithms themselves.

## 📚 Fundamental BFS Algorithms

### **1. Basic Graph BFS**
*Standard breadth-first traversal of graph nodes*

```rust
use std::collections::{VecDeque, HashSet};

pub fn basic_bfs<T>(
    graph: &HashMap<T, Vec<T>>,
    start: T,
    visit: impl Fn(&T),
) where T: Clone + Eq + std::hash::Hash {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back(start.clone());
    visited.insert(start);
    
    while let Some(current) = queue.pop_front() {
        visit(&current);
        
        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }
}
```

### **2. BFS Component Detection**
*Find all connected components using BFS*

```rust
pub fn find_connected_components<T>(
    graph: &HashMap<T, Vec<T>>,
) -> Vec<Vec<T>>
where T: Clone + Eq + std::hash::Hash {
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    
    for node in graph.keys() {
        if !visited.contains(node) {
            let mut component = Vec::new();
            let mut queue = VecDeque::new();
            
            queue.push_back(node.clone());
            visited.insert(node.clone());
            
            while let Some(current) = queue.pop_front() {
                component.push(current.clone());
                
                if let Some(neighbors) = graph.get(&current) {
                    for neighbor in neighbors {
                        if !visited.contains(neighbor) {
                            visited.insert(neighbor.clone());
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
            
            components.push(component);
        }
    }
    
    components
}
```

### **3. BFS Tree Construction**
*Build BFS spanning tree with parent tracking*

```rust
pub struct BfsTree<T> {
    pub parent: HashMap<T, Option<T>>,
    pub level: HashMap<T, u32>,
    pub children: HashMap<T, Vec<T>>,
}

pub fn build_bfs_tree<T>(
    graph: &HashMap<T, Vec<T>>,
    root: T,
) -> BfsTree<T>
where T: Clone + Eq + std::hash::Hash {
    let mut tree = BfsTree {
        parent: HashMap::new(),
        level: HashMap::new(),
        children: HashMap::new(),
    };
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    // Initialize root
    queue.push_back(root.clone());
    visited.insert(root.clone());
    tree.parent.insert(root.clone(), None);
    tree.level.insert(root.clone(), 0);
    tree.children.insert(root.clone(), Vec::new());
    
    while let Some(current) = queue.pop_front() {
        let current_level = tree.level[&current];
        
        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    visited.insert(neighbor.clone());
                    
                    // Update tree structure
                    tree.parent.insert(neighbor.clone(), Some(current.clone()));
                    tree.level.insert(neighbor.clone(), current_level + 1);
                    tree.children.insert(neighbor.clone(), Vec::new());
                    
                    // Add to current node's children
                    tree.children.get_mut(&current).unwrap().push(neighbor.clone());
                    
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }
    
    tree
}
```

### **4. BFS Cycle Detection**
*Detect cycles in undirected graphs using BFS*

```rust
pub fn has_cycle_bfs<T>(
    graph: &HashMap<T, Vec<T>>,
) -> bool
where T: Clone + Eq + std::hash::Hash {
    let mut visited = HashSet::new();
    
    for start_node in graph.keys() {
        if !visited.contains(start_node) {
            let mut queue = VecDeque::new();
            let mut parent = HashMap::new();
            
            queue.push_back(start_node.clone());
            visited.insert(start_node.clone());
            parent.insert(start_node.clone(), None);
            
            while let Some(current) = queue.pop_front() {
                if let Some(neighbors) = graph.get(&current) {
                    for neighbor in neighbors {
                        if !visited.contains(neighbor) {
                            visited.insert(neighbor.clone());
                            parent.insert(neighbor.clone(), Some(current.clone()));
                            queue.push_back(neighbor.clone());
                        } else if Some(neighbor) != parent.get(&current) {
                            // Found back edge (cycle)
                            return true;
                        }
                    }
                }
            }
        }
    }
    
    false
}
```

### **5. BFS Bipartite Check**
*Determine if graph is bipartite using 2-coloring*

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Color { Red, Blue }

pub fn is_bipartite<T>(
    graph: &HashMap<T, Vec<T>>,
) -> bool
where T: Clone + Eq + std::hash::Hash {
    let mut colors = HashMap::new();
    
    for start_node in graph.keys() {
        if !colors.contains_key(start_node) {
            let mut queue = VecDeque::new();
            
            queue.push_back(start_node.clone());
            colors.insert(start_node.clone(), Color::Red);
            
            while let Some(current) = queue.pop_front() {
                let current_color = colors[&current];
                let next_color = match current_color {
                    Color::Red => Color::Blue,
                    Color::Blue => Color::Red,
                };
                
                if let Some(neighbors) = graph.get(&current) {
                    for neighbor in neighbors {
                        if let Some(&neighbor_color) = colors.get(neighbor) {
                            if neighbor_color == current_color {
                                return false; // Same color as current - not bipartite
                            }
                        } else {
                            colors.insert(neighbor.clone(), next_color);
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }
    }
    
    true
}
```

## 🌊 Advanced BFS Variants

### **6. Parallel BFS**
*Multi-level parallel BFS for large graphs*

```rust
use rayon::prelude::*;

pub fn parallel_bfs<T>(
    graph: &HashMap<T, Vec<T>>,
    start: T,
) -> HashMap<T, u32>
where T: Clone + Eq + std::hash::Hash + Send + Sync {
    let mut distances = HashMap::new();
    let mut current_level = vec![start.clone()];
    distances.insert(start, 0);
    let mut level = 0;
    
    while !current_level.is_empty() {
        level += 1;
        let next_level: Vec<T> = current_level
            .par_iter()
            .flat_map(|node| {
                graph.get(node).unwrap_or(&Vec::new()).iter()
                    .filter(|neighbor| !distances.contains_key(neighbor))
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .collect();
        
        // Update distances for next level
        let mut new_distances = HashMap::new();
        for node in &next_level {
            new_distances.insert(node.clone(), level);
        }
        
        distances.extend(new_distances);
        current_level = next_level;
    }
    
    distances
}
```

### **7. BFS with Custom Queue**
*BFS using different queue implementations for optimization*

```rust
use std::collections::VecDeque;

pub trait Queue<T> {
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn is_empty(&self) -> bool;
}

pub struct VecDequeAdapter<T>(VecDeque<T>);

impl<T> Queue<T> for VecDequeAdapter<T> {
    fn push(&mut self, item: T) { self.0.push_back(item); }
    fn pop(&mut self) -> Option<T> { self.0.pop_front() }
    fn is_empty(&self) -> bool { self.0.is_empty() }
}

pub fn generic_bfs<T, Q>(
    graph: &HashMap<T, Vec<T>>,
    start: T,
    queue: &mut Q,
) -> Vec<T>
where 
    T: Clone + Eq + std::hash::Hash,
    Q: Queue<T>,
{
    let mut visited = HashSet::new();
    let mut result = Vec::new();
    
    queue.push(start.clone());
    visited.insert(start);
    
    while !queue.is_empty() {
        if let Some(current) = queue.pop() {
            result.push(current.clone());
            
            if let Some(neighbors) = graph.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push(neighbor.clone());
                    }
                }
            }
        }
    }
    
    result
}
```

## 📊 Algorithm Analysis

### **Time Complexity**
- **Basic BFS**: O(V + E) where V = vertices, E = edges
- **Component Detection**: O(V + E) for all components
- **Tree Construction**: O(V + E) with additional tree storage
- **Cycle Detection**: O(V + E) worst case
- **Bipartite Check**: O(V + E) for 2-coloring

### **Space Complexity**
- **Visited Set**: O(V) for marking visited nodes
- **Queue Storage**: O(V) in worst case (all nodes in queue)
- **Tree Storage**: O(V) for parent/level tracking
- **Total**: O(V) auxiliary space

### **Performance Characteristics**
```rust
// Performance comparison for different implementations
#[cfg(test)]
mod benchmarks {
    use criterion::{black_box, Criterion};
    
    pub fn bench_bfs_implementations(c: &mut Criterion) {
        let graph = create_large_graph(10000, 50000);
        
        c.bench_function("basic_bfs", |b| {
            b.iter(|| basic_bfs(black_box(&graph), black_box(0), |_| {}))
        });
        
        c.bench_function("bfs_tree", |b| {
            b.iter(|| build_bfs_tree(black_box(&graph), black_box(0)))
        });
        
        c.bench_function("parallel_bfs", |b| {
            b.iter(|| parallel_bfs(black_box(&graph), black_box(0)))
        });
    }
}
```

## 🔧 Mission 7 Integration

### **Graph Structure Compatibility**
```rust
// Integration with Mission 7 graph implementation
use mission7::Graph;

impl<N, E> Graph<N, E> 
where 
    N: Clone + Eq + std::hash::Hash,
    E: Clone,
{
    pub fn bfs_traversal(&self, start: &N) -> Vec<N> {
        let adjacency_map = self.to_adjacency_map();
        let mut result = Vec::new();
        
        basic_bfs(&adjacency_map, start.clone(), |node| {
            result.push(node.clone());
        });
        
        result
    }
    
    pub fn find_components(&self) -> Vec<Vec<N>> {
        let adjacency_map = self.to_adjacency_map();
        find_connected_components(&adjacency_map)
    }
    
    pub fn is_connected(&self) -> bool {
        self.find_components().len() <= 1
    }
}
```

### **REQ-4 Algorithm Foundation Support**
- Provides visited tracking infrastructure used by BFS
- Path reconstruction for BFS pathfinding results  
- Result structures for component analysis

## 🎯 Use Cases & Applications

### **1. Social Network Analysis**
- Find degrees of separation between users
- Detect friend groups (connected components)
- Analyze network structure and clustering

### **2. Web Crawling & Site Maps**
- Breadth-first website exploration
- Build site hierarchies and link structures
- Detect broken links and isolated pages

### **3. Network Topology**
- Analyze computer network connectivity
- Find network bottlenecks and critical nodes
- Detect network partitions and islands

### **4. Game Development**
- Level connectivity validation
- AI pathfinding preparation
- Zone/area detection in game worlds

### **5. Data Structure Validation**
- Tree structure verification
- Cycle detection in dependencies
- Data flow analysis

## 🔗 Related Algorithms

### **Graph Traversal Family**
- [[BFS Patterns]] - Common BFS usage patterns and templates
- [[DFS Patterns]] - Depth-first search alternatives
- [[Graph Algorithms]] - Comprehensive graph algorithm reference

### **Pathfinding Extensions**  
- [[BFS Pathfinding]] - Shortest path algorithms using BFS
- [[Dijkstra Algorithm]] - Weighted shortest paths (Mission 9)
- [[A-Star Algorithm]] - Heuristic-guided pathfinding (Mission 9)

### **Optimization Variants**
- [[BFS Optimization]] - Performance improvements and variants
- [[Parallel Graph Algorithms]] - Multi-threaded graph processing
- [[Cache-Efficient Traversal]] - Memory optimization techniques

## 💡 Key Insights

1. **Queue is Fundamental**: BFS behavior completely depends on FIFO queue discipline
2. **Level-by-Level Guarantee**: Ensures shortest path in unweighted graphs
3. **Memory vs Time Trade-off**: Higher memory usage but guarantees optimality
4. **Parallel Opportunities**: Level-based processing enables parallelization
5. **Foundation for Advanced Algorithms**: Many complex algorithms build on BFS

### **Algorithm Selection Guidelines**
```
Use Basic BFS when:
✅ Need simple graph traversal
✅ Want to visit all reachable nodes
✅ Memory is not a constraint

Use BFS Tree when:  
✅ Need parent-child relationships
✅ Want level information
✅ Building spanning trees

Use Component Detection when:
✅ Need to find disconnected parts
✅ Analyzing graph structure
✅ Preparing for parallel processing

Use Specialized Variants when:
✅ Have specific constraints (bipartite, cycles)
✅ Need optimization for large graphs
✅ Want domain-specific functionality
```

**BFS Philosophy**: 
> "Explore systematically, level by level. In the world of unweighted graphs, the first path found is the optimal path." 🌊

---

*Links: [[BFS Patterns]] | [[BFS Optimization]] | [[BFS Pathfinding]] | [[Mission7 Overview]] | [[Graph Algorithms]] | [[DFS Patterns]] | [[mission-7]] | [[zettel-index]]*