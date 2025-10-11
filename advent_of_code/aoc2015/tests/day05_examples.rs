use aoc2015::solver::day05::{solve_part1, solve_part2};

#[test]
fn day05_part1_example() {
    let test_input = "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";

    // These are the examples from the problem statement
    // ugknbfddgicrmopn is nice (3 vowels, dd double letter, no forbidden)
    // aaa is nice (3 vowels, aa double letter, no forbidden)
    // jchzalrnumimnmhp is naughty (no double letter)
    // haegwjzuvuyypxyu is naughty (contains xy)
    // dvszwmarrgswjxmb is naughty (only 1 vowel)

    let result = solve_part1(test_input).unwrap();
    assert_eq!(result, "2"); // Should find 2 nice strings
}

#[test]
fn day05_part2_example() {
    // Part 2 examples from AoC problem:
    let test_cases = vec![
        ("qjhvhtzxzqqjkmpb", true),  // nice: qj...qj and zxz
        ("xxyxx", true),             // nice: xx...xx and xyx
        ("uurcxstgmygtbstg", false), // naughty: no pair without overlapping
        ("ieodomkazucvgmuy", false), // naughty: no letter repeating with one between
    ];

    for (input, expected) in test_cases {
        let result = solve_part2(input).unwrap();
        let expected_str = if expected { "1" } else { "0" };
        assert_eq!(result, expected_str, "Failed for input: {}", input);
    }
}

#[test]
fn day05_part2_detailed_tests() {
    // Test individual rules and combinations

    // These should be NICE (satisfy both rules)
    assert_eq!(solve_part2("xyxy").unwrap(), "1"); // xy pair + xyx pattern
    assert_eq!(solve_part2("xyxxy").unwrap(), "1"); // xy pair + xyx pattern
    assert_eq!(solve_part2("aabaa").unwrap(), "1"); // aa pair + aba pattern

    // These should be NAUGHTY (missing one or both rules)
    assert_eq!(solve_part2("aabcdefgaa").unwrap(), "0"); // Has aa twice but no letter with one between
    assert_eq!(solve_part2("aaa").unwrap(), "0"); // aa overlaps, no valid non-overlapping pair
    assert_eq!(solve_part2("xyx").unwrap(), "0"); // Has letter with one between but no pair
    assert_eq!(solve_part2("abcdefeghi").unwrap(), "0"); // Has e with one between but no pair
}
