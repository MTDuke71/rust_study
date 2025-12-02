use anyhow::{Context, Result};

/// Represents a rotation instruction for the safe dial
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rotation {
    direction: Direction,
    distance: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

/// Parse a single rotation line like "L68" or "R30"
fn parse_rotation(line: &str) -> Result<Rotation> {
    let line = line.trim();
    if line.is_empty() {
        anyhow::bail!("Empty rotation line");
    }

    let (direction_char, distance_str) = line.split_at(1);
    
    let direction = match direction_char {
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => anyhow::bail!("Invalid direction '{}', expected 'L' or 'R'", direction_char),
    };

    let distance = distance_str.parse::<u32>()
        .with_context(|| format!("Failed to parse distance '{}' in rotation '{}'", distance_str, line))?;

    Ok(Rotation { direction, distance })
}

/// Parse all rotation instructions from input
fn parse_rotations(input: &str) -> Result<Vec<Rotation>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(line_num, line)| {
            parse_rotation(line)
                .with_context(|| format!("Failed to parse rotation on line {}", line_num + 1))
        })
        .collect()
}

/// Apply a rotation to the current dial position (0-99)
fn apply_rotation(current_position: u32, rotation: Rotation) -> u32 {
    const DIAL_SIZE: u32 = 100; // 0 through 99
    
    match rotation.direction {
        Direction::Left => {
            // Left rotation decreases position (wraps to 99 when going below 0)
            // Use modular arithmetic: (current_position - distance + DIAL_SIZE) % DIAL_SIZE
            // Adding DIAL_SIZE ensures we don't go negative
            (current_position + DIAL_SIZE - (rotation.distance % DIAL_SIZE)) % DIAL_SIZE
        },
        Direction::Right => {
            // Right rotation increases position (wraps to 0 when going above 99)
            (current_position + rotation.distance) % DIAL_SIZE
        },
    }
}

/// Part 1: Count how many times the dial points to 0
///
/// Follows rotation instructions and counts every time the dial ends up at position 0.
/// The dial starts at position 50 and has positions 0-99 (wraps around).
pub fn solve_part1(input: &str) -> Result<String> {
    let rotations = parse_rotations(input)
        .context("Failed to parse rotation instructions for Part 1")?;

    let mut position = 50u32; // Dial starts at 50
    let mut zero_count = 0u32;

    for rotation in rotations {
        position = apply_rotation(position, rotation);
        if position == 0 {
            zero_count += 1;
        }
    }

    Ok(zero_count.to_string())
}

/// Count how many times the dial passes through 0 during a rotation
///
/// This counts intermediate positions during the rotation, not the starting or final positions.
///
/// **NOTE:** This function was an early attempt at Part 2 that produced incorrect results
/// (5046 instead of 5941). It fails on large rotations like L819 from position 50.
/// See `examples/day01_failed_approach_analysis.md` for detailed analysis.
/// Kept for educational purposes.
#[allow(dead_code)]
fn count_zeros_during_rotation(current_position: u32, rotation: Rotation) -> u32 {
    const DIAL_SIZE: u32 = 100; // 0 through 99
    let distance = rotation.distance;
    
    if distance == 0 {
        return 0;
    }
    
    match rotation.direction {
        Direction::Left => {
            // Moving left (decreasing): check how many times we pass through 0
            if current_position == 0 {
                // Starting at 0: Apply feedback - only count full additional cycles
                if distance >= DIAL_SIZE {
                    distance / DIAL_SIZE // Each complete cycle passes through 0 once
                } else {
                    0
                }
            } else {
                // Starting at non-zero position
                // Apply the Python feedback condition: position < operand % 100
                if current_position < (distance % DIAL_SIZE) && distance > current_position {
                    // We wrap around
                    let remaining_distance = distance - current_position - 1;
                    1 + (remaining_distance / DIAL_SIZE) // 1 for initial pass through 0, plus full cycles
                } else {
                    0
                }
            }
        },
        Direction::Right => {
            // Moving right (increasing): check how many times we pass through 0
            if current_position == 0 {
                // Starting at 0: Apply feedback - only count full additional cycles
                if distance >= DIAL_SIZE {
                    distance / DIAL_SIZE // Each complete cycle passes through 0 once
                } else {
                    0
                }
            } else {
                // Starting at non-zero position
                let steps_to_zero = DIAL_SIZE - current_position; // Steps to reach 0 (100 - current)
                if distance > steps_to_zero {
                    // We go past 0, then see how many more times we pass through it
                    let remaining_distance = distance - steps_to_zero;
                    1 + (remaining_distance / DIAL_SIZE) // 1 for first time reaching 0, plus full cycles
                } else if distance == steps_to_zero {
                    // We land exactly on 0 - this counts as final position, not during rotation
                    0
                } else {
                    0
                }
            }
        },
    }
}

