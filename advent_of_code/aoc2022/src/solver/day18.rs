//! Day 18: Boiling Boulders
//!
//! 3D surface area of a lava droplet made of 1x1x1 cubes.
//! Part 1: Total surface area (count exposed faces).
//! Part 2: Exterior surface area only (exclude trapped air pockets).

use std::collections::{HashSet, VecDeque};

// ============================================================================
// Data Types
// ============================================================================

pub type ParsedData = Vec<(i32, i32, i32)>;

const NEIGHBORS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

// ============================================================================
// Parsing
// ============================================================================

pub fn parse_input(input: &str) -> ParsedData {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();
            (x, y, z)
        })
        .collect()
}

// ============================================================================
// Part 1: Total surface area
// ============================================================================

/// Count faces of each cube not adjacent to another cube.
/// Each cube has 6 faces. For each neighbor that exists, subtract 1.
pub fn solve_part1_with_data(cubes: &ParsedData) -> u64 {
    let set: HashSet<(i32, i32, i32)> = cubes.iter().copied().collect();
    let mut surface = 0u64;

    for &(x, y, z) in cubes {
        for (dx, dy, dz) in NEIGHBORS {
            if !set.contains(&(x + dx, y + dy, z + dz)) {
                surface += 1;
            }
        }
    }
    surface
}

// ============================================================================
// Part 2: Exterior surface area (flood fill from outside)
// ============================================================================

/// BFS flood fill from outside the bounding box.
/// Only count faces where the flood reaches a cube — internal air pockets
/// are never reached by the flood, so their faces don't count.
pub fn solve_part2_with_data(cubes: &ParsedData) -> u64 {
    let set: HashSet<(i32, i32, i32)> = cubes.iter().copied().collect();

    // Bounding box with 1 unit of padding so flood can wrap around
    let min_x = cubes.iter().map(|c| c.0).min().unwrap() - 1;
    let max_x = cubes.iter().map(|c| c.0).max().unwrap() + 1;
    let min_y = cubes.iter().map(|c| c.1).min().unwrap() - 1;
    let max_y = cubes.iter().map(|c| c.1).max().unwrap() + 1;
    let min_z = cubes.iter().map(|c| c.2).min().unwrap() - 1;
    let max_z = cubes.iter().map(|c| c.2).max().unwrap() + 1;

    // BFS from corner of bounding box (guaranteed to be outside)
    let start = (min_x, min_y, min_z);
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(start);
    queue.push_back(start);

    let mut exterior_surface = 0u64;

    while let Some((x, y, z)) = queue.pop_front() {
        for (dx, dy, dz) in NEIGHBORS {
            let nx = x + dx;
            let ny = y + dy;
            let nz = z + dz;

            // Stay within bounding box
            if nx < min_x || nx > max_x || ny < min_y || ny > max_y || nz < min_z || nz > max_z {
                continue;
            }

            if set.contains(&(nx, ny, nz)) {
                // Hit a lava cube face — count it
                exterior_surface += 1;
            } else if visited.insert((nx, ny, nz)) {
                // Empty space not yet visited — continue flooding
                queue.push_back((nx, ny, nz));
            }
        }
    }

    exterior_surface
}

// ============================================================================
// Entry point
// ============================================================================

pub fn solve(input: &str) -> (u64, u64) {
    let cubes = parse_input(input);
    (solve_part1_with_data(&cubes), solve_part2_with_data(&cubes))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_parse() {
        let cubes = parse_input(EXAMPLE);
        assert_eq!(cubes.len(), 13);
        assert_eq!(cubes[0], (2, 2, 2));
    }

    #[test]
    fn test_part1_example() {
        let cubes = parse_input(EXAMPLE);
        assert_eq!(solve_part1_with_data(&cubes), 64);
    }

    #[test]
    fn test_part2_example() {
        let cubes = parse_input(EXAMPLE);
        assert_eq!(solve_part2_with_data(&cubes), 58);
    }

    #[test]
    fn test_simple_pair() {
        // Two adjacent cubes: 10 exposed faces (6+6 - 2 shared)
        let cubes = vec![(1, 1, 1), (2, 1, 1)];
        assert_eq!(solve_part1_with_data(&cubes), 10);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day18.txt");
        let cubes = parse_input(input);
        assert_eq!(solve_part1_with_data(&cubes), 3522);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day18.txt");
        let cubes = parse_input(input);
        assert_eq!(solve_part2_with_data(&cubes), 2074);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day18.txt");
        assert_eq!(solve(input), (3522, 2074));
    }
}
