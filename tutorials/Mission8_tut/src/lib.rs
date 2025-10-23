//! # Mission 8 Tutorial: BFS/DFS Algorithms - Educational Support Library
//!
//! This tutorial library provides shared utilities, examples, and helper functions
//! for learning breadth-first search (BFS) and depth-first search (DFS) algorithms
//! with a focus on pedagogical clarity and progressive disclosure.
//!
//! ## Tutorial Philosophy
//!
//! This tutorial follows evidence-based learning principles:
//! - **Progressive Disclosure**: Start simple, add complexity gradually
//! - **Hands-On Learning**: Every concept has runnable examples
//! - **Error Anticipation**: Show common mistakes and correct patterns
//! - **Multiple Learning Styles**: Visual, textual, and kinesthetic approaches
//! - **Self-Assessment**: Check understanding with exercises
//!
//! ## Tutorial Progression
//!
//! - **Step 1**: Algorithm Trait Design ✅
//! - **Step 2**: Generic BFS & DFS Implementation (this step)
//! - **Step 3**: Algorithm Composition
//! - **Step 4**: Performance Benchmarking
//! - **Step 5**: Maze Solver Application
//! - **Step 6**: Integration Testing
//! - **Step 7**: Final Review & Documentation
//!
//! ## Quick Start
//!
//! ```rust
//! use mission8_tut::{create_sample_graph, print_traversal};
//!
//! // Create a simple graph for learning
//! let graph = create_sample_graph();
//!
//! // Use tutorial utilities
//! print_traversal("BFS Example", &[0, 1, 2, 3]);
//! ```

use std::collections::HashMap;
use std::fmt::Debug;

/// Creates a standard 4-node graph used throughout the tutorial.
///
/// This graph structure is used consistently across examples to help
/// students focus on algorithm differences rather than graph complexity.
///
/// Graph structure:
/// ```text
///     0
///    / \
///   1   2
///    \ /
///     3
/// ```
///
/// # Examples
///
/// ```rust
/// use mission8_tut::create_sample_graph;
///
/// let graph = create_sample_graph();
/// assert_eq!(graph[&0], vec![1, 2]);
/// assert_eq!(graph[&1], vec![3]);
/// assert_eq!(graph[&2], vec![3]);
/// assert_eq!(graph[&3], vec![]);
/// ```
pub fn create_sample_graph() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![3]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![]);
    graph
}

/// Creates a larger example graph for demonstrating algorithm differences.
///
/// This graph has multiple levels and branches to clearly show the
/// difference between BFS (level-order) and DFS (depth-first) traversal.
///
/// Graph structure:
/// ```text
///       0
///      / \
///     1   2
///    /|   |\
///   3 4   5 6
/// ```
///
/// # Examples
///
/// ```rust
/// use mission8_tut::create_tree_graph;
///
/// let graph = create_tree_graph();
/// assert_eq!(graph[&0], vec![1, 2]);
/// assert_eq!(graph[&1], vec![3, 4]);
/// assert_eq!(graph[&2], vec![5, 6]);
/// ```
pub fn create_tree_graph() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![3, 4]);
    graph.insert(2, vec![5, 6]);
    graph.insert(3, vec![]);
    graph.insert(4, vec![]);
    graph.insert(5, vec![]);
    graph.insert(6, vec![]);
    graph
}

/// Creates a linear graph for testing sequential algorithms.
///
/// Linear graphs are useful for understanding algorithm behavior
/// on simple, predictable structures.
///
/// Graph structure:
/// ```text
/// 0 -> 1 -> 2 -> 3
/// ```
///
/// # Examples
///
/// ```rust
/// use mission8_tut::create_linear_graph;
///
/// let graph = create_linear_graph();
/// assert_eq!(graph[&0], vec![1]);
/// assert_eq!(graph[&1], vec![2]);
/// assert_eq!(graph[&2], vec![3]);
/// assert_eq!(graph[&3], vec![]);
/// ```
pub fn create_linear_graph() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![]);
    graph
}

/// Creates a graph with cycles for advanced examples.
///
/// Cyclic graphs are important for testing that algorithms
/// handle visited tracking correctly and don't get stuck in loops.
///
/// Graph structure:
/// ```text
/// 0 -> 1 -> 2
/// ^         |
/// +---------+
/// ```
///
/// # Examples
///
/// ```rust
/// use mission8_tut::create_cyclic_graph;
///
/// let graph = create_cyclic_graph();
/// assert_eq!(graph[&0], vec![1]);
/// assert_eq!(graph[&1], vec![2]);
/// assert_eq!(graph[&2], vec![0]);  // Creates cycle
/// ```
pub fn create_cyclic_graph() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![0]); // Creates cycle back to 0
    graph
}

