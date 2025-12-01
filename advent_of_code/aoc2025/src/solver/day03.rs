use anyhow::{Context, Result};
use regex::Regex;

/// Parse valid mul instructions from corrupted memory
///
/// Finds all instances of `mul(X,Y)` where X and Y are 1-3 digit numbers.
/// Invalid patterns like `mul(4*`, `mul(6,9!`, `?(12,34)`, or `mul ( 2 , 4 )` are ignored.
fn parse_mul_instructions(input: &str) -> Result<Vec<(i32, i32)>> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .context("Failed to compile regex for mul instructions")?;

    let mut instructions = Vec::new();

    for captures in re.captures_iter(input) {
        let x_str = &captures[1];
        let y_str = &captures[2];

        let x = x_str.parse::<i32>().with_context(|| {
            format!(
                "Failed to parse first number: '{}' (full match: '{}')",
                x_str,
                captures.get(0).unwrap().as_str()
            )
        })?;
        let y = y_str.parse::<i32>().with_context(|| {
            format!(
                "Failed to parse second number: '{}' (full match: '{}')",
                y_str,
                captures.get(0).unwrap().as_str()
            )
        })?;

        instructions.push((x, y));
    }

    Ok(instructions)
}

/// Parse mul instructions and conditional statements from corrupted memory
///
/// Finds all instances of:
/// - `mul(X,Y)` where X and Y are 1-3 digit numbers
/// - `do()` which enables future mul instructions
/// - `don't()` which disables future mul instructions
///
/// Returns a vector of instructions in order, where each instruction is either:
/// - `Mul(x, y)` for multiplication
/// - `Do` to enable multiplications
/// - `Dont` to disable multiplications
#[derive(Debug, PartialEq)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

fn parse_instructions_with_conditionals(input: &str) -> Result<Vec<Instruction>> {
    let re = Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\))|(do\(\)|don't\(\))")
        .context("Failed to compile regex for instructions with conditionals")?;

    let mut instructions = Vec::new();

    for captures in re.captures_iter(input) {
        if let Some(conditional) = captures.get(3) {
            // This is a conditional instruction
            match conditional.as_str() {
                "do()" => instructions.push(Instruction::Do),
                "don't()" => instructions.push(Instruction::Dont),
                _ => {
                    return Err(anyhow::anyhow!(
                        "Unknown conditional: {}",
                        conditional.as_str()
                    ))
                }
            }
        } else if let (Some(x_match), Some(y_match)) = (captures.get(1), captures.get(2)) {
            // This is a mul instruction
            let x = x_match
                .as_str()
                .parse::<i32>()
                .with_context(|| format!("Failed to parse first number: {}", x_match.as_str()))?;
            let y = y_match
                .as_str()
                .parse::<i32>()
                .with_context(|| format!("Failed to parse second number: {}", y_match.as_str()))?;

            instructions.push(Instruction::Mul(x, y));
        }
    }

    Ok(instructions)
}

/// Solve Part 1: Sum all valid mul instructions
///
/// Scans the corrupted memory for uncorrupted `mul(X,Y)` instructions and adds up
/// the results of all multiplications.
///
/// # Arguments
///
/// * `input` - The corrupted memory as a string
///
/// # Returns
///
/// The sum of all multiplication results
///
/// # Example
///
/// ```
/// use aoc2024::solver::day03::solve_part1;
///
/// let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
/// assert_eq!(solve_part1(input).unwrap(), "161");
/// ```
pub fn solve_part1(input: &str) -> Result<String> {
    let instructions = parse_mul_instructions(input.trim())
        .context("Failed to parse mul instructions from input")?;

    let total: i32 = instructions.iter().map(|(x, y)| x * y).sum();

    Ok(total.to_string())
}

