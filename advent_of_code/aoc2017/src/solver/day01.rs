//! Day 1: Inverse Captcha
//!
//! Find the sum of all digits that match the next digit in a circular list (Part 1)
//! or the digit halfway around the list (Part 2).

fn parse_digits(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

pub fn solve(input: &str) -> (u32, u32) {
    let digits = parse_digits(input);
    let len = digits.len();

    // Part 1: compare each digit with the next (wrapping around)
    let part1: u32 = digits
        .iter()
        .enumerate()
        .filter(|&(i, &d)| d == digits[(i + 1) % len])
        .map(|(_, &d)| d)
        .sum();

    // Part 2: compare each digit with the one halfway around
    let half = len / 2;
    let part2: u32 = digits
        .iter()
        .enumerate()
        .filter(|&(i, &d)| d == digits[(i + half) % len])
        .map(|(_, &d)| d)
        .sum();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(solve("1122").0, 3);
        assert_eq!(solve("1111").0, 4);
        assert_eq!(solve("1234").0, 0);
        assert_eq!(solve("91212129").0, 9);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(solve("1212").1, 6);
        assert_eq!(solve("1221").1, 0);
        assert_eq!(solve("123425").1, 4);
        assert_eq!(solve("123123").1, 12);
        assert_eq!(solve("12131415").1, 4);
    }
}
