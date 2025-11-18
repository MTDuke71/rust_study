//! Day 5 Requirements Tests: Advanced Heuristics & Multi-Objective Pathfinding
//!
//! This test suite validates the implementation of REQ-5: Multi-objective pathfinding support
//! and advanced algorithmic features for Mission 9.
//!
//! Test Categories:
//! - Multi-objective pathfinding with Pareto-optimal solutions
//! - Constraint-based pathfinding with various restrictions
//! - Advanced heuristic combinations and custom objectives
//! - Performance and correctness validation

use mission9::*;
use std::collections::HashSet;

/// Create a test graph for advanced pathfinding scenarios
fn create_advanced_test_graph() -> SimpleWeightedGraph {
    let mut graph = SimpleWeightedGraph::new();

    // Create a 4x4 grid graph for comprehensive testing
    //  0 - 1 - 2 - 3
    //  |   |   |   |
    //  4 - 5 - 6 - 7
    //  |   |   |   |
    //  8 - 9 -10 -11
    //  |   |   |   |
    // 12 -13 -14 -15

    // Horizontal edges
    for row in 0..4 {
        for col in 0..3 {
            let from = row * 4 + col;
            let to = row * 4 + col + 1;
            graph.add_undirected_edge(from, to, 1.0);
        }
    }

    // Vertical edges
    for row in 0..3 {
        for col in 0..4 {
            let from = row * 4 + col;
            let to = (row + 1) * 4 + col;
            graph.add_undirected_edge(from, to, 1.0);
        }
    }

    // Add some expensive shortcuts for testing
    graph.add_undirected_edge(0, 15, 8.0); // Expensive direct route
    graph.add_undirected_edge(3, 12, 8.0); // Another expensive shortcut

    graph
}

/// Create heuristic context with coordinates for the 4x4 grid
fn create_grid_heuristic_context() -> HeuristicContext {
    let mut context = HeuristicContext::new();

    for row in 0..4 {
        for col in 0..4 {
            let node = row * 4 + col;
            context.add_coordinates(node, col as f64, row as f64);
        }
    }

    context
}

// ===== REQ-5: Multi-Objective Pathfinding Tests =====

#[test]
fn test_req5_single_objective_cost() {
    let graph = create_advanced_test_graph();

    let pathfinder = MultiObjectiveAstar::new().add_objective(multi_objective::CostObjective);

    let result = pathfinder.find_pareto_optimal_paths(&graph, 0, 15).unwrap();

    assert_eq!(result.pareto_optimal_solutions.len(), 1);
    assert_eq!(result.objective_names, vec!["Cost"]);

    let solution = &result.pareto_optimal_solutions[0];
    assert_eq!(solution.path.first(), Some(&0));
    assert_eq!(solution.path.last(), Some(&15));
    assert!(solution.objective_values[0] > 0.0);
}

#[test]
fn test_req5_cost_vs_time_objectives() {
    let graph = create_advanced_test_graph();
    let mut time_objective = multi_objective::TimeObjective::new();

    // Make some edges slow (high time multiplier)
    time_objective.set_time_multiplier(0, 1, 3.0);
    time_objective.set_time_multiplier(4, 5, 3.0);
    time_objective.set_time_multiplier(8, 9, 3.0);

    let pathfinder = MultiObjectiveAstar::new()
        .add_objective(multi_objective::CostObjective)
        .add_objective(time_objective);

    let result = pathfinder.find_pareto_optimal_paths(&graph, 0, 15).unwrap();

    assert!(!result.pareto_optimal_solutions.is_empty());
    assert_eq!(result.objective_names, vec!["Cost", "Time"]);

    // Check that all solutions are non-dominated
    for (i, solution1) in result.pareto_optimal_solutions.iter().enumerate() {
        for (j, solution2) in result.pareto_optimal_solutions.iter().enumerate() {
            if i != j {
                assert!(
                    !solution1.dominates(solution2),
                    "Solution {} dominates solution {} in Pareto set",
                    i,
                    j
                );
            }
        }

        assert_eq!(solution1.objective_values.len(), 2);
        assert!(solution1.objective_values[0] > 0.0); // Cost
        assert!(solution1.objective_values[1] > 0.0); // Time
    }
}

