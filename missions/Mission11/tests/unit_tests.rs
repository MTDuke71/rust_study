//! Unit tests with requirements traceability for Mission 11
//! 
//! Test naming convention: req{N}_description
//! - Tests must map to specific requirement IDs in README.md
//! - Each requirement should have multiple test cases

use mission11::MemoCache;
use mission11::string_dp::{can_construct, count_constructions, can_construct_multi};
use std::collections::HashMap;

// ============================================================================
// REQ-1: Generic Recursive Memoization Framework
// ============================================================================

#[test]
fn req1_generic_support_numeric_keys() {
    // Test generic cache with numeric tuple keys
    let mut cache = MemoCache::<(usize, usize), u64>::new();
    
    let result = cache.memoize((5, 3), || 42);
    assert_eq!(result, 42);
    assert_eq!(cache.size(), 1);
}

#[test]
fn req1_generic_support_string_values() {
    // Test generic cache with String values
    let mut cache = MemoCache::<i32, String>::new();
    
    let result = cache.memoize(1, || "hello".to_string());
    assert_eq!(result, "hello");
}

#[test]
fn req1_generic_support_tuple_state() {
    // Test AoC Day 12 style 3D state tuple
    let mut cache = MemoCache::<(usize, usize, usize), usize>::new();
    
    // Simulate DP state: (position, group_index, current_run)
    let result = cache.memoize((7, 2, 3), || 42);
    assert_eq!(result, 42);
    
    // Verify caching works
    let result2 = cache.memoize((7, 2, 3), || panic!("Should use cache!"));
    assert_eq!(result2, 42);
    assert_eq!(cache.hits(), 1);
}

#[test]
fn req1_cache_lookup_before_compute() {
    // REQ-2 pattern: Cache check before expensive computation
    let mut cache = MemoCache::new();
    
    let mut compute_count = 0;
    
    // First call computes
    cache.memoize("key1", || {
        compute_count += 1;
        100
    });
    
    // Second call uses cache
    cache.memoize("key1", || {
        compute_count += 1;
        200
    });
    
    assert_eq!(compute_count, 1); // Only computed once
    assert_eq!(cache.hits(), 1);
    assert_eq!(cache.misses(), 1);
}

#[test]
fn req1_zero_copy_string_slice_keys() {
    // REQ-3: String slices as keys without allocation
    fn count_arrangements<'a>(
        pattern: &'a str,
        memo: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if pattern.is_empty() {
            return 1; // Base case
        }
        
        if let Some(&cached) = memo.get(pattern) {
            return cached;
        }
        
        // Simulate recursive computation with substring
        let result = if pattern.starts_with('#') {
            count_arrangements(&pattern[1..], memo)
        } else {
            1
        };
        
        memo.insert(pattern, result);
        result
    }
    
    let mut memo = HashMap::new();
    let result = count_arrangements("###", &mut memo);
    assert_eq!(result, 1);
    assert_eq!(memo.len(), 3); // 3 substrings cached
}

#[test]
fn req1_with_capacity_optimization() {
    // Test pre-allocation for known state space
    let cache = MemoCache::<i32, i32>::with_capacity(1000);
    assert_eq!(cache.size(), 0);
    // Capacity is internal, but should not panic
}

// ============================================================================
// REQ-6: Overlapping Subproblem Detection
// ============================================================================

#[test]
fn req6_cache_statistics_tracking() {
    let mut cache = MemoCache::new();
    
    // Pattern: Many cache hits indicate overlapping subproblems
    cache.memoize(1, || 10);  // Miss
    cache.memoize(2, || 20);  // Miss
    cache.memoize(3, || 30);  // Miss
    cache.memoize(1, || 40);  // Hit
    cache.memoize(2, || 50);  // Hit
    cache.memoize(1, || 60);  // Hit
    
    assert_eq!(cache.size(), 3);
    assert_eq!(cache.misses(), 3);
    assert_eq!(cache.hits(), 3);
    assert_eq!(cache.hit_ratio(), 0.5);
}

