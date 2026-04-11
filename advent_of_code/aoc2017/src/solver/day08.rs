//! Day 8: I Heard You Like Registers
//!
//! Execute conditional register instructions and track the largest
//! value both at the end and at any point during execution.

use std::collections::HashMap;

#[derive(Debug)]
struct Instruction<'a> {
    target: &'a str,
    delta: i64,
    cond_reg: &'a str,
    cond_op: &'a str,
    cond_val: i64,
}

fn parse_input(input: &str) -> Vec<Instruction<'_>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // Format: target inc/dec amount if cond_reg cond_op cond_val
            let amount: i64 = parts[2].parse().unwrap();
            let delta = if parts[1] == "inc" { amount } else { -amount };
            Instruction {
                target: parts[0],
                delta,
                cond_reg: parts[4],
                cond_op: parts[5],
                cond_val: parts[6].parse().unwrap(),
            }
        })
        .collect()
}

fn check_condition(reg_val: i64, op: &str, cond_val: i64) -> bool {
    match op {
        ">" => reg_val > cond_val,
        "<" => reg_val < cond_val,
        ">=" => reg_val >= cond_val,
        "<=" => reg_val <= cond_val,
        "==" => reg_val == cond_val,
        "!=" => reg_val != cond_val,
        _ => unreachable!("unknown operator: {op}"),
    }
}

fn execute(instructions: &[Instruction<'_>]) -> (i64, i64) {
    let mut registers: HashMap<&str, i64> = HashMap::new();
    let mut all_time_max = 0;

    for instr in instructions {
        let cond_val = *registers.get(instr.cond_reg).unwrap_or(&0);
        if check_condition(cond_val, instr.cond_op, instr.cond_val) {
            let reg = registers.entry(instr.target).or_insert(0);
            *reg += instr.delta;
            if *reg > all_time_max {
                all_time_max = *reg;
            }
        }
    }

    let final_max = registers.values().copied().max().unwrap_or(0);
    (final_max, all_time_max)
}

pub fn solve(input: &str) -> (i64, i64) {
    let instructions = parse_input(input);
    execute(&instructions)
}

pub fn solve_part1(input: &str) -> i64 {
    solve(input).0
}

pub fn solve_part2(input: &str) -> i64 {
    solve(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    #[test]
    fn test_parse() {
        let instructions = parse_input(EXAMPLE);
        assert_eq!(instructions.len(), 4);
        assert_eq!(instructions[0].target, "b");
        assert_eq!(instructions[0].delta, 5);
        assert_eq!(instructions[0].cond_reg, "a");
        assert_eq!(instructions[0].cond_op, ">");
        assert_eq!(instructions[0].cond_val, 1);
        // "dec -10" should become +10 (negated negative)
        assert_eq!(instructions[2].delta, 10);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 1);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 10);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day08.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, 3612);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day08.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, 3818);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day08.txt");
        assert_eq!(solve(input), (3612, 3818));
    }
}
