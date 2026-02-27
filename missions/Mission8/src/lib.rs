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
    type Node: Copy + Eq + std::hash::Hash + std::fmt::Debug;

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
pub struct BFSState<N: Copy + Eq + std::hash::Hash + std::fmt::Debug> {
    /// Nodes that have been visited
    visited: HashSet<N>,

    /// Queue of nodes to process (FIFO)
    queue: VecDeque<N>,

    /// Parent of each node (for path reconstruction) - will be used in REQ-2
    _parent: HashMap<N, N>,
}

impl<N: Copy + Eq + std::hash::Hash + std::fmt::Debug> BFSState<N> {
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
pub struct DFSState<N: Copy + Eq + std::hash::Hash + std::fmt::Debug> {
    /// Nodes that have been visited
    visited: HashSet<N>,

    /// Stack of nodes to process (LIFO)
    stack: Vec<N>,

    /// Parent of each node (for path reconstruction) - will be used in REQ-2
    _parent: HashMap<N, N>,
}

impl<N: Copy + Eq + std::hash::Hash + std::fmt::Debug> DFSState<N> {
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

// REQ-1: Generic BFS/DFS Algorithm Implementations
// Completed on Day 2 (Oct 16) of the mission

/// Performs breadth-first search on any graph type.
///
/// BFS explores nodes level by level, using a queue (FIFO) to process nodes.
/// This guarantees that nodes are visited in order of their distance from the start.
///
/// # Time Complexity
/// O(V + E) where V = number of vertices, E = number of edges
///
/// # Space Complexity  
/// O(V) for the visited set and queue
///
/// # Arguments
///
/// * `graph` - Any type implementing the Graph trait
/// * `start` - The starting node for traversal
///
/// # Returns
///
/// Vector of nodes in the order they were visited (level-order traversal)
///
/// # Examples
///
/// ```rust
/// use mission8::{Graph, bfs};
/// use std::collections::HashMap;
///
/// // Create adjacency list representation
/// let mut adj_list: HashMap<u32, Vec<u32>> = HashMap::new();
/// adj_list.insert(0, vec![1, 2]);
/// adj_list.insert(1, vec![3]);
/// adj_list.insert(2, vec![3]);
/// adj_list.insert(3, vec![]);
///
/// let visited = bfs(&adj_list, 0);
/// assert_eq!(visited, vec![0, 1, 2, 3]);  // Level-order traversal
/// ```
pub fn bfs<G: Graph>(graph: &G, start: G::Node) -> Vec<G::Node> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result = Vec::new();

    // Check if start node exists in graph
    if !graph.contains(start) {
        return result; // Return empty vector for non-existent start node
    }

    // Start with the initial node
    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        result.push(current);

        // Add all unvisited neighbors to queue
        for neighbor in graph.neighbors(current) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    result
}

/// Performs depth-first search on any graph type.
///
/// DFS explores as far as possible along each branch before backtracking,
/// using an explicit stack (LIFO) to avoid recursion and stack overflow.
///
/// # Time Complexity
/// O(V + E) where V = number of vertices, E = number of edges
///
/// # Space Complexity
/// O(V) for the visited set and stack
///
/// # Arguments
///
/// * `graph` - Any type implementing the Graph trait
/// * `start` - The starting node for traversal
///
/// # Returns
///
/// Vector of nodes in the order they were visited (depth-first order)
///
/// # Examples
///
/// ```rust
/// use mission8::{Graph, dfs};
/// use std::collections::HashMap;
///
/// // Create adjacency list representation
/// let mut adj_list: HashMap<u32, Vec<u32>> = HashMap::new();
/// adj_list.insert(0, vec![1, 2]);
/// adj_list.insert(1, vec![3]);
/// adj_list.insert(2, vec![3]);
/// adj_list.insert(3, vec![]);
///
/// let visited = dfs(&adj_list, 0);
/// // DFS order depends on neighbor order, but will explore deeply first
/// assert!(visited.contains(&0) && visited.contains(&1) && visited.contains(&2) && visited.contains(&3));
/// assert_eq!(visited[0], 0);  // Always starts with start node
/// ```
pub fn dfs<G: Graph>(graph: &G, start: G::Node) -> Vec<G::Node> {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut result = Vec::new();

    // Check if start node exists in graph
    if !graph.contains(start) {
        return result; // Return empty vector for non-existent start node
    }

    // Start with the initial node
    stack.push(start);

    while let Some(current) = stack.pop() {
        if !visited.contains(&current) {
            visited.insert(current);
            result.push(current);

            // Add all unvisited neighbors to stack
            // Note: We reverse to maintain consistent ordering across runs
            let mut neighbors = graph.neighbors(current);
            neighbors.reverse();

            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    stack.push(neighbor);
                }
            }
        }
    }

    result
}

