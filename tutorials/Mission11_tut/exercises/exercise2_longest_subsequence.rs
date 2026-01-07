//! Exercise 2: Longest Common Subsequence (LCS)
//!
//! **Learning Goals**:
//! - Practice 2D state space (two indices)
//! - Use HashMap<(usize, usize), T> for memoization
//! - Compare space complexity: HashMap vs 2D Vec
//! - Understand recurrence relations
//!
//! **Problem**: Find the length of the longest common subsequence between two strings.
//!
//! **IMPORTANT**: A subsequence can have letters in between as long as the order is preserved.
//! This is different from a substring, which must be contiguous.
//!
//! **Example**:
//! - s1 = "ABCDEF", s2 = "ADBEF" → LCS = "ABEF" (length 4)
//!   - "ABEF" appears in both: A-B---EF and A-D-B-EF (letters can be skipped)
//! - s1 = "AGGTAB", s2 = "GXTXAYB" → LCS = "GTAB" (length 4)
//!   - "GTAB" appears as: G--T-AB and G-XT-A-YB (non-contiguous is fine!)
//!
//! **Recurrence**:
//! ```text
//! lcs(i, j) = {
//!     0                                if i >= len(s1) or j >= len(s2)
//!     1 + lcs(i+1, j+1)               if s1[i] == s2[j]
//!     max(lcs(i+1, j), lcs(i, j+1))   otherwise
//! }
//! ```

use std::collections::HashMap;

fn main() {
    println!("=== Exercise 2: Longest Common Subsequence ===\n");

    demonstrate_naive_step_by_step();
    demonstrate_lcs();
    compare_approaches();
    test_edge_cases();

    println!("\n✅ Exercise 2 completed!");
}

// ============================================================================
// TODO: Implement these 3 approaches
// ============================================================================

/// Approach 1: Naive Recursion (SLOW - exponential)
///
/// Try with short strings first to see the explosion
fn lcs_naive(s1: &str, s2: &str) -> usize {
    lcs_naive_helper(s1.as_bytes(), s2.as_bytes(), 0, 0)
}

fn lcs_naive_helper(s1: &[u8], s2: &[u8], i: usize, j: usize) -> usize {
    // TODO: Implement naive recursion
    // 1. Base case: if i >= s1.len() or j >= s2.len(), return 0
    // 2. If s1[i] == s2[j], return 1 + lcs(i+1, j+1)
    // 3. Otherwise, return max(lcs(i+1, j), lcs(i, j+1))
    if i >= s1.len() || j >= s2.len() {
        return 0;
    }

    if s1[i] == s2[j] {
        1 + lcs_naive_helper(s1, s2, i + 1, j + 1)
    } else {
        let skip_s1 = lcs_naive_helper(s1, s2, i + 1, j);
        let skip_s2 = lcs_naive_helper(s1, s2, i, j + 1);
        skip_s1.max(skip_s2)
    }
}

/// Approach 2: Top-Down with HashMap (FAST - O(m×n))
///
/// Use HashMap<(usize, usize), usize> to cache results
fn lcs_memoized(s1: &str, s2: &str) -> usize {
    let mut memo = HashMap::new();
    lcs_memo_helper(s1.as_bytes(), s2.as_bytes(), 0, 0, &mut memo)
}

fn lcs_memo_helper(
    s1: &[u8],
    s2: &[u8],
    i: usize,
    j: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // TODO: Implement memoized version
    // 1. Check base case
    // 2. Check memo with key (i, j)
    // 3. Compute recursively if not cached
    // 4. Store in memo before returning
    if i >= s1.len() || j >= s2.len() {
        return 0;
    }

    if let Some(&result) = memo.get(&(i, j)) {
        return result;
    }

    let result = if s1[i] == s2[j] {
        1 + lcs_memo_helper(s1, s2, i + 1, j + 1, memo)
    } else {
        let skip_s1 = lcs_memo_helper(s1, s2, i + 1, j, memo);
        let skip_s2 = lcs_memo_helper(s1, s2, i, j + 1, memo);
        skip_s1.max(skip_s2)
    };

    memo.insert((i, j), result);
    result
}

