//! Step 4: Algorithm Foundation
//!
//! This step builds the foundation for graph algorithms by introducing visited tracking,
//! path reconstruction, and algorithm result structures.
//!
//! ## Learning Objectives
//! - Understand visited tracking for graph algorithms
//! - Learn path reconstruction techniques
//! - Explore algorithm result structures
//! - Build foundation for DFS and BFS algorithms
//!
//! ## Alignment with Mission 7
//! This step directly implements **REQ-4: Algorithm Foundation** by teaching
//! the infrastructure needed for graph traversal algorithms.
//!
//! ## Calendar Alignment
//! **Day 4 (Oct 11)**: Graph Algorithms Foundation
//! - Prepare for BFS/DFS: visited tracking, queue/stack
//! - Algorithm result structures and utilities

use mission7::{Graph, GraphType, NodeId};
use mission7_tut::tutorial_utils;
use std::collections::{HashSet, VecDeque};

fn main() {
    println!("=== Step 4: Algorithm Foundation ===\n");
    
    // 1. Visited Tracking
    visited_tracking();
    
    // 2. Path Reconstruction
    path_reconstruction();
    
    // 3. Algorithm Result Structures
    algorithm_result_structures();
    
    // 4. Queue and Stack Infrastructure
    queue_stack_infrastructure();
    
    // 5. Algorithm Utilities
    algorithm_utilities();
    
    // 6. Foundation for DFS and BFS
    foundation_for_traversal();
    
    println!("\n=== Step 4 Complete ===");
    println!("Next: Step 5 - DFS Implementation");
}

fn visited_tracking() {
    println!("1. Visited Tracking");
    println!("==================");
    
    println!("Why do we need visited tracking?");
    println!("  • Prevent infinite loops in cyclic graphs");
    println!("  • Ensure each node is processed only once");
    println!("  • Track algorithm progress and completion");
    println!();
    
    // Create a graph with cycles
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    
    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(c, d);
    graph.add_edge(d, a); // Creates a cycle
    
    println!("Example graph with cycle: A -> B -> C -> D -> A");
    println!();
    
    // Demonstrate visited tracking
    println!("Visited Tracking Example:");
    let mut visited = HashSet::new();
    let mut path = Vec::new();
    
    // Simulate traversal with visited tracking
    let start = a;
    let mut current = start;
    
    println!("Starting traversal from node A:");
    for step in 0..8 { // Limit steps to prevent infinite loop
        if visited.insert(current) {
            path.push(current);
            let node_name = graph.get_node(current).unwrap();
            println!("  Step {}: Visit {} (new)", step + 1, node_name);
            
            // Move to next unvisited neighbor
            let mut next_node = None;
            for &neighbor in graph.neighbors(current) {
                if !visited.contains(&neighbor) {
                    next_node = Some(neighbor);
                    break;
                }
            }
            
            if let Some(next) = next_node {
                current = next;
            } else {
                println!("  No more unvisited neighbors, traversal complete");
                break;
            }
        } else {
            let node_name = graph.get_node(current).unwrap();
            println!("  Step {}: Visit {} (already visited - cycle detected!)", step + 1, node_name);
            break;
        }
    }
    
    println!();
    println!("Final visited set: {:?}", visited);
    println!("Traversal path: {:?}", path);
    println!();
    
    // Show different visited tracking approaches
    println!("Visited Tracking Approaches:");
    println!("  1. HashSet<NodeId> - O(1) lookup, O(V) space");
    println!("  2. Vec<bool> - O(1) lookup, O(V) space, faster for small graphs");
    println!("  3. BitSet - O(1) lookup, O(V/8) space, most memory efficient");
    println!();
}

