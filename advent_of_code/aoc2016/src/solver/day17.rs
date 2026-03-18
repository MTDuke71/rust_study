//! Day 17: Two Steps Forward
//!
//! BFS through a 4x4 grid where door states depend on MD5 hash
//! of passcode + path taken so far. Find shortest path to (3,3).

use md5::{Digest, Md5};
use std::collections::VecDeque;

/// Parse: just the passcode string
fn parse_input(input: &str) -> &str {
    input.trim()
}

/// Compute which doors are open for the given passcode + path.
/// Returns [up, down, left, right] as bools.
/// A door is open when its nibble is >= 0xb (11), i.e. hex chars b-f.
fn open_doors(passcode: &str, path: &str) -> [bool; 4] {
    let mut hasher = Md5::new();
    hasher.update(passcode.as_bytes());
    hasher.update(path.as_bytes());
    let result = hasher.finalize();

    // First 2 bytes → 4 nibbles → 4 door states
    [
        (result[0] >> 4) >= 0xb,   // Up:    high nibble of byte 0
        (result[0] & 0x0f) >= 0xb, // Down:  low nibble of byte 0
        (result[1] >> 4) >= 0xb,   // Left:  high nibble of byte 1
        (result[1] & 0x0f) >= 0xb, // Right: low nibble of byte 1
    ]
}

/// Direction labels and (dx, dy) offsets
const DIRS: [(char, i32, i32); 4] = [
    ('U', 0, -1),  // Up
    ('D', 0, 1),   // Down
    ('L', -1, 0),  // Left
    ('R', 1, 0),   // Right
];

/// BFS to find shortest path to (3,3)
fn solve_part1_with_data(passcode: &str) -> String {
    let mut queue: VecDeque<(i32, i32, String)> = VecDeque::new();
    queue.push_back((0, 0, String::new()));

    while let Some((x, y, path)) = queue.pop_front() {
        // Reached the vault
        if x == 3 && y == 3 {
            return path;
        }

        let doors = open_doors(passcode, &path);

        for (i, &(dir_char, dx, dy)) in DIRS.iter().enumerate() {
            if !doors[i] {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            // Stay in bounds (0..4 grid)
            if !(0..=3).contains(&nx) || !(0..=3).contains(&ny) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(dir_char);
            queue.push_back((nx, ny, new_path));
        }
    }

    String::from("NO PATH")
}

/// BFS/DFS to find longest path to (3,3).
/// We must explore ALL paths — don't stop at vault, just record length.
fn solve_part2_with_data(passcode: &str) -> usize {
    let mut queue: VecDeque<(i32, i32, String)> = VecDeque::new();
    queue.push_back((0, 0, String::new()));
    let mut longest = 0;

    while let Some((x, y, path)) = queue.pop_front() {
        // Reached the vault — record length but don't explore further
        // (vault doors are all open, but we stop here per the rules)
        if x == 3 && y == 3 {
            longest = longest.max(path.len());
            continue;
        }

        let doors = open_doors(passcode, &path);

        for (i, &(dir_char, dx, dy)) in DIRS.iter().enumerate() {
            if !doors[i] {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            if !(0..=3).contains(&nx) || !(0..=3).contains(&ny) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(dir_char);
            queue.push_back((nx, ny, new_path));
        }
    }

    longest
}

pub fn solve(input: &str) -> (String, String) {
    let passcode = parse_input(input);
    (
        solve_part1_with_data(passcode),
        solve_part2_with_data(passcode).to_string(),
    )
}

pub fn solve_part1(input: &str) -> String {
    let passcode = parse_input(input);
    solve_part1_with_data(passcode)
}

pub fn solve_part2(input: &str) -> String {
    let passcode = parse_input(input);
    solve_part2_with_data(passcode).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(solve_part1_with_data("ihgpwlah"), "DDRRRD");
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(solve_part1_with_data("kglvqrro"), "DDUDRLRRUDRD");
    }

    #[test]
    fn test_part1_example3() {
        assert_eq!(
            solve_part1_with_data("ulqzkmiv"),
            "DRURDRUDDLLDLUURRDULRLDUUDDDRR"
        );
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(solve_part2_with_data("ihgpwlah"), 370);
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(solve_part2_with_data("kglvqrro"), 492);
    }

    #[test]
    fn test_part2_example3() {
        assert_eq!(solve_part2_with_data("ulqzkmiv"), 830);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day17.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, "DUDRLRRDDR");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day17.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, "788");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day17.txt");
        assert_eq!(
            solve(input),
            ("DUDRLRRDDR".to_string(), "788".to_string())
        );
    }
}
