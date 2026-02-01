use anyhow::{Context, Result};
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Machine {
    target: Vec<bool>, // Target state for each light (true = on, false = off)
    buttons: Vec<HashSet<usize>>, // Each button's affected lights
}

impl Machine {
    fn num_lights(&self) -> usize {
        self.target.len()
    }

    fn num_buttons(&self) -> usize {
        self.buttons.len()
    }
}

/// Parse a single machine line
fn parse_machine(line: &str) -> Result<Machine> {
    // Example: [##...#] (1,3,4,5) (2,3,5) (0,2,3) {24,27,40,30,25,17}

    // Extract target state from [brackets]
    let bracket_start = line.find('[').context("Missing opening bracket")?;
    let bracket_end = line.find(']').context("Missing closing bracket")?;
    let target_str = &line[bracket_start + 1..bracket_end];

    let target: Vec<bool> = target_str.chars().map(|c| c == '#').collect();

    // Extract buttons from (parentheses)
    let mut buttons = Vec::new();
    let after_bracket = &line[bracket_end + 1..];

    // Find all (X,Y,Z) patterns
    let mut start = 0;
    while let Some(paren_start) = after_bracket[start..].find('(') {
        let paren_start = start + paren_start;
        if let Some(paren_end) = after_bracket[paren_start..].find(')') {
            let paren_end = paren_start + paren_end;
            let button_str = &after_bracket[paren_start + 1..paren_end];

            // Parse comma-separated numbers
            let affected_lights: HashSet<usize> = button_str
                .split(',')
                .filter_map(|s| s.trim().parse::<usize>().ok())
                .collect();

            buttons.push(affected_lights);
            start = paren_end + 1;
        } else {
            break;
        }
    }

    Ok(Machine { target, buttons })
}

/// Solve a single machine using Gaussian elimination over GF(2)
/// Returns the minimum number of button presses needed
fn solve_machine(machine: &Machine) -> usize {
    let num_lights = machine.num_lights();
    let num_buttons = machine.num_buttons();

    // Build augmented matrix [A | b] where:
    // - A[i][j] = 1 if button j affects light i, 0 otherwise
    // - b[i] = 1 if light i should be ON, 0 if OFF

    let mut matrix: Vec<Vec<u8>> = vec![vec![0; num_buttons + 1]; num_lights];

    // Fill in matrix
    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &light_idx in button {
            if light_idx < num_lights {
                matrix[light_idx][button_idx] = 1;
            }
        }
    }

    // Fill in target column (augmented part)
    for (light_idx, &should_be_on) in machine.target.iter().enumerate() {
        matrix[light_idx][num_buttons] = if should_be_on { 1 } else { 0 };
    }

    // Gaussian elimination over GF(2) to get reduced row echelon form
    let mut pivot_cols = Vec::new();
    let mut row = 0;

    for col in 0..num_buttons {
        // Find pivot (any row with 1 in this column)
        if let Some(pivot_row) = (row..num_lights).find(|&r| matrix[r][col] == 1) {
            // Swap rows
            matrix.swap(row, pivot_row);
            pivot_cols.push((row, col));

            // Eliminate all other 1s in this column (XOR in GF(2))
            let pivot_row_values = matrix[row].clone();
            for (other_row_idx, other_row) in matrix.iter_mut().enumerate() {
                if other_row_idx != row && other_row[col] == 1 {
                    for (c, &pivot_value) in
                        pivot_row_values.iter().enumerate().take(num_buttons + 1)
                    {
                        other_row[c] ^= pivot_value;
                    }
                }
            }

            row += 1;
        }
    }

    // Identify free variables (non-pivot columns)
    let pivot_col_set: HashSet<usize> = pivot_cols.iter().map(|(_, col)| *col).collect();
    let free_vars: Vec<usize> = (0..num_buttons)
        .filter(|col| !pivot_col_set.contains(col))
        .collect();

    // If no free variables, we have a unique solution
    if free_vars.is_empty() {
        let mut solution = vec![0u8; num_buttons];
        for (row, col) in pivot_cols {
            solution[col] = matrix[row][num_buttons];
        }
        return solution.iter().map(|&x| x as usize).sum();
    }

    // Try all combinations of free variables to find minimum
    let num_free = free_vars.len();
    let mut min_presses = usize::MAX;

    for free_mask in 0..(1 << num_free) {
        let mut solution = vec![0u8; num_buttons];

        // Set free variables according to mask
        for (i, &free_col) in free_vars.iter().enumerate() {
            solution[free_col] = ((free_mask >> i) & 1) as u8;
        }

        // Solve for pivot variables using back-substitution
        for (row, col) in pivot_cols.iter().rev() {
            let mut sum = matrix[*row][num_buttons];
            for (other_col, &val) in solution.iter().enumerate() {
                if other_col != *col && val == 1 {
                    sum ^= matrix[*row][other_col];
                }
            }
            solution[*col] = sum;
        }

        let presses: usize = solution.iter().map(|&x| x as usize).sum();
        min_presses = min_presses.min(presses);
    }

    min_presses
}

pub fn solve_part1(input: &str) -> Result<String> {
    let mut total_presses = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let machine = parse_machine(line)?;
        let presses = solve_machine(&machine);
        total_presses += presses;
    }

    Ok(total_presses.to_string())
}

/// Parse joltage requirements from the {curly braces}
fn parse_joltages(line: &str) -> Result<Vec<i64>> {
    let curly_start = line.find('{').context("Missing opening curly brace")?;
    let curly_end = line.find('}').context("Missing closing curly brace")?;
    let joltage_str = &line[curly_start + 1..curly_end];

    joltage_str
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<i64>()
                .with_context(|| format!("Failed to parse joltage: {}", s))
        })
        .collect()
}

