use mission8::*;
use std::collections::HashMap;

/// Comprehensive demonstration of all Mission 8 BFS/DFS algorithms
///
/// This example showcases:
/// - Basic BFS and DFS traversal
/// - Shortest path finding
/// - Cycle detection
/// - Connected components analysis
/// - Real-world problem solving
fn main() {
    println!("=== Mission 8: BFS/DFS Algorithms Demonstration ===\n");

    // Create sample graphs for demonstration
    let simple_graph = create_simple_graph();
    let cyclic_graph = create_cyclic_graph();
    let disconnected_graph = create_disconnected_graph();

    demonstrate_basic_traversals(&simple_graph);
    demonstrate_shortest_path(&simple_graph);
    demonstrate_cycle_detection(&cyclic_graph);
    demonstrate_connected_components(&disconnected_graph);
    demonstrate_error_handling(&simple_graph);
    demonstrate_performance_characteristics();

    println!("🎉 Mission 8 demonstration complete!");
    println!("📖 Next steps:");
    println!("  • Try the maze solver: cargo run -p mission8 --example maze_solver");
    println!("  • Analyze networks: cargo run -p mission8 --example network_analyzer");
    println!("  • Resolve dependencies: cargo run -p mission8 --example dependency_resolver");
}

/// Demonstrate basic BFS and DFS traversals
fn demonstrate_basic_traversals(graph: &HashMap<u32, Vec<u32>>) {
    println!("📚 Section 1: Basic Graph Traversals");
    println!("===================================");

    // Show the graph structure
    println!("🔗 Graph structure:");
    for (node, neighbors) in graph {
        println!("  {} -> {:?}", node, neighbors);
    }
    println!();

    // BFS traversal
    println!("🔍 BFS Traversal (breadth-first, level-order):");
    let bfs_result = bfs(graph, 0);
    println!("  Starting from node 0: {:?}", bfs_result);
    println!("  ✅ BFS visits nodes level by level (distance 0, 1, 2, ...)");

    // DFS traversal
    println!("\n🔍 DFS Traversal (depth-first, go deep before wide):");
    let dfs_result = dfs(graph, 0);
    println!("  Starting from node 0: {:?}", dfs_result);
    println!("  ✅ DFS explores deeply before backtracking");

    println!("\n💡 Key Insights:");
    println!("  • BFS and DFS visit same nodes but in different order");
    println!("  • Both have O(V + E) time complexity");
    println!("  • Choice depends on what you're trying to solve\n");
}

/// Demonstrate shortest path finding with BFS
fn demonstrate_shortest_path(graph: &HashMap<u32, Vec<u32>>) {
    println!("📚 Section 2: Shortest Path Finding");
    println!("==================================");

    println!("🎯 Finding shortest paths in the graph:");

    // Test several path queries
    let test_cases = vec![
        (0, 3, "Easy path"),
        (0, 2, "Direct neighbor"),
        (1, 3, "Longer path"),
    ];

    for (start, end, description) in test_cases {
        match shortest_path(graph, start, end) {
            Ok(path) => {
                println!(
                    "  {} ({}→{}): {:?} (length: {})",
                    description,
                    start,
                    end,
                    path,
                    path.len() - 1
                );
            }
            Err(e) => {
                println!("  {} ({}→{}): Error - {:?}", description, start, end, e);
            }
        }
    }

    println!("\n💡 Why BFS finds shortest path:");
    println!("  • BFS explores nodes by distance from start");
    println!("  • First time we reach target = shortest path found");
    println!("  • Only works for unweighted graphs\n");
}

/// Demonstrate cycle detection algorithms
fn demonstrate_cycle_detection(graph: &HashMap<u32, Vec<u32>>) {
    println!("📚 Section 3: Cycle Detection");
    println!("============================");

    println!("🔗 Cyclic graph structure:");
    for (node, neighbors) in graph {
        println!("  {} -> {:?}", node, neighbors);
    }

    // Check for cycles
    let has_cycles = has_cycle(graph);
    println!(
        "\n🔍 Cycle detection result: {}",
        if has_cycles {
            "CYCLE FOUND ❌"
        } else {
            "NO CYCLES ✅"
        }
    );

    // Find actual cycle if it exists
    if let Some(cycle) = find_cycle(graph) {
        println!("📍 Actual cycle found: {:?}", cycle);
        println!(
            "   This means: {} → ... → {} → {} (completes cycle)",
            cycle[0],
            cycle[cycle.len() - 1],
            cycle[0]
        );
    }

    println!("\n💡 How cycle detection works:");
    println!("  • Use DFS with back-edge detection");
    println!("  • Track node states: White → Gray → Black");
    println!("  • Back edge to Gray node = cycle found");
    println!("  • Applications: dependency analysis, deadlock detection\n");
}

