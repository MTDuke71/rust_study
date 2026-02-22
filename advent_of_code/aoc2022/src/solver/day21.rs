//! Day 21: Monkey Math
//!
//! Monkeys yell numbers or compute operations on other monkeys' values.
//! Part 1: Evaluate the expression tree rooted at "root".
//! Part 2: root becomes equality check; find what "humn" must yell.

use std::collections::HashMap;

// ============================================================================
// Data Types
// ============================================================================

#[derive(Debug, Clone)]
pub enum Monkey<'a> {
    Num(i64),
    Op {
        left: &'a str,
        op: char,
        right: &'a str,
    },
}

pub type ParsedData<'a> = HashMap<&'a str, Monkey<'a>>;

// ============================================================================
// Parsing
// ============================================================================

pub fn parse_input(input: &str) -> ParsedData<'_> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (name, rest) = line.split_once(": ").unwrap();
            let monkey = if let Ok(n) = rest.trim().parse::<i64>() {
                Monkey::Num(n)
            } else {
                let parts: Vec<&str> = rest.split_whitespace().collect();
                Monkey::Op {
                    left: parts[0],
                    op: parts[1].chars().next().unwrap(),
                    right: parts[2],
                }
            };
            (name, monkey)
        })
        .collect()
}

// ============================================================================
// Part 1: Evaluate expression tree
// ============================================================================

fn eval(monkeys: &ParsedData<'_>, name: &str) -> i64 {
    match &monkeys[name] {
        Monkey::Num(n) => *n,
        Monkey::Op { left, op, right } => {
            let l = eval(monkeys, left);
            let r = eval(monkeys, right);
            match op {
                '+' => l + r,
                '-' => l - r,
                '*' => l * r,
                '/' => l / r,
                _ => unreachable!(),
            }
        }
    }
}

pub fn solve_part1(monkeys: &ParsedData<'_>) -> i64 {
    eval(monkeys, "root")
}

// ============================================================================
// Part 2: Solve for humn
// ============================================================================

/// Returns true if the subtree rooted at `name` contains "humn".
fn contains_humn(monkeys: &ParsedData<'_>, name: &str) -> bool {
    if name == "humn" {
        return true;
    }
    match &monkeys[name] {
        Monkey::Num(_) => false,
        Monkey::Op { left, right, .. } => {
            contains_humn(monkeys, left) || contains_humn(monkeys, right)
        }
    }
}

/// Walk down the tree from `name` toward "humn", inverting operations.
/// `target` is the value this subtree must equal.
fn solve_for_humn(monkeys: &ParsedData<'_>, name: &str, target: i64) -> i64 {
    if name == "humn" {
        return target;
    }

    match &monkeys[name] {
        Monkey::Num(_) => panic!("reached leaf {name} without finding humn"),
        Monkey::Op { left, op, right } => {
            let humn_in_left = contains_humn(monkeys, left);
            let (humn_side, known_side) = if humn_in_left {
                (*left, *right)
            } else {
                (*right, *left)
            };
            let known_val = eval(monkeys, known_side);

            // Invert the operation to find what humn_side must equal.
            // If humn is on the left:  left op known = target
            // If humn is on the right: known op right = target
            let new_target = if humn_in_left {
                match op {
                    '+' => target - known_val,        // left + k = t  → left = t - k
                    '-' => target + known_val,        // left - k = t  → left = t + k
                    '*' => target / known_val,        // left * k = t  → left = t / k
                    '/' => target * known_val,        // left / k = t  → left = t * k
                    _ => unreachable!(),
                }
            } else {
                match op {
                    '+' => target - known_val,        // k + right = t → right = t - k
                    '-' => known_val - target,        // k - right = t → right = k - t
                    '*' => target / known_val,        // k * right = t → right = t / k
                    '/' => known_val / target,        // k / right = t → right = k / t
                    _ => unreachable!(),
                }
            };

            solve_for_humn(monkeys, humn_side, new_target)
        }
    }
}

pub fn solve_part2(monkeys: &ParsedData<'_>) -> i64 {
    // root's two children must be equal
    let (left, right) = match &monkeys["root"] {
        Monkey::Op { left, right, .. } => (*left, *right),
        _ => panic!("root must be an operation"),
    };

    let humn_in_left = contains_humn(monkeys, left);
    let (humn_side, known_side) = if humn_in_left {
        (left, right)
    } else {
        (right, left)
    };

    let target = eval(monkeys, known_side);
    solve_for_humn(monkeys, humn_side, target)
}

// ============================================================================
// Entry point
// ============================================================================

pub fn solve(input: &str) -> (i64, i64) {
    let monkeys = parse_input(input);
    (solve_part1(&monkeys), solve_part2(&monkeys))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_parse() {
        let monkeys = parse_input(EXAMPLE);
        assert_eq!(monkeys.len(), 15);
        assert!(matches!(monkeys["humn"], Monkey::Num(5)));
        assert!(matches!(
            monkeys["root"],
            Monkey::Op { op: '+', .. }
        ));
    }

    #[test]
    fn test_part1_example() {
        let monkeys = parse_input(EXAMPLE);
        assert_eq!(solve_part1(&monkeys), 152);
    }

    #[test]
    fn test_part2_example() {
        let monkeys = parse_input(EXAMPLE);
        assert_eq!(solve_part2(&monkeys), 301);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day21.txt");
        assert_eq!(solve(input), (160_274_622_817_992, 3_087_390_115_721));
    }
}
