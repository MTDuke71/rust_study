//! Step 5: Boolean → Counting Transformation - Same Structure, Different Semantics
//!
//! **Learning Objective**: By the end of this step, you will understand how to transform
//! existence checks (boolean) into exhaustive counting (numeric) with minimal code changes.
//!
//! **Prerequisites**: Step 4 (generic cache), understanding of combinatorics
//!
//! **Next Step Preview**: Step 6 will introduce bottom-up tabulation as an alternative
//! to top-down memoization, comparing the trade-offs of each approach.

use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("=== Step 5: Boolean → Counting Transformation ===\n");
    
    demonstrate_core_concept();
    show_common_patterns();
    handle_edge_cases();
    validate_understanding();
    
    println!("\n✅ Step 5 completed! Ready for Step 6.");
    println!("💡 Key Insight: DP patterns are composable - same structure, different aggregation!");
}

/// Core concept demonstration
///
/// Shows how to transform boolean existence to exhaustive counting.
/// Key learning: Change return type and aggregation, keep recursion structure.
fn demonstrate_core_concept() {
    println!("--- Core Concept: From 'Can Make?' to 'How Many Ways?' ---");
    
    let patterns = vec!["r", "wr", "b", "br"];
    let design = "brr";
    
    println!("\nProblem: Design '{}' from patterns {:?}", design, patterns);
    
    // Part 1: Boolean existence
    println!("\n1. Boolean Version (Part 1):");
    println!("   Question: CAN we make this design?");
    let mut memo_bool = HashMap::new();
    let exists = can_make_boolean(&patterns, design, &mut memo_bool);
    println!("   Answer: {}", exists);
    println!("   Cache size: {} (stores true/false for each substring)", memo_bool.len());
    
    // Part 2: Count all ways
    println!("\n2. Counting Version (Part 2):");
    println!("   Question: HOW MANY ways can we make it?");
    let mut memo_count = HashMap::new();
    let count = count_ways(&patterns, design, &mut memo_count);
    println!("   Answer: {} ways", count);
    println!("   Cache size: {} (stores count for each substring)", memo_count.len());
    
    println!("\n3. Comparison:");
    println!("   Both visit same subproblems");
    println!("   Both use same recursive structure");
    println!("   Difference: boolean OR vs numeric SUM");
    
    explain_transformation();
}

fn explain_transformation() {
    println!("\n--- Understanding the Transformation ---");
    
    println!("\nBoolean version (existence):");
    println!("  if can_make(rest):");
    println!("      return true     ← Stop at first success");
    println!("  return false        ← No solution found");
    
    println!("\nCounting version (enumerate all):");
    println!("  total = 0");
    println!("  for each pattern:");
    println!("      total += count_ways(rest)  ← Sum all paths");
    println!("  return total        ← Total number of ways");
    
    println!("\nKey insight:");
    println!("  Boolean: Short-circuit on first success (OR aggregation)");
    println!("  Counting: Explore all paths and sum (SUM aggregation)");
}

/// Boolean version: Does ANY solution exist?
fn can_make_boolean<'a>(
    patterns: &[&str],
    design: &'a str,
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if design.is_empty() {
        return true;  // Base case: empty string succeeded
    }
    
    if let Some(&result) = memo.get(design) {
        return result;
    }
    
    // Try each pattern - stop at first success (OR)
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            if can_make_boolean(patterns, rest, memo) {
                memo.insert(design, true);
                return true;  // Short-circuit!
            }
        }
    }
    
    memo.insert(design, false);
    false
}

/// Counting version: How many TOTAL solutions exist?
fn count_ways<'a>(
    patterns: &[&str],
    design: &'a str,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if design.is_empty() {
        return 1;  // Base case: one way to make empty string
    }
    
    if let Some(&result) = memo.get(design) {
        return result;
    }
    
    // Try each pattern - sum all paths (SUM)
    let mut total = 0;
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            total += count_ways(patterns, rest, memo);  // Accumulate!
        }
    }
    
    memo.insert(design, total);
    total
}

