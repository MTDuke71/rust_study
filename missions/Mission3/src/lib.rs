//! # Mission 3: Binary Search with Traits, Slices, and Iterators
//!
//! This module implements binary search algorithms demonstrating Rust's trait system,
//! slice borrowing, and iterator patterns. Designed for Advent of Code-style problems.
//!
//! ## Requirements Fulfilled
//! - REQ-1: Slice-based binary search with proper error handling
//! - REQ-2: Trait-based abstraction for searchable containers
//! - REQ-3: Iterator integration for zero-cost search operations
//! - REQ-4: Custom ordering and key extraction support
//! - REQ-5: Lifetime-safe borrowing patterns
//! - REQ-6: AoC-style utility functions
//!
//! ## Quick Start
//! ```rust
//! use mission3::binary_search;
//!
//! let data = [1, 3, 5, 7, 9, 11];
//!
//! // Basic search
//! assert_eq!(binary_search::search_slice(&data, &5), Ok(2));
//! assert_eq!(binary_search::search_slice(&data, &6), Err(3));
//!
//! // Custom ordering search
//! let people = [(2, "Bob"), (1, "Alice"), (3, "Charlie")];
//! let sorted_by_id: Vec<_> = people.iter().collect();
//! // Search by custom criteria...
//! ```
//!
//! ## Performance Characteristics
//! - Time: O(log n) for all search operations
//! - Space: O(1) for basic search, O(k) for range operations
//! - Zero-cost abstractions: trait usage has no runtime overhead

pub mod aoc_utils;
pub mod binary_search;
pub mod search_iter;
pub mod searchable;

// Re-export main functionality
pub use aoc_utils::*;
pub use binary_search::*;
pub use search_iter::*;
pub use searchable::Searchable;
