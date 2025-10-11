use aoc2015::prelude::*;

#[test]
fn day01_example_works() {
    let input = include_str!("../inputs/day01_example.txt");
    let (p1, p2) = run_day(1, input).unwrap();
    // For AoC 2015 Day 1 with the given input:
    // Part 1: Final floor Santa ends up on
    // Part 2: Position of first character that causes basement entry
    assert_eq!(p1, "138");
    assert_eq!(p2, "1771");
}
