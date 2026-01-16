//! REQ-4: Boolean → Counting DP Transformation Demo
//!
//! This example demonstrates the canonical transformation pattern between:
//! - **Existence check (Part 1)**: "Can we build this?" → bool
//! - **Exhaustive counting (Part 2)**: "How many ways can we build this?" → u64
//!
//! # Key Insight
//!
//! The recursive structure is **identical**. Only three things change:
//! 1. Return type: `bool` → `u64`
//! 2. Base case value: `true` → `1`
//! 3. Reduction operator: early-exit **OR** → accumulative **SUM**
//!
//! # Real-World Application
//!
//! This pattern appears in:
//! - AoC 2024 Day 19 (Linen Layout): Part 1 = can construct, Part 2 = count ways
//! - AoC 2024 Day 12 (Hot Springs): Part 1 = valid arrangement, Part 2 = count valid
//! - Climbing stairs: Part 1 = can reach top, Part 2 = count all paths
//!
//! # Requirements Satisfied
//! - REQ-4: Boolean → Counting transformation with identical structure
//! - REQ-2: Top-down DP pattern template (4 steps)
//! - REQ-3: Zero-copy string slice caching
//! - REQ-6: Cache statistics demonstrate overlapping subproblems

use std::collections::HashMap;

fn main() {
    println!("=== REQ-4: Boolean → Counting DP Transformation ===\n");

    demo_pattern_matching();
    demo_climbing_stairs();
    demo_cache_statistics();
    demo_performance_comparison();
}

/// Pattern matching example: Can we build a word from letter tiles?
fn demo_pattern_matching() {
    println!("--- Pattern Matching: Word Construction ---\n");

    let patterns = vec!["a", "b", "ab", "ba"];
    let targets = vec!["ab", "aba", "abab", "xyz"];

    for target in &targets {
        let mut bool_memo = HashMap::new();
        let mut count_memo = HashMap::new();

        let exists = can_build(target, &patterns, &mut bool_memo);
        let ways = count_ways(target, &patterns, &mut count_memo);

        println!("Target: \"{}\"", target);
        println!("  Part 1 (boolean): Can build? {}", exists);
        println!("  Part 2 (counting): How many ways? {}", ways);
        println!("  Cache sizes: bool={}, count={}", bool_memo.len(), count_memo.len());
        println!();
    }
}

/// Climbing stairs example: Can you reach the top? How many distinct ways?
fn demo_climbing_stairs() {
    println!("--- Climbing Stairs Problem ---\n");
    println!("Rules: You can climb 1 or 2 steps at a time");
    println!("Question: Can you reach step N? How many ways?\n");

    let stairs = vec![3, 5, 10, 20];

    for &n in &stairs {
        let mut bool_memo = HashMap::new();
        let mut count_memo = HashMap::new();

        let can_reach = can_reach_step(n, &mut bool_memo);
        let ways = count_ways_to_reach(n, &mut count_memo);

        println!("Stairs: {}", n);
        println!("  Part 1: Can reach? {} (always true for positive n)", can_reach);
        println!("  Part 2: How many ways? {}", ways);
        println!();
    }
}

/// Demonstrate cache statistics showing overlapping subproblems
fn demo_cache_statistics() {
    println!("--- Cache Statistics: Overlapping Subproblems ---\n");

    let patterns = vec!["a", "aa", "aaa"];
    let target = "aaaaaaaa"; // 8 'a's

    let mut count_memo = HashMap::new();
    let ways = count_ways(target, &patterns, &mut count_memo);

    println!("Target: \"{}\" (8 'a's)", target);
    println!("Patterns: {:?}", patterns);
    println!("Total ways to construct: {}", ways);
    println!("Unique subproblems cached: {}", count_memo.len());
    println!("\nCache contents (substring → count):");

    let mut cache_vec: Vec<_> = count_memo.iter().collect();
    cache_vec.sort_by_key(|(k, _)| k.len());

    for (substring, &count) in cache_vec {
        println!("  \"{}\" (len {}): {} ways", substring, substring.len(), count);
    }

    println!("\nKey insight: Without memoization, this would compute {} paths!", ways);
    println!("With memoization, we only compute {} unique subproblems.", count_memo.len());
    println!();
}

/// Performance comparison: Naive vs Memoized
fn demo_performance_comparison() {
    println!("--- Performance Comparison ---\n");

    let patterns = vec!["a", "aa"];
    let target = "aaaaaaaa"; // 8 'a's

    // Memoized version (fast)
    let start = std::time::Instant::now();
    let mut count_memo = HashMap::new();
    let ways_fast = count_ways(target, &patterns, &mut count_memo);
    let duration_fast = start.elapsed();

    println!("Target: \"{}\" (8 'a's)", target);
    println!("Patterns: {:?}", patterns);
    println!("\nMemoized approach:");
    println!("  Ways to construct: {}", ways_fast);
    println!("  Cache size: {}", count_memo.len());
    println!("  Time: {:?}", duration_fast);

    // Naive version would be exponential - don't actually run it for large n!
    println!("\nNaive recursive approach (theoretical):");
    println!("  Would recompute same substrings repeatedly");
    println!("  Time complexity: O(P^L) where P=patterns, L=length");
    println!("  For this input: O(2^8) = ~256 recursive calls");
    println!("  With memoization: O(P×L) = O(2×8) = 16 unique states");
    println!();
}

