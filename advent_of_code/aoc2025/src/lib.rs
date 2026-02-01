//! # Advent of Code 2024 Solutions
//!
//! **Dual-track learning system** combining algorithmic mastery with idiomatic Rust conversion
//! from existing Python solutions. Demonstrates functional programming patterns, comprehensive
//! error handling, and production-ready code quality.
//!
//! ## 🎯 **Philosophy**
//!
//! This crate implements a **dual learning approach**:
//! - 🧩 **Algorithm Analysis** - Understanding problem patterns and computational complexity
//! - 🦀 **Rust Conversion** - Translating Python solutions into idiomatic Rust with safety and performance improvements
//! - 📚 **Educational Documentation** - Comprehensive analysis of both algorithmic insights and language-specific improvements
//!
//! ## 📚 **Modules**
//!
//! ### Core Infrastructure
//! - [`parser`] - Robust input parsing utilities for AoC formats with Result-based error handling
//! - [`grid`] - 2D coordinate systems and spatial algorithms for grid-based problems
//! - [`solver`] - Day-specific solution implementations with comprehensive testing
//!
//! ## 🧪 **Testing Strategy**
//!
//! Every solution includes comprehensive test coverage:
//! - **Unit tests** - Individual function validation with edge cases
//! - **Integration tests** - End-to-end problem solving verification
//! - **Example tests** - Provided AoC examples with expected outputs
//! - **Error handling tests** - Invalid input and parsing failure scenarios
//!
//! ## 📊 **Educational Value**
//!
//! Each implementation demonstrates:
//! - **Functional patterns** - Iterator chains, Result types, pattern matching
//! - **Performance optimization** - Zero-cost abstractions, in-place operations
//! - **Safety improvements** - Compile-time guarantees vs runtime errors
//! - **Idiomatic Rust** - Ownership semantics, trait usage, error propagation
//!
//! ## 🔗 **Integration with Learning Tracks**
//!
//! This AoC work integrates with the workspace's **3-track learning system**:
//! - **V-Cycle Missions** - Data structure implementations support AoC patterns
//! - **Daily Study** - Functional programming concepts applied in competitive context
//! - **Rust Book** - Language features demonstrated through practical problem-solving
//!
//! ## 🏃‍♂️ **Quick Start**
//!
//! ```rust,no_run
//! use aoc2025::prelude::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Run any day's solution (returns both parts)
//!     let (part1, part2) = run_day(1, "3   4\n4   3\n2   5")?;
//!     println!("Day 1 Part 1: {}, Part 2: {}", part1, part2);
//!     
//!     // Use day-specific solver directly
//!     let result = day01::solve_part1("3   4\n4   3\n2   5")?;
//!     println!("Direct result: {}", result);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## 🔧 **Development Commands**
//!
//! ```bash
//! # Run all tests
//! cargo test -p aoc2024
//!
//! # Run specific day with debug output
//! cargo run -p aoc2024 -- 1 inputs/day01.txt --debug
//!
//! # Test with example data
//! cargo run -p aoc2024 -- 1 tests/data/day01_simple.txt
//!
//! # Documentation with examples
//! cargo doc --open --document-private-items
//! ```

pub mod grid;
pub mod parser;
pub mod solver;

// Re-export a tiny prelude so binaries/tests can `use aoc2024::prelude::*`
pub mod prelude {
    pub use crate::grid::*;
    pub use crate::parser::*;
    pub use crate::solver::{day01, run_day, run_day_with_timeout};
    // Add more days here as you implement them:
    // pub use crate::solver::{day02, day03, day04, day05, day06, day07, day08, day09};
    // pub use crate::solver::{day10, day11, day12};
    // pub use crate::solver::{day13, day14, day15};
    // pub use crate::solver::{day16, day17, day18, day19, day20};
    // pub use crate::solver::{day21, day22, day23, day24, day25};
}
