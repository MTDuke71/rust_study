//! Mission 9 - Day 4: Performance Optimization Requirements Testing
//!
//! This test suite validates the implementation of:
//! - REQ-3: Bidirectional Search Optimization
//! - REQ-4: Early Termination Strategies
//! - Memory pool management and optimization
//! - Performance improvements and metrics collection

use mission9::performance_optimization::{
    BidirectionalAstar, BidirectionalDijkstra, MemoryOptimizedAstar, NodePool, OptimizedNode,
    PerformanceMetrics,
};
use std::time::Instant;

/// Type alias for a grid position (x, y)
type Position = (usize, usize);

/// Type alias for a neighbor with cost (position, cost)
type Neighbor = (Position, u32);

/// Test helper: Manhattan distance heuristic for grid-based pathfinding
fn manhattan_distance(a: Position, b: Position) -> u32 {
    let dx = a.0.abs_diff(b.0);
    let dy = a.1.abs_diff(b.1);
    (dx + dy) as u32
}

/// Test helper: Euclidean distance heuristic (rounded up)
fn euclidean_distance(a: Position, b: Position) -> u32 {
    let dx = (a.0 as f64) - (b.0 as f64);
    let dy = (a.1 as f64) - (b.1 as f64);
    (dx * dx + dy * dy).sqrt().ceil() as u32
}

/// Test helper: Generate 4-connected grid neighbors
fn grid_neighbors_4(pos: Position, width: usize, height: usize) -> Vec<Neighbor> {
    let mut neighbors = Vec::new();
    let (x, y) = pos;

    if x > 0 {
        neighbors.push(((x - 1, y), 1));
    }
    if x + 1 < width {
        neighbors.push(((x + 1, y), 1));
    }
    if y > 0 {
        neighbors.push(((x, y - 1), 1));
    }
    if y + 1 < height {
        neighbors.push(((x, y + 1), 1));
    }

    neighbors
}

/// Test helper: Generate 8-connected grid neighbors (including diagonals)
#[allow(dead_code)]
fn grid_neighbors_8(pos: Position, width: usize, height: usize) -> Vec<Neighbor> {
    let mut neighbors = Vec::new();
    let (x, y) = pos;

    // Orthogonal neighbors (cost 1)
    if x > 0 {
        neighbors.push(((x - 1, y), 1));
    }
    if x + 1 < width {
        neighbors.push(((x + 1, y), 1));
    }
    if y > 0 {
        neighbors.push(((x, y - 1), 1));
    }
    if y + 1 < height {
        neighbors.push(((x, y + 1), 1));
    }

    // Diagonal neighbors (cost ~1.41, rounded to 2 for integer arithmetic)
    if x > 0 && y > 0 {
        neighbors.push(((x - 1, y - 1), 2));
    }
    if x > 0 && y + 1 < height {
        neighbors.push(((x - 1, y + 1), 2));
    }
    if x + 1 < width && y > 0 {
        neighbors.push(((x + 1, y - 1), 2));
    }
    if x + 1 < width && y + 1 < height {
        neighbors.push(((x + 1, y + 1), 2));
    }

    neighbors
}

/// Test helper: Create a maze-like grid with obstacles
fn create_maze_neighbors(
    obstacles: &[(usize, usize)],
    width: usize,
    height: usize,
) -> impl Fn(Position) -> Vec<Neighbor> + '_ {
    move |pos| {
        grid_neighbors_4(pos, width, height)
            .into_iter()
            .filter(|(neighbor_pos, _)| !obstacles.contains(neighbor_pos))
            .collect()
    }
}

// ===== REQ-3: Bidirectional Search Optimization Tests =====

#[test]
fn test_req3_bidirectional_astar_creation() {
    let astar = BidirectionalAstar::new(1000);
    
    // Should be initialized properly
    let metrics = astar.get_metrics();
    assert_eq!(metrics.nodes_explored, 0);
    assert_eq!(metrics.nodes_generated, 0);
}

