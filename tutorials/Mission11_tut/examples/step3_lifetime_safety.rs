//! Step 3: Lifetime Safety - Master Borrowed Keys in Memoization
//!
//! **Learning Objective**: By the end of this step, you will understand lifetime
//! parameters in recursive functions and why they're needed for zero-copy caching.
//!
//! **Prerequisites**: Step 2 (HashMap memoization), basic lifetime understanding
//!
//! **Next Step Preview**: Step 4 will generalize this pattern into a reusable cache
//! abstraction that works across different problem types.

use std::collections::HashMap;

fn main() {
    println!("=== Step 3: Lifetime Safety ===\n");
    
    demonstrate_core_concept();
    show_common_patterns();
    handle_edge_cases();
    validate_understanding();
    
    println!("\n✅ Step 3 completed! Ready for Step 4.");
    println!("💡 Key Insight: Lifetimes enable zero-copy caching without allocation!");
}

/// Core concept demonstration
///
/// Shows why lifetime parameters are necessary and what they guarantee.
/// Key learning: Lifetime connects input string to cached keys.
fn demonstrate_core_concept() {
    println!("--- Core Concept: Understanding Lifetimes in Memoization ---");
    
    println!("\n1. The Problem: Why do we need lifetimes?");
    println!("   HashMap stores &str references as keys");
    println!("   References must be proven to live long enough");
    println!("   Compiler needs to know: How long do cache keys live?");
    
    println!("\n2. The Solution: Lifetime parameter 'a");
    println!("   fn can_make<'a>(design: &'a str, memo: &mut HashMap<&'a str, bool>)");
    println!("   This says: 'design' and cache keys share the same lifetime");
    
    println!("\n3. Why this works:");
    println!("   - Cache keys are substrings of 'design' (via strip_prefix)");
    println!("   - If 'design' lives long enough, all its substrings do too");
    println!("   - Compiler verifies: cache can't outlive the source string");
    
    demonstrate_zero_copy_advantage();
}

/// Demonstrates the performance advantage of zero-copy caching
fn demonstrate_zero_copy_advantage() {
    println!("\n--- Zero-Copy vs Allocating Keys ---");
    
    let patterns = vec!["r", "wr", "b", "br"];
    let design = "brwrrbrwrrbrwrr";
    
    // Zero-copy with &str keys (what we use)
    let mut memo_borrowed: HashMap<&str, bool> = HashMap::new();
    let result1 = can_make_with_lifetimes(&patterns, design, &mut memo_borrowed);
    
    println!("Zero-copy (&str keys):");
    println!("  Result: {}", result1);
    println!("  Memory: Stores only references, no string allocation");
    println!("  Cache keys point into original 'design' string");
    
    // Compare to String keys (allocating version)
    let mut memo_owned: HashMap<String, bool> = HashMap::new();
    let result2 = can_make_with_owned_keys(&patterns, design, &mut memo_owned);
    
    println!("\nAllocating (String keys):");
    println!("  Result: {}", result2);
    println!("  Memory: Each key is a heap-allocated String");
    println!("  Cache keys are copies of substrings");
    
    println!("\nLifetime version is more efficient:");
    println!("  ✓ No heap allocations for keys");
    println!("  ✓ Better cache locality");
    println!("  ✓ Faster key insertion and lookup");
}

/// Correct version with lifetime parameters
fn can_make_with_lifetimes<'a>(
    patterns: &[&str],
    design: &'a str,              // 'a: lifetime of input
    memo: &mut HashMap<&'a str, bool>,  // Cache keys share lifetime 'a
) -> bool {
    if design.is_empty() {
        return true;
    }
    
    if let Some(&result) = memo.get(design) {
        return result;
    }
    
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            // 'rest' is a substring of 'design', so it also has lifetime 'a
            if can_make_with_lifetimes(patterns, rest, memo) {
                memo.insert(design, true);
                return true;
            }
        }
    }
    
    memo.insert(design, false);
    false
}

/// Alternative with owned String keys (allocates memory)
fn can_make_with_owned_keys(
    patterns: &[&str],
    design: &str,
    memo: &mut HashMap<String, bool>,
) -> bool {
    if design.is_empty() {
        return true;
    }
    
    // Must convert &str to String for lookup
    if let Some(&result) = memo.get(design) {
        return result;
    }
    
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            if can_make_with_owned_keys(patterns, rest, memo) {
                memo.insert(design.to_string(), true);  // Allocates!
                return true;
            }
        }
    }
    
    memo.insert(design.to_string(), false);  // Allocates!
    false
}

