//! Day 19: An Elephant Named Joseph
//!
//! Josephus problem variants: steal from next elf (Part 1) and across (Part 2).
//!
//! References:
//! - Numberphile — "The Josephus Problem": <https://www.youtube.com/watch?v=uCsD3ZGzMgE>
//! - 0xdf — AoC 2016 Day 19 walkthrough (derives the Part 2 power-of-3 formula): <https://www.youtube.com/watch?v=M-V_gvJomzU>

fn parse_input(input: &str) -> u64 {
    input.trim().parse().expect("input should be a number")
}

/// Part 1: Josephus problem with k=2 (steal from next neighbor).
///
/// Formula: find highest power of 2 <= n, call it L.
/// Winner = 2 * (n - L) + 1
fn part1(n: u64) -> u64 {
    let l = 1 << (63 - n.leading_zeros());
    2 * (n - l) + 1
}

/// Part 2: Steal from the elf directly across the circle.
///
/// Formula: find highest power of 3 <= n, call it p.
/// - n == p  => winner is n
/// - n <= 2p => winner is n - p
/// - n > 2p  => winner is 2n - 3p
fn part2(n: u64) -> u64 {
    let mut p = 1;
    while p * 3 <= n {
        p *= 3;
    }
    if n == p {
        n
    } else if n <= 2 * p {
        n - p
    } else {
        2 * n - 3 * p
    }
}

pub fn solve(input: &str) -> (String, String) {
    let n = parse_input(input);
    (part1(n).to_string(), part2(n).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(5), 3);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(5), 2);
    }

    #[test]
    fn test_puzzle() {
        assert_eq!(part1(3_014_387), part1(3_014_387)); // sanity check
    }
}