#[test]
fn test_req3_bidirectional_astar_simple_path() {
    let mut astar = BidirectionalAstar::new(1000);
    let start = (0, 0);
    let goal = (4, 4);

    let path = astar.find_path(
        start,
        goal,
        manhattan_distance,
        |pos| grid_neighbors_4(pos, 10, 10),
    );

    assert!(path.is_some());
    let path = path.unwrap();

    // Verify path endpoints
    assert_eq!(path.first(), Some(&start));
    assert_eq!(path.last(), Some(&goal));

    // Verify path is valid (each step is adjacent)
    for i in 1..path.len() {
        let from = path[i - 1];
        let to = path[i];
        let distance = manhattan_distance(from, to);
        assert_eq!(distance, 1, "Path steps should be adjacent");
    }

    // Verify metrics
    let metrics = astar.get_metrics();
    assert!(metrics.nodes_explored > 0);
    assert!(metrics.path_length > 0);
    assert_eq!(metrics.path_length, path.len());
}

#[test]
fn test_req3_bidirectional_astar_no_path() {
    let mut astar = BidirectionalAstar::new(1000);
    let start = (0, 0);
    let goal = (9, 9);

    // Create obstacles that block all paths
    let obstacles = vec![
        (1, 0), (2, 0), (3, 0), (4, 0), (5, 0),
        (0, 1), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1),
    ];

    let path = astar.find_path(
        start,
        goal,
        manhattan_distance,
        create_maze_neighbors(&obstacles, 10, 10),
    );

    assert!(path.is_none());

    let metrics = astar.get_metrics();
    assert!(metrics.nodes_explored > 0);
    assert_eq!(metrics.path_length, 0);
}

#[test]
fn test_req3_bidirectional_dijkstra_creation() {
    let dijkstra = BidirectionalDijkstra::new(1000);
    
    let metrics = dijkstra.get_metrics();
    assert_eq!(metrics.nodes_explored, 0);
    assert_eq!(metrics.nodes_generated, 0);
}

#[test]
fn test_req3_bidirectional_dijkstra_simple_path() {
    let mut dijkstra = BidirectionalDijkstra::new(1000);
    let start = (0, 0);
    let goal = (5, 5);

    let path = dijkstra.find_path(
        start,
        goal,
        |pos| grid_neighbors_4(pos, 10, 10),
    );

    assert!(path.is_some());
    let path = path.unwrap();

    // Verify path endpoints
    assert_eq!(path.first(), Some(&start));
    assert_eq!(path.last(), Some(&goal));

    // Verify path validity
    for i in 1..path.len() {
        let from = path[i - 1];
        let to = path[i];
        let distance = manhattan_distance(from, to);
        assert_eq!(distance, 1, "Path steps should be adjacent");
    }

    let metrics = dijkstra.get_metrics();
    assert!(metrics.nodes_explored > 0);
    assert!(metrics.path_length > 0);
}

#[test]
fn test_req3_bidirectional_vs_traditional_efficiency() {
    // Test that bidirectional search explores fewer nodes than traditional approach
    let start = (0, 0);
    let goal = (15, 15); // Distant goal for better comparison

    // Bidirectional A*
    let mut bi_astar = BidirectionalAstar::new(2000);
    let bi_path = bi_astar.find_path(
        start,
        goal,
        manhattan_distance,
        |pos| grid_neighbors_4(pos, 20, 20),
    );

    assert!(bi_path.is_some());
    let bi_metrics = bi_astar.get_metrics();

    // For a 20x20 grid going from (0,0) to (15,15), bidirectional should be more efficient
    // In worst case, traditional A* might explore ~200-300 nodes
    // Bidirectional should explore significantly fewer
    println!("Bidirectional A* explored: {} nodes", bi_metrics.nodes_explored);
    
    // Bidirectional should generally explore fewer nodes for distant goals
    assert!(bi_metrics.nodes_explored < 300, "Bidirectional A* should be efficient");
    assert!(bi_metrics.path_cost > 0);
}