/// Common usage patterns
///
/// Shows how lifetimes propagate through recursive calls.
fn show_common_patterns() {
    println!("\n--- Common Patterns ---");
    
    pattern_1_lifetime_propagation();
    pattern_2_multiple_lifetimes();
    pattern_3_compiler_errors();
}

fn pattern_1_lifetime_propagation() {
    println!("\nPattern 1: Lifetime propagation in recursion");
    
    println!("  When we call: can_make_with_lifetimes(patterns, rest, memo)");
    println!("  - 'rest' is a substring of original 'design'");
    println!("  - 'rest' automatically has lifetime 'a");
    println!("  - Lifetime flows through all recursive calls");
    println!("  - Compiler verifies safety automatically");
    
    let patterns = vec!["ab", "cd"];
    let design = "abcd";
    let mut memo = HashMap::new();
    
    let result = can_make_with_lifetimes(&patterns, design, &mut memo);
    println!("\n  Example: design='{}', result={}", design, result);
    println!("  All cache keys ('cd', 'd', '') are substrings of 'abcd'");
}

fn pattern_2_multiple_lifetimes() {
    println!("\nPattern 2: Why patterns don't need lifetime 'a");
    
    println!("  patterns: &[&str]  - No lifetime parameter needed");
    println!("  design: &'a str    - Needs lifetime 'a");
    println!("\n  Reason:");
    println!("  - Patterns are only read, never stored in cache");
    println!("  - Cache keys come from 'design', not from 'patterns'");
    println!("  - Only 'design' substrings need lifetime tracking");
    
    // This works fine - patterns can have different lifetime than design
    let patterns = vec!["x", "y"];
    let design_string = String::from("xy");
    let mut memo = HashMap::new();
    
    let result = can_make_with_lifetimes(&patterns, &design_string, &mut memo);
    println!("\n  Example: patterns and design have independent lifetimes: {}", result);
}

fn pattern_3_compiler_errors() {
    println!("\nPattern 3: Common compiler errors (and how to fix them)");
    
    println!("\n  ERROR: 'design' doesn't live long enough");
    println!("  ❌ fn bad<'a>(patterns: &[&str], design: &str, memo: &mut HashMap<&'a str, bool>)");
    println!("  Problem: 'design' has no lifetime, but cache keys need 'a");
    println!("\n  ✅ fn good<'a>(patterns: &[&str], design: &'a str, memo: &mut HashMap<&'a str, bool>)");
    println!("  Solution: Add 'a to 'design' parameter");
    
    println!("\n  ERROR: lifetime mismatch");
    println!("  ❌ fn bad(design: &str, memo: &mut HashMap<&'static str, bool>)");
    println!("  Problem: Cache requires 'static lifetime, but design doesn't live forever");
    println!("\n  ✅ fn good<'a>(design: &'a str, memo: &mut HashMap<&'a str, bool>)");
    println!("  Solution: Use same lifetime 'a for both");
}

/// Edge case handling
///
/// Demonstrates lifetime behavior in special cases.
fn handle_edge_cases() {
    println!("\n--- Edge Cases ---");
    
    cache_outlives_function();
    multiple_inputs_same_cache();
    empty_string_lifetime();
}

fn cache_outlives_function() {
    println!("\nEdge Case 1: Cache can outlive function calls (with 'static strings)");
    
    let patterns = vec!["r", "wr"];
    let mut memo = HashMap::new();
    
    {
        let design1 = "wrr";  // String literal - has 'static lifetime!
        can_make_with_lifetimes(&patterns, design1, &mut memo);
        println!("  After first call: cache has {} entries", memo.len());
    }
    // design1 VARIABLE dropped, but "wrr" DATA lives forever ('static)
    // Cache keys still valid because they point to 'static string literal
    
    {
        let design2 = "wr";   // Another 'static string literal
        can_make_with_lifetimes(&patterns, design2, &mut memo);
        println!("  After second call: cache has {} entries", memo.len());
    }
    // design2 VARIABLE dropped, but "wr" DATA still lives ('static)
    
    println!("  ✅ Works because: String literals have 'static lifetime");
    println!("  ⚠️  Would FAIL with String::from() - data freed when dropped!");
    
    println!("\n  Example that would NOT compile:");
    println!("     let mut memo = HashMap::new();");
    println!("     {{");
    println!("         let design = String::from(\"wrr\");  // Heap allocated");
    println!("         solve(&design, &mut memo);");
    println!("     }}  // ERROR: design freed, but memo has references to it!");
}

