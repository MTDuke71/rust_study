//! Tests for REQ-2: Algorithm Composition
//!
//! This test suite validates that composed algorithms (shortest_path, has_cycle,
//! find_cycle, connected_components) work correctly on various graph types and
//! handle edge cases properly.

use mission8::{connected_components, find_cycle, has_cycle, shortest_path, Graph, GraphError};
use std::collections::HashMap;

/// Create a simple 4-node test graph
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

/// Create a graph with multiple disconnected components
/// Component 1: 0 <-> 1 <-> 2
/// Component 2: 3 <-> 4
fn create_disconnected_components() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![0, 2]);
    graph.insert(2, vec![1]);
    graph.insert(3, vec![4]);
    graph.insert(4, vec![3]);
    graph
}

/// Create a cyclic graph
/// Cycle: 0 -> 1 -> 2 -> 0
fn create_cyclic_graph() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![0]);
    graph
}

/// Create an acyclic DAG
/// 0 -> 1 -> 3
/// 0 -> 2 -> 3
fn create_acyclic_dag() -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![3]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![]);
    graph
}

// ============================================================================
// SHORTEST PATH TESTS
// ============================================================================

#[test]
fn req2_shortest_path_simple_graph() {
    let graph = create_simple_graph();
    let result = shortest_path(&graph, 0, 3);

    assert!(result.is_ok());
    let path = result.unwrap();

    // Path must start at 0 and end at 3
    assert_eq!(path[0], 0);
    assert_eq!(path[path.len() - 1], 3);

    // Path length should be 3 (0 -> 1 -> 3 or 0 -> 2 -> 3)
    assert_eq!(path.len(), 3);

    // All nodes in path must be reachable
    assert!(path.iter().all(|&n| graph.contains(n)));
}

#[test]
fn req2_shortest_path_direct_edge() {
    let graph = create_simple_graph();
    let result = shortest_path(&graph, 0, 1);

    assert!(result.is_ok());
    let path = result.unwrap();

    // Direct edge: shortest path is just 0 -> 1
    assert_eq!(path, vec![0, 1]);
}

#[test]
fn req2_shortest_path_same_node() {
    let graph = create_simple_graph();
    let result = shortest_path(&graph, 2, 2);

    assert!(result.is_ok());
    let path = result.unwrap();

    // Path to same node is just that node
    assert_eq!(path, vec![2]);
}

#[test]
fn req2_shortest_path_no_path() {
    let graph = create_disconnected_components();
    let result = shortest_path(&graph, 0, 3);

    // Component 1: {0, 1, 2}
    // Component 2: {3, 4}
    // No path between different components
    assert!(result.is_err());

    match result {
        Err(GraphError::NoPathExists { .. }) => {
            // Expected error
        }
        _ => panic!("Expected NoPathExists error"),
    }
}

#[test]
fn req2_shortest_path_two_paths_chooses_shortest() {
    // Graph: 0 -> 1 -> 3
    //        0 -> 2 -> 3
    let graph = create_simple_graph();
    let path = shortest_path(&graph, 0, 3).unwrap();

    // BFS should find a shortest path of length 3
    assert_eq!(path.len(), 3);

    // Both possible paths have same length in this case
    // But BFS guarantees shortest
    assert_eq!(path[0], 0);
    assert_eq!(path[2], 3);
}

#[test]
fn req2_shortest_path_nonexistent_start_node() {
    let graph = create_simple_graph();
    let result = shortest_path(&graph, 99, 1);

    // Start node doesn't exist
    assert!(result.is_err());
    match result {
        Err(GraphError::InvalidInput(_)) => {
            // Expected error
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

#[test]
fn req2_shortest_path_with_cycles() {
    // Even with cycles, shortest path should still work
    let graph = create_cyclic_graph();
    let result = shortest_path(&graph, 0, 2);

    assert!(result.is_ok());
    let path = result.unwrap();

    // Direct path: 0 -> 1 -> 2
    assert_eq!(path, vec![0, 1, 2]);
}

#[test]
fn req2_shortest_path_linear_chain() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![4]);
    graph.insert(4, vec![]);

    let path = shortest_path(&graph, 0, 4).unwrap();

    // Only one path: 0 -> 1 -> 2 -> 3 -> 4
    assert_eq!(path, vec![0, 1, 2, 3, 4]);
}

// ============================================================================
// CYCLE DETECTION TESTS
// ============================================================================

#[test]
fn req2_has_cycle_acyclic_dag() {
    let graph = create_acyclic_dag();

    // DAG should not have cycles
    assert!(!has_cycle(&graph));
}

#[test]
fn req2_has_cycle_cyclic_graph() {
    let graph = create_cyclic_graph();

    // This graph has cycle: 0 -> 1 -> 2 -> 0
    assert!(has_cycle(&graph));
}

#[test]
fn req2_has_cycle_simple_self_loop() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![0]); // Self-loop

    assert!(has_cycle(&graph));
}