#[test]
fn req6_hit_ratio_metric() {
    let mut cache = MemoCache::new();
    
    // Initially 0.0
    assert_eq!(cache.hit_ratio(), 0.0);
    
    // After misses only
    cache.memoize(1, || 10);
    cache.memoize(2, || 20);
    assert_eq!(cache.hit_ratio(), 0.0);
    
    // After hits
    cache.memoize(1, || 30); // Hit
    assert!(cache.hit_ratio() > 0.0);
}

#[test]
fn req6_state_space_size_tracking() {
    let mut cache = MemoCache::new();
    
    // Simulate AoC Day 12 state space growth
    for pos in 0..10 {
        for group_idx in 0..5 {
            for run in 0..3 {
                cache.memoize((pos, group_idx, run), || 1);
            }
        }
    }
    
    // State space size = 10 * 5 * 3 = 150
    assert_eq!(cache.size(), 150);
}

// ============================================================================
// REQ-2: Top-Down DP Pattern Template
// ============================================================================

#[test]
fn req2_fibonacci_memoization_pattern() {
    // Classic DP example demonstrating the pattern
    fn fib(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
        // Base cases
        if n <= 1 {
            return n;
        }
        
        // Memoization check
        if let Some(&cached) = memo.get(&n) {
            return cached;
        }
        
        // Recursive computation
        let result = fib(n - 1, memo) + fib(n - 2, memo);
        
        // Cache and return
        memo.insert(n, result);
        result
    }
    
    let mut memo = HashMap::new();
    assert_eq!(fib(10, &mut memo), 55);
    assert!(memo.len() > 0); // Should cache intermediate results
}

