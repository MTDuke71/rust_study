//! Integration tests for Mission 7: Graph Representation
//!
//! These tests validate the complete V-Cycle requirements through real-world scenarios
//! and comprehensive integration testing.

use mission7::{utils, Graph, GraphType};

// ============================================================================
// REQ-1: Graph Structure and Node Storage
// ============================================================================

#[test] // REQ-1: Graph structure with different data types
fn req1_graph_structure() {
    // String data
    let graph_str: Graph<&str> = Graph::new_directed();
    assert_eq!(graph_str.node_count(), 0);
    assert_eq!(graph_str.edge_count(), 0);
    assert!(graph_str.is_empty());
    assert_eq!(graph_str.graph_type, GraphType::Directed);

    // Integer data
    let graph_int: Graph<i32> = Graph::new_undirected();
    assert_eq!(graph_int.graph_type, GraphType::Undirected);

    // Custom struct data
    #[derive(Debug, Clone, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    let mut graph_person: Graph<Person> = Graph::new_directed();
    let alice = graph_person.add_node(Person {
        name: "Alice".to_string(),
        age: 30,
    });

    assert_eq!(graph_person.node_count(), 1);
    assert_eq!(graph_person.get_node(alice).unwrap().name, "Alice");
}

#[test] // REQ-1: Node storage with O(1) addition
fn req1_node_storage() {
    let mut graph: Graph<String> = Graph::new_directed();

    // Add multiple nodes
    let nodes: Vec<_> = (0..100)
        .map(|i| graph.add_node(format!("Node{}", i)))
        .collect();

    assert_eq!(graph.node_count(), 100);

    // Verify node IDs are sequential
    for (i, &node_id) in nodes.iter().enumerate() {
        assert_eq!(node_id, i);
        assert_eq!(graph.get_node(node_id).unwrap(), &format!("Node{}", i));
    }
}

#[test] // REQ-1: Adjacency list representation efficiency
fn req1_adjacency_list() {
    let mut graph: Graph<&str> = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    // Build adjacency structure
    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);

    // Verify neighbors
    let neighbors_a = graph.neighbors(a);
    assert_eq!(neighbors_a.len(), 2);
    assert!(neighbors_a.contains(&b));
    assert!(neighbors_a.contains(&c));

    let neighbors_b = graph.neighbors(b);
    assert_eq!(neighbors_b.len(), 1);
    assert!(neighbors_b.contains(&d));

    let neighbors_d = graph.neighbors(d);
    assert_eq!(neighbors_d.len(), 0);
}

// ============================================================================
// REQ-2: Edge Management Operations
// ============================================================================

#[test] // REQ-2: Add edge with validation
fn req2_add_edge() {
    let mut graph: Graph<&str> = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");

    // Valid edge addition
    assert!(graph.add_edge(a, b));
    assert!(graph.has_edge(a, b));
    assert_eq!(graph.edge_count(), 1);

    // Duplicate edge should not increase count
    assert!(graph.add_edge(a, b));
    assert_eq!(graph.edge_count(), 1);

    // Invalid edge (non-existent node)
    assert!(!graph.add_edge(a, 999));
    assert_eq!(graph.edge_count(), 1);
}

#[test] // REQ-2: Remove edge operations
fn req2_remove_edge() {
    let mut graph: Graph<&str> = Graph::new_undirected();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    graph.add_edge(a, b);
    graph.add_edge(b, c);
    assert_eq!(graph.edge_count(), 2);

    // Remove existing edge
    assert!(graph.remove_edge(a, b));
    assert!(!graph.has_edge(a, b));
    assert!(!graph.has_edge(b, a)); // Undirected
    assert_eq!(graph.edge_count(), 1);

    // Remove non-existent edge
    assert!(!graph.remove_edge(a, b));
    assert_eq!(graph.edge_count(), 1);

    // Verify other edges still exist
    assert!(graph.has_edge(b, c));
}

#[test] // REQ-2: Edge validation for directed vs undirected
fn req2_edge_validation() {
    // Directed graph
    let mut directed: Graph<&str> = Graph::new_directed();
    let a = directed.add_node("A");
    let b = directed.add_node("B");

    directed.add_edge(a, b);
    assert!(directed.has_edge(a, b));
    assert!(!directed.has_edge(b, a)); // One-way only
    assert_eq!(directed.edge_count(), 1);

    // Undirected graph
    let mut undirected: Graph<&str> = Graph::new_undirected();
    let c = undirected.add_node("C");
    let d = undirected.add_node("D");

    undirected.add_edge(c, d);
    assert!(undirected.has_edge(c, d));
    assert!(undirected.has_edge(d, c)); // Bidirectional
    assert_eq!(undirected.edge_count(), 1); // Still counts as 1 edge
}

