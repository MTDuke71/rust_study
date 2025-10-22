//! Core pathfinder trait and path representation

use crate::error::PathfindingError;
use crate::graph::WeightedGraph;
use crate::{NodeId, PathfindingResult, Weight};

/// Result of a pathfinding operation
#[derive(Debug, Clone, PartialEq)]
pub struct PathResult {
    /// The sequence of nodes from start to goal
    pub path: Vec<NodeId>,
    /// Total cost/distance of the path
    pub cost: Weight,
    /// Number of nodes explored during search
    pub nodes_explored: usize,
    /// Time taken for the search (in milliseconds)
    pub search_time_ms: u64,
}

impl PathResult {
    /// Create a new path result
    pub fn new(path: Vec<NodeId>, cost: Weight) -> Self {
        Self {
            path,
            cost,
            nodes_explored: 0,
            search_time_ms: 0,
        }
    }
    
    /// Create a path result with performance metrics
    pub fn with_metrics(
        path: Vec<NodeId>,
        cost: Weight,
        nodes_explored: usize,
        search_time_ms: u64,
    ) -> Self {
        Self {
            path,
            cost,
            nodes_explored,
            search_time_ms,
        }
    }
    
    /// Check if this is an empty path (no solution found)
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }
    
    /// Get the number of steps in the path
    pub fn step_count(&self) -> usize {
        if self.path.len() <= 1 {
            0
        } else {
            self.path.len() - 1
        }
    }
    
    /// Get the start node (first node in path)
    pub fn start(&self) -> Option<NodeId> {
        self.path.first().copied()
    }
    
    /// Get the goal node (last node in path)
    pub fn goal(&self) -> Option<NodeId> {
        self.path.last().copied()
    }
    
    /// Calculate average cost per step
    pub fn average_step_cost(&self) -> Weight {
        let steps = self.step_count();
        if steps == 0 {
            0.0
        } else {
            self.cost / steps as Weight
        }
    }
}

/// Core trait for pathfinding algorithms
pub trait Pathfinder {
    /// Find the shortest path between two nodes
    fn find_path<G: WeightedGraph>(
        &self,
        graph: &G,
        start: NodeId,
        goal: NodeId,
    ) -> PathfindingResult<PathResult>;
    
    /// Validate input parameters before pathfinding
    fn validate_input<G: WeightedGraph>(
        &self,
        graph: &G,
        start: NodeId,
        goal: NodeId,
    ) -> PathfindingResult<()> {
        // Validate graph structure
        graph.validate()?;
        
        // Check if start node exists
        if !graph.contains_node(start) {
            return Err(PathfindingError::InvalidStartNode { node: start });
        }
        
        // Check if goal node exists
        if !graph.contains_node(goal) {
            return Err(PathfindingError::InvalidGoalNode { node: goal });
        }
        
        Ok(())
    }
    
    /// Get algorithm name for debugging/logging
    fn algorithm_name(&self) -> &'static str;
    
    /// Check if the algorithm supports negative weights
    fn supports_negative_weights(&self) -> bool {
        false // Most algorithms don't support negative weights
    }
    
    /// Get expected time complexity as a string
    fn time_complexity(&self) -> &'static str {
        "O(unknown)"
    }
    
    /// Get expected space complexity as a string  
    fn space_complexity(&self) -> &'static str {
        "O(unknown)"
    }
}

/// Configuration options for pathfinding algorithms
#[derive(Debug, Clone)]
pub struct PathfindingConfig {
    /// Maximum number of nodes to explore before giving up
    pub max_iterations: Option<usize>,
    /// Maximum time to spend searching (in seconds)
    pub timeout_seconds: Option<u64>,
    /// Whether to collect detailed metrics during search
    pub collect_metrics: bool,
    /// Whether to enable early termination optimizations
    pub enable_optimizations: bool,
}

impl Default for PathfindingConfig {
    fn default() -> Self {
        Self {
            max_iterations: None,
            timeout_seconds: None,
            collect_metrics: true,
            enable_optimizations: true,
        }
    }
}

impl PathfindingConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set maximum iterations
    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = Some(max_iterations);
        self
    }
    
    /// Set timeout in seconds
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = Some(timeout_seconds);
        self
    }
    
    /// Enable or disable metrics collection
    pub fn with_metrics(mut self, collect_metrics: bool) -> Self {
        self.collect_metrics = collect_metrics;
        self
    }
    
    /// Enable or disable optimizations
    pub fn with_optimizations(mut self, enable_optimizations: bool) -> Self {
        self.enable_optimizations = enable_optimizations;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_path_result_creation() {
        let path = vec![0, 1, 2, 3];
        let cost = 10.5;
        let result = PathResult::new(path.clone(), cost);
        
        assert_eq!(result.path, path);
        assert_eq!(result.cost, cost);
        assert_eq!(result.nodes_explored, 0);
        assert_eq!(result.search_time_ms, 0);
        assert!(!result.is_empty());
        assert_eq!(result.step_count(), 3);
        assert_eq!(result.start(), Some(0));
        assert_eq!(result.goal(), Some(3));
        assert_eq!(result.average_step_cost(), 10.5 / 3.0);
    }
    
    #[test]
    fn test_empty_path_result() {
        let result = PathResult::new(vec![], 0.0);
        
        assert!(result.is_empty());
        assert_eq!(result.step_count(), 0);
        assert_eq!(result.start(), None);
        assert_eq!(result.goal(), None);
        assert_eq!(result.average_step_cost(), 0.0);
    }
    
    #[test]
    fn test_single_node_path() {
        let result = PathResult::new(vec![0], 0.0);
        
        assert!(!result.is_empty());
        assert_eq!(result.step_count(), 0);
        assert_eq!(result.start(), Some(0));
        assert_eq!(result.goal(), Some(0));
        assert_eq!(result.average_step_cost(), 0.0);
    }
    
    #[test]
    fn test_path_result_with_metrics() {
        let path = vec![0, 1, 2];
        let cost = 5.0;
        let nodes_explored = 10;
        let search_time_ms = 50;
        
        let result = PathResult::with_metrics(path.clone(), cost, nodes_explored, search_time_ms);
        
        assert_eq!(result.path, path);
        assert_eq!(result.cost, cost);
        assert_eq!(result.nodes_explored, nodes_explored);
        assert_eq!(result.search_time_ms, search_time_ms);
    }
    
    #[test]
    fn test_pathfinding_config() {
        let config = PathfindingConfig::new()
            .with_max_iterations(1000)
            .with_timeout(30)
            .with_metrics(false)
            .with_optimizations(true);
        
        assert_eq!(config.max_iterations, Some(1000));
        assert_eq!(config.timeout_seconds, Some(30));
        assert!(!config.collect_metrics);
        assert!(config.enable_optimizations);
    }
    
    #[test]
    fn test_pathfinding_config_defaults() {
        let config = PathfindingConfig::default();
        
        assert_eq!(config.max_iterations, None);
        assert_eq!(config.timeout_seconds, None);
        assert!(config.collect_metrics);
        assert!(config.enable_optimizations);
    }
}