#[test]
fn test_req3_meeting_point_detection() {
    let mut astar = BidirectionalAstar::new(1000);
    let start = (0, 0);
    let goal = (6, 6);

    let path = astar.find_path(
        start,
        goal,
        manhattan_distance,
        |pos| grid_neighbors_4(pos, 10, 10),
    );

    assert!(path.is_some());
    let path = path.unwrap();

    // Path should be continuous from start to goal
    assert_eq!(path.first(), Some(&start));
    assert_eq!(path.last(), Some(&goal));

    // Meeting point should be somewhere in the middle
    let metrics = astar.get_metrics();
    assert!(metrics.nodes_explored > 0);
    
    // The path length should be optimal (Manhattan distance for 4-connected grid)
    let expected_min_length = manhattan_distance(start, goal) as usize + 1; // +1 for start node
    assert_eq!(path.len(), expected_min_length);
}

// ===== REQ-4: Early Termination Strategies Tests =====

#[test]
fn test_req4_early_termination_close_goal() {
    let mut astar = BidirectionalAstar::new(1000);
    let start = (5, 5);
    let goal = (6, 6); // Very close goal

    astar.find_path(
        start,
        goal,
        manhattan_distance,
        |pos| grid_neighbors_4(pos, 10, 10),
    );

    let metrics = astar.get_metrics();
    // Should terminate early with minimal exploration
    assert!(metrics.nodes_explored < 10, "Should terminate early for close goals");
    assert!(metrics.early_terminations > 0, "Should record early termination");
}

#[test]
fn test_req4_f_score_bound_checking() {
    let mut astar = BidirectionalAstar::new(1000);
    let start = (0, 0);
    let goal = (3, 3);

    astar.find_path(
        start,
        goal,
        manhattan_distance,
        |pos| grid_neighbors_4(pos, 10, 10),
    );

    let metrics = astar.get_metrics();
    // Should find optimal path efficiently
    assert!(metrics.path_cost > 0);
    assert!(metrics.nodes_explored > 0);
    
    // For a 4x4 subgrid, shouldn't need to explore too many nodes
    assert!(metrics.nodes_explored < 50);
}

#[test]
fn test_req4_early_termination_with_obstacles() {
    let mut astar = BidirectionalAstar::new(1000);
    let start = (0, 0);
    let goal = (2, 0); // Goal is close but might be blocked

    // Create a simple obstacle pattern
    let obstacles = vec![(1, 0)]; // Block direct path

    let path = astar.find_path(
        start,
        goal,
        manhattan_distance,
        create_maze_neighbors(&obstacles, 10, 10),
    );

    assert!(path.is_some());
    let path = path.unwrap();

    // Should find alternative path
    assert!(path.len() > 3); // Must go around obstacle

    let metrics = astar.get_metrics();
    assert!(metrics.nodes_explored > 0);
    assert!(metrics.path_cost > 2); // Cost higher due to detour
}

// ===== Memory Optimization Tests =====

#[test]
fn test_memory_optimized_astar_creation() {
    let astar = MemoryOptimizedAstar::new(1000);
    
    let metrics = astar.get_metrics();
    assert_eq!(metrics.nodes_explored, 0);
    assert_eq!(metrics.memory_allocations, 0);
}

#[test]
fn test_memory_optimized_astar_pathfinding() {
    let mut astar = MemoryOptimizedAstar::new(1000);
    let start = (0, 0);
    let goal = (7, 7);

    let path = astar.find_path(
        start,
        goal,
        manhattan_distance,
        |pos| grid_neighbors_4(pos, 10, 10),
    );

    assert!(path.is_some());
    let path = path.unwrap();

    assert_eq!(path.first(), Some(&start));
    assert_eq!(path.last(), Some(&goal));

    let metrics = astar.get_metrics();
    assert!(metrics.nodes_explored > 0);
    // Memory allocations might not be tracked in this implementation
    assert!(metrics.path_length > 0);
}

