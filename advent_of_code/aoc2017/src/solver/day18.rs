//! Day 18: Duet
//!
//! A tiny assembly interpreter with 26 integer registers (a..z).
//!
//! - **Part 1** ("sound card" mode): `snd X` plays a sound of frequency X;
//!   `rcv X` recovers the last played sound when X ≠ 0. Report the first
//!   recovered frequency.
//! - **Part 2** ("duet" mode): two programs run concurrently with distinct
//!   register sets (p = 0 and p = 1) and FIFO message queues. `snd X` sends
//!   X to the other program's queue; `rcv X` pops one value from its own
//!   queue (blocks if empty). Report how many times program 1 sends.

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Value {
    Reg(usize),
    Num(i64),
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Snd(Value),
    Set(usize, Value),
    Add(usize, Value),
    Mul(usize, Value),
    Mod(usize, Value),
    Rcv(usize),
    Jgz(Value, Value),
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
        .map(|line| {
            let mut it = line.split_whitespace();
            let op = it.next().expect("op");
            let x = it.next().expect("x");
            let y = it.next();
            match op {
                "snd" => Instr::Snd(parse_value(x)),
                "set" => Instr::Set(parse_reg(x), parse_value(y.unwrap())),
                "add" => Instr::Add(parse_reg(x), parse_value(y.unwrap())),
                "mul" => Instr::Mul(parse_reg(x), parse_value(y.unwrap())),
                "mod" => Instr::Mod(parse_reg(x), parse_value(y.unwrap())),
                "rcv" => Instr::Rcv(parse_reg(x)),
                "jgz" => Instr::Jgz(parse_value(x), parse_value(y.unwrap())),
                _ => panic!("unknown instruction: {op}"),
            }
        })
        .collect()
}

#[inline]
fn val(v: Value, regs: &[i64; 26]) -> i64 {
    match v {
        Value::Reg(r) => regs[r],
        Value::Num(n) => n,
    }
}

fn solve_part1_with_data(instrs: &[Instr]) -> i64 {
    let mut regs = [0i64; 26];
    let mut last_sound: i64 = 0;
    let mut pc: i64 = 0;
    while pc >= 0 && (pc as usize) < instrs.len() {
        match instrs[pc as usize] {
            Instr::Snd(v) => {
                last_sound = val(v, &regs);
                pc += 1;
            }
            Instr::Set(r, v) => {
                regs[r] = val(v, &regs);
                pc += 1;
            }
            Instr::Add(r, v) => {
                regs[r] += val(v, &regs);
                pc += 1;
            }
            Instr::Mul(r, v) => {
                regs[r] *= val(v, &regs);
                pc += 1;
            }
            Instr::Mod(r, v) => {
                regs[r] %= val(v, &regs);
                pc += 1;
            }
            Instr::Rcv(r) => {
                if regs[r] != 0 {
                    return last_sound;
                }
                pc += 1;
            }
            Instr::Jgz(x, y) => {
                if val(x, &regs) > 0 {
                    pc += val(y, &regs);
                } else {
                    pc += 1;
                }
            }
        }
    }
    last_sound
}

#[derive(Debug)]
struct Program {
    regs: [i64; 26],
    pc: i64,
    send_count: usize,
}

impl Program {
    fn new(pid: i64) -> Self {
        let mut regs = [0i64; 26];
        regs[(b'p' - b'a') as usize] = pid;
        Self {
            regs,
            pc: 0,
            send_count: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum StepResult {
    Ok,
    Blocked,
    Terminated,
}

fn step(p: &mut Program, instrs: &[Instr], inbox: &mut VecDeque<i64>, outbox: &mut VecDeque<i64>) -> StepResult {
    if p.pc < 0 || (p.pc as usize) >= instrs.len() {
        return StepResult::Terminated;
    }
    match instrs[p.pc as usize] {
        Instr::Snd(v) => {
            outbox.push_back(val(v, &p.regs));
            p.send_count += 1;
            p.pc += 1;
        }
        Instr::Set(r, v) => {
            p.regs[r] = val(v, &p.regs);
            p.pc += 1;
        }
        Instr::Add(r, v) => {
            p.regs[r] += val(v, &p.regs);
            p.pc += 1;
        }
        Instr::Mul(r, v) => {
            p.regs[r] *= val(v, &p.regs);
            p.pc += 1;
        }
        Instr::Mod(r, v) => {
            p.regs[r] %= val(v, &p.regs);
            p.pc += 1;
        }
        Instr::Rcv(r) => match inbox.pop_front() {
            Some(x) => {
                p.regs[r] = x;
                p.pc += 1;
            }
            None => return StepResult::Blocked,
        },
        Instr::Jgz(x, y) => {
            if val(x, &p.regs) > 0 {
                p.pc += val(y, &p.regs);
            } else {
                p.pc += 1;
            }
        }
    }
    StepResult::Ok
}

fn solve_part2_with_data(instrs: &[Instr]) -> usize {
    let mut p0 = Program::new(0);
    let mut p1 = Program::new(1);
    let mut q0: VecDeque<i64> = VecDeque::new(); // messages for p0
    let mut q1: VecDeque<i64> = VecDeque::new(); // messages for p1
    loop {
        let r0 = step(&mut p0, instrs, &mut q0, &mut q1);
        let r1 = step(&mut p1, instrs, &mut q1, &mut q0);
        // Deadlock: both non-Ok and neither queue advances either program.
        let p0_done = matches!(r0, StepResult::Blocked | StepResult::Terminated);
        let p1_done = matches!(r1, StepResult::Blocked | StepResult::Terminated);
        if p0_done && p1_done {
            // Confirm true deadlock: each blocked program has no message waiting.
            let p0_stuck = r0 == StepResult::Terminated || q0.is_empty();
            let p1_stuck = r1 == StepResult::Terminated || q1.is_empty();
            if p0_stuck && p1_stuck {
                return p1.send_count;
            }
        }
    }
}

pub fn solve(input: &str) -> (i64, usize) {
    let instrs = parse_input(input);
    (solve_part1_with_data(&instrs), solve_part2_with_data(&instrs))
}

pub fn solve_part1(input: &str) -> i64 {
    solve_part1_with_data(&parse_input(input))
}

pub fn solve_part2(input: &str) -> usize {
    solve_part2_with_data(&parse_input(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_P1: &str = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

    const EXAMPLE_P2: &str = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";

    #[test]
    fn test_parse() {
        let instrs = parse_input(EXAMPLE_P1);
        assert_eq!(instrs.len(), 10);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE_P1), 4);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE_P2), 3);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day18.txt");
        assert_eq!(solve_part1(input), 3188);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day18.txt");
        assert_eq!(solve_part2(input), 7112);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day18.txt");
        assert_eq!(solve(input), (3188, 7112));
    }
}
