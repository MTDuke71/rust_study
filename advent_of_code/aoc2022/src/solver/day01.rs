//! Day 1: Calorie Counting
//!
//! **Problem**: Elves carrying food inventories. Find the elf with the most calories.
//!
//! **Approach**: Parse groups separated by blank lines, sum each group, find max.

// ============================================================================
// Data Structures
// ============================================================================

/// Parsed elf inventories - each elf has a total calorie count
#[derive(Debug, Clone)]
pub struct ElfInventories {
    pub totals: Vec<usize>,
}

// ============================================================================
// Parsing
// ============================================================================

pub fn parse_input(input: &str) -> ElfInventories {
    let totals = input
        .split("\n\n") // Split by blank lines
        .map(|group| {
            group
                .lines()
                .filter_map(|line| line.trim().parse::<usize>().ok())
                .sum()
        })
        .collect();

    ElfInventories { totals }
}

// ============================================================================
// Part 1 Logic
// ============================================================================

fn solve_part1_impl(data: &ElfInventories) -> usize {
    // Find the maximum calories carried by any single elf
    *data.totals.iter().max().unwrap_or(&0)
}

// ============================================================================
// Part 2 Logic
// ============================================================================

fn solve_part2_impl(data: &ElfInventories) -> usize {
    // Find the sum of the top 3 elves' calories
    let mut sorted = data.totals.clone();
    sorted.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending
    sorted.iter().take(3).sum()
}

// ============================================================================
// Public API
// ============================================================================

/// Solve both parts efficiently (parse once)
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    (solve_part1_impl(&data), solve_part2_impl(&data))
}

/// Solve Part 1 only (for testing)
pub fn solve_part1(input: &str) -> usize {
    solve_part1_impl(&parse_input(input))
}

/// Solve Part 2 only (for testing)
pub fn solve_part2(input: &str) -> usize {
    solve_part2_impl(&parse_input(input))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_parse() {
        let data = parse_input(EXAMPLE);
        assert_eq!(data.totals.len(), 5);
        assert_eq!(data.totals[0], 6000); // First elf
        assert_eq!(data.totals[1], 4000); // Second elf
        assert_eq!(data.totals[2], 11000); // Third elf
        assert_eq!(data.totals[3], 24000); // Fourth elf
        assert_eq!(data.totals[4], 10000); // Fifth elf
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 24000);
    }

    #[test]
    fn test_part2_example() {
        // Top 3: 24000 + 11000 + 10000 = 45000
        assert_eq!(solve_part2(EXAMPLE), 45000);
    }
}
