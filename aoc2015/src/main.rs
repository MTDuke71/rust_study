
use anyhow::{bail, Result};
use std::fs;
use aoc2015::prelude::*;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let day: usize = args.next().ok_or_else(|| anyhow::anyhow!("usage: cargo run -- <day> [input_file]"))?.parse()?;
    
    // If no input file provided, use default: inputs/day{day:02}_example.txt
    let path = args.next().unwrap_or_else(|| format!("inputs/day{day:02}_example.txt"));
    
    let input = fs::read_to_string(&path)
        .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", path, e))?;
    
    let (p1, p2) = run_day(day, &input)?;
    println!("Day {day} Part 1: {p1}");
    println!("Day {day} Part 2: {p2}");
    Ok(())
}
