//! Day 10: Balance Bots — Flow Diagram Generator
//!
//! Traces the simulation step-by-step and outputs a text flow diagram.
//! Usage: cargo run -p aoc2016 --example day10_flow

use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug, Clone, Copy)]
enum Dest {
    Bot(usize),
    Output(usize),
}

impl std::fmt::Display for Dest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dest::Bot(id) => write!(f, "bot {id}"),
            Dest::Output(id) => write!(f, "OUTPUT {id}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    low: Dest,
    high: Dest,
}

fn main() {
    let input = include_str!("../inputs/day10.txt");
    let mut output = String::new();

    // Parse
    let mut assignments: Vec<(usize, usize)> = Vec::new();
    let mut rules: HashMap<usize, Rule> = HashMap::new();

    for line in input.lines() {
        let w: Vec<&str> = line.split_whitespace().collect();
        if w[0] == "value" {
            let value: usize = w[1].parse().unwrap();
            let bot: usize = w[5].parse().unwrap();
            assignments.push((value, bot));
        } else {
            let bot_id: usize = w[1].parse().unwrap();
            let low_id: usize = w[6].parse().unwrap();
            let high_id: usize = w[11].parse().unwrap();
            let low = if w[5] == "bot" { Dest::Bot(low_id) } else { Dest::Output(low_id) };
            let high = if w[10] == "bot" { Dest::Bot(high_id) } else { Dest::Output(high_id) };
            rules.insert(bot_id, Rule { low, high });
        }
    }

    // Header
    writeln!(output, "Day 10: Balance Bots — Chip Flow Diagram").unwrap();
    writeln!(output, "==========================================").unwrap();
    writeln!(output).unwrap();

    // Initial assignments
    writeln!(output, "=== INITIAL CHIP ASSIGNMENTS ===").unwrap();
    writeln!(output).unwrap();
    let mut sorted_assignments = assignments.clone();
    sorted_assignments.sort_by_key(|&(val, _)| val);
    for &(value, bot) in &sorted_assignments {
        writeln!(output, "  value {value:>2} ──→ bot {bot}").unwrap();
    }
    writeln!(output).unwrap();

    // Simulate with trace
    let mut chips: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut outputs: HashMap<usize, usize> = HashMap::new();

    for &(value, bot) in &assignments {
        chips.entry(bot).or_default().push(value);
    }

    writeln!(output, "=== SIMULATION TRACE ===").unwrap();
    writeln!(output).unwrap();

    let mut step = 0;
    loop {
        // Find bot with 2 chips (pick lowest bot_id for deterministic output)
        let mut ready: Vec<usize> = chips.iter()
            .filter(|(_, v)| v.len() == 2)
            .map(|(&id, _)| id)
            .collect();
        ready.sort();

        if ready.is_empty() {
            break;
        }

        for &bot_id in &ready {
            step += 1;
            let mut held = chips.remove(&bot_id).unwrap();
            held.sort();
            let (lo, hi) = (held[0], held[1]);

            let rule = rules[&bot_id];

            let marker = if lo == 17 && hi == 61 {
                " ◄◄◄ PART 1 ANSWER: This bot compares 17 and 61!"
            } else {
                ""
            };

            writeln!(output,
                "Step {step:>3}: bot {bot_id:>3} fires  [{lo:>2}, {hi:>2}]  ──→  low({lo:>2}) to {:<12}  high({hi:>2}) to {:<12}{marker}",
                format!("{}", rule.low),
                format!("{}", rule.high),
            ).unwrap();

            // Distribute
            match rule.low {
                Dest::Bot(id) => chips.entry(id).or_default().push(lo),
                Dest::Output(id) => { outputs.insert(id, lo); }
            }
            match rule.high {
                Dest::Bot(id) => chips.entry(id).or_default().push(hi),
                Dest::Output(id) => { outputs.insert(id, hi); }
            }
        }
    }

    writeln!(output).unwrap();
    writeln!(output, "=== SUMMARY ===").unwrap();
    writeln!(output).unwrap();
    writeln!(output, "Total steps (bot firings): {step}").unwrap();
    writeln!(output, "Chips in system: {}", assignments.len()).unwrap();
    writeln!(output, "Output bins filled: {}", outputs.len()).unwrap();

    writeln!(output).unwrap();
    writeln!(output, "=== OUTPUT BINS ===").unwrap();
    writeln!(output).unwrap();
    let mut sorted_outputs: Vec<_> = outputs.iter().collect();
    sorted_outputs.sort_by_key(|(&id, _)| id);
    for (&id, &val) in &sorted_outputs {
        let marker = if id <= 2 { " ◄ Part 2" } else { "" };
        writeln!(output, "  output {id:>2} = {val:>2}{marker}").unwrap();
    }

    writeln!(output).unwrap();
    let p2 = outputs[&0] * outputs[&1] * outputs[&2];
    writeln!(output, "Part 2: output[0] × output[1] × output[2] = {} × {} × {} = {}",
        outputs[&0], outputs[&1], outputs[&2], p2).unwrap();

    // Trace the path of chips 17 and 61 specifically
    writeln!(output).unwrap();
    writeln!(output, "=== CHIP 17 AND CHIP 61 JOURNEY ===").unwrap();
    writeln!(output).unwrap();

    // Re-simulate to track specific chips
    let mut chips: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(value, bot) in &assignments {
        chips.entry(bot).or_default().push(value);
    }

    let mut chip17_path: Vec<String> = Vec::new();
    let mut chip61_path: Vec<String> = Vec::new();

    // Find initial locations
    for &(value, bot) in &assignments {
        if value == 17 { chip17_path.push(format!("value 17 ──→ bot {bot}")); }
        if value == 61 { chip61_path.push(format!("value 61 ──→ bot {bot}")); }
    }

    loop {
        let mut ready: Vec<usize> = chips.iter()
            .filter(|(_, v)| v.len() == 2)
            .map(|(&id, _)| id)
            .collect();
        ready.sort();
        if ready.is_empty() { break; }

        for &bot_id in &ready {
            let mut held = chips.remove(&bot_id).unwrap();
            held.sort();
            let (lo, hi) = (held[0], held[1]);
            let rule = rules[&bot_id];

            // Track chip 17
            if lo == 17 {
                chip17_path.push(format!("bot {bot_id} (low) ──→ {}", rule.low));
            }
            if hi == 17 {
                chip17_path.push(format!("bot {bot_id} (high) ──→ {}", rule.high));
            }

            // Track chip 61
            if lo == 61 {
                chip61_path.push(format!("bot {bot_id} (low) ──→ {}", rule.low));
            }
            if hi == 61 {
                chip61_path.push(format!("bot {bot_id} (high) ──→ {}", rule.high));
            }

            match rule.low {
                Dest::Bot(id) => chips.entry(id).or_default().push(lo),
                Dest::Output(id) => { let _ = id; }
            }
            match rule.high {
                Dest::Bot(id) => chips.entry(id).or_default().push(hi),
                Dest::Output(id) => { let _ = id; }
            }
        }
    }

    writeln!(output, "Chip 17:").unwrap();
    for (i, step) in chip17_path.iter().enumerate() {
        let prefix = if i == 0 { "  " } else { "  → " };
        writeln!(output, "{prefix}{step}").unwrap();
    }
    writeln!(output).unwrap();
    writeln!(output, "Chip 61:").unwrap();
    for (i, step) in chip61_path.iter().enumerate() {
        let prefix = if i == 0 { "  " } else { "  → " };
        writeln!(output, "{prefix}{step}").unwrap();
    }

    // Write to file
    let path = "advent_of_code/aoc2016/Problem_Statements/days/day10_flow.txt";
    std::fs::write(path, &output).unwrap();
    println!("Flow diagram written to {path}");
    println!();
    print!("{output}");
}
