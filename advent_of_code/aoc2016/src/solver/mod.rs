use anyhow::{bail, Result};

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
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;

pub fn run_day(day: usize, input: &str) -> Result<(String, String)> {
    match day {
        1 => {
            let (p1, p2) = day01::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        2 => {
            let (p1, p2) = day02::solve(input);
            Ok((p1, p2))
        },
        3 => {
            let (p1, p2) = day03::solve(input);
            Ok((p1, p2))
        },
        4 => {
            let (p1, p2) = day04::solve(input);
            Ok((p1, p2))
        },
        5 => {
            let (p1, p2) = day05::solve(input);
            Ok((p1, p2))
        },
        6 => {
            let (p1, p2) = day06::solve(input);
            Ok((p1, p2))
        },
        7 => {
            let (p1, p2) = day07::solve(input);
            Ok((p1, p2))
        },
        8 => {
            let (p1, p2) = day08::solve(input);
            Ok((p1, p2))
        },
        9 => {
            let (p1, p2) = day09::solve(input);
            Ok((p1, p2))
        },
        10 => {
            let (p1, p2) = day10::solve(input);
            Ok((p1, p2))
        },
        11 => {
            let (p1, p2) = day11::solve(input);
            Ok((p1, p2))
        },

        12 => {
            let (p1, p2) = day12::solve(input);
            Ok((p1, p2))
        },
        13 => {
            let (p1, p2) = day13::solve(input);
            Ok((p1, p2))
        },
        14 => {
            let (p1, p2) = day14::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        15 => {
            let (p1, p2) = day15::solve(input);
            Ok((p1, p2))
        },
        16 => {
            let (p1, p2) = day16::solve(input);
            Ok((p1, p2))
        },
        17 => {
            let (p1, p2) = day17::solve(input);
            Ok((p1, p2))
        },
        18 => {
            let (p1, p2) = day18::solve(input);
            Ok((p1, p2))
        },
        19 => {
            let (p1, p2) = day19::solve(input);
            Ok((p1, p2))
        },
        20 => {
            let (p1, p2) = day20::solve(input);
            Ok((p1, p2))
        },
        21 => {
            let (p1, p2) = day21::solve(input);
            Ok((p1, p2))
        },
        22 => {
            let (p1, p2) = day22::solve(input);
            Ok((p1, p2))
        },
        _ => bail!("Day {day} not implemented yet (valid range: 1-25). To implement day {day}, create src/solver/day{day:02}.rs and uncomment the corresponding lines in mod.rs"),
    }
}
