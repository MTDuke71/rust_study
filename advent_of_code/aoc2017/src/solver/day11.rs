//! Day 11: Hex Ed
//!
//! Hex grid navigation using cube coordinates (q + r + s = 0).

/// Hex distance from origin in cube coordinates.
fn hex_distance(q: i32, r: i32, s: i32) -> i32 {
    q.abs().max(r.abs()).max(s.abs())
}

fn solve_both(input: &str) -> (i32, i32) {
    let mut q = 0i32;
    let mut r = 0i32;
    let mut s = 0i32;
    let mut max_dist = 0;

    for step in input.trim().split(',') {
        match step {
            "n" => {
                r -= 1;
                s += 1;
            }
            "ne" => {
                q += 1;
                r -= 1;
            }
            "se" => {
                q += 1;
                s -= 1;
            }
            "s" => {
                r += 1;
                s -= 1;
            }
            "sw" => {
                q -= 1;
                r += 1;
            }
            "nw" => {
                q -= 1;
                s += 1;
            }
            _ => unreachable!("invalid direction: {step}"),
        }
        max_dist = max_dist.max(hex_distance(q, r, s));
    }

    (hex_distance(q, r, s), max_dist)
}

pub fn solve(input: &str) -> (i32, i32) {
    solve_both(input)
}

pub fn solve_part1(input: &str) -> i32 {
    solve_both(input).0
}

pub fn solve_part2(input: &str) -> i32 {
    solve_both(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight_ne() {
        assert_eq!(solve_part1("ne,ne,ne"), 3);
    }

    #[test]
    fn test_cancel_out() {
        assert_eq!(solve_part1("ne,ne,sw,sw"), 0);
    }

    #[test]
    fn test_diagonal_collapse() {
        assert_eq!(solve_part1("ne,ne,s,s"), 2);
    }

    #[test]
    fn test_mixed() {
        assert_eq!(solve_part1("se,sw,se,sw,sw"), 3);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day11.txt");
        assert_eq!(solve_part1(input), 722);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day11.txt");
        assert_eq!(solve_part2(input), 1551);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day11.txt");
        assert_eq!(solve(input), (722, 1551));
    }
}
