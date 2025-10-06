//! Mission 7 Demo: Graph Representation and Algorithms
//!
//! This demo showcases the graph data structure implementation with
//! various graph types, traversal algorithms, and real-world applications.

use mission7::{Graph, GraphType, NodeId, utils};

fn main() {
    println!("=== Mission 7: Graph Representation Demo ===\n");
    
    // 1. Basic Graph Operations
    basic_graph_operations();
    
    // 2. Directed vs Undirected Graphs
    directed_vs_undirected();
    
    // 3. DFS and BFS Traversal
    traversal_algorithms();
    
    // 4. Path Finding
    path_finding();
    
    // 5. Cycle Detection
    cycle_detection();
    
    // 6. Connected Components
    connected_components();
    
    // 7. Graph Utilities
    graph_utilities();
    
    // 8. Real-world Example: Social Network
    social_network_example();
    
    println!("\n=== Demo Complete ===");
}

fn basic_graph_operations() {
    println!("1. Basic Graph Operations");
    println!("========================");
    
    let mut graph = Graph::new_directed();
    
    // Add nodes
    let alice = graph.add_node("Alice");
    let bob = graph.add_node("Bob");
    let charlie = graph.add_node("Charlie");
    let diana = graph.add_node("Diana");
    
    println!("Added nodes: Alice({}), Bob({}), Charlie({}), Diana({})", 
             alice, bob, charlie, diana);
    
    // Add edges
    graph.add_edge(alice, bob);
    graph.add_edge(bob, charlie);
    graph.add_edge(charlie, diana);
    graph.add_edge(diana, alice);
    
    println!("Added edges: Alice->Bob, Bob->Charlie, Charlie->Diana, Diana->Alice");
    println!("Graph stats: {} nodes, {} edges", graph.node_count(), graph.edge_count());
    
    // Check neighbors
    println!("Alice's neighbors: {:?}", graph.neighbors(alice));
    println!("Bob's neighbors: {:?}", graph.neighbors(bob));
    println!("Alice's degree: {}", graph.degree(alice));
    
    println!();
}

fn directed_vs_undirected() {
    println!("2. Directed vs Undirected Graphs");
    println!("================================");
    
    // Directed graph
    let mut directed = Graph::new_directed();
    let a = directed.add_node("A");
    let b = directed.add_node("B");
    directed.add_edge(a, b);
    
    println!("Directed graph:");
    println!("  A->B exists: {}", directed.has_edge(a, b));
    println!("  B->A exists: {}", directed.has_edge(b, a));
    
    // Undirected graph
    let mut undirected = Graph::new_undirected();
    let a = undirected.add_node("A");
    let b = undirected.add_node("B");
    undirected.add_edge(a, b);
    
    println!("Undirected graph:");
    println!("  A->B exists: {}", undirected.has_edge(a, b));
    println!("  B->A exists: {}", undirected.has_edge(b, a));
    
    println!();
}

fn traversal_algorithms() {
    println!("3. DFS and BFS Traversal");
    println!("========================");
    
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    
    // Create a tree-like structure
    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);
    graph.add_edge(b, e);
    
    println!("Graph structure:");
    println!("    A");
    println!("   / \\");
    println!("  B   C");
    println!(" / \\");
    println!("D   E");
    
    // DFS traversal
    let dfs_result = graph.dfs(a);
    println!("DFS traversal: {:?}", dfs_result.path);
    
    // BFS traversal
    let bfs_result = graph.bfs(a);
    println!("BFS traversal: {:?}", bfs_result.path);
    
    // Iterative DFS
    let dfs_iter_result = graph.dfs_iterative(a);
    println!("Iterative DFS: {:?}", dfs_iter_result.path);
    
    println!();
}

fn path_finding() {
    println!("4. Path Finding");
    println!("===============");
    
    let mut graph = Graph::new_undirected();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    
    // Create a path: A-B-C-D-E
    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(c, d);
    graph.add_edge(d, e);
    
    println!("Finding shortest path from A to E:");
    if let Some(path) = graph.shortest_path(a, e) {
        println!("  Path: {:?}", path);
        println!("  Distance: {}", path.len() - 1);
    } else {
        println!("  No path found");
    }
    
    // BFS with distance information
    let bfs_result = graph.bfs(a);
    println!("Distances from A:");
    for (node, distance) in bfs_result.distances.iter().enumerate() {
        if let Some(dist) = distance {
            println!("  Node {}: distance {}", node, dist);
        }
    }
    
    println!();
}

