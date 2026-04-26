//! Day 25: The Halting Problem
//!
//! Simulate a Turing machine described by a structured blueprint, then count
//! the number of `1`s on the tape after the configured number of steps. The
//! tape is conceptually infinite in both directions; we back it with a single
//! `Vec<u8>` of bits-stored-as-bytes plus an offset so that index `pos +
//! offset` in the buffer corresponds to logical tape position `pos`. When the
//! cursor reaches either end we double the buffer and re-center, so the
//! amortised cost of growth is O(1) per step.
//!
//! Part 2 is the traditional 50th-star freebie — there is no second puzzle.

#[derive(Debug, Clone, Copy)]
struct Rule {
    write: u8,
    delta: i8,
    next: u8,
}

#[derive(Debug, Clone)]
struct Machine {
    /// `rules[state][value]` is the rule fired when in `state` reading `value`.
    rules: Vec<[Rule; 2]>,
    start: u8,
    steps: u64,
}

fn state_index(c: char) -> u8 {
    (c as u8) - b'A'
}

fn parse_input(input: &str) -> Machine {
    let normalized = input.replace("\r\n", "\n");
    let mut blocks = normalized.split("\n\n").filter(|b| !b.trim().is_empty());

    let header = blocks.next().expect("header block");
    let mut header_lines = header.lines();
    let begin_line = header_lines.next().expect("Begin in state line");
    let start_char = begin_line
        .trim_end_matches('.')
        .chars()
        .last()
        .expect("state letter");
    let start = state_index(start_char);

    let steps_line = header_lines.next().expect("Perform line");
    let steps: u64 = steps_line
        .split_whitespace()
        .find_map(|w| w.parse().ok())
        .expect("step count");

    let state_blocks: Vec<&str> = blocks.collect();
    let mut rules: Vec<[Rule; 2]> = vec![
        [
            Rule { write: 0, delta: 0, next: 0 },
            Rule { write: 0, delta: 0, next: 0 },
        ];
        state_blocks.len()
    ];

    for block in state_blocks {
        let lines: Vec<&str> = block.lines().collect();
        let header = lines[0]; // "In state X:"
        let state_char = header
            .trim_end_matches(':')
            .chars()
            .last()
            .expect("state letter in block");
        let state = state_index(state_char) as usize;

        // Each rule occupies four lines starting at offsets 1 and 5.
        let parse_rule = |start_line: usize| -> Rule {
            let write_line = lines[start_line + 1];
            let write_digit = write_line
                .trim_end_matches('.')
                .chars()
                .last()
                .expect("write digit");
            let write = write_digit.to_digit(10).expect("0 or 1") as u8;

            let move_line = lines[start_line + 2];
            let delta: i8 = if move_line.contains("right") { 1 } else { -1 };

            let next_line = lines[start_line + 3];
            let next_char = next_line
                .trim_end_matches('.')
                .chars()
                .last()
                .expect("next state letter");
            let next = state_index(next_char);

            Rule { write, delta, next }
        };

        rules[state] = [parse_rule(1), parse_rule(5)];
    }

    Machine { rules, start, steps }
}

/// Simulate the machine and return the count of `1`s on the tape.
///
/// The tape grows on demand: when the cursor would step outside the buffer
/// we allocate one twice as large, copy the existing contents into the
/// middle, and shift the offset so logical positions stay stable.
fn run_machine(m: &Machine) -> u32 {
    let mut tape: Vec<u8> = vec![0u8; 1 << 16];
    let mut offset: isize = 1 << 15;
    let mut pos: isize = 0;
    let mut state = m.start as usize;

    for _ in 0..m.steps {
        let mut idx = pos + offset;
        if idx < 0 || (idx as usize) >= tape.len() {
            let old_len = tape.len();
            let new_len = old_len * 2;
            let extra = old_len / 2;
            let mut new_tape = vec![0u8; new_len];
            new_tape[extra..extra + old_len].copy_from_slice(&tape);
            tape = new_tape;
            offset += extra as isize;
            idx = pos + offset;
        }
        let i = idx as usize;
        let val = tape[i] as usize;
        let rule = &m.rules[state][val];
        tape[i] = rule.write;
        pos += rule.delta as isize;
        state = rule.next as usize;
    }

    tape.iter().map(|&b| b as u32).sum()
}

fn solve_part1_with_data(m: &Machine) -> u32 {
    run_machine(m)
}

pub fn solve(input: &str) -> (u32, String) {
    let m = parse_input(input);
    let checksum = run_machine(&m);
    (checksum, "Merry Christmas! 🎄".to_string())
}

pub fn solve_part1(input: &str) -> u32 {
    solve_part1_with_data(&parse_input(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
";

    #[test]
    fn test_parse_example() {
        let m = parse_input(EXAMPLE);
        assert_eq!(m.start, 0);
        assert_eq!(m.steps, 6);
        assert_eq!(m.rules.len(), 2);
        // State A, value 0: write 1, move right, next B.
        assert_eq!(m.rules[0][0].write, 1);
        assert_eq!(m.rules[0][0].delta, 1);
        assert_eq!(m.rules[0][0].next, 1);
        // State A, value 1: write 0, move left, next B.
        assert_eq!(m.rules[0][1].write, 0);
        assert_eq!(m.rules[0][1].delta, -1);
        assert_eq!(m.rules[0][1].next, 1);
        // State B, value 0: write 1, move left, next A.
        assert_eq!(m.rules[1][0].write, 1);
        assert_eq!(m.rules[1][0].delta, -1);
        assert_eq!(m.rules[1][0].next, 0);
        // State B, value 1: write 1, move right, next A.
        assert_eq!(m.rules[1][1].write, 1);
        assert_eq!(m.rules[1][1].delta, 1);
        assert_eq!(m.rules[1][1].next, 0);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 3);
    }

    #[test]
    fn test_solve_example() {
        let (checksum, msg) = solve(EXAMPLE);
        assert_eq!(checksum, 3);
        assert!(msg.contains("Merry Christmas"));
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day25.txt");
        let (checksum, _) = solve(input);
        assert_eq!(checksum, 4287);
    }
}
