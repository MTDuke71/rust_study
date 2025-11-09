//! # Cycle Detection in Undirected Graphs
//!
//! This example demonstrates two approaches to cycle detection in undirected graphs:
//! 1. **Union-Find approach**: O(E α(V)) - efficient for online edge additions
//! 2. **DFS approach**: O(V + E) - traditional graph traversal method
//!
//! Union-Find is particularly useful for dynamic graphs where edges are added
//! incrementally and you need to detect cycles at each step.

use mission10::UnionFind;
use std::collections::{HashMap, HashSet};

/// Represents an undirected graph using adjacency lists
#[derive(Debug, Clone)]
pub struct Graph {
    pub vertices: usize,
    pub adj_list: HashMap<usize, Vec<usize>>,
    pub edges: Vec<(usize, usize)>,
}

impl Graph {
    /// Create a new graph with specified number of vertices
    pub fn new(vertices: usize) -> Self {
        Self {
            vertices,
            adj_list: HashMap::new(),
            edges: Vec::new(),
        }
    }

    /// Add an undirected edge between vertices u and v
    pub fn add_edge(&mut self, u: usize, v: usize) -> Result<(), String> {
        if u >= self.vertices || v >= self.vertices {
            return Err(format!("Invalid vertices: ({}, {}) for graph with {} vertices", 
                             u, v, self.vertices));
        }

        if u == v {
            return Err("Self-loops not supported in undirected graphs".to_string());
        }

        // Add to adjacency list (undirected = both directions)
        self.adj_list.entry(u).or_default().push(v);
        self.adj_list.entry(v).or_default().push(u);
        
        // Store edge for reference
        self.edges.push((u, v));
        
        Ok(())
    }

    /// Get neighbors of a vertex
    pub fn neighbors(&self, vertex: usize) -> Vec<usize> {
        self.adj_list.get(&vertex).cloned().unwrap_or_default()
    }

    /// Print graph representation
    pub fn print(&self) {
        println!("Graph with {} vertices and {} edges:", self.vertices, self.edges.len());
        for vertex in 0..self.vertices {
            let neighbors = self.neighbors(vertex);
            println!("  {}: {:?}", vertex, neighbors);
        }
    }
}

/// Detect cycles using Union-Find approach
///
/// This approach processes edges one by one. For each edge (u, v):
/// - If u and v are already connected, adding this edge creates a cycle
/// - Otherwise, union u and v
///
/// **Advantages:**
/// - Online algorithm: can detect cycles as edges are added
/// - Efficient for sparse graphs
/// - Good for dynamic connectivity problems
///
/// **Time Complexity:** O(E α(V)) where α is inverse Ackermann function
pub fn detect_cycle_union_find(graph: &Graph) -> Result<Option<(usize, usize)>, String> {
    let mut uf = UnionFind::new(graph.vertices);
    
    for &(u, v) in &graph.edges {
        // Check if u and v are already connected
        if uf.connected(u, v)? {
            return Ok(Some((u, v))); // Found cycle-creating edge
        }
        
        // Union the vertices
        uf.union(u, v)?;
    }
    
    Ok(None) // No cycle found
}

/// Detect cycles using DFS approach
///
/// This approach uses depth-first search with a visited set and parent tracking.
/// A cycle exists if we encounter a visited vertex that is not our immediate parent.
///
/// **Advantages:**
/// - Can find the actual cycle path
/// - Works well for dense graphs
/// - Classic graph algorithm
///
/// **Time Complexity:** O(V + E)
pub fn detect_cycle_dfs(graph: &Graph) -> Option<Vec<usize>> {
    let mut visited = HashSet::new();
    let mut parent: HashMap<usize, usize> = HashMap::new();
    
    // Try starting DFS from each unvisited vertex
    for start in 0..graph.vertices {
        if !visited.contains(&start) {
            if let Some(cycle) = dfs_visit(graph, start, &mut visited, &mut parent, None) {
                return Some(cycle);
            }
        }
    }
    
    None
}

/// DFS helper function for cycle detection
fn dfs_visit(
    graph: &Graph,
    vertex: usize,
    visited: &mut HashSet<usize>,
    parent: &mut HashMap<usize, usize>,
    parent_vertex: Option<usize>,
) -> Option<Vec<usize>> {
    visited.insert(vertex);
    
    for &neighbor in &graph.neighbors(vertex) {
        if Some(neighbor) == parent_vertex {
            continue; // Skip immediate parent to avoid false positive
        }
        
        if visited.contains(&neighbor) {
            // Found cycle! Reconstruct the cycle path
            let mut cycle = vec![neighbor, vertex];
            let mut current = vertex;
            
            // Trace back through parents until we reach the cycle start
            while let Some(&p) = parent.get(&current) {
                if p == neighbor {
                    break;
                }
                cycle.push(p);
                current = p;
            }
            
            cycle.reverse();
            return Some(cycle);
        }
        
        parent.insert(neighbor, vertex);
        if let Some(cycle) = dfs_visit(graph, neighbor, visited, parent, Some(vertex)) {
            return Some(cycle);
        }
    }
    
    None
}

