# 📚 **Rust Documentation Standards**

This document outlines the official Rust documentation standards and best practices.

## 🎯 **Core Principles**

1. **Write for your users** - Documentation should be from the user's perspective
2. **Provide working examples** - All examples should compile and run
3. **Be comprehensive** - Document all public APIs
4. **Use consistent formatting** - Follow standard conventions

## 📝 **Documentation Comment Types**

### **Item Documentation: `///`**

Used for documenting functions, structs, enums, modules, and other items:

```rust
/// Validates bracket sequences in a string.
/// 
/// This function checks if brackets are properly matched and nested.
/// Returns `Ok(())` if valid, or `Err(BracketError)` with details if invalid.
/// 
/// # Examples
/// 
/// ```rust
/// use brackets_basic::validate_brackets;
/// 
/// assert!(validate_brackets("()").is_ok());
/// assert!(validate_brackets("(]").is_err());
/// ```
pub fn validate_brackets(s: &str) -> Result<(), BracketError> {
    // implementation
}
```

### **Module Documentation: `//!`**

Used at the top of files to document entire modules or crates:

```rust
//! # Bracket Validation Library
//! 
//! This library provides functionality for validating bracket sequences
//! in text, ensuring proper matching and nesting.
//! 
//! ## Quick Start
//! 
//! ```rust
//! use brackets_basic::validate_brackets;
//! 
//! let result = validate_brackets("([{}])");
//! assert!(result.is_ok());
//! ```
```

## 📋 **Required Documentation Sections**

### **1. Summary Line**
- **First line** should be a brief, clear summary
- **No markdown** in the first line
- **Complete sentence** ending with period

```rust
/// Validates that brackets are properly matched and nested.
```

### **2. Detailed Description** 
- **Explain what it does** and why someone would use it
- **Include context** and background information
- **Mention important behaviors** or constraints

```rust
/// Validates that brackets are properly matched and nested.
/// 
/// This function implements a stack-based algorithm to ensure brackets
/// follow proper nesting rules. All non-bracket characters are ignored.
/// The algorithm runs in O(n) time and O(n) space complexity.
```

### **3. Parameters Section: `# Arguments`**

```rust
/// # Arguments
/// 
/// * `s` - The input string to validate
/// * `ignore_case` - Whether to treat brackets case-insensitively
```

### **4. Return Values: `# Returns`**

```rust
/// # Returns
/// 
/// * `Ok(())` - If all brackets are properly matched
/// * `Err(BracketError)` - If validation fails, with error details
```

### **5. Examples Section: `# Examples`**

**Most important section!** Should include:
- **Basic usage examples**
- **Common use cases** 
- **Error handling examples**
- **Edge cases**

```rust
/// # Examples
/// 
/// ## Valid bracket sequences
/// 
/// ```rust
/// use brackets_basic::validate_brackets;
/// 
/// assert!(validate_brackets("()").is_ok());
/// assert!(validate_brackets("([{}])").is_ok());
/// assert!(validate_brackets("hello(world)").is_ok());
/// ```
/// 
/// ## Invalid sequences  
/// 
/// ```rust
/// use brackets_basic::validate_brackets;
/// 
/// assert!(validate_brackets("(]").is_err());
/// assert!(validate_brackets("((").is_err());
/// ```
```

### **6. Panics Section: `# Panics`**

Document when/why a function might panic:

```rust
/// # Panics
/// 
/// Panics if the input string contains invalid UTF-8 sequences.
```

### **7. Errors Section: `# Errors`**

For functions returning `Result`:

```rust
/// # Errors
/// 
/// Returns `BracketError` in the following cases:
/// 
/// * `UnexpectedClosing` - Found closing bracket without opener
/// * `MismatchedPair` - Wrong type of closing bracket
/// * `UnclosedOpenings` - Opening brackets never closed
```

### **8. Safety Section: `# Safety`**

For `unsafe` functions:

```rust
/// # Safety
/// 
/// The caller must ensure that the pointer is valid and points to
/// a properly initialized value.
```

## 🏗️ **Struct and Enum Documentation**

### **Structs**

