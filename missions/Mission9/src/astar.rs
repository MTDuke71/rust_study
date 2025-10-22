//! A* (A-star) pathfinding algorithm implementation

use crate::error::PathfindingError;
use crate::graph::WeightedGraph;
use crate::heuristic::{Heuristic, HeuristicContext, EuclideanHeuristic};
use crate::pathfinder::{PathResult, Pathfinder, PathfindingConfig};
use crate::priority_queue::{BinaryHeapQueue, PathfindingQueue, QueueNode};
use crate::{NodeId, PathfindingResult, Weight};
use std::collections::HashMap;
use std::time::Instant;

/// A* algorithm pathfinder implementation
/// 
/// Uses heuristic guidance to find optimal paths more efficiently than Dijkstra.
/// Requires admissible heuristic for optimality guarantee.
/// Time complexity: O(b^d) where b = branching factor, d = solution depth
/// Space complexity: O(b^d)
#[derive(Debug, Clone)]
pub struct AstarPathfinder<H: Heuristic> {
    heuristic: H,
    config: PathfindingConfig,
}

impl<H: Heuristic> AstarPathfinder<H> {
    /// Create a new A* pathfinder with specified heuristic
    pub fn new(heuristic: H) -> Self {
        Self {
            heuristic,
            config: PathfindingConfig::default(),
        }
    }
    
    /// Create a new A* pathfinder with custom configuration
    pub fn with_config(heuristic: H, config: PathfindingConfig) -> Self {
        Self { heuristic, config }
    }
    
    /// Get reference to the heuristic function
    pub fn heuristic(&self) -> &H {
        &self.heuristic
    }
    
