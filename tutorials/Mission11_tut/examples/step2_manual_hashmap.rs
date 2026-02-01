//! Step 2: Manual HashMap Memoization - Transform Exponential to Linear
//!
//! **Learning Objective**: By the end of this step, you will understand how to add
//! memoization with HashMap and observe the dramatic performance transformation.
//!
//! **Prerequisites**: Step 1 (naive recursion), HashMap basics (Mission 5)
//!
//! **Next Step Preview**: Step 3 will address lifetime safety issues that arise
//! when using borrowed strings as HashMap keys.

use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("=== Step 2: Manual HashMap Memoization ===\n");

    demonstrate_core_concept();
    show_common_patterns();
    handle_edge_cases();
    validate_understanding();

    println!("\n✅ Step 2 completed! Ready for Step 3.");
    println!("💡 Key Insight: Memoization transforms exponential → linear with minimal code!");
}

/// Core concept demonstration
///
/// Shows how adding a HashMap cache eliminates redundant computation.
/// Key learning: Cache lookup before computing, cache result before returning.
fn demonstrate_core_concept() {
    println!("--- Core Concept: Adding Memoization ---");

    let patterns = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
    let design = "brwrrggbrwrr"; // Longer than Step 1

    println!("Problem: Can we make '{}' from patterns?", design);
    println!("Design length: {} chars", design.len());

    // Without memoization (from Step 1)
    println!("\nWithout memoization:");
    let start = Instant::now();
    let result_naive = can_make_naive(&patterns, design);
    let duration_naive = start.elapsed();
    println!("  Result: {}, Time: {:?}", result_naive, duration_naive);

    // With memoization
    println!("\nWith memoization:");
    let mut memo = HashMap::new();
    let start = Instant::now();
    let result_memo = can_make_memo(&patterns, design, &mut memo);
    let duration_memo = start.elapsed();
    println!("  Result: {}, Time: {:?}", result_memo, duration_memo);
    println!("  Cache size: {} entries", memo.len());

    if duration_naive.as_micros() > 0 {
        let speedup = duration_naive.as_micros() as f64 / duration_memo.as_micros().max(1) as f64;
        println!("  Speedup: {:.1}x faster!", speedup);
    }

    println!("\n⚠️  Note: At 12 chars, memoization appears SLOWER due to HashMap overhead!");
    println!("   For small inputs, cache operations cost more than they save.");
    println!(
        "   Real benefits appear with longer designs (15+ chars) - see validation section below."
    );

    println!("\nKey Insight: Cache stores results for each unique substring");
    println!("  Before computing: Check if we've seen this substring before");
    println!("  After computing: Store result so we never recompute it");
}

/// Naive recursion from Step 1 (for comparison)
fn can_make_naive(patterns: &[&str], design: &str) -> bool {
    if design.is_empty() {
        return true;
    }

    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            if can_make_naive(patterns, rest) {
                return true;
            }
        }
    }
    false
}

/// Memoized version with HashMap cache
///
/// Key changes from naive version:
/// 1. Added HashMap parameter to store results
/// 2. Check cache before computing
/// 3. Store result in cache before returning
fn can_make_memo<'a>(
    patterns: &[&str],
    design: &'a str,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    // Base case: empty string
    if design.is_empty() {
        return true;
    }

    // Check cache first - O(1) lookup
    if let Some(&result) = memo.get(design) {
        return result; // Cache hit! No computation needed
    }

    // Cache miss - need to compute
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            if can_make_memo(patterns, rest, memo) {
                memo.insert(design, true); // Cache before returning
                return true;
            }
        }
    }

    // No solution found - cache negative result too!
    memo.insert(design, false);
    false
}

/// Common usage patterns
///
/// Shows the pattern applied to different scenarios.
fn show_common_patterns() {
    println!("\n--- Common Patterns ---");

    pattern_1_cache_statistics();
    pattern_2_progressive_difficulty();
    pattern_3_cache_reuse();
}

fn pattern_1_cache_statistics() {
    println!("\nPattern 1: Measuring cache effectiveness");

    let patterns = vec!["r", "wr", "b", "g", "br"];
    let design = "bwrrrbgbrbr";

    let mut memo = HashMap::new();
    let result = can_make_memo(&patterns, design, &mut memo);

    println!("  Design: '{}' (length {})", design, design.len());
    println!("  Result: {}", result);
    println!("  Cache entries: {}", memo.len());
    println!(
        "  Max possible substrings: {}",
        design.len() * (design.len() + 1) / 2
    );
    println!(
        "  Cache efficiency: {:.1}%",
        100.0 * memo.len() as f64 / (design.len() * (design.len() + 1) / 2) as f64
    );
}

