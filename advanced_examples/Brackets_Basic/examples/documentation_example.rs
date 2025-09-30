// Let me show you how to improve the brackets.rs documentation
// This demonstrates applying Rust documentation standards to your existing code

use brackets_basic::{validate_brackets, BracketErrorKind};

fn main() {
    println!("📚 Rust Documentation Standards Applied to Brackets Validation\n");
    
    // Let's test the documented functionality
    demonstrate_validation_with_documentation();
}

fn demonstrate_validation_with_documentation() {
    println!("🎯 Testing the documented behavior:\n");
    
    // Test cases that would be in the documentation
    let test_cases = vec![
        ("", "Empty string (edge case)"),
        ("()", "Simple parentheses"),
        ("[]", "Simple square brackets"),
        ("{}", "Simple curly braces"),
        ("([{}])", "Nested all types"),
        ("hello(world[test{data}more]end)", "Mixed with text"),
        (")", "Unexpected closing"),
        ("(]", "Mismatched pair"),
        ("(((", "Unclosed openings"),
    ];
    
    for (input, description) in test_cases {
        match validate_brackets(input) {
            Ok(()) => println!("✅ '{}' → Valid ({})", input, description),
            Err(error) => {
                let error_type = match error.kind {
                    BracketErrorKind::UnexpectedClosing { .. } => "UnexpectedClosing",
                    BracketErrorKind::MismatchedPair { .. } => "MismatchedPair",
                    BracketErrorKind::UnclosedOpenings { .. } => "UnclosedOpenings",
                };
                println!("❌ '{}' → {} at pos {} ({})", input, error_type, error.index, description);
            }
        }
    }
}