/// Implementation of Graph trait for HashMap-based adjacency lists.
///
/// This allows BFS/DFS algorithms to work directly on `HashMap<Node, Vec<Node>>`
/// structures, which is the most common graph representation.
///
/// # Examples
///
/// ```rust
/// use mission8::{Graph, bfs, dfs};
/// use std::collections::HashMap;
///
/// let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
/// graph.insert(1, vec![2, 3]);
/// graph.insert(2, vec![4]);
/// graph.insert(3, vec![4]);
/// graph.insert(4, vec![]);
///
/// // Now we can use BFS/DFS directly
/// let bfs_result = bfs(&graph, 1);
/// let dfs_result = dfs(&graph, 1);
///
/// assert_eq!(bfs_result[0], 1);  // Starts with node 1
/// assert_eq!(dfs_result[0], 1);  // Starts with node 1
/// ```
impl<N> Graph for HashMap<N, Vec<N>>
where
    N: Copy + Eq + std::hash::Hash + std::fmt::Debug,
{
    type Node = N;

    fn neighbors(&self, node: Self::Node) -> Vec<Self::Node> {
        match self.get(&node) {
            Some(neighbors) => neighbors.clone(),
            None => Vec::new(),
        }
    }

    fn contains(&self, node: Self::Node) -> bool {
        self.contains_key(&node)
    }

    fn nodes(&self) -> Vec<Self::Node> {
        self.keys().copied().collect()
    }
}

/// REQ-2: Finds the shortest path between two nodes using BFS.
///
/// This function uses BFS with parent tracking to find the shortest path
/// in an unweighted graph. BFS guarantees the shortest path because it
/// explores level by level.
///
/// # Arguments
///
/// * `graph` - The graph to search
/// * `start` - The starting node
/// * `end` - The destination node
///
/// # Returns
///
/// * `Ok(Vec<Node>)` - Path from start to end (inclusive)
/// * `Err(GraphError::NoPathExists)` - If no path exists
///
/// # Time Complexity
///
/// O(V + E) where V is the number of vertices and E is the number of edges
///
/// # Space Complexity
///
/// O(V) for the visited set, parent map, and queue
///
/// # Examples
///
/// ```rust
/// use mission8::{Graph, shortest_path};
/// use std::collections::HashMap;
///
/// let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
/// graph.insert(0, vec![1, 2]);
/// graph.insert(1, vec![3]);
/// graph.insert(2, vec![3]);
/// graph.insert(3, vec![]);
///
/// let path = shortest_path(&graph, 0, 3);
/// assert!(path.is_ok());
/// let path = path.unwrap();
/// assert_eq!(path[0], 0);  // Starts at source
/// assert_eq!(path[path.len() - 1], 3);  // Ends at destination
/// assert_eq!(path.len(), 3);  // Path length is 3
/// ```
pub fn shortest_path<G: Graph>(
    graph: &G,
    start: G::Node,
    end: G::Node,
) -> Result<Vec<G::Node>, GraphError> {
    // Quick check: start node must exist
    if !graph.contains(start) {
        return Err(GraphError::InvalidInput(
            "Start node not found in graph".to_string(),
        ));
    }

    // If start and end are the same, path is just that node
    if start == end {
        return Ok(vec![start]);
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut parent: HashMap<G::Node, G::Node> = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == end {
            // Reconstruct path from end to start
            let mut path = vec![end];
            let mut node = end;

            while let Some(&p) = parent.get(&node) {
                path.push(p);
                node = p;
            }

            path.reverse();
            return Ok(path);
        }

        for neighbor in graph.neighbors(current) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                parent.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }
    }

    Err(GraphError::NoPathExists {
        from: format!("{:?}", start),
        to: format!("{:?}", end),
    })
}

