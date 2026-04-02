//! Day 2: Corruption Checksum
//!
//! For a spreadsheet of numbers, compute a checksum based on row differences (Part 1)
//! or evenly divisible pairs (Part 2).

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve_part1_with_data(rows: &[Vec<u32>]) -> u32 {
    rows.iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}

fn solve_part2_with_data(rows: &[Vec<u32>]) -> u32 {
    rows.iter()
        .map(|row| {
            for (i, &a) in row.iter().enumerate() {
                for &b in &row[i + 1..] {
                    let (big, small) = if a > b { (a, b) } else { (b, a) };
                    if big % small == 0 {
                        return big / small;
                    }
                }
            }
            unreachable!("each row should have an evenly divisible pair")
        })
        .sum()
}

pub fn solve(input: &str) -> (u32, u32) {
    let rows = parse_input(input);
    (solve_part1_with_data(&rows), solve_part2_with_data(&rows))
}

pub fn solve_part1(input: &str) -> u32 {
    let rows = parse_input(input);
    solve_part1_with_data(&rows)
}

pub fn solve_part2(input: &str) -> u32 {
    let rows = parse_input(input);
    solve_part2_with_data(&rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_P1: &str = "5 1 9 5\n7 5 3\n2 4 6 8";
    const EXAMPLE_P2: &str = "5 9 2 8\n9 4 7 3\n3 8 6 5";

    #[test]
    fn test_parse() {
        let rows = parse_input(EXAMPLE_P1);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0], vec![5, 1, 9, 5]);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE_P1), 18);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE_P2), 9);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day02.txt");
        assert_eq!(solve_part1(input), 41919);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day02.txt");
        assert_eq!(solve_part2(input), 303);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day02.txt");
        assert_eq!(solve(input), (41919, 303));
    }
}