/// Common usage patterns
///
/// Shows the transformation applied to different problems.
fn show_common_patterns() {
    println!("\n--- Common Patterns ---");
    
    pattern_1_multiple_solutions();
    pattern_2_fibonacci_comparison();
    pattern_3_performance_comparison();
}

fn pattern_1_multiple_solutions() {
    println!("\nPattern 1: Designs with multiple solution paths");
    
    let patterns = vec!["r", "wr", "b", "br"];
    
    let test_cases = vec![
        "r",      // 1 way: "r"
        "br",     // 2 ways: "b"+"r" OR "br"
        "brr",    // 2 ways: "br"+"r" OR "b"+"r"+"r"
        "wrr",    // 1 way: "wr"+"r"
    ];
    
    println!("\n  Design | Exists? | Count | Explanation");
    println!("  ------ | ------- | ----- | -----------");
    
    for design in test_cases {
        let mut memo_bool = HashMap::new();
        let mut memo_count = HashMap::new();
        
        let exists = can_make_boolean(&patterns, design, &mut memo_bool);
        let count = count_ways(&patterns, design, &mut memo_count);
        
        println!("  {:6} | {:7} | {:5} | {} unique paths", 
                 design, exists, count, count);
    }
}

fn pattern_2_fibonacci_comparison() {
    println!("\nPattern 2: Boolean vs Counting Fibonacci");
    
    // Why Fibonacci? The recurrence relation is f(n) = f(n-1) + f(n-2)
    // To reach position n, you can arrive from:
    //   - Position n-1 with a step of 1 → f(n-1) ways
    //   - Position n-2 with a step of 2 → f(n-2) ways
    // Total ways: f(n) = f(n-1) + f(n-2) ← Fibonacci!
    //
    // Sequence: f(0)=1, f(1)=1, f(2)=2, f(3)=3, f(4)=5, f(5)=8, f(7)=21...
    
    println!("\n  Boolean version: Can we reach N with steps of 1 or 2?");
    println!("  Counting version: How many ways to reach N?");
    
    for n in [3, 5, 7, 10] {
        let exists = true;  // Always possible with steps 1,2
        let count = fib_count_ways(n);
        
        println!("  n={:2}: exists={}, ways={:3}", n, exists, count);
    }
    
    // Example: For n=5, the 8 ways are:
    // 1. 1+1+1+1+1
    // 2. 2+1+1+1
    // 3. 1+2+1+1
    // 4. 1+1+2+1
    // 5. 1+1+1+2
    // 6. 2+2+1
    // 7. 2+1+2
    // 8. 1+2+2
    // (All unique orderings of steps that sum to 5)
    
    println!("\n  Observation: All N reachable, but exponentially many paths");
}

fn fib_count_ways(n: u64) -> u64 {
    if n == 0 { return 1; }
    if n == 1 { return 1; }
    
    let mut memo = HashMap::new();
    fib_count_recursive(n, &mut memo)
}

fn fib_count_recursive(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    }
    
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    
    // Count ways via n-1 + count ways via n-2
    let count = fib_count_recursive(n - 1, memo) + fib_count_recursive(n - 2, memo);
    memo.insert(n, count);
    count
}

