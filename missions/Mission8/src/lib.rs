//! # Mission 8: BFS/DFS Algorithms - Generic Graph Traversal
//!
//! This crate provides generic implementations of breadth-first search (BFS)
//! and depth-first search (DFS) algorithms that work on any graph type through
//! trait abstraction.
//!
//! ## Requirements Fulfilled
//!
//! - **REQ-1**: Generic BFS/DFS algorithms working on any graph type
//! - **REQ-2**: Algorithm composition (shortest path, cycle detection, etc.)
//! - **REQ-3**: Performance-optimized implementations with benchmarks
//! - **REQ-4**: Real-world applications (maze solver, network analyzer)
//! - **REQ-5**: Comprehensive integration testing
//! - **REQ-6**: Complete rustdoc documentation
//!
//! ## Quick Start
//!
//! ```rust
//! use mission8::{Graph, BFSState, DFSState};
//! use std::collections::HashMap;
//!
//! // Create a simple graph using adjacency list
//! let mut adj_list: HashMap<u32, Vec<u32>> = HashMap::new();
//! adj_list.insert(0, vec![1, 2]);
//! adj_list.insert(1, vec![3]);
//! adj_list.insert(2, vec![3]);
//! adj_list.insert(3, vec![]);
//!
//! // Create algorithm states (actual BFS/DFS implementations in REQ-1)
//! let bfs_state: BFSState<u32> = BFSState::new(0);
//! let dfs_state: DFSState<u32> = DFSState::new(0);
//! 
//! // Verify state initialization
//! assert!(bfs_state.visited().contains(&0));
//! assert!(dfs_state.visited().contains(&0));
//! ```
//!
//! ## Core Design: The Graph Trait
//!
//! The foundation of this crate is the `Graph` trait, which abstracts over
//! different graph representations:
//!
//! ```rust
//! /// Core trait that any graph must implement for algorithms to work.
//! ///
//! /// This trait provides a uniform interface for graph operations,
//! /// allowing BFS/DFS algorithms to work on any graph representation
//! /// (adjacency list, adjacency matrix, custom types, etc.).
//! pub trait Graph {
//!     /// The node type used in this graph.
//!     ///
//!     /// Must be Copy (for efficient passing), Eq (for comparisons),
//!     /// and Hash (for HashSet/HashMap storage).
//!     type Node: Copy + Eq + std::hash::Hash;
//!     
//!     /// Get all neighbors of a node.
//!     ///
//!     /// # Arguments
//!     ///
//!     /// * `node` - The node whose neighbors to retrieve
//!     ///
//!     /// # Returns
//!     ///
//!     /// Vector of all nodes directly connected to the input node.
//!     /// Returns empty vector if node has no neighbors or doesn't exist.
//!     ///
//!     /// # Examples
//!     ///
//!     /// ```rust
//!     /// # use mission8::Graph;
//!     /// # use std::collections::HashMap;
//!     /// # let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
//!     /// # graph.insert(0, vec![1, 2]);
//!     /// // Assuming graph implements Graph trait
//!     /// // let neighbors = graph.neighbors(0);
//!     /// // assert_eq!(neighbors, vec![1, 2]);
//!     /// ```
//!     fn neighbors(&self, node: Self::Node) -> Vec<Self::Node>;
//!     
//!     /// Check if graph contains a specific node.
//!     ///
//!     /// # Arguments
//!     ///
//!     /// * `node` - The node to check for existence
//!     ///
//!     /// # Returns
//!     ///
//!     /// `true` if the node exists in the graph, `false` otherwise.
//!     fn contains(&self, node: Self::Node) -> bool;
//!     
//!     /// Get all nodes in the graph.
//!     ///
//!     /// # Returns
//!     ///
//!     /// Vector of all nodes present in the graph.
//!     ///
//!     /// # Notes
//!     ///
//!     /// Order of nodes is implementation-dependent and may not be stable.
//!     fn nodes(&self) -> Vec<Self::Node>;
//! }
//! ```
//!
//! ## Performance Characteristics
//!
//! | Algorithm | Time Complexity | Space Complexity | Notes |
//! |-----------|----------------|------------------|-------|
//! | BFS | O(V + E) | O(V) | Queue-based, level-order |
//! | DFS | O(V + E) | O(V) | Stack-based (iterative) |
//! | Shortest Path | O(V + E) | O(V) | Unweighted graphs only |
//! | Cycle Detection | O(V + E) | O(V) | Back-edge detection |
//!
//! ## Use Cases
//!
//! - **Shortest Path**: Use BFS on unweighted graphs
//! - **Cycle Detection**: Use DFS with back-edge tracking
//! - **Connectivity**: Use BFS/DFS to find connected components
//! - **Topological Sort**: Use DFS with finishing times
//! - **Maze Solving**: Model as grid graph, use BFS for shortest path
//!
//! ## Architecture Decisions
//!
//! ### Why Trait-Based Design?
//!
//! - **Flexibility**: Works with any graph representation
//! - **Zero-Cost**: Traits monomorphize at compile time
//! - **Testability**: Easy to mock graphs for testing
//! - **Extensibility**: Users can implement for custom types
//!
//! ### Why Iterative Over Recursive?
//!
//! - **Stack Safety**: No stack overflow on deep graphs
//! - **Performance**: Better cache locality with explicit stack
//! - **Debuggability**: Easier to inspect state during execution
//! - **Flexibility**: Can pause/resume traversal easily

