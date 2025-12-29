//! Step 1: Naive Recursion - Experience Exponential Complexity
//!
//! **Learning Objective**: By the end of this step, you will understand why dynamic
//! programming is necessary by experiencing exponential complexity firsthand.
//!
//! **Prerequisites**: Basic recursion understanding (Mission 1), string operations
//!
//! **Next Step Preview**: Step 2 will add memoization to transform this exponential
//! algorithm into a linear one with minimal code changes.

use std::time::Instant;

fn main() {
    println!("=== Step 1: Naive Recursion ===\n");
    
    demonstrate_core_concept();
    show_common_patterns();
    handle_edge_cases();
    validate_understanding();
    
    println!("\n✅ Step 1 completed! Ready for Step 2.");
    println!("💡 Key Insight: Without caching, we solve the same subproblem exponentially many times!");
}

/// Core concept demonstration
///
/// This function shows the fundamental recursive pattern for pattern matching.
/// Key learning: Recursive subproblem decomposition without optimization.
fn demonstrate_core_concept() {
    println!("--- Core Concept: Pattern Matching with Recursion ---");
    
    // Simple example from AoC 2024 Day 19
    let patterns = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
    let design = "brwrr";
    
    println!("Problem: Can we make '{}' from patterns {:?}?", design, patterns);
    
    let start = Instant::now();
    let result = can_make(&patterns, design);
    let duration = start.elapsed();
    
    println!("Result: {} (took {:?})", result, duration);
    println!("Key Insight: Each recursive call checks if a prefix matches, then recurses on remainder");
}

/// Basic recursive pattern matching
///
/// Checks if a design string can be constructed from available patterns.
/// Uses recursive decomposition: try each pattern as prefix, recurse on suffix.
fn can_make(patterns: &[&str], design: &str) -> bool {
    // Base case: empty string can always be made
    if design.is_empty() {
        return true;
    }
    
    // Try each pattern as a potential prefix
    for pattern in patterns {
        if let Some(rest) = design.strip_prefix(pattern) {
            // Recursively check if the remainder can be made
            if can_make(patterns, rest) {
                return true;
            }
        }
    }
    
    // No pattern worked as a prefix
    false
}

/// Common usage patterns
///
/// Shows how the core concept applies in typical scenarios.
/// Builds confidence through repeated application.
fn show_common_patterns() {
    println!("\n--- Common Patterns ---");
    
    pattern_1_short_designs();
    pattern_2_impossible_designs();
    pattern_3_multiple_solutions();
}

fn pattern_1_short_designs() {
    println!("\nPattern 1: Short designs (fast)");
    let patterns = vec!["r", "wr", "b"];
    
    let test_cases = vec![
        ("r", true),
        ("wr", true),
        ("wrr", true),
        ("brr", true),
        ("bwr", true),
    ];
    
    for (design, expected) in test_cases {
        let result = can_make(&patterns, design);
        println!("  '{}' -> {} (expected: {})", design, result, expected);
        assert_eq!(result, expected);
    }
}

fn pattern_2_impossible_designs() {
    println!("\nPattern 2: Impossible designs");
    let patterns = vec!["r", "wr", "b"];
    
    // These cannot be made from the patterns
    let impossible = vec!["g", "rg", "bwg"];
    
    for design in impossible {
        let result = can_make(&patterns, design);
        println!("  '{}' -> {} (should be false)", design, result);
        assert!(!result);
    }
}

fn pattern_3_multiple_solutions() {
    println!("\nPattern 3: Multiple solution paths");
    let patterns = vec!["r", "wr", "b", "br"];
    
    // "brr" can be made as: b+r+r OR br+r
    let design = "brr";
    let result = can_make(&patterns, design);
    println!("  '{}' can be made: {} (has multiple paths)", design, result);
    println!("  Note: Algorithm stops at first successful path (doesn't count all)");
    assert!(result);
}

/// Edge case handling
///
/// Demonstrates what happens in boundary conditions.
/// Critical for building robust understanding.
fn handle_edge_cases() {
    println!("\n--- Edge Cases ---");
    
    empty_design_case();
    single_pattern_case();
    no_matching_patterns_case();
}

fn empty_design_case() {
    println!("\nEdge Case 1: Empty design");
    let patterns = vec!["r", "wr"];
    let design = "";
    
    let result = can_make(&patterns, design);
    println!("  Empty design: {} (base case returns true)", result);
    assert!(result);
}

fn single_pattern_case() {
    println!("\nEdge Case 2: Single pattern that matches exactly");
    let patterns = vec!["hello"];
    let design = "hello";
    
    let result = can_make(&patterns, design);
    println!("  Exact match: {}", result);
    assert!(result);
}

