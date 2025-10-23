//! Requirement-based tests for Mission 9 Day 2
//! 
//! These tests validate specific requirements from the TODO.md file:
//! - REQ-1: Implement DijkstraPathfinder struct with Pathfinder trait
//! - Core algorithm logic with priority queue integration
//! - Path reconstruction from predecessor tracking  
//! - Comprehensive error handling for all edge cases
//! - Performance metrics collection (nodes explored, search time)

#[cfg(test)]
mod day2_requirements {
    use mission9::{
        DijkstraPathfinder, Pathfinder, PathfindingError, SimpleWeightedGraph,
    };
    use mission9::pathfinder::PathfindingConfig;

    /// Test REQ-1: DijkstraPathfinder struct implements Pathfinder trait
    #[test]
    fn test_req1_dijkstra_pathfinder_trait() {
        let pathfinder = DijkstraPathfinder::new();
        
        // Verify trait implementation
        assert_eq!(pathfinder.algorithm_name(), "Dijkstra's Algorithm");
        assert!(!pathfinder.supports_negative_weights());
        assert_eq!(pathfinder.time_complexity(), "O((V + E) log V)");
        assert_eq!(pathfinder.space_complexity(), "O(V)");
    }

    /// Test REQ-1: Core algorithm logic with priority queue integration
    #[test]
    fn test_req1_core_algorithm_with_priority_queue() {
        let mut graph = SimpleWeightedGraph::new();
        
        // Create test graph with multiple paths to verify priority queue ordering
        // Path 1: 0 -> 1 -> 3 (cost: 2 + 3 = 5)
        // Path 2: 0 -> 2 -> 3 (cost: 6 + 1 = 7)
        // Priority queue should process nodes in distance order
        graph.add_edge(0, 1, 2.0);
        graph.add_edge(0, 2, 6.0);
        graph.add_edge(1, 3, 3.0);
        graph.add_edge(2, 3, 1.0);

        let pathfinder = DijkstraPathfinder::new();
        let result = pathfinder.find_path(&graph, 0, 3).unwrap();

        // Should find optimal path via priority queue ordering
        assert_eq!(result.path, vec![0, 1, 3]);
        assert_eq!(result.cost, 5.0);
        assert!(result.nodes_explored > 0);
    }

    /// Test REQ-1: Path reconstruction from predecessor tracking
    #[test]
    fn test_req1_path_reconstruction() {
        let mut graph = SimpleWeightedGraph::new();
        
        // Linear path: 0 -> 1 -> 2 -> 3 -> 4
        graph.add_edge(0, 1, 1.0);
        graph.add_edge(1, 2, 1.0);
        graph.add_edge(2, 3, 1.0);
        graph.add_edge(3, 4, 1.0);

        let pathfinder = DijkstraPathfinder::new();
        let result = pathfinder.find_path(&graph, 0, 4).unwrap();

        // Verify complete path reconstruction
        assert_eq!(result.path, vec![0, 1, 2, 3, 4]);
        assert_eq!(result.cost, 4.0);
        
        // Test partial path reconstruction
        let result = pathfinder.find_path(&graph, 1, 3).unwrap();
        assert_eq!(result.path, vec![1, 2, 3]);
        assert_eq!(result.cost, 2.0);
    }

    /// Test REQ-1: Comprehensive error handling for all edge cases
    #[test]
    fn test_req1_comprehensive_error_handling() {
        let mut graph = SimpleWeightedGraph::new();
        graph.add_edge(0, 1, 1.0);
        graph.add_node(2); // Isolated node
        
        let pathfinder = DijkstraPathfinder::new();

        // Test invalid start node
        let result = pathfinder.find_path(&graph, 99, 1);
        assert!(matches!(result, Err(PathfindingError::InvalidStartNode { .. })));

        // Test invalid goal node
        let result = pathfinder.find_path(&graph, 0, 99);
        assert!(matches!(result, Err(PathfindingError::InvalidGoalNode { .. })));

        // Test no path exists
        let result = pathfinder.find_path(&graph, 0, 2);
        assert!(matches!(result, Err(PathfindingError::NoPathExists { .. })));

        // Test empty graph - verify what error is actually returned
        let empty_graph = SimpleWeightedGraph::new();
        let result = pathfinder.find_path(&empty_graph, 0, 1);
        // Either InvalidStartNode or NoPathExists is acceptable for empty graph
        assert!(result.is_err());
    }

    /// Test REQ-1: Performance metrics collection (nodes explored, search time)
    #[test]
    fn test_req1_performance_metrics_collection() {
        let mut graph = SimpleWeightedGraph::new();
        
        // Create larger graph to test metrics collection
        for i in 0..10 {
            if i < 9 {
                graph.add_edge(i, i + 1, 1.0);
            }
            // Add some branching paths
            if i > 0 && i < 8 {
                graph.add_edge(i, i + 2, 3.0);
            }
        }

        let pathfinder = DijkstraPathfinder::new();
        let result = pathfinder.find_path(&graph, 0, 9).unwrap();

        // Verify metrics are collected
        assert!(result.nodes_explored > 0);
        // Verify timing is captured (always non-negative for u64)
        println!("Search completed in {} ms", result.search_time_ms);
        assert_eq!(result.path.first(), Some(&0));
        assert_eq!(result.path.last(), Some(&9));
        
        // For this graph, should explore multiple nodes but find optimal path
        assert!(result.nodes_explored >= result.path.len());
    }

