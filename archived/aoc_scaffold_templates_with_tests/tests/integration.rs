
use aoc_scaffold::prelude::*;

#[test]
fn day01_example_works() {
    let input = include_str!("../inputs/day01_example.txt");
    let (p1,p2) = run_day(1, input).unwrap();
    assert_eq!(p1, "24000");
    assert_eq!(p2, "45000");
}