fn multiple_inputs_same_cache() {
    println!("\nEdge Case 2: Mixing string literals in same cache (works!)");
    
    println!("  ✅ This DOES compile (both 'static):");
    println!("     let design1 = \"abc\";              // 'static");
    println!("     let design2 = \"xyz\";              // 'static");
    println!("     let mut memo = HashMap::new();");
    println!("     solve(design1, &mut memo);");
    println!("     solve(design2, &mut memo);  // OK - both 'static");
    
    println!("\n  ❌ This won't compile (heap allocated):");
    println!("     let design1 = String::from(\"abc\");");
    println!("     let design2 = String::from(\"xyz\");");
    println!("     let mut memo = HashMap::new();");
    println!("     solve(&design1, &mut memo);      // memo borrows design1");
    println!("     solve(&design2, &mut memo);      // ERROR: memo already borrowed!");
    
    println!("\n  Problem: Can't have two different lifetimes in same HashMap");
    println!("  - Keys from design1 have lifetime 'a1");
    println!("  - Keys from design2 have lifetime 'a2");
    println!("  - HashMap<&'a str, bool> can only have ONE lifetime 'a");
    
    println!("\n  Solutions:");
    println!("  - Use 'static string literals (both 'a = 'static)");
    println!("  - Use separate caches for each source string");
    println!("  - Use owned String keys (allocates, but flexible)");
}

fn empty_string_lifetime() {
    println!("\nEdge Case 3: Empty string literal has 'static lifetime");
    
    let patterns = vec!["r"];
    let mut memo: HashMap<&str, bool> = HashMap::new();
    
    // Empty string literal has 'static lifetime
    can_make_with_lifetimes(&patterns, "", &mut memo);
    
    println!("  Empty string \"\" has 'static lifetime");
    println!("  Works with any lifetime parameter 'a");
    println!("  ('static outlives everything)");
}

/// Understanding validation
///
/// Exercises to validate lifetime understanding.
fn validate_understanding() {
    println!("\n--- Validation Exercises ---");
    
    println!("\n🤔 Lifetime comprehension check:");
    
    // Exercise 1
    println!("\nExercise 1: Will this compile?");
    println!("  fn mystery<'a>(x: &str, cache: &mut HashMap<&'a str, bool>) -> bool");
    println!("  Prediction: _____ (yes/no)");
    println!("  Actual: NO - 'x' needs lifetime 'a to insert into cache");
    
    // Exercise 2
    println!("\nExercise 2: What does 'a represent?");
    println!("  fn solve<'a>(design: &'a str, memo: &mut HashMap<&'a str, bool>)");
    println!("  a) Lifetime of the function");
    println!("  b) Lifetime of the cache");
    println!("  c) Lifetime of the input string and its substrings");
    println!("  Actual: c - 'a is the lifetime of 'design' and all its substrings");
    
    // Exercise 3
    println!("\nExercise 3: Why use &str instead of String for keys?");
    println!("  a) &str is faster to type");
    println!("  b) Zero-copy - no allocation needed");
    println!("  c) HashMap requires it");
    println!("  Actual: b - Avoids heap allocation for every cache key");
    
    println!("\n✅ If you answered correctly, you understand lifetimes!");
    println!("🔄 If confused, review demonstrate_core_concept() section.");
}

#[cfg(test)]
mod step_tests {
    use super::*;
    
    #[test]
    fn test_lifetime_version_correctness() {
        let patterns = vec!["r", "wr", "b"];
        let design = "wrr";
        let mut memo = HashMap::new();
        
        let result = can_make_with_lifetimes(&patterns, design, &mut memo);
        assert!(result);
    }
    
    #[test]
    fn test_owned_version_correctness() {
        let patterns = vec!["r", "wr", "b"];
        let design = "wrr";
        let mut memo = HashMap::new();
        
        let result = can_make_with_owned_keys(&patterns, design, &mut memo);
        assert!(result);
    }
    
    #[test]
    fn test_cache_survives_scope() {
        let patterns = vec!["r", "wr"];
        let mut memo = HashMap::new();
        
        {
            let design = "wrr";
            can_make_with_lifetimes(&patterns, design, &mut memo);
        }
        // memo still valid here, has entries from previous call
        assert!(memo.len() > 0);
    }
    
    #[test]
    fn test_substring_lifetimes() {
        let design_string = String::from("wrr");
        let patterns = vec!["r", "wr"];
        let mut memo = HashMap::new();
        
        let result = can_make_with_lifetimes(&patterns, &design_string, &mut memo);
        assert!(result);
        
        // Cache keys are valid as long as design_string is valid
        assert!(memo.contains_key("rr") || memo.contains_key("r"));
    }
}
