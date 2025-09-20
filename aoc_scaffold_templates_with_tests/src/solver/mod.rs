
use anyhow::{Result, bail};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

pub fn run_day(day: usize, input: &str) -> Result<(String, String)> {
    match day {
        1 => Ok((day01::solve_part1(input)?, day01::solve_part2(input)?)),
        2 => Ok((day02::solve_part1(input)?, day02::solve_part2(input)?)),
        3 => Ok((day03::solve_part1(input)?, day03::solve_part2(input)?)),
        4 => Ok((day04::solve_part1(input)?, day04::solve_part2(input)?)),
        5 => Ok((day05::solve_part1(input)?, day05::solve_part2(input)?)),
        _ => bail!("day {day} not implemented"),
    }
}
