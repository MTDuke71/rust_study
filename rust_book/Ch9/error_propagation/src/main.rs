use std::error::Error;
use std::fmt;
/// # Chapter 9.3 - Error Propagation with the ? Operator
///
/// This module demonstrates error propagation using the ? operator.
///
/// ## Key Concepts:
/// 1. The ? operator for early returns
/// 2. Automatic error conversion with From trait
/// 3. Using ? with Option types
/// 4. Combining ? with Result and Option
/// 5. Error propagation in function chains
///
/// ## Examples:
/// - Manual vs automatic error propagation
/// - File operations with ? operator
/// - Option propagation patterns
/// - Custom error type conversions
use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    println!("=== Chapter 9.3: Error Propagation with ? Operator ===\n");

    // Example 1: Manual vs automatic propagation
    demonstrate_manual_vs_automatic_propagation();

    // Example 2: ? operator with files
    demonstrate_file_operations();

    // Example 3: ? with Option
    demonstrate_option_propagation();

    // Example 4: Custom error conversions
    demonstrate_custom_error_conversions();

    // Example 5: Chaining operations
    demonstrate_operation_chaining();

    // Example 6: Mixed Result and Option
    demonstrate_mixed_types();
}

/// Demonstrates the difference between manual and automatic error propagation
fn demonstrate_manual_vs_automatic_propagation() {
    println!("1. Manual vs Automatic Error Propagation:");

    // Create test files
    setup_test_files();

    println!("   Manual propagation (verbose):");
    match read_username_manual() {
        Ok(username) => println!("   📝 Username (manual): {}", username),
        Err(e) => println!("   ❌ Manual error: {}", e),
    }

    println!(
        "   
   Automatic propagation with ? (concise):"
    );
    match read_username_automatic() {
        Ok(username) => println!("   📝 Username (automatic): {}", username),
        Err(e) => println!("   ❌ Automatic error: {}", e),
    }

    println!(
        "   
   Super concise version:"
    );
    match read_username_super_concise() {
        Ok(username) => println!("   📝 Username (concise): {}", username),
        Err(e) => println!("   ❌ Concise error: {}", e),
    }

    println!();
}

/// Manual error propagation (don't do this!)
fn read_username_manual() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");

    let mut username_file = username_file_result?;

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username.trim().to_string()),
        Err(e) => Err(e),
    }
}

/// Automatic error propagation with ? operator
fn read_username_automatic() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username.trim().to_string())
}

/// Super concise version using std::fs
fn read_username_super_concise() -> Result<String, io::Error> {
    let username = std::fs::read_to_string("username.txt")?;
    Ok(username.trim().to_string())
}

/// Helper function to create test files
fn setup_test_files() {
    // Create username.txt
    if let Ok(mut file) = File::create("username.txt") {
        let _ = writeln!(file, "alice_cooper");
    }

    // Create config.txt for later examples
    if let Ok(mut file) = File::create("config.txt") {
        let _ = writeln!(file, "port=8080");
        let _ = writeln!(file, "host=localhost");
        let _ = writeln!(file, "debug=true");
    }
}