#[test] // REQ-2: Neighbor lookup operations
fn req2_neighbor_lookup() {
    let mut graph: Graph<i32> = Graph::new_directed();
    let nodes: Vec<_> = (0..5).map(|i| graph.add_node(i)).collect();

    // Build a star pattern: center -> all others
    let center = nodes[0];
    for &node in &nodes[1..] {
        graph.add_edge(center, node);
    }

    // Check neighbor count (degree)
    assert_eq!(graph.degree(center), 4);

    // Verify all neighbors
    let neighbors = graph.neighbors(center);
    for i in 1..5 {
        assert!(neighbors.contains(&nodes[i]));
    }

    // Leaf nodes have no outgoing edges
    for &node in &nodes[1..] {
        assert_eq!(graph.degree(node), 0);
    }
}

// ============================================================================
// REQ-3: Graph Building and Validation
// ============================================================================

#[test] // REQ-3: Graph construction from edge list
fn req3_graph_construction() {
    let mut graph: Graph<char> = Graph::new_undirected();

    // Add nodes
    let nodes: Vec<_> = "ABCDEF".chars().map(|c| graph.add_node(c)).collect();

    // Define edges
    let edges = [
        (0, 1),
        (0, 2), // A connects to B, C
        (1, 3),
        (1, 4), // B connects to D, E
        (2, 4),
        (2, 5), // C connects to E, F
        (3, 4), // D connects to E
    ];

    // Build graph from edge list
    for (from_idx, to_idx) in edges.iter() {
        graph.add_edge(nodes[*from_idx], nodes[*to_idx]);
    }

    assert_eq!(graph.node_count(), 6);
    assert_eq!(graph.edge_count(), 7);

    // Verify specific connections
    assert!(graph.has_edge(nodes[0], nodes[1])); // A-B
    assert!(graph.has_edge(nodes[1], nodes[3])); // B-D
    assert!(graph.has_edge(nodes[2], nodes[5])); // C-F
}

#[test] // REQ-3: Connectivity checking
fn req3_connectivity() {
    // Connected graph
    let mut connected: Graph<&str> = Graph::new_undirected();
    let a = connected.add_node("A");
    let b = connected.add_node("B");
    let c = connected.add_node("C");

    connected.add_edge(a, b);
    connected.add_edge(b, c);

    // BFS should reach all nodes from any starting point
    let bfs_result = connected.bfs(a);
    assert_eq!(bfs_result.visited.len(), 3);

    // Disconnected graph
    let mut disconnected: Graph<&str> = Graph::new_undirected();
    let x = disconnected.add_node("X");
    let y = disconnected.add_node("Y");
    let z = disconnected.add_node("Z");

    disconnected.add_edge(x, y);
    // Z is isolated

    let bfs_result = disconnected.bfs(x);
    assert_eq!(bfs_result.visited.len(), 2);
    assert!(!bfs_result.visited.contains(&z));
}

#[test] // REQ-3: Connected components analysis
fn req3_components() {
    let mut graph: Graph<&str> = Graph::new_undirected();

    // Component 1: Triangle
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(c, a);

    // Component 2: Line
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    graph.add_edge(d, e);

    // Component 3: Isolated node
    let f = graph.add_node("F");

    let components = graph.connected_components();
    assert_eq!(components.len(), 3);

    // Verify component sizes
    let sizes: Vec<_> = components.iter().map(|c| c.len()).collect();
    assert!(sizes.contains(&3)); // Triangle
    assert!(sizes.contains(&2)); // Line
    assert!(sizes.contains(&1)); // Isolated (F)

    // Verify F is in its own component
    assert!(components
        .iter()
        .any(|comp| comp.len() == 1 && comp.contains(&f)));
}

#[test] // REQ-3: Graph statistics
fn req3_statistics() {
    let mut graph: Graph<i32> = Graph::new_undirected();
    let nodes: Vec<_> = (0..5).map(|i| graph.add_node(i)).collect();

    // Create a path graph: 0-1-2-3-4
    for i in 0..4 {
        graph.add_edge(nodes[i], nodes[i + 1]);
    }

    assert_eq!(graph.node_count(), 5);
    assert_eq!(graph.edge_count(), 4);

    // Degree distribution
    assert_eq!(graph.degree(nodes[0]), 1); // Endpoint
    assert_eq!(graph.degree(nodes[2]), 2); // Middle
    assert_eq!(graph.degree(nodes[4]), 1); // Endpoint

    // Test with utility function
    let stats = utils::graph_stats(&graph);
    assert_eq!(stats.node_count, 5);
    assert_eq!(stats.edge_count, 4);
    assert_eq!(stats.max_degree, 2);
    assert_eq!(stats.min_degree, 1);
}