#[test]
fn test_req5_safety_objective() {
    let graph = create_advanced_test_graph();
    let mut safety_objective = multi_objective::SafetyObjective::new(5.0);

    // Mark center nodes as dangerous
    safety_objective.add_danger_zones(&[5, 6, 9, 10]);

    let pathfinder = MultiObjectiveAstar::new()
        .add_objective(multi_objective::CostObjective)
        .add_objective(safety_objective);

    let result = pathfinder.find_pareto_optimal_paths(&graph, 0, 15).unwrap();

    assert!(!result.pareto_optimal_solutions.is_empty());
    assert_eq!(result.objective_names, vec!["Cost", "Safety"]);

    // At least one solution should avoid dangerous nodes
    let safe_solution_exists = result.pareto_optimal_solutions.iter().any(|solution| {
        !solution
            .path
            .iter()
            .any(|&node| [5, 6, 9, 10].contains(&node))
    });
    assert!(safe_solution_exists, "Should find at least one safe path");
}

#[test]
fn test_req5_three_objective_optimization() {
    let graph = create_advanced_test_graph();
    let mut time_objective = multi_objective::TimeObjective::new();
    let mut safety_objective = multi_objective::SafetyObjective::new(3.0);

    time_objective.set_time_multiplier(1, 2, 2.0);
    time_objective.set_time_multiplier(5, 6, 2.0);
    safety_objective.add_danger_zones(&[6, 10]);

    let pathfinder = MultiObjectiveAstar::new()
        .add_objective(multi_objective::CostObjective)
        .add_objective(time_objective)
        .add_objective(safety_objective);

    let result = pathfinder.find_pareto_optimal_paths(&graph, 0, 15).unwrap();

    assert!(!result.pareto_optimal_solutions.is_empty());
    assert_eq!(result.objective_names, vec!["Cost", "Time", "Safety"]);

    for solution in &result.pareto_optimal_solutions {
        assert_eq!(solution.objective_values.len(), 3);
        assert!(solution.objective_values[0] > 0.0); // Cost
        assert!(solution.objective_values[1] > 0.0); // Time
        assert!(solution.objective_values[2] >= 0.0); // Safety (can be 0)
    }
}

#[test]
fn test_req5_pareto_optimality_verification() {
    let graph = create_advanced_test_graph();
    let safety_objective = multi_objective::SafetyObjective::new(2.0);

    let pathfinder = MultiObjectiveAstar::new()
        .add_objective(multi_objective::CostObjective)
        .add_objective(safety_objective);

    let result = pathfinder.find_pareto_optimal_paths(&graph, 0, 7).unwrap();

    // Verify Pareto optimality property
    for (i, solution1) in result.pareto_optimal_solutions.iter().enumerate() {
        for (j, solution2) in result.pareto_optimal_solutions.iter().enumerate() {
            if i != j {
                // No solution should dominate another in the Pareto set
                assert!(!solution1.dominates(solution2));
                assert!(!solution2.dominates(solution1));
            }
        }
    }
}

