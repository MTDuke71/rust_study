//! # State Management Pattern Recognition Module
//!
//! Recognizes and implements the most common AoC state management patterns:
//! - Memoization and caching strategies
//! - Complex state tracking with HashMap/HashSet
//! - Optimization patterns (dynamic programming)
//! - Cycle detection and state space reduction
//!
//! ## Requirements Satisfied: REQ-P3
//!
//! ### Common AoC State Management Problem Types:
//! 1. **Memoization**: Recursive problems with overlapping subproblems (AoC 2021 Day 6, 2020 Day 10)
//! 2. **State Tracking**: Complex game states, simulations (AoC 2020 Day 17, 2018 Day 18)
//! 3. **Cycle Detection**: Finding patterns in state evolution (AoC 2022 Day 17, 2020 Day 14)
//! 4. **Set Operations**: Union, intersection, difference on state sets (AoC 2020 Day 6, 2021 Day 8)

use crate::{AocPattern, PatternResult, PatternError, PatternComplexity};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

/// Generic memoization cache for recursive computations
/// 
/// # Requirements Satisfied: REQ-P3, REQ-P4
/// Provides O(1) lookup for previously computed results
#[derive(Debug)]
pub struct MemoizationCache<K, V> 
where 
    K: Eq + Hash + Clone,
    V: Clone,
{
    cache: HashMap<K, V>,
    hits: usize,
    misses: usize,
}

impl<K, V> MemoizationCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Create new memoization cache
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    
    /// Get value from cache or compute if not present
    /// 
    /// # Requirements Satisfied: REQ-P3
    /// Classic memoization pattern for AoC recursive problems
    /// 
    /// # Example
    /// ```
    /// use aoc_pattern_recognition::state_patterns::MemoizationCache;
    /// 
    /// let mut cache = MemoizationCache::new();
    /// let result = cache.get_or_compute(5, |n| n * n);
    /// assert_eq!(result, 25);
    /// ```
    pub fn get_or_compute<F>(&mut self, key: K, compute_fn: F) -> V
    where
        F: FnOnce(&K) -> V,
    {
        if let Some(value) = self.cache.get(&key) {
            self.hits += 1;
            value.clone()
        } else {
            self.misses += 1;
            let value = compute_fn(&key);
            self.cache.insert(key, value.clone());
            value
        }
    }
    
    /// Check if key exists in cache
    pub fn contains_key(&self, key: &K) -> bool {
        self.cache.contains_key(key)
    }
    
    /// Get cache statistics
    pub fn stats(&self) -> (usize, usize, f64) {
        let total = self.hits + self.misses;
        let hit_rate = if total > 0 { self.hits as f64 / total as f64 } else { 0.0 };
        (self.hits, self.misses, hit_rate)
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }
    
    /// Get cache size
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    
    /// Direct access to cached value (for manual memoization)
    pub fn get(&self, key: &K) -> Option<&V> {
        self.cache.get(key)
    }
    
    /// Direct insertion into cache (for manual memoization)
    pub fn insert(&mut self, key: K, value: V) {
        self.cache.insert(key, value);
    }
}

impl<K, V> Default for MemoizationCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

/// Fibonacci sequence with memoization (classic AoC-style recursive problem)
/// 
/// # Requirements Satisfied: REQ-P3
/// Demonstrates memoization pattern that appears in many AoC problems
pub fn cached_fibonacci(cache: &mut MemoizationCache<u64, u64>, n: u64) -> u64 {
    if let Some(&cached) = cache.get(&n) {
        return cached;
    }
    
    let result = match n {
        0 => 0,
        1 => 1,
        _ => {
            let a = cached_fibonacci(cache, n - 1);
            let b = cached_fibonacci(cache, n - 2);
            a + b
        }
    };
    
    cache.insert(n, result);
    result
}

