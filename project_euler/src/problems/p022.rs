//! # Problem 22: Names Scores
//!
//! Using a text file containing over five-thousand first names, sort them
//! into alphabetical order. Then, working out the alphabetical value of
//! each name, multiply this value by its alphabetical position in the list
//! to obtain a name score.
//!
//! What is the total of all the name scores in the file?
//!
//! ## Mathematical Foundation
//!
//! Alphabetical value: map each letter to its ordinal position (A=1, ..., Z=26),
//! then sum. The name score is `alphabetical_value × sorted_position`.
//!
//! Example: COLIN → 3+15+12+9+14 = 53, at position 938 → 53 × 938 = 49714.
//!
//! See [[math-foundations/project-euler-p022]] for full analysis.
//!
//! ## Complexity
//!
//! - Time: O(n log n) for sorting, O(n × k) for scoring (k = avg name length)
//! - Space: O(n) for storing names

/// Compute the alphabetical value of a name.
///
/// Each letter is worth its position in the alphabet (A=1, B=2, ..., Z=26).
///
/// # Examples
/// ```
/// use project_euler::problems::p022::alphabetical_value;
/// assert_eq!(alphabetical_value("COLIN"), 53);
/// ```
pub fn alphabetical_value(name: &str) -> u64 {
    name.bytes()
        .filter(|b| b.is_ascii_uppercase())
        .map(|b| (b - b'A' + 1) as u64)
        .sum()
}

/// Parse comma-separated quoted names from a string.
///
/// Input format: `"NAME1","NAME2","NAME3"`
///
/// # Examples
/// ```
/// use project_euler::problems::p022::parse_names;
/// let names = parse_names(r#""ALICE","BOB","CAROL""#);
/// assert_eq!(names, vec!["ALICE", "BOB", "CAROL"]);
/// ```
pub fn parse_names(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_string())
        .collect()
}

/// Compute the total name score for a list of names.
///
/// Names are sorted alphabetically, then each name's score is
/// `alphabetical_value × (1-based position)`.
///
/// # Examples
/// ```
/// use project_euler::problems::p022::total_name_score;
/// let input = r#""COLIN","MARY","ALICE""#;
/// let total = total_name_score(input);
/// // Sorted: ALICE(1), COLIN(2), MARY(3)
/// // ALICE: (1+12+9+3+5) × 1 = 30
/// // COLIN: (3+15+12+9+14) × 2 = 106
/// // MARY:  (13+1+18+25) × 3 = 171
/// assert_eq!(total, 30 + 106 + 171);
/// ```
pub fn total_name_score(input: &str) -> u64 {
    let mut names = parse_names(input);
    names.sort();

    names
        .iter()
        .enumerate()
        .map(|(i, name)| alphabetical_value(name) * (i as u64 + 1))
        .sum()
}

/// Solve Problem 22 with the embedded names file.
pub fn solve() -> u64 {
    let input = include_str!("../../Problem_Statements/0022_names.txt");
    total_name_score(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabetical_value_colin() {
        // C=3, O=15, L=12, I=9, N=14 → 53
        assert_eq!(alphabetical_value("COLIN"), 53);
    }

    #[test]
    fn test_alphabetical_value_a() {
        assert_eq!(alphabetical_value("A"), 1);
    }

    #[test]
    fn test_alphabetical_value_z() {
        assert_eq!(alphabetical_value("Z"), 26);
    }

    #[test]
    fn test_parse_names() {
        let names = parse_names(r#""ALICE","BOB""#);
        assert_eq!(names, vec!["ALICE", "BOB"]);
    }

    #[test]
    fn test_total_name_score_small() {
        let input = r#""COLIN","MARY","ALICE""#;
        // Sorted: ALICE(1), COLIN(2), MARY(3)
        let alice = (1 + 12 + 9 + 3 + 5) * 1; // 30
        let colin = (3 + 15 + 12 + 9 + 14) * 2; // 106
        let mary = (13 + 1 + 18 + 25) * 3; // 171
        assert_eq!(total_name_score(input), alice + colin + mary);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 871198282);
    }
}