fn path_reconstruction() {
    println!("2. Path Reconstruction");
    println!("=====================");
    
    println!("Path reconstruction is used to:");
    println!("  • Find the actual path between two nodes");
    println!("  • Trace back from destination to source");
    println!("  • Build parent-child relationships");
    println!();
    
    // Create a graph for path finding
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    
    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(c, d);
    graph.add_edge(d, e);
    graph.add_edge(a, c); // Shortcut
    
    println!("Graph for path reconstruction:");
    println!("  A -> B -> C -> D -> E");
    println!("  A -> C (shortcut)");
    println!();
    
    // Simulate path reconstruction
    println!("Path Reconstruction Example:");
    
    // Parent array to track where each node came from
    let mut parents = vec![None; graph.node_count()];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    let start = a;
    let end = e;
    
    queue.push_back(start);
    visited.insert(start);
    
    println!("Finding path from A to E:");
    println!();
    
    // Simulate BFS traversal
    while let Some(current) = queue.pop_front() {
        let current_name = graph.get_node(current).unwrap();
        println!("  Processing node: {}", current_name);
        
        if current == end {
            println!("  Found destination: {}", current_name);
            break;
        }
        
        for &neighbor in graph.neighbors(current) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                parents[neighbor] = Some(current);
                queue.push_back(neighbor);
                
                let neighbor_name = graph.get_node(neighbor).unwrap();
                println!("    Add {} to queue (parent: {})", neighbor_name, current_name);
            }
        }
    }
    
    println!();
    
    // Reconstruct path
    println!("Path Reconstruction:");
    let mut path = Vec::new();
    let mut current = end;
    
    while let Some(parent) = parents[current] {
        path.push(current);
        current = parent;
    }
    path.push(start);
    path.reverse();
    
    println!("  Path from A to E:");
    for (i, &node_id) in path.iter().enumerate() {
        let node_name = graph.get_node(node_id).unwrap();
        if i == 0 {
            print!("    {}", node_name);
        } else {
            print!(" -> {}", node_name);
        }
    }
    println!();
    println!("  Path length: {} edges", path.len() - 1);
    println!();
    
    // Show parent relationships
    println!("Parent Relationships:");
    for (i, parent) in parents.iter().enumerate() {
        if let Some(parent_id) = parent {
            let node_name = graph.get_node(i).unwrap();
            let parent_name = graph.get_node(*parent_id).unwrap();
            println!("  {} -> {}", node_name, parent_name);
        }
    }
    println!();
}

fn algorithm_result_structures() {
    println!("3. Algorithm Result Structures");
    println!("=============================");
    
    println!("Algorithm result structures store:");
    println!("  • The path taken during traversal");
    println!("  • Set of visited nodes");
    println!("  • Additional algorithm-specific data");
    println!();
    
    // Define result structures
    #[derive(Debug)]
    struct TraversalResult {
        path: Vec<NodeId>,
        visited: HashSet<NodeId>,
        steps: usize,
    }
    
    #[derive(Debug)]
    struct PathfindingResult {
        path: Vec<NodeId>,
        distance: usize,
        visited: HashSet<NodeId>,
        parents: Vec<Option<NodeId>>,
    }
    
    #[derive(Debug)]
    struct ComponentResult {
        components: Vec<Vec<NodeId>>,
        component_count: usize,
        largest_component_size: usize,
    }
    
    println!("Example Result Structures:");
    println!();
    
    // Create a graph for demonstration
    let mut graph = Graph::new_undirected();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    
    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(d, e);
    
    // Simulate traversal result
    let traversal_result = TraversalResult {
        path: vec![a, b, c],
        visited: [a, b, c].iter().cloned().collect(),
        steps: 3,
    };
    
    println!("Traversal Result:");
    println!("  {:?}", traversal_result);
    println!();
    
    // Simulate pathfinding result
    let pathfinding_result = PathfindingResult {
        path: vec![a, b, c],
        distance: 2,
        visited: [a, b, c].iter().cloned().collect(),
        parents: vec![None, Some(a), Some(b), None, None],
    };
    
    println!("Pathfinding Result:");
    println!("  {:?}", pathfinding_result);
    println!();
    
    // Simulate component result
    let component_result = ComponentResult {
        components: vec![vec![a, b, c], vec![d, e]],
        component_count: 2,
        largest_component_size: 3,
    };
    
    println!("Component Result:");
    println!("  {:?}", component_result);
    println!();
    
    // Show how to use result structures
    println!("Using Result Structures:");
    println!("  Traversal path length: {}", traversal_result.path.len());
    println!("  Nodes visited: {}", traversal_result.visited.len());
    println!("  Pathfinding distance: {}", pathfinding_result.distance);
    println!("  Number of components: {}", component_result.component_count);
    println!("  Largest component size: {}", component_result.largest_component_size);
    println!();
}

