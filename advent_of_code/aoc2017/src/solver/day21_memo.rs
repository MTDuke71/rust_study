//! Day 21: Fractal Art — Memoized alternate implementation
//!
//! Algorithmic optimization using a structural property of the fractal:
//!
//! > Starting from any 3×3 block, three iterations produce a 9×9 grid
//! > whose nine 3×3 sub-blocks evolve *independently* under every
//! > subsequent iteration.
//!
//! This holds because after the three iterations the next step (iter 4)
//! does a /3 split, whose block boundaries align exactly with those
//! sub-blocks. Every 3-iteration "round" thereafter does the same.
//!
//! So we memoize:
//!
//! ```text
//! f(block, 0) = popcount(block)
//! f(block, d) = Σ f(sub, d-1)   for sub in expand3(block)
//! ```
//!
//! where `expand3` maps a 3×3 block to the nine 3×3 sub-blocks it becomes
//! after 3 iterations. Both caches are tiny: there are at most `2⁹ = 512`
//! distinct 3×3 blocks, and the recursion visits at most 512 × (depth+1)
//! distinct `(block, depth)` keys. For Part 2 (18 iter = 6 depth), that's
//! ≤ 3584 total cache entries.
//!
//! Part 1 (5 iter) isn't a multiple of 3, so we delegate to
//! [`super::day21_bitpacked::solve_part1`] — it's already ~20 µs there.
//!
//! Complexity comparison:
//! - Reference / bit-packed: O((3/2)ⁿ) pixels at iteration n → 4.78M cells at n=18
//! - This module: O(depth × |distinct blocks|) ≈ O(7 × 512) ≈ 3584 ops

use std::collections::HashMap;

use crate::solver::day21_bitpacked::{
    self, initial_grid, pack_block, parse_rules, step, Grid,
};

/// Run 3 iterations starting from a 3×3 block, return the 9 sub-blocks
/// (row-major) of the resulting 9×9 grid.
fn expand3(block: u16, rules_2: &[u16; 16], rules_3: &[u16; 512]) -> [u16; 9] {
    let mut cells = vec![false; 9];
    for (i, cell) in cells.iter_mut().enumerate() {
        *cell = ((block >> i) & 1) == 1;
    }
    let mut grid = Grid { cells, n: 3 };
    for _ in 0..3 {
        grid = step(&grid, rules_2, rules_3);
    }
    // grid is 9×9; decompose into nine 3×3 sub-blocks.
    let mut sub_blocks = [0u16; 9];
    for br in 0..3 {
        for bc in 0..3 {
            sub_blocks[br * 3 + bc] = pack_block(&grid, br * 3, bc * 3, 3);
        }
    }
    sub_blocks
}

/// Pixel count after `3 * depth` iterations, starting from a 3×3 block.
/// Memoizes both `expand3` results and the counts themselves.
fn count_after(
    block: u16,
    depth: usize,
    expand_cache: &mut [Option<[u16; 9]>; 512],
    count_cache: &mut HashMap<(u16, usize), u64>,
    rules_2: &[u16; 16],
    rules_3: &[u16; 512],
) -> u64 {
    if depth == 0 {
        return block.count_ones() as u64;
    }
    if let Some(&cached) = count_cache.get(&(block, depth)) {
        return cached;
    }
    let sub_blocks = match expand_cache[block as usize] {
        Some(s) => s,
        None => {
            let s = expand3(block, rules_2, rules_3);
            expand_cache[block as usize] = Some(s);
            s
        }
    };
    let total: u64 = sub_blocks
        .iter()
        .map(|&sub| count_after(sub, depth - 1, expand_cache, count_cache, rules_2, rules_3))
        .sum();
    count_cache.insert((block, depth), total);
    total
}

/// Bit-packed form of the puzzle's starting grid `.#./..#/###`.
fn initial_block() -> u16 {
    let grid = initial_grid();
    pack_block(&grid, 0, 0, 3)
}

pub fn solve_part1(input: &str) -> usize {
    // 5 iter isn't 3k; the memo trick doesn't apply directly.
    day21_bitpacked::solve_part1(input)
}

pub fn solve_part2(input: &str) -> usize {
    let (rules_2, rules_3) = parse_rules(input);
    let mut expand_cache: [Option<[u16; 9]>; 512] = [None; 512];
    let mut count_cache: HashMap<(u16, usize), u64> = HashMap::new();
    count_after(
        initial_block(),
        6, // 18 / 3
        &mut expand_cache,
        &mut count_cache,
        &rules_2,
        &rules_3,
    ) as usize
}

pub fn solve(input: &str) -> (usize, usize) {
    (solve_part1(input), solve_part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_block_packing() {
        // .#./..#/### → bits at (0,1), (1,2), (2,0), (2,1), (2,2)
        //             = row-major positions 1, 5, 6, 7, 8
        //             = 0b1_1110_0010 = 482
        assert_eq!(initial_block(), 0b1_1110_0010);
    }

    #[test]
    fn test_expand3_agrees_with_mechanical_iteration() {
        // expand3(block) must equal: run step() three times, pack 9 sub-blocks.
        let input = include_str!("../../inputs/day21.txt");
        let (rules_2, rules_3) = parse_rules(input);
        let initial = initial_block();

        let expected = {
            let mut g = initial_grid();
            for _ in 0..3 {
                g = step(&g, &rules_2, &rules_3);
            }
            let mut subs = [0u16; 9];
            for br in 0..3 {
                for bc in 0..3 {
                    subs[br * 3 + bc] = pack_block(&g, br * 3, bc * 3, 3);
                }
            }
            subs
        };
        let actual = expand3(initial, &rules_2, &rules_3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2_matches_reference() {
        let input = include_str!("../../inputs/day21.txt");
        assert_eq!(solve_part2(input), 2_425_195);
    }

    #[test]
    fn test_part1_matches_reference() {
        // Part 1 delegates to the bit-packed solver; just verify the delegation works.
        let input = include_str!("../../inputs/day21.txt");
        assert_eq!(solve_part1(input), 167);
    }

    #[test]
    fn test_both_parts() {
        let input = include_str!("../../inputs/day21.txt");
        assert_eq!(solve(input), (167, 2_425_195));
    }
}
