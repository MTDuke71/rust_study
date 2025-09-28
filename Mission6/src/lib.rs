//! # Mission 6: Grids & 2D Arrays - Spatial Data Structures
//!
//! This library implements 2D grid data structures essential for competitive programming
//! and algorithm development. We focus on coordinate systems, navigation patterns, pathfinding,
//! and spatial operations - all critical for Advent of Code and similar challenges.
//!
//! ## Requirements Fulfilled
//!
//! - **REQ-1**: Generic Grid<T> with safe bounds checking and flexible indexing
//! - **REQ-2**: Coordinate system with direction navigation and distance calculations
//! - **REQ-3**: Pathfinding algorithms (A*, BFS) optimized for 2D spaces
//! - **REQ-4**: AoC-specific utilities (flood fill, connected components, parsing)
//! - **REQ-5**: Performance-optimized memory layout and access patterns
//! - **REQ-6**: Integration with existing Mission collections (HashMap, HashSet)
//!
//! ## Quick Start
//!
//! ```rust
//! use mission6::{Grid, Coord, Direction, PathFinder};
//!
//! // Create a 5x5 grid
//! let mut grid = Grid::new(5, 5, '.');
//! grid[(2, 2)] = '#'; // Set obstacle
//!
//! // Navigate with coordinates
//! let start = Coord::new(0, 0);
//! let end = start.move_in_direction(Direction::East, 2);
//!
//! // Find path avoiding obstacles
//! let path = PathFinder::bfs(&grid, start, end, |cell| *cell != '#');
//! ```
//!
//! ## Performance Characteristics
//!
//! - **Grid Access**: O(1) with bounds checking
//! - **Neighbor Iteration**: O(1) per neighbor (4 or 8 directions)
//! - **BFS Pathfinding**: O(V + E) where V = cells, E = edges
//! - **A* Pathfinding**: O(b^d) where b = branching factor, d = depth
//! - **Flood Fill**: O(n) where n = connected component size
//!
//! ## V-Cycle Development Summary
//!
//! ### Requirements Phase
//! - REQ-1: Grid creation with generic types and bounds checking
//! - REQ-2: Coordinate system with direction-based navigation
//! - REQ-3: Pathfinding with obstacle avoidance and heuristics
//! - REQ-4: AoC utilities for parsing and spatial algorithms
//! - REQ-5: Performance optimization for competitive programming
//! - REQ-6: Integration with Mission 5 HashMap/HashSet patterns
//!
//! ### Design Phase
//! - Generic Grid<T> struct with row-major storage
//! - Coord struct with arithmetic operations and direction methods
//! - Direction enum with rotation and vector operations
//! - PathFinder trait-based system for algorithm composition
//! - Memory-efficient storage and cache-friendly access patterns
//!
//! ### Implementation Phase
//! - Safe indexing with bounds checking and panic-free operations
//! - Iterator patterns for neighbors, rows, columns, and regions
//! - BFS and A* implementations with customizable cost functions
//! - Flood fill and connected component analysis
//! - AoC input parsing for common grid formats
//!
//! ### Verification Phase
//! - Unit tests for each requirement (req1_*, req2_*, etc.)
//! - Property-based tests for coordinate arithmetic
//! - Pathfinding correctness with known optimal solutions
//! - Performance benchmarks against std library alternatives
//!
//! ### Validation Phase
//! - Integration with real AoC problems (2015 Day 3, Day 6, etc.)
//! - Performance comparison with Vec<Vec<T>> naive implementations
//! - Memory usage analysis and optimization verification
//! - Cross-validation with Mission 5 caching patterns
//!
//! This mission bridges the gap between basic collections and advanced algorithms,
//! providing the spatial reasoning tools needed for complex competitive programming challenges.

// Core grid functionality
pub mod grid;
pub mod coord;
pub mod direction;

// Algorithms and utilities
pub mod pathfinding;
pub mod flood_fill;
pub mod aoc_utils;

