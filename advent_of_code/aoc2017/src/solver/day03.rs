//! Day 3: Spiral Memory
//!
//! Part 1: Given a position in a spiral grid, compute the Manhattan distance back to square 1.
//! Part 2: Walk the spiral writing neighbor-sums; find the first value exceeding the input.

use std::collections::HashMap;

fn parse_input(input: &str) -> u64 {
    input.trim().parse().unwrap()
}

/// Find Manhattan distance from square `n` to square 1 in the spiral.
///
/// Ring k contains squares (2k-1)²+1 through (2k+1)².
/// Each ring has 4 sides of length 2k.
/// Manhattan distance = k + |offset from nearest side midpoint|.
fn solve_part1_with_data(n: u64) -> u64 {
    if n == 1 {
        return 0;
    }

    // Find which ring: smallest odd square >= n gives the side length
    let root = (n as f64).sqrt().ceil() as u64;
    let side = if root.is_multiple_of(2) { root + 1 } else { root };
    let k = (side - 1) / 2;

    // Position within the ring (0-indexed)
    let ring_start = (2 * k - 1) * (2 * k - 1) + 1;
    let pos_in_ring = n - ring_start;

    // Side length is 2k, midpoints are at k-1, 3k-1, 5k-1, 7k-1
    let side_len = 2 * k;
    let pos_on_side = pos_in_ring % side_len;
    let offset = (pos_on_side as i64 - (k as i64 - 1)).unsigned_abs();

    k + offset
}

/// Walk the spiral, writing the sum of all filled neighbors at each step.
/// Return the first value written that exceeds `target`.
fn solve_part2_with_data(target: u64) -> u64 {
    let mut grid: HashMap<(i64, i64), u64> = HashMap::new();
    grid.insert((0, 0), 1);

    // Spiral directions: right, up, left, down
    let dirs: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let neighbors: [(i64, i64); 8] = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    let (mut x, mut y) = (0i64, 0i64);
    let mut dir = 0;
    let mut steps_in_dir = 1;
    let mut steps_taken = 0;
    let mut turns = 0;

    loop {
        x += dirs[dir].0;
        y += dirs[dir].1;
        steps_taken += 1;

        let val: u64 = neighbors.iter()
            .map(|(dx, dy)| grid.get(&(x + dx, y + dy)).unwrap_or(&0))
            .sum();

        if val > target {
            return val;
        }
        grid.insert((x, y), val);

        // Spiral: after walking steps_in_dir, turn left.
        // Increase step count every 2 turns.
        if steps_taken == steps_in_dir {
            steps_taken = 0;
            dir = (dir + 1) % 4;
            turns += 1;
            if turns % 2 == 0 {
                steps_in_dir += 1;
            }
        }
    }
}

pub fn solve(input: &str) -> (u64, u64) {
    let n = parse_input(input);
    (solve_part1_with_data(n), solve_part2_with_data(n))
}

pub fn solve_part1(input: &str) -> u64 {
    solve_part1_with_data(parse_input(input))
}

pub fn solve_part2(input: &str) -> u64 {
    solve_part2_with_data(parse_input(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(solve_part1_with_data(1), 0);
        assert_eq!(solve_part1_with_data(12), 3);
        assert_eq!(solve_part1_with_data(23), 2);
        assert_eq!(solve_part1_with_data(1024), 31);
    }

    #[test]
    fn test_part1_ring_boundaries() {
        // Ring 1 corners (odd perfect squares): 3, 5, 7, 9
        assert_eq!(solve_part1_with_data(9), 2);   // corner of ring 1 (max distance = 2k)
        assert_eq!(solve_part1_with_data(3), 2);    // another corner

        // Ring 1 midpoints (min distance = k)
        assert_eq!(solve_part1_with_data(2), 1);    // midpoint right side
        assert_eq!(solve_part1_with_data(4), 1);    // midpoint top side
        assert_eq!(solve_part1_with_data(6), 1);    // midpoint left side
        assert_eq!(solve_part1_with_data(8), 1);    // midpoint bottom side

        // Ring 2 start/end boundaries
        assert_eq!(solve_part1_with_data(10), 3);   // first square of ring 2
        assert_eq!(solve_part1_with_data(25), 4);   // last square of ring 2 (corner)

        // Ring 2 midpoints (distance = k = 2)
        assert_eq!(solve_part1_with_data(11), 2);
        assert_eq!(solve_part1_with_data(15), 2);
        assert_eq!(solve_part1_with_data(19), 2);
        assert_eq!(solve_part1_with_data(23), 2);

        // Ring 3 start/end
        assert_eq!(solve_part1_with_data(26), 5);   // first square of ring 3
        assert_eq!(solve_part1_with_data(49), 6);   // last square of ring 3 (corner)
    }

    #[test]
    fn test_part2_sequence() {
        // First few spiral sums: 1, 1, 2, 4, 5, 10, 11, 23, 25, 26, ...
        assert_eq!(solve_part2_with_data(1), 2);
        assert_eq!(solve_part2_with_data(3), 4);
        assert_eq!(solve_part2_with_data(5), 10);
        assert_eq!(solve_part2_with_data(25), 26);
        assert_eq!(solve_part2_with_data(747), 806);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day03.txt");
        assert_eq!(solve_part1(input), 438);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day03.txt");
        assert_eq!(solve_part2(input), 266330);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day03.txt");
        assert_eq!(solve(input), (438, 266330));
    }
}
