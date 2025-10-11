//! # Mission 6 Tutorial Library
//!
//! This tutorial library provides step-by-step learning examples for mastering
//! grids, 2D arrays, and spatial algorithms in Rust.
//!
//! ## Tutorial Progression
//!
//! The tutorial follows a 7-step progressive learning approach:
//!
//! 1. **Grid Setup** - Basic grid creation and type safety
//! 2. **Grid Indexing** - Safe access patterns and iterators  
//! 3. **Coordinates** - Navigation and distance calculations
//! 4. **Pathfinding** - BFS and A* algorithm implementation
//! 5. **AoC Utilities** - Flood fill and competitive programming patterns
//! 6. **Performance** - Optimization and benchmarking techniques
//! 7. **Documentation** - Integration and professional examples
//!
//! ## Using This Library
//!
//! Each tutorial step is designed to be self-contained and executable:
//!
//! ```bash
//! cargo run --example step1_grid_setup
//! cargo run --example step2_grid_indexing
//! # ... continue through step7
//! ```
//!
//! ## Learning Objectives
//!
//! By the end of this tutorial you will have mastered:
//! - 2D grid representation and memory layout
//! - Coordinate system navigation and bounds checking
//! - Pathfinding algorithms (BFS, A*) for grid-based problems
//! - Flood fill operations and connected component analysis
//! - Performance optimization for large grid operations
//! - Integration patterns with other Rust data structures
//!
//! ## Integration with Mission6
//!
//! This tutorial is designed to complement Mission6 (the main implementation):
//! - Tutorial provides guided learning and hands-on practice
//! - Mission6 provides production-ready implementation with comprehensive testing
//! - Both should be completed together for maximum learning impact

/// Tutorial helper functions and common patterns
pub mod tutorial_helpers {
    use std::fmt;

    /// Simple 2D coordinate for tutorial examples
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TutorialCoord {
        pub x: usize,
        pub y: usize,
    }

    impl TutorialCoord {
        pub fn new(x: usize, y: usize) -> Self {
            Self { x, y }
        }

        pub fn manhattan_distance(&self, other: &TutorialCoord) -> usize {
            let dx = self.x.abs_diff(other.x);
            let dy = self.y.abs_diff(other.y);
            dx + dy
        }
    }

    impl fmt::Display for TutorialCoord {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    /// Simple grid for tutorial demonstrations
    #[derive(Debug, Clone)]
    pub struct TutorialGrid<T> {
        data: Vec<T>,
        width: usize,
        height: usize,
    }

    impl<T: Clone> TutorialGrid<T> {
        pub fn new(width: usize, height: usize, default_value: T) -> Self {
            Self {
                data: vec![default_value; width * height],
                width,
                height,
            }
        }

        pub fn width(&self) -> usize {
            self.width
        }

        pub fn height(&self) -> usize {
            self.height
        }

        pub fn get(&self, coord: TutorialCoord) -> Option<&T> {
            if coord.x < self.width && coord.y < self.height {
                Some(&self.data[coord.y * self.width + coord.x])
            } else {
                None
            }
        }

        pub fn set(&mut self, coord: TutorialCoord, value: T) -> bool {
            if coord.x < self.width && coord.y < self.height {
                self.data[coord.y * self.width + coord.x] = value;
                true
            } else {
                false
            }
        }

        pub fn in_bounds(&self, coord: TutorialCoord) -> bool {
            coord.x < self.width && coord.y < self.height
        }
    }

    /// Visualization helper for tutorial examples
    impl<T: fmt::Display + Clone> fmt::Display for TutorialGrid<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for y in 0..self.height {
                for x in 0..self.width {
                    let coord = TutorialCoord::new(x, y);
                    if let Some(value) = self.get(coord) {
                        write!(f, "{}", value)?;
                    }
                }
                if y < self.height - 1 {
                    writeln!(f)?;
                }
            }
            Ok(())
        }
    }

    /// Print section header for tutorial organization
    pub fn print_section(title: &str) {
        println!("\n🔷 {}", title);
        println!("{}", "─".repeat(50));
    }

    /// Print step completion message
    pub fn print_step_complete(step: &str) {
        println!("\n✅ {} - Complete!", step);
    }
}
