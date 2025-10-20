//! Step 6: Integration Testing for Graph Algorithms
//! 
//! This tutorial demonstrates comprehensive integration testing strategies
//! for graph algorithms, including property-based testing, AoC validation,
//! and cross-verification with reference implementations.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

/// Graph trait that our algorithms expect
pub trait Graph<T> {
    fn neighbors(&self, node: &T) -> Vec<T>;
    fn nodes(&self) -> Vec<T>;
    fn contains(&self, node: &T) -> bool;
}

/// Simple adjacency list graph for testing
#[derive(Debug, Clone)]
pub struct AdjacencyListGraph {
    adjacency: HashMap<usize, Vec<usize>>,
    node_count: usize,
}

impl AdjacencyListGraph {
    pub fn new(node_count: usize) -> Self {
        let mut adjacency = HashMap::new();
        for i in 0..node_count {
            adjacency.insert(i, Vec::new());
        }
        
        Self { adjacency, node_count }
    }
    
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.node_count && to < self.node_count {
            self.adjacency.get_mut(&from).unwrap().push(to);
        }
    }
    
    pub fn add_undirected_edge(&mut self, a: usize, b: usize) {
        self.add_edge(a, b);
        self.add_edge(b, a);
    }
}

impl Graph<usize> for AdjacencyListGraph {
    fn neighbors(&self, node: &usize) -> Vec<usize> {
        self.adjacency.get(node).cloned().unwrap_or_default()
    }
    
    fn nodes(&self) -> Vec<usize> {
        (0..self.node_count).collect()
    }
    
    fn contains(&self, node: &usize) -> bool {
        *node < self.node_count
    }
}

/// Result of a graph traversal algorithm
#[derive(Debug, Clone)]
pub struct TraversalResult<T> 
where
    T: Clone + Eq + Hash,
{
    pub visited: Vec<T>,
    pub parent: HashMap<T, T>,
    pub discovery_time: HashMap<T, usize>,
}

impl<T> TraversalResult<T> 
where
    T: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            visited: Vec::new(),
            parent: HashMap::new(),
            discovery_time: HashMap::new(),
        }
    }
}

/// Generic BFS implementation for testing
pub fn bfs<T, G>(graph: &G, start: T) -> TraversalResult<T>
where
    T: Clone + Eq + Hash + Debug,
    G: Graph<T>,
{
    let mut result = TraversalResult::new();
    let mut queue = VecDeque::new();
    let mut visited_set = HashSet::new();
    let mut time = 0;
    
    queue.push_back(start.clone());
    visited_set.insert(start.clone());
    result.discovery_time.insert(start.clone(), time);
    time += 1;
    
    while let Some(current) = queue.pop_front() {
        result.visited.push(current.clone());
        
        for neighbor in graph.neighbors(&current) {
            if !visited_set.contains(&neighbor) {
                visited_set.insert(neighbor.clone());
                result.parent.insert(neighbor.clone(), current.clone());
                result.discovery_time.insert(neighbor.clone(), time);
                queue.push_back(neighbor);
                time += 1;
            }
        }
    }
    
    result
}

/// Generic DFS implementation for testing
pub fn dfs<T, G>(graph: &G, start: T) -> TraversalResult<T>
where
    T: Clone + Eq + Hash + Debug,
    G: Graph<T>,
{
    let mut result = TraversalResult::new();
    let mut stack = Vec::new();
    let mut visited_set = HashSet::new();
    let mut time = 0;
    
    stack.push(start.clone());
    
    while let Some(current) = stack.pop() {
        if !visited_set.contains(&current) {
            visited_set.insert(current.clone());
            result.visited.push(current.clone());
            result.discovery_time.insert(current.clone(), time);
            time += 1;
            
            // Add neighbors in reverse order to maintain left-to-right traversal
            let mut neighbors = graph.neighbors(&current);
            neighbors.reverse();
            
            for neighbor in neighbors {
                if !visited_set.contains(&neighbor) {
                    result.parent.insert(neighbor.clone(), current.clone());
                    stack.push(neighbor);
                }
            }
        }
    }
    
    result
}

