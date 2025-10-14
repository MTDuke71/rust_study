//! Step 6: BFS Implementation
//!
//! This step implements breadth-first search (BFS) with shortest path capabilities,
//! level-order traversal, and distance calculation.
//!
//! ## Learning Objectives
//! - Implement breadth-first search using VecDeque
//! - Learn shortest path finding (unweighted)
//! - Understand level-order traversal
//! - Master distance calculation and path reconstruction
//!
//! ## Alignment with Mission 7
//! This step directly implements **REQ-6: BFS Implementation** by teaching
//! comprehensive breadth-first search techniques.
//!
//! ## Calendar Alignment
//! **Day 6 (Oct 13)**: BFS Implementation
//! - Implement breadth-first search using VecDeque
//! - Shortest path finding (unweighted)
//! - Level-order traversal

use mission7::{Graph, NodeId};
use std::collections::{HashSet, VecDeque};

fn main() {
    println!("=== Step 6: BFS Implementation ===\n");

    // 1. Understanding BFS
    understanding_bfs();

    // 2. BFS Implementation
    bfs_implementation();

    // 3. Shortest Path with BFS
    shortest_path_with_bfs();

    // 4. Level-Order Traversal
    level_order_traversal();

    // 5. Distance Calculation
    distance_calculation();

    // 6. BFS vs DFS Comparison
    bfs_vs_dfs_comparison();

    exercise();

    println!("\n=== Step 6 Complete ===");
    println!("Next: Step 7 - Integration Project");
}

fn understanding_bfs() {
    println!("1. Understanding BFS");
    println!("==================");

    println!("Breadth-First Search (BFS) Characteristics:");
    println!("  • Explores all nodes at the current level before going deeper");
    println!("  • Uses a queue (FIFO - First In, First Out)");
    println!("  • Guarantees shortest path in unweighted graphs");
    println!("  • Good for level-order traversal and shortest path finding");
    println!();

    // Create a tree-like graph for demonstration
    let mut graph = Graph::new_directed();
    let root = graph.add_node("Root");
    let left = graph.add_node("Left");
    let right = graph.add_node("Right");
    let left_left = graph.add_node("Left-Left");
    let left_right = graph.add_node("Left-Right");
    let right_left = graph.add_node("Right-Left");
    let right_right = graph.add_node("Right-Right");

    graph.add_edge(root, left);
    graph.add_edge(root, right);
    graph.add_edge(left, left_left);
    graph.add_edge(left, left_right);
    graph.add_edge(right, right_left);
    graph.add_edge(right, right_right);

    println!("Example Tree Structure:");
    println!("        Root");
    println!("       /    \\");
    println!("    Left    Right");
    println!("   /   \\    /   \\");
    println!(" L-L  L-R  R-L  R-R");
    println!();

    println!("BFS Traversal Order (level by level):");
    println!("  Level 0: Root");
    println!("  Level 1: Left, Right");
    println!("  Level 2: Left-Left, Left-Right, Right-Left, Right-Right");
    println!();

    println!("BFS vs DFS Comparison:");
    println!(
        "  BFS: Root -> Left -> Right -> Left-Left -> Left-Right -> Right-Left -> Right-Right"
    );
    println!(
        "  DFS: Root -> Left -> Left-Left -> Left-Right -> Right -> Right-Left -> Right-Right"
    );
    println!();

    println!("BFS Advantages:");
    println!("  • Finds shortest path in unweighted graphs");
    println!("  • Explores nodes level by level");
    println!("  • Good for finding nodes at specific distances");
    println!();
}

