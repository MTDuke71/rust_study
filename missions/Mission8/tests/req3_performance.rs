//! REQ-3 Performance Validation Tests
//!
//! These tests verify that algorithms scale correctly with graph size
//! and maintain expected time/space complexity characteristics.

use mission8::{bfs, dfs, shortest_path, has_cycle, connected_components};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Helper to create graphs of varying sizes for performance testing
fn create_test_graph(size: u32) -> HashMap<u32, Vec<u32>> {
    let mut graph = HashMap::new();
    
    // Create a connected graph where each node connects to a few others
    for i in 0..size {
        let neighbors = vec![
            (i + 1) % size,           // Next node (circular)
            (i + size / 2) % size,    // Node at opposite side
            (i + size / 4) % size,    // Node at quarter distance
        ];
        graph.insert(i, neighbors);
    }
    
    graph
}

/// Helper to measure execution time of a function
fn measure_time<F, R>(func: F) -> (Duration, R)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();
    (duration, result)
}

#[test]
fn test_bfs_linear_time() {
    // Test that BFS scales linearly with graph size (O(V + E))
    let sizes = vec![100, 200, 400, 800];
    let mut times = Vec::new();
    
    for &size in &sizes {
        let graph = create_test_graph(size);
        let (duration, _) = measure_time(|| bfs(&graph, 0));
        
        times.push(duration);
        
        // Verify BFS visits all nodes in connected graph
        let visited = bfs(&graph, 0);
        assert_eq!(visited.len(), size as usize, 
                  "BFS should visit all {} nodes", size);
    }
    
    // Verify roughly linear scaling (allowing for variance)
    // Time for 800 nodes should be less than 15x time for 100 nodes
    // (allows for system variance and increased edge density)
    assert!(times[3] < times[0] * 15, 
           "BFS should scale roughly linearly, got {:?}", times);
    
    // Each BFS should complete reasonably quickly
    for (i, &time) in times.iter().enumerate() {
        assert!(time < Duration::from_millis(100), 
               "BFS on {} nodes took {:?}, should be under 100ms", 
               sizes[i], time);
    }
}

#[test]
fn test_dfs_linear_time() {
    // Test that DFS scales linearly with graph size (O(V + E))
    let sizes = vec![100, 200, 400, 800];
    let mut times = Vec::new();
    
    for &size in &sizes {
        let graph = create_test_graph(size);
        let (duration, _) = measure_time(|| dfs(&graph, 0));
        
        times.push(duration);
        
        // Verify DFS visits all nodes in connected graph
        let visited = dfs(&graph, 0);
        assert_eq!(visited.len(), size as usize, 
                  "DFS should visit all {} nodes", size);
    }
    
    // Verify roughly linear scaling (allowing for variance)
    assert!(times[3] < times[0] * 15, 
           "DFS should scale roughly linearly, got {:?}", times);
    
    // Each DFS should complete reasonably quickly
    for (i, &time) in times.iter().enumerate() {
        assert!(time < Duration::from_millis(100), 
               "DFS on {} nodes took {:?}, should be under 100ms", 
               sizes[i], time);
    }
}

#[test]
fn test_shortest_path_performance() {
    // Test shortest path performance on graphs of different sizes
    let sizes = vec![100, 500, 1000];
    
    for &size in &sizes {
        let graph = create_test_graph(size);
        let start_node = 0;
        let end_node = size / 2; // Target node at middle
        
        let (duration, path) = measure_time(|| {
            shortest_path(&graph, start_node, end_node)
        });
        
        // Verify path exists (graph is connected)
        assert!(path.is_ok(), "Should find path in connected graph");
        
        // Verify reasonable performance (should complete quickly)
        assert!(duration < Duration::from_millis(50), 
               "Shortest path on {} nodes took {:?}, should be under 50ms", 
               size, duration);
        
        // Verify path is reasonable length (not searching entire graph)
        if let Ok(p) = path {
            assert!(p.len() <= size as usize / 2, 
                   "Path length {} seems too long for graph size {}", 
                   p.len(), size);
        }
    }
}

#[test]
fn test_cycle_detection_performance() {
    // Test cycle detection on graphs with and without cycles
    let sizes = vec![100, 500, 1000];
    
    for &size in &sizes {
        // Test on cyclic graph (our test graphs have cycles)
        let cyclic_graph = create_test_graph(size);
        let (duration, has_cycle_result) = measure_time(|| {
            has_cycle(&cyclic_graph)
        });
        
        // Should detect cycle in our cyclic test graph
        assert!(has_cycle_result, "Should detect cycle in cyclic graph");
        
        // Should complete quickly
        assert!(duration < Duration::from_millis(50), 
               "Cycle detection on {} nodes took {:?}, should be under 50ms", 
               size, duration);
        
        // Test on acyclic graph (tree structure)
        let mut acyclic_graph = HashMap::new();
        for i in 0..size {
            if i > 0 {
                acyclic_graph.insert(i, vec![i - 1]); // Tree structure
            } else {
                acyclic_graph.insert(i, vec![]); // Root node
            }
        }
        
        let (duration2, has_cycle_result2) = measure_time(|| {
            has_cycle(&acyclic_graph)
        });
        
        // Should NOT detect cycle in acyclic graph
        assert!(!has_cycle_result2, "Should not detect cycle in acyclic graph");
        
        // Should complete quickly
        assert!(duration2 < Duration::from_millis(50), 
               "Cycle detection on acyclic {} nodes took {:?}, should be under 50ms", 
               size, duration2);
    }
}

