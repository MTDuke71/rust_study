//! Day 16: Dragon Checksum
//!
//! Fill a disk using modified dragon curve expansion, then compute
//! a checksum by repeatedly pairing bits until odd length.

/// The initial state (puzzle input bits)
fn parse_input(input: &str) -> Vec<bool> {
    input.trim().bytes().map(|b| b == b'1').collect()
}

/// Dragon curve: a + 0 + reverse(flip(a))
/// Grows data until it reaches `disk_size`, then truncates.
fn dragon_fill(initial: &[bool], disk_size: usize) -> Vec<bool> {
    let mut data = initial.to_vec();
    while data.len() < disk_size {
        // b = reversed, bit-flipped copy of a
        let b: Vec<bool> = data.iter().rev().map(|&bit| !bit).collect();
        data.push(false); // the joining '0'
        data.extend(b);
    }
    data.truncate(disk_size);
    data
}

/// Checksum: pair up bits (same=1, diff=0), repeat until odd length
fn checksum(data: &[bool]) -> String {
    let mut current = data.to_vec();
    while current.len().is_multiple_of(2) {
        current = current
            .chunks(2)
            .map(|pair| pair[0] == pair[1])
            .collect();
    }
    current.iter().map(|&b| if b { '1' } else { '0' }).collect()
}

fn solve_with_disk_size(initial: &[bool], disk_size: usize) -> String {
    let filled = dragon_fill(initial, disk_size);
    checksum(&filled)
}

fn solve_part1_with_data(data: &[bool]) -> String {
    solve_with_disk_size(data, 272)
}

fn solve_part2_with_data(data: &[bool]) -> String {
    solve_with_disk_size(data, 35_651_584)
}

pub fn solve(input: &str) -> (String, String) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}

pub fn solve_part1(input: &str) -> String {
    let data = parse_input(input);
    solve_part1_with_data(&data)
}

pub fn solve_part2(input: &str) -> String {
    let data = parse_input(input);
    solve_part2_with_data(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dragon_fill_example() {
        // "10000" with disk size 20 -> "10000011110010000111"
        let initial = parse_input("10000");
        let filled = dragon_fill(&initial, 20);
        let s: String = filled.iter().map(|&b| if b { '1' } else { '0' }).collect();
        assert_eq!(s, "10000011110010000111");
    }

    #[test]
    fn test_checksum_example() {
        let data: Vec<bool> = "110010110100"
            .bytes()
            .map(|b| b == b'1')
            .collect();
        assert_eq!(checksum(&data), "100");
    }

    #[test]
    fn test_full_example() {
        // Initial "10000", disk size 20 -> checksum "01100"
        let initial = parse_input("10000");
        assert_eq!(solve_with_disk_size(&initial, 20), "01100");
    }

    #[test]
    fn test_dragon_step_examples() {
        // "1" -> "100"
        let d = dragon_fill(&parse_input("1"), 3);
        let s: String = d.iter().map(|&b| if b { '1' } else { '0' }).collect();
        assert_eq!(s, "100");

        // "0" -> "001"
        let d = dragon_fill(&parse_input("0"), 3);
        let s: String = d.iter().map(|&b| if b { '1' } else { '0' }).collect();
        assert_eq!(s, "001");

        // "11111" -> "11111000000"
        let d = dragon_fill(&parse_input("11111"), 11);
        let s: String = d.iter().map(|&b| if b { '1' } else { '0' }).collect();
        assert_eq!(s, "11111000000");
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day16.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, "10010010110011010");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day16.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, "01010100101011100");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day16.txt");
        assert_eq!(solve(input), ("10010010110011010".to_string(), "01010100101011100".to_string()));
    }
}
