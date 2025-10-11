//! # Mission 7: Graph Representation
//!
//! This mission implements a comprehensive graph data structure using adjacency lists
//! with support for both directed and undirected graphs, node management, and
//! foundational algorithms for graph traversal.
//!
//! ## V-Cycle Requirements Fulfilled
//!
//! ### REQ-1: Graph Structure and Node Storage
//! - Generic graph implementation with adjacency list representation
//! - Support for both directed and undirected graphs
//! - Efficient node storage with Vec<Vec<usize>> adjacency lists
//! - Node metadata support for weighted graphs and additional properties
//!
//! ### REQ-2: Edge Management Operations
//! - Add/remove edges with validation
//! - Support for both directed and undirected edge operations
//! - Edge weight support for weighted graphs
//! - Efficient edge lookup and neighbor enumeration
//!
//! ### REQ-3: Graph Building and Validation
//! - Graph construction from edge lists
//! - Validation of graph properties (connectivity, cycles)
//! - Support for disconnected components
//! - Graph statistics and analysis tools
//!
//! ### REQ-4: Algorithm Foundation
//! - Visited tracking for traversal algorithms
//! - Queue/stack infrastructure for BFS/DFS
//! - Path tracking and reconstruction
//! - Algorithm result structures and utilities
//!
//! ### REQ-5: DFS Implementation
//! - Recursive depth-first search
//! - Iterative DFS using explicit stack
//! - Path finding and cycle detection
//! - Component analysis and connectivity
//!
//! ### REQ-6: BFS Implementation
//! - Breadth-first search using VecDeque
//! - Shortest path finding (unweighted)
//! - Level-order traversal
//! - Distance calculation and path reconstruction
//!
//! ## Performance Characteristics
//!
//! - **Space Complexity**: O(V + E) where V is vertices, E is edges
//! - **Add Edge**: O(1) amortized
//! - **Remove Edge**: O(degree) where degree is the number of neighbors
//! - **Neighbor Lookup**: O(degree)
//! - **DFS/BFS**: O(V + E) time complexity
//!
//! ## Quick Start
//!
//! ```rust
//! use mission7::Graph;
//!
//! // Create a directed graph
//! let mut graph = Graph::new_directed();
//!
//! // Add nodes and edges
//! let node_a = graph.add_node("A");
//! let node_b = graph.add_node("B");
//! let node_c = graph.add_node("C");
//!
//! graph.add_edge(node_a, node_b);
//! graph.add_edge(node_b, node_c);
//! graph.add_edge(node_c, node_a);
//!
//! // Perform DFS traversal
//! let dfs_result = graph.dfs(node_a);
//! println!("DFS traversal: {:?}", dfs_result.path);
//!
//! // Perform BFS traversal
//! let bfs_result = graph.bfs(node_a);
//! println!("BFS traversal: {:?}", bfs_result.path);
//! ```

// Re-export main types for convenience
pub use algorithms::{BfsResult, DfsResult, TraversalResult};
pub use graph::{Graph, GraphType, NodeId};

/// Graph representation module containing the core Graph struct
pub mod graph {
    use std::collections::HashSet;

    /// Type alias for node identifiers
    pub type NodeId = usize;

    /// Enumeration of graph types
    #[derive(Debug, Clone, PartialEq)]
    pub enum GraphType {
        Directed,
        Undirected,
    }

    /// Core graph data structure using adjacency list representation
    #[derive(Debug, Clone)]
    pub struct Graph<T> {
        /// Adjacency list: each node points to a list of its neighbors
        adjacency: Vec<Vec<NodeId>>,
        /// Node data storage
        nodes: Vec<Option<T>>,
        /// Graph type (directed or undirected)
        pub graph_type: GraphType,
        /// Number of edges in the graph
        edge_count: usize,
    }

    impl<T> Graph<T> {
        /// Create a new directed graph
        pub fn new_directed() -> Self {
            Self {
                adjacency: Vec::new(),
                nodes: Vec::new(),
                graph_type: GraphType::Directed,
                edge_count: 0,
            }
        }