fn queue_stack_infrastructure() {
    println!("4. Queue and Stack Infrastructure");
    println!("===============================");
    
    println!("Queue and Stack are fundamental for graph algorithms:");
    println!("  • Queue (FIFO) - used for BFS (breadth-first search)");
    println!("  • Stack (LIFO) - used for DFS (depth-first search)");
    println!("  • VecDeque - efficient double-ended queue");
    println!("  • Vec - can be used as stack with push/pop");
    println!();
    
    // Demonstrate queue operations
    println!("Queue Operations (FIFO - First In, First Out):");
    let mut queue = VecDeque::new();
    
    queue.push_back("A");
    queue.push_back("B");
    queue.push_back("C");
    
    println!("  Queue after adding A, B, C: {:?}", queue);
    
    while let Some(item) = queue.pop_front() {
        println!("  Pop: {} (queue: {:?})", item, queue);
    }
    println!();
    
    // Demonstrate stack operations
    println!("Stack Operations (LIFO - Last In, First Out):");
    let mut stack = Vec::new();
    
    stack.push("A");
    stack.push("B");
    stack.push("C");
    
    println!("  Stack after adding A, B, C: {:?}", stack);
    
    while let Some(item) = stack.pop() {
        println!("  Pop: {} (stack: {:?})", item, stack);
    }
    println!();
    
    // Show how they're used in graph algorithms
    println!("Graph Algorithm Usage:");
    println!("  BFS (Breadth-First Search):");
    println!("    • Use queue to process nodes level by level");
    println!("    • First node at distance 1, then distance 2, etc.");
    println!("    • Guarantees shortest path in unweighted graphs");
    println!();
    
    println!("  DFS (Depth-First Search):");
    println!("    • Use stack to go deep before going wide");
    println!("    • Explore one path completely before backtracking");
    println!("    • Good for cycle detection and path finding");
    println!();
    
    // Demonstrate with a simple graph
    let mut graph = Graph::new_directed();
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
    println!("  B   C");
    println!("  |   |");
    println!("  D---D");
    println!();
    
    // Simulate BFS with queue
    println!("BFS Simulation (using queue):");
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    queue.push_back(a);
    visited.insert(a);
    
    while let Some(current) = queue.pop_front() {
        let current_name = graph.get_node(current).unwrap();
        println!("  Process: {}", current_name);
        
        for &neighbor in graph.neighbors(current) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
                let neighbor_name = graph.get_node(neighbor).unwrap();
                println!("    Add to queue: {}", neighbor_name);
            }
        }
    }
    println!();
    
    // Simulate DFS with stack
    println!("DFS Simulation (using stack):");
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    
    stack.push(a);
    visited.insert(a);
    
    while let Some(current) = stack.pop() {
        let current_name = graph.get_node(current).unwrap();
        println!("  Process: {}", current_name);
        
        for &neighbor in graph.neighbors(current) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                stack.push(neighbor);
                let neighbor_name = graph.get_node(neighbor).unwrap();
                println!("    Add to stack: {}", neighbor_name);
            }
        }
    }
    println!();
}

