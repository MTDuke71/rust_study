//! Step 5: DFS Implementation
//!
//! This step implements depth-first search (DFS) with both recursive and iterative
//! approaches, including path finding and cycle detection.
//!
//! ## Learning Objectives
//! - Implement recursive depth-first search
//! - Implement iterative DFS using explicit stack
//! - Learn path finding and cycle detection
//! - Understand component analysis and connectivity
//!
//! ## Alignment with Mission 7
//! This step directly implements **REQ-5: DFS Implementation** by teaching
//! comprehensive depth-first search techniques.
//!
//! ## Calendar Alignment
//! **Day 5 (Oct 12)**: DFS Implementation
//! - Implement depth-first search with recursion and stack
//! - Path finding and cycle detection
//! - Component analysis and connectivity

use mission7::{Graph, NodeId};
use std::collections::HashSet;

fn main() {
    println!("=== Step 5: DFS Implementation ===\n");

    // 1. Understanding DFS
    understanding_dfs();

    // 2. Recursive DFS Implementation
    recursive_dfs_implementation();

    // 3. Iterative DFS Implementation
    iterative_dfs_implementation();

    // 4. Path Finding with DFS
    path_finding_with_dfs();

    // 5. Cycle Detection with DFS
    cycle_detection_with_dfs();

    // 6. Component Analysis with DFS
    component_analysis_with_dfs();

    println!("\n=== Step 5 Complete ===");
    println!("Next: Step 6 - BFS Implementation");
}

fn understanding_dfs() {
    println!("1. Understanding DFS");
    println!("==================");

    println!("Depth-First Search (DFS) Characteristics:");
    println!("  • Explores as far as possible along each branch before backtracking");
    println!("  • Uses a stack (LIFO - Last In, First Out)");
    println!("  • Can be implemented recursively or iteratively");
    println!("  • Good for path finding and cycle detection");
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

    println!("DFS Traversal Order (going deep first):");
    println!("  1. Start at Root");
    println!("  2. Go to Left (deep)");
    println!("  3. Go to Left-Left (deeper)");
    println!("  4. Backtrack to Left");
    println!("  5. Go to Left-Right (deep)");
    println!("  6. Backtrack to Left");
    println!("  7. Backtrack to Root");
    println!("  8. Go to Right (deep)");
    println!("  9. Go to Right-Left (deeper)");
    println!("  10. Backtrack to Right");
    println!("  11. Go to Right-Right (deep)");
    println!();

    println!("DFS vs BFS Comparison:");
    println!(
        "  DFS: Root -> Left -> Left-Left -> Left-Right -> Right -> Right-Left -> Right-Right"
    );
    println!(
        "  BFS: Root -> Left -> Right -> Left-Left -> Left-Right -> Right-Left -> Right-Right"
    );
    println!();
}

fn recursive_dfs_implementation() {
    println!("2. Recursive DFS Implementation");
    println!("==============================");

    println!("Recursive DFS is natural because:");
    println!("  • Function call stack acts as the DFS stack");
    println!("  • Recursion handles backtracking automatically");
    println!("  • Code is clean and easy to understand");
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

    println!("Graph for DFS:");
    println!("    A");
    println!("   / \\");
    println!("  B   C");
    println!("  |   |");
    println!("  D   E");
    println!();

    // Recursive DFS implementation
    fn dfs_recursive<T: std::fmt::Display>(
        graph: &Graph<T>,
        node: NodeId,
        visited: &mut HashSet<NodeId>,
        path: &mut Vec<NodeId>,
    ) {
        if visited.insert(node) {
            path.push(node);
            let node_name = graph.get_node(node).unwrap();
            println!("  Visit: {}", node_name);

            // Recursively visit all unvisited neighbors
            for &neighbor in graph.neighbors(node) {
                if !visited.contains(&neighbor) {
                    dfs_recursive(graph, neighbor, visited, path);
                }
            }
        }
    }

    println!("Recursive DFS Traversal:");
    let mut visited = HashSet::new();
    let mut path = Vec::new();

    dfs_recursive(&graph, a, &mut visited, &mut path);

    println!();
    println!("DFS Result:");
    println!("  Path: {:?}", path);
    println!("  Visited: {:?}", visited);
    println!();

    // Show the recursive call stack
    println!("Recursive Call Stack Simulation:");
    println!("  dfs_recursive(A)");
    println!("    dfs_recursive(B)");
    println!("      dfs_recursive(D)");
    println!("      return from D");
    println!("    return from B");
    println!("    dfs_recursive(C)");
    println!("      dfs_recursive(E)");
    println!("      return from E");
    println!("    return from C");
    println!("  return from A");
    println!();
}