fn pattern_3_performance_comparison() {
    println!("\nPattern 3: Performance - Boolean vs Counting");
    
    let patterns = vec!["r", "wr", "b", "g", "br", "rb"];
    let design = "bwrrrbgbrbwrr";
    
    // Counting version (run FIRST to avoid cache warming)
    let mut memo_count = HashMap::new();
    let start = Instant::now();
    let count = count_ways(&patterns, design, &mut memo_count);
    let time_count = start.elapsed();
    
    // Boolean version (run SECOND)
    let mut memo_bool = HashMap::new();
    let start = Instant::now();
    let exists = can_make_boolean(&patterns, design, &mut memo_bool);
    let time_bool = start.elapsed();
    
    // Counting version 2 (run THIRD to demonstrate cache warming)
    let mut memo_count2 = HashMap::new();
    let start = Instant::now();
    let count2 = count_ways(&patterns, design, &mut memo_count2);
    let time_count2 = start.elapsed();
    
    println!("  Design: '{}' (length {})", design, design.len());
    println!("\n  Counting version (cold cache - first run):");
    println!("    Time: {:?}", time_count);
    println!("    Result: {} ways", count);
    println!("    Cache size: {}", memo_count.len());
    
    println!("\n  Boolean version (warm cache - second run):");
    println!("    Time: {:?}", time_bool);
    println!("    Result: {}", exists);
    println!("    Cache size: {}", memo_bool.len());
    
    println!("\n  Counting version 2 (warm cache - third run):");
    println!("    Time: {:?}", time_count2);
    println!("    Result: {} ways", count2);
    println!("    Cache size: {}", memo_count2.len());
    
    println!("\n  🔍 Cache Warming Effect:");
    println!("    First algorithm (cold cache) suffers cache misses loading code/data");
    println!("    Subsequent runs (warm cache) benefit from already-loaded cache lines");
    println!("    Notice: Counting v1 (cold) vs Counting v2 (warm) shows same algorithm faster!");
    println!("\n  Observation: Similar performance (both visit same subproblems)");
    println!("  But counting does more work per subproblem (sum vs OR)");
}

/// Edge case handling
///
/// Demonstrates transformation behavior in special cases.
fn handle_edge_cases() {
    println!("\n--- Edge Cases ---");
    
    empty_string_case();
    impossible_design_case();
    single_solution_case();
}

fn empty_string_case() {
    println!("\nEdge Case 1: Empty string");
    
    let patterns = vec!["r"];
    let design = "";
    
    let mut memo_bool = HashMap::new();
    let mut memo_count = HashMap::new();
    
    let exists = can_make_boolean(&patterns, design, &mut memo_bool);
    let count = count_ways(&patterns, design, &mut memo_count);
    
    println!("  Boolean: {} (empty string is valid)", exists);
    println!("  Counting: {} way (exactly one way to make nothing)", count);
    println!("  Base case: true → 1");
}

fn impossible_design_case() {
    println!("\nEdge Case 2: Impossible design");
    
    let patterns = vec!["r", "wr"];
    let design = "xyz";
    
    let mut memo_bool = HashMap::new();
    let mut memo_count = HashMap::new();
    
    let exists = can_make_boolean(&patterns, design, &mut memo_bool);
    let count = count_ways(&patterns, design, &mut memo_count);
    
    println!("  Boolean: {} (no solution)", exists);
    println!("  Counting: {} ways (zero ways to make impossible)", count);
    println!("  Base case: false → 0");
}

fn single_solution_case() {
    println!("\nEdge Case 3: Unique solution path");
    
    let patterns = vec!["abc"];  // Only one pattern
    let design = "abc";
    
    let mut memo_bool = HashMap::new();
    let mut memo_count = HashMap::new();
    
    let exists = can_make_boolean(&patterns, design, &mut memo_bool);
    let count = count_ways(&patterns, design, &mut memo_count);
    
    println!("  Boolean: {} (exists)", exists);
    println!("  Counting: {} way (only one way)", count);
    println!("  When there's one path: true → 1");
}

