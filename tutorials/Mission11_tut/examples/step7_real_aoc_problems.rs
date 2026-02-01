//! Step 7: Real AoC Problems - Production-Ready Implementation
//!
//! **Learning Objective**: By the end of this step, you will be able to apply all
//! learned DP patterns to real competitive programming problems from Advent of Code.
//!
//! **Prerequisites**: Steps 1-6 (all DP patterns)
//!
//! **Mission Ready**: After this step, you're prepared to implement Mission 11's
//! production-grade DP library with full testing and instrumentation.

use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("=== Step 7: Real AoC Problems ===\n");

    demonstrate_core_concept();
    show_common_patterns();
    handle_edge_cases();
    validate_understanding();

    println!("\n✅ Step 7 completed! Tutorial finished!");
    println!("🎯 Next: Apply these patterns to Mission 11 for production implementation");
    println!("💡 Key Insight: Pattern recognition is the skill - implementation is mechanical!");
}

/// Core concept demonstration
///
/// Shows complete solution to AoC 2024 Day 19: Linen Layout
/// Key learning: End-to-end application of DP patterns.
fn demonstrate_core_concept() {
    println!("--- Core Concept: AoC 2024 Day 19 - Linen Layout ---");

    println!("\nProblem:");
    println!("  Given towel patterns and desired designs,");
    println!("  Part 1: How many designs are possible?");
    println!("  Part 2: How many total ways to make all designs?");

    let sample_input = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    solve_day19(sample_input);
}

/// Complete solution for AoC 2024 Day 19
fn solve_day19(input: &str) {
    let (patterns, designs) = parse_input(input);

    println!("\nInput:");
    println!("  Patterns: {:?}", patterns);
    println!("  Designs: {} total", designs.len());

    // Part 1: Count how many designs are possible (using clean API)
    let start = Instant::now();
    let part1_count = designs
        .iter()
        .filter(|design| can_make_design(&patterns, design))
        .count();
    let part1_time = start.elapsed();

    println!(
        "\nPart 1: {} designs are possible (took {:?})",
        part1_count, part1_time
    );

    // Part 2: Count total number of ways to make all designs (using clean API)
    let start = Instant::now();
    let part2_sum: u64 = designs
        .iter()
        .map(|design| count_ways_to_make(&patterns, design))
        .sum();
    let part2_time = start.elapsed();

    println!(
        "\nPart 2: {} total arrangements (took {:?})",
        part2_sum, part2_time
    );

    println!("\n✓ Complete AoC solution using DP patterns from Steps 1-6!");
}

/// Parse AoC input format (handles both \n\n and single \n separation)
fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    // If there's only one part, try splitting by single newline
    let (patterns_str, designs_str) = if parts.len() == 1 {
        // Find first newline and split there
        if let Some(first_newline) = input.find('\n') {
            (&input[..first_newline], &input[first_newline + 1..])
        } else {
            (input, "")
        }
    } else {
        (parts[0], parts[1])
    };

    let patterns: Vec<&str> = patterns_str.split(", ").map(|s| s.trim()).collect();

    let designs: Vec<&str> = designs_str
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    (patterns, designs)
}

/// Check if design can be made (public API, matches AoC structure)
fn can_make_design(patterns: &[&str], design: &str) -> bool {
    let mut memo = HashMap::new();
    can_make(patterns, design, &mut memo)
}

/// Part 1: Check if design can be made (boolean existence)
fn can_make<'a>(patterns: &[&str], design: &'a str, memo: &mut HashMap<&'a str, bool>) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(&result) = memo.get(design) {
        return result;
    }

    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            if can_make(patterns, rest, memo) {
                memo.insert(design, true);
                return true;
            }
        }
    }

    memo.insert(design, false);
    false
}

/// Count ways to make design (public API, matches AoC structure)
fn count_ways_to_make(patterns: &[&str], design: &str) -> u64 {
    let mut memo = HashMap::new();
    count_ways(patterns, design, &mut memo)
}

