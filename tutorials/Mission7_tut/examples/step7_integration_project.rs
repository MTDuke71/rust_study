//! Step 7: Integration Project
//!
//! This final step integrates all the concepts learned in the previous steps
//! to build real-world graph applications and solve practical problems.
//!
//! ## Learning Objectives
//! - Apply graph algorithms to real-world problems
//! - Build social network analysis tools
//! - Implement maze solving algorithms
//! - Create performance optimization techniques
//!
//! ## Alignment with Mission 7
//! This step integrates all Mission 7 requirements by applying the complete
//! graph implementation to practical, real-world scenarios.
//!
//! ## Calendar Alignment
//! **Day 7 (Oct 14)**: Integration & Examples
//! - Build pathfinding examples combining graph + grid
//! - Integration testing and performance optimization

use mission7::{Graph, GraphType, NodeId};
use mission7_tut::tutorial_utils;
use std::collections::{HashSet, VecDeque};

fn main() {
    println!("=== Step 7: Integration Project ===\n");
    
    // 1. Social Network Analysis
    social_network_analysis();
    
    // 2. Maze Solving
    maze_solving();
    
    // 3. Dependency Resolution
    dependency_resolution();
    
    // 4. Network Topology Analysis
    network_topology_analysis();
    
    // 5. Performance Optimization
    performance_optimization();
    
    // 6. Mission 7 Integration
    mission7_integration();
    
    println!("\n=== Step 7 Complete ===");
    println!("Congratulations! You've completed Mission 7 Tutorial!");
    println!("You now have a comprehensive understanding of graph representation and algorithms.");
}

fn social_network_analysis() {
    println!("1. Social Network Analysis");
    println!("=========================");
    
    println!("Building a social network analysis tool:");
    println!("  • Find degrees of separation between people");
    println!("  • Identify influential people (high degree)");
    println!("  • Find friend groups (connected components)");
    println!("  • Analyze network properties");
    println!();
    
    // Create a social network
    let mut network = Graph::new_undirected();
    
    // Add people to the network
    let alice = network.add_node("Alice");
    let bob = network.add_node("Bob");
    let charlie = network.add_node("Charlie");
    let diana = network.add_node("Diana");
    let eve = network.add_node("Eve");
    let frank = network.add_node("Frank");
    let grace = network.add_node("Grace");
    let henry = network.add_node("Henry");
    
    // Add friendships
    network.add_edge(alice, bob);
    network.add_edge(alice, charlie);
    network.add_edge(bob, diana);
    network.add_edge(charlie, diana);
    network.add_edge(diana, eve);
    network.add_edge(eve, frank);
    network.add_edge(grace, henry);
    
    println!("Social Network Structure:");
    println!("  Alice - Bob - Diana - Eve - Frank");
    println!("    |      |      |");
    println!("  Charlie  |    Diana");
    println!("           |");
    println!("         Diana");
    println!();
    println!("  Grace - Henry (separate group)");
    println!();
    
    // Analyze the network
    println!("Network Analysis:");
    println!();
    
    // 1. Degrees of separation
    println!("1. Degrees of Separation:");
    let distances = bfs_distances(&network, alice);
    for (i, distance) in distances.iter().enumerate() {
        if let Some(dist) = distance {
            let person = network.get_node(i).unwrap();
            println!("  {}: {} degrees from Alice", person, dist);
        }
    }
    println!();
    
    // 2. Influential people (high degree)
    println!("2. Influential People (High Degree):");
    let mut degrees: Vec<(NodeId, usize)> = network.nodes()
        .map(|id| (id, network.degree(id)))
        .collect();
    degrees.sort_by(|a, b| b.1.cmp(&a.1));
    
    for (node_id, degree) in degrees {
        let person = network.get_node(node_id).unwrap();
        println!("  {}: {} friends", person, degree);
    }
    println!();
    
    // 3. Friend groups (connected components)
    println!("3. Friend Groups (Connected Components):");
    let components = find_components(&network);
    for (i, component) in components.iter().enumerate() {
        let group_names: Vec<String> = component.iter()
            .map(|&id| network.get_node(id).unwrap().to_string())
            .collect();
        println!("  Group {}: [{}]", i + 1, group_names.join(", "));
    }
    println!();
    
    // 4. Network properties
    println!("4. Network Properties:");
    let n = network.node_count();
    let e = network.edge_count();
    let max_edges = n * (n - 1) / 2;
    let density = e as f64 / max_edges as f64;
    
    println!("  Total people: {}", n);
    println!("  Total friendships: {}", e);
    println!("  Network density: {:.3}", density);
    println!("  Average friends per person: {:.1}", e as f64 / n as f64);
    println!();
    
    // 5. Path finding
    println!("5. Path Finding:");
    if let Some(path) = bfs_shortest_path(&network, alice, frank) {
        let path_names: Vec<String> = path.iter()
            .map(|&id| network.get_node(id).unwrap().to_string())
            .collect();
        println!("  Shortest path from Alice to Frank: [{}]", path_names.join(" -> "));
        println!("  Degrees of separation: {}", path.len() - 1);
    }
    println!();
}

