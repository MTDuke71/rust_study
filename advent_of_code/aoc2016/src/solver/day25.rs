//! Day 25: Clock Signal
//!
//! Extended assembunny VM with `out` instruction that transmits a value.
//! Find the lowest positive `a` that produces the clock signal 0, 1, 0, 1, ...
//!
//! The program computes `a + 15*170 = a + 2550`, then repeatedly divides by 2,
//! outputting the remainder (binary digits, LSB first). For 0,1,0,1,... we need
//! `a + 2550` to have the bit pattern `101010...10` in binary.
//!
//! Part 2 is the traditional "collect all 50 stars" freebie.

#[derive(Debug, Clone, Copy)]
enum Value {
    Lit(i64),
    Reg(usize),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
    Out(Value),
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
                "out" => Instruction::Out(parse_value(parts[1])),
                _ => panic!("Unknown instruction: {}", parts[0]),
            }
        })
        .collect()
}

fn resolve(regs: &[i64; 4], v: Value) -> i64 {
    match v {
        Value::Lit(n) => n,
        Value::Reg(r) => regs[r],
    }
}

/// Run the VM with initial `a` value. Returns true if it produces valid clock signal
/// (0, 1, 0, 1, ...) for at least `required_outputs` values.
fn produces_clock_signal(program: &[Instruction], init_a: i64, required_outputs: usize) -> bool {
    let mut regs = [0i64; 4];
    regs[0] = init_a;
    let mut pc: usize = 0;
    let mut output_count = 0;
    let mut expected = 0; // alternates 0, 1, 0, 1, ...
    let max_steps = 1_000_000;
    let mut steps = 0;

    while pc < program.len() && steps < max_steps {
        steps += 1;
        match program[pc] {
            Instruction::Cpy(src, Value::Reg(dst)) => {
                regs[dst] = resolve(&regs, src);
                pc += 1;
            }
            Instruction::Cpy(_, Value::Lit(_)) => {
                pc += 1;
            }
            Instruction::Inc(Value::Reg(r)) => {
                regs[r] += 1;
                pc += 1;
            }
            Instruction::Inc(Value::Lit(_)) => {
                pc += 1;
            }
            Instruction::Dec(Value::Reg(r)) => {
                regs[r] -= 1;
                pc += 1;
            }
            Instruction::Dec(Value::Lit(_)) => {
                pc += 1;
            }
            Instruction::Jnz(test, offset) => {
                if resolve(&regs, test) != 0 {
                    pc = (pc as i64 + resolve(&regs, offset)) as usize;
                } else {
                    pc += 1;
                }
            }
            Instruction::Out(val) => {
                let output = resolve(&regs, val);
                if output != expected {
                    return false;
                }
                output_count += 1;
                if output_count >= required_outputs {
                    return true;
                }
                expected = 1 - expected;
                pc += 1;
            }
        }
    }

    false
}

fn solve_part1(program: &[Instruction]) -> i64 {
    for a in 1.. {
        if produces_clock_signal(program, a, 50) {
            return a;
        }
    }
    unreachable!()
}

pub fn solve(input: &str) -> (String, String) {
    let program = parse_input(input);
    let p1 = solve_part1(&program);
    (p1.to_string(), "Deliver the Signal!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTUAL: &str = include_str!("../../inputs/day25.txt");

    #[test]
    fn test_parse() {
        let program = parse_input(ACTUAL);
        assert_eq!(program.len(), 30);
        // Last meaningful instruction: out b
        assert!(matches!(program[27], Instruction::Out(Value::Reg(1))));
    }

    #[test]
    fn test_part1_actual() {
        let program = parse_input(ACTUAL);
        assert_eq!(solve_part1(&program), 180);
    }

    #[test]
    fn test_clock_signal_verification() {
        // a=180 should produce clock signal, a=179 should not
        let program = parse_input(ACTUAL);
        assert!(produces_clock_signal(&program, 180, 50));
        assert!(!produces_clock_signal(&program, 179, 50));
    }

    #[test]
    fn test_both_parts_actual() {
        let (p1, p2) = solve(ACTUAL);
        assert_eq!(p1, "180");
        assert_eq!(p2, "Deliver the Signal!");
    }
}
