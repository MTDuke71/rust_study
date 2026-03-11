//! Day 11: Radioisotope Thermoelectric Generators
//!
//! BFS state-space search with canonical state representation.
//! Key optimization: element pairs are interchangeable, so sort them
//! to collapse equivalent states.

use std::collections::{HashSet, VecDeque};

/// Each element has a generator floor and a microchip floor (0-indexed).
/// State = elevator floor + sorted list of (gen_floor, chip_floor) pairs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    elevator: u8,
    /// Sorted pairs of (generator_floor, microchip_floor) for each element.
    pairs: Vec<(u8, u8)>,
}

impl State {
    fn canonicalize(&mut self) {
        self.pairs.sort();
    }

    fn is_goal(&self) -> bool {
        self.pairs.iter().all(|&(g, m)| g == 3 && m == 3)
    }

    /// Check if the current state is valid (no fried chips).
    /// A chip is fried if it's on a floor with a generator that isn't its own,
    /// AND its own generator is on a different floor.
    fn is_valid(&self) -> bool {
        for &(gen_floor, chip_floor) in &self.pairs {
            if gen_floor != chip_floor {
                // Chip is unshielded — check if any other generator is on chip's floor
                if self.pairs.iter().any(|&(g, _)| g == chip_floor) {
                    return false;
                }
            }
        }
        true
    }
}

/// Parse input text to extract element pairs with their floor assignments.
fn parse_input(input: &str) -> State {
    let input = input.replace("\r\n", "\n");
    let mut elements: Vec<String> = Vec::new();
    let mut gen_floors: Vec<u8> = Vec::new();
    let mut chip_floors: Vec<u8> = Vec::new();

    for (floor_idx, line) in input.trim().lines().enumerate() {
        let floor = floor_idx as u8;
        let words: Vec<&str> = line.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            if *word == "generator" || word.starts_with("generator") {
                // Previous word is the element name
                let element = words[i - 1]
                    .trim_start_matches("a ")
                    .to_string();
                if let Some(pos) = elements.iter().position(|e| e == &element) {
                    gen_floors[pos] = floor;
                } else {
                    elements.push(element.clone());
                    gen_floors.push(floor);
                    chip_floors.push(255); // placeholder
                }
            }
            if word.contains("compatible") {
                // Word is like "XXX-compatible" — extract element name
                let element = word.split('-').next().unwrap().to_string();
                if let Some(pos) = elements.iter().position(|e| e == &element) {
                    chip_floors[pos] = floor;
                } else {
                    elements.push(element.clone());
                    gen_floors.push(255); // placeholder
                    chip_floors.push(floor);
                }
            }
        }
    }

    let pairs: Vec<(u8, u8)> = gen_floors
        .into_iter()
        .zip(chip_floors)
        .collect();

    let mut state = State {
        elevator: 0,
        pairs,
    };
    state.canonicalize();
    state
}

/// BFS to find minimum steps to move everything to floor 4.
fn bfs(initial: State) -> usize {
    let mut visited: HashSet<State> = HashSet::new();
    let mut queue: VecDeque<(State, usize)> = VecDeque::new();

    visited.insert(initial.clone());
    queue.push_back((initial, 0));

    while let Some((state, steps)) = queue.pop_front() {
        if state.is_goal() {
            return steps;
        }

        // Collect indices of items on the current elevator floor
        let floor = state.elevator;
        let mut items_on_floor: Vec<(usize, bool)> = Vec::new(); // (element_idx, is_generator)

        for (i, &(g, m)) in state.pairs.iter().enumerate() {
            if g == floor {
                items_on_floor.push((i, true)); // generator
            }
            if m == floor {
                items_on_floor.push((i, false)); // microchip
            }
        }

        // Try moving 1 or 2 items to adjacent floors
        let directions: Vec<u8> = if floor == 0 {
            vec![1]
        } else if floor == 3 {
            vec![2]
        } else {
            vec![1, 2]
        };

        let new_floors: Vec<u8> = directions
            .iter()
            .map(|&d| if d == 1 { floor + 1 } else { floor - 1 })
            .collect();

        for &new_floor in &new_floors {
            // Try moving 2 items (prefer moving 2 up, 1 down for efficiency)
            for i in 0..items_on_floor.len() {
                for j in (i + 1)..items_on_floor.len() {
                    let mut next = state.clone();
                    next.elevator = new_floor;
                    apply_move(&mut next, items_on_floor[i], floor, new_floor);
                    apply_move(&mut next, items_on_floor[j], floor, new_floor);
                    next.canonicalize();

                    if next.is_valid() && visited.insert(next.clone()) {
                        queue.push_back((next, steps + 1));
                    }
                }
            }

            // Try moving 1 item
            for &item in &items_on_floor {
                let mut next = state.clone();
                next.elevator = new_floor;
                apply_move(&mut next, item, floor, new_floor);
                next.canonicalize();

                if next.is_valid() && visited.insert(next.clone()) {
                    queue.push_back((next, steps + 1));
                }
            }
        }
    }

    unreachable!("No solution found")
}

fn apply_move(state: &mut State, item: (usize, bool), _from: u8, to: u8) {
    let (elem_idx, is_gen) = item;
    if is_gen {
        state.pairs[elem_idx].0 = to;
    } else {
        state.pairs[elem_idx].1 = to;
    }
}

fn solve_part1_with_data(data: &State) -> usize {
    bfs(data.clone())
}

fn solve_part2_with_data(data: &State) -> usize {
    // Part 2: Add elerium and dilithium pairs to floor 1 (index 0)
    let mut extended = data.clone();
    extended.pairs.push((0, 0)); // elerium generator + microchip on floor 1
    extended.pairs.push((0, 0)); // dilithium generator + microchip on floor 1
    extended.canonicalize();
    bfs(extended)
}

pub fn solve(input: &str) -> (String, String) {
    let data = parse_input(input);
    (
        solve_part1_with_data(&data).to_string(),
        solve_part2_with_data(&data).to_string(),
    )
}

pub fn solve_part1(input: &str) -> String {
    let data = parse_input(input);
    solve_part1_with_data(&data).to_string()
}

pub fn solve_part2(input: &str) -> String {
    let data = parse_input(input);
    solve_part2_with_data(&data).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    #[test]
    fn test_parse_example() {
        let state = parse_input(EXAMPLE);
        // hydrogen: gen=1, chip=0; lithium: gen=2, chip=0
        // sorted: (1,0), (2,0)
        assert_eq!(state.elevator, 0);
        assert_eq!(state.pairs.len(), 2);
        assert_eq!(state.pairs, vec![(1, 0), (2, 0)]);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), "11");
    }

    #[test]
    fn test_parse_actual() {
        let input = include_str!("../../inputs/day11.txt");
        let state = parse_input(input);
        assert_eq!(state.pairs.len(), 5); // 5 elements
        assert_eq!(state.elevator, 0);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day11.txt");
        assert_eq!(solve_part1(input), "47");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day11.txt");
        assert_eq!(solve_part2(input), "71");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day11.txt");
        assert_eq!(solve(input), ("47".to_string(), "71".to_string()));
    }
}