// ============================================================================
// PART 1: BOOLEAN DP (Existence Check)
// ============================================================================

/// Can the target be constructed from the patterns?
///
/// # Pattern: Early-Exit OR Logic
///
/// Returns `true` as soon as **any** valid construction is found.
/// Short-circuits on first success - doesn't explore all possibilities.
///
/// # Structure (REQ-2)
/// 1. Base case: Empty string → true
/// 2. Cache check: Return cached result if exists
/// 3. Recursive computation: Try each pattern, return true if any succeeds
/// 4. Cache and return: Store false if no pattern works
fn can_build<'a>(
    target: &'a str,
    patterns: &[&str],
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    // REQ-2 Step 1: Base case
    if target.is_empty() {
        return true; // ← Base value: true (success)
    }

    // REQ-2 Step 2: Memoization check
    if let Some(&cached) = memo.get(target) {
        return cached;
    }

    // REQ-2 Step 3: Recursive computation
    for pattern in patterns {
        // REQ-3: Zero-copy string slice operation
        if let Some(remainder) = target.strip_prefix(pattern) {
            if can_build(remainder, patterns, memo) {
                memo.insert(target, true);
                return true; // ← Early exit: OR logic
            }
        }
    }

    // REQ-2 Step 4: Cache and return
    memo.insert(target, false);
    false
}

// ============================================================================
// PART 2: COUNTING DP (Exhaustive Enumeration)
// ============================================================================

/// Count all distinct ways to construct target from patterns
///
/// # Pattern: Accumulative SUM Logic
///
/// Returns sum of **all** valid constructions.
/// No early exit - explores every possible path to completion.
///
/// # Structure (IDENTICAL to can_build!)
/// 1. Base case: Empty string → 1 (one way to succeed)
/// 2. Cache check: Return cached count if exists
/// 3. Recursive computation: Try each pattern, sum all successful paths
/// 4. Cache and return: Store total count
///
/// # Transformation from can_build
/// - Line 1: `true` → `1`
/// - Line 2: `HashMap<_, bool>` → `HashMap<_, u64>`
/// - Line 3: `if success { return true }` → `total += recursive_call()`
/// - Line 4: `false` → `0` (no ways found)
fn count_ways<'a>(
    target: &'a str,
    patterns: &[&str],
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    // REQ-2 Step 1: Base case
    if target.is_empty() {
        return 1; // ← Base value: 1 (one way to complete)
    }

    // REQ-2 Step 2: Memoization check (IDENTICAL)
    if let Some(&cached) = memo.get(target) {
        return cached;
    }

    // REQ-2 Step 3: Recursive computation
    let mut total = 0; // ← Accumulator instead of early-exit flag

    for pattern in patterns {
        // REQ-3: Zero-copy string slice operation (IDENTICAL)
        if let Some(remainder) = target.strip_prefix(pattern) {
            total += count_ways(remainder, patterns, memo); // ← SUM all paths
        }
    }

    // REQ-2 Step 4: Cache and return
    memo.insert(target, total);
    total
}

// ============================================================================
// CLIMBING STAIRS VARIANT
// ============================================================================

/// Can you reach step n? (Spoiler: always true for positive n)
fn can_reach_step(n: usize, memo: &mut HashMap<usize, bool>) -> bool {
    // Base cases
    if n == 0 {
        return true; // At ground level
    }
    if n == 1 {
        return true; // One step up
    }

    // Memoization
    if let Some(&cached) = memo.get(&n) {
        return cached;
    }

    // Recursive: Can reach n-1 OR n-2, then take final step(s)
    let result = can_reach_step(n - 1, memo) || can_reach_step(n - 2, memo);

    memo.insert(n, result);
    result
}

/// Count all distinct ways to reach step n
fn count_ways_to_reach(n: usize, memo: &mut HashMap<usize, u64>) -> u64 {
    // Base cases (Fibonacci-like)
    if n == 0 {
        return 1; // One way: don't move
    }
    if n == 1 {
        return 1; // One way: single step
    }

    // Memoization (IDENTICAL structure)
    if let Some(&cached) = memo.get(&n) {
        return cached;
    }

    // Recursive: SUM paths from n-1 and n-2
    let result = count_ways_to_reach(n - 1, memo) + count_ways_to_reach(n - 2, memo);

    memo.insert(n, result);
    result
}
