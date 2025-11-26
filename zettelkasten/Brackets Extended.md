# 🔧 Brackets Extended

*Advanced configurable bracket validation system with custom alphabets, multi-error reporting, and flexible policies*

---

## 🎯 **Project Overview**

Brackets Extended is a comprehensive bracket validation library that extends basic bracket matching with three powerful advanced features. It represents the evolution from simple stack-based validation to a production-ready, configurable validation system.

### **Evolution from Basic to Extended**

```
Brackets Basic (REQ-1 to REQ-6)    →    Brackets Extended (REQ-7 to REQ-9)
├─ Fixed alphabet: ()[]{}                ├─ Configurable alphabet: any pairs
├─ Stop at first error                   ├─ Collect all errors option
├─ Simple error reporting                ├─ Detailed error contexts
└─ String-only validation                └─ Multiple iterator APIs
```

## 🚀 **Three Extended Requirements**

### **REQ-7: Configurable Alphabet**

*Support arbitrary opener→closer mappings beyond traditional brackets*

```rust
use brackets_extended::{Alphabet, Options, validate_with_options};

// Traditional brackets
let standard = Alphabet::default(); // ()[]{}

// HTML/XML validation
let html = Alphabet::with_pairs(&[('<', '>')]);

// Custom programming language
let custom = Alphabet::with_pairs(&[
    ('(', ')'),    // Function calls
    ('[', ']'),    // Array access
    ('{', '}'),    // Code blocks
    ('<', '>'),    // Generic types
    ('«', '»'),    // String literals
]);

let mut opts = Options::default();
opts.alphabet = custom;

// Now validates custom bracket pairs
assert!(validate_with_options("func<T>(arr[0])", &opts).is_ok());
```

**Use Cases:**

- **HTML/XML parsing**: `<div>content</div>`
- **Generic type validation**: `Vec<HashMap<String, i32>>`
- **Mathematical notation**: `⌊x⌋`, `⌈x⌉` (floor/ceiling)
- **Custom DSLs**: Domain-specific bracket pairs

### **REQ-8: Error Collection Mode**

*Choose between stop-at-first or collect-all error reporting*

```rust
use brackets_extended::{ErrorMode, Options, validate_with_options};

let input = ")(([)]"; // Multiple errors

// Stop at first error (default)
let mut stop_opts = Options::default();
stop_opts.error_mode = ErrorMode::StopAtFirst;
let result = validate_with_options(input, &stop_opts);
// Returns single error

// Collect all errors
let mut collect_opts = Options::default();
collect_opts.error_mode = ErrorMode::CollectAll;
let errors = validate_with_options(input, &collect_opts).unwrap_err();
// Returns Vec<BracketError> with all issues

for error in &errors {
    println!("Error at position {}: {}", error.position, error.message);
}
```

**Applications:**

- **IDE integration**: Show all syntax errors at once
- **Educational tools**: Highlight every mistake for learning
- **Linting systems**: Comprehensive error reporting
- **Batch processing**: Validate multiple expressions

### **REQ-9: Unclosed Policy**

*Control which unclosed bracket to report when multiple remain open*

```rust
use brackets_extended::{UnclosedPolicy, Options, validate_with_options};

let input = "(((["; // Multiple unclosed brackets

// Latest Open (LIFO) - reports innermost unclosed
let mut latest_opts = Options::default();
latest_opts.unclosed_policy = UnclosedPolicy::LatestOpen;
let error = validate_with_options(input, &latest_opts).unwrap_err();
// Reports '[' at position 3 (most recent)

// Earliest Open (FIFO) - reports outermost unclosed  
let mut earliest_opts = Options::default();
earliest_opts.unclosed_policy = UnclosedPolicy::EarliestOpen;
let error = validate_with_options(input, &earliest_opts).unwrap_err();
// Reports '(' at position 0 (first opened)
```

**Policy Comparison:**

| Input | LatestOpen Reports | EarliestOpen Reports | Use Case |
|-------|-------------------|----------------------|----------|
| `((([` | `[` at pos 3 | `(` at pos 0 | Code completion vs error recovery |
| `{[(` | `(` at pos 2 | `{` at pos 0 | IDE hints vs compiler errors |

## 🔄 **Iterator APIs**

### **Three Flexible Validation Interfaces**

```rust
use brackets_extended::{validate_brackets, validate_iter, validate_indexed, Options};

let opts = Options::default();

// 1. String API (convenience)
let result = validate_brackets("([{}])");

// 2. Character Iterator API
let chars = "([{}])".chars();
let result = validate_iter(chars, &opts);

// 3. Indexed Iterator API (preserves positions)
let text = "hello (world) goodbye";
let indexed_chars = text.char_indices()
    .filter(|(_, c)| "()[]{}".contains(*c)); // Extract only brackets
let result = validate_indexed(indexed_chars, &opts);
```

### **Iterator API Advantages**

