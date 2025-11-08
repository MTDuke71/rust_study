//! # Kruskal's Minimum Spanning Tree Algorithm
//!
//! This example demonstrates how Union-Find is used in Kruskal's algorithm to find
//! the Minimum Spanning Tree (MST) of a weighted graph. Union-Find is essential
//! for detecting cycles during edge addition.
//!
//! ## Algorithm Overview
//!
//! 1. Sort all edges by weight (ascending)
//! 2. Initialize Union-Find with all vertices
//! 3. For each edge (u, v, weight) in sorted order:
//!    - If u and v are not connected, add edge to MST and union them
//!    - If u and v are already connected, skip (would create cycle)
//! 4. Stop when MST has n-1 edges (for n vertices)
//!
//! ## Time Complexity
//!
//! - O(E log E) for sorting edges
//! - O(E α(V)) for Union-Find operations
//! - **Total: O(E log E)** where E = edges, V = vertices

use mission10::UnionFind;
use std::cmp::Ordering;

/// Represents a weighted edge in a graph
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge {
    pub u: usize,      // First vertex
    pub v: usize,      // Second vertex  
    pub weight: i32,   // Edge weight
}

impl Edge {
    pub fn new(u: usize, v: usize, weight: i32) -> Self {
        Self { u, v, weight }
    }
}

impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Result of Kruskal's MST algorithm
#[derive(Debug)]
pub struct MSTResult {
    pub edges: Vec<Edge>,           // Edges in the MST
    pub total_weight: i32,          // Total weight of MST
    pub vertices_connected: usize,  // Number of vertices in MST
}

impl MSTResult {
    /// Check if the result represents a valid spanning tree
    pub fn is_spanning_tree(&self, num_vertices: usize) -> bool {
        self.edges.len() == num_vertices.saturating_sub(1) && 
        self.vertices_connected == num_vertices
    }

    /// Get the MST as a formatted string for display
    pub fn format_tree(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("MST with {} edges, total weight: {}\n", 
                                self.edges.len(), self.total_weight));
        
        for (i, edge) in self.edges.iter().enumerate() {
            result.push_str(&format!("  {}: ({}, {}) weight {}\n", 
                           i + 1, edge.u, edge.v, edge.weight));
        }
        
        result
    }
}

/// Finds the Minimum Spanning Tree using Kruskal's algorithm
///
/// # Arguments
///
/// * `vertices` - Number of vertices in the graph (labeled 0 to vertices-1)
/// * `edges` - Vector of weighted edges
///
/// # Returns
///
/// * `Ok(MSTResult)` - The MST with edges and total weight
/// * `Err(String)` - If the graph is not connected or has invalid vertices
///
/// # Example
///
/// ```
/// use mission10::examples::kruskal_mst::{Edge, kruskals_mst};
///
/// let edges = vec![
///     Edge::new(0, 1, 4),
///     Edge::new(0, 2, 3),
///     Edge::new(1, 2, 1),
///     Edge::new(1, 3, 2),
///     Edge::new(2, 3, 4),
/// ];
///
/// let mst = kruskals_mst(4, edges).unwrap();
/// assert_eq!(mst.total_weight, 6); // Edges: (1,2,1), (1,3,2), (0,2,3)
/// assert!(mst.is_spanning_tree(4));
/// ```
pub fn kruskals_mst(vertices: usize, mut edges: Vec<Edge>) -> Result<MSTResult, String> {
    if vertices == 0 {
        return Ok(MSTResult {
            edges: vec![],
            total_weight: 0,
            vertices_connected: 0,
        });
    }

    // Validate that all edges reference valid vertices
    for edge in &edges {
        if edge.u >= vertices || edge.v >= vertices {
            return Err(format!("Edge ({}, {}) references invalid vertex (max: {})", 
                             edge.u, edge.v, vertices - 1));
        }
    }

    // Step 1: Sort edges by weight (ascending)
    edges.sort();

    // Step 2: Initialize Union-Find structure
    let mut uf = UnionFind::new(vertices);
    
    // Step 3: Process edges in order, adding to MST if no cycle
    let mut mst_edges = Vec::new();
    let mut total_weight = 0;

    for edge in edges {
        // Check if adding this edge would create a cycle
        if uf.union(edge.u, edge.v).map_err(|e| format!("Union-Find error: {}", e))? {
            // No cycle - add edge to MST
            mst_edges.push(edge);
            total_weight += edge.weight;
            
            // Early termination: MST complete when we have n-1 edges
            if mst_edges.len() == vertices - 1 {
                break;
            }
        }
        // If union returned false, edge would create cycle - skip it
    }

    // Check if we found a spanning tree (graph must be connected)
    let components = uf.count();
    if components > 1 {
        return Err(format!("Graph is not connected - {} components found, MST requires 1", 
                          components));
    }

    Ok(MSTResult {
        edges: mst_edges,
        total_weight,
        vertices_connected: vertices,
    })
}

