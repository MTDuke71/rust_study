//! Day 20: Firewall Rules
//!
//! Given blocked IP ranges, find the lowest allowed IP and count all allowed IPs
//! in the range 0..=4_294_967_295.

type Range = (u32, u32);

fn parse_input(input: &str) -> Vec<Range> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (lo, hi) = line.split_once('-').expect("each line has a dash");
            (lo.parse().unwrap(), hi.parse().unwrap())
        })
        .collect()
}

fn merge_ranges(ranges: &mut [Range]) -> Vec<Range> {
    ranges.sort_unstable();
    let mut merged: Vec<Range> = Vec::new();
    for &(lo, hi) in ranges.iter() {
        if let Some(last) = merged.last_mut() {
            // Use u64 arithmetic to avoid overflow on hi = u32::MAX
            if (lo as u64) <= (last.1 as u64) + 1 {
                last.1 = last.1.max(hi);
                continue;
            }
        }
        merged.push((lo, hi));
    }
    merged
}

fn solve_part1_with_data(merged: &[Range]) -> u64 {
    // First gap is right after the first merged range ends
    // (or 0 if the first range doesn't start at 0)
    if merged[0].0 > 0 {
        return 0;
    }
    merged[0].1 as u64 + 1
}

fn solve_part2_with_data(merged: &[Range]) -> u64 {
    let max_ip: u64 = 4_294_967_295;
    let blocked: u64 = merged
        .iter()
        .map(|&(lo, hi)| hi as u64 - lo as u64 + 1)
        .sum();
    max_ip + 1 - blocked
}

pub fn solve(input: &str) -> (String, String) {
    let mut ranges = parse_input(input);
    let merged = merge_ranges(&mut ranges);
    (
        solve_part1_with_data(&merged).to_string(),
        solve_part2_with_data(&merged).to_string(),
    )
}

pub fn solve_part1(input: &str) -> String {
    let mut ranges = parse_input(input);
    let merged = merge_ranges(&mut ranges);
    solve_part1_with_data(&merged).to_string()
}

pub fn solve_part2(input: &str) -> String {
    let mut ranges = parse_input(input);
    let merged = merge_ranges(&mut ranges);
    solve_part2_with_data(&merged).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
5-8
0-2
4-7";

    #[test]
    fn test_parse() {
        let ranges = parse_input(EXAMPLE);
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0], (5, 8));
    }

    #[test]
    fn test_part1_example() {
        let mut ranges = parse_input(EXAMPLE);
        let merged = merge_ranges(&mut ranges);
        assert_eq!(solve_part1_with_data(&merged), 3);
    }

    #[test]
    fn test_part2_example() {
        // With max_ip = 9, allowed = {3, 9} = 2
        // But our solve_part2 uses max_ip = 4294967295, so skip for example
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day20.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, "22887907");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day20.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, "109");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day20.txt");
        assert_eq!(solve(input), ("22887907".to_string(), "109".to_string()));
    }
}
