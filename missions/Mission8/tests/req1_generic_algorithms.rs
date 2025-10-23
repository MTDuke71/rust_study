//! Tests for REQ-1: Generic BFS/DFS Algorithm Implementation
//!
//! This test suite verifies that the BFS and DFS algorithms work correctly
//! on different graph types and handle various edge cases properly.

use mission8::{bfs, dfs, Graph};
use std::collections::HashMap;

/// Create a simple 4-node test graph for algorithm validation
///
/// Graph structure:
///     0
///    / \
///   1   2
///    \ /
///     3
fn create_simple_graph() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![3]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![]);
    graph
}

/// Create a disconnected graph for testing unreachable nodes
///
/// Graph structure:
///   0 --- 1    2 --- 3
fn create_disconnected_graph() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![0]); // Make it bidirectional
    graph.insert(2, vec![3]);
    graph.insert(3, vec![2]); // Make it bidirectional
    graph
}

/// Create a linear graph for testing sequential traversal
///
/// Graph structure: 0 -> 1 -> 2 -> 3
fn create_linear_graph() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![]);
    graph
}

#[test]
fn req1_test_bfs_simple_graph() {
    let graph = create_simple_graph();
    let result = bfs(&graph, 0);

    // BFS should visit nodes in level-order: 0, then {1,2}, then {3}
    assert_eq!(result[0], 0); // Start node first
    assert_eq!(result.len(), 4); // All nodes visited

    // Node 0 should be first
    assert_eq!(result[0], 0);

    // Nodes 1 and 2 should come before node 3 (level-order)
    let pos_1 = result.iter().position(|&x| x == 1).unwrap();
    let pos_2 = result.iter().position(|&x| x == 2).unwrap();
    let pos_3 = result.iter().position(|&x| x == 3).unwrap();

    assert!(pos_1 < pos_3, "Node 1 should be visited before node 3");
    assert!(pos_2 < pos_3, "Node 2 should be visited before node 3");
}

#[test]
fn req1_test_bfs_disconnected_graph() {
    let graph = create_disconnected_graph();
    let result = bfs(&graph, 0);

    // BFS from node 0 should only reach nodes 0 and 1
    assert_eq!(result.len(), 2);
    assert!(result.contains(&0));
    assert!(result.contains(&1));
    assert!(!result.contains(&2)); // Node 2 unreachable from 0
    assert!(!result.contains(&3)); // Node 3 unreachable from 0
}

#[test]
fn req1_test_bfs_single_node() {
    let mut graph = HashMap::new();
    graph.insert(42, vec![]); // Single node with no neighbors

    let result = bfs(&graph, 42);

    assert_eq!(result, vec![42]); // Only the start node
}

#[test]
fn req1_test_bfs_empty_neighbors() {
    let graph = create_linear_graph();
    let result = bfs(&graph, 3); // Start from leaf node

    assert_eq!(result, vec![3]); // Only node 3, no neighbors to visit
}

#[test]
fn req1_test_dfs_simple_graph() {
    let graph = create_simple_graph();
    let result = dfs(&graph, 0);

    // DFS should visit all nodes, starting with 0
    assert_eq!(result[0], 0); // Start node first
    assert_eq!(result.len(), 4); // All nodes visited

    // All nodes should be present
    assert!(result.contains(&0));
    assert!(result.contains(&1));
    assert!(result.contains(&2));
    assert!(result.contains(&3));
}

#[test]
fn req1_test_dfs_vs_bfs_traversal_order() {
    let graph = create_linear_graph();

    let bfs_result = bfs(&graph, 0);
    let dfs_result = dfs(&graph, 0);

    // Both should visit all nodes
    assert_eq!(bfs_result.len(), 4);
    assert_eq!(dfs_result.len(), 4);

    // Both should start with node 0
    assert_eq!(bfs_result[0], 0);
    assert_eq!(dfs_result[0], 0);

    // For linear graph, both BFS and DFS should produce same order
    assert_eq!(bfs_result, vec![0, 1, 2, 3]);
    assert_eq!(dfs_result, vec![0, 1, 2, 3]);
}