#[test]
fn test_connected_components_performance() {
    // Test connected components on single and multiple component graphs
    let component_sizes = vec![100, 300, 500];
    
    for &size in &component_sizes {
        // Single component (connected graph)
        let single_graph = create_test_graph(size);
        let (duration1, components1) = measure_time(|| {
            connected_components(&single_graph)
        });
        
        // Should find exactly one component
        assert_eq!(components1.len(), 1, 
                  "Connected graph should have 1 component");
        assert_eq!(components1[0].len(), size as usize, 
                  "Component should contain all {} nodes", size);
        
        // Should complete quickly
        assert!(duration1 < Duration::from_millis(100), 
               "Component finding on {} nodes took {:?}, should be under 100ms", 
               size, duration1);
        
        // Multiple components (disconnected graph)
        let num_components = 5;
        let nodes_per_component = size / num_components;
        let mut multi_graph = HashMap::new();
        
        for comp in 0..num_components {
            let base = comp * nodes_per_component;
            for i in 0..nodes_per_component {
                let node = base + i;
                let next = base + (i + 1) % nodes_per_component;
                multi_graph.insert(node, vec![next]);
            }
        }
        
        let (duration2, components2) = measure_time(|| {
            connected_components(&multi_graph)
        });
        
        // Should find expected number of components
        assert_eq!(components2.len(), num_components as usize, 
                  "Should find {} components", num_components);
        
        // Should complete quickly
        assert!(duration2 < Duration::from_millis(100), 
               "Component finding on disconnected {} nodes took {:?}, should be under 100ms", 
               size, duration2);
    }
}

#[test]
fn test_space_complexity() {
    // Test that algorithms use reasonable memory (O(V) space)
    // This is hard to measure precisely, but we can check basic bounds
    
    let size = 1000;
    let graph = create_test_graph(size);
    
    // BFS space usage test
    let visited_bfs = bfs(&graph, 0);
    assert_eq!(visited_bfs.len(), size as usize, 
              "BFS should visit all nodes exactly once");
    
    // DFS space usage test  
    let visited_dfs = dfs(&graph, 0);
    assert_eq!(visited_dfs.len(), size as usize, 
              "DFS should visit all nodes exactly once");
    
    // Shortest path space usage
    if let Ok(path) = shortest_path(&graph, 0, size - 1) {
        assert!(path.len() <= size as usize, 
               "Shortest path should not be longer than graph size");
    }
    
    // Connected components space usage
    let components = connected_components(&graph);
    let total_nodes: usize = components.iter().map(|c| c.len()).sum();
    assert_eq!(total_nodes, size as usize, 
              "Components should account for all nodes exactly once");
}

#[test]
fn test_large_graph_performance() {
    // Test that algorithms can handle reasonably large graphs
    let large_size = 5000;
    let graph = create_test_graph(large_size);
    
    // All algorithms should complete within reasonable time
    let (bfs_time, bfs_result) = measure_time(|| bfs(&graph, 0));
    assert!(bfs_time < Duration::from_millis(500), 
           "BFS on {} nodes took {:?}, should be under 500ms", 
           large_size, bfs_time);
    assert_eq!(bfs_result.len(), large_size as usize);
    
    let (dfs_time, dfs_result) = measure_time(|| dfs(&graph, 0));
    assert!(dfs_time < Duration::from_millis(500), 
           "DFS on {} nodes took {:?}, should be under 500ms", 
           large_size, dfs_time);
    assert_eq!(dfs_result.len(), large_size as usize);
    
    let (path_time, _) = measure_time(|| {
        shortest_path(&graph, 0, large_size - 1)
    });
    assert!(path_time < Duration::from_millis(100), 
           "Shortest path on {} nodes took {:?}, should be under 100ms", 
           large_size, path_time);
    
    let (cycle_time, _) = measure_time(|| has_cycle(&graph));
    assert!(cycle_time < Duration::from_millis(200), 
           "Cycle detection on {} nodes took {:?}, should be under 200ms", 
           large_size, cycle_time);
    
    let (comp_time, _) = measure_time(|| connected_components(&graph));
    assert!(comp_time < Duration::from_millis(1000), 
           "Component finding on {} nodes took {:?}, should be under 1000ms", 
           large_size, comp_time);
}

#[test]
fn test_algorithm_correctness_under_load() {
    // Verify algorithms remain correct under performance pressure
    let size = 2000;
    let graph = create_test_graph(size);
    
    // Run multiple times to check consistency
    for _ in 0..5 {
        let bfs1 = bfs(&graph, 0);
        let bfs2 = bfs(&graph, 0);
        
        // BFS should visit same number of nodes consistently
        assert_eq!(bfs1.len(), bfs2.len(), 
                  "BFS should be deterministic in node count");
        
        // All visited nodes should be unique
        let mut unique_bfs = bfs1.clone();
        unique_bfs.sort();
        unique_bfs.dedup();
        assert_eq!(unique_bfs.len(), bfs1.len(), 
                  "BFS should not visit nodes multiple times");
        
        // DFS consistency
        let dfs1 = dfs(&graph, 0);
        let dfs2 = dfs(&graph, 0);
        assert_eq!(dfs1.len(), dfs2.len(), 
                  "DFS should be deterministic in node count");
        
        // All visited nodes should be unique
        let mut unique_dfs = dfs1.clone();
        unique_dfs.sort();
        unique_dfs.dedup();
        assert_eq!(unique_dfs.len(), dfs1.len(), 
                  "DFS should not visit nodes multiple times");
    }
}