fn iterative_dfs_implementation() {
    println!("3. Iterative DFS Implementation");
    println!("==============================");

    println!("Iterative DFS uses an explicit stack:");
    println!("  • More control over the traversal process");
    println!("  • Avoids potential stack overflow for deep graphs");
    println!("  • Can be easier to debug and modify");
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

    // Iterative DFS implementation
    fn dfs_iterative<T: std::fmt::Display>(
        graph: &Graph<T>,
        start: NodeId,
    ) -> (Vec<NodeId>, HashSet<NodeId>) {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        let mut stack = Vec::new();

        stack.push(start);

        while let Some(node) = stack.pop() {
            if visited.insert(node) {
                path.push(node);
                let node_name = graph.get_node(node).unwrap();
                println!("  Visit: {}", node_name);

                // Push neighbors in reverse order to maintain left-to-right traversal
                for &neighbor in graph.neighbors(node).iter().rev() {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }

        (path, visited)
    }

    println!("Iterative DFS Traversal:");
    let (path, visited) = dfs_iterative(&graph, a);

    println!();
    println!("DFS Result:");
    println!("  Path: {:?}", path);
    println!("  Visited: {:?}", visited);
    println!();

    // Show the stack operations
    println!("Stack Operations Simulation:");
    println!("  Push A");
    println!("  Pop A, visit A, push C, push B");
    println!("  Pop B, visit B, push D");
    println!("  Pop D, visit D");
    println!("  Pop C, visit C, push E");
    println!("  Pop E, visit E");
    println!();

    // Compare recursive vs iterative
    println!("Recursive vs Iterative DFS:");
    println!("  Recursive: Uses function call stack, natural backtracking");
    println!("  Iterative: Uses explicit stack, more control");
    println!("  Both: Produce the same traversal order");
    println!();
}

fn path_finding_with_dfs() {
    println!("4. Path Finding with DFS");
    println!("=======================");

    println!("DFS can find paths between nodes:");
    println!("  • Not guaranteed to find shortest path");
    println!("  • Good for finding any path");
    println!("  • Can find all possible paths");
    println!();

    // Create a graph with multiple paths
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");

    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);
    graph.add_edge(c, d);
    graph.add_edge(d, e);

    println!("Graph with Multiple Paths:");
    println!("    A");
    println!("   / \\");
    println!("  B   C");
    println!("  |   |");
    println!("  D---D");
    println!("  |");
    println!("  E");
    println!();

    // DFS path finding implementation
    fn dfs_path_finding<T>(graph: &Graph<T>, start: NodeId, end: NodeId) -> Option<Vec<NodeId>> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        fn dfs_helper<T>(
            graph: &Graph<T>,
            current: NodeId,
            end: NodeId,
            visited: &mut HashSet<NodeId>,
            path: &mut Vec<NodeId>,
        ) -> bool {
            if current == end {
                path.push(current);
                return true;
            }

            if visited.insert(current) {
                path.push(current);

                for &neighbor in graph.neighbors(current) {
                    if !visited.contains(&neighbor)
                        && dfs_helper(graph, neighbor, end, visited, path)
                    {
                        return true;
                    }
                }

                path.pop(); // Backtrack
            }

            false
        }

        if dfs_helper(graph, start, end, &mut visited, &mut path) {
            Some(path)
        } else {
            None
        }
    }

    println!("DFS Path Finding:");
    let start = a;
    let end = e;

    if let Some(path) = dfs_path_finding(&graph, start, end) {
        println!("  Path from A to E found:");
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
        println!("  No path found from A to E");
    }
    println!();

    // Show that DFS doesn't guarantee shortest path
    println!("Note: DFS doesn't guarantee shortest path");
    println!("  • DFS might find: A -> B -> D -> E (3 edges)");
    println!("  • Shortest path: A -> C -> D -> E (3 edges)");
    println!("  • For shortest path, use BFS instead");
    println!();
}

