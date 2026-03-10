//! Day 9: Explosives in Cyberspace
//!
//! Decompress a file using markers `(LENxREP)` that repeat the next LEN
//! characters REP times. Part 1: single-level. Part 2: recursive expansion.

fn parse_input(input: &str) -> &[u8] {
    input.trim().as_bytes()
}

fn solve_part1_with_data(data: &[u8]) -> usize {
    let mut len = 0;
    let mut i = 0;
    while i < data.len() {
        if data[i] == b'(' {
            // Find closing paren
            let close = data[i..].iter().position(|&b| b == b')').unwrap() + i;
            let marker = std::str::from_utf8(&data[i + 1..close]).unwrap();
            let (chars, repeat) = marker.split_once('x').unwrap();
            let chars: usize = chars.parse().unwrap();
            let repeat: usize = repeat.parse().unwrap();
            len += chars * repeat;
            i = close + 1 + chars;
        } else {
            len += 1;
            i += 1;
        }
    }
    len
}

fn decompressed_len(data: &[u8]) -> usize {
    let mut len = 0;
    let mut i = 0;
    while i < data.len() {
        if data[i] == b'(' {
            let close = data[i..].iter().position(|&b| b == b')').unwrap() + i;
            let marker = std::str::from_utf8(&data[i + 1..close]).unwrap();
            let (chars, repeat) = marker.split_once('x').unwrap();
            let chars: usize = chars.parse().unwrap();
            let repeat: usize = repeat.parse().unwrap();
            let start = close + 1;
            len += decompressed_len(&data[start..start + chars]) * repeat;
            i = start + chars;
        } else {
            len += 1;
            i += 1;
        }
    }
    len
}

fn solve_part2_with_data(data: &[u8]) -> usize {
    decompressed_len(data)
}

pub fn solve(input: &str) -> (String, String) {
    let data = parse_input(input);
    (
        solve_part1_with_data(data).to_string(),
        solve_part2_with_data(data).to_string(),
    )
}

pub fn solve_part1(input: &str) -> usize {
    solve_part1_with_data(parse_input(input))
}

pub fn solve_part2(input: &str) -> usize {
    solve_part2_with_data(parse_input(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_advent() {
        assert_eq!(solve_part1("ADVENT"), 6);
    }

    #[test]
    fn test_part1_repeat() {
        assert_eq!(solve_part1("A(1x5)BC"), 7);
    }

    #[test]
    fn test_part1_triple() {
        assert_eq!(solve_part1("A(2x2)BCD"), 6);
    }

    #[test]
    fn test_part1_nested_literal() {
        // Markers inside repeated section are literal in Part 1
        assert_eq!(solve_part1("(6x1)(1x3)A"), 6);
    }

    #[test]
    fn test_part1_double_marker() {
        assert_eq!(solve_part1("X(8x2)(3x3)ABCY"), 18);
    }

    #[test]
    fn test_part2_no_markers() {
        assert_eq!(solve_part2("ADVENT"), 6);
    }

    #[test]
    fn test_part2_nested() {
        // (1x3)A → AAA = 3, then (27x12) repeats that whole section 12 times
        assert_eq!(solve_part2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241_920);
    }

    #[test]
    fn test_part2_mixed() {
        assert_eq!(
            solve_part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day09.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, "120765");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day09.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, "11658395076");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day09.txt");
        assert_eq!(solve(input), ("120765".into(), "11658395076".into()));
    }
}
