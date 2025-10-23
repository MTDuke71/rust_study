//! # Mission 9: Dijkstra & A* Pathfinding Algorithms
//!
//! Advanced pathfinding algorithms for weighted graphs with optimal path guarantees.
//!
//! ## Quick Start
//!
//! ```rust
//! use mission9::{DijkstraPathfinder, Pathfinder, SimpleWeightedGraph};
//!
//! // Create a simple weighted graph
//! let mut graph = SimpleWeightedGraph::new();
//! graph.add_edge(0, 1, 2.0);
//! graph.add_edge(1, 2, 3.0);
//!
//! // Find shortest path
//! let pathfinder = DijkstraPathfinder::new();
//! let result = pathfinder.find_path(&graph, 0, 2).unwrap();
//!
//! println!("Shortest path: {:?}, Cost: {:.1}", result.path, result.cost);
//! ```
//!
//! ## Core Components
//!
//! - **Priority Queue**: Efficient data structure for pathfinding algorithms
//! - **Dijkstra's Algorithm**: Guaranteed shortest path for weighted graphs  
//! - **A* Algorithm**: Heuristic-guided pathfinding for faster goal-directed search
//! - **Weighted Graph Trait**: Generic interface for graph data structures
//! - **Heuristic Functions**: Pluggable distance estimation for A*
//!
//! ## Features
//!
//! - `visualize`: Enable graph visualization capabilities
//! - `parallel`: Enable parallel pathfinding algorithms

pub mod astar;
pub mod dijkstra;
pub mod error;
pub mod graph;
pub mod heuristic;
pub mod pathfinder;
pub mod priority_queue;

// Re-export main types for convenience
pub use astar::AstarPathfinder;
pub use dijkstra::DijkstraPathfinder;
pub use error::PathfindingError;
pub use graph::{SimpleWeightedGraph, WeightedGraph};
pub use heuristic::{EuclideanHeuristic, Heuristic, HeuristicContext, ManhattanHeuristic};
pub use pathfinder::Pathfinder;
pub use priority_queue::PathfindingQueue;

/// Common result type for pathfinding operations
pub type PathfindingResult<T> = Result<T, PathfindingError>;

/// Node identifier type (can be customized per application)
pub type NodeId = u32;

/// Edge weight type (can be customized per application)  
pub type Weight = f64;
