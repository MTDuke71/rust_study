# Section 12.6: Writing Error Messages to Standard Error Instead of Standard Output

## Overview

This section covers the important distinction between **standard output (stdout)** and **standard error (stderr)** streams. Writing errors to stderr instead of stdout is crucial for proper Unix/Linux pipeline compatibility and allows users to redirect output and errors separately.

---

## 🎯 Key Concepts

### 1. **Standard Output vs Standard Error**

**Standard Output (`stdout`)**
- Used for normal program output
- Command: `println!()`
- Redirection: `program > output.txt`

**Standard Error (`stderr`)**
- Used for error messages and diagnostics
- Command: `eprintln!()`
- Redirection: `program 2> errors.txt`

**Why it matters:**
```bash
# Separate output from errors
program > output.txt 2> errors.txt

# Capture only successful output (errors go to console)
program > results.txt

# Discard errors
program 2>/dev/null
```

### 2. **Using eprintln! Macro**

Print error messages to stderr instead of stdout:

```rust
use std::process;

fn main() {
    let config = Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
```

### 3. **Error Handling Pattern**

Proper structure for CLI error handling:

```rust
use std::error::Error;
use std::fs;

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);  // ✅ Normal output to stdout
    }
    
    Ok(())
}

fn main() {
    let config = Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);  // ✅ Errors to stderr
        std::process::exit(1);
    });
    
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);  // ✅ Errors to stderr
        std::process::exit(1);
    }
}
```

### 4. **Exit Codes**

Proper program termination with meaningful exit codes:

```rust
use std::process;

fn main() {
    // Success case
    println!("Program completed successfully");
    process::exit(0);  // ✅ Exit with success
    
    // Error cases
    eprintln!("Fatal error occurred");
    process::exit(1);  // ✅ Exit with error
}
```

**Standard Exit Codes:**
- `0` - Success
- `1` - General error
- `2` - Misuse of shell command
- `126` - Command cannot execute
- `127` - Command not found

### 5. **Testing Error Output**

You can't easily test stderr/stdout separation with standard Rust tests, but you can structure code to be testable:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_logic() {
        let query = "duct";
        let contents = "Rust\nsafe, fast, productive.";
        
        let results = search(query, contents);
        assert_eq!(vec!["safe, fast, productive."], results);
    }
    
    // Error handling is tested at the integration level
    // (not in unit tests where stderr redirection is tested)
}
```

---

## 📋 Common Patterns

### Pattern 1: Proper Main Function with Error Handling

```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
    
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
```

### Pattern 2: Clean Error Messages

```rust
// ❌ BAD - Confusing error message to stdout
println!("Problem reading file: file not found");

// ✅ GOOD - Clear error message to stderr
eprintln!("Error: Failed to read file: {}", filename);
eprintln!("Reason: {}", err);
```

### Pattern 3: Conditional Output

```rust
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)
        .map_err(|e| format!("Failed to read file '{}': {}", config.filename, e))?;
    
    let results = search(&config.query, &contents);
    
    // Only print results if found
    if results.is_empty() {
        eprintln!("No matches found for '{}'", config.query);
    } else {
        for line in results {
            println!("{}", line);  // Results go to stdout
        }
    }
    
    Ok(())
}
```

### Pattern 4: Multiple Error Levels

```rust
use std::process;

fn main() {
    // Warning level (stderr but continues)
    if some_warning_condition {
        eprintln!("Warning: {}", warning_message);
    }
    
    // Error level (stderr and exits)
    if some_error_condition {
        eprintln!("Error: {}", error_message);
        process::exit(1);
    }
    
    // Info/Results level (stdout)
    println!("Results: {}", results);
}
```

---

## 🧪 Testing Considerations

### Unit Tests
Test the logic independently:

```rust
#[test]
fn test_search() {
    let result = search("test", "this is a test");
    assert_eq!(vec!["this is a test"], result);
}
```

### Integration Tests
You can test the entire CLI behavior:

```rust
#[test]
#[should_panic]
fn test_missing_file() {
    // This would be tested by running the actual binary
    // and checking the exit code and stderr output
}
```

### Manual Testing
For stderr/stdout separation, test manually:

```bash
# Run and capture both streams separately
./program arg1 > output.txt 2> errors.txt

# View output
cat output.txt

# View errors
cat errors.txt

# Capture only stderr
./program arg1 2>&1 1>/dev/null

# Discard stderr, keep stdout
./program arg1 2>/dev/null
```

---

## 🔑 Key Takeaways

| Concept | Use Case | Example |
|---------|----------|---------|
| **stdout** | Normal program output | `println!("{}", result);` |
| **stderr** | Error messages | `eprintln!("Error: {}", e);` |
| **Exit codes** | Signal success/failure | `process::exit(0)` or `process::exit(1)` |
| **Error handling** | Proper error propagation | Use `Result<T, E>` with `?` operator |
| **User experience** | Clear error messages | Descriptive, actionable error text |

### Best Practices

1. **Always use `eprintln!()` for errors** - Not `println!()`
2. **Provide clear error messages** - Include context and suggestions
3. **Use appropriate exit codes** - 0 for success, non-zero for errors
4. **Separate concerns** - Errors and results are different outputs
5. **Test error handling** - Include error cases in tests

---

## 💡 Real-World Example

```rust
use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // Skip program name
        
        let query = args.next()
            .ok_or("Didn't get a query string")?;
        
        let filename = args.next()
            .ok_or("Didn't get a file name")?;
        
        Ok(Config { query, filename })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let results = search(&config.query, &contents);
    
    if results.is_empty() {
        eprintln!("No matches found for '{}'", config.query);
    } else {
        for line in results {
            println!("{}", line);
        }
    }
    
    Ok(())
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
```

---

## 📚 Exercises

1. **Create a simple CLI program** that reads a file and searches for a pattern
   - Use `println!()` for search results
   - Use `eprintln!()` for all errors
   - Return appropriate exit codes

2. **Test stderr/stdout separation**:
   ```bash
   cargo run -- pattern test_file.txt > output.txt 2> errors.txt
   ```

3. **Add informative error messages** that include:
   - What went wrong
   - Why it went wrong
   - Suggestions for fixing it

4. **Implement warning messages** that go to stderr but don't exit

---

## 🔗 Related Content

**From Chapter 12:**
- [[12.1 Accepting Command Line Arguments]]
- [[12.3 Refactoring to Improve Modularity and Error Handling]]
- [[12.4 Testing]]
- [[12.5 Environment Variables]]

**From Earlier Chapters:**
- [[Chapter 9 - Error Handling]] - Detailed error handling strategies

**Further Reading:**
- [Standard Error (stderr) in Unix](https://en.wikipedia.org/wiki/Standard_streams)
- [Rust std::process documentation](https://doc.rust-lang.org/std/process/)
- [The Rust Book - Ch9: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

## 🎓 Summary

**Key Points:**
- Use `eprintln!()` for error messages (goes to stderr)
- Use `println!()` for normal output (goes to stdout)
- Use `std::process::exit()` with proper exit codes
- This separation allows users to redirect output independently
- Proper error handling is essential for production CLI tools
- Clear, actionable error messages improve user experience

**Practice:** Build a CLI tool that properly separates normal output from error output and uses appropriate exit codes!

---

*Tags: #rust-book #chapter12 #cli #error-handling #stderr #stdout #exit-codes #best-practices*
*Links: [[Rust Book MOC]] | [[zettelkasten/rust_book/rust-book-ch12]]*
