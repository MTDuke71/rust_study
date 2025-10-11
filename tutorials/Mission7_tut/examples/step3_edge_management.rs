//! Step 3: Edge Management
//!
//! This step focuses on comprehensive edge management, including adding/removing edges,
//! validation, and efficient neighbor operations.
//!
//! ## Learning Objectives
//! - Master edge addition and removal operations
//! - Learn edge validation and error handling
//! - Understand neighbor lookup and enumeration
//! - Explore graph statistics and analysis
//!
//! ## Alignment with Mission 7
//! This step directly implements **REQ-2: Edge Management Operations** by teaching
//! comprehensive edge handling and validation techniques.
//!
//! ## Calendar Alignment
//! **Day 3 (Oct 10)**: Graph Building & Edge Management
//! - Add methods: add_edge, remove_edge, neighbors
//! - Implement direction enums and bounds checking

use mission7::Graph;
// use mission7_tut::tutorial_utils;

/// Helper function to print neighbors with their actual data
fn print_neighbors_with_data(graph: &Graph<&str>, node_id: usize, node_name: &str) {
    let neighbors = graph.neighbors(node_id);
    let neighbor_names: Vec<&str> = neighbors.iter()
        .filter_map(|&id| graph.get_node(id))
        .copied()
        .collect();
    
    println!("  {}'s neighbor IDs: {:?} → {} (count: {})", 
        node_name, 
        neighbors, 
        if neighbor_names.is_empty() { 
            "none".to_string() 
        } else { 
            neighbor_names.join(", ") 
        },
        neighbors.len()
    );
}

fn main() {
    println!("=== Step 3: Edge Management ===\n");
    
    // 1. Adding Edges
    adding_edges();
    
    // 2. Removing Edges
    removing_edges();
    
    // 3. Edge Validation
    edge_validation();
    
    // 4. Neighbor Operations
    neighbor_operations();
    
    // 5. Graph Statistics
    graph_statistics();
    
    // 6. Edge Iteration
    edge_iteration();

    exercise();

    
    println!("\n=== Step 3 Complete ===");
    println!("Next: Step 4 - Algorithm Foundation");
}

fn adding_edges() {
    println!("1. Adding Edges");
    println!("==============");
    
    println!("Edge Addition Process:");
    println!();
    
    // Create a directed graph
    let mut directed = Graph::new_directed();
    let a = directed.add_node("A");
    let b = directed.add_node("B");
    let c = directed.add_node("C");
    
    println!("Step 1: Create nodes A, B, C");
    println!("  Node IDs: A={}, B={}, C={}", a, b, c);
    println!();
    
    // Add edges one by one
    println!("Step 2: Add edges");
    
    let result1 = directed.add_edge(a, b);
    println!("  Add A -> B: {}", if result1 { "Success" } else { "Failed" });
    print_neighbors_with_data(&directed, a, "A");
    print_neighbors_with_data(&directed, b, "B");
    println!("  Total edge count: {}", directed.edge_count());
    println!();
    
    let result2 = directed.add_edge(b, c);
    println!("  Add B -> C: {}", if result2 { "Success" } else { "Failed" });
    print_neighbors_with_data(&directed, b, "B");
    print_neighbors_with_data(&directed, c, "C");
    println!("  Total edge count: {}", directed.edge_count());
    println!();
    
    let result3 = directed.add_edge(c, a);
    println!("  Add C -> A: {}", if result3 { "Success" } else { "Failed" });
    print_neighbors_with_data(&directed, c, "C");
    print_neighbors_with_data(&directed, a, "A");
    println!("  Total edge count: {}", directed.edge_count());
    println!("  👆 Note: A's neighbor count is still 1 (only B)");
    println!("     Adding C→A doesn't change who A points to!");
    println!();
    
    // Try adding duplicate edge
    let result4 = directed.add_edge(a, b);
    println!("  Add A -> B again: {}", if result4 { "Success" } else { "Failed" });
    println!("  Total edge count: {} (unchanged - duplicate prevented)", directed.edge_count());
    println!();
    
    // Undirected graph example
    println!("Undirected Graph Example:");
    let mut undirected = Graph::new_undirected();
    let x = undirected.add_node("X");
    let y = undirected.add_node("Y");
    
    undirected.add_edge(x, y);
    println!("  Add X <-> Y (undirected)");
    println!("  X's neighbors: {:?}", undirected.neighbors(x));
    println!("  Y's neighbors: {:?}", undirected.neighbors(y));
    println!("  Edge count: {}", undirected.edge_count());
    println!();
}