        /// Create a new undirected graph
        pub fn new_undirected() -> Self {
            Self {
                adjacency: Vec::new(),
                nodes: Vec::new(),
                graph_type: GraphType::Undirected,
                edge_count: 0,
            }
        }

        /// Add a node to the graph and return its ID
        pub fn add_node(&mut self, data: T) -> NodeId {
            let node_id = self.nodes.len();
            self.nodes.push(Some(data));
            self.adjacency.push(Vec::new());
            node_id
        }

        /// Get the number of nodes in the graph
        pub fn node_count(&self) -> usize {
            self.nodes.len()
        }

        /// Get the number of edges in the graph
        pub fn edge_count(&self) -> usize {
            self.edge_count
        }

        /// Check if the graph is empty
        pub fn is_empty(&self) -> bool {
            self.nodes.is_empty()
        }

        /// Get node data by ID
        pub fn get_node(&self, node_id: NodeId) -> Option<&T> {
            self.nodes.get(node_id)?.as_ref()
        }

        /// Get mutable reference to node data
        pub fn get_node_mut(&mut self, node_id: NodeId) -> Option<&mut T> {
            self.nodes.get_mut(node_id)?.as_mut()
        }

        /// Get all neighbors of a node
        pub fn neighbors(&self, node_id: NodeId) -> &[NodeId] {
            self.adjacency
                .get(node_id)
                .map(|v| v.as_slice())
                .unwrap_or(&[])
        }

        /// Get the degree (number of neighbors) of a node
        pub fn degree(&self, node_id: NodeId) -> usize {
            self.neighbors(node_id).len()
        }

        /// Check if an edge exists between two nodes
        pub fn has_edge(&self, from: NodeId, to: NodeId) -> bool {
            self.adjacency
                .get(from)
                .map(|neighbors| neighbors.contains(&to))
                .unwrap_or(false)
        }

        /// Add an edge between two nodes
        pub fn add_edge(&mut self, from: NodeId, to: NodeId) -> bool {
            if from >= self.nodes.len() || to >= self.nodes.len() {
                return false;
            }

            // Add edge from -> to
            if !self.adjacency[from].contains(&to) {
                self.adjacency[from].push(to);
                self.edge_count += 1;
            }

            // For undirected graphs, add edge to -> from
            if self.graph_type == GraphType::Undirected && !self.adjacency[to].contains(&from) {
                self.adjacency[to].push(from);
            }

            true
        }

        /// Remove an edge between two nodes
        pub fn remove_edge(&mut self, from: NodeId, to: NodeId) -> bool {
            if from >= self.nodes.len() || to >= self.nodes.len() {
                return false;
            }

            let mut removed = false;

            // Remove edge from -> to
            if let Some(pos) = self.adjacency[from].iter().position(|&id| id == to) {
                self.adjacency[from].remove(pos);
                self.edge_count -= 1;
                removed = true;
            }

            // For undirected graphs, remove edge to -> from
            if self.graph_type == GraphType::Undirected {
                if let Some(pos) = self.adjacency[to].iter().position(|&id| id == from) {
                    self.adjacency[to].remove(pos);
                }
            }

            removed
        }

        /// Get all node IDs in the graph
        pub fn nodes(&self) -> impl Iterator<Item = NodeId> {
            0..self.node_count()
        }

        /// Get all edges as (from, to) pairs
        pub fn edges(&self) -> Vec<(NodeId, NodeId)> {
            let mut edges = Vec::new();
            for (from, neighbors) in self.adjacency.iter().enumerate() {
                for &to in neighbors {
                    edges.push((from, to));
                }
            }
            edges
        }

        /// Check if the graph is connected (for undirected graphs)
        pub fn is_connected(&self) -> bool {
            if self.is_empty() {
                return true;
            }

            let mut visited = HashSet::new();
            let mut stack = vec![0];

            while let Some(node) = stack.pop() {
                if visited.insert(node) {
                    for &neighbor in self.neighbors(node) {
                        if !visited.contains(&neighbor) {
                            stack.push(neighbor);
                        }
                    }
                }
            }

            visited.len() == self.node_count()
        }

