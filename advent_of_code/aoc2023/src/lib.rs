//! # Advent of Code 2023 Solutions
//!
//! **Two-pass learning system** emphasizing Rust mastery through competitive programming.
//! Following the workflow guide: Phase 1 (45min solve) + Phase 2 (1hr polish).
//!
//! ## 🎯 **Philosophy**
//!
//! This crate implements a **focused two-pass approach**:
//! - 🏃 **Phase 1 (Solve)** - Get working solution in 45 minutes, test with examples
//! - 🔧 **Phase 2 (Polish)** - Refactor, document, extract patterns, optimize (1 hour)
//! - 📚 **Pattern Extraction** - Build reusable algorithms library in `src/patterns/`
//! - 🦀 **Rust Mastery** - Idiomatic code, zero-copy optimizations, trait implementations
//!
//! ## 📚 **Modules**
//!
//! ### Core Infrastructure
//! - [`parser`] - Robust input parsing utilities for AoC formats
//! - [`grid`] - 2D coordinate systems and spatial algorithms
//! - [`solver`] - Day-specific solution implementations
//!
//! ## 🔗 **Mission Integration**
//!
//! Reuses validated mission components:
//! - **Mission 5** - HashMap/HashSet for frequency counting, caching
//! - **Mission 6** - Grid for 2D pathfinding, cellular automata
//! - **Mission 8** - Graph algorithms (BFS, DFS, shortest paths)
//! - **Mission 10** - UnionFind for connected components, grouping
//!
//! ## 🏃‍♂️ **Quick Start**
//!
//! ```bash
//! # Run with real input
//! cargo run -p aoc2023 -- 1
//!
//! # Run with custom input file
//! cargo run -p aoc2023 -- 1 advent_of_code/aoc2023/inputs/day01_example.txt
//!
//! # Run with debug output
//! cargo run -p aoc2023 -- 1 --debug
//!
//! # Run tests
//! cargo test -p aoc2023
//!
//! # Benchmarks
//! cargo bench -p aoc2023
//! ```

pub mod parser;
pub mod solver;

// Re-export prelude for easy imports
pub mod prelude {
    pub use crate::parser::*;
    pub use crate::solver::run_day;
}
