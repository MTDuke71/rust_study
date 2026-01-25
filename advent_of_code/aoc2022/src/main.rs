use anyhow::{Context, Result};
use aoc2022::run_day;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <day> [--example]", args[0]);
        eprintln!("       {} all", args[0]);
        std::process::exit(1);
    }
    
    let day_arg = &args[1];
    let use_example = args.get(2).map(|s| s == "--example").unwrap_or(false);
    
    if day_arg == "all" {
        run_all_days()?;
    } else {
        let day: usize = day_arg.parse()
            .context("Day must be a number between 1 and 25")?;
        
        if !(1..=25).contains(&day) {
            anyhow::bail!("Day must be between 1 and 25");
        }
        
        run_single_day(day, use_example)?;
    }
    
    Ok(())
}

fn run_single_day(day: usize, use_example: bool) -> Result<()> {
    let filename = if use_example {
        format!("inputs/day{:02}_example.txt", day)
    } else {
        format!("inputs/day{:02}.txt", day)
    };
    
    let input = fs::read_to_string(&filename)
        .with_context(|| format!("Failed to read {}", filename))?;
    
    let (part1, part2) = run_day(day, &input)?;
    
    println!("Day {} Part 1: {}", day, part1);
    println!("Day {} Part 2: {}", day, part2);
    
    Ok(())
}

fn run_all_days() -> Result<()> {
    println!("Running all implemented days...\n");
    
    for day in 1..=25 {
        let filename = format!("inputs/day{:02}.txt", day);
        
        if let Ok(input) = fs::read_to_string(&filename) {
            match run_day(day, &input) {
                Ok((part1, part2)) => {
                    println!("Day {:2} | Part 1: {:<15} | Part 2: {}", 
                             day, part1, part2);
                }
                Err(_) => continue, // Day not implemented yet
            }
        }
    }
    
    Ok(())
}