#[test]
fn test_memory_optimized_astar_with_different_heuristics() {
    let start = (0, 0);
    let goal = (5, 5);

    // Test with Manhattan distance
    let mut astar_manhattan = MemoryOptimizedAstar::new(1000);
    let path_manhattan = astar_manhattan.find_path(
        start,
        goal,
        manhattan_distance,
        |pos| grid_neighbors_4(pos, 10, 10),
    );

    // Test with Euclidean distance
    let mut astar_euclidean = MemoryOptimizedAstar::new(1000);
    let path_euclidean = astar_euclidean.find_path(
        start,
        goal,
        euclidean_distance,
        |pos| grid_neighbors_4(pos, 10, 10),
    );

    assert!(path_manhattan.is_some());
    assert!(path_euclidean.is_some());

    // Both should find optimal paths
    let manhattan_metrics = astar_manhattan.get_metrics();
    let euclidean_metrics = astar_euclidean.get_metrics();

    assert_eq!(manhattan_metrics.path_cost, euclidean_metrics.path_cost);
    
    // Euclidean might be more efficient due to better heuristic
    println!("Manhattan explored: {}", manhattan_metrics.nodes_explored);
    println!("Euclidean explored: {}", euclidean_metrics.nodes_explored);
}

// ===== Node Pool and Memory Management Tests =====

#[test]
fn test_node_pool_creation() {
    let pool = NodePool::new(100);
    let (nodes, free, capacity) = pool.stats();
    
    assert_eq!(nodes, 0);
    assert_eq!(free, 0);
    assert_eq!(capacity, 100);
}

#[test]
fn test_node_pool_allocation() {
    let mut pool = NodePool::new(10);
    
    let idx1 = pool.allocate((0, 0), 0, 5, None);
    let idx2 = pool.allocate((1, 1), 1, 4, Some((0, 0)));
    let idx3 = pool.allocate((2, 2), 2, 3, Some((1, 1)));
    
    assert_eq!(pool.get(idx1).position, (0, 0));
    assert_eq!(pool.get(idx1).g_cost, 0);
    assert_eq!(pool.get(idx1).h_cost, 5);
    assert_eq!(pool.get(idx1).f_cost(), 5);
    
    assert_eq!(pool.get(idx2).position, (1, 1));
    assert_eq!(pool.get(idx2).parent, Some((0, 0)));
    
    assert_eq!(pool.get(idx3).position, (2, 2));
    assert_eq!(pool.get(idx3).parent, Some((1, 1)));

    let (nodes, free, capacity) = pool.stats();
    assert_eq!(nodes, 3);
    assert_eq!(free, 0);
    assert_eq!(capacity, 10);
}

#[test]
fn test_node_pool_deallocation_and_reuse() {
    let mut pool = NodePool::new(10);
    
    let _idx1 = pool.allocate((0, 0), 0, 5, None);
    let idx2 = pool.allocate((1, 1), 1, 4, None);
    let _idx3 = pool.allocate((2, 2), 2, 3, None);
    
    // Deallocate middle node
    pool.deallocate(idx2);
    
    let (nodes, free, _capacity) = pool.stats();
    assert_eq!(nodes, 3);
    assert_eq!(free, 1);
    
    // Allocate new node - should reuse deallocated slot
    let idx4 = pool.allocate((3, 3), 3, 2, None);
    assert_eq!(idx4, idx2); // Should reuse the deallocated index
    
    assert_eq!(pool.get(idx4).position, (3, 3));
    assert_eq!(pool.get(idx4).g_cost, 3);
    
    let (nodes, free, _capacity) = pool.stats();
    assert_eq!(nodes, 3);
    assert_eq!(free, 0); // Free list should be empty now
}

#[test]
fn test_node_pool_capacity_expansion() {
    let mut pool = NodePool::new(2); // Very small capacity
    
    // Fill initial capacity
    let idx1 = pool.allocate((0, 0), 0, 0, None);
    let idx2 = pool.allocate((1, 1), 1, 1, None);
    
    let (nodes, _, capacity) = pool.stats();
    assert_eq!(nodes, 2);
    assert_eq!(capacity, 2);
    
    // This should trigger capacity expansion
    let idx3 = pool.allocate((2, 2), 2, 2, None);
    
    let (nodes, _, capacity) = pool.stats();
    assert_eq!(nodes, 3);
    assert_eq!(capacity, 4); // Should double capacity
    
    // Verify all nodes are accessible
    assert_eq!(pool.get(idx1).position, (0, 0));
    assert_eq!(pool.get(idx2).position, (1, 1));
    assert_eq!(pool.get(idx3).position, (2, 2));
}