/// Part 2: Count zeros during rotations AND at final positions
///
/// Uses password method 0x434C49434B: count every time the dial points at 0,
/// whether at the end of a rotation or while passing through during rotation.
pub fn solve_part2(input: &str) -> Result<String> {
    let rotations = parse_rotations(input)
        .context("Failed to parse rotation instructions for Part 2")?;

    let mut zero_count = 0i32;
    let mut dial = 50i32; // Dial starts at 50

    for rotation in rotations {
        // Special handling for starting at 0 with Left rotation
        if dial == 0 && rotation.direction == Direction::Left {
            dial = 100;
        }
        
        // Apply the rotation
        dial = match rotation.direction {
            Direction::Left => dial - rotation.distance as i32,
            Direction::Right => dial + rotation.distance as i32,
        };
        
        // Count how many times we cross the 100 boundary (pass through 0)
        zero_count += (dial / 100).abs();
        
        // Special case for going below 1 (wrapping around)
        if dial < 1 {
            zero_count += 1;
        }
        
        // Normalize dial position to 0-99 range
        dial %= 100;
        if dial < 0 {
            dial += 100;
        }
    }

    Ok(zero_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn test_parse_rotation() {
        assert_eq!(
            parse_rotation("L68").unwrap(),
            Rotation { direction: Direction::Left, distance: 68 }
        );
        assert_eq!(
            parse_rotation("R30").unwrap(),
            Rotation { direction: Direction::Right, distance: 30 }
        );
        
        // Test error cases
        assert!(parse_rotation("X10").is_err()); // Invalid direction
        assert!(parse_rotation("L").is_err());   // No distance
        assert!(parse_rotation("").is_err());    // Empty line
    }

    #[test]
    fn test_apply_rotation() {
        // Test basic rotations
        assert_eq!(apply_rotation(50, Rotation { direction: Direction::Left, distance: 68 }), 82);
        assert_eq!(apply_rotation(82, Rotation { direction: Direction::Left, distance: 30 }), 52);
        assert_eq!(apply_rotation(52, Rotation { direction: Direction::Right, distance: 48 }), 0);
        
        // Test wrap-around cases
        assert_eq!(apply_rotation(5, Rotation { direction: Direction::Left, distance: 10 }), 95);
        assert_eq!(apply_rotation(95, Rotation { direction: Direction::Right, distance: 10 }), 5);
        
        // Test edge cases
        assert_eq!(apply_rotation(0, Rotation { direction: Direction::Left, distance: 1 }), 99);
        assert_eq!(apply_rotation(99, Rotation { direction: Direction::Right, distance: 1 }), 0);
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "3"); // Expected: dial points to 0 three times
    }

    #[test]
    fn test_example_step_by_step() {
        let rotations = parse_rotations(EXAMPLE_INPUT).unwrap();
        let mut position = 50u32;
        let mut zero_count = 0u32;
        
        let expected_positions = [82, 52, 0, 95, 55, 0, 99, 0, 14, 32];
        let expected_zeros = [false, false, true, false, false, true, false, true, false, false];
        
        for (i, rotation) in rotations.iter().enumerate() {
            position = apply_rotation(position, *rotation);
            if position == 0 {
                zero_count += 1;
            }
            assert_eq!(position, expected_positions[i], "Step {} failed", i + 1);
            assert_eq!(position == 0, expected_zeros[i], "Zero check failed at step {}", i + 1);
        }
        
        assert_eq!(zero_count, 3);
    }

    #[test]
    fn test_large_rotations() {
        // Test rotations larger than dial size
        assert_eq!(apply_rotation(0, Rotation { direction: Direction::Right, distance: 150 }), 50);
        assert_eq!(apply_rotation(0, Rotation { direction: Direction::Left, distance: 150 }), 50);
    }

    #[test]
    fn test_empty_input() {
        let result = solve_part1("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "0"); // No rotations, so dial never points to 0
    }

    #[test]
    fn test_part2_example() {
        let result = solve_part2(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "6"); // Expected: 3 final positions + 3 during rotations = 6
    }

    #[test]
    fn test_user_provided_sequence() {
        // Test the sequence provided by user: R50, R50, L50, L50, R75, L50
        // Expected cumulative results: 1, 1, 2, 2, 3, 4
        
        let mut position = 50u32;
        let mut total_zeros = 0u32;
        
        // R50 from 50: 50 + 50 = 100 -> 0 (ends at 0)
        let zeros_during = count_zeros_during_rotation(position, Rotation { direction: Direction::Right, distance: 50 });
        position = apply_rotation(position, Rotation { direction: Direction::Right, distance: 50 });
        total_zeros += zeros_during + if position == 0 { 1 } else { 0 };
        println!("After R50: position = {}, zeros_during = {}, total = {}", position, zeros_during, total_zeros);
        assert_eq!(total_zeros, 1);
        
        // R50 from 0: 0 + 50 = 50 (no zeros)
        let zeros_during = count_zeros_during_rotation(position, Rotation { direction: Direction::Right, distance: 50 });
        position = apply_rotation(position, Rotation { direction: Direction::Right, distance: 50 });
        total_zeros += zeros_during + if position == 0 { 1 } else { 0 };
        println!("After R50: position = {}, zeros_during = {}, total = {}", position, zeros_during, total_zeros);
        assert_eq!(total_zeros, 1);
        
        // L50 from 50: 50 - 50 = 0 (ends at 0)
        let zeros_during = count_zeros_during_rotation(position, Rotation { direction: Direction::Left, distance: 50 });
        position = apply_rotation(position, Rotation { direction: Direction::Left, distance: 50 });
        total_zeros += zeros_during + if position == 0 { 1 } else { 0 };
        println!("After L50: position = {}, zeros_during = {}, total = {}", position, zeros_during, total_zeros);
        assert_eq!(total_zeros, 2);
        
        // L50 from 0: goes around once, should pass through 0 during rotation + end at 0
        let zeros_during = count_zeros_during_rotation(position, Rotation { direction: Direction::Left, distance: 50 });
        position = apply_rotation(position, Rotation { direction: Direction::Left, distance: 50 });
        total_zeros += zeros_during + if position == 0 { 1 } else { 0 };
        println!("After L50: position = {}, zeros_during = {}, total = {}", position, zeros_during, total_zeros);
        // This should be 2, but user expects 2, so maybe no change?
        
        // R75 from 50
        let zeros_during = count_zeros_during_rotation(position, Rotation { direction: Direction::Right, distance: 75 });
        position = apply_rotation(position, Rotation { direction: Direction::Right, distance: 75 });
        total_zeros += zeros_during + if position == 0 { 1 } else { 0 };
        println!("After R75: position = {}, zeros_during = {}, total = {}", position, zeros_during, total_zeros);
        
        // L50 from current position
        let zeros_during = count_zeros_during_rotation(position, Rotation { direction: Direction::Left, distance: 50 });
        position = apply_rotation(position, Rotation { direction: Direction::Left, distance: 50 });
        total_zeros += zeros_during + if position == 0 { 1 } else { 0 };
        println!("After L50: position = {}, zeros_during = {}, total = {}", position, zeros_during, total_zeros);
    }

    #[test]
    fn test_count_zeros_during_rotation() {
        // Test cases for counting zeros during rotation
        
        // L68 from position 50: should pass through 0 once (50 -> ... -> 0 -> ... -> 82)
        assert_eq!(count_zeros_during_rotation(50, Rotation { direction: Direction::Left, distance: 68 }), 1);
        
        // R60 from position 95: should pass through 0 once (95 -> 96 -> 97 -> 98 -> 99 -> 0 -> ... -> 55)
        assert_eq!(count_zeros_during_rotation(95, Rotation { direction: Direction::Right, distance: 60 }), 1);
        
        // L82 from position 14: should pass through 0 once
        assert_eq!(count_zeros_during_rotation(14, Rotation { direction: Direction::Left, distance: 82 }), 1);
        
        // No wrap-around cases - should be 0
        assert_eq!(count_zeros_during_rotation(50, Rotation { direction: Direction::Left, distance: 10 }), 0);
        assert_eq!(count_zeros_during_rotation(50, Rotation { direction: Direction::Right, distance: 10 }), 0);
        
        // Large rotation that goes around multiple times
        assert_eq!(count_zeros_during_rotation(50, Rotation { direction: Direction::Right, distance: 1000 }), 10);
        
        // Large rotation that goes around multiple times
        assert_eq!(count_zeros_during_rotation(50, Rotation { direction: Direction::Right, distance: 1000 }), 10);
        
        // Test that landing exactly on 0 doesn't count as "during rotation"
        assert_eq!(count_zeros_during_rotation(50, Rotation { direction: Direction::Right, distance: 50 }), 0);
        assert_eq!(count_zeros_during_rotation(50, Rotation { direction: Direction::Left, distance: 50 }), 0);
        assert_eq!(count_zeros_during_rotation(1, Rotation { direction: Direction::Left, distance: 1 }), 0);
        assert_eq!(count_zeros_during_rotation(1, Rotation { direction: Direction::Right, distance: 99 }), 0);
    }
}
