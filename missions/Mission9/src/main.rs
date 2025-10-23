use mission9::{
    AstarPathfinder, DijkstraPathfinder, EuclideanHeuristic, Heuristic, HeuristicContext,
    ManhattanHeuristic, Pathfinder, SimpleWeightedGraph, WeightedGraph,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Mission 9: Advanced Pathfinding Algorithms");
    println!("================================================");

    // Create a simple test graph
    let mut graph = SimpleWeightedGraph::new();

    // Build a diamond-shaped graph for demonstration
    //     0
    //    / \
    //   1   2
    //    \ /
    //     3
    graph.add_edge(0, 1, 2.0);
    graph.add_edge(0, 2, 6.0);
    graph.add_edge(1, 3, 3.0);
    graph.add_edge(2, 3, 1.0);

    println!("\n📊 Graph Structure:");
    println!("Nodes: {}", graph.node_count());
    println!("Edges: {}", graph.edge_count());
    println!("Path: 0 → 1 (cost: 2.0), 0 → 2 (cost: 6.0)");
    println!("Path: 1 → 3 (cost: 3.0), 2 → 3 (cost: 1.0)");

    // Test Dijkstra's algorithm
    println!("\n🔍 Dijkstra's Algorithm:");
    println!("-----------------------");

    let dijkstra = DijkstraPathfinder::new();
    let dijkstra_result = dijkstra.find_path(&graph, 0, 3)?;

    println!("Algorithm: {}", dijkstra.algorithm_name());
    println!("Time Complexity: {}", dijkstra.time_complexity());
    println!("Space Complexity: {}", dijkstra.space_complexity());
    println!("Path: {:?}", dijkstra_result.path);
    println!("Cost: {:.1}", dijkstra_result.cost);
    println!("Nodes Explored: {}", dijkstra_result.nodes_explored);
    println!("Search Time: {}ms", dijkstra_result.search_time_ms);

    // Test A* algorithm (with zero heuristic - equivalent to Dijkstra)
    println!("\n⭐ A* Algorithm (Euclidean Heuristic):");
    println!("-------------------------------------");

    let astar = AstarPathfinder::new(EuclideanHeuristic);
    let astar_result = astar.find_path(&graph, 0, 3)?;

    println!("Algorithm: {}", astar.algorithm_name());
    println!("Time Complexity: {}", astar.time_complexity());
    println!("Space Complexity: {}", astar.space_complexity());
    println!("Heuristic: {}", astar.heuristic().name());
    println!("Path: {:?}", astar_result.path);
    println!("Cost: {:.1}", astar_result.cost);
    println!("Nodes Explored: {}", astar_result.nodes_explored);
    println!("Search Time: {}ms", astar_result.search_time_ms);

    // Demonstrate coordinate-based A* on a grid
    println!("\n🗺️  Grid-based A* with Manhattan Heuristic:");
    println!("-------------------------------------------");

    let mut grid_graph = SimpleWeightedGraph::new();
    let mut context = HeuristicContext::new();

    // Create a simple 3x3 grid for coordinate-based pathfinding
    for y in 0..3 {
        for x in 0..3 {
            let node = y * 3 + x;
            context.add_coordinates(node, x as f64, y as f64);

            // Add horizontal connections
            if x < 2 {
                let right = y * 3 + (x + 1);
                grid_graph.add_undirected_edge(node, right, 1.0);
            }

            // Add vertical connections
            if y < 2 {
                let below = (y + 1) * 3 + x;
                grid_graph.add_undirected_edge(node, below, 1.0);
            }
        }
    }

    let grid_astar = AstarPathfinder::new(ManhattanHeuristic);
    let grid_result = grid_astar.find_path_with_context(&grid_graph, 0, 8, &context)?;

    println!("Grid Size: 3x3 ({} nodes)", grid_graph.node_count());
    println!("Start: (0,0) → Goal: (2,2)");
    println!("Heuristic: {}", grid_astar.heuristic().name());
    println!("Path: {:?}", grid_result.path);
    println!("Cost: {:.1}", grid_result.cost);
    println!("Nodes Explored: {}", grid_result.nodes_explored);
    println!("Search Time: {}ms", grid_result.search_time_ms);

    // Performance comparison
    println!("\n📈 Performance Comparison:");
    println!("-------------------------");
    println!("Dijkstra vs A* (simple graph):");
    println!(
        "  Dijkstra explored {} nodes",
        dijkstra_result.nodes_explored
    );
    println!("  A* explored {} nodes", astar_result.nodes_explored);

    if dijkstra_result.nodes_explored > astar_result.nodes_explored {
        println!("  🏆 A* was more efficient!");
    } else if dijkstra_result.nodes_explored < astar_result.nodes_explored {
        println!("  🏆 Dijkstra was more efficient!");
    } else {
        println!("  🤝 Both algorithms performed equally!");
    }

    println!("\n✅ Mission 9 foundation complete!");
    println!("📚 Next steps: Continue with Mission9_tut for guided learning");

    Ok(())
}
