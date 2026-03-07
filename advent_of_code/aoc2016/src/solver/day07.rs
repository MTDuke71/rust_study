//! Day 7: Internet Protocol Version 7
//!
//! Determine which IPv7 addresses support TLS and SSL.
//! Part 1: Count IPs with ABBA outside brackets but not inside.
//! Part 2: Count IPs supporting SSL (ABA outside, corresponding BAB inside).

fn has_abba(s: &[u8]) -> bool {
    s.windows(4).any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

fn supports_tls(line: &str) -> bool {
    let bytes = line.as_bytes();
    let mut in_bracket = false;
    let mut abba_outside = false;
    let mut start = 0;

    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'[' => {
                if !in_bracket && has_abba(&bytes[start..i]) {
                    abba_outside = true;
                }
                in_bracket = true;
                start = i + 1;
            }
            b']' => {
                if in_bracket && has_abba(&bytes[start..i]) {
                    return false;
                }
                in_bracket = false;
                start = i + 1;
            }
            _ => {}
        }
    }
    if !in_bracket && has_abba(&bytes[start..]) {
        abba_outside = true;
    }
    abba_outside
}

fn supports_ssl(line: &str) -> bool {
    let bytes = line.as_bytes();
    let mut supernets: Vec<&[u8]> = Vec::new();
    let mut hypernets: Vec<&[u8]> = Vec::new();
    let mut in_bracket = false;
    let mut start = 0;

    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'[' => {
                supernets.push(&bytes[start..i]);
                in_bracket = true;
                start = i + 1;
            }
            b']' => {
                hypernets.push(&bytes[start..i]);
                in_bracket = false;
                start = i + 1;
            }
            _ => {}
        }
    }
    if !in_bracket {
        supernets.push(&bytes[start..]);
    }

    for sn in &supernets {
        for w in sn.windows(3) {
            if w[0] == w[2] && w[0] != w[1] {
                let bab = [w[1], w[0], w[1]];
                if hypernets.iter().any(|hn| hn.windows(3).any(|h| h == bab)) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn solve(input: &str) -> (String, String) {
    let p1 = input.lines().filter(|l| supports_tls(l)).count();
    let p2 = input.lines().filter(|l| supports_ssl(l)).count();
    (p1.to_string(), p2.to_string())
}

pub fn solve_part1(input: &str) -> String {
    input.lines().filter(|l| supports_tls(l)).count().to_string()
}

pub fn solve_part2(input: &str) -> String {
    input.lines().filter(|l| supports_ssl(l)).count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_examples() {
        assert!(supports_tls("abba[mnop]qrst"));
        assert!(!supports_tls("abcd[bddb]xyyx"));
        assert!(!supports_tls("aaaa[qwer]tyui"));
        assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn test_ssl_examples() {
        assert!(supports_ssl("aba[bab]xyz"));
        assert!(!supports_ssl("xyx[xyx]xyx"));
        assert!(supports_ssl("aaa[kek]eke"));
        assert!(supports_ssl("zazbz[bzb]cdb"));
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day07.txt");
        assert_eq!(solve_part1(input), "118");
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day07.txt");
        assert_eq!(solve_part2(input), "260");
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day07.txt");
        assert_eq!(solve(input), ("118".to_string(), "260".to_string()));
    }
}