/// State tracking pattern for complex simulations
/// 
/// # Requirements Satisfied: REQ-P3
/// Manages complex state evolution with history tracking
#[derive(Debug, Clone)]
pub struct StateTracker<T> 
where 
    T: Clone + Eq + Hash,
{
    current_state: T,
    state_history: Vec<T>,
    state_set: HashSet<T>,
    generation: usize,
}

impl<T> StateTracker<T>
where
    T: Clone + Eq + Hash,
{
    /// Create new state tracker with initial state
    pub fn new(initial_state: T) -> Self {
        let mut state_set = HashSet::new();
        state_set.insert(initial_state.clone());
        
        Self {
            current_state: initial_state.clone(),
            state_history: vec![initial_state],
            state_set,
            generation: 0,
        }
    }
    
    /// Advance to next state
    /// 
    /// # Requirements Satisfied: REQ-P3
    /// Tracks state evolution and detects cycles
    pub fn advance<F>(&mut self, transition_fn: F) -> bool
    where
        F: FnOnce(&T) -> T,
    {
        let next_state = transition_fn(&self.current_state);
        
        // Check if we've seen this state before (cycle detection)
        let is_new_state = self.state_set.insert(next_state.clone());
        
        self.current_state = next_state.clone();
        self.state_history.push(next_state);
        self.generation += 1;
        
        is_new_state
    }
    
    /// Get current state
    pub fn current(&self) -> &T {
        &self.current_state
    }
    
    /// Get generation number
    pub fn generation(&self) -> usize {
        self.generation
    }
    
    /// Check if state has been seen before
    pub fn has_seen_state(&self, state: &T) -> bool {
        self.state_set.contains(state)
    }
    
    /// Find cycle in state history
    /// 
    /// # Requirements Satisfied: REQ-P3, REQ-P4
    /// Critical for AoC problems with repeating patterns
    pub fn find_cycle(&self) -> Option<(usize, usize)> {
        let mut seen_positions = HashMap::new();
        
        for (pos, state) in self.state_history.iter().enumerate() {
            if let Some(&first_pos) = seen_positions.get(state) {
                return Some((first_pos, pos - first_pos));
            }
            seen_positions.insert(state, pos);
        }
        
        None
    }
    
    /// Get state at specific generation (with cycle extrapolation)
    /// 
    /// # Requirements Satisfied: REQ-P3, REQ-P4
    /// Allows fast computation of states far in the future
    pub fn state_at_generation(&self, target_gen: usize) -> Option<&T> {
        if target_gen < self.state_history.len() {
            return self.state_history.get(target_gen);
        }
        
        // Check if we can use cycle detection
        if let Some((cycle_start, cycle_length)) = self.find_cycle() {
            if target_gen >= cycle_start {
                let position_in_cycle = (target_gen - cycle_start) % cycle_length;
                return self.state_history.get(cycle_start + position_in_cycle);
            }
        }
        
        None
    }
}

/// Set operations pattern for state analysis
/// 
/// # Requirements Satisfied: REQ-P3
/// Common in AoC problems involving group operations
pub struct SetOperations;

impl SetOperations {
    /// Find intersection of multiple sets
    /// 
    /// # Requirements Satisfied: REQ-P3
    /// Common in AoC problems finding common elements
    pub fn intersection<T>(sets: &[HashSet<T>]) -> HashSet<T>
    where
        T: Clone + Eq + Hash,
    {
        if sets.is_empty() {
            return HashSet::new();
        }
        
        let mut result = sets[0].clone();
        for set in &sets[1..] {
            result = result.intersection(set).cloned().collect();
        }
        
        result
    }
    
    /// Find union of multiple sets
    pub fn union<T>(sets: &[HashSet<T>]) -> HashSet<T>
    where
        T: Clone + Eq + Hash,
    {
        let mut result = HashSet::new();
        for set in sets {
            result.extend(set.iter().cloned());
        }
        result
    }
    
    /// Find symmetric difference (elements in exactly one set)
    pub fn symmetric_difference<T>(a: &HashSet<T>, b: &HashSet<T>) -> HashSet<T>
    where
        T: Clone + Eq + Hash,
    {
        a.symmetric_difference(b).cloned().collect()
    }
    
