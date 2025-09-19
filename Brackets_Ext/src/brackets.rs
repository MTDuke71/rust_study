//! Advanced bracket validation with configurable alphabets and error handling.
//!
//! This module provides the core bracket validation functionality with support for:
//! - Custom bracket alphabets beyond the standard `()`, `[]`, `{}`
//! - Flexible error reporting (stop-at-first vs collect-all)  
//! - Configurable policies for handling unclosed brackets
//! - Multiple validation APIs for different input types
//!
//! # Examples
//!
//! ```rust
//! use brackets_extended::*;
//!
//! // Basic validation
//! assert!(validate_brackets("([{}])").is_ok());
//!
//! // Custom alphabet with angle brackets
//! let mut opts = Options::default();
//! opts.alphabet = Alphabet::with_pairs(&[('(', ')'), ('<', '>')]);
//! assert!(validate_with_options("(<content>)", &opts).is_ok());
//!
//! // Collect all errors instead of stopping at first
//! opts.error_mode = ErrorMode::CollectAll;
//! let errors = validate_with_options(")](", &opts).unwrap_err();
//! assert!(errors.len() > 1);
//! ```

use mission1::Stack;
use std::collections::HashMap;

/// Defines the set of valid bracket pairs for validation.
///
/// An alphabet specifies which characters are considered opening brackets
/// and their corresponding closing bracket characters. This allows the
/// validation system to support custom bracket types beyond the standard
/// parentheses `()`, square brackets `[]`, and curly braces `{}`.
///
/// # Examples
///
/// ```rust
/// use brackets_extended::Alphabet;
///
/// // Default ASCII brackets: (), [], {}
/// let default = Alphabet::default_ascii();
/// assert!(default.is_opener('('));
/// assert!(default.is_closer(')'));
/// assert_eq!(default.expected_for('('), Some(')'));
///
/// // Custom alphabet with angle brackets
/// let custom = Alphabet::with_pairs(&[
///     ('(', ')'),
///     ('[', ']'), 
///     ('<', '>'),  // Add angle brackets
/// ]);
/// assert!(custom.is_opener('<'));
/// assert!(custom.is_closer('>'));
///
/// // Mathematical notation
/// let math = Alphabet::with_pairs(&[
///     ('⟨', '⟩'),  // Angle brackets for inner products
///     ('⌊', '⌋'),  // Floor function brackets
///     ('⌈', '⌉'),  // Ceiling function brackets
/// ]);
/// ```
///
/// # Performance
///
/// - `is_opener()`: O(1) hash table lookup
/// - `is_closer()`: O(n) where n is the number of bracket pairs
/// - `expected_for()`: O(1) hash table lookup
#[derive(Clone, Debug)]
pub struct Alphabet { 
    /// Mapping from opening bracket characters to their corresponding closing characters
    pub open_to_close: HashMap<char, char> 
}

impl Alphabet {
    /// Creates a new alphabet from an array of (opener, closer) pairs.
    ///
    /// # Arguments
    ///
    /// * `pairs` - A slice of tuples where each tuple contains an opening
    ///   bracket character and its corresponding closing bracket character.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brackets_extended::Alphabet;
    ///
    /// // Standard programming brackets
    /// let code = Alphabet::with_pairs(&[
    ///     ('(', ')'),
    ///     ('[', ']'),
    ///     ('{', '}'),
    /// ]);
    ///
    /// // HTML/XML tags (simplified)
    /// let html = Alphabet::with_pairs(&[('<', '>')]);
    ///
    /// // Mathematical notation
    /// let math = Alphabet::with_pairs(&[
    ///     ('⟨', '⟩'),  // Angle brackets
    ///     ('⌊', '⌋'),  // Floor brackets
    /// ]);
    /// ```
    pub fn with_pairs(pairs: &[(char, char)]) -> Self {
        let mut m = HashMap::new();
        for &(o, c) in pairs { m.insert(o, c); }
        Self { open_to_close: m }
    }
    