/// Understanding validation
///
/// Exercises to validate transformation understanding.
fn validate_understanding() {
    println!("\n--- Validation Exercises ---");
    
    println!("\n🤔 Transformation comprehension check:");
    
    // Exercise 1
    println!("\nExercise 1: Predict the count");
    let patterns1 = vec!["a", "aa"];
    let design1 = "aaa";
    println!("  Patterns: {:?}, Design: '{}'", patterns1, design1);
    println!("  Paths: 'a'+'a'+'a', 'a'+'aa', 'aa'+'a'");
    println!("  Prediction: How many ways? _____");
    
    let mut memo1 = HashMap::new();
    let count1 = count_ways(&patterns1, design1, &mut memo1);
    println!("  Actual: {} ways", count1);
    
    // Exercise 2
    println!("\nExercise 2: Code transformation");
    println!("  Boolean: if can_make(x) {{ return true; }}");
    println!("  Counting equivalent: _____");
    println!("  Actual: total += count_ways(x);");
    
    // Exercise 3
    println!("\nExercise 3: Base case transformation");
    println!("  Boolean base (empty string): return _____");
    println!("  Counting base (empty string): return _____");
    println!("  Actual: Boolean → true, Counting → 1");
    println!("  Why? 'true' means exists, '1' means one way to make nothing");
    
    // Practical validation
    println!("\n--- Practical Validation ---");
    compare_both_versions();
}

fn compare_both_versions() {
    println!("\nComparing both versions on same inputs:");
    
    let patterns = vec!["r", "wr", "b", "br"];
    let designs = vec![
        "r",
        "wr", 
        "brr",
        "brwrr",
        "xyz",  // Impossible
    ];
    
    println!("\n  Design  | Exists? | Count | Relationship");
    println!("  ------- | ------- | ----- | ------------");
    
    for design in designs {
        let mut memo_bool = HashMap::new();
        let mut memo_count = HashMap::new();
        
        let exists = can_make_boolean(&patterns, design, &mut memo_bool);
        let count = count_ways(&patterns, design, &mut memo_count);
        
        let relationship = if exists {
            format!("exists ⟹ count ≥ 1 (count={})", count)
        } else {
            "!exists ⟹ count = 0".to_string()
        };
        
        println!("  {:7} | {:7} | {:5} | {}", design, exists, count, relationship);
    }
    
    println!("\n  Invariant: exists == (count > 0)");
    println!("  Boolean tells IF, counting tells HOW MANY");
}

#[cfg(test)]
mod step_tests {
    use super::*;
    
    #[test]
    fn test_boolean_counting_equivalence() {
        let patterns = vec!["r", "wr", "b"];
        let designs = vec!["r", "wr", "wrr", "xyz"];
        
        for design in designs {
            let mut memo_bool = HashMap::new();
            let mut memo_count = HashMap::new();
            
            let exists = can_make_boolean(&patterns, design, &mut memo_bool);
            let count = count_ways(&patterns, design, &mut memo_count);
            
            // Invariant: exists iff count > 0
            assert_eq!(exists, count > 0);
        }
    }
    
    #[test]
    fn test_counting_multiple_paths() {
        let patterns = vec!["r", "rr"];
        let design = "rr";
        
        let mut memo = HashMap::new();
        let count = count_ways(&patterns, design, &mut memo);
        
        // Two ways: "r" + "r" OR "rr"
        assert_eq!(count, 2);
    }
    
    #[test]
    fn test_counting_base_cases() {
        let patterns = vec!["a"];
        let mut memo = HashMap::new();
        
        // Empty string: 1 way
        assert_eq!(count_ways(&patterns, "", &mut memo), 1);
        
        // Impossible: 0 ways
        assert_eq!(count_ways(&patterns, "b", &mut memo), 0);
    }
    
    #[test]
    fn test_fibonacci_counting() {
        // Ways to reach N with steps 1 or 2
        assert_eq!(fib_count_ways(0), 1);  // One way: do nothing
        assert_eq!(fib_count_ways(1), 1);  // One way: step 1
        assert_eq!(fib_count_ways(2), 2);  // Two ways: 1+1 or 2
        assert_eq!(fib_count_ways(3), 3);  // Three ways
        assert_eq!(fib_count_ways(4), 5);  // Five ways
    }
}