fn no_matching_patterns_case() {
    println!("\nEdge Case 3: No patterns match at all");
    let patterns = vec!["a", "b", "c"];
    let design = "xyz";
    
    let result = can_make(&patterns, design);
    println!("  No matches: {} (exhausts all options)", result);
    assert!(!result);
}

/// Understanding validation
///
/// Self-check exercises to ensure learning objectives achieved.
/// Student should be able to predict all outputs before running.
fn validate_understanding() {
    println!("\n--- Validation Exercises ---");
    
    println!("\n🤔 Before running: Can you predict these outputs?");
    
    // Scenario 1: Simple case
    let patterns1 = vec!["ab", "cd"];
    let design1 = "abcd";
    println!("\nScenario 1: patterns={:?}, design='{}'", patterns1, design1);
    println!("  Prediction: _____ (true/false)");
    let result1 = can_make(&patterns1, design1);
    println!("  Actual: {}", result1);
    
    // Scenario 2: Overlapping patterns
    let patterns2 = vec!["a", "aa", "aaa"];
    let design2 = "aaaa";
    println!("\nScenario 2: patterns={:?}, design='{}'", patterns2, design2);
    println!("  Prediction: _____ (true/false)");
    let result2 = can_make(&patterns2, design2);
    println!("  Actual: {}", result2);
    
    // Scenario 3: Longer design (starts to get slow!)
    let patterns3 = vec!["r", "wr", "b", "g", "br"];
    let design3 = "bwrrg";
    println!("\nScenario 3: patterns={:?}, design='{}'", patterns3, design3);
    println!("  Prediction: _____ (true/false)");
    let result3 = can_make(&patterns3, design3);
    println!("  Actual: {}", result3);
    
    println!("\n✅ If you predicted all correctly, you understand recursive decomposition!");
    println!("🔄 If not, review the demonstrate_core_concept() function above.");
    
    // Performance demonstration
    println!("\n--- Performance Reality Check ---");
    demonstrate_exponential_growth();
}

/// Demonstrates exponential time complexity with increasing input sizes
fn demonstrate_exponential_growth() {
    let patterns = vec!["r", "wr", "b", "g", "br", "rb"];
    
    // Progressively longer designs
    let test_sizes = vec![
        ("brwrr", 5),           // Very fast
        ("bwrrrbgbr", 10),      // Still fast
        ("rbbrwgbrwrr", 12),    // Getting slower
    ];
    
    println!("\nMeasuring time complexity growth:");
    println!("(Warning: Don't try designs longer than 15 chars without memoization!)");
    
    for (design, length) in test_sizes {
        let start = Instant::now();
        let result = can_make(&patterns, design);
        let duration = start.elapsed();
        
        println!("  Length {}: {:?} (result: {})", length, duration, result);
    }
    
    println!("\n💡 Notice: Time grows exponentially with design length!");
    println!("   This is because we solve the same subproblems over and over.");
    println!("   Example: For 'brwrr', we might check 'wrr' multiple times.");
}

#[cfg(test)]
mod step_tests {
    use super::*;
    
    #[test]
    fn test_basic_pattern_matching() {
        let patterns = vec!["r", "wr", "b"];
        
        assert!(can_make(&patterns, "r"));
        assert!(can_make(&patterns, "wr"));
        assert!(can_make(&patterns, "wrr"));
        assert!(can_make(&patterns, "bwr"));
    }
    
    #[test]
    fn test_impossible_designs() {
        let patterns = vec!["r", "wr", "b"];
        
        assert!(!can_make(&patterns, "g"));
        assert!(!can_make(&patterns, "x"));
        assert!(!can_make(&patterns, "rwg"));
    }
    
    #[test]
    fn test_empty_design() {
        let patterns = vec!["r", "wr"];
        assert!(can_make(&patterns, ""));
    }
    
    #[test]
    fn test_multiple_solution_paths() {
        let patterns = vec!["r", "br", "b"];
        
        // "brr" can be made as either "b" + "r" + "r" or "br" + "r"
        assert!(can_make(&patterns, "brr"));
    }
    
    #[test]
    fn test_overlapping_patterns() {
        let patterns = vec!["a", "aa", "aaa"];
        
        // Should be able to make any number of 'a's
        assert!(can_make(&patterns, "a"));
        assert!(can_make(&patterns, "aa"));
        assert!(can_make(&patterns, "aaa"));
        assert!(can_make(&patterns, "aaaa"));
    }
    
    #[test]
    fn test_no_patterns() {
        let patterns: Vec<&str> = vec![];
        
        // Only empty design can be made with no patterns
        assert!(can_make(&patterns, ""));
        assert!(!can_make(&patterns, "anything"));
    }
}