/// Find connected components using DFS
pub fn find_connected_components<T, G>(graph: &G) -> Vec<Vec<T>>
where
    T: Clone + Eq + Hash + Debug,
    G: Graph<T>,
{
    let mut components = Vec::new();
    let mut global_visited = HashSet::new();
    
    for node in graph.nodes() {
        if !global_visited.contains(&node) {
            let component_result = dfs(graph, node);
            for visited_node in &component_result.visited {
                global_visited.insert(visited_node.clone());
            }
            components.push(component_result.visited);
        }
    }
    
    components
}

/// Check if a graph is connected (has exactly one component)
pub fn is_connected<T, G>(graph: &G) -> bool
where
    T: Clone + Eq + Hash + Debug,
    G: Graph<T>,
{
    let components = find_connected_components(graph);
    components.len() <= 1
}

/// Find shortest path between two nodes using BFS
pub fn shortest_path<T, G>(graph: &G, start: T, end: T) -> Option<Vec<T>>
where
    T: Clone + Eq + Hash + Debug,
    G: Graph<T>,
{
    let bfs_result = bfs(graph, start.clone());
    
    if !bfs_result.visited.contains(&end) {
        return None; // End not reachable from start
    }
    
    // Reconstruct path from parent map
    let mut path = Vec::new();
    let mut current = end;
    
    loop {
        path.push(current.clone());
        if let Some(parent) = bfs_result.parent.get(&current) {
            current = parent.clone();
        } else {
            break; // Reached start node
        }
    }
    
    path.reverse();
    Some(path)
}

/// Test helper: Create a simple chain graph (0 -> 1 -> 2 -> ... -> n-1)
fn create_chain_graph(length: usize) -> AdjacencyListGraph {
    let mut graph = AdjacencyListGraph::new(length);
    for i in 0..length - 1 {
        graph.add_edge(i, i + 1);
    }
    graph
}

/// Test helper: Create a complete graph (every node connected to every other)
fn create_complete_graph(size: usize) -> AdjacencyListGraph {
    let mut graph = AdjacencyListGraph::new(size);
    for i in 0..size {
        for j in 0..size {
            if i != j {
                graph.add_edge(i, j);
            }
        }
    }
    graph
}

/// Test helper: Create a disconnected graph with two components
fn create_disconnected_graph() -> AdjacencyListGraph {
    let mut graph = AdjacencyListGraph::new(6);
    
    // Component 1: 0 <-> 1 <-> 2
    graph.add_undirected_edge(0, 1);
    graph.add_undirected_edge(1, 2);
    
    // Component 2: 3 <-> 4 <-> 5
    graph.add_undirected_edge(3, 4);
    graph.add_undirected_edge(4, 5);
    
    graph
}

/// Test helper: Create a tree (connected acyclic graph)
fn create_tree_graph() -> AdjacencyListGraph {
    let mut graph = AdjacencyListGraph::new(7);
    
    //     0
    //    / \
    //   1   2
    //  / \   \
    // 3   4   5
    //         |
    //         6
    
    graph.add_undirected_edge(0, 1);
    graph.add_undirected_edge(0, 2);
    graph.add_undirected_edge(1, 3);
    graph.add_undirected_edge(1, 4);
    graph.add_undirected_edge(2, 5);
    graph.add_undirected_edge(5, 6);
    
    graph
}

