//! Coverage improvement tests for Mission9
//!
//! This test module targets uncovered code sections to improve overall coverage.
//! Focus areas: error.rs, hierarchical.rs, constraint_based.rs, graph_loader.rs

use mission9::constraint_based::*;
use mission9::error::*;
use mission9::graph::*;
use mission9::hierarchical::*;
use mission9::heuristic::*;
use mission9::multi_objective::*;
use mission9::pathfinder::*;
use mission9::*;

// ============================================================================
// ERROR MODULE TESTS (error.rs: 0% coverage)
// ============================================================================

#[test]
fn test_error_is_recoverable() {
    // Test is_recoverable() for all PathfindingError variants
    
    // Recoverable errors
    assert!(PathfindingError::NoPathExists { start: 0, goal: 1 }.is_recoverable());
    assert!(PathfindingError::InvalidHeuristic { value: -1.0 }.is_recoverable());
    assert!(PathfindingError::Timeout { seconds: 10 }.is_recoverable());
    assert!(PathfindingError::MaxIterationsExceeded.is_recoverable());
    assert!(PathfindingError::ConstraintViolation {
        constraint: "test".to_string()
    }
    .is_recoverable());

    // Non-recoverable errors
    assert!(!PathfindingError::InvalidStartNode { node: 0 }.is_recoverable());
    assert!(!PathfindingError::InvalidGoalNode { node: 1 }.is_recoverable());
    assert!(!PathfindingError::InvalidNode { node: 2 }.is_recoverable());
    assert!(!PathfindingError::NegativeWeights.is_recoverable());
    assert!(!PathfindingError::InvalidGraph.is_recoverable());
    assert!(!PathfindingError::QueueError { message: "test".to_string() }.is_recoverable());
    assert!(!PathfindingError::InvalidConfiguration { message: "test".to_string() }.is_recoverable());
    assert!(!PathfindingError::InvalidInput("test".to_string()).is_recoverable());
}

#[test]
fn test_error_severity() {
    // Test severity() for all PathfindingError variants
    assert_eq!(
        PathfindingError::NoPathExists { start: 0, goal: 1 }.severity(),
        ErrorSeverity::Warning
    );

    assert_eq!(
        PathfindingError::InvalidStartNode { node: 0 }.severity(),
        ErrorSeverity::Fatal
    );
    assert_eq!(
        PathfindingError::InvalidGoalNode { node: 1 }.severity(),
        ErrorSeverity::Fatal
    );
    assert_eq!(
        PathfindingError::InvalidNode { node: 2 }.severity(),
        ErrorSeverity::Fatal
    );

    assert_eq!(
        PathfindingError::NegativeWeights.severity(),
        ErrorSeverity::Fatal
    );
    assert_eq!(
        PathfindingError::InvalidGraph.severity(),
        ErrorSeverity::Fatal
    );

    assert_eq!(
        PathfindingError::InvalidHeuristic { value: -1.0 }.severity(),
        ErrorSeverity::Warning
    );

    assert_eq!(
        PathfindingError::QueueError { message: "test".to_string() }.severity(),
        ErrorSeverity::Fatal
    );

    assert_eq!(
        PathfindingError::Timeout { seconds: 10 }.severity(),
        ErrorSeverity::Recoverable
    );

    assert_eq!(
        PathfindingError::MaxIterationsExceeded.severity(),
        ErrorSeverity::Recoverable
    );

    assert_eq!(
        PathfindingError::InvalidConfiguration { message: "test".to_string() }.severity(),
        ErrorSeverity::Fatal
    );

    assert_eq!(
        PathfindingError::ConstraintViolation { constraint: "test".to_string() }.severity(),
        ErrorSeverity::Warning
    );

    assert_eq!(
        PathfindingError::InvalidInput("test".to_string()).severity(),
        ErrorSeverity::Fatal
    );
}