#[test]
fn test_req5_multi_objective_result_analysis() {
    let graph = create_advanced_test_graph();
    let mut time_objective = multi_objective::TimeObjective::new();
    time_objective.set_time_multiplier(0, 4, 0.5); // Fast vertical route

    let pathfinder = MultiObjectiveAstar::new()
        .add_objective(multi_objective::CostObjective)
        .add_objective(time_objective);

    let result = pathfinder.find_pareto_optimal_paths(&graph, 0, 8).unwrap();

    // Test result analysis methods
    let best_cost = result.best_for_objective(0);
    assert!(best_cost.is_some());

    let best_time = result.best_for_objective(1);
    assert!(best_time.is_some());

    let compromise = result.compromise_solution();
    assert!(compromise.is_some());

    // Verify that best solutions are actually optimal for their objectives
    if result.pareto_optimal_solutions.len() > 1 {
        let best_cost_value = best_cost.unwrap().objective_values[0];
        let best_time_value = best_time.unwrap().objective_values[1];

        for solution in &result.pareto_optimal_solutions {
            assert!(solution.objective_values[0] >= best_cost_value);
            assert!(solution.objective_values[1] >= best_time_value);
        }
    }
}

#[test]
fn test_req5_empty_objectives_error() {
    let graph = create_advanced_test_graph();
    let pathfinder = MultiObjectiveAstar::new(); // No objectives

    let result = pathfinder.find_pareto_optimal_paths(&graph, 0, 15);

    assert!(result.is_err());
    match result.unwrap_err() {
        PathfindingError::InvalidConfiguration { message } => {
            assert!(message.contains("No objectives specified"));
        }
        e => panic!("Expected InvalidConfiguration, got {:?}", e),
    }
}

// ===== Constraint-Based Pathfinding Tests =====

#[test]
fn test_req5_forbidden_zone_constraint() {
    let graph = create_advanced_test_graph();
    let context = create_grid_heuristic_context();

    let pathfinder = ConstrainedAstar::new(ManhattanHeuristic).add_constraint(
        constraint_based::PathConstraintType::ForbiddenZone(
            constraint_based::ForbiddenZoneConstraint::new().add_forbidden_nodes(&[5, 6, 9, 10]),
        ),
    ); // Block center

    let result = pathfinder
        .find_constrained_path(&graph, 0, 15, &context)
        .unwrap();

    // Path should not contain forbidden nodes
    for &forbidden in &[5, 6, 9, 10] {
        assert!(
            !result.path.contains(&forbidden),
            "Path contains forbidden node {}: {:?}",
            forbidden,
            result.path
        );
    }

    assert_eq!(result.path.first(), Some(&0));
    assert_eq!(result.path.last(), Some(&15));
    assert!(result.cost > 0.0);
}

