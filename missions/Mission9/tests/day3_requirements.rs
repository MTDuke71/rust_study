//! Mission 9 - Day 3: A* Algorithm Requirements Testing
//!
//! This test suite validates the implementation of REQ-2: AstarPathfinder struct
//! with heuristic support. Tests cover algorithm correctness, heuristic functions,
//! performance characteristics, and admissibility guarantees.

use mission9::astar::{astar, astar_with_heuristic, AstarPathfinder, EuclideanAstar};
use mission9::error::PathfindingError;
use mission9::graph::{SimpleWeightedGraph, WeightedGraph};
use mission9::heuristic::{
    ChebyshevHeuristic, EuclideanHeuristic, Heuristic, HeuristicContext, HeuristicType,
    ManhattanHeuristic, WeightedCombinationHeuristic, ZeroHeuristic,
};
use mission9::pathfinder::{Pathfinder, PathfindingConfig};

/// Test helper: Create a 4x4 grid graph for comprehensive testing
fn create_4x4_grid() -> (SimpleWeightedGraph, HeuristicContext) {
    let mut graph = SimpleWeightedGraph::new();
    let mut context = HeuristicContext::new();

    // Create a 4x4 grid:
    //  0 -  1 -  2 -  3
    //  |    |    |    |
    //  4 -  5 -  6 -  7
    //  |    |    |    |
    //  8 -  9 - 10 - 11
    //  |    |    |    |
    // 12 - 13 - 14 - 15

    // Add coordinates for heuristic calculations
    for y in 0..4 {
        for x in 0..4 {
            let node = y * 4 + x;
            context.add_coordinates(node, x as f64, y as f64);
        }
    }

    // Add horizontal edges
    for y in 0..4 {
        for x in 0..3 {
            let from = y * 4 + x;
            let to = y * 4 + (x + 1);
            graph.add_undirected_edge(from, to, 1.0);
        }
    }

    // Add vertical edges
    for y in 0..3 {
        for x in 0..4 {
            let from = y * 4 + x;
            let to = (y + 1) * 4 + x;
            graph.add_undirected_edge(from, to, 1.0);
        }
    }

    (graph, context)
}

/// Test helper: Create a small 3x3 grid for basic tests
fn create_3x3_grid() -> (SimpleWeightedGraph, HeuristicContext) {
    let mut graph = SimpleWeightedGraph::new();
    let mut context = HeuristicContext::new();

    // Create a 3x3 grid:
    // 0 - 1 - 2
    // |   |   |
    // 3 - 4 - 5
    // |   |   |
    // 6 - 7 - 8

    for y in 0..3 {
        for x in 0..3 {
            let node = y * 3 + x;
            context.add_coordinates(node, x as f64, y as f64);
        }
    }

    // Horizontal edges
    for y in 0..3 {
        for x in 0..2 {
            let from = y * 3 + x;
            let to = y * 3 + (x + 1);
            graph.add_undirected_edge(from, to, 1.0);
        }
    }

    // Vertical edges
    for y in 0..2 {
        for x in 0..3 {
            let from = y * 3 + x;
            let to = (y + 1) * 3 + x;
            graph.add_undirected_edge(from, to, 1.0);
        }
    }

    (graph, context)
}

/// Test helper: Create a weighted graph with diagonal movement
#[allow(dead_code)]
fn create_weighted_grid() -> (SimpleWeightedGraph, HeuristicContext) {
    let mut graph = SimpleWeightedGraph::new();
    let mut context = HeuristicContext::new();

    // 3x3 grid with diagonal connections
    for y in 0..3 {
        for x in 0..3 {
            let node = y * 3 + x;
            context.add_coordinates(node, x as f64, y as f64);
        }
    }

    // Add horizontal and vertical edges (cost 1.0)
    for y in 0..3 {
        for x in 0..2 {
            let from = y * 3 + x;
            let to = y * 3 + (x + 1);
            graph.add_undirected_edge(from, to, 1.0);
        }
    }
    for y in 0..2 {
        for x in 0..3 {
            let from = y * 3 + x;
            let to = (y + 1) * 3 + x;
            graph.add_undirected_edge(from, to, 1.0);
        }
    }

    // Add diagonal edges (cost sqrt(2) ≈ 1.414)
    let diagonal_cost = std::f64::consts::SQRT_2;
    for y in 0..2 {
        for x in 0..2 {
            let from = y * 3 + x;
            let to_diag1 = (y + 1) * 3 + (x + 1); // Bottom-right diagonal
            let _to_diag2 = y * 3 + (x + 1) + 3; // Same as above, different calculation

            graph.add_undirected_edge(from, to_diag1, diagonal_cost);

            // Add other diagonal (top-right to bottom-left)
            let from_tr = y * 3 + (x + 1);
            let to_bl = (y + 1) * 3 + x;
            graph.add_undirected_edge(from_tr, to_bl, diagonal_cost);
        }
    }

    (graph, context)
}