fn cycle_detection_with_dfs() {
    println!("5. Cycle Detection with DFS");
    println!("==========================");

    println!("DFS is excellent for cycle detection:");
    println!("  • Use recursion stack to detect back edges");
    println!("  • Back edge = edge to an ancestor in the recursion stack");
    println!("  • Can detect cycles in directed graphs");
    println!();

    // Create graphs with and without cycles
    let mut graph_with_cycle = Graph::new_directed();
    let a = graph_with_cycle.add_node("A");
    let b = graph_with_cycle.add_node("B");
    let c = graph_with_cycle.add_node("C");
    let d = graph_with_cycle.add_node("D");

    graph_with_cycle.add_edge(a, b);
    graph_with_cycle.add_edge(b, c);
    graph_with_cycle.add_edge(c, d);
    graph_with_cycle.add_edge(d, a); // Creates a cycle

    let mut graph_without_cycle = Graph::new_directed();
    let a2 = graph_without_cycle.add_node("A");
    let b2 = graph_without_cycle.add_node("B");
    let c2 = graph_without_cycle.add_node("C");
    let d2 = graph_without_cycle.add_node("D");

    graph_without_cycle.add_edge(a2, b2);
    graph_without_cycle.add_edge(b2, c2);
    graph_without_cycle.add_edge(c2, d2);
    // No cycle

    println!("Graph with cycle: A -> B -> C -> D -> A");
    println!("Graph without cycle: A -> B -> C -> D");
    println!();

    // Cycle detection implementation
    fn has_cycle_dfs<T>(graph: &Graph<T>) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node in graph.nodes() {
            if !visited.contains(&node)
                && has_cycle_dfs_helper(graph, node, &mut visited, &mut rec_stack)
            {
                return true;
            }
        }

        false
    }

    fn has_cycle_dfs_helper<T>(
        graph: &Graph<T>,
        node: NodeId,
        visited: &mut HashSet<NodeId>,
        rec_stack: &mut HashSet<NodeId>,
    ) -> bool {
        visited.insert(node);
        rec_stack.insert(node);

        for &neighbor in graph.neighbors(node) {
            if !visited.contains(&neighbor) {
                if has_cycle_dfs_helper(graph, neighbor, visited, rec_stack) {
                    return true;
                }
            } else if rec_stack.contains(&neighbor) {
                // Back edge found - cycle detected
                return true;
            }
        }

        rec_stack.remove(&node);
        false
    }

    println!("Cycle Detection Results:");
    let has_cycle1 = has_cycle_dfs(&graph_with_cycle);
    let has_cycle2 = has_cycle_dfs(&graph_without_cycle);

    println!(
        "  Graph with cycle: {}",
        if has_cycle1 { "Has cycle" } else { "No cycle" }
    );
    println!(
        "  Graph without cycle: {}",
        if has_cycle2 { "Has cycle" } else { "No cycle" }
    );
    println!();

    // Show the recursion stack for cycle detection
    println!("Cycle Detection Process:");
    println!("  • visited: nodes that have been completely processed");
    println!("  • rec_stack: nodes currently in the recursion stack");
    println!("  • Back edge: edge to a node in the recursion stack");
    println!("  • Cycle found when we encounter a back edge");
    println!();
}