/// REQ-2: Detects if a graph contains a cycle using DFS.
///
/// This function uses DFS with three-color node tracking to detect cycles.
/// Nodes can be in three states:
/// - White (unvisited)
/// - Gray (currently visiting)
/// - Black (completely visited)
///
/// A back edge (edge to a gray node) indicates a cycle.
///
/// # Arguments
///
/// * `graph` - The graph to check for cycles
///
/// # Returns
///
/// `true` if a cycle exists, `false` otherwise
///
/// # Time Complexity
///
/// O(V + E) where V is the number of vertices and E is the number of edges
///
/// # Space Complexity
///
/// O(V) for the color tracking and recursion stack
///
/// # Examples
///
/// ```rust
/// use mission8::{Graph, has_cycle};
/// use std::collections::HashMap;
///
/// // Acyclic graph (DAG)
/// let mut acyclic: HashMap<u32, Vec<u32>> = HashMap::new();
/// acyclic.insert(0, vec![1]);
/// acyclic.insert(1, vec![2]);
/// acyclic.insert(2, vec![]);
/// assert!(!has_cycle(&acyclic));
///
/// // Cyclic graph
/// let mut cyclic: HashMap<u32, Vec<u32>> = HashMap::new();
/// cyclic.insert(0, vec![1]);
/// cyclic.insert(1, vec![2]);
/// cyclic.insert(2, vec![0]);  // Creates cycle
/// assert!(has_cycle(&cyclic));
/// ```
pub fn has_cycle<G: Graph>(graph: &G) -> bool {
    let mut color: HashMap<G::Node, Color> = HashMap::new();

    // Initialize all nodes as white (unvisited)
    for node in graph.nodes() {
        color.insert(node, Color::White);
    }

    // Check each node (to handle disconnected components)
    for node in graph.nodes() {
        if *color.get(&node).unwrap_or(&Color::White) == Color::White
            && has_cycle_dfs(graph, node, &mut color)
        {
            return true;
        }
    }

    false
}

/// Helper for cycle detection using DFS with three-color tracking
#[allow(dead_code)]
fn has_cycle_dfs<G: Graph>(graph: &G, node: G::Node, color: &mut HashMap<G::Node, Color>) -> bool {
    color.insert(node, Color::Gray);

    for neighbor in graph.neighbors(node) {
        match color.get(&neighbor).unwrap_or(&Color::White) {
            Color::Gray => {
                // Back edge found - indicates a cycle
                return true;
            }
            Color::White => {
                if has_cycle_dfs(graph, neighbor, color) {
                    return true;
                }
            }
            Color::Black => {
                // Already processed this subtree
                continue;
            }
        }
    }

    color.insert(node, Color::Black);
    false
}

/// Node color states for DFS-based cycle detection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum Color {
    White, // Unvisited
    Gray,  // Currently visiting
    Black, // Completely visited
}

/// REQ-2: Finds a cycle in the graph if one exists.
///
/// This function returns the actual cycle path if a cycle is detected.
/// Returns `None` if the graph is acyclic (a DAG).
///
/// # Arguments
///
/// * `graph` - The graph to search for cycles
///
/// # Returns
///
/// * `Some(Vec<Node>)` - Nodes that form a cycle
/// * `None` - If graph is acyclic
///
/// # Time Complexity
///
/// O(V + E)
///
/// # Space Complexity
///
/// O(V)
///
/// # Examples
///
/// ```rust
/// use mission8::{Graph, find_cycle};
/// use std::collections::HashMap;
///
/// let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
/// graph.insert(0, vec![1]);
/// graph.insert(1, vec![2]);
/// graph.insert(2, vec![0]);  // Cycle: 0 -> 1 -> 2 -> 0
///
/// let cycle = find_cycle(&graph);
/// assert!(cycle.is_some());
/// let cycle = cycle.unwrap();
/// assert!(cycle.len() >= 3);  // At least 3 nodes in cycle
/// ```
pub fn find_cycle<G: Graph>(graph: &G) -> Option<Vec<G::Node>> {
    let mut color: HashMap<G::Node, Color> = HashMap::new();
    let mut parent: HashMap<G::Node, G::Node> = HashMap::new();
    let mut path: Vec<G::Node> = Vec::new();

    for node in graph.nodes() {
        color.insert(node, Color::White);
    }

    for node in graph.nodes() {
        if *color.get(&node).unwrap_or(&Color::White) == Color::White {
            path.clear();
            if find_cycle_dfs(graph, node, &mut color, &mut parent, &mut path) {
                return Some(path.clone());
            }
        }
    }

    None
}