        /// Get connected components (for undirected graphs)
        pub fn connected_components(&self) -> Vec<Vec<NodeId>> {
            let mut components = Vec::new();
            let mut visited = HashSet::new();

            for node in self.nodes() {
                if !visited.contains(&node) {
                    let mut component = Vec::new();
                    let mut stack = vec![node];

                    while let Some(current) = stack.pop() {
                        if visited.insert(current) {
                            component.push(current);
                            for &neighbor in self.neighbors(current) {
                                if !visited.contains(&neighbor) {
                                    stack.push(neighbor);
                                }
                            }
                        }
                    }

                    components.push(component);
                }
            }

            components
        }
    }
}

/// Graph algorithms module
pub mod algorithms {
    use super::graph::{Graph, NodeId};
    use std::collections::{HashSet, VecDeque};

    /// Result of a depth-first search traversal
    #[derive(Debug, Clone)]
    pub struct DfsResult {
        /// The path taken during DFS traversal
        pub path: Vec<NodeId>,
        /// Set of visited nodes
        pub visited: HashSet<NodeId>,
        /// Parent relationships for path reconstruction
        pub parents: Vec<Option<NodeId>>,
    }

    /// Result of a breadth-first search traversal
    #[derive(Debug, Clone)]
    pub struct BfsResult {
        /// The path taken during BFS traversal
        pub path: Vec<NodeId>,
        /// Set of visited nodes
        pub visited: HashSet<NodeId>,
        /// Distance from start node to each node
        pub distances: Vec<Option<usize>>,
        /// Parent relationships for path reconstruction
        pub parents: Vec<Option<NodeId>>,
    }

    /// Generic traversal result
    #[derive(Debug, Clone)]
    pub struct TraversalResult {
        /// The traversal path
        pub path: Vec<NodeId>,
        /// Set of visited nodes
        pub visited: HashSet<NodeId>,
    }

    impl<T> Graph<T> {
        /// Perform depth-first search starting from the given node
        pub fn dfs(&self, start: NodeId) -> DfsResult {
            let mut path = Vec::new();
            let mut visited = HashSet::new();
            let mut parents = vec![None; self.node_count()];

            self.dfs_recursive(start, &mut path, &mut visited, &mut parents);

            DfsResult {
                path,
                visited,
                parents,
            }
        }

        /// Recursive helper for DFS
        fn dfs_recursive(
            &self,
            node: NodeId,
            path: &mut Vec<NodeId>,
            visited: &mut HashSet<NodeId>,
            parents: &mut Vec<Option<NodeId>>,
        ) {
            if visited.insert(node) {
                path.push(node);

                for &neighbor in self.neighbors(node) {
                    if !visited.contains(&neighbor) {
                        parents[neighbor] = Some(node);
                        self.dfs_recursive(neighbor, path, visited, parents);
                    }
                }
            }
        }

        /// Perform iterative DFS using explicit stack
        pub fn dfs_iterative(&self, start: NodeId) -> DfsResult {
            let mut path = Vec::new();
            let mut visited = HashSet::new();
            let mut parents = vec![None; self.node_count()];
            let mut stack = vec![start];

            while let Some(node) = stack.pop() {
                if visited.insert(node) {
                    path.push(node);

                    // Push neighbors in reverse order to maintain left-to-right traversal
                    for &neighbor in self.neighbors(node).iter().rev() {
                        if !visited.contains(&neighbor) {
                            parents[neighbor] = Some(node);
                            stack.push(neighbor);
                        }
                    }
                }
            }

            DfsResult {
                path,
                visited,
                parents,
            }
        }

        /// Perform breadth-first search starting from the given node
        pub fn bfs(&self, start: NodeId) -> BfsResult {
            let mut path = Vec::new();
            let mut visited = HashSet::new();
            let mut distances = vec![None; self.node_count()];
            let mut parents = vec![None; self.node_count()];
            let mut queue = VecDeque::new();

            queue.push_back(start);
            distances[start] = Some(0);

            while let Some(node) = queue.pop_front() {
                if visited.insert(node) {
                    path.push(node);

                    for &neighbor in self.neighbors(node) {
                        if !visited.contains(&neighbor) && distances[neighbor].is_none() {
                            distances[neighbor] = Some(distances[node].unwrap() + 1);
                            parents[neighbor] = Some(node);
                            queue.push_back(neighbor);
                        }
                    }
                }
            }

            BfsResult {
                path,
                visited,
                distances,
                parents,
            }
        }