/* 
Here's how the current brackets.rs could be improved with proper documentation:

//! Bracket validation module providing comprehensive bracket matching functionality.
//! 
//! This module implements a stack-based algorithm for validating bracket sequences,
//! ensuring proper matching and nesting according to standard rules.

use crate::stack::Stack;

/// The different types of bracket validation errors that can occur.
/// 
/// This enum categorizes the three fundamental bracket validation failures,
/// providing specific context for each type of error.
/// 
/// # Variants
/// 
/// * [`UnexpectedClosing`] - A closing bracket with no matching opener
/// * [`MismatchedPair`] - Wrong type of closing bracket for the most recent opener  
/// * [`UnclosedOpenings`] - Opening brackets that were never closed
/// 
/// # Examples
/// 
/// ```rust
/// use brackets_basic::{validate_brackets, BracketErrorKind};
/// 
/// // UnexpectedClosing example
/// let result = validate_brackets(")");
/// if let Err(error) = result {
///     match error.kind {
///         BracketErrorKind::UnexpectedClosing { found } => {
///             println!("Found unexpected '{}' at position {}", found, error.index);
///         }
///         _ => {}
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BracketErrorKind {
    /// Found a closing bracket without a corresponding opening bracket.
    /// 
    /// This occurs when the validation stack is empty but a closing bracket
    /// is encountered in the input.
    /// 
    /// # Fields
    /// 
    /// * `found` - The unexpected closing bracket character
    UnexpectedClosing { 
        /// The closing bracket that appeared without a matching opener
        found: char 
    },
    
    /// Found a closing bracket that doesn't match the most recent opening bracket.
    /// 
    /// This violates proper nesting order - brackets must be closed in LIFO
    /// (Last-In-First-Out) order to maintain proper nesting.
    /// 
    /// # Fields
    /// 
    /// * `expected` - The closing bracket we expected based on the most recent opener
    /// * `found` - The closing bracket we actually encountered
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use brackets_basic::{validate_brackets, BracketErrorKind};
    /// 
    /// let result = validate_brackets("(]");
    /// if let Err(error) = result {
    ///     match error.kind {
    ///         BracketErrorKind::MismatchedPair { expected, found } => {
    ///             assert_eq!(expected, ')');
    ///             assert_eq!(found, ']');
    ///         }
    ///         _ => panic!("Wrong error type"),
    ///     }
    /// }
    /// ```
    MismatchedPair { 
        /// The closing bracket character that was expected
        expected: char, 
        /// The closing bracket character that was actually found
        found: char 
    },
    
    /// Found unclosed opening brackets when input ended.
    /// 
    /// This occurs when the input string ends but there are still unmatched
    /// opening brackets remaining on the validation stack.
    /// 
    /// # Fields
    /// 
    /// * `expected` - The closing bracket needed for the first unclosed opener
    /// * `open_index` - Position where the unclosed opening bracket was found
    UnclosedOpenings { 
        /// The closing bracket character needed to close the first unclosed opener
        expected: char, 
        /// Zero-based index where the unclosed opening bracket appears in the input
        open_index: usize 
    },
}

/// A bracket validation error with position and type information.
/// 
/// This struct provides complete context about bracket validation failures,
/// including the exact position where the error was detected and the specific
/// type of error that occurred.
/// 
/// # Fields
/// 
/// * `index` - Zero-based character position where the error was detected
/// * `kind` - The specific type of bracket validation error
/// 
/// # Examples
/// 
/// ```rust
/// use brackets_basic::{validate_brackets, BracketErrorKind};
/// 
/// let result = validate_brackets("hello(world]");
/// match result {
///     Err(error) => {
///         println!("Error at position {}", error.index);
///         match error.kind {
///             BracketErrorKind::MismatchedPair { expected, found } => {
///                 println!("Expected '{}' but found '{}'", expected, found);
///             }
///             _ => {}
///         }
///     }
///     Ok(()) => println!("Valid bracket sequence"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BracketError {
    /// Zero-based index in the input string where the error was detected
    pub index: usize,
    /// The specific type of bracket validation error that occurred
    pub kind: BracketErrorKind,
}

/// Maps opening bracket characters to their corresponding closing characters.
/// 
/// This helper function defines the valid bracket pairs supported by the
/// validation algorithm.
/// 
/// # Arguments
/// 
/// * `c` - The character to check for opening bracket status
/// 
/// # Returns
/// 
/// * `Some(char)` - The corresponding closing bracket if `c` is a valid opener
/// * `None` - If `c` is not a recognized opening bracket
/// 
/// # Supported Bracket Pairs
/// 
/// * `(` → `)`
/// * `[` → `]`  
/// * `{` → `}`
fn opening_to_expected_closer(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        _   => None,
    }
}

/// Determines whether a character is a closing bracket.
/// 
/// # Arguments
/// 
/// * `c` - The character to test
/// 
/// # Returns
/// 
/// `true` if the character is `)`, `]`, or `}`, `false` otherwise.
fn is_closing(c: char) -> bool {
    matches!(c, ')' | ']' | '}')
}

/// Validates that brackets in the input are properly matched and nested.
/// 
/// This function implements a classic stack-based algorithm to ensure brackets
/// follow proper nesting rules. Only `()`, `[]`, and `{}` are considered
/// brackets - all other characters in the input are completely ignored.
/// 
/// The algorithm enforces these rules:
/// 1. Every opening bracket must have a corresponding closing bracket
/// 2. Brackets must be properly nested (LIFO order)  
/// 3. Bracket types must match (e.g., `(` must close with `)`, not `]`)
/// 
/// # Arguments
/// 
/// * `s` - The input string to validate
/// 
/// # Returns
/// 
/// * `Ok(())` - If all brackets are properly matched and nested
/// * `Err(BracketError)` - If validation fails, with error position and type
/// 
/// # Errors
/// 
/// This function returns `BracketError` in the following cases:
/// 
/// * `UnexpectedClosing` - Found `)`, `]`, or `}` without a matching opener
/// * `MismatchedPair` - Found wrong closing bracket type (e.g., `(` closed by `]`)
/// * `UnclosedOpenings` - Input ended with unmatched opening brackets remaining
/// 
/// # Time Complexity
/// 
/// O(n) where n is the length of the input string. Each character is processed
/// exactly once.
/// 
/// # Space Complexity
/// 
/// O(n) in the worst case, when the input consists entirely of opening brackets.
/// The space is used by the internal stack to track unmatched opening brackets.
/// 
/// # Examples
/// 
/// ## Valid bracket sequences
/// 
/// ```rust
/// use brackets_basic::validate_brackets;
/// 
/// // Empty strings are valid
/// assert!(validate_brackets("").is_ok());
/// 
/// // Simple matched pairs
/// assert!(validate_brackets("()").is_ok());
/// assert!(validate_brackets("[]").is_ok());
/// assert!(validate_brackets("{}").is_ok());
/// 
/// // Properly nested sequences
/// assert!(validate_brackets("([{}])").is_ok());
/// assert!(validate_brackets("{[()]}").is_ok());
/// 
/// // Mixed with other characters (which are ignored)
/// assert!(validate_brackets("hello(world[test{data}more]end)").is_ok());
/// assert!(validate_brackets("func(a, b) { return a + b; }").is_ok());
/// ```
/// 
/// ## Invalid bracket sequences
/// 
/// ```rust
/// use brackets_basic::{validate_brackets, BracketErrorKind};
/// 
/// // Unexpected closing brackets
/// let result = validate_brackets(")");
/// assert!(matches!(result, Err(error) if matches!(error.kind, BracketErrorKind::UnexpectedClosing { .. })));
/// 
/// // Mismatched bracket types
/// let result = validate_brackets("(]");
/// assert!(matches!(result, Err(error) if matches!(error.kind, BracketErrorKind::MismatchedPair { .. })));
/// 
/// // Unclosed opening brackets
/// let result = validate_brackets("(((");
/// assert!(matches!(result, Err(error) if matches!(error.kind, BracketErrorKind::UnclosedOpenings { .. })));
/// ```
/// 
/// ## Error position reporting
/// 
/// ```rust
/// use brackets_basic::validate_brackets;
/// 
/// let result = validate_brackets("hello(world]");
/// if let Err(error) = result {
///     assert_eq!(error.index, 11); // Position of the ']' character
/// }
/// ```
pub fn validate_brackets(s: &str) -> Result<(), BracketError> {
    // Stack stores (expected_closer, open_index) pairs
    // The open_index helps us report where unclosed brackets were opened
    let mut st: Stack<(char, usize)> = Stack::new();
    
    for (i, ch) in s.char_indices() {
        if let Some(expected) = opening_to_expected_closer(ch) {
            // Found an opening bracket - push expected closer and position
            st.push((expected, i));
        } else if is_closing(ch) {
            // Found a closing bracket - verify it matches our expectation
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
        // All other characters are ignored (not brackets)
    }
    
    // Check if there are any unclosed opening brackets remaining
    if let Some((expected, open_idx)) = st.pop() {
        return Err(BracketError {
            index: open_idx,
            kind: BracketErrorKind::UnclosedOpenings { expected, open_index: open_idx },
        });
    }
    
    // All brackets properly matched and nested
    Ok(())
}

*/