/// Demonstrate connected components analysis
fn demonstrate_connected_components(graph: &HashMap<u32, Vec<u32>>) {
    println!("📚 Section 4: Connected Components Analysis");
    println!("==========================================");

    println!("🔗 Disconnected graph structure:");
    for (node, neighbors) in graph {
        println!("  {} -> {:?}", node, neighbors);
    }

    let components = connected_components(graph);

    println!("\n🏝️  Connected components found: {}", components.len());
    for (i, component) in components.iter().enumerate() {
        println!(
            "  Component {}: {:?} ({} nodes)",
            i + 1,
            component,
            component.len()
        );
    }

    println!("\n💡 Connected components applications:");
    println!("  • Social network analysis (friend groups)");
    println!("  • Network segmentation (isolated subnets)");
    println!("  • Image processing (connected regions)");
    println!("  • Supply chain analysis (supplier clusters)\n");
}

/// Demonstrate error handling and edge cases
fn demonstrate_error_handling(graph: &HashMap<u32, Vec<u32>>) {
    println!("📚 Section 5: Error Handling & Edge Cases");
    println!("=========================================");

    println!("🧪 Testing error conditions:");

    // Test with non-existent start node
    match shortest_path(graph, 999, 0) {
        Ok(_) => println!("  ❌ Should have failed with non-existent start node"),
        Err(e) => println!("  ✅ Non-existent start node (999): {:?}", e),
    }

    // Test with non-existent end node
    match shortest_path(graph, 0, 999) {
        Ok(_) => println!("  ❌ Should have failed with non-existent end node"),
        Err(e) => println!("  ✅ Non-existent end node (999): {:?}", e),
    }

    // Test path to unreachable node (using disconnected graph)
    let disconnected = create_disconnected_graph();
    match shortest_path(&disconnected, 0, 5) {
        Ok(_) => println!("  ❌ Should have failed - nodes not connected"),
        Err(e) => println!("  ✅ Unreachable destination (0→5): {:?}", e),
    }

    println!("\n💡 Robust error handling:");
    println!("  • Validate input nodes exist in graph");
    println!("  • Return meaningful error messages");
    println!("  • Use Result<T, E> for recoverable errors");
    println!("  • Enable caller to handle failures gracefully\n");
}

/// Demonstrate performance characteristics and optimization tips
fn demonstrate_performance_characteristics() {
    println!("📚 Section 6: Performance Characteristics");
    println!("=========================================");

    // Create graphs of different sizes for timing
    let small_graph = create_performance_graph(100);
    let medium_graph = create_performance_graph(1000);

    println!("⏱️  Performance measurements:");

    // Time BFS on different graph sizes
    let start_time = std::time::Instant::now();
    let _result = bfs(&small_graph, 0);
    let small_time = start_time.elapsed();

    let start_time = std::time::Instant::now();
    let _result = bfs(&medium_graph, 0);
    let medium_time = start_time.elapsed();

    println!("  BFS on 100-node graph:   {:?}", small_time);
    println!("  BFS on 1000-node graph:  {:?}", medium_time);
    println!(
        "  Scaling factor: {:.1}x (should be ~10x for linear)",
        medium_time.as_nanos() as f64 / small_time.as_nanos() as f64
    );

    println!("\n⚡ Performance optimization tips:");
    println!("  • Use Vec<bool> instead of HashSet<NodeId> for visited tracking (3x faster)");
    println!("  • Pre-allocate collections: VecDeque::with_capacity(), HashSet::with_capacity()");
    println!("  • Choose iterative over recursive DFS (avoids stack overflow)");
    println!("  • Consider graph representation: HashMap vs Vec for adjacency lists");
    println!("  • Use early termination when target found in shortest_path");

    println!("\n📊 Complexity summary:");
    println!("  Algorithm              | Time      | Space     | Use Case");
    println!("  ---------------------- | --------- | --------- | ------------------");
    println!("  BFS                    | O(V + E)  | O(V)      | Shortest path, levels");
    println!("  DFS                    | O(V + E)  | O(V)      | Cycles, components");
    println!("  Shortest Path (BFS)    | O(V + E)  | O(V)      | Unweighted graphs");
    println!("  Cycle Detection (DFS)  | O(V + E)  | O(V)      | Dependency analysis");
    println!("  Connected Components   | O(V + E)  | O(V)      | Network analysis");
    println!();
}

// Helper functions to create test graphs

fn create_simple_graph() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]); // 0 connects to 1 and 2
    graph.insert(1, vec![3]); // 1 connects to 3
    graph.insert(2, vec![3]); // 2 connects to 3
    graph.insert(3, vec![]); // 3 has no outgoing edges
    graph
}

fn create_cyclic_graph() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]); // 0 → 1
    graph.insert(1, vec![2]); // 1 → 2
    graph.insert(2, vec![0, 3]); // 2 → 0, 3 (creates cycle: 0→1→2→0)
    graph.insert(3, vec![]); // 3 has no outgoing edges
    graph
}

fn create_disconnected_graph() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    // Component 1: {0, 1, 2}
    graph.insert(0, vec![1]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![]);

    // Component 2: {3, 4}
    graph.insert(3, vec![4]);
    graph.insert(4, vec![]);

    // Component 3: {5} (isolated)
    graph.insert(5, vec![]);

    graph
}

fn create_performance_graph(size: u32) -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    for i in 0..size {
        // Create a sparse graph where each node connects to next node and one random node
        let neighbors = if i < size - 1 {
            vec![i + 1, (i + size / 2) % size]
        } else {
            vec![0] // Last node connects back to start
        };
        graph.insert(i, neighbors);
    }
    graph
}