#[test]
fn req2_has_cycle_disconnected_components() {
    let graph = create_disconnected_components();

    // No cycles in this graph (just bidirectional edges, not true cycles)
    // Note: In directed graphs, {0 <-> 1} means 0->1 and 1->0, creating a cycle
    assert!(has_cycle(&graph));
}

#[test]
fn req2_has_cycle_single_node() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![]);

    // Single node with no edges
    assert!(!has_cycle(&graph));
}

#[test]
fn req2_has_cycle_two_nodes_no_cycle() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![]);

    assert!(!has_cycle(&graph));
}

#[test]
fn req2_has_cycle_two_nodes_with_cycle() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![0]);

    assert!(has_cycle(&graph));
}

#[test]
fn req2_has_cycle_complex_cycle() {
    let mut graph = HashMap::new();
    // Create a more complex cycle
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![3]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![0]); // Back edge creating cycle

    assert!(has_cycle(&graph));
}

#[test]
fn req2_has_cycle_multiple_components_with_cycle_in_one() {
    let mut graph = HashMap::new();
    // Component 1: acyclic
    graph.insert(0, vec![1]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![]);

    // Component 2: cyclic
    graph.insert(3, vec![4]);
    graph.insert(4, vec![3]);

    // Graph has cycle because one component has one
    assert!(has_cycle(&graph));
}

// ============================================================================
// FIND CYCLE TESTS
// ============================================================================

#[test]
fn req2_find_cycle_cyclic_graph() {
    let graph = create_cyclic_graph();

    let cycle = find_cycle(&graph);
    assert!(cycle.is_some());

    let cycle_nodes = cycle.unwrap();
    // Should form a cycle
    assert!(cycle_nodes.len() >= 3);
}

#[test]
fn req2_find_cycle_acyclic_returns_none() {
    let graph = create_acyclic_dag();

    let cycle = find_cycle(&graph);
    assert!(cycle.is_none());
}

#[test]
fn req2_find_cycle_self_loop() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![0]);

    let cycle = find_cycle(&graph);
    assert!(cycle.is_some());
    // Self-loop forms a cycle of length 1
    assert_eq!(cycle.unwrap().len(), 1);
}

#[test]
fn req2_find_cycle_simple_two_node_cycle() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![0]);

    let cycle = find_cycle(&graph);
    assert!(cycle.is_some());

    let cycle_nodes = cycle.unwrap();
    assert_eq!(cycle_nodes.len(), 2);
}

#[test]
fn req2_find_cycle_larger_graph() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![4]);
    graph.insert(4, vec![2]); // Cycle starts here: 2 -> 3 -> 4 -> 2

    let cycle = find_cycle(&graph);
    assert!(cycle.is_some());

    let cycle_nodes = cycle.unwrap();
    // Cycle: 2 -> 3 -> 4 -> 2 (3 nodes)
    assert_eq!(cycle_nodes.len(), 3);
}

// ============================================================================
// CONNECTED COMPONENTS TESTS
// ============================================================================

#[test]
fn req2_connected_components_single_component() {
    let graph = create_acyclic_dag();

    let components = connected_components(&graph);

    // All nodes are in one component
    assert_eq!(components.len(), 1);

    // All 4 nodes are in that component
    assert_eq!(components[0].len(), 4);
}

#[test]
fn req2_connected_components_multiple_components() {
    let graph = create_disconnected_components();

    let components = connected_components(&graph);

    // Should have 2 components
    assert_eq!(components.len(), 2);

    // One component has 3 nodes, the other has 2 nodes (order may vary)
    let mut sizes: Vec<usize> = components.iter().map(|c| c.len()).collect();
    sizes.sort();
    assert_eq!(sizes, vec![2, 3]);
}