#[test]
fn req1_test_dfs_depth_first_behavior() {
    // Create a tree where DFS behavior is clearly different from BFS
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![3, 4]);
    graph.insert(2, vec![5, 6]);
    graph.insert(3, vec![]);
    graph.insert(4, vec![]);
    graph.insert(5, vec![]);
    graph.insert(6, vec![]);

    let dfs_result = dfs(&graph, 0);
    let bfs_result = bfs(&graph, 0);

    // Both should visit all 7 nodes
    assert_eq!(dfs_result.len(), 7);
    assert_eq!(bfs_result.len(), 7);

    // DFS explores depth first, so should visit one branch completely
    // before exploring the other branch
    let _pos_1_dfs = dfs_result.iter().position(|&x| x == 1).unwrap();
    let pos_3_dfs = dfs_result.iter().position(|&x| x == 3).unwrap();
    let pos_2_dfs = dfs_result.iter().position(|&x| x == 2).unwrap();

    // In DFS, node 3 (child of 1) should come before node 2 (sibling of 1)
    // This shows depth-first behavior
    assert!(
        pos_3_dfs < pos_2_dfs,
        "DFS should explore depth-first: node 3 before node 2"
    );
}

#[test]
fn req1_test_multiple_graph_types() {
    // Test that algorithms work on different representations

    // Adjacency list (HashMap)
    let mut adj_list = HashMap::new();
    adj_list.insert("A", vec!["B", "C"]);
    adj_list.insert("B", vec!["D"]);
    adj_list.insert("C", vec!["D"]);
    adj_list.insert("D", vec![]);

    let bfs_result = bfs(&adj_list, "A");
    let dfs_result = dfs(&adj_list, "A");

    // Should work with string nodes
    assert_eq!(bfs_result[0], "A");
    assert_eq!(dfs_result[0], "A");
    assert_eq!(bfs_result.len(), 4);
    assert_eq!(dfs_result.len(), 4);

    // Test with different numeric types
    let mut int_graph = HashMap::new();
    int_graph.insert(1i32, vec![2i32, 3i32]);
    int_graph.insert(2i32, vec![]);
    int_graph.insert(3i32, vec![]);

    let result = bfs(&int_graph, 1i32);
    assert_eq!(result[0], 1i32);
    assert_eq!(result.len(), 3);
}

#[test]
fn req1_test_graph_trait_implementation() {
    let mut graph = HashMap::new();
    graph.insert(1, vec![2, 3]);
    graph.insert(2, vec![]);
    graph.insert(3, vec![]);

    // Test Graph trait methods
    assert!(graph.contains(1));
    assert!(!graph.contains(99));

    let neighbors = graph.neighbors(1);
    assert_eq!(neighbors.len(), 2);
    assert!(neighbors.contains(&2));
    assert!(neighbors.contains(&3));

    let all_nodes = graph.nodes();
    assert_eq!(all_nodes.len(), 3);
    assert!(all_nodes.contains(&1));
    assert!(all_nodes.contains(&2));
    assert!(all_nodes.contains(&3));
}

#[test]
fn req1_test_empty_graph_neighbors() {
    let graph: HashMap<i32, Vec<i32>> = HashMap::new();

    // Non-existent node should return empty neighbors
    let neighbors = graph.neighbors(99);
    assert_eq!(neighbors.len(), 0);

    // Empty graph should contain no nodes
    let nodes = graph.nodes();
    assert_eq!(nodes.len(), 0);
}

#[test]
fn req1_test_bfs_cycle_handling() {
    // Create a graph with cycles to ensure algorithms terminate
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![0]); // Creates cycle: 0 -> 1 -> 2 -> 0

    let bfs_result = bfs(&graph, 0);
    let dfs_result = dfs(&graph, 0);

    // Should visit each node exactly once despite cycle
    assert_eq!(bfs_result.len(), 3);
    assert_eq!(dfs_result.len(), 3);

    // Should not get stuck in infinite loop
    assert!(bfs_result.contains(&0));
    assert!(bfs_result.contains(&1));
    assert!(bfs_result.contains(&2));
}
