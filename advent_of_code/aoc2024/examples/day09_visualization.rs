use anyhow::{Context, Result};
use aoc2024::solver::day09;
use std::env;
use std::fs;

fn load_input() -> Result<String> {
    if let Some(path) = env::args().nth(1) {
        fs::read_to_string(&path).with_context(|| format!("Failed to read {path}"))
    } else {
        Ok(include_str!("../inputs/day09_example.txt")
            .trim()
            .to_string())
    }
}

fn print_frames(label: &str, frames: &[String]) {
    println!("\n=== {label} ({} frames captured) ===", frames.len());
    for (idx, frame) in frames.iter().enumerate() {
        println!("Step {idx:02}: {frame}");
    }
}

fn main() -> Result<()> {
    let input = load_input()?;
    let part1_frames = day09::visualize_part1(&input, 15)?;
    let part2_frames = day09::visualize_part2(&input, 15)?;

    println!("Day 09 Visualization using input length {}", input.len());
    print_frames("Part 1 block compaction", &part1_frames);
    print_frames("Part 2 whole-file compaction", &part2_frames);

    Ok(())
}