/// Part 2: Count all ways to make design (exhaustive counting)
fn count_ways<'a>(patterns: &[&str], design: &'a str, memo: &mut HashMap<&'a str, u64>) -> u64 {
    if design.is_empty() {
        return 1;
    }

    if let Some(&result) = memo.get(design) {
        return result;
    }

    let mut total = 0;
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            total += count_ways(patterns, rest, memo);
        }
    }

    memo.insert(design, total);
    total
}

/// Common usage patterns
///
/// Shows variations and optimizations of the core pattern.
fn show_common_patterns() {
    println!("\n--- Common Patterns ---");

    pattern_1_cache_reuse();
    pattern_2_bottom_up_variant();
    pattern_3_performance_analysis();
}

fn pattern_1_cache_reuse() {
    println!("\nPattern 1: Reusing cache across multiple designs");

    let patterns = vec!["r", "wr", "b"];
    let designs = vec!["wrr", "brr", "wrb"];

    // Shared cache approach
    let mut shared_cache = HashMap::new();

    println!("  Using shared cache:");
    for design in &designs {
        let before = shared_cache.len();
        let result = can_make(&patterns, design, &mut shared_cache);
        let after = shared_cache.len();

        println!(
            "    '{}': {} (cache grew by {} entries)",
            design,
            result,
            after - before
        );
    }

    println!("  Final cache: {} total entries", shared_cache.len());
    println!("  Benefit: Later designs benefit from earlier cached substrings");
}

fn pattern_2_bottom_up_variant() {
    println!("\nPattern 2: Bottom-up DP variant (position-based)");

    let patterns = vec!["r", "wr", "b"];
    let design = "wrr";

    let result = can_make_bottom_up(&patterns, design);
    println!("  Design: '{}'", design);
    println!("  Result: {} (using bottom-up approach)", result);
    println!("  Note: Top-down usually clearer for string problems");
}

fn can_make_bottom_up(patterns: &[&str], design: &str) -> bool {
    let n = design.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    for i in 0..n {
        if !dp[i] {
            continue;
        }

        for pattern in patterns {
            let end = i + pattern.len();
            if end <= n && design[i..].starts_with(pattern) {
                dp[end] = true;
            }
        }
    }

    dp[n]
}

fn pattern_3_performance_analysis() {
    println!("\nPattern 3: Performance analysis with instrumentation");

    let patterns = vec!["r", "wr", "b", "g", "br", "rb", "gb"];
    let design = "rbgbrwgrbwgrbwrr";

    // With instrumentation
    let mut cache = InstrumentedCache::new();
    let result = can_make_instrumented(&patterns, design, &mut cache);

    println!("  Design: '{}' (length {})", design, design.len());
    println!("  Result: {}", result);
    println!("\n  Cache statistics:");
    println!("    Hits: {}", cache.hits);
    println!("    Misses: {}", cache.misses);
    println!("    Hit rate: {:.1}%", cache.hit_rate());
    println!("    Size: {}", cache.cache.len());
    println!("    Max substring length: {}", design.len());
    println!(
        "    Cache efficiency: {:.1}%",
        100.0 * cache.cache.len() as f64 / design.len() as f64
    );
}

/// Instrumented cache for statistics
struct InstrumentedCache<'a> {
    cache: HashMap<&'a str, bool>,
    hits: usize,
    misses: usize,
}

impl<'a> InstrumentedCache<'a> {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    fn get(&mut self, key: &'a str) -> Option<&bool> {
        if let Some(value) = self.cache.get(key) {
            self.hits += 1;
            Some(value)
        } else {
            self.misses += 1;
            None
        }
    }

    fn insert(&mut self, key: &'a str, value: bool) {
        self.cache.insert(key, value);
    }

    fn hit_rate(&self) -> f64 {
        if self.hits + self.misses == 0 {
            0.0
        } else {
            100.0 * self.hits as f64 / (self.hits + self.misses) as f64
        }
    }
}

