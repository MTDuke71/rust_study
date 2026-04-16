//! Day 15: Dueling Generators — Naive Baseline (Step 0)
//!
//! The original straightforward solution before any optimization.
//! Kept as a reference to contrast with the optimized version in day15.rs.
//!
//! Performance: ~333ms (vs 22.7ms optimized — 14.7× slower)
//!
//! What makes this simple:
//! - No Rayon, no parallelism
//! - Standard `%` operator for modulo
//! - Sequential loops for both parts
//! - Part 2 uses inner spin loops to filter values in place
//!
//! What changed in the optimized version:
//! - Step 1: `% MODULUS` → `mersenne_mod()` (bit shift + add, no DIV instruction)
//! - Step 2: Sequential Part 1 → 40 parallel blocks via `mod_pow` jump-ahead
//! - Step 3: Sequential Part 2 → 16 parallel filtered blocks per generator

const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;
const MODULUS: u64 = 2147483647;
const MASK_16: u64 = 0xFFFF;

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

#[inline]
fn next_value(prev: u64, factor: u64) -> u64 {
    (prev * factor) % MODULUS
}

fn solve_part1_with_data(seeds: &Seeds) -> u32 {
    let mut a = seeds.a;
    let mut b = seeds.b;
    let mut matches = 0u32;
    for _ in 0..40_000_000 {
        a = next_value(a, FACTOR_A);
        b = next_value(b, FACTOR_B);
        if a & MASK_16 == b & MASK_16 {
            matches += 1;
        }
    }
    matches
}

fn solve_part2_with_data(seeds: &Seeds) -> u32 {
    let mut a = seeds.a;
    let mut b = seeds.b;
    let mut matches = 0u32;
    for _ in 0..5_000_000 {
        loop {
            a = next_value(a, FACTOR_A);
            if a & 3 == 0 {
                break;
            }
        }
        loop {
            b = next_value(b, FACTOR_B);
            if b & 7 == 0 {
                break;
            }
        }
        if a & MASK_16 == b & MASK_16 {
            matches += 1;
        }
    }
    matches
}

pub fn solve(input: &str) -> (u32, u32) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Generator A starts with 65\nGenerator B starts with 8921";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve(EXAMPLE).0, 588);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve(EXAMPLE).1, 309);
    }
}
