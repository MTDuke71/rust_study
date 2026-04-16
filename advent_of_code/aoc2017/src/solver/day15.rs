//! Day 15: Dueling Generators
//!
//! Two generators produce values via `next = (prev * factor) % 2147483647`.
//! Part 1: count matches in lowest 16 bits across 40M pairs.
//! Part 2: same, but A filters to multiples of 4, B to multiples of 8, over 5M pairs.

use rayon::prelude::*;

const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;
const MODULUS: u64 = 2147483647;
const MASK_16: u64 = 0xFFFF;
const PART1_PAIRS: u64 = 40_000_000;
const PART2_PAIRS: usize = 5_000_000;
const BLOCK_SIZE: u64 = 1_000_000;

#[derive(Debug, Clone, Copy)]
struct Seeds {
    a: u64,
    b: u64,
}

fn parse_input(input: &str) -> Seeds {
    let mut lines = input.lines();
    let a = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let b = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    Seeds { a, b }
}

/// Fast modulo for Mersenne prime 2³¹−1: split at bit 31, add halves, conditional subtract.
#[inline]
fn mersenne_mod(n: u64) -> u64 {
    let sum = (n & MODULUS) + (n >> 31);
    if sum >= MODULUS { sum - MODULUS } else { sum }
}

#[inline]
fn next_value(prev: u64, factor: u64) -> u64 {
    mersenne_mod(prev * factor)
}

/// Compute `base^exp mod (2^31 - 1)` via exponentiation by squaring.
fn mod_pow(mut base: u64, mut exp: u64) -> u64 {
    let mut result = 1u64;
    base %= MODULUS;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mersenne_mod(result * base);
        }
        exp >>= 1;
        base = mersenne_mod(base * base);
    }
    result
}

/// Jump a generator ahead by `steps`: `seed * factor^steps mod m`.
#[inline]
fn jump_ahead(seed: u64, factor: u64, steps: u64) -> u64 {
    mersenne_mod(seed * mod_pow(factor, steps))
}

/// Count matches in a block of `count` sequential pairs starting from given seeds.
fn count_block(mut a: u64, mut b: u64, count: u64) -> u32 {
    let mut matches = 0u32;
    for _ in 0..count {
        a = next_value(a, FACTOR_A);
        b = next_value(b, FACTOR_B);
        if (a ^ b) & MASK_16 == 0 {
            matches += 1;
        }
    }
    matches
}

fn solve_part1_with_data(seeds: &Seeds) -> u32 {
    let num_blocks = PART1_PAIRS / BLOCK_SIZE;

    (0..num_blocks)
        .into_par_iter()
        .map(|block| {
            let offset = block * BLOCK_SIZE;
            let a = jump_ahead(seeds.a, FACTOR_A, offset);
            let b = jump_ahead(seeds.b, FACTOR_B, offset);
            count_block(a, b, BLOCK_SIZE)
        })
        .sum()
}

/// Generate filtered values from a raw block of `raw_count` LCG steps starting at `seed`.
fn generate_filtered_block(seed: u64, factor: u64, filter_mask: u64, raw_count: u64) -> Vec<u16> {
    let expected = (raw_count / (filter_mask + 1)) as usize;
    let mut values = Vec::with_capacity(expected + expected / 8);
    let mut v = seed;
    for _ in 0..raw_count {
        v = next_value(v, factor);
        if v & filter_mask == 0 {
            values.push(v as u16);
        }
    }
    values
}

/// Split raw generation into parallel blocks, filter each, concatenate, trim to exactly `count`.
fn generate_filtered_parallel(seed: u64, factor: u64, filter_mask: u64, count: usize) -> Vec<u16> {
    let raw_estimate = (count as u64) * (filter_mask + 1) * 105 / 100;
    let num_blocks = 16u64;
    let block_raw = raw_estimate / num_blocks;
    let total_raw = block_raw * num_blocks;

    let mut block_results: Vec<(u64, Vec<u16>)> = (0..num_blocks)
        .into_par_iter()
        .map(|i| {
            let offset = i * block_raw;
            let block_seed = jump_ahead(seed, factor, offset);
            (i, generate_filtered_block(block_seed, factor, filter_mask, block_raw))
        })
        .collect();

    block_results.sort_unstable_by_key(|(i, _)| *i);

    let mut all: Vec<u16> = Vec::with_capacity(count + count / 8);
    for (_, block) in block_results {
        all.extend_from_slice(&block);
    }

    if all.len() >= count {
        all.truncate(count);
        return all;
    }

    let mut v = jump_ahead(seed, factor, total_raw);
    while all.len() < count {
        v = next_value(v, factor);
        if v & filter_mask == 0 {
            all.push(v as u16);
        }
    }
    all
}

fn solve_part2_with_data(seeds: &Seeds) -> u32 {
    let (vals_a, vals_b) = rayon::join(
        || generate_filtered_parallel(seeds.a, FACTOR_A, 3, PART2_PAIRS),
        || generate_filtered_parallel(seeds.b, FACTOR_B, 7, PART2_PAIRS),
    );

    vals_a
        .iter()
        .zip(vals_b.iter())
        .filter(|(a, b)| a == b)
        .count() as u32
}

pub fn solve(input: &str) -> (u32, u32) {
    let seeds = parse_input(input);
    (solve_part1_with_data(&seeds), solve_part2_with_data(&seeds))
}

pub fn solve_part1(input: &str) -> u32 {
    let seeds = parse_input(input);
    solve_part1_with_data(&seeds)
}

pub fn solve_part2(input: &str) -> u32 {
    let seeds = parse_input(input);
    solve_part2_with_data(&seeds)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Generator A starts with 65\nGenerator B starts with 8921";

    #[test]
    fn test_parse() {
        let seeds = parse_input(EXAMPLE);
        assert_eq!(seeds.a, 65);
        assert_eq!(seeds.b, 8921);
    }

    #[test]
    fn test_first_values() {
        let mut a = 65u64;
        let mut b = 8921u64;
        a = next_value(a, FACTOR_A);
        b = next_value(b, FACTOR_B);
        assert_eq!(a, 1092455);
        assert_eq!(b, 430625591);
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(FACTOR_A, 1), FACTOR_A);
        let mut v = 65u64;
        for _ in 0..100 {
            v = next_value(v, FACTOR_A);
        }
        assert_eq!(jump_ahead(65, FACTOR_A, 100), v);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 588);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 309);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day15.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, 609);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day15.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, 253);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day15.txt");
        assert_eq!(solve(input), (609, 253));
    }
}
