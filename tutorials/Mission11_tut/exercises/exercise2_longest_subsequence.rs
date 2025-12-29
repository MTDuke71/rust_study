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
//! **Example**:
//! - s1 = "ABCDEF", s2 = "ADBEF" → LCS = "ABEF" (length 4)
//! - s1 = "AGGTAB", s2 = "GXTXAYB" → LCS = "GTAB" (length 4)
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

#[allow(dead_code, unused_variables)]
fn lcs_naive_helper(s1: &[u8], s2: &[u8], i: usize, j: usize) -> usize {
    // TODO: Implement naive recursion
    // 1. Base case: if i >= s1.len() or j >= s2.len(), return 0
    // 2. If s1[i] == s2[j], return 1 + lcs(i+1, j+1)
    // 3. Otherwise, return max(lcs(i+1, j), lcs(i, j+1))
    todo!("Implement lcs_naive_helper");
}

/// Approach 2: Top-Down with HashMap (FAST - O(m×n))
///
/// Use HashMap<(usize, usize), usize> to cache results
fn lcs_memoized(s1: &str, s2: &str) -> usize {
    let mut memo = HashMap::new();
    lcs_memo_helper(s1.as_bytes(), s2.as_bytes(), 0, 0, &mut memo)
}

#[allow(dead_code, unused_variables)]
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
    todo!("Implement lcs_memo_helper");
}

/// Approach 3: Bottom-Up with 2D Table (FAST - O(m×n), no recursion)
///
/// Build table from bottom-right to top-left (or reverse)
#[allow(dead_code, unused_variables)]
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
    todo!("Implement lcs_bottom_up");
}

// ============================================================================
// BONUS: Space-Optimized Version
// ============================================================================

/// Approach 4: Space-Optimized O(min(m, n))
///
/// Only need current and previous row/column
#[allow(dead_code, unused_variables)]
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
    todo!("Implement lcs_space_optimized");
}

// ============================================================================
// BONUS: Reconstruct the actual LCS string (not just length)
// ============================================================================

/// Reconstruct the actual LCS string
#[allow(dead_code, unused_variables)]
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
    todo!("Implement lcs_reconstruct");
}

// ============================================================================
// Tests and Demonstrations
// ============================================================================

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
        ("ABCDEF", "ADBEF"),          // Small
        ("PROGRAMMING", "GAMING"),     // Medium
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
