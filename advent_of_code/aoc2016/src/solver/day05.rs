//! Day 5: How About a Nice Game of Chess?
//!
//! Find passwords by mining MD5 hashes with leading zeroes.
//! Part 1: 6th hex char of qualifying hashes (sequential).
//! Part 2: 6th char = position, 7th char = password character (positional).
//!
//! Optimized: single-pass mining + Rayon parallel batches.

use md5::{Md5, Digest};
use rayon::prelude::*;

const BATCH_SIZE: u64 = 500_000;

fn find_hits_in_batch(door_id: &[u8], start: u64) -> Vec<(u64, [u8; 16])> {
    (start..start + BATCH_SIZE)
        .into_par_iter()
        .filter_map(|i| {
            let mut hasher = Md5::new();
            hasher.update(door_id);
            hasher.update(i.to_string().as_bytes());
            let hash = hasher.finalize();
            if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
                Some((i, hash.into()))
            } else {
                None
            }
        })
        .collect()
}

fn solve_both(door_id: &str) -> (String, String) {
    let id_bytes = door_id.as_bytes();
    let mut p1 = String::with_capacity(8);
    let mut p2 = [None; 8];
    let mut p2_found = 0;
    let mut batch_start = 0u64;

    while p1.len() < 8 || p2_found < 8 {
        let mut hits = find_hits_in_batch(id_bytes, batch_start);
        hits.sort_unstable_by_key(|(i, _)| *i);

        for (_, hash) in hits {
            if p1.len() < 8 {
                let nibble = hash[2] & 0x0F;
                p1.push(char::from_digit(nibble as u32, 16).unwrap());
            }
            if p2_found < 8 {
                let pos = (hash[2] & 0x0F) as usize;
                if pos < 8 && p2[pos].is_none() {
                    let nibble = (hash[3] >> 4) & 0x0F;
                    p2[pos] = Some(char::from_digit(nibble as u32, 16).unwrap());
                    p2_found += 1;
                }
            }
        }
        batch_start += BATCH_SIZE;
    }

    let p2_str: String = p2.iter().map(|c| c.unwrap()).collect();
    (p1, p2_str)
}

pub fn solve(input: &str) -> (String, String) {
    solve_both(input.trim())
}

pub fn solve_part1(input: &str) -> String {
    solve_both(input.trim()).0
}

pub fn solve_part2(input: &str) -> String {
    solve_both(input.trim()).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1("abc"), "18f47a30");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2("abc"), "05ace8e3");
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day05.txt");
        assert_eq!(solve_part1(input), "d4cd2ee1");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day05.txt");
        assert_eq!(solve_part2(input), "f2c730e5");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day05.txt");
        assert_eq!(solve(input), ("d4cd2ee1".to_string(), "f2c730e5".to_string()));
    }
}
