//! Day 23: Coprocessor Conflagration
//!
//! A reduced variant of the Day 18 assembly: 8 registers (a..h) initialised
//! to 0, and four opcodes — `set`, `sub`, `mul`, `jnz`. Part 1 runs the
//! program straight through in debug mode and reports how many times the
//! `mul` instruction fires.

#[derive(Debug, Clone, Copy)]
enum Value {
    Reg(usize),
    Num(i64),
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Set(usize, Value),
    Sub(usize, Value),
    Mul(usize, Value),
    Jnz(Value, Value),
}

fn parse_reg(s: &str) -> usize {
    (s.as_bytes()[0] - b'a') as usize
}

fn parse_value(s: &str) -> Value {
    let b = s.as_bytes()[0];
    if b.is_ascii_lowercase() {
        Value::Reg((b - b'a') as usize)
    } else {
        Value::Num(s.parse().expect("valid integer"))
    }
}

fn parse_input(input: &str) -> Vec<Instr> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut it = line.split_whitespace();
            let op = it.next().expect("op");
            let x = it.next().expect("x");
            let y = it.next().expect("y");
            match op {
                "set" => Instr::Set(parse_reg(x), parse_value(y)),
                "sub" => Instr::Sub(parse_reg(x), parse_value(y)),
                "mul" => Instr::Mul(parse_reg(x), parse_value(y)),
                "jnz" => Instr::Jnz(parse_value(x), parse_value(y)),
                _ => panic!("unknown instruction: {op}"),
            }
        })
        .collect()
}

#[inline]
fn val(v: Value, regs: &[i64; 8]) -> i64 {
    match v {
        Value::Reg(r) => regs[r],
        Value::Num(n) => n,
    }
}

fn solve_part1_with_data(instrs: &[Instr]) -> usize {
    let mut regs = [0i64; 8];
    let mut pc: i64 = 0;
    let mut mul_count: usize = 0;
    while pc >= 0 && (pc as usize) < instrs.len() {
        match instrs[pc as usize] {
            Instr::Set(r, v) => {
                regs[r] = val(v, &regs);
                pc += 1;
            }
            Instr::Sub(r, v) => {
                regs[r] -= val(v, &regs);
                pc += 1;
            }
            Instr::Mul(r, v) => {
                regs[r] *= val(v, &regs);
                mul_count += 1;
                pc += 1;
            }
            Instr::Jnz(x, y) => {
                if val(x, &regs) != 0 {
                    pc += val(y, &regs);
                } else {
                    pc += 1;
                }
            }
        }
    }
    mul_count
}

/// Extract the Part 2 loop bounds by reading the program's setup section
/// rather than simulating it. The canonical AoC 2017 Day 23 layout is:
///
/// ```text
///  0: set b <seed>
///  4: mul b <mult>
///  5: sub b -<add>     (negative literal → adds |add|)
///  7: sub c -<range>
/// 30: sub b -<step>
/// ```
///
/// With a=1, b starts at `seed*mult + add` and the outer loop runs while
/// b ≤ b + range, incrementing by `step`. The d/e double-loop is a naive
/// primality test: it sets f=0 iff b is composite, and the terminal
/// `sub h -1` increments h when f=0. So h counts composites in the AP.
fn extract_bounds(instrs: &[Instr]) -> (i64, i64, i64) {
    let seed = match instrs[0] {
        Instr::Set(_, Value::Num(n)) => n,
        _ => panic!("line 0 not `set b <num>`"),
    };
    let mult = match instrs[4] {
        Instr::Mul(_, Value::Num(n)) => n,
        _ => panic!("line 4 not `mul b <num>`"),
    };
    let add = match instrs[5] {
        Instr::Sub(_, Value::Num(n)) => -n,
        _ => panic!("line 5 not `sub b <num>`"),
    };
    let range = match instrs[7] {
        Instr::Sub(_, Value::Num(n)) => -n,
        _ => panic!("line 7 not `sub c <num>`"),
    };
    let step = match instrs[30] {
        Instr::Sub(_, Value::Num(n)) => -n,
        _ => panic!("line 30 not `sub b <num>`"),
    };
    let b_start = seed * mult + add;
    let b_end = b_start + range;
    (b_start, b_end, step)
}

#[inline]
fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3i64;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn solve_part2_with_data(instrs: &[Instr]) -> i64 {
    let (b_start, b_end, step) = extract_bounds(instrs);
    let mut h = 0i64;
    let mut n = b_start;
    while n <= b_end {
        if !is_prime(n) {
            h += 1;
        }
        n += step;
    }
    h
}

pub fn solve(input: &str) -> (usize, i64) {
    let instrs = parse_input(input);
    (solve_part1_with_data(&instrs), solve_part2_with_data(&instrs))
}

pub fn solve_part1(input: &str) -> usize {
    solve_part1_with_data(&parse_input(input))
}

pub fn solve_part2(input: &str) -> i64 {
    solve_part2_with_data(&parse_input(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_actual() {
        let input = include_str!("../../inputs/day23.txt");
        let instrs = parse_input(input);
        assert_eq!(instrs.len(), 32);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day23.txt");
        assert_eq!(solve_part1(input), 8281);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day23.txt");
        assert_eq!(solve_part2(input), 911);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day23.txt");
        assert_eq!(solve(input), (8281, 911));
    }

    #[test]
    fn test_bounds_match_setup_simulation() {
        // Cross-check extract_bounds against actually simulating the setup
        // section with a=1 up to line 8.
        let instrs = parse_input(include_str!("../../inputs/day23.txt"));
        let mut regs = [0i64; 8];
        regs[0] = 1;
        let mut pc: i64 = 0;
        while (pc as usize) < 8 {
            match instrs[pc as usize] {
                Instr::Set(r, v) => {
                    regs[r] = val(v, &regs);
                    pc += 1;
                }
                Instr::Sub(r, v) => {
                    regs[r] -= val(v, &regs);
                    pc += 1;
                }
                Instr::Mul(r, v) => {
                    regs[r] *= val(v, &regs);
                    pc += 1;
                }
                Instr::Jnz(x, y) => {
                    if val(x, &regs) != 0 {
                        pc += val(y, &regs);
                    } else {
                        pc += 1;
                    }
                }
            }
        }
        let (b_start, b_end, step) = extract_bounds(&instrs);
        assert_eq!(regs[1], b_start, "b after setup");
        assert_eq!(regs[2], b_end, "c after setup");
        assert_eq!(step, 17, "step constant");
    }

    #[test]
    fn test_is_prime_basics() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(97));
        assert!(!is_prime(91)); // 7 * 13
        assert!(is_prime(109_297)); // largest prime below 109_300
        assert!(!is_prime(109_300)); // endpoint of Part 2 AP is composite
        assert!(!is_prime(126_300));
    }
}
