//! # Step 1: Algorithm Trait Design
//!
//! **Day 1 (Oct 15)**: Foundation - Designing the Graph trait
//!
//! ## Learning Objectives
//!
//! - Understand why trait abstraction is needed for generic algorithms
//! - Design a trait that works with multiple graph representations
//! - Learn about trait bounds (Copy, Eq, Hash)
//! - Separate algorithm state from data structure
//!
//! ## Key Concepts
//!
//! ### The Graph Trait
//!
//! Instead of writing BFS/DFS for each graph type (adjacency list, matrix, etc.),
//! we define a `Graph` trait that captures the essential operations:
//!
//! ```rust,no_run
//! pub trait Graph {
//!     type Node: Copy + Eq + std::hash::Hash;
//!     fn neighbors(&self, node: Self::Node) -> Vec<Self::Node>;
//!     fn contains(&self, node: Self::Node) -> bool;
//!     fn nodes(&self) -> Vec<Self::Node>;
//! }
//! ```
//!
//! ### Why This Design?
//!
//! 1. **Generic algorithms**: Write BFS/DFS once, works everywhere
//! 2. **Zero-cost abstraction**: Traits monomorphize at compile time (no runtime overhead)
//! 3. **Testability**: Easy to create mock graphs for testing
//! 4. **Extensibility**: Users can implement for custom types

use std::collections::HashMap;

/// Example 1: Graph trait definition (conceptual)
///
/// This shows what the trait looks like. The actual trait is in mission8::Graph.
fn explain_trait_design() {
    println!("=== Step 1: Algorithm Trait Design ===\n");
    
    println!("📚 Understanding the Graph Trait\n");
    
    println!("The Graph trait requires three essential operations:");
    println!("1. neighbors(node) - Get all nodes connected to this node");
    println!("2. contains(node) - Check if a node exists in the graph");
    println!("3. nodes() - Get all nodes in the graph\n");
    
    println!("Why these three? Because BFS/DFS needs to:");
    println!("  • Start at a node (needs: contains)");
    println!("  • Explore neighbors (needs: neighbors)");
    println!("  • Process all nodes (needs: nodes)\n");
}

/// Example 2: Adjacency list implementation
///
/// This is the most common graph representation in competitive programming.
struct AdjacencyList {
    edges: HashMap<u32, Vec<u32>>,
}

impl AdjacencyList {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }
    
    fn add_edge(&mut self, from: u32, to: u32) {
        self.edges.entry(from).or_insert_with(Vec::new).push(to);
    }
    
    // Simulate Graph trait methods (actual trait impl would be in mission8)
    fn neighbors(&self, node: u32) -> Vec<u32> {
        self.edges.get(&node).cloned().unwrap_or_default()
    }
    
    fn contains(&self, node: u32) -> bool {
        self.edges.contains_key(&node)
    }
    
    fn nodes(&self) -> Vec<u32> {
        self.edges.keys().copied().collect()
    }
}

/// Example 3: Building a simple graph
fn example_adjacency_list() {
    println!("=== Example: Adjacency List Graph ===\n");
    
    let mut graph = AdjacencyList::new();
    
    // Build a simple graph:
    //     0 → 1 → 3
    //     ↓   
    //     2 → 3
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 3);
    
    println!("Graph structure:");
    println!("  0 → [1, 2]");
    println!("  1 → [3]");
    println!("  2 → [3]");
    println!("  3 → []\n");
    
    // Test trait methods
    println!("Testing Graph trait operations:");
    println!("  neighbors(0) = {:?}", graph.neighbors(0));
    println!("  neighbors(1) = {:?}", graph.neighbors(1));
    println!("  neighbors(3) = {:?}", graph.neighbors(3));
    println!("  contains(0) = {}", graph.contains(0));
    println!("  contains(5) = {}", graph.contains(5));
    println!("  all nodes = {:?}\n", graph.nodes());
}