#[test]
fn req2_connected_components_single_node() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![]);

    let components = connected_components(&graph);

    assert_eq!(components.len(), 1);
    assert_eq!(components[0], vec![0]);
}

#[test]
fn req2_connected_components_isolated_nodes() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![]);
    graph.insert(1, vec![]);
    graph.insert(2, vec![]);

    let components = connected_components(&graph);

    // Each node is its own component
    assert_eq!(components.len(), 3);

    // Each component has exactly 1 node
    for component in components {
        assert_eq!(component.len(), 1);
    }
}

#[test]
fn req2_connected_components_all_connected() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2, 3]);
    graph.insert(1, vec![0]);
    graph.insert(2, vec![0]);
    graph.insert(3, vec![0]);

    let components = connected_components(&graph);

    // All nodes connected through node 0
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].len(), 4);
}

#[test]
fn req2_connected_components_with_cycles() {
    let graph = create_cyclic_graph();

    let components = connected_components(&graph);

    // All nodes in the cycle form one component
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].len(), 3);
}

#[test]
fn req2_connected_components_three_components() {
    let mut graph = HashMap::new();

    // Component 1: 0 <-> 1
    graph.insert(0, vec![1]);
    graph.insert(1, vec![0]);

    // Component 2: 2 -> 3 -> 4
    graph.insert(2, vec![3]);
    graph.insert(3, vec![4]);
    graph.insert(4, vec![]);

    // Component 3: isolated 5
    graph.insert(5, vec![]);

    let components = connected_components(&graph);

    assert_eq!(components.len(), 3);

    // Find component sizes (order may vary)
    let mut sizes: Vec<usize> = components.iter().map(|c| c.len()).collect();
    sizes.sort();

    assert_eq!(sizes, vec![1, 2, 3]);
}

#[test]
fn req2_connected_components_completeness() {
    let graph = create_disconnected_components();
    let components = connected_components(&graph);

    // All nodes should appear exactly once
    let all_nodes: Vec<u32> = components.iter().flat_map(|c| c.iter().copied()).collect();
    assert_eq!(all_nodes.len(), 5);

    // Each node should appear exactly once
    for node in 0..5 {
        assert_eq!(all_nodes.iter().filter(|&&n| n == node).count(), 1);
    }
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[test]
fn req2_shortest_path_respects_connectivity() {
    // In a disconnected graph, shortest_path should fail properly
    let graph = create_disconnected_components();

    // Within component: should work
    let path1 = shortest_path(&graph, 0, 2);
    assert!(path1.is_ok());

    // Across components: should fail
    let path2 = shortest_path(&graph, 0, 4);
    assert!(path2.is_err());
}

#[test]
fn req2_cycle_detection_and_components_consistency() {
    let graph = create_cyclic_graph();

    // If graph has cycle, it should still have components
    assert!(has_cycle(&graph));

    let components = connected_components(&graph);
    assert!(!components.is_empty());
}

#[test]
fn req2_algorithms_consistent_on_string_nodes() {
    // Test that algorithms work with different node types
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    graph.insert("A", vec!["B"]);
    graph.insert("B", vec!["C"]);
    graph.insert("C", vec![]);

    let path = shortest_path(&graph, "A", "C").unwrap();
    assert_eq!(path, vec!["A", "B", "C"]);

    let has_cyc = has_cycle(&graph);
    assert!(!has_cyc);

    let components = connected_components(&graph);
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].len(), 3);
}

#[test]
fn req2_large_graph_performance() {
    // Create a larger graph to ensure algorithms scale
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();

    // Create a chain: 0 -> 1 -> 2 -> ... -> 99
    for i in 0..99 {
        graph.insert(i, vec![i + 1]);
    }
    graph.insert(99, vec![]);

    // Should find path quickly
    let path = shortest_path(&graph, 0, 99).unwrap();
    assert_eq!(path.len(), 100);

    // Should detect no cycles quickly
    assert!(!has_cycle(&graph));

    // Should find all nodes in one component
    let components = connected_components(&graph);
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].len(), 100);
}
