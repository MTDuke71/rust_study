//! Day 6: Memory Reallocation
//!
//! Detect cycles in memory bank redistribution by tracking seen states.

use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn redistribute(banks: &mut [u32]) {
    let len = banks.len();
    // Find bank with most blocks (lowest index wins ties)
    let (max_idx, &max_val) = banks
        .iter()
        .enumerate()
        .max_by(|(i, a), (j, b)| a.cmp(b).then(j.cmp(i)))
        .unwrap();

    banks[max_idx] = 0;
    let full = max_val as usize / len;
    let remainder = max_val as usize % len;

    for bank in banks.iter_mut() {
        *bank += full as u32;
    }
    for offset in 1..=remainder {
        banks[(max_idx + offset) % len] += 1;
    }
}

fn solve_both(input: &str) -> (usize, usize) {
    let mut banks = parse_input(input);
    let mut seen: HashMap<Vec<u32>, usize> = HashMap::new();
    let mut step = 0;

    loop {
        if let Some(&first_seen) = seen.get(&banks) {
            return (step, step - first_seen);
        }
        seen.insert(banks.clone(), step);
        redistribute(&mut banks);
        step += 1;
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    solve_both(input)
}

pub fn solve_part1(input: &str) -> usize {
    solve_both(input).0
}

pub fn solve_part2(input: &str) -> usize {
    solve_both(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0\t2\t7\t0";

    #[test]
    fn test_parse() {
        assert_eq!(parse_input(EXAMPLE), vec![0, 2, 7, 0]);
    }

    #[test]
    fn test_redistribute_step1() {
        let mut banks = vec![0, 2, 7, 0];
        redistribute(&mut banks);
        assert_eq!(banks, vec![2, 4, 1, 2]);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 5);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 4);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day06.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, 6681);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day06.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, 2392);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day06.txt");
        assert_eq!(solve(input), (6681, 2392));
    }
}