/// Compare performance of both approaches
pub fn benchmark_cycle_detection(graph: &Graph) -> Result<(), String> {
    println!("🔍 Comparing Cycle Detection Approaches");
    println!("Graph: {} vertices, {} edges", graph.vertices, graph.edges.len());
    
    // Test Union-Find approach
    let start = std::time::Instant::now();
    let uf_result = detect_cycle_union_find(graph)?;
    let uf_time = start.elapsed();
    
    // Test DFS approach  
    let start = std::time::Instant::now();
    let dfs_result = detect_cycle_dfs(graph);
    let dfs_time = start.elapsed();
    
    println!("\nResults:");
    println!("• Union-Find: {:?} (took {:?})", 
             uf_result.map(|edge| format!("Cycle detected at edge {:?}", edge))
                      .unwrap_or_else(|| "No cycle".to_string()), 
             uf_time);
    
    let dfs_display = dfs_result.as_ref()
        .map(|cycle| format!("Cycle found: {:?}", cycle))
        .unwrap_or_else(|| "No cycle".to_string());
    
    println!("• DFS: {} (took {:?})", dfs_display, dfs_time);
    
    // Verify both approaches agree
    let uf_has_cycle = uf_result.is_some();
    let dfs_has_cycle = dfs_result.is_some();
    
    if uf_has_cycle == dfs_has_cycle {
        println!("✅ Both approaches agree!");
    } else {
        println!("❌ Approaches disagree - this shouldn't happen!");
    }
    
    Ok(())
}

/// Example: Simple cycle detection
fn example_simple_cycle() -> Result<(), String> {
    println!("=== Simple Cycle Detection Example ===");
    
    // Create a graph with a simple 3-cycle: 0-1-2-0
    let mut graph = Graph::new(4);
    graph.add_edge(0, 1)?;
    graph.add_edge(1, 2)?;
    graph.add_edge(2, 0)?; // This creates the cycle
    graph.add_edge(3, 1)?; // Connected but doesn't create cycle
    
    graph.print();
    
    // Detect cycle with Union-Find
    match detect_cycle_union_find(&graph)? {
        Some((u, v)) => println!("Union-Find: Cycle detected when adding edge ({}, {})", u, v),
        None => println!("Union-Find: No cycle detected"),
    }
    
    // Detect cycle with DFS
    match detect_cycle_dfs(&graph) {
        Some(cycle) => println!("DFS: Cycle found: {:?}", cycle),
        None => println!("DFS: No cycle detected"),
    }
    
    println!();
    Ok(())
}

/// Example: No cycle case
fn example_no_cycle() -> Result<(), String> {
    println!("=== No Cycle Example (Tree) ===");
    
    // Create a tree (connected acyclic graph)
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1)?;
    graph.add_edge(0, 2)?;
    graph.add_edge(1, 3)?;
    graph.add_edge(1, 4)?;
    
    graph.print();
    
    benchmark_cycle_detection(&graph)?;
    println!();
    
    Ok(())
}

/// Example: Multiple cycles
fn example_multiple_cycles() -> Result<(), String> {
    println!("=== Multiple Cycles Example ===");
    
    // Create a graph with multiple cycles
    let mut graph = Graph::new(6);
    
    // First cycle: 0-1-2-0
    graph.add_edge(0, 1)?;
    graph.add_edge(1, 2)?;
    graph.add_edge(2, 0)?;
    
    // Second cycle: 3-4-5-3
    graph.add_edge(3, 4)?;
    graph.add_edge(4, 5)?;
    graph.add_edge(5, 3)?;
    
    // Connect the two cycles
    graph.add_edge(2, 3)?;
    
    graph.print();
    
    println!("\n🔍 Union-Find processes edges in order:");
    let mut uf = UnionFind::new(graph.vertices);
    
    for (i, &(u, v)) in graph.edges.iter().enumerate() {
        print!("Step {}: Edge ({}, {}) -> ", i + 1, u, v);
        
        if uf.connected(u, v)? {
            println!("CYCLE DETECTED! ⚠️");
            break;
        } else {
            uf.union(u, v)?;
            println!("Added (components: {})", uf.count());
        }
    }
    
    println!();
    Ok(())
}

