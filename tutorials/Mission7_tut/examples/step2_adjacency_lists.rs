//! Step 2: Adjacency Lists
//!
//! This step dives deep into adjacency list representation, the most common and efficient
//! way to represent graphs in computer programs.
//!
//! ## Learning Objectives
//! - Understand adjacency list representation
//! - Learn how to build and manipulate adjacency lists
//! - Explore memory efficiency considerations
//! - Practice node storage and management
//!
//! ## Alignment with Mission 7
//! This step directly implements **REQ-1: Graph Structure and Node Storage** by
//! teaching the core adjacency list implementation and node management.
//!
//! ## Calendar Alignment
//! **Day 2 (Oct 9)**: Adjacency Lists & Node Storage
//! - Adjacency list representation with Vec<Vec<usize>>
//! - Node storage and management
//! - Memory efficiency considerations

use mission7::{Graph};
//use mission7_tut::tutorial_utils;

fn main() {
    println!("=== Step 2: Adjacency Lists ===\n");
    
    // 1. What are Adjacency Lists?
    what_are_adjacency_lists();
    
    // 2. Building Adjacency Lists
    building_adjacency_lists();
    
    // 3. Memory Efficiency
    memory_efficiency();
    
    // 4. Node Storage Patterns
    node_storage_patterns();
    
    // 5. Adjacency List Operations
    adjacency_list_operations();
    
    // 6. Comparison with Other Representations
    representation_comparison();
    
    println!("\n=== Step 2 Complete ===");
    println!("Next: Step 3 - Edge Management");

    // Optional Exercise
    exercise(); 
}

fn what_are_adjacency_lists() {
    println!("1. What are Adjacency Lists?");
    println!("===========================");
    
    println!("An adjacency list is a way to represent a graph where:");
    println!("  • Each node has a list of its neighbors");
    println!("  • We use an array/vector to store these lists");
    println!("  • Each list contains the IDs of connected nodes");
    println!();
    
    // Create a simple graph to demonstrate
    let mut graph = Graph::new_undirected();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    
    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);
    graph.add_edge(c, d);
    
    println!("Example Graph:");
    println!("    A");
    println!("   / \\");
    println!("  B---C");
    println!("  |   |");
    println!("  D---D");
    println!();
    
    println!("Adjacency List Representation:");
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
            println!("  Node {} (ID: {}): [{}]", data, node_id, neighbors.join(", "));
        }
    }
    println!();
}

fn building_adjacency_lists() {
    println!("2. Building Adjacency Lists");
    println!("==========================");
    
    println!("Step-by-step process:");
    println!();
    
    // Step 1: Create empty graph
    let mut graph = Graph::new_directed();
    println!("Step 1: Create empty graph");
    println!("  adjacency: []");
    println!("  nodes: []");
    println!();
    
    // Step 2: Add first node
    let a = graph.add_node("A");
    println!("Step 2: Add node A");
    println!("  adjacency: [[]]");
    println!("  nodes: [Some(\"A\")]");
    println!("  Node A gets ID: {}", a);
    println!();
    
    // Step 3: Add second node
    let b = graph.add_node("B");
    println!("Step 3: Add node B");
    println!("  adjacency: [[], []]");
    println!("  nodes: [Some(\"A\"), Some(\"B\")]");
    println!("  Node B gets ID: {}", b);
    println!();
    
    // Step 4: Add edge
    graph.add_edge(a, b);
    println!("Step 4: Add edge A -> B");
    println!("  adjacency: [[1], []]");
    println!("  nodes: [Some(\"A\"), Some(\"B\")]");
    println!("  Node A's adjacency list now contains: {:?}", graph.neighbors(a));
    println!();
    
    // Step 5: Add more nodes and edges
    let c = graph.add_node("C");
    graph.add_edge(b, c);
    graph.add_edge(c, a);
    
    println!("Step 5: Add more nodes and edges");
    println!("  adjacency: [[1], [2], [0]]");
    println!("  nodes: [Some(\"A\"), Some(\"B\"), Some(\"C\")]");
    println!();
    
    // Show final structure
    println!("Final Adjacency List:");
    for node_id in graph.nodes() {
        if let Some(data) = graph.get_node(node_id) {
            println!("  {}: {:?}", data, graph.neighbors(node_id));
        }
    }
    println!();
}