fn cycle_detection() {
    println!("5. Cycle Detection");
    println!("==================");
    
    // Graph with cycle
    let mut graph_with_cycle = Graph::new_directed();
    let a = graph_with_cycle.add_node("A");
    let b = graph_with_cycle.add_node("B");
    let c = graph_with_cycle.add_node("C");
    
    graph_with_cycle.add_edge(a, b);
    graph_with_cycle.add_edge(b, c);
    graph_with_cycle.add_edge(c, a);
    
    println!("Graph with cycle (A->B->C->A):");
    println!("  Has cycle: {}", graph_with_cycle.has_cycle());
    
    // Graph without cycle
    let mut graph_without_cycle = Graph::new_directed();
    let a = graph_without_cycle.add_node("A");
    let b = graph_without_cycle.add_node("B");
    let c = graph_without_cycle.add_node("C");
    
    graph_without_cycle.add_edge(a, b);
    graph_without_cycle.add_edge(b, c);
    
    println!("Graph without cycle (A->B->C):");
    println!("  Has cycle: {}", graph_without_cycle.has_cycle());
    
    println!();
}

fn connected_components() {
    println!("6. Connected Components");
    println!("=======================");
    
    let mut graph = Graph::new_undirected();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    let f = graph.add_node("F");
    
    // Create two disconnected components
    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(d, e);
    graph.add_edge(e, f);
    
    println!("Graph with two components:");
    println!("  Component 1: A-B-C");
    println!("  Component 2: D-E-F");
    
    let components = graph.connected_components();
    println!("  Found {} components:", components.len());
    for (i, component) in components.iter().enumerate() {
        println!("    Component {}: {:?}", i + 1, component);
    }
    
    println!("  Is connected: {}", graph.is_connected());
    
    println!();
}

fn graph_utilities() {
    println!("7. Graph Utilities");
    println!("==================");
    
    // Create a complete graph
    let complete = utils::complete_graph(4, "Node");
    println!("Complete graph with 4 nodes:");
    println!("  Nodes: {}, Edges: {}", complete.node_count(), complete.edge_count());
    
    // Create a path graph
    let path = utils::path_graph(5, "Node");
    println!("Path graph with 5 nodes:");
    println!("  Nodes: {}, Edges: {}", path.node_count(), path.edge_count());
    
    // Create a cycle graph
    let cycle = utils::cycle_graph(6, "Node");
    println!("Cycle graph with 6 nodes:");
    println!("  Nodes: {}, Edges: {}", cycle.node_count(), cycle.edge_count());
    
    // Graph statistics
    let stats = utils::graph_stats(&complete);
    println!("Complete graph statistics:");
    println!("  Density: {:.2}", stats.density);
    println!("  Average degree: {:.2}", stats.avg_degree);
    println!("  Max degree: {}", stats.max_degree);
    println!("  Min degree: {}", stats.min_degree);
    
    println!();
}

fn social_network_example() {
    println!("8. Real-world Example: Social Network");
    println!("=====================================");
    
    let mut network = Graph::new_undirected();
    
    // Add people to the network
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
    network.add_edge(eve, frank);
    
    println!("Social network with {} people and {} friendships", 
             network.node_count(), network.edge_count());
    
    // Find mutual friends
    println!("Alice's friends: {:?}", network.neighbors(alice));
    println!("Bob's friends: {:?}", network.neighbors(bob));
    
    // Find shortest path between two people
    if let Some(path) = network.shortest_path(alice, frank) {
        println!("Shortest path from Alice to Frank: {:?}", path);
        println!("  Degrees of separation: {}", path.len() - 1);
    }
    
    // Find all connected components (friend groups)
    let friend_groups = network.connected_components();
    println!("Friend groups: {}", friend_groups.len());
    for (i, group) in friend_groups.iter().enumerate() {
        println!("  Group {}: {:?}", i + 1, group);
    }
    
    // Network statistics
    let stats = utils::graph_stats(&network);
    println!("Network statistics:");
    println!("  Average friends per person: {:.1}", stats.avg_degree);
    println!("  Most popular person has {} friends", stats.max_degree);
    println!("  Network density: {:.2}", stats.density);
    
    println!();
}
