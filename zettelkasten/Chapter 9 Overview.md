# Chapter 9 Overview - Error Handling

**Tags:** #rust-book #chapter-overview #error-handling #result #panic #recoverable
**Created:** 2025-10-22
**Related:** [[Chapter 8]], [[Chapter 10]], [[Custom Error Types]], [[Error Types]], [[Rust Concepts MOC]]

## Chapter Summary

**Chapter 9: Error Handling** teaches Rust's philosophy and mechanisms for dealing with errors. Unlike languages that use exceptions, Rust categorizes errors into two main types: recoverable errors (handled with `Result<T, E>`) and unrecoverable errors (handled with `panic!`).

## Learning Objectives

By completing Chapter 9, you will understand:
- The difference between recoverable and unrecoverable errors
- When and how to use `panic!` for unrecoverable errors
- Using `Result<T, E>` for recoverable error handling
- Error propagation with the `?` operator
- When to panic vs return errors
- Creating custom error types

## Key Concepts

### 1. Unrecoverable Errors with panic!

#### When Programs Should Panic
```rust
// Explicit panic
fn main() {
    panic!("crash and burn");
}

// Index out of bounds (implicit panic)
fn main() {
    let v = vec![1, 2, 3];
    v[99];  // This will panic!
}
```

#### Panic Backtrace
```bash
# Set environment variable for detailed backtrace
RUST_BACKTRACE=1 cargo run

# Even more detailed backtrace  
RUST_BACKTRACE=full cargo run
```

### 2. Recoverable Errors with Result

#### Basic Result Usage
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

#### Shortcuts: unwrap and expect
```rust
use std::fs::File;

fn main() {
    // unwrap: returns value or panics with default message
    let greeting_file = File::open("hello.txt").unwrap();

    // expect: returns value or panics with custom message  
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

### 3. Propagating Errors

#### Manual Error Propagation
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),  // Early return with error
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

#### The ? Operator Shortcut
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;  // ? propagates error
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;      // ? propagates error
    Ok(username)
}

// Even shorter with chaining
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Shortest with fs::read_to_string
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}
```

#### The ? Operator with Option
```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()  // ? works with Option too
}
```

### 4. To panic! or Not to panic!

#### When to Use panic!
- **Examples, prototype code, and tests** - Quick failure is acceptable
- **When you have more information than the compiler** - You know error won't occur
- **Unrecoverable state** - Program cannot continue safely

```rust
use std::net::IpAddr;

// This will panic if parsing fails, but we know it won't
let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
```

#### When to Return Result
- **Library code** - Let caller decide how to handle errors
- **Expected failure conditions** - File not found, network errors, etc.
- **Recoverable situations** - Program can continue with different behavior

### 5. Guidelines for Error Handling

#### Creating Custom Error Types
```rust
#[derive(Debug)]
pub struct GuessError {
    message: String,
}

impl std::fmt::Display for GuessError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GuessError {}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Result<Guess, GuessError> {
        if value < 1 {
            return Err(GuessError {
                message: "Guess value must be greater than or equal to 1".to_string(),
            });
        } else if value > 100 {
            return Err(GuessError {
                message: "Guess value must be less than or equal to 100".to_string(),
            });
        }

        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

## Practical Examples

### File Operations with Error Handling
```rust
use std::fs;
use std::io;

fn process_config_file() -> Result<String, io::Error> {
    let contents = fs::read_to_string("config.toml")?;
    
    // Process the configuration
    let processed = contents.to_uppercase();
    
    Ok(processed)
}

fn main() {
    match process_config_file() {
        Ok(config) => println!("Config processed: {}", config),
        Err(e) => eprintln!("Failed to process config: {}", e),
    }
}
```

### Network Request with Multiple Error Types
```rust
use std::num::ParseIntError;
use std::io;

#[derive(Debug)]
enum NetworkError {
    Io(io::Error),
    Parse(ParseIntError),
    InvalidPort,
}

impl From<io::Error> for NetworkError {
    fn from(error: io::Error) -> Self {
        NetworkError::Io(error)
    }
}

impl From<ParseIntError> for NetworkError {
    fn from(error: ParseIntError) -> Self {
        NetworkError::Parse(error)
    }
}

fn connect_to_server(address: &str) -> Result<(), NetworkError> {
    let parts: Vec<&str> = address.split(':').collect();
    
    if parts.len() != 2 {
        return Err(NetworkError::InvalidPort);
    }
    
    let port: u16 = parts[1].parse()?;  // ? converts ParseIntError to NetworkError
    
    if port < 1024 {
        return Err(NetworkError::InvalidPort);
    }
    
    // Simulate network operation that might fail
    // std::net::TcpStream::connect(address)?;  // ? converts io::Error
    
    Ok(())
}
```

### Validation with Multiple Checks
```rust
#[derive(Debug)]
struct ValidationError {
    field: String,
    message: String,
}

fn validate_email(email: &str) -> Result<(), ValidationError> {
    if email.is_empty() {
        return Err(ValidationError {
            field: "email".to_string(),
            message: "Email cannot be empty".to_string(),
        });
    }
    
    if !email.contains('@') {
        return Err(ValidationError {
            field: "email".to_string(),
            message: "Email must contain @ symbol".to_string(),
        });
    }
    
    Ok(())
}