    /// Count unique elements across all sets
    pub fn count_unique<T>(sets: &[HashSet<T>]) -> usize
    where
        T: Clone + Eq + Hash,
    {
        Self::union(sets).len()
    }
}

/// Dynamic programming pattern for optimization problems
/// 
/// # Requirements Satisfied: REQ-P3, REQ-P4
/// Classic DP approach common in AoC optimization problems
pub struct DynamicProgramming;

impl DynamicProgramming {
    /// Solve knapsack problem (common AoC optimization pattern)
    /// 
    /// # Requirements Satisfied: REQ-P3
    /// Template for resource allocation problems
    pub fn knapsack(weights: &[usize], values: &[usize], capacity: usize) -> usize {
        let n = weights.len();
        let mut dp = vec![vec![0; capacity + 1]; n + 1];
        
        for i in 1..=n {
            for w in 1..=capacity {
                if weights[i - 1] <= w {
                    dp[i][w] = dp[i - 1][w].max(
                        dp[i - 1][w - weights[i - 1]] + values[i - 1]
                    );
                } else {
                    dp[i][w] = dp[i - 1][w];
                }
            }
        }
        
        dp[n][capacity]
    }
    
    /// Longest common subsequence (string/sequence problems)
    /// 
    /// # Requirements Satisfied: REQ-P3
    /// Common in AoC string manipulation problems
    pub fn longest_common_subsequence(a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let m = a_chars.len();
        let n = b_chars.len();
        
        let mut dp = vec![vec![0; n + 1]; m + 1];
        
        for i in 1..=m {
            for j in 1..=n {
                if a_chars[i - 1] == b_chars[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
            }
        }
        
        dp[m][n]
    }
    
    /// Minimum edit distance (Levenshtein distance)
    /// 
    /// # Requirements Satisfied: REQ-P3
    /// Useful for string similarity problems
    pub fn edit_distance(a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let m = a_chars.len();
        let n = b_chars.len();
        
        let mut dp = vec![vec![0; n + 1]; m + 1];
        
        // Initialize base cases
        for i in 0..=m {
            dp[i][0] = i;
        }
        for j in 0..=n {
            dp[0][j] = j;
        }
        
        for i in 1..=m {
            for j in 1..=n {
                if a_chars[i - 1] == b_chars[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
                }
            }
        }
        
        dp[m][n]
    }
}

/// Sliding window pattern for sequence analysis
/// 
/// # Requirements Satisfied: REQ-P3, REQ-P4
/// Efficient pattern for range-based problems
pub struct SlidingWindow;

impl SlidingWindow {
    /// Find maximum sum in sliding window
    /// 
    /// # Requirements Satisfied: REQ-P4
    /// O(n) solution for range optimization problems
    pub fn max_sum_window(arr: &[i32], window_size: usize) -> Option<i32> {
        if arr.len() < window_size {
            return None;
        }
        
        // Calculate sum of first window
        let mut window_sum: i32 = arr[..window_size].iter().sum();
        let mut max_sum = window_sum;
        
        // Slide the window
        for i in window_size..arr.len() {
            window_sum = window_sum - arr[i - window_size] + arr[i];
            max_sum = max_sum.max(window_sum);
        }
        
        Some(max_sum)
    }
    
