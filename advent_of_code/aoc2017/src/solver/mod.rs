use anyhow::{bail, Result};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub fn run_day(day: usize, input: &str) -> Result<(String, String)> {
    match day {
        1 => {
            let (p1, p2) = day01::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        }
        2 => {
            let (p1, p2) = day02::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        }
        3 => {
            let (p1, p2) = day03::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        }
        4 => {
            let (p1, p2) = day04::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        }
        _ => bail!("Day {day} not implemented yet (valid range: 1-25). To implement day {day}, create src/solver/day{day:02}.rs and add it to mod.rs"),
    }
}
