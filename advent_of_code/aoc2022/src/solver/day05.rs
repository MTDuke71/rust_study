//! Day 5: Supply Stacks
//!
//! **Problem**: Rearrange crates between stacks using a crane.
//!
//! **Approach**: Parse stack configuration and move instructions, simulate crane operations.
//!
//! **Key Insights**:
//! - Part 1: Crane moves crates one at a time (LIFO - stack behavior)
//! - Stacks are represented as `Vec<char>` (top is end of vec)

// ============================================================================
// Data Structures
// ============================================================================

/// A single move instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub count: usize,
    pub from: usize, // 1-indexed
    pub to: usize,   // 1-indexed
}

/// The initial configuration and move instructions
#[derive(Debug, Clone)]
pub struct CargoArrangement {
    pub stacks: Vec<Vec<char>>,
    pub moves: Vec<Move>,
}

// ============================================================================
// Parsing
// ============================================================================

pub fn parse_input(input: &str) -> CargoArrangement {
    let parts: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(
        parts.len(),
        2,
        "Input must have stacks and moves separated by blank line"
    );

    let stacks = parse_stacks(parts[0]);
    let moves = parse_moves(parts[1]);

    CargoArrangement { stacks, moves }
}

fn parse_stacks(stack_text: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = stack_text.lines().collect();
    if lines.is_empty() {
        return Vec::new();
    }

    // Last line contains stack numbers - use it to determine count
    let last_line = lines.last().unwrap();
    let num_stacks = last_line
        .split_whitespace()
        .last()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);

    let mut stacks = vec![Vec::new(); num_stacks];

    // Process lines from bottom to top (excluding the number line)
    for line in lines.iter().rev().skip(1) {
        // Each crate position is at index: 1, 5, 9, 13, ... (i.e., 1 + 4*n)
        for (stack_idx, crate_col) in (1..).step_by(4).enumerate() {
            if crate_col >= line.len() {
                break;
            }

            if let Some(c) = line.chars().nth(crate_col) {
                if c.is_alphabetic() {
                    stacks[stack_idx].push(c);
                }
            }
        }
    }

    stacks
}

fn parse_moves(moves_text: &str) -> Vec<Move> {
    moves_text
        .lines()
        .filter_map(|line| {
            // Format: "move N from X to Y"
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 6 && parts[0] == "move" {
                Some(Move {
                    count: parts[1].parse().ok()?,
                    from: parts[3].parse().ok()?,
                    to: parts[5].parse().ok()?,
                })
            } else {
                None
            }
        })
        .collect()
}

// ============================================================================
// Part 1 Logic
// ============================================================================

fn solve_part1_impl(data: &CargoArrangement) -> String {
    let mut stacks = data.stacks.clone();

    // Execute each move (one crate at a time)
    for &mv in &data.moves {
        for _ in 0..mv.count {
            if let Some(crate_item) = stacks[mv.from - 1].pop() {
                stacks[mv.to - 1].push(crate_item);
            }
        }
    }

    // Collect top crates
    stacks.iter().filter_map(|stack| stack.last()).collect()
}

// ============================================================================
// Part 2 Logic
// ============================================================================

fn solve_part2_impl(data: &CargoArrangement) -> String {
    // CrateMover 9001 - moves multiple crates at once, preserving order
    let mut stacks = data.stacks.clone();

    for movement in &data.moves {
        let from_idx = movement.from - 1;
        let to_idx = movement.to - 1;

        // Calculate where to split the source stack
        let stack_len = stacks[from_idx].len();
        let split_at = stack_len.saturating_sub(movement.count);

        // Take the crates off the source stack (preserving order)
        let crates_to_move = stacks[from_idx].split_off(split_at);

        // Add them to the destination stack (in same order)
        stacks[to_idx].extend(crates_to_move);
    }

    // Get top crate from each stack
    stacks.iter().filter_map(|stack| stack.last()).collect()
}

// ============================================================================
// Public API
// ============================================================================

/// Solve both parts efficiently (parse once)
pub fn solve(input: &str) -> (String, String) {
    let data = parse_input(input);
    (solve_part1_impl(&data), solve_part2_impl(&data))
}

/// Solve Part 1 only (for testing)
pub fn solve_part1(input: &str) -> String {
    solve_part1_impl(&parse_input(input))
}

/// Solve Part 2 only (for testing)
pub fn solve_part2(input: &str) -> String {
    solve_part2_impl(&parse_input(input))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_parse_stacks() {
        let data = parse_input(EXAMPLE);
        assert_eq!(data.stacks.len(), 3);

        // Stack 1: Z, N (bottom to top)
        assert_eq!(data.stacks[0], vec!['Z', 'N']);

        // Stack 2: M, C, D
        assert_eq!(data.stacks[1], vec!['M', 'C', 'D']);

        // Stack 3: P
        assert_eq!(data.stacks[2], vec!['P']);
    }

    #[test]
    fn test_parse_moves() {
        let data = parse_input(EXAMPLE);
        assert_eq!(data.moves.len(), 4);

        assert_eq!(
            data.moves[0],
            Move {
                count: 1,
                from: 2,
                to: 1
            }
        );
        assert_eq!(
            data.moves[1],
            Move {
                count: 3,
                from: 1,
                to: 3
            }
        );
        assert_eq!(
            data.moves[2],
            Move {
                count: 2,
                from: 2,
                to: 1
            }
        );
        assert_eq!(
            data.moves[3],
            Move {
                count: 1,
                from: 1,
                to: 2
            }
        );
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), "CMZ");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), "MCD");
    }
}