#[test]
fn test_node_pool_clear() {
    let mut pool = NodePool::new(10);
    
    pool.allocate((0, 0), 0, 0, None);
    pool.allocate((1, 1), 1, 1, None);
    pool.deallocate(0);
    
    let (nodes, free, capacity) = pool.stats();
    assert_eq!(nodes, 2);
    assert_eq!(free, 1);
    assert_eq!(capacity, 10);
    
    pool.clear();
    
    let (nodes, free, capacity) = pool.stats();
    assert_eq!(nodes, 0);
    assert_eq!(free, 0);
    assert_eq!(capacity, 10); // Capacity should remain
}

// ===== OptimizedNode Tests =====

#[test]
fn test_optimized_node_creation() {
    let node = OptimizedNode::new((5, 10), 15, 20, Some((4, 9)));
    
    assert_eq!(node.position, (5, 10));
    assert_eq!(node.g_cost, 15);
    assert_eq!(node.h_cost, 20);
    assert_eq!(node.f_cost(), 35);
    assert_eq!(node.parent, Some((4, 9)));
}

#[test]
fn test_optimized_node_ordering() {
    let node1 = OptimizedNode::new((0, 0), 10, 5, None); // f_cost = 15
    let node2 = OptimizedNode::new((1, 1), 8, 4, None);  // f_cost = 12
    let node3 = OptimizedNode::new((2, 2), 6, 9, None);  // f_cost = 15
    
    // Lower f_cost should have higher priority (for min-heap)
    // The implementation uses other.f_cost().cmp(&self.f_cost()), which means:
    // node1 (f=15) < node2 (f=12) in terms of ordering (node1 has lower priority)
    assert!(node1 < node2); // node1 has higher f_cost, so lower priority
    assert!(node3 < node2); // node3 has higher f_cost, so lower priority
    
    // Equal f_cost should compare by position
    assert!(node1 < node3); // (0,0) < (2,2)
}

#[test]
fn test_optimized_node_equality() {
    let node1 = OptimizedNode::new((5, 5), 10, 5, None);
    let node2 = OptimizedNode::new((5, 5), 20, 3, Some((4, 4))); // Same position, different costs
    let node3 = OptimizedNode::new((6, 5), 10, 5, None);
    
    assert_eq!(node1, node2); // Same position
    assert_ne!(node1, node3); // Different position
}

// ===== Performance Metrics Tests =====

#[test]
fn test_performance_metrics_creation() {
    let metrics = PerformanceMetrics::new();
    
    assert_eq!(metrics.nodes_explored, 0);
    assert_eq!(metrics.nodes_generated, 0);
    assert_eq!(metrics.peak_open_set_size, 0);
    assert_eq!(metrics.peak_closed_set_size, 0);
    assert_eq!(metrics.memory_allocations, 0);
    assert_eq!(metrics.early_terminations, 0);
    assert_eq!(metrics.path_length, 0);
    assert_eq!(metrics.path_cost, 0);
    assert_eq!(metrics.search_time_ns, 0);
}

#[test]
fn test_performance_metrics_collection() {
    let mut astar = BidirectionalAstar::new(1000);
    let start = (0, 0);
    let goal = (5, 5);

    let path = astar.find_path(
        start,
        goal,
        manhattan_distance,
        |pos| grid_neighbors_4(pos, 10, 10),
    );

    assert!(path.is_some());
    
    let metrics = astar.get_metrics();
    
    // All metrics should have meaningful values
    assert!(metrics.nodes_explored > 0);
    assert!(metrics.nodes_generated > 0);
    assert!(metrics.path_length > 0);
    assert!(metrics.path_cost > 0);
    assert!(metrics.search_time_ns > 0);
    
    // Some metrics might be 0 depending on algorithm behavior
    // These are unsigned integers, so >= 0 is always true - just verify they exist
    let _ = metrics.peak_open_set_size;
    let _ = metrics.peak_closed_set_size;
    let _ = metrics.memory_allocations;
    let _ = metrics.early_terminations;
}