    /// Creates the default ASCII alphabet with standard programming brackets.
    ///
    /// Supports the three most common bracket types:
    /// - Parentheses: `()`
    /// - Square brackets: `[]`  
    /// - Curly braces: `{}`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brackets_extended::Alphabet;
    ///
    /// let alphabet = Alphabet::default_ascii();
    /// assert!(alphabet.is_opener('('));
    /// assert!(alphabet.is_opener('['));
    /// assert!(alphabet.is_opener('{'));
    /// assert!(alphabet.is_closer(')'));
    /// assert!(alphabet.is_closer(']'));
    /// assert!(alphabet.is_closer('}'));
    ///
    /// // Angle brackets not supported by default
    /// assert!(!alphabet.is_opener('<'));
    /// ```
    pub fn default_ascii() -> Self {
        Self::with_pairs(&[('(', ')'), ('[', ']'), ('{', '}')])
    }
    
    /// Checks if a character is an opening bracket in this alphabet.
    ///
    /// # Performance
    ///
    /// This operation is O(1) using hash table lookup.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brackets_extended::Alphabet;
    ///
    /// let alphabet = Alphabet::default_ascii();
    /// assert!(alphabet.is_opener('('));
    /// assert!(alphabet.is_opener('['));
    /// assert!(!alphabet.is_opener(')'));
    /// assert!(!alphabet.is_opener('x'));
    /// ```
    #[inline] 
    pub fn is_opener(&self, ch: char) -> bool { 
        self.open_to_close.contains_key(&ch) 
    }
    
    /// Checks if a character is a closing bracket in this alphabet.
    ///
    /// # Performance
    ///
    /// This operation is O(n) where n is the number of bracket pairs,
    /// as it searches through all possible closing characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brackets_extended::Alphabet;
    ///
    /// let alphabet = Alphabet::default_ascii();
    /// assert!(alphabet.is_closer(')'));
    /// assert!(alphabet.is_closer(']'));
    /// assert!(!alphabet.is_closer('('));
    /// assert!(!alphabet.is_closer('x'));
    /// ```
    #[inline] 
    pub fn is_closer(&self, ch: char) -> bool { 
        self.open_to_close.values().any(|&v| v == ch) 
    }
    
    /// Returns the expected closing character for a given opening bracket.
    ///
    /// # Arguments
    ///
    /// * `opener` - The opening bracket character
    ///
    /// # Returns
    ///
    /// * `Some(char)` - The corresponding closing bracket character
    /// * `None` - If the character is not a valid opener in this alphabet
    ///
    /// # Performance
    ///
    /// This operation is O(1) using hash table lookup.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brackets_extended::Alphabet;
    ///
    /// let alphabet = Alphabet::default_ascii();
    /// assert_eq!(alphabet.expected_for('('), Some(')'));
    /// assert_eq!(alphabet.expected_for('['), Some(']'));
    /// assert_eq!(alphabet.expected_for('{'), Some('}'));
    /// assert_eq!(alphabet.expected_for('<'), None);
    /// assert_eq!(alphabet.expected_for('x'), None);
    /// ```
    #[inline] 
    pub fn expected_for(&self, opener: char) -> Option<char> { 
        self.open_to_close.get(&opener).copied() 
    }
}

