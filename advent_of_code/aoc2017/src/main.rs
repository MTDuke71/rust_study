use anyhow::{Context, Result};
use aoc2017::run_day;
use std::env;
use std::fs;
use std::time::Instant;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day> [--example] [--bench[=N]]", args[0]);
        eprintln!("       {} all [--bench[=N]]", args[0]);
        eprintln!();
        eprintln!("Flags:");
        eprintln!("  --example    Use dayXX_example.txt input");
        eprintln!("  --bench      Time the solve (single run)");
        eprintln!("  --bench=100  Benchmark with N iterations");
        std::process::exit(1);
    }

    let day_arg = &args[1];
    let flags: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();
    let use_example = flags.contains(&"--example");
    let bench_iters = parse_bench_flag(&flags);

    if day_arg == "all" {
        run_all_days(bench_iters)?;
    } else {
        let day: usize = day_arg
            .parse()
            .context("Day must be a number between 1 and 25")?;

        if !(1..=25).contains(&day) {
            anyhow::bail!("Day must be between 1 and 25");
        }

        run_single_day(day, use_example, bench_iters)?;
    }

    Ok(())
}

/// Parse --bench or --bench=N from flags. Returns None if no bench flag, Some(n) otherwise.
fn parse_bench_flag(flags: &[&str]) -> Option<usize> {
    for flag in flags {
        if *flag == "--bench" {
            return Some(1);
        }
        if let Some(n) = flag.strip_prefix("--bench=") {
            return Some(n.parse().unwrap_or(1));
        }
    }
    None
}

fn run_single_day(day: usize, use_example: bool, bench_iters: Option<usize>) -> Result<()> {
    let filename = if use_example {
        format!("inputs/day{:02}_example.txt", day)
    } else {
        format!("inputs/day{:02}.txt", day)
    };

    let input =
        fs::read_to_string(&filename).with_context(|| format!("Failed to read {}", filename))?;

    match bench_iters {
        Some(n) => bench_day(day, &input, n)?,
        None => {
            let (part1, part2) = run_day(day, &input)?;
            println!("Day {} Part 1: {}", day, part1);
            println!("Day {} Part 2: {}", day, part2);
        }
    }

    Ok(())
}

fn bench_day(day: usize, input: &str, iters: usize) -> Result<()> {
    // Warmup run
    let (part1, part2) = run_day(day, input)?;

    let start = Instant::now();
    for _ in 0..iters {
        let _ = run_day(day, input)?;
    }
    let elapsed = start.elapsed();

    let avg = elapsed / iters as u32;
    println!(
        "Day {:2} | Part 1: {:<20} | Part 2: {:<20} | {:>10} ({} iters)",
        day,
        part1,
        part2,
        format_duration(avg),
        iters
    );

    Ok(())
}

fn run_all_days(bench_iters: Option<usize>) -> Result<()> {
    let iters = bench_iters.unwrap_or(1);
    let is_bench = bench_iters.is_some();

    if is_bench {
        println!("Benchmarking all days ({} iters each)...\n", iters);
    } else {
        println!("Running all implemented days...\n");
    }

    let mut total = std::time::Duration::ZERO;
    let mut days_run = 0;

    for day in 1..=25 {
        let filename = format!("inputs/day{:02}.txt", day);

        if let Ok(input) = fs::read_to_string(&filename) {
            match run_day(day, &input) {
                Ok((part1, part2)) => {
                    if is_bench {
                        let start = Instant::now();
                        for _ in 0..iters {
                            let _ = run_day(day, &input);
                        }
                        let elapsed = start.elapsed();
                        let avg = elapsed / iters as u32;
                        total += avg;
                        days_run += 1;
                        println!(
                            "Day {:2} | Part 1: {:<20} | Part 2: {:<20} | {:>10}",
                            day,
                            part1,
                            part2,
                            format_duration(avg)
                        );
                    } else {
                        let start = Instant::now();
                        let _ = run_day(day, &input);
                        let elapsed = start.elapsed();
                        total += elapsed;
                        days_run += 1;
                        println!(
                            "Day {:2} | Part 1: {:<20} | Part 2: {:<20} | {:>10}",
                            day,
                            part1,
                            part2,
                            format_duration(elapsed)
                        );
                    }
                }
                Err(_) => continue,
            }
        }
    }

    if days_run > 0 {
        println!("{}", "-".repeat(80));
        println!("{:>58} Total: {:>10}", "", format_duration(total));
    }

    Ok(())
}

fn format_duration(d: std::time::Duration) -> String {
    let nanos = d.as_nanos();
    if nanos < 1_000 {
        format!("{}ns", nanos)
    } else if nanos < 1_000_000 {
        format!("{:.2}\u{00b5}s", nanos as f64 / 1_000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.2}ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.2}s", nanos as f64 / 1_000_000_000.0)
    }
}