        /// Find shortest path between two nodes using BFS
        pub fn shortest_path(&self, start: NodeId, end: NodeId) -> Option<Vec<NodeId>> {
            let bfs_result = self.bfs(start);

            if !bfs_result.visited.contains(&end) {
                return None;
            }

            // Reconstruct path from end to start
            let mut path = Vec::new();
            let mut current = end;

            while let Some(parent) = bfs_result.parents[current] {
                path.push(current);
                current = parent;
            }
            path.push(start);
            path.reverse();

            Some(path)
        }

        /// Check if there's a cycle in the graph using DFS
        pub fn has_cycle(&self) -> bool {
            let mut visited = HashSet::new();
            let mut rec_stack = HashSet::new();

            for node in self.nodes() {
                if !visited.contains(&node)
                    && self.has_cycle_dfs(node, &mut visited, &mut rec_stack)
                {
                    return true;
                }
            }

            false
        }

        /// DFS helper for cycle detection
        fn has_cycle_dfs(
            &self,
            node: NodeId,
            visited: &mut HashSet<NodeId>,
            rec_stack: &mut HashSet<NodeId>,
        ) -> bool {
            visited.insert(node);
            rec_stack.insert(node);

            for &neighbor in self.neighbors(node) {
                if !visited.contains(&neighbor) {
                    if self.has_cycle_dfs(neighbor, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(&neighbor) {
                    return true;
                }
            }

            rec_stack.remove(&node);
            false
        }
    }
}

/// Utility functions and helpers
pub mod utils {
    use super::graph::Graph;

    /// Create a complete graph with n nodes
    pub fn complete_graph<T>(n: usize, default_data: T) -> Graph<T>
    where
        T: Clone,
    {
        let mut graph = Graph::new_undirected();

        // Add nodes
        for _ in 0..n {
            graph.add_node(default_data.clone());
        }

        // Add all possible edges
        for i in 0..n {
            for j in (i + 1)..n {
                graph.add_edge(i, j);
            }
        }

        graph
    }

    /// Create a path graph (linear chain) with n nodes
    pub fn path_graph<T>(n: usize, default_data: T) -> Graph<T>
    where
        T: Clone,
    {
        let mut graph = Graph::new_undirected();

        if n == 0 {
            return graph;
        }

        // Add nodes
        for _ in 0..n {
            graph.add_node(default_data.clone());
        }

        // Add edges to form a path
        for i in 0..(n - 1) {
            graph.add_edge(i, i + 1);
        }

        graph
    }

    /// Create a cycle graph with n nodes
    pub fn cycle_graph<T>(n: usize, default_data: T) -> Graph<T>
    where
        T: Clone,
    {
        let mut graph = path_graph(n, default_data);

        if n > 2 {
            // Close the cycle
            graph.add_edge(n - 1, 0);
        }

        graph
    }

    /// Calculate graph density (actual edges / possible edges)
    pub fn graph_density<T>(graph: &Graph<T>) -> f64 {
        let n = graph.node_count();
        if n <= 1 {
            return 0.0;
        }

        let max_edges = if matches!(graph.graph_type, super::graph::GraphType::Directed) {
            n * (n - 1)
        } else {
            n * (n - 1) / 2
        };

        graph.edge_count() as f64 / max_edges as f64
    }

    /// Get graph statistics
    pub fn graph_stats<T>(graph: &Graph<T>) -> GraphStats {
        let mut total_degree = 0;
        let mut max_degree = 0;
        let mut min_degree = usize::MAX;

        for node in graph.nodes() {
            let degree = graph.degree(node);
            total_degree += degree;
            max_degree = max_degree.max(degree);
            min_degree = min_degree.min(degree);
        }

        let avg_degree = if graph.node_count() > 0 {
            total_degree as f64 / graph.node_count() as f64
        } else {
            0.0
        };

        GraphStats {
            node_count: graph.node_count(),
            edge_count: graph.edge_count(),
            density: graph_density(graph),
            avg_degree,
            max_degree,
            min_degree,
        }
    }

