//! Visual simulation for Day 14: Regolith Reservoir
//!
//! Watch sand fall and accumulate in the cave system!
//!
//! Usage:
//!   cargo run --example day14_visual               # Part 1 (24 grains for example)
//!   cargo run --example day14_visual part2         # Part 2 (93 grains for example)
//!   cargo run --example day14_visual part1 fast    # Faster animation (10ms delay)
//!   cargo run --example day14_visual part1 slow    # Slower animation (100ms delay)
//!   cargo run --example day14_visual part1 50      # Limit to first 50 grains

use aoc2022::solver::day14;

const EXAMPLE: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Parse arguments
    let part = if args.len() > 1 { &args[1] } else { "part1" };
    let speed = if args.len() > 2 { &args[2] } else { "normal" };

    // Determine delay based on speed argument
    let delay_ms = match speed {
        "fast" => 10,
        "slow" => 100,
        "instant" => 1,
        _ => {
            // Check if it's a number (max grains limit)
            speed.parse::<usize>().ok();
            30 // Default normal speed
        }
    };

    // Check for max grains limit
    let max_grains = if args.len() > 2 {
        args[2].parse::<usize>().ok()
    } else {
        None
    };

    // Use example input (small and fast)
    // For actual input, use: include_str!("../inputs/day14.txt")
    let input = EXAMPLE;
    let data = day14::parse(input);

    println!("🏜️  Day 14: Regolith Reservoir - Visual Simulation");
    println!("================================================\n");

    match part {
        "part1" => {
            println!("Part 1: Sand falls until abyss");
            println!("Delay: {}ms per step", delay_ms);
            if let Some(max) = max_grains {
                println!("Limited to first {} grains\n", max);
            }
            println!("Press Ctrl+C to stop\n");

            let count = day14::part1_visual(&data, delay_ms, max_grains);
            println!("\n✅ Simulation complete!");
            println!("Expected: 24, Got: {}", count);
        }
        "part2" => {
            println!("Part 2: Sand accumulates until source blocked");
            println!("Delay: {}ms per step", delay_ms);
            if let Some(max) = max_grains {
                println!("Limited to first {} grains\n", max);
            } else {
                println!("⚠️  Warning: Full Part 2 takes ~93 grains (may be slow)\n");
            }
            println!("Press Ctrl+C to stop\n");

            let count = day14::part2_visual(&data, delay_ms, max_grains);
            println!("\n✅ Simulation complete!");
            println!("Expected: 93, Got: {}", count);
        }
        _ => {
            println!("Usage: cargo run --example day14_visual [part1|part2] [fast|normal|slow|<max_grains>]");
            println!();
            println!("Examples:");
            println!("  cargo run --example day14_visual");
            println!("  cargo run --example day14_visual part2");
            println!("  cargo run --example day14_visual part1 fast");
            println!("  cargo run --example day14_visual part1 50");
        }
    }
}
