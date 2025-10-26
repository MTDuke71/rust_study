# Error Messages Demo - Chapter 12.6

This is a comprehensive demonstration of proper error handling and stderr/stdout separation in Rust CLI applications, as covered in Chapter 12.6 of The Rust Programming Language book.

## 🎯 What This Demonstrates

### Key Concepts
- **Standard Output (stdout)** vs **Standard Error (stderr)** separation
- Using `println!()` for normal output and `eprintln!()` for errors
- Proper error handling with `Result<T, E>` and `?` operator
- Meaningful exit codes (0 = success, 1 = error)
- Clear, actionable error messages

### Features Implemented
- ✅ Proper argument parsing with helpful error messages
- ✅ Case-sensitive and case-insensitive text search
- ✅ Environment variable support (`CASE_INSENSITIVE`)
- ✅ Comprehensive error handling for file operations
- ✅ Usage help and examples
- ✅ Stream separation demonstration
- ✅ Unit tests for core functionality

## 🚀 How to Run

### Basic Usage
```bash
# Show help
cargo run

# Search for text (case-sensitive)
cargo run nobody poem.txt

# Search for text (case-insensitive)
CASE_INSENSITIVE=1 cargo run NOBODY poem.txt

# Demo different error message types
cargo run --demo-errors
```

### Testing Stream Separation
```bash
# Capture stdout and stderr separately
cargo run nobody poem.txt > results.txt 2> errors.txt

# View results (search matches)
cat results.txt

# View errors/debug info
cat errors.txt

# Capture only results, discard errors
cargo run nobody poem.txt 2>/dev/null

# Capture only errors, discard results  
cargo run nobody poem.txt 2>&1 1>/dev/null
```

### Run the Demo Script
```bash
# PowerShell (Windows)
./demo.ps1

# Bash (Linux/macOS)
chmod +x demo.sh && ./demo.sh
```

### Run Tests
```bash
cargo test
```

## 📋 Example Outputs

### Successful Search (stdout + stderr)
```
Configuration: Config { query: "nobody", filename: "poem.txt", case_sensitive: true }
Found 2 match(es):
I'm nobody! Who are you?
Are you nobody, too?
```

**What goes where:**
- `Configuration: ...` → **stderr** (debug info)
- `Found 2 match(es):` → **stderr** (count info)
- Search results → **stdout** (can be redirected)

### Error Case
```
❌ Problem parsing arguments: Missing required argument: FILENAME

🦀 Error Messages Demo - Chapter 12.6
=====================================

USAGE:
    error_messages_demo QUERY FILENAME
...
```

**Exit code:** `1` (error)

### File Not Found
```
Configuration: Config { query: "test", filename: "missing.txt", case_sensitive: true }
❌ Application error: Failed to read file 'missing.txt': No such file or directory (os error 2)
```

**Exit code:** `1` (error)

## 🧪 Testing

The example includes comprehensive unit tests:

```bash
cargo test

running 5 tests
test tests::test_case_insensitive_search ... ok
test tests::test_case_sensitive_search ... ok
test tests::test_config_creation_missing_filename ... ok
test tests::test_config_creation_missing_query ... ok
test tests::test_config_creation_success ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 🔑 Key Learning Points

### 1. Stream Separation
- **stdout** (`println!`) - For program output that users might want to redirect
- **stderr** (`eprintln!`) - For errors, warnings, and debug information

### 2. Error Handling Pattern
```rust
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("❌ Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    if let Err(e) = run(config) {
        eprintln!("❌ Application error: {}", e);
        process::exit(1);
    }
}
```

### 3. Proper Error Messages
- Include context (what operation failed)
- Include the underlying error (why it failed)
- Suggest solutions when possible
- Use consistent formatting and emoji for clarity

### 4. Exit Codes
- `0` - Success
- `1` - General error
- Show help without error code `0`

## 📁 Files Included

- `src/main.rs` - Main application with comprehensive error handling
- `poem.txt` - Sample text file for searching
- `demo.ps1` - PowerShell script demonstrating all features
- `Cargo.toml` - Project configuration
- `README.md` - This documentation

## 🔗 Related

This example complements:
- **Chapter 12.1** - Command line arguments
- **Chapter 12.3** - Refactoring and error handling
- **Chapter 12.5** - Environment variables
- **Chapter 9** - Comprehensive error handling

## 💡 Try These Experiments

1. **Redirect streams separately:**
   ```bash
   cargo run nobody poem.txt > results.txt 2> debug.txt
   ```

2. **Chain with other commands:**
   ```bash
   cargo run nobody poem.txt | wc -l
   ```

3. **Handle different error cases:**
   ```bash
   cargo run              # Missing arguments
   cargo run query        # Missing filename  
   cargo run query missing.txt  # File not found
   ```

4. **Test case sensitivity:**
   ```bash
   cargo run NOBODY poem.txt                    # No matches
   CASE_INSENSITIVE=1 cargo run NOBODY poem.txt # Matches found
   ```

This example provides a complete, production-ready CLI application that demonstrates all the error handling best practices from Chapter 12.6!