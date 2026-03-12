//! Day 11: Optimal path trace for Part 1
//!
//! Runs BFS with parent tracking to reconstruct the shortest path,
//! then writes a step-by-step floor diagram to Problem_Statements/days/day11_trace.txt

use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    elevator: u8,
    pairs: Vec<(u8, u8)>, // (gen_floor, chip_floor) per element, sorted
}

impl State {
    fn canonicalize(&mut self) {
        self.pairs.sort();
    }

    fn is_goal(&self) -> bool {
        self.pairs.iter().all(|&(g, m)| g == 3 && m == 3)
    }

    fn is_valid(&self) -> bool {
        for &(gen_floor, chip_floor) in &self.pairs {
            if gen_floor != chip_floor {
                if self.pairs.iter().any(|&(g, _)| g == chip_floor) {
                    return false;
                }
            }
        }
        true
    }
}

fn apply_move(state: &mut State, item: (usize, bool), to: u8) {
    let (idx, is_gen) = item;
    if is_gen {
        state.pairs[idx].0 = to;
    } else {
        state.pairs[idx].1 = to;
    }
}

fn parse_input(input: &str) -> (State, Vec<String>) {
    let input = input.replace("\r\n", "\n");
    let mut elements: Vec<String> = Vec::new();
    let mut gen_floors: Vec<u8> = Vec::new();
    let mut chip_floors: Vec<u8> = Vec::new();

    for (floor_idx, line) in input.trim().lines().enumerate() {
        let floor = floor_idx as u8;
        let words: Vec<&str> = line.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            if *word == "generator" || word.starts_with("generator") {
                let element = words[i - 1].trim_start_matches("a ").to_string();
                if let Some(pos) = elements.iter().position(|e| e == &element) {
                    gen_floors[pos] = floor;
                } else {
                    elements.push(element);
                    gen_floors.push(floor);
                    chip_floors.push(255);
                }
            }
            if word.contains("compatible") {
                let element = word.split('-').next().unwrap().to_string();
                if let Some(pos) = elements.iter().position(|e| e == &element) {
                    chip_floors[pos] = floor;
                } else {
                    elements.push(element);
                    gen_floors.push(255);
                    chip_floors.push(floor);
                }
            }
        }
    }

    let pairs: Vec<(u8, u8)> = gen_floors.into_iter().zip(chip_floors).collect();
    let mut state = State { elevator: 0, pairs };
    // Sort elements alongside pairs so labels stay matched
    let mut indexed: Vec<(String, (u8, u8))> = elements.into_iter().zip(state.pairs.iter().copied()).collect();
    indexed.sort_by_key(|(_, p)| *p);
    let names: Vec<String> = indexed.iter().map(|(n, _)| n.clone()).collect();
    state.pairs = indexed.into_iter().map(|(_, p)| p).collect();

    (state, names)
}

/// BFS with parent tracking — returns the full optimal path.
fn bfs_with_path(initial: State) -> Vec<State> {
    let mut visited: HashSet<State> = HashSet::new();
    let mut parent: HashMap<State, State> = HashMap::new();
    let mut queue: VecDeque<State> = VecDeque::new();

    visited.insert(initial.clone());
    queue.push_back(initial.clone());

    let mut goal_state: Option<State> = None;

    while let Some(state) = queue.pop_front() {
        if state.is_goal() {
            goal_state = Some(state);
            break;
        }

        let floor = state.elevator;
        let mut items_on_floor: Vec<(usize, bool)> = Vec::new();

        for (i, &(g, m)) in state.pairs.iter().enumerate() {
            if g == floor {
                items_on_floor.push((i, true));
            }
            if m == floor {
                items_on_floor.push((i, false));
            }
        }

        let new_floors: Vec<u8> = match floor {
            0 => vec![1],
            3 => vec![2],
            _ => vec![floor + 1, floor - 1],
        };

        for &new_floor in &new_floors {
            // 2-item moves
            for i in 0..items_on_floor.len() {
                for j in (i + 1)..items_on_floor.len() {
                    let mut next = state.clone();
                    next.elevator = new_floor;
                    apply_move(&mut next, items_on_floor[i], new_floor);
                    apply_move(&mut next, items_on_floor[j], new_floor);
                    next.canonicalize();

                    if next.is_valid() && visited.insert(next.clone()) {
                        parent.insert(next.clone(), state.clone());
                        queue.push_back(next);
                    }
                }
            }
            // 1-item moves
            for &item in &items_on_floor {
                let mut next = state.clone();
                next.elevator = new_floor;
                apply_move(&mut next, item, new_floor);
                next.canonicalize();

                if next.is_valid() && visited.insert(next.clone()) {
                    parent.insert(next.clone(), state.clone());
                    queue.push_back(next);
                }
            }
        }
    }

    // Reconstruct path
    let goal = goal_state.expect("No solution found");
    let mut path = vec![goal.clone()];
    let mut current = goal;
    while let Some(prev) = parent.get(&current) {
        path.push(prev.clone());
        current = prev.clone();
    }
    path.reverse();
    path
}