/// Demonstrates file operations with ? operator
fn demonstrate_file_operations() {
    println!("2. File operations with ? operator:");

    // Multiple file operations
    fn process_config_files() -> Result<String, io::Error> {
        let username = std::fs::read_to_string("username.txt")?;
        let config = std::fs::read_to_string("config.txt")?;

        Ok(format!(
            "User: {}, Config lines: {}",
            username.trim(),
            config.lines().count()
        ))
    }

    match process_config_files() {
        Ok(info) => println!("   📁 {}", info),
        Err(e) => println!("   ❌ File processing error: {}", e),
    }

    // File creation and writing
    fn create_and_write_file(filename: &str, content: &str) -> Result<(), io::Error> {
        let mut file = File::create(filename)?;
        file.write_all(content.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    match create_and_write_file("output.txt", "Hello, Error Propagation!") {
        Ok(()) => println!("   ✅ File created successfully"),
        Err(e) => println!("   ❌ File creation error: {}", e),
    }

    println!();
}

/// Demonstrates ? operator with Option types
fn demonstrate_option_propagation() {
    println!("3. ? operator with Option types:");

    // Option propagation in calculations
    fn calculate_average(numbers: &[i32]) -> Option<f64> {
        let first = numbers.first()?; // Returns None if empty
        let last = numbers.get(numbers.len().checked_sub(1)?)?; // Safe subtraction

        Some((*first + *last) as f64 / 2.0)
    }

    let test_cases = [
        vec![1, 2, 3, 4, 5],
        vec![10],
        vec![], // Empty vector
    ];

    for (i, numbers) in test_cases.iter().enumerate() {
        match calculate_average(numbers) {
            Some(avg) => println!(
                "   Case {}: {:?} -> Average of first/last: {:.1}",
                i + 1,
                numbers,
                avg
            ),
            None => println!(
                "   Case {}: {:?} -> None (empty or invalid)",
                i + 1,
                numbers
            ),
        }
    }

    println!();

    // Chain of Option operations
    fn parse_and_process(s: &str) -> Option<String> {
        let num: i32 = s.parse().ok()?; // Convert Result to Option
        let doubled = num.checked_mul(2)?; // Handles overflow
        let as_string = doubled.to_string();

        Some(format!("Processed: {}", as_string))
    }

    let test_strings = vec!["42", "1073741824", "not_a_number"]; // Last causes overflow
    for s in test_strings {
        match parse_and_process(s) {
            Some(result) => println!("   '{}' -> {}", s, result),
            None => println!("   '{}' -> None (parse error or overflow)", s),
        }
    }

    println!();
}

/// Custom error type for demonstration
#[derive(Debug)]
enum ProcessingError {
    Io(io::Error),
    Parse(String),
    Business(String),
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcessingError::Io(err) => write!(f, "IO error: {}", err),
            ProcessingError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ProcessingError::Business(msg) => write!(f, "Business logic error: {}", msg),
        }
    }
}

impl Error for ProcessingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ProcessingError::Io(err) => Some(err),
            _ => None,
        }
    }
}

// Automatic conversion from io::Error
impl From<io::Error> for ProcessingError {
    fn from(err: io::Error) -> Self {
        ProcessingError::Io(err)
    }
}

/// Demonstrates custom error conversions with ? operator
fn demonstrate_custom_error_conversions() {
    println!("4. Custom error conversions:");

    fn process_user_file(filename: &str) -> Result<u32, ProcessingError> {
        // ? automatically converts io::Error to ProcessingError via From trait
        let content = std::fs::read_to_string(filename)?;

        // Manual conversion for parse errors
        let number: u32 = content
            .trim()
            .parse()
            .map_err(|_| ProcessingError::Parse(format!("Invalid number in {}", filename)))?;

        // Business logic validation
        if number == 0 {
            return Err(ProcessingError::Business(
                "Number cannot be zero".to_string(),
            ));
        }

        if number > 1000 {
            return Err(ProcessingError::Business("Number too large".to_string()));
        }

        Ok(number * 2)
    }

    // Create test files with different content
    let test_cases = vec![
        ("valid_number.txt", "42"),
        ("zero_number.txt", "0"),
        ("large_number.txt", "1001"),
        ("invalid_number.txt", "not_a_number"),
    ];

    for (filename, content) in test_cases {
        if let Ok(mut file) = File::create(filename) {
            let _ = writeln!(file, "{}", content);
        }

        match process_user_file(filename) {
            Ok(result) => println!("   ✅ {} -> processed: {}", filename, result),
            Err(e) => {
                println!("   ❌ {} -> Error: {}", filename, e);
                if let Some(source) = e.source() {
                    println!("      Caused by: {}", source);
                }
            }
        }
    }

    println!();
}

/// Demonstrates chaining operations with ? operator
fn demonstrate_operation_chaining() {
    println!("5. Chaining operations:");

    // Complex operation chain
    fn process_pipeline(input: &str) -> Result<String, ProcessingError> {
        // Step 1: Read from file
        let content = std::fs::read_to_string(input)?;

        // Step 2: Parse each line as number
        let mut numbers = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            let num: i32 = line.trim().parse().map_err(|_| {
                ProcessingError::Parse(format!(
                    "Invalid number on line {}: '{}'",
                    line_num + 1,
                    line
                ))
            })?;
            numbers.push(num);
        }

        // Step 3: Business logic validation
        if numbers.is_empty() {
            return Err(ProcessingError::Business("No numbers found".to_string()));
        }

        // Step 4: Calculate and format result
        let sum: i32 = numbers.iter().sum();
        let average = sum as f64 / numbers.len() as f64;

        Ok(format!(
            "Numbers: {:?}, Sum: {}, Average: {:.2}",
            numbers, sum, average
        ))
    }

    // Create test file with numbers
    if let Ok(mut file) = File::create("numbers.txt") {
        let _ = writeln!(file, "10");
        let _ = writeln!(file, "20");
        let _ = writeln!(file, "30");
    }

    match process_pipeline("numbers.txt") {
        Ok(result) => println!("   📊 Pipeline result: {}", result),
        Err(e) => println!("   ❌ Pipeline error: {}", e),
    }

    println!();
}

