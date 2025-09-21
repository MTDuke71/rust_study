
pub mod parser;
pub mod grid;
pub mod solver;

// Re-export a tiny prelude so binaries/tests can `use aoc_scaffold::prelude::*`
pub mod prelude {
    pub use crate::parser::*;
    pub use crate::grid::*;
    pub use crate::solver::{run_day, day01, day02, day03, day04, day05};
}
