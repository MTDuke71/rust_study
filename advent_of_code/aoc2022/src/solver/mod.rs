use anyhow::{bail, Result};

// Import implemented days - uncomment as you implement them
pub mod day01;
// pub mod day02;
// ... etc

pub fn run_day(day: usize, input: &str) -> Result<(String, String)> {
    match day {
        1 => {
            let (p1, p2) = day01::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        // 2 => Ok((day02::solve_part1(input)?, day02::solve_part2(input)?)),
        // ... etc

        _ => bail!("Day {day} not implemented yet (valid range: 1-25). To implement day {day}, create src/solver/day{day:02}.rs and uncomment the corresponding lines in mod.rs"),
    }
}
