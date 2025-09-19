//! # Brackets Basic - Simple Bracket Validation Library
//! 
//! A straightforward, efficient bracket validation library that checks for proper
//! nesting and matching of parentheses `()`, square brackets `[]`, and curly braces `{}`.
//! Perfect for syntax validation, expression parsing, and educational purposes.
//!
//! ## Features
//!
//! - **⚡ Fast Validation**: O(n) time complexity with minimal memory allocation
//! - **🎯 Simple API**: Single function call for most use cases
//! - **📍 Detailed Errors**: Precise error locations and types for debugging
//! - **🌐 Unicode Support**: Full UTF-8 character support with proper byte indexing
//! - **🔒 Memory Safe**: Built with Rust's ownership system for guaranteed safety
//! - **📚 Well Tested**: Comprehensive test suite with edge case coverage
//!
//! ## Quick Start
//!
//! ```rust
//! use brackets_basic::validate_brackets;
//!
//! // Valid bracket sequences
//! assert!(validate_brackets("()").is_ok());
//! assert!(validate_brackets("([{}])").is_ok());
//! assert!(validate_brackets("Hello (world [test])!").is_ok());
//!
//! // Invalid sequences return detailed errors
//! let result = validate_brackets("([)]");
//! assert!(result.is_err());
//! ```
//!
//! ## Error Handling
//!
//! The library provides detailed error information for debugging:
//!
//! ```rust
//! use brackets_basic::{validate_brackets, BracketErrorKind};
//!
//! match validate_brackets("([)]") {
//!     Ok(()) => println!("Valid brackets"),
//!     Err(error) => {
//!         println!("Error at position {}: {:?}", error.index, error.kind);
//!         match error.kind {
//!             BracketErrorKind::MismatchedPair { expected, found } => {
//!                 println!("Expected '{}', but found '{}'", expected, found);
//!             },
//!             BracketErrorKind::UnexpectedClosing { found } => {
//!                 println!("Unexpected closing bracket: '{}'", found);
//!             },
//!             BracketErrorKind::UnclosedOpenings { expected, open_index } => {
//!                 println!("Unclosed bracket at {}, expected '{}'", open_index, expected);
//!             },
//!         }
//!     }
//! }
//! ```
//!
//! ## Use Cases
//!
//! - **Code Editors**: Syntax highlighting and error detection
//! - **Compilers/Interpreters**: Expression and block validation
//! - **Educational Tools**: Teaching proper bracket usage
//! - **Data Validation**: Ensuring well-formed input
//! - **Configuration Parsers**: Validating structured data formats
//!
//! ## Performance
//!
//! This library is optimized for performance:
//! - Single pass through input (O(n) time)
//! - Minimal memory allocation (stack-based tracking)
//! - Zero-copy string processing
//! - Efficient UTF-8 character iteration
//!
//! ## Supported Bracket Types
//!
//! | Opener | Closer | Name |
//! |--------|--------|------|
//! | `(`    | `)`    | Parentheses |
//! | `[`    | `]`    | Square Brackets |
//! | `{`    | `}`    | Curly Braces |
//!
//! All other characters are ignored during validation.

pub mod brackets;
// mod experiments;
// mod integration_tests;

// Use Mission1's stack instead of our own duplicate
pub use mission1::Stack;
pub use brackets::*;