fn can_make_instrumented<'a>(
    patterns: &[&str],
    design: &'a str,
    cache: &mut InstrumentedCache<'a>,
) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(&result) = cache.get(design) {
        return result;
    }

    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            if can_make_instrumented(patterns, rest, cache) {
                cache.insert(design, true);
                return true;
            }
        }
    }

    cache.insert(design, false);
    false
}

/// Edge case handling
///
/// Demonstrates production-ready error handling and edge cases.
fn handle_edge_cases() {
    println!("\n--- Edge Cases ---");

    empty_input();
    impossible_designs();
    large_inputs();
}

fn empty_input() {
    println!("\nEdge Case 1: Empty input");

    let patterns: Vec<&str> = vec![];
    let designs: Vec<&str> = vec![];

    let mut cache = HashMap::new();
    let mut count = 0;

    for design in &designs {
        if can_make(&patterns, design, &mut cache) {
            count += 1;
        }
    }

    println!("  Empty patterns + empty designs: {} possible", count);
    println!("  Handles gracefully - vacuous truth");
}

fn impossible_designs() {
    println!("\nEdge Case 2: All designs impossible");

    let patterns = vec!["a", "b"];
    let designs = vec!["x", "y", "z"];

    let mut cache = HashMap::new();
    let mut count = 0;

    for design in &designs {
        if can_make(&patterns, design, &mut cache) {
            count += 1;
        }
    }

    println!("  Patterns: {:?}", patterns);
    println!("  Designs: {:?}", designs);
    println!("  Possible: {} (correctly returns 0)", count);
    println!("  Cache contains negative results");
}

fn large_inputs() {
    println!("\nEdge Case 3: Large input performance");

    let patterns = vec!["r", "wr", "b", "g", "br", "rb", "gb", "bwu"];

    // Generate a moderately long design
    let design = "bwrrbrgbrwrrbwrrbrwrgbrwrr"; // 25 chars

    let mut cache = HashMap::new();
    let start = Instant::now();
    let result = can_make(&patterns, design, &mut cache);
    let duration = start.elapsed();

    println!("  Design length: {} chars", design.len());
    println!("  Time: {:?}", duration);
    println!("  Result: {}", result);
    println!("  Cache entries: {}", cache.len());
    println!("  Performance: Linear with string length (thanks to memoization!)");
}

/// Understanding validation
///
/// Final validation of all concepts.
fn validate_understanding() {
    println!("\n--- Validation Exercises ---");

    println!("\n🤔 Production readiness check:");

    exercise_1_full_problem();
    exercise_2_optimization();
    exercise_3_debugging();
}

fn exercise_1_full_problem() {
    println!("\nExercise 1: Solve complete AoC problem");

    let input = "\
r, wr, b

r
wr
wrr
brr
xyz";

    println!("  Input: 3 patterns, 5 designs");

    let (patterns, designs) = parse_input(input);
    let mut bool_cache = HashMap::new();
    let mut count_cache = HashMap::new();

    let mut part1 = 0;
    let mut part2 = 0u64;

    for design in &designs {
        if can_make(&patterns, design, &mut bool_cache) {
            part1 += 1;
        }
        part2 += count_ways(&patterns, design, &mut count_cache);
    }

    println!("  Part 1: {} possible", part1);
    println!("  Part 2: {} total ways", part2);
    println!("  ✓ Complete solution using learned patterns!");
}

fn exercise_2_optimization() {
    println!("\nExercise 2: Identify optimization opportunities");

    println!("  1. Pattern sorting: Sort patterns by length (longest first)");
    println!("     Benefit: Find matches faster for long patterns");

    println!("  2. Pattern indexing: Group patterns by first character");
    println!("     Benefit: Only check relevant patterns");

    println!("  3. Cache warming: Pre-compute common substrings");
    println!("     Benefit: Faster subsequent queries");

    println!("\n  For Mission 11: Implement and benchmark these optimizations!");
}

