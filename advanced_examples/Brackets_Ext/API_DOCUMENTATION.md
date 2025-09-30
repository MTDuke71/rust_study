# 📚 Brackets Extended - Comprehensive API Documentation

## Overview

The `brackets_extended` crate provides advanced bracket validation with configurable alphabets, flexible error reporting, and multiple validation strategies. This documentation covers all APIs, use cases, and best practices.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Core Concepts](#core-concepts)
3. [API Reference](#api-reference)
4. [Configuration Options](#configuration-options)
5. [Error Handling](#error-handling)
6. [Performance Guide](#performance-guide)
7. [Real-World Examples](#real-world-examples)
8. [Migration Guide](#migration-guide)

---

## Quick Start

### Basic Usage

```rust
use brackets_extended::validate_brackets;

// Simple validation (traditional API)
assert!(validate_brackets("([{}])").is_ok());
assert!(validate_brackets("([)]").is_err());
```

### Advanced Usage

```rust
use brackets_extended::{Options, Alphabet, ErrorMode, validate_with_options};

// Configure custom options
let mut opts = Options::default();
opts.alphabet = Alphabet::with_pairs(&[('(', ')'), ('<', '>')]);
opts.error_mode = ErrorMode::CollectAll;

// Validate with custom configuration
let result = validate_with_options("(<invalid>", &opts);
```

---

## Core Concepts

### 1. Alphabets

An **Alphabet** defines which characters are considered brackets and their pairs.

```rust
use brackets_extended::Alphabet;

// Default: (), [], {}
let default = Alphabet::default_ascii();

// Custom: HTML-style 
let html = Alphabet::with_pairs(&[('<', '>')]);

// Mathematical notation
let math = Alphabet::with_pairs(&[
    ('⟨', '⟩'),  // Angle brackets
    ('⌊', '⌋'),  // Floor function
    ('⌈', '⌉'),  // Ceiling function
]);
```

### 2. Error Modes

Control how errors are reported during validation.

```rust
use brackets_extended::ErrorMode;

// Stop at first error (default, faster)
ErrorMode::StopAtFirst

// Collect all errors (comprehensive reporting)
ErrorMode::CollectAll
```

### 3. Unclosed Policies

When multiple brackets are unclosed, choose which one to report.

```rust
use brackets_extended::UnclosedPolicy;

// Report the most recently opened bracket (LIFO/stack behavior)
UnclosedPolicy::LatestOpen  // Default

// Report the earliest opened bracket (FIFO/queue behavior)  
UnclosedPolicy::EarliestOpen
```

---

## API Reference

### Validation Functions

#### `validate_brackets(s: &str) -> Result<(), BracketError>`

**Traditional single-error API for backward compatibility.**

```rust
use brackets_extended::{validate_brackets, BracketErrorKind};

// Basic validation
assert!(validate_brackets("()").is_ok());

// Error handling
if let Err(error) = validate_brackets("(]") {
    println!("Error at position {}: {:?}", error.index, error.kind);
}
```

**Use Cases:**
- Simple pass/fail validation
- Legacy code integration
- Performance-critical applications
- Quick validation checks

---

#### `validate_with_options(s: &str, opts: &Options) -> Result<(), Vec<BracketError>>`

**Primary string validation API with full configuration support.**

```rust
use brackets_extended::{validate_with_options, Options, ErrorMode};

let mut opts = Options::default();
opts.error_mode = ErrorMode::CollectAll;

let errors = validate_with_options(")](", &opts).unwrap_err();
println!("Found {} errors", errors.len());
```

**Use Cases:**
- IDE integration
- Code linters and formatters
- Educational tools
- Configuration file validation

---

#### `validate_iter<I>(iter: I, opts: &Options) -> Result<(), Vec<BracketError>>`

**Character-based streaming API with character position reporting.**

```rust
use brackets_extended::{validate_iter, Options};

let opts = Options::default();

// From string characters
let result = validate_iter("([)]".chars(), &opts);

// From filtered characters
let text = "hello(world]end";
let brackets_only = text.chars().filter(|&c| "()[]{}<>".contains(c));
let result = validate_iter(brackets_only, &opts);
```

**Use Cases:**
- Character stream processing
- Filtering input before validation
- Character-based error reporting
- Working with character iterators

---

#### `validate_indexed<I>(iter: I, opts: &Options) -> Result<(), Vec<BracketError>>`

**Pre-indexed streaming API for custom position tracking.**

```rust
use brackets_extended::{validate_indexed, Options};

let opts = Options::default();

// Use string's byte indices
let text = "hello(world]";
let result = validate_indexed(text.char_indices(), &opts);

// Custom position mapping (conceptual example)
let custom_positions = vec![
    (10, '('),  // Custom position 10
    (20, '['),  // Custom position 20
    (30, ')'),  // Custom position 30
    (40, ']'),  // Custom position 40
];
let result = validate_indexed(custom_positions, &opts);
```

**Use Cases:**
- Pre-parsed token streams
- Custom position tracking (line/column)
- Integration with existing parsers
- Performance optimization

---

## Configuration Options

### Options Structure

```rust
use brackets_extended::{Options, Alphabet, ErrorMode, UnclosedPolicy};

let opts = Options {
    alphabet: Alphabet::default_ascii(),
    error_mode: ErrorMode::StopAtFirst,
    unclosed_policy: UnclosedPolicy::LatestOpen,
};
```

### Alphabet Configurations

#### Standard Programming Languages

```rust
// Default ASCII (most programming languages)
let code = Alphabet::default_ascii(); // (), [], {}

// Extended programming (some languages use angle brackets)
let extended = Alphabet::with_pairs(&[
    ('(', ')'),
    ('[', ']'),
    ('{', '}'),
    ('<', '>'),
]);
```

#### Markup Languages

```rust
// HTML/XML tags
let html = Alphabet::with_pairs(&[('<', '>')]);

// Markdown-style  
let markdown = Alphabet::with_pairs(&[
    ('(', ')'),   // Links
    ('[', ']'),   // Link text
]);
```

#### Mathematical Notation

```rust
let math = Alphabet::with_pairs(&[
    ('(', ')'),   // Standard grouping
    ('[', ']'),   // Matrix notation
    ('⟨', '⟩'),   // Inner products
    ('⌊', '⌋'),   // Floor function
    ('⌈', '⌉'),   // Ceiling function
    ('|', '|'),   // Absolute value (if treating as brackets)
]);
```

#### Custom Applications

```rust
// Configuration files with custom delimiters
let config = Alphabet::with_pairs(&[
    ('{', '}'),   // Objects
    ('[', ']'),   // Arrays
    ('⟨', '⟩'),   // Special sections
]);

// Domain-specific languages
let dsl = Alphabet::with_pairs(&[
    ('❮', '❯'),   // Custom delimiters
    ('⟪', '⟫'),   // Multi-character concepts
]);
```

### Error Mode Strategies

#### StopAtFirst (Default)

```rust
let mut opts = Options::default();
opts.error_mode = ErrorMode::StopAtFirst;

// Advantages:
// - Faster performance
// - Lower memory usage
// - Traditional parser behavior
// - Quick fail-fast validation

// Use when:
// - Performance is critical
// - Only need to know if input is valid/invalid
// - Processing large amounts of data
// - Building high-throughput systems
```

#### CollectAll

```rust
let mut opts = Options::default();
opts.error_mode = ErrorMode::CollectAll;

// Advantages:
// - Comprehensive error reporting
// - Better user experience
// - Educational value
// - Batch error handling

// Use when:
// - Building IDEs or editors
// - Educational tools
// - Code linting
// - User-facing applications
```

### Unclosed Policy Strategies

#### LatestOpen (Default - LIFO)

```rust
let mut opts = Options::default();
opts.unclosed_policy = UnclosedPolicy::LatestOpen;

// Reports the most recently opened bracket
// Example: "((([" reports position of '[' (position 3)

// Advantages:
// - Natural stack-based thinking
// - "What was I just working on?"
// - Matches most parser implementations
// - Good for incremental editing

// Use when:
// - Building code editors
// - Interactive applications
// - Users think in terms of recent context
```

#### EarliestOpen (FIFO)

```rust
let mut opts = Options::default();
opts.unclosed_policy = UnclosedPolicy::EarliestOpen;

// Reports the earliest opened bracket
// Example: "((([" reports position of first '(' (position 0)

// Advantages:
// - Structural analysis perspective
// - "Where did the problem start?"
// - Good for understanding overall structure
// - Helps identify root cause

// Use when:
// - Static analysis tools
// - Debugging complex nesting
// - Educational tools teaching structure
// - Batch processing with structural focus
```

---

## Error Handling

### Error Types

```rust
use brackets_extended::{BracketError, BracketErrorKind};

// All errors include position information
pub struct BracketError {
    pub index: usize,           // Position where error occurred
    pub kind: BracketErrorKind, // Specific error type
}
```

### Error Kinds

#### UnexpectedClosing

```rust
// Occurs when a closing bracket has no matching opener
let result = validate_brackets(")");
if let Err(error) = result {
    match error.kind {
        BracketErrorKind::UnexpectedClosing { found } => {
            println!("Unexpected '{}' at position {}", found, error.index);
        }
        _ => {}
    }
}
```

#### MismatchedPair

```rust
// Occurs when brackets are interleaved incorrectly
let result = validate_brackets("([)]");
if let Err(error) = result {
    match error.kind {
        BracketErrorKind::MismatchedPair { expected, found } => {
            println!("Expected '{}' but found '{}' at position {}", 
                    expected, found, error.index);
        }
        _ => {}
    }
}
```

#### UnclosedOpenings

```rust
// Occurs when brackets are left unclosed at end of input
let result = validate_brackets("(((");
if let Err(error) = result {
    match error.kind {
        BracketErrorKind::UnclosedOpenings { expected, open_index } => {
            println!("Expected '{}' to close bracket opened at position {}", 
                    expected, open_index);
        }
        _ => {}
    }
}
```

### Error Recovery Strategies

#### Single Error Processing

```rust
use brackets_extended::validate_brackets;

match validate_brackets(input) {
    Ok(()) => {
        // Process valid input
        println!("Input is valid");
    }
    Err(error) => {
        // Handle single error
        report_error(&error);
        // Potentially try to fix and re-validate
    }
}
```

#### Multiple Error Processing

```rust
use brackets_extended::{validate_with_options, Options, ErrorMode};

let mut opts = Options::default();
opts.error_mode = ErrorMode::CollectAll;

match validate_with_options(input, &opts) {
    Ok(()) => {
        println!("Input is valid");
    }
    Err(errors) => {
        // Process all errors
        for (i, error) in errors.iter().enumerate() {
            println!("Error {}: {:?} at position {}", 
                    i + 1, error.kind, error.index);
        }
        
        // Categorize errors
        let unexpected_count = errors.iter()
            .filter(|e| matches!(e.kind, BracketErrorKind::UnexpectedClosing { .. }))
            .count();
        let mismatched_count = errors.iter()
            .filter(|e| matches!(e.kind, BracketErrorKind::MismatchedPair { .. }))
            .count();
        let unclosed_count = errors.iter()
            .filter(|e| matches!(e.kind, BracketErrorKind::UnclosedOpenings { .. }))
            .count();
            
        println!("Summary: {} unexpected, {} mismatched, {} unclosed",
                unexpected_count, mismatched_count, unclosed_count);
    }
}
```

---

## Performance Guide

### Time Complexity

All validation functions are **O(n)** where n is the number of characters processed.

### Space Complexity

**O(d)** where d is the maximum nesting depth of brackets.

### Memory Usage

- **Stack allocation**: Uses custom stack for tracking bracket pairs
- **Minimal heap allocation**: Only for error collection in `CollectAll` mode
- **Iterator efficiency**: Zero-copy processing with iterators

### Performance Tips

#### Choose the Right API

```rust
// Fastest: Single error, early termination
validate_brackets(input)

// Fast: Single error with custom options
validate_with_options(input, &opts_with_stop_first)

// Slower: Multiple error collection
let mut opts = Options::default();
opts.error_mode = ErrorMode::CollectAll;
validate_with_options(input, &opts)
```

#### Optimize for Your Use Case

```rust
// High-throughput validation (fastest)
let opts = Options {
    alphabet: Alphabet::default_ascii(),
    error_mode: ErrorMode::StopAtFirst,
    unclosed_policy: UnclosedPolicy::LatestOpen, // Doesn't matter for StopAtFirst
};

// Comprehensive error reporting (slower but more informative)
let opts = Options {
    alphabet: Alphabet::default_ascii(),
    error_mode: ErrorMode::CollectAll,
    unclosed_policy: UnclosedPolicy::LatestOpen,
};
```

#### Batch Processing

```rust
// Process multiple inputs efficiently
let opts = Options::default();

for input in large_input_set {
    match validate_with_options(input, &opts) {
        Ok(()) => process_valid_input(input),
        Err(errors) => handle_errors(input, errors),
    }
}
```

---

## Real-World Examples

### IDE Integration

```rust
use brackets_extended::{Options, ErrorMode, validate_with_options};

fn validate_code_for_ide(code: &str) -> Vec<BracketError> {
    let mut opts = Options::default();
    opts.error_mode = ErrorMode::CollectAll; // Show all errors to user
    
    match validate_with_options(code, &opts) {
        Ok(()) => Vec::new(),
        Err(errors) => errors,
    }
}

// Usage in IDE
let errors = validate_code_for_ide(user_code);
for error in errors {
    display_error_in_editor(error.index, &error.kind);
}
```

### Configuration File Validation

```rust
use brackets_extended::{Options, Alphabet, validate_with_options};

fn validate_json_like_config(config: &str) -> Result<(), String> {
    let opts = Options {
        alphabet: Alphabet::with_pairs(&[
            ('{', '}'), // Objects
            ('[', ']'), // Arrays
        ]),
        error_mode: ErrorMode::StopAtFirst, // Fast validation
        unclosed_policy: UnclosedPolicy::EarliestOpen,
    };
    
    match validate_with_options(config, &opts) {
        Ok(()) => Ok(()),
        Err(errors) => {
            let error = &errors[0];
            Err(format!("Configuration error at position {}: {:?}", 
                       error.index, error.kind))
        }
    }
}
```

### Mathematical Expression Validator

```rust
use brackets_extended::{Options, Alphabet, validate_with_options};

fn validate_math_expression(expr: &str) -> bool {
    let opts = Options {
        alphabet: Alphabet::with_pairs(&[
            ('(', ')'),   // Standard grouping
            ('[', ']'),   // Matrix notation
            ('⟨', '⟩'),   // Inner products
            ('⌊', '⌋'),   // Floor function
            ('⌈', '⌉'),   // Ceiling function
        ]),
        error_mode: ErrorMode::StopAtFirst,
        unclosed_policy: UnclosedPolicy::LatestOpen,
    };
    
    validate_with_options(expr, &opts).is_ok()
}

// Usage
assert!(validate_math_expression("sin(⌊x⌋ + ⟨v, w⟩)"));
assert!(!validate_math_expression("sin(⌊x⌉ + ⟨v, w⟩)")); // Mismatched floor/ceiling
```

### HTML/XML Tag Validation

```rust
use brackets_extended::{Options, Alphabet, ErrorMode, validate_with_options};

fn validate_html_brackets(html: &str) -> Vec<BracketError> {
    let opts = Options {
        alphabet: Alphabet::with_pairs(&[('<', '>')]),
        error_mode: ErrorMode::CollectAll, // Show all tag errors
        unclosed_policy: UnclosedPolicy::EarliestOpen, // Show first unclosed tag
    };
    
    match validate_with_options(html, &opts) {
        Ok(()) => Vec::new(),
        Err(errors) => errors,
    }
}

// Usage
let html = "<div><p>Content</p><span>More content</div>"; // Missing </span>
let errors = validate_html_brackets(html);
// Will report the unclosed <span> tag
```

### Educational Tool

```rust
use brackets_extended::{Options, ErrorMode, validate_with_options, BracketErrorKind};

fn explain_bracket_errors(code: &str) -> String {
    let mut opts = Options::default();
    opts.error_mode = ErrorMode::CollectAll;
    
    match validate_with_options(code, &opts) {
        Ok(()) => "✅ All brackets are properly matched!".to_string(),
        Err(errors) => {
            let mut explanation = format!("❌ Found {} bracket errors:\n\n", errors.len());
            
            for (i, error) in errors.iter().enumerate() {
                explanation.push_str(&format!("{}. At position {}: ", i + 1, error.index));
                
                match &error.kind {
                    BracketErrorKind::UnexpectedClosing { found } => {
                        explanation.push_str(&format!(
                            "Unexpected closing bracket '{}'. No matching opening bracket found.\n",
                            found
                        ));
                    }
                    BracketErrorKind::MismatchedPair { expected, found } => {
                        explanation.push_str(&format!(
                            "Mismatched brackets! Expected '{}' but found '{}'. Check your bracket pairs.\n",
                            expected, found
                        ));
                    }
                    BracketErrorKind::UnclosedOpenings { expected, open_index } => {
                        explanation.push_str(&format!(
                            "Unclosed bracket! Need '{}' to close the bracket opened at position {}.\n",
                            expected, open_index
                        ));
                    }
                }
            }
            
            explanation
        }
    }
}

// Usage in educational context
let student_code = "function test() { if (true { return false; }";
println!("{}", explain_bracket_errors(student_code));
```

---

## Migration Guide

### From Simple Validation Libraries

If you're coming from a simple bracket validation library:

```rust
// Old way (hypothetical simple library)
// if is_balanced(input) { ... }

// New way
use brackets_extended::validate_brackets;

if validate_brackets(input).is_ok() {
    // Input is valid
}
```

### From Custom Implementations

If you have a custom bracket validator:

```rust
// Old custom implementation
fn my_bracket_validator(input: &str) -> bool {
    // Custom logic...
}

// Migration to brackets_extended
use brackets_extended::{validate_brackets, Options, Alphabet};

// For standard brackets
fn migrated_validator(input: &str) -> bool {
    validate_brackets(input).is_ok()
}

// For custom brackets (if your old implementation supported them)
fn custom_migrated_validator(input: &str) -> bool {
    let opts = Options {
        alphabet: Alphabet::with_pairs(&[/* your custom pairs */]),
        ..Default::default()
    };
    brackets_extended::validate_with_options(input, &opts).is_ok()
}
```

### Adding Error Reporting

Enhance existing validation with detailed error reporting:

```rust
// Before: Just pass/fail
if validate_brackets(input).is_ok() {
    process_input(input);
} else {
    println!("Invalid brackets");
}

// After: Detailed error reporting
use brackets_extended::{validate_with_options, Options, ErrorMode};

let mut opts = Options::default();
opts.error_mode = ErrorMode::CollectAll;

match validate_with_options(input, &opts) {
    Ok(()) => process_input(input),
    Err(errors) => {
        for error in errors {
            println!("Error at position {}: {:?}", error.index, error.kind);
        }
    }
}
```

---

## Summary

The `brackets_extended` crate provides a comprehensive, flexible bracket validation system suitable for a wide range of applications from simple validation to complex IDE integration. Key benefits:

- **🔤 Configurable**: Support any bracket pairs
- **📝 Comprehensive**: Detailed error reporting with positions
- **🚀 Fast**: O(n) performance with minimal memory usage
- **🌐 Unicode**: Full UTF-8 support
- **🎯 Flexible**: Multiple APIs for different use cases
- **📚 Well-documented**: Extensive examples and guidance

Choose the API that best fits your use case, configure options for your specific needs, and leverage the detailed error information to provide excellent user experiences in your applications.