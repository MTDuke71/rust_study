//! Day 4: High-Entropy Passphrases
//!
//! Part 1: Count passphrases containing no duplicate words.
//! Part 2: Count passphrases where no two words are anagrams of each other.

use std::collections::HashSet;

type Passphrase<'a> = Vec<&'a str>;

fn parse_input(input: &str) -> Vec<Passphrase<'_>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_whitespace().collect())
        .collect()
}

fn has_no_duplicates(phrase: &[&str]) -> bool {
    let mut seen = HashSet::with_capacity(phrase.len());
    phrase.iter().all(|word| seen.insert(*word))
}

fn has_no_anagrams(phrase: &[&str]) -> bool {
    let mut seen = HashSet::with_capacity(phrase.len());
    phrase.iter().all(|word| {
        let mut sorted: Vec<u8> = word.bytes().collect();
        sorted.sort_unstable();
        seen.insert(sorted)
    })
}

fn solve_part1_with_data(data: &[Passphrase<'_>]) -> usize {
    data.iter().filter(|p| has_no_duplicates(p)).count()
}

fn solve_part2_with_data(data: &[Passphrase<'_>]) -> usize {
    data.iter().filter(|p| has_no_anagrams(p)).count()
}

pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}

pub fn solve_part1(input: &str) -> usize {
    let data = parse_input(input);
    solve_part1_with_data(&data)
}

pub fn solve_part2(input: &str) -> usize {
    let data = parse_input(input);
    solve_part2_with_data(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert!(has_no_duplicates(&["aa", "bb", "cc", "dd", "ee"]));
        assert!(!has_no_duplicates(&["aa", "bb", "cc", "dd", "aa"]));
        assert!(has_no_duplicates(&["aa", "bb", "cc", "dd", "aaa"]));
    }

    #[test]
    fn test_part2_examples() {
        assert!(has_no_anagrams(&["abcde", "fghij"]));
        assert!(!has_no_anagrams(&["abcde", "xyz", "ecdab"]));
        assert!(has_no_anagrams(&["a", "ab", "abc", "abd", "abf", "abj"]));
        assert!(has_no_anagrams(&["iiii", "oiii", "ooii", "oooi", "oooo"]));
        assert!(!has_no_anagrams(&["oiii", "ioii", "iioi", "iiio"]));
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/day04.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, 325);
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/day04.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, 119);
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/day04.txt");
        assert_eq!(solve(input), (325, 119));
    }
}