/// Example: Small graph MST
fn example_small_graph() -> Result<(), String> {
    println!("=== Small Graph MST Example ===");
    
    // Create a simple 4-vertex graph
    //     4
    //  0 --- 1
    //  |  1 /|
    //  |   / | 2
    //  |3 /  |
    //  | /   |
    //  2 --- 3
    //     4
    
    let edges = vec![
        Edge::new(0, 1, 4),
        Edge::new(0, 2, 3),  
        Edge::new(1, 2, 1),  // This will be selected (minimum weight)
        Edge::new(1, 3, 2),  // This will be selected
        Edge::new(2, 3, 4),  // This will be skipped (creates cycle)
    ];

    let mst = kruskals_mst(4, edges)?;
    
    println!("Input graph has 5 edges");
    println!("{}", mst.format_tree());
    println!("Edges selected: (1,2) weight 1, (1,3) weight 2, (0,2) weight 3");
    println!("Edge (2,3) weight 4 was skipped to avoid cycle");
    println!("Is valid spanning tree: {}\n", mst.is_spanning_tree(4));
    
    Ok(())
}

/// Example: Larger graph demonstrating algorithm efficiency
fn example_larger_graph() -> Result<(), String> {
    println!("=== Larger Graph MST Example ===");
    
    // Create a 6-vertex complete graph with random weights
    let mut edges = Vec::new();
    let weights = [
        [0, 5, 3, 4, 6, 8],
        [5, 0, 2, 7, 1, 4],
        [3, 2, 0, 6, 9, 2],
        [4, 7, 6, 0, 3, 1],
        [6, 1, 9, 3, 0, 5],
        [8, 4, 2, 1, 5, 0],
    ];
    
    // Generate all edges from adjacency matrix
    for i in 0..6 {
        for j in (i + 1)..6 {
            edges.push(Edge::new(i, j, weights[i][j]));
        }
    }
    
    println!("Complete graph with 6 vertices and {} edges", edges.len());
    
    let mst = kruskals_mst(6, edges)?;
    
    println!("{}", mst.format_tree());
    println!("Algorithm processed {} edges but only selected {} for MST", 
             15, mst.edges.len()); // 6 choose 2 = 15 total edges
    println!("Efficiency: {}% of edges were actually needed\n", 
             (mst.edges.len() * 100) / 15);
    
    Ok(())
}

/// Example: Disconnected graph (should fail)
fn example_disconnected_graph() {
    println!("=== Disconnected Graph Example (Error Case) ===");
    
    // Create two separate triangles - not connected
    let edges = vec![
        Edge::new(0, 1, 1),
        Edge::new(1, 2, 2), 
        Edge::new(2, 0, 3),  // Triangle 1: vertices 0,1,2
        Edge::new(3, 4, 1),
        Edge::new(4, 5, 2),  // Triangle 2: vertices 3,4,5 (disconnected)
    ];

    match kruskals_mst(6, edges) {
        Ok(_) => println!("ERROR: Should have failed for disconnected graph!"),
        Err(msg) => println!("Correctly detected disconnected graph: {}\n", msg),
    }
}

