//! Day 6: Tuning Trouble
//!
//! **Problem**: Find first position where the last N characters are all unique.
//!
//! **Approach**: Sliding window with bitset uniqueness check.
//!
//! **Key Insights**:
//! - Part 1: Window size 4 (start-of-packet marker)
//! - Part 2: Window size 14 (start-of-message marker)
//! - Bitset uniqueness: set bit per char, popcount == window size means all unique

// ============================================================================
// Core Algorithm
// ============================================================================

/// Find the first position where the last `window_size` characters are all unique.
/// Returns the 1-based position (number of characters processed).
fn find_marker(input: &[u8], window_size: usize) -> usize {
    input
        .windows(window_size)
        .position(|window| {
            // Use a u32 bitset - each bit represents a letter (a=0, b=1, ..., z=25)
            let mut bits: u32 = 0;
            for &b in window {
                bits |= 1 << (b - b'a');
            }
            bits.count_ones() as usize == window_size
        })
        .map(|pos| pos + window_size)
        .expect("No marker found in input")
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
