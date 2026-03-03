//! Day 2: Bathroom Security
//!
//! Navigate a keypad with UDLR instructions. Each line of instructions
//! produces one digit of the bathroom code. Moves that would leave
//! the keypad are ignored.

// Sentinel border: '.' ring eliminates all bounds/negative checks.
// Same idea as the 12×10 mailbox in chess programming.
const KEYPAD1: &[&[u8]] = &[
    b".....",
    b".123.",
    b".456.",
    b".789.",
    b".....",
];

const KEYPAD2: &[&[u8]] = &[
    b".......",
    b"...1...",
    b"..234..",
    b".56789.",
    b"..ABC..",
    b"...D...",
    b".......",
];

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

/// Walk a keypad following UDLR instructions. The sentinel '.' border
/// means we only need one check: is the target cell valid?
fn decode(keypad: &[&[u8]], start: (usize, usize), lines: &[&str]) -> String {
    let (mut r, mut c) = start;
    let mut code = String::new();

    for line in lines {
        for &ch in line.as_bytes() {
            let (nr, nc) = match ch {
                b'U' => (r - 1, c),
                b'D' => (r + 1, c),
                b'L' => (r, c - 1),
                b'R' => (r, c + 1),
                _ => continue,
            };
            if keypad[nr][nc] != b'.' {
                r = nr;
                c = nc;
            }
        }
        code.push(keypad[r][c] as char);
    }

    code
}

fn solve_part1_with_data(lines: &[&str]) -> String {
    decode(KEYPAD1, (2, 2), lines) // start at '5'
}

fn solve_part2_with_data(lines: &[&str]) -> String {
    decode(KEYPAD2, (3, 1), lines) // start at '5'
}

pub fn solve(input: &str) -> (String, String) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}

pub fn solve_part1(input: &str) -> String {
    solve_part1_with_data(&parse_input(input))
}

pub fn solve_part2(input: &str) -> String {
    solve_part2_with_data(&parse_input(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "ULL\nRRDDD\nLURDL\nUUUUD";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), "1985");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), "5DB3");
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day02.txt");
        assert_eq!(solve_part1(input), "69642");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day02.txt");
        assert_eq!(solve(input), ("69642".to_string(), "8CB23".to_string()));
    }
}
