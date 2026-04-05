//! Day 5: A Maze of Twisty Trampolines, All Alike
//!
//! Follow jump offsets, mutating them after each step, until escaping the list.

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

fn solve_part1_with_data(data: &[i32]) -> usize {
    let mut jumps = data.to_vec();
    let mut pc: i32 = 0;
    let mut steps = 0;
    let len = jumps.len() as i32;

    while pc >= 0 && pc < len {
        let offset = jumps[pc as usize];
        jumps[pc as usize] += 1;
        pc += offset;
        steps += 1;
    }

    steps
}

fn solve_part2_with_data(data: &[i32]) -> usize {
    let mut jumps = data.to_vec();
    let mut pc: i32 = 0;
    let mut steps = 0;
    let len = jumps.len() as i32;

    while pc >= 0 && pc < len {
        let offset = jumps[pc as usize];
        if offset >= 3 {
            jumps[pc as usize] -= 1;
        } else {
            jumps[pc as usize] += 1;
        }
        pc += offset;
        steps += 1;
    }

    steps
}

pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}

pub fn solve_part1(input: &str) -> usize {
    let data = parse_input(input);
    solve_part1_with_data(&data)
}

pub fn solve_part2(input: &str) -> usize {
    let data = parse_input(input);
    solve_part2_with_data(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0\n3\n0\n1\n-3";

    #[test]
    fn test_parse() {
        let data = parse_input(EXAMPLE);
        assert_eq!(data, vec![0, 3, 0, 1, -3]);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 5);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 10);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day05.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, 374_269);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day05.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, 27_720_699);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day05.txt");
        assert_eq!(solve(input), (374_269, 27_720_699));
    }
}