/// Demonstrate cycle detection during MST construction
fn example_cycle_detection() -> Result<(), String> {
    println!("=== Cycle Detection Example ===");
    
    // Create edges that would form cycles if all were added
    let edges = vec![
        Edge::new(0, 1, 1),  // Will be added (lowest weight)
        Edge::new(1, 2, 2),  // Will be added
        Edge::new(0, 2, 5),  // Will be SKIPPED (creates cycle with 0-1-2)
        Edge::new(2, 3, 3),  // Will be added
        Edge::new(0, 3, 4),  // Will be SKIPPED (creates cycle)
        Edge::new(1, 3, 6),  // Will be SKIPPED (creates cycle)
    ];

    println!("Processing edges in weight order:");
    let mut uf = UnionFind::new(4);
    let mut mst_edges = Vec::new();
    let mut sorted_edges = edges.clone();
    sorted_edges.sort();
    
    for edge in sorted_edges {
        print!("Edge ({}, {}) weight {}: ", edge.u, edge.v, edge.weight);
        
        if uf.connected(edge.u, edge.v)? {
            println!("SKIPPED (would create cycle)");
        } else {
            uf.union(edge.u, edge.v)?;
            mst_edges.push(edge);
            println!("ADDED to MST");
        }
    }
    
    println!("\nFinal MST: {} edges selected out of {} total", 
             mst_edges.len(), edges.len());
    
    Ok(())
}

fn main() -> Result<(), String> {
    println!("🌳 Kruskal's Minimum Spanning Tree Algorithm with Union-Find\n");
    
    example_small_graph()?;
    example_larger_graph()?; 
    example_disconnected_graph();
    example_cycle_detection()?;
    
    println!("✅ All examples completed successfully!");
    println!("\n💡 Key Insights:");
    println!("• Union-Find detects cycles in O(α(n)) time per edge");
    println!("• Kruskal's algorithm is optimal for sparse graphs");
    println!("• Edge sorting dominates the time complexity: O(E log E)");
    println!("• MST always has exactly n-1 edges for n vertices");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_graph_mst() {
        let edges = vec![
            Edge::new(0, 1, 4),
            Edge::new(0, 2, 3),
            Edge::new(1, 2, 1),
            Edge::new(1, 3, 2),
            Edge::new(2, 3, 4),
        ];

        let mst = kruskals_mst(4, edges).unwrap();
        assert_eq!(mst.edges.len(), 3);
        assert_eq!(mst.total_weight, 6);
        assert!(mst.is_spanning_tree(4));
    }

    #[test]
    fn test_single_vertex() {
        let mst = kruskals_mst(1, vec![]).unwrap();
        assert_eq!(mst.edges.len(), 0);
        assert_eq!(mst.total_weight, 0);
        assert!(mst.is_spanning_tree(1));
    }

    #[test]
    fn test_disconnected_graph() {
        let edges = vec![
            Edge::new(0, 1, 1),
            Edge::new(2, 3, 1), // Disconnected from 0-1
        ];

        assert!(kruskals_mst(4, edges).is_err());
    }

    #[test]
    fn test_invalid_vertex() {
        let edges = vec![Edge::new(0, 5, 1)]; // Vertex 5 doesn't exist in 4-vertex graph
        assert!(kruskals_mst(4, edges).is_err());
    }

    #[test]
    fn test_edge_ordering() {
        let mut e1 = Edge::new(0, 1, 5);
        let mut e2 = Edge::new(2, 3, 3);
        let mut e3 = Edge::new(4, 5, 5);
        
        assert!(e2 < e1);  // Lower weight comes first
        assert!(e1 == e3); // Same weight are equal
    }
}