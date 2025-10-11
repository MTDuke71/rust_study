// Debug Day 5 Part 2 logic
use regex::Regex;

fn main() {
    println!("🔍 Debugging Day 5 Part 2 Logic");
    println!("================================");

    let test_cases = vec![
        "xyxy",             // Should have xy pair but no letter with one between -> false
        "aabcdefgaa",       // Should have aa pair but no letter with one between -> false
        "aaa",              // Should NOT have non-overlapping pair -> false
        "xyx",              // Should have letter with one between but no pair -> false
        "abcdefeghi",       // Should have e with one between but no pair -> false
        "xyxxy",            // Should have xy pair + xyx pattern -> true
        "aabaa",            // Should have aa pair + aba pattern -> true
        "qjhvhtzxzqqjkmpb", // AoC example: should be true
        "xxyxx",            // AoC example: should be true
    ];

    for test_str in test_cases {
        println!("\nTesting: '{}'", test_str);
        let has_pair = has_non_overlapping_pair(test_str);
        let has_between = has_letter_with_one_between(test_str);
        let is_nice = has_pair && has_between;

        println!("  Non-overlapping pair: {}", has_pair);
        println!("  Letter with one between: {}", has_between);
        println!("  → Nice: {}", is_nice);

        if has_pair {
            debug_pairs(test_str);
        }
        if has_between {
            debug_letter_between(test_str);
        }
    }
}

fn has_non_overlapping_pair(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len().saturating_sub(1) {
        let pair = (chars[i], chars[i + 1]);

        for j in (i + 2)..chars.len().saturating_sub(1) {
            if chars[j] == pair.0 && chars[j + 1] == pair.1 {
                return true;
            }
        }
    }

    false
}

fn has_letter_with_one_between(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len().saturating_sub(2) {
        if chars[i] == chars[i + 2] {
            return true;
        }
    }

    false
}

fn debug_pairs(s: &str) {
    let chars: Vec<char> = s.chars().collect();
    println!("    Found pairs:");

    for i in 0..chars.len().saturating_sub(1) {
        let pair = (chars[i], chars[i + 1]);

        for j in (i + 2)..chars.len().saturating_sub(1) {
            if chars[j] == pair.0 && chars[j + 1] == pair.1 {
                println!("      '{}{}' at positions {} and {}", pair.0, pair.1, i, j);
            }
        }
    }
}

fn debug_letter_between(s: &str) {
    let chars: Vec<char> = s.chars().collect();
    println!("    Found letter-with-one-between:");

    for i in 0..chars.len().saturating_sub(2) {
        if chars[i] == chars[i + 2] {
            println!(
                "      '{}' at positions {} and {} (with '{}' between)",
                chars[i],
                i,
                i + 2,
                chars[i + 1]
            );
        }
    }
}