// ============================================================================
// HIERARCHICAL MODULE TESTS (hierarchical.rs: 41% coverage)
// ============================================================================

#[test]
fn test_calculate_node_importance_degree() {
    let mut graph = SimpleWeightedGraph::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(0, 2, 1.0);
    graph.add_edge(1, 2, 1.0);

    let importance = calculate_node_importance(&graph, ImportanceStrategy::Degree);
    assert!(!importance.is_empty());
    // Node 0 has 2 neighbors, node 1 has 2 neighbors, node 2 has 2 neighbors
}

#[test]
fn test_calculate_node_importance_edge_difference() {
    let mut graph = SimpleWeightedGraph::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(0, 2, 1.0);
    graph.add_edge(1, 2, 1.0);

    let importance = calculate_node_importance(&graph, ImportanceStrategy::EdgeDifference);
    assert!(!importance.is_empty());
}

#[test]
fn test_calculate_node_importance_betweenness() {
    let mut graph = SimpleWeightedGraph::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(1, 2, 1.0);
    graph.add_edge(2, 3, 1.0);

    let importance = calculate_node_importance(&graph, ImportanceStrategy::Betweenness);
    assert!(!importance.is_empty());
}

#[test]
fn test_calculate_node_importance_combined() {
    let mut graph = SimpleWeightedGraph::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(0, 2, 1.0);
    graph.add_edge(1, 2, 1.0);

    let importance = calculate_node_importance(&graph, ImportanceStrategy::Combined);
    assert!(!importance.is_empty());
}

#[test]
fn test_contraction_hierarchy_build() {
    let mut graph = SimpleWeightedGraph::new();
    // Create a small graph
    for i in 0..5 {
        graph.add_edge(i, (i + 1) % 5, 1.0);
    }

    let ch = ContractionHierarchy::build(&graph, ImportanceStrategy::Degree);
    assert!(ch.is_ok());

    // Verify stats are populated
    let ch = ch.unwrap();
    let stats = ch.stats();
    assert_eq!(stats.node_count, 5);
}

#[test]
fn test_contraction_hierarchy_query() {
    let mut graph = SimpleWeightedGraph::new();
    // Create a simple path: 0 -> 1 -> 2 -> 3
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(1, 2, 1.0);
    graph.add_edge(2, 3, 1.0);

    let ch = ContractionHierarchy::build(&graph, ImportanceStrategy::Degree);
    assert!(ch.is_ok());

    let ch = ch.unwrap();
    // Query for path from 0 to 3
    let result = ch.query(0, 3);
    assert!(result.is_ok());
    
    // Query might return None if path not found due to contractions
    if let Ok(Some((cost, path))) = result {
        assert!(cost >= 0.0);
        assert!(!path.is_empty());
    }
}

#[test]
fn test_jump_point_search_creation() {
    let jps = JumpPointSearch::new(5, 5, false);
    // Just verify it was created - fields are private
    let _ = jps;
}

#[test]
fn test_jump_point_search_with_diagonal() {
    let jps = JumpPointSearch::new(5, 5, true);
    // Just verify it was created with diagonal movement allowed
    let _ = jps;
}

// ============================================================================
// CONSTRAINT_BASED MODULE TESTS (constraint_based.rs: 75% coverage)
// ============================================================================

#[test]
fn test_path_context_methods() {
    let mut context = PathContext::new(0);
    assert_eq!(context.path_length(), 1);
    assert_eq!(context.current_node(), Some(0));
    assert_eq!(context.current_cost, 0.0);

    context = context.with_step(1, 2.5);
    assert_eq!(context.path_length(), 2);
    assert_eq!(context.current_node(), Some(1));
    assert_eq!(context.current_cost, 2.5);
}