/// Helper for finding cycle path using DFS
#[allow(dead_code)]
fn find_cycle_dfs<G: Graph>(
    graph: &G,
    node: G::Node,
    color: &mut HashMap<G::Node, Color>,
    parent: &mut HashMap<G::Node, G::Node>,
    path: &mut Vec<G::Node>,
) -> bool {
    color.insert(node, Color::Gray);
    path.push(node);

    for neighbor in graph.neighbors(node) {
        if *color.get(&neighbor).unwrap_or(&Color::White) == Color::Gray {
            // Found back edge - construct cycle
            // Find the position of the neighbor in the current path
            if let Some(pos) = path.iter().position(|&n| n == neighbor) {
                // Keep only the cycle part: from neighbor to current node
                let cycle_start = pos;
                let mut cycle = Vec::new();

                // Add nodes from the cycle start to the end
                for &node in path.iter().skip(cycle_start) {
                    cycle.push(node);
                }

                // Replace the path with the cycle
                path.clear();
                path.extend(cycle);
                return true;
            }
        } else if *color.get(&neighbor).unwrap_or(&Color::White) == Color::White {
            parent.insert(neighbor, node);
            if find_cycle_dfs(graph, neighbor, color, parent, path) {
                return true;
            }
        }
    }

    path.pop();
    color.insert(node, Color::Black);
    false
}

/// REQ-2: Finds all connected components in the graph.
///
/// A connected component is a maximal set of nodes where there is a path
/// between any two nodes in the set. This function returns a vector where
/// each element is a vector of nodes forming one component.
///
/// # Arguments
///
/// * `graph` - The graph to analyze
///
/// # Returns
///
/// Vector of components, where each component is a vector of nodes
///
/// # Time Complexity
///
/// O(V + E)
///
/// # Space Complexity
///
/// O(V)
///
/// # Examples
///
/// ```rust
/// use mission8::{Graph, connected_components};
/// use std::collections::HashMap;
///
/// // Graph with 2 components
/// let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
/// graph.insert(0, vec![1]);
/// graph.insert(1, vec![0]);  // Component 1: {0, 1}
/// graph.insert(2, vec![3]);
/// graph.insert(3, vec![2]);  // Component 2: {2, 3}
///
/// let components = connected_components(&graph);
/// assert_eq!(components.len(), 2);  // Two components
/// assert_eq!(components[0].len() + components[1].len(), 4);  // 4 nodes total
/// ```
pub fn connected_components<G: Graph>(graph: &G) -> Vec<Vec<G::Node>> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();

    // Pre-build reverse adjacency map once: O(V + E)
    let mut reverse: HashMap<G::Node, Vec<G::Node>> = HashMap::new();
    for node in graph.nodes() {
        for neighbor in graph.neighbors(node) {
            reverse.entry(neighbor).or_default().push(node);
        }
    }

    for node in graph.nodes() {
        if !visited.contains(&node) {
            let component = bfs_collect_component(graph, node, &mut visited, &reverse);
            components.push(component);
        }
    }

    components
}

