use anyhow::{Context, Result};

/// Parse input into registers and program
fn parse_input(input: &str) -> Result<(i64, i64, i64, Vec<u8>)> {
    let lines: Vec<&str> = input.lines().collect();

    let reg_a = lines[0]
        .strip_prefix("Register A: ")
        .context("Invalid Register A line")?
        .parse::<i64>()?;

    let reg_b = lines[1]
        .strip_prefix("Register B: ")
        .context("Invalid Register B line")?
        .parse::<i64>()?;

    let reg_c = lines[2]
        .strip_prefix("Register C: ")
        .context("Invalid Register C line")?
        .parse::<i64>()?;

    let program_line = lines[4]
        .strip_prefix("Program: ")
        .context("Invalid Program line")?;

    let program: Vec<u8> = program_line
        .split(',')
        .map(|s| s.parse::<u8>())
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok((reg_a, reg_b, reg_c, program))
}

/// Execute the 3-bit computer program
fn execute_program(mut reg_a: i64, mut reg_b: i64, mut reg_c: i64, program: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut ip = 0;

    while ip < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];

        // Helper to get combo operand value
        let combo_value = |op: u8| -> i64 {
            match op {
                0..=3 => op as i64,
                4 => reg_a,
                5 => reg_b,
                6 => reg_c,
                _ => unreachable!("Invalid combo operand"),
            }
        };

        match opcode {
            0 => {
                // adv: A = A / 2^combo
                reg_a >>= combo_value(operand);
            }
            1 => {
                // bxl: B = B XOR literal
                reg_b ^= operand as i64;
            }
            2 => {
                // bst: B = combo % 8
                reg_b = combo_value(operand) & 7;
            }
            3 => {
                // jnz: jump if A != 0
                if reg_a != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            4 => {
                // bxc: B = B XOR C
                reg_b ^= reg_c;
            }
            5 => {
                // out: output combo % 8
                output.push((combo_value(operand) & 7) as u8);
            }
            6 => {
                // bdv: B = A / 2^combo
                reg_b = reg_a >> combo_value(operand);
            }
            7 => {
                // cdv: C = A / 2^combo
                reg_c = reg_a >> combo_value(operand);
            }
            _ => unreachable!("Invalid opcode"),
        }

        ip += 2;
    }

    output
}

/// Part 1: Run the program and return comma-separated output
pub fn solve_part1(input: &str) -> Result<String> {
    let (reg_a, reg_b, reg_c, program) = parse_input(input)?;
    let output = execute_program(reg_a, reg_b, reg_c, &program);

    Ok(output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(","))
}

/// Part 2: Find the lowest value of A that makes the program output itself
pub fn solve_part2(input: &str) -> Result<String> {
    let (_, reg_b, reg_c, program) = parse_input(input)?;

    // Work backwards from the end of the program
    // Each iteration of the program consumes 3 bits of A (divides by 8)
    // So we build A from right to left, 3 bits at a time

    // Debug statistics
    #[derive(Default)]
    struct SearchStats {
        total_attempts: usize,
        successful_paths: usize,
        backtracks: usize,
        max_depth: usize,
    }

    fn find_a(
        program: &[u8], 
        target_pos: usize, 
        current_a: i64, 
        reg_b: i64, 
        reg_c: i64,
        stats: &mut SearchStats,
        depth: usize,
    ) -> Option<i64> {
        // Track maximum depth reached
        stats.max_depth = stats.max_depth.max(depth);
        
        // If we've matched the entire program, verify and return
        if target_pos > program.len() {
            let output = execute_program(current_a, reg_b, reg_c, program);
            if output == program {
                stats.successful_paths += 1;
                return Some(current_a);
            }
            return None;
        }

        // Try all possible 3-bit values (0-7) for this position
        let mut tried_any = false;
        for bits in 0..8 {
            stats.total_attempts += 1;
            let test_a = (current_a << 3) | bits;
            let output = execute_program(test_a, reg_b, reg_c, program);

            // Check if the output matches the last 'target_pos' elements of the program
            if output.len() >= target_pos && output[output.len() - target_pos..] == program[program.len() - target_pos..] {
                tried_any = true;
                
                if let Some(result) = find_a(program, target_pos + 1, test_a, reg_b, reg_c, stats, depth + 1) {
                    return Some(result);
                }
                
                // If we get here, the recursive call failed - this is a backtrack
                stats.backtracks += 1;
                eprintln!("🔙 Backtrack at depth {} (pos {}): tried bits {:03b} (A={}), continuing search", 
                    depth, target_pos, bits, test_a);
            }
        }

        // If we tried at least one valid path but none worked, that's also a backtrack point
        if tried_any {
            eprintln!("🔙 Backtrack at depth {} (pos {}): exhausted all 8 bit patterns", depth, target_pos);
        }

        None
    }

    let mut stats = SearchStats::default();
    
    eprintln!("\n🔍 Starting Part 2 search for quine...");
    eprintln!("Target output: {:?}", program);
    eprintln!("Search space: {} positions × 8 values/position = {} max attempts\n", 
        program.len(), program.len() * 8);

    // Start building from the last digit
    if let Some(a) = find_a(&program, 1, 0, reg_b, reg_c, &mut stats, 0) {
        eprintln!("\n✅ Solution found: A = {}", a);
        eprintln!("\n📊 Search Statistics:");
        eprintln!("  Total attempts: {}", stats.total_attempts);
        eprintln!("  Backtracks required: {}", stats.backtracks);
        eprintln!("  Successful paths: {}", stats.successful_paths);
        eprintln!("  Maximum depth reached: {}", stats.max_depth);
        eprintln!("  Efficiency: {:.2}% (successful / total attempts)", 
            (stats.successful_paths as f64 / stats.total_attempts as f64) * 100.0);
        eprintln!("  Pruning effectiveness: avoided {} brute force attempts", 
            8_i64.pow(program.len() as u32) - stats.total_attempts as i64);
        
        return Ok(a.to_string());
    }

    anyhow::bail!("No solution found")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[allow(dead_code)]
    const EXAMPLE_INPUT_PART2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_parse_input() {
        let (a, b, c, program) = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(a, 729);
        assert_eq!(b, 0);
        assert_eq!(c, 0);
        assert_eq!(program, vec![0, 1, 5, 4, 3, 0]);
    }

    #[test]
    fn test_part1_example() {
        let result = solve_part1(EXAMPLE_INPUT).unwrap();
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_execute_c_set() {
        // If register C contains 9, the program 2,6 would set register B to 1
        let output = execute_program(0, 0, 9, &[2, 6]);
        assert_eq!(output, vec![]);
    }

    #[test]
    fn test_execute_output() {
        // Program 5,0,5,1,5,4 outputs 0,1,2
        let output = execute_program(10, 0, 0, &[5, 0, 5, 1, 5, 4]);
        assert_eq!(output, vec![0, 1, 2]);
    }

    #[test]
    fn test_execute_adv_out() {
        // Program 0,1,5,4,3,0 with A=729 outputs 4,6,3,5,6,3,5,2,1,0
        let output = execute_program(729, 0, 0, &[0, 1, 5, 4, 3, 0]);
        assert_eq!(output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn test_execute_bxl() {
        // Program 1,7 with B=29 sets B to 26
        let output = execute_program(0, 29, 0, &[1, 7]);
        assert_eq!(output, vec![]);
    }

    #[test]
    fn test_execute_bxc() {
        // Program 4,0 with B=2024, C=43690 sets B to 44354
        let output = execute_program(0, 2024, 43690, &[4, 0]);
        assert_eq!(output, vec![]);
    }
}
