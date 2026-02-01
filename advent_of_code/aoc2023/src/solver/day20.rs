//! Day 20: Pulse Propagation - Digital Logic Circuit Simulation
//!
//! **Problem**: Simulate a network of interconnected modules (flip-flops, conjunctions, broadcaster)
//! processing high/low pulses in FIFO order. Count total pulses after 1000 button presses.
//!
//! **Approach**:
//! - Part 1: State machine simulation with pulse queue (BFS-style processing)
//! - Part 2: Cycle detection + LCM for synchronization (identical pattern to Day 8)
//!
//! **Mathematical Foundation**:
//!
//! This problem demonstrates four key mathematical concepts:
//!
//! 1. **State Machines** - Each module is a finite state machine with transition rules:
//!    - FlipFlop: Boolean state (on/off), transitions on low pulse
//!    - Conjunction: Map state (memory of inputs), transitions on any pulse
//!    - Broadcaster: Stateless relay
//!
//!    See `zettelkasten/math-foundations/state-machines.md` for theory
//!
//! 2. **Linear Feedback Shift Registers (LFSRs)** - The 4 chains are LFSRs with feedback loops.
//!    Different feedback polynomials → different periods despite same length!
//!    See `zettelkasten/math-foundations/linear-feedback-shift-registers.md`
//!
//! 3. **Cycle Detection** - By Pigeonhole Principle, deterministic state machines on finite
//!    state spaces must eventually cycle. We exploit this to avoid 238 trillion iterations.
//!    See `zettelkasten/math-foundations/pigeonhole-principle-cycle-detection.md`
//!
//! 4. **Number Theory (LCM)** - When multiple independent cycles must align, the synchronization
//!    point is the Least Common Multiple of their periods.
//!    See `zettelkasten/math-foundations/number-theory-basics.md`
//!
//! **Key Patterns**:
//! - State machine with multiple module types (enum dispatch)
//! - FIFO queue processing (Mission 2 pattern)
//! - HashMap for module lookup (Mission 5 pattern)
//! - Cycle detection + LCM (same as Day 8 ghost synchronization)
//!
//! **Module Types**:
//! - Flip-flop (`%`): Toggle on low pulse (ignore high), emit opposite state
//! - Conjunction (`&`): Remember last pulse from each input, emit low only if ALL high
//! - Broadcaster: Relay pulse to all destinations
//!
//! **Zettelkasten Links**:
//! - [[state-machines]] - Finite state machine theory
//! - [[pigeonhole-principle-cycle-detection]] - Why cycles are guaranteed
//! - [[number-theory-basics]] - LCM for cycle alignment
//! - [[mission-2]] - Queue data structure
//! - [[mission-5]] - HashMap implementation

