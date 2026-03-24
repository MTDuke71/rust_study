//! Day 23: Safe Cracking
//!
//! Extended assembunny VM with `tgl` (toggle) instruction that modifies
//! instructions at runtime. Computes `a! + C` where C is input-dependent.
//! Includes multiplication loop optimization for Part 2 (a=12 → 12! iterations).

/// A value: either a literal integer or a register reference.
#[derive(Debug, Clone, Copy)]
enum Value {
    Lit(i64),
    Reg(usize), // 0=a, 1=b, 2=c, 3=d
}

/// Assembunny instructions (extended with Tgl).
#[derive(Debug, Clone, Copy)]
enum Instruction {
    Cpy(Value, Value), // Value dst to support invalid toggles (cpy x 1)
    Inc(Value),        // Value to support invalid toggles (inc 1)
    Dec(Value),        // Value to support invalid toggles (dec 1)
    Jnz(Value, Value), // Both Value to support toggled cpy→jnz with reg offset
    Tgl(Value),        // Toggle instruction at PC + x
}

fn parse_value(token: &str) -> Value {
    match token {
        "a" => Value::Reg(0),
        "b" => Value::Reg(1),
        "c" => Value::Reg(2),
        "d" => Value::Reg(3),
        _ => Value::Lit(token.parse().expect("invalid literal")),
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "cpy" => Instruction::Cpy(parse_value(parts[1]), parse_value(parts[2])),
                "inc" => Instruction::Inc(parse_value(parts[1])),
                "dec" => Instruction::Dec(parse_value(parts[1])),
                "jnz" => Instruction::Jnz(parse_value(parts[1]), parse_value(parts[2])),
                "tgl" => Instruction::Tgl(parse_value(parts[1])),
                _ => panic!("Unknown instruction: {}", parts[0]),
            }
        })
        .collect()
}

/// Toggle an instruction per Day 23 rules:
/// - 1-arg: inc ↔ dec (non-inc becomes inc)
/// - 2-arg: jnz ↔ cpy (non-jnz becomes jnz)
fn toggle(instr: Instruction) -> Instruction {
    match instr {
        // One-argument: inc becomes dec, everything else becomes inc
        Instruction::Inc(x) => Instruction::Dec(x),
        Instruction::Dec(x) => Instruction::Inc(x),
        Instruction::Tgl(x) => Instruction::Inc(x),
        // Two-argument: jnz becomes cpy, everything else becomes jnz
        Instruction::Jnz(x, y) => Instruction::Cpy(x, y),
        Instruction::Cpy(x, y) => Instruction::Jnz(x, y),
    }
}

fn resolve(regs: &[i64; 4], v: Value) -> i64 {
    match v {
        Value::Lit(n) => n,
        Value::Reg(r) => regs[r],
    }
}

/// Detect multiplication loop pattern at `pc`:
///   pc+0: cpy b c
///   pc+1: inc a
///   pc+2: dec c
///   pc+3: jnz c -2
///   pc+4: dec d
///   pc+5: jnz d -5
/// If matched, returns (a_reg, b_reg, c_reg, d_reg).
fn detect_mul_loop(program: &[Instruction], pc: usize) -> Option<(usize, usize, usize, usize)> {
    if pc + 5 >= program.len() {
        return None;
    }
    // cpy B C
    let (b_reg, c_reg) = match program[pc] {
        Instruction::Cpy(Value::Reg(b), Value::Reg(c)) => (b, c),
        _ => return None,
    };
    // inc A
    let a_reg = match program[pc + 1] {
        Instruction::Inc(Value::Reg(a)) => a,
        _ => return None,
    };
    // dec C
    match program[pc + 2] {
        Instruction::Dec(Value::Reg(r)) if r == c_reg => {}
        _ => return None,
    }
    // jnz C -2
    match program[pc + 3] {
        Instruction::Jnz(Value::Reg(r), Value::Lit(-2)) if r == c_reg => {}
        _ => return None,
    }
    // dec D
    let d_reg = match program[pc + 4] {
        Instruction::Dec(Value::Reg(d)) => d,
        _ => return None,
    };
    // jnz D -5
    match program[pc + 5] {
        Instruction::Jnz(Value::Reg(r), Value::Lit(-5)) if r == d_reg => {}
        _ => return None,
    }
    Some((a_reg, b_reg, c_reg, d_reg))
}

