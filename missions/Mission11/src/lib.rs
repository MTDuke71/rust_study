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
//! # Usage Example
//! ```rust
//! use mission11::MemoCache;
//! use std::collections::HashMap;
//! 
//! // Generic memoization for any key/value types
//! let mut cache = MemoCache::<(usize, usize), u64>::new();
//! 
//! // Memoized computation
//! let result = cache.memoize((5, 3), || {
//!     // Expensive computation here
//!     42
//! });
//! ```
//! 
//! # Performance Characteristics
//! - Cache lookup: O(1) average case (HashMap)
//! - Memory usage: O(U) where U = unique states explored
//! - Recursion depth: Limited by call stack (typically 8KB-2MB)
//! 
//! # Integration Points
//! - [[AoC 2023 Day 12]]: Hot Springs constraint satisfaction
//! - [[AoC 2024 Day 19]]: Linen Layout string pattern matching
//! - [[AoC 2024 Day 11]]: Stone Multiplication numeric state caching

pub mod memo_cache;

// Re-export core types
pub use memo_cache::MemoCache;