fn memory_efficiency() {
    println!("3. Memory Efficiency");
    println!("===================");
    
    println!("Adjacency List Memory Usage:");
    println!();
    
    // Create graphs of different sizes
    let sizes = [5, 10, 20, 50];
    
    for &size in &sizes {
        let mut graph = Graph::new_undirected();
        
        // Add nodes
        for i in 0..size {
            graph.add_node(format!("Node{}", i));
        }
        
        // Add some edges (sparse graph)
        for i in 0..size {
            if i + 1 < size {
                graph.add_edge(i, i + 1);
            }
        }
        
        let node_memory = size * 8; // 8 bytes per node ID
        let edge_memory = graph.edge_count() * 8; // 8 bytes per edge
        let total_memory = node_memory + edge_memory;
        
        println!("Graph with {} nodes, {} edges:", size, graph.edge_count());
        println!("  Node storage: {} bytes", node_memory);
        println!("  Edge storage: {} bytes", edge_memory);
        println!("  Total: {} bytes", total_memory);
        println!("  Memory per node: {:.1} bytes", total_memory as f64 / size as f64);
        println!();
    }
    
    println!("Memory Efficiency Benefits:");
    println!("  • Only stores actual edges (not empty connections)");
    println!("  • Space complexity: O(V + E) where V=vertices, E=edges");
    println!("  • Efficient for sparse graphs (few edges relative to nodes)");
    println!("  • Easy to iterate over neighbors");
    println!();
}

fn node_storage_patterns() {
    println!("4. Node Storage Patterns");
    println!("=======================");
    
    println!("Different ways to store node data:");
    println!();
    
    // Pattern 1: Simple data
    println!("Pattern 1: Simple Data (String)");
    let mut graph1 = Graph::new_directed();
    let a = graph1.add_node("Alice");
    let b = graph1.add_node("Bob");
    println!("  Node {}: {}", a, graph1.get_node(a).unwrap());
    println!("  Node {}: {}", b, graph1.get_node(b).unwrap());
    println!();
    
    // Pattern 2: Numeric data
    println!("Pattern 2: Numeric Data (i32)");
    let mut graph2 = Graph::new_directed();
    let node1 = graph2.add_node(42);
    let node2 = graph2.add_node(100);
    println!("  Node {}: {}", node1, graph2.get_node(node1).unwrap());
    println!("  Node {}: {}", node2, graph2.get_node(node2).unwrap());
    println!();
    
    // Pattern 3: Complex data
    println!("Pattern 3: Complex Data (Struct)");
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    struct Person {
        name: String,
        age: u32,
    }
    
    let mut graph3 = Graph::new_directed();
    let alice = graph3.add_node(Person { name: "Alice".to_string(), age: 25 });
    let bob = graph3.add_node(Person { name: "Bob".to_string(), age: 30 });
    
    println!("  Node {}: {:?}", alice, graph3.get_node(alice).unwrap());
    println!("  Node {}: {:?}", bob, graph3.get_node(bob).unwrap());
    println!();
    
    // Pattern 4: Optional data
    println!("Pattern 4: Optional Data (Option<T>)");
    let mut graph4 = Graph::new_directed();
    let node1 = graph4.add_node(Some("Data"));
    let node2 = graph4.add_node(None::<&str>);
    
    println!("  Node {}: {:?}", node1, graph4.get_node(node1).unwrap());
    println!("  Node {}: {:?}", node2, graph4.get_node(node2).unwrap());
    println!();
    
    println!("Node ID Management:");
    println!("  • Node IDs are assigned sequentially (0, 1, 2, ...)");
    println!("  • IDs are stable (don't change when edges are added/removed)");
    println!("  • IDs can be used as array indices for efficient lookup");
    println!("  • Node data can be any type that implements Clone");
    println!();
}

