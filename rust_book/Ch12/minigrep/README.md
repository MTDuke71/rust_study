# minigrep - Complete Command-Line Text Search Tool

A complete, production-quality implementation of a text search tool demonstrating all Chapter 12 concepts from the Rust Book.

## Overview

`minigrep` is a command-line utility that searches text files for a query string, displaying matching lines with line numbers. It demonstrates:

- ✅ Command-line argument parsing
- ✅ File I/O operations
- ✅ Error handling patterns
- ✅ Project organization (lib.rs + main.rs)
- ✅ Test-driven development
- ✅ Environment variables
- ✅ Standard error handling (stderr vs stdout)

---

## 🏗️ Project Structure

```
minigrep/
├── Cargo.toml           # Project manifest
├── poem.txt            # Sample data file
└── src/
    ├── lib.rs          # Core functionality and tests
    └── main.rs         # CLI entry point and error handling
```

---

## 🚀 Building and Running

### Build the project
```bash
cargo build
```

### Run with cargo
```bash
cargo run -- "query" poem.txt
```

### Run the binary directly
```bash
./target/debug/minigrep "query" poem.txt
```

### Run tests
```bash
cargo test
cargo test -- --nocapture     # Show output
cargo test -- --test-threads=1 # Run sequentially
```

---

## 📋 Usage Examples

### Basic search (case-sensitive)
```bash
cargo run -- "Rust" poem.txt
```

Output:
```
Found 5 match(es) for 'Rust':

  13 | Rust is a systems programming language.
  14 | Rust emphasizes safety and performance.
  16 | The Rust community is welcoming and supportive.
  17 | Learning Rust takes time but is rewarding.
```

### Case-insensitive search
```bash
CASE_INSENSITIVE=1 cargo run -- "rust" poem.txt
```

### Search with error redirection
```bash
cargo run -- "nobody" poem.txt 2> errors.txt
```

### Search non-existent file
```bash
cargo run -- "query" nonexistent.txt
```

Error output to stderr:
```
Application error: No such file or directory (os error 2)
```

---

## 🔑 Key Components

### `lib.rs` - Core Library

#### `Config` Struct
```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

Represents the application configuration built from command-line arguments and environment variables.

#### `Config::new()` Method
```rust
pub fn new(mut args: std::env::Args) -> Result<Config, &'static str>
```

Creates a Config from command-line arguments with proper error handling. Returns:
- `Ok(Config)` - Valid configuration
- `Err(&str)` - Error message if arguments are invalid

#### `run()` Function
```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>>
```

Main application logic that:
1. Reads the file
2. Chooses search strategy (case-sensitive or not)
3. Displays results or "no matches" message
4. Handles errors with `?` operator

#### `search()` Function
```rust
pub fn search(query: &str, contents: &str) -> Vec<SearchResult>
```

Case-sensitive search. Returns vector of (line_number, line_content) tuples.

#### `search_case_insensitive()` Function
```rust
pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<SearchResult>
```

Case-insensitive search using `.to_lowercase()`.

### `main.rs` - CLI Entry Point

Demonstrates proper CLI patterns:

```rust
fn main() {
    // Parse arguments with error handling
    let config = minigrep::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);  // ✅ Error to stderr
        process::exit(1);                                  // ✅ Exit code 1
    });

    // Run application with error handling
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);            // ✅ Error to stderr
        process::exit(1);                                  // ✅ Exit code 1
    }
}
```

**Key Patterns:**
- `unwrap_or_else()` - Handle Config errors gracefully
- `eprintln!()` - Send errors to stderr, not stdout
- `process::exit()` - Exit with proper status code
- `if let Err(e) = ...` - Handle runtime errors

---

## 🧪 Comprehensive Test Suite

The project includes 25+ tests covering:

### Case-Sensitive Search Tests
- ✅ Single match
- ✅ Multiple matches
- ✅ No matches
- ✅ Line numbers accuracy

### Case-Insensitive Search Tests
- ✅ Mixed case queries
- ✅ All lowercase queries
- ✅ All uppercase queries
- ✅ Complex case variations

### Edge Cases
- ✅ Empty query string
- ✅ Empty file contents
- ✅ Multi-word queries
- ✅ Special characters

### Configuration Tests
- ✅ Valid configuration
- ✅ Missing query error
- ✅ Missing filename error

### Run all tests
```bash
cargo test
```

### Run specific test
```bash
cargo test test_search_single_match
```

### Run tests with output
```bash
cargo test -- --nocapture
```

---

## 🎓 Learning Outcomes

By studying this project, you'll understand:

### 1. **Project Organization**
- Separate concerns: business logic (lib.rs) vs CLI (main.rs)
- Clear module boundaries
- Testable architecture

### 2. **Error Handling**
- `Result<T, E>` types
- `unwrap_or_else()` for custom error handling
- `?` operator for error propagation
- Distinguishing stdout vs stderr

### 3. **Command-Line Development**
- Parsing arguments with `env::args()`
- Environment variables with `env::var()`
- Exit codes and `process::exit()`
- Proper error messages

### 4. **Testing Strategy**
- Unit tests in lib.rs
- Test organization with test modules
- Edge case coverage
- Assertion patterns

### 5. **Rust Idioms**
- String slices vs String ownership
- Lifetime annotations
- Iterator patterns
- Pattern matching with `if let`

---

## 📊 Test Coverage

```
Tests by Category:
├── Case-Sensitive Search (4 tests)
├── Case-Insensitive Search (4 tests)
├── Edge Cases (4 tests)
└── Configuration (3 tests)

