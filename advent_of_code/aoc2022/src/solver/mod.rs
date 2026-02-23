use anyhow::{bail, Result};

// Import implemented days - uncomment as you implement them
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
pub mod day23;
// ... etc

pub fn run_day(day: usize, input: &str) -> Result<(String, String)> {
    match day {
        1 => {
            let (p1, p2) = day01::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        2 => {
            let (p1, p2) = day02::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        3 => {
            let (p1, p2) = day03::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        4 => {
            let (p1, p2) = day04::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        5 => {
            let (p1, p2) = day05::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        6 => {
            let (p1, p2) = day06::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        7 => {
            let (p1, p2) = day07::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        8 => {
            let (p1, p2) = day08::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        9 => {
            let (p1, p2) = day09::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        10 => {
            let (p1, p2) = day10::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        11 => {
            let (p1, p2) = day11::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        12 => {
            let (p1, p2) = day12::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        13 => {
            let (p1, p2) = day13::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        14 => {
            let data = day14::parse(input);
            let p1 = day14::part1(&data);
            let p2 = day14::part2(&data);
            Ok((p1.to_string(), p2.to_string()))
        },
        15 => {
            let sensors = day15::parse(input);
            let p1 = day15::part1(&sensors);
            let p2 = day15::part2(&sensors);
            Ok((p1.to_string(), p2.to_string()))
        },
        16 => {
            let (p1, p2) = day16::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        17 => {
            let (p1, p2) = day17::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        18 => {
            let (p1, p2) = day18::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        19 => {
            let (p1, p2) = day19::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        20 => {
            let (p1, p2) = day20::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        21 => {
            let (p1, p2) = day21::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        22 => {
            let (p1, p2) = day22::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        23 => {
            let (p1, p2) = day23::solve(input);
            Ok((p1.to_string(), p2.to_string()))
        },
        // ... etc

        _ => bail!("Day {day} not implemented yet (valid range: 1-25). To implement day {day}, create src/solver/day{day:02}.rs and uncomment the corresponding lines in mod.rs"),
    }
}
