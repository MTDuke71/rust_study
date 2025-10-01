
use anyhow::Result;
use std::fs;
use aoc2015::prelude::*;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let day: usize = args.next().ok_or_else(|| anyhow::anyhow!("usage: cargo run -- <day> [input_file] [--debug]"))?.parse()?;
    
    // Process remaining arguments
    let mut input_file = None;
    let mut debug = false;
    
    for arg in args {
        if arg == "--debug" || arg == "-d" {
            debug = true;
        } else if input_file.is_none() {
            input_file = Some(arg);
        }
    }
    
    // If no input file provided, use default: inputs/day{day:02}_example.txt
    let path = input_file.unwrap_or_else(|| format!("inputs/day{day:02}_example.txt"));
    
    let input = fs::read_to_string(&path)
        .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", path, e))?;
    
    // For Day 7, use debug-enabled functions if debug flag is set
    let (p1, p2) = if day == 7 && debug {
        use aoc2015::solver::day07::{solve_part1_with_debug, solve_part2_with_debug};
        println!("🐛 Debug mode enabled for Day 7");
        println!("📁 Using input file: {}", path);
        println!();
        (solve_part1_with_debug(&input, true)?, solve_part2_with_debug(&input, true)?)
    } else {
        run_day(day, &input)?
    };
    
    if debug && day == 7 {
        println!("\n📊 Final Results:");
    }
    println!("Day {day} Part 1: {p1}");
    println!("Day {day} Part 2: {p2}");
    Ok(())
}
