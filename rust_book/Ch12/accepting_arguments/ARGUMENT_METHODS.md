# Command Line Argument Handling Methods in Rust

## 🔗 Zettelkasten Links
- **Knowledge Hub**: [[Command Line Argument Methods]] - Comprehensive zettelkasten entry
- **Chapter Context**: [[zettelkasten/rust_book/rust-book-ch12]] - Parent chapter overview
- **Related**: [[Rust CLI Applications]], [[Error Handling Patterns]], [[Input Validation]]

This document demonstrates different approaches to handling command-line arguments in Rust, based on the examples in `main.rs`.

## Overview

The program demonstrates 4 different methods for handling command-line arguments, progressing from basic to more robust approaches.

## Method 1: Basic Argument Collection

### Description
Simple collection of all command-line arguments into a vector.

### Code Example
```rust
use std::env;

fn basic_args() {
    let args: Vec<String> = env::args().collect();
    
    println!("Program name: {}", args[0]);
    println!("Total arguments: {}", args.len());
    
    if args.len() > 1 {
        for (i, arg) in args.iter().enumerate().skip(1) {
            println!("  [{}]: {}", i, arg);
        }
    }
}
```

### Pros
- Simple and straightforward
- Shows all arguments clearly
- Good for debugging

### Cons
- No validation
- No error handling
- Requires manual parsing

## Method 2: Direct Argument Parsing

### Description
Assumes specific argument positions and parses them directly.

### Code Example
```rust
fn argument_parsing() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        println!("Usage: {} <search_query> <filename>", args[0]);
        return;
    }
    
    let query = &args[1];
    let filename = &args[2];
    
    println!("Search query: '{}'", query);
    println!("Target file: '{}'", filename);
}
```

### Pros
- Clear and direct
- Easy to understand
- Minimal code

### Cons
- Can panic if not enough arguments
- Basic error handling only
- No validation of argument content

## Method 3: Argument Validation with Result

### Description
Comprehensive validation using Result type for proper error handling.

### Code Example
```rust
fn validate_args(args: &[String]) -> Result<(&str, &str), &'static str> {
    if args.len() < 3 {
        return Err("Not enough arguments. Expected: <query> <filename>");
    }
    
    if args.len() > 3 {
        return Err("Too many arguments. Expected: <query> <filename>");
    }
    
    let query = &args[1];
    let filename = &args[2];
    
    if query.is_empty() {
        return Err("Search query cannot be empty");
    }
    
    if !filename.contains('.') {
        return Err("Filename should have an extension");
    }
    
    Ok((query, filename))
}

fn argument_validation() {
    let args: Vec<String> = env::args().collect();
    
    match validate_args(&args) {
        Ok((query, filename)) => {
            println!("Valid arguments:");
            println!("   Query: '{}' (length: {})", query, query.len());
            println!("   File: '{}' (extension: {})", 
                filename, 
                filename.split('.').next_back().unwrap_or("none")
            );
        }
        Err(error_msg) => {
            println!("Validation error: {}", error_msg);
        }
    }
}
```

### Pros
- Comprehensive validation
- Proper error handling with Result
- Detailed error messages
- Type safety

### Cons
- More complex code
- Requires understanding of Result type

## Method 4: Safe Argument Access Patterns

### Description
Demonstrates various safe ways to access arguments without panicking.

### Code Example
```rust
fn safe_argument_patterns() {
    let args: Vec<String> = env::args().collect();
    
    // ❌ Risky: Direct indexing (can panic)
    // let query = &args[1];  // Could panic if no args!
    
    // ✅ Safe: Using get() method
    if let Some(query) = args.get(1) {
        println!("Query found: '{}'", query);
    } else {
        println!("No query provided");
    }
    
    // ✅ Safe: Pattern matching on length
    match args.len() {
        1 => println!("No arguments provided"),
        2 => println!("Only query provided: '{}'", args[1]),
        3 => println!("Query: '{}', File: '{}'", args[1], args[2]),
        _ => println!("Too many arguments: {}", args.len() - 1),
    }
}
```

### Pros
- Panic-safe
- Multiple approaches shown
- Good for learning defensive programming
- Handles edge cases well

### Cons
- More verbose
- Multiple patterns to choose from

## Usage Examples

### Running the Program

```bash
# No arguments - shows help
cargo run

# Proper usage
cargo run rust poem.txt

# Multi-word query (use quotes)
cargo run "hello world" data.csv

# Too many arguments (demonstrates error handling)
cargo run search file.txt extra argument

# Direct executable usage
./target/debug/minigrep_args.exe "pattern" "source.rs"
```

### Expected Output Formats

#### Success Case
```
✅ Parsed arguments successfully:
   Search query: 'rust'
   Target file: 'poem.txt'

✅ Valid arguments:
   Query: 'rust' (length: 4)
   File: 'poem.txt' (extension: txt)
```

#### Error Cases
```
❌ Usage: program_name <search_query> <filename>
❌ Validation error: Not enough arguments. Expected: <query> <filename>
❌ Validation error: Too many arguments. Expected: <query> <filename>
```

## Best Practices

### 1. Always Validate Input
- Check argument count before accessing
- Validate argument content
- Provide clear error messages

### 2. Use Safe Access Methods
```rust
// ✅ Good
if let Some(arg) = args.get(1) { /* use arg */ }

// ❌ Risky
let arg = &args[1]; // Can panic!
```

### 3. Handle Errors Gracefully
```rust
match validate_args(&args) {
    Ok((query, filename)) => { /* process */ }
    Err(msg) => {
        eprintln!("Error: {}", msg);
        std::process::exit(1);
    }
}
```

### 4. Consider Using Argument Parsing Libraries

For complex applications, consider using libraries like:
- **clap** - Full-featured argument parsing
- **structopt** - Derive-based argument parsing
- **argh** - Lightweight alternative

## Related Topics

- [Chapter 12.2: Reading Files](../reading_files/)
- [Error Handling in Rust](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Command Line Applications in Rust](https://rust-cli.github.io/book/)

## Key Takeaways

1. **Start Simple**: Begin with basic argument collection for learning
2. **Add Validation**: Always validate input in real applications
3. **Handle Errors**: Use Result types for proper error handling
4. **Be Safe**: Avoid direct indexing that can panic
5. **Consider Libraries**: For complex CLI apps, use dedicated parsing libraries

This progression from basic to advanced shows how Rust's type system and error handling can create robust command-line applications.