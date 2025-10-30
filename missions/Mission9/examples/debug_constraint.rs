use mission9::*;

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

fn main() {
    let graph = create_advanced_test_graph();
    let pathfinder = AstarPathfinder::new(heuristic::ZeroHeuristic);
    
    println!("Testing unconstrained path from 0 to 15:");
    match pathfinder.find_path(&graph, 0, 15) {
        Ok(result) => {
            println!("Path: {:?}", result.path);
            println!("Path length: {}", result.path.len());
            println!("Cost: {}", result.cost);
        }
        Err(e) => println!("Error: {:?}", e),
    }
    
    println!("\nTesting with MaxLength=3 constraint:");
    let constrained_pathfinder = constraint_based::ConstrainedAstar::new(heuristic::ZeroHeuristic)
        .add_constraint(constraint_based::PathConstraintType::MaxLength(
            constraint_based::MaxLengthConstraint::new(3)
        ));
    
    let context = HeuristicContext::new(); // Default empty context
    match constrained_pathfinder.find_constrained_path(&graph, 0, 15, &context) {
        Ok(result) => {
            println!("ERROR: Found path when should have failed!");
            println!("Path: {:?}", result.path);
            println!("Path length: {}", result.path.len());
        }
        Err(e) => println!("Correctly failed: {:?}", e),
    }
}