fn exercise_3_debugging() {
    println!("\nExercise 3: Debugging techniques");

    println!("  1. Add debug prints to show recursion depth");
    println!("  2. Track which patterns match at each step");
    println!("  3. Visualize cache growth over time");
    println!("  4. Profile with instrumentation (hits/misses)");

    println!("\n  Example debug output:");
    debug_example();
}

fn debug_example() {
    let patterns = vec!["r", "wr"];
    let design = "wrr";

    println!("\n  Debug trace for design 'wrr':");
    let mut cache = HashMap::new();
    can_make_debug(&patterns, design, &mut cache, 0);
}

fn can_make_debug<'a>(
    patterns: &[&str],
    design: &'a str,
    memo: &mut HashMap<&'a str, bool>,
    depth: usize,
) -> bool {
    let indent = "  ".repeat(depth);
    println!("{}  Checking: '{}'", indent, design);

    if design.is_empty() {
        println!("{}  → Base case: true", indent);
        return true;
    }

    if let Some(&result) = memo.get(design) {
        println!("{}  → Cache hit: {}", indent, result);
        return result;
    }

    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            println!("{}  Try pattern '{}'", indent, pattern);
            if can_make_debug(patterns, rest, memo, depth + 1) {
                memo.insert(design, true);
                println!("{}  → Success via '{}'", indent, pattern);
                return true;
            }
        }
    }

    memo.insert(design, false);
    println!("{}  → Failed", indent);
    false
}

#[cfg(test)]
mod step_tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        let (patterns, designs) = parse_input(input);
        assert_eq!(patterns.len(), 8);
        assert_eq!(designs.len(), 8);

        let possible = designs
            .iter()
            .filter(|design| can_make_design(&patterns, design))
            .count();

        // AoC 2024 Day 19 Part 1 expected answer
        assert_eq!(possible, 6);
    }

    #[test]
    fn test_part2_example() {
        let input = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        let (patterns, designs) = parse_input(input);

        let total: u64 = designs
            .iter()
            .map(|design| count_ways_to_make(&patterns, design))
            .sum();

        // AoC 2024 Day 19 Part 2 expected answer
        assert_eq!(total, 16);
    }

    #[test]
    fn test_count_ways_individual() {
        let input = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        let (patterns, _) = parse_input(input);

        // Test specific designs match AoC expected values
        assert_eq!(count_ways_to_make(&patterns, "brwrr"), 2);
        assert_eq!(count_ways_to_make(&patterns, "bggr"), 1);
        assert_eq!(count_ways_to_make(&patterns, "gbbr"), 4);
        assert_eq!(count_ways_to_make(&patterns, "rrbgbr"), 6);
        assert_eq!(count_ways_to_make(&patterns, "bwurrg"), 1);
        assert_eq!(count_ways_to_make(&patterns, "brgr"), 2);

        // Impossible designs should have 0 ways
        assert_eq!(count_ways_to_make(&patterns, "ubwu"), 0);
        assert_eq!(count_ways_to_make(&patterns, "bbrgwb"), 0);
    }

    #[test]
    fn test_bottom_up_equivalence() {
        let patterns = vec!["r", "wr", "b"];
        let designs = vec!["r", "wr", "wrr", "xyz"];

        for design in designs {
            let mut cache = HashMap::new();
            let td = can_make(&patterns, design, &mut cache);
            let bu = can_make_bottom_up(&patterns, design);

            assert_eq!(td, bu);
        }
    }

    #[test]
    fn test_instrumentation() {
        let patterns = vec!["r", "wr"];
        let design = "wrr";

        let mut cache = InstrumentedCache::new();
        let result = can_make_instrumented(&patterns, design, &mut cache);

        assert!(result);
        assert!(cache.hits + cache.misses > 0);
        assert!(cache.hit_rate() >= 0.0 && cache.hit_rate() <= 100.0);
    }
}
