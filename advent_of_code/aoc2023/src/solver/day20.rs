//! Day 20: Pulse Propagation - Digital Logic Circuit Simulation
//!
//! **Problem**: Simulate a network of interconnected modules (flip-flops, conjunctions, broadcaster)
//! processing high/low pulses in FIFO order. Count total pulses after 1000 button presses.
//!
//! **Approach**:
//! - Part 1: State machine simulation with pulse queue (BFS-style processing)
//! - Parse module types, destinations, and build dependency graph
//! - Process pulses in queue order (not recursively!)
//!
//! **Key Patterns**:
//! - State machine with multiple module types
//! - FIFO queue processing (Mission 2 pattern)
//! - HashMap for module lookup (Mission 5 pattern)
//!
//! **Module Types**:
//! - Flip-flop (`%`): Toggle on low pulse (ignore high), emit opposite state
//! - Conjunction (`&`): Remember last pulse from each input, emit low only if ALL high
//! - Broadcaster: Relay pulse to all destinations
//!
//! **Zettelkasten Links**: [[state-machine-rust]], [[mission-2]], [[mission-5]]

use std::collections::{HashMap, VecDeque};
use crate::math_utils::lcm;

/// Pulse type: High or Low
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pulse {
    Low,
    High,
}

/// Module type with state
#[derive(Debug, Clone)]
pub enum Module {
    /// Flip-flop: stores on/off state
    FlipFlop { on: bool, destinations: Vec<String> },
    /// Conjunction: stores last pulse from each input
    Conjunction {
        memory: HashMap<String, Pulse>,
        destinations: Vec<String>,
    },
    /// Broadcaster: simple relay
    Broadcaster { destinations: Vec<String> },
}

impl Module {
    /// Get destinations for this module
    fn destinations(&self) -> &[String] {
        match self {
            Module::FlipFlop { destinations, .. } => destinations,
            Module::Conjunction { destinations, .. } => destinations,
            Module::Broadcaster { destinations } => destinations,
        }
    }

    /// Process incoming pulse, return outgoing pulse if any
    /// Returns None if pulse should be ignored (e.g., flip-flop receiving high)
    fn process(&mut self, from: &str, pulse: Pulse) -> Option<Pulse> {
        match self {
            Module::FlipFlop { on, .. } => {
                // Flip-flop ignores high pulses
                if pulse == Pulse::High {
                    None
                } else {
                    // Flip state on low pulse
                    *on = !*on;
                    Some(if *on { Pulse::High } else { Pulse::Low })
                }
            }
            Module::Conjunction { memory, .. } => {
                // Update memory for this input
                memory.insert(from.to_string(), pulse);
                
                // If all inputs are high, send low; otherwise send high
                let all_high = memory.values().all(|&p| p == Pulse::High);
                Some(if all_high { Pulse::Low } else { Pulse::High })
            }
            Module::Broadcaster { .. } => {
                // Broadcaster just relays the pulse
                Some(pulse)
            }
        }
    }
}

/// Parse module from input line
/// Format: "broadcaster -> a, b, c" or "%a -> b" or "&inv -> a"
fn parse_module(line: &str) -> (String, Module) {
    let (name_part, dest_part) = line.split_once(" -> ").expect("invalid format");
    
    let destinations: Vec<String> = dest_part
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    if name_part == "broadcaster" {
        ("broadcaster".to_string(), Module::Broadcaster { destinations })
    } else if let Some(name) = name_part.strip_prefix('%') {
        (
            name.to_string(),
            Module::FlipFlop {
                on: false,
                destinations,
            },
        )
    } else if let Some(name) = name_part.strip_prefix('&') {
        (
            name.to_string(),
            Module::Conjunction {
                memory: HashMap::new(), // Will be initialized later with inputs
                destinations,
            },
        )
    } else {
        panic!("Unknown module type: {}", name_part);
    }
}

