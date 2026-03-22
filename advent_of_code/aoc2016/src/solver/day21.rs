//! Day 21: Scrambled Letters and Hash
//!
//! Scramble a password by applying a series of string operations,
//! then unscramble by reversing them.

#[derive(Debug, Clone)]
enum Op {
    SwapPos(usize, usize),
    SwapLetter(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBased(u8),
    Reverse(usize, usize),
    Move(usize, usize),
}

fn parse_input(input: &str) -> Vec<Op> {
    input
        .trim()
        .lines()
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            match (words[0], words[1]) {
                ("swap", "position") => {
                    Op::SwapPos(words[2].parse().unwrap(), words[5].parse().unwrap())
                }
                ("swap", "letter") => {
                    Op::SwapLetter(words[2].as_bytes()[0], words[5].as_bytes()[0])
                }
                ("rotate", "left") => Op::RotateLeft(words[2].parse().unwrap()),
                ("rotate", "right") => Op::RotateRight(words[2].parse().unwrap()),
                ("rotate", "based") => Op::RotateBased(words[6].as_bytes()[0]),
                ("reverse", _) => {
                    Op::Reverse(words[2].parse().unwrap(), words[4].parse().unwrap())
                }
                ("move", _) => {
                    Op::Move(words[2].parse().unwrap(), words[5].parse().unwrap())
                }
                _ => panic!("unknown instruction: {line}"),
            }
        })
        .collect()
}

fn apply(pw: &mut Vec<u8>, op: &Op) {
    let len = pw.len();
    match *op {
        Op::SwapPos(x, y) => pw.swap(x, y),
        Op::SwapLetter(a, b) => {
            let x = pw.iter().position(|&c| c == a).unwrap();
            let y = pw.iter().position(|&c| c == b).unwrap();
            pw.swap(x, y);
        }
        Op::RotateLeft(n) => pw.rotate_left(n % len),
        Op::RotateRight(n) => pw.rotate_right(n % len),
        Op::RotateBased(letter) => {
            let idx = pw.iter().position(|&c| c == letter).unwrap();
            let rot = (1 + idx + if idx >= 4 { 1 } else { 0 }) % len;
            pw.rotate_right(rot);
        }
        Op::Reverse(x, y) => pw[x..=y].reverse(),
        Op::Move(x, y) => {
            let ch = pw.remove(x);
            pw.insert(y, ch);
        }
    }
}

fn unapply(pw: &mut Vec<u8>, op: &Op) {
    let len = pw.len();
    match *op {
        // Self-inverse operations
        Op::SwapPos(_, _) | Op::SwapLetter(_, _) | Op::Reverse(_, _) => apply(pw, op),
        // Reverse rotations
        Op::RotateLeft(n) => pw.rotate_right(n % len),
        Op::RotateRight(n) => pw.rotate_left(n % len),
        // Reverse move: move from Y back to X
        Op::Move(x, y) => {
            let ch = pw.remove(y);
            pw.insert(x, ch);
        }
        // Reverse rotate-based: brute force — try all left rotations until forward
        // application would produce the current state
        Op::RotateBased(_letter) => {
            let target = pw.clone();
            for _ in 0..len {
                pw.rotate_left(1);
                let mut test = pw.clone();
                apply(&mut test, op);
                if test == target {
                    return;
                }
            }
            panic!("could not reverse rotate-based");
        }
    }
}

fn scramble(password: &str, ops: &[Op]) -> String {
    let mut pw: Vec<u8> = password.bytes().collect();
    for op in ops {
        apply(&mut pw, op);
    }
    String::from_utf8(pw).unwrap()
}

fn unscramble(scrambled: &str, ops: &[Op]) -> String {
    let mut pw: Vec<u8> = scrambled.bytes().collect();
    for op in ops.iter().rev() {
        unapply(&mut pw, op);
    }
    String::from_utf8(pw).unwrap()
}

pub fn solve(input: &str) -> (String, String) {
    let ops = parse_input(input);
    let part1 = scramble("abcdefgh", &ops);
    let part2 = unscramble("fbgdceah", &ops);
    (part1, part2)
}

pub fn solve_part1(input: &str) -> String {
    let ops = parse_input(input);
    scramble("abcdefgh", &ops)
}

pub fn solve_part2(input: &str) -> String {
    let ops = parse_input(input);
    unscramble("fbgdceah", &ops)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

    #[test]
    fn test_parse() {
        let ops = parse_input(EXAMPLE);
        assert_eq!(ops.len(), 8);
    }

    #[test]
    fn test_example_step_by_step() {
        let ops = parse_input(EXAMPLE);
        let mut pw: Vec<u8> = b"abcde".to_vec();

        apply(&mut pw, &ops[0]); // swap position 4 with 0
        assert_eq!(&pw, b"ebcda");

        apply(&mut pw, &ops[1]); // swap letter d with b
        assert_eq!(&pw, b"edcba");

        apply(&mut pw, &ops[2]); // reverse positions 0 through 4
        assert_eq!(&pw, b"abcde");

        apply(&mut pw, &ops[3]); // rotate left 1
        assert_eq!(&pw, b"bcdea");

        apply(&mut pw, &ops[4]); // move position 1 to 4
        assert_eq!(&pw, b"bdeac");

        apply(&mut pw, &ops[5]); // move position 3 to 0
        assert_eq!(&pw, b"abdec");

        apply(&mut pw, &ops[6]); // rotate based on position of letter b
        assert_eq!(&pw, b"ecabd");

        apply(&mut pw, &ops[7]); // rotate based on position of letter d
        assert_eq!(&pw, b"decab");
    }

    #[test]
    fn test_example_scramble() {
        let ops = parse_input(EXAMPLE);
        let mut pw: Vec<u8> = b"abcde".to_vec();
        for op in &ops {
            apply(&mut pw, op);
        }
        assert_eq!(String::from_utf8(pw).unwrap(), "decab");
    }

    #[test]
    fn test_example_unscramble() {
        let ops = parse_input(EXAMPLE);
        let mut pw: Vec<u8> = b"decab".to_vec();
        for op in ops.iter().rev() {
            unapply(&mut pw, op);
        }
        assert_eq!(String::from_utf8(pw).unwrap(), "abcde");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day21.txt");
        assert_eq!(
            solve(input),
            ("hcdefbag".to_string(), "fbhaegdc".to_string())
        );
    }
}
