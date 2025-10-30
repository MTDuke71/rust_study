#[test]
fn debug_constraint_pathfinding() {
    use mission9::*;
    
    let graph = create_advanced_test_graph();
    let context = create_grid_heuristic_context();
    
    // Test basic A* first
    let basic_pathfinder = AstarPathfinder::new(ManhattanHeuristic);
    let basic_result = basic_pathfinder.find_path_with_context(&graph, 0, 15, &context);
    println!("Basic A* result: {:?}", basic_result);
    
    // Test constraint-based without constraints
    let constraint_pathfinder = ConstrainedAstar::new(ManhattanHeuristic);
    let constraint_result = constraint_pathfinder.find_constrained_path(&graph, 0, 15, &context);
    println!("Constraint-based result: {:?}", constraint_result);
}
