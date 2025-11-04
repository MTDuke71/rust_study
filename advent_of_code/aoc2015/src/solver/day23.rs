/// AoC 2015 Day 23: Opening the Turing Lock
///
/// Part 1: Execute assembly program with register a = 0, return register b
/// Part 2: Execute assembly program with register a = 1, return register b
///
/// ## Problem Overview
///
/// Computer with two registers (a, b) and 6 instructions:
/// - hlf r: register r = r / 2
/// - tpl r: register r = r * 3
/// - inc r: register r += 1
/// - jmp offset: jump by offset
/// - jie r, offset: jump if register r is even
/// - jio r, offset: jump if register r == 1
///
/// Program exits when trying to execute beyond last instruction.
///
/// ## Solution Approach
///
/// Implement register machine that:
/// 1. Parses assembly instructions into enum variants
/// 2. Executes program step by step with instruction pointer
/// 3. Handles jumps and conditional jumps
/// 4. Tracks register values throughout execution
/// 5. Traces execution for debugging
use anyhow::Result;

/// Represents the two registers in the computer
#[derive(Debug, Clone)]
pub struct Registers {
    pub a: i64,
    pub b: i64,
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    /// Create registers with both set to 0
    pub fn new() -> Self {
        Self { a: 0, b: 0 }
    }

    /// Create registers with specific initial values
    pub fn with_values(a: i64, b: i64) -> Self {
        Self { a, b }
    }

    /// Get register value by name
    pub fn get(&self, reg: &str) -> i64 {
        match reg {
            "a" => self.a,
            "b" => self.b,
            _ => panic!("Unknown register: {}", reg),
        }
    }

    /// Set register value by name
    pub fn set(&mut self, reg: &str, value: i64) {
        match reg {
            "a" => self.a = value,
            "b" => self.b = value,
            _ => panic!("Unknown register: {}", reg),
        }
    }
}

/// Assembly instructions supported by the computer
#[derive(Debug, Clone)]
pub enum Instruction {
    /// hlf r: set register r to half its current value
    Half(String),
    /// tpl r: set register r to triple its current value
    Triple(String),
    /// inc r: increment register r by 1
    Increment(String),
    /// jmp offset: jump by offset relative to current instruction
    Jump(i64),
    /// jie r, offset: jump if register r is even
    JumpIfEven(String, i64),
    /// jio r, offset: jump if register r equals 1
    JumpIfOne(String, i64),
}

impl Instruction {
    /// Parse a single instruction from string
    /// Format: "opcode operand1[, operand2]"
    pub fn parse(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[0] {
            "hlf" => Ok(Instruction::Half(parts[1].to_string())),
            "tpl" => Ok(Instruction::Triple(parts[1].to_string())),
            "inc" => Ok(Instruction::Increment(parts[1].to_string())),
            "jmp" => {
                let offset = Self::parse_offset(parts[1])?;
                Ok(Instruction::Jump(offset))
            }
            "jie" => {
                let reg = parts[1].trim_end_matches(',');
                let offset = Self::parse_offset(parts[2])?;
                Ok(Instruction::JumpIfEven(reg.to_string(), offset))
            }
            "jio" => {
                let reg = parts[1].trim_end_matches(',');
                let offset = Self::parse_offset(parts[2])?;
                Ok(Instruction::JumpIfOne(reg.to_string(), offset))
            }
            _ => Err(anyhow::anyhow!("Unknown instruction: {}", line)),
        }
    }

    /// Parse jump offset (handles + and - prefixes)
    fn parse_offset(s: &str) -> Result<i64> {
        s.parse::<i64>()
            .map_err(|_| anyhow::anyhow!("Invalid offset: {}", s))
    }
}

/// Computer that executes the assembly program
#[derive(Debug, Clone)]
pub struct Computer {
    pub registers: Registers,
    pub program: Vec<Instruction>,
    pub pc: usize, // Program counter (instruction pointer)
}

impl Computer {
    /// Create computer with program and initial registers
    pub fn new(program: Vec<Instruction>, registers: Registers) -> Self {
        Self {
            registers,
            program,
            pc: 0,
        }
    }

    /// Execute one instruction and update program counter
    /// Returns true if execution should continue, false if program ended
    pub fn step(&mut self) -> Result<bool> {
        if self.pc >= self.program.len() {
            return Ok(false); // Program ended
        }

        let instruction = &self.program[self.pc];
        let mut next_pc = self.pc as i64 + 1; // Default: advance to next instruction

        match instruction {
            Instruction::Half(reg) => {
                let value = self.registers.get(reg);
                self.registers.set(reg, value / 2);
            }
            Instruction::Triple(reg) => {
                let value = self.registers.get(reg);
                self.registers.set(reg, value * 3);
            }
            Instruction::Increment(reg) => {
                let value = self.registers.get(reg);
                self.registers.set(reg, value + 1);
            }
            Instruction::Jump(offset) => {
                next_pc = self.pc as i64 + offset;
            }
            Instruction::JumpIfEven(reg, offset) => {
                let value = self.registers.get(reg);
                if value % 2 == 0 {
                    next_pc = self.pc as i64 + offset;
                }
            }
            Instruction::JumpIfOne(reg, offset) => {
                let value = self.registers.get(reg);
                if value == 1 {
                    next_pc = self.pc as i64 + offset;
                }
            }
        }

        self.pc = next_pc as usize;
        Ok(true)
    }

    /// Execute entire program until completion
    /// Returns final register values
    pub fn execute(&mut self) -> Result<&Registers> {
        while self.step()? {
            // Continue executing
        }
        Ok(&self.registers)
    }

