//! Day 6: Tuning Trouble
//!
//! **Problem**: Find first position where the last N characters are all unique.
//!
//! **Approach**: Rolling XOR bitset with O(1) per slide.
//!
//! **Key Insights**:
//! - Part 1: Window size 4 (start-of-packet marker)
//! - Part 2: Window size 14 (start-of-message marker)
//! - XOR toggles bits: odd count = ON, even count = OFF
//! - If popcount == window_size, all chars appear exactly once (unique)
//! - Rolling XOR: XOR out leaving char, XOR in entering char — O(1) per slide

// ============================================================================
// Core Algorithm
// ============================================================================

/// Find the first position where the last `window_size` characters are all unique.
/// Returns the 1-based position (number of characters processed).
///
/// Uses a rolling XOR bitset: each bit represents a letter (a=0, b=1, ..., z=25).
/// XOR toggles bits — a character appearing an odd number of times has its bit ON,
/// even number of times has its bit OFF. If `count_ones() == window_size`, then
/// `w` distinct bits are ON, meaning `w` distinct characters each appear an odd
/// number of times. Since we have exactly `w` characters total, each must appear
/// exactly once — uniqueness confirmed.
///
/// Rolling update: XOR out the leaving byte, XOR in the entering byte.
/// Complexity: O(n) total — two XOR ops + one popcount per slide.
fn find_marker(input: &[u8], window_size: usize) -> usize {
    let mut bits: u32 = 0;

    for (i, &b) in input.iter().enumerate() {
        // XOR in the entering character (toggles its bit)
        bits ^= 1 << (b - b'a');

        // XOR out the leaving character (once window is full)
        if i >= window_size {
            bits ^= 1 << (input[i - window_size] - b'a');
        }

        // Check: window is full and all chars are unique
        if i >= window_size - 1 && bits.count_ones() as usize == window_size {
            return i + 1;
        }
    }
    panic!("No marker found in input");
}

// ============================================================================
// Part 1 Logic
// ============================================================================

fn solve_part1_impl(input: &str) -> usize {
    find_marker(input.trim().as_bytes(), 4)
}

// ============================================================================
// Part 2 Logic
// ============================================================================

fn solve_part2_impl(input: &str) -> usize {
    find_marker(input.trim().as_bytes(), 14)
}

// ============================================================================
// Public API
// ============================================================================

/// Solve both parts
pub fn solve(input: &str) -> (usize, usize) {
    (solve_part1_impl(input), solve_part2_impl(input))
}

/// Solve Part 1 only (for testing)
pub fn solve_part1(input: &str) -> usize {
    solve_part1_impl(input)
}

/// Solve Part 2 only (for testing)
pub fn solve_part2(input: &str) -> usize {
    solve_part2_impl(input)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(solve_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
