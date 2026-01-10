//! # Day 8: Haunted Wasteland
//!
//! Navigate through a network of nodes following L/R instructions.
//!
//! ## Part 1
//! Starting at `AAA`, follow left/right instructions until reaching `ZZZ`.
//! Count the number of steps required.
//!
//! ## Part 2
//! Ghost navigation: Start at all nodes ending with 'A', find when all paths
//! simultaneously reach nodes ending with 'Z'. Uses LCM optimization to avoid
//! simulating trillions of steps.
//!
//! ## Algorithm
//! - Parse instructions and node network into HashMap (Mission 5)
//! - Part 1: Simple path following - O(n) where n is number of steps
//! - Part 2: Cycle detection + LCM - O(k × m) where k is number of ghosts, m is avg cycle length
//! - Complexity: O(n) for parsing, O(log n) for LCM calculation
//!
//! ## Mathematical Foundation
//!
//! **Graph Theory**: Network represented as directed graph with labeled edges.
//! See `zettelkasten/math-foundations/graph-theory-fundamentals.md` for theory.
//!
//! **Number Theory**: Ghost synchronization uses GCD/LCM for cycle alignment.
//! See `zettelkasten/math-foundations/number-theory-basics.md` for theory.
//!
//! ## Mission Integration
//! - **Mission 5 HashMap**: O(1) node lookups
//!
//! ## Performance
//! - Part 1: ~1.5ms (19,637 steps)
//! - Part 2: ~6.7ms (8,811,050,362,409 steps via LCM - brute force impossible)

use anyhow::{Context, Result};
use std::collections::HashMap;

/// Parsed network representation
#[derive(Debug, Clone)]
struct Network {
    /// Left/Right instructions
    instructions: Vec<char>,
    /// Node mapping: node -> (left_node, right_node)
    nodes: HashMap<String, (String, String)>,
}

impl Network {
    /// Parse input into Network structure
    ///
    /// # Input Format
    /// ```text
    /// LLR
    ///
    /// AAA = (BBB, CCC)
    /// BBB = (DDD, EEE)
    /// ```
    fn parse(input: &str) -> Result<Self> {
        let mut lines = input.lines();

        // Parse instructions (first line)
        let instructions: Vec<char> = lines
            .next()
            .context("Missing instructions line")?
            .chars()
            .collect();

        // Skip empty line
        lines.next();

        // Parse node mappings
        let mut nodes = HashMap::new();
        for line in lines {
            if line.trim().is_empty() {
                continue;
            }

            // Format: "AAA = (BBB, CCC)"
            let parts: Vec<&str> = line.split(" = ").collect();
            if parts.len() != 2 {
                anyhow::bail!("Invalid line format: {}", line);
            }

            let node = parts[0].to_string();

            // Parse (left, right)
            let lr = parts[1]
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(", ")
                .collect::<Vec<_>>();

            if lr.len() != 2 {
                anyhow::bail!("Invalid node pair: {}", parts[1]);
            }

            nodes.insert(node, (lr[0].to_string(), lr[1].to_string()));
        }

        Ok(Network {
            instructions,
            nodes,
        })
    }

    /// Navigate from start to end node following instructions
    fn navigate(&self, start: &str, end: &str) -> Result<usize> {
        let mut current = start.to_string();
        let mut steps = 0;
        let mut instruction_idx = 0;

        while current != end {
            let (left, right) = self
                .nodes
                .get(&current)
                .context(format!("Node {} not found", current))?;

            let instruction = self.instructions[instruction_idx];
            current = match instruction {
                'L' => left.clone(),
                'R' => right.clone(),
                _ => anyhow::bail!("Invalid instruction: {}", instruction),
            };

            steps += 1;
            instruction_idx = (instruction_idx + 1) % self.instructions.len();
        }

        Ok(steps)
    }

