use anyhow::Result;

/// Represents a single claw machine configuration
#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: (i64, i64), // (dx, dy) movement for button A
    button_b: (i64, i64), // (dx, dy) movement for button B
    prize: (i64, i64),    // (x, y) prize location
}

/// Parse a button line like "Button A: X+94, Y+34" -> (94, 34)
fn parse_button_line(line: &str) -> Result<(i64, i64)> {
    // Find the X and Y values after the colon
    let parts: Vec<&str> = line.split(": ").collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid button line format: {}", line);
    }

    let coords: Vec<&str> = parts[1].split(", ").collect();
    if coords.len() != 2 {
        anyhow::bail!("Invalid button coordinates: {}", parts[1]);
    }

    // Parse X+94 -> 94
    let x = coords[0]
        .trim_start_matches("X+")
        .trim_start_matches("X")
        .parse::<i64>()?;
    // Parse Y+34 -> 34
    let y = coords[1]
        .trim_start_matches("Y+")
        .trim_start_matches("Y")
        .parse::<i64>()?;

    Ok((x, y))
}

/// Parse a prize line like "Prize: X=8400, Y=5400" -> (8400, 5400)
fn parse_prize_line(line: &str) -> Result<(i64, i64)> {
    let parts: Vec<&str> = line.split(": ").collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid prize line format: {}", line);
    }

    let coords: Vec<&str> = parts[1].split(", ").collect();
    if coords.len() != 2 {
        anyhow::bail!("Invalid prize coordinates: {}", parts[1]);
    }

    // Parse X=8400 -> 8400
    let x = coords[0].trim_start_matches("X=").parse::<i64>()?;
    // Parse Y=5400 -> 5400
    let y = coords[1].trim_start_matches("Y=").parse::<i64>()?;

    Ok((x, y))
}

/// Parse all claw machines from input
fn parse_input(input: &str) -> Result<Vec<ClawMachine>> {
    // Collect lines and group by blank line separators
    let mut machines = Vec::new();
    let mut current_block = Vec::new();
    
    for line in input.lines() {
        if line.trim().is_empty() {
            if !current_block.is_empty() {
                machines.push(parse_machine_from_lines(&current_block)?);
                current_block.clear();
            }
        } else {
            current_block.push(line);
        }
    }
    
    // Don't forget the last block
    if !current_block.is_empty() {
        machines.push(parse_machine_from_lines(&current_block)?);
    }
    
    Ok(machines)
}

/// Parse a machine from collected lines
fn parse_machine_from_lines(lines: &[&str]) -> Result<ClawMachine> {
    if lines.len() < 3 {
        anyhow::bail!("Expected 3 lines for a machine, got {}", lines.len());
    }

    Ok(ClawMachine {
        button_a: parse_button_line(lines[0])?,
        button_b: parse_button_line(lines[1])?,
        prize: parse_prize_line(lines[2])?,
    })
}

/// Solve the linear system for a single claw machine using Cramer's rule.
/// 
/// We need to solve:
///   a * Ax + b * Bx = Px
///   a * Ay + b * By = Py
/// 
/// Using Cramer's rule:
///   det = Ax * By - Bx * Ay
///   a = (Px * By - Bx * Py) / det
///   b = (Ax * Py - Px * Ay) / det
/// 
/// Returns Some((a, b)) if valid integer solution exists, None otherwise.
fn solve_machine(machine: &ClawMachine) -> Option<(i64, i64)> {
    let (ax, ay) = machine.button_a;
    let (bx, by) = machine.button_b;
    let (px, py) = machine.prize;

    // Calculate determinant
    let det = ax * by - bx * ay;
    
    if det == 0 {
        // Lines are parallel - no unique solution
        // (Could have infinite solutions if collinear with prize, but problem implies unique solutions)
        return None;
    }

    // Calculate numerators for Cramer's rule
    let a_num = px * by - bx * py;
    let b_num = ax * py - px * ay;

    // Check if we get integer solutions
    if a_num % det != 0 || b_num % det != 0 {
        return None;
    }

    let a = a_num / det;
    let b = b_num / det;

    // Verify solution is valid (non-negative button presses)
    if a < 0 || b < 0 {
        return None;
    }

    Some((a, b))
}

/// Calculate the minimum tokens needed for a solution
/// Button A costs 3 tokens, Button B costs 1 token
fn calculate_cost(a: i64, b: i64) -> i64 {
    3 * a + b
}

