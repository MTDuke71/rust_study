
pub mod parser;
pub mod grid;
pub mod solver;

// Re-export a tiny prelude so binaries/tests can `use aoc2015::prelude::*`
pub mod prelude {
    pub use crate::parser::*;
    pub use crate::grid::*;
    pub use crate::solver::{run_day, day01, day02, day03, day04, day05};
    // Add more days here as you implement them:
    // pub use crate::solver::{day06, day07, day08, day09, day10};
    // pub use crate::solver::{day11, day12, day13, day14, day15};
    // pub use crate::solver::{day16, day17, day18, day19, day20};
    // pub use crate::solver::{day21, day22, day23, day24, day25};
}
