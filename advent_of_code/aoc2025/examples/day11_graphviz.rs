//! Emit a Graphviz DOT representation of AoC 2025 Day 11's device graph.
//!
//! Highlights the 4 key nodes from Part 2:
//! - `svr` (start)
//! - `dac` and `fft` (required)
//! - `out` (target)
//!
//! Usage:
//!   cargo run -p aoc2025 --example day11_graphviz > day11.dot
//!   dot -Tsvg day11.dot > day11.svg
//!   # or paste day11.dot into an online Graphviz viewer

use std::collections::{BTreeSet, HashMap};

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let (source, targets) = line.split_once(": ")?;
            let neighbors = targets
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>();

            Some((source.to_string(), neighbors))
        })
        .collect()
}

fn main() {
    let input = include_str!("../inputs/day11.txt");
    let graph = parse_input(input);

    // Collect all nodes (including nodes that only appear as targets).
    let mut nodes: BTreeSet<String> = BTreeSet::new();
    for (src, dsts) in &graph {
        nodes.insert(src.clone());
        for dst in dsts {
            nodes.insert(dst.clone());
        }
    }

    // DOT header.
    println!("digraph day11 {{");
    println!("  rankdir=LR;");
    println!("  bgcolor=\"transparent\";");
    println!("  node [shape=circle, fontname=\"Segoe UI\", fontsize=10];");
    println!("  edge [color=\"#777777\", arrowsize=0.7];");
    println!();

    // Key nodes.
    // Keep styling simple so it renders consistently in online viewers.
    println!("  svr [style=filled, fillcolor=\"#FFD166\", penwidth=2];");
    println!("  dac [style=filled, fillcolor=\"#06D6A0\", penwidth=2];");
    println!("  fft [style=filled, fillcolor=\"#06D6A0\", penwidth=2];");
    println!("  out [style=filled, fillcolor=\"#EF476F\", penwidth=2];");
    println!();

    // Emit edges.
    for (src, dsts) in &graph {
        for dst in dsts {
            println!("  {} -> {};", src, dst);
        }
    }

    // Ensure any isolated nodes still appear (shouldn't happen, but harmless).
    println!();
    for node in nodes {
        // Don’t re-emit key nodes; they already have styling.
        if node == "svr" || node == "dac" || node == "fft" || node == "out" {
            continue;
        }
        if !graph.contains_key(&node) {
            // A node that only appears as a target.
            println!("  {};", node);
        }
    }

    println!("}}");
}
