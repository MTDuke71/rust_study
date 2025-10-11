//! Step 1: Graph Fundamentals
//!
//! This step introduces the basic concepts of graphs, their types, and why they're important
//! in computer science and real-world applications.
//!
//! ## Learning Objectives
//! - Understand what graphs are and their components
//! - Learn the difference between directed and undirected graphs
//! - Explore basic graph terminology
//! - See simple graph representations
//!
//! ## Alignment with Mission 7
//! This step supports **REQ-1: Graph Structure and Node Storage** by introducing
//! the fundamental concepts needed for graph implementation.
//!
//! ## Calendar Alignment
//! **Day 1 (Oct 8)**: Mission 7 Setup (Graph Representation)
//! - Understanding graph fundamentals
//! - Basic graph terminology and concepts
//! - Introduction to graph representations

use mission7::{Graph, GraphType};
use mission7_tut::tutorial_utils;

fn main() {
    println!("=== Step 1: Graph Fundamentals ===\n");

    // 1. What are Graphs?
    what_are_graphs();

    // 2. Graph Components
    graph_components();

    // 3. Directed vs Undirected Graphs
    directed_vs_undirected();

    // 4. Graph Terminology
    graph_terminology();

    // 5. Simple Graph Representation
    simple_representation();

    // 6. Real-world Examples
    real_world_examples();

    println!("\n=== Step 1 Complete ===");
    println!("Next: Step 2 - Adjacency Lists");
}

fn what_are_graphs() {
    println!("1. What are Graphs?");
    println!("==================");

    println!("A graph is a collection of:");
    println!("  • Nodes (vertices) - represent entities");
    println!("  • Edges (connections) - represent relationships");
    println!();

    println!("Graphs are used to model:");
    println!("  • Social networks (people and friendships)");
    println!("  • Computer networks (routers and connections)");
    println!("  • Maps (cities and roads)");
    println!("  • Dependencies (tasks and prerequisites)");
    println!("  • Web pages (pages and links)");
    println!();

    // Simple visual representation
    println!("Example Graph:");
    println!("    A");
    println!("   / \\");
    println!("  B---C");
    println!("  |   |");
    println!("  D---E");
    println!();
}

fn graph_components() {
    println!("2. Graph Components");
    println!("==================");

    // Create a simple graph to demonstrate components
    let mut graph = Graph::new_undirected();

    // Add nodes (vertices)
    let alice = graph.add_node("Alice");
    let bob = graph.add_node("Bob");
    let charlie = graph.add_node("Charlie");
    let diana = graph.add_node("Diana");

    println!("Nodes (Vertices):");
    println!("  Alice (ID: {})", alice);
    println!("  Bob (ID: {})", bob);
    println!("  Charlie (ID: {})", charlie);
    println!("  Diana (ID: {})", diana);
    println!();

    // Add edges (connections)
    graph.add_edge(alice, bob);
    graph.add_edge(bob, charlie);
    graph.add_edge(charlie, diana);
    graph.add_edge(diana, alice);

    println!("Edges (Connections):");
    println!("  Alice <-> Bob");
    println!("  Bob <-> Charlie");
    println!("  Charlie <-> Diana");
    println!("  Diana <-> Alice");
    println!();

    println!("Graph Statistics:");
    println!("  Total nodes: {}", graph.node_count());
    println!("  Total edges: {}", graph.edge_count());
    println!();
}

fn directed_vs_undirected() {
    println!("3. Directed vs Undirected Graphs");
    println!("================================");

    // Undirected graph example
    println!("Undirected Graph (bidirectional relationships):");
    let mut undirected = Graph::new_undirected();
    let a = undirected.add_node("A");
    let b = undirected.add_node("B");

    undirected.add_edge(a, b);

    println!("  A <-> B (friendship, road connection)");
    println!("  Edge A->B exists: {}", undirected.has_edge(a, b));
    println!("  Edge B->A exists: {}", undirected.has_edge(b, a));
    println!();

    // Directed graph example
    println!("Directed Graph (one-way relationships):");
    let mut directed = Graph::new_directed();
    let a = directed.add_node("A");
    let b = directed.add_node("B");

    directed.add_edge(a, b);

    println!("  A -> B (follows, depends on, points to)");
    println!("  Edge A->B exists: {}", directed.has_edge(a, b));
    println!("  Edge B->A exists: {}", directed.has_edge(b, a));
    println!();

    // Real-world examples
    println!("Real-world Examples:");
    println!("  Undirected: Facebook friendships, road networks");
    println!("  Directed: Twitter follows, web page links, task dependencies");
    println!();
}