/// Demonstrates mixing Result and Option with conversions
fn demonstrate_mixed_types() {
    println!("6. Mixing Result and Option:");

    // Function that works with both Result and Option
    fn safe_divide_and_sqrt(a: f64, b: f64) -> Result<Option<f64>, String> {
        if b == 0.0 {
            return Err("Division by zero".to_string());
        }

        let division_result = a / b;

        if division_result < 0.0 {
            Ok(None) // Can't take sqrt of negative
        } else {
            Ok(Some(division_result.sqrt()))
        }
    }

    let test_cases = vec![
        (16.0, 4.0),  // Valid case: sqrt(4) = 2
        (16.0, 0.0),  // Division by zero
        (-16.0, 4.0), // Negative result
    ];

    for (a, b) in test_cases {
        match safe_divide_and_sqrt(a, b) {
            Ok(Some(result)) => println!("   {} ÷ {} then √ = {:.2}", a, b, result),
            Ok(None) => println!("   {} ÷ {} = negative (no sqrt)", a, b),
            Err(msg) => println!("   {} ÷ {} -> Error: {}", a, b, msg),
        }
    }

    // Converting between Result and Option
    fn convert_examples() {
        println!(
            "   
   Converting between Result and Option:"
        );

        // Result to Option with .ok()
        let result: Result<i32, &str> = Ok(42);
        let option: Option<i32> = result.ok();
        println!("   Result::Ok(42).ok() = {:?}", option);

        // Option to Result with .ok_or()
        let option: Option<i32> = Some(42);
        let result: Result<i32, &str> = option.ok_or("No value");
        println!("   Some(42).ok_or(msg) = {:?}", result);

        // None to Result
        let option: Option<i32> = None;
        let result: Result<i32, &str> = option.ok_or("No value");
        println!("   None.ok_or(msg) = {:?}", result);
    }

    convert_examples();

    // Cleanup test files
    cleanup_test_files();

    println!();
}

/// Cleanup function to remove test files
fn cleanup_test_files() {
    let files_to_remove = vec![
        "username.txt",
        "config.txt",
        "output.txt",
        "valid_number.txt",
        "zero_number.txt",
        "large_number.txt",
        "invalid_number.txt",
        "numbers.txt",
    ];

    for file in files_to_remove {
        let _ = std::fs::remove_file(file);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_mark_with_result() {
        fn might_fail(should_fail: bool) -> Result<i32, String> {
            if should_fail {
                Err("Failed".to_string())
            } else {
                Ok(42)
            }
        }

        fn chain_operations(should_fail: bool) -> Result<i32, String> {
            let value = might_fail(should_fail)?;
            Ok(value * 2)
        }

        assert_eq!(chain_operations(false), Ok(84));
        assert_eq!(chain_operations(true), Err("Failed".to_string()));
    }

    #[test]
    fn test_question_mark_with_option() {
        fn get_first_char(s: &str) -> Option<char> {
            s.chars().next()
        }

        fn process_string(s: &str) -> Option<String> {
            let first_char = get_first_char(s)?;
            Some(format!("First char: {}", first_char))
        }

        assert_eq!(process_string("hello"), Some("First char: h".to_string()));
        assert_eq!(process_string(""), None);
    }

    #[test]
    fn test_custom_error_conversion() {
        fn parse_number(s: &str) -> Result<i32, ProcessingError> {
            let num: i32 = s
                .parse()
                .map_err(|_| ProcessingError::Parse("Invalid number".to_string()))?;
            Ok(num)
        }

        assert!(parse_number("42").is_ok());
        assert_eq!(parse_number("42").unwrap(), 42);
        assert!(matches!(
            parse_number("abc"),
            Err(ProcessingError::Parse(_))
        ));
    }
}
