//! Day 23: Unstable Diffusion
//!
//! Elf spreading simulation on an infinite grid.
//! Part 1: Count empty tiles in bounding box after 10 rounds.
//! Part 2: Find first round where no elf moves.

use rustc_hash::{FxHashMap, FxHashSet};

/// Packed (row, col) into a single i64 for faster hashing.
/// High 32 bits = row, low 32 bits = col.
type Pos = i64;

#[inline(always)]
fn pack(r: i32, c: i32) -> Pos {
    ((r as i64) << 32) | (c as u32 as i64)
}

#[inline(always)]
fn unpack(p: Pos) -> (i32, i32) {
    ((p >> 32) as i32, p as i32)
}

/// Eight neighbor offsets as (dr, dc): N, NE, E, SE, S, SW, W, NW
const NEIGHBORS: [(i32, i32); 8] = [
    (-1, 0),  // N
    (-1, 1),  // NE
    (0, 1),   // E
    (1, 1),   // SE
    (1, 0),   // S
    (1, -1),  // SW
    (0, -1),  // W
    (-1, -1), // NW
];

/// Direction move deltas as (dr, dc): N, S, W, E
const DIR_DELTAS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

/// Bitmasks for 3-neighbor direction checks (indices match NEIGHBOR_DELTAS)
const DIR_MASKS: [u8; 4] = [
    (1 << 0) | (1 << 1) | (1 << 7), // N: N, NE, NW
    (1 << 4) | (1 << 3) | (1 << 5), // S: S, SE, SW
    (1 << 6) | (1 << 7) | (1 << 5), // W: W, NW, SW
    (1 << 2) | (1 << 1) | (1 << 3), // E: E, NE, SE
];

fn parse(input: &str) -> FxHashSet<Pos> {
    let mut elves = FxHashSet::default();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                elves.insert(pack(row as i32, col as i32));
            }
        }
    }
    elves
}

/// Simulate one round. Returns true if any elf moved.
fn step(
    elves: &mut FxHashSet<Pos>,
    round: usize,
    proposals: &mut FxHashMap<Pos, (Pos, u8)>,
    elf_vec: &mut Vec<Pos>,
) -> bool {
    proposals.clear();

    // Collect to Vec for cache-friendly iteration
    elf_vec.clear();
    elf_vec.extend(elves.iter());

    for &elf in elf_vec.iter() {
        let (r, c) = unpack(elf);

        // Build 8-neighbor occupancy bitmask
        let mut occupied: u8 = 0;
        for (i, &(dr, dc)) in NEIGHBORS.iter().enumerate() {
            if elves.contains(&pack(r + dr, c + dc)) {
                occupied |= 1 << i;
            }
        }

        if occupied == 0 {
            continue;
        }

        // Try each direction — bitmask test instead of 3 hash lookups
        for i in 0..4 {
            let dir_idx = (round + i) % 4;
            if occupied & DIR_MASKS[dir_idx] == 0 {
                let (dr, dc) = DIR_DELTAS[dir_idx];
                let dest = pack(r + dr, c + dc);
                proposals
                    .entry(dest)
                    .and_modify(|e| e.1 += 1)
                    .or_insert((elf, 1));
                break;
            }
        }
    }

    // Phase 2: move sole proposers
    let mut moved = false;
    for (&dest, &(src, count)) in proposals.iter() {
        if count == 1 {
            elves.remove(&src);
            elves.insert(dest);
            moved = true;
        }
    }

    moved
}

fn bounding_box_empty(elves: &FxHashSet<Pos>) -> usize {
    let (mut min_r, mut max_r) = (i32::MAX, i32::MIN);
    let (mut min_c, mut max_c) = (i32::MAX, i32::MIN);
    for &p in elves.iter() {
        let (r, c) = unpack(p);
        min_r = min_r.min(r);
        max_r = max_r.max(r);
        min_c = min_c.min(c);
        max_c = max_c.max(c);
    }
    let area = (max_r - min_r + 1) as usize * (max_c - min_c + 1) as usize;
    area - elves.len()
}

pub fn solve(input: &str) -> (usize, usize) {
    let input = input.replace("\r\n", "\n");
    let mut elves = parse(&input);
    let mut proposals = FxHashMap::default();
    let mut elf_vec = Vec::with_capacity(elves.len());

    // Single run: checkpoint at round 10 for Part 1, continue for Part 2
    let mut part1 = 0;
    let mut round = 0;
    loop {
        if round == 10 {
            part1 = bounding_box_empty(&elves);
        }
        let moved = step(&mut elves, round, &mut proposals, &mut elf_vec);
        round += 1;
        if !moved {
            break;
        }
    }
    if round <= 10 {
        part1 = bounding_box_empty(&elves);
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