fn bfs_implementation() {
    println!("2. BFS Implementation");
    println!("===================");

    println!("BFS Implementation Steps:");
    println!("  1. Create a queue and add the starting node");
    println!("  2. Mark the starting node as visited");
    println!("  3. While queue is not empty:");
    println!("     a. Dequeue a node");
    println!("     b. Process the node");
    println!("     c. Enqueue all unvisited neighbors");
    println!("     d. Mark neighbors as visited");
    println!();

    // Create a graph for demonstration
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");

    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);
    graph.add_edge(c, e);

    println!("Graph for BFS:");
    println!("    A");
    println!("   / \\");
    println!("  B   C");
    println!("  |   |");
    println!("  D   E");
    println!();

    // BFS implementation
    fn bfs_traversal<T: std::fmt::Display>(
        graph: &Graph<T>,
        start: NodeId,
    ) -> (Vec<NodeId>, HashSet<NodeId>) {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            path.push(node);
            let node_name = graph.get_node(node).unwrap();
            println!("  Visit: {}", node_name);

            // Add all unvisited neighbors to the queue
            for &neighbor in graph.neighbors(node) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                    let neighbor_name = graph.get_node(neighbor).unwrap();
                    println!("    Add to queue: {}", neighbor_name);
                }
            }
        }

        (path, visited)
    }

    println!("BFS Traversal:");
    let (path, visited) = bfs_traversal(&graph, a);

    println!();
    println!("BFS Result:");
    println!("  Path: {:?}", path);
    println!("  Visited: {:?}", visited);
    println!();

    // Show the queue operations
    println!("Queue Operations Simulation:");
    println!("  Enqueue A");
    println!("  Dequeue A, visit A, enqueue B, enqueue C");
    println!("  Dequeue B, visit B, enqueue D");
    println!("  Dequeue C, visit C, enqueue E");
    println!("  Dequeue D, visit D");
    println!("  Dequeue E, visit E");
    println!();
}

fn shortest_path_with_bfs() {
    println!("3. Shortest Path with BFS");
    println!("========================");

    println!("BFS guarantees shortest path in unweighted graphs:");
    println!("  • First time we visit a node, we found the shortest path to it");
    println!("  • Use parent array to reconstruct the path");
    println!("  • Distance is the number of edges in the path");
    println!();

    // Create a graph with multiple paths
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    let f = graph.add_node("F");

    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);
    graph.add_edge(c, d);
    graph.add_edge(d, e);
    graph.add_edge(e, f);

    println!("Graph with Multiple Paths:");
    println!("    A");
    println!("   / \\");
    println!("  B   C");
    println!("  |   |");
    println!("  D---D");
    println!("  |");
    println!("  E");
    println!("  |");
    println!("  F");
    println!();

    // BFS shortest path implementation
    fn bfs_shortest_path<T>(graph: &Graph<T>, start: NodeId, end: NodeId) -> Option<Vec<NodeId>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parents = vec![None; graph.node_count()];

        queue.push_back(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            if node == end {
                // Reconstruct path
                let mut path = Vec::new();
                let mut current = end;

                while let Some(parent) = parents[current] {
                    path.push(current);
                    current = parent;
                }
                path.push(start);
                path.reverse();

                return Some(path);
            }

            for &neighbor in graph.neighbors(node) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    parents[neighbor] = Some(node);
                    queue.push_back(neighbor);
                }
            }
        }

        None
    }

    println!("BFS Shortest Path Finding:");
    let start = a;
    let end = f;

    if let Some(path) = bfs_shortest_path(&graph, start, end) {
        println!("  Shortest path from A to F found:");
        for (i, &node_id) in path.iter().enumerate() {
            let node_name = graph.get_node(node_id).unwrap();
            if i == 0 {
                print!("    {}", node_name);
            } else {
                print!(" -> {}", node_name);
            }
        }
        println!();
        println!("  Path length: {} edges", path.len() - 1);
    } else {
        println!("  No path found from A to F");
    }
    println!();

    // Show why BFS finds shortest path
    println!("Why BFS Finds Shortest Path:");
    println!("  • BFS explores nodes level by level");
    println!("  • First time we visit a node, we found the shortest path");
    println!("  • All paths of length k are explored before paths of length k+1");
    println!("  • This guarantees the shortest path in unweighted graphs");
    println!();
}

