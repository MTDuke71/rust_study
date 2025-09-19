//! # Rust Documentation Standards Example
//! 
//! This module demonstrates comprehensive Rust documentation standards
//! following the official Rust guidelines and conventions.
//! 
//! ## Key Principles
//! 
//! - Use `///` for item documentation (functions, structs, etc.)
//! - Use `//!` for module/crate level documentation  
//! - Write documentation from the user's perspective
//! - Include examples that actually compile and run
//! - Document all public APIs
//! 
//! ## Examples
//! 
//! ```rust
//! use brackets_basic::validate_brackets;
//! 
//! let result = validate_brackets("()");
//! assert!(result.is_ok());
//! ```

use mission1::Stack;

/// Represents the different types of bracket validation errors.
/// 
/// This enum categorizes the three fundamental types of bracket mismatches
/// that can occur during validation, providing specific context for each error type.
/// 
/// # Variants
/// 
/// * [`UnexpectedClosing`](BracketErrorKind::UnexpectedClosing) - A closing bracket with no matching opener
/// * [`MismatchedPair`](BracketErrorKind::MismatchedPair) - Wrong type of closing bracket  
/// * [`UnclosedOpenings`](BracketErrorKind::UnclosedOpenings) - Opening bracket never closed
/// 
/// # Examples
/// 
/// ```rust
/// use brackets_basic::{validate_brackets, BracketErrorKind};
/// 
/// // Example of UnexpectedClosing
/// let result = validate_brackets(")");
/// if let Err(error) = result {
///     match error.kind {
///         BracketErrorKind::UnexpectedClosing { found } => {
///             assert_eq!(found, ')');
///         }
///         _ => panic!("Wrong error type"),
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BracketErrorKind {
    /// Found a closing bracket without a corresponding opening bracket.
    /// 
    /// This occurs when the stack is empty but we encounter a closing bracket.
    /// 
    /// # Fields
    /// 
    /// * `found` - The unexpected closing bracket character
    UnexpectedClosing { 
        /// The closing bracket that was found unexpectedly
        found: char 
    },
    
    /// Found a closing bracket that doesn't match the most recent opening bracket.
    /// 
    /// This violates the LIFO (Last-In-First-Out) requirement of proper nesting.
    /// 
    /// # Fields
    /// 
    /// * `expected` - The closing bracket we expected to see
    /// * `found` - The closing bracket we actually found
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use brackets_basic::validate_brackets;
    /// 
    /// // This should fail: expected ']' but found ')'
    /// let result = validate_brackets("[)");
    /// assert!(result.is_err());
    /// ```
    MismatchedPair { 
        /// The closing bracket we expected based on the most recent opener
        expected: char, 
        /// The closing bracket we actually encountered
        found: char 
    },
    
    /// Found unclosed opening brackets at the end of input.
    /// 
    /// This occurs when the input ends but there are still unmatched
    /// opening brackets on the stack.
    /// 
    /// # Fields
    /// 
    /// * `expected` - The closing bracket needed for the first unclosed opener
    /// * `open_index` - Position in the input where the unclosed bracket was found
    UnclosedOpenings { 
        /// The closing bracket character needed to close the opener
        expected: char, 
        /// Zero-based index where the unclosed opening bracket appears
        open_index: usize 
    },
}

/// A bracket validation error containing position and error type information.
/// 
/// This struct provides complete context about bracket validation failures,
/// including the exact position where the error occurred and the specific
/// type of error encountered.
/// 
/// # Fields
/// 
/// * `index` - Zero-based character position where the error was detected
/// * `kind` - Specific type of bracket error that occurred
/// 
/// # Examples
/// 
/// ```rust
/// use brackets_basic::{validate_brackets, BracketErrorKind};
/// 
/// let result = validate_brackets("(]");
/// match result {
///     Err(error) => {
///         assert_eq!(error.index, 1); // Error at position 1
///         match error.kind {
///             BracketErrorKind::MismatchedPair { expected, found } => {
///                 assert_eq!(expected, ')');
///                 assert_eq!(found, ']');
///             }
///             _ => panic!("Unexpected error type"),
///         }
///     }
///     Ok(()) => panic!("Should have failed"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BracketError {
    /// Zero-based index in the input string where the error was detected
    pub index: usize,
    /// Specific type of bracket validation error
    pub kind: BracketErrorKind,
}

/// Maps opening bracket characters to their corresponding closing characters.
/// 
/// This is an internal helper function that defines the valid bracket pairs
/// supported by the validator.
/// 
/// # Arguments
/// 
/// * `c` - The character to check
/// 
/// # Returns
/// 
/// * `Some(char)` - The corresponding closing bracket if `c` is an opening bracket
/// * `None` - If `c` is not a recognized opening bracket
/// 
/// # Supported Brackets
/// 
/// * `(` → `)`  
/// * `[` → `]`
/// * `{` → `}`
/// 
/// # Examples
/// 
/// ```rust
/// # fn opening_to_expected_closer(c: char) -> Option<char> {
/// #     match c {
/// #         '(' => Some(')'),
/// #         '[' => Some(']'),
/// #         '{' => Some('}'),
/// #         _ => None,
/// #     }
/// # }
/// assert_eq!(opening_to_expected_closer('('), Some(')'));
/// assert_eq!(opening_to_expected_closer('['), Some(']'));
/// assert_eq!(opening_to_expected_closer('a'), None);
/// ```
fn opening_to_expected_closer(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        _   => None,
    }
}