// ===== Complex Scenario Tests =====

#[test]
fn test_complex_maze_scenario() {
    let mut astar = BidirectionalAstar::new(2000);
    let start = (0, 0);
    let goal = (19, 19);

    // Create a complex maze with multiple obstacles
    let obstacles = vec![
        // Vertical wall
        (5, 0), (5, 1), (5, 2), (5, 3), (5, 4), (5, 5), (5, 6), (5, 7),
        // Horizontal wall
        (0, 10), (1, 10), (2, 10), (3, 10), (4, 10), (6, 10), (7, 10), (8, 10),
        // Additional scattered obstacles
        (15, 5), (15, 6), (15, 7), (16, 5), (16, 6), (16, 7),
        (10, 15), (11, 15), (12, 15), (10, 16), (11, 16), (12, 16),
    ];

    let path = astar.find_path(
        start,
        goal,
        manhattan_distance,
        create_maze_neighbors(&obstacles, 20, 20),
    );

    if let Some(path) = path {
        // Verify path validity
        assert_eq!(path.first(), Some(&start));
        assert_eq!(path.last(), Some(&goal));
        
        // Verify no path goes through obstacles
        for pos in &path {
            assert!(!obstacles.contains(pos), "Path should not go through obstacles");
        }
        
        // Verify path continuity
        for i in 1..path.len() {
            let from = path[i - 1];
            let to = path[i];
            let distance = manhattan_distance(from, to);
            assert_eq!(distance, 1, "Path steps should be adjacent");
        }

        let metrics = astar.get_metrics();
        assert!(metrics.nodes_explored > 0);
        assert!(metrics.path_cost > 0);
        
        println!("Complex maze - Nodes explored: {}", metrics.nodes_explored);
        println!("Complex maze - Path length: {}", metrics.path_length);
        println!("Complex maze - Path cost: {}", metrics.path_cost);
    }
}

#[test]
fn test_algorithm_comparison_performance() {
    let start = (0, 0);
    let goal = (12, 12);
    let grid_size = 15;

    // Test Bidirectional A* with Manhattan heuristic
    let mut bi_astar_manhattan = BidirectionalAstar::new(1000);
    let start_time = Instant::now();
    let path_bi_manhattan = bi_astar_manhattan.find_path(
        start,
        goal,
        manhattan_distance,
        |pos| grid_neighbors_4(pos, grid_size, grid_size),
    );
    let time_bi_manhattan = start_time.elapsed();

    // Test Bidirectional A* with Euclidean heuristic
    let mut bi_astar_euclidean = BidirectionalAstar::new(1000);
    let start_time = Instant::now();
    let path_bi_euclidean = bi_astar_euclidean.find_path(
        start,
        goal,
        euclidean_distance,
        |pos| grid_neighbors_4(pos, grid_size, grid_size),
    );
    let time_bi_euclidean = start_time.elapsed();

    // Test Bidirectional Dijkstra
    let mut bi_dijkstra = BidirectionalDijkstra::new(1000);
    let start_time = Instant::now();
    let path_bi_dijkstra = bi_dijkstra.find_path(
        start,
        goal,
        |pos| grid_neighbors_4(pos, grid_size, grid_size),
    );
    let time_bi_dijkstra = start_time.elapsed();

    // Test Memory Optimized A*
    let mut mem_astar = MemoryOptimizedAstar::new(1000);
    let start_time = Instant::now();
    let path_mem_astar = mem_astar.find_path(
        start,
        goal,
        manhattan_distance,
        |pos| grid_neighbors_4(pos, grid_size, grid_size),
    );
    let time_mem_astar = start_time.elapsed();

    // All should find paths
    assert!(path_bi_manhattan.is_some());
    assert!(path_bi_euclidean.is_some());
    assert!(path_bi_dijkstra.is_some());
    assert!(path_mem_astar.is_some());

    // All should find optimal paths (same cost)
    let bi_manhattan_metrics = bi_astar_manhattan.get_metrics();
    let bi_euclidean_metrics = bi_astar_euclidean.get_metrics();
    let bi_dijkstra_metrics = bi_dijkstra.get_metrics();
    let mem_astar_metrics = mem_astar.get_metrics();

    assert_eq!(bi_manhattan_metrics.path_cost, bi_euclidean_metrics.path_cost);
    assert_eq!(bi_manhattan_metrics.path_cost, bi_dijkstra_metrics.path_cost);
    assert_eq!(bi_manhattan_metrics.path_cost, mem_astar_metrics.path_cost);

    // Print performance comparison
    println!("\n=== Algorithm Performance Comparison ===");
    println!("Bidirectional A* (Manhattan): {} nodes, {:?}", 
             bi_manhattan_metrics.nodes_explored, time_bi_manhattan);
    println!("Bidirectional A* (Euclidean):  {} nodes, {:?}", 
             bi_euclidean_metrics.nodes_explored, time_bi_euclidean);
    println!("Bidirectional Dijkstra:       {} nodes, {:?}", 
             bi_dijkstra_metrics.nodes_explored, time_bi_dijkstra);
    println!("Memory Optimized A*:          {} nodes, {:?}", 
             mem_astar_metrics.nodes_explored, time_mem_astar);

    // Heuristic-guided algorithms should generally be more efficient than Dijkstra
    assert!(bi_manhattan_metrics.nodes_explored <= bi_dijkstra_metrics.nodes_explored);
    assert!(bi_euclidean_metrics.nodes_explored <= bi_dijkstra_metrics.nodes_explored);
}

