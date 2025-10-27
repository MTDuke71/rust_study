# MiniGrep - A Rust Command-Line Search Tool

**Tags:** #rust-book #chapter12 #cli #environment-variables #minigrep  
**Created:** 2025-10-22  
**Related:** [[zettelkasten/rust_book/rust-book-ch12]], [[CLI Design Patterns]], [[Environment Variables]], [[Error Handling Patterns]], [[Rust Book MOC]]

**Chapter 12.5: Complete Standalone Implementation**

A simplified version of the `grep` command-line tool, demonstrating Rust's capabilities for building robust CLI applications with environment variable configuration.

---

## 📚 Overview

This is the **complete, final implementation** from Chapter 12 of The Rust Book. It combines all concepts from previous sections into a fully functional command-line search tool with:

- ✅ **Command-line argument parsing**
- ✅ **File I/O with comprehensive error handling**
- ✅ **Environment variable configuration**
- ✅ **Case-sensitive and case-insensitive search**
- ✅ **Proper project organization**
- ✅ **Educational examples and demonstrations**

---

## 🚀 Quick Start

### Installation & Build
```bash
# Clone and navigate to the project
cd rust_book/Ch12/environment_variables

# Build the project
cargo build --release

# Or run directly
cargo run <search_query> <filename>
```

### Basic Usage
```bash
# Search for "nobody" in poem.txt (case-sensitive by default)
cargo run nobody poem.txt

# Case-insensitive search using environment variable
CASE_INSENSITIVE=1 cargo run nobody poem.txt

# Windows PowerShell
$env:CASE_INSENSITIVE="1"; cargo run nobody poem.txt

# Or run the specific binary directly
cargo run --bin minigrep_env nobody poem.txt
```

---

## 📖 Usage Documentation

### Command Syntax
```
minigrep <search_query> <filename>
```

### Arguments
- **`search_query`** - The text to search for in the file
- **`filename`** - The path to the file to search in

### Environment Variables
- **`CASE_INSENSITIVE`** - If set (to any value), enables case-insensitive search
- **`DEBUG`** - Set to 'true' to enable debug output (example)
- **`VERBOSE`** - Set to '1' to enable verbose output (example)
- **`MAX_RESULTS`** - Number (default: 10) for maximum search results (example)

### Examples
```bash
# Basic search (case-sensitive)
cargo run rust poem.txt

# Case-insensitive search
CASE_INSENSITIVE=1 cargo run rust poem.txt

# Search with different terms
cargo run "nobody" poem.txt
cargo run "are you" poem.txt
cargo run "frog" poem.txt

# Test error handling
cargo run                          # Shows usage message
cargo run search nonexistent.txt   # Shows file not found error

# Using specific binary name
cargo run --bin minigrep_env rust poem.txt
```

---

## 🎯 Features

### ✅ **Core Search Functionality**
- **Case-sensitive search** (default behavior)
- **Case-insensitive search** (via `CASE_INSENSITIVE` environment variable)
- **Line-by-line matching** with full line output
- **Multiple matches per line** supported

### ✅ **Robust Error Handling**
- **Missing arguments** - Clear usage instructions
- **File not found** - Helpful error messages
- **Permission denied** - Appropriate error context
- **Invalid UTF-8** - Graceful handling

### ✅ **Educational Components**
- **Interactive examples** demonstrating each concept
- **Environment variable tutorials**
- **Best practices demonstrations**
- **Real-world usage patterns**

### ✅ **Production-Ready Features**
- **Proper exit codes** (0 for success, 1 for errors)
- **Standard error output** for errors (`eprintln!`)
- **Standard output** for results (`println!`)
- **Configuration validation**

---

## 🛠️ Technical Details

### Architecture
```
main.rs
├── Config struct          - Command-line argument management
├── run() function        - Main application logic
├── search() functions    - Core search algorithms
├── example functions     - Educational demonstrations
└── main() function       - CLI entry point and error handling
```

### Dependencies
- **Standard Library Only** - No external crates required
- `std::env` - Environment variables and command-line arguments
- `std::fs` - File system operations
- `std::error` - Error handling traits
- `std::process` - Exit code management

