//! Day 10: Knot Hash
//!
//! Circular list reversal hash — used again in Day 14.

/// Perform one round of knot-tying on the circular list.
fn knot_round(list: &mut [u8], lengths: &[usize], pos: &mut usize, skip: &mut usize) {
    let n = list.len();
    for &len in lengths {
        // Reverse the span [pos..pos+len] with wrapping
        let mut a = *pos;
        let mut b = (*pos + len - 1) % n;
        for _ in 0..len / 2 {
            list.swap(a, b);
            a = (a + 1) % n;
            b = (b + n - 1) % n;
        }
        *pos = (*pos + len + *skip) % n;
        *skip += 1;
    }
}

fn solve_part1_with_data(lengths: &[usize]) -> u32 {
    let mut list: Vec<u8> = (0..=255).collect();
    let mut pos = 0;
    let mut skip = 0;
    knot_round(&mut list, lengths, &mut pos, &mut skip);
    list[0] as u32 * list[1] as u32
}

/// Computes the full knot hash of `input`, returning the 16 dense-hash bytes.
///
/// Reused by Day 14 to produce the 128-bit rows of the disk grid.
pub fn knot_hash_bytes(input: &str) -> [u8; 16] {
    let mut lengths: Vec<usize> = input.trim().bytes().map(|b| b as usize).collect();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    let mut list: Vec<u8> = (0..=255).collect();
    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..64 {
        knot_round(&mut list, &lengths, &mut pos, &mut skip);
    }

    // Dense hash: XOR each block of 16 into a single byte
    let mut out = [0u8; 16];
    for (i, chunk) in list.chunks(16).enumerate() {
        out[i] = chunk.iter().fold(0u8, |acc, &x| acc ^ x);
    }
    out
}

fn solve_part2_with_data(input: &str) -> String {
    knot_hash_bytes(input)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub fn solve(input: &str) -> (u32, String) {
    let lengths: Vec<usize> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let part1 = solve_part1_with_data(&lengths);
    let part2 = solve_part2_with_data(input);
    (part1, part2)
}

pub fn solve_part1(input: &str) -> u32 {
    let lengths: Vec<usize> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    solve_part1_with_data(&lengths)
}

pub fn solve_part2(input: &str) -> String {
    solve_part2_with_data(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_small_example() {
        // Example uses list size 5, so we test the knot_round directly
        let mut list: Vec<u8> = vec![0, 1, 2, 3, 4];
        let lengths = vec![3, 4, 1, 5];
        let mut pos = 0;
        let mut skip = 0;
        knot_round(&mut list, &lengths, &mut pos, &mut skip);
        assert_eq!(list[0] as u32 * list[1] as u32, 12);
    }

    #[test]
    fn test_part2_empty() {
        assert_eq!(solve_part2(""), "a2582a3a0e66e6e86e3812dcb672a272");
    }

    #[test]
    fn test_part2_aoc2017() {
        assert_eq!(solve_part2("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    }

    #[test]
    fn test_part2_123() {
        assert_eq!(solve_part2("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    }

    #[test]
    fn test_part2_124() {
        assert_eq!(solve_part2("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day10.txt");
        assert_eq!(solve_part1(input), 20056);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day10.txt");
        assert_eq!(solve_part2(input), "d9a7de4a809c56bf3a9465cb84392c8e");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day10.txt");
        let (p1, p2) = solve(input);
        assert_eq!(p1, 20056);
        assert_eq!(p2, "d9a7de4a809c56bf3a9465cb84392c8e");
    }
}
