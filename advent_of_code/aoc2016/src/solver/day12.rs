//! Day 12: Leonardo's Monorail
//!
//! Simple assembunny VM with 4 registers (a, b, c, d) and 4 instructions.
//! Built with step-by-step trace capability for understanding execution flow.

/// A value that can be either a literal integer or a register reference.
#[derive(Debug, Clone, Copy)]
enum Value {
    Lit(i64),
    Reg(usize), // 0=a, 1=b, 2=c, 3=d
}

/// The four assembunny instructions.
#[derive(Debug, Clone, Copy)]
enum Instruction {
    Cpy(Value, usize),  // cpy src dst_reg
    Inc(usize),         // inc reg
    Dec(usize),         // dec reg
    Jnz(Value, i64),    // jnz test offset
}

/// Map register name to index: a=0, b=1, c=2, d=3.
fn reg_index(name: &str) -> usize {
    match name {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("Unknown register: {name}"),
    }
}

/// Parse a token as either a register reference or a literal value.
fn parse_value(token: &str) -> Value {
    match token {
        "a" | "b" | "c" | "d" => Value::Reg(reg_index(token)),
        _ => Value::Lit(token.parse().expect("invalid literal")),
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "cpy" => Instruction::Cpy(parse_value(parts[1]), reg_index(parts[2])),
                "inc" => Instruction::Inc(reg_index(parts[1])),
                "dec" => Instruction::Dec(reg_index(parts[1])),
                "jnz" => Instruction::Jnz(
                    parse_value(parts[1]),
                    parts[2].parse().expect("invalid jump offset"),
                ),
                _ => panic!("Unknown instruction: {}", parts[0]),
            }
        })
        .collect()
}

const REG_NAMES: [char; 4] = ['a', 'b', 'c', 'd'];

/// A single trace entry: what happened at each step.
#[derive(Debug, Clone)]
struct TraceEntry {
    pc: usize,
    instruction: String,
    regs_after: [i64; 4],
}

impl std::fmt::Display for TraceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:>2}] {:<12}  a={:<6} b={:<6} c={:<6} d={:<6}",
            self.pc, self.instruction,
            self.regs_after[0], self.regs_after[1],
            self.regs_after[2], self.regs_after[3],
        )
    }
}

/// The assembunny virtual machine.
struct Vm {
    regs: [i64; 4],
    pc: usize,
    program: Vec<Instruction>,
    trace: Vec<TraceEntry>,
    tracing: bool,
}

impl Vm {
    fn new(program: Vec<Instruction>, tracing: bool) -> Self {
        Self {
            regs: [0; 4],
            pc: 0,
            program,
            trace: Vec::new(),
            tracing,
        }
    }

    /// Resolve a Value to its concrete i64.
    fn resolve(&self, v: Value) -> i64 {
        match v {
            Value::Lit(n) => n,
            Value::Reg(r) => self.regs[r],
        }
    }

    /// Format an instruction for trace display.
    fn fmt_instr(instr: &Instruction) -> String {
        match instr {
            Instruction::Cpy(v, r) => {
                let src = match v {
                    Value::Lit(n) => n.to_string(),
                    Value::Reg(r) => REG_NAMES[*r].to_string(),
                };
                format!("cpy {} {}", src, REG_NAMES[*r])
            }
            Instruction::Inc(r) => format!("inc {}", REG_NAMES[*r]),
            Instruction::Dec(r) => format!("dec {}", REG_NAMES[*r]),
            Instruction::Jnz(v, off) => {
                let test = match v {
                    Value::Lit(n) => n.to_string(),
                    Value::Reg(r) => REG_NAMES[*r].to_string(),
                };
                format!("jnz {} {}", test, off)
            }
        }
    }

    /// Execute one instruction. Returns false if program has halted.
    fn step(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }

        let instr = self.program[self.pc];
        let old_pc = self.pc;

        match instr {
            Instruction::Cpy(src, dst) => {
                self.regs[dst] = self.resolve(src);
                self.pc += 1;
            }
            Instruction::Inc(r) => {
                self.regs[r] += 1;
                self.pc += 1;
            }
            Instruction::Dec(r) => {
                self.regs[r] -= 1;
                self.pc += 1;
            }
            Instruction::Jnz(test, offset) => {
                if self.resolve(test) != 0 {
                    self.pc = (self.pc as i64 + offset) as usize;
                } else {
                    self.pc += 1;
                }
            }
        }

        if self.tracing {
            self.trace.push(TraceEntry {
                pc: old_pc,
                instruction: Self::fmt_instr(&instr),
                regs_after: self.regs,
            });
        }

        true
    }

    /// Run the program to completion.
    fn run(&mut self) {
        while self.step() {}
    }

    /// Print the full trace log (used in tests for step-through debugging).
    #[allow(dead_code)]
    fn print_trace(&self) {
        println!("  PC  Instruction   a       b       c       d");
        println!("{}", "-".repeat(58));
        for entry in &self.trace {
            println!("{entry}");
        }
        println!("{}", "-".repeat(58));
        println!(
            "Halted after {} steps. Registers: a={}, b={}, c={}, d={}",
            self.trace.len(),
            self.regs[0], self.regs[1], self.regs[2], self.regs[3],
        );
    }
}

fn run_vm(input: &str, init_c: i64) -> i64 {
    let program = parse(input);
    let mut vm = Vm::new(program, false);
    vm.regs[2] = init_c;
    vm.run();
    vm.regs[0]
}

pub fn solve_part1(input: &str) -> i64 {
    run_vm(input, 0)
}

pub fn solve_part2(input: &str) -> i64 {
    run_vm(input, 1)
}

pub fn solve(input: &str) -> (String, String) {
    (solve_part1(input).to_string(), solve_part2(input).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

    #[test]
    fn test_parse() {
        let program = parse(EXAMPLE);
        assert_eq!(program.len(), 6);
        // First instruction: cpy 41 a
        assert!(matches!(program[0], Instruction::Cpy(Value::Lit(41), 0)));
        // Last instruction: dec a
        assert!(matches!(program[5], Instruction::Dec(0)));
    }

    #[test]
    fn test_example_with_trace() {
        let program = parse(EXAMPLE);
        let mut vm = Vm::new(program, true);
        vm.run();
        vm.print_trace();
        assert_eq!(vm.regs[0], 42); // register a = 42
    }

    const ACTUAL: &str = include_str!("../../inputs/day12.txt");

    #[test]
    fn test_part1_actual() {
        let program = parse(ACTUAL);
        let mut vm = Vm::new(program, false);
        vm.run();
        assert_eq!(vm.regs[0], 318_117);
    }

    #[test]
    fn test_part2_actual() {
        let program = parse(ACTUAL);
        let mut vm = Vm::new(program, false);
        vm.regs[2] = 1; // c = 1
        vm.run();
        assert_eq!(vm.regs[0], 9_227_771);
    }

    #[test]
    fn test_both_parts_actual() {
        let (p1, p2) = super::solve(ACTUAL);
        assert_eq!(p1, "318117");
        assert_eq!(p2, "9227771");
    }
}
