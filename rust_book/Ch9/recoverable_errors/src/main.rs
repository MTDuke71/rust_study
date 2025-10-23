use std::fmt;
/// # Chapter 9.2 - Recoverable Errors with Result<T, E>
///
/// This module demonstrates recoverable error handling using Result<T, E>.
///
/// ## Key Concepts:
/// 1. Result<T, E> enum structure
/// 2. Pattern matching with Result
/// 3. Error propagation techniques  
/// 4. Shortcuts: unwrap() and expect()
/// 5. Transforming errors with map_err()
///
/// ## Examples:
/// - File operations with error handling
/// - Custom Result types
/// - Error transformation and chaining
/// - Best practices for Result handling
use std::fs::File;
use std::io::{self, Write};

fn main() {
    println!("=== Chapter 9.2: Recoverable Errors with Result<T, E> ===\n");

    // Example 1: Basic Result handling
    demonstrate_basic_result_handling();

    // Example 2: Specific error handling
    demonstrate_specific_error_handling();

    // Example 3: Result shortcuts
    demonstrate_result_shortcuts();

    // Example 4: Error transformation
    demonstrate_error_transformation();

    // Example 5: Custom Result types
    demonstrate_custom_result_types();

    // Example 6: Combinators
    demonstrate_result_combinators();
}

/// Demonstrates basic Result<T, E> pattern matching
fn demonstrate_basic_result_handling() {
    println!("1. Basic Result<T, E> handling:");

    // Attempt to open a file
    let filename = "example.txt";
    println!("   Attempting to open '{}'...", filename);

    let file_result = File::open(filename);

    match file_result {
        Ok(_file) => {
            println!("   ✅ Successfully opened {}", filename);
        }
        Err(error) => {
            println!("   ❌ Failed to open {}: {}", filename, error);
            println!("   Error kind: {:?}", error.kind());
        }
    }

    println!();
}

/// Demonstrates handling specific error types
fn demonstrate_specific_error_handling() {
    println!("2. Specific error type handling:");

    let filename = "test_file.txt";
    let file_result = File::open(filename);

    let _file = match file_result {
        Ok(file) => {
            println!("   ✅ File opened successfully");
            file
        }
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                println!("   📄 File not found, creating '{}'...", filename);
                match File::create(filename) {
                    Ok(file) => {
                        println!("   ✅ File created successfully");
                        file
                    }
                    Err(create_error) => {
                        println!("   ❌ Failed to create file: {}", create_error);
                        return;
                    }
                }
            }
            io::ErrorKind::PermissionDenied => {
                println!("   🔒 Permission denied accessing {}", filename);
                return;
            }
            other_error => {
                println!("   ❌ Other error opening file: {:?}", other_error);
                return;
            }
        },
    };

    // Clean up
    std::fs::remove_file(filename).ok();
    println!();
}

/// Demonstrates Result shortcuts: unwrap() and expect()
fn demonstrate_result_shortcuts() {
    println!("3. Result shortcuts (use with caution!):");

    // Create a file we know will exist
    let filename = "temp_file.txt";
    let mut file = File::create(filename).expect("Failed to create temp file");
    writeln!(file, "Hello, World!").expect("Failed to write to file");
    drop(file); // Close the file

    println!("   Created temporary file: {}", filename);

    // Now demonstrate shortcuts
    println!("   Using expect() - panics with custom message:");
    let content =
        std::fs::read_to_string(filename).expect("Should have been able to read the file");
    println!("   File content: '{}'", content.trim());

    println!(
        "   
   ⚠️  Shortcuts like unwrap() and expect():
   • unwrap() - panics with default message on Err
   • expect(msg) - panics with custom message on Err  
   • Use only when you're certain no error will occur
   • Better for prototypes than production code"
    );

    // Clean up
    std::fs::remove_file(filename).ok();
    println!();
}

/// Demonstrates error transformation with map_err()
fn demonstrate_error_transformation() {
    println!("4. Error transformation:");

    // Function that transforms io::Error to String
    fn read_file_as_string(filename: &str) -> Result<String, String> {
        std::fs::read_to_string(filename)
            .map_err(|err| format!("Failed to read '{}': {}", filename, err))
    }

    // Test with non-existent file
    match read_file_as_string("nonexistent.txt") {
        Ok(content) => println!("   Content: {}", content),
        Err(error_msg) => println!("   ❌ {}", error_msg),
    }

    // Function that provides context
    fn parse_number_with_context(s: &str) -> Result<i32, String> {
        s.parse::<i32>()
            .map_err(|_| format!("'{}' is not a valid integer", s))
    }

    let test_inputs = vec!["42", "not_a_number", "123"];
    for input in test_inputs {
        match parse_number_with_context(input) {
            Ok(num) => println!("   '{}' -> {}", input, num),
            Err(msg) => println!("   ❌ {}", msg),
        }
    }

    println!();
}