fn removing_edges() {
    println!("2. Removing Edges");
    println!("================");
    
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    
    // Add multiple edges
    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, c);
    graph.add_edge(c, d);
    
    println!("Initial graph with {} edges:", graph.edge_count());
    for node_id in graph.nodes() {
        if let Some(data) = graph.get_node(node_id) {
            println!("  {}: {:?}", data, graph.neighbors(node_id));
        }
    }
    println!();
    
    // Remove edges one by one
    println!("Removing edges:");
    
    let result1 = graph.remove_edge(a, c);
    println!("  Remove A -> C: {}", if result1 { "Success" } else { "Failed" });
    println!("  A's neighbors: {:?}", graph.neighbors(a));
    println!("  Edge count: {}", graph.edge_count());
    println!();
    
    let result2 = graph.remove_edge(b, c);
    println!("  Remove B -> C: {}", if result2 { "Success" } else { "Failed" });
    println!("  B's neighbors: {:?}", graph.neighbors(b));
    println!("  Edge count: {}", graph.edge_count());
    println!();
    
    // Try removing non-existent edge
    let result3 = graph.remove_edge(a, d);
    println!("  Remove A -> D (doesn't exist): {}", if result3 { "Success" } else { "Failed" });
    println!("  Edge count: {} (should be same)", graph.edge_count());
    println!();
    
    // Undirected graph removal
    println!("Undirected Graph Removal:");
    let mut undirected = Graph::new_undirected();
    let x = undirected.add_node("X");
    let y = undirected.add_node("Y");
    let z = undirected.add_node("Z");
    
    undirected.add_edge(x, y);
    undirected.add_edge(y, z);
    
    println!("  Before removal:");
    println!("    X: {:?}", undirected.neighbors(x));
    println!("    Y: {:?}", undirected.neighbors(y));
    println!("    Z: {:?}", undirected.neighbors(z));
    println!("    Edge count: {}", undirected.edge_count());
    
    undirected.remove_edge(x, y);
    println!("  After removing X <-> Y:");
    println!("    X: {:?}", undirected.neighbors(x));
    println!("    Y: {:?}", undirected.neighbors(y));
    println!("    Z: {:?}", undirected.neighbors(z));
    println!("    Edge count: {}", undirected.edge_count());
    println!();
}

fn edge_validation() {
    println!("3. Edge Validation");
    println!("=================");
    
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    
    println!("Edge Validation Tests:");
    println!();
    
    // Valid edge
    println!("Test 1: Valid edge");
    let result1 = graph.add_edge(a, b);
    println!("  Add A -> B: {}", if result1 { "Valid" } else { "Invalid" });
    println!("  A -> B exists: {}", graph.has_edge(a, b));
    println!();
    
    // Self-loop
    println!("Test 2: Self-loop");
    let result2 = graph.add_edge(a, a);
    println!("  Add A -> A: {}", if result2 { "Valid" } else { "Invalid" });
    println!("  A -> A exists: {}", graph.has_edge(a, a));
    println!("  A's neighbors: {:?}", graph.neighbors(a));
    println!();
    
    // Invalid node IDs
    println!("Test 3: Invalid node IDs");
    let result3 = graph.add_edge(999, b);
    println!("  Add 999 -> B: {}", if result3 { "Valid" } else { "Invalid" });
    
    let result4 = graph.add_edge(a, 999);
    println!("  Add A -> 999: {}", if result4 { "Valid" } else { "Invalid" });
    println!();
    
    // Edge existence check
    println!("Test 4: Edge existence checks");
    println!("  A -> B exists: {}", graph.has_edge(a, b));
    println!("  B -> A exists: {}", graph.has_edge(b, a));
    println!("  A -> C exists: {}", graph.has_edge(a, c));
    println!("  B -> C exists: {}", graph.has_edge(b, c));
    println!();
    
    // Duplicate edge handling
    println!("Test 5: Duplicate edge handling");
    let result5 = graph.add_edge(a, b);
    println!("  Add A -> B again: {}", if result5 { "Added" } else { "Ignored" });
    println!("  Edge count: {}", graph.edge_count());
    println!("  A's neighbors: {:?}", graph.neighbors(a));
    println!();
}

