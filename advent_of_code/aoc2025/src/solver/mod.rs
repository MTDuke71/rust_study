use anyhow::{bail, Result};
use std::time::Duration;

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
pub mod day10_alt; // Alternative Z3-based implementation
pub mod day11;
pub mod day12;

pub fn run_day(day: usize, input: &str) -> Result<(String, String)> {
    match day {
        1 => Ok((day01::solve_part1(input)?, day01::solve_part2(input)?)),
        2 => Ok((
            day02::solve_part1(input)?.to_string(),
            day02::solve_part2(input)?.to_string(),
        )),
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

/// Run a day's solution with timeout protection for each part
///
/// This prevents suboptimal implementations from hanging indefinitely.
/// Each part gets its own timeout window.
pub async fn run_day_with_timeout(
    day: usize,
    input: &str,
    timeout: Duration,
) -> Result<(String, String)> {
    let input_owned = input.to_string();

    // Run Part 1 with timeout
    let input_clone = input_owned.clone();
    let part1 = match tokio::time::timeout(
        timeout,
        tokio::task::spawn_blocking(move || run_single_part(day, &input_clone, 1)),
    )
    .await
    {
        Ok(Ok(result)) => result?,
        Ok(Err(e)) => bail!("Part 1 panicked: {}", e),
        Err(_) => bail!(
            "⏱️  Part 1 exceeded {} second timeout - implementation may be suboptimal",
            timeout.as_secs()
        ),
    };

    println!("✅ Part 1 completed: {}", part1);

    // Run Part 2 with timeout
    let part2 = match tokio::time::timeout(
        timeout,
        tokio::task::spawn_blocking(move || run_single_part(day, &input_owned, 2)),
    )
    .await
    {
        Ok(Ok(result)) => result?,
        Ok(Err(e)) => bail!("Part 2 panicked: {}", e),
        Err(_) => bail!(
            "⏱️  Part 2 exceeded {} second timeout - implementation may be suboptimal",
            timeout.as_secs()
        ),
    };

    Ok((part1, part2))
}

/// Helper to run a single part (1 or 2) for a given day
fn run_single_part(day: usize, input: &str, part: usize) -> Result<String> {
    match (day, part) {
        (1, 1) => day01::solve_part1(input),
        (1, 2) => day01::solve_part2(input),
        (2, 1) => Ok(day02::solve_part1(input)?.to_string()),
        (2, 2) => Ok(day02::solve_part2(input)?.to_string()),
        (3, 1) => day03::solve_part1(input),
        (3, 2) => day03::solve_part2(input),
        (4, 1) => day04::solve_part1(input),
        (4, 2) => day04::solve_part2(input),
        (5, 1) => day05::solve_part1(input),
        (5, 2) => day05::solve_part2(input),
        (6, 1) => day06::solve_part1(input),
        (6, 2) => day06::solve_part2(input),
        (7, 1) => day07::solve_part1(input),
        (7, 2) => day07::solve_part2(input),
        (8, 1) => day08::solve_part1(input),
        (8, 2) => day08::solve_part2(input),
        (9, 1) => day09::solve_part1(input),
        (9, 2) => day09::solve_part2(input),
        (10, 1) => day10::solve_part1(input),
        (10, 2) => day10::solve_part2(input),
        (11, 1) => day11::solve_part1(input),
        (11, 2) => day11::solve_part2(input),
        (12, 1) => day12::solve_part1(input),
        (12, 2) => day12::solve_part2(input),
        _ => bail!("Day {} part {} not implemented", day, part),
    }
}
