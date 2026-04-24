//! Day 22: Sporifica Virus
//!
//! A virus carrier walks an infinite 2D grid of nodes, each clean or infected.
//! We track infected nodes as a sparse set of (x, y) coordinates centred on
//! the input grid's middle cell, which sits at the origin. The carrier
//! starts at the origin facing up; each burst it turns based on the current
//! node's state, flips that state, then moves one step.

use std::collections::HashSet;

/// Parsed input: the set of initially-infected cells, using the centre of the
/// input grid as the origin. Positive y goes *up* so that "facing up" means
/// incrementing y, matching the puzzle's intuition.
#[derive(Debug, Clone)]
struct ParsedData {
    infected: HashSet<(i32, i32)>,
}

fn parse_input(input: &str) -> ParsedData {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    let h = lines.len() as i32;
    let w = lines[0].len() as i32;
    // Centre of the grid maps to (0, 0). For a 25×25 input the centre cell is
    // at row 12, col 12 → offset = 12 in both axes.
    let cx = w / 2;
    let cy = h / 2;

    let mut infected = HashSet::new();
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                let x = c as i32 - cx;
                // Flip row so that row 0 (top of map) is the largest y.
                let y = cy - r as i32;
                infected.insert((x, y));
            }
        }
    }
    ParsedData { infected }
}

/// Turn right: (dx, dy) → (dy, -dx).
fn turn_right(d: (i32, i32)) -> (i32, i32) {
    (d.1, -d.0)
}

/// Turn left: (dx, dy) → (-dy, dx).
fn turn_left(d: (i32, i32)) -> (i32, i32) {
    (-d.1, d.0)
}

fn solve_part1_with_data(data: &ParsedData) -> usize {
    let mut infected = data.infected.clone();
    let mut pos: (i32, i32) = (0, 0);
    let mut dir: (i32, i32) = (0, 1); // facing up (y-up)
    let mut caused = 0usize;

    for _ in 0..10_000 {
        if infected.contains(&pos) {
            dir = turn_right(dir);
            infected.remove(&pos);
        } else {
            dir = turn_left(dir);
            infected.insert(pos);
            caused += 1;
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
    caused
}

// Part 2 state machine encoded as u8:
//   0 = Clean, 1 = Weakened, 2 = Infected, 3 = Flagged.
// The transition is (state + 1) mod 4, so we can advance with a single add.
const CLEAN: u8 = 0;
const WEAKENED: u8 = 1;
const INFECTED: u8 = 2;
const FLAGGED: u8 = 3;

/// Side of the bounded grid for Part 2. The carrier stays near the origin
/// across 10M bursts (well under ±200 empirically); 1024 gives generous
/// headroom at 1 MiB total.
const BOUND: usize = 1024;
const HALF: i32 = (BOUND / 2) as i32;

fn solve_part2_with_data(data: &ParsedData) -> usize {
    // Flat row-major grid. Index by (y + HALF) * BOUND + (x + HALF).
    let mut grid: Vec<u8> = vec![CLEAN; BOUND * BOUND];
    for &(x, y) in &data.infected {
        let idx = ((y + HALF) as usize) * BOUND + (x + HALF) as usize;
        grid[idx] = INFECTED;
    }

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir: (i32, i32) = (0, 1); // up (y-up)
    let mut caused = 0usize;

    for _ in 0..10_000_000 {
        let idx = ((y + HALF) as usize) * BOUND + (x + HALF) as usize;
        let state = grid[idx];
        match state {
            CLEAN => dir = turn_left(dir),
            WEAKENED => {}                         // no turn; will become infected
            INFECTED => dir = turn_right(dir),
            FLAGGED => dir = (-dir.0, -dir.1),     // reverse
            _ => unreachable!("state is a u8 masked to 0..=3"),
        }
        // Advance state: Clean→Weakened→Infected→Flagged→Clean.
        let next = (state + 1) & 0b11;
        grid[idx] = next;
        if next == INFECTED {
            caused += 1;
        }
        x += dir.0;
        y += dir.1;
    }
    caused
}

pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}

pub fn solve_part1(input: &str) -> usize {
    let data = parse_input(input);
    solve_part1_with_data(&data)
}

pub fn solve_part2(input: &str) -> usize {
    let data = parse_input(input);
    solve_part2_with_data(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
..#
#..
...
";

    #[test]
    fn test_part1_example_70_bursts() {
        // Puzzle: after 70 bursts, 41 infections caused.
        let data = parse_input(EXAMPLE);
        let mut infected = data.infected.clone();
        let mut pos: (i32, i32) = (0, 0);
        let mut dir: (i32, i32) = (0, 1);
        let mut caused = 0usize;
        for _ in 0..70 {
            if infected.contains(&pos) {
                dir = turn_right(dir);
                infected.remove(&pos);
            } else {
                dir = turn_left(dir);
                infected.insert(pos);
                caused += 1;
            }
            pos = (pos.0 + dir.0, pos.1 + dir.1);
        }
        assert_eq!(caused, 41);
    }

    #[test]
    fn test_part1_example_10k_bursts() {
        assert_eq!(solve_part1(EXAMPLE), 5587);
    }

    #[test]
    fn test_part2_example_100_bursts_26() {
        // Puzzle: after 100 bursts of the evolved virus, 26 infections caused.
        let data = parse_input(EXAMPLE);
        let mut grid: Vec<u8> = vec![CLEAN; BOUND * BOUND];
        for &(x, y) in &data.infected {
            let idx = ((y + HALF) as usize) * BOUND + (x + HALF) as usize;
            grid[idx] = INFECTED;
        }
        let (mut x, mut y) = (0i32, 0i32);
        let mut dir = (0i32, 1i32);
        let mut caused = 0usize;
        for _ in 0..100 {
            let idx = ((y + HALF) as usize) * BOUND + (x + HALF) as usize;
            let state = grid[idx];
            match state {
                CLEAN => dir = turn_left(dir),
                WEAKENED => {}
                INFECTED => dir = turn_right(dir),
                _ => dir = (-dir.0, -dir.1),
            }
            let next = (state + 1) & 0b11;
            grid[idx] = next;
            if next == INFECTED {
                caused += 1;
            }
            x += dir.0;
            y += dir.1;
        }
        assert_eq!(caused, 26);
    }

    #[test]
    fn test_part2_example_10m_bursts() {
        // Puzzle: after 10,000,000 bursts of the evolved virus, 2,511,944
        // infections. This is the Part 2 target answer for the example.
        assert_eq!(solve_part2(EXAMPLE), 2_511_944);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day22.txt");
        assert_eq!(solve(input), (5240, 2_512_144));
    }

    #[test]
    fn test_parse_example_centre_and_infected() {
        let data = parse_input(EXAMPLE);
        // 3×3 grid, centre is (1, 1) → origin. The two '#' cells are at
        // (row=0, col=2) and (row=1, col=0) in map coords.
        //   (0,2) → x= 2-1= 1, y= 1-0= 1  → ( 1,  1)
        //   (1,0) → x= 0-1=-1, y= 1-1= 0  → (-1,  0)
        assert!(data.infected.contains(&(1, 1)));
        assert!(data.infected.contains(&(-1, 0)));
        assert_eq!(data.infected.len(), 2);
        // Origin itself is clean in the example.
        assert!(!data.infected.contains(&(0, 0)));
    }
}