    /// Find all anagrams in string using sliding window
    /// 
    /// # Requirements Satisfied: REQ-P3, REQ-P4
    /// Character frequency tracking pattern
    pub fn find_anagrams(text: &str, pattern: &str) -> Vec<usize> {
        if text.len() < pattern.len() {
            return Vec::new();
        }
        
        let text_chars: Vec<char> = text.chars().collect();
        let pattern_chars: Vec<char> = pattern.chars().collect();
        let pattern_len = pattern_chars.len();
        
        // Count pattern characters
        let mut pattern_count = HashMap::new();
        for ch in pattern_chars {
            *pattern_count.entry(ch).or_insert(0) += 1;
        }
        
        let mut window_count = HashMap::new();
        let mut result = Vec::new();
        
        // Initialize first window
        for i in 0..pattern_len {
            *window_count.entry(text_chars[i]).or_insert(0) += 1;
        }
        
        if window_count == pattern_count {
            result.push(0);
        }
        
        // Slide the window
        for i in pattern_len..text_chars.len() {
            // Add new character
            *window_count.entry(text_chars[i]).or_insert(0) += 1;
            
            // Remove old character
            let old_char = text_chars[i - pattern_len];
            if let Some(count) = window_count.get_mut(&old_char) {
                *count -= 1;
                if *count == 0 {
                    window_count.remove(&old_char);
                }
            }
            
            if window_count == pattern_count {
                result.push(i - pattern_len + 1);
            }
        }
        
        result
    }
}

/// Pattern for detecting cycles in sequences
/// 
/// # Requirements Satisfied: REQ-P3, REQ-P4
/// Floyd's cycle detection (tortoise and hare)
pub struct CycleDetection;

impl CycleDetection {
    /// Detect cycle in sequence using function iteration
    /// 
    /// # Requirements Satisfied: REQ-P3, REQ-P4
    /// O(1) space cycle detection for state machines
    pub fn floyd_cycle_detection<T, F>(start: T, next_fn: F) -> Option<(usize, usize)>
    where
        T: Clone + Eq,
        F: Fn(&T) -> T,
    {
        let mut tortoise = start.clone();
        let mut hare = start.clone();
        
        // Phase 1: Detect if cycle exists
        loop {
            tortoise = next_fn(&tortoise);
            hare = next_fn(&next_fn(&hare));
            
            if tortoise == hare {
                break;
            }
        }
        
        // Phase 2: Find cycle start
        let mut mu = 0; // Start of cycle
        tortoise = start;
        while tortoise != hare {
            tortoise = next_fn(&tortoise);
            hare = next_fn(&hare);
            mu += 1;
        }
        
        // Phase 3: Find cycle length
        let mut lambda = 1; // Length of cycle
        hare = next_fn(&tortoise);
        while tortoise != hare {
            hare = next_fn(&hare);
            lambda += 1;
        }
        
        Some((mu, lambda))
    }
}

/// State management pattern recognizer
/// 
/// # Requirements Satisfied: REQ-P5
/// Helps identify when to use different state management patterns
pub struct StatePatternRecognizer;

impl StatePatternRecognizer {
    /// Analyze problem and suggest state management patterns
    /// 
    /// # Requirements Satisfied: REQ-P5
    /// Pattern recognition for learning
    pub fn analyze_problem(&self, description: &str) -> Vec<String> {
        let mut patterns = Vec::new();
        let lower_desc = description.to_lowercase();
        
        if lower_desc.contains("fibonacci") || lower_desc.contains("recursive") {
            patterns.push("Memoization recommended for recursive subproblems".to_string());
        }
        
        if lower_desc.contains("cycle") || lower_desc.contains("repeat") {
            patterns.push("Cycle detection for repeating patterns".to_string());
        }
        
        if lower_desc.contains("optimization") || lower_desc.contains("maximum") || lower_desc.contains("minimum") {
            patterns.push("Dynamic programming for optimization".to_string());
        }
        
        if lower_desc.contains("window") || lower_desc.contains("range") {
            patterns.push("Sliding window for range-based problems".to_string());
        }
        
        if lower_desc.contains("intersection") || lower_desc.contains("union") || lower_desc.contains("common") {
            patterns.push("Set operations for group analysis".to_string());
        }
        
        if lower_desc.contains("simulation") || lower_desc.contains("generation") {
            patterns.push("State tracking for complex simulations".to_string());
        }
        
        if patterns.is_empty() {
            patterns.push("Basic state management with HashMap/HashSet".to_string());
        }
        
        patterns
    }
}