fn neighbor_operations() {
    println!("4. Neighbor Operations");
    println!("=====================");
    
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    
    // Create a complex graph
    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);
    graph.add_edge(c, d);
    graph.add_edge(d, e);
    graph.add_edge(e, a);
    
    println!("Neighbor Operations:");
    println!();
    
    // Get neighbors
    println!("1. Get Neighbors:");
    for node_id in graph.nodes() {
        if let Some(data) = graph.get_node(node_id) {
            let neighbors: Vec<String> = graph.neighbors(node_id)
                .iter()
                .map(|&id| {
                    graph.get_node(id)
                        .map(|d| d.to_string())
                        .unwrap_or_else(|| format!("Node{}", id))
                })
                .collect();
            println!("  {}: [{}]", data, neighbors.join(", "));
        }
    }
    println!();
    
    // Get degree
    println!("2. Get Degree (number of neighbors):");
    for node_id in graph.nodes() {
        if let Some(data) = graph.get_node(node_id) {
            println!("  {}: degree {}", data, graph.degree(node_id));
        }
    }
    println!();
    
    // Find nodes with specific degrees
    println!("3. Find nodes with specific degrees:");
    let mut degree_counts = std::collections::HashMap::new();
    for node_id in graph.nodes() {
        let degree = graph.degree(node_id);
        degree_counts.entry(degree).or_insert(Vec::new()).push(node_id);
    }
    
    for (degree, nodes) in degree_counts {
        let node_names: Vec<String> = nodes.iter()
            .map(|&id| graph.get_node(id).unwrap().to_string())
            .collect();
        println!("  Degree {}: [{}]", degree, node_names.join(", "));
    }
    println!();
    
    // Check if two nodes are neighbors
    println!("4. Check if nodes are neighbors:");
    let pairs = [(a, b), (a, c), (b, c), (c, d), (d, e), (e, a)];
    for (node1, node2) in pairs {
        let name1 = graph.get_node(node1).unwrap();
        let name2 = graph.get_node(node2).unwrap();
        let are_neighbors = graph.has_edge(node1, node2);
        println!("  {} -> {}: {}", name1, name2, are_neighbors);
    }
    println!();
}

fn graph_statistics() {
    println!("5. Graph Statistics");
    println!("==================");
    
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    
    // Create a graph with various connectivity patterns
    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);
    graph.add_edge(c, d);
    graph.add_edge(d, e);
    
    println!("Graph Statistics:");
    println!();
    
    // Basic statistics
    println!("Basic Statistics:");
    println!("  Node count: {}", graph.node_count());
    println!("  Edge count: {}", graph.edge_count());
    println!("  Is empty: {}", graph.is_empty());
    println!();
    
    // Degree statistics
    println!("Degree Statistics:");
    let mut degrees = Vec::new();
    for node_id in graph.nodes() {
        let degree = graph.degree(node_id);
        degrees.push(degree);
        if let Some(data) = graph.get_node(node_id) {
            println!("  {}: degree {}", data, degree);
        }
    }
    
    if !degrees.is_empty() {
        let min_degree = degrees.iter().min().unwrap();
        let max_degree = degrees.iter().max().unwrap();
        let avg_degree = degrees.iter().sum::<usize>() as f64 / degrees.len() as f64;
        
        println!("  Minimum degree: {}", min_degree);
        println!("  Maximum degree: {}", max_degree);
        println!("  Average degree: {:.2}", avg_degree);
    }
    println!();
    
    // Graph density
    let n = graph.node_count();
    let max_edges = n * (n - 1); // Directed graph
    let density = if max_edges > 0 {
        graph.edge_count() as f64 / max_edges as f64
    } else {
        0.0
    };
    
    println!("Graph Density:");
    println!("  Actual edges: {}", graph.edge_count());
    println!("  Maximum possible edges: {}", max_edges);
    println!("  Density: {:.3}", density);
    println!("  Sparsity: {:.3}", 1.0 - density);
    println!();
    
    // Connectivity analysis
    println!("Connectivity Analysis:");
    let components = graph.connected_components();
    println!("  Connected components: {}", components.len());
    for (i, component) in components.iter().enumerate() {
        let component_names: Vec<String> = component.iter()
            .map(|&id| graph.get_node(id).unwrap().to_string())
            .collect();
        println!("    Component {}: [{}]", i + 1, component_names.join(", "));
    }
    println!("  Is connected: {}", graph.is_connected());
    println!();
}

