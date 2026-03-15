//! Day 15: Timing is Everything
//!
//! Find the first time to press a button so a capsule falls through
//! all rotating discs. Each disc must be at position 0 when the capsule
//! reaches it (disc N is reached N seconds after button press).

/// A rotating disc with a fixed number of positions
#[derive(Debug, Clone)]
struct Disc {
    number: usize,     // disc number (1-based, also = time offset)
    positions: usize,  // total positions on this disc
    initial: usize,    // position at time=0
}

fn parse_input(input: &str) -> Vec<Disc> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // "Disc #1 has 17 positions; at time=0, it is at position 1."
            let parts: Vec<&str> = line.split_whitespace().collect();
            let number: usize = parts[1][1..].parse().unwrap();
            let positions: usize = parts[3].parse().unwrap();
            let initial: usize = parts[11].trim_end_matches('.').parse().unwrap();
            Disc { number, positions, initial }
        })
        .collect()
}

// --- Brute-force approach: linear scan ---

/// Check if all discs are at position 0 when the capsule reaches them
fn all_aligned(discs: &[Disc], time: usize) -> bool {
    discs.iter().all(|d| (d.initial + time + d.number).is_multiple_of(d.positions))
}

fn solve_brute(discs: &[Disc]) -> usize {
    (0..).find(|&t| all_aligned(discs, t)).unwrap()
}

// --- CRT approach: combine congruences ---

/// Extended Euclidean algorithm: returns (gcd, x, y) where a*x + b*y = gcd
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

/// Combine two congruences: t ≡ a (mod m) and t ≡ b (mod n)
/// Returns (combined_remainder, combined_modulus)
fn combine(a: i64, m: i64, b: i64, n: i64) -> (i64, i64) {
    let (g, x, _) = extended_gcd(m, n);
    let lcm = m / g * n;
    let diff = ((b - a) % n + n) % n;
    let solution = a + m * ((diff / g * x) % (n / g));
    ((solution % lcm + lcm) % lcm, lcm)
}

/// Solve using Chinese Remainder Theorem — O(n) where n = number of discs
fn solve_crt(discs: &[Disc]) -> usize {
    let mut remainder: i64 = 0;
    let mut modulus: i64 = 1;

    for d in discs {
        // t ≡ -(initial + number) (mod positions)
        let r = (-(d.initial as i64) - d.number as i64).rem_euclid(d.positions as i64);
        let m = d.positions as i64;
        (remainder, modulus) = combine(remainder, modulus, r, m);
    }
    remainder as usize
}

// --- Solve functions ---

fn solve_part1_with_data(discs: &[Disc]) -> usize {
    solve_crt(discs)
}

fn solve_part2_with_data(discs: &[Disc]) -> usize {
    let mut extended = discs.to_vec();
    extended.push(Disc {
        number: discs.len() + 1,
        positions: 11,
        initial: 0,
    });
    solve_crt(&extended)
}

pub fn solve(input: &str) -> (String, String) {
    let discs = parse_input(input);
    (
        solve_part1_with_data(&discs).to_string(),
        solve_part2_with_data(&discs).to_string(),
    )
}

pub fn solve_part1(input: &str) -> String {
    let discs = parse_input(input);
    solve_part1_with_data(&discs).to_string()
}

pub fn solve_part2(input: &str) -> String {
    let discs = parse_input(input);
    solve_part2_with_data(&discs).to_string()
}

/// Brute-force solve for benchmarking comparison
pub fn solve_brute_force(input: &str) -> (String, String) {
    let discs = parse_input(input);
    let mut extended = discs.to_vec();
    extended.push(Disc {
        number: discs.len() + 1,
        positions: 11,
        initial: 0,
    });
    (
        solve_brute(&discs).to_string(),
        solve_brute(&extended).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";

    #[test]
    fn test_parse() {
        let discs = parse_input(EXAMPLE);
        assert_eq!(discs.len(), 2);
        assert_eq!(discs[0].positions, 5);
        assert_eq!(discs[0].initial, 4);
        assert_eq!(discs[1].positions, 2);
        assert_eq!(discs[1].initial, 1);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), "5");
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day15.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, "317371");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day15.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, "2080951");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day15.txt");
        assert_eq!(solve(input), ("317371".to_string(), "2080951".to_string()));
    }

    #[test]
    fn test_crt_matches_brute_force() {
        let input = include_str!("../../inputs/day15.txt");
        assert_eq!(solve(input), solve_brute_force(input));
    }

    #[test]
    fn test_crt_example() {
        let discs = parse_input(EXAMPLE);
        assert_eq!(solve_crt(&discs), 5);
    }
}