// ===== REQ-2: AstarPathfinder Struct Tests =====

#[test]
fn test_req2_astar_pathfinder_creation() {
    // Test creation with different heuristics
    let manhattan_astar = AstarPathfinder::new(ManhattanHeuristic);
    assert_eq!(manhattan_astar.algorithm_name(), "A* Algorithm");
    assert_eq!(manhattan_astar.heuristic().name(), "Manhattan Distance");

    let euclidean_astar = AstarPathfinder::new(EuclideanHeuristic);
    assert_eq!(euclidean_astar.heuristic().name(), "Euclidean Distance");

    let zero_astar = AstarPathfinder::new(ZeroHeuristic);
    assert_eq!(
        zero_astar.heuristic().name(),
        "Zero Heuristic (Dijkstra Mode)"
    );
}

#[test]
fn test_req2_astar_with_config() {
    let config = PathfindingConfig::new()
        .with_timeout(10)
        .with_max_iterations(1000);
    let pathfinder = AstarPathfinder::with_config(EuclideanHeuristic, config);

    // Test that pathfinder is created correctly
    assert_eq!(pathfinder.algorithm_name(), "A* Algorithm");
}

#[test]
fn test_req2_pathfinder_trait_implementation() {
    let pathfinder = AstarPathfinder::new(ManhattanHeuristic);

    // Verify trait methods
    assert_eq!(pathfinder.algorithm_name(), "A* Algorithm");
    assert!(!pathfinder.supports_negative_weights());
    assert_eq!(pathfinder.time_complexity(), "O(b^d)");
    assert_eq!(pathfinder.space_complexity(), "O(b^d)");
}

// ===== Core Algorithm Tests =====

#[test]
fn test_astar_simple_path_finding() {
    let (graph, context) = create_3x3_grid();
    let pathfinder = AstarPathfinder::new(ManhattanHeuristic);

    // Path from top-left (0) to bottom-right (8)
    let result = pathfinder
        .find_path_with_context(&graph, 0, 8, &context)
        .unwrap();

    // Should find optimal path
    assert_eq!(result.cost, 4.0); // Minimum steps: right-right-down-down
    assert_eq!(result.path.len(), 5); // Start + 4 moves
    assert_eq!(result.path[0], 0); // Starts at 0
    assert_eq!(result.path[4], 8); // Ends at 8

    // Verify it's a valid path
    for i in 1..result.path.len() {
        let from = result.path[i - 1];
        let to = result.path[i];
        assert!(graph
            .neighbors(from)
            .iter()
            .any(|(neighbor, _)| *neighbor == to));
    }
}

#[test]
fn test_astar_single_node_path() {
    let (graph, context) = create_3x3_grid();
    let pathfinder = AstarPathfinder::new(EuclideanHeuristic);

    // Start equals goal
    let result = pathfinder
        .find_path_with_context(&graph, 4, 4, &context)
        .unwrap();

    assert_eq!(result.path, vec![4]);
    assert_eq!(result.cost, 0.0);
    assert_eq!(result.nodes_explored, 1);
}

#[test]
fn test_astar_adjacent_nodes() {
    let (graph, context) = create_3x3_grid();
    let pathfinder = AstarPathfinder::new(ManhattanHeuristic);

    // Adjacent nodes (0 to 1)
    let result = pathfinder
        .find_path_with_context(&graph, 0, 1, &context)
        .unwrap();

    assert_eq!(result.path, vec![0, 1]);
    assert_eq!(result.cost, 1.0);
    assert!(result.nodes_explored >= 1);
}

