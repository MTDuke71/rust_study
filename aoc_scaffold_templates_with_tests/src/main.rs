
use anyhow::{bail, Result};
use std::fs;
use aoc_scaffold::prelude::*;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let day: usize = args.next().ok_or_else(|| anyhow::anyhow!("usage: cargo run -- <day> <input_file>"))?.parse()?;
    let path = args.next().ok_or_else(|| anyhow::anyhow!("usage: cargo run -- <day> <input_file>"))?;
    let input = fs::read_to_string(path)?;
    let (p1, p2) = run_day(day, &input)?;
    println!("Day {day} Part 1: {p1}");
    println!("Day {day} Part 2: {p2}");
    Ok(())
}