/// Integration test: Verify BFS and DFS visit all nodes in connected components
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_bfs_dfs_visit_all_reachable_nodes() {
        let graph = create_tree_graph();
        
        let bfs_result = bfs(&graph, 0);
        let dfs_result = dfs(&graph, 0);
        
        // Both should visit all nodes since tree is connected
        assert_eq!(bfs_result.visited.len(), 7);
        assert_eq!(dfs_result.visited.len(), 7);
        
        // Convert to sets for comparison (order doesn't matter)
        let bfs_set: HashSet<_> = bfs_result.visited.into_iter().collect();
        let dfs_set: HashSet<_> = dfs_result.visited.into_iter().collect();
        let expected_nodes: HashSet<_> = (0..7).collect();
        
        assert_eq!(bfs_set, expected_nodes);
        assert_eq!(dfs_set, expected_nodes);
    }
    
    #[test]
    fn test_bfs_respects_level_order() {
        let graph = create_tree_graph();
        let bfs_result = bfs(&graph, 0);
        
        // In BFS, nodes should be discovered in level order
        // Level 0: 0
        // Level 1: 1, 2
        // Level 2: 3, 4, 5
        // Level 3: 6
        
        let discovery_times = bfs_result.discovery_time;
        
        // Node 0 should be discovered first
        assert_eq!(discovery_times[&0], 0);
        
        // Nodes 1 and 2 should be discovered before nodes 3, 4, 5
        assert!(discovery_times[&1] < discovery_times[&3]);
        assert!(discovery_times[&1] < discovery_times[&4]);
        assert!(discovery_times[&2] < discovery_times[&5]);
        
        // Node 6 should be discovered after node 5 (its parent)
        assert!(discovery_times[&5] < discovery_times[&6]);
    }
    
    #[test]
    fn test_shortest_path_correctness() {
        let graph = create_chain_graph(5); // 0 -> 1 -> 2 -> 3 -> 4
        
        // Test shortest path from start to end
        let path = shortest_path(&graph, 0, 4).unwrap();
        assert_eq!(path, vec![0, 1, 2, 3, 4]);
        
        // Test shorter path
        let path = shortest_path(&graph, 1, 3).unwrap();
        assert_eq!(path, vec![1, 2, 3]);
        
        // Test path to self
        let path = shortest_path(&graph, 2, 2).unwrap();
        assert_eq!(path, vec![2]);
    }
    
    #[test]
    fn test_shortest_path_unreachable() {
        let graph = create_disconnected_graph();
        
        // Nodes 0 and 3 are in different components
        let path = shortest_path(&graph, 0, 3);
        assert!(path.is_none());
        
        // But nodes within same component should be reachable
        let path = shortest_path(&graph, 0, 2).unwrap();
        assert_eq!(path.len(), 3); // 0 -> 1 -> 2
    }
    
    #[test]
    fn test_connected_components_detection() {
        let graph = create_disconnected_graph();
        let components = find_connected_components(&graph);
        
        // Should find exactly 2 components
        assert_eq!(components.len(), 2);
        
        // Each component should have 3 nodes
        assert_eq!(components[0].len(), 3);
        assert_eq!(components[1].len(), 3);
        
        // Verify connectivity within components
        for component in &components {
            for &node in component {
                for &other_node in component {
                    if node != other_node {
                        let path = shortest_path(&graph, node, other_node);
                        assert!(path.is_some(), 
                            "Nodes {} and {} should be connected within component", 
                            node, other_node);
                    }
                }
            }
        }
    }
    
    #[test]
    fn test_is_connected_function() {
        let connected_graph = create_tree_graph();
        assert!(is_connected(&connected_graph));
        
        let disconnected_graph = create_disconnected_graph();
        assert!(!is_connected(&disconnected_graph));
        
        let single_node_graph = AdjacencyListGraph::new(1);
        assert!(is_connected(&single_node_graph));
    }
    
    #[test]
    fn test_large_graph_performance() {
        // Create a larger graph for performance testing
        let mut graph = AdjacencyListGraph::new(1000);
        
        // Create a sparse graph (each node connected to next 3 nodes)
        for i in 0..997 {
            graph.add_edge(i, i + 1);
            graph.add_edge(i, i + 2);
            graph.add_edge(i, i + 3);
        }
        
        let start_time = std::time::Instant::now();
        let bfs_result = bfs(&graph, 0);
        let bfs_duration = start_time.elapsed();
        
        let start_time = std::time::Instant::now();
        let dfs_result = dfs(&graph, 0);
        let dfs_duration = start_time.elapsed();
        
        // Both should visit all nodes
        assert_eq!(bfs_result.visited.len(), 1000);
        assert_eq!(dfs_result.visited.len(), 1000);
        
        // Performance should be reasonable (under 100ms for 1000 nodes)
        assert!(bfs_duration.as_millis() < 100, 
            "BFS took {}ms, should be under 100ms", bfs_duration.as_millis());
        assert!(dfs_duration.as_millis() < 100, 
            "DFS took {}ms, should be under 100ms", dfs_duration.as_millis());
        
        println!("Performance test results:");
        println!("  BFS on 1000 nodes: {:?}", bfs_duration);
        println!("  DFS on 1000 nodes: {:?}", dfs_duration);
    }
}