- **Memory efficiency**: Stream processing for large inputs
- **Flexible preprocessing**: Filter, transform, or modify input
- **Position preservation**: Maintain original byte/character positions
- **Composability**: Chain with other iterator operations

```rust
// Advanced: Validate brackets in code comments only
let code = r#"
fn main() { // This has (brackets)
    println!("Hello [world]"); // More (brackets) here
    /* Block comment with {braces} */
}
"#;

let bracket_positions: Vec<(usize, char)> = code
    .char_indices()
    .filter(|(i, c)| {
        // Only check brackets inside comments
        let line = code[..*i].lines().last().unwrap_or("");
        (line.contains("//") || line.contains("/*")) && "()[]{}".contains(*c)
    })
    .collect();

let result = validate_indexed(bracket_positions, &opts);
```

## 🏗️ **Architecture and Design**

### **Core Components**

```rust
// Main validation engine
pub struct BracketValidator {
    stack: Vec<OpenBracket>,
    alphabet: Alphabet,
    error_mode: ErrorMode,
    unclosed_policy: UnclosedPolicy,
}

// Configuration
#[derive(Default)]
pub struct Options {
    pub alphabet: Alphabet,
    pub error_mode: ErrorMode,
    pub unclosed_policy: UnclosedPolicy,
}

// Flexible error collection
pub enum ValidationResult {
    Ok,
    SingleError(BracketError),
    MultipleErrors(Vec<BracketError>),
}
```

### **Key Design Patterns**

**1. Configuration Object Pattern**

```rust
let mut opts = Options::default();
opts.alphabet = Alphabet::with_pairs(&[('<', '>')]);
opts.error_mode = ErrorMode::CollectAll;
opts.unclosed_policy = UnclosedPolicy::EarliestOpen;
```

**2. Strategy Pattern for Error Handling**

```rust
enum ErrorMode {
    StopAtFirst,    // Early return strategy
    CollectAll,     // Accumulation strategy
}
```

**3. Iterator Trait Abstraction**

```rust
// Generic over any iterator that produces characters
fn validate_iter<I>(iter: I, opts: &Options) -> ValidationResult
where
    I: IntoIterator<Item = char>
```

## 🎯 **Mission Integration**

### **Mission1: Stack Implementation Foundation**

```rust
// Uses Mission1 Stack<T> internally
use crate::stack::Stack;

impl BracketValidator {
    fn validate_char(&mut self, ch: char, position: usize) -> Result<(), BracketError> {
        if let Some(closer) = self.alphabet.get_closer(ch) {
            // Opening bracket - push to stack
            self.stack.push(OpenBracket { ch, position, closer });
        } else if self.alphabet.is_closer(ch) {
            // Closing bracket - check stack
            match self.stack.pop() {
                Some(open) if open.closer == ch => { /* Match! */ }
                Some(open) => return Err(BracketError::Mismatch { 
                    expected: open.closer, 
                    found: ch, 
                    position 
                }),
                None => return Err(BracketError::UnexpectedClosing { ch, position }),
            }
        }
        Ok(())
    }
}
```

### **AoC Pattern Applications**

- **String parsing**: Advanced bracket validation in AoC problems
- **Error accumulation**: Collect all parsing errors for debug output
- **Custom grammars**: Validate problem-specific syntax rules
- **Position tracking**: Maintain accurate error locations in large inputs

## 🧪 **Testing Strategy**

### **Requirements-Based Testing**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test] // REQ-7
    fn req7_configurable_alphabet_with_angles() {
        let alphabet = Alphabet::with_pairs(&[('<', '>')]);
        let mut opts = Options::default();
        opts.alphabet = alphabet;
        
        assert!(validate_with_options("<>", &opts).is_ok());
        assert!(validate_with_options("([", &opts).is_err()); // Standard brackets not configured
    }

    #[test] // REQ-8  
    fn req8_collect_all_errors() {
        let mut opts = Options::default();
        opts.error_mode = ErrorMode::CollectAll;
        
        let errors = validate_with_options(")(([)]", &opts).unwrap_err();
        assert!(errors.len() >= 2); // Multiple errors collected
    }

    #[test] // REQ-9
    fn req9_unclosed_policy_latest_vs_earliest() {
        let input = "((([";
        
        let mut latest_opts = Options::default();
        latest_opts.unclosed_policy = UnclosedPolicy::LatestOpen;
        let latest_error = validate_with_options(input, &latest_opts).unwrap_err();
        
        let mut earliest_opts = Options::default();
        earliest_opts.unclosed_policy = UnclosedPolicy::EarliestOpen;
        let earliest_error = validate_with_options(input, &earliest_opts).unwrap_err();
        
        // Different positions reported based on policy
        assert_ne!(latest_error[0].position, earliest_error[0].position);
    }
}
```

### **Integration Testing with CSV Data**

```rust
// tests/brackets_extended_integration.rs
use std::fs;
use csv::Reader;