// ============================================================================
// REQ-4: Algorithm Foundation
// ============================================================================

#[test] // REQ-4: Visited tracking for traversal
fn req4_visited_tracking() {
    let mut graph: Graph<&str> = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(c, d);
    graph.add_edge(d, a); // Create cycle

    let dfs_result = graph.dfs(a);

    // All nodes should be visited
    assert_eq!(dfs_result.visited.len(), 4);
    assert!(dfs_result.visited.contains(&a));
    assert!(dfs_result.visited.contains(&b));
    assert!(dfs_result.visited.contains(&c));
    assert!(dfs_result.visited.contains(&d));
}

#[test] // REQ-4: Path reconstruction
fn req4_path_reconstruction() {
    let mut graph: Graph<char> = Graph::new_directed();
    let nodes: Vec<_> = "ABCDE".chars().map(|c| graph.add_node(c)).collect();

    // Build path: A -> B -> C -> D -> E
    for i in 0..4 {
        graph.add_edge(nodes[i], nodes[i + 1]);
    }

    let bfs_result = graph.bfs(nodes[0]);

    // Check parent relationships for path reconstruction
    assert_eq!(bfs_result.parents[nodes[1]], Some(nodes[0])); // B's parent is A
    assert_eq!(bfs_result.parents[nodes[2]], Some(nodes[1])); // C's parent is B
    assert_eq!(bfs_result.parents[nodes[4]], Some(nodes[3])); // E's parent is D

    // Reconstruct path using shortest_path
    let path = graph.shortest_path(nodes[0], nodes[4]);
    assert_eq!(
        path,
        Some(vec![nodes[0], nodes[1], nodes[2], nodes[3], nodes[4]])
    );
}

#[test] // REQ-4: Algorithm infrastructure - result structures
fn req4_algorithm_infrastructure() {
    let mut graph: Graph<&str> = Graph::new_undirected();
    let a = graph.add_node("A");
    let b = graph.add_node("B");

    graph.add_edge(a, b);

    // DFS result structure
    let dfs_result = graph.dfs(a);
    assert!(!dfs_result.path.is_empty());
    assert!(!dfs_result.visited.is_empty());

    // BFS result structure
    let bfs_result = graph.bfs(a);
    assert!(!bfs_result.path.is_empty());
    assert!(!bfs_result.visited.is_empty());
    assert!(!bfs_result.distances.is_empty());
    assert!(!bfs_result.parents.is_empty());
}

// ============================================================================
// REQ-5: DFS Implementation
// ============================================================================

#[test] // REQ-5: DFS recursive traversal
fn req5_dfs_recursive() {
    let mut graph: Graph<&str> = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    // Tree structure
    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);

    let dfs_result = graph.dfs(a);

    // Should visit all reachable nodes
    assert_eq!(dfs_result.path.len(), 4);
    assert_eq!(dfs_result.visited.len(), 4);

    // A should be first
    assert_eq!(dfs_result.path[0], a);
}

#[test] // REQ-5: DFS iterative with explicit stack
fn req5_dfs_iterative() {
    let mut graph: Graph<i32> = Graph::new_directed();
    let nodes: Vec<_> = (0..6).map(|i| graph.add_node(i)).collect();

    // Build binary tree structure
    graph.add_edge(nodes[0], nodes[1]);
    graph.add_edge(nodes[0], nodes[2]);
    graph.add_edge(nodes[1], nodes[3]);
    graph.add_edge(nodes[1], nodes[4]);
    graph.add_edge(nodes[2], nodes[5]);

    let dfs_result = graph.dfs_iterative(nodes[0]);

    // Should visit all nodes
    assert_eq!(dfs_result.path.len(), 6);
    assert_eq!(dfs_result.visited.len(), 6);
}

#[test] // REQ-5: DFS path finding
fn req5_dfs_pathfinding() {
    let mut graph: Graph<&str> = Graph::new_directed();
    let start = graph.add_node("Start");
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let end = graph.add_node("End");

    graph.add_edge(start, a);
    graph.add_edge(a, b);
    graph.add_edge(b, end);

    let dfs_result = graph.dfs(start);

    // Should find path to end
    assert!(dfs_result.visited.contains(&end));
    assert!(dfs_result.path.contains(&end));
}