fn component_analysis_with_dfs() {
    println!("6. Component Analysis with DFS");
    println!("=============================");

    println!("DFS can find connected components:");
    println!("  • Each DFS call from an unvisited node finds one component");
    println!("  • Useful for analyzing graph connectivity");
    println!("  • Can find strongly connected components in directed graphs");
    println!();

    // Create a graph with multiple components
    let mut graph = Graph::new_undirected();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    let _f = graph.add_node("F");

    // Component 1: A-B-C
    graph.add_edge(a, b);
    graph.add_edge(b, c);

    // Component 2: D-E
    graph.add_edge(d, e);

    // Component 3: F (isolated)
    // No edges for F

    println!("Graph with Multiple Components:");
    println!("  Component 1: A-B-C");
    println!("  Component 2: D-E");
    println!("  Component 3: F (isolated)");
    println!();

    // Component analysis implementation
    fn find_components<T>(graph: &Graph<T>) -> Vec<Vec<NodeId>> {
        let mut components = Vec::new();
        let mut visited = HashSet::new();

        for node in graph.nodes() {
            if !visited.contains(&node) {
                let mut component = Vec::new();
                dfs_component(graph, node, &mut visited, &mut component);
                components.push(component);
            }
        }

        components
    }

    fn dfs_component<T>(
        graph: &Graph<T>,
        node: NodeId,
        visited: &mut HashSet<NodeId>,
        component: &mut Vec<NodeId>,
    ) {
        if visited.insert(node) {
            component.push(node);

            for &neighbor in graph.neighbors(node) {
                if !visited.contains(&neighbor) {
                    dfs_component(graph, neighbor, visited, component);
                }
            }
        }
    }

    println!("Component Analysis Results:");
    let components = find_components(&graph);

    for (i, component) in components.iter().enumerate() {
        let component_names: Vec<String> = component
            .iter()
            .map(|&id| graph.get_node(id).unwrap().to_string())
            .collect();
        println!("  Component {}: [{}]", i + 1, component_names.join(", "));
    }

    println!();
    println!("Component Statistics:");
    println!("  Total components: {}", components.len());
    println!(
        "  Largest component size: {}",
        components.iter().map(|c| c.len()).max().unwrap()
    );
    println!(
        "  Smallest component size: {}",
        components.iter().map(|c| c.len()).min().unwrap()
    );
    println!();

    // Show DFS traversal for each component
    println!("DFS Traversal for Each Component:");
    for (i, component) in components.iter().enumerate() {
        if let Some(&start) = component.first() {
            let mut visited = HashSet::new();
            let mut path = Vec::new();

            fn dfs_traversal<T>(
                graph: &Graph<T>,
                node: NodeId,
                visited: &mut HashSet<NodeId>,
                path: &mut Vec<NodeId>,
            ) {
                if visited.insert(node) {
                    path.push(node);
                    for &neighbor in graph.neighbors(node) {
                        if !visited.contains(&neighbor) {
                            dfs_traversal(graph, neighbor, visited, path);
                        }
                    }
                }
            }

            dfs_traversal(&graph, start, &mut visited, &mut path);

            let path_names: Vec<String> = path
                .iter()
                .map(|&id| graph.get_node(id).unwrap().to_string())
                .collect();
            println!("  Component {} DFS: [{}]", i + 1, path_names.join(" -> "));
        }
    }
    println!();
}
// Exercise: Implement your own DFS
#[allow(dead_code)]
fn exercise() {
    println!("Exercise: Implement Your Own DFS");
    println!("===============================");
    println!("Try implementing DFS for:");
    println!("  1. A tree structure");
    println!("  2. A graph with cycles");
    println!("  3. A disconnected graph");
    println!();
    println!("For each graph:");
    println!("  • Implement both recursive and iterative DFS");
    println!("  • Find paths between nodes");
    println!("  • Detect cycles");
    println!("  • Find connected components");
    println!();
    println!("Compare the results and understand when to use each approach!");
    println!();
}