// Re-export main types for convenience
pub use grid::Grid;
pub use coord::Coord;
pub use direction::Direction;
pub use pathfinding::{PathFinder, AStarResult, BfsResult};
pub use flood_fill::{FloodFill, FloodFillResult};
pub use aoc_utils::{AocGridParser, GridVisualizer, AocAlgorithms};

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // REQ-1: Grid creation and basic operations
    fn req1_grid_creation_and_indexing() {
        let mut grid = Grid::new(3, 3, 0);
        
        // Test dimensions
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid.len(), 9);
        
        // Test indexing
        grid[(1, 1)] = 42;
        assert_eq!(grid[(1, 1)], 42);
        
        // Test coordinate indexing
        let coord = Coord::new(1, 1);
        assert_eq!(grid[coord], 42);
        
        // Test bounds checking
        assert!(!grid.in_bounds(Coord::new(3, 3)));
        assert!(grid.in_bounds(Coord::new(2, 2)));
    }

    #[test] // REQ-2: Coordinate system and navigation
    fn req2_coordinate_navigation() {
        let start = Coord::new(5, 5);
        
        // Test direction movement
        let north = start.move_in_direction(Direction::North, 1);
        assert_eq!(north, Coord::new(5, 4));
        
        let east = start.move_in_direction(Direction::East, 2);
        assert_eq!(east, Coord::new(7, 5));
        
        // Test distance calculations
        assert_eq!(start.manhattan_distance(north), 1);
        assert_eq!(start.manhattan_distance(east), 2);
        
        // Test neighbor generation
        let neighbors: Vec<_> = start.neighbors_4().collect();
        assert_eq!(neighbors.len(), 4);
        assert!(neighbors.contains(&Coord::new(5, 4))); // North
        assert!(neighbors.contains(&Coord::new(6, 5))); // East
    }

    #[test] // REQ-3: Pathfinding algorithms
    fn req3_pathfinding_algorithms() {
        let mut grid = Grid::new(5, 5, '.');
        // Create a simple maze
        grid[(1, 1)] = '#';
        grid[(1, 2)] = '#';
        grid[(1, 3)] = '#';
        
        let start = Coord::new(0, 0);
        let end = Coord::new(4, 4);
        
        // Test BFS pathfinding
        let path = PathFinder::bfs(&grid, start, end, |&cell| cell != '#');
        assert!(path.is_some());
        
        let path = path.unwrap();
        assert!(path.len() > 0);
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], end);
    }

    #[test] // REQ-4: AoC utilities
    fn req4_aoc_utilities() {
        let input = "..#..\n.#...\n....#";
        
        // Test grid parsing
        let grid = AocGridParser::parse_char_grid(input);
        assert_eq!(grid.width(), 5);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid[(2, 0)], '#');
        assert_eq!(grid[(1, 1)], '#');
        
        // Test flood fill
        let mut fill_grid = grid.clone();
        let filled = FloodFill::fill_4(&mut fill_grid, Coord::new(0, 0), '.', 'X');
        assert!(filled.len() > 0);
        assert_eq!(fill_grid[(0, 0)], 'X');
    }

    #[test] // REQ-5: Performance characteristics
    fn req5_performance_characteristics() {
        let size = 100;
        let grid = Grid::new(size, size, 0u32);
        
        // Test that large grids can be created efficiently
        assert_eq!(grid.len(), size * size);
        
        // Test iterator performance (should not allocate)
        let sum: u32 = grid.iter().sum();
        assert_eq!(sum, 0);
        
        // Test coordinate generation performance
        let coord_count = grid.enumerate().count();
        assert_eq!(coord_count, size * size);
    }

    #[test] // REQ-6: Integration with Mission 5 patterns
    fn req6_mission5_integration() {
        use std::collections::{HashMap, HashSet};
        
        let mut grid = Grid::new(3, 3, '.');
        grid[(1, 1)] = 'X';
        
        // Test that Coord works as HashMap key
        let mut coord_map: HashMap<Coord, String> = HashMap::new();
        coord_map.insert(Coord::new(1, 1), "center".to_string());
        assert_eq!(coord_map.get(&Coord::new(1, 1)), Some(&"center".to_string()));
        
        // Test that Coord works in HashSet
        let mut visited: HashSet<Coord> = HashSet::new();
        for (coord, value) in grid.enumerate() {
            if *value == 'X' {
                visited.insert(coord);
            }
        }
        assert!(visited.contains(&Coord::new(1, 1)));
        assert_eq!(visited.len(), 1);
    }
}