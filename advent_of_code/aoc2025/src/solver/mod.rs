use anyhow::{bail, Result};

// Import implemented days - only uncomment when days are implemented
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;

pub fn run_day(day: usize, input: &str) -> Result<(String, String)> {
    match day {
        1 => Ok((day01::solve_part1(input)?, day01::solve_part2(input)?)),
        2 => Ok((day02::solve_part1(input)?.to_string(), day02::solve_part2(input)?.to_string())),
        3 => Ok((day03::solve_part1(input)?, day03::solve_part2(input)?)),
        4 => Ok((day04::solve_part1(input)?, day04::solve_part2(input)?)),
        5 => Ok((day05::solve_part1(input)?, day05::solve_part2(input)?)),
        6 => Ok((day06::solve_part1(input)?, day06::solve_part2(input)?)),
        7 => Ok((day07::solve_part1(input)?, day07::solve_part2(input)?)),
        8 => Ok((day08::solve_part1(input)?, day08::solve_part2(input)?)),
        9 => Ok((day09::solve_part1(input)?, day09::solve_part2(input)?)),
        10 => Ok((day10::solve_part1(input)?, day10::solve_part2(input)?)),
        11 => Ok((day11::solve_part1(input)?, day11::solve_part2(input)?)),
        12 => Ok((day12::solve_part1(input)?, day12::solve_part2(input)?)),
        

        _ => bail!("Day {day} not implemented yet (valid range: 1-12)."),
    }
}
