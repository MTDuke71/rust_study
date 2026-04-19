//! Reachability probe: count how many times each pc is executed by each program.

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Value { Reg(usize), Num(i64) }

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

fn parse_reg(s: &str) -> usize { (s.as_bytes()[0] - b'a') as usize }
fn parse_value(s: &str) -> Value {
    let b = s.as_bytes()[0];
    if b.is_ascii_lowercase() { Value::Reg((b - b'a') as usize) }
    else { Value::Num(s.parse().unwrap()) }
}

fn parse_input(input: &str) -> Vec<Instr> {
    input.lines().map(|line| {
        let mut it = line.split_whitespace();
        let op = it.next().unwrap();
        let x = it.next().unwrap();
        let y = it.next();
        match op {
            "snd" => Instr::Snd(parse_value(x)),
            "set" => Instr::Set(parse_reg(x), parse_value(y.unwrap())),
            "add" => Instr::Add(parse_reg(x), parse_value(y.unwrap())),
            "mul" => Instr::Mul(parse_reg(x), parse_value(y.unwrap())),
            "mod" => Instr::Mod(parse_reg(x), parse_value(y.unwrap())),
            "rcv" => Instr::Rcv(parse_reg(x)),
            "jgz" => Instr::Jgz(parse_value(x), parse_value(y.unwrap())),
            _ => panic!(),
        }
    }).collect()
}

fn val(v: Value, regs: &[i64; 26]) -> i64 {
    match v { Value::Reg(r) => regs[r], Value::Num(n) => n }
}

struct P { regs: [i64; 26], pc: i64, hits: Vec<u64> }

impl P {
    fn new(pid: i64, n: usize) -> Self {
        let mut regs = [0i64; 26];
        regs[15] = pid;
        Self { regs, pc: 0, hits: vec![0; n] }
    }
}

#[derive(PartialEq, Eq)]
enum R { Ok, Blocked, Term }

fn step(p: &mut P, ins: &[Instr], inbox: &mut VecDeque<i64>, outbox: &mut VecDeque<i64>) -> R {
    if p.pc < 0 || (p.pc as usize) >= ins.len() { return R::Term; }
    let i = p.pc as usize;
    p.hits[i] += 1;
    match ins[i] {
        Instr::Snd(v) => { outbox.push_back(val(v, &p.regs)); p.pc += 1; }
        Instr::Set(r,v) => { p.regs[r] = val(v, &p.regs); p.pc += 1; }
        Instr::Add(r,v) => { p.regs[r] += val(v, &p.regs); p.pc += 1; }
        Instr::Mul(r,v) => { p.regs[r] *= val(v, &p.regs); p.pc += 1; }
        Instr::Mod(r,v) => { p.regs[r] %= val(v, &p.regs); p.pc += 1; }
        Instr::Rcv(r) => match inbox.pop_front() {
            Some(x) => { p.regs[r] = x; p.pc += 1; }
            None => { return R::Blocked; } // count blocked attempts too
        }
        Instr::Jgz(x,y) => {
            if val(x,&p.regs) > 0 { p.pc += val(y,&p.regs); } else { p.pc += 1; }
        }
    }
    R::Ok
}

fn main() {
    let input = std::fs::read_to_string("inputs/day18.txt").unwrap();
    let ins = parse_input(&input);
    let n = ins.len();
    let mut p0 = P::new(0, n);
    let mut p1 = P::new(1, n);
    let mut q0 = VecDeque::new();
    let mut q1 = VecDeque::new();
    loop {
        let r0 = step(&mut p0, &ins, &mut q0, &mut q1);
        let r1 = step(&mut p1, &ins, &mut q1, &mut q0);
        let d0 = matches!(r0, R::Blocked | R::Term);
        let d1 = matches!(r1, R::Blocked | R::Term);
        if d0 && d1 {
            let s0 = r0 == R::Term || q0.is_empty();
            let s1 = r1 == R::Term || q1.is_empty();
            if s0 && s1 { break; }
        }
    }
    println!("pc | p0 hits    | p1 hits");
    println!("---+------------+-----------");
    for i in 0..n {
        println!("{:>2} | {:>10} | {:>10}", i, p0.hits[i], p1.hits[i]);
    }
}
