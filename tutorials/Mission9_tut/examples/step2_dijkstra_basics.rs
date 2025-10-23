//! # Step 2: Dijkstra's Algorithm Basics
//!
//! **Time Estimate**: 30 minutes
//! **Requirements Addressed**: REQ-1 (Dijkstra's Algorithm Implementation)
//!
//! ## Learning Objectives
//! - Understand Dijkstra's shortest path algorithm
//! - Implement the core algorithm with step-by-step explanation
//! - Practice with weighted graph traversal
//! - Build path reconstruction from predecessor tracking
//! - Analyze algorithm performance and correctness guarantees
//!
//! ## Key Concepts
//! - **Shortest Path**: Guaranteed optimal path in graphs with non-negative weights
//! - **Relaxation**: Updating distance when shorter path is found
//! - **Priority Queue**: Always process closest unvisited node next
//! - **Predecessor Tracking**: Record path for reconstruction
//! - **Visited Set**: Avoid reprocessing nodes with final distances
//!
//! ## Algorithm Overview
//!
//! Dijkstra's algorithm finds shortest paths from a source to all vertices:
//! 1. **Initialize**: Set source distance to 0, all others to infinity
//! 2. **Priority Queue**: Add source to queue with distance 0
//! 3. **Main Loop**: While queue not empty:
//!    - Extract node with minimum distance
//!    - Mark as visited (distance is now final)
//!    - **Relax** all neighbors: if distance through current < known distance, update
//! 4. **Terminate**: When goal node is visited or queue empty
//!
//! ## Complexity Analysis
//! - **Time**: O((V + E) log V) with binary heap
//! - **Space**: O(V) for distances, predecessors, and visited set
//! - **Optimality**: Guaranteed shortest path with non-negative weights

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

/// Node identifier for our examples
pub type NodeId = u32;

/// Edge weight type (non-negative for Dijkstra)
pub type Weight = f64;

/// Wrapper to make f64 orderable for priority queue
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrderedFloat(pub f64);

impl OrderedFloat {
    pub fn new(value: f64) -> Self {
        assert!(value >= 0.0, "Dijkstra requires non-negative weights");
        assert!(!value.is_nan(), "Weight cannot be NaN");
        Self(value)
    }
}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

impl Eq for OrderedFloat {}

impl From<f64> for OrderedFloat {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

/// Priority queue item for Dijkstra's algorithm
/// Contains node ID and current best distance
#[derive(Debug, Clone, PartialEq, Eq)]
struct DijkstraItem {
    node: NodeId,
    distance: OrderedFloat,
}

impl DijkstraItem {
    fn new(node: NodeId, distance: f64) -> Self {
        Self {
            node,
            distance: OrderedFloat::new(distance),
        }
    }
}

// Reverse ordering for min-heap behavior (BinaryHeap is max-heap)
impl PartialOrd for DijkstraItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse comparison for min-heap
        other.distance.cmp(&self.distance)
    }
}

/// Simple weighted graph for demonstrations
#[derive(Debug, Clone)]
pub struct SimpleGraph {
    /// Adjacency list: node -> vec of (neighbor, weight)
    edges: HashMap<NodeId, Vec<(NodeId, Weight)>>,
    nodes: HashSet<NodeId>,
}

impl SimpleGraph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            nodes: HashSet::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: NodeId) {
        self.nodes.insert(node);
        self.edges.entry(node).or_insert_with(Vec::new);
    }

    /// Add an edge (automatically adds nodes if they don't exist)
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, weight: Weight) {
        assert!(weight >= 0.0, "Dijkstra requires non-negative weights");
        
        self.add_node(from);
        self.add_node(to);
        
        self.edges.get_mut(&from).unwrap().push((to, weight));
    }

    /// Add bidirectional edge
    pub fn add_bidirectional_edge(&mut self, a: NodeId, b: NodeId, weight: Weight) {
        self.add_edge(a, b, weight);
        self.add_edge(b, a, weight);
    }

    /// Get neighbors of a node
    pub fn neighbors(&self, node: NodeId) -> &[(NodeId, Weight)] {
        self.edges.get(&node).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Check if node exists
    pub fn contains_node(&self, node: NodeId) -> bool {
        self.nodes.contains(&node)
    }

    /// Get all nodes
    pub fn nodes(&self) -> &HashSet<NodeId> {
        &self.nodes
    }
}

/// Result of pathfinding operation
#[derive(Debug, Clone, PartialEq)]
pub struct PathResult {
    /// The path from start to goal (empty if no path)
    pub path: Vec<NodeId>,
    /// Total cost of the path
    pub cost: Weight,
    /// Number of nodes explored during search
    pub nodes_explored: usize,
}

impl PathResult {
    pub fn new(path: Vec<NodeId>, cost: Weight, nodes_explored: usize) -> Self {
        Self {
            path,
            cost,
            nodes_explored,
        }
    }