#[test]
fn test_req5_max_cost_constraint() {
    let graph = create_advanced_test_graph();
    let context = create_grid_heuristic_context();

    let pathfinder = ConstrainedAstar::new(ManhattanHeuristic).add_constraint(
        constraint_based::PathConstraintType::MaxCost(constraint_based::MaxCostConstraint::new(
            7.0,
        )),
    );

    let result = pathfinder.find_constrained_path(&graph, 0, 15, &context);

    match result {
        Ok(path_result) => {
            assert!(
                path_result.cost <= 7.0,
                "Path cost {} exceeds limit 7.0",
                path_result.cost
            );
        }
        Err(PathfindingError::NoPathExists { .. }) => {
            // Acceptable if no path exists within cost limit
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_req5_max_length_constraint() {
    let graph = create_advanced_test_graph();
    let context = create_grid_heuristic_context();

    let pathfinder = ConstrainedAstar::new(ManhattanHeuristic).add_constraint(
        constraint_based::PathConstraintType::MaxLength(
            constraint_based::MaxLengthConstraint::new(5),
        ),
    );

    let result = pathfinder.find_constrained_path(&graph, 0, 3, &context);

    match result {
        Ok(path_result) => {
            assert!(
                path_result.path.len() <= 5,
                "Path length {} exceeds limit 5",
                path_result.path.len()
            );
        }
        Err(PathfindingError::NoPathExists { .. }) => {
            // Acceptable if no path exists within length limit
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_req5_no_cycle_constraint() {
    let graph = create_advanced_test_graph();
    let context = create_grid_heuristic_context();

    let pathfinder = ConstrainedAstar::new(ManhattanHeuristic).add_constraint(
        constraint_based::PathConstraintType::NoCycle(constraint_based::NoCycleConstraint),
    );

    let result = pathfinder
        .find_constrained_path(&graph, 0, 15, &context)
        .unwrap();

    // Verify no cycles in path
    let mut visited = HashSet::new();
    for &node in &result.path {
        assert!(
            !visited.contains(&node),
            "Cycle detected: node {} appears twice in path {:?}",
            node,
            result.path
        );
        visited.insert(node);
    }
}

#[test]
fn test_req5_multiple_constraints_combination() {
    let graph = create_advanced_test_graph();
    let context = create_grid_heuristic_context();

    let pathfinder = ConstrainedAstar::new(ManhattanHeuristic)
        .add_constraint(constraint_based::PathConstraintType::ForbiddenZone(
            constraint_based::ForbiddenZoneConstraint::new().add_forbidden_node(5),
        ))
        .add_constraint(constraint_based::PathConstraintType::MaxLength(
            constraint_based::MaxLengthConstraint::new(8),
        ))
        .add_constraint(constraint_based::PathConstraintType::NoCycle(
            constraint_based::NoCycleConstraint,
        ));

    let result = pathfinder
        .find_constrained_path(&graph, 0, 15, &context)
        .unwrap();

    // Check all constraints are satisfied
    assert!(!result.path.contains(&5)); // No forbidden zones
    assert!(result.path.len() <= 8); // Max length

    // No cycles
    let mut visited = HashSet::new();
    for &node in &result.path {
        assert!(!visited.contains(&node));
        visited.insert(node);
    }
}

#[test]
fn test_req5_constraint_preset_functions() {
    let graph = create_advanced_test_graph();
    let context = create_grid_heuristic_context();

    // Test avoid_nodes preset
    let pathfinder = constraint_based::presets::avoid_zones(ManhattanHeuristic, &[5, 10]);
    let result = pathfinder
        .find_constrained_path(&graph, 0, 15, &context)
        .unwrap();
    assert!(!result.path.contains(&5));
    assert!(!result.path.contains(&10));

    // Test limited_search preset
    let pathfinder = constraint_based::presets::limited_search(ManhattanHeuristic, 10.0, 8);
    let result = pathfinder.find_constrained_path(&graph, 0, 12, &context);
    assert!(result.is_ok());

    // Test acyclic_search preset
    let pathfinder = constraint_based::presets::acyclic_search(ManhattanHeuristic);
    let result = pathfinder
        .find_constrained_path(&graph, 0, 15, &context)
        .unwrap();

    let mut visited = HashSet::new();
    for &node in &result.path {
        assert!(!visited.contains(&node));
        visited.insert(node);
    }

    // Test safe_pathfinding preset
    let pathfinder =
        constraint_based::presets::safe_pathfinding(ManhattanHeuristic, &[6], 12.0, 10);
    let result = pathfinder
        .find_constrained_path(&graph, 0, 15, &context)
        .unwrap();
    assert!(!result.path.contains(&6));
}

#[test]
fn test_req5_impossible_constraints() {
    let graph = create_advanced_test_graph();
    let context = create_grid_heuristic_context();

    // Create impossible constraints by forbidding the goal node
    let pathfinder = ConstrainedAstar::new(ManhattanHeuristic).add_constraint(
        constraint_based::PathConstraintType::ForbiddenZone(
            constraint_based::ForbiddenZoneConstraint::new().add_forbidden_node(15),
        ),
    );

    let result = pathfinder.find_constrained_path(&graph, 0, 15, &context);

    assert!(result.is_err());
    match result.unwrap_err() {
        PathfindingError::NoPathExists { start, goal } => {
            assert_eq!(start, 0);
            assert_eq!(goal, 15);
        }
        e => panic!("Expected NoPathExists, got {:?}", e),
    }
}

// ===== Advanced Heuristic Tests =====

#[test]
fn test_req5_weighted_combination_heuristic() {
    let graph = create_advanced_test_graph();
    let context = create_grid_heuristic_context();

    let weighted_heuristic = heuristic::WeightedCombinationHeuristic::new()
        .add_manhattan(0.7)
        .add_euclidean(0.3);

    let pathfinder = AstarPathfinder::new(weighted_heuristic);
    let result = pathfinder
        .find_path_with_context(&graph, 0, 15, &context)
        .unwrap();

    assert_eq!(result.path.first(), Some(&0));
    assert_eq!(result.path.last(), Some(&15));
    assert!(result.cost > 0.0);
    assert!(result.nodes_explored > 0);
}

#[test]
fn test_req5_heuristic_admissibility() {
    let manhattan = ManhattanHeuristic;
    let euclidean = EuclideanHeuristic;
    let chebyshev = heuristic::ChebyshevHeuristic;
    let zero = heuristic::ZeroHeuristic;

    assert!(manhattan.is_admissible());
    assert!(euclidean.is_admissible());
    assert!(chebyshev.is_admissible());
    assert!(zero.is_admissible());

    let weighted = heuristic::WeightedCombinationHeuristic::new()
        .add_manhattan(0.5)
        .add_euclidean(0.5);
    assert!(weighted.is_admissible());

    // Test with negative weight (should not be admissible)
    let invalid_weighted = heuristic::WeightedCombinationHeuristic::new()
        .add_manhattan(-0.5) // Negative weight
        .add_euclidean(1.0);
    assert!(!invalid_weighted.is_admissible());
}

// ===== Integration and Performance Tests =====

#[test]
fn test_req5_multi_objective_with_constraints() {
    // This test combines multi-objective optimization with constraints
    // It's more complex but tests real-world scenarios

    let graph = create_advanced_test_graph();
    let mut safety_objective = multi_objective::SafetyObjective::new(4.0);
    safety_objective.add_danger_zones(&[5, 6]);

    let pathfinder = MultiObjectiveAstar::new()
        .add_objective(multi_objective::CostObjective)
        .add_objective(safety_objective)
        .with_max_solutions(20);

    let result = pathfinder.find_pareto_optimal_paths(&graph, 0, 15).unwrap();

    assert!(!result.pareto_optimal_solutions.is_empty());
    assert!(result.pareto_optimal_solutions.len() <= 20);

    // Check that solutions respect both objectives
    for solution in &result.pareto_optimal_solutions {
        assert_eq!(solution.objective_values.len(), 2);
        assert!(solution.objective_values[0] > 0.0); // Cost
        assert!(solution.objective_values[1] >= 0.0); // Safety
    }
}

#[test]
fn test_req5_performance_with_large_objectives() {
    let graph = create_advanced_test_graph();
    let time_objective = multi_objective::TimeObjective::new();
    let safety_objective = multi_objective::SafetyObjective::new(1.0);

    let pathfinder = MultiObjectiveAstar::new()
        .add_objective(multi_objective::CostObjective)
        .add_objective(time_objective)
        .add_objective(safety_objective)
        .with_max_solutions(10) // Limit to keep test fast
        .with_epsilon(0.2); // Use approximation for speed

    let start_time = std::time::Instant::now();
    let result = pathfinder.find_pareto_optimal_paths(&graph, 0, 15).unwrap();
    let elapsed = start_time.elapsed();

    assert!(elapsed.as_millis() < 1000); // Should complete within 1 second
    assert!(!result.pareto_optimal_solutions.is_empty());
    // The algorithm might find slightly more than the limit during processing
    assert!(result.pareto_optimal_solutions.len() <= 12);
}

#[test]
fn test_req5_constraint_validation_edge_cases() {
    let graph = create_advanced_test_graph();
    let context = create_grid_heuristic_context();

    // Test with start node forbidden (should fail)
    let pathfinder = ConstrainedAstar::new(ManhattanHeuristic).add_constraint(
        constraint_based::PathConstraintType::ForbiddenZone(
            constraint_based::ForbiddenZoneConstraint::new().add_forbidden_node(0),
        ),
    );

    let result = pathfinder.find_constrained_path(&graph, 0, 15, &context);
    assert!(result.is_err());

    // Test with goal node reachable but path restricted
    let pathfinder = ConstrainedAstar::new(ManhattanHeuristic).add_constraint(
        constraint_based::PathConstraintType::MaxLength(
            constraint_based::MaxLengthConstraint::new(1),
        ),
    );

    let result = pathfinder.find_constrained_path(&graph, 0, 15, &context);
    // Should fail because max length=1 means we can't take any steps from start
    assert!(result.is_err());
}

#[test]
fn test_req5_algorithm_correctness_verification() {
    let graph = create_advanced_test_graph();

    // Compare multi-objective with single objective (should match when using only cost)
    let multi_obj_pathfinder =
        MultiObjectiveAstar::new().add_objective(multi_objective::CostObjective);

    let single_obj_pathfinder = AstarPathfinder::new(heuristic::ZeroHeuristic);

    let multi_result = multi_obj_pathfinder
        .find_pareto_optimal_paths(&graph, 0, 7)
        .unwrap();
    let single_result = single_obj_pathfinder.find_path(&graph, 0, 7).unwrap();

    assert_eq!(multi_result.pareto_optimal_solutions.len(), 1);
    let multi_solution = &multi_result.pareto_optimal_solutions[0];

    // Costs should be equal (or very close due to floating point)
    assert!((multi_solution.total_cost - single_result.cost).abs() < 1e-10);
}

#[test]
fn test_req5_comprehensive_integration() {
    // Final integration test combining all Day 5 features
    let graph = create_advanced_test_graph();
    let context = create_grid_heuristic_context();

    // 1. Test multi-objective pathfinding
    let mut time_objective = multi_objective::TimeObjective::new();
    time_objective.set_time_multiplier(1, 5, 0.5);

    let multi_pathfinder = MultiObjectiveAstar::new()
        .add_objective(multi_objective::CostObjective)
        .add_objective(time_objective);

    let multi_result = multi_pathfinder
        .find_pareto_optimal_paths(&graph, 0, 8)
        .unwrap();
    assert!(!multi_result.pareto_optimal_solutions.is_empty());

    // 2. Test constraint-based pathfinding
    let constrained_pathfinder = ConstrainedAstar::new(ManhattanHeuristic)
        .add_constraint(constraint_based::PathConstraintType::MaxCost(
            constraint_based::MaxCostConstraint::new(10.0),
        ))
        .add_constraint(constraint_based::PathConstraintType::NoCycle(
            constraint_based::NoCycleConstraint,
        ));

    let constrained_result = constrained_pathfinder
        .find_constrained_path(&graph, 0, 8, &context)
        .unwrap();
    assert!(constrained_result.cost <= 10.0);

    // 3. Test weighted heuristic
    let weighted_heuristic = heuristic::WeightedCombinationHeuristic::new()
        .add_manhattan(0.6)
        .add_euclidean(0.4);

    let heuristic_pathfinder = AstarPathfinder::new(weighted_heuristic);
    let heuristic_result = heuristic_pathfinder
        .find_path_with_context(&graph, 0, 8, &context)
        .unwrap();

    // All pathfinders should find valid paths
    assert_eq!(
        multi_result.pareto_optimal_solutions[0].path.first(),
        Some(&0)
    );
    assert_eq!(constrained_result.path.first(), Some(&0));
    assert_eq!(heuristic_result.path.first(), Some(&0));

    assert_eq!(
        multi_result.pareto_optimal_solutions[0].path.last(),
        Some(&8)
    );
    assert_eq!(constrained_result.path.last(), Some(&8));
    assert_eq!(heuristic_result.path.last(), Some(&8));
}