```rust
/// Represents a bracket validation error.
/// 
/// Contains the position where the error occurred and the specific
/// type of error encountered during validation.
/// 
/// # Fields
/// 
/// * `index` - Zero-based position where error was detected
/// * `kind` - Specific type of bracket error
/// 
/// # Examples
/// 
/// ```rust
/// use brackets_basic::{validate_brackets, BracketErrorKind};
/// 
/// let result = validate_brackets("(]");
/// if let Err(error) = result {
///     println!("Error at position {}: {:?}", error.index, error.kind);
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BracketError {
    /// Zero-based index where the error was detected
    pub index: usize,
    /// Specific type of bracket validation error  
    pub kind: BracketErrorKind,
}
```

### **Enums**

```rust
/// The different types of bracket validation errors.
/// 
/// This enum categorizes the three fundamental bracket validation
/// failures that can occur during processing.
/// 
/// # Variants
/// 
/// * [`UnexpectedClosing`] - Closing bracket without matching opener
/// * [`MismatchedPair`] - Wrong type of closing bracket
/// * [`UnclosedOpenings`] - Opening bracket never closed
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BracketErrorKind {
    /// Found a closing bracket without a corresponding opener.
    UnexpectedClosing { 
        /// The unexpected closing bracket character
        found: char 
    },
    // ... other variants
}
```

## 🧪 **Testing Documentation**

### **Doctests**
Examples in doc comments are automatically tested:

```rust
/// Adds two numbers together.
/// 
/// # Examples
/// 
/// ```rust
/// use mylib::add;
/// 
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### **Hidden Lines in Doctests**
Use `# ` to hide setup code:

```rust
/// # Examples
/// 
/// ```rust
/// # use mylib::MyStruct;
/// # let mut instance = MyStruct::new();
/// instance.do_something();
/// ```
```

## 🛠️ **Documentation Tools**

### **Generate Documentation**
```bash
cargo doc --open
```

### **Test Documentation Examples**
```bash
cargo test --doc
```

### **Check Documentation Coverage**
```bash
cargo +nightly doc --document-private-items
```

## ✅ **Best Practices**

1. **Write examples first** - This helps clarify the API design
2. **Test your examples** - Run `cargo test --doc` regularly  
3. **Be specific in error documentation** - List all possible error conditions
4. **Use links** - Link to related types and functions with `[Type]` or `[function]`
5. **Keep it updated** - Documentation should match the current implementation
6. **Think about your users** - What do they need to know to use this effectively?

## 🎯 **Documentation Checklist**

For every public item, ensure you have:

- [ ] Clear, concise summary line
- [ ] Detailed description of behavior  
- [ ] Parameter documentation (if applicable)
- [ ] Return value documentation (if applicable)
- [ ] At least one working example
- [ ] Error conditions documented (for Result types)
- [ ] Panic conditions documented (if applicable)
- [ ] Links to related functionality

## 🌟 **Real-World Example**

Here's how the current `validate_brackets` function could be fully documented:

```rust
/// Validates that brackets in a string are properly matched and nested.
/// 
/// This function implements a stack-based algorithm to verify that bracket
/// sequences follow proper nesting rules. Only `()`, `[]`, and `{}` are
/// considered brackets; all other characters are ignored.
/// 
/// # Arguments
/// 
/// * `s` - The input string to validate
/// 
/// # Returns
/// 
/// * `Ok(())` - If all brackets are properly matched and nested
/// * `Err(BracketError)` - If validation fails, containing error details
/// 
/// # Errors
/// 
/// Returns `BracketError` in these cases:
/// 
/// * `UnexpectedClosing` - Found `)`, `]`, or `}` without matching opener
/// * `MismatchedPair` - Wrong closing bracket type (e.g., `(` closed by `]`)  
/// * `UnclosedOpenings` - Input ended with unmatched opening brackets
/// 
/// # Time Complexity
/// 
/// O(n) where n is the length of the input string.
/// 
/// # Examples
/// 
/// ```rust
/// use brackets_basic::validate_brackets;
/// 
/// // Valid sequences
/// assert!(validate_brackets("").is_ok());
/// assert!(validate_brackets("()[]{}").is_ok());
/// assert!(validate_brackets("([{}])").is_ok());
/// assert!(validate_brackets("hello(world[test])").is_ok());
/// 
/// // Invalid sequences
/// assert!(validate_brackets(")").is_err());      // Unexpected closing
/// assert!(validate_brackets("(]").is_err());     // Mismatched pair  
/// assert!(validate_brackets("(((").is_err());    // Unclosed openings
/// ```
pub fn validate_brackets(s: &str) -> Result<(), BracketError> {
    // implementation...
}
```

This follows all Rust documentation standards and provides comprehensive information for users! 🎉