    pub fn no_path(nodes_explored: usize) -> Self {
        Self {
            path: Vec::new(),
            cost: f64::INFINITY,
            nodes_explored,
        }
    }
}

/// Comprehensive Dijkstra's algorithm implementation with detailed logging
pub fn dijkstra_with_explanation(
    graph: &SimpleGraph,
    start: NodeId,
    goal: NodeId,
    verbose: bool,
) -> Result<PathResult, String> {
    // Input validation
    if !graph.contains_node(start) {
        return Err(format!("Start node {} does not exist in graph", start));
    }
    if !graph.contains_node(goal) {
        return Err(format!("Goal node {} does not exist in graph", goal));
    }

    if verbose {
        println!("🎯 Starting Dijkstra's Algorithm:");
        println!("   Source: {}, Goal: {}", start, goal);
        println!("   Graph has {} nodes", graph.nodes().len());
    }

    // Handle trivial case
    if start == goal {
        if verbose {
            println!("✅ Trivial case: start == goal");
        }
        return Ok(PathResult::new(vec![start], 0.0, 1));
    }

    // Algorithm state
    let mut priority_queue = BinaryHeap::<DijkstraItem>::new();
    let mut distances: HashMap<NodeId, Weight> = HashMap::new();
    let mut predecessors: HashMap<NodeId, NodeId> = HashMap::new();
    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut nodes_explored = 0;

    // Initialize: set start distance to 0, add to queue
    distances.insert(start, 0.0);
    priority_queue.push(DijkstraItem::new(start, 0.0));

    if verbose {
        println!("\n📊 Algorithm Execution:");
        println!("Step 0: Initialize start node {} with distance 0.0", start);
    }

    let mut step = 1;

    // Main algorithm loop
    while let Some(current_item) = priority_queue.pop() {
        let current_node = current_item.node;
        let current_distance = current_item.distance.0;

        // Skip if already visited (outdated entry in priority queue)
        if visited.contains(&current_node) {
            continue;
        }

        // Mark as visited - distance is now final
        visited.insert(current_node);
        nodes_explored += 1;

        if verbose {
            println!(
                "Step {}: Visit node {} (distance: {:.1})",
                step, current_node, current_distance
            );
        }

        // Check if we reached the goal
        if current_node == goal {
            if verbose {
                println!("🎉 Goal reached! Reconstructing path...");
            }
            break;
        }

        // Relax all neighbors
        for &(neighbor, edge_weight) in graph.neighbors(current_node) {
            // Skip already visited neighbors
            if visited.contains(&neighbor) {
                continue;
            }

            let new_distance = current_distance + edge_weight;
            let should_update = match distances.get(&neighbor) {
                Some(&existing_distance) => new_distance < existing_distance,
                None => true,
            };

            if should_update {
                let old_distance = distances.get(&neighbor).copied().unwrap_or(f64::INFINITY);
                distances.insert(neighbor, new_distance);
                predecessors.insert(neighbor, current_node);
                priority_queue.push(DijkstraItem::new(neighbor, new_distance));

                if verbose {
                    if old_distance == f64::INFINITY {
                        println!(
                            "   → Discover neighbor {} with distance {:.1}",
                            neighbor, new_distance
                        );
                    } else {
                        println!(
                            "   → Relax neighbor {} ({:.1} → {:.1})",
                            neighbor, old_distance, new_distance
                        );
                    }
                }
            } else if verbose {
                println!(
                    "   → Skip neighbor {} (current: {:.1}, via this path: {:.1})",
                    neighbor,
                    distances.get(&neighbor).unwrap_or(&f64::INFINITY),
                    new_distance
                );
            }
        }

        step += 1;
    }

    // Check if goal was reached
    if !visited.contains(&goal) {
        if verbose {
            println!("❌ No path exists from {} to {}", start, goal);
        }
        return Ok(PathResult::no_path(nodes_explored));
    }

    // Reconstruct path from predecessors
    let mut path = Vec::new();
    let mut current = goal;
    
    while current != start {
        path.push(current);
        match predecessors.get(&current) {
            Some(&prev) => current = prev,
            None => return Err("Path reconstruction failed - no predecessor found".to_string()),
        }
    }
    path.push(start);
    path.reverse();

    let final_cost = distances.get(&goal).copied().unwrap_or(f64::INFINITY);

    if verbose {
        println!("\n✅ Path found!");
        println!("   Path: {:?}", path);
        println!("   Cost: {:.1}", final_cost);
        println!("   Nodes explored: {}", nodes_explored);
    }

    Ok(PathResult::new(path, final_cost, nodes_explored))
}

/// Simplified Dijkstra implementation for comparison
pub fn dijkstra_simple(graph: &SimpleGraph, start: NodeId, goal: NodeId) -> Result<PathResult, String> {
    dijkstra_with_explanation(graph, start, goal, false)
}