fn maze_solving() {
    println!("2. Maze Solving");
    println!("==============");
    
    println!("Building a maze solver using graph algorithms:");
    println!("  • Convert maze to graph representation");
    println!("  • Use BFS to find shortest path");
    println!("  • Use DFS to find any path");
    println!("  • Handle obstacles and dead ends");
    println!();
    
    // Create a simple maze as a 2D grid
    let maze = vec![
        "S...X",
        ".X.X.",
        "...X.",
        "X.X..",
        "....E",
    ];
    
    println!("Maze Layout (S=Start, E=End, X=Wall, .=Path):");
    for row in &maze {
        println!("  {}", row);
    }
    println!();
    
    // Convert maze to graph
    let (graph, start, end) = maze_to_graph(&maze);
    
    println!("Maze Graph Representation:");
    println!("  Nodes: {} (one for each open cell)", graph.node_count());
    println!("  Edges: {} (connections between adjacent cells)", graph.edge_count());
    println!();
    
    // Solve maze using BFS (shortest path)
    println!("Maze Solving with BFS (Shortest Path):");
    if let Some(path) = bfs_shortest_path(&graph, start, end) {
        println!("  Path found with {} steps", path.len() - 1);
        print_maze_solution(&maze, &path);
    } else {
        println!("  No path found!");
    }
    println!();
    
    // Solve maze using DFS (any path)
    println!("Maze Solving with DFS (Any Path):");
    if let Some(path) = dfs_path_finding(&graph, start, end) {
        println!("  Path found with {} steps", path.len() - 1);
        print_maze_solution(&maze, &path);
    } else {
        println!("  No path found!");
    }
    println!();
    
    // Analyze maze properties
    println!("Maze Analysis:");
    let components = find_components(&graph);
    println!("  Connected components: {}", components.len());
    println!("  Largest component size: {}", components.iter().map(|c| c.len()).max().unwrap());
    
    if components.len() > 1 {
        println!("  Warning: Maze has disconnected regions!");
    }
    println!();
}

