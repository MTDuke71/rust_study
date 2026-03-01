//! Day 1: No Time for a Taxicab
//!
//! Follow a sequence of turn+walk instructions on a city grid.
//! Track Manhattan distance from origin.

#[derive(Debug, Clone, Copy)]
struct Instruction {
    turn: i32,  // +1 = right, -1 = left
    steps: i32,
}

// Direction vectors: North, East, South, West (clockwise)
const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().split(", ").map(|s| {
        let turn = if s.starts_with('R') { 1 } else { -1 };
        let steps = s[1..].parse().unwrap();
        Instruction { turn, steps }
    }).collect()
}

/// Walk all instructions and return final Manhattan distance from origin.
fn solve_part1_with_data(instructions: &[Instruction]) -> i32 {
    let mut dir = 0i32; // 0=N, 1=E, 2=S, 3=W
    let (mut x, mut y) = (0i32, 0i32);

    for inst in instructions {
        dir = (dir + inst.turn).rem_euclid(4);
        let (dx, dy) = DIRS[dir as usize];
        x += dx * inst.steps;
        y += dy * inst.steps;
    }

    x.abs() + y.abs()
}

/// Walk block-by-block and return Manhattan distance to first revisited location.
fn solve_part2_with_data(instructions: &[Instruction]) -> i32 {
    use std::collections::HashSet;

    let mut dir = 0i32;
    let (mut x, mut y) = (0i32, 0i32);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((x, y));

    'outer: for inst in instructions {
        dir = (dir + inst.turn).rem_euclid(4);
        let (dx, dy) = DIRS[dir as usize];
        for _ in 0..inst.steps {
            x += dx;
            y += dy;
            if !visited.insert((x, y)) {
                break 'outer;
            }
        }
    }

    x.abs() + y.abs()
}

pub fn solve(input: &str) -> (i32, i32) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}

pub fn solve_part1(input: &str) -> i32 {
    solve_part1_with_data(&parse_input(input))
}

pub fn solve_part2(input: &str) -> i32 {
    solve_part2_with_data(&parse_input(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(solve_part1("R2, L3"), 5);
        assert_eq!(solve_part1("R2, R2, R2"), 2);
        assert_eq!(solve_part1("R5, L5, R5, R3"), 12);
    }

    #[test]
    fn test_part2_example() {
        // R8 → (8,0), R4 → (8,-4), R4 → (4,-4), R8 → (4,4) crosses (4,0) again
        assert_eq!(solve_part2("R8, R4, R4, R8"), 4);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day01.txt");
        assert_eq!(solve_part1(input), 161);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day01.txt");
        assert_eq!(solve(input), (161, 110));
    }
}
