//! Day 9: Stream Processing
//!
//! State machine parser for nested groups `{}` and garbage `<>` with `!` escaping.

/// Parsed results from a single pass through the stream.
#[derive(Debug)]
struct StreamResult {
    total_score: u32,
    garbage_count: u32,
}

fn parse_stream(input: &str) -> StreamResult {
    let mut total_score = 0u32;
    let mut garbage_count = 0u32;
    let mut depth = 0u32;
    let mut in_garbage = false;
    let mut skip_next = false;

    for ch in input.trim().chars() {
        if skip_next {
            skip_next = false;
            continue;
        }

        if in_garbage {
            match ch {
                '!' => skip_next = true,
                '>' => in_garbage = false,
                _ => garbage_count += 1,
            }
        } else {
            match ch {
                '{' => {
                    depth += 1;
                }
                '}' => {
                    total_score += depth;
                    depth -= 1;
                }
                '<' => in_garbage = true,
                '!' => skip_next = true,
                _ => {} // commas, etc.
            }
        }
    }

    StreamResult {
        total_score,
        garbage_count,
    }
}

fn solve_part1_with_data(result: &StreamResult) -> u32 {
    result.total_score
}

fn solve_part2_with_data(result: &StreamResult) -> u32 {
    result.garbage_count
}

pub fn solve(input: &str) -> (u32, u32) {
    let data = parse_stream(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}

pub fn solve_part1(input: &str) -> u32 {
    let data = parse_stream(input);
    solve_part1_with_data(&data)
}

pub fn solve_part2(input: &str) -> u32 {
    let data = parse_stream(input);
    solve_part2_with_data(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_single_group() {
        assert_eq!(solve_part1("{}"), 1);
    }

    #[test]
    fn test_score_nested() {
        assert_eq!(solve_part1("{{{}}}"), 6);
    }

    #[test]
    fn test_score_siblings() {
        assert_eq!(solve_part1("{{},{}}"), 5);
    }

    #[test]
    fn test_score_complex() {
        assert_eq!(solve_part1("{{{},{},{{}}}}"), 16);
    }

    #[test]
    fn test_score_with_garbage() {
        assert_eq!(solve_part1("{<a>,<a>,<a>,<a>}"), 1);
    }

    #[test]
    fn test_score_groups_with_garbage() {
        assert_eq!(solve_part1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    }

    #[test]
    fn test_score_escaped_in_garbage() {
        assert_eq!(solve_part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    }

    #[test]
    fn test_score_escaped_close() {
        assert_eq!(solve_part1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn test_garbage_count_empty() {
        assert_eq!(solve_part2("<>"), 0);
    }

    #[test]
    fn test_garbage_count_chars() {
        assert_eq!(solve_part2("<random characters>"), 17);
    }

    #[test]
    fn test_garbage_count_angles() {
        assert_eq!(solve_part2("<<<<>"), 3);
    }

    #[test]
    fn test_garbage_count_escaped() {
        assert_eq!(solve_part2("<{!>}>"), 2);
    }

    #[test]
    fn test_garbage_count_double_escape() {
        assert_eq!(solve_part2("<!!>"), 0);
    }

    #[test]
    fn test_garbage_count_mixed() {
        assert_eq!(solve_part2("<!!!>>"), 0);
    }

    #[test]
    fn test_garbage_count_multi() {
        assert_eq!(solve_part2(r#"<{o"i!a,<{i<a>"#), 10);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day09.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, 7640);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day09.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, 4368);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day09.txt");
        assert_eq!(solve(input), (7640, 4368));
    }
}