/// Specific types of bracket validation errors.
///
/// This enum provides detailed information about what kind of bracket
/// error occurred, enabling precise error reporting and recovery strategies.
///
/// # Examples
///
/// ```rust
/// use brackets_extended::{validate_brackets, BracketErrorKind};
///
/// // Unexpected closing bracket
/// if let Err(err) = validate_brackets(")") {
///     match err.kind {
///         BracketErrorKind::UnexpectedClosing { found } => {
///             println!("Unexpected '{}' at position {}", found, err.index);
///         }
///         _ => {}
///     }
/// }
///
/// // Mismatched pair
/// if let Err(err) = validate_brackets("(]") {
///     match err.kind {
///         BracketErrorKind::MismatchedPair { expected, found } => {
///             println!("Expected '{}' but found '{}' at position {}", 
///                     expected, found, err.index);
///         }
///         _ => {}
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BracketErrorKind {
    /// An unexpected closing bracket was encountered with no matching opener.
    ///
    /// This occurs when a closing bracket appears without a corresponding
    /// opening bracket, either at the start of input or after all previous
    /// brackets have been properly closed.
    ///
    /// # Fields
    ///
    /// * `found` - The unexpected closing bracket character
    UnexpectedClosing { 
        /// The closing bracket character that was encountered unexpectedly
        found: char 
    },
    
    /// A closing bracket doesn't match its corresponding opening bracket.
    ///
    /// This occurs when brackets are interleaved incorrectly, such as `([)]`
    /// where the `]` closes the `(` instead of the `[`.
    ///
    /// # Fields
    ///
    /// * `expected` - The closing bracket that was expected
    /// * `found` - The closing bracket that was actually found
    MismatchedPair { 
        /// The closing bracket character that was expected
        expected: char, 
        /// The closing bracket character that was actually found
        found: char 
    },
    
    /// One or more opening brackets were never closed.
    ///
    /// This occurs at the end of input when there are still opening brackets
    /// on the stack. The specific bracket reported depends on the configured
    /// `UnclosedPolicy` (latest-open vs earliest-open).
    ///
    /// # Fields
    ///
    /// * `expected` - The closing bracket that was needed
    /// * `open_index` - The position where the unclosed opening bracket appeared
    UnclosedOpenings { 
        /// The closing bracket character that was expected
        expected: char, 
        /// The position in the input where the unclosed opening bracket appeared
        open_index: usize 
    },
}

/// A bracket validation error with position information.
///
/// This structure provides complete information about a bracket validation
/// error, including both the position where the error occurred and the
/// specific type of error.
///
/// # Examples
///
/// ```rust
/// use brackets_extended::{validate_brackets, BracketError, BracketErrorKind};
///
/// if let Err(error) = validate_brackets("(]") {
///     println!("Error at position {}: {:?}", error.index, error.kind);
///     
///     match error.kind {
///         BracketErrorKind::MismatchedPair { expected, found } => {
///             println!("Expected '{}' but found '{}'", expected, found);
///         }
///         _ => {}
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BracketError { 
    /// The byte position in the input where the error occurred
    pub index: usize, 
    /// The specific type of bracket error that occurred
    pub kind: BracketErrorKind 
}

/// Controls how the validator handles multiple errors during validation.
///
/// This enum determines whether validation stops immediately upon finding
/// the first error or continues to collect all possible errors in the input.
///
/// # Variants
///
/// * `StopAtFirst` - Stop validation at the first error (traditional behavior)
/// * `CollectAll` - Continue validation and collect all errors found
///
/// # Examples
///
/// ```rust
/// use brackets_extended::{Options, ErrorMode, validate_with_options};
///
/// let input = ")]("; // Multiple errors: unexpected closer + unclosed opener
///
/// // Stop at first error (default)
/// let mut opts = Options::default();
/// opts.error_mode = ErrorMode::StopAtFirst;
/// let errors = validate_with_options(input, &opts).unwrap_err();
/// assert_eq!(errors.len(), 1); // Only the first error
///
/// // Collect all errors
/// opts.error_mode = ErrorMode::CollectAll;  
/// let errors = validate_with_options(input, &opts).unwrap_err();
/// assert!(errors.len() > 1); // Multiple errors collected
/// ```
///
/// # Use Cases
///
/// - **StopAtFirst**: Best for performance-critical applications, traditional parsers
/// - **CollectAll**: Best for IDEs, linters, educational tools, comprehensive error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorMode { 
    /// Stop validation immediately upon finding the first error
    StopAtFirst, 
    /// Continue validation and collect all errors found
    CollectAll 
}