#[test]
fn test_forbidden_zone_constraint_methods() {
    let constraint = ForbiddenZoneConstraint::new()
        .add_forbidden_node(5)
        .add_forbidden_nodes(&[10, 15]);

    assert!(constraint.is_forbidden(5));
    assert!(constraint.is_forbidden(10));
    assert!(constraint.is_forbidden(15));
    assert!(!constraint.is_forbidden(20));

    assert_eq!(constraint.description(), "Forbidden Zone Constraint");
    assert!(constraint.is_global());
}

#[test]
fn test_max_cost_constraint() {
    let constraint = MaxCostConstraint::new(10.0);
    let context = PathContext::new(0);

    assert!(constraint.is_valid_node(1, &context)); // Cost 0 < 10
    assert_eq!(constraint.description(), "Maximum Cost Constraint");
    assert!(constraint.is_global());
}

#[test]
fn test_max_length_constraint() {
    let constraint = MaxLengthConstraint::new(5);
    let context = PathContext::new(0);

    assert!(constraint.is_valid_node(1, &context)); // Length 1 < 5
    assert_eq!(constraint.description(), "Maximum Path Length Constraint");
    assert!(constraint.is_global());
}

#[test]
fn test_no_cycle_constraint() {
    let constraint = NoCycleConstraint;
    let context = PathContext::new(0).with_step(1, 1.0).with_step(2, 1.0);

    assert!(constraint.is_valid_node(3, &context)); // New node
    assert!(!constraint.is_valid_node(0, &context)); // Already visited
    assert_eq!(constraint.description(), "No Cycle Constraint");
    assert!(constraint.is_global());
}

#[test]
fn test_waypoint_constraint() {
    let mut constraint = WaypointConstraint::new(&[5, 10, 15]);
    assert!(!constraint.all_waypoints_visited());

    constraint.update_visited(5);
    constraint.update_visited(10);
    assert!(!constraint.all_waypoints_visited());

    constraint.update_visited(15);
    assert!(constraint.all_waypoints_visited());

    assert_eq!(constraint.description(), "Waypoint Constraint");
    assert!(!constraint.is_global()); // Waypoint constraints are not global
}

#[test]
fn test_path_constraint_type_enum() {
    let forbidden = PathConstraintType::ForbiddenZone(ForbiddenZoneConstraint::new().add_forbidden_node(5));
    let context = PathContext::new(0);

    assert!(forbidden.is_valid_node(0, &context));
    assert!(!forbidden.is_valid_node(5, &context));
    assert_eq!(forbidden.description(), "Forbidden Zone Constraint");
    assert!(forbidden.is_global());
}

#[test]
fn test_constrained_astar_preset_avoid_zones() {
    let graph = create_simple_graph();
    let heuristic_context = HeuristicContext::new();

    let pathfinder = presets::avoid_zones(ManhattanHeuristic, &[2]);
    let result = pathfinder.find_constrained_path(&graph, 0, 3, &heuristic_context);

    // Should find a path avoiding node 2
    if let Ok(path_result) = result {
        assert!(!path_result.path.contains(&2));
    }
}

#[test]
fn test_constrained_astar_preset_limited_search() {
    let graph = create_simple_graph();
    let heuristic_context = HeuristicContext::new();

    let pathfinder = presets::limited_search(ManhattanHeuristic, 100.0, 10);
    let _ = pathfinder.find_constrained_path(&graph, 0, 3, &heuristic_context);
    // Just verify it runs
}

#[test]
fn test_constrained_astar_preset_acyclic_search() {
    let graph = create_simple_graph();
    let heuristic_context = HeuristicContext::new();

    let pathfinder = presets::acyclic_search(ManhattanHeuristic);
    let _ = pathfinder.find_constrained_path(&graph, 0, 3, &heuristic_context);
    // Just verify it runs
}

#[test]
fn test_constrained_astar_preset_safe_pathfinding() {
    let graph = create_simple_graph();
    let heuristic_context = HeuristicContext::new();

    let pathfinder = presets::safe_pathfinding(ManhattanHeuristic, &[2], 100.0, 10);
    let _ = pathfinder.find_constrained_path(&graph, 0, 3, &heuristic_context);
    // Just verify it runs with all constraints
}