fn graph_terminology() {
    println!("4. Graph Terminology");
    println!("===================");

    let mut graph = Graph::new_undirected();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(c, d);

    println!("Key Terms:");
    println!();

    // Degree
    println!("Degree - number of connections:");
    println!("  Node A degree: {}", graph.degree(a));
    println!("  Node B degree: {}", graph.degree(b));
    println!("  Node C degree: {}", graph.degree(c));
    println!("  Node D degree: {}", graph.degree(d));
    println!();

    // Neighbors
    println!("Neighbors - connected nodes:");
    println!("  A's neighbors: {:?}", graph.neighbors(a));
    println!("  B's neighbors: {:?}", graph.neighbors(b));
    println!("  C's neighbors: {:?}", graph.neighbors(c));
    println!("  D's neighbors: {:?}", graph.neighbors(d));
    println!();

    // Path
    println!("Path - sequence of connected nodes:");
    println!("  Path from A to D: A -> B -> C -> D");
    println!("  Path length: 3 edges");
    println!();

    // Cycle
    println!("Cycle - path that starts and ends at the same node:");
    graph.add_edge(d, a); // Create a cycle
    println!("  Added edge D -> A to create cycle: A -> B -> C -> D -> A");
    println!();
}

fn simple_representation() {
    println!("5. Simple Graph Representation");
    println!("=============================");

    // Create a small graph
    let mut graph = Graph::new_undirected();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(c, a);

    println!("Graph Structure:");
    println!("  Nodes: A, B, C");
    println!("  Edges: A-B, B-C, C-A");
    println!();

    // Show adjacency list representation
    println!("Adjacency List Representation:");
    for node_id in graph.nodes() {
        if let Some(data) = graph.get_node(node_id) {
            let neighbors: Vec<String> = graph
                .neighbors(node_id)
                .iter()
                .map(|&id| {
                    graph
                        .get_node(id)
                        .map(|d| d.to_string())
                        .unwrap_or_else(|| format!("Node{}", id))
                })
                .collect();
            println!("  {}: [{}]", data, neighbors.join(", "));
        }
    }
    println!();

    // Show edge list representation
    println!("Edge List Representation:");
    for (from, to) in graph.edges() {
        let from_data = graph.get_node(from).unwrap();
        let to_data = graph.get_node(to).unwrap();
        println!("  {} -> {}", from_data, to_data);
    }
    println!();
}

fn real_world_examples() {
    println!("6. Real-world Examples");
    println!("=====================");

    // Social network example
    println!("Social Network (Undirected Graph):");
    let mut social = Graph::new_undirected();
    let alice = social.add_node("Alice");
    let bob = social.add_node("Bob");
    let charlie = social.add_node("Charlie");
    let diana = social.add_node("Diana");

    social.add_edge(alice, bob);
    social.add_edge(bob, charlie);
    social.add_edge(charlie, diana);
    social.add_edge(diana, alice);

    println!("  People: Alice, Bob, Charlie, Diana");
    println!("  Friendships: Alice-Bob, Bob-Charlie, Charlie-Diana, Diana-Alice");
    println!(
        "  Network density: {:.2}",
        social.edge_count() as f64 / (social.node_count() * (social.node_count() - 1) / 2) as f64
    );
    println!();

    // Web page links example
    println!("Web Page Links (Directed Graph):");
    let mut web = Graph::new_directed();
    let home = web.add_node("Home");
    let about = web.add_node("About");
    let contact = web.add_node("Contact");
    let products = web.add_node("Products");

    web.add_edge(home, about);
    web.add_edge(home, contact);
    web.add_edge(home, products);
    web.add_edge(about, contact);
    web.add_edge(products, home);

    println!("  Pages: Home, About, Contact, Products");
    println!("  Links: Home->About, Home->Contact, Home->Products, About->Contact, Products->Home");
    println!();

    // Task dependencies example
    println!("Task Dependencies (Directed Acyclic Graph):");
    let mut tasks = Graph::new_directed();
    let design = tasks.add_node("Design");
    let code = tasks.add_node("Code");
    let test = tasks.add_node("Test");
    let deploy = tasks.add_node("Deploy");

    tasks.add_edge(design, code);
    tasks.add_edge(code, test);
    tasks.add_edge(test, deploy);

    println!("  Tasks: Design, Code, Test, Deploy");
    println!("  Dependencies: Design->Code, Code->Test, Test->Deploy");
    println!("  Critical path: Design -> Code -> Test -> Deploy");
    println!();
}

// Exercise: Create your own graph
fn exercise() {
    println!("Exercise: Create Your Own Graph");
    println!("==============================");
    println!("Try creating a graph that represents:");
    println!("  1. Your family tree (directed)");
    println!("  2. Your friend group (undirected)");
    println!("  3. A simple map with cities and roads (undirected)");
    println!();
    println!("For each graph, think about:");
    println!("  • What do the nodes represent?");
    println!("  • What do the edges represent?");
    println!("  • Is it directed or undirected?");
    println!("  • How many nodes and edges does it have?");
    println!();
}