/// Determines if a character is a closing bracket.
/// 
/// # Arguments
/// 
/// * `c` - The character to test
/// 
/// # Returns
/// 
/// `true` if the character is `)`, `]`, or `}`, `false` otherwise.
/// 
/// # Examples
/// 
/// ```rust
/// # fn is_closing(c: char) -> bool {
/// #     matches!(c, ')' | ']' | '}')
/// # }
/// assert!(is_closing(')'));
/// assert!(is_closing(']'));
/// assert!(is_closing('}'));
/// assert!(!is_closing('('));
/// assert!(!is_closing('a'));
/// ```
fn is_closing(c: char) -> bool {
    matches!(c, ')' | ']' | '}')
}

/// Validates that brackets in the input string are properly matched and nested.
/// 
/// This function implements a classic stack-based algorithm to ensure that:
/// 1. Every opening bracket has a corresponding closing bracket
/// 2. Brackets are properly nested (LIFO order)
/// 3. Bracket types match (e.g., `(` closes with `)`, not `]`)
/// 
/// All non-bracket characters are ignored during validation.
/// 
/// # Arguments
/// 
/// * `s` - The input string to validate
/// 
/// # Returns
/// 
/// * `Ok(())` - If all brackets are properly matched and nested
/// * `Err(BracketError)` - If validation fails, with details about the error
/// 
/// # Supported Bracket Types
/// 
/// * Parentheses: `(` and `)`
/// * Square brackets: `[` and `]`  
/// * Curly braces: `{` and `}`
/// 
/// # Time Complexity
/// 
/// O(n) where n is the length of the input string.
/// 
/// # Space Complexity
/// 
/// O(n) in the worst case (when all characters are opening brackets).
/// 
/// # Examples
/// 
/// ## Valid bracket sequences
/// 
/// ```rust
/// use brackets_basic::validate_brackets;
/// 
/// // Empty string is valid
/// assert!(validate_brackets("").is_ok());
/// 
/// // Simple pairs
/// assert!(validate_brackets("()").is_ok());
/// assert!(validate_brackets("[]").is_ok());
/// assert!(validate_brackets("{}").is_ok());
/// 
/// // Nested brackets  
/// assert!(validate_brackets("([{}])").is_ok());
/// 
/// // Mixed with other characters
/// assert!(validate_brackets("hello(world[test{data}more]end)").is_ok());
/// ```
/// 
/// ## Invalid bracket sequences
/// 
/// ```rust
/// use brackets_basic::{validate_brackets, BracketErrorKind};
/// 
/// // Unexpected closing bracket
/// let result = validate_brackets(")");
/// assert!(matches!(result, Err(error) if matches!(error.kind, BracketErrorKind::UnexpectedClosing { .. })));
/// 
/// // Mismatched pair
/// let result = validate_brackets("(]");
/// assert!(matches!(result, Err(error) if matches!(error.kind, BracketErrorKind::MismatchedPair { .. })));
/// 
/// // Unclosed opening
/// let result = validate_brackets("(");
/// assert!(matches!(result, Err(error) if matches!(error.kind, BracketErrorKind::UnclosedOpenings { .. })));
/// ```
/// 
/// # Algorithm Details
/// 
/// The algorithm uses a stack to track "promises" - when we see an opening
/// bracket, we push the expected closing bracket onto the stack. When we see
/// a closing bracket, we pop from the stack and verify it matches.
/// 
/// This approach naturally handles the nested structure requirement because
/// the stack enforces LIFO (Last-In-First-Out) ordering, which corresponds
/// to proper nesting.
/// 
/// # Error Reporting
/// 
/// When validation fails, the error includes:
/// - The exact character position where the error was detected
/// - The specific type of error (unexpected closing, mismatch, or unclosed)
/// - Additional context like expected vs. found characters
/// 
/// The algorithm reports the *first* error encountered, making it useful
/// for providing actionable feedback to users.
pub fn validate_brackets(s: &str) -> Result<(), BracketError> {
    // Stack stores (expected_closer, open_index) pairs
    // The open_index helps us report where unclosed brackets were opened
    let mut st: Stack<(char, usize)> = Stack::new();
    
    for (i, ch) in s.char_indices() {
        if let Some(expected) = opening_to_expected_closer(ch) {
            // Found an opening bracket - push the expected closer
            st.push((expected, i));
        } else if is_closing(ch) {
            // Found a closing bracket - verify it matches expectation
            match st.pop() {
                None => {
                    // Stack is empty but we found a closing bracket
                    return Err(BracketError {
                        index: i,
                        kind: BracketErrorKind::UnexpectedClosing { found: ch },
                    });
                }
                Some((expected, _open_idx)) => {
                    // Check if the closing bracket matches what we expected
                    if ch != expected {
                        return Err(BracketError {
                            index: i,
                            kind: BracketErrorKind::MismatchedPair { expected, found: ch },
                        });
                    }
                    // Match successful - continue processing
                }
            }
        }
        // Ignore all other characters (non-brackets)
    }
    
    // Check for unclosed opening brackets
    if let Some((expected, open_idx)) = st.pop() {
        return Err(BracketError {
            index: open_idx,
            kind: BracketErrorKind::UnclosedOpenings { expected, open_index: open_idx },
        });
    }
    
    // All brackets properly matched
    Ok(())
}

fn main() {
    println!("📚 Rust Documentation Standards Example");
    println!("This demonstrates comprehensive documentation practices for Rust projects.");
}