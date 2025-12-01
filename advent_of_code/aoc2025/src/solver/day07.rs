use anyhow::{anyhow, Result};

/// Represents a calibration equation with a test value and list of numbers
#[derive(Debug, Clone, PartialEq)]
pub struct Equation {
    pub test_value: u64,
    pub numbers: Vec<u64>,
}

/// Available operators for calibration equations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Multiply,
    Concatenate, // Part 2 only
}

impl Operator {
    /// Apply the operator to two values
    pub fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
            Operator::Concatenate => {
                // Convert to strings, concatenate, then parse back
                let combined = format!("{}{}", left, right);
                combined.parse().unwrap_or(0) // In practice, this should not fail for valid inputs
            }
        }
    }
}

/// Parse input text into a vector of Equation structs
pub fn parse_equations(input: &str) -> Result<Vec<Equation>> {
    let mut equations = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid equation format: {}", line));
        }

        let test_value = parts[0]
            .trim()
            .parse::<u64>()
            .map_err(|_| anyhow!("Invalid test value: {}", parts[0]))?;

        let numbers = parts[1]
            .split_whitespace()
            .map(|n| n.parse::<u64>())
            .collect::<std::result::Result<Vec<u64>, _>>()
            .map_err(|_| anyhow!("Invalid numbers in equation: {}", parts[1]))?;

        if numbers.is_empty() {
            return Err(anyhow!("Equation must have at least one number: {}", line));
        }

        equations.push(Equation {
            test_value,
            numbers,
        });
    }

    Ok(equations)
}

/// Evaluate an expression with given numbers and operators (left-to-right)
pub fn evaluate_expression(numbers: &[u64], operators: &[Operator]) -> u64 {
    if numbers.is_empty() {
        return 0;
    }

    if numbers.len() == 1 {
        return numbers[0];
    }

    if operators.len() != numbers.len() - 1 {
        // Invalid operator count, return 0 as error indicator
        return 0;
    }

    let mut result = numbers[0];

    for (i, &op) in operators.iter().enumerate() {
        result = op.apply(result, numbers[i + 1]);
    }

    result
}

/// Generate all possible operator combinations for N positions
pub fn generate_operator_combinations(
    positions: usize,
    include_concat: bool,
) -> Vec<Vec<Operator>> {
    if positions == 0 {
        return vec![vec![]];
    }

    let operators = if include_concat {
        vec![Operator::Add, Operator::Multiply, Operator::Concatenate]
    } else {
        vec![Operator::Add, Operator::Multiply]
    };

    let mut combinations = Vec::new();
    let total_combinations = operators.len().pow(positions as u32);

    for i in 0..total_combinations {
        let mut combination = Vec::with_capacity(positions);
        let mut num = i;

        for _ in 0..positions {
            combination.push(operators[num % operators.len()]);
            num /= operators.len();
        }

        combinations.push(combination);
    }

    combinations
}

/// Check if an equation can be solved with any operator combination
pub fn can_equation_be_solved(equation: &Equation, include_concat: bool) -> bool {
    if equation.numbers.len() == 1 {
        return equation.numbers[0] == equation.test_value;
    }

    let positions = equation.numbers.len() - 1;
    let combinations = generate_operator_combinations(positions, include_concat);

    for operators in combinations {
        let result = evaluate_expression(&equation.numbers, &operators);
        if result == equation.test_value {
            return true;
        }
    }

    false
}

/// Solve Part 1: Sum test values of equations solvable with + and * operators
pub fn solve_part1(input: &str) -> Result<String> {
    let equations = parse_equations(input)?;

    let sum: u64 = equations
        .iter()
        .filter(|eq| can_equation_be_solved(eq, false))
        .map(|eq| eq.test_value)
        .sum();

    Ok(sum.to_string())
}

