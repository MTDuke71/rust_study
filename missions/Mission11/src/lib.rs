//! Mission 11: Dynamic Programming with Memoization
//!
//! This module implements memoization patterns for dynamic programming following
//! V-Cycle methodology with complete requirements traceability and performance validation.
//!
//! # Requirements Overview
//! - REQ-1: Generic recursive memoization framework
//! - REQ-2: Top-down DP pattern template
//! - REQ-3: Zero-copy string slice caching
//! - REQ-4: Boolean → Counting DP transformation
//! - REQ-5: State space design patterns
//! - REQ-6: Overlapping subproblem detection
//! - REQ-7: Bottom-up DP alternative (comparison)
//! - REQ-8: Real AoC problem integration
//!
//! # Quick Start
//!
//! ## Generic Memoization Cache
//!
//! ```rust
//! use mission11::MemoCache;
//!
//! let mut cache = MemoCache::<(usize, usize), u64>::new();
//! let result = cache.memoize((5, 3), || {
//!     // Expensive computation here
//!     42
//! });
//! ```
//!
//! ## String Pattern Matching (Boolean DP)
//!
//! ```rust
//! use mission11::string_dp::can_construct;
//! use std::collections::HashMap;
//!
//! let patterns = vec!["ab", "cd"];
//! let target = "abcd";
//!
//! let mut memo = HashMap::new();
//! assert!(can_construct(target, &patterns, &mut memo));
//! ```
//!
//! ## Counting Arrangements (Counting DP)
//!
//! ```rust
//! use mission11::string_dp::count_constructions;
//! use std::collections::HashMap;
//!
//! let patterns = vec!["a", "aa"];
//! let target = "aa";
//!
//! let mut memo = HashMap::new();
//! let count = count_constructions(target, &patterns, &mut memo);
//! assert_eq!(count, 2); // "a"+"a" and "aa"
//! ```
//!
//! # Performance Characteristics
//! - Cache lookup: O(1) average case (HashMap)
//! - Memory usage: O(U) where U = unique states explored
//! - Recursion depth: Limited by call stack (typically 8KB-2MB)
//!
//! # Top-Down DP Pattern (REQ-2)
//!
//! All recursive DP functions follow this canonical pattern:
//!
//! ```text
//! fn solve<'a>(state: &'a State, memo: &mut HashMap<&'a State, Result>) -> Result {
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
//! # State Space Design Patterns (REQ-5)
//!
//! Five canonical patterns for representing DP state in Rust:
//!
//! ```rust
//! use mission11::state_patterns::*;
//! use std::collections::HashMap;
//!
//! // Pattern 1: String Suffixes (&'a str)
//! let patterns = vec!["ab", "cd"];
//! let mut memo1 = HashMap::new();
//! string_suffix_pattern("abcd", &patterns, &mut memo1);
//!
//! // Pattern 2: Index Positions (usize)
//! let arr = vec![1, 2, 3];
//! let mut memo2 = HashMap::new();
//! index_position_pattern(0, &arr, &mut memo2);
//!
//! // Pattern 3: Coordinate Pairs ((x, y))
//! let grid = vec![vec![1, 2], vec![3, 4]];
//! let mut memo3 = HashMap::new();
//! coordinate_pair_pattern((0, 0), (1, 1), &grid, &mut memo3);
//!
//! // Pattern 4: Composite State (u64, usize)
//! let mut memo4 = HashMap::new();
//! composite_state_pattern((125, 3), &mut memo4);
//!
//! // Pattern 5: Custom Structs
//! let mut memo5 = HashMap::new();
//! let state = GameState { player_hp: 50, boss_hp: 100, turn: 0, mana: 500 };
//! custom_struct_pattern(state, &mut memo5);
//! ```
//!
//! # Integration Points
//! - [[AoC 2023 Day 12]]: Hot Springs constraint satisfaction
//! - [[AoC 2024 Day 19]]: Linen Layout string pattern matching
//! - [[AoC 2024 Day 11]]: Stone Multiplication numeric state caching

pub mod instrumentation;
pub mod memo_cache;
pub mod state_patterns;
pub mod string_dp;

// Re-export core types
pub use instrumentation::{CacheStats, InstrumentedCache, PerformanceComparison};
pub use memo_cache::MemoCache;
