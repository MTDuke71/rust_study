//! Day 17: Spinlock
//!
//! A circular buffer insertion algorithm. Part 1 simulates 2017 insertions
//! and returns the value immediately after 2017. Part 2 requires 50,000,000
//! insertions — solved by observing position 0 never changes, so we only
//! track the most recent value inserted at position 1.

fn parse_input(input: &str) -> usize {
    input.trim().parse().expect("valid step count")
}

fn solve_part1_with_data(steps: usize) -> usize {
    let mut buf = Vec::with_capacity(2018);
    buf.push(0);
    let mut pos = 0;
    for i in 1..=2017 {
        pos = (pos + steps) % i + 1;
        buf.insert(pos, i);
    }
    buf[(pos + 1) % buf.len()]
}

fn solve_part2_with_data(steps: usize) -> usize {
    // Position 0 never changes, so we only track the most recent value
    // written to position 1 — iterations where (pos + steps) % i == 0.
    //
    // Optimization: when pos + steps < i, no wrap occurs and insertion
    // happens at pos+1 (never 1). We can batch all consecutive no-wrap
    // iterations into a single arithmetic jump. The inner "wrap" loop
    // runs ~steps * ln(50M/steps) ≈ 4.3k iterations instead of 50M.
    const N: usize = 50_000_000;
    let mut pos = 0usize;
    let mut value_at_1 = 0usize;
    let mut i = 1usize;
    while i <= N {
        if pos + steps < i {
            // Batch: max k such that all of j = 0..k-1 are no-wraps.
            // No-wrap at j ⇔ j*steps < i - pos - steps. So k = ⌈D/steps⌉.
            let d = i - pos - steps;
            let k = d.div_ceil(steps).min(N + 1 - i);
            pos += k * (steps + 1);
            i += k;
        } else {
            pos = (pos + steps) % i;
            if pos == 0 {
                value_at_1 = i;
            }
            pos += 1;
            i += 1;
        }
    }
    value_at_1
}

pub fn solve(input: &str) -> (usize, usize) {
    let steps = parse_input(input);
    (solve_part1_with_data(steps), solve_part2_with_data(steps))
}

pub fn solve_part1(input: &str) -> usize {
    solve_part1_with_data(parse_input(input))
}

pub fn solve_part2(input: &str) -> usize {
    solve_part2_with_data(parse_input(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_input("363\n"), 363);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1("3"), 638);
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day17.txt");
        assert_eq!(solve_part1(input), 136);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day17.txt");
        assert_eq!(solve_part2(input), 1_080_289);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day17.txt");
        assert_eq!(solve(input), (136, 1_080_289));
    }
}
