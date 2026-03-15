//! Day 14: One-Time Pad
//!
//! Find one-time pad keys by matching MD5 hash triplets with quintuplets
//! within a 1000-index lookahead window.
//! Part 1: Plain MD5 hashing.
//! Part 2: Key-stretched hashing (2017 rounds of MD5 per index).

use md5::{Digest, Md5};
use rayon::prelude::*;

/// Compute the lowercase hex MD5 of `input`.
fn md5_hex(input: &[u8]) -> String {
    let hash = Md5::digest(input);
    // Manual hex formatting to avoid format! overhead
    let mut hex = String::with_capacity(32);
    for byte in hash.iter() {
        hex.push(char::from_digit((byte >> 4) as u32, 16).unwrap());
        hex.push(char::from_digit((byte & 0x0F) as u32, 16).unwrap());
    }
    hex
}

/// Compute a key-stretched hash: MD5 the salt+index, then MD5 the hex result
/// 2016 more times (2017 total).
fn stretched_hash(salt: &str, index: u64) -> String {
    let mut hex = md5_hex(format!("{salt}{index}").as_bytes());
    for _ in 0..2016 {
        hex = md5_hex(hex.as_bytes());
    }
    hex
}

/// Plain MD5 hash of salt+index.
fn plain_hash(salt: &str, index: u64) -> String {
    md5_hex(format!("{salt}{index}").as_bytes())
}

/// Find the first triplet character in a hex string (if any).
fn find_triplet(hex: &str) -> Option<u8> {
    let bytes = hex.as_bytes();
    bytes
        .windows(3)
        .find(|w| w[0] == w[1] && w[1] == w[2])
        .map(|w| w[0])
}

/// Check if the hex string contains five consecutive copies of `ch`.
fn has_quintuplet(hex: &str, ch: u8) -> bool {
    let target = [ch; 5];
    hex.as_bytes().windows(5).any(|w| w == target)
}

const BATCH_SIZE: usize = 2000;

/// Find the index of the 64th one-time pad key using the given hash function.
/// Hashes are computed in parallel batches with Rayon, then matched sequentially.
fn find_64th_key<F>(salt: &str, hash_fn: F) -> u64
where
    F: Fn(&str, u64) -> String + Sync,
{
    let mut cache: Vec<String> = Vec::new();

    // Pre-compute hashes in parallel batches
    let ensure_cached = |cache: &mut Vec<String>, up_to: usize| {
        if up_to < cache.len() {
            return;
        }
        let start = cache.len();
        let batch: Vec<String> = (start..=up_to)
            .into_par_iter()
            .map(|i| hash_fn(salt, i as u64))
            .collect();
        cache.extend(batch);
    };

    let mut keys_found = 0;
    let mut index = 0usize;

    loop {
        // Ensure we have enough hashes for current index + 1000 lookahead
        // Batch in chunks to amortize Rayon overhead
        let needed = index + 1000;
        if needed >= cache.len() {
            let batch_end = needed + BATCH_SIZE;
            ensure_cached(&mut cache, batch_end);
        }

        if let Some(ch) = find_triplet(&cache[index]) {
            let found = (index + 1..=index + 1000)
                .any(|j| has_quintuplet(&cache[j], ch));

            if found {
                keys_found += 1;
                if keys_found == 64 {
                    return index as u64;
                }
            }
        }

        index += 1;
    }
}

fn parse_input(input: &str) -> &str {
    input.trim()
}

fn solve_part1_with_data(salt: &str) -> u64 {
    find_64th_key(salt, plain_hash)
}

fn solve_part2_with_data(salt: &str) -> u64 {
    find_64th_key(salt, stretched_hash)
}

pub fn solve(input: &str) -> (u64, u64) {
    let salt = parse_input(input);
    (solve_part1_with_data(salt), solve_part2_with_data(salt))
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
    fn test_triplet_detection() {
        assert_eq!(find_triplet("abc0cc888def"), Some(b'8'));
        assert_eq!(find_triplet("abcdef"), None);
        assert_eq!(find_triplet("aabbc"), None);
    }

    #[test]
    fn test_quintuplet_detection() {
        assert!(has_quintuplet("abc88888def", b'8'));
        assert!(!has_quintuplet("abc8888def", b'8'));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1("abc"), 22728);
    }

    #[test]
    #[ignore] // Slow (~30s with key stretching)
    fn test_part2_example() {
        assert_eq!(solve_part2("abc"), 22551);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day14.txt");
        let result = solve_part1(input);
        assert_eq!(result, 18626);
    }

    #[test]
    #[ignore] // Slow
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day14.txt");
        let result = solve_part2(input);
        assert_eq!(result, 20092);
    }

    #[test]
    #[ignore] // Slow
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day14.txt");
        assert_eq!(solve(input), (18626, 20092));
    }
}