fn run_vm(program: &[Instruction], init_a: i64) -> i64 {
    let mut prog = program.to_vec();
    let mut regs = [0i64; 4];
    regs[0] = init_a;
    let mut pc: usize = 0;

    while pc < prog.len() {
        // Try multiplication loop optimization
        if let Some((a, b, c, d)) = detect_mul_loop(&prog, pc) {
            regs[a] += regs[b] * regs[d];
            regs[c] = 0;
            regs[d] = 0;
            pc += 6;
            continue;
        }

        match prog[pc] {
            Instruction::Cpy(src, Value::Reg(dst)) => {
                regs[dst] = resolve(&regs, src);
                pc += 1;
            }
            Instruction::Cpy(_, Value::Lit(_)) => {
                // Invalid target (toggled), skip
                pc += 1;
            }
            Instruction::Inc(Value::Reg(r)) => {
                regs[r] += 1;
                pc += 1;
            }
            Instruction::Inc(Value::Lit(_)) => {
                pc += 1; // Invalid, skip
            }
            Instruction::Dec(Value::Reg(r)) => {
                regs[r] -= 1;
                pc += 1;
            }
            Instruction::Dec(Value::Lit(_)) => {
                pc += 1; // Invalid, skip
            }
            Instruction::Jnz(test, offset) => {
                if resolve(&regs, test) != 0 {
                    pc = (pc as i64 + resolve(&regs, offset)) as usize;
                } else {
                    pc += 1;
                }
            }
            Instruction::Tgl(x) => {
                let target = pc as i64 + resolve(&regs, x);
                if target >= 0 && (target as usize) < prog.len() {
                    let t = target as usize;
                    prog[t] = toggle(prog[t]);
                }
                pc += 1;
            }
        }
    }

    regs[0]
}

fn solve_part1_with_data(program: &[Instruction]) -> i64 {
    run_vm(program, 7)
}

fn solve_part2_with_data(program: &[Instruction]) -> i64 {
    run_vm(program, 12)
}

pub fn solve(input: &str) -> (String, String) {
    let program = parse_input(input);
    (
        solve_part1_with_data(&program).to_string(),
        solve_part2_with_data(&program).to_string(),
    )
}

pub fn solve_part1(input: &str) -> i64 {
    let program = parse_input(input);
    solve_part1_with_data(&program)
}

pub fn solve_part2(input: &str) -> i64 {
    let program = parse_input(input);
    solve_part2_with_data(&program)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

    #[test]
    fn test_parse() {
        let program = parse_input(EXAMPLE);
        assert_eq!(program.len(), 7);
        assert!(matches!(program[0], Instruction::Cpy(Value::Lit(2), Value::Reg(0))));
        assert!(matches!(program[1], Instruction::Tgl(Value::Reg(0))));
    }

    #[test]
    fn test_part1_example() {
        let program = parse_input(EXAMPLE);
        assert_eq!(run_vm(&program, 2), 3);
    }

    const ACTUAL: &str = include_str!("../../inputs/day23.txt");

    #[test]
    fn test_part1_actual() {
        assert_eq!(solve_part1(ACTUAL), 12480);
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(solve_part2(ACTUAL), 479009040);
    }

    #[test]
    fn test_both_parts_actual() {
        let (p1, p2) = solve(ACTUAL);
        assert_eq!(p1, "12480");
        assert_eq!(p2, "479009040");
    }
}