    /// Internal implementation of A* algorithm
    fn astar_internal<G: WeightedGraph>(
        &self,
        graph: &G,
        start: NodeId,
        goal: NodeId,
        context: &HeuristicContext,
    ) -> PathfindingResult<PathResult> {
        let start_time = Instant::now();
        
        // Initialize data structures
        let mut open_set = BinaryHeapQueue::new();
        let mut g_scores: HashMap<NodeId, Weight> = HashMap::new();
        let mut f_scores: HashMap<NodeId, Weight> = HashMap::new();
        let mut predecessors: HashMap<NodeId, NodeId> = HashMap::new();
        let mut closed_set: std::collections::HashSet<NodeId> = std::collections::HashSet::new();
        let mut nodes_explored = 0;
        
        // Initialize start node
        let start_h = self.heuristic.estimate_distance(start, goal, context);
        g_scores.insert(start, 0.0);
        f_scores.insert(start, start_h);
        open_set.push(start, start_h);
        
        while !open_set.is_empty() {
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
            
            let QueueNode { node: current, cost: _ } = open_set.pop().unwrap();
            
            // Skip if already processed
            if closed_set.contains(&current) {
                continue;
            }
            
            closed_set.insert(current);
            nodes_explored += 1;
            
            // Check if we've reached the goal
            if current == goal {
                let path = self.reconstruct_path(&predecessors, start, goal)?;
                let final_cost = g_scores[&goal];
                let search_time_ms = start_time.elapsed().as_millis() as u64;
                
                return Ok(PathResult::with_metrics(
                    path,
                    final_cost,
                    nodes_explored,
                    search_time_ms,
                ));
            }
            
            let current_g = g_scores[&current];
            
            // Explore neighbors
            for (neighbor, edge_weight) in graph.neighbors(current) {
                if closed_set.contains(&neighbor) {
                    continue;
                }
                
                let tentative_g = current_g + edge_weight;
                
                let is_better_path = g_scores
                    .get(&neighbor)
                    .map_or(true, |&existing_g| tentative_g < existing_g);
                
                if is_better_path {
                    // Update scores and path
                    predecessors.insert(neighbor, current);
                    g_scores.insert(neighbor, tentative_g);
                    
                    let h_cost = self.heuristic.estimate_distance(neighbor, goal, context);
                    
                    // Validate heuristic (optional safety check)
                    if h_cost.is_infinite() || h_cost.is_nan() || h_cost < 0.0 {
                        return Err(PathfindingError::InvalidHeuristic { value: h_cost });
                    }
                    
                    let f_cost = tentative_g + h_cost;
                    f_scores.insert(neighbor, f_cost);
                    open_set.push(neighbor, f_cost);
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

impl<H: Heuristic> Pathfinder for AstarPathfinder<H> {
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
        
        // Create default context (can be extended for specific use cases)
        let context = HeuristicContext::default();
        
        // Run A* algorithm
        self.astar_internal(graph, start, goal, &context)
    }
    
    fn algorithm_name(&self) -> &'static str {
        "A* Algorithm"
    }
    
    fn supports_negative_weights(&self) -> bool {
        false // A* requires non-negative weights for optimality
    }
    
    fn time_complexity(&self) -> &'static str {
        "O(b^d)"
    }
    
    fn space_complexity(&self) -> &'static str {
        "O(b^d)"
    }
}

/// A* pathfinder with context support for coordinate-based heuristics
impl<H: Heuristic> AstarPathfinder<H> {
    /// Find path with explicit heuristic context
    pub fn find_path_with_context<G: WeightedGraph>(
        &self,
        graph: &G,
        start: NodeId,
        goal: NodeId,
        context: &HeuristicContext,
    ) -> PathfindingResult<PathResult> {
        // Validate input parameters
        self.validate_input(graph, start, goal)?;
        
        // Handle trivial case
        if start == goal {
            return Ok(PathResult::with_metrics(vec![start], 0.0, 1, 0));
        }
        
        // Run A* algorithm with provided context
        self.astar_internal(graph, start, goal, context)
    }
}

/// Type alias for A* with Euclidean heuristic (common case)
pub type EuclideanAstar = AstarPathfinder<EuclideanHeuristic>;

impl EuclideanAstar {
    /// Create A* pathfinder with Euclidean heuristic
    pub fn euclidean() -> Self {
        Self::new(EuclideanHeuristic)
    }
}

/// Convenience function for single-use A* pathfinding with Euclidean heuristic
pub fn astar<G: WeightedGraph>(
    graph: &G,
    start: NodeId,
    goal: NodeId,
) -> PathfindingResult<PathResult> {
    let pathfinder = EuclideanAstar::euclidean();
    pathfinder.find_path(graph, start, goal)
}

/// Convenience function for A* with custom heuristic
pub fn astar_with_heuristic<G: WeightedGraph, H: Heuristic>(
    graph: &G,
    start: NodeId,
    goal: NodeId,
    heuristic: H,
    context: &HeuristicContext,
) -> PathfindingResult<PathResult> {
    let pathfinder = AstarPathfinder::new(heuristic);
    pathfinder.find_path_with_context(graph, start, goal, context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::SimpleWeightedGraph;
    use crate::heuristic::{ManhattanHeuristic, ZeroHeuristic};
    
    fn create_grid_graph() -> (SimpleWeightedGraph, HeuristicContext) {
        let mut graph = SimpleWeightedGraph::new();
        let mut context = HeuristicContext::new();
        
        // Create a 3x3 grid:
        // 0 - 1 - 2
        // |   |   |
        // 3 - 4 - 5
        // |   |   |
        // 6 - 7 - 8
        
        // Add coordinates for heuristic calculations
        for y in 0..3 {
            for x in 0..3 {
                let node = y * 3 + x;
                context.add_coordinates(node, x as f64, y as f64);
            }
        }
        
        // Add horizontal edges
        for y in 0..3 {
            for x in 0..2 {
                let from = y * 3 + x;
                let to = y * 3 + (x + 1);
                graph.add_undirected_edge(from, to, 1.0);
            }
        }
        
        // Add vertical edges
        for y in 0..2 {
            for x in 0..3 {
                let from = y * 3 + x;
                let to = (y + 1) * 3 + x;
                graph.add_undirected_edge(from, to, 1.0);
            }
        }
        
        (graph, context)
    }
    
    #[test]
    fn test_astar_simple_path() {
        let (graph, context) = create_grid_graph();
        let pathfinder = AstarPathfinder::new(ManhattanHeuristic);
        
        // Path from top-left (0) to bottom-right (8)
        let result = pathfinder
            .find_path_with_context(&graph, 0, 8, &context)
            .unwrap();
        
        // Should find optimal path with Manhattan distance
        assert_eq!(result.cost, 4.0); // Minimum steps in grid
        assert_eq!(result.path.len(), 5); // Start + 4 steps
        assert_eq!(result.path[0], 0);
        assert_eq!(result.path[4], 8);
    }
    
    #[test]
    fn test_astar_vs_dijkstra_efficiency() {
        let (graph, context) = create_grid_graph();
        
        // A* with good heuristic
        let astar_pathfinder = AstarPathfinder::new(ManhattanHeuristic);
        let astar_result = astar_pathfinder
            .find_path_with_context(&graph, 0, 8, &context)
            .unwrap();
        
        // A* with zero heuristic (equivalent to Dijkstra)
        let dijkstra_like = AstarPathfinder::new(ZeroHeuristic);
        let dijkstra_result = dijkstra_like.find_path(&graph, 0, 8).unwrap();
        
        // Both should find optimal path
        assert_eq!(astar_result.cost, dijkstra_result.cost);
        
        // A* should explore fewer nodes (in most cases)
        // Note: This might not always hold for small graphs
        println!("A* explored: {}", astar_result.nodes_explored);
        println!("Dijkstra-like explored: {}", dijkstra_result.nodes_explored);
    }
    
    #[test]
    fn test_astar_same_node() {
        let (graph, context) = create_grid_graph();
        let pathfinder = AstarPathfinder::new(EuclideanHeuristic);
        
        let result = pathfinder
            .find_path_with_context(&graph, 0, 0, &context)
            .unwrap();
        
        assert_eq!(result.path, vec![0]);
        assert_eq!(result.cost, 0.0);
        assert_eq!(result.nodes_explored, 1);
    }
    
    #[test]
    fn test_astar_no_path() {
        let mut graph = SimpleWeightedGraph::new();
        graph.add_node(0);
        graph.add_node(1); // Disconnected nodes
        
        let pathfinder = AstarPathfinder::new(ZeroHeuristic);
        let result = pathfinder.find_path(&graph, 0, 1);
        
        assert!(matches!(result, Err(PathfindingError::NoPathExists { .. })));
    }
    
    #[test]
    fn test_euclidean_astar_type_alias() {
        let (graph, context) = create_grid_graph();
        let pathfinder = EuclideanAstar::euclidean();
        
        let result = pathfinder
            .find_path_with_context(&graph, 0, 8, &context)
            .unwrap();
        
        assert_eq!(result.cost, 4.0);
        assert!(result.nodes_explored > 0);
    }
    
    #[test]
    fn test_astar_convenience_functions() {
        let (graph, context) = create_grid_graph();
        
        // Test basic A* convenience function
        let result1 = astar(&graph, 0, 8).unwrap();
        assert_eq!(result1.cost, 4.0);
        
        // Test A* with custom heuristic
        let result2 = astar_with_heuristic(&graph, 0, 8, ManhattanHeuristic, &context).unwrap();
        assert_eq!(result2.cost, 4.0);
    }
    
    #[test]
    fn test_pathfinder_trait_implementation() {
        let pathfinder = AstarPathfinder::new(EuclideanHeuristic);
        
        assert_eq!(pathfinder.algorithm_name(), "A* Algorithm");
        assert!(!pathfinder.supports_negative_weights());
        assert_eq!(pathfinder.time_complexity(), "O(b^d)");
        assert_eq!(pathfinder.space_complexity(), "O(b^d)");
    }
    
    #[test]
    fn test_heuristic_access() {
        let pathfinder = AstarPathfinder::new(ManhattanHeuristic);
        
        let heuristic = pathfinder.heuristic();
        assert_eq!(heuristic.name(), "Manhattan Distance");
        assert!(heuristic.is_admissible());
    }
    
    #[test]
    fn test_astar_with_timeout() {
        let (graph, context) = create_grid_graph();
        let config = PathfindingConfig::new().with_timeout(0); // Immediate timeout
        let pathfinder = AstarPathfinder::with_config(EuclideanHeuristic, config);
        
        let result = pathfinder.find_path_with_context(&graph, 0, 8, &context);
        
        // Should timeout or complete quickly
        match result {
            Err(PathfindingError::Timeout { .. }) => {
                // Expected timeout
            }
            Ok(_) => {
                // Completed before timeout - also acceptable for small graphs
            }
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }
}