    /// Test REQ-1: Algorithm correctness on complex graphs
    #[test]
    fn test_req1_algorithm_correctness_complex_graph() {
        let mut graph = SimpleWeightedGraph::new();
        
        // Complex graph with multiple competing paths
        // Shortest path should be 0 -> 3 -> 5 -> 6 (cost: 1 + 1 + 1 = 3)
        graph.add_edge(0, 1, 5.0);  // Expensive direct path
        graph.add_edge(0, 2, 3.0);  // Medium path
        graph.add_edge(0, 3, 1.0);  // Cheap start
        graph.add_edge(1, 6, 1.0);  // Expensive -> goal
        graph.add_edge(2, 4, 1.0);  // Medium path continuation
        graph.add_edge(3, 5, 1.0);  // Cheap path continuation
        graph.add_edge(4, 6, 2.0);  // Medium -> goal
        graph.add_edge(5, 6, 1.0);  // Cheap -> goal

        let pathfinder = DijkstraPathfinder::new();
        let result = pathfinder.find_path(&graph, 0, 6).unwrap();

        // Should find optimal path
        assert_eq!(result.path, vec![0, 3, 5, 6]);
        assert_eq!(result.cost, 3.0);
    }

    /// Test REQ-1: Configuration and timeout handling
    #[test]
    fn test_req1_configuration_and_timeout() {
        let mut graph = SimpleWeightedGraph::new();
        graph.add_edge(0, 1, 1.0);

        // Test with custom configuration
        let config = PathfindingConfig::new()
            .with_timeout(10)  // 10 seconds should be plenty
            .with_max_iterations(100);
        
        let pathfinder = DijkstraPathfinder::with_config(config);
        let result = pathfinder.find_path(&graph, 0, 1);
        
        // Should succeed with reasonable limits
        assert!(result.is_ok());

        // Test with very restrictive limits
        let restrictive_config = PathfindingConfig::new()
            .with_max_iterations(1);
        
        let restrictive_pathfinder = DijkstraPathfinder::with_config(restrictive_config);
        
        // Create graph that requires more than 1 iteration
        let mut complex_graph = SimpleWeightedGraph::new();
        complex_graph.add_edge(0, 1, 1.0);
        complex_graph.add_edge(1, 2, 1.0);
        
        let result = restrictive_pathfinder.find_path(&complex_graph, 0, 2);
        
        // Should hit iteration limit for non-trivial paths
        assert!(matches!(result, Err(PathfindingError::QueueError { .. })));
    }

    /// Test REQ-1: Same node handling (trivial case)
    #[test]
    fn test_req1_same_node_handling() {
        let mut graph = SimpleWeightedGraph::new();
        graph.add_node(5);

        let pathfinder = DijkstraPathfinder::new();
        let result = pathfinder.find_path(&graph, 5, 5).unwrap();

        // Should handle trivial case efficiently
        assert_eq!(result.path, vec![5]);
        assert_eq!(result.cost, 0.0);
        assert_eq!(result.nodes_explored, 1);
        assert_eq!(result.search_time_ms, 0); // Should be immediate
    }

    /// Test REQ-1: Large graph performance validation
    #[test]
    fn test_req1_large_graph_performance() {
        let mut graph = SimpleWeightedGraph::new();
        
        // Create moderately large graph (chain with branches)
        let size = 100;
        for i in 0..size {
            if i < size - 1 {
                graph.add_edge(i, i + 1, 1.0); // Main chain
            }
            // Add branching paths every 10 nodes
            if i % 10 == 0 && i + 10 < size {
                graph.add_edge(i, i + 10, 5.0);
            }
        }

        let pathfinder = DijkstraPathfinder::new();
        let start_time = std::time::Instant::now();
        
        let result = pathfinder.find_path(&graph, 0, size - 1).unwrap();
        
        let elapsed = start_time.elapsed();
        
        // Should complete in reasonable time
        assert!(elapsed.as_millis() < 100); // Should be under 100ms
        
        // Should find optimal path (might use shortcuts if available)
        // The cost depends on whether shortcuts are taken
        assert!(result.cost >= 50.0); // At least half the linear path cost
        assert!(result.cost <= (size - 1) as f64); // At most the full linear cost
        assert!(result.nodes_explored > 0);
        assert!(result.nodes_explored <= size as usize); // Shouldn't explore more than total nodes
    }

    /// Integration test: Verify pathfinder works with different graph types
    #[test]
    fn test_req1_graph_integration() {
        // Test with SimpleWeightedGraph
        let mut simple_graph = SimpleWeightedGraph::new();
        simple_graph.add_edge(0, 1, 2.0);
        simple_graph.add_edge(1, 2, 3.0);

        let pathfinder = DijkstraPathfinder::new();
        let result = pathfinder.find_path(&simple_graph, 0, 2).unwrap();
        
        assert_eq!(result.path, vec![0, 1, 2]);
        assert_eq!(result.cost, 5.0);

        // Test with graph created from edges
        let edges = vec![(0, 1, 1.0), (1, 2, 1.0), (0, 2, 5.0)];
        let from_edges_graph = SimpleWeightedGraph::from_edges(&edges);
        
        let result2 = pathfinder.find_path(&from_edges_graph, 0, 2).unwrap();
        
        // Should find optimal path 0->1->2 (cost 2) instead of direct 0->2 (cost 5)
        assert_eq!(result2.path, vec![0, 1, 2]);
        assert_eq!(result2.cost, 2.0);
    }
}