    /// Execute with tracing (prints register values each step)
    /// Returns final register values
    pub fn execute_with_trace(&mut self) -> Result<String> {
        let mut trace = String::new();
        trace.push_str(&format!("Initial state: a={}, b={}\n", self.registers.a, self.registers.b));

        let mut step_count = 0;
        while self.step()? {
            step_count += 1;
            trace.push_str(&format!("After step {}: a={}, b={} (pc={})\n",
                step_count, self.registers.a, self.registers.b, self.pc));
        }

        trace.push_str(&format!("Final state: a={}, b={}\n", self.registers.a, self.registers.b));
        Ok(trace)
    }
}

/// Parse program from multi-line string
pub fn parse_program(input: &str) -> Result<Vec<Instruction>> {
    input.lines()
        .map(|line| Instruction::parse(line.trim()))
        .collect()
}

/// Solve Part 1: Execute with register a = 0
pub fn solve_part1(input: &str) -> Result<String> {
    let program = parse_program(input)?;
    let registers = Registers::with_values(0, 0);
    let mut computer = Computer::new(program, registers);

    let final_registers = computer.execute()?;
    Ok(final_registers.b.to_string())
}

/// Solve Part 2: Execute with register a = 1
pub fn solve_part2(input: &str) -> Result<String> {
    let program = parse_program(input)?;
    let registers = Registers::with_values(1, 0);
    let mut computer = Computer::new(program, registers);

    let final_registers = computer.execute()?;
    Ok(final_registers.b.to_string())
}

/// Get execution trace for debugging
pub fn get_execution_trace(input: &str, initial_a: i64) -> Result<String> {
    let program = parse_program(input)?;
    let registers = Registers::with_values(initial_a, 0);
    let mut computer = Computer::new(program, registers);

    computer.execute_with_trace()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        // Test each instruction type
        assert!(matches!(Instruction::parse("hlf a").unwrap(), Instruction::Half(ref r) if r == "a"));
        assert!(matches!(Instruction::parse("tpl b").unwrap(), Instruction::Triple(ref r) if r == "b"));
        assert!(matches!(Instruction::parse("inc a").unwrap(), Instruction::Increment(ref r) if r == "a"));
        assert!(matches!(Instruction::parse("jmp +2").unwrap(), Instruction::Jump(2)));
        assert!(matches!(Instruction::parse("jie a, +4").unwrap(), Instruction::JumpIfEven(ref r, 4) if r == "a"));
        assert!(matches!(Instruction::parse("jio b, -3").unwrap(), Instruction::JumpIfOne(ref r, -3) if r == "b"));
    }

    #[test]
    fn test_simple_execution() {
        let program = vec![
            Instruction::Increment("a".to_string()),
            Instruction::Increment("a".to_string()),
            Instruction::Increment("b".to_string()),
        ];

        let registers = Registers::new();
        let mut computer = Computer::new(program, registers);

        let final_registers = computer.execute().unwrap();
        assert_eq!(final_registers.a, 2);
        assert_eq!(final_registers.b, 1);
    }

    #[test]
    fn test_jump_execution() {
        let program = vec![
            Instruction::Jump(2), // Jump to instruction 2 (0-indexed)
            Instruction::Increment("a".to_string()), // This should be skipped
            Instruction::Increment("b".to_string()), // This should execute
        ];

        let registers = Registers::new();
        let mut computer = Computer::new(program, registers);

        let final_registers = computer.execute().unwrap();
        assert_eq!(final_registers.a, 0); // Skipped
        assert_eq!(final_registers.b, 1); // Executed
    }

    #[test]
    fn test_conditional_jumps() {
        // Test jie (jump if even)
        let program = vec![
            Instruction::Increment("a".to_string()), // a = 1 (odd)
            Instruction::JumpIfEven("a".to_string(), 2), // Should not jump
            Instruction::Increment("b".to_string()), // Should execute (b = 1)
            Instruction::Increment("b".to_string()), // Should execute (b = 2)
        ];

        let registers = Registers::new();
        let mut computer = Computer::new(program, registers);

        let final_registers = computer.execute().unwrap();
        assert_eq!(final_registers.a, 1);
        assert_eq!(final_registers.b, 2);

        // Test jio (jump if one)
        let program2 = vec![
            Instruction::Increment("a".to_string()), // a = 1
            Instruction::JumpIfOne("a".to_string(), 2), // Should jump
            Instruction::Increment("b".to_string()), // Should be skipped
            Instruction::Increment("b".to_string()), // Should execute
        ];

        let registers2 = Registers::new();
        let mut computer2 = Computer::new(program2, registers2);

        let final_registers2 = computer2.execute().unwrap();
        assert_eq!(final_registers2.a, 1);
        assert_eq!(final_registers2.b, 1); // Only the jumped-to instruction
    }

    #[test]
    fn test_example_program() {
        // The example from the problem statement
        let input = "inc a\njio a, +2\ntpl a\ninc a";
        let program = parse_program(input).unwrap();

        let registers = Registers::new();
        let mut computer = Computer::new(program, registers);

        let final_registers = computer.execute().unwrap();
        assert_eq!(final_registers.a, 2); // As stated in the problem
    }

    #[test]
    fn test_solve_part1() {
        let input = "inc a\njio a, +2\ntpl a\ninc a";
        let result = solve_part1(input).unwrap();
        assert_eq!(result, "0"); // Register b ends at 0 in this simple program
    }

    #[test]
    fn test_solve_part2() {
        let input = "inc a\njio a, +2\ntpl a\ninc a";
        let result = solve_part2(input).unwrap();
        assert_eq!(result, "0"); // Register b still ends at 0
    }
}