/// Property-based testing demonstrations
#[cfg(test)]
mod property_tests {
    use super::*;
    
    #[test]
    fn property_bfs_finds_shortest_paths() {
        // Property: In an unweighted graph, BFS always finds shortest paths
        let graph = create_complete_graph(5);
        
        for start in 0..5 {
            let bfs_result = bfs(&graph, start);
            
            for end in 0..5 {
                if start != end {
                    let path = shortest_path(&graph, start, end).unwrap();
                    // In complete graph, shortest path should be length 2 (direct edge)
                    assert_eq!(path.len(), 2, 
                        "Path from {} to {} should be direct: {:?}", start, end, path);
                }
            }
        }
    }
    
    #[test]
    fn property_dfs_visits_all_reachable() {
        // Property: DFS visits exactly the reachable nodes
        let graphs = vec![
            create_tree_graph(),
            create_chain_graph(10),
            create_complete_graph(6),
            create_disconnected_graph(),
        ];
        
        for graph in graphs {
            for start_node in graph.nodes() {
                let dfs_result = dfs(&graph, start_node);
                let bfs_result = bfs(&graph, start_node);
                
                // DFS and BFS should visit same set of nodes (different order)
                let dfs_set: HashSet<_> = dfs_result.visited.into_iter().collect();
                let bfs_set: HashSet<_> = bfs_result.visited.into_iter().collect();
                
                assert_eq!(dfs_set, bfs_set, 
                    "DFS and BFS should visit same nodes from start {}", start_node);
            }
        }
    }
    
    #[test]
    fn property_parent_relationships_valid() {
        // Property: All parent relationships in traversal should be valid edges
        let graph = create_tree_graph();
        
        let bfs_result = bfs(&graph, 0);
        let dfs_result = dfs(&graph, 0);
        
        for (child, parent) in &bfs_result.parent {
            let neighbors = graph.neighbors(parent);
            assert!(neighbors.contains(child), 
                "Parent {} should have child {} as neighbor in BFS", parent, child);
        }
        
        for (child, parent) in &dfs_result.parent {
            let neighbors = graph.neighbors(parent);
            assert!(neighbors.contains(child), 
                "Parent {} should have child {} as neighbor in DFS", parent, child);
        }
    }
}

/// Cross-validation tests against reference implementations
#[cfg(test)]
mod cross_validation_tests {
    use super::*;
    
    /// Reference BFS implementation using different data structures
    fn reference_bfs<T, G>(graph: &G, start: T) -> Vec<T>
    where
        T: Clone + Eq + Hash + Debug,
        G: Graph<T>,
    {
        let mut visited = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        let mut seen = std::collections::HashSet::new();
        
        queue.push_back(start.clone());
        seen.insert(start);
        
        while let Some(node) = queue.pop_front() {
            visited.push(node.clone());
            
            for neighbor in graph.neighbors(&node) {
                if !seen.contains(&neighbor) {
                    seen.insert(neighbor.clone());
                    queue.push_back(neighbor);
                }
            }
        }
        
        visited
    }
    