/// Example: Online cycle detection (dynamic graph)
fn example_online_detection() -> Result<(), String> {
    println!("=== Online Cycle Detection Example ===");
    println!("Simulating dynamic graph where edges are added one by one");
    
    let vertices = 4;
    let mut uf = UnionFind::new(vertices);
    
    // Simulate adding edges dynamically
    let edges_to_add = vec![
        (0, 1, "Connect nodes 0 and 1"),
        (1, 2, "Connect nodes 1 and 2"), 
        (3, 2, "Connect node 3 to the component"),
        (0, 3, "Try to connect 0 and 3 - will this create a cycle?"),
        (0, 2, "Try to connect 0 and 2 directly"),
    ];
    
    for (u, v, description) in edges_to_add {
        println!("\n📍 {}", description);
        
        if u >= vertices || v >= vertices {
            println!("❌ Invalid edge ({}, {}) - vertices must be 0-{}", u, v, vertices - 1);
            continue;
        }
        
        if uf.connected(u, v)? {
            println!("⚠️  Edge ({}, {}) would create a CYCLE - rejecting", u, v);
        } else {
            uf.union(u, v)?;
            println!("✅ Edge ({}, {}) added successfully", u, v);
            println!("   Components remaining: {}", uf.count());
        }
    }
    
    println!("\n🎯 Final graph structure:");
    println!("   Connected components: {}", uf.count());
    
    if uf.count() == 1 {
        println!("   All vertices are connected!");
    }
    
    println!();
    Ok(())
}

/// Demonstrate when to use each approach
fn algorithm_comparison_guide() {
    println!("=== Algorithm Comparison Guide ===");
    println!();
    println!("🔄 **Union-Find Approach**");
    println!("   ✅ Best for: Dynamic graphs, online algorithms");
    println!("   ✅ Advantages: Incremental, handles edge additions efficiently");
    println!("   ✅ Time: O(E α(V)) - very fast in practice");
    println!("   ❌ Limitations: Only detects existence, not the actual cycle");
    println!();
    println!("🌲 **DFS Approach**");
    println!("   ✅ Best for: Static graphs, when you need the cycle path");
    println!("   ✅ Advantages: Finds actual cycle, classic algorithm");
    println!("   ✅ Time: O(V + E) - optimal for dense graphs");  
    println!("   ❌ Limitations: Must process entire graph each time");
    println!();
    println!("📊 **When to Use Each:**");
    println!("   • **Sparse graphs + online**: Union-Find");
    println!("   • **Dense graphs + batch processing**: DFS");
    println!("   • **Need cycle path**: DFS");
    println!("   • **Incremental edge additions**: Union-Find");
    println!();
}

fn main() -> Result<(), String> {
    println!("🔍 Cycle Detection in Undirected Graphs\n");
    
    example_simple_cycle()?;
    example_no_cycle()?;
    example_multiple_cycles()?;
    example_online_detection()?;
    algorithm_comparison_guide();
    
    println!("✅ All cycle detection examples completed!");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1).unwrap();
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(2, 0).unwrap(); // Creates cycle
        
        // Both approaches should detect cycle
        assert!(detect_cycle_union_find(&graph).unwrap().is_some());
        assert!(detect_cycle_dfs(&graph).is_some());
    }

    #[test]
    fn test_no_cycle_tree() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1).unwrap();
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(1, 3).unwrap(); // Tree structure
        
        // Both approaches should find no cycle
        assert!(detect_cycle_union_find(&graph).unwrap().is_none());
        assert!(detect_cycle_dfs(&graph).is_none());
    }

    #[test]
    fn test_single_vertex() {
        let graph = Graph::new(1);
        
        // Single vertex has no cycle
        assert!(detect_cycle_union_find(&graph).unwrap().is_none());
        assert!(detect_cycle_dfs(&graph).is_none());
    }

    #[test]
    fn test_disconnected_components() {
        let mut graph = Graph::new(6);
        
        // Two triangles (each has a cycle)
        graph.add_edge(0, 1).unwrap();
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(2, 0).unwrap(); // Cycle 1
        
        graph.add_edge(3, 4).unwrap();
        graph.add_edge(4, 5).unwrap();
        graph.add_edge(5, 3).unwrap(); // Cycle 2
        
        // Both approaches should detect at least one cycle
        assert!(detect_cycle_union_find(&graph).unwrap().is_some());
        assert!(detect_cycle_dfs(&graph).is_some());
    }

    #[test]
    fn test_invalid_edge() {
        let mut graph = Graph::new(3);
        
        // Try to add edge with invalid vertex
        assert!(graph.add_edge(0, 5).is_err());
        assert!(graph.add_edge(0, 0).is_err()); // Self-loop
    }
}