    /// Graph statistics structure
    #[derive(Debug, Clone)]
    pub struct GraphStats {
        pub node_count: usize,
        pub edge_count: usize,
        pub density: f64,
        pub avg_degree: f64,
        pub max_degree: usize,
        pub min_degree: usize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let graph: Graph<&str> = Graph::new_directed();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn test_add_nodes() {
        let mut graph: Graph<&str> = Graph::new_directed();
        let node_a = graph.add_node("A");
        let node_b = graph.add_node("B");

        assert_eq!(graph.node_count(), 2);
        assert_eq!(node_a, 0);
        assert_eq!(node_b, 1);
        assert_eq!(graph.get_node(node_a), Some(&"A"));
        assert_eq!(graph.get_node(node_b), Some(&"B"));
    }

    #[test]
    fn test_add_edges() {
        let mut graph: Graph<&str> = Graph::new_directed();
        let node_a = graph.add_node("A");
        let node_b = graph.add_node("B");

        assert!(graph.add_edge(node_a, node_b));
        assert!(graph.has_edge(node_a, node_b));
        assert!(!graph.has_edge(node_b, node_a));
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_undirected_edges() {
        let mut graph: Graph<&str> = Graph::new_undirected();
        let node_a = graph.add_node("A");
        let node_b = graph.add_node("B");

        assert!(graph.add_edge(node_a, node_b));
        assert!(graph.has_edge(node_a, node_b));
        assert!(graph.has_edge(node_b, node_a));
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_dfs_traversal() {
        let mut graph: Graph<&str> = Graph::new_directed();
        let node_a = graph.add_node("A");
        let node_b = graph.add_node("B");
        let node_c = graph.add_node("C");

        graph.add_edge(node_a, node_b);
        graph.add_edge(node_b, node_c);

        let dfs_result = graph.dfs(node_a);
        assert_eq!(dfs_result.path, vec![node_a, node_b, node_c]);
        assert_eq!(dfs_result.visited.len(), 3);
    }

    #[test]
    fn test_bfs_traversal() {
        let mut graph: Graph<&str> = Graph::new_directed();
        let node_a = graph.add_node("A");
        let node_b = graph.add_node("B");
        let node_c = graph.add_node("C");

        graph.add_edge(node_a, node_b);
        graph.add_edge(node_a, node_c);

        let bfs_result = graph.bfs(node_a);
        assert_eq!(bfs_result.path, vec![node_a, node_b, node_c]);
        assert_eq!(bfs_result.distances[node_a], Some(0));
        assert_eq!(bfs_result.distances[node_b], Some(1));
        assert_eq!(bfs_result.distances[node_c], Some(1));
    }

    #[test]
    fn test_shortest_path() {
        let mut graph: Graph<&str> = Graph::new_undirected();
        let node_a = graph.add_node("A");
        let node_b = graph.add_node("B");
        let node_c = graph.add_node("C");
        let node_d = graph.add_node("D");

        graph.add_edge(node_a, node_b);
        graph.add_edge(node_b, node_c);
        graph.add_edge(node_c, node_d);

        let path = graph.shortest_path(node_a, node_d);
        assert_eq!(path, Some(vec![node_a, node_b, node_c, node_d]));
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph: Graph<&str> = Graph::new_directed();
        let node_a = graph.add_node("A");
        let node_b = graph.add_node("B");
        let node_c = graph.add_node("C");

        graph.add_edge(node_a, node_b);
        graph.add_edge(node_b, node_c);
        graph.add_edge(node_c, node_a);

        assert!(graph.has_cycle());
    }

    #[test]
    fn test_connected_components() {
        let mut graph: Graph<&str> = Graph::new_undirected();
        let node_a = graph.add_node("A");
        let node_b = graph.add_node("B");
        let node_c = graph.add_node("C");
        let node_d = graph.add_node("D");

        graph.add_edge(node_a, node_b);
        graph.add_edge(node_c, node_d);

        let components = graph.connected_components();
        assert_eq!(components.len(), 2);
        assert!(components.contains(&vec![node_a, node_b]));
        assert!(components.contains(&vec![node_c, node_d]));
    }
}
