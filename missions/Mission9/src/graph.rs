//! Graph data structures and traits for pathfinding algorithms

use crate::error::PathfindingError;
use crate::{NodeId, PathfindingResult, Weight};
use std::collections::{HashMap, HashSet};

/// Core trait for weighted graphs used in pathfinding
pub trait WeightedGraph {
    /// Get all neighbors of a node with their edge weights
    fn neighbors(&self, node: NodeId) -> Vec<(NodeId, Weight)>;
    
    /// Check if a node exists in the graph
    fn contains_node(&self, node: NodeId) -> bool;
    
    /// Get all nodes in the graph
    fn nodes(&self) -> Vec<NodeId>;
    
    /// Get the number of nodes in the graph
    fn node_count(&self) -> usize;
    
    /// Get the number of edges in the graph
    fn edge_count(&self) -> usize;
    
    /// Validate the graph structure for pathfinding
    fn validate(&self) -> PathfindingResult<()> {
        if self.node_count() == 0 {
            return Err(PathfindingError::InvalidGraph);
        }
        
        // Check for negative weights (Dijkstra's algorithm requirement)
        for node in self.nodes() {
            for (_, weight) in self.neighbors(node) {
                if weight < 0.0 {
                    return Err(PathfindingError::NegativeWeights);
                }
            }
        }
        
        Ok(())
    }
}

/// Simple adjacency list implementation of a weighted graph
#[derive(Debug, Clone)]
pub struct SimpleWeightedGraph {
    /// Adjacency list: node -> [(neighbor, weight), ...]
    adjacency: HashMap<NodeId, Vec<(NodeId, Weight)>>,
    /// Set of all nodes for efficient contains checks
    nodes: HashSet<NodeId>,
    /// Total number of edges
    edge_count: usize,
}

impl SimpleWeightedGraph {
    /// Create a new empty graph
    pub fn new() -> Self {
        Self {
            adjacency: HashMap::new(),
            nodes: HashSet::new(),
            edge_count: 0,
        }
    }
    
    /// Create a graph with initial capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            adjacency: HashMap::with_capacity(capacity),
            nodes: HashSet::with_capacity(capacity),
            edge_count: 0,
        }
    }
    
    /// Add a node to the graph
    pub fn add_node(&mut self, node: NodeId) {
        if !self.nodes.contains(&node) {
            self.nodes.insert(node);
            self.adjacency.entry(node).or_default();
        }
    }
    
    /// Add a directed edge with weight
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, weight: Weight) {
        self.add_node(from);
        self.add_node(to);
        
        self.adjacency
            .entry(from)
            .or_default()
            .push((to, weight));
        
        self.edge_count += 1;
    }
    
    /// Add an undirected edge (bidirectional)
    pub fn add_undirected_edge(&mut self, from: NodeId, to: NodeId, weight: Weight) {
        self.add_edge(from, to, weight);
        self.add_edge(to, from, weight);
    }
    
    /// Remove all edges from a node
    pub fn clear_edges(&mut self, node: NodeId) {
        if let Some(edges) = self.adjacency.get(&node) {
            self.edge_count -= edges.len();
        }
        self.adjacency.entry(node).or_default().clear();
    }
    
    /// Get the weight of a specific edge
    pub fn get_edge_weight(&self, from: NodeId, to: NodeId) -> Option<Weight> {
        self.adjacency
            .get(&from)?
            .iter()
            .find(|(neighbor, _)| *neighbor == to)
            .map(|(_, weight)| *weight)
    }
    
    /// Create a graph from edge list
    pub fn from_edges(edges: &[(NodeId, NodeId, Weight)]) -> Self {
        let mut graph = Self::with_capacity(edges.len());
        
        for &(from, to, weight) in edges {
            graph.add_edge(from, to, weight);
        }
        
        graph
    }
}

impl Default for SimpleWeightedGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl WeightedGraph for SimpleWeightedGraph {
    fn neighbors(&self, node: NodeId) -> Vec<(NodeId, Weight)> {
        self.adjacency
            .get(&node).cloned()
            .unwrap_or_default()
    }
    
    fn contains_node(&self, node: NodeId) -> bool {
        self.nodes.contains(&node)
    }
    
    fn nodes(&self) -> Vec<NodeId> {
        self.nodes.iter().copied().collect()
    }
    
    fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    fn edge_count(&self) -> usize {
        self.edge_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_graph() {
        let graph = SimpleWeightedGraph::new();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.validate().is_err()); // Empty graph is invalid
    }
    
    #[test]
    fn test_single_node() {
        let mut graph = SimpleWeightedGraph::new();
        graph.add_node(0);
        
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.contains_node(0));
        assert!(graph.validate().is_ok()); // Single node is valid
    }
    
    #[test]
    fn test_simple_edge() {
        let mut graph = SimpleWeightedGraph::new();
        graph.add_edge(0, 1, 2.5);
        
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.neighbors(0), vec![(1, 2.5)]);
        assert_eq!(graph.neighbors(1), vec![]);
        assert_eq!(graph.get_edge_weight(0, 1), Some(2.5));
        assert_eq!(graph.get_edge_weight(1, 0), None);
    }
    
    #[test]
    fn test_undirected_edge() {
        let mut graph = SimpleWeightedGraph::new();
        graph.add_undirected_edge(0, 1, 3.0);
        
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 2);
        assert_eq!(graph.get_edge_weight(0, 1), Some(3.0));
        assert_eq!(graph.get_edge_weight(1, 0), Some(3.0));
    }
    
    #[test]
    fn test_negative_weight_validation() {
        let edges = vec![(0, 1, 2.0), (1, 2, -1.0)]; // Negative weight
        let graph = SimpleWeightedGraph::from_edges(&edges);
        
        assert!(graph.validate().is_err());
        if let Err(PathfindingError::NegativeWeights) = graph.validate() {
            // Expected error
        } else {
            panic!("Expected NegativeWeights error");
        }
    }
    
    #[test]
    fn test_from_edges() {
        let edges = vec![
            (0, 1, 2.0),
            (1, 2, 3.0),
            (0, 2, 5.0),
        ];
        
        let graph = SimpleWeightedGraph::from_edges(&edges);
        
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 3);
        assert!(graph.contains_node(0));
        assert!(graph.contains_node(1));
        assert!(graph.contains_node(2));
        assert!(graph.validate().is_ok());
    }
}