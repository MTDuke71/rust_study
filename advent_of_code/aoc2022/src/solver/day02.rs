//! Day 2: Rock Paper Scissors
//!
//! **Problem**: Score a Rock-Paper-Scissors tournament using an encrypted strategy guide.
//!
//! **Approach**: Single-pass with two 3x3 lookup tables for efficient scoring.
//!
//! # Part 1
//! - Column 1: Opponent's move (A=Rock, B=Paper, C=Scissors)
//! - Column 2: Your move (X=Rock, Y=Paper, Z=Scissors)
//! - Score = shape score (1/2/3) + outcome score (0/3/6)
//!
//! # Part 2
//! - Column 2 reinterpreted: X=Lose, Y=Draw, Z=Win
//! - Score = shape score (1/2/3) + outcome score (0/3/6)

// ============================================================================
// Data Structures
// ============================================================================

/// Parsed game rounds - each round is a pair of bytes (opponent, you)
#[derive(Debug, Clone)]
pub struct GameRounds {
    pub rounds: Vec<(u8, u8)>,
}

// ============================================================================
// Lookup Tables
// ============================================================================

/// Part 1 scoring: [opponent][your_move] = total_score
/// Opponent: A=0, B=1, C=2 (Rock, Paper, Scissors)
/// You: X=0, Y=1, Z=2 (Rock, Paper, Scissors)
const PART1_SCORES: [[u8; 3]; 3] = [
    // Opponent plays Rock (A)
    [1 + 3, 2 + 6, 3], // You play: Rock (draw), Paper (win), Scissors (lose=0)
    // Opponent plays Paper (B)
    [1, 2 + 3, 3 + 6], // You play: Rock (lose=0), Paper (draw), Scissors (win)
    // Opponent plays Scissors (C)
    [1 + 6, 2, 3 + 3], // You play: Rock (win), Paper (lose=0), Scissors (draw)
];

/// Part 2 scoring: [opponent][desired_outcome] = total_score
/// Opponent: A=0, B=1, C=2 (Rock, Paper, Scissors)
/// Outcome: X=0, Y=1, Z=2 (Lose, Draw, Win)
const PART2_SCORES: [[u8; 3]; 3] = [
    // Opponent plays Rock (A)
    [3, 1 + 3, 2 + 6], // Lose (Scissors=3+0), Draw (Rock), Win (Paper)
    // Opponent plays Paper (B)
    [1, 2 + 3, 3 + 6], // Lose (Rock=1+0), Draw (Paper), Win (Scissors)
    // Opponent plays Scissors (C)
    [2, 3 + 3, 1 + 6], // Lose (Paper=2+0), Draw (Scissors), Win (Rock)
];

// ============================================================================
// Parsing
// ============================================================================

pub fn parse_input(input: &str) -> GameRounds {
    let rounds = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let bytes = line.as_bytes();
            if bytes.len() >= 3 {
                // Convert A/B/C to 0/1/2 and X/Y/Z to 0/1/2
                let opponent = bytes[0].wrapping_sub(b'A');
                let you = bytes[2].wrapping_sub(b'X');
                Some((opponent, you))
            } else {
                None
            }
        })
        .collect();

    GameRounds { rounds }
}

// ============================================================================
// Part 1 Logic
// ============================================================================

fn solve_part1_impl(data: &GameRounds) -> usize {
    data.rounds
        .iter()
        .map(|&(opp, you)| PART1_SCORES[opp as usize][you as usize] as usize)
        .sum()
}

// ============================================================================
// Part 2 Logic
// ============================================================================

fn solve_part2_impl(data: &GameRounds) -> usize {
    data.rounds
        .iter()
        .map(|&(opp, outcome)| PART2_SCORES[opp as usize][outcome as usize] as usize)
        .sum()
}

// ============================================================================
// Public API
// ============================================================================

pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    (solve_part1_impl(&data), solve_part2_impl(&data))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
A Y
B X
C Z";

    #[test]
    fn test_parse() {
        let data = parse_input(EXAMPLE);
        assert_eq!(data.rounds.len(), 3);
        assert_eq!(data.rounds[0], (0, 1)); // A=0, Y=1
        assert_eq!(data.rounds[1], (1, 0)); // B=1, X=0
        assert_eq!(data.rounds[2], (2, 2)); // C=2, Z=2
    }

    #[test]
    fn test_part1_example() {
        let data = parse_input(EXAMPLE);
        assert_eq!(solve_part1_impl(&data), 15);
    }

    #[test]
    fn test_part1_breakdown() {
        // A Y: Rock vs Paper = 2 + 6 = 8
        let round1 = parse_input("A Y");
        assert_eq!(solve_part1_impl(&round1), 8);

        // B X: Paper vs Rock = 1 + 0 = 1
        let round2 = parse_input("B X");
        assert_eq!(solve_part1_impl(&round2), 1);

        // C Z: Scissors vs Scissors = 3 + 3 = 6
        let round3 = parse_input("C Z");
        assert_eq!(solve_part1_impl(&round3), 6);
    }

    #[test]
    fn test_part2_example() {
        let data = parse_input(EXAMPLE);
        assert_eq!(solve_part2_impl(&data), 12);
    }

    #[test]
    fn test_part2_breakdown() {
        // A Y: Rock + Draw = Rock (1) + 3 = 4
        let round1 = parse_input("A Y");
        assert_eq!(solve_part2_impl(&round1), 4);

        // B X: Paper + Lose = Rock (1) + 0 = 1
        let round2 = parse_input("B X");
        assert_eq!(solve_part2_impl(&round2), 1);

        // C Z: Scissors + Win = Rock (1) + 6 = 7
        let round3 = parse_input("C Z");
        assert_eq!(solve_part2_impl(&round3), 7);
    }
}