use crate::math_utils::lcm;
use std::collections::{HashMap, VecDeque};

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

    let destinations: Vec<String> = dest_part.split(", ").map(|s| s.to_string()).collect();

    if name_part == "broadcaster" {
        (
            "broadcaster".to_string(),
            Module::Broadcaster { destinations },
        )
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
            inputs.entry(dest.clone()).or_default().push(name.clone());
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

/// Debug function: Visualize circuit topology from broadcaster to rx
/// Shows module types, connections, and hierarchical structure
#[allow(dead_code)]
pub fn visualize_circuit(input: &str) {
    let modules = parse_input(input);

    println!("\n╔═══════════════════════════════════════════════════════════════════╗");
    println!("║          Day 20 Circuit Topology Visualization                    ║");
    println!(
        "║          Total Modules: {}                                        ║",
        modules.len()
    );
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    // Build reverse mapping for easier traversal
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    for (name, module) in &modules {
        for dest in module.destinations() {
            inputs.entry(dest.clone()).or_default().push(name.clone());
        }
    }

    // Helper to get module type symbol
    let get_type_symbol = |name: &str| -> (&str, &str) {
        if let Some(module) = modules.get(name) {
            match module {
                Module::FlipFlop { .. } => ("%", "FlipFlop"),
                Module::Conjunction { .. } => ("&", "Conjunction"),
                Module::Broadcaster { .. } => (" ", "Broadcaster"),
            }
        } else {
            (" ", "Output")
        }
    };

    // BFS traversal from broadcaster
    let mut visited = std::collections::HashSet::new();
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    queue.push_back(("broadcaster".to_string(), 0));
    visited.insert("broadcaster".to_string());

    let mut level_groups: HashMap<usize, Vec<String>> = HashMap::new();

    while let Some((name, level)) = queue.pop_front() {
        level_groups.entry(level).or_default().push(name.clone());

        if let Some(module) = modules.get(&name) {
            for dest in module.destinations() {
                if !visited.contains(dest) {
                    visited.insert(dest.clone());
                    queue.push_back((dest.clone(), level + 1));
                }
            }
        }
    }

    // Print by levels
    let max_level = *level_groups.keys().max().unwrap_or(&0);

    for level in 0..=max_level {
        if let Some(nodes) = level_groups.get(&level) {
            println!("Level {}: {} modules", level, nodes.len());
            println!("─────────────────────────────────────────────────────────────────");

            for node_name in nodes {
                let (symbol, type_name) = get_type_symbol(node_name);
                let module_inputs = inputs.get(node_name).cloned().unwrap_or_default();

                print!("  {}{:<15} ({:<12})", symbol, node_name, type_name);

                if let Some(module) = modules.get(node_name) {
                    let dests = module.destinations();
                    if !dests.is_empty() {
                        print!(" → [");
                        for (i, dest) in dests.iter().enumerate() {
                            if i > 0 {
                                print!(", ");
                            }
                            let (dest_symbol, _) = get_type_symbol(dest);
                            print!("{}{}", dest_symbol, dest);
                        }
                        print!("]");
                    }
                } else {
                    print!(" [OUTPUT]");
                }

                if !module_inputs.is_empty() {
                    print!("  ← [");
                    for (i, input) in module_inputs.iter().enumerate() {
                        if i > 0 {
                            print!(", ");
                        }
                        let (input_symbol, _) = get_type_symbol(input);
                        print!("{}{}", input_symbol, input);
                    }
                    print!("]");
                }

                println!();
            }
            println!();
        }
    }

    // Find and highlight the path to rx
    println!("═══════════════════════════════════════════════════════════════════");
    println!("Critical Path to 'rx' (Part 2 Analysis):");
    println!("═══════════════════════════════════════════════════════════════════\n");

    if let Some(rx_inputs) = inputs.get("rx") {
        for rx_feeder in rx_inputs {
            let (symbol, type_name) = get_type_symbol(rx_feeder);
            println!("rx ← {}{} ({})", symbol, rx_feeder, type_name);

            if let Some(feeder_inputs) = inputs.get(rx_feeder) {
                println!(
                    "\n{}{} has {} inputs (must ALL be HIGH to send LOW to rx):",
                    symbol,
                    rx_feeder,
                    feeder_inputs.len()
                );
                for (i, input) in feeder_inputs.iter().enumerate() {
                    let (input_symbol, input_type) = get_type_symbol(input);
                    println!(
                        "  {}. {}{:<15} ({:<12})",
                        i + 1,
                        input_symbol,
                        input,
                        input_type
                    );

                    // Show what feeds into each counter
                    if let Some(counter_inputs) = inputs.get(input) {
                        println!(
                            "     ↑ Fed by {} modules: {:?}",
                            counter_inputs.len(),
                            counter_inputs
                        );
                    }
                }
            }
        }
    } else {
        println!("No module feeds 'rx' (check for 'rx' in destinations)");
    }

    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("Module Type Summary:");
    println!("═══════════════════════════════════════════════════════════════════");

    let mut flip_flops = 0;
    let mut conjunctions = 0;
    let mut broadcasters = 0;

    for module in modules.values() {
        match module {
            Module::FlipFlop { .. } => flip_flops += 1,
            Module::Conjunction { .. } => conjunctions += 1,
            Module::Broadcaster { .. } => broadcasters += 1,
        }
    }

    println!("  Flip-Flops (%)    : {}", flip_flops);
    println!("  Conjunctions (&)  : {}", conjunctions);
    println!("  Broadcasters      : {}", broadcasters);
    println!("  Total             : {}", modules.len());
    println!("═══════════════════════════════════════════════════════════════════\n");
}

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

    // Safety limit: prevent infinite loop if problem is invalid
    // Set to 4000 to demonstrate safety escape (actual cycles are 3793-4007)
    const MAX_ITERATIONS: u64 = 5_000;

    // Keep pressing button until we find all cycles
    while cycle_lengths.len() < rx_feeder_inputs.len() {
        button_presses += 1;

        // Safety check: prevent infinite loop on invalid input
        if button_presses > MAX_ITERATIONS {
            panic!(
                "Exceeded {} iterations without finding all cycles. Found {}/{} cycles: {:?}",
                MAX_ITERATIONS,
                cycle_lengths.len(),
                rx_feeder_inputs.len(),
                cycle_lengths
            );
        }

        let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

        while let Some((from, to, pulse)) = queue.pop_front() {
            // Check if this is one of the inputs to rx_feeder sending HIGH
            if to == rx_input && pulse == Pulse::High && !cycle_lengths.contains_key(&from) {
                println!(
                    "  Found cycle for '{}' at iteration {}",
                    from, button_presses
                );
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
