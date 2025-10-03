// Day 08 Tests
// https://adventofcode.com/2015/day/8

use aoc2015::solver::day08::{solve_part1, solve_part2};

// ============================================================================
// Part 1 Tests: Code Length vs Memory Length
// ============================================================================

#[test]
fn test_part1_empty_string() {
    // "" is 2 characters of code (the quotes), but 0 characters in memory
    // Input file format includes the quotes
    let input = r#""""#;
    let result = solve_part1(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "2");
}

#[test]
fn test_part1_simple_string() {
    // "abc" is 5 characters of code (quotes + abc), but 3 characters in memory
    // Input file format includes the quotes
    let input = r#""abc""#;
    let result = solve_part1(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "2");
}

#[test]
fn test_part1_escaped_quote() {
    // "aaa\"aaa" is 10 characters of code, but 7 characters in memory
    // Input file format includes the outer quotes
    let input = r#""aaa\"aaa""#;
    let result = solve_part1(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "3");
}

#[test]
fn test_part1_hex_escape() {
    // "\x27" is 6 characters of code, but 1 character in memory (')
    // Input file format includes the outer quotes
    let input = r#""\x27""#;
    let result = solve_part1(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "5");
}

#[test]
fn test_part1_escaped_backslash() {
    // "\\" is 4 characters of code, but 1 character in memory (\)
    // Input file format includes the outer quotes
    let input = r#""\\""#;
    let result = solve_part1(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "3");
}

#[test]
fn test_part1_example_from_problem() {
    // The full example from the problem statement
    // "" -> 2 code - 0 memory = 2
    // "abc" -> 5 code - 3 memory = 2
    // "aaa\"aaa" -> 10 code - 7 memory = 3
    // "\x27" -> 6 code - 1 memory = 5
    // Total: 23 code - 11 memory = 12
    // Input file format includes the outer quotes on each line
    let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
    let result = solve_part1(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "12");
}

#[test]
fn test_part1_multiple_escapes() {
    // "a\"b\\c\x41" with outer quotes in input
    // Code length: 13
    // Memory: a"b\cA = 6 characters (after unescaping and stripping quotes)
    // But current impl: unescapes to "a"b\cA" (8 chars with quotes), then -2 = 6
    // Difference: 13 - 6 = 7
    let input = r#""a\"b\\c\x41""#;
    let result = solve_part1(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "7");
}

#[test]
fn test_part1_multiple_hex_escapes() {
    // "\x61\x62\x63" = "abc" in memory (with quotes in input)
    // Code length: 14
    // unescapes to "abc" (5 chars with quotes), then -2 = 3
    // Difference: 14 - 3 = 11
    let input = r#""\x61\x62\x63""#;
    let result = solve_part1(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "11");
}

#[test]
fn test_part1_multiple_lines() {
    // Two simple strings with quotes in input
    // "abc" -> 5 code - 3 memory = 2
    // "test" -> 6 code - 4 memory = 2
    // Total difference: 4
    let input = r#""abc"
"test""#;
    let result = solve_part1(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "4");
}

#[test]
fn test_part1_empty_input() {
    let input = "";
    let result = solve_part1(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0");
}

#[test]
fn test_part1_whitespace_handling() {
    // Test with empty strings and whitespace-only lines
    // "" -> 2 code - 0 memory = 2
    // "abc" -> 5 code - 3 memory = 2
    // (whitespace line is skipped by trim/is_empty)
    // "test" -> 6 code - 4 memory = 2
    // Total: 6
    let input = r#"""
"abc"
   
"test""#;
    let result = solve_part1(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "6");
}

// ============================================================================
// Part 2 Tests: String Encoding
// ============================================================================

#[test]
fn test_part2_empty_string() {
    // "" encodes to "\"\"", an increase from 2 to 6 (+4)
    let input = r#""""#;
    let result = solve_part2(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "4");
}

#[test]
fn test_part2_simple_string() {
    // "abc" encodes to "\"abc\"", an increase from 5 to 9 (+4)
    let input = r#""abc""#;
    let result = solve_part2(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "4");
}

#[test]
fn test_part2_escaped_quote() {
    // "aaa\"aaa" encodes to "\"aaa\\\"aaa\"", an increase from 10 to 16 (+6)
    let input = r#""aaa\"aaa""#;
    let result = solve_part2(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "6");
}

#[test]
fn test_part2_hex_escape() {
    // "\x27" encodes to "\"\\x27\"", an increase from 6 to 11 (+5)
    let input = r#""\x27""#;
    let result = solve_part2(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "5");
}

#[test]
fn test_part2_example_from_problem() {
    // The full example from the problem statement
    // "" -> increase of 4
    // "abc" -> increase of 4
    // "aaa\"aaa" -> increase of 6
    // "\x27" -> increase of 5
    // Total increase: 19
    let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
    let result = solve_part2(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "19");
}

#[test]
fn test_part2_escaped_backslash() {
    // "\\" has 1 backslash in original (4 chars total)
    // Encoded: "\"\\\\"" (8 chars: quote + backslash + quote + backslash + backslash + backslash + backslash + quote)
    // Wait, let me think through this more carefully
    // Original string in file: "\\" (4 characters: quote + backslash + backslash + quote)
    // To encode this, we need to escape each character:
    // - quote becomes \"
    // - backslash becomes \\
    // - backslash becomes \\
    // - quote becomes \"
    // Wrapped in quotes: "\"\\\\\"" (10 chars)
    // Increase: 10 - 4 = 6
    let input = r#""\\""#;
    let result = solve_part2(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "6");
}

#[test]
fn test_part2_multiple_lines() {
    // "abc" -> 5 to 9 (+4)
    // "test" -> 6 to 10 (+4)
    // Total increase: 8
    let input = r#""abc"
"test""#;
    let result = solve_part2(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "8");
}