/// Format a state as a floor diagram.
fn format_state(state: &State, labels: &[String], step: usize) -> String {
    let mut out = String::new();
    if step == 0 {
        out.push_str("--- INITIAL STATE ---\n");
    } else {
        out.push_str(&format!("--- Step {step} ---\n"));
    }

    // Build abbreviated labels: first 2 chars, capitalized
    let abbrevs: Vec<String> = labels
        .iter()
        .map(|name| {
            let mut chars = name.chars();
            let first = chars.next().unwrap_or('?').to_ascii_uppercase();
            let second = chars.next().unwrap_or('?').to_ascii_lowercase();
            format!("{first}{second}")
        })
        .collect();

    for floor in (0u8..4).rev() {
        let elevator = if state.elevator == floor { "E " } else { ". " };
        let mut items = String::new();
        for (i, &(g, m)) in state.pairs.iter().enumerate() {
            if g == floor {
                items.push_str(&format!(" {}G", abbrevs[i]));
            } else {
                items.push_str("  . ");
            }
            if m == floor {
                items.push_str(&format!(" {}M", abbrevs[i]));
            } else {
                items.push_str("  . ");
            }
        }
        out.push_str(&format!("F{} {}{}\n", floor + 1, elevator, items));
    }
    out
}

/// Describe what moved between two states.
///
/// Since pairs are canonicalized (sorted), we can't track by index.
/// Instead, count generators and chips per floor in both states and diff.
fn describe_move(prev: &State, next: &State, _labels: &[String]) -> String {
    let dir = if next.elevator > prev.elevator { "UP" } else { "DOWN" };
    let from = prev.elevator;
    let to = next.elevator;

    // Count generators and chips per floor for each state
    let mut prev_gens = [0u8; 4];
    let mut prev_chips = [0u8; 4];
    let mut next_gens = [0u8; 4];
    let mut next_chips = [0u8; 4];

    for &(g, m) in &prev.pairs {
        prev_gens[g as usize] += 1;
        prev_chips[m as usize] += 1;
    }
    for &(g, m) in &next.pairs {
        next_gens[g as usize] += 1;
        next_chips[m as usize] += 1;
    }

    // Items that left the source floor = prev count - next count on that floor
    // Items that arrived at dest floor = next count - prev count on that floor
    let gens_moved = prev_gens[from as usize].saturating_sub(next_gens[from as usize]);
    let chips_moved = prev_chips[from as usize].saturating_sub(next_chips[from as usize]);

    let mut parts: Vec<String> = Vec::new();
    if gens_moved > 0 {
        parts.push(format!("{}G", gens_moved));
    }
    if chips_moved > 0 {
        parts.push(format!("{}M", chips_moved));
    }

    format!(
        "  Elevator {dir}: F{} → F{}  [{}]\n",
        from + 1,
        to + 1,
        parts.join(", ")
    )
}

fn main() {
    let input = include_str!("../inputs/day11.txt");
    let (state, names) = parse_input(input);

    println!("Tracing optimal Part 1 path...");
    println!("Elements: {:?}", names);
    println!("Initial: {:?}", state.pairs);

    let path = bfs_with_path(state);
    println!("Optimal solution: {} steps", path.len() - 1);

    let mut output = String::new();
    output.push_str("Day 11: Radioisotope Thermoelectric Generators — Optimal Path Trace\n");
    output.push_str("===================================================================\n\n");
    output.push_str(&format!("Elements (canonical order): {:?}\n", names));
    output.push_str(&format!("Optimal solution: {} steps\n", path.len() - 1));
    output.push_str(&format!(
        "Legend: {}G/M = generator/microchip, E = elevator\n\n",
        names
            .iter()
            .map(|n| {
                let mut c = n.chars();
                let first = c.next().unwrap_or('?').to_ascii_uppercase();
                let second = c.next().unwrap_or('?').to_ascii_lowercase();
                format!("{first}{second}")
            })
            .collect::<Vec<_>>()
            .join(", ")
    ));
    output.push_str(&format!("{}\n", "=".repeat(67)));

    for (step, state) in path.iter().enumerate() {
        if step > 0 {
            output.push_str(&describe_move(&path[step - 1], state, &names));
            output.push('\n');
        }
        output.push_str(&format_state(state, &names, step));
        output.push('\n');
    }

    output.push_str(&format!("{}\n", "=".repeat(67)));
    output.push_str(&format!("Total: {} steps to move all items to F4\n", path.len() - 1));
    output.push_str("BFS guarantees this is the minimum possible.\n");

    let out_path = "Problem_Statements/days/day11_trace.txt";
    fs::write(out_path, &output).expect("Failed to write trace file");
    println!("Trace written to {out_path}");
}