fn edge_iteration() {
    println!("6. Edge Iteration");
    println!("================");
    
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    
    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);
    graph.add_edge(c, d);
    graph.add_edge(d, a);
    
    println!("Edge Iteration Methods:");
    println!();
    
    // Method 1: Using edges() method
    println!("Method 1: Using edges() method");
    let edges = graph.edges();
    println!("  All edges: {:?}", edges);
    for (from, to) in edges {
        let from_name = graph.get_node(from).unwrap();
        let to_name = graph.get_node(to).unwrap();
        println!("    {} -> {}", from_name, to_name);
    }
    println!();
    
    // Method 2: Iterating through nodes and neighbors
    println!("Method 2: Iterating through nodes and neighbors");
    for node_id in graph.nodes() {
        if let Some(data) = graph.get_node(node_id) {
            for &neighbor_id in graph.neighbors(node_id) {
                if let Some(neighbor_data) = graph.get_node(neighbor_id) {
                    println!("    {} -> {}", data, neighbor_data);
                }
            }
        }
    }
    println!();
    
    // Method 3: Finding specific edge patterns
    println!("Method 3: Finding specific edge patterns");
    
    // Find all edges from a specific node
    let from_node = a;
    let from_name = graph.get_node(from_node).unwrap();
    println!("  Edges from {}:", from_name);
    for &neighbor_id in graph.neighbors(from_node) {
        if let Some(neighbor_data) = graph.get_node(neighbor_id) {
            println!("    {} -> {}", from_name, neighbor_data);
        }
    }
    println!();
    
    // Find all edges to a specific node
    let to_node = d;
    let to_name = graph.get_node(to_node).unwrap();
    println!("  Edges to {}:", to_name);
    for node_id in graph.nodes() {
        if graph.has_edge(node_id, to_node) {
            if let Some(from_data) = graph.get_node(node_id) {
                println!("    {} -> {}", from_data, to_name);
            }
        }
    }
    println!();
    
    // Find bidirectional edges (for undirected graphs)
    println!("Method 4: Bidirectional edges (undirected graph)");
    let mut undirected = Graph::new_undirected();
    let x = undirected.add_node("X");
    let y = undirected.add_node("Y");
    let z = undirected.add_node("Z");
    
    undirected.add_edge(x, y);
    undirected.add_edge(y, z);
    
    println!("  Undirected graph edges:");
    for (from, to) in undirected.edges() {
        let from_name = undirected.get_node(from).unwrap();
        let to_name = undirected.get_node(to).unwrap();
        println!("    {} <-> {}", from_name, to_name);
    }
    println!();
}

// Exercise: Edge management practice
fn exercise() {
    println!("Exercise: Edge Management Practice");
    println!("=================================");
    println!("Create a graph and practice:");
    println!("  1. Add edges between all nodes");
    println!("  2. Remove some edges and observe changes");
    println!("  3. Calculate graph statistics");
    println!("  4. Find the most connected node");
    println!("  5. Check for cycles in the graph");
    println!();
    println!("Try with both directed and undirected graphs!");
    println!();
}