/// Prints a formatted traversal result for educational clarity.
///
/// This function provides consistent, clear output formatting
/// across all tutorial examples, making it easier for students
/// to understand algorithm results.
///
/// # Arguments
///
/// * `algorithm_name` - Descriptive name for the traversal (e.g., "BFS", "DFS")
/// * `visited_nodes` - Nodes in the order they were visited
///
/// # Examples
///
/// ```rust
/// use mission8_tut::print_traversal;
///
/// print_traversal("BFS Level-Order", &[0, 1, 2, 3]);
/// // Output: "BFS Level-Order: [0, 1, 2, 3]"
/// ```
pub fn print_traversal<T: Debug>(algorithm_name: &str, visited_nodes: &[T]) {
    println!("  {}: {:?}", algorithm_name, visited_nodes);
}

/// Visualizes the current state of a queue during BFS.
///
/// This educational helper shows students what's happening
/// inside the BFS algorithm at each step.
///
/// # Arguments
///
/// * `step` - Current step number in the algorithm
/// * `current_node` - Node being processed
/// * `queue` - Current state of the BFS queue
/// * `visited` - Set of already visited nodes
///
/// # Examples
///
/// ```rust
/// use mission8_tut::visualize_bfs_state;
/// use std::collections::{VecDeque, HashSet};
///
/// let mut queue = VecDeque::new();
/// queue.push_back(1);
/// queue.push_back(2);
///
/// let mut visited = HashSet::new();
/// visited.insert(0);
///
/// visualize_bfs_state(1, 0, &queue, &visited);
/// ```
pub fn visualize_bfs_state<T>(
    step: usize,
    current_node: T,
    queue: &std::collections::VecDeque<T>,
    visited: &std::collections::HashSet<T>,
) where
    T: Debug + Clone + std::hash::Hash + Eq,
{
    println!("  Step {}: Processing node {:?}", step, current_node);
    println!("    Queue: {:?}", queue);
    println!("    Visited: {:?}", visited);
}

/// Visualizes the current state of a stack during DFS.
///
/// This educational helper shows students what's happening
/// inside the DFS algorithm at each step.
///
/// # Arguments
///
/// * `step` - Current step number in the algorithm
/// * `current_node` - Node being processed
/// * `stack` - Current state of the DFS stack
/// * `visited` - Set of already visited nodes
///
/// # Examples
///
/// ```rust
/// use mission8_tut::visualize_dfs_state;
/// use std::collections::HashSet;
///
/// let stack = vec![1, 2];
///
/// let mut visited = HashSet::new();
/// visited.insert(0);
///
/// visualize_dfs_state(1, 0, &stack, &visited);
/// ```
pub fn visualize_dfs_state<T>(
    step: usize,
    current_node: T,
    stack: &[T],
    visited: &std::collections::HashSet<T>,
) where
    T: Debug + Clone + std::hash::Hash + Eq,
{
    println!("  Step {}: Processing node {:?}", step, current_node);
    println!("    Stack: {:?}", stack);
    println!("    Visited: {:?}", visited);
}

/// Demonstrates the difference between BFS and DFS on the same graph.
///
/// This is a key educational function that helps students understand
/// why different algorithms produce different traversal orders.
///
/// # Arguments
///
/// * `graph_name` - Description of the graph being used
/// * `bfs_result` - Result from BFS traversal
/// * `dfs_result` - Result from DFS traversal
///
/// # Examples
///
/// ```rust
/// use mission8_tut::compare_algorithms;
///
/// let bfs_order = vec![0, 1, 2, 3];
/// let dfs_order = vec![0, 1, 3, 2];
///
/// compare_algorithms("Simple Graph", &bfs_order, &dfs_order);
/// ```
pub fn compare_algorithms<T: Debug + PartialEq>(
    graph_name: &str,
    bfs_result: &[T],
    dfs_result: &[T],
) {
    println!("\n📊 Algorithm Comparison on {}:", graph_name);
    print_traversal("BFS (Level-Order)", bfs_result);
    print_traversal("DFS (Depth-First)", dfs_result);

    if bfs_result == dfs_result {
        println!("  ℹ️  Note: Both algorithms produce the same order for this graph");
    } else {
        println!("  ℹ️  Note: Different traversal orders show algorithm characteristics");
    }
}

/// Explains why BFS guarantees shortest path in unweighted graphs.
///
/// This educational function helps students understand the theoretical
/// foundation behind BFS being optimal for shortest path problems.
///
/// # Examples
///
/// ```rust
/// use mission8_tut::explain_bfs_shortest_path;
///
/// explain_bfs_shortest_path();
/// ```
pub fn explain_bfs_shortest_path() {
    println!("🎯 Why BFS Finds Shortest Paths:");
    println!("  1. BFS visits nodes level by level");
    println!("  2. Level k contains all nodes at distance k from start");
    println!("  3. When a node is first visited, we've found the shortest path");
    println!("  4. This only works for unweighted graphs (all edges cost 1)");
}