#[test] // REQ-5: DFS component analysis
fn req5_dfs_components() {
    let mut graph: Graph<char> = Graph::new_undirected();

    // Component 1
    let a = graph.add_node('A');
    let b = graph.add_node('B');
    graph.add_edge(a, b);

    // Component 2
    let c = graph.add_node('C');
    let d = graph.add_node('D');
    let e = graph.add_node('E');
    graph.add_edge(c, d);
    graph.add_edge(d, e);

    // DFS from A should only reach component 1
    let dfs_result = graph.dfs(a);
    assert_eq!(dfs_result.visited.len(), 2);
    assert!(dfs_result.visited.contains(&a));
    assert!(dfs_result.visited.contains(&b));
    assert!(!dfs_result.visited.contains(&c));

    // Use connected_components to find all
    let components = graph.connected_components();
    assert_eq!(components.len(), 2);
}

// ============================================================================
// REQ-6: BFS Implementation
// ============================================================================

#[test] // REQ-6: BFS traversal with level order
fn req6_bfs_traversal() {
    let mut graph: Graph<i32> = Graph::new_directed();
    let nodes: Vec<_> = (0..7).map(|i| graph.add_node(i)).collect();

    // Build tree: level-order
    //       0
    //      / \
    //     1   2
    //    / \   \
    //   3   4   5
    //  /
    // 6
    graph.add_edge(nodes[0], nodes[1]);
    graph.add_edge(nodes[0], nodes[2]);
    graph.add_edge(nodes[1], nodes[3]);
    graph.add_edge(nodes[1], nodes[4]);
    graph.add_edge(nodes[2], nodes[5]);
    graph.add_edge(nodes[3], nodes[6]);

    let bfs_result = graph.bfs(nodes[0]);

    // Should visit all nodes
    assert_eq!(bfs_result.visited.len(), 7);

    // Check level-order traversal path
    assert_eq!(bfs_result.path[0], nodes[0]); // Level 0
}

#[test] // REQ-6: Shortest path in unweighted graph
fn req6_shortest_path() {
    let mut graph: Graph<&str> = Graph::new_undirected();
    let nodes: Vec<_> = ["A", "B", "C", "D", "E"]
        .iter()
        .map(|&s| graph.add_node(s))
        .collect();

    // Create graph with multiple paths from A to E
    graph.add_edge(nodes[0], nodes[1]); // A-B
    graph.add_edge(nodes[1], nodes[2]); // B-C
    graph.add_edge(nodes[2], nodes[4]); // C-E (path: A-B-C-E = 3 edges, 4 nodes)
    graph.add_edge(nodes[0], nodes[3]); // A-D
    graph.add_edge(nodes[3], nodes[4]); // D-E (path: A-D-E = 2 edges, 3 nodes) - SHORTEST

    let path = graph.shortest_path(nodes[0], nodes[4]);
    assert!(path.is_some());

    let path = path.unwrap();
    assert_eq!(path.len(), 3); // A-D-E (shortest path with 3 nodes)
    assert_eq!(path[0], nodes[0]); // Start at A
    assert_eq!(path[path.len() - 1], nodes[4]); // End at E
}

#[test] // REQ-6: Distance calculation
fn req6_distance_calculation() {
    let mut graph: Graph<char> = Graph::new_undirected();
    let nodes: Vec<_> = "ABCDEF".chars().map(|c| graph.add_node(c)).collect();

    // Create chain: A-B-C-D-E-F
    for i in 0..5 {
        graph.add_edge(nodes[i], nodes[i + 1]);
    }

    let bfs_result = graph.bfs(nodes[0]);

    // Check distances from A
    assert_eq!(bfs_result.distances[nodes[0]], Some(0)); // A to A
    assert_eq!(bfs_result.distances[nodes[1]], Some(1)); // A to B
    assert_eq!(bfs_result.distances[nodes[2]], Some(2)); // A to C
    assert_eq!(bfs_result.distances[nodes[3]], Some(3)); // A to D
    assert_eq!(bfs_result.distances[nodes[4]], Some(4)); // A to E
    assert_eq!(bfs_result.distances[nodes[5]], Some(5)); // A to F
}

