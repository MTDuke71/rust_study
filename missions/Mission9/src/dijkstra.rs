//! Dijkstra's shortest path algorithm implementation

use crate::error::PathfindingError;
use crate::graph::WeightedGraph;
use crate::pathfinder::{PathResult, Pathfinder, PathfindingConfig};
use crate::priority_queue::{BinaryHeapQueue, PathfindingQueue};
use crate::{NodeId, PathfindingResult, Weight};
use std::collections::HashMap;
use std::time::Instant;

/// Dijkstra's algorithm pathfinder implementation
/// 
/// Guarantees finding the shortest path in graphs with non-negative edge weights.
/// Time complexity: O((V + E) log V) where V = vertices, E = edges
/// Space complexity: O(V)
#[derive(Debug, Clone)]
pub struct DijkstraPathfinder {
    config: PathfindingConfig,
}

impl DijkstraPathfinder {
    /// Create a new Dijkstra pathfinder with default configuration
    pub fn new() -> Self {
        Self {
            config: PathfindingConfig::default(),
        }
    }
    
    /// Create a new Dijkstra pathfinder with custom configuration
    pub fn with_config(config: PathfindingConfig) -> Self {
        Self { config }
    }
    
    /// Internal implementation of Dijkstra's algorithm
    fn dijkstra_internal<G: WeightedGraph>(
        &self,
        graph: &G,
        start: NodeId,
        goal: NodeId,
    ) -> PathfindingResult<PathResult> {
        let start_time = Instant::now();
        
        // Initialize data structures
        let mut queue = BinaryHeapQueue::new();
        let mut distances: HashMap<NodeId, Weight> = HashMap::new();
        let mut predecessors: HashMap<NodeId, NodeId> = HashMap::new();
        let mut visited: std::collections::HashSet<NodeId> = std::collections::HashSet::new();
        let mut nodes_explored = 0;
        
        // Initialize start node
        distances.insert(start, 0.0);
        queue.push(start, 0.0);
        
        while !queue.is_empty() {
            // Check for timeout
            if let Some(timeout) = self.config.timeout_seconds {
                if start_time.elapsed().as_secs() >= timeout {
                    return Err(PathfindingError::Timeout { seconds: timeout });
                }
            }
            
            // Check for maximum iterations
            if let Some(max_iter) = self.config.max_iterations {
                if nodes_explored >= max_iter {
                    return Err(PathfindingError::QueueError {
                        message: format!("Exceeded maximum iterations: {}", max_iter),
                    });
                }
            }
            
            let current_node = queue.pop().unwrap();
            let current_id = current_node.node;
            let current_distance = current_node.cost;
            
            // Skip if we've already processed this node with a better distance
            if visited.contains(&current_id) {
                continue;
            }
            
            visited.insert(current_id);
            nodes_explored += 1;
            
            // Early termination if we reached the goal
            if current_id == goal {
                let path = self.reconstruct_path(&predecessors, start, goal)?;
                let search_time_ms = start_time.elapsed().as_millis() as u64;
                
                return Ok(PathResult::with_metrics(
                    path,
                    current_distance,
                    nodes_explored,
                    search_time_ms,
                ));
            }
            
            // Explore neighbors
            for (neighbor_id, edge_weight) in graph.neighbors(current_id) {
                if visited.contains(&neighbor_id) {
                    continue;
                }
                
                let new_distance = current_distance + edge_weight;
                let is_better_path = distances
                    .get(&neighbor_id)
                    .map_or(true, |&existing_distance| new_distance < existing_distance);
                
                if is_better_path {
                    distances.insert(neighbor_id, new_distance);
                    predecessors.insert(neighbor_id, current_id);
                    queue.push(neighbor_id, new_distance);
                }
            }
        }
        
        // No path found
        Err(PathfindingError::NoPathExists { start, goal })
    }
    
    /// Reconstruct the path from start to goal using predecessors
    fn reconstruct_path(
        &self,
        predecessors: &HashMap<NodeId, NodeId>,
        start: NodeId,
        goal: NodeId,
    ) -> PathfindingResult<Vec<NodeId>> {
        let mut path = Vec::new();
        let mut current = goal;
        
        // Trace back from goal to start
        while current != start {
            path.push(current);
            
            match predecessors.get(&current) {
                Some(&prev) => current = prev,
                None => {
                    return Err(PathfindingError::NoPathExists { start, goal });
                }
            }
        }
        
        path.push(start);
        path.reverse();
        
        Ok(path)
    }
}

impl Default for DijkstraPathfinder {
    fn default() -> Self {
        Self::new()
    }
}

impl Pathfinder for DijkstraPathfinder {
    fn find_path<G: WeightedGraph>(
        &self,
        graph: &G,
        start: NodeId,
        goal: NodeId,
    ) -> PathfindingResult<PathResult> {
        // Validate input parameters
        self.validate_input(graph, start, goal)?;
        
        // Handle trivial case
        if start == goal {
            return Ok(PathResult::with_metrics(vec![start], 0.0, 1, 0));
        }
        
        // Run Dijkstra's algorithm
        self.dijkstra_internal(graph, start, goal)
    }
    