/// Solve Part 2: Sum test values of equations solvable with +, *, and || operators
pub fn solve_part2(input: &str) -> Result<String> {
    let equations = parse_equations(input)?;

    let sum: u64 = equations
        .iter()
        .filter(|eq| can_equation_be_solved(eq, true))
        .map(|eq| eq.test_value)
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Sample input for testing
    const SAMPLE_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    // ===== PHASE 1: INPUT PARSING TESTS =====

    #[test]
    fn test_parse_single_equation() {
        let input = "190: 10 19";
        let equations = parse_equations(input).unwrap();

        assert_eq!(equations.len(), 1);
        assert_eq!(equations[0].test_value, 190);
        assert_eq!(equations[0].numbers, vec![10, 19]);
    }

    #[test]
    fn test_parse_multiple_equations() {
        let input = "190: 10 19\n3267: 81 40 27";
        let equations = parse_equations(input).unwrap();

        assert_eq!(equations.len(), 2);
        assert_eq!(
            equations[0],
            Equation {
                test_value: 190,
                numbers: vec![10, 19]
            }
        );
        assert_eq!(
            equations[1],
            Equation {
                test_value: 3267,
                numbers: vec![81, 40, 27]
            }
        );
    }

    #[test]
    fn test_parse_sample_input() {
        let equations = parse_equations(SAMPLE_INPUT).unwrap();
        assert_eq!(equations.len(), 9);

        // Verify first and last equations
        assert_eq!(
            equations[0],
            Equation {
                test_value: 190,
                numbers: vec![10, 19]
            }
        );
        assert_eq!(
            equations[8],
            Equation {
                test_value: 292,
                numbers: vec![11, 6, 16, 20]
            }
        );
    }

    #[test]
    fn test_parse_edge_cases() {
        // Single number
        let equations = parse_equations("42: 42").unwrap();
        assert_eq!(
            equations[0],
            Equation {
                test_value: 42,
                numbers: vec![42]
            }
        );

        // Empty input
        let equations = parse_equations("").unwrap();
        assert_eq!(equations.len(), 0);

        // Whitespace handling
        let equations = parse_equations("  100:  10   20  ").unwrap();
        assert_eq!(
            equations[0],
            Equation {
                test_value: 100,
                numbers: vec![10, 20]
            }
        );
    }

    #[test]
    fn test_parse_invalid_input() {
        // Missing colon
        assert!(parse_equations("190 10 19").is_err());

        // Invalid test value
        assert!(parse_equations("abc: 10 19").is_err());

        // Invalid numbers
        assert!(parse_equations("190: 10 abc").is_err());

        // No numbers after colon
        assert!(parse_equations("190:").is_err());
    }

    // ===== PHASE 2: EXPRESSION EVALUATION TESTS =====

    #[test]
    fn test_evaluate_addition() {
        let numbers = vec![10, 19];
        let operators = vec![Operator::Add];
        assert_eq!(evaluate_expression(&numbers, &operators), 29);
    }

    #[test]
    fn test_evaluate_multiplication() {
        let numbers = vec![10, 19];
        let operators = vec![Operator::Multiply];
        assert_eq!(evaluate_expression(&numbers, &operators), 190);
    }

    #[test]
    fn test_evaluate_left_to_right() {
        // 81 + 40 * 27 = (81 + 40) * 27 = 121 * 27 = 3267
        let numbers = vec![81, 40, 27];
        let operators = vec![Operator::Add, Operator::Multiply];
        assert_eq!(evaluate_expression(&numbers, &operators), 3267);

        // Different order: 81 * 40 + 27 = (81 * 40) + 27 = 3240 + 27 = 3267
        let operators2 = vec![Operator::Multiply, Operator::Add];
        assert_eq!(evaluate_expression(&numbers, &operators2), 3267);
    }

    #[test]
    fn test_evaluate_concatenation() {
        let numbers = vec![15, 6];
        let operators = vec![Operator::Concatenate];
        assert_eq!(evaluate_expression(&numbers, &operators), 156);

        // Multi-digit concatenation
        let numbers2 = vec![12, 345];
        let operators2 = vec![Operator::Concatenate];
        assert_eq!(evaluate_expression(&numbers2, &operators2), 12345);
    }

    #[test]
    fn test_evaluate_complex_expression() {
        // 11 + 6 * 16 + 20 = ((11 + 6) * 16) + 20 = (17 * 16) + 20 = 272 + 20 = 292
        let numbers = vec![11, 6, 16, 20];
        let operators = vec![Operator::Add, Operator::Multiply, Operator::Add];
        assert_eq!(evaluate_expression(&numbers, &operators), 292);
    }

    #[test]
    fn test_evaluate_edge_cases() {
        // Single number
        assert_eq!(evaluate_expression(&[42], &[]), 42);

        // Empty numbers
        assert_eq!(evaluate_expression(&[], &[]), 0);

        // Mismatched operators count
        assert_eq!(evaluate_expression(&[1, 2], &[]), 0);
        assert_eq!(
            evaluate_expression(&[1, 2], &[Operator::Add, Operator::Multiply]),
            0
        );
    }

    #[test]
    fn test_operator_apply() {
        assert_eq!(Operator::Add.apply(10, 5), 15);
        assert_eq!(Operator::Multiply.apply(10, 5), 50);
        assert_eq!(Operator::Concatenate.apply(10, 5), 105);
        assert_eq!(Operator::Concatenate.apply(123, 456), 123456);
    }

    // ===== PHASE 3: COMBINATION GENERATION TESTS =====

    #[test]
    fn test_generate_combinations_two_ops() {
        let combinations = generate_operator_combinations(1, false);
        assert_eq!(combinations.len(), 2);
        assert!(combinations.contains(&vec![Operator::Add]));
        assert!(combinations.contains(&vec![Operator::Multiply]));
    }

    #[test]
    fn test_generate_combinations_three_ops() {
        let combinations = generate_operator_combinations(2, false);
        assert_eq!(combinations.len(), 4);

        let expected = vec![
            vec![Operator::Add, Operator::Add],
            vec![Operator::Multiply, Operator::Add],
            vec![Operator::Add, Operator::Multiply],
            vec![Operator::Multiply, Operator::Multiply],
        ];

        for combo in expected {
            assert!(
                combinations.contains(&combo),
                "Missing combination: {:?}",
                combo
            );
        }
    }

    #[test]
    fn test_generate_combinations_part2() {
        let combinations = generate_operator_combinations(1, true);
        assert_eq!(combinations.len(), 3);
        assert!(combinations.contains(&vec![Operator::Add]));
        assert!(combinations.contains(&vec![Operator::Multiply]));
        assert!(combinations.contains(&vec![Operator::Concatenate]));
    }

    #[test]
    fn test_combination_count() {
        // Part 1: 2^N combinations
        assert_eq!(generate_operator_combinations(0, false).len(), 1); // 2^0 = 1
        assert_eq!(generate_operator_combinations(1, false).len(), 2); // 2^1 = 2
        assert_eq!(generate_operator_combinations(2, false).len(), 4); // 2^2 = 4
        assert_eq!(generate_operator_combinations(3, false).len(), 8); // 2^3 = 8

        // Part 2: 3^N combinations
        assert_eq!(generate_operator_combinations(0, true).len(), 1); // 3^0 = 1
        assert_eq!(generate_operator_combinations(1, true).len(), 3); // 3^1 = 3
        assert_eq!(generate_operator_combinations(2, true).len(), 9); // 3^2 = 9
        assert_eq!(generate_operator_combinations(3, true).len(), 27); // 3^3 = 27
    }

    #[test]
    fn test_empty_combinations() {
        let combinations = generate_operator_combinations(0, false);
        assert_eq!(combinations.len(), 1);
        assert_eq!(combinations[0], vec![]);
    }

    // ===== PHASE 4: EQUATION VALIDATION TESTS =====

    #[test]
    fn test_validate_solvable_equation() {
        let equation = Equation {
            test_value: 190,
            numbers: vec![10, 19],
        };
        assert!(can_equation_be_solved(&equation, false));
        // 10 * 19 = 190 ✓
    }

    #[test]
    fn test_validate_unsolvable_equation() {
        let equation = Equation {
            test_value: 83,
            numbers: vec![17, 5],
        };
        assert!(!can_equation_be_solved(&equation, false));
        // 17 + 5 = 22, 17 * 5 = 85, neither matches 83
    }

    #[test]
    fn test_validate_multiple_solutions() {
        let equation = Equation {
            test_value: 3267,
            numbers: vec![81, 40, 27],
        };
        assert!(can_equation_be_solved(&equation, false));
        // Both 81 + 40 * 27 = 3267 and 81 * 40 + 27 = 3267 work
    }

    #[test]
    fn test_validate_with_concatenation() {
        // Test equations that can only be solved with concatenation
        let equation1 = Equation {
            test_value: 156,
            numbers: vec![15, 6],
        };
        assert!(!can_equation_be_solved(&equation1, false)); // Can't solve without concat
        assert!(can_equation_be_solved(&equation1, true)); // 15 || 6 = 156

        let equation2 = Equation {
            test_value: 7290,
            numbers: vec![6, 8, 6, 15],
        };
        assert!(!can_equation_be_solved(&equation2, false)); // Can't solve without concat
        assert!(can_equation_be_solved(&equation2, true)); // 6 * 8 || 6 * 15 = 48 || 6 * 15 = 486 * 15 = 7290

        let equation3 = Equation {
            test_value: 192,
            numbers: vec![17, 8, 14],
        };
        assert!(!can_equation_be_solved(&equation3, false)); // Can't solve without concat
        assert!(can_equation_be_solved(&equation3, true)); // 17 || 8 + 14 = 178 + 14 = 192
    }

    #[test]
    fn test_validate_single_number() {
        let equation1 = Equation {
            test_value: 42,
            numbers: vec![42],
        };
        assert!(can_equation_be_solved(&equation1, false)); // Exact match

        let equation2 = Equation {
            test_value: 42,
            numbers: vec![43],
        };
        assert!(!can_equation_be_solved(&equation2, false)); // No match
    }

    #[test]
    fn test_validate_sample_equations() {
        let equations = parse_equations(SAMPLE_INPUT).unwrap();

        // Part 1: Only 3 equations should be solvable
        let solvable_part1: Vec<_> = equations
            .iter()
            .filter(|eq| can_equation_be_solved(eq, false))
            .collect();
        assert_eq!(solvable_part1.len(), 3);

        // Should be equations with test values: 190, 3267, 292
        let solvable_values: Vec<u64> = solvable_part1.iter().map(|eq| eq.test_value).collect();
        assert!(solvable_values.contains(&190));
        assert!(solvable_values.contains(&3267));
        assert!(solvable_values.contains(&292));

        // Part 2: 6 equations should be solvable (3 more with concatenation)
        let solvable_part2: Vec<_> = equations
            .iter()
            .filter(|eq| can_equation_be_solved(eq, true))
            .collect();
        assert_eq!(solvable_part2.len(), 6);
    }

    // ===== PHASE 5: SOLUTION INTEGRATION TESTS =====

    #[test]
    fn test_calculate_sample_part1() {
        let result = solve_part1(SAMPLE_INPUT).unwrap();
        assert_eq!(result, "3749");
        // Sum of 190 + 3267 + 292 = 3749
    }

    #[test]
    fn test_calculate_sample_part2() {
        let result = solve_part2(SAMPLE_INPUT).unwrap();
        assert_eq!(result, "11387");
        // Sum of 190 + 3267 + 292 + 156 + 7290 + 192 = 11387
    }

    #[test]
    fn test_calculate_empty_input() {
        assert_eq!(solve_part1("").unwrap(), "0");
        assert_eq!(solve_part2("").unwrap(), "0");
    }

    #[test]
    fn test_calculate_no_valid_equations() {
        let input = "100: 1 2\n200: 3 4"; // No valid combinations
        assert_eq!(solve_part1(input).unwrap(), "0");
        assert_eq!(solve_part2(input).unwrap(), "0");
    }

    #[test]
    fn test_calculate_single_valid_equation() {
        let input = "30: 10 20"; // 10 + 20 = 30
        assert_eq!(solve_part1(input).unwrap(), "30");
        assert_eq!(solve_part2(input).unwrap(), "30");
    }

    #[test]
    fn test_invalid_input_handling() {
        assert!(solve_part1("invalid input").is_err());
        assert!(solve_part2("invalid input").is_err());
    }

    // ===== ADDITIONAL COMPREHENSIVE TESTS =====

    #[test]
    fn test_complex_concatenation_scenarios() {
        // Test complex concatenation with multiple digits
        let equation = Equation {
            test_value: 123456789,
            numbers: vec![123, 456, 789],
        };
        // 123 || 456 || 789 = 123456789
        assert!(can_equation_be_solved(&equation, true));

        // Mixed operations with concatenation
        let _equation2 = Equation {
            test_value: 2468,
            numbers: vec![2, 4, 6, 8],
        };
        // 2 * 4 || 6 + 8 = 8 || 6 + 8 = 86 + 8 = 94 (not 2468)
        // 2 || 4 * 6 + 8 = 24 * 6 + 8 = 144 + 8 = 152 (not 2468)
        // 2 + 4 || 6 * 8 = 6 || 6 * 8 = 6 || 48 = 648 (not 2468)
        // Let's check if there's a valid combination...
        // This might not have a solution, but we're testing the mechanism
    }

    #[test]
    fn test_large_numbers() {
        // Test with larger numbers to ensure no overflow issues
        let equation = Equation {
            test_value: 1000000,
            numbers: vec![1000, 1000],
        };
        assert!(can_equation_be_solved(&equation, false)); // 1000 * 1000 = 1000000
    }

    #[test]
    fn test_performance_with_many_operators() {
        // Test with equations requiring many operators (but not too many for tests)
        let _equation = Equation {
            test_value: 16,
            numbers: vec![2, 2, 2, 2, 2],
        };
        // This has 4 operator positions = 2^4 = 16 combinations for part 1
        // Should find: 2 * 2 * 2 * 2 / 2... wait, no division. Let's use addition:
        // 2 + 2 + 2 + 2 + 2 = 10 (not 16)
        // 2 * 2 + 2 + 2 + 2 = 4 + 6 = 10 (not 16)
        // 2 * 2 * 2 + 2 + 2 = 8 + 4 = 12 (not 16)
        // 2 * 2 * 2 * 2 + 2 = 16 + 2 = 18 (not 16)
        // Let's try: 2 + 2 * 2 * 2 + 2 = (2 + 2) * 2 * 2 + 2 = 4 * 2 * 2 + 2 = 16 + 2 = 18
        // Hmm, let's use a number that works: 2 * 2 * 2 * 2 = 16
        let equation2 = Equation {
            test_value: 16,
            numbers: vec![2, 2, 2, 2],
        };
        assert!(can_equation_be_solved(&equation2, false)); // 2 * 2 * 2 * 2 = 16
    }
}