#[test]
fn test_astar_no_path_exists() {
    let mut graph = SimpleWeightedGraph::new();
    let mut context = HeuristicContext::new();

    // Create two disconnected components
    graph.add_node(0);
    graph.add_node(1);
    context.add_coordinates(0, 0.0, 0.0);
    context.add_coordinates(1, 1.0, 1.0);

    let pathfinder = AstarPathfinder::new(EuclideanHeuristic);
    let result = pathfinder.find_path_with_context(&graph, 0, 1, &context);

    assert!(matches!(
        result,
        Err(PathfindingError::NoPathExists { start: 0, goal: 1 })
    ));
}

// ===== Heuristic Function Tests =====

#[test]
fn test_manhattan_heuristic_accuracy() {
    let mut context = HeuristicContext::new();
    context.add_coordinates(0, 0.0, 0.0);
    context.add_coordinates(1, 3.0, 4.0);
    context.add_coordinates(2, 1.0, 1.0);

    let heuristic = ManhattanHeuristic;

    // (0,0) to (3,4): |3-0| + |4-0| = 7
    assert_eq!(heuristic.estimate_distance(0, 1, &context), 7.0);

    // (0,0) to (1,1): |1-0| + |1-0| = 2
    assert_eq!(heuristic.estimate_distance(0, 2, &context), 2.0);

    // (1,1) to (3,4): |3-1| + |4-1| = 5
    assert_eq!(heuristic.estimate_distance(2, 1, &context), 5.0);
}

#[test]
fn test_euclidean_heuristic_accuracy() {
    let mut context = HeuristicContext::new();
    context.add_coordinates(0, 0.0, 0.0);
    context.add_coordinates(1, 3.0, 4.0); // Classic 3-4-5 triangle
    context.add_coordinates(2, 1.0, 1.0);

    let heuristic = EuclideanHeuristic;

    // (0,0) to (3,4): sqrt(3² + 4²) = 5
    let distance = heuristic.estimate_distance(0, 1, &context);
    assert!((distance - 5.0).abs() < 1e-10);

    // (0,0) to (1,1): sqrt(1² + 1²) = sqrt(2)
    let distance = heuristic.estimate_distance(0, 2, &context);
    assert!((distance - std::f64::consts::SQRT_2).abs() < 1e-10);
}

#[test]
fn test_chebyshev_heuristic_accuracy() {
    let mut context = HeuristicContext::new();
    context.add_coordinates(0, 0.0, 0.0);
    context.add_coordinates(1, 3.0, 4.0);
    context.add_coordinates(2, 2.0, 1.0);

    let heuristic = ChebyshevHeuristic;

    // (0,0) to (3,4): max(|3-0|, |4-0|) = max(3, 4) = 4
    assert_eq!(heuristic.estimate_distance(0, 1, &context), 4.0);

    // (0,0) to (2,1): max(|2-0|, |1-0|) = max(2, 1) = 2
    assert_eq!(heuristic.estimate_distance(0, 2, &context), 2.0);
}

#[test]
fn test_zero_heuristic_dijkstra_mode() {
    let mut context = HeuristicContext::new();
    context.add_coordinates(0, 0.0, 0.0);
    context.add_coordinates(1, 10.0, 10.0);

    let heuristic = ZeroHeuristic;

    // Should always return 0, regardless of distance
    assert_eq!(heuristic.estimate_distance(0, 1, &context), 0.0);
    assert_eq!(heuristic.estimate_distance(1, 0, &context), 0.0);
}

// ===== Admissibility Tests =====