#[test]
fn test_with_aoc_dataset() {
    let data = fs::read_to_string("tests/data/brackets_extended.txt").unwrap();
    let expected = fs::read_to_string("tests/data/brackets_extended.expected.csv").unwrap();
    
    let mut reader = Reader::from_reader(expected.as_bytes());
    
    for (input, expected_result) in data.lines().zip(reader.records()) {
        let record = expected_result.unwrap();
        let should_pass: bool = record[1].parse().unwrap();
        
        let opts = Options::default();
        let result = validate_with_options(input, &opts);
        
        assert_eq!(result.is_ok(), should_pass, 
                   "Failed for input: {}", input);
    }
}
```

## 📊 **Performance Characteristics**

### **Time Complexity**

- **O(n)** where n is input length
- **Single pass** through input with stack operations
- **Constant time** bracket lookup with HashMap alphabet

### **Memory Usage**

```rust
// Stack grows with nesting depth, not input length
// Worst case: O(n) for input like "((((((("
// Best case: O(1) for balanced input like "()()()"

let deep_nesting = "(".repeat(1000) + &")".repeat(1000);  // O(1000) stack
let flat_sequence = "()".repeat(1000);                     // O(1) stack
```

### **Error Collection Overhead**

```rust
// StopAtFirst: Early return, minimal allocation
// CollectAll: Vec<BracketError> grows with error count

// Benchmark different error modes
fn benchmark_error_modes(input: &str) {
    // Fast: stops at first error
    let stop_result = validate_with_options(input, &stop_first_opts);
    
    // Slower: collects all errors (but more informative)
    let collect_result = validate_with_options(input, &collect_all_opts);
}
```

## 🔗 **Related Projects and Concepts**

### **Project Series**

- **[[../Brackets_Basic/README|Brackets Basic]]**: Foundation (REQ-1 to REQ-6)
- **[[mission-1]]**: Stack implementation used internally
- **[[AoC Collection Problems]]**: Bracket parsing in competitive programming

### **Core Concepts**

- **[[Error Handling Patterns]]**: Multi-error collection strategies
- **[[Iterator Design Patterns]]**: Flexible input processing
- **[[Configuration Design Patterns]]**: Options and builder patterns
- **[[Performance Optimization]]**: Memory and time complexity analysis

### **Advanced Topics**

- **[[Unicode and String Processing]]**: Proper UTF-8 handling
- **[[Parser Combinators]]**: Building more complex parsers
- **[[Compiler Design]]**: Lexical analysis and syntax validation

## 🎓 **Learning Outcomes**

### **Software Engineering Skills**

- **Requirements evolution**: Extending basic requirements to advanced features
- **API design**: Creating flexible, composable interfaces
- **Configuration patterns**: Options structs and builder patterns
- **Error handling**: Single vs multiple error collection strategies

### **Rust-Specific Learning**

- **Iterator trait system**: Working with generic iterator bounds
- **Error propagation**: `Result<T, E>` vs `Result<T, Vec<E>>`
- **Memory management**: Stack-based algorithms with owned data
- **Performance optimization**: Zero-allocation paths for common cases

### **Algorithm Design**

- **Stack-based parsing**: Classical application of LIFO data structure
- **Policy pattern**: Configurable behavior without code duplication
- **Stream processing**: Iterator-based validation for large inputs

## 🚀 **Real-World Applications**

### **IDE Integration**

```rust
// Language server integration
pub fn validate_code_brackets(source: &str, language: &Language) -> Vec<Diagnostic> {
    let alphabet = language.bracket_alphabet();
    let mut opts = Options::default();
    opts.alphabet = alphabet;
    opts.error_mode = ErrorMode::CollectAll;
    
    match validate_with_options(source, &opts) {
        Ok(_) => Vec::new(),
        Err(errors) => errors.into_iter().map(|e| Diagnostic {
            range: e.position..e.position + 1,
            severity: DiagnosticSeverity::Error,
            message: e.message,
        }).collect(),
    }
}
```

### **Educational Tools**

```rust
// Highlight all bracket errors for learning
pub fn create_learning_feedback(input: &str) -> LearningReport {
    let mut opts = Options::default();
    opts.error_mode = ErrorMode::CollectAll;
    
    match validate_with_options(input, &opts) {
        Ok(_) => LearningReport::success("Perfect bracket matching!"),
        Err(errors) => LearningReport::with_errors(
            errors.into_iter()
                .map(|e| format!("Position {}: {}", e.position, e.explanation()))
                .collect()
        ),
    }
}
```

---

*Tags: #brackets-extended #validation #stack #configuration #iterators #error-handling #aoc #advanced-examples #requirements #api-design #performance #testing #real-world-applications*

*Links: [[zettel-index]] | [[mission-1]] | [[AoC Patterns MOC]] | [[Collections MOC]] | [[Error Handling Patterns]] | [[Iterator Design Patterns]] | [[Performance Optimization]] | [[Testing Strategies]]*