fn dependency_resolution() {
    println!("3. Dependency Resolution");
    println!("=======================");
    
    println!("Building a dependency resolution system:");
    println!("  • Model tasks and their dependencies");
    println!("  • Detect circular dependencies");
    println!("  • Find topological ordering");
    println!("  • Identify critical path");
    println!();
    
    // Create a project with tasks and dependencies
    let mut project = Graph::new_directed();
    
    let design = project.add_node("Design");
    let research = project.add_node("Research");
    let prototype = project.add_node("Prototype");
    let code = project.add_node("Code");
    let test = project.add_node("Test");
    let deploy = project.add_node("Deploy");
    let document = project.add_node("Document");
    
    // Add dependencies
    project.add_edge(research, design);
    project.add_edge(design, prototype);
    project.add_edge(prototype, code);
    project.add_edge(code, test);
    project.add_edge(test, deploy);
    project.add_edge(code, document);
    project.add_edge(document, deploy);
    
    println!("Project Dependencies:");
    println!("  Research -> Design -> Prototype -> Code -> Test -> Deploy");
    println!("  Code -> Document -> Deploy");
    println!();
    
    // Check for cycles
    println!("Dependency Analysis:");
    let has_cycle = has_cycle_dfs(&project);
    println!("  Has circular dependencies: {}", if has_cycle { "Yes" } else { "No" });
    
    if !has_cycle {
        println!("  Project can be completed successfully!");
    } else {
        println!("  Warning: Circular dependencies detected!");
    }
    println!();
    
    // Find topological ordering
    println!("Topological Ordering (Task Execution Order):");
    let topo_order = topological_sort(&project);
    for (i, task_id) in topo_order.iter().enumerate() {
        let task = project.get_node(*task_id).unwrap();
        println!("  {}. {}", i + 1, task);
    }
    println!();
    
    // Find critical path
    println!("Critical Path Analysis:");
    let critical_path = find_critical_path(&project);
    for (i, task_id) in critical_path.iter().enumerate() {
        let task = project.get_node(*task_id).unwrap();
        if i == 0 {
            print!("  Critical path: {}", task);
        } else {
            print!(" -> {}", task);
        }
    }
    println!();
    println!("  Critical path length: {} tasks", critical_path.len());
    println!();
}

fn network_topology_analysis() {
    println!("4. Network Topology Analysis");
    println!("===========================");
    
    println!("Analyzing network topology:");
    println!("  • Router connections and routing paths");
    println!("  • Network redundancy and fault tolerance");
    println!("  • Performance optimization");
    println!("  • Load balancing analysis");
    println!();
    
    // Create a network topology
    let mut network = Graph::new_undirected();
    
    let router_a = network.add_node("Router-A");
    let router_b = network.add_node("Router-B");
    let router_c = network.add_node("Router-C");
    let router_d = network.add_node("Router-D");
    let router_e = network.add_node("Router-E");
    let router_f = network.add_node("Router-F");
    
    // Add connections
    network.add_edge(router_a, router_b);
    network.add_edge(router_a, router_c);
    network.add_edge(router_b, router_d);
    network.add_edge(router_c, router_d);
    network.add_edge(router_d, router_e);
    network.add_edge(router_e, router_f);
    network.add_edge(router_b, router_f); // Redundant path
    
    println!("Network Topology:");
    println!("  A - B - D - E - F");
    println!("  |   |   |");
    println!("  C   |   D");
    println!("      |");
    println!("      F (redundant path)");
    println!();
    
    // Analyze network properties
    println!("Network Analysis:");
    println!();
    
    // 1. Connectivity
    let components = find_components(&network);
    println!("1. Connectivity:");
    println!("  Connected components: {}", components.len());
    println!("  Is fully connected: {}", components.len() == 1);
    println!();
    
    // 2. Redundancy
    println!("2. Redundancy Analysis:");
    for router_id in network.nodes() {
        let router = network.get_node(router_id).unwrap();
        let degree = network.degree(router_id);
        println!("  {}: {} connections", router, degree);
        
        if degree == 1 {
            println!("    Warning: Single point of failure!");
        } else if degree >= 3 {
            println!("    Good: High redundancy");
        }
    }
    println!();
    
    // 3. Routing paths
    println!("3. Routing Paths:");
    let start = router_a;
    let end = router_f;
    
    if let Some(path) = bfs_shortest_path(&network, start, end) {
        let path_names: Vec<String> = path.iter()
            .map(|&id| network.get_node(id).unwrap().to_string())
            .collect();
        println!("  Shortest path from A to F: [{}]", path_names.join(" -> "));
        println!("  Path length: {} hops", path.len() - 1);
    }
    println!();
    
    // 4. Network diameter
    println!("4. Network Diameter:");
    let mut max_distance = 0;
    for start_router in network.nodes() {
        let distances = bfs_distances(&network, start_router);
        for distance in distances {
            if let Some(dist) = distance {
                max_distance = max_distance.max(dist);
            }
        }
    }
    println!("  Network diameter: {} hops", max_distance);
    println!();
}