/// Solve Part 2: Sum enabled mul instructions with conditional processing
///
/// Handles conditional statements in the corrupted memory:
/// - `do()` enables future mul instructions
/// - `don't()` disables future mul instructions
/// - At the beginning, mul instructions are enabled
///
/// Only the most recent `do()` or `don't()` instruction applies.
///
/// # Arguments
///
/// * `input` - The corrupted memory as a string
///
/// # Returns
///
/// The sum of all enabled multiplication results
///
/// # Example
///
/// ```
/// use aoc2024::solver::day03::solve_part2;
///
/// let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
/// assert_eq!(solve_part2(input).unwrap(), "48");
/// ```
pub fn solve_part2(input: &str) -> Result<String> {
    let instructions = parse_instructions_with_conditionals(input.trim())
        .context("Failed to parse instructions with conditionals from input")?;

    let mut enabled = true; // mul instructions are enabled at the beginning
    let mut total = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Mul(x, y) => {
                if enabled {
                    total += x * y;
                }
            }
            Instruction::Do => {
                enabled = true;
            }
            Instruction::Dont => {
                enabled = false;
            }
        }
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test examples from the problem statement
    const PART1_EXAMPLE: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const PART2_EXAMPLE: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parse_mul_instructions() {
        let instructions = parse_mul_instructions(PART1_EXAMPLE).unwrap();

        // Only valid mul instructions should be parsed
        assert_eq!(instructions.len(), 4);
        assert_eq!(instructions[0], (2, 4));
        assert_eq!(instructions[1], (5, 5));
        assert_eq!(instructions[2], (11, 8));
        assert_eq!(instructions[3], (8, 5));
    }

    #[test]
    fn test_parse_instructions_with_conditionals() {
        let instructions = parse_instructions_with_conditionals(PART2_EXAMPLE).unwrap();

        // Expected: mul(2,4), don't(), mul(5,5), mul(11,8), do() [from "undo()"], mul(8,5)
        assert_eq!(instructions.len(), 6);
        assert_eq!(instructions[0], Instruction::Mul(2, 4));
        assert_eq!(instructions[1], Instruction::Dont);
        assert_eq!(instructions[2], Instruction::Mul(5, 5));
        assert_eq!(instructions[3], Instruction::Mul(11, 8));
        assert_eq!(instructions[4], Instruction::Do); // This is "do()" from "undo()"
        assert_eq!(instructions[5], Instruction::Mul(8, 5));
    }

    #[test]
    fn test_solve_part1_example() {
        let result = solve_part1(PART1_EXAMPLE).unwrap();

        // 2*4 + 5*5 + 11*8 + 8*5 = 8 + 25 + 88 + 40 = 161
        assert_eq!(result, "161");
    }

    #[test]
    fn test_solve_part2_example() {
        let result = solve_part2(PART2_EXAMPLE).unwrap();

        // 2*4 + 8*5 = 8 + 40 = 48 (mul(5,5) and mul(11,8) are disabled)
        assert_eq!(result, "48");
    }

    #[test]
    fn test_solve_part2_with_do_instruction() {
        let input = "mul(2,4)don't()mul(5,5)do()mul(8,5)";
        let result = solve_part2(input).unwrap();

        // 2*4 + 8*5 = 8 + 40 = 48 (mul(5,5) disabled by don't(), mul(8,5) re-enabled by do())
        assert_eq!(result, "48");
    }

    #[test]
    fn test_invalid_mul_instructions() {
        let input = "mul(4* mul(6,9! ?(12,34) mul ( 2 , 4 )";
        let instructions = parse_mul_instructions(input).unwrap();

        // None of these should be valid
        assert_eq!(instructions.len(), 0);
    }

    #[test]
    fn test_large_numbers() {
        let input = "mul(123,456)";
        let instructions = parse_mul_instructions(input).unwrap();

        assert_eq!(instructions.len(), 1);
        assert_eq!(instructions[0], (123, 456));

        let result = solve_part1(input).unwrap();
        assert_eq!(result, "56088"); // 123 * 456
    }

    #[test]
    fn test_four_digit_numbers_ignored() {
        let input = "mul(1234,5678) mul(12,34)";
        let instructions = parse_mul_instructions(input).unwrap();

        // Only 1-3 digit numbers are valid
        assert_eq!(instructions.len(), 1);
        assert_eq!(instructions[0], (12, 34));
    }

    #[test]
    fn test_consecutive_conditionals() {
        let input = "mul(2,3)don't()don't()do()don't()mul(4,5)do()mul(6,7)";
        let result = solve_part2(input).unwrap();

        // mul(2,3)=6 (enabled), mul(4,5) disabled, mul(6,7)=42 (re-enabled)
        // Total: 6 + 42 = 48
        assert_eq!(result, "48");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(solve_part1("").unwrap(), "0");
        assert_eq!(solve_part2("").unwrap(), "0");
    }

    #[test]
    fn test_no_valid_instructions() {
        let input = "invalid mul invalid don't invalid do invalid";
        assert_eq!(solve_part1(input).unwrap(), "0");
        assert_eq!(solve_part2(input).unwrap(), "0");
    }
}
