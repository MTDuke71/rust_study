//! Day 25: Full of Hot Air
//!
//! SNAFU: balanced base-5 number system with digits = (-2), - (-1), 0, 1, 2.
//! Part 1: Sum all SNAFU numbers, return result as SNAFU.
//! Part 2: No Part 2 (Day 25 tradition - collect all other stars).

/// Convert a single SNAFU digit to its decimal value.
fn snafu_digit(c: char) -> i64 {
    match c {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!("invalid SNAFU digit: {c}"),
    }
}

/// Convert a SNAFU string to a decimal number.
fn snafu_to_decimal(s: &str) -> i64 {
    s.chars().fold(0i64, |acc, c| acc * 5 + snafu_digit(c))
}

/// Convert a decimal number to a SNAFU string.
fn decimal_to_snafu(mut n: i64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut digits = Vec::new();
    while n != 0 {
        let rem = n % 5;
        n /= 5;
        // rem is 0..4, but SNAFU digits are -2..2
        // If rem is 3 or 4, we carry +1 to next position
        let (digit, carry) = match rem {
            0 => ('0', 0),
            1 => ('1', 0),
            2 => ('2', 0),
            3 => ('=', 1), // -2 + carry: 3 = 5*1 + (-2)
            4 => ('-', 1), // -1 + carry: 4 = 5*1 + (-1)
            _ => unreachable!(),
        };
        digits.push(digit);
        n += carry;
    }
    digits.iter().rev().collect()
}

pub fn solve(input: &str) -> (String, String) {
    let sum: i64 = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(snafu_to_decimal)
        .sum();

    let p1 = decimal_to_snafu(sum);
    let p2 = "Merry Christmas!".to_string();
    (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(snafu_to_decimal("1=-0-2"), 1747);
        assert_eq!(snafu_to_decimal("12111"), 906);
        assert_eq!(snafu_to_decimal("2=0="), 198);
        assert_eq!(snafu_to_decimal("21"), 11);
        assert_eq!(snafu_to_decimal("2=01"), 201);
        assert_eq!(snafu_to_decimal("111"), 31);
        assert_eq!(snafu_to_decimal("20012"), 1257);
        assert_eq!(snafu_to_decimal("112"), 32);
        assert_eq!(snafu_to_decimal("1=-1="), 353);
        assert_eq!(snafu_to_decimal("1-12"), 107);
        assert_eq!(snafu_to_decimal("12"), 7);
        assert_eq!(snafu_to_decimal("1="), 3);
        assert_eq!(snafu_to_decimal("122"), 37);
    }

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!(decimal_to_snafu(1), "1");
        assert_eq!(decimal_to_snafu(2), "2");
        assert_eq!(decimal_to_snafu(3), "1=");
        assert_eq!(decimal_to_snafu(4), "1-");
        assert_eq!(decimal_to_snafu(5), "10");
        assert_eq!(decimal_to_snafu(6), "11");
        assert_eq!(decimal_to_snafu(7), "12");
        assert_eq!(decimal_to_snafu(8), "2=");
        assert_eq!(decimal_to_snafu(9), "2-");
        assert_eq!(decimal_to_snafu(10), "20");
        assert_eq!(decimal_to_snafu(15), "1=0");
        assert_eq!(decimal_to_snafu(20), "1-0");
        assert_eq!(decimal_to_snafu(2022), "1=11-2");
        assert_eq!(decimal_to_snafu(12345), "1-0---0");
        assert_eq!(decimal_to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn test_example() {
        let (p1, _) = solve(EXAMPLE);
        assert_eq!(p1, "2=-1=0");
    }
}
