use aoc2015::solver::day11::*;

// ============================================================================
// Increment Password Tests
// ============================================================================

#[test]
fn test_increment_simple() {
    assert_eq!(increment_password("xx"), "xy");
    assert_eq!(increment_password("xy"), "xz");
}

#[test]
fn test_increment_wrap_single() {
    assert_eq!(increment_password("xz"), "ya");
}

#[test]
fn test_increment_wrap_multiple() {
    assert_eq!(increment_password("azz"), "baa");
    assert_eq!(increment_password("zzz"), "aaa"); // Full wrap
}

// ============================================================================
// Increasing Straight Tests
// ============================================================================

#[test]
fn test_has_straight_basic() {
    assert!(has_increasing_straight("abc"));
    assert!(has_increasing_straight("bcd"));
    assert!(has_increasing_straight("xyz"));
}

#[test]
fn test_has_straight_embedded() {
    assert!(has_increasing_straight("hijklmmn")); // Contains "hij"
    assert!(has_increasing_straight("abcdffaa")); // Contains "abc"
    assert!(has_increasing_straight("xxabcyy")); // Contains "abc"
}

#[test]
fn test_no_straight() {
    assert!(!has_increasing_straight("abd")); // Skips 'c'
    assert!(!has_increasing_straight("abbceffg")); // No consecutive run
    assert!(!has_increasing_straight("aabccdef")); // No 3-letter straight
}

// ============================================================================
// Forbidden Characters Tests
// ============================================================================

#[test]
fn test_no_forbidden_chars_valid() {
    assert!(has_no_forbidden_chars("abcdefgh"));
    assert!(has_no_forbidden_chars("ghjaabcc"));
    assert!(has_no_forbidden_chars("abcdffaa"));
}

#[test]
fn test_has_forbidden_i() {
    assert!(!has_no_forbidden_chars("hijklmmn"));
    assert!(!has_no_forbidden_chars("ghijklmn"));
    assert!(!has_no_forbidden_chars("xxxixxxx"));
}

#[test]
fn test_has_forbidden_o() {
    assert!(!has_no_forbidden_chars("abcdefgo"));
    assert!(!has_no_forbidden_chars("oxxxxxxx"));
}

#[test]
fn test_has_forbidden_l() {
    assert!(!has_no_forbidden_chars("abcdefgl"));
    assert!(!has_no_forbidden_chars("lxxxxxxx"));
}

// ============================================================================
// Two Pairs Tests
// ============================================================================

#[test]
fn test_has_two_pairs_basic() {
    assert!(has_two_pairs("aabb"));
    assert!(has_two_pairs("xxyy"));
}

#[test]
fn test_has_two_pairs_embedded() {
    assert!(has_two_pairs("abbceffg")); // bb and ff
    assert!(has_two_pairs("ghjaabcc")); // aa and cc (or bb and cc)
    assert!(has_two_pairs("aabcdefgg")); // aa and gg
}

#[test]
fn test_has_two_pairs_non_overlapping() {
    assert!(has_two_pairs("aaaa")); // Two pairs: aa at 0-1 and aa at 2-3
    assert!(has_two_pairs("aabaa")); // aa at 0-1 and aa at 3-4
}

#[test]
fn test_not_enough_pairs() {
    assert!(!has_two_pairs("abbcegjk")); // Only bb
    assert!(!has_two_pairs("aaa")); // Only one pair (aa)
    assert!(!has_two_pairs("abcdefgh")); // No pairs
}

// ============================================================================
// Full Password Validation Tests
// ============================================================================

#[test]
fn test_example_failures() {
    // hijklmmn: has straight (hij) but contains i and l
    assert!(!is_valid_password("hijklmmn"));

    // abbceffg: has pairs (bb, ff) but no straight
    assert!(!is_valid_password("abbceffg"));

    // abbcegjk: only one pair (bb)
    assert!(!is_valid_password("abbcegjk"));
}

#[test]
fn test_example_successes() {
    // abcdffaa: has straight (abc), pairs (ff, aa), no forbidden chars
    assert!(is_valid_password("abcdffaa"));

    // ghjaabcc: has straight (should have), pairs (aa, bb or cc), no forbidden
    // Note: Verify this actually has a straight!
    assert!(is_valid_password("ghjaabcc"));
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_edge_case_all_z() {
    // What happens when incrementing "zzzzzzzz"?
    // Should wrap to "aaaaaaaa"
    assert_eq!(increment_password("zzzzzzzz"), "aaaaaaaa");
}

#[test]
fn test_edge_case_single_pair() {
    // Password with only one pair should fail
    assert!(!is_valid_password("abcdefaa"));
}

#[test]
fn test_edge_case_three_in_row() {
    // "aaa" should count as one pair, not two
    // Wait... actually "aaa" contains two overlapping pairs!
    // But based on the problem: pairs must be non-overlapping
    // So "aaa" should have: one pair at position 0-1, or one pair at 1-2
    // Let's verify the problem statement interpretation
    assert!(!has_two_pairs("aaa")); // Only one valid non-overlapping pair
}

// ============================================================================
// Integration Tests (Finding Next Valid Password)
// ============================================================================

#[test]
#[ignore] // Remove #[ignore] once solve_part1 is implemented
fn test_find_next_password_example1() {
    // abcdefgh → next valid should be abcdffaa
    let result = solve_part1("abcdefgh").unwrap();
    assert_eq!(result, "abcdffaa");
}

#[test]
#[ignore] // Remove #[ignore] once solve_part1 is implemented
fn test_find_next_password_example2() {
    // ghijklmn → next valid should be ghjaabcc
    let result = solve_part1("ghijklmn").unwrap();
    assert_eq!(result, "ghjaabcc");
}