/// Helper function to collect nodes in a connected component using BFS
///
/// This function treats the graph as undirected for connected components.
/// It follows both outgoing and incoming edges to find all reachable nodes.
fn bfs_collect_component<G: Graph>(
    graph: &G,
    start: G::Node,
    visited: &mut HashSet<G::Node>,
    reverse: &HashMap<G::Node, Vec<G::Node>>,
) -> Vec<G::Node> {
    let mut component = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        component.push(current);

        // Follow outgoing edges (current -> neighbor)
        for neighbor in graph.neighbors(current) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }

        // Follow incoming edges (neighbor -> current) for undirected behavior
        if let Some(predecessors) = reverse.get(&current) {
            for &node in predecessors {
                if !visited.contains(&node) {
                    visited.insert(node);
                    queue.push_back(node);
                }
            }
        }
    }

    component
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // ============================================================================
    // GRAPH ERROR TESTS
    // ============================================================================

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

    // ============================================================================
    // GRAPH TRAIT IMPLEMENTATION TESTS
    // ============================================================================

    #[test]
    fn test_hashmap_graph_implementation() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1, 2]);
        graph.insert(1, vec![3]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![]);

        // Test neighbors
        assert_eq!(graph.neighbors(0), vec![1, 2]);
        assert_eq!(graph.neighbors(1), vec![3]);
        assert_eq!(graph.neighbors(2), vec![3]);
        assert_eq!(graph.neighbors(3), vec![]);
        assert_eq!(graph.neighbors(99), vec![]); // Non-existent node

        // Test contains
        assert!(graph.contains(0));
        assert!(graph.contains(1));
        assert!(graph.contains(2));
        assert!(graph.contains(3));
        assert!(!graph.contains(99));

        // Test nodes
        let mut nodes = graph.nodes();
        nodes.sort();
        assert_eq!(nodes, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_hashmap_graph_with_string_slices() {
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        graph.insert("A", vec!["B"]);
        graph.insert("B", vec!["C"]);
        graph.insert("C", vec![]);

        assert_eq!(graph.neighbors("A"), vec!["B"]);
        assert!(graph.contains("A"));
        assert!(!graph.contains("D"));
    }

    #[test]
    fn test_empty_graph() {
        let graph: HashMap<u32, Vec<u32>> = HashMap::new();

        assert_eq!(graph.neighbors(0), vec![]);
        assert!(!graph.contains(0));
        assert_eq!(graph.nodes(), vec![]);
    }

    // ============================================================================
    // BFS/DFS CORE ALGORITHM TESTS
    // ============================================================================

    #[test]
    fn test_bfs_simple_graph() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1, 2]);
        graph.insert(1, vec![3]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![]);

        let result = bfs(&graph, 0);
        assert_eq!(result[0], 0); // Start node first
        assert_eq!(result.len(), 4); // All nodes visited
        assert!(result.contains(&0));
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_dfs_simple_graph() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1, 2]);
        graph.insert(1, vec![3]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![]);

        let result = dfs(&graph, 0);
        assert_eq!(result[0], 0); // Start node first
        assert_eq!(result.len(), 4); // All nodes visited
        assert!(result.contains(&0));
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(42, vec![]);

        let result = bfs(&graph, 42);
        assert_eq!(result, vec![42]);
    }

    #[test]
    fn test_dfs_single_node() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(42, vec![]);

        let result = dfs(&graph, 42);
        assert_eq!(result, vec![42]);
    }

    #[test]
    fn test_bfs_disconnected_graph() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1]);
        graph.insert(1, vec![0]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![2]);

        let result = bfs(&graph, 0);
        assert_eq!(result.len(), 2); // Only nodes 0 and 1
        assert!(result.contains(&0));
        assert!(result.contains(&1));
        assert!(!result.contains(&2));
        assert!(!result.contains(&3));
    }

    // ============================================================================
    // STATE MANAGEMENT TESTS
    // ============================================================================

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

    #[test]
    fn test_bfs_state_with_string_slices() {
        let state: BFSState<&str> = BFSState::new("start");
        assert!(state.visited().contains(&"start"));
        assert!(state.has_next());
    }

    // ============================================================================
    // SHORTEST PATH TESTS
    // ============================================================================

    #[test]
    fn test_shortest_path_simple() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1]);
        graph.insert(1, vec![2]);
        graph.insert(2, vec![]);

        let path = shortest_path(&graph, 0, 2);
        assert!(path.is_ok());
        assert_eq!(path.unwrap(), vec![0, 1, 2]);
    }

    #[test]
    fn test_shortest_path_same_node() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1]);
        graph.insert(1, vec![]);

        let path = shortest_path(&graph, 0, 0);
        assert!(path.is_ok());
        assert_eq!(path.unwrap(), vec![0]);
    }

    #[test]
    fn test_shortest_path_no_path() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1]);
        graph.insert(1, vec![]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![]);

        let path = shortest_path(&graph, 0, 2);
        assert!(path.is_err());
        match path {
            Err(GraphError::NoPathExists { from, to }) => {
                assert_eq!(from, "0");
                assert_eq!(to, "2");
            }
            _ => panic!("Expected NoPathExists error"),
        }
    }

    #[test]
    fn test_shortest_path_invalid_start() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1]);
        graph.insert(1, vec![]);

        let path = shortest_path(&graph, 99, 1);
        assert!(path.is_err());
        match path {
            Err(GraphError::InvalidInput(_)) => {}
            _ => panic!("Expected InvalidInput error"),
        }
    }

    // ============================================================================
    // CYCLE DETECTION TESTS
    // ============================================================================

    #[test]
    fn test_has_cycle_acyclic() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1]);
        graph.insert(1, vec![2]);
        graph.insert(2, vec![]);

        assert!(!has_cycle(&graph));
    }

    #[test]
    fn test_has_cycle_cyclic() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1]);
        graph.insert(1, vec![2]);
        graph.insert(2, vec![0]);

        assert!(has_cycle(&graph));
    }

    #[test]
    fn test_has_cycle_self_loop() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![0]);

        assert!(has_cycle(&graph));
    }

    #[test]
    fn test_find_cycle_acyclic() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1]);
        graph.insert(1, vec![2]);
        graph.insert(2, vec![]);

        assert!(find_cycle(&graph).is_none());
    }

    #[test]
    fn test_find_cycle_cyclic() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1]);
        graph.insert(1, vec![2]);
        graph.insert(2, vec![0]);

        let cycle = find_cycle(&graph);
        assert!(cycle.is_some());
        let cycle_nodes = cycle.unwrap();
        assert!(cycle_nodes.len() >= 3);
        assert!(cycle_nodes.contains(&0));
        assert!(cycle_nodes.contains(&1));
        assert!(cycle_nodes.contains(&2));
    }

    #[test]
    fn test_find_cycle_self_loop() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![0]);

        let cycle = find_cycle(&graph);
        assert!(cycle.is_some());
        assert_eq!(cycle.unwrap().len(), 1);
    }

    // ============================================================================
    // CONNECTED COMPONENTS TESTS
    // ============================================================================

    #[test]
    fn test_connected_components_single() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1]);
        graph.insert(1, vec![2]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![]);

        let components = connected_components(&graph);
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].len(), 4);
    }

    #[test]
    fn test_connected_components_multiple() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1]);
        graph.insert(1, vec![0]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![2]);

        let components = connected_components(&graph);
        assert_eq!(components.len(), 2);

        // Find component sizes
        let mut sizes: Vec<usize> = components.iter().map(|c| c.len()).collect();
        sizes.sort();
        assert_eq!(sizes, vec![2, 2]);
    }

    #[test]
    fn test_connected_components_isolated() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![]);
        graph.insert(1, vec![]);
        graph.insert(2, vec![]);

        let components = connected_components(&graph);
        assert_eq!(components.len(), 3);

        for component in components {
            assert_eq!(component.len(), 1);
        }
    }

    #[test]
    fn test_connected_components_empty() {
        let graph: HashMap<u32, Vec<u32>> = HashMap::new();
        let components = connected_components(&graph);
        assert_eq!(components.len(), 0);
    }

    // ============================================================================
    // INTEGRATION TESTS
    // ============================================================================

    #[test]
    fn test_all_algorithms_on_same_graph() {
        let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
        graph.insert(0, vec![1, 2]);
        graph.insert(1, vec![3]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![]);

        // Test BFS
        let bfs_result = bfs(&graph, 0);
        assert_eq!(bfs_result.len(), 4);

        // Test DFS
        let dfs_result = dfs(&graph, 0);
        assert_eq!(dfs_result.len(), 4);

        // Test shortest path
        let path = shortest_path(&graph, 0, 3);
        assert!(path.is_ok());
        assert_eq!(path.unwrap().len(), 3);

        // Test cycle detection
        assert!(!has_cycle(&graph));
        assert!(find_cycle(&graph).is_none());

        // Test connected components
        let components = connected_components(&graph);
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].len(), 4);
    }

    #[test]
    fn test_algorithms_with_string_slice_nodes() {
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        graph.insert("A", vec!["B"]);
        graph.insert("B", vec!["C"]);
        graph.insert("C", vec![]);

        // Test BFS with string slices
        let bfs_result = bfs(&graph, "A");
        assert_eq!(bfs_result.len(), 3);
        assert_eq!(bfs_result[0], "A");

        // Test shortest path with string slices
        let path = shortest_path(&graph, "A", "C");
        assert!(path.is_ok());
        assert_eq!(path.unwrap(), vec!["A", "B", "C"]);

        // Test connected components with string slices
        let components = connected_components(&graph);
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].len(), 3);
    }
}
