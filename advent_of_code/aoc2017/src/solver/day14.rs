//! Day 14: Disk Defragmentation
//!
//! Build a 128x128 disk grid from 128 knot hashes (Day 10).
//! Part 1: count used squares. Part 2: count connected regions (Mission 10 UnionFind).

use crate::solver::day10::knot_hash_bytes;
use mission10::UnionFind;

/// 128 rows × 16 bytes (128 bits) = entire disk state.
type Grid = [[u8; 16]; 128];

fn parse_input(input: &str) -> Grid {
    let key = input.trim();
    let mut grid = [[0u8; 16]; 128];
    for (row, row_bytes) in grid.iter_mut().enumerate() {
        *row_bytes = knot_hash_bytes(&format!("{key}-{row}"));
    }
    grid
}

/// Returns true if square (r, c) is used (bit = 1).
fn is_used(grid: &Grid, r: usize, c: usize) -> bool {
    let byte = grid[r][c / 8];
    let bit = 7 - (c % 8);
    (byte >> bit) & 1 == 1
}

fn solve_part1_with_data(grid: &Grid) -> u32 {
    grid.iter()
        .flat_map(|row| row.iter())
        .map(|b| b.count_ones())
        .sum()
}

fn solve_part2_with_data(grid: &Grid) -> usize {
    let idx = |r: usize, c: usize| r * 128 + c;
    let mut uf = UnionFind::new(128 * 128);

    // Union each used cell with its right and down neighbors (if used).
    // This covers all 4 directions since left/up get unioned when their owner visits.
    for r in 0..128 {
        for c in 0..128 {
            if !is_used(grid, r, c) {
                continue;
            }
            if c + 1 < 128 && is_used(grid, r, c + 1) {
                uf.union(idx(r, c), idx(r, c + 1)).unwrap();
            }
            if r + 1 < 128 && is_used(grid, r + 1, c) {
                uf.union(idx(r, c), idx(r + 1, c)).unwrap();
            }
        }
    }

    // Count distinct roots among used cells.
    let mut roots = std::collections::HashSet::new();
    for r in 0..128 {
        for c in 0..128 {
            if is_used(grid, r, c) {
                roots.insert(uf.find(idx(r, c)).unwrap());
            }
        }
    }
    roots.len()
}

pub fn solve(input: &str) -> (u32, usize) {
    let grid = parse_input(input);
    (solve_part1_with_data(&grid), solve_part2_with_data(&grid))
}

pub fn solve_part1(input: &str) -> u32 {
    let grid = parse_input(input);
    solve_part1_with_data(&grid)
}

pub fn solve_part2(input: &str) -> usize {
    let grid = parse_input(input);
    solve_part2_with_data(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "flqrgnkx";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 8108);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 1242);
    }

    #[test]
    fn test_is_used_first_row() {
        // Example first 8 columns of flqrgnkx-0 should match "##.#.#.."
        let grid = parse_input(EXAMPLE);
        let expected = [true, true, false, true, false, true, false, false];
        for (c, &want) in expected.iter().enumerate() {
            assert_eq!(is_used(&grid, 0, c), want, "col {c}");
        }
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day14.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, 8106);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day14.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, 1164);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day14.txt");
        assert_eq!(solve(input), (8106, 1164));
    }
}
