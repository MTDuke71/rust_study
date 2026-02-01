use std::collections::HashMap;

/// Represents a keypad with button positions and a gap location
struct Keypad {
    positions: HashMap<char, (i32, i32)>,
    gap: (i32, i32),
}

impl Keypad {
    /// Create the numeric keypad layout
    /// ```text
    /// +---+---+---+
    /// | 7 | 8 | 9 |
    /// +---+---+---+
    /// | 4 | 5 | 6 |
    /// +---+---+---+
    /// | 1 | 2 | 3 |
    /// +---+---+---+
    ///     | 0 | A |
    ///     +---+---+
    /// ```
    fn numeric() -> Self {
        let mut positions = HashMap::new();
        positions.insert('7', (0, 0));
        positions.insert('8', (1, 0));
        positions.insert('9', (2, 0));
        positions.insert('4', (0, 1));
        positions.insert('5', (1, 1));
        positions.insert('6', (2, 1));
        positions.insert('1', (0, 2));
        positions.insert('2', (1, 2));
        positions.insert('3', (2, 2));
        positions.insert('0', (1, 3));
        positions.insert('A', (2, 3));

        Keypad {
            positions,
            gap: (0, 3),
        }
    }

    /// Create the directional keypad layout
    /// ```text
    ///     +---+---+
    ///     | ^ | A |
    /// +---+---+---+
    /// | < | v | > |
    /// +---+---+---+
    /// ```
    fn directional() -> Self {
        let mut positions = HashMap::new();
        positions.insert('^', (1, 0));
        positions.insert('A', (2, 0));
        positions.insert('<', (0, 1));
        positions.insert('v', (1, 1));
        positions.insert('>', (2, 1));

        Keypad {
            positions,
            gap: (0, 0),
        }
    }

    /// Generate all shortest movement sequences from one button to another
    /// Returns sequences that avoid the gap
    fn get_move_sequences(&self, from: char, to: char) -> Vec<String> {
        let (x1, y1) = self.positions[&from];
        let (x2, y2) = self.positions[&to];

        // If already at target, no movement needed
        if from == to {
            return vec![String::new()];
        }

        let dx = x2 - x1;
        let dy = y2 - y1;

        // Generate horizontal and vertical movement strings
        let horizontal = if dx > 0 {
            ">".repeat(dx as usize)
        } else {
            "<".repeat((-dx) as usize)
        };

        let vertical = if dy > 0 {
            "v".repeat(dy as usize)
        } else {
            "^".repeat((-dy) as usize)
        };

        let mut sequences = Vec::new();

        // Try horizontal-first path
        if dx != 0 && dy != 0 {
            // Check if horizontal-first path goes through gap
            let intermediate = (x2, y1);
            if intermediate != self.gap {
                sequences.push(format!("{}{}", horizontal, vertical));
            }

            // Check if vertical-first path goes through gap
            let intermediate = (x1, y2);
            if intermediate != self.gap {
                sequences.push(format!("{}{}", vertical, horizontal));
            }
        } else {
            // Only one direction needed
            sequences.push(format!("{}{}", horizontal, vertical));
        }

        sequences
    }
}

type Memo = HashMap<(String, usize), usize>;

/// Calculate minimum number of button presses needed at depth levels
/// 
/// # Arguments
/// * `sequence` - The sequence to type at this level
/// * `depth` - How many levels of robots remain (0 = you typing directly)
/// * `is_numeric` - Whether this level uses numeric keypad (only true for final robot)
/// * `memo` - Memoization cache
fn min_presses(
    sequence: &str,
    depth: usize,
    is_numeric: bool,
    memo: &mut Memo,
) -> usize {
    // Base case: at depth 0, you type the sequence directly
    if depth == 0 {
        return sequence.len();
    }

    // Check memoization cache
    let key = (sequence.to_string(), depth);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    // Select appropriate keypad
    let keypad = if is_numeric {
        Keypad::numeric()
    } else {
        Keypad::directional()
    };

    let mut total = 0;
    let mut current = 'A'; // All robots start at A

    // Process each button in the sequence
    for target in sequence.chars() {
        // Get all shortest paths from current to target
        let moves = keypad.get_move_sequences(current, target);

        // Try each possible path and pick the one that results in minimum presses at next level
        let min_cost = moves
            .iter()
            .map(|path| {
                // Add 'A' to press the button after moving
                let full_sequence = format!("{}A", path);
                // Next level always uses directional keypad (except the final numeric one)
                min_presses(&full_sequence, depth - 1, false, memo)
            })
            .min()
            .unwrap_or(0);

        total += min_cost;
        current = target; // Update current position
    }

    // Cache the result
    memo.insert(key, total);
    total
}