/// Explains why DFS is better for some problems like cycle detection.
///
/// This educational function helps students understand when to choose
/// DFS over BFS based on the problem requirements.
///
/// # Examples
///
/// ```rust
/// use mission8_tut::explain_dfs_advantages;
///
/// explain_dfs_advantages();
/// ```
pub fn explain_dfs_advantages() {
    println!("🎯 When DFS is Better Than BFS:");
    println!("  1. Cycle Detection: DFS can track 'back edges'");
    println!("  2. Topological Sort: DFS finishing times are key");
    println!("  3. Memory Usage: DFS stack is often smaller than BFS queue");
    println!("  4. Path Enumeration: DFS naturally tracks current path");
}

/// Common mistakes students make when implementing BFS/DFS.
///
/// This function proactively addresses typical errors to prevent
/// frustration and improve learning outcomes.
///
/// # Examples
///
/// ```rust
/// use mission8_tut::show_common_mistakes;
///
/// show_common_mistakes();
/// ```
pub fn show_common_mistakes() {
    println!("⚠️  Common BFS/DFS Implementation Mistakes:");
    println!("  ❌ Not marking nodes visited when adding to queue/stack");
    println!("  ❌ Using recursion for DFS (can cause stack overflow)");
    println!("  ❌ Forgetting to handle disconnected graph components");
    println!("  ❌ Not checking if start node exists in graph");
    println!("  ❌ Modifying graph structure during traversal");
    println!();
    println!("  ✅ Correct Pattern: Mark visited when enqueuing/pushing");
    println!("  ✅ Use explicit stack/queue data structures");
    println!("  ✅ Handle edge cases (empty graph, single node)");
}

/// Explains how BFS finds shortest paths using parent tracking.
///
/// This educational function clarifies the parent tracking technique
/// that enables path reconstruction in BFS.
pub fn explain_shortest_path_technique() {
    println!("🎯 Parent Tracking for Path Reconstruction:");
    println!("  1. As we visit each node, record where it came from");
    println!("  2. When we find the destination, we have the path!");
    println!("  3. Walk backwards from destination to source using parent map");
    println!("  4. Reverse to get source-to-destination path");
    println!();
    println!("  Example: 0 -> 1 -> 3");
    println!("           parent[1] = 0");
    println!("           parent[3] = 1");
    println!("           Walk: 3 <- 1 <- 0, reverse -> [0, 1, 3]");
}

/// Explains cycle detection using DFS state tracking.
///
/// This educational function clarifies the three-color approach for
/// detecting cycles in directed graphs.
pub fn explain_cycle_detection() {
    println!("🎯 Three-Color Cycle Detection (DFS):");
    println!("  White = Unvisited");
    println!("  Gray = Currently processing (in recursion stack)");
    println!("  Black = Finished processing (all children explored)");
    println!();
    println!("  Back Edge = Edge to gray node = CYCLE!");
    println!("  Why? Gray node is ancestor on current path");
    println!("  Edge back to it completes a cycle");
    println!();
    println!("  Example: 0 -> 1 -> 2");
    println!("           When at 2, edge 2->0 is back edge (0 is gray)");
    println!("           Cycle found: 0 -> 1 -> 2 -> 0");
}

/// Explains connected components conceptually.
///
/// This educational function helps students understand what connected
/// components are and why they matter.
pub fn explain_connected_components() {
    println!("🎯 Connected Components:");
    println!("  = Maximal groups of nodes that can reach each other");
    println!("  = Isolated \"islands\" in the graph");
    println!("  = Finding components = BFS/DFS from each unvisited node");
    println!();
    println!("  Example: 3 components");
    println!("  Component 1: {{0, 1, 2}} <- all reachable");
    println!("  Component 2: {{3, 4}}   <- isolated pair");
    println!("  Component 3: {{5}}      <- lone node");
    println!();
    println!("  Application: Network analysis, social graphs, etc.");
}

/// Shows common composition patterns.
///
/// This educational function demonstrates how to combine algorithms
/// to solve higher-level problems.
pub fn explain_algorithm_composition() {
    println!("🎯 Composing Algorithms:");
    println!("  Shortest Path = BFS + parent tracking");
    println!("  Cycle Detection = DFS + state tracking");
    println!("  Components = BFS/DFS from multiple starts");
    println!("  Topological Sort = DFS + finishing times");
    println!();
    println!("  Key Insight: Basic algorithms are building blocks");
    println!("  Combine them to solve complex problems");
}

/// Compares algorithm approaches for common problems.
pub fn compare_approaches() {
    println!("📊 Algorithm Comparison Matrix:");
    println!();
    println!("  Problem              | BFS         | DFS         | Notes");
    println!("  -------------------- | ----------- | ----------- | --------");
    println!("  Shortest Path        | ✅ Best     | ⚠️  Suboptimal| BFS guarantees");
    println!("  Cycle Detection      | ❌ Complex  | ✅ Best     | DFS detects back edges");
    println!("  Connectivity         | ✅ Works    | ✅ Works    | Either works");
    println!("  Components          | ✅ Works    | ✅ Works    | Either works");
    println!("  Topological Sort     | ❌ N/A     | ✅ Best     | DFS finishing times");
}
