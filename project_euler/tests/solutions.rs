//! Project Euler Solutions Tests
//!
//! Test correct answers for implemented problems.

use project_euler::problems::p001;

#[test]
fn test_problem_001() {
    assert_eq!(p001::solve(), 233168);
}

#[test]
fn test_problem_001_example() {
    // Example from problem statement: below 10
    let result = p001::sum_multiples(3, 10) + p001::sum_multiples(5, 10) - p001::sum_multiples(15, 10);
    assert_eq!(result, 23);
}