fn pattern_2_progressive_difficulty() {
    println!("\nPattern 2: Performance scales with design length");

    let patterns = vec!["r", "wr", "b", "g", "br", "rb"];

    let test_cases = vec![
        "brwrr",            // 5 chars
        "bwrrrbgbr",        // 9 chars
        "rbbrwgbrwrrbw",    // 13 chars
        "bwrrrgbrwrrbwgbr", // 16 chars (would timeout without memo!)
    ];

    for design in test_cases {
        let mut memo = HashMap::new();
        let start = Instant::now();
        let result = can_make_memo(&patterns, design, &mut memo);
        let duration = start.elapsed();

        println!(
            "  Length {}: {:?}, cache size: {}, result: {}",
            design.len(),
            duration,
            memo.len(),
            result
        );
    }
}

fn pattern_3_cache_reuse() {
    println!("\nPattern 3: Reusing cache across multiple queries");

    let patterns = vec!["r", "wr", "b"];
    let mut memo = HashMap::new();

    let designs = vec!["brwr", "wrb", "brwrb"];

    println!("  Solving multiple designs with shared cache:");
    for design in designs {
        let cache_size_before = memo.len();
        let result = can_make_memo(&patterns, design, &mut memo);
        let cache_size_after = memo.len();

        println!(
            "    '{}': {} (cache grew by {} entries)",
            design,
            result,
            cache_size_after - cache_size_before
        );
    }

    println!(
        "  Final cache size: {} (contains results for all substrings seen)",
        memo.len()
    );
}

/// Edge case handling
///
/// Demonstrates how memoization handles special cases.
fn handle_edge_cases() {
    println!("\n--- Edge Cases ---");

    empty_design_cached();
    impossible_design_cached();
    cache_negative_results();
}

fn empty_design_cached() {
    println!("\nEdge Case 1: Empty design (base case, not cached)");

    let patterns = vec!["r"];
    let mut memo = HashMap::new();

    let result = can_make_memo(&patterns, "", &mut memo);
    println!("  Result: {}", result);
    println!(
        "  Cache size: {} (empty string returns immediately)",
        memo.len()
    );
    assert_eq!(memo.len(), 0); // Empty string doesn't enter cache
}

fn impossible_design_cached() {
    println!("\nEdge Case 2: Impossible design (negative results cached too)");

    let patterns = vec!["r", "wr"];
    let mut memo = HashMap::new();

    let result = can_make_memo(&patterns, "xyz", &mut memo);
    println!("  Result: {}", result);
    println!("  Cache entries: {:?}", memo);

    // Cache contains false for "xyz", "yz", "z"
    assert!(memo.contains_key("xyz"));
    assert_eq!(memo.get("xyz"), Some(&false));
}

fn cache_negative_results() {
    println!("\nEdge Case 3: Importance of caching negative results");

    let patterns = vec!["a", "aa"];
    let design = "aaaaab"; // Impossible due to 'b'

    let mut memo = HashMap::new();
    let start = Instant::now();
    let result = can_make_memo(&patterns, design, &mut memo);
    let duration = start.elapsed();

    println!("  Design: '{}' (impossible)", design);
    println!("  Result: {}", result);
    println!("  Time: {:?}", duration);
    println!("  Cache size: {}", memo.len());
    println!("  Without caching false, we'd recompute impossible suffixes!");
}

/// Understanding validation
///
/// Exercises to validate memoization understanding.
fn validate_understanding() {
    println!("\n--- Validation Exercises ---");

    println!("\n🤔 Predict cache behavior before running:");

    // Scenario 1: How many cache entries?
    println!("\nScenario 1: Cache size prediction");
    let patterns1 = vec!["ab"];
    let design1 = "abab";
    println!("  Design: '{}', patterns: {:?}", design1, patterns1);
    println!("  Prediction: How many cache entries? _____");

    let mut memo1 = HashMap::new();
    can_make_memo(&patterns1, design1, &mut memo1);
    println!("  Actual cache size: {}", memo1.len());
    println!("  Cache contents: {:?}", memo1);

    // Scenario 2: Cache hit rate
    println!("\nScenario 2: Multiple calls with same substring");
    let patterns2 = vec!["r", "rr"];
    let design2 = "rrrr";
    println!("  Design: '{}' (many repeated substrings)", design2);

    let mut memo2 = HashMap::new();
    can_make_memo(&patterns2, design2, &mut memo2);
    println!(
        "  Cache entries: {} (note: 'rr' appears multiple times in recursion)",
        memo2.len()
    );

    println!("\n✅ If you understood cache behavior, you grasp memoization!");
    println!("🔄 If confused, review the demonstrate_core_concept() section.");

    // Performance demonstration
    println!("\n--- Performance Reality Check ---");
    demonstrate_exponential_vs_linear();
}

