//! Unit tests with requirements traceability for Mission 11
//! 
//! Test naming convention: req{N}_description
//! - Tests must map to specific requirement IDs in README.md
//! - Each requirement should have multiple test cases

use mission11::MemoCache;
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
