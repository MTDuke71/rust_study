// Day 08 Tests

use aoc2015::solver::day08::{solve_part1, solve_part2};

#[test]
fn test_part1_placeholder() {
    let input = r#""hello""#;
    let result = solve_part1(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0");
}

#[test]
fn test_part2_placeholder() {
    let input = r#""hello""#;
    let result = solve_part2(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0");
}

#[test]
fn test_empty_input() {
    let input = "";
    assert!(solve_part1(input).is_ok());
    assert!(solve_part2(input).is_ok());
}
