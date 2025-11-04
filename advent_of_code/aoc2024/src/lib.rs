//! # Advent of Code 2015 Solutions
//!
//! **Professional-grade AoC solutions** demonstrating V-Cycle development methodology,
//! comprehensive testing, multiple algorithmic approaches, and production-ready code quality.
//!
//! ## 🎯 **Philosophy**
//!
//! This crate treats AoC problems as **engineering challenges** with:
//! - Requirements-driven development (REQ-1, REQ-2, etc.)
//! - Test-driven development with comprehensive coverage
//! - Multiple implementation approaches for educational comparison
//! - Performance analysis and optimization
//! - Visual representations and interactive examples
//!
//! ## 📚 **Modules**
//!
//! ### Core Infrastructure
//! - [`parser`] - Robust input parsing utilities for AoC formats
//! - [`grid`] - 2D coordinate systems and spatial algorithms
//! - [`solver`] - Day-specific solution implementations
//!
//! ### Advanced Features  
//! - **Examples** - Alternative algorithms, visualizations, and comparisons
//! - **Mission Integration** - Examples demonstrating Mission5 MemoCache and other workspace components
//!
//! ## 🚀 **Examples Directory**
//!
//! The `examples/` folder contains **alternative implementations** and **educational content**:
//!
//! ### Day 6: Probably a Fire Hazard
//! - **`day06_hashmap.rs`** - HashMap-based alternative to Grid implementation
//! - **`day06_verification.rs`** - Cross-validation between Grid and HashMap approaches  
//! - **`day06_visualizer.rs`** - BMP bitmap generation with Christmas tree Easter eggs
//! - **`day06_brightness_visualizer.rs`** - RGB heat map visualization
//! - **`HASHMAP_ANALYSIS.md`** - Comprehensive performance and design analysis
//!
//! ### Day 4: The Ideal Stocking Stuffer
//! - **`day04_unit_test_examples.rs`** - Demonstrates TDD methodology
//! - **`md5_analysis.rs`** - MD5 hash analysis and optimization patterns
//! - **`optimized_brute_force.rs`** - Performance optimization techniques
//!
//! ### Day 10: Elves Look, Elves Say
//! - **`day10_with_memo.rs`** - Mission5 MemoCache integration for look-and-say sequences
//!
//! ## 🧪 **Testing Strategy**
//!
//! Every solution includes multiple test categories:
//! - **Unit tests** - Individual function validation
//! - **Integration tests** - End-to-end problem solving
//! - **Example tests** - Provided AoC examples
//! - **Performance tests** - Benchmark comparisons
//! - **Cross-validation** - Multiple algorithm verification
//!
//! ## 🎨 **Visualization & Interactivity**
//!
//! Selected problems include visual representations:
//! - **Bitmap generation** (Day 6 light grids)
//! - **Heat map visualizations** (Day 6 brightness patterns)
//! - **Christmas Easter eggs** (Festive patterns in visualizations)
//! - **Performance comparisons** (Algorithm benchmarking)
//!
//! ## 📊 **Educational Value**
//!
//! Each implementation demonstrates:
//! - **Multiple approaches** - Grid vs HashMap, iterative vs recursive
//! - **Trade-off analysis** - Performance vs memory vs readability  
//! - **Real-world patterns** - Caching, memoization, optimization
//! - **Engineering discipline** - Requirements traceability, comprehensive testing
//!
//! ## 🔗 **Integration with Learning Tracks**
//!
//! This AoC work integrates with the workspace's **3-track learning system**:
//! - **V-Cycle Missions** - Mission6 (Grids) supports Day 6 solutions
//! - **Daily Study** - HashMap/HashSet concepts from Week 2 applied to Day 6 alternative
//! - **Rust Book** - Ownership, traits, and collections concepts in practical use
//!
//! ## 🏃‍♂️ **Quick Start**
//!
//! ```rust,no_run
//! use aoc2015::prelude::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Run any day's solution (returns both parts)
//!     let (part1, part2) = run_day(6, "turn on 0,0 through 999,999")?;
//!     println!("Day 6 Part 1: {}, Part 2: {}", part1, part2);
//!     
//!     // Use Grid-based solver directly
//!     let grid_result = day06::solve_part1("turn on 0,0 through 2,2")?;
//!     println!("Grid result: {}", grid_result);
//!     
//!     Ok(())
//! }
//! ```
//!
//! **Examples to run**:
//! ```bash
//! # Cross-validate implementations
//! cargo run --example day06_verification
//!
//! # Try HashMap alternative  
//! cargo run --example day06_hashmap
//!
//! # Generate visualizations
//! cargo run --example day06_visualizer
//! ```
//!
//! ## 🔧 **Development Commands**
//!
//! ```bash
//! # Run all tests
//! cargo test
//!
//! # Generate visualizations
//! cargo run --example day06_visualizer
//! cargo run --example day06_brightness_visualizer
//!
//! # Performance comparison
//! cargo run --example day06_verification
//!
//! # Documentation with examples
//! cargo doc --open --document-private-items
//!
//! # Run specific day
//! cargo run --bin day06
//! ```

pub mod grid;
pub mod parser;
pub mod solver;
// pub mod day10_with_memo;  // Example of using Mission5 MemoCache (TODO: Create when needed)

// Re-export a tiny prelude so binaries/tests can `use aoc2015::prelude::*`
pub mod prelude {
    pub use crate::grid::*;
    pub use crate::parser::*;
    pub use crate::solver::{day01, run_day};
    // pub use crate::day10_with_memo::*;
    // Add more days here as you implement them:
    // pub use crate::solver::{day02, day03, day04, day05, day06, day07, day08, day09};
    // pub use crate::solver::{day10, day11, day12};
    // pub use crate::solver::{day13, day14, day15};
    // pub use crate::solver::{day16, day17, day18, day19, day20};
    // pub use crate::solver::{day21, day22, day23, day24, day25};
}