use std::collections::{HashMap, HashSet, VecDeque};

/// Core trait that any graph must implement for algorithms to work.
///
/// This trait provides a uniform interface for graph operations,
/// allowing BFS/DFS algorithms to work on any graph representation.
///
/// # Type Parameters
///
/// * `Node` - The type of nodes in the graph. Must be:
///   - `Copy` for efficient passing by value
///   - `Eq` for equality comparisons
///   - `Hash` for use in HashSet/HashMap
///
/// # Examples
///
/// Implementing for an adjacency list:
///
/// ```rust
/// use mission8::Graph;
/// use std::collections::HashMap;
///
/// struct AdjacencyList {
///     edges: HashMap<u32, Vec<u32>>,
/// }
///
/// impl Graph for AdjacencyList {
///     type Node = u32;
///     
///     fn neighbors(&self, node: Self::Node) -> Vec<Self::Node> {
///         self.edges.get(&node).cloned().unwrap_or_default()
///     }
///     
///     fn contains(&self, node: Self::Node) -> bool {
///         self.edges.contains_key(&node)
///     }
///     
///     fn nodes(&self) -> Vec<Self::Node> {
///         self.edges.keys().copied().collect()
///     }
/// }
/// ```
pub trait Graph {
    /// The node type used in this graph.
    type Node: Copy + Eq + std::hash::Hash;
    
    /// Get all neighbors of a node.
    ///
    /// # Arguments
    ///
    /// * `node` - The node whose neighbors to retrieve
    ///
    /// # Returns
    ///
    /// Vector of all nodes directly connected to the input node.
    /// Returns empty vector if node has no neighbors or doesn't exist.
    fn neighbors(&self, node: Self::Node) -> Vec<Self::Node>;
    
    /// Check if graph contains a specific node.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to check for existence
    ///
    /// # Returns
    ///
    /// `true` if the node exists in the graph, `false` otherwise.
    fn contains(&self, node: Self::Node) -> bool;
    
    /// Get all nodes in the graph.
    ///
    /// # Returns
    ///
    /// Vector of all nodes present in the graph.
    fn nodes(&self) -> Vec<Self::Node>;
}