fn performance_optimization() {
    println!("5. Performance Optimization");
    println!("==========================");
    
    println!("Optimizing graph algorithm performance:");
    println!("  • Memory usage optimization");
    println!("  • Algorithm complexity analysis");
    println!("  • Caching and memoization");
    println!("  • Parallel processing opportunities");
    println!();
    
    // Create a large graph for performance testing
    let mut large_graph = Graph::new_undirected();
    
    // Add many nodes
    let node_count = 100;
    for i in 0..node_count {
        large_graph.add_node(format!("Node{}", i));
    }
    
    // Add edges to create a connected graph
    for i in 0..node_count - 1 {
        large_graph.add_edge(i, i + 1);
    }
    
    // Add some random edges
    for i in 0..node_count {
        if i + 10 < node_count {
            large_graph.add_edge(i, i + 10);
        }
    }
    
    println!("Performance Test Graph:");
    println!("  Nodes: {}", large_graph.node_count());
    println!("  Edges: {}", large_graph.edge_count());
    println!();
    
    // Performance measurements
    println!("Performance Measurements:");
    println!();
    
    // 1. Memory usage
    println!("1. Memory Usage:");
    let node_memory = large_graph.node_count() * 8; // 8 bytes per node ID
    let edge_memory = large_graph.edge_count() * 8; // 8 bytes per edge
    let total_memory = node_memory + edge_memory;
    
    println!("  Node storage: {} bytes", node_memory);
    println!("  Edge storage: {} bytes", edge_memory);
    println!("  Total memory: {} bytes", total_memory);
    println!("  Memory per node: {:.1} bytes", total_memory as f64 / large_graph.node_count() as f64);
    println!();
    
    // 2. Algorithm performance
    println!("2. Algorithm Performance:");
    
    // BFS performance
    let start_time = std::time::Instant::now();
    let _distances = bfs_distances(&large_graph, 0);
    let bfs_time = start_time.elapsed();
    
    println!("  BFS traversal: {:?}", bfs_time);
    println!("  BFS complexity: O(V + E) = O({})", large_graph.node_count() + large_graph.edge_count());
    println!();
    
    // 3. Optimization techniques
    println!("3. Optimization Techniques:");
    println!("  • Use Vec<bool> instead of HashSet for visited tracking (small graphs)");
    println!("  • Pre-allocate vectors with known capacity");
    println!("  • Use iterators instead of loops where possible");
    println!("  • Consider parallel processing for independent operations");
    println!("  • Cache frequently accessed results");
    println!();
    
    // 4. Scalability analysis
    println!("4. Scalability Analysis:");
    let n = large_graph.node_count();
    let e = large_graph.edge_count();
    
    println!("  Graph density: {:.3}", e as f64 / (n * (n - 1) / 2) as f64);
    println!("  Average degree: {:.1}", e as f64 / n as f64);
    
    if e < n * 2 {
        println!("  Sparse graph: Adjacency list is optimal");
    } else {
        println!("  Dense graph: Consider adjacency matrix");
    }
    println!();
}