#[test] // REQ-6: BFS level-order properties
fn req6_level_order() {
    let mut graph: Graph<&str> = Graph::new_undirected();

    // Build a square with diagonals
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(c, d);
    graph.add_edge(d, a);
    graph.add_edge(a, c); // Diagonal

    let bfs_result = graph.bfs(a);

    // All nodes at distance 1 or 2 from A
    assert_eq!(bfs_result.distances[a], Some(0));
    assert_eq!(bfs_result.distances[b], Some(1));
    assert_eq!(bfs_result.distances[c], Some(1)); // Via diagonal
    assert_eq!(bfs_result.distances[d], Some(1)); // Via edge
}

// ============================================================================
// Real-world Integration Scenarios
// ============================================================================

#[test]
fn integration_social_network() {
    let mut network: Graph<&str> = Graph::new_undirected();

    // Add people
    let alice = network.add_node("Alice");
    let bob = network.add_node("Bob");
    let charlie = network.add_node("Charlie");
    let diana = network.add_node("Diana");
    let eve = network.add_node("Eve");
    let frank = network.add_node("Frank");

    // Add friendships
    network.add_edge(alice, bob);
    network.add_edge(alice, charlie);
    network.add_edge(bob, diana);
    network.add_edge(charlie, diana);
    network.add_edge(diana, eve);
    // Frank is isolated - verify he has no connections
    assert!(network.neighbors(frank).is_empty());

    // Find shortest connection between Alice and Eve
    let path = network.shortest_path(alice, eve);
    assert!(path.is_some());

    // Find friend groups (connected components)
    let groups = network.connected_components();
    assert_eq!(groups.len(), 2); // Main group + Frank

    // Network statistics
    let stats = utils::graph_stats(&network);
    assert_eq!(stats.node_count, 6);
    assert_eq!(stats.edge_count, 5);
}

#[test]
fn integration_maze_solving() {
    // Represent a 3x3 grid maze as a graph
    //  S # #
    //  . . #
    //  # . E

    let mut maze: Graph<(i32, i32)> = Graph::new_undirected();

    // Add walkable cells
    let start = maze.add_node((0, 0));
    let cell_10 = maze.add_node((1, 0));
    let cell_20 = maze.add_node((2, 0));
    let cell_11 = maze.add_node((1, 1));
    let end = maze.add_node((2, 1));

    // Add connections (4-directional movement)
    maze.add_edge(start, cell_10);
    maze.add_edge(cell_10, cell_11);
    maze.add_edge(cell_10, cell_20);
    maze.add_edge(cell_20, end);
    maze.add_edge(cell_11, end);

    // Find shortest path
    let solution = maze.shortest_path(start, end);
    assert!(solution.is_some());

    let path = solution.unwrap();
    assert_eq!(path[0], start);
    assert_eq!(path[path.len() - 1], end);
}

#[test]
fn integration_dependency_resolution() {
    let mut deps: Graph<&str> = Graph::new_directed();

    // Software dependencies
    let app = deps.add_node("App");
    let auth = deps.add_node("Auth");
    let database = deps.add_node("Database");
    let logging = deps.add_node("Logging");
    let config = deps.add_node("Config");

    // Dependencies: App -> Auth -> Database -> Logging -> Config
    deps.add_edge(app, auth);
    deps.add_edge(auth, database);
    deps.add_edge(database, logging);
    deps.add_edge(logging, config);
    deps.add_edge(app, config); // Direct dependency too

    // Check for cycles (should be DAG)
    assert!(!deps.has_cycle());

    // Topological order via DFS
    let dfs_result = deps.dfs(app);
    assert_eq!(dfs_result.visited.len(), 5);
}

#[test]
fn integration_network_topology() {
    let mut network: Graph<&str> = Graph::new_undirected();

    // Router network
    let gateway = network.add_node("Gateway");
    let router_a = network.add_node("RouterA");
    let router_b = network.add_node("RouterB");
    let router_c = network.add_node("RouterC");
    let server = network.add_node("Server");

    // Network connections
    network.add_edge(gateway, router_a);
    network.add_edge(gateway, router_b);
    network.add_edge(router_a, router_c);
    network.add_edge(router_b, router_c);
    network.add_edge(router_c, server);

    // Find routing path
    let route = network.shortest_path(gateway, server);
    assert!(route.is_some());

    // Verify redundancy (multiple paths exist)
    let path = route.unwrap();
    assert!(path.len() <= 4); // At most 3 hops

    // Network connectivity
    let bfs_result = network.bfs(gateway);
    assert_eq!(bfs_result.visited.len(), 5); // All routers reachable
}
