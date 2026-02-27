//! Day 17: Pyroclastic Flow
//!
//! Tetris-like simulation: rocks fall in a 7-wide chamber, pushed by jets.
//! Part 1: Tower height after 2,022 rocks.
//! Part 2: Tower height after 1,000,000,000,000 rocks (cycle detection).

use std::collections::HashMap;

const WIDTH: usize = 7;

// Each rock shape as (x, y) offsets from bottom-left anchor point.
// y=0 is the bottom row of the shape, x=0 is leftmost.
const ROCKS: [&[(usize, usize)]; 5] = [
    // ####
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    // .#.
    // ###
    // .#.
    &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
    // ..#
    // ..#
    // ###
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    // #
    // #
    // #
    // #
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    // ##
    // ##
    &[(0, 0), (1, 0), (0, 1), (1, 1)],
];

// ============================================================================
// Data Structures
// ============================================================================

/// Parsed jet directions: -1 for left, +1 for right
pub type ParsedData = Vec<i8>;

struct Chamber {
    /// Each row is a bitmask of 7 columns (bit 0 = left wall side)
    rows: Vec<u8>,
    height: usize,
}

impl Chamber {
    fn new() -> Self {
        Self {
            rows: Vec::with_capacity(4096),
            height: 0,
        }
    }

    fn is_blocked(&self, x: i64, y: i64, rock: &[(usize, usize)]) -> bool {
        for &(dx, dy) in rock {
            let nx = x + dx as i64;
            let ny = y + dy as i64;
            if nx < 0 || nx >= WIDTH as i64 || ny < 0 {
                return true;
            }
            let ny = ny as usize;
            if ny < self.rows.len() && self.rows[ny] & (1 << nx) != 0 {
                return true;
            }
        }
        false
    }

    fn place(&mut self, x: i64, y: i64, rock: &[(usize, usize)]) {
        for &(dx, dy) in rock {
            let nx = x + dx as i64;
            let ny = (y + dy as i64) as usize;
            while self.rows.len() <= ny {
                self.rows.push(0);
            }
            self.rows[ny] |= 1 << nx;
            if ny + 1 > self.height {
                self.height = ny + 1;
            }
        }
    }

    /// Snapshot of top rows for cycle detection key.
    /// Uses the top 30 rows (or fewer if tower is shorter).
    fn top_profile(&self) -> Vec<u8> {
        let depth = 30.min(self.rows.len());
        self.rows[self.rows.len() - depth..].to_vec()
    }
}

// ============================================================================
// Parsing
// ============================================================================

pub fn parse_input(input: &str) -> ParsedData {
    input
        .trim()
        .bytes()
        .map(|b| match b {
            b'<' => -1,
            b'>' => 1,
            _ => panic!("unexpected jet char"),
        })
        .collect()
}

// ============================================================================
// Simulation
// ============================================================================

type CycleKey = (usize, usize, Vec<u8>);
type CycleVal = (u64, u64, usize);

fn simulate(jets: &[i8], num_rocks: u64) -> u64 {
    let mut chamber = Chamber::new();
    let mut jet_idx = 0usize;

    // Cycle detection state: (rock_idx % 5, jet_idx, top_profile) -> (rock_count, height, jet_idx)
    let mut seen: HashMap<CycleKey, CycleVal> = HashMap::new();
    let mut extra_height: u64 = 0;
    let mut rock_count: u64 = 0;

    while rock_count < num_rocks {
        let rock_type = (rock_count % 5) as usize;
        let rock = ROCKS[rock_type];

        // Spawn position: left edge at x=2, bottom at height+3
        let mut x: i64 = 2;
        let mut y: i64 = chamber.height as i64 + 3;

        loop {
            // Jet push
            let dx = jets[jet_idx % jets.len()] as i64;
            jet_idx += 1;
            if !chamber.is_blocked(x + dx, y, rock) {
                x += dx;
            }

            // Fall down
            if chamber.is_blocked(x, y - 1, rock) {
                chamber.place(x, y, rock);
                break;
            }
            y -= 1;
        }

        rock_count += 1;

        // Cycle detection (only check after some initial rocks settle)
        if extra_height == 0 && chamber.height > 30 {
            let key = (rock_type, jet_idx % jets.len(), chamber.top_profile());
            if let Some(&(prev_rock, prev_height, prev_jet_idx)) = seen.get(&key) {
                // Found a cycle!
                let cycle_len = rock_count - prev_rock;
                let cycle_height = chamber.height as u64 - prev_height;
                let cycle_jets = jet_idx - prev_jet_idx;
                let remaining = num_rocks - rock_count;
                let full_cycles = remaining / cycle_len;
                let remainder = remaining % cycle_len;
                let jet_pos = jet_idx % jets.len();
                eprintln!("  Cycle detected! rock {rock_count} matches rock {prev_rock} (jet pos {jet_pos}/{}, rock shape {rock_type})", jets.len());
                eprintln!(
                    "    cycle: {cycle_len} rocks, {cycle_jets} jets ({:.1}× full pattern), {cycle_height} height",
                    cycle_jets as f64 / jets.len() as f64
                );
                eprintln!(
                    "    skip {full_cycles} cycles ({} height) | {remainder} rocks remain",
                    full_cycles * cycle_height
                );
                extra_height = full_cycles * cycle_height;
                rock_count += full_cycles * cycle_len;
                seen.clear(); // Stop detecting — only report the first cycle
                              // Continue simulating the remainder
            } else {
                seen.insert(key, (rock_count, chamber.height as u64, jet_idx));
            }
        }
    }

    chamber.height as u64 + extra_height
}

// ============================================================================
// Part 1 Logic (Internal - accepts parsed data)
// ============================================================================

fn solve_part1_with_data(jets: &ParsedData) -> u64 {
    simulate(jets, 2022)
}

// ============================================================================
// Part 2 Logic (Internal - accepts parsed data)
// ============================================================================

fn solve_part2_with_data(jets: &ParsedData) -> u64 {
    simulate(jets, 1_000_000_000_000)
}

// ============================================================================
// Public API
// ============================================================================

pub fn solve(input: &str) -> (u64, u64) {
    let jets = parse_input(input);
    (solve_part1_with_data(&jets), solve_part2_with_data(&jets))
}

pub fn solve_part1(input: &str) -> u64 {
    let jets = parse_input(input);
    solve_part1_with_data(&jets)
}

pub fn solve_part2(input: &str) -> u64 {
    let jets = parse_input(input);
    solve_part2_with_data(&jets)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_parse() {
        let jets = parse_input(EXAMPLE);
        assert_eq!(jets.len(), 40);
        assert_eq!(jets[0], 1); // '>'
        assert_eq!(jets[3], -1); // '<'
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 3068);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 1_514_285_714_288);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day17.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, 3109);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day17.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, 1_541_449_275_365);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day17.txt");
        assert_eq!(solve(input), (3109, 1_541_449_275_365));
    }
}