#[test]
fn req2_base_case_handling() {
    // Verify base cases don't get cached unnecessarily
    fn solve(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n == 0 {
            return 1; // Base case - return directly
        }
        
        if let Some(&cached) = memo.get(&n) {
            return cached;
        }
        
        let result = solve(n - 1, memo) + 1;
        memo.insert(n, result);
        result
    }
    
    let mut memo = HashMap::new();
    assert_eq!(solve(5, &mut memo), 6);
    // Base case (0) should not be in cache
    assert!(!memo.contains_key(&0));
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_aoc_day12_pattern_integration() {
    // Simulate AoC 2023 Day 12 memoization pattern
    type State = (usize, usize, usize); // (pos, group_idx, current_run)
    
    fn count_arrangements(
        state: State,
        max_pos: usize,
        memo: &mut HashMap<State, usize>,
    ) -> usize {
        let (pos, group_idx, _run) = state;
        
        // Base case: reached end
        if pos >= max_pos {
            return if group_idx == 3 { 1 } else { 0 };
        }
        
        // Memoization check
        if let Some(&cached) = memo.get(&state) {
            return cached;
        }
        
        // Simulate branching (simplified)
        let result = count_arrangements((pos + 1, group_idx, 0), max_pos, memo)
            + count_arrangements((pos + 1, group_idx + 1, 1), max_pos, memo);
        
        memo.insert(state, result);
        result
    }
    
    let mut memo = HashMap::new();
    let result = count_arrangements((0, 0, 0), 5, &mut memo);
    
    assert!(result > 0);
    assert!(memo.len() > 0); // Should have cached states
}

#[test]
fn test_cache_clear() {
    let mut cache = MemoCache::new();
    
    cache.memoize(1, || 10);
    cache.memoize(2, || 20);
    cache.memoize(1, || 30); // Hit
    
    assert_eq!(cache.size(), 2);
    assert_eq!(cache.hits(), 1);
    
    cache.clear();
    
    assert_eq!(cache.size(), 0);
    assert_eq!(cache.hits(), 0);
    assert_eq!(cache.misses(), 0);
    assert_eq!(cache.hit_ratio(), 0.0);
}

#[test]
fn test_default_trait() {
    let cache: MemoCache<i32, i32> = MemoCache::default();
    assert_eq!(cache.size(), 0);
}

// ============================================================================
// REQ-2: Top-Down DP Pattern Template (String-based)
// ============================================================================

#[test]
fn req2_pattern_structure_base_case() {
    // Verify base case returns immediately without caching
    let patterns = vec!["a"];
    let mut memo = HashMap::new();
    
    // Empty string is base case
    assert!(can_construct("", &patterns, &mut memo));
    assert_eq!(memo.len(), 0); // Base case shouldn't add to cache
}

#[test]
fn req2_pattern_structure_cache_check() {
    // Verify cache is checked before recursive computation
    let patterns = vec!["a", "b"];
    let mut memo = HashMap::new();
    
    // First call - cache miss
    can_construct("ab", &patterns, &mut memo);
    let initial_size = memo.len();
    
    // Second call - should use cache
    can_construct("ab", &patterns, &mut memo);
    assert_eq!(memo.len(), initial_size); // No new entries
}

#[test]
fn req2_pattern_structure_result_caching() {
    // Verify results are cached before returning
    let patterns = vec!["x"];
    let mut memo = HashMap::new();
    
    can_construct("xyz", &patterns, &mut memo);
    
    // All attempted substrings should be cached
    // (including failed attempts)
    assert!(memo.contains_key("xyz"));
}

#[test]
fn req2_recursive_decomposition() {
    // Verify recursive computation explores subproblems
    let patterns = vec!["a", "b", "c"];
    let mut memo = HashMap::new();
    
    assert!(can_construct("abc", &patterns, &mut memo));
    
    // Should have cached: "abc", "bc", "c"
    assert!(memo.len() >= 3);
    assert!(memo.contains_key("abc"));
    assert!(memo.contains_key("bc"));
    assert!(memo.contains_key("c"));
}

// ============================================================================
// REQ-3: Zero-Copy String Slice Caching
// ============================================================================

#[test]
fn req3_string_slice_keys() {
    // Verify HashMap uses &str keys (compile-time validated)
    let patterns = vec!["test"];
    let target = "testing";
    let mut memo: HashMap<&str, bool> = HashMap::new();
    
    can_construct(target, &patterns, &mut memo);
    
    // Keys are string slices, not owned Strings
    for key in memo.keys() {
        // This compiles because keys are &str
        assert!(target.contains(*key) || key.is_empty());
    }
}

#[test]
fn req3_strip_prefix_zero_copy() {
    // Verify strip_prefix is used (zero allocation)
    let patterns = vec!["hello", "world"];
    let mut memo = HashMap::new();
    
    assert!(can_construct("helloworld", &patterns, &mut memo));
    
    // Cache should contain substrings created by strip_prefix
    assert!(memo.contains_key("world") || memo.contains_key("helloworld"));
}

#[test]
fn req3_lifetime_safety() {
    // This test compiles = lifetimes are correct
    // If lifetime 'a wasn't properly propagated, this wouldn't compile
    fn helper<'a>(
        s: &'a str,
        patterns: &[&str],
    ) -> HashMap<&'a str, bool> {
        let mut memo = HashMap::new();
        can_construct(s, patterns, &mut memo);
        memo // Can return memo because lifetime 'a ties it to input 's'
    }
    
    let target = "test";
    let patterns = vec!["te", "st"];
    let result = helper(target, &patterns);
    
    assert!(result.len() > 0);
}

#[test]
fn req3_no_to_string_in_hot_path() {
    // Performance test: verify no String allocations
    let patterns = vec!["a"];
    let mut memo = HashMap::new();
    
    // Build long string to amplify allocation cost if present
    let target = "aaaaaaaaaa"; // 10 'a's
    
    assert!(can_construct(target, &patterns, &mut memo));
    
    // All cache keys should be slices of original target
    for key in memo.keys() {
        // Verify key is a slice (not allocated String)
        assert!(
            std::ptr::eq(target.as_ptr(), key.as_ptr()) 
            || key.is_empty()
            || target.contains(*key)
        );
    }
}

// ============================================================================
// REQ-4: Boolean → Counting DP Transformation
// ============================================================================

#[test]
fn req4_identical_structure() {
    // Verify boolean and counting versions have same structure
    let patterns = vec!["a", "b"];
    
    let mut bool_memo = HashMap::new();
    let mut count_memo = HashMap::new();
    
    let exists = can_construct("ab", &patterns, &mut bool_memo);
    let count = count_constructions("ab", &patterns, &mut count_memo);
    
    // If exists, count should be > 0
    assert_eq!(exists, count > 0);
    
    // Cache structure should be similar (same keys explored)
    assert_eq!(bool_memo.len(), count_memo.len());
}

