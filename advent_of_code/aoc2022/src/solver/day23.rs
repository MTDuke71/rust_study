//! Day 23: Unstable Diffusion
//!
//! Elf spreading simulation on an infinite grid.
//! Part 1: Count empty tiles in bounding box after 10 rounds.
//! Part 2: Find first round where no elf moves.

use rustc_hash::{FxHashMap, FxHashSet};

type Pos = (i32, i32);

/// Eight neighbors in order: N, NE, E, SE, S, SW, W, NW
const NEIGHBORS: [Pos; 8] = [
    (-1, 0),  // N
    (-1, 1),  // NE
    (0, 1),   // E
    (1, 1),   // SE
    (1, 0),   // S
    (1, -1),  // SW
    (0, -1),  // W
    (-1, -1), // NW
];

/// Direction checks: (check positions indices into NEIGHBORS, move delta)
/// N: check N, NE, NW → move N
/// S: check S, SE, SW → move S
/// W: check W, NW, SW → move W
/// E: check E, NE, SE → move E
const DIRECTIONS: [(Pos, [usize; 3]); 4] = [
    ((-1, 0), [0, 1, 7]), // N: check N, NE, NW
    ((1, 0), [4, 3, 5]),  // S: check S, SE, SW
    ((0, -1), [6, 7, 5]), // W: check W, NW, SW
    ((0, 1), [2, 1, 3]),  // E: check E, NE, SE
];

fn parse(input: &str) -> FxHashSet<Pos> {
    let mut elves = FxHashSet::default();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                elves.insert((row as i32, col as i32));
            }
        }
    }
    elves
}

/// Simulate one round. Returns true if any elf moved.
fn step(elves: &mut FxHashSet<Pos>, round: usize) -> bool {
    // Phase 1: proposals
    // Map from proposed destination → list of elves proposing it
    let mut proposals: FxHashMap<Pos, Vec<Pos>> = FxHashMap::default();

    for &elf in elves.iter() {
        // Check if any neighbor is occupied
        let has_neighbor = NEIGHBORS.iter().any(|&(dr, dc)| {
            elves.contains(&(elf.0 + dr, elf.1 + dc))
        });

        if !has_neighbor {
            continue; // Elf stays put
        }

        // Try each direction in rotated order
        let mut proposed = false;
        for i in 0..4 {
            let dir_idx = (round + i) % 4;
            let (delta, checks) = &DIRECTIONS[dir_idx];

            let clear = checks.iter().all(|&ci| {
                let n = NEIGHBORS[ci];
                !elves.contains(&(elf.0 + n.0, elf.1 + n.1))
            });

            if clear {
                let dest = (elf.0 + delta.0, elf.1 + delta.1);
                proposals.entry(dest).or_default().push(elf);
                proposed = true;
                break;
            }
        }

        if !proposed {
            // All four directions blocked, elf stays
        }
    }

    // Phase 2: movement - only move if sole proposer
    let mut moved = false;
    for (dest, proposers) in &proposals {
        if proposers.len() == 1 {
            elves.remove(&proposers[0]);
            elves.insert(*dest);
            moved = true;
        }
    }

    moved
}

fn bounding_box_empty(elves: &FxHashSet<Pos>) -> usize {
    let min_r = elves.iter().map(|p| p.0).min().unwrap();
    let max_r = elves.iter().map(|p| p.0).max().unwrap();
    let min_c = elves.iter().map(|p| p.1).min().unwrap();
    let max_c = elves.iter().map(|p| p.1).max().unwrap();

    let area = (max_r - min_r + 1) as usize * (max_c - min_c + 1) as usize;
    area - elves.len()
}

pub fn solve(input: &str) -> (usize, usize) {
    let input = input.replace("\r\n", "\n");
    let mut elves = parse(&input);

    // Part 1: 10 rounds
    let mut elves_p1 = elves.clone();
    for round in 0..10 {
        step(&mut elves_p1, round);
    }
    let part1 = bounding_box_empty(&elves_p1);

    // Part 2: find first round where no elf moves
    let mut round = 0;
    loop {
        let moved = step(&mut elves, round);
        round += 1;
        if !moved {
            break;
        }
    }
    let part2 = round;

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn test_example_part1() {
        let (p1, _) = solve(EXAMPLE);
        assert_eq!(p1, 110);
    }

    #[test]
    fn test_example_part2() {
        let (_, p2) = solve(EXAMPLE);
        assert_eq!(p2, 20);
    }

    #[test]
    fn test_actual_input() {
        let input = include_str!("../../inputs/day23.txt");
        let (p1, p2) = solve(input);
        assert_eq!(p1, 3780);
        assert_eq!(p2, 930);
    }
}