/// Controls which unclosed bracket to report when multiple brackets remain unclosed.
///
/// When validation reaches the end of input with unclosed brackets on the stack,
/// this policy determines which bracket position to report in the error.
///
/// # Variants
///
/// * `LatestOpen` - Report the most recently opened bracket (LIFO/stack behavior)
/// * `EarliestOpen` - Report the earliest opened bracket (FIFO/queue behavior)
///
/// # Examples
///
/// ```rust
/// use brackets_extended::{Options, UnclosedPolicy, validate_with_options};
///
/// let input = "(((["; // Multiple nested unclosed brackets
///
/// // Report latest opened bracket (position 3: the '[')
/// let mut latest_opts = Options::default();
/// latest_opts.unclosed_policy = UnclosedPolicy::LatestOpen;
/// // Error will report position 3
///
/// // Report earliest opened bracket (position 0: first '(')  
/// let mut earliest_opts = Options::default();
/// earliest_opts.unclosed_policy = UnclosedPolicy::EarliestOpen;
/// // Error will report position 0
/// ```
///
/// # Use Cases
///
/// - **LatestOpen**: Natural for stack-based thinking, "what was I just working on?"
/// - **EarliestOpen**: Better for structural analysis, "where did the problem start?"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnclosedPolicy { 
    /// Report the most recently opened bracket (LIFO behavior)
    LatestOpen, 
    /// Report the earliest opened bracket (FIFO behavior)
    EarliestOpen 
}

/// Configuration options for bracket validation.
///
/// This structure combines all configurable aspects of bracket validation,
/// allowing fine-tuned control over validation behavior for different use cases.
///
/// # Examples
///
/// ```rust
/// use brackets_extended::{Options, Alphabet, ErrorMode, UnclosedPolicy};
///
/// // Default configuration
/// let default_opts = Options::default();
///
/// // Custom configuration for IDE-like behavior
/// let ide_opts = Options {
///     alphabet: Alphabet::default_ascii(),
///     error_mode: ErrorMode::CollectAll,        // Show all errors
///     unclosed_policy: UnclosedPolicy::LatestOpen, // LIFO reporting
/// };
///
/// // Configuration for HTML validation
/// let html_opts = Options {
///     alphabet: Alphabet::with_pairs(&[('<', '>')]),
///     error_mode: ErrorMode::StopAtFirst,
///     unclosed_policy: UnclosedPolicy::EarliestOpen,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Options {
    /// The alphabet defining valid bracket pairs
    pub alphabet: Alphabet,
    /// How to handle multiple errors during validation
    pub error_mode: ErrorMode,
    /// Which unclosed bracket to report when multiple are unclosed
    pub unclosed_policy: UnclosedPolicy,
}

impl Default for Options {
    /// Creates default validation options suitable for most common use cases.
    ///
    /// Default configuration:
    /// - **Alphabet**: Standard ASCII brackets `()`, `[]`, `{}`
    /// - **Error Mode**: Stop at first error (traditional behavior)
    /// - **Unclosed Policy**: Report latest opened bracket (LIFO)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use brackets_extended::{Options, validate_with_options};
    ///
    /// let opts = Options::default();
    /// assert!(validate_with_options("([{}])", &opts).is_ok());
    /// assert!(validate_with_options("([)]", &opts).is_err());
    /// ```
    fn default() -> Self {
        Self {
            alphabet: Alphabet::default_ascii(),
            error_mode: ErrorMode::StopAtFirst,
            unclosed_policy: UnclosedPolicy::LatestOpen,
        }
    }
}

