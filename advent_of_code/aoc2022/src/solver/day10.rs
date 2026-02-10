//! # Day 10: Cathode-Ray Tube
//!
//! CPU simulation with a single X register and two instructions:
//! - `noop`: Takes 1 cycle, does nothing
//! - `addx V`: Takes 2 cycles, adds V to X after completion
//!
//! Part 1: Calculate signal strength (cycle * X) at specific cycles (20, 60, 100, 140, 180, 220)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

/// Parse the input into a list of instructions
fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "noop" => Instruction::Noop,
                "addx" => {
                    let value = parts[1].parse().expect("invalid addx value");
                    Instruction::Addx(value)
                }
                _ => panic!("unknown instruction: {}", parts[0]),
            }
        })
        .collect()
}

/// Internal: Solve Part 1 with pre-parsed instructions
fn solve_part1_with_instructions(instructions: &[Instruction]) -> i32 {
    let mut x = 1i32; // Register X starts at 1
    let mut cycle = 0;
    let mut signal_sum = 0;
    
    for instruction in instructions {
        let cycles_to_execute = instruction.cycles();
        
        for _ in 0..cycles_to_execute {
            cycle += 1;
            
            // Check if we're at a target cycle (20, 60, 100, 140, 180, 220)
            if cycle == 20 || (cycle > 20 && (cycle - 20) % 40 == 0) {
                let signal_strength = cycle * x;
                signal_sum += signal_strength;
            }
        }
        
        // After the instruction completes, update X if it's addx
        if let Instruction::Addx(value) = instruction {
            x += value;
        }
    }
    
    signal_sum
}

pub fn solve_part1(input: &str) -> i32 {
    let instructions = parse_instructions(input);
    solve_part1_with_instructions(&instructions)
}

/// Internal: Solve Part 2 with pre-parsed instructions
fn solve_part2_with_instructions(instructions: &[Instruction]) -> String {
    let mut x = 1i32; // Sprite center position
    let mut cycle = 0;
    let mut crt = String::new();
    
    for instruction in instructions {
        let cycles_to_execute = instruction.cycles();
        
        for _ in 0..cycles_to_execute {
            // Calculate CRT column position (0-39 within current row)
            let crt_col = cycle % 40;
            
            // Sprite is 3 pixels wide, centered on X: [X-1, X, X+1]
            // Draw '#' if CRT position overlaps with sprite
            if (x - 1..=x + 1).contains(&crt_col) {
                crt.push('#');
            } else {
                crt.push('.');
            }
            
            cycle += 1;
            
            // Add newline after every 40 pixels
            if cycle % 40 == 0 {
                crt.push('\n');
            }
        }
        
        // After instruction completes, update X
        if let Instruction::Addx(value) = instruction {
            x += value;
        }
    }
    
    crt.trim_end().to_string()
}

pub fn solve_part2(input: &str) -> String {
    let instructions = parse_instructions(input);
    solve_part2_with_instructions(&instructions)
}

pub fn solve(input: &str) -> (i32, String) {
    let instructions = parse_instructions(input);
    (
        solve_part1_with_instructions(&instructions),
        solve_part2_with_instructions(&instructions),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_parse_instructions() {
        let input = "noop\naddx 3\naddx -5";
        let instructions = parse_instructions(input);
        assert_eq!(instructions.len(), 3);
        assert_eq!(instructions[0], Instruction::Noop);
        assert_eq!(instructions[1], Instruction::Addx(3));
        assert_eq!(instructions[2], Instruction::Addx(-5));
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 13140);
    }

    #[test]
    fn test_part2_example() {
        let expected = "##..##..##..##..##..##..##..##..##..##..\n\
                        ###...###...###...###...###...###...###.\n\
                        ####....####....####....####....####....\n\
                        #####.....#####.....#####.....#####.....\n\
                        ######......######......######......####\n\
                        #######.......#######.......#######.....";
        assert_eq!(solve_part2(EXAMPLE), expected);
    }
}
