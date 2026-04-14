//! Day 13: Packet Scanners
//!
//! Calculate scanner severity and find the minimum delay to pass through undetected.

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (depth_str, range_str) = line.split_once(": ").expect("invalid line format");
            (
                depth_str.trim().parse().expect("invalid depth"),
                range_str.trim().parse().expect("invalid range"),
            )
        })
        .collect()
}

// Scanner of range R bounces top→bottom→top with period 2*(R-1).
// It's at the top iff (picosecond mod period) == 0.

fn solve_part1_with_data(data: &[(u32, u32)]) -> u32 {
    data.iter()
        .filter(|&&(depth, range)| depth % (2 * (range - 1)) == 0)
        .map(|&(depth, range)| depth * range)
        .sum()
}

fn solve_part2_with_data(data: &[(u32, u32)]) -> u32 {
    (0..)
        .find(|&delay| {
            data.iter()
                .all(|&(depth, range)| (delay + depth) % (2 * (range - 1)) != 0)
        })
        .expect("a valid delay must exist")
}

pub fn solve(input: &str) -> (u32, u32) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}

pub fn solve_part1(input: &str) -> u32 {
    let data = parse_input(input);
    solve_part1_with_data(&data)
}

pub fn solve_part2(input: &str) -> u32 {
    let data = parse_input(input);
    solve_part2_with_data(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
0: 3
1: 2
4: 4
6: 4";

    #[test]
    fn test_parse() {
        let data = parse_input(EXAMPLE);
        assert_eq!(data, vec![(0, 3), (1, 2), (4, 4), (6, 4)]);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 24);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 10);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day13.txt");
        assert_eq!(solve_part1(input), 1840);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day13.txt");
        assert_eq!(solve_part2(input), 3850260);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day13.txt");
        assert_eq!(solve(input), (1840, 3850260));
    }
}