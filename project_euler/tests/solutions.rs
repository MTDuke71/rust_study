//! Project Euler Solutions Tests
//!
//! Test correct answers for implemented problems.

use project_euler::problems::{p001, p006, p007, p008, p009, p010, p011, p012, p013, p015, p016, p017, p018, p067};

#[test]
fn test_problem_001() {
    assert_eq!(p001::solve(), 233168);
}

#[test]
fn test_problem_001_example() {
    // Example from problem statement: below 10
    let result =
        p001::sum_multiples(3, 10) + p001::sum_multiples(5, 10) - p001::sum_multiples(15, 10);
    assert_eq!(result, 23);
}

#[test]
fn test_problem_006() {
    assert_eq!(p006::solve(), 25164150);
}

#[test]
fn test_problem_006_example() {
    // Example from problem statement: first 10 natural numbers
    assert_eq!(p006::sum_square_difference(10), 2640);
}

#[test]
fn test_problem_007() {
    assert_eq!(p007::solve(), 104743);
}

#[test]
fn test_problem_007_example() {
    // Example from problem statement: 6th prime is 13
    assert_eq!(p007::nth_prime(6), 13);
}

#[test]
fn test_problem_008() {
    assert_eq!(p008::solve(), 23514624000);
}

#[test]
fn test_problem_008_example() {
    // Example from problem statement: 4 adjacent digits = 9 × 9 × 8 × 9 = 5832
    // Testing with a simple string containing 9989 which has product 9 × 9 × 8 × 9 = 5832
    assert_eq!(p008::largest_product_in_series("9989", 4), 5832);
}

#[test]
fn test_problem_009() {
    assert_eq!(p009::solve(), 31875000);
}

#[test]
fn test_problem_009_example() {
    // Example from problem statement: 3 + 4 + 5 = 12
    let triplet = p009::find_pythagorean_triplet(12).unwrap();
    assert_eq!(triplet.a, 3);
    assert_eq!(triplet.b, 4);
    assert_eq!(triplet.c, 5);
    assert_eq!(triplet.product(), 60);
}

#[test]
fn test_problem_010() {
    assert_eq!(p010::solve(), 142913828922);
}

#[test]
fn test_problem_010_example() {
    // Example from problem statement: primes below 10 sum to 17
    assert_eq!(p010::sum_primes_below(10), 17);
}

#[test]
fn test_problem_011() {
    assert_eq!(p011::solve(), 70600674);
}

#[test]
fn test_problem_011_small_grid() {
    // Test with a small 4x4 grid
    let grid = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    assert_eq!(p011::largest_product_in_grid(&grid), 43680);
}

#[test]
fn test_problem_012() {
    assert_eq!(p012::solve(), 76576500);
}

#[test]
fn test_problem_012_example() {
    // Example from problem statement: first triangle with > 5 divisors is 28
    assert_eq!(p012::first_triangle_with_divisors(5), 28);
}

#[test]
fn test_problem_013() {
    assert_eq!(p013::solve(), 5537376230);
}

#[test]
fn test_problem_013_string() {
    let result = p013::first_ten_digits_of_sum();
    assert_eq!(result, "5537376230");
    assert_eq!(result.len(), 10);
}

#[test]
fn test_problem_015() {
    assert_eq!(p015::solve(), 137846528820);
}

#[test]
fn test_problem_015_example() {
    // Example from problem statement: 6 routes through 2×2 grid
    assert_eq!(p015::lattice_paths(2), 6);
}

#[test]
fn test_problem_016() {
    assert_eq!(p016::solve(), 1366);
}

#[test]
fn test_problem_016_example() {
    // Example from problem statement: 2^15 = 32768, digit sum = 26
    assert_eq!(p016::power_digit_sum(2, 15), 26);
}

#[test]
fn test_problem_017() {
    assert_eq!(p017::solve(), 21124);
}

#[test]
fn test_problem_017_example() {
    // Example from problem statement: 1–5 = 3+3+5+4+4 = 19 letters
    assert_eq!(p017::total_letter_count(5), 19);
}

#[test]
fn test_problem_018() {
    assert_eq!(p018::solve(), 1074);
}

#[test]
fn test_problem_018_example() {
    // Example from problem statement: 4-row triangle, max path = 23
    let mut t = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
    assert_eq!(p018::max_path_sum(&mut t), 23);
}

#[test]
fn test_problem_067() {
    assert_eq!(p067::solve(), 7273);
}
