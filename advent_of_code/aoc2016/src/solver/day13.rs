//! Day 13: A Maze of Twisty Little Cubicles
//!
//! BFS through a procedurally-generated maze where walls are determined
//! by bit-counting a formula based on (x, y) coordinates.

use std::collections::{HashSet, VecDeque};

/// Returns true if (x, y) is an open space for the given designer number.
fn is_open(x: u64, y: u64, fav: u64) -> bool {
    let val = x * x + 3 * x + 2 * x * y + y + y * y + fav;
    val.count_ones() & 1 == 0
}

/// BFS result containing both answers: distance to target and count of
/// positions reachable within `max_steps`.
struct BfsResult {
    dist_to_target: u64,
    reachable_within_limit: usize,
}

fn bfs(fav: u64, target: (u64, u64), max_steps: u64) -> BfsResult {
    let mut visited: HashSet<(u64, u64)> = HashSet::new();
    let mut queue: VecDeque<(u64, u64, u64)> = VecDeque::new(); // (x, y, steps)

    queue.push_back((1, 1, 0));
    visited.insert((1, 1));

    let mut dist_to_target = 0;
    let mut reachable_within_limit = 0;

    while let Some((x, y, steps)) = queue.pop_front() {
        if steps <= max_steps {
            reachable_within_limit += 1;
        }

        if (x, y) == target {
            dist_to_target = steps;
        }

        // Don't explore beyond max_steps (both answers found by then)
        if steps >= max_steps && dist_to_target > 0 {
            continue;
        }

        // Try all 4 neighbors (no negatives since coords are u64)
        let neighbors = [
            (x + 1, y),
            (x.wrapping_sub(1), y),
            (x, y + 1),
            (x, y.wrapping_sub(1)),
        ];

        for (nx, ny) in neighbors {
            // u64 wrapping_sub on 0 gives u64::MAX, which fails is_open naturally,
            // but let's be explicit
            if nx < 1000 && ny < 1000 && is_open(nx, ny, fav) && visited.insert((nx, ny)) {
                queue.push_back((nx, ny, steps + 1));
            }
        }
    }

    BfsResult {
        dist_to_target,
        reachable_within_limit,
    }
}

fn parse_input(input: &str) -> u64 {
    input.trim().parse().expect("input should be a number")
}

fn solve_both(fav: u64) -> (u64, usize) {
    let result = bfs(fav, (31, 39), 50);
    (result.dist_to_target, result.reachable_within_limit)
}

pub fn solve(input: &str) -> (String, String) {
    let fav = parse_input(input);
    let (p1, p2) = solve_both(fav);
    (p1.to_string(), p2.to_string())
}

pub fn solve_part1(input: &str) -> u64 {
    let fav = parse_input(input);
    bfs(fav, (31, 39), 50).dist_to_target
}

pub fn solve_part2(input: &str) -> usize {
    let fav = parse_input(input);
    bfs(fav, (31, 39), 50).reachable_within_limit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_open() {
        // With favorite number 10, the example grid:
        // (0,0) open, (1,0) wall, (2,0) open
        assert!(is_open(0, 0, 10));
        assert!(!is_open(1, 0, 10));
        assert!(is_open(2, 0, 10));
    }

    #[test]
    fn test_example_part1() {
        // With fav=10, shortest path to (7,4) is 11 steps
        let result = bfs(10, (7, 4), 50);
        assert_eq!(result.dist_to_target, 11);
    }

    const ACTUAL: &str = include_str!("../../inputs/day13.txt");

    #[test]
    fn test_part1_actual() {
        assert_eq!(solve_part1(ACTUAL), 92);
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(solve_part2(ACTUAL), 124);
    }

    #[test]
    fn test_both_parts_actual() {
        let (p1, p2) = solve(ACTUAL);
        assert_eq!(p1, "92");
        assert_eq!(p2, "124");
    }
}
