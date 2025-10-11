//! # Competitive Programming: BFS Maze Solver with RingBufferQueue
//!
//! This crate demonstrates the optimal use case for `RingBufferQueue` in competitive programming:
//! **Breadth-First Search (BFS) on bounded grids** where queue capacity is known and
//! cache efficiency is critical for performance.
//!
//! ## Problem: Shortest Path in Grid Maze
//!
//! Classic competitive programming problem found in contests like Codeforces, AtCoder,
//! and ICPC. Given a grid maze with walls and open paths, find the shortest path from
//! start to goal using BFS.
//!
//! ## Why RingBufferQueue is Perfect Here
//!
//! 1. **Bounded Problem**: Maximum queue size = `rows × cols` (entire grid)
//! 2. **Cache Efficiency**: Contiguous memory layout improves cache hits during BFS
//! 3. **Predictable Memory**: Fixed allocation prevents runtime allocation overhead
//! 4. **O(1) Operations**: Critical for time-sensitive competitive programming
//!
//! ## Quick Start
//!
//! ```rust
//! use competitive_ring_bfs::MazeSolver;
//!
//! let maze = vec![
//!     "S...".to_string(),
//!     ".##.".to_string(),
//!     "...G".to_string(),
//! ];
//!
//! let solver = MazeSolver::new(maze).expect("Valid maze");
//! match solver.find_shortest_path() {
//!     Some(path) => println!("Path length: {}", path.len()),
//!     None => println!("No path exists"),
//! }
//! ```
//!
//! ## Requirements Fulfilled
//!
//! This implementation follows V-Cycle methodology with complete traceability:
//!
//! ### Grid Processing Requirements
//! - **REQ-G1**: Parse text-based grid maze format
//! - **REQ-G2**: Identify start (S) and goal (G) positions
//! - **REQ-G3**: Handle walls (#) and open paths (.)
//! - **REQ-G4**: Validate grid format and boundaries
//!
//! ### BFS Algorithm Requirements  
//! - **REQ-B1**: Implement standard BFS with distance tracking
//! - **REQ-B2**: Explore 4-directional movement (up, down, left, right)
//! - **REQ-B3**: Use RingBufferQueue with capacity = `rows × cols`
//! - **REQ-B4**: Track visited cells to prevent cycles
//! - **REQ-B5**: Guarantee shortest path (BFS property)
//!
//! ### Path Reconstruction Requirements
//! - **REQ-P1**: Store parent pointers during BFS traversal
//! - **REQ-P2**: Reconstruct path from goal back to start
//! - **REQ-P3**: Return path as sequence of coordinates
//! - **REQ-P4**: Handle no-path case gracefully
//!
//! ## Performance Characteristics
//!
//! | Metric                | RingBufferQueue | std::VecDeque | std::collections::VecDeque |
//! |-----------------------|-----------------|---------------|-----------------------------|
//! | Memory Layout         | Contiguous      | Contiguous    | Ring buffer in heap        |
//! | Cache Efficiency      | Excellent       | Good          | Good                       |
//! | Allocation Strategy   | Fixed upfront   | Dynamic       | Dynamic                    |
//! | Operation Complexity  | O(1)           | Amortized O(1)| Amortized O(1)             |
//! | Memory Predictability | Perfect         | Variable      | Variable                   |
//!
//! ## Competitive Programming Applications
//!
//! This pattern applies to many contest problems:
//!
//! - **Grid BFS**: Shortest path, flood fill, connected components
//! - **Level-order traversal**: Tree/graph problems with bounded depth
//! - **Multi-source BFS**: Nearest facility, multi-start shortest path
//! - **State-space search**: Game states, puzzle solving (bounded state space)
//!
//! ## Usage vs LinkedQueue
//!
//! **Use RingBufferQueue when**:
//! - Grid/matrix problems with known bounds
//! - Performance is critical (competitive programming)
//! - Memory usage must be predictable
//! - Cache efficiency matters
//!
//! **Use LinkedQueue when**:
//! - Tree/graph problems with unknown size
//! - Memory is constrained (allocate as needed)
//! - Queue size varies dramatically
//! - Occasional allocation overhead is acceptable

pub mod maze;
pub mod solver;

// Re-export main types for convenient access
pub use maze::{Cell, Grid, Position};
pub use solver::MazeSolver;