### Error Handling Strategy
- **Result<T, E>** types for recoverable errors
- **? operator** for error propagation
- **unwrap_or_else()** for configuration errors
- **Box<dyn Error>** for trait objects

---

## 📝 Code Structure

### Configuration Management
```rust
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // Argument parsing with validation
        // Environment variable checking
    }
}
```

### Search Implementation
```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_toLowerCase().contains(&query))
        .collect()
}
```

### Main Application Flow
```rust
fn main() {
    // 1. Educational examples (if run without args)
    // 2. Parse configuration from command-line
    // 3. Run main application logic
    // 4. Handle errors gracefully
    // 5. Provide helpful feedback
}
```

---

## 🧪 Testing & Validation

### Automatic Sample Creation
The program automatically creates `poem.txt` if it doesn't exist:
```text
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public like a frog
To tell one's name the livelong day
To an admiring bog!
```

### Test Cases
```bash
# Test case-sensitive search
cargo run nobody poem.txt
# Expected: 2 matches (lines with "nobody")

# Test case-insensitive search  
CASE_INSENSITIVE=1 cargo run NOBODY poem.txt
# Expected: 2 matches (same lines, case-insensitive)

# Test no matches
cargo run xyz poem.txt
# Expected: "No matches found" message

# Test error handling
cargo run search nonexistent.txt
# Expected: File not found error
```

### Built-in Examples
Run without arguments to see educational examples:
```bash
cargo run
```
This demonstrates:
- Environment variable usage
- Configuration management
- Search behavior comparison
- Best practices

---

## 🎓 Learning Objectives

This complete implementation demonstrates:

### **Chapter 12 Concepts**
1. **Command-line argument parsing** - `std::env::args()`
2. **File I/O operations** - `std::fs::read_to_string()`
3. **Error handling** - `Result<T, E>` and `?` operator
4. **Project organization** - Separating concerns
5. **Environment variables** - `std::env::var()`

### **Rust Best Practices**
- **Ownership and borrowing** - Lifetime parameters in search functions
- **Error propagation** - Using `?` operator effectively
- **Iterator patterns** - `.lines().filter().collect()`
- **Configuration management** - Centralized `Config` struct
- **Documentation** - Comprehensive inline documentation

### **Real-world Patterns**
- **CLI application structure** - Argument parsing → Configuration → Execution → Output
- **Environment-based configuration** - Runtime behavior modification
- **Graceful error handling** - User-friendly error messages
- **Exit code management** - Proper process termination

---

## 🔗 Related Learning

### **Previous Sections**
- `../accepting_arguments/` - Basic argument parsing
- `../reading_files/` - File I/O fundamentals
- `../refactoring/` - Code organization
- `../testing/` - Test-driven development

### **Next Steps**
- [[zettelkasten/rust_book/rust-book-ch13]] - Functional programming with iterators
- [[mission8 Overview]] - Advanced project structure  
- [[Mission 9 Overview]] - CLI pathfinding tools

### **Rust Book Integration**
- Applies concepts from Chapters 1-11
- Foundation for advanced topics in Chapters 13+
- Real-world demonstration of Rust's capabilities
- See [[zettelkasten/rust_book/rust-book-ch12]] for complete learning path

---

## 💡 Tips & Troubleshooting

### **Common Issues**
1. **"Didn't get a query string"** → Provide both search term and filename
2. **File not found** → Check file path and permissions
3. **No matches found** → Try case-insensitive search with `CASE_INSENSITIVE=1`

### **Performance Notes**
- Loads entire file into memory (suitable for text files)
- Linear search through file contents
- Efficient for typical use cases (< 100MB files)

### **Extensions & Modifications**
- Add regular expression support
- Implement recursive directory search
- Add output formatting options
- Include line numbers in output
- Add color highlighting

---

## 📄 License & Attribution

This implementation is part of **The Rust Programming Language** book examples, adapted for educational purposes with enhanced documentation and examples.

**Original Source**: [The Rust Book Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

---

*This is a complete, standalone Rust program demonstrating production-ready CLI development patterns. Perfect for learning Rust's approach to system programming and command-line tools.*