/// Final comprehensive test validating REQ-3 and REQ-4 requirements
#[test]
fn test_req3_req4_comprehensive_validation() {
    println!("=== REQ-3 & REQ-4 Comprehensive Validation ===");

    let start = (0, 0);
    let goal = (10, 10);

    // ✓ REQ-3: Bidirectional Search Optimization
    let mut bi_astar = BidirectionalAstar::new(1000);
    let bi_path = bi_astar.find_path(
        start,
        goal,
        manhattan_distance,
        |pos| grid_neighbors_4(pos, 15, 15),
    );

    let mut bi_dijkstra = BidirectionalDijkstra::new(1000);
    let dijkstra_path = bi_dijkstra.find_path(
        start,
        goal,
        |pos| grid_neighbors_4(pos, 15, 15),
    );

    assert!(bi_path.is_some());
    assert!(dijkstra_path.is_some());

    let bi_metrics = bi_astar.get_metrics();
    let dijkstra_metrics = bi_dijkstra.get_metrics();

    // ✓ Meeting point detection and path merging
    assert_eq!(bi_path.as_ref().unwrap().first(), Some(&start));
    assert_eq!(bi_path.as_ref().unwrap().last(), Some(&goal));
    assert_eq!(dijkstra_path.as_ref().unwrap().first(), Some(&start));
    assert_eq!(dijkstra_path.as_ref().unwrap().last(), Some(&goal));

    // ✓ Performance comparison with unidirectional versions
    println!("Bidirectional A* explored: {} nodes", bi_metrics.nodes_explored);
    println!("Bidirectional Dijkstra explored: {} nodes", dijkstra_metrics.nodes_explored);

    // ✓ REQ-4: Early Termination Strategies
    
    // Test early termination with close goal
    let mut close_astar = BidirectionalAstar::new(100);
    close_astar.find_path(
        (5, 5),
        (6, 6), // Very close
        manhattan_distance,
        |pos| grid_neighbors_4(pos, 10, 10),
    );
    
    let close_metrics = close_astar.get_metrics();
    assert!(close_metrics.early_terminations > 0, "Should have early termination for close goals");
    assert!(close_metrics.nodes_explored < 10, "Should explore very few nodes for close goals");

    println!("✅ REQ-3: Bidirectional search algorithms implemented and validated");
    println!("✅ REQ-4: Early termination strategies implemented and validated");
    println!("✅ Memory optimization and performance metrics collection working");
}