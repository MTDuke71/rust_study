//! Day 16: Permutation Promenade
//!
//! Dance moves applied to 16 programs (a-p). Part 2 requires
//! cycle detection to handle 1 billion repetitions.

#[derive(Debug, Clone, Copy)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .trim()
        .split(',')
        .map(|m| {
            let bytes = m.as_bytes();
            match bytes[0] {
                b's' => Move::Spin(m[1..].parse().unwrap()),
                b'x' => {
                    let (a, b) = m[1..].split_once('/').unwrap();
                    Move::Exchange(a.parse().unwrap(), b.parse().unwrap())
                }
                b'p' => Move::Partner(bytes[1], bytes[3]),
                _ => panic!("unknown move: {m}"),
            }
        })
        .collect()
}

fn dance(programs: &mut [u8; 16], moves: &[Move]) {
    for &m in moves {
        match m {
            Move::Spin(n) => {
                let mut buf = [0u8; 16];
                buf[..n].copy_from_slice(&programs[16 - n..]);
                buf[n..].copy_from_slice(&programs[..16 - n]);
                *programs = buf;
            }
            Move::Exchange(a, b) => programs.swap(a, b),
            Move::Partner(a, b) => {
                let pa = programs.iter().position(|&x| x == a).unwrap();
                let pb = programs.iter().position(|&x| x == b).unwrap();
                programs.swap(pa, pb);
            }
        }
    }
}

fn programs_to_string(programs: &[u8; 16]) -> String {
    String::from_utf8(programs.to_vec()).unwrap()
}

fn solve_part1_with_data(moves: &[Move]) -> String {
    let mut programs: [u8; 16] = *b"abcdefghijklmnop";
    dance(&mut programs, moves);
    programs_to_string(&programs)
}

fn solve_part2_with_data(moves: &[Move]) -> String {
    let initial: [u8; 16] = *b"abcdefghijklmnop";
    let mut programs = initial;

    // Store every state during cycle detection (16 bytes each, ~960 bytes for cycle of 60)
    let mut history: Vec<[u8; 16]> = vec![initial];
    loop {
        dance(&mut programs, moves);
        if programs == initial {
            break;
        }
        history.push(programs);
    }

    // Direct lookup — no repeated dances needed
    let idx = (1_000_000_000 % history.len()) as usize;
    programs_to_string(&history[idx])
}

pub fn solve(input: &str) -> (String, String) {
    let moves = parse_input(input);
    (solve_part1_with_data(&moves), solve_part2_with_data(&moves))
}

pub fn solve_part1(input: &str) -> String {
    let moves = parse_input(input);
    solve_part1_with_data(&moves)
}

pub fn solve_part2(input: &str) -> String {
    let moves = parse_input(input);
    solve_part2_with_data(&moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "s1,x3/4,pe/b";

    #[test]
    fn test_parse() {
        let moves = parse_input(EXAMPLE);
        assert_eq!(moves.len(), 3);
    }

    #[test]
    fn test_part1_example() {
        // Example uses 5 programs (a-e), not 16
        let moves = parse_input(EXAMPLE);
        let mut programs: [u8; 16] = *b"abcdefghijklmnop";
        dance(&mut programs, &moves);
        // After s1: pabcdefghijklmno
        // After x3/4: pabdcefghijklmno
        // After pe/b: paedcbfghijklmno
        assert_eq!(programs_to_string(&programs), "paedcbfghijklmno");
    }

    #[test]
    fn test_cycle_length() {
        let input = include_str!("../../inputs/day16.txt");
        let moves = parse_input(input);
        let initial: [u8; 16] = *b"abcdefghijklmnop";
        let mut programs = initial;
        let mut cycle = 0;
        for i in 1..=1000 {
            dance(&mut programs, &moves);
            if programs == initial {
                cycle = i;
                break;
            }
        }
        println!("Cycle length: {cycle} dances");
        println!("1_000_000_000 % {cycle} = {}", 1_000_000_000 % cycle);
        assert!(cycle > 0, "should find a cycle");
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day16.txt");
        assert_eq!(solve_part1(input), "olgejankfhbmpidc");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day16.txt");
        assert_eq!(solve_part2(input), "gfabehpdojkcimnl");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day16.txt");
        assert_eq!(solve(input), ("olgejankfhbmpidc".to_string(), "gfabehpdojkcimnl".to_string()));
    }
}