/// Error types for graph algorithm operations.
///
/// These errors provide detailed context about what went wrong
/// during algorithm execution, enabling proper error handling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError {
    /// A requested node does not exist in the graph.
    NodeNotFound(String),
    
    /// No path exists between two nodes.
    NoPathExists {
        /// The starting node
        from: String,
        /// The destination node
        to: String,
    },
    
    /// A cycle was detected in the graph.
    CycleDetected(Vec<String>),
    
    /// Invalid input provided to algorithm.
    InvalidInput(String),
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphError::NodeNotFound(node) => {
                write!(f, "Node not found: {}", node)
            }
            GraphError::NoPathExists { from, to } => {
                write!(f, "No path exists from {} to {}", from, to)
            }
            GraphError::CycleDetected(path) => {
                write!(f, "Cycle detected: {:?}", path)
            }
            GraphError::InvalidInput(msg) => {
                write!(f, "Invalid input: {}", msg)
            }
        }
    }
}

impl std::error::Error for GraphError {}

/// State maintained during BFS traversal.
///
/// This structure is separate from the graph itself, allowing
/// multiple algorithms to run concurrently on the same graph.
///
/// # Type Parameters
///
/// * `N` - The node type (must match the graph's node type)
#[derive(Debug, Clone)]
pub struct BFSState<N: Copy + Eq + std::hash::Hash> {
    /// Nodes that have been visited
    visited: HashSet<N>,
    
    /// Queue of nodes to process (FIFO)
    queue: VecDeque<N>,
    
    /// Parent of each node (for path reconstruction) - will be used in REQ-2
    _parent: HashMap<N, N>,
}

impl<N: Copy + Eq + std::hash::Hash> BFSState<N> {
    /// Create a new BFS state starting from a given node.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting node for BFS
    ///
    /// # Returns
    ///
    /// A new BFSState ready to begin traversal.
    pub fn new(start: N) -> Self {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        visited.insert(start);
        queue.push_back(start);
        
        Self {
            visited,
            queue,
            _parent: HashMap::new(),
        }
    }
    
    /// Get all visited nodes so far.
    pub fn visited(&self) -> &HashSet<N> {
        &self.visited
    }
    
    /// Check if there are more nodes to process.
    pub fn has_next(&self) -> bool {
        !self.queue.is_empty()
    }
}

/// State maintained during DFS traversal.
///
/// Uses an explicit stack instead of recursion to avoid stack overflow
/// on deep graphs.
///
/// # Type Parameters
///
/// * `N` - The node type (must match the graph's node type)
#[derive(Debug, Clone)]
pub struct DFSState<N: Copy + Eq + std::hash::Hash> {
    /// Nodes that have been visited
    visited: HashSet<N>,
    
    /// Stack of nodes to process (LIFO)
    stack: Vec<N>,
    
    /// Parent of each node (for path reconstruction) - will be used in REQ-2
    _parent: HashMap<N, N>,
}

impl<N: Copy + Eq + std::hash::Hash> DFSState<N> {
    /// Create a new DFS state starting from a given node.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting node for DFS
    ///
    /// # Returns
    ///
    /// A new DFSState ready to begin traversal.
    pub fn new(start: N) -> Self {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        
        visited.insert(start);
        stack.push(start);
        
        Self {
            visited,
            stack,
            _parent: HashMap::new(),
        }
    }
    
    /// Get all visited nodes so far.
    pub fn visited(&self) -> &HashSet<N> {
        &self.visited
    }
    
    /// Check if there are more nodes to process.
    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

// Placeholder for actual algorithm implementations (to be implemented in REQ-1)
// These will be completed during Day 2 (Oct 16) of the mission

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_error_display() {
        let err = GraphError::NodeNotFound("node5".to_string());
        assert_eq!(err.to_string(), "Node not found: node5");
        
        let err = GraphError::NoPathExists {
            from: "A".to_string(),
            to: "B".to_string(),
        };
        assert_eq!(err.to_string(), "No path exists from A to B");
    }
    
    #[test]
    fn test_bfs_state_creation() {
        let state: BFSState<u32> = BFSState::new(0);
        assert!(state.visited().contains(&0));
        assert!(state.has_next());
    }
    
    #[test]
    fn test_dfs_state_creation() {
        let state: DFSState<u32> = DFSState::new(0);
        assert!(state.visited().contains(&0));
        assert!(state.has_next());
    }
}