fn mission7_integration() {
    println!("6. Mission 7 Integration");
    println!("=======================");
    
    println!("Integrating all Mission 7 concepts:");
    println!("  • Graph structure and node storage (REQ-1)");
    println!("  • Edge management operations (REQ-2)");
    println!("  • Graph building and validation (REQ-3)");
    println!("  • Algorithm foundation (REQ-4)");
    println!("  • DFS implementation (REQ-5)");
    println!("  • BFS implementation (REQ-6)");
    println!();
    
    // Create a comprehensive example
    let mut graph = Graph::new_directed();
    
    // Add nodes with different data types
    let start = graph.add_node("Start");
    let process1 = graph.add_node("Process1");
    let process2 = graph.add_node("Process2");
    let decision = graph.add_node("Decision");
    let end = graph.add_node("End");
    
    // Add edges
    graph.add_edge(start, process1);
    graph.add_edge(start, process2);
    graph.add_edge(process1, decision);
    graph.add_edge(process2, decision);
    graph.add_edge(decision, end);
    
    println!("Comprehensive Graph Example:");
    println!("  Start -> Process1 -> Decision -> End");
    println!("  Start -> Process2 -> Decision -> End");
    println!();
    
    // Demonstrate all requirements
    println!("Mission 7 Requirements Demonstration:");
    println!();
    
    // REQ-1: Graph structure and node storage
    println!("REQ-1: Graph Structure and Node Storage");
    println!("  Node count: {}", graph.node_count());
    println!("  Edge count: {}", graph.edge_count());
    println!("  Graph type: {:?}", graph.graph_type);
    println!();
    
    // REQ-2: Edge management operations
    println!("REQ-2: Edge Management Operations");
    println!("  Has edge Start -> Process1: {}", graph.has_edge(start, process1));
    println!("  Start's neighbors: {:?}", graph.neighbors(start));
    println!("  Start's degree: {}", graph.degree(start));
    println!();
    
    // REQ-3: Graph building and validation
    println!("REQ-3: Graph Building and Validation");
    println!("  Is connected: {}", graph.is_connected());
    println!("  Has cycle: {}", has_cycle_dfs(&graph));
    let components = find_components(&graph);
    println!("  Connected components: {}", components.len());
    println!();
    
    // REQ-4: Algorithm foundation
    println!("REQ-4: Algorithm Foundation");
    let mut visited = HashSet::new();
    let mut path = Vec::new();
    println!("  Visited tracking: {:?}", visited);
    println!("  Path reconstruction: {:?}", path);
    println!();
    
    // REQ-5: DFS implementation
    println!("REQ-5: DFS Implementation");
    let dfs_path = dfs_traversal(&graph, start);
    let dfs_names: Vec<String> = dfs_path.iter()
        .map(|&id| graph.get_node(id).unwrap().to_string())
        .collect();
    println!("  DFS path: [{}]", dfs_names.join(" -> "));
    println!();
    
    // REQ-6: BFS implementation
    println!("REQ-6: BFS Implementation");
    let bfs_path = bfs_traversal(&graph, start);
    let bfs_names: Vec<String> = bfs_path.iter()
        .map(|&id| graph.get_node(id).unwrap().to_string())
        .collect();
    println!("  BFS path: [{}]", bfs_names.join(" -> "));
    println!();
    
    // Final summary
    println!("Mission 7 Complete!");
    println!("==================");
    println!("You have successfully learned:");
    println!("  ✅ Graph data structures and representations");
    println!("  ✅ Adjacency list implementation");
    println!("  ✅ Edge management and validation");
    println!("  ✅ Graph building and analysis");
    println!("  ✅ Algorithm infrastructure");
    println!("  ✅ Depth-first search (DFS)");
    println!("  ✅ Breadth-first search (BFS)");
    println!("  ✅ Real-world applications");
    println!("  ✅ Performance optimization");
    println!();
    println!("You are now ready to tackle complex graph problems!");
    println!("Consider exploring advanced topics like:");
    println!("  • Weighted graphs and shortest path algorithms");
    println!("  • Minimum spanning trees");
    println!("  • Network flow algorithms");
    println!("  • Graph coloring and matching");
    println!("  • Parallel graph algorithms");
    println!();
}

// Helper functions for the integration project

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