    fn algorithm_name(&self) -> &'static str {
        "Dijkstra's Algorithm"
    }
    
    fn supports_negative_weights(&self) -> bool {
        false
    }
    
    fn time_complexity(&self) -> &'static str {
        "O((V + E) log V)"
    }
    
    fn space_complexity(&self) -> &'static str {
        "O(V)"
    }
}

/// Convenience function for single-use Dijkstra pathfinding
pub fn dijkstra<G: WeightedGraph>(
    graph: &G,
    start: NodeId,
    goal: NodeId,
) -> PathfindingResult<PathResult> {
    let pathfinder = DijkstraPathfinder::new();
    pathfinder.find_path(graph, start, goal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::SimpleWeightedGraph;
    
    fn create_test_graph() -> SimpleWeightedGraph {
        let mut graph = SimpleWeightedGraph::new();
        
        // Create a simple diamond graph:
        //   0
        //  / \
        // 1   2
        //  \ /
        //   3
        graph.add_edge(0, 1, 2.0);
        graph.add_edge(0, 2, 6.0);
        graph.add_edge(1, 3, 3.0);
        graph.add_edge(2, 3, 1.0);
        
        graph
    }
    
    #[test]
    fn test_dijkstra_simple_path() {
        let graph = create_test_graph();
        let pathfinder = DijkstraPathfinder::new();
        
        let result = pathfinder.find_path(&graph, 0, 3).unwrap();
        
        // Should find path 0 -> 1 -> 3 with cost 5.0
        assert_eq!(result.path, vec![0, 1, 3]);
        assert_eq!(result.cost, 5.0);
        assert!(result.nodes_explored > 0);
    }
    
    #[test]
    fn test_dijkstra_same_node() {
        let graph = create_test_graph();
        let pathfinder = DijkstraPathfinder::new();
        
        let result = pathfinder.find_path(&graph, 0, 0).unwrap();
        
        assert_eq!(result.path, vec![0]);
        assert_eq!(result.cost, 0.0);
        assert_eq!(result.nodes_explored, 1);
    }
    
    #[test]
    fn test_dijkstra_no_path() {
        let mut graph = SimpleWeightedGraph::new();
        graph.add_node(0);
        graph.add_node(1); // Disconnected nodes
        
        let pathfinder = DijkstraPathfinder::new();
        let result = pathfinder.find_path(&graph, 0, 1);
        
        assert!(matches!(result, Err(PathfindingError::NoPathExists { .. })));
    }
    
    #[test]
    fn test_dijkstra_invalid_nodes() {
        let graph = create_test_graph();
        let pathfinder = DijkstraPathfinder::new();
        
        // Invalid start node
        let result = pathfinder.find_path(&graph, 99, 0);
        assert!(matches!(result, Err(PathfindingError::InvalidStartNode { .. })));
        
        // Invalid goal node  
        let result = pathfinder.find_path(&graph, 0, 99);
        assert!(matches!(result, Err(PathfindingError::InvalidGoalNode { .. })));
    }
    
    #[test]
    fn test_dijkstra_with_timeout() {
        let graph = create_test_graph();
        let config = PathfindingConfig::new().with_timeout(0); // Immediate timeout
        let pathfinder = DijkstraPathfinder::with_config(config);
        
        let result = pathfinder.find_path(&graph, 0, 3);
        
        // Should timeout (though might complete if very fast)
        if let Err(PathfindingError::Timeout { .. }) = result {
            // Expected timeout error
        } else if let Ok(_) = result {
            // Algorithm completed before timeout - also acceptable
        } else {
            panic!("Unexpected error type");
        }
    }
    
    #[test]
    fn test_dijkstra_with_max_iterations() {
        let graph = create_test_graph();
        let config = PathfindingConfig::new().with_max_iterations(1);
        let pathfinder = DijkstraPathfinder::with_config(config);
        
        let result = pathfinder.find_path(&graph, 0, 3);
        
        // Should exceed iteration limit for non-trivial paths
        assert!(matches!(result, Err(PathfindingError::QueueError { .. })));
    }
    
    #[test]
    fn test_convenience_function() {
        let graph = create_test_graph();
        
        let result = dijkstra(&graph, 0, 3).unwrap();
        
        assert_eq!(result.path, vec![0, 1, 3]);
        assert_eq!(result.cost, 5.0);
    }
    
    #[test]
    fn test_pathfinder_trait_implementation() {
        let pathfinder = DijkstraPathfinder::new();
        
        assert_eq!(pathfinder.algorithm_name(), "Dijkstra's Algorithm");
        assert!(!pathfinder.supports_negative_weights());
        assert_eq!(pathfinder.time_complexity(), "O((V + E) log V)");
        assert_eq!(pathfinder.space_complexity(), "O(V)");
    }
}