Total: 15+ comprehensive tests
```

Run with coverage (if installed):
```bash
cargo tarpaulin --out Html
```

---

## 🔍 Code Walkthrough

### Example: How search() Works

```rust
pub fn search(query: &str, contents: &str) -> Vec<SearchResult> {
    let mut results = Vec::new();

    // Iterate through lines with line numbers
    for (line_num, line) in contents.lines().enumerate() {
        // Check if line contains query
        if line.contains(query) {
            // Store 1-indexed line number and line content
            results.push((line_num + 1, line.to_string()));
        }
    }

    results
}
```

**Why this works:**
- `enumerate()` gives us (index, value) pairs
- `line_num` starts at 0, so we add 1 for display
- `line.to_string()` converts &str to owned String
- Collect all matches before returning

### Example: How Config::new() Works

```rust
pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next(); // Skip "minigrep"
    
    let query = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a query string"),
    };
    
    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file name"),
    };
    
    let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();
    
    Ok(Config { query, filename, case_sensitive })
}
```

**Error Handling Pattern:**
- Return `Err` early if argument missing
- Use `match` for `Option` handling
- Check environment variable for configuration
- Return `Ok(Config)` on success

---

## 🐛 Common Issues and Solutions

### Issue: File not found
```
Application error: No such file or directory (os error 2)
```

**Solution:** Specify full path or check file exists
```bash
cargo run -- "query" ./poem.txt
```

### Issue: No output (found 0 matches)
```
No matches found for 'xyz'
```

**Solution:** Verify query string:
- Check spelling
- Try case-insensitive search: `CASE_INSENSITIVE=1 cargo run -- "query" file.txt`

### Issue: Tests fail
```bash
cargo test -- --nocapture  # Show test output
```

Check that poem.txt exists and has expected content.

---

## 🔗 Chapter 12 Concepts Demonstrated

| Concept | Location | Example |
|---------|----------|---------|
| Command-line arguments | main.rs, lib.rs | `Config::new()` |
| File I/O | lib.rs `run()` | `fs::read_to_string()` |
| Error handling | main.rs, lib.rs | `Result<T, E>`, `?` operator |
| Modularity | src/lib.rs + src/main.rs | Separated concerns |
| Testing | lib.rs | 15+ test cases |
| Environment variables | lib.rs | `env::var("CASE_INSENSITIVE")` |
| Standard error | main.rs | `eprintln!()` |

---

## 📖 Further Exploration

### Extend minigrep with:
1. **Output formatting** - Add `--color` flag for colored output
2. **Line count** - Show total matching lines
3. **Regex support** - Use `regex` crate for pattern matching
4. **Recursive search** - Search in directories with `--recursive`
5. **Performance** - Use `rayon` for parallel search

### Related exercises:
- Modify to search multiple files
- Add `--ignore-case` command-line flag (instead of env var)
- Implement word-boundary matching
- Add context lines (lines before/after matches)

---

## ✅ Checklist for Understanding

- [ ] Understand how Config parsing works
- [ ] Trace through a search operation step-by-step
- [ ] Modify search to use regex
- [ ] Add a new command-line flag
- [ ] Run all tests and understand each one
- [ ] Explain error handling to someone else
- [ ] Implement a new feature (color output, etc.)

---

## 📚 Resources

- [Chapter 12 - The Rust Book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [std::env](https://doc.rust-lang.org/std/env/)
- [std::fs](https://doc.rust-lang.org/std/fs/)
- [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

---

## 🎯 Summary

**minigrep** is a complete, well-organized CLI application that:
- ✅ Demonstrates all Chapter 12 concepts
- ✅ Includes comprehensive tests
- ✅ Uses Rust idioms and best practices
- ✅ Separates concerns effectively
- ✅ Handles errors gracefully
- ✅ Includes clear documentation

Perfect for learning and reference! 🦀

---

*Tags: #rust-book #chapter12 #cli #complete-project #testing #error-handling #best-practices*
*Links: [[zettelkasten/rust_book/rust-book-ch12]] | [[Rust Book MOC]]*
