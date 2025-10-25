//! # Step 3: A* Search Implementation
//!
//! This tutorial covers implementing the A* pathfinding algorithm, which uses heuristics
//! to guide the search more efficiently than Dijkstra's algorithm.
//!
//! ## Learning Objectives
//! - Understand A* algorithm fundamentals: f(n) = g(n) + h(n)
//! - Implement admissible heuristics (Manhattan and Euclidean distance)
//! - Manage open and closed sets efficiently
//! - Compare A* performance with Dijkstra's algorithm
//!
//! ## Key Concepts
//! - **g(n)**: Cost from start to current node (same as Dijkstra)
//! - **h(n)**: Estimated cost from current node to goal (heuristic)
//! - **f(n)**: Total estimated cost (g(n) + h(n))
//! - **Admissible heuristic**: h(n) <= actual cost to goal
//! - **Open set**: Nodes to be evaluated
//! - **Closed set**: Nodes already evaluated

use std::collections::{BinaryHeap, HashSet, HashMap};
use std::cmp::Ordering;

/// Represents a node with its evaluation in A*
#[derive(Debug, Clone, Eq, PartialEq)]
struct AstarNode {
    id: usize,
    g: usize,      // Cost from start
    h: usize,      // Heuristic estimate to goal
    f: usize,      // Total estimated cost (g + h)
}

impl Ord for AstarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap: lower f value has higher priority
        // Use reverse for min-heap behavior
        other.f.cmp(&self.f)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for AstarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Manhattan distance heuristic
/// Useful for grid-based pathfinding where only 4-way movement is allowed
fn manhattan_distance(from: (usize, usize), to: (usize, usize)) -> usize {
    let dx = from.0.abs_diff(to.0);
    let dy = from.1.abs_diff(to.1);
    dx + dy
}

/// Euclidean distance heuristic
/// More accurate for grid-based pathfinding with diagonal movement
fn euclidean_distance(from: (usize, usize), to: (usize, usize)) -> usize {
    let dx = from.0.abs_diff(to.0);
    let dy = from.1.abs_diff(to.1);
    ((dx * dx + dy * dy) as f64).sqrt().ceil() as usize
}

/// Simple graph representation for tutorial
struct SimpleGraph {
    nodes: Vec<(usize, usize)>,  // positions
    edges: Vec<Vec<(usize, usize)>>, // adjacency with weights
}