fn algorithm_utilities() {
    println!("5. Algorithm Utilities");
    println!("=====================");
    
    println!("Common utility functions for graph algorithms:");
    println!();
    
    // Utility function to find unvisited neighbors
    fn get_unvisited_neighbors<T>(
        graph: &Graph<T>,
        node: NodeId,
        visited: &HashSet<NodeId>,
    ) -> Vec<NodeId> {
        graph.neighbors(node)
            .iter()
            .filter(|&&neighbor| !visited.contains(&neighbor))
            .cloned()
            .collect()
    }
    
    // Utility function to check if all nodes are visited
    fn all_nodes_visited<T>(graph: &Graph<T>, visited: &HashSet<NodeId>) -> bool {
        graph.nodes().all(|node| visited.contains(&node))
    }
    
    // Utility function to find next unvisited node
    fn find_next_unvisited<T>(graph: &Graph<T>, visited: &HashSet<NodeId>) -> Option<NodeId> {
        graph.nodes().find(|&node| !visited.contains(&node))
    }
    
    // Create a graph for demonstration
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    
    graph.add_edge(a, b);
    graph.add_edge(b, c);
    graph.add_edge(c, d);
    
    println!("Utility Functions Demo:");
    println!();
    
    // Test get_unvisited_neighbors
    let mut visited = HashSet::new();
    visited.insert(a);
    
    let unvisited = get_unvisited_neighbors(&graph, a, &visited);
    println!("Unvisited neighbors of A: {:?}", unvisited);
    
    let unvisited = get_unvisited_neighbors(&graph, b, &visited);
    println!("Unvisited neighbors of B: {:?}", unvisited);
    println!();
    
    // Test all_nodes_visited
    println!("All nodes visited: {}", all_nodes_visited(&graph, &visited));
    visited.insert(b);
    visited.insert(c);
    visited.insert(d);
    println!("All nodes visited: {}", all_nodes_visited(&graph, &visited));
    println!();
    
    // Test find_next_unvisited
    let mut visited = HashSet::new();
    visited.insert(a);
    visited.insert(c);
    
    if let Some(next) = find_next_unvisited(&graph, &visited) {
        let next_name = graph.get_node(next).unwrap();
        println!("Next unvisited node: {} (ID: {})", next_name, next);
    }
    println!();
    
    // Additional utility functions
    println!("Additional Utility Functions:");
    println!("  • is_connected() - check if graph is connected");
    println!("  • get_degree() - get number of neighbors");
    println!("  • has_cycle() - detect cycles in the graph");
    println!("  • get_components() - find connected components");
    println!("  • shortest_path() - find shortest path between nodes");
    println!();
}

fn foundation_for_traversal() {
    println!("6. Foundation for DFS and BFS");
    println!("============================");
    
    println!("Now we have all the building blocks for graph traversal:");
    println!("  ✅ Visited tracking (HashSet<NodeId>)");
    println!("  ✅ Path reconstruction (parent array)");
    println!("  ✅ Result structures (TraversalResult, etc.)");
    println!("  ✅ Queue/Stack infrastructure (VecDeque, Vec)");
    println!("  ✅ Utility functions (neighbor lookup, etc.)");
    println!();
    
    // Create a graph for demonstration
    let mut graph = Graph::new_directed();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    
    graph.add_edge(a, b);
    graph.add_edge(a, c);
    graph.add_edge(b, d);
    graph.add_edge(c, e);
    
    println!("Example Graph for Traversal:");
    println!("    A");
    println!("   / \\");
    println!("  B   C");
    println!("  |   |");
    println!("  D   E");
    println!();
    
    // Show the foundation components
    println!("Foundation Components:");
    println!();
    
    // 1. Visited tracking
    let mut visited = HashSet::new();
    println!("1. Visited tracking: {:?}", visited);
    
    // 2. Path reconstruction
    let mut parents = vec![None; graph.node_count()];
    println!("2. Parent array: {:?}", parents);
    
    // 3. Queue for BFS
    let mut queue = VecDeque::new();
    println!("3. Queue: {:?}", queue);
    
    // 4. Stack for DFS
    let mut stack = Vec::new();
    println!("4. Stack: {:?}", stack);
    
    // 5. Result structure
    #[derive(Debug)]
    struct TraversalResult {
        path: Vec<NodeId>,
        visited: HashSet<NodeId>,
    }
    
    let result = TraversalResult {
        path: Vec::new(),
        visited: HashSet::new(),
    };
    println!("5. Result structure: {:?}", result);
    println!();
    
    // Show how they work together
    println!("How They Work Together:");
    println!("  • Start with empty visited set and result structure");
    println!("  • Use queue/stack to manage traversal order");
    println!("  • Track parents for path reconstruction");
    println!("  • Build result as traversal progresses");
    println!("  • Return complete result when done");
    println!();
    
    println!("Ready for DFS and BFS Implementation!");
    println!("  • DFS: Use stack, go deep first");
    println!("  • BFS: Use queue, go wide first");
    println!("  • Both: Use visited tracking and path reconstruction");
    println!();
}

// Exercise: Build algorithm foundation
fn exercise() {
    println!("Exercise: Build Algorithm Foundation");
    println!("===================================");
    println!("Practice building the foundation for graph algorithms:");
    println!("  1. Create a graph with multiple paths");
    println!("  2. Implement visited tracking");
    println!("  3. Build parent array for path reconstruction");
    println!("  4. Create result structures");
    println!("  5. Test utility functions");
    println!();
    println!("This foundation will be used in the next steps for DFS and BFS!");
    println!();
}
