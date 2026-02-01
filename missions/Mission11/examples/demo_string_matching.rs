//! Demo: String Pattern Matching with Memoization
//!
//! Demonstrates REQ-2 (Top-Down DP Pattern) and REQ-3 (Zero-Copy String Caching)
//! using the AoC 2024 Day 19 pattern (Linen Layout).
//!
//! This example shows:
//! 1. Boolean DP (existence check with early-exit OR)
//! 2. Counting DP (exhaustive enumeration with accumulative SUM)
//! 3. Performance impact of memoization
//! 4. Zero-copy string slice operations

use mission11::string_dp::{can_construct, count_constructions};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("=== Mission 11: String Pattern Matching with Memoization ===\n");

    demo_basic_pattern_matching();
    demo_counting_arrangements();
    demo_performance_comparison();
    demo_aoc_day19_simulation();
}

/// Basic pattern matching (Boolean DP)
fn demo_basic_pattern_matching() {
    println!("--- Part 1: Existence Check (Boolean DP) ---\n");

    let patterns = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

    let test_cases = vec![
        ("brwrr", true),
        ("bggr", true),
        ("ubwu", false), // No 'u' pattern
        ("brgr", true),
    ];

    for (design, expected) in test_cases {
        let mut memo = HashMap::new();
        let start = Instant::now();
        let result = can_construct(design, &patterns, &mut memo);
        let elapsed = start.elapsed();

        let status = if result {
            "✅ Possible"
        } else {
            "❌ Impossible"
        };
        println!(
            "  Design: {:10} → {} (cached: {} states, took: {:?})",
            design,
            status,
            memo.len(),
            elapsed
        );

        assert_eq!(result, expected);
    }

    println!();
}

/// Counting all possible arrangements (Counting DP)
fn demo_counting_arrangements() {
    println!("--- Part 2: Count All Arrangements (Counting DP) ---\n");

    let patterns = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

    let designs = vec!["brwrr", "bggr", "gbbr", "rrbgbr", "brgr"];

    let mut total_arrangements = 0;

    for design in designs {
        let mut memo = HashMap::new();
        let start = Instant::now();
        let count = count_constructions(design, &patterns, &mut memo);
        let elapsed = start.elapsed();

        total_arrangements += count;

        println!(
            "  Design: {:10} → {} arrangements (cached: {} states, took: {:?})",
            design,
            count,
            memo.len(),
            elapsed
        );
    }

    println!(
        "\n  Total arrangements across all designs: {}\n",
        total_arrangements
    );
}

/// Demonstrate performance impact of memoization
fn demo_performance_comparison() {
    println!("--- Part 3: Memoization Performance Impact ---\n");

    let patterns = vec!["a", "aa", "aaa"];

    // Test with increasing target lengths
    for length in [5, 10, 15, 20] {
        let target: String = "a".repeat(length);

        let mut memo = HashMap::new();
        let start = Instant::now();
        let result = can_construct(&target, &patterns, &mut memo);
        let elapsed = start.elapsed();

        println!(
            "  Target length: {:2} → Result: {:5}, Cache size: {:3}, Time: {:?}",
            length,
            result,
            memo.len(),
            elapsed
        );
    }

    println!();
    println!("  Note: Without memoization, length=20 would take ~3^20 = 3.5B operations!");
    println!("        With memoization: O(patterns × length) = ~60 operations\n");
}

/// Simulate AoC 2024 Day 19 problem
fn demo_aoc_day19_simulation() {
    println!("--- Part 4: AoC 2024 Day 19 Simulation ---\n");

    let towel_patterns = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

    let desired_designs = vec![
        "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu",   // Impossible
        "bwurrg", // Impossible
        "brgr", "bbrgwb", // Impossible
    ];

    println!("  Available towel patterns: {:?}\n", towel_patterns);

    // Part 1: How many designs are possible?
    let mut possible_count = 0;
    let mut part1_cache_total = 0;

    for design in &desired_designs {
        let mut memo = HashMap::new();
        if can_construct(design, &towel_patterns, &mut memo) {
            possible_count += 1;
            part1_cache_total += memo.len();
            println!("  ✅ {} (cached {} states)", design, memo.len());
        } else {
            part1_cache_total += memo.len();
            println!("  ❌ {} (cached {} states)", design, memo.len());
        }
    }

    println!("\n  Part 1 Answer: {} designs are possible", possible_count);
    println!(
        "  Total cache entries across all designs: {}\n",
        part1_cache_total
    );

    // Part 2: Total number of different arrangements
    let mut total_arrangements = 0u64;
    let mut part2_cache_total = 0;

    println!("  Counting arrangements for possible designs:\n");

    for design in &desired_designs {
        let mut memo = HashMap::new();
        let count = count_constructions(design, &towel_patterns, &mut memo);

        if count > 0 {
            total_arrangements += count;
            part2_cache_total += memo.len();
            println!(
                "  {} → {} arrangements (cached {} states)",
                design,
                count,
                memo.len()
            );
        }
    }

    println!(
        "\n  Part 2 Answer: {} total arrangements",
        total_arrangements
    );
    println!(
        "  Average cache size per design: {:.1}\n",
        part2_cache_total as f64 / possible_count as f64
    );
}

/// Demonstrate REQ-4: Boolean → Counting transformation
#[allow(dead_code)]
fn demo_transformation_pattern() {
    println!("--- REQ-4: Boolean → Counting Transformation ---\n");

    let patterns = vec!["a", "aa"];
    let target = "aa";

    // Boolean version: Can we construct "aa"?
    let mut bool_memo = HashMap::new();
    let exists = can_construct(target, &patterns, &mut bool_memo);
    println!("  Boolean DP: Can construct \"{}\"? {}", target, exists);
    println!("  Cache states: {}", bool_memo.len());

    // Counting version: How many ways to construct "aa"?
    let mut count_memo = HashMap::new();
    let count = count_constructions(target, &patterns, &mut count_memo);
    println!(
        "\n  Counting DP: Ways to construct \"{}\"? {}",
        target, count
    );
    println!("  Cache states: {}", count_memo.len());

    println!("\n  Analysis:");
    println!("    - \"aa\" = \"a\" + \"a\" (first way)");
    println!("    - \"aa\" = \"aa\" (second way)");
    println!("    - Total: {} ways\n", count);

    println!("  Transformation:");
    println!("    - Base case: true → 1");
    println!("    - Reduction: OR (early-exit) → SUM (accumulate)");
    println!("    - Cache type: HashMap<&str, bool> → HashMap<&str, u64>");
    println!("    - Structure: IDENTICAL (only reduction operator changes)\n");
}