/// Demonstrates how memoization transforms exponential → linear complexity
///
/// Uses the same test cases from Step 1 that showed exponential growth.
/// Now watch how memoization makes them trivial!
fn demonstrate_exponential_vs_linear() {
    // Same patterns as Step 1: force maximum backtracking on FAILURE
    let patterns = vec!["r", "rr", "rrr"];

    println!("\nComparing naive vs memoized with exponential-forcing patterns:");
    println!("💡 Patterns: ['r', 'rr', 'rrr'] with 'rrr...x' designs (that FAIL!)");
    println!("   These designs force trying ALL r-combinations before failing on 'x'");
    println!("   Without memoization: exponential (Fibonacci growth ~6,765 calls at length 20!)");
    println!("   With memoization: linear (each substring computed only ONCE!)");
    println!();

    // Start with smaller sizes that naive can still handle
    let test_sizes = vec![
        ("rrrrx", 5),
        ("rrrrrrx", 7),
        ("rrrrrrrrx", 9),
        ("rrrrrrrrrrx", 11),
        ("rrrrrrrrrrrrx", 13),
        ("rrrrrrrrrrrrrrx", 15),
        ("rrrrrrrrrrrrrrrrx", 17),
        ("rrrrrrrrrrrrrrrrrrx", 19),
        ("rrrrrrrrrrrrrrrrrrrrx", 21), // Naive starts to struggle
    ];

    println!("Length | Naive Time     | Memo Time      | Speedup  | Cache");
    println!("-------|----------------|----------------|----------|-------");

    let mut last_cache_size = 0;
    for (design, length) in &test_sizes {
        // Naive version - will get exponentially slower
        let start_naive = Instant::now();
        let result_naive = can_make_naive(&patterns, design);
        let time_naive = start_naive.elapsed();

        // Memoized version - stays fast!
        let mut memo = HashMap::new();
        let start_memo = Instant::now();
        let result_memo = can_make_memo(&patterns, design, &mut memo);
        let time_memo = start_memo.elapsed();

        assert_eq!(result_naive, result_memo); // Should always match!

        last_cache_size = memo.len();

        let speedup = if time_memo.as_nanos() > 0 {
            time_naive.as_nanos() as f64 / time_memo.as_nanos() as f64
        } else {
            1.0
        };

        println!(
            "  {:2}   | {:12.1?} | {:12.1?} | {:7.1}x | {} entries",
            length,
            time_naive,
            time_memo,
            speedup,
            memo.len()
        );

        // Stop if naive is taking too long (> 1 second)
        if time_naive.as_secs() >= 1 {
            println!("\n⚠️  Naive version getting too slow - stopping early!");
            println!("   (Memoized could easily handle length 100+ in microseconds!)");
            break;
        }
    }

    println!("\n🎯 Key Observations:");
    println!("   • Naive: Time DOUBLES with each +2 length (exponential!)");
    println!("   • Memo: Time stays roughly constant (linear in design length!)");
    println!("   • Speedup: Grows exponentially as problems get harder");
    println!(
        "   • Cache: Only {} entries needed (one per unique suffix)",
        last_cache_size
    );
    println!("\n💡 This is why DP/memoization is essential for AoC Day 19!");
    println!("   Real inputs have 60+ character designs - impossible without memoization!");
}

#[cfg(test)]
mod step_tests {
    use super::*;

    #[test]
    fn test_memoization_correctness() {
        let patterns = vec!["r", "wr", "b"];
        let mut memo = HashMap::new();

        assert!(can_make_memo(&patterns, "r", &mut memo));
        assert!(can_make_memo(&patterns, "wr", &mut memo));
        assert!(can_make_memo(&patterns, "wrr", &mut memo));
        assert!(!can_make_memo(&patterns, "g", &mut memo));
    }

    #[test]
    fn test_cache_accumulation() {
        let patterns = vec!["r", "wr"];
        let mut memo = HashMap::new();

        can_make_memo(&patterns, "wrr", &mut memo);
        let size_after_first = memo.len();

        // Calling with same design shouldn't grow cache
        can_make_memo(&patterns, "wrr", &mut memo);
        assert_eq!(memo.len(), size_after_first);
    }

    #[test]
    fn test_negative_caching() {
        let patterns = vec!["a"];
        let mut memo = HashMap::new();

        assert!(!can_make_memo(&patterns, "b", &mut memo));
        assert!(memo.contains_key("b"));
        assert_eq!(memo.get("b"), Some(&false));
    }

    #[test]
    fn test_performance_improvement() {
        let patterns = vec!["r", "wr", "b", "br"];
        let design = "brwrrbrwrrbrwrr"; // Long enough to show difference

        // Naive version (might be slow!)
        let start_naive = Instant::now();
        let result_naive = can_make_naive(&patterns, design);
        let time_naive = start_naive.elapsed();

        // Memoized version
        let mut memo = HashMap::new();
        let start_memo = Instant::now();
        let result_memo = can_make_memo(&patterns, design, &mut memo);
        let time_memo = start_memo.elapsed();

        assert_eq!(result_naive, result_memo);
        // Memoized should be faster (or at least not slower)
        assert!(time_memo <= time_naive || time_naive.as_micros() < 100);
    }
}
