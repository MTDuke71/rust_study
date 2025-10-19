/// Tests for Mission 8 real-world applications (REQ-4)
/// 
/// These tests verify that the BFS/DFS algorithms work correctly in
/// realistic application scenarios like maze solving, network analysis,
/// and dependency resolution.
use mission8::*;
use std::collections::HashMap;

#[test]
fn req4_maze_like_grid_pathfinding() {
    // Test BFS pathfinding in a 2D grid (maze-like scenario)
    
    // Create a simple grid graph where each cell connects to adjacent cells
    let mut grid_graph: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    
    // Define a 3x3 grid with walls (missing connections)
    let walkable_cells = vec![
        (0, 0), (0, 1), (0, 2),  // Top row
        (1, 0),         (1, 2),  // Middle row (blocked center)
        (2, 0), (2, 1), (2, 2),  // Bottom row
    ];
    
    // Add connections between adjacent walkable cells
    for &(row, col) in &walkable_cells {
        let mut neighbors = Vec::new();
        
        // Check 4-directional adjacency
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_pos = (row + dr, col + dc);
            if walkable_cells.contains(&new_pos) {
                neighbors.push(new_pos);
            }
        }
        
        grid_graph.insert((row, col), neighbors);
    }
    
    // Test shortest path from top-left to bottom-right
    let start = (0, 0);
    let end = (2, 2);
    
    let path_result = shortest_path(&grid_graph, start, end);
    assert!(path_result.is_ok(), "Should find path in connected grid");
    
    let path = path_result.unwrap();
    assert_eq!(path[0], start, "Path should start at start position");
    assert_eq!(path[path.len() - 1], end, "Path should end at end position");
    assert!(path.len() >= 2, "Path should have at least start and end");
}

#[test]
fn req4_maze_blocked_path_detection() {
    // Test that BFS correctly identifies when no path exists
    
    let mut blocked_graph: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    
    // Create two isolated islands
    blocked_graph.insert((0, 0), vec![(0, 1)]);
    blocked_graph.insert((0, 1), vec![(0, 0)]);
    blocked_graph.insert((2, 2), vec![(2, 3)]);
    blocked_graph.insert((2, 3), vec![(2, 2)]);
    
    // Try to find path between islands
    let path_result = shortest_path(&blocked_graph, (0, 0), (2, 2));
    assert!(path_result.is_err(), "Should not find path between isolated islands");
}

#[test]
fn req4_network_connectivity_analysis() {
    // Test network connectivity analysis using connected_components
    
    // Create a network with multiple partitions
    let mut network: HashMap<u32, Vec<u32>> = HashMap::new();
    
    // Partition 1: Router + 2 computers
    network.insert(1, vec![2, 3]);  // Router
    network.insert(2, vec![1]);     // Computer 1
    network.insert(3, vec![1]);     // Computer 2
    
    // Partition 2: Server + computer (isolated)
    network.insert(4, vec![5]);     // Server
    network.insert(5, vec![4]);     // Computer 3
    
    // Isolated node
    network.insert(6, vec![]);      // Isolated computer
    
    let components = connected_components(&network);
    assert_eq!(components.len(), 3, "Should detect 3 network partitions");
    
    // Check component sizes
    let mut sizes: Vec<usize> = components.iter().map(|c| c.len()).collect();
    sizes.sort();
    assert_eq!(sizes, vec![1, 2, 3], "Components should be sizes 1, 2, and 3");
}

#[test]
fn req4_network_shortest_path_analysis() {
    // Test shortest path calculation in network topology
    
    let mut network: HashMap<u32, Vec<u32>> = HashMap::new();
    
    // Create linear network: PC1 - Router - Switch - PC2
    network.insert(1, vec![2]);     // PC1 connects to Router
    network.insert(2, vec![1, 3]);  // Router connects to PC1 and Switch
    network.insert(3, vec![2, 4]);  // Switch connects to Router and PC2
    network.insert(4, vec![3]);     // PC2 connects to Switch
    
    let path_result = shortest_path(&network, 1, 4);
    assert!(path_result.is_ok(), "Should find path in connected network");
    
    let path = path_result.unwrap();
    assert_eq!(path.len(), 4, "Path should be PC1 → Router → Switch → PC2");
    assert_eq!(path, vec![1, 2, 3, 4], "Should find optimal 4-hop path");
}

#[test]
fn req4_dependency_cycle_detection() {
    // Test circular dependency detection using DFS-based cycle detection
    
    let mut deps: HashMap<u32, Vec<u32>> = HashMap::new();
    
    // Create circular dependency: A → B → C → A
    deps.insert(1, vec![2]);  // Package A depends on B
    deps.insert(2, vec![3]);  // Package B depends on C
    deps.insert(3, vec![1]);  // Package C depends on A (creates cycle!)
    
    // Test cycle detection using has_cycle
    let has_cycle_result = has_cycle(&deps);
    assert!(has_cycle_result, "Should detect circular dependency");
}

#[test]
fn req4_dependency_acyclic_validation() {
    // Test that acyclic dependency graphs are accepted
    
    let mut deps: HashMap<u32, Vec<u32>> = HashMap::new();
    
    // Create acyclic dependency chain: App → HTTP → JSON → Utils
    deps.insert(1, vec![2]);     // App depends on HTTP
    deps.insert(2, vec![3]);     // HTTP depends on JSON
    deps.insert(3, vec![4]);     // JSON depends on Utils
    deps.insert(4, vec![]);      // Utils has no dependencies
    
    let has_cycle_result = has_cycle(&deps);
    assert!(!has_cycle_result, "Should not detect cycles in acyclic graph");
}

