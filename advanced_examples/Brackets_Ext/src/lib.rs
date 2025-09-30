//! # Brackets Extended - Advanced Bracket Validation Library
//! 
//! A comprehensive, configurable bracket validation system supporting custom alphabets,
//! flexible error reporting, and multiple validation strategies. Designed for real-world
//! applications including IDEs, linters, and educational tools.
//!
//! ## Features
//!
//! - **🔤 Configurable Alphabets**: Support any bracket pairs, not just `()`, `[]`, `{}`
//! - **📝 Error Collection Modes**: Stop at first error or collect all errors
//! - **🏗️ Unclosed Policies**: Choose between latest-open (LIFO) or earliest-open (FIFO) reporting
//! - **🔄 Iterator APIs**: Work with strings, char iterators, or indexed iterators
//! - **⚡ Performance**: O(n) time complexity with minimal memory allocation
//! - **🌐 Unicode Support**: Full UTF-8 character support with proper byte indexing
//!
//! ## Quick Start
//!
//! ```rust
//! use brackets_extended::{validate_brackets, Options, Alphabet, ErrorMode};
//!
//! // Basic validation (traditional API)
//! assert!(validate_brackets("([{}])").is_ok());
//! assert!(validate_brackets("([)]").is_err());
//!
//! // Advanced validation with custom options
//! let mut opts = Options::default();
//! opts.alphabet = Alphabet::with_pairs(&[('(', ')'), ('<', '>')]);
//! opts.error_mode = ErrorMode::CollectAll;
//!
//! // Now supports angle brackets and collects all errors
//! let result = brackets_extended::validate_with_options("(<>", &opts);
//! ```
//!
//! ## Extended Requirements
//!
//! This library implements three key extended requirements:
//!
//! ### REQ-7: Configurable Alphabet
//! 
//! Define custom bracket pairs for specialized use cases:
//!
//! ```rust
//! use brackets_extended::{Alphabet, Options, validate_with_options};
//!
//! // HTML/XML-style validation
//! let html_alphabet = Alphabet::with_pairs(&[('<', '>')]);
//! let mut opts = Options::default();
//! opts.alphabet = html_alphabet;
//! 
//! assert!(validate_with_options("<div><span>content</span></div>", &opts).is_ok());
//! ```
//!
//! ### REQ-8: Error Collection Mode
//!
//! Choose between stopping at the first error or collecting all errors:
//!
//! ```rust
//! use brackets_extended::{Options, ErrorMode, validate_with_options};
//!
//! let mut opts = Options::default();
//! opts.error_mode = ErrorMode::CollectAll;
//!
//! let errors = validate_with_options(")](", &opts).unwrap_err();
//! assert!(errors.len() > 1); // Multiple errors collected
//! ```
//!
//! ### REQ-9: Unclosed Policy Options
//!
//! Control which unclosed bracket to report when multiple are unclosed:
//!
//! ```rust
//! use brackets_extended::{Options, UnclosedPolicy, validate_with_options};
//!
//! let input = "((([";
//!
//! // Report the latest opened bracket (LIFO behavior)
//! let mut latest_opts = Options::default();
//! latest_opts.unclosed_policy = UnclosedPolicy::LatestOpen;
//!
//! // Report the earliest opened bracket (FIFO behavior)  
//! let mut earliest_opts = Options::default();
//! earliest_opts.unclosed_policy = UnclosedPolicy::EarliestOpen;
//! ```
//!
//! ## Iterator APIs
//!
//! Multiple APIs for different input types:
//!
//! ```rust
//! use brackets_extended::{validate_iter, validate_indexed, Options};
//!
//! let opts = Options::default();
//!
//! // Character iterator
//! let result = validate_iter("([)]".chars(), &opts);
//!
//! // Indexed iterator (preserves original byte positions)
//! let text = "hello(world]";
//! let result = validate_indexed(text.char_indices(), &opts);
//! ```
//!
//! ## Performance Characteristics
//!
//! - **Time Complexity**: O(n) where n is the number of characters
//! - **Space Complexity**: O(d) where d is the maximum nesting depth
//! - **Memory Efficiency**: Stack-based algorithm with minimal allocations
//! - **Unicode Support**: Handles multi-byte UTF-8 characters correctly
//!
//! ## Real-World Applications
//!
//! - **Code Editors**: Syntax highlighting and error reporting
//! - **Linters**: Comprehensive bracket validation for multiple languages
//! - **Educational Tools**: Show students all bracket errors at once
//! - **Configuration Parsers**: Validate JSON, YAML, and custom formats
//! - **Mathematical Software**: Support specialized notation brackets
//!
//! ## Architecture
//!
//! The library is built around three core concepts:
//!
//! 1. **Alphabet**: Defines which characters are brackets and their pairs
//! 2. **Options**: Configures validation behavior (alphabet, error mode, policies)
//! 3. **Error Types**: Detailed error information with positions and context

pub mod brackets;

// Use Mission1's stack instead of our own duplicate
pub use mission1::Stack;
pub use brackets::*;