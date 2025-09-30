//! # Competitive Programming: Tree Diameter with LinkedQueue
//!
//! This crate demonstrates the optimal use case for `LinkedQueue` in competitive programming:
//! **Tree diameter calculation using multi-source BFS** where queue size is unbounded
//! and dynamic growth is essential for unknown problem sizes.
//!
//! ## Problem: Tree Diameter (Longest Path in Tree)
//!
//! Classic competitive programming problem found in contests like Codeforces, AtCoder,
//! USACO, and ACM ICPC. Given an unweighted tree, find the longest simple path between
//! any two nodes (the diameter).
//!
//! ## Algorithm: Two-Pass BFS
//!
//! 1. **First BFS**: Start from any node, find the farthest node from it
//! 2. **Second BFS**: Start from that farthest node, find the farthest node from it
//! 3. **Result**: The distance from step 2 is the tree diameter
//!
//! ## Why LinkedQueue is Perfect Here
//!
//! 1. **Unbounded Problem**: Tree size unknown, queue grows dynamically
//! 2. **Variable Queue Size**: Different BFS runs have different queue patterns  
//! 3. **Memory Efficiency**: Allocate only as needed, no wasted capacity
//! 4. **No Capacity Limits**: Can handle trees of any size within memory
//! 5. **Two Separate BFS**: Each gets fresh queue without capacity concerns
//!
//! ## Quick Start
//!
//! ```rust
//! use competitive_linked_tree::TreeDiameter;
//!
//! // Create tree with edges: 0-1, 1-2, 2-3, 3-4 (line graph)
//! let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
//! let diameter_finder = TreeDiameter::from_edges(5, edges).expect("Valid tree");
//!
//! match diameter_finder.find_diameter() {
//!     Some((diameter, path)) => {
//!         println!("Diameter: {}", diameter);
//!         println!("Path: {:?}", path);
//!     }
//!     None => println!("Empty tree"),
//! }
//! ```
//!
//! ## Requirements Fulfilled
//!
//! This implementation follows V-Cycle methodology with complete traceability:
//!
//! ### Tree Construction Requirements
//! - **REQ-T1**: Build adjacency list from edge list input
//! - **REQ-T2**: Support undirected tree representation
//! - **REQ-T3**: Handle nodes numbered 0 to n-1
//! - **REQ-T4**: Validate tree properties (n-1 edges, connected)
//!
//! ### BFS Algorithm Requirements
//! - **REQ-B1**: Implement BFS with distance and parent tracking
//! - **REQ-B2**: Use LinkedQueue for unbounded growth during traversal
//! - **REQ-B3**: Find farthest node from any given starting node
//! - **REQ-B4**: Support multiple independent BFS runs
//! - **REQ-B5**: Handle trees of arbitrary size (1 to 10^6+ nodes)
//!
//! ### Diameter Calculation Requirements
//! - **REQ-D1**: Execute two-pass BFS algorithm correctly
//! - **REQ-D2**: Return diameter length and actual path
//! - **REQ-D3**: Reconstruct path using parent pointers
//! - **REQ-D4**: Handle edge cases (single node, empty tree)
//! - **REQ-D5**: Guarantee correctness for any tree structure
//!
//! ## Performance Characteristics
//!
//! | Metric                | LinkedQueue     | std::VecDeque | std::collections::VecDeque |
//! |-----------------------|-----------------|---------------|----------------------------|
//! | Memory Growth         | Dynamic O(n)    | Dynamic O(n)  | Dynamic O(n)              |
//! | Allocation Pattern    | As needed       | Exponential   | Exponential               |
//! | Memory Overhead       | Minimal         | Some slack    | Some slack                |
//! | Operation Complexity  | O(1)           | Amortized O(1)| Amortized O(1)            |
//! | Memory Fragmentation  | Linked nodes    | Contiguous    | Contiguous                |
//! | Peak Memory Usage     | Exactly needed  | May overshoot | May overshoot             |
//!
//! ## Competitive Programming Applications
//!
//! This pattern applies to many tree/graph contest problems:
//!
//! - **Tree Diameter**: Longest path, tree center finding
//! - **Graph BFS**: Unknown graph size, dynamic exploration
//! - **Level-order Problems**: Binary trees, n-ary trees of unknown depth
//! - **Multi-source BFS**: Simultaneous expansion from multiple sources
//! - **Parsing Problems**: Tree building from input with unknown structure
//!
//! ## Usage vs RingBufferQueue
//!
//! **Use LinkedQueue when**:
//! - Tree/graph problems with unknown size
//! - Queue size varies dramatically between runs
//! - Memory efficiency is more important than cache performance
//! - Multiple independent BFS traversals needed
//! - Input size not bounded or very large
//!
//! **Use RingBufferQueue when**:
//! - Grid/matrix problems with known bounds  
//! - Single BFS run with predictable queue size
//! - Cache performance is critical
//! - Memory allocation overhead must be minimized
//!
//! ## Time Complexity
//!
//! - **Tree Construction**: O(n) where n = number of nodes
//! - **Single BFS**: O(n + m) where m = number of edges (m = n-1 for trees)
//! - **Diameter Algorithm**: O(n) since we do two BFS runs
//! - **Space Complexity**: O(n) for adjacency list + queue + visited array

pub mod tree;
pub mod diameter;

// Re-export main types for convenient access
pub use tree::{Tree, NodeId};
pub use diameter::TreeDiameter;