impl SimpleGraph {
    fn new(positions: Vec<(usize, usize)>) -> Self {
        let n = positions.len();
        SimpleGraph {
            nodes: positions,
            edges: vec![Vec::new(); n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: usize) {
        if from < self.edges.len() && to < self.edges.len() {
            self.edges[from].push((to, weight));
        }
    }
}

/// A* algorithm implementation
fn astar(
    graph: &SimpleGraph,
    start: usize,
    goal: usize,
    heuristic: fn((usize, usize), (usize, usize)) -> usize,
) -> Option<Vec<usize>> {
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut g_scores = HashMap::new();
    let mut came_from = HashMap::new();

    // Initialize
    let start_h = heuristic(graph.nodes[start], graph.nodes[goal]);
    open_set.push(AstarNode {
        id: start,
        g: 0,
        h: start_h,
        f: start_h,
    });
    g_scores.insert(start, 0);

    while let Some(current) = open_set.pop() {
        if current.id == goal {
            // Reconstruct path
            let mut path = vec![goal];
            let mut node_id = goal;
            while node_id != start {
                node_id = came_from[&node_id];
                path.push(node_id);
            }
            path.reverse();
            return Some(path);
        }

        closed_set.insert(current.id);

        // Explore neighbors
        for &(neighbor, weight) in &graph.edges[current.id] {
            if closed_set.contains(&neighbor) {
                continue;
            }

            let tentative_g = current.g + weight;

            if !g_scores.contains_key(&neighbor) || tentative_g < g_scores[&neighbor] {
                came_from.insert(neighbor, current.id);
                g_scores.insert(neighbor, tentative_g);

                let h = heuristic(graph.nodes[neighbor], graph.nodes[goal]);
                let f = tentative_g + h;

                open_set.push(AstarNode {
                    id: neighbor,
                    g: tentative_g,
                    h,
                    f,
                });
            }
        }
    }

    None
}

// ============================================================================
// EXERCISES
// ============================================================================

fn main() {
    println!("=== A* Algorithm Implementation Tutorial ===\n");

    // Exercise 1: Basic A* Search
    exercise_1_basic_astar();

    // Exercise 2: Compare heuristics
    exercise_2_heuristic_comparison();

    // Exercise 3: Analyze admissibility
    exercise_3_heuristic_admissibility();
}

/// Exercise 1: Implement basic A* search
fn exercise_1_basic_astar() {
    println!("📚 Exercise 1: Basic A* Search");
    println!("────────────────────────────\n");

    // Create a simple 4x4 grid
    let mut graph = SimpleGraph::new(vec![
        (0, 0), (0, 1), (0, 2), (0, 3),
        (1, 0), (1, 1), (1, 2), (1, 3),
        (2, 0), (2, 1), (2, 2), (2, 3),
        (3, 0), (3, 1), (3, 2), (3, 3),
    ]);

    // Add edges (simplified 4-way movement)
    for row in 0..4 {
        for col in 0..4 {
            let node = row * 4 + col;
            if col < 3 { graph.add_edge(node, node + 1, 1); }  // right
            if col > 0 { graph.add_edge(node, node - 1, 1); }  // left
            if row < 3 { graph.add_edge(node, node + 4, 1); }  // down
            if row > 0 { graph.add_edge(node, node - 4, 1); }  // up
        }
    }

    let start = 0;    // Top-left (0,0)
    let goal = 15;    // Bottom-right (3,3)

    match astar(&graph, start, goal, manhattan_distance) {
        Some(path) => {
            println!("✓ Path found with {} steps", path.len() - 1);
            println!("Path: {:?}", path);
            println!("Grid positions:");
            for node_id in &path {
                let pos = graph.nodes[*node_id];
                println!("  Node {}: ({}, {})", node_id, pos.0, pos.1);
            }
        }
        None => println!("✗ No path found"),
    }

    println!("\n✓ Exercise 1 complete: Basic A* finds shortest path\n");
}

/// Exercise 2: Compare Manhattan vs Euclidean heuristics
fn exercise_2_heuristic_comparison() {
    println!("📚 Exercise 2: Heuristic Comparison");
    println!("──────────────────────────────────\n");

    // Create a simple graph
    let mut graph = SimpleGraph::new(vec![
        (0, 0), (0, 1), (0, 2), (0, 3),
        (1, 0), (1, 1), (1, 2), (1, 3),
        (2, 0), (2, 1), (2, 2), (2, 3),
        (3, 0), (3, 1), (3, 2), (3, 3),
    ]);

    // Add edges
    for row in 0..4 {
        for col in 0..4 {
            let node = row * 4 + col;
            if col < 3 { graph.add_edge(node, node + 1, 1); }
            if col > 0 { graph.add_edge(node, node - 1, 1); }
            if row < 3 { graph.add_edge(node, node + 4, 1); }
            if row > 0 { graph.add_edge(node, node - 4, 1); }
        }
    }

    let start = 0;
    let goal = 15;

    println!("From (0,0) to (3,3):");
    
    // Manhattan heuristic
    if let Some(path_m) = astar(&graph, start, goal, manhattan_distance) {
        println!("  Manhattan: {} steps", path_m.len() - 1);
    }

    // Euclidean heuristic
    if let Some(path_e) = astar(&graph, start, goal, euclidean_distance) {
        println!("  Euclidean: {} steps", path_e.len() - 1);
    }

    println!("\n✓ Exercise 2 complete: Different heuristics can guide search\n");
}

/// Exercise 3: Understand heuristic admissibility
fn exercise_3_heuristic_admissibility() {
    println!("📚 Exercise 3: Heuristic Admissibility");
    println!("─────────────────────────────────────\n");

    let start_pos = (0, 0);
    let goal_pos = (3, 3);
    let actual_distance = 6;  // Manhattan distance in grid = 3 + 3

    println!("From (0,0) to (3,3):");
    println!("Actual distance (Manhattan): {}\n", actual_distance);

    let h_manhattan = manhattan_distance(start_pos, goal_pos);
    let h_euclidean = euclidean_distance(start_pos, goal_pos);

    println!("Heuristic estimates:");
    println!("  Manhattan: {} (≤ {}? {})", 
        h_manhattan, actual_distance, h_manhattan <= actual_distance);
    println!("  Euclidean: {} (≤ {}? {})", 
        h_euclidean, actual_distance, h_euclidean <= actual_distance);

    println!("\n✓ Exercise 3 complete: Both heuristics are admissible\n");
}

// ============================================================================
// DEBUGGING EXERCISES
// ============================================================================

#[allow(dead_code)]
/// Debug: What happens with non-admissible heuristics?
fn debug_non_admissible_heuristic() {
    println!("🐛 Debug: Non-admissible heuristics can lead to suboptimal paths");
    println!("   A* guarantees optimality only with admissible heuristics");
}

#[allow(dead_code)]
/// Debug: Understanding open vs closed sets
fn debug_open_closed_sets() {
    println!("🐛 Debug: Open set = nodes to explore, Closed set = nodes explored");
    println!("   This prevents revisiting nodes and ensures efficiency");
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astar_finds_path() {
        let mut graph = SimpleGraph::new(vec![
            (0, 0), (0, 1), (0, 2),
            (1, 0), (1, 1), (1, 2),
            (2, 0), (2, 1), (2, 2),
        ]);

        for row in 0..3 {
            for col in 0..3 {
                let node = row * 3 + col;
                if col < 2 { graph.add_edge(node, node + 1, 1); }
                if col > 0 { graph.add_edge(node, node - 1, 1); }
                if row < 2 { graph.add_edge(node, node + 3, 1); }
                if row > 0 { graph.add_edge(node, node - 3, 1); }
            }
        }

        let path = astar(&graph, 0, 8, manhattan_distance);
        assert!(path.is_some());
        assert!(path.as_ref().unwrap().len() >= 5); // At least 4 steps + 1
    }

    #[test]
    fn test_manhattan_heuristic() {
        let h = manhattan_distance((0, 0), (3, 3));
        assert_eq!(h, 6);
    }

    #[test]
    fn test_euclidean_heuristic() {
        let h = euclidean_distance((0, 0), (3, 3));
        assert!(h <= 6);
    }
}
