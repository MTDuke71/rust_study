//! Day 11: Monkey in the Middle
//!
//! Simulate monkeys passing items and tracking worry levels.
//! Each monkey has an operation, divisibility test, and throw targets.

use std::collections::VecDeque;

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square, // old * old
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::Add(n) => old + n,
            Operation::Multiply(n) => old * n,
            Operation::Square => old * old,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Clone)]
struct MonkeyGame {
    monkeys: Vec<Monkey>,
}

// ============================================================================
// Parsing
// ============================================================================

fn parse_input(input: &str) -> MonkeyGame {
    let mut monkeys = Vec::new();
    let sections: Vec<&str> = input.split("\n\n").collect();

    for section in sections {
        let lines: Vec<&str> = section.lines().collect();
        if lines.len() < 6 {
            continue;
        }

        // Parse starting items (line 1)
        let items_str = lines[1].split(':').nth(1).unwrap().trim();
        let items: VecDeque<u64> = items_str
            .split(", ")
            .filter_map(|s| s.parse().ok())
            .collect();

        // Parse operation (line 2)
        let op_str = lines[2].split("= old ").nth(1).unwrap().trim();
        let operation = if op_str == "* old" {
            Operation::Square
        } else {
            let parts: Vec<&str> = op_str.split_whitespace().collect();
            let value: u64 = parts[1].parse().unwrap();
            match parts[0] {
                "+" => Operation::Add(value),
                "*" => Operation::Multiply(value),
                _ => panic!("Unknown operation"),
            }
        };

        // Parse test divisibility (line 3)
        let divisible_by: u64 = lines[3]
            .split("divisible by ")
            .nth(1)
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        // Parse targets (lines 4 and 5)
        let if_true: usize = lines[4]
            .split("monkey ")
            .nth(1)
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        let if_false: usize = lines[5]
            .split("monkey ")
            .nth(1)
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            divisible_by,
            if_true,
            if_false,
        });
    }

    MonkeyGame { monkeys }
}

// ============================================================================
// Part 1 Logic (Internal - accepts parsed data)
// ============================================================================

fn solve_part1_with_data(game: &MonkeyGame) -> u64 {
    let mut game = game.clone();
    let mut inspection_counts = vec![0u64; game.monkeys.len()];

    // Simulate 20 rounds
    for _round in 0..20 {
        // Note: Can't use iterator here - we mutate game.monkeys while iterating
        #[allow(clippy::needless_range_loop)]
        for monkey_idx in 0..game.monkeys.len() {
            // Process all items this monkey is holding
            while let Some(worry) = game.monkeys[monkey_idx].items.pop_front() {
                inspection_counts[monkey_idx] += 1;

                // Apply operation
                let new_worry = game.monkeys[monkey_idx].operation.apply(worry);

                // Relief: divide by 3
                let new_worry = new_worry / 3;

                // Test and throw
                let target = if new_worry.is_multiple_of(game.monkeys[monkey_idx].divisible_by) {
                    game.monkeys[monkey_idx].if_true
                } else {
                    game.monkeys[monkey_idx].if_false
                };

                game.monkeys[target].items.push_back(new_worry);
            }
        }
    }

    // Find top 2 inspection counts
    inspection_counts.sort_unstable();
    inspection_counts.reverse();
    inspection_counts[0] * inspection_counts[1]
}

// ============================================================================
// Part 2 Logic (Internal - accepts parsed data)
// ============================================================================

fn solve_part2_with_data(game: &MonkeyGame) -> u64 {
    let mut game = game.clone();
    let mut inspection_counts = vec![0u64; game.monkeys.len()];

    // Key insight: To keep worry levels manageable, use modulo of the product
    // of all divisors. This preserves all divisibility properties while
    // preventing overflow.
    let modulo: u64 = game.monkeys.iter().map(|m| m.divisible_by).product();

    // Simulate 10,000 rounds (no worry relief!)
    for _round in 0..10_000 {
        // Note: Can't use iterator here - we mutate game.monkeys while iterating
        #[allow(clippy::needless_range_loop)]
        for monkey_idx in 0..game.monkeys.len() {
            // Process all items this monkey is holding
            while let Some(worry) = game.monkeys[monkey_idx].items.pop_front() {
                inspection_counts[monkey_idx] += 1;

                // Apply operation
                let mut new_worry = game.monkeys[monkey_idx].operation.apply(worry);

                // NO relief in Part 2! But use modulo to keep numbers manageable
                new_worry %= modulo;

                // Test and throw
                let target = if new_worry.is_multiple_of(game.monkeys[monkey_idx].divisible_by) {
                    game.monkeys[monkey_idx].if_true
                } else {
                    game.monkeys[monkey_idx].if_false
                };

                game.monkeys[target].items.push_back(new_worry);
            }
        }
    }

    // Find top 2 inspection counts
    inspection_counts.sort_unstable();
    inspection_counts.reverse();
    inspection_counts[0] * inspection_counts[1]
}

// ============================================================================
// Public API
// ============================================================================

/// Solve both parts efficiently (PARSE ONCE!)
pub fn solve(input: &str) -> (u64, u64) {
    let game = parse_input(input);
    (solve_part1_with_data(&game), solve_part2_with_data(&game))
}

/// Solve Part 1 only (for testing)
pub fn solve_part1(input: &str) -> u64 {
    solve_part1_with_data(&parse_input(input))
}

/// Solve Part 2 only (for testing)
pub fn solve_part2(input: &str) -> u64 {
    solve_part2_with_data(&parse_input(input))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_parse() {
        let game = parse_input(EXAMPLE);
        assert_eq!(game.monkeys.len(), 4);
        assert_eq!(game.monkeys[0].items.len(), 2);
        assert_eq!(game.monkeys[0].divisible_by, 23);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 10605);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 2713310158);
    }
}
