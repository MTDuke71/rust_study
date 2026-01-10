use anyhow::Result;
use aoc2023::prelude::*;
use std::fs;
use std::time::Instant;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let day: usize = args
        .next()
        .ok_or_else(|| {
            anyhow::anyhow!("usage: cargo run -p aoc2023 -- <day> [input_file] [--debug]")
        })?
        .parse()?;

    // Process remaining arguments
    let mut input_file = None;
    let mut debug = false;
    let mut bench = false;

    for arg in args {
        if arg == "--debug" || arg == "-d" {
            debug = true;
        } else if arg == "--bench" || arg == "-b" {
            bench = true;
        } else if input_file.is_none() {
            input_file = Some(arg);
        }
    }

    // If no input file provided, use default: inputs/day{day:02}.txt
    let path =
        input_file.unwrap_or_else(|| format!("advent_of_code/aoc2023/inputs/day{day:02}.txt"));

    let input = fs::read_to_string(&path)
        .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", path, e))?;

    if bench {
        // Benchmark mode: run multiple iterations and report timing
        const WARMUP: usize = 10;
        const ITERATIONS: usize = 100;

        // Warmup
        for _ in 0..WARMUP {
            let _ = run_day(day, &input)?;
        }

        // Time both parts by running the full solver multiple times
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _ = run_day(day, &input)?;
        }
        let total_time = start.elapsed() / ITERATIONS as u32;

        let (p1, p2) = run_day(day, &input)?;

        println!("Day {day} Part 1: {p1}");
        println!("Day {day} Part 2: {p2}");
        println!("Total: {:?}", total_time);
    } else {
        // Normal mode
        let start = Instant::now();
        let (p1, p2) = run_day(day, &input)?;
        let elapsed = start.elapsed();

        if debug {
            println!("\n📊 Final Results:");
        }
        println!("Day {day} Part 1: {p1}");
        println!("Day {day} Part 2: {p2}");
        if debug {
            println!("Time: {:?}", elapsed);
        }
    }
    Ok(())
}