/// Custom error type for demonstration
#[derive(Debug, PartialEq)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    #[allow(dead_code)]
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Division by zero"),
            MathError::NegativeSquareRoot => write!(f, "Square root of negative number"),
            MathError::Overflow => write!(f, "Mathematical overflow"),
        }
    }
}

/// Demonstrates custom Result types
fn demonstrate_custom_result_types() {
    println!("5. Custom Result types:");

    fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
        if b == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }

    fn safe_sqrt(x: f64) -> Result<f64, MathError> {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    type MathTest = (&'static str, Box<dyn Fn() -> Result<f64, MathError>>);
    let test_cases: Vec<MathTest> = vec![
        ("10.0 ÷ 2.0", Box::new(|| safe_divide(10.0, 2.0))),
        ("10.0 ÷ 0.0", Box::new(|| safe_divide(10.0, 0.0))),
        ("√16.0", Box::new(|| safe_sqrt(16.0))),
        ("√(-4.0)", Box::new(|| safe_sqrt(-4.0))),
    ];

    for (description, operation) in test_cases {
        match operation() {
            Ok(result) => println!("   {} = {:.2}", description, result),
            Err(error) => println!("   {} -> Error: {}", description, error),
        }
    }

    println!();
}

/// Demonstrates Result combinators and chaining
fn demonstrate_result_combinators() {
    println!("6. Result combinators and chaining:");

    // and_then for chaining Results
    fn parse_and_double(s: &str) -> Result<i32, String> {
        s.parse::<i32>()
            .map_err(|_| format!("Failed to parse '{}'", s))
            .and_then(|n| {
                if n > 1000 {
                    Err("Number too large to double".to_string())
                } else {
                    Ok(n * 2)
                }
            })
    }

    let inputs = vec!["21", "abc", "1001", "500"];
    for input in inputs {
        match parse_and_double(input) {
            Ok(result) => println!("   '{}' -> doubled = {}", input, result),
            Err(msg) => println!("   '{}' -> Error: {}", input, msg),
        }
    }

    println!();

    // or_else for providing fallbacks
    fn get_number_or_default(s: &str) -> Result<i32, String> {
        s.parse::<i32>().or_else(|_| {
            println!("     Parsing failed, trying default...");
            Ok(0) // Default value
        })
    }

    println!("   Using or_else for fallback values:");
    let test_values = vec!["42", "invalid"];
    for value in test_values {
        match get_number_or_default(value) {
            Ok(num) => println!("   '{}' -> {}", value, num),
            Err(msg) => println!("   '{}' -> Error: {}", value, msg),
        }
    }

    println!();

    // Combining multiple Results
    fn combine_results() -> Result<String, String> {
        let num1 = "42"
            .parse::<i32>()
            .map_err(|_| "First parse failed".to_string())?;
        let num2 = "24"
            .parse::<i32>()
            .map_err(|_| "Second parse failed".to_string())?;

        Ok(format!("Sum: {}", num1 + num2))
    }

    match combine_results() {
        Ok(result) => println!("   Combined result: {}", result),
        Err(error) => println!("   ❌ {}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide() {
        fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
            if b == 0.0 {
                Err(MathError::DivisionByZero)
            } else {
                Ok(a / b)
            }
        }

        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
        assert!(matches!(
            safe_divide(10.0, 0.0),
            Err(MathError::DivisionByZero)
        ));
    }

    #[test]
    fn test_result_mapping() {
        let result: Result<i32, &str> = Ok(42);
        let mapped = result.map(|x| x * 2);
        assert_eq!(mapped, Ok(84));

        let error_result: Result<i32, &str> = Err("error");
        let mapped_error = error_result.map(|x| x * 2);
        assert_eq!(mapped_error, Err("error"));
    }

    #[test]
    fn test_error_transformation() {
        fn parse_with_context(s: &str) -> Result<i32, String> {
            s.parse::<i32>()
                .map_err(|_| format!("Failed to parse '{}'", s))
        }

        assert_eq!(parse_with_context("42"), Ok(42));
        assert_eq!(
            parse_with_context("abc"),
            Err("Failed to parse 'abc'".to_string())
        );
    }
}
