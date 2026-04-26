//! Measure the actual tape footprint for Day 25:
//! - cursor min/max positions visited
//! - count of cells the cursor *touched* (vs cells holding a 1 at the end)

use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Rule {
    write: u8,
    delta: i8,
    next: u8,
}

fn state_index(c: char) -> u8 {
    (c as u8) - b'A'
}

fn parse(input: &str) -> (u8, u64, Vec<[Rule; 2]>) {
    let normalized = input.replace("\r\n", "\n");
    let mut blocks = normalized.split("\n\n").filter(|b| !b.trim().is_empty());

    let header = blocks.next().unwrap();
    let mut hl = header.lines();
    let start_char = hl.next().unwrap().trim_end_matches('.').chars().last().unwrap();
    let start = state_index(start_char);
    let steps: u64 = hl.next().unwrap()
        .split_whitespace()
        .find_map(|w| w.parse().ok())
        .unwrap();

    let state_blocks: Vec<&str> = blocks.collect();
    let mut rules = vec![
        [Rule { write: 0, delta: 0, next: 0 }, Rule { write: 0, delta: 0, next: 0 }];
        state_blocks.len()
    ];

    for block in state_blocks {
        let lines: Vec<&str> = block.lines().collect();
        let state_char = lines[0].trim_end_matches(':').chars().last().unwrap();
        let state = state_index(state_char) as usize;

        let parse_rule = |start_line: usize| -> Rule {
            let write = lines[start_line + 1]
                .trim_end_matches('.').chars().last().unwrap()
                .to_digit(10).unwrap() as u8;
            let delta: i8 = if lines[start_line + 2].contains("right") { 1 } else { -1 };
            let next = state_index(
                lines[start_line + 3].trim_end_matches('.').chars().last().unwrap()
            );
            Rule { write, delta, next }
        };

        rules[state] = [parse_rule(1), parse_rule(5)];
    }

    (start, steps, rules)
}

fn main() {
    let input = std::fs::read_to_string("inputs/day25.txt").expect("inputs/day25.txt");
    let (start, steps, rules) = parse(&input);

    let mut tape: Vec<u8> = vec![0u8; 1 << 16];
    let mut offset: isize = 1 << 15;
    let mut pos: isize = 0;
    let mut state = start as usize;

    let mut min_pos: isize = 0;
    let mut max_pos: isize = 0;
    let mut visited: HashSet<isize> = HashSet::new();
    visited.insert(0);

    for _ in 0..steps {
        let idx = pos + offset;
        let i = idx as usize;
        let val = tape[i] as usize;
        let rule = &rules[state][val];
        tape[i] = rule.write;
        pos += rule.delta as isize;
        state = rule.next as usize;

        if pos < min_pos { min_pos = pos; }
        if pos > max_pos { max_pos = pos; }
        visited.insert(pos);
    }

    let ones: usize = tape.iter().map(|&b| b as usize).sum();
    let footprint = (max_pos - min_pos + 1) as usize;

    println!("Steps run:          {}", steps);
    println!("Min position:       {}", min_pos);
    println!("Max position:       {}", max_pos);
    println!("Footprint (range):  {} cells", footprint);
    println!("Distinct cells touched: {}", visited.len());
    println!("Cells holding 1 at end: {}", ones);
}