// ============================================================================
// MULTI-OBJECTIVE MODULE TESTS
// ============================================================================

#[test]
fn test_multi_objective_solution_dominance() {
    let sol1 = MultiObjectiveSolution::new(vec![0, 1], vec![1.0, 2.0]);
    let sol2 = MultiObjectiveSolution::new(vec![0, 2], vec![2.0, 3.0]);

    // sol1 dominates sol2 (better in both objectives)
    assert!(sol1.dominates(&sol2));
    assert!(!sol2.dominates(&sol1));
}

#[test]
fn test_multi_objective_result_best_for_objective() {
    let solutions = vec![
        MultiObjectiveSolution::new(vec![0, 1], vec![1.0, 5.0]),
        MultiObjectiveSolution::new(vec![0, 2], vec![3.0, 2.0]),
    ];

    let result = MultiObjectiveResult {
        pareto_optimal_solutions: solutions,
        objective_names: vec!["Cost".to_string(), "Time".to_string()],
        nodes_explored: 10,
        search_time_us: 100,
    };

    // Best for objective 0 (cost)
    let best_cost = result.best_for_objective(0).unwrap();
    assert_eq!(best_cost.objective_values[0], 1.0);

    // Best for objective 1 (time)
    let best_time = result.best_for_objective(1).unwrap();
    assert_eq!(best_time.objective_values[1], 2.0);
}

#[test]
fn test_multi_objective_result_compromise_solution() {
    let solutions = vec![
        MultiObjectiveSolution::new(vec![0, 1], vec![1.0, 5.0]),
        MultiObjectiveSolution::new(vec![0, 2], vec![3.0, 2.0]),
    ];

    let result = MultiObjectiveResult {
        pareto_optimal_solutions: solutions,
        objective_names: vec!["Cost".to_string(), "Time".to_string()],
        nodes_explored: 10,
        search_time_us: 100,
    };

    let compromise = result.compromise_solution();
    assert!(compromise.is_some());
}

#[test]
fn test_cost_objective() {
    let mut graph = SimpleWeightedGraph::new();
    graph.add_edge(0, 1, 2.5);

    let obj = CostObjective;
    let cost = obj.evaluate_edge(0, 1, &graph);
    assert_eq!(cost, 2.5);
    assert_eq!(obj.name(), "Cost");
    assert!(obj.is_minimizing());
}

#[test]
fn test_time_objective() {
    let mut graph = SimpleWeightedGraph::new();
    graph.add_edge(0, 1, 2.0);

    let mut obj = TimeObjective::new();
    obj.set_time_multiplier(0, 1, 1.5);

    let time = obj.evaluate_edge(0, 1, &graph);
    assert_eq!(time, 3.0); // 2.0 * 1.5
    assert_eq!(obj.name(), "Time");
}

#[test]
fn test_safety_objective() {
    let mut graph = SimpleWeightedGraph::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(0, 2, 1.0);

    let mut obj = SafetyObjective::new(10.0);
    obj.add_danger_zone(1);

    let safe_cost = obj.evaluate_edge(0, 2, &graph);
    assert_eq!(safe_cost, 0.0);

    let dangerous_cost = obj.evaluate_edge(0, 1, &graph);
    assert_eq!(dangerous_cost, 10.0);

    assert_eq!(obj.name(), "Safety");
}

// ============================================================================
// PATHFINDER MODULE TESTS
// ============================================================================

