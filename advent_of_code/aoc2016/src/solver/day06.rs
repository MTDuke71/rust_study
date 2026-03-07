//! Day 6: Signals and Noise
//!
//! Decode error-corrected messages using column-wise character frequency.
//! Part 1: Most common character per column.
//! Part 2: Least common character per column.

fn parse_input(input: &str) -> Vec<[u32; 26]> {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let mut freq = vec![[0u32; 26]; width];
    for line in &lines {
        for (col, ch) in line.bytes().enumerate() {
            freq[col][(ch - b'a') as usize] += 1;
        }
    }
    freq
}

fn solve_part1_with_data(freq: &[[u32; 26]]) -> String {
    freq.iter()
        .map(|col| {
            let idx = col.iter().enumerate().max_by_key(|&(_, &c)| c).unwrap().0;
            (b'a' + idx as u8) as char
        })
        .collect()
}

fn solve_part2_with_data(freq: &[[u32; 26]]) -> String {
    freq.iter()
        .map(|col| {
            let idx = col
                .iter()
                .enumerate()
                .filter(|&(_, &c)| c > 0)
                .min_by_key(|&(_, &c)| c)
                .unwrap()
                .0;
            (b'a' + idx as u8) as char
        })
        .collect()
}

pub fn solve(input: &str) -> (String, String) {
    let freq = parse_input(input);
    (solve_part1_with_data(&freq), solve_part2_with_data(&freq))
}

pub fn solve_part1(input: &str) -> String {
    let freq = parse_input(input);
    solve_part1_with_data(&freq)
}

pub fn solve_part2(input: &str) -> String {
    let freq = parse_input(input);
    solve_part2_with_data(&freq)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    #[test]
    fn test_parse() {
        let freq = parse_input(EXAMPLE);
        assert_eq!(freq.len(), 6);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), "easter");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), "advent");
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day06.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, "zcreqgiv");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day06.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, "pljvorrk");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day06.txt");
        assert_eq!(solve(input), ("zcreqgiv".to_string(), "pljvorrk".to_string()));
    }
}