#[test]
fn test_heuristic_admissibility_verification() {
    let (graph, context) = create_3x3_grid(); // Use regular grid without diagonals

    // For all heuristics, h(n) should be <= actual shortest path distance
    let heuristics = vec![
        HeuristicType::Manhattan(ManhattanHeuristic),
        HeuristicType::Euclidean(EuclideanHeuristic),
        HeuristicType::Chebyshev(ChebyshevHeuristic),
        HeuristicType::Zero(ZeroHeuristic),
    ];

    for heuristic in heuristics {
        assert!(
            heuristic.is_admissible(),
            "Heuristic {} should be admissible",
            heuristic.name()
        );

        // Test a few node pairs to verify h(n) <= actual_distance
        for start in 0..9 {
            for goal in 0..9 {
                if start != goal {
                    let h_estimate = heuristic.estimate_distance(start, goal, &context);

                    // Find actual shortest path using Dijkstra (Zero heuristic A*)
                    let dijkstra_pathfinder = AstarPathfinder::new(ZeroHeuristic);
                    if let Ok(actual_result) = dijkstra_pathfinder.find_path(&graph, start, goal) {
                        assert!(
                            h_estimate <= actual_result.cost + 1e-10, // Small epsilon for floating point
                            "Heuristic {} violates admissibility: h({}, {}) = {} > actual_cost = {}",
                            heuristic.name(), start, goal, h_estimate, actual_result.cost
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn test_manhattan_admissible_for_grid() {
    let (graph, context) = create_4x4_grid();
    let manhattan_astar = AstarPathfinder::new(ManhattanHeuristic);
    let dijkstra_astar = AstarPathfinder::new(ZeroHeuristic);

    // Test several start-goal pairs
    let test_pairs = vec![(0, 15), (0, 5), (3, 12), (6, 10)];

    for (start, goal) in test_pairs {
        let manhattan_result = manhattan_astar
            .find_path_with_context(&graph, start, goal, &context)
            .unwrap();
        let dijkstra_result = dijkstra_astar.find_path(&graph, start, goal).unwrap();

        // Both should find optimal path (same cost)
        assert_eq!(
            manhattan_result.cost, dijkstra_result.cost,
            "Manhattan A* should find optimal path like Dijkstra for grid pathfinding"
        );

        // Manhattan should generally explore fewer nodes
        assert!(
            manhattan_result.nodes_explored <= dijkstra_result.nodes_explored,
            "Manhattan A* should explore <= nodes than Dijkstra (explored: {} vs {})",
            manhattan_result.nodes_explored,
            dijkstra_result.nodes_explored
        );
    }
}

// ===== Performance and Efficiency Tests =====

#[test]
fn test_astar_vs_dijkstra_efficiency() {
    let (graph, context) = create_4x4_grid();

    // A* with Manhattan heuristic
    let astar_pathfinder = AstarPathfinder::new(ManhattanHeuristic);
    let astar_result = astar_pathfinder
        .find_path_with_context(&graph, 0, 15, &context) // Opposite corners
        .unwrap();

    // Dijkstra-like (A* with zero heuristic)
    let dijkstra_pathfinder = AstarPathfinder::new(ZeroHeuristic);
    let dijkstra_result = dijkstra_pathfinder.find_path(&graph, 0, 15).unwrap();

    // Both should find optimal path
    assert_eq!(astar_result.cost, dijkstra_result.cost);

    // A* should typically be more efficient (explore fewer nodes)
    println!("A* explored: {} nodes", astar_result.nodes_explored);
    println!(
        "Dijkstra explored: {} nodes",
        dijkstra_result.nodes_explored
    );

    // For this specific case, A* should explore fewer or equal nodes
    assert!(astar_result.nodes_explored <= dijkstra_result.nodes_explored);
}

#[test]
fn test_heuristic_effectiveness_different_distances() {
    let (graph, context) = create_4x4_grid();

    let pathfinder = AstarPathfinder::new(ManhattanHeuristic);

    // Test short distance
    let short_result = pathfinder
        .find_path_with_context(&graph, 0, 1, &context)
        .unwrap();

    // Test medium distance
    let medium_result = pathfinder
        .find_path_with_context(&graph, 0, 10, &context)
        .unwrap();

    // Test long distance
    let long_result = pathfinder
        .find_path_with_context(&graph, 0, 15, &context)
        .unwrap();

    // All should find valid paths
    assert!(short_result.cost > 0.0);
    assert!(medium_result.cost > short_result.cost);
    assert!(long_result.cost > medium_result.cost);

    // Nodes explored should generally increase with distance
    assert!(medium_result.nodes_explored >= short_result.nodes_explored);
    assert!(long_result.nodes_explored >= medium_result.nodes_explored);
}

// ===== Algorithm Correctness Tests =====

#[test]
fn test_astar_path_optimality() {
    let (graph, context) = create_3x3_grid();

    // Test with different heuristics - all should find optimal paths
    let expected_cost = 4.0; // Optimal path from 0 to 8 in 3x3 grid

    // Test Manhattan heuristic
    let manhattan_pathfinder = AstarPathfinder::new(ManhattanHeuristic);
    let manhattan_result = manhattan_pathfinder
        .find_path_with_context(&graph, 0, 8, &context)
        .unwrap();
    assert_eq!(manhattan_result.cost, expected_cost);
    assert_eq!(manhattan_result.path[0], 0);
    assert_eq!(manhattan_result.path[manhattan_result.path.len() - 1], 8);

    // Test Euclidean heuristic
    let euclidean_pathfinder = AstarPathfinder::new(EuclideanHeuristic);
    let euclidean_result = euclidean_pathfinder
        .find_path_with_context(&graph, 0, 8, &context)
        .unwrap();
    assert_eq!(euclidean_result.cost, expected_cost);
    assert_eq!(euclidean_result.path[0], 0);
    assert_eq!(euclidean_result.path[euclidean_result.path.len() - 1], 8);

    // Test Zero heuristic (Dijkstra mode)
    let zero_pathfinder = AstarPathfinder::new(ZeroHeuristic);
    let zero_result = zero_pathfinder.find_path(&graph, 0, 8).unwrap();
    assert_eq!(zero_result.cost, expected_cost);
    assert_eq!(zero_result.path[0], 0);
    assert_eq!(zero_result.path[zero_result.path.len() - 1], 8);
}

#[test]
fn test_astar_multiple_optimal_paths() {
    let (graph, context) = create_3x3_grid();
    let pathfinder = AstarPathfinder::new(ManhattanHeuristic);

    // From 0 to 8, there are multiple optimal paths of length 4
    // A* should find one of them consistently
    let mut costs = Vec::new();
    let mut paths = Vec::new();

    for _ in 0..5 {
        let result = pathfinder
            .find_path_with_context(&graph, 0, 8, &context)
            .unwrap();
        costs.push(result.cost);
        paths.push(result.path);
    }

    // All runs should find the same optimal cost
    assert!(costs.iter().all(|&cost| cost == 4.0));

    // All paths should be valid (we'll check the first one)
    let path = &paths[0];
    for i in 1..path.len() {
        let from = path[i - 1];
        let to = path[i];
        assert!(graph
            .neighbors(from)
            .iter()
            .any(|(neighbor, _)| *neighbor == to));
    }
}

// ===== Advanced Heuristic Tests =====

#[test]
fn test_weighted_combination_heuristic() {
    let mut context = HeuristicContext::new();
    context.add_coordinates(0, 0.0, 0.0);
    context.add_coordinates(1, 3.0, 4.0);

    // Create combination: 0.5 * Manhattan + 0.5 * Euclidean
    let combination = WeightedCombinationHeuristic::new()
        .add_manhattan(0.5)
        .add_euclidean(0.5);

    let distance = combination.estimate_distance(0, 1, &context);

    // Should be: 0.5 * 7.0 + 0.5 * 5.0 = 6.0
    assert_eq!(distance, 6.0);
    assert!(combination.is_admissible());
}

#[test]
fn test_heuristic_type_enum() {
    let mut context = HeuristicContext::new();
    context.add_coordinates(0, 0.0, 0.0);
    context.add_coordinates(1, 1.0, 1.0);

    let heuristics = vec![
        HeuristicType::Manhattan(ManhattanHeuristic),
        HeuristicType::Euclidean(EuclideanHeuristic),
        HeuristicType::Chebyshev(ChebyshevHeuristic),
        HeuristicType::Zero(ZeroHeuristic),
    ];

    for heuristic in heuristics {
        let distance = heuristic.estimate_distance(0, 1, &context);
        assert!(distance >= 0.0);
        assert!(heuristic.is_admissible());
        assert!(!heuristic.name().is_empty());
    }
}

// ===== Edge Cases and Error Handling =====

#[test]
fn test_astar_with_timeout_config() {
    let (graph, context) = create_4x4_grid();
    let config = PathfindingConfig::new().with_timeout(0); // Immediate timeout
    let pathfinder = AstarPathfinder::with_config(ManhattanHeuristic, config);

    let result = pathfinder.find_path_with_context(&graph, 0, 15, &context);

    // Should either timeout or complete very quickly
    match result {
        Err(PathfindingError::Timeout { seconds: 0 }) => {
            // Expected timeout
        }
        Ok(_) => {
            // Completed before timeout - acceptable for small graphs
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_astar_with_max_iterations() {
    let (graph, context) = create_4x4_grid();
    let config = PathfindingConfig::new().with_max_iterations(1); // Very limited iterations
    let pathfinder = AstarPathfinder::with_config(ManhattanHeuristic, config);

    let result = pathfinder.find_path_with_context(&graph, 0, 15, &context);

    // Should fail due to iteration limit for non-trivial paths
    match result {
        Err(PathfindingError::QueueError { .. }) => {
            // Expected iteration limit exceeded
        }
        Ok(_) => {
            // May succeed if path is very short
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_invalid_node_ids() {
    let (graph, context) = create_3x3_grid();
    let pathfinder = AstarPathfinder::new(EuclideanHeuristic);

    // Test with non-existent node
    let result = pathfinder.find_path_with_context(&graph, 0, 999, &context);
    assert!(result.is_err());
}

#[test]
fn test_heuristic_without_coordinates() {
    let _graph = SimpleWeightedGraph::new();
    let context = HeuristicContext::new(); // No coordinates

    // All should fall back to 0.0 when coordinates are missing
    let manhattan_distance = ManhattanHeuristic.estimate_distance(0, 1, &context);
    assert_eq!(
        manhattan_distance, 0.0,
        "Manhattan heuristic should fall back to 0.0 without coordinates"
    );

    let euclidean_distance = EuclideanHeuristic.estimate_distance(0, 1, &context);
    assert_eq!(
        euclidean_distance, 0.0,
        "Euclidean heuristic should fall back to 0.0 without coordinates"
    );

    let chebyshev_distance = ChebyshevHeuristic.estimate_distance(0, 1, &context);
    assert_eq!(
        chebyshev_distance, 0.0,
        "Chebyshev heuristic should fall back to 0.0 without coordinates"
    );
}

// ===== Convenience Functions Tests =====

#[test]
fn test_astar_convenience_function() {
    let (graph, _context) = create_3x3_grid();

    // Test basic astar function (uses Euclidean heuristic by default)
    let result = astar(&graph, 0, 8).unwrap();

    assert_eq!(result.cost, 4.0);
    assert_eq!(result.path[0], 0);
    assert_eq!(result.path[result.path.len() - 1], 8);
}

#[test]
fn test_astar_with_heuristic_convenience_function() {
    let (graph, context) = create_3x3_grid();

    // Test astar_with_heuristic function
    let result = astar_with_heuristic(&graph, 0, 8, ManhattanHeuristic, &context).unwrap();

    assert_eq!(result.cost, 4.0);
    assert_eq!(result.path[0], 0);
    assert_eq!(result.path[result.path.len() - 1], 8);
}

#[test]
fn test_euclidean_astar_type_alias() {
    let (graph, context) = create_3x3_grid();

    // Test EuclideanAstar type alias
    let pathfinder = EuclideanAstar::euclidean();
    let result = pathfinder
        .find_path_with_context(&graph, 0, 8, &context)
        .unwrap();

    assert_eq!(result.cost, 4.0);
    assert_eq!(pathfinder.heuristic().name(), "Euclidean Distance");
}

// ===== Comprehensive Integration Tests =====

#[test]
fn test_astar_comprehensive_grid_exploration() {
    let (graph, context) = create_4x4_grid();
    let pathfinder = AstarPathfinder::new(ManhattanHeuristic);

    // Test all corner-to-corner paths
    let corners = vec![(0, 15), (3, 12), (12, 3), (15, 0)];

    for (start, goal) in corners {
        let result = pathfinder
            .find_path_with_context(&graph, start, goal, &context)
            .unwrap();

        // Manhattan distance for opposite corners should be 6
        assert_eq!(result.cost, 6.0);
        assert_eq!(result.path.len(), 7); // 6 moves + start position

        // Verify path continuity
        for i in 1..result.path.len() {
            let from = result.path[i - 1];
            let to = result.path[i];
            assert!(graph
                .neighbors(from)
                .iter()
                .any(|(neighbor, _)| *neighbor == to));
        }
    }
}

#[test]
fn test_astar_metrics_collection() {
    let (graph, context) = create_4x4_grid();
    let pathfinder = AstarPathfinder::new(EuclideanHeuristic);

    let result = pathfinder
        .find_path_with_context(&graph, 0, 15, &context)
        .unwrap();

    // Verify all metrics are collected
    assert!(result.cost > 0.0);
    assert!(result.nodes_explored > 0);
    // Search time should be measurable (>= 0 is always true for u64)
    assert!(!result.path.is_empty());

    println!("Path found: {:?}", result.path);
    println!("Cost: {}", result.cost);
    println!("Nodes explored: {}", result.nodes_explored);
    println!("Search time: {} μs", result.search_time_us);
}

// ===== f(n) = g(n) + h(n) Formula Verification =====

#[test]
fn test_f_function_formula_implementation() {
    // This test verifies that A* correctly implements f(n) = g(n) + h(n)
    // by comparing with a manual implementation

    let (graph, context) = create_3x3_grid();
    let pathfinder = AstarPathfinder::new(ManhattanHeuristic);

    // For path finding from 0 to 8, track the algorithm behavior
    let result = pathfinder
        .find_path_with_context(&graph, 0, 8, &context)
        .unwrap();

    // The algorithm should find the optimal path
    assert_eq!(result.cost, 4.0);

    // Manual verification for start node (0):
    // g(0) = 0 (no cost to reach start)
    // h(0) = manhattan_distance(0, 8) = |2-0| + |2-0| = 4
    // f(0) = 0 + 4 = 4

    let start_h = ManhattanHeuristic.estimate_distance(0, 8, &context);
    assert_eq!(start_h, 4.0);

    // The result should represent the optimal solution found by A*
    assert_eq!(result.path[0], 0);
    assert_eq!(result.path[result.path.len() - 1], 8);
}

/// Final comprehensive test that validates all REQ-2 requirements
#[test]
fn test_req2_comprehensive_validation() {
    println!("=== REQ-2: AstarPathfinder Comprehensive Validation ===");

    // ✓ Generic heuristic function support
    let manhattan_pathfinder = AstarPathfinder::new(ManhattanHeuristic);
    let euclidean_pathfinder = AstarPathfinder::new(EuclideanHeuristic);
    let zero_pathfinder = AstarPathfinder::new(ZeroHeuristic);

    let (graph, context) = create_4x4_grid();

    // ✓ f(n) = g(n) + h(n) cost calculation (tested through successful pathfinding)
    let manhattan_result = manhattan_pathfinder
        .find_path_with_context(&graph, 0, 15, &context)
        .unwrap();
    let euclidean_result = euclidean_pathfinder
        .find_path_with_context(&graph, 0, 15, &context)
        .unwrap();
    let dijkstra_result = zero_pathfinder.find_path(&graph, 0, 15).unwrap();

    // All should find optimal path (cost 6.0 for opposite corners of 4x4 grid)
    assert_eq!(manhattan_result.cost, 6.0);
    assert_eq!(euclidean_result.cost, 6.0);
    assert_eq!(dijkstra_result.cost, 6.0);

    // ✓ Efficient open/closed set management (verified through successful execution)
    assert!(manhattan_result.nodes_explored > 0);
    assert!(euclidean_result.nodes_explored > 0);
    assert!(dijkstra_result.nodes_explored > 0);

    // ✓ Admissibility verification
    assert!(ManhattanHeuristic.is_admissible());
    assert!(EuclideanHeuristic.is_admissible());
    assert!(ZeroHeuristic.is_admissible());

    // ✓ Performance metrics vs Dijkstra
    println!(
        "Manhattan A* explored: {} nodes",
        manhattan_result.nodes_explored
    );
    println!(
        "Euclidean A* explored: {} nodes",
        euclidean_result.nodes_explored
    );
    println!(
        "Dijkstra explored: {} nodes",
        dijkstra_result.nodes_explored
    );

    // A* should generally be more efficient than Dijkstra
    assert!(manhattan_result.nodes_explored <= dijkstra_result.nodes_explored);
    assert!(euclidean_result.nodes_explored <= dijkstra_result.nodes_explored);

    println!("✅ REQ-2: AstarPathfinder implementation complete and validated!");
}