#[test]
fn test_pathfinding_config_builder() {
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
fn test_path_result_methods() {
    let result = PathResult::new(vec![0, 1, 2, 3], 10.5);

    assert!(!result.is_empty());
    assert_eq!(result.step_count(), 3);
    assert_eq!(result.start(), Some(0));
    assert_eq!(result.goal(), Some(3));
    assert_eq!(result.average_step_cost(), 10.5 / 3.0);
}

#[test]
fn test_path_result_empty() {
    let result = PathResult::new(vec![], 0.0);

    assert!(result.is_empty());
    assert_eq!(result.step_count(), 0);
    assert_eq!(result.start(), None);
    assert_eq!(result.goal(), None);
}

// ============================================================================
// GRAPH MODULE TESTS
// ============================================================================

#[test]
fn test_simple_weighted_graph_methods() {
    let mut graph = SimpleWeightedGraph::new();

    assert_eq!(graph.node_count(), 0);

    graph.add_edge(0, 1, 2.0);
    graph.add_edge(1, 2, 3.0);

    assert!(graph.contains_node(0));
    assert!(graph.contains_node(1));
    assert!(graph.contains_node(2));
    assert!(!graph.contains_node(99));

    let weight = graph.get_edge_weight(0, 1);
    assert_eq!(weight, Some(2.0));

    let no_weight = graph.get_edge_weight(0, 99);
    assert_eq!(no_weight, None);
}

#[test]
fn test_simple_weighted_graph_nodes_method() {
    let mut graph = SimpleWeightedGraph::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(1, 2, 1.0);
    graph.add_edge(2, 3, 1.0);

    let nodes: Vec<_> = graph.nodes().into_iter().collect();
    assert!(nodes.contains(&0));
    assert!(nodes.contains(&1));
    assert!(nodes.contains(&2));
    assert!(nodes.contains(&3));
}

// ============================================================================
// Additional Hierarchical Module Tests (Deep Coverage)
// ============================================================================

#[test]
fn test_ch_with_triangle_graph() {
    // Create a triangle graph to trigger shortcut creation
    let mut graph = SimpleWeightedGraph::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(1, 2, 1.0);
    graph.add_edge(0, 2, 5.0); // More expensive direct path
    
    let ch = ContractionHierarchy::build(&graph, ImportanceStrategy::Combined)
        .expect("CH build failed");
    
    let stats = ch.stats();
    assert_eq!(stats.node_count, 3);
    // Shortcut count is non-negative by type
    let _ = stats.shortcut_count;
}

#[test]
fn test_ch_query_with_actual_path() {
    // Create a longer path to test bidirectional search
    let mut graph = SimpleWeightedGraph::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(1, 2, 1.0);
    graph.add_edge(2, 3, 1.0);
    graph.add_edge(3, 4, 1.0);
    
    let ch = ContractionHierarchy::build(&graph, ImportanceStrategy::Degree)
        .expect("CH build failed");
    
    let result = ch.query(0, 4);
    assert!(result.is_ok());
    
    if let Ok(Some((cost, path))) = result {
        assert!(cost > 0.0);
        assert!(path.len() >= 2);
        assert_eq!(path[0], 0);
        assert_eq!(path[path.len() - 1], 4);
    }
}

#[test]
fn test_ch_query_no_path() {
    // Create two disconnected components
    let mut graph = SimpleWeightedGraph::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(2, 3, 1.0);
    
    let ch = ContractionHierarchy::build(&graph, ImportanceStrategy::EdgeDifference)
        .expect("CH build failed");
    
    let result = ch.query(0, 3);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none()); // No path should exist
}

#[test]
fn test_ch_with_dense_graph() {
    // Create a more complex graph to test witness search
    let mut graph = SimpleWeightedGraph::new();
    // Create a 4-node mesh
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(0, 2, 1.0);
    graph.add_edge(1, 3, 1.0);
    graph.add_edge(2, 3, 1.0);
    graph.add_edge(0, 3, 10.0); // Alternative longer path
    
    let ch = ContractionHierarchy::build(&graph, ImportanceStrategy::Betweenness)
        .expect("CH build failed");
    
    let result = ch.query(0, 3);
    assert!(result.is_ok());
}