/// Example 4: Why trait bounds matter (Copy, Eq, Hash)
fn explain_trait_bounds() {
    println!("=== Understanding Trait Bounds ===\n");
    
    println!("The Graph trait requires: type Node: Copy + Eq + Hash\n");
    
    println!("Why Copy?");
    println!("  • Nodes are passed by value frequently (no lifetime issues)");
    println!("  • Efficient for small types (u32, usize, etc.)");
    println!("  • Example: let n = node; // No clone() needed\n");
    
    println!("Why Eq?");
    println!("  • Need to compare nodes for equality");
    println!("  • Example: if current_node == target_node {{ ... }}\n");
    
    println!("Why Hash?");
    println!("  • Need to store nodes in HashSet (visited tracking)");
    println!("  • Need to use nodes as HashMap keys (parent tracking)");
    println!("  • Example: visited.insert(node); // Requires Hash\n");
}

/// Example 5: Alternative graph representation (for comparison)
struct AdjacencyMatrix {
    matrix: Vec<Vec<bool>>,
    size: usize,
}

impl AdjacencyMatrix {
    fn new(size: usize) -> Self {
        Self {
            matrix: vec![vec![false; size]; size],
            size,
        }
    }
    
    fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.size && to < self.size {
            self.matrix[from][to] = true;
        }
    }
    
    // Simulate Graph trait methods
    fn neighbors(&self, node: usize) -> Vec<usize> {
        if node >= self.size {
            return Vec::new();
        }
        self.matrix[node]
            .iter()
            .enumerate()
            .filter(|(_, &connected)| connected)
            .map(|(idx, _)| idx)
            .collect()
    }
    
    fn contains(&self, node: usize) -> bool {
        node < self.size
    }
    
    #[allow(dead_code)]
    fn nodes(&self) -> Vec<usize> {
        (0..self.size).collect()
    }
}

fn example_adjacency_matrix() {
    println!("=== Example: Adjacency Matrix Graph ===\n");
    
    let mut graph = AdjacencyMatrix::new(4);
    
    // Same graph as before, different representation
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 3);
    
    println!("Graph structure (4×4 matrix):");
    println!("     0  1  2  3");
    for (i, row) in graph.matrix.iter().enumerate() {
        print!("  {}  ", i);
        for &val in row {
            print!("{:>2} ", if val { "T" } else { "F" });
        }
        println!();
    }
    println!();
    
    println!("Testing Graph trait operations:");
    println!("  neighbors(0) = {:?}", graph.neighbors(0));
    println!("  neighbors(1) = {:?}", graph.neighbors(1));
    println!("  contains(0) = {}", graph.contains(0));
    println!("  contains(5) = {}\n", graph.contains(5));
}

/// Example 6: Algorithm state design (separate from graph)
fn explain_state_separation() {
    println!("=== Algorithm State Design ===\n");
    
    println!("Key principle: Separate algorithm state from graph data\n");
    
    println!("BFS State contains:");
    println!("  • visited: HashSet<Node>  - Nodes already processed");
    println!("  • queue: VecDeque<Node>   - Nodes to process (FIFO)");
    println!("  • parent: HashMap<N, N>   - For path reconstruction\n");
    
    println!("DFS State contains:");
    println!("  • visited: HashSet<Node>  - Nodes already processed");
    println!("  • stack: Vec<Node>        - Nodes to process (LIFO)");
    println!("  • parent: HashMap<N, N>   - For path reconstruction\n");
    
    println!("Benefits of separation:");
    println!("  ✅ Graph remains immutable during traversal");
    println!("  ✅ Can run multiple algorithms concurrently");
    println!("  ✅ State can be saved/restored for debugging");
    println!("  ✅ Easier to test and reason about\n");
}

/// Example 7: Summary and next steps
fn summary() {
    println!("=== Summary: Trait Design Principles ===\n");
    
    println!("✅ What we learned:");
    println!("  1. Graph trait abstracts over different representations");
    println!("  2. Trait bounds (Copy, Eq, Hash) enable efficient algorithms");
    println!("  3. Adjacency list vs matrix: different trade-offs");
    println!("  4. Separate algorithm state from graph data\n");
    
    println!("🎯 Next step (Day 2):");
    println!("  Implement generic BFS and DFS using this trait design");
    println!("  File: step2_generic_bfs_dfs.rs\n");
    
    println!("📝 Exercise:");
    println!("  Try implementing the Graph trait for a different representation");
    println!("  (e.g., edge list, or graph from a grid)\n");
}

fn main() {
    explain_trait_design();
    example_adjacency_list();
    explain_trait_bounds();
    example_adjacency_matrix();
    explain_state_separation();
    summary();
}