/// Solve Part 2: Find minimum button presses to reach target joltage values
/// Uses Integer Linear Programming via good_lp crate
/// Based on Tom Wilkinson's solution approach
fn solve_machine_joltage(machine: &Machine, targets: &[i64]) -> usize {
    use good_lp::*;

    let num_buttons = machine.num_buttons();

    if num_buttons == 0 || targets.is_empty() {
        return 0;
    }

    // Convert HashSet buttons to Vec for easier indexing
    let raw_buttons: Vec<Vec<usize>> = machine
        .buttons
        .iter()
        .map(|set| set.iter().copied().collect())
        .collect();

    let mut vars = ProblemVariables::new();

    // Create a variable for each button (number of times pressed)
    // Constrained to be non-negative integers
    let mut button_presses = Vec::new();
    for _ in 0..num_buttons {
        let variable = vars.add(variable().min(0).integer());
        button_presses.push(variable);
    }

    // Objective: minimize sum of all button presses
    let mut problem = vars
        .minimise(button_presses.iter().sum::<Expression>())
        .using(default_solver);

    // Create expressions for each counter's value
    let mut expressions = vec![Expression::with_capacity(num_buttons); targets.len()];

    for (button_idx, button_affects) in raw_buttons.iter().enumerate() {
        for &counter_idx in button_affects {
            // Each button press increments the counters it affects
            expressions[counter_idx] += button_presses[button_idx];
        }
    }

    // Add constraints: each counter must equal its target value
    for (expression, &target) in expressions.into_iter().zip(targets.iter()) {
        problem.add_constraint(expression.eq(target as f64));
    }

    // Solve the ILP problem
    match problem.solve() {
        Ok(solution) => {
            let values: Vec<f64> = button_presses.iter().map(|&v| solution.value(v)).collect();
            let total: f64 = values.iter().sum();
            let rounded_total = total.round() as usize;

            // Debug: check if solution is truly integer
            let is_integer = values.iter().all(|&v| (v - v.round()).abs() < 1e-6);
            if !is_integer {
                eprintln!(
                    "WARNING: Non-integer solution detected! Values: {:?}",
                    values
                );
                eprintln!("Total: {} (rounded to {})", total, rounded_total);
            }

            rounded_total
        }
        Err(_) => 0, // No solution found
    }
}

pub fn solve_part2(input: &str) -> Result<String> {
    let mut total_presses = 0;
    let mut machine_num = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        machine_num += 1;
        let machine = parse_machine(line)?;
        let joltages = parse_joltages(line)?;
        eprintln!(
            "Processing machine {}: {} counters, {} buttons, targets: {:?}",
            machine_num,
            joltages.len(),
            machine.num_buttons(),
            joltages
        );
        let presses = solve_machine_joltage(&machine, &joltages);
        eprintln!(
            "  -> {} presses (total so far: {})",
            presses,
            total_presses + presses
        );
        total_presses += presses;
    }

    Ok(total_presses.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_parse_machine() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_machine(line).unwrap();

        assert_eq!(machine.num_lights(), 4);
        assert_eq!(machine.target, vec![false, true, true, false]);
        assert_eq!(machine.num_buttons(), 6);

        // Check first button affects light 3
        assert!(machine.buttons[0].contains(&3));
        assert_eq!(machine.buttons[0].len(), 1);

        // Check second button affects lights 1 and 3
        assert!(machine.buttons[1].contains(&1));
        assert!(machine.buttons[1].contains(&3));
        assert_eq!(machine.buttons[1].len(), 2);
    }

    #[test]
    fn test_solve_first_machine() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_machine(line).unwrap();
        let presses = solve_machine(&machine);

        // Example says minimum is 2 presses
        assert_eq!(presses, 2);
    }

    #[test]
    fn test_solve_second_machine() {
        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = parse_machine(line).unwrap();
        let presses = solve_machine(&machine);

        // Example says minimum is 3 presses
        assert_eq!(presses, 3);
    }

    #[test]
    fn test_solve_third_machine() {
        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = parse_machine(line).unwrap();
        let presses = solve_machine(&machine);

        // Example says minimum is 2 presses
        assert_eq!(presses, 2);
    }

    #[test]
    fn test_example_total() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "7"); // 2 + 3 + 2 = 7
    }

    #[test]
    fn test_parse_joltages() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let joltages = parse_joltages(line).unwrap();
        assert_eq!(joltages, vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_part2_first_machine() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = parse_machine(line).unwrap();
        let joltages = parse_joltages(line).unwrap();
        let presses = solve_machine_joltage(&machine, &joltages);

        // Example says minimum is 10 presses
        assert_eq!(presses, 10);
    }

    #[test]
    fn test_part2_second_machine() {
        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = parse_machine(line).unwrap();
        let joltages = parse_joltages(line).unwrap();
        let presses = solve_machine_joltage(&machine, &joltages);

        // Example says minimum is 12 presses
        assert_eq!(presses, 12);
    }

    #[test]
    fn test_part2_third_machine() {
        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = parse_machine(line).unwrap();
        let joltages = parse_joltages(line).unwrap();
        let presses = solve_machine_joltage(&machine, &joltages);

        // Example says minimum is 11 presses
        assert_eq!(presses, 11);
    }

    #[test]
    fn test_part2_example_total() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result, "33"); // 10 + 12 + 11 = 33
    }
}