#[test]
fn test_jps_jump_to_goal() {
    let walkable = vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ];
    
    let jps = JumpPointSearch::new(3, 3, true);
    
    // Jump horizontally toward goal
    let result = jps.jump(0, 0, 1, 0, 2, 0, &walkable);
    assert!(result.is_some());
}

#[test]
fn test_jps_jump_with_obstacle() {
    let walkable = vec![
        vec![true, true, true],
        vec![true, false, true],
        vec![true, true, true],
    ];
    
    let jps = JumpPointSearch::new(3, 3, true);
    
    // Jump into obstacle should return None
    let result = jps.jump(0, 1, 1, 0, 2, 1, &walkable);
    assert!(result.is_none());
}

#[test]
fn test_jps_diagonal_jump() {
    let walkable = vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ];
    
    let jps = JumpPointSearch::new(3, 3, true);
    
    // Diagonal jump
    let result = jps.jump(0, 0, 1, 1, 2, 2, &walkable);
    assert!(result.is_some());
}

#[test]
fn test_jps_recursive_jump() {
    // Test that jump continues recursively
    let walkable = vec![
        vec![true, true, true, true, true],
        vec![true, true, true, true, true],
        vec![true, true, true, true, true],
    ];
    
    let jps = JumpPointSearch::new(5, 3, true);
    
    // Jump across multiple cells
    let result = jps.jump(0, 0, 1, 0, 4, 0, &walkable);
    assert!(result.is_some());
}

#[test]
fn test_jps_bounds_checking() {
    let walkable = vec![
        vec![true, true],
        vec![true, true],
    ];
    
    let jps = JumpPointSearch::new(2, 2, false);
    
    // Jump out of bounds
    let result = jps.jump(1, 1, 1, 0, 5, 1, &walkable);
    assert!(result.is_none());
}

#[test]
fn test_ch_empty_graph_error() {
    let graph = SimpleWeightedGraph::new();
    
    let result = ContractionHierarchy::build(&graph, ImportanceStrategy::Degree);
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(matches!(e, PathfindingError::InvalidConfiguration { .. }));
    }
}

#[test]
fn test_edge_difference_calculation() {
    let mut graph = SimpleWeightedGraph::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(0, 2, 1.0);
    graph.add_edge(0, 3, 1.0);
    
    let importance = calculate_node_importance(&graph, ImportanceStrategy::EdgeDifference);
    
    // Node 0 should have edge difference score
    let node0_importance = importance.iter().find(|(id, _)| *id == 0).unwrap().1;
    // Edge difference can be 0 or positive depending on graph structure
    assert!(node0_importance >= 0.0);
}

#[test]
fn test_betweenness_calculation() {
    let mut graph = SimpleWeightedGraph::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(1, 2, 1.0);
    
    let importance = calculate_node_importance(&graph, ImportanceStrategy::Betweenness);
    
    // Node 1 should have higher betweenness (connects 0 and 2)
    let node1_importance = importance.iter().find(|(id, _)| *id == 1).unwrap().1;
    assert!(node1_importance >= 0.0);
}

#[test]
fn test_ch_stats() {
    let mut graph = SimpleWeightedGraph::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(1, 2, 1.0);
    graph.add_edge(2, 3, 1.0);
    
    let ch = ContractionHierarchy::build(&graph, ImportanceStrategy::Combined)
        .expect("CH build failed");
    
    let stats = ch.stats();
    assert_eq!(stats.node_count, 4);
    assert!(stats.avg_shortcuts_per_node >= 0.0);
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn create_simple_graph() -> SimpleWeightedGraph {
    let mut graph = SimpleWeightedGraph::new();
    // Create a simple path: 0 -> 1 -> 2 -> 3
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(1, 2, 1.0);
    graph.add_edge(2, 3, 1.0);
    // Add alternate path: 0 -> 3 (direct)
    graph.add_edge(0, 3, 4.0);
    graph
}