fn adjacency_list_operations() {
    println!("5. Adjacency List Operations");
    println!("===========================");
    
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    
    // Add edges
    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);
    graph.add_edge(c, d);
    
    println!("Graph Operations:");
    println!();
    
    // Check if edge exists
    println!("Edge Existence Check:");
    println!("  A -> B exists: {}", graph.has_edge(a, b));
    println!("  B -> A exists: {}", graph.has_edge(b, a));
    println!("  A -> D exists: {}", graph.has_edge(a, d));
    println!();
    
    // Get neighbors
    println!("Neighbor Lookup:");
    println!("  A's neighbors: {:?}", graph.neighbors(a));
    println!("  B's neighbors: {:?}", graph.neighbors(b));
    println!("  C's neighbors: {:?}", graph.neighbors(c));
    println!("  D's neighbors: {:?}", graph.neighbors(d));
    println!();
    
    // Get degree
    println!("Degree Calculation:");
    println!("  A's degree: {}", graph.degree(a));
    println!("  B's degree: {}", graph.degree(b));
    println!("  C's degree: {}", graph.degree(c));
    println!("  D's degree: {}", graph.degree(d));
    println!();
    
    // Remove edge
    println!("Edge Removal:");
    println!("  Before: A's neighbors: {:?}", graph.neighbors(a));
    graph.remove_edge(a, c);
    println!("  After removing A -> C: A's neighbors: {:?}", graph.neighbors(a));
    println!("  Edge count: {}", graph.edge_count());
    println!();
    
    // Add edge back
    graph.add_edge(a, c);
    println!("  After adding A -> C back: A's neighbors: {:?}", graph.neighbors(a));
    println!("  Edge count: {}", graph.edge_count());
    println!();
}

fn representation_comparison() {
    println!("6. Comparison with Other Representations");
    println!("=======================================");
    
    println!("Adjacency List vs Adjacency Matrix:");
    println!();
    
    let n = 5; // Number of nodes
    let edges = 6; // Number of edges
    
    // Adjacency List
    let list_memory = n * 8 + edges * 8; // 8 bytes per node + 8 bytes per edge
    
    // Adjacency Matrix
    let matrix_memory = n * n * 8; // n×n matrix, 8 bytes per entry
    
    println!("Memory Usage for {} nodes, {} edges:", n, edges);
    println!("  Adjacency List: {} bytes", list_memory);
    println!("  Adjacency Matrix: {} bytes", matrix_memory);
    println!("  List is {:.1}x more memory efficient", matrix_memory as f64 / list_memory as f64);
    println!();
    
    println!("Operation Complexity:");
    println!("  Operation          | Adjacency List | Adjacency Matrix");
    println!("  -------------------|----------------|-----------------");
    println!("  Add Edge           | O(1)           | O(1)");
    println!("  Remove Edge        | O(degree)      | O(1)");
    println!("  Check Edge         | O(degree)      | O(1)");
    println!("  Get Neighbors      | O(degree)      | O(V)");
    println!("  Memory Usage       | O(V + E)       | O(V²)");
    println!();
    
    println!("When to Use Adjacency List:");
    println!("  ✅ Sparse graphs (few edges relative to nodes)");
    println!("  ✅ Need to iterate over neighbors frequently");
    println!("  ✅ Memory is a concern");
    println!("  ✅ Graph size is large");
    println!();
    
    println!("When to Use Adjacency Matrix:");
    println!("  ✅ Dense graphs (many edges relative to nodes)");
    println!("  ✅ Need fast edge existence checks");
    println!("  ✅ Graph size is small");
    println!("  ✅ Memory is not a concern");
    println!();
}

// Exercise: Build your own adjacency list
fn exercise() {
    println!("Exercise: Build Your Own Adjacency List");
    println!("======================================");
    println!("Try building adjacency lists for:");
    println!("  1. A complete graph with 4 nodes");
    println!("  2. A path graph with 5 nodes");
    println!("  3. A star graph (one central node connected to all others)");
    println!();
    println!("For each graph:");
    println!("  • Calculate the memory usage");
    println!("  • Count the total number of edges");
    println!("  • Find the node with the highest degree");
    println!("  • Check if it's more efficient than an adjacency matrix");
    println!();
}