    /// Navigate from start node until reaching any node ending with suffix
    /// Returns (steps, end_node)
    fn navigate_until_suffix(&self, start: &str, suffix: char) -> Result<(usize, String)> {
        let mut current = start.to_string();
        let mut steps = 0;
        let mut instruction_idx = 0;

        // Take at least one step (important when start already ends with suffix)
        loop {
            let (left, right) = self
                .nodes
                .get(&current)
                .context(format!("Node {} not found", current))?;

            let instruction = self.instructions[instruction_idx];
            current = match instruction {
                'L' => left.clone(),
                'R' => right.clone(),
                _ => anyhow::bail!("Invalid instruction: {}", instruction),
            };

            steps += 1;
            instruction_idx = (instruction_idx + 1) % self.instructions.len();

            // Check if we reached the target
            if current.ends_with(suffix) {
                break;
            }
        }

        Ok((steps, current))
    }

    /// Find all nodes ending with a specific character
    fn find_nodes_ending_with(&self, suffix: char) -> Vec<String> {
        self.nodes
            .keys()
            .filter(|node| node.ends_with(suffix))
            .cloned()
            .collect()
    }

    /// Navigate all ghost paths simultaneously (using LCM optimization)
    fn navigate_ghosts(&self) -> Result<usize> {
        // Find all starting nodes (ending with 'A')
        let start_nodes = self.find_nodes_ending_with('A');

        if start_nodes.is_empty() {
            anyhow::bail!("No starting nodes found (nodes ending with 'A')");
        }

        println!("\n=== Ghost Path Analysis ===");
        println!("Total ghosts: {}", start_nodes.len());
        println!();

        // For each starting node, find cycle length to reach node ending with 'Z'
        let mut cycle_lengths = Vec::new();
        for start in &start_nodes {
            let (first_steps, first_z_node) = self.navigate_until_suffix(start, 'Z')?;

            // Verify cycle: continue from first **Z to next **Z
            let (cycle_steps, second_z_node) = self.navigate_until_suffix(&first_z_node, 'Z')?;

            println!("Ghost starting at {}:", start);
            println!(
                "  {} → {} in {} steps (initial)",
                start, first_z_node, first_steps
            );
            println!(
                "  {} → {} in {} steps (cycle)",
                first_z_node, second_z_node, cycle_steps
            );

            if first_steps == cycle_steps && first_z_node == second_z_node {
                println!("  ✓ Perfect cycle: period = {} steps", cycle_steps);
            } else {
                println!(
                    "  ⚠ Different periods! Initial: {}, Cycle: {}",
                    first_steps, cycle_steps
                );
            }
            println!();

            // Use the cycle period (not initial period) for LCM
            cycle_lengths.push(cycle_steps);
        }

        // Find LCM of all cycle lengths
        let result = cycle_lengths.iter().fold(1, |acc, &x| lcm(acc, x));

        println!("LCM of all cycle lengths: {}", result);
        println!("===========================\n");

        Ok(result)
    }
}

/// Calculate Greatest Common Divisor using Euclidean algorithm
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Calculate Least Common Multiple
fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

pub fn solve_part1(input: &str) -> Result<String> {
    let network = Network::parse(input)?;
    let steps = network.navigate("AAA", "ZZZ")?;
    Ok(steps.to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let network = Network::parse(input)?;
    let steps = network.navigate_ghosts()?;
    Ok(steps.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_GHOSTS: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_parse_network() {
        let network = Network::parse(EXAMPLE1).unwrap();
        assert_eq!(network.instructions, vec!['R', 'L']);
        assert_eq!(network.nodes.len(), 7);
        assert_eq!(
            network.nodes.get("AAA"),
            Some(&("BBB".to_string(), "CCC".to_string()))
        );
    }

    #[test]
    fn test_part1_example1() {
        let result = solve_part1(EXAMPLE1).unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_part1_example2() {
        let result = solve_part1(EXAMPLE2).unwrap();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 5), 1);
        assert_eq!(gcd(100, 50), 50);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(21, 6), 42);
        assert_eq!(lcm(12, 18), 36);
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE_GHOSTS).unwrap();
        assert_eq!(result, "6");
    }
}