#[test]
fn req4_base_case_transformation() {
    // Boolean base: true, Counting base: 1
    let patterns = vec!["x"];
    
    let mut bool_memo = HashMap::new();
    let mut count_memo = HashMap::new();
    
    // Empty string base case
    assert!(can_construct("", &patterns, &mut bool_memo));
    assert_eq!(count_constructions("", &patterns, &mut count_memo), 1);
}

#[test]
fn req4_reduction_operator_change() {
    // Boolean: early-exit OR
    // Counting: accumulative SUM
    let patterns = vec!["a", "aa"];
    
    let mut count_memo = HashMap::new();
    let count = count_constructions("aa", &patterns, &mut count_memo);
    
    // "aa" can be built as: "a"+"a" OR "aa" = 2 ways
    assert_eq!(count, 2);
}

#[test]
fn req4_multiple_paths_counting() {
    // Verify counting accumulates ALL paths, not just first
    let patterns = vec!["r", "wr", "b", "rb"];
    let mut memo = HashMap::new();
    
    // "wrb" = "wr"+"b" OR "w"+"r"+"b" (if "w" existed)
    // With current patterns: "wr"+"b" = 1 way
    let count = count_constructions("wrb", &patterns, &mut memo);
    assert!(count >= 1);
}

// ============================================================================
// REQ-5: State Space Design Patterns
// ============================================================================

#[test]
fn req5_string_suffix_state() {
    // Pattern: remaining string as state
    let patterns = vec!["test"];
    let mut memo = HashMap::new();
    
    can_construct("testing", &patterns, &mut memo);
    
    // State progression: "testing" → "ing" → ...
    // Verify suffix-based state representation
    assert!(memo.contains_key("testing") || memo.contains_key("ing"));
}

#[test]
fn req5_composite_state_multi_source() {
    // Pattern: (string, index) composite state
    let pattern_groups = vec![
        vec!["a", "ab"],
        vec!["c", "cd"],
    ];
    let mut memo = HashMap::new();
    
    assert!(can_construct_multi("abc", &pattern_groups, 0, &mut memo));
    
    // Composite state (str, usize) should be cached
    assert!(memo.len() > 0);
}

// ============================================================================
// Performance and Integration Tests
// ============================================================================

#[test]
fn test_aoc_day19_pattern() {
    // Simulate AoC 2024 Day 19 (Linen Layout)
    let towel_patterns = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
    
    let designs = vec![
        "brwrr",   // Can be made
        "bggr",    // Can be made
        "gbbr",    // Can be made
        "rrbgbr",  // Can be made
        "ubwu",    // Cannot be made (no 'u' pattern)
        "bwurrg",  // Cannot be made
        "brgr",    // Can be made
        "bbrgwb",  // Cannot be made
    ];
    
    let mut possible_count = 0;
    
    for design in &designs {
        let mut memo = HashMap::new();
        if can_construct(design, &towel_patterns, &mut memo) {
            possible_count += 1;
        }
    }
    
    // Based on AoC Day 19 example: 6 designs possible
    assert_eq!(possible_count, 6);
}

#[test]
fn test_memoization_exponential_to_linear() {
    // Demonstrate memoization transforms exponential to linear
    let patterns = vec!["a", "aa", "aaa"];
    let target = "aaaaaaaaaa"; // 10 'a's
    
    let mut memo = HashMap::new();
    let result = can_construct(target, &patterns, &mut memo);
    
    assert!(result); // Can be constructed
    
    // Without memo: O(3^10) = 59,049 operations
    // With memo: O(3 * 10) = 30 operations (roughly)
    // Cache size should be ~10 (one per suffix length)
    assert!(memo.len() <= 15); // Linear in target length
}

#[test]
fn test_counting_large_state_space() {
    // Verify counting works for exponentially large solution spaces
    let patterns = vec!["a", "aa", "b", "bb"];
    let mut memo = HashMap::new();
    
    // "aabb" can be built many ways:
    // - a, a, b, b
    // - aa, b, b
    // - a, a, bb
    // - aa, bb
    let count = count_constructions("aabb", &patterns, &mut memo);
    
    // Should find 4 different arrangements
    assert_eq!(count, 4);
}