/// Calculate total tokens for all solvable machines
fn total_tokens(machines: &[ClawMachine]) -> i64 {
    machines
        .iter()
        .filter_map(solve_machine)
        .map(|(a, b)| calculate_cost(a, b))
        .sum()
}

pub fn solve_part1(input: &str) -> Result<String> {
    let machines = parse_input(input)?;
    Ok(total_tokens(&machines).to_string())
}

pub fn solve_part2(input: &str) -> Result<String> {
    let mut machines = parse_input(input)?;
    
    // Add 10000000000000 to each prize coordinate
    const OFFSET: i64 = 10_000_000_000_000;
    for machine in &mut machines {
        machine.prize.0 += OFFSET;
        machine.prize.1 += OFFSET;
    }

    Ok(total_tokens(&machines).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_parse_button_line() {
        assert_eq!(parse_button_line("Button A: X+94, Y+34").unwrap(), (94, 34));
        assert_eq!(parse_button_line("Button B: X+22, Y+67").unwrap(), (22, 67));
    }

    #[test]
    fn test_parse_prize_line() {
        assert_eq!(parse_prize_line("Prize: X=8400, Y=5400").unwrap(), (8400, 5400));
    }

    #[test]
    fn test_parse_machine() {
        let lines = vec![
            "Button A: X+94, Y+34",
            "Button B: X+22, Y+67",
            "Prize: X=8400, Y=5400",
        ];
        let machine = parse_machine_from_lines(&lines).unwrap();
        assert_eq!(machine.button_a, (94, 34));
        assert_eq!(machine.button_b, (22, 67));
        assert_eq!(machine.prize, (8400, 5400));
    }

    #[test]
    fn test_parse_input() {
        let machines = parse_input(EXAMPLE).unwrap();
        assert_eq!(machines.len(), 4);
    }

    #[test]
    fn test_solve_machine_1() {
        // Machine 1: solvable with A=80, B=40, cost=280
        let machine = ClawMachine {
            button_a: (94, 34),
            button_b: (22, 67),
            prize: (8400, 5400),
        };
        let solution = solve_machine(&machine);
        assert_eq!(solution, Some((80, 40)));
        assert_eq!(calculate_cost(80, 40), 280);
    }

    #[test]
    fn test_solve_machine_2() {
        // Machine 2: NOT solvable
        let machine = ClawMachine {
            button_a: (26, 66),
            button_b: (67, 21),
            prize: (12748, 12176),
        };
        let solution = solve_machine(&machine);
        assert_eq!(solution, None);
    }

    #[test]
    fn test_solve_machine_3() {
        // Machine 3: solvable with A=38, B=86, cost=200
        let machine = ClawMachine {
            button_a: (17, 86),
            button_b: (84, 37),
            prize: (7870, 6450),
        };
        let solution = solve_machine(&machine);
        assert_eq!(solution, Some((38, 86)));
        assert_eq!(calculate_cost(38, 86), 200);
    }

    #[test]
    fn test_solve_machine_4() {
        // Machine 4: NOT solvable
        let machine = ClawMachine {
            button_a: (69, 23),
            button_b: (27, 71),
            prize: (18641, 10279),
        };
        let solution = solve_machine(&machine);
        assert_eq!(solution, None);
    }

    #[test]
    fn test_part1_example() {
        // Total: 280 + 200 = 480 (machines 1 and 3)
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result, "480");
    }

    #[test]
    fn test_part2_machine_2_becomes_solvable() {
        // Machine 2 becomes solvable in part 2 after adding offset
        const OFFSET: i64 = 10_000_000_000_000;
        let machine = ClawMachine {
            button_a: (26, 66),
            button_b: (67, 21),
            prize: (12748 + OFFSET, 12176 + OFFSET),
        };
        let solution = solve_machine(&machine);
        assert!(solution.is_some());
    }

    #[test]
    fn test_part2_machine_4_becomes_solvable() {
        // Machine 4 becomes solvable in part 2 after adding offset
        const OFFSET: i64 = 10_000_000_000_000;
        let machine = ClawMachine {
            button_a: (69, 23),
            button_b: (27, 71),
            prize: (18641 + OFFSET, 10279 + OFFSET),
        };
        let solution = solve_machine(&machine);
        assert!(solution.is_some());
    }
}
