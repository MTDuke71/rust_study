use anyhow::{bail, Result};

// Import implemented days - uncomment as you implement them
// pub mod day01;
// pub mod day02;
// pub mod day03;
// ... etc

pub fn run_day(day: usize, _input: &str) -> Result<(String, String)> {
    match day {
        // Uncomment as you implement each day:
        // 1 => Ok((day01::solve_part1(input)?, day01::solve_part2(input)?)),
        // 2 => Ok((day02::solve_part1(input)?, day02::solve_part2(input)?)),
        // 3 => Ok((day03::solve_part1(input)?, day03::solve_part2(input)?)),
        // ... continue for all 25 days
        _ => bail!("Day {} not implemented yet", day),
    }
}
