//! # Mission 7 Tutorial: Graph Representation and Algorithms
//!
//! This tutorial provides a step-by-step learning progression for understanding
//! graph data structures, adjacency lists, and fundamental graph algorithms.
//!
//! ## Learning Objectives
//!
//! By completing this tutorial, you will:
//! - Understand graph data structures and their representations
//! - Master adjacency list implementation and operations
//! - Learn edge management and graph validation techniques
//! - Implement depth-first search (DFS) algorithms
//! - Implement breadth-first search (BFS) algorithms
//! - Apply graph algorithms to real-world problems
//!
//! ## Prerequisites
//!
//! - Basic understanding of Rust ownership and borrowing
//! - Familiarity with vectors and collections
//! - Knowledge of basic data structures (from previous missions)
//! - Understanding of recursion and iteration
//!
//! ## Tutorial Structure
//!
//! ### Step 1: Graph Fundamentals
//! - What are graphs and why they matter
//! - Directed vs undirected graphs
//! - Basic graph terminology
//! - Simple graph representation
//!
//! ### Step 2: Adjacency Lists
//! - Adjacency list representation
//! - Node storage and management
//! - Memory efficiency considerations
//! - Building adjacency lists
//!
//! ### Step 3: Edge Management
//! - Adding and removing edges
//! - Edge validation and error handling
//! - Neighbor lookup and enumeration
//! - Graph statistics and analysis
//!
//! ### Step 4: Algorithm Foundation
//! - Visited tracking for algorithms
//! - Path reconstruction techniques
//! - Algorithm result structures
//! - Foundation for DFS and BFS
//!
//! ### Step 5: DFS Implementation
//! - Recursive depth-first search
//! - Iterative DFS with explicit stack
//! - Path finding and cycle detection
//! - Component analysis
//!
//! ### Step 6: BFS Implementation
//! - Breadth-first search with queue
//! - Shortest path algorithms
//! - Level-order traversal
//! - Distance calculation
//!
//! ### Step 7: Integration Project
//! - Real-world graph applications
//! - Social network analysis
//! - Maze solving algorithms
//! - Performance optimization
//!
//! ## How to Use This Tutorial
//!
//! 1. **Read the step overview** in each example file
//! 2. **Run the code** to see the concepts in action
//! 3. **Modify the examples** to experiment with different scenarios
//! 4. **Complete the exercises** at the end of each step
//! 5. **Build the integration project** to apply all concepts
//!
//! ## Running the Examples
//!
//! ```bash
//! # Run individual steps
//! cargo run --example step1_graph_fundamentals
//! cargo run --example step2_adjacency_lists
//! cargo run --example step3_edge_management
//! cargo run --example step4_algorithm_foundation
//! cargo run --example step5_dfs_implementation
//! cargo run --example step6_bfs_implementation
//! cargo run --example step7_integration_project
//!
//! # Run all steps in sequence
//! cargo run --example step1_graph_fundamentals && \
//! cargo run --example step2_adjacency_lists && \
//! cargo run --example step3_edge_management && \
//! cargo run --example step4_algorithm_foundation && \
//! cargo run --example step5_dfs_implementation && \
//! cargo run --example step6_bfs_implementation && \
//! cargo run --example step7_integration_project
//! ```
//!
//! ## Alignment with Mission 7
//!
//! This tutorial directly supports Mission 7 requirements:
//! - **REQ-1**: Graph structure and node storage (Steps 1-2)
//! - **REQ-2**: Edge management operations (Step 3)
//! - **REQ-3**: Graph building and validation (Steps 3-4)
//! - **REQ-4**: Algorithm foundation (Step 4)
//! - **REQ-5**: DFS implementation (Step 5)
//! - **REQ-6**: BFS implementation (Step 6)
//!
//! Each step builds toward the complete Mission 7 implementation,
//! providing hands-on experience with all concepts before tackling
//! the full mission requirements.

/// Utility functions and helpers for tutorial examples
pub mod tutorial_utils {
    use mission7::{Graph, NodeId};

    /// Create a simple example graph for tutorial purposes
    pub fn create_example_graph() -> Graph<&'static str> {
        let mut graph = Graph::new_directed();

        let a = graph.add_node("A");
        let b = graph.add_node("B");
        let c = graph.add_node("C");
        let d = graph.add_node("D");

        graph.add_edge(a, b);
        graph.add_edge(b, c);
        graph.add_edge(c, d);
        graph.add_edge(d, a);

        graph
    }

    /// Create a tree-like graph for traversal examples
    pub fn create_tree_graph() -> Graph<&'static str> {
        let mut graph = Graph::new_directed();

        let root = graph.add_node("Root");
        let left = graph.add_node("Left");
        let right = graph.add_node("Right");
        let left_left = graph.add_node("Left-Left");
        let left_right = graph.add_node("Left-Right");
        let right_left = graph.add_node("Right-Left");

        graph.add_edge(root, left);
        graph.add_edge(root, right);
        graph.add_edge(left, left_left);
        graph.add_edge(left, left_right);
        graph.add_edge(right, right_left);

        graph
    }

    /// Print graph structure in a readable format
    pub fn print_graph_structure<T>(graph: &Graph<T>)
    where
        T: std::fmt::Display,
    {
        println!("Graph Structure:");
        println!("  Nodes: {}", graph.node_count());
        println!("  Edges: {}", graph.edge_count());

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

                println!("  {} -> [{}]", data, neighbors.join(", "));
            }
        }
    }

    /// Print traversal results in a readable format
    pub fn print_traversal_result<T>(graph: &Graph<T>, path: &[NodeId], algorithm: &str)
    where
        T: std::fmt::Display,
    {
        println!("{} Traversal:", algorithm);
        let path_str: Vec<String> = path
            .iter()
            .map(|&id| {
                graph
                    .get_node(id)
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| format!("Node{}", id))
            })
            .collect();
        println!("  Path: {}", path_str.join(" -> "));
        println!("  Length: {}", path.len());
    }
}