/// Calculate complexity for a single code
fn calculate_complexity(code: &str, depth: usize, memo: &mut Memo) -> usize {
    // Extract numeric part (e.g., "029A" -> 29)
    let numeric_part: usize = code[..code.len() - 1]
        .trim_start_matches('0')
        .parse()
        .unwrap_or(0);

    // Calculate minimum button presses
    // The final robot uses numeric keypad, all others use directional
    let length = min_presses(code, depth, true, memo);

    numeric_part * length
}

pub fn solve_part1(input: &str) -> usize {
    let codes: Vec<&str> = input.trim().lines().collect();
    let mut memo = HashMap::new();

    // Part 1: depth = 3
    // Level 0: You (directional)
    // Level 1: Robot 1 (directional)
    // Level 2: Robot 2 (directional)
    // Level 3: Final robot (numeric)
    codes
        .iter()
        .map(|code| calculate_complexity(code, 3, &mut memo))
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    let codes: Vec<&str> = input.trim().lines().collect();
    let mut memo = HashMap::new();

    // Part 2: depth = 26
    // Level 0: You (directional)
    // Level 1-25: 25 robots (directional)
    // Level 26: Final robot (numeric)
    codes
        .iter()
        .map(|code| calculate_complexity(code, 26, &mut memo))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_numeric_keypad() {
        let keypad = Keypad::numeric();
        assert_eq!(keypad.positions[&'7'], (0, 0));
        assert_eq!(keypad.positions[&'A'], (2, 3));
        assert_eq!(keypad.gap, (0, 3));
    }

    #[test]
    fn test_directional_keypad() {
        let keypad = Keypad::directional();
        assert_eq!(keypad.positions[&'^'], (1, 0));
        assert_eq!(keypad.positions[&'A'], (2, 0));
        assert_eq!(keypad.positions[&'<'], (0, 1));
        assert_eq!(keypad.gap, (0, 0));
    }

    #[test]
    fn test_move_sequences_no_gap() {
        let keypad = Keypad::numeric();
        let sequences = keypad.get_move_sequences('A', '0');
        // A(2,3) to 0(1,3): just move left
        assert_eq!(sequences, vec!["<"]);
    }

    #[test]
    fn test_move_sequences_with_gap_avoidance() {
        let keypad = Keypad::numeric();
        let sequences = keypad.get_move_sequences('A', '7');
        // A(2,3) to 7(0,0): need to avoid gap at (0,3)
        // Can do: ^^< or <^^ but NOT <vv (goes through gap)
        // Actually: up-then-left or left-then-up
        // Left-then-up would go through (0,3) which is gap, so should only have up-then-left
        assert!(sequences.len() <= 2);
        assert!(sequences.iter().all(|s| s.contains('^') && s.contains('<')));
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 126384);
    }

    #[test]
    fn test_single_code_029a() {
        let mut memo = HashMap::new();
        let complexity = calculate_complexity("029A", 3, &mut memo);
        // According to problem: length = 68, numeric = 29
        // complexity = 68 * 29 = 1972
        assert_eq!(complexity, 1972);
    }

    #[test]
    fn test_memoization() {
        let mut memo = HashMap::new();
        let result1 = min_presses("A", 1, false, &mut memo);
        let result2 = min_presses("A", 1, false, &mut memo);
        assert_eq!(result1, result2);
        assert!(memo.contains_key(&("A".to_string(), 1)));
    }
}
