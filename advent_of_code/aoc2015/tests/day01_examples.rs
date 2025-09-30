use aoc2015::prelude::*;

#[test]
fn day01_simple_examples() {
    // Test some simple examples from the problem description
    
    // Part 1 examples: 
    // "((" should end up on floor 2
    let (p1, _) = run_day(1, "((").unwrap();
    assert_eq!(p1, "2");
    
    // "))" should end up on floor -2  
    let (p1, _) = run_day(1, "))").unwrap();
    assert_eq!(p1, "-2");
    
    // "(((" should end up on floor 3
    let (p1, _) = run_day(1, "(((").unwrap();
    assert_eq!(p1, "3");
    
    // "(()(()(" should end up on floor 2 (4 open, 2 close = +2)
    let (p1, _) = run_day(1, "(()(()()").unwrap();
    assert_eq!(p1, "2");
    
    // "))((((" should end up on floor 2 (4 open, 2 close = +2) 
    let (p1, _) = run_day(1, "))((((").unwrap();
    assert_eq!(p1, "2");
}

#[test] 
fn day01_basement_examples() {
    // Part 2 examples:
    // ")" should enter basement at position 1
    let (_, p2) = run_day(1, ")").unwrap();
    assert_eq!(p2, "1");
    
    // "()())" should enter basement at position 5
    let (_, p2) = run_day(1, "()())").unwrap();
    assert_eq!(p2, "5");
}

#[test]
fn day01_no_basement() {
    // Test case where Santa never enters the basement
    let (p1, p2) = run_day(1, "((((").unwrap();
    assert_eq!(p1, "4");
    assert_eq!(p2, "Never enters basement");
}