fn level_order_traversal() {
    println!("4. Level-Order Traversal");
    println!("=======================");

    println!("Level-order traversal processes nodes level by level:");
    println!("  • Level 0: starting node");
    println!("  • Level 1: nodes at distance 1");
    println!("  • Level 2: nodes at distance 2");
    println!("  • And so on...");
    println!();

    // Create a tree for demonstration
    let mut graph = Graph::new_directed();
    let root = graph.add_node("Root");
    let level1_1 = graph.add_node("L1-1");
    let level1_2 = graph.add_node("L1-2");
    let level2_1 = graph.add_node("L2-1");
    let level2_2 = graph.add_node("L2-2");
    let level2_3 = graph.add_node("L2-3");
    let level3_1 = graph.add_node("L3-1");
    let level3_2 = graph.add_node("L3-2");

    graph.add_edge(root, level1_1);
    graph.add_edge(root, level1_2);
    graph.add_edge(level1_1, level2_1);
    graph.add_edge(level1_1, level2_2);
    graph.add_edge(level1_2, level2_3);
    graph.add_edge(level2_1, level3_1);
    graph.add_edge(level2_2, level3_2);

    println!("Tree Structure:");
    println!("      Root");
    println!("     /    \\");
    println!("  L1-1    L1-2");
    println!("  /  \\      |");
    println!("L2-1 L2-2  L2-3");
    println!(" |    |");
    println!("L3-1 L3-2");
    println!();

    // Level-order traversal implementation
    fn level_order_traversal<T>(graph: &Graph<T>, start: NodeId) -> Vec<Vec<NodeId>> {
        let mut levels = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited.insert(start);

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut current_level = Vec::new();

            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    current_level.push(node);

                    // Add children to queue for next level
                    for &neighbor in graph.neighbors(node) {
                        if !visited.contains(&neighbor) {
                            visited.insert(neighbor);
                            queue.push_back(neighbor);
                        }
                    }
                }
            }

            levels.push(current_level);
        }

        levels
    }

    println!("Level-Order Traversal:");
    let levels = level_order_traversal(&graph, root);

    for (i, level) in levels.iter().enumerate() {
        let level_names: Vec<String> = level
            .iter()
            .map(|&id| graph.get_node(id).unwrap().to_string())
            .collect();
        println!("  Level {}: [{}]", i, level_names.join(", "));
    }
    println!();

    // Show the traversal process
    println!("Traversal Process:");
    println!("  Level 0: Start with Root");
    println!("  Level 1: Process Root, add L1-1 and L1-2 to queue");
    println!("  Level 2: Process L1-1 and L1-2, add their children to queue");
    println!("  Level 3: Process L2-1, L2-2, and L2-3, add their children to queue");
    println!("  Level 4: Process L3-1 and L3-2");
    println!();
}

fn distance_calculation() {
    println!("5. Distance Calculation");
    println!("=====================");

    println!("BFS can calculate distances from a starting node:");
    println!("  • Distance = number of edges in the shortest path");
    println!("  • Use distance array to track distances");
    println!("  • Initialize with infinity, update as we visit nodes");
    println!();

    // Create a graph for demonstration
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    let f = graph.add_node("F");

    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);
    graph.add_edge(c, d);
    graph.add_edge(d, e);
    graph.add_edge(e, f);

    println!("Graph for Distance Calculation:");
    println!("    A");
    println!("   / \\");
    println!("  B   C");
    println!("  |   |");
    println!("  D---D");
    println!("  |");
    println!("  E");
    println!("  |");
    println!("  F");
    println!();

    // BFS distance calculation implementation
    fn bfs_distances<T>(graph: &Graph<T>, start: NodeId) -> Vec<Option<usize>> {
        let mut distances = vec![None; graph.node_count()];
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited.insert(start);
        distances[start] = Some(0);

        while let Some(node) = queue.pop_front() {
            let current_distance = distances[node].unwrap();

            for &neighbor in graph.neighbors(node) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    distances[neighbor] = Some(current_distance + 1);
                    queue.push_back(neighbor);
                }
            }
        }

        distances
    }

    println!("BFS Distance Calculation:");
    let distances = bfs_distances(&graph, a);

    println!("  Distances from A:");
    for (i, distance) in distances.iter().enumerate() {
        if let Some(dist) = distance {
            let node_name = graph.get_node(i).unwrap();
            println!("    {}: distance {}", node_name, dist);
        }
    }
    println!();

    // Show the distance calculation process
    println!("Distance Calculation Process:");
    println!("  Start: A at distance 0");
    println!("  Level 1: B and C at distance 1");
    println!("  Level 2: D at distance 2");
    println!("  Level 3: E at distance 3");
    println!("  Level 4: F at distance 4");
    println!();

    // Find nodes at specific distances
    println!("Nodes at Specific Distances:");
    for distance in 0..=4 {
        let nodes_at_distance: Vec<String> = distances
            .iter()
            .enumerate()
            .filter(|(_, &dist)| dist == Some(distance))
            .map(|(i, _)| graph.get_node(i).unwrap().to_string())
            .collect();

        if !nodes_at_distance.is_empty() {
            println!(
                "  Distance {}: [{}]",
                distance,
                nodes_at_distance.join(", ")
            );
        }
    }
    println!();
}

