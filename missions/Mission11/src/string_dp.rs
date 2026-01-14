//! String-based dynamic programming patterns with zero-copy caching
//! 
//! # Requirements Satisfied
//! - REQ-2: Top-down DP pattern template
//! - REQ-3: Zero-copy string slice caching
//! - REQ-4: Boolean → Counting DP transformation
//! 
//! This module demonstrates the canonical recursive DP pattern optimized for
//! string processing problems using lifetime-parametric caching with `&'a str` keys.
//! 
//! # Pattern Structure
//! 
//! The top-down DP pattern follows this template:
//! 
//! ```text
//! fn solve_recursive<'a>(
//!     state: &'a State,
//!     memo: &mut HashMap<&'a State, Result>,
//! ) -> Result {
//!     // 1. Base case check
//!     if is_base_case(state) { return base_value(); }
//!     
//!     // 2. Memoization check
//!     if let Some(&cached) = memo.get(state) { return cached; }
//!     
//!     // 3. Recursive computation
//!     let result = compute_from_subproblems(state, memo);
//!     
//!     // 4. Cache and return
//!     memo.insert(state, result);
//!     result
//! }
//! ```
//! 
//! # Zero-Copy String Operations
//! 
//! Key performance optimization: Use `&'a str` slices as cache keys instead of `String`.
//! This avoids heap allocations in the critical path:
//! 
//! - ✅ `&pattern[1..]` - Zero-copy substring slice
//! - ✅ `pattern.strip_prefix(s)` - Zero-copy prefix removal
//! - ✅ `HashMap<&'a str, V>` - Borrowed keys, no allocation
//! - ❌ `pattern.to_string()` - Heap allocation (avoid!)
//! 
//! # Examples
//! 
//! ## Existence Check (Boolean DP)
//! 
//! ```rust
//! use std::collections::HashMap;
//! use mission11::string_dp::can_construct;
//! 
//! let patterns = vec!["ab", "cd"];
//! let target = "abcd";
//! 
//! let mut memo = HashMap::new();
//! assert!(can_construct(target, &patterns, &mut memo));
//! ```
//! 
//! ## Counting Arrangements (Numeric DP)
//! 
//! ```rust
//! use std::collections::HashMap;
//! use mission11::string_dp::count_constructions;
//! 
//! let patterns = vec!["a", "aa"];
//! let target = "aa";
//! 
//! let mut memo = HashMap::new();
//! let count = count_constructions(target, &patterns, &mut memo);
//! assert_eq!(count, 2); // "a"+"a" and "aa"
//! ```

use std::collections::HashMap;

/// Checks if target string can be constructed from pattern pieces (existence check)
/// 
/// # Requirements Satisfied
/// - REQ-2: Top-down DP pattern with base case, cache check, recursion, insert
/// - REQ-3: Zero-copy string slice caching with &'a str keys
/// 
/// # Pattern: Boolean DP (Early-Exit OR Logic)
/// 
/// Returns `true` if **any** combination of patterns can build the target.
/// Uses early-exit optimization: stops on first successful path found.
/// 
/// # Arguments
/// - `target`: Remaining string to construct (borrowed slice)
/// - `patterns`: Available pattern pieces
/// - `memo`: Memoization cache with borrowed string keys
/// 
/// # Returns
/// - `true` if target can be constructed from patterns
/// - `false` if no valid construction exists
/// 
/// # Performance
/// - **Without memoization**: O(P^L) exponential (P = patterns, L = target length)
/// - **With memoization**: O(P × L) polynomial
/// - **Memory**: O(L) unique substrings cached (not L²)
/// 
/// # Examples
/// 
/// ```rust
/// use std::collections::HashMap;
/// use mission11::string_dp::can_construct;
/// 
/// let patterns = vec!["r", "wr", "b"];
/// let mut memo = HashMap::new();
/// 
/// assert!(can_construct("wrb", &patterns, &mut memo));
/// assert!(!can_construct("xyz", &patterns, &mut memo));
/// ```
pub fn can_construct<'a>(
    target: &'a str,
    patterns: &[&str],
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    // REQ-2: Step 1 - Base case check
    if target.is_empty() {
        return true; // Empty string can always be constructed
    }

    // REQ-2: Step 2 - Memoization check
    if let Some(&cached) = memo.get(target) {
        return cached;
    }

    // REQ-2: Step 3 - Recursive computation
    // Try each pattern as a prefix match
    for pattern in patterns {
        // REQ-3: Zero-copy string operation (strip_prefix returns Option<&str>)
        if let Some(remainder) = target.strip_prefix(pattern) {
            // Recursive call with remainder slice (no allocation!)
            if can_construct(remainder, patterns, memo) {
                // Early exit on first success (OR logic)
                memo.insert(target, true);
                return true;
            }
        }
    }

    // REQ-2: Step 4 - Cache and return
    memo.insert(target, false);
    false
}