#[test]
fn req4_dependency_transitive_analysis() {
    // Test transitive dependency analysis
    
    let mut deps: HashMap<u32, Vec<u32>> = HashMap::new();
    
    // Create dependency tree
    deps.insert(1, vec![2, 3]);  // App depends on HTTP and Auth
    deps.insert(2, vec![4]);     // HTTP depends on Utils
    deps.insert(3, vec![4]);     // Auth depends on Utils  
    deps.insert(4, vec![]);      // Utils has no dependencies
    
    // Test that we can reach all transitive dependencies
    let visited_from_app = dfs(&deps, 1);
    
    assert!(visited_from_app.contains(&1), "Should visit App itself");
    assert!(visited_from_app.contains(&2), "Should visit HTTP dependency");
    assert!(visited_from_app.contains(&3), "Should visit Auth dependency");
    assert!(visited_from_app.contains(&4), "Should visit transitive Utils dependency");
    assert_eq!(visited_from_app.len(), 4, "Should visit all 4 packages transitively");
}

#[test]
fn req4_algorithms_integrate_with_applications() {
    // Integration test ensuring core algorithms work for application scenarios
    
    // 1. BFS for shortest path (maze solving) - use bidirectional graph for pathfinding
    let mut path_graph: HashMap<u32, Vec<u32>> = HashMap::new();
    path_graph.insert(0, vec![1]);
    path_graph.insert(1, vec![0, 2]);
    path_graph.insert(2, vec![1]);
    
    let bfs_path = shortest_path(&path_graph, 0, 2);
    assert!(bfs_path.is_ok(), "BFS should find shortest path");
    assert_eq!(bfs_path.unwrap(), vec![0, 1, 2], "BFS should find optimal path");
    
    // 2. Connected components (network analysis)
    let components = connected_components(&path_graph);
    assert_eq!(components.len(), 1, "Connected graph should have 1 component");
    
    // 3. Cycle detection (dependency analysis) - use directed acyclic graph
    let mut acyclic_graph: HashMap<u32, Vec<u32>> = HashMap::new();
    acyclic_graph.insert(0, vec![1]);     // 0 → 1
    acyclic_graph.insert(1, vec![2]);     // 1 → 2  
    acyclic_graph.insert(2, vec![]);      // 2 has no outgoing edges
    
    let has_cycle_result = has_cycle(&acyclic_graph);
    assert!(!has_cycle_result, "Acyclic graph should not have cycles");
    
    // 4. DFS traversal (dependency tree walking)
    let visited = dfs(&path_graph, 0);
    assert_eq!(visited.len(), 3, "DFS should visit all connected nodes");
}

#[test]
fn req4_edge_cases_handled_correctly() {
    // Test edge cases that applications might encounter
    
    // 1. Empty graph
    let empty_graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let empty_components = connected_components(&empty_graph);
    assert_eq!(empty_components.len(), 0, "Empty graph should have no components");
    
    // 2. Single node
    let mut single_node: HashMap<u32, Vec<u32>> = HashMap::new();
    single_node.insert(42, vec![]);
    
    let single_components = connected_components(&single_node);
    assert_eq!(single_components.len(), 1, "Single node should form one component");
    assert_eq!(single_components[0], vec![42], "Component should contain the single node");
    
    // 3. Self-loop
    let mut self_loop: HashMap<u32, Vec<u32>> = HashMap::new();
    self_loop.insert(1, vec![1]);  // Node points to itself
    
    let self_loop_cycle = has_cycle(&self_loop);
    assert!(self_loop_cycle, "Self-loop should be detected as a cycle");
    
    // 4. Disconnected path query
    let mut disconnected: HashMap<u32, Vec<u32>> = HashMap::new();
    disconnected.insert(1, vec![]);
    disconnected.insert(2, vec![]);
    
    let no_path = shortest_path(&disconnected, 1, 2);
    assert!(no_path.is_err(), "Should fail to find path between disconnected nodes");
}

#[test]
fn req4_performance_characteristics() {
    // Test that algorithms perform well on realistic application-sized graphs
    
    // Create a moderately-sized graph (tree topology - acyclic but connected)
    let mut tree_graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let root = 0;
    let num_nodes = 100;
    
    // Create a tree structure: root → children (no back edges)
    let mut root_children = Vec::new();
    for i in 1..=num_nodes {
        root_children.push(i);
        tree_graph.insert(i, vec![]);  // Leaves have no outgoing edges
    }
    tree_graph.insert(root, root_children);
    
    // Test that algorithms complete quickly on this graph
    let start_time = std::time::Instant::now();
    
    // BFS shortest path (from root to any leaf)
    let path = shortest_path(&tree_graph, 0, num_nodes);
    assert!(path.is_ok(), "Should find path in tree topology");
    assert_eq!(path.unwrap(), vec![0, num_nodes], "Should find direct path from root to leaf");
    
    // Connected components  
    let components = connected_components(&tree_graph);
    assert_eq!(components.len(), 1, "Tree should be fully connected");
    assert_eq!(components[0].len(), (num_nodes + 1) as usize, "Component should contain all nodes");
    
    // Cycle detection
    let has_cycle_result = has_cycle(&tree_graph);
    assert!(!has_cycle_result, "Tree topology should be acyclic");
    
    let elapsed = start_time.elapsed();
    assert!(elapsed.as_millis() < 100, "Algorithms should complete quickly on 100-node graph");
    
    println!("✅ Performance test completed in {:?} for {}-node graph", elapsed, num_nodes + 1);
}