fn validate_user_data(name: &str, email: &str) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    
    if name.is_empty() {
        errors.push(ValidationError {
            field: "name".to_string(),
            message: "Name cannot be empty".to_string(),
        });
    }
    
    if let Err(e) = validate_email(email) {
        errors.push(e);
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

## Integration with Other Concepts

### Error Handling in Collections
```rust
fn parse_numbers(strings: Vec<&str>) -> Result<Vec<i32>, std::num::ParseIntError> {
    strings.into_iter()
           .map(|s| s.parse::<i32>())  // Returns Result<i32, ParseIntError>
           .collect()                  // collect() can return Result<Vec<T>, E>
}

// Usage
let input = vec!["1", "2", "3", "4"];
match parse_numbers(input) {
    Ok(numbers) => println!("Numbers: {:?}", numbers),
    Err(e) => eprintln!("Parse error: {}", e),
}
```

### Error Handling with Iterators
```rust
fn process_files(filenames: &[&str]) -> Result<Vec<String>, io::Error> {
    filenames.iter()
             .map(|&filename| std::fs::read_to_string(filename))
             .collect()  // Short-circuits on first error
}

// Alternative: collect successful results, log errors
fn process_files_partial(filenames: &[&str]) -> Vec<String> {
    filenames.iter()
             .filter_map(|&filename| {
                 match std::fs::read_to_string(filename) {
                     Ok(content) => Some(content),
                     Err(e) => {
                         eprintln!("Failed to read {}: {}", filename, e);
                         None
                     }
                 }
             })
             .collect()
}
```

## Testing Error Conditions

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_valid() {
        let guess = Guess::new(50);
        assert!(guess.is_ok());
        assert_eq!(guess.unwrap().value(), 50);
    }

    #[test]
    fn test_guess_too_low() {
        let guess = Guess::new(0);
        assert!(guess.is_err());
    }

    #[test]
    fn test_guess_too_high() {
        let guess = Guess::new(101);
        assert!(guess.is_err());
    }
    
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn test_guess_panic_message() {
        Guess::new(200).unwrap();
    }
}
```

## Best Practices from Chapter 9

1. **Use `Result<T, E>` for recoverable errors**
   - File operations, network requests, parsing
   - When the caller can handle the error meaningfully

2. **Use `panic!` for unrecoverable errors**
   - Bugs in your program logic
   - Contract violations (like array bounds)
   - When continuing would be unsafe

3. **Use the `?` operator for error propagation**
   - Cleaner than manual match statements
   - Works with both `Result` and `Option`
   - Automatically converts error types with `From` trait

4. **Create custom error types for domain-specific errors**
   - Implement `Debug`, `Display`, and `Error` traits
   - Use enums for multiple error variants
   - Provide helpful error messages

5. **Consider using `expect()` over `unwrap()`**
   - Provides context when panic occurs
   - Documents why you believe the operation won't fail

## Relationship to Other Chapters

### Previous Chapters
- **Chapter 6 (Enums):** `Result` is an enum with `Ok` and `Err` variants
- **Chapter 8 (Collections):** Error handling when working with vectors and strings

### Following Chapters  
- **Chapter 10 (Generics):** `Result<T, E>` uses generic type parameters
- **Advanced features:** Custom error traits and error handling libraries

## Common Patterns in Rust Study Projects

### Advent of Code Error Handling
```rust
fn parse_input(input: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    input.lines()
         .map(|line| line.parse::<i32>().map_err(Into::into))
         .collect()
}
```

### Mission Project Validation
```rust
fn validate_solution(answer: i32, expected_range: std::ops::Range<i32>) -> Result<i32, String> {
    if expected_range.contains(&answer) {
        Ok(answer)
    } else {
        Err(format!("Answer {} is outside valid range {:?}", answer, expected_range))
    }
}
```

### File Processing in Examples
```rust
fn load_test_data(filename: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(format!("test_data/{}", filename))
}
```

## Key Chapter 9 Takeaways

1. **Error handling is explicit** - No hidden exceptions
2. **Two error categories** - Recoverable (`Result`) vs unrecoverable (`panic!`)
3. **The `?` operator** - Elegant error propagation
4. **Custom error types** - Create domain-specific error handling
5. **Error handling philosophy** - Fail fast vs graceful degradation

## Study Notes and Exercises

Practice implementing error handling in these scenarios:
- File I/O operations with multiple failure modes
- User input validation with detailed error messages  
- Network operations with timeout and retry logic
- Parsing structured data with multiple error types

## Related Concepts

- [[Custom Error Types]] - Advanced error type creation
- [[Result]] - Deep dive into Result type usage
- [[Option]] - Similar pattern for representing absence
- [[Chapter 6]] - Enum foundations for Result type
- [[Chapter 10]] - Generic foundations for Result<T, E>

---

*Chapter 9: Master Rust's explicit, type-safe approach to error handling with Result types and panic strategies.*