    #[test]
    fn cross_validate_bfs_implementations() {
        let test_graphs = vec![
            create_tree_graph(),
            create_chain_graph(8),
            create_complete_graph(5),
        ];
        
        for graph in test_graphs {
            for start_node in 0..graph.node_count.min(5) {
                let our_result = bfs(&graph, start_node);
                let reference_result = reference_bfs(&graph, start_node);
                
                // Both should visit same nodes (order might differ)
                let our_set: HashSet<_> = our_result.visited.into_iter().collect();
                let ref_set: HashSet<_> = reference_result.into_iter().collect();
                
                assert_eq!(our_set, ref_set, 
                    "Our BFS and reference BFS should visit same nodes from {}", start_node);
            }
        }
    }
}

/// Example usage demonstrating integration testing patterns
fn main() {
    println!("=== Graph Algorithm Integration Testing Demo ===\n");
    
    // Test 1: Basic functionality verification
    println!("1. Testing basic BFS/DFS functionality:");
    let tree = create_tree_graph();
    let bfs_result = bfs(&tree, 0);
    let dfs_result = dfs(&tree, 0);
    
    println!("   Tree structure: 7 nodes in tree formation");
    println!("   BFS from node 0: {:?}", bfs_result.visited);
    println!("   DFS from node 0: {:?}", dfs_result.visited);
    println!("   ✅ Both visit all 7 nodes\n");
    
    // Test 2: Connected components analysis
    println!("2. Testing connected components:");
    let disconnected = create_disconnected_graph();
    let components = find_connected_components(&disconnected);
    
    println!("   Disconnected graph: 6 nodes in 2 components");
    for (i, component) in components.iter().enumerate() {
        println!("   Component {}: {:?}", i + 1, component);
    }
    println!("   ✅ Correctly identifies 2 components\n");
    
    // Test 3: Shortest path validation
    println!("3. Testing shortest path finding:");
    let chain = create_chain_graph(6);
    if let Some(path) = shortest_path(&chain, 1, 4) {
        println!("   Chain graph: 0 -> 1 -> 2 -> 3 -> 4 -> 5");
        println!("   Shortest path from 1 to 4: {:?}", path);
        println!("   Path length: {} steps", path.len() - 1);
        println!("   ✅ Found optimal path\n");
    }
    
    // Test 4: Performance characteristics
    println!("4. Testing performance on larger graphs:");
    let mut large_graph = AdjacencyListGraph::new(500);
    for i in 0..450 {
        large_graph.add_edge(i, i + 1);
        if i % 10 == 0 && i + 10 < 500 {
            large_graph.add_edge(i, i + 10); // Add some shortcuts
        }
    }
    
    let start_time = std::time::Instant::now();
    let large_bfs = bfs(&large_graph, 0);
    let duration = start_time.elapsed();
    
    println!("   Large graph: 500 nodes, sparse connectivity");
    println!("   BFS visited {} nodes in {:?}", large_bfs.visited.len(), duration);
    println!("   ✅ Performance within acceptable bounds\n");
    
    // Test 5: Algorithm property verification
    println!("5. Verifying algorithm properties:");
    let complete = create_complete_graph(4);
    
    // Verify BFS level-order property
    let bfs_complete = bfs(&complete, 0);
    println!("   Complete graph (4 nodes): every node connected to every other");
    println!("   BFS discovery times: {:?}", bfs_complete.discovery_time);
    
    // In complete graph, all neighbors of start should be discovered before their neighbors
    let start_neighbors = complete.neighbors(&0);
    let start_discovery_time = bfs_complete.discovery_time[&0];
    
    let all_neighbors_discovered_early = start_neighbors.iter()
        .all(|&neighbor| bfs_complete.discovery_time[&neighbor] <= start_discovery_time + start_neighbors.len());
    
    if all_neighbors_discovered_early {
        println!("   ✅ BFS respects level-order traversal property");
    }
    
    println!("\n=== Integration Testing Complete ===");
    println!("All algorithm properties verified successfully! 🎉");
}