/// Build module network from input
/// Also initializes conjunction module memory with all inputs
fn parse_input(input: &str) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = input
        .lines()
        .map(|line| parse_module(line.trim()))
        .collect();

    // Build reverse mapping: for each module, track what modules send to it
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    for (name, module) in &modules {
        for dest in module.destinations() {
            inputs
                .entry(dest.clone())
                .or_insert_with(Vec::new)
                .push(name.clone());
        }
    }

    // Initialize conjunction module memory with all inputs (defaulting to Low)
    for (name, module) in modules.iter_mut() {
        if let Module::Conjunction { memory, .. } = module {
            if let Some(input_names) = inputs.get(name) {
                for input_name in input_names {
                    memory.insert(input_name.clone(), Pulse::Low);
                }
            }
        }
    }

    modules
}

pub fn part1(input: &str) -> u64 {
    let mut modules = parse_input(input);
    
    let mut low_count = 0u64;
    let mut high_count = 0u64;

    // Press button 1000 times
    for _ in 0..1000 {
        // Queue of (source, destination, pulse)
        let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();
        
        // Button press sends low pulse to broadcaster
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        low_count += 1;

        // Process all pulses in queue (FIFO order - critical!)
        while let Some((from, to, pulse)) = queue.pop_front() {
            // Get the destination module (might not exist for "output" modules)
            if let Some(module) = modules.get_mut(&to) {
                // Process pulse, get output pulse if any
                if let Some(output_pulse) = module.process(&from, pulse) {
                    // Send output pulse to all destinations
                    for dest in module.destinations().to_vec() {
                        queue.push_back((to.clone(), dest, output_pulse));
                        match output_pulse {
                            Pulse::Low => low_count += 1,
                            Pulse::High => high_count += 1,
                        }
                    }
                }
            }
        }
    }

    low_count * high_count
}

// GCD/LCM functions imported from crate::math_utils for reuse

pub fn part2(input: &str) -> u64 {
    let mut modules = parse_input(input);
    
    // Find the module that feeds into "rx"
    // In the puzzle input, this is a conjunction module
    let rx_input = modules
        .iter()
        .find(|(_, module)| module.destinations().contains(&"rx".to_string()))
        .map(|(name, _)| name.clone())
        .expect("No module feeds rx");

    // Get the inputs to the rx feeder (these must all be HIGH for rx to get LOW)
    let rx_feeder_inputs: Vec<String> = modules
        .iter()
        .filter(|(_, module)| module.destinations().contains(&rx_input))
        .map(|(name, _)| name.clone())
        .collect();

    // Track cycle length for each input to the rx feeder
    let mut cycle_lengths: HashMap<String, u64> = HashMap::new();
    let mut button_presses = 0u64;

    // Keep pressing button until we find all cycles
    while cycle_lengths.len() < rx_feeder_inputs.len() {
        button_presses += 1;
        
        let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

        while let Some((from, to, pulse)) = queue.pop_front() {
            // Check if this is one of the inputs to rx_feeder sending HIGH
            if to == rx_input && pulse == Pulse::High && !cycle_lengths.contains_key(&from) {
                cycle_lengths.insert(from.clone(), button_presses);
            }

            if let Some(module) = modules.get_mut(&to) {
                if let Some(output_pulse) = module.process(&from, pulse) {
                    for dest in module.destinations().to_vec() {
                        queue.push_back((to.clone(), dest, output_pulse));
                    }
                }
            }
        }
    }

    // The answer is the LCM of all cycle lengths
    cycle_lengths.values().copied().reduce(lcm).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        // After 1 button press: 8 low, 4 high
        // After 1000 presses: 8000 low, 4000 high = 32000000
        assert_eq!(part1(input), 32000000);
    }

    #[test]
    fn test_example2() {
        let input = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        // After 1000 presses: 4250 low, 2750 high = 11687500
        assert_eq!(part1(input), 11687500);
    }
}
