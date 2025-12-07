use anyhow::Result;
use aoc2025::prelude::*;
use std::fs;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let day: usize = args
        .next()
        .ok_or_else(|| anyhow::anyhow!("usage: cargo run -- <day> [input_file] [--debug]"))?
        .parse()?;

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

    // If no input file provided, use default: inputs/day{day:02}.txt
    let path = input_file.unwrap_or_else(|| format!("inputs/day{day:02}.txt"));

    let input = fs::read_to_string(&path)
        .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", path, e))?;

    // Run solution for the specified day with 10-second timeout
    let timeout_duration = Duration::from_secs(10);
    
    println!("⏱️  Running with {}-second timeout per part...", timeout_duration.as_secs());
    
    let (p1, p2) = match tokio::time::timeout(timeout_duration * 2, async {
        run_day_with_timeout(day, &input, timeout_duration).await
    }).await {
        Ok(result) => result?,
        Err(_) => {
            anyhow::bail!("❌ Total execution exceeded {} seconds", timeout_duration.as_secs() * 2);
        }
    };

    if debug {
        println!("\n📊 Final Results:");
    }
    println!("✅ Day {day} Part 1: {p1}");
    println!("✅ Day {day} Part 2: {p2}");
    Ok(())
}