/// Counts all possible ways to construct target from patterns (counting DP)
/// 
/// # Requirements Satisfied
/// - REQ-2: Top-down DP pattern (identical structure to can_construct)
/// - REQ-3: Zero-copy string slice caching
/// - REQ-4: Boolean → Counting transformation (only reduction operator changes)
/// 
/// # Pattern: Counting DP (Accumulative SUM Logic)
/// 
/// Returns count of **all** valid combinations that build the target.
/// Uses accumulation: sums paths from all recursive branches.
/// 
/// # Transformation from Boolean DP
/// 
/// | Aspect | Boolean (can_construct) | Counting (count_constructions) |
/// |--------|-------------------------|--------------------------------|
/// | Base case | `true` | `1` (one way to succeed) |
/// | Cache type | `bool` | `u64` |
/// | Reduction | Early-exit OR | Accumulative SUM |
/// | Return | First success | All paths summed |
/// 
/// # Arguments
/// - `target`: Remaining string to construct
/// - `patterns`: Available pattern pieces
/// - `memo`: Memoization cache (now with u64 values)
/// 
/// # Returns
/// Number of distinct ways to construct target from patterns
/// 
/// # Performance
/// Same complexity as `can_construct` but computes all paths:
/// - **Time**: O(P × L) with memoization
/// - **Space**: O(L) unique substrings
/// 
/// # Examples
/// 
/// ```rust
/// use std::collections::HashMap;
/// use mission11::string_dp::count_constructions;
/// 
/// let patterns = vec!["r", "wr", "b", "rb"];
/// let mut memo = HashMap::new();
/// 
/// // "rb" can be built as: "r"+"b" OR "rb" = 2 ways
/// assert_eq!(count_constructions("rb", &patterns, &mut memo), 2);
/// ```
pub fn count_constructions<'a>(
    target: &'a str,
    patterns: &[&str],
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    // REQ-2: Step 1 - Base case check
    if target.is_empty() {
        return 1; // One way to construct empty string (do nothing)
    }

    // REQ-2: Step 2 - Memoization check
    if let Some(&cached) = memo.get(target) {
        return cached;
    }

    // REQ-2: Step 3 - Recursive computation
    let mut total = 0; // Accumulator for counting paths

    for pattern in patterns {
        // REQ-3: Zero-copy string operation
        if let Some(remainder) = target.strip_prefix(pattern) {
            // REQ-4: SUM all paths instead of early-exit OR
            total += count_constructions(remainder, patterns, memo);
        }
    }

    // REQ-2: Step 4 - Cache and return
    memo.insert(target, total);
    total
}

/// Advanced pattern: Multi-source string matching with alternation
/// 
/// # Requirements Satisfied
/// - REQ-2: Top-down DP pattern
/// - REQ-3: Zero-copy caching
/// - REQ-5: State space design (string suffix state)
/// 
/// Demonstrates matching multiple alternative pattern sets (like regex `(a|b)(c|d)`).
/// 
/// # Examples
/// 
/// ```rust
/// use std::collections::HashMap;
/// use mission11::string_dp::can_construct_multi;
/// 
/// let pattern_groups = vec![
///     vec!["a", "ab"],
///     vec!["b", "c"],
/// ];
/// let mut memo = HashMap::new();
/// 
/// // Can build "abc" as: "ab" from group1 + "c" from group2
/// assert!(can_construct_multi("abc", &pattern_groups, 0, &mut memo));
/// ```
pub fn can_construct_multi<'a>(
    target: &'a str,
    pattern_groups: &[Vec<&str>],
    group_idx: usize,
    memo: &mut HashMap<(&'a str, usize), bool>,
) -> bool {
    // REQ-2: Base cases
    if target.is_empty() && group_idx == pattern_groups.len() {
        return true; // All groups matched, target exhausted
    }

    if group_idx >= pattern_groups.len() {
        return false; // Out of groups but target remains
    }

    // REQ-2: Cache check with composite state
    let state = (target, group_idx);
    if let Some(&cached) = memo.get(&state) {
        return cached;
    }

    // REQ-2: Recursive computation
    let current_patterns = &pattern_groups[group_idx];
    
    for pattern in current_patterns {
        // REQ-3: Zero-copy operation
        if let Some(remainder) = target.strip_prefix(pattern) {
            if can_construct_multi(remainder, pattern_groups, group_idx + 1, memo) {
                memo.insert(state, true);
                return true;
            }
        }
    }

    memo.insert(state, false);
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_construct_basic() {
        let patterns = vec!["r", "wr", "b"];
        let mut memo = HashMap::new();

        assert!(can_construct("wrb", &patterns, &mut memo));
        assert_eq!(memo.len(), 2); // "wrb" and "b" cached
    }

    #[test]
    fn test_can_construct_impossible() {
        let patterns = vec!["a", "b"];
        let mut memo = HashMap::new();

        assert!(!can_construct("c", &patterns, &mut memo));
    }

    #[test]
    fn test_count_constructions_multiple_ways() {
        let patterns = vec!["r", "wr", "b", "rb"];
        let mut memo = HashMap::new();

        // "rb" = "r"+"b" OR "rb" = 2 ways
        let count = count_constructions("rb", &patterns, &mut memo);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_count_constructions_single_way() {
        let patterns = vec!["abc"];
        let mut memo = HashMap::new();

        assert_eq!(count_constructions("abc", &patterns, &mut memo), 1);
    }

    #[test]
    fn test_zero_copy_no_allocations() {
        // This test validates REQ-3: zero-copy string slicing
        let patterns = vec!["a", "b", "c"];
        let target = "abc";
        let mut memo = HashMap::new();

        can_construct(target, &patterns, &mut memo);

        // All cache keys should be slices of original target
        // (Compile-time validated by lifetime 'a parameter)
        for key in memo.keys() {
            // key is &str slice - no heap allocation occurred
            assert!(target.contains(*key) || key.is_empty());
        }
    }

    #[test]
    fn test_memoization_avoids_recomputation() {
        let patterns = vec!["a", "aa", "aaa"];
        let mut memo = HashMap::new();

        // Force overlapping subproblems
        can_construct("aaaa", &patterns, &mut memo);

        // Should have cached subproblems
        assert!(memo.len() > 1);
    }
}