fn bfs_vs_dfs_comparison() {
    println!("6. BFS vs DFS Comparison");
    println!("=======================");

    println!("BFS vs DFS Comparison:");
    println!();

    // Create a graph for comparison
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");

    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);
    graph.add_edge(c, e);

    println!("Graph for Comparison:");
    println!("    A");
    println!("   / \\");
    println!("  B   C");
    println!("  |   |");
    println!("  D   E");
    println!();

    // BFS implementation
    fn bfs<T>(graph: &Graph<T>, start: NodeId) -> Vec<NodeId> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            path.push(node);
            for &neighbor in graph.neighbors(node) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        path
    }

    // DFS implementation
    fn dfs<T>(graph: &Graph<T>, start: NodeId) -> Vec<NodeId> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        let mut stack = Vec::new();

        stack.push(start);
        visited.insert(start);

        while let Some(node) = stack.pop() {
            path.push(node);
            for &neighbor in graph.neighbors(node).iter().rev() {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    stack.push(neighbor);
                }
            }
        }

        path
    }

    println!("Traversal Comparison:");
    let bfs_path = bfs(&graph, a);
    let dfs_path = dfs(&graph, a);

    let bfs_names: Vec<String> = bfs_path
        .iter()
        .map(|&id| graph.get_node(id).unwrap().to_string())
        .collect();
    let dfs_names: Vec<String> = dfs_path
        .iter()
        .map(|&id| graph.get_node(id).unwrap().to_string())
        .collect();

    println!("  BFS: [{}]", bfs_names.join(" -> "));
    println!("  DFS: [{}]", dfs_names.join(" -> "));
    println!();

    println!("When to Use BFS:");
    println!("  ✅ Shortest path in unweighted graphs");
    println!("  ✅ Level-order traversal");
    println!("  ✅ Finding nodes at specific distances");
    println!("  ✅ Social network analysis (degrees of separation)");
    println!();

    println!("When to Use DFS:");
    println!("  ✅ Cycle detection");
    println!("  ✅ Path finding (any path)");
    println!("  ✅ Topological sorting");
    println!("  ✅ Finding strongly connected components");
    println!();

    println!("Performance Comparison:");
    println!("  • Both have O(V + E) time complexity");
    println!("  • BFS uses O(V) space for queue");
    println!("  • DFS uses O(V) space for stack (or recursion stack)");
    println!("  • Choice depends on the problem requirements");
    println!();
}

// Exercise: Implement your own BFS
fn exercise() {
    println!("Exercise: Implement Your Own BFS");
    println!("===============================");
    println!("Try implementing BFS for:");
    println!("  1. A tree structure");
    println!("  2. A graph with cycles");
    println!("  3. A disconnected graph");
    println!();
    println!("For each graph:");
    println!("  • Implement BFS traversal");
    println!("  • Find shortest paths");
    println!("  • Calculate distances");
    println!("  • Perform level-order traversal");
    println!();
    println!("Compare BFS with DFS and understand when to use each!");
    println!();
}