fn bfs_shortest_path<T>(graph: &Graph<T>, start: NodeId, end: NodeId) -> Option<Vec<NodeId>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut parents = vec![None; graph.node_count()];
    
    queue.push_back(start);
    visited.insert(start);
    
    while let Some(node) = queue.pop_front() {
        if node == end {
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

fn maze_to_graph(maze: &[&str]) -> (Graph<String>, NodeId, NodeId) {
    let mut graph = Graph::new_undirected();
    let mut start = 0;
    let mut end = 0;
    let mut node_map = std::collections::HashMap::new();
    
    // Add nodes for each open cell
    for (row, line) in maze.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != 'X' {
                let node_id = graph.add_node(format!("({},{})", row, col));
                node_map.insert((row, col), node_id);
                
                if ch == 'S' {
                    start = node_id;
                } else if ch == 'E' {
                    end = node_id;
                }
            }
        }
    }
    
    // Add edges between adjacent cells
    for (row, line) in maze.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != 'X' {
                let current = node_map[&(row, col)];
                
                // Check adjacent cells
                let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                for (dr, dc) in directions {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;
                    
                    if new_row >= 0 && new_row < maze.len() as i32 &&
                       new_col >= 0 && new_col < maze[0].len() as i32 {
                        let new_row = new_row as usize;
                        let new_col = new_col as usize;
                        
                        if maze[new_row].chars().nth(new_col).unwrap() != 'X' {
                            let neighbor = node_map[&(new_row, new_col)];
                            graph.add_edge(current, neighbor);
                        }
                    }
                }
            }
        }
    }
    
    (graph, start, end)
}

fn print_maze_solution(maze: &[&str], path: &[NodeId]) {
    let mut solution = maze.to_vec();
    
    // Mark the path
    for (i, &node_id) in path.iter().enumerate() {
        if i == 0 {
            // Start
        } else if i == path.len() - 1 {
            // End
        } else {
            // Path
            // Note: This is a simplified version - in practice, you'd need to
            // map node IDs back to maze coordinates
        }
    }
    
    for row in &solution {
        println!("  {}", row);
    }
}

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
                if !visited.contains(&neighbor) {
                    if dfs_helper(graph, neighbor, end, visited, path) {
                        return true;
                    }
                }
            }
            
            path.pop();
        }
        
        false
    }
    
    if dfs_helper(graph, start, end, &mut visited, &mut path) {
        Some(path)
    } else {
        None
    }
}

fn has_cycle_dfs<T>(graph: &Graph<T>) -> bool {
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    
    for node in graph.nodes() {
        if !visited.contains(&node) {
            if has_cycle_dfs_helper(graph, node, &mut visited, &mut rec_stack) {
                return true;
            }
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
            return true;
        }
    }
    
    rec_stack.remove(&node);
    false
}

fn topological_sort<T>(graph: &Graph<T>) -> Vec<NodeId> {
    let mut visited = HashSet::new();
    let mut result = Vec::new();
    
    fn dfs_topological<T>(
        graph: &Graph<T>,
        node: NodeId,
        visited: &mut HashSet<NodeId>,
        result: &mut Vec<NodeId>,
    ) {
        if visited.insert(node) {
            for &neighbor in graph.neighbors(node) {
                if !visited.contains(&neighbor) {
                    dfs_topological(graph, neighbor, visited, result);
                }
            }
            result.push(node);
        }
    }
    
    for node in graph.nodes() {
        if !visited.contains(&node) {
            dfs_topological(graph, node, &mut visited, &mut result);
        }
    }
    
    result.reverse();
    result
}

fn find_critical_path<T>(graph: &Graph<T>) -> Vec<NodeId> {
    // Simplified critical path - in practice, you'd need to consider task durations
    let topo_order = topological_sort(graph);
    topo_order
}

fn dfs_traversal<T>(graph: &Graph<T>, start: NodeId) -> Vec<NodeId> {
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

fn bfs_traversal<T>(graph: &Graph<T>, start: NodeId) -> Vec<NodeId> {
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