/// Exercise 1: Basic Dijkstra on Simple Graph
fn exercise_1_basic_dijkstra() {
    println!("🔧 Exercise 1: Basic Dijkstra Implementation");

    // Create a simple diamond-shaped graph:
    //     0
    //   /   \\
    //  2     6
    // 1       2
    //  \\   /
    //    3
    //    1
    //    3
    let mut graph = SimpleGraph::new();
    graph.add_edge(0, 1, 2.0);
    graph.add_edge(0, 2, 6.0);
    graph.add_edge(1, 3, 3.0);
    graph.add_edge(2, 3, 1.0);

    println!("\nGraph structure:");
    println!("  0 --2--> 1 --3--> 3");
    println!("  |                 ^");
    println!("  6                 |");
    println!("  v                 1");
    println!("  2 --------------->");

    // TODO: Run Dijkstra from node 0 to node 3
    println!("\n🎯 Finding shortest path from 0 to 3:");
    
    match dijkstra_with_explanation(&graph, 0, 3, true) {
        Ok(result) => {
            println!("\nResult Summary:");
            println!("  Shortest path: {:?}", result.path);
            println!("  Total cost: {:.1}", result.cost);
            println!("  Nodes explored: {}", result.nodes_explored);
            
            // Expected: path [0, 1, 3] with cost 5.0
            assert_eq!(result.path, vec![0, 1, 3]);
            assert_eq!(result.cost, 5.0);
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("✅ Exercise 1 completed!");
}

/// Exercise 2: Comparative Analysis
fn exercise_2_path_comparison() {
    println!("\n📊 Exercise 2: Path Comparison Analysis");

    // Create graph with multiple paths between nodes
    let mut graph = SimpleGraph::new();
    
    // Multiple paths from 0 to 4:
    // Path 1: 0 -> 1 -> 4 (cost: 3 + 2 = 5)
    // Path 2: 0 -> 2 -> 3 -> 4 (cost: 1 + 1 + 1 = 3) <- optimal
    // Path 3: 0 -> 2 -> 4 (cost: 1 + 8 = 9)
    graph.add_edge(0, 1, 3.0);
    graph.add_edge(0, 2, 1.0);
    graph.add_edge(1, 4, 2.0);
    graph.add_edge(2, 3, 1.0);
    graph.add_edge(2, 4, 8.0);
    graph.add_edge(3, 4, 1.0);

    println!("\nMulti-path graph:");
    println!("     3");
    println!("  0 --> 1");
    println!("  |     |");
    println!("  1     2");
    println!("  v     v");
    println!("  2 --> 4");
    println!("  | 1 / ^");
    println!("  1   8 |");
    println!("  v /   1");
    println!("  3 -----");

    // TODO: Test different start-goal combinations
    let test_cases = vec![(0, 4), (0, 3), (1, 3)];

    for (start, goal) in test_cases {
        println!("\n🎯 Path from {} to {}:", start, goal);
        
        match dijkstra_simple(&graph, start, goal) {
            Ok(result) => {
                if result.path.is_empty() {
                    println!("  No path exists");
                } else {
                    println!("  Path: {:?}", result.path);
                    println!("  Cost: {:.1}", result.cost);
                    println!("  Nodes explored: {}", result.nodes_explored);
                }
            }
            Err(e) => println!("  Error: {}", e),
        }
    }

    println!("✅ Exercise 2 completed!");
}

/// Exercise 3: Edge Cases and Error Handling
fn exercise_3_edge_cases() {
    println!("\n🚨 Exercise 3: Edge Cases and Error Handling");

    let mut graph = SimpleGraph::new();
    graph.add_edge(0, 1, 1.0);
    graph.add_edge(1, 2, 1.0);
    // Node 3 is isolated

    let test_cases = vec![
        (0, 0, "Same start and goal"),
        (0, 2, "Normal case"),
        (0, 3, "No path exists"),
        (0, 99, "Invalid goal node"),
        (99, 0, "Invalid start node"),
    ];

    for (start, goal, description) in test_cases {
        println!("\n📋 Test: {} ({} -> {})", description, start, goal);
        
        match dijkstra_simple(&graph, start, goal) {
            Ok(result) => {
                if result.path.is_empty() {
                    println!("  Result: No path found");
                } else {
                    println!("  Result: Path {:?}, cost {:.1}", result.path, result.cost);
                }
            }
            Err(e) => {
                println!("  Result: Error - {}", e);
            }
        }
    }

    println!("✅ Exercise 3 completed!");
}

/// Exercise 4: Performance Analysis
fn exercise_4_performance_analysis() {
    println!("\n⚡ Exercise 4: Performance Analysis");

    // Create larger graphs to test performance
    let sizes = vec![10, 50, 100];

    for size in sizes {
        let mut graph = SimpleGraph::new();
        
        // Create a grid-like graph: each node connects to next few nodes
        for i in 0..size {
            for j in 1..=3 {
                if i + j < size {
                    graph.add_edge(i, i + j, j as f64);
                }
            }
        }

        let start_time = std::time::Instant::now();
        
        match dijkstra_simple(&graph, 0, size - 1) {
            Ok(result) => {
                let elapsed = start_time.elapsed();
                println!("Graph size {}: Path length {}, Time {:?}", 
                    size, result.path.len(), elapsed);
            }
            Err(e) => println!("Graph size {}: Error - {}", size, e),
        }
    }

    println!("✅ Performance analysis completed!");
}

/// Advanced Exercise: Algorithm Invariants and Correctness
fn advanced_exercise_algorithm_correctness() {
    println!("\n🔍 Advanced Exercise: Algorithm Correctness");

    let mut graph = SimpleGraph::new();
    
    // Create a graph where greedy choice might seem wrong
    // But Dijkstra still finds optimal path
    graph.add_edge(0, 1, 10.0);
    graph.add_edge(0, 2, 1.0);
    graph.add_edge(2, 1, 1.0);

    println!("\nCorrectness test graph:");
    println!("  0 --10--> 1");
    println!("  |         ^");
    println!("  1         |");
    println!("  v         1");
    println!("  2 --------");

    println!("\n🔬 Analyzing algorithm correctness:");

    match dijkstra_with_explanation(&graph, 0, 1, true) {
        Ok(result) => {
            println!("\n📈 Correctness Analysis:");
            println!("  • Direct path 0→1: cost 10.0");
            println!("  • Indirect path 0→2→1: cost 1.0 + 1.0 = 2.0");
            println!("  • Algorithm chose: {:?} with cost {:.1}", result.path, result.cost);
            println!("  • Optimal? {}", if result.cost == 2.0 { "YES ✅" } else { "NO ❌" });
            
            // Verify optimality
            assert_eq!(result.path, vec![0, 2, 1]);
            assert_eq!(result.cost, 2.0);
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("✅ Algorithm correctness verified!");
}

// Test module for validation
#[cfg(test)]
mod step2_tests {
    use super::*;

    #[test]
    fn test_simple_dijkstra() {
        let mut graph = SimpleGraph::new();
        graph.add_edge(0, 1, 1.0);
        graph.add_edge(1, 2, 1.0);

        let result = dijkstra_simple(&graph, 0, 2).unwrap();
        assert_eq!(result.path, vec![0, 1, 2]);
        assert_eq!(result.cost, 2.0);
    }

    #[test]
    fn test_no_path() {
        let mut graph = SimpleGraph::new();
        graph.add_node(0);
        graph.add_node(1); // Disconnected

        let result = dijkstra_simple(&graph, 0, 1).unwrap();
        assert!(result.path.is_empty());
        assert_eq!(result.cost, f64::INFINITY);
    }

    #[test]
    fn test_same_node() {
        let mut graph = SimpleGraph::new();
        graph.add_node(0);

        let result = dijkstra_simple(&graph, 0, 0).unwrap();
        assert_eq!(result.path, vec![0]);
        assert_eq!(result.cost, 0.0);
    }

    #[test]
    fn test_optimal_path_selection() {
        let mut graph = SimpleGraph::new();
        graph.add_edge(0, 1, 10.0);
        graph.add_edge(0, 2, 1.0);
        graph.add_edge(2, 1, 1.0);

        let result = dijkstra_simple(&graph, 0, 1).unwrap();
        assert_eq!(result.path, vec![0, 2, 1]);
        assert_eq!(result.cost, 2.0);
    }

    #[test]
    fn test_invalid_nodes() {
        let graph = SimpleGraph::new();
        
        assert!(dijkstra_simple(&graph, 0, 1).is_err());
        assert!(dijkstra_simple(&graph, 99, 0).is_err());
    }
}

/// Main function to run all exercises
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Mission 9 Tutorial - Step 2: Dijkstra's Algorithm Basics");
    println!("═══════════════════════════════════════════════════════════════");
    
    exercise_1_basic_dijkstra();
    exercise_2_path_comparison();
    exercise_3_edge_cases();
    exercise_4_performance_analysis();
    advanced_exercise_algorithm_correctness();

    println!("\n🎉 Step 2 Complete!");
    println!("📚 Key Takeaways:");
    println!("  • Dijkstra guarantees shortest paths with non-negative weights");
    println!("  • Priority queue ensures optimal node processing order");
    println!("  • Relaxation updates distances when shorter paths are found");
    println!("  • Path reconstruction uses predecessor tracking");
    println!("  • Time complexity: O((V + E) log V)");
    
    println!("\n➡️  Next: cargo run --example step3_astar_implementation");

    Ok(())
}