/// Approach 3: Bottom-Up with 2D Table (FAST - O(m×n), no recursion)
///
/// Build table from bottom-right to top-left (or reverse)
fn lcs_bottom_up(s1: &str, s2: &str) -> usize {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let m = s1_bytes.len();
    let n = s2_bytes.len();

    // TODO: Implement bottom-up DP
    // 1. Create (m+1) × (n+1) table initialized to 0
    // 2. Fill table from bottom-right to top-left
    //    dp[i][j] = 1 + dp[i+1][j+1] if s1[i] == s2[j]
    //    dp[i][j] = max(dp[i+1][j], dp[i][j+1]) otherwise
    // 3. Return dp[0][0]
    if m == 0 || n == 0 {
        return 0;
    }

    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Fill from bottom-right to top-left
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if s1_bytes[i] == s2_bytes[j] {
                dp[i][j] = 1 + dp[i + 1][j + 1];
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    dp[0][0]
}

// ============================================================================
// BONUS: Space-Optimized Version
// ============================================================================

/// Approach 4: Space-Optimized O(min(m, n))
///
/// Only need current and previous row/column
fn lcs_space_optimized(s1: &str, s2: &str) -> usize {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();

    // Ensure s2 is the shorter one (optimize space based on shorter string)
    let (shorter, longer) = if s1_bytes.len() <= s2_bytes.len() {
        (s1_bytes, s2_bytes)
    } else {
        (s2_bytes, s1_bytes)
    };

    // TODO: Implement space-optimized version
    // 1. Create two arrays of size n+1 (where n = shorter.len())
    // 2. Process longer string one character at a time
    // 3. Swap curr and prev after each iteration
    // Hint: Only need O(min(m, n)) space instead of O(m×n)
    let n = shorter.len();
    let mut prev = vec![0; n + 1];
    let mut curr = vec![0; n + 1];

    for i in (0..longer.len()).rev() {
        for j in (0..n).rev() {
            curr[j] = if longer[i] == shorter[j] {
                1 + prev[j + 1]
            } else {
                curr[j + 1].max(prev[j])
            };
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[0]
}

// ============================================================================
// BONUS: Reconstruct the actual LCS string (not just length)
// ============================================================================

/// Reconstruct the actual LCS string
fn lcs_reconstruct(s1: &str, s2: &str) -> String {
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let m = s1_bytes.len();
    let n = s2_bytes.len();

    // TODO: Build DP table, then backtrack to reconstruct string
    // 1. Build full DP table
    // 2. Starting from dp[0][0], follow the path:
    //    - If s1[i] == s2[j], add to result and move to (i+1, j+1)
    //    - Otherwise, move in direction of larger value
    if m == 0 || n == 0 {
        return String::new();
    }

    // Build DP table
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if s1_bytes[i] == s2_bytes[j] {
                dp[i][j] = 1 + dp[i + 1][j + 1];
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    // Backtrack to reconstruct
    let mut result = String::new();
    let mut i = 0;
    let mut j = 0;

    while i < m && j < n {
        if s1_bytes[i] == s2_bytes[j] {
            result.push(s1_bytes[i] as char);
            i += 1;
            j += 1;
        } else if dp[i + 1][j] > dp[i][j + 1] {
            i += 1;
        } else {
            j += 1;
        }
    }

    result
}

// ============================================================================
// Tests and Demonstrations
// ============================================================================

/// Step-by-step demonstration of naive recursion with short strings
fn demonstrate_naive_step_by_step() {
    println!("--- Approach 1: Naive Recursion Step-by-Step ---\n");
    
    let s1 = "ABC";
    let s2 = "AC";
    
    println!("Finding LCS of '{}' and '{}'", s1, s2);
    println!("Expected: 'AC' (length 2)\n");
    
    let result = lcs_naive_traced(s1, s2);
    
    println!("\n✓ Final Result: LCS length = {}", result);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Now show how to RECONSTRUCT the actual subsequence
    demonstrate_reconstruction(s1, s2);
}

/// Traced version of naive LCS that shows each step
fn lcs_naive_traced(s1: &str, s2: &str) -> usize {
    lcs_naive_traced_helper(s1.as_bytes(), s2.as_bytes(), 0, 0, 0)
}

fn lcs_naive_traced_helper(s1: &[u8], s2: &[u8], i: usize, j: usize, depth: usize) -> usize {
    let indent = "  ".repeat(depth);
    
    // Show current call
    let s1_char = if i < s1.len() { 
        format!("'{}'", s1[i] as char) 
    } else { 
        "END".to_string() 
    };
    let s2_char = if j < s2.len() { 
        format!("'{}'", s2[j] as char) 
    } else { 
        "END".to_string() 
    };
    
    println!("{}lcs(i={}, j={}) → s1[{}]={}, s2[{}]={}", 
             indent, i, j, i, s1_char, j, s2_char);
    
    // Base case
    if i >= s1.len() || j >= s2.len() {
        println!("{}  → BASE CASE: return 0", indent);
        return 0;
    }
    
    // Characters match
    if s1[i] == s2[j] {
        println!("{}  ✓ MATCH! '{}' == '{}'", indent, s1[i] as char, s2[j] as char);
        println!("{}  → Computing: 1 + lcs({}, {})", indent, i+1, j+1);
        let result = 1 + lcs_naive_traced_helper(s1, s2, i + 1, j + 1, depth + 1);
        println!("{}  ← Returns: {}", indent, result);
        result
    } else {
        println!("{}  ✗ NO MATCH: '{}' != '{}'", indent, s1[i] as char, s2[j] as char);
        println!("{}  → Try BOTH paths:", indent);
        
        println!("{}    Path A: Skip s1[{}] (advance i)", indent, i);
        let skip_s1 = lcs_naive_traced_helper(s1, s2, i + 1, j, depth + 1);
        
        println!("{}    Path B: Skip s2[{}] (advance j)", indent, j);
        let skip_s2 = lcs_naive_traced_helper(s1, s2, i, j + 1, depth + 1);
        
        let result = skip_s1.max(skip_s2);
        println!("{}  ← Max({}, {}) = {}", indent, skip_s1, skip_s2, result);
        result
    }
}

/// Demonstrate how to reconstruct the actual subsequence (not just length)
fn demonstrate_reconstruction(s1: &str, s2: &str) {
    println!("--- Reconstructing the Actual Subsequence ---\n");
    println!("Strings: '{}' and '{}'", s1, s2);
    
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();
    let m = s1_bytes.len();
    let n = s2_bytes.len();
    
    // Step 1: Build the DP table
    println!("\nStep 1: Build DP table (same as bottom-up approach)");
    let mut dp = vec![vec![0; n + 1]; m + 1];
    
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if s1_bytes[i] == s2_bytes[j] {
                dp[i][j] = 1 + dp[i + 1][j + 1];
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }
    
    // Print the DP table
    println!("\nDP Table (dp[i][j] = LCS length from position i,j onwards):");
    print!("     ");
    for j in 0..=n {
        if j < n {
            print!("  {:2} ", s2_bytes[j] as char);
        } else {
            print!(" END ");
        }
    }
    println!();
    
    for i in 0..=m {
        if i < m {
            print!("{:2}:  ", s1_bytes[i] as char);
        } else {
            print!("END: ");
        }
        for j in 0..=n {
            print!("  {:2} ", dp[i][j]);
        }
        println!();
    }
    
    // Step 2: Backtrack through the table
    println!("\nStep 2: Backtrack to reconstruct the subsequence");
    let mut result = String::new();
    let mut i = 0;
    let mut j = 0;
    
    while i < m && j < n {
        let s1_char = s1_bytes[i] as char;
        let s2_char = s2_bytes[j] as char;
        
        if s1_bytes[i] == s2_bytes[j] {
            println!("  At ({},{}): '{}' == '{}' → INCLUDE in LCS", 
                     i, j, s1_char, s2_char);
            println!("             Moving diagonally to ({},{})", i+1, j+1);
            result.push(s1_bytes[i] as char);
            i += 1;
            j += 1;
        } else if dp[i + 1][j] > dp[i][j + 1] {
            println!("  At ({},{}): '{}' != '{}' → dp[{}][{}]={} > dp[{}][{}]={}", 
                     i, j, s1_char, s2_char, 
                     i+1, j, dp[i+1][j], 
                     i, j+1, dp[i][j+1]);
            println!("             Skip s1[{}]='{}', move down to ({},{})", 
                     i, s1_char, i+1, j);
            i += 1;
        } else {
            println!("  At ({},{}): '{}' != '{}' → dp[{}][{}]={} >= dp[{}][{}]={}", 
                     i, j, s1_char, s2_char,
                     i, j+1, dp[i][j+1],
                     i+1, j, dp[i+1][j]);
            println!("             Skip s2[{}]='{}', move right to ({},{})", 
                     j, s2_char, i, j+1);
            j += 1;
        }
    }
    
    println!("\n✓ Reconstructed LCS: \"{}\"", result);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
}

fn demonstrate_lcs() {
    println!("--- Core Concept: LCS Examples ---\n");

    let test_cases = vec![
        ("ABCDEF", "ADBEF"),
        ("AGGTAB", "GXTXAYB"),
        ("ABCDGH", "AEDFHR"),
        ("ABC", "AC"),
        ("ABC", "DEF"),
    ];

    for (s1, s2) in test_cases {
        let length = lcs_bottom_up(s1, s2);
        let sequence = lcs_reconstruct(s1, s2);
        println!("LCS('{}', '{}') = {} ('{}')", s1, s2, length, sequence);
    }
}

fn compare_approaches() {
    println!("\n--- Performance Comparison ---\n");

    let test_cases = vec![
        ("ABCDEF", "ADBEF"),               // Small
        ("PROGRAMMING", "GAMING"),         // Medium
        ("ABCDEFGHIJKLMNOP", "ACEGIKMOQ"), // Large for naive
    ];

    for (s1, s2) in test_cases {
        println!("Comparing '{}' vs '{}':", s1, s2);

        // Naive (only for small inputs)
        if s1.len() <= 10 && s2.len() <= 10 {
            let start = std::time::Instant::now();
            let result = lcs_naive(s1, s2);
            let duration = start.elapsed();
            println!("  Naive:      {} (took {:?})", result, duration);
        } else {
            println!("  Naive:      <skipped - too slow>");
        }

        // Memoized
        let start = std::time::Instant::now();
        let result = lcs_memoized(s1, s2);
        let duration = start.elapsed();
        println!("  Memoized:   {} (took {:?})", result, duration);

        // Bottom-up
        let start = std::time::Instant::now();
        let result = lcs_bottom_up(s1, s2);
        let duration = start.elapsed();
        println!("  Bottom-up:  {} (took {:?})", result, duration);

        // Space-optimized
        let start = std::time::Instant::now();
        let result = lcs_space_optimized(s1, s2);
        let duration = start.elapsed();
        println!("  Optimized:  {} (took {:?})", result, duration);

        println!();
    }
}

fn test_edge_cases() {
    println!("--- Edge Cases ---\n");

    let edge_cases = vec![
        ("", "ABC", "Empty first string"),
        ("ABC", "", "Empty second string"),
        ("", "", "Both empty"),
        ("AAA", "AAA", "Identical strings"),
        ("ABC", "DEF", "No common chars"),
        ("A", "A", "Single char match"),
        ("AAAA", "AA", "Repeated chars"),
    ];

    for (s1, s2, description) in edge_cases {
        let result = lcs_bottom_up(s1, s2);
        println!("{:30} → LCS = {}", description, result);
    }
}

// ============================================================================
// SOLUTIONS (Don't peek until you've tried!)
// ============================================================================

#[cfg(test)]
mod solutions {
    use super::*;

    // Solution 1: Naive Recursion
    #[allow(dead_code)]
    fn lcs_naive_solution_helper(s1: &[u8], s2: &[u8], i: usize, j: usize) -> usize {
        if i >= s1.len() || j >= s2.len() {
            return 0;
        }

        if s1[i] == s2[j] {
            1 + lcs_naive_solution_helper(s1, s2, i + 1, j + 1)
        } else {
            let skip_s1 = lcs_naive_solution_helper(s1, s2, i + 1, j);
            let skip_s2 = lcs_naive_solution_helper(s1, s2, i, j + 1);
            skip_s1.max(skip_s2)
        }
    }

    // Solution 2: Memoized
    #[allow(dead_code)]
    fn lcs_memo_solution_helper(
        s1: &[u8],
        s2: &[u8],
        i: usize,
        j: usize,
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if i >= s1.len() || j >= s2.len() {
            return 0;
        }

        if let Some(&result) = memo.get(&(i, j)) {
            return result;
        }

        let result = if s1[i] == s2[j] {
            1 + lcs_memo_solution_helper(s1, s2, i + 1, j + 1, memo)
        } else {
            let skip_s1 = lcs_memo_solution_helper(s1, s2, i + 1, j, memo);
            let skip_s2 = lcs_memo_solution_helper(s1, s2, i, j + 1, memo);
            skip_s1.max(skip_s2)
        };

        memo.insert((i, j), result);
        result
    }

    // Solution 3: Bottom-Up
    #[allow(dead_code)]
    fn lcs_bottom_up_solution(s1: &str, s2: &str) -> usize {
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();
        let m = s1_bytes.len();
        let n = s2_bytes.len();

        if m == 0 || n == 0 {
            return 0;
        }

        let mut dp = vec![vec![0; n + 1]; m + 1];

        // Fill from bottom-right to top-left
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if s1_bytes[i] == s2_bytes[j] {
                    dp[i][j] = 1 + dp[i + 1][j + 1];
                } else {
                    dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
                }
            }
        }

        dp[0][0]
    }

    // Solution 4: Space-Optimized
    #[allow(dead_code)]
    fn lcs_space_optimized_solution(s1: &str, s2: &str) -> usize {
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();

        let (shorter, longer) = if s1_bytes.len() <= s2_bytes.len() {
            (s1_bytes, s2_bytes)
        } else {
            (s2_bytes, s1_bytes)
        };

        let n = shorter.len();
        let mut prev = vec![0; n + 1];
        let mut curr = vec![0; n + 1];

        for i in (0..longer.len()).rev() {
            for j in (0..n).rev() {
                curr[j] = if longer[i] == shorter[j] {
                    1 + prev[j + 1]
                } else {
                    curr[j + 1].max(prev[j])
                };
            }
            std::mem::swap(&mut prev, &mut curr);
        }

        prev[0]
    }

    // Solution 5: Reconstruct String
    #[allow(dead_code)]
    fn lcs_reconstruct_solution(s1: &str, s2: &str) -> String {
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();
        let m = s1_bytes.len();
        let n = s2_bytes.len();

        if m == 0 || n == 0 {
            return String::new();
        }

        // Build DP table
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if s1_bytes[i] == s2_bytes[j] {
                    dp[i][j] = 1 + dp[i + 1][j + 1];
                } else {
                    dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
                }
            }
        }

        // Backtrack to reconstruct
        let mut result = String::new();
        let mut i = 0;
        let mut j = 0;

        while i < m && j < n {
            if s1_bytes[i] == s2_bytes[j] {
                result.push(s1_bytes[i] as char);
                i += 1;
                j += 1;
            } else if dp[i + 1][j] > dp[i][j + 1] {
                i += 1;
            } else {
                j += 1;
            }
        }

        result
    }

    #[test]
    fn test_solutions() {
        let test_cases = vec![
            ("ABCDEF", "ADBEF", 4),
            ("AGGTAB", "GXTXAYB", 4),
            ("ABC", "AC", 2),
            ("ABC", "DEF", 0),
            ("", "ABC", 0),
            ("AAA", "AAA", 3),
        ];

        for (s1, s2, expected) in test_cases {
            assert_eq!(lcs_bottom_up_solution(s1, s2), expected);
            assert_eq!(lcs_space_optimized_solution(s1, s2), expected);
        }
    }
}