/// Core bracket validation engine that works with indexed character iterators.
///
/// This is the fundamental validation function that all other validation APIs
/// ultimately delegate to. It processes characters along with their positions,
/// maintaining a stack of expected closing brackets and their positions.
///
/// # Arguments
///
/// * `iter` - An iterator yielding `(index, char)` pairs where `index` is the
///   byte position of the character in the original input
/// * `opts` - Configuration options controlling validation behavior
///
/// # Returns
///
/// * `Ok(())` - If all brackets are properly matched and nested
/// * `Err(Vec<BracketError>)` - If validation fails, with detailed error information
///
/// # Performance
///
/// - **Time Complexity**: O(n) where n is the number of characters
/// - **Space Complexity**: O(d) where d is the maximum nesting depth
/// - **Memory**: Minimal allocation, reuses provided iterator
///
/// # Algorithm
///
/// 1. Iterate through each `(position, character)` pair
/// 2. For opening brackets: push expected closer and position onto stack
/// 3. For closing brackets: check against top of stack for matches
/// 4. Handle errors according to configured `ErrorMode`
/// 5. At end, report unclosed brackets according to `UnclosedPolicy`
///
/// # Examples
///
/// ```rust
/// use brackets_extended::{validate_indexed_iter, Options};
///
/// let input = "hello(world]";
/// let indexed_chars: Vec<(usize, char)> = input.char_indices().collect();
/// let opts = Options::default();
///
/// let result = validate_indexed_iter(indexed_chars, &opts);
/// assert!(result.is_err()); // Mismatched pair: ( and ]
/// ```
pub fn validate_indexed_iter<I>(iter: I, opts: &Options) -> Result<(), Vec<BracketError>>
where
    I: IntoIterator<Item = (usize, char)>,
{
    let mut st: Stack<(char, usize)> = Stack::new(); // (expected_closer, open_idx)
    let mut errors: Vec<BracketError> = Vec::new();
    let alph = &opts.alphabet;

    for (i, ch) in iter {
        if alph.is_opener(ch) {
            let expected = alph.expected_for(ch).expect("alphabet inconsistent");
            st.push((expected, i));
        } else if alph.is_closer(ch) {
            match st.pop() {
                None => {
                    let e = BracketError { index: i, kind: BracketErrorKind::UnexpectedClosing { found: ch } };
                    if matches!(opts.error_mode, ErrorMode::StopAtFirst) { return Err(vec![e]); }
                    errors.push(e);
                }
                Some((expected, _open_idx)) => {
                    if ch != expected {
                        let e = BracketError { index: i, kind: BracketErrorKind::MismatchedPair { expected, found: ch } };
                        if matches!(opts.error_mode, ErrorMode::StopAtFirst) { return Err(vec![e]); }
                        errors.push(e);
                    }
                }
            }
        } else {
            // ignore
        }
    }

    if !st.is_empty() {
        match opts.unclosed_policy {
            UnclosedPolicy::LatestOpen => {
                if let Some((expected, open_idx)) = st.pop() {
                    let e = BracketError { index: open_idx, kind: BracketErrorKind::UnclosedOpenings { expected, open_index: open_idx } };
                    if matches!(opts.error_mode, ErrorMode::StopAtFirst) { return Err(vec![e]); }
                    errors.push(e);
                }
            }
            UnclosedPolicy::EarliestOpen => {
                let mut tmp: Vec<(char, usize)> = Vec::new();
                while let Some(x) = st.pop() { tmp.push(x); }
                if let Some(&(expected, open_idx)) = tmp.last() {
                    let e = BracketError { index: open_idx, kind: BracketErrorKind::UnclosedOpenings { expected, open_index: open_idx } };
                    if matches!(opts.error_mode, ErrorMode::StopAtFirst) { return Err(vec![e]); }
                    errors.push(e);
                }
            }
        }
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

/// Traditional bracket validation API with single error reporting.
///
/// This function provides backward compatibility with simpler validation APIs
/// that expect a single error. It uses default options (ASCII brackets, stop-at-first)
/// and returns only the first error encountered.
///
/// # Arguments
///
/// * `s` - The string to validate
///
/// # Returns
///
/// * `Ok(())` - If all brackets are properly matched and nested
/// * `Err(BracketError)` - The first error encountered (if any)
///
/// # Examples
///
/// ```rust
/// use brackets_extended::{validate_brackets, BracketErrorKind};
///
/// // Valid bracket sequences
/// assert!(validate_brackets("").is_ok());
/// assert!(validate_brackets("()").is_ok());
/// assert!(validate_brackets("([{}])").is_ok());
/// assert!(validate_brackets("function(arg1, arg2) { return [1, 2, 3]; }").is_ok());
///
/// // Invalid bracket sequences
/// assert!(validate_brackets("(").is_err());
/// assert!(validate_brackets(")").is_err());
/// assert!(validate_brackets("([)]").is_err());
///
/// // Detailed error information
/// if let Err(error) = validate_brackets("(]") {
///     assert_eq!(error.index, 1); // Error at position 1
///     match error.kind {
///         BracketErrorKind::MismatchedPair { expected, found } => {
///             assert_eq!(expected, ')');
///             assert_eq!(found, ']');
///         }
///         _ => panic!("Expected MismatchedPair error"),
///     }
/// }
/// ```
///
/// # Use Cases
///
/// - Simple validation where only pass/fail is needed
/// - Legacy code integration
/// - Performance-critical applications (stops at first error)
/// - Quick validation checks
pub fn validate_brackets(s: &str) -> Result<(), BracketError> {
    match validate_with_options(s, &Options::default()) {
        Ok(()) => Ok(()),
        Err(mut errs) => Err(errs.remove(0)),
    }
}

/// Configurable bracket validation API for strings with full error reporting.
///
/// This is the primary string validation API that supports all extended features:
/// custom alphabets, error collection modes, and unclosed policies. Returns
/// detailed error information with proper UTF-8 byte positioning.
///
/// # Arguments
///
/// * `s` - The string to validate
/// * `opts` - Configuration options controlling validation behavior
///
/// # Returns
///
/// * `Ok(())` - If all brackets are properly matched and nested
/// * `Err(Vec<BracketError>)` - Detailed errors with byte positions
///
/// # Examples
///
/// ```rust
/// use brackets_extended::{validate_with_options, Options, Alphabet, ErrorMode};
///
/// // Basic validation with default options
/// let opts = Options::default();
/// assert!(validate_with_options("([{}])", &opts).is_ok());
///
/// // Custom alphabet with angle brackets
/// let mut html_opts = Options::default();
/// html_opts.alphabet = Alphabet::with_pairs(&[('<', '>')]);
/// assert!(validate_with_options("<tag>content</tag>", &html_opts).is_ok());
///
/// // Collect all errors instead of stopping at first
/// let mut collect_opts = Options::default();
/// collect_opts.error_mode = ErrorMode::CollectAll;
/// let errors = validate_with_options(")](", &collect_opts).unwrap_err();
/// assert!(errors.len() > 1); // Multiple errors collected
///
/// // UTF-8 support with proper byte indexing
/// let result = validate_with_options("函数(参数]", &opts);
/// if let Err(errors) = result {
///     // Error positions are byte indices, not character indices
///     println!("Error at byte position: {}", errors[0].index);
/// }
/// ```
///
/// # Performance
///
/// - **Time Complexity**: O(n) where n is the number of bytes in the string
/// - **Space Complexity**: O(d) where d is the maximum nesting depth
/// - **UTF-8 Handling**: Correctly handles multi-byte characters
///
/// # Use Cases
///
/// - IDE integration with comprehensive error reporting
/// - Code linters and formatters
/// - Educational tools showing all bracket errors
/// - Configuration file validation
/// - Custom markup language validation
pub fn validate_with_options(s: &str, opts: &Options) -> Result<(), Vec<BracketError>> {
    let mut idx = 0usize;
    let iter = s.chars().map(move |ch| {
        let here = idx;
        idx += ch.len_utf8();
        (here, ch)
    });
    validate_indexed_iter(iter, opts)
}

/// Character-based streaming validation API.
///
/// This API works with character iterators and uses character positions
/// (not byte positions) for error reporting. Useful when working with
/// character-based input streams or when byte positions are not relevant.
///
/// # Arguments
///
/// * `iter` - An iterator yielding characters
/// * `opts` - Configuration options controlling validation behavior
///
/// # Returns
///
/// * `Ok(())` - If all brackets are properly matched and nested
/// * `Err(Vec<BracketError>)` - Errors with character positions (not byte positions)
///
/// # Examples
///
/// ```rust
/// use brackets_extended::{validate_iter, Options};
///
/// let opts = Options::default();
///
/// // Character iterator from string
/// let result = validate_iter("([)]".chars(), &opts);
/// assert!(result.is_err());
///
/// // Character iterator from vector
/// let chars = vec!['(', '[', ')', ']'];
/// let result = validate_iter(chars.into_iter(), &opts);
/// if let Err(errors) = result {
///     // Error index is character position (2), not byte position
///     assert_eq!(errors[0].index, 2);
/// }
///
/// // Works with any character iterator
/// let text = "hello(world]";
/// let filtered_chars = text.chars().filter(|&c| "()[]{}<>".contains(c));
/// let result = validate_iter(filtered_chars, &opts);
/// ```
///
/// # Performance
///
/// - **Time Complexity**: O(n) where n is the number of characters
/// - **Space Complexity**: O(d) where d is the maximum nesting depth
/// - **Iterator Efficiency**: Works with any character iterator, including lazy ones
///
/// # Use Cases
///
/// - Processing character streams
/// - Filtering input before validation
/// - Character-based error reporting
/// - Working with character-oriented data structures
pub fn validate_iter<I>(iter: I, opts: &Options) -> Result<(), Vec<BracketError>>
where
    I: IntoIterator<Item = char>,
{
    let mut pos = 0usize;
    let indexed = iter.into_iter().map(|ch| {
        let i = pos;
        pos += 1;
        (i, ch)
    });
    validate_indexed_iter(indexed, opts)
}

/// Pre-indexed streaming validation API.
///
/// This API is for cases where you already have character positions available,
/// such as when working with parsed data structures or when you need precise
/// control over position reporting.
///
/// # Arguments
///
/// * `iter` - An iterator yielding `(position, character)` pairs
/// * `opts` - Configuration options controlling validation behavior
///
/// # Returns
///
/// * `Ok(())` - If all brackets are properly matched and nested
/// * `Err(Vec<BracketError>)` - Errors with the provided position values
///
/// # Examples
///
/// ```rust
/// use brackets_extended::{validate_indexed, Options};
///
/// let opts = Options::default();
///
/// // Use string's built-in char_indices() for byte positions
/// let text = "hello(world]";
/// let result = validate_indexed(text.char_indices(), &opts);
/// if let Err(errors) = result {
///     // Error position is byte index from char_indices()
///     println!("Error at byte {}: {:?}", errors[0].index, errors[0].kind);
/// }
///
/// // Custom position mapping
/// let chars_with_line_col = vec![
///     ((1, 5), '('),  // Line 1, Column 5
///     ((1, 6), '['),  // Line 1, Column 6
///     ((1, 7), ')'),  // Line 1, Column 7 - error!
///     ((1, 8), ']'),  // Line 1, Column 8
/// ];
/// 
/// // This example won't compile as-is since validate_indexed expects (usize, char)
/// // but demonstrates the concept of custom position tracking
/// ```
///
/// # Performance
///
/// - **Time Complexity**: O(n) where n is the number of position/character pairs
/// - **Space Complexity**: O(d) where d is the maximum nesting depth
/// - **Zero Position Overhead**: Uses provided positions directly
///
/// # Use Cases
///
/// - Working with pre-parsed token streams
/// - Custom position tracking (line/column, custom offsets)
/// - Integration with existing parsing infrastructure
/// - Performance optimization when positions are already available
pub fn validate_indexed<I>(iter: I, opts: &Options) -> Result<(), Vec<BracketError>>
where
    I: IntoIterator<Item = (usize, char)>,
{
    validate_indexed_iter(iter, opts)
}
