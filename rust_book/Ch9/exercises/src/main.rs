use std::collections::HashMap;
use std::error::Error;
use std::fmt;
/// # Chapter 9 - Error Handling Exercises
///
/// Comprehensive practice problems covering all aspects of error handling in Rust.
///
/// ## Exercise Categories:
/// 1. Basic Result and Option handling
/// 2. Error propagation with ? operator
/// 3. Custom error types and conversions
/// 4. Real-world error handling scenarios
/// 5. Advanced error patterns and best practices
///
/// ## Instructions:
/// - Run with: cargo run
/// - Run tests with: cargo test
/// - Each exercise builds on previous concepts
/// - Solutions are provided with detailed explanations
use std::io;

fn main() {
    println!("=== Chapter 9: Error Handling Exercises ===\n");

    // Exercise 1: Basic error handling
    exercise_1_basic_result_handling();

    // Exercise 2: File operations with error handling
    exercise_2_file_operations();

    // Exercise 3: Custom error types
    exercise_3_custom_errors();

    // Exercise 4: Error propagation chains
    exercise_4_error_propagation();

    // Exercise 5: Configuration parser (real-world scenario)
    exercise_5_config_parser();

    // Exercise 6: Advanced patterns
    exercise_6_advanced_patterns();
}

/// Exercise 1: Basic Result and Option Handling
///
/// Practice fundamental error handling patterns
fn exercise_1_basic_result_handling() {
    println!("📚 Exercise 1: Basic Result and Option Handling");
    println!("===============================================\n");

    // Problem 1: Safe division function
    println!("Problem 1: Safe Division");

    fn safe_divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err("Cannot divide by zero".to_string())
        } else if denominator.is_nan() || numerator.is_nan() {
            Err("Cannot perform division with NaN".to_string())
        } else {
            Ok(numerator / denominator)
        }
    }

    let test_cases = vec![(10.0, 2.0), (10.0, 0.0), (0.0, 5.0), (f64::NAN, 2.0)];

    for (num, den) in test_cases {
        match safe_divide(num, den) {
            Ok(result) => println!("  {} ÷ {} = {:.2}", num, den, result),
            Err(msg) => println!("  {} ÷ {} -> Error: {}", num, den, msg),
        }
    }

    println!();

    // Problem 2: Safe array access
    println!("Problem 2: Safe Array Access");

    fn get_element_with_bounds_check<T: Clone>(arr: &[T], index: usize) -> Option<T> {
        if index < arr.len() {
            Some(arr[index].clone())
        } else {
            None
        }
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let test_indices = vec![0, 2, 5, 10];

    for index in test_indices {
        match get_element_with_bounds_check(&numbers, index) {
            Some(value) => println!("  numbers[{}] = {}", index, value),
            None => println!("  numbers[{}] = None (out of bounds)", index),
        }
    }

    println!();

    // Problem 3: Chain of Option operations
    println!("Problem 3: Option Chaining");

    fn process_user_id(input: &str) -> Option<String> {
        let id: u32 = input.parse().ok()?; // Convert Result to Option
        let valid_id = if id > 0 && id < 10000 { Some(id) } else { None }?;
        Some(format!("USER_{:04}", valid_id))
    }

    let test_ids = vec!["123", "0", "12345", "abc"];
    for input in test_ids {
        match process_user_id(input) {
            Some(user_id) => println!("  '{}' -> {}", input, user_id),
            None => println!("  '{}' -> Invalid ID", input),
        }
    }

    println!("\n");
}

/// Exercise 2: File Operations with Error Handling
///
/// Practice real-world file I/O error scenarios
fn exercise_2_file_operations() {
    println!("📁 Exercise 2: File Operations with Error Handling");
    println!("==================================================\n");

    // Problem 1: Safe file reading
    fn read_file_safe(filename: &str) -> Result<String, String> {
        std::fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read '{}': {}", filename, e))
    }

    // Create test file
    let test_content = "Hello, World!\nThis is a test file.\nLine 3 here.";
    if let Err(e) = std::fs::write("test_file.txt", test_content) {
        println!("⚠️  Failed to create test file: {}", e);
        return;
    }

    println!("Problem 1: Safe File Reading");
    match read_file_safe("test_file.txt") {
        Ok(content) => println!(
            "  ✅ File content ({} chars): {}",
            content.len(),
            content.lines().next().unwrap_or("")
        ),
        Err(msg) => println!("  ❌ {}", msg),
    }

    match read_file_safe("nonexistent.txt") {
        Ok(content) => println!("  Content: {}", content),
        Err(msg) => println!("  ❌ {}", msg),
    }

    println!();

    // Problem 2: File backup with error handling
    println!("Problem 2: File Backup Operation");

    fn backup_file(original: &str, backup: &str) -> Result<usize, io::Error> {
        let content = std::fs::read(original)?;
        std::fs::write(backup, &content)?;
        Ok(content.len())
    }

    match backup_file("test_file.txt", "test_file_backup.txt") {
        Ok(bytes) => println!("  ✅ Backed up {} bytes", bytes),
        Err(e) => println!("  ❌ Backup failed: {}", e),
    }

    // Problem 3: File validation
    println!("Problem 3: File Validation");

    fn validate_file_contents(filename: &str) -> Result<(usize, usize), String> {
        let content =
            std::fs::read_to_string(filename).map_err(|e| format!("Cannot read file: {}", e))?;

        if content.is_empty() {
            return Err("File is empty".to_string());
        }

        let line_count = content.lines().count();
        let word_count = content.split_whitespace().count();

        if line_count == 0 {
            Err("No lines found".to_string())
        } else {
            Ok((line_count, word_count))
        }
    }

    match validate_file_contents("test_file.txt") {
        Ok((lines, words)) => println!("  ✅ File stats: {} lines, {} words", lines, words),
        Err(msg) => println!("  ❌ Validation failed: {}", msg),
    }

    println!("\n");
}

/// Custom error types for exercise 3
#[derive(Debug, Clone, PartialEq)]
enum CalculatorError {
    DivisionByZero,
    #[allow(dead_code)]
    InvalidOperation(String),
    Overflow,
    UnderFlow,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "Division by zero"),
            CalculatorError::InvalidOperation(op) => write!(f, "Invalid operation: {}", op),
            CalculatorError::Overflow => write!(f, "Mathematical overflow"),
            CalculatorError::UnderFlow => write!(f, "Mathematical underflow"),
        }
    }
}

impl Error for CalculatorError {}

/// Calculator struct for error handling exercises
struct Calculator;

/// Exercise 3: Custom Error Types
///
/// Practice designing and using custom error types
fn exercise_3_custom_errors() {
    println!("🔧 Exercise 3: Custom Error Types");
    println!("=================================\n");

    // Problem 1: Calculator with custom errors
    println!("Problem 1: Calculator with Custom Errors");


    type CalculatorTest = (&'static str, Box<dyn Fn() -> Result<f64, CalculatorError>>);
    let calc_tests: Vec<CalculatorTest> = vec![
        ("10 ÷ 2", Box::new(|| Calculator::divide(10.0, 2.0))),
        ("10 ÷ 0", Box::new(|| Calculator::divide(10.0, 0.0))),
        ("2^10", Box::new(|| Calculator::power(2.0, 10.0))),
        ("2^1000", Box::new(|| Calculator::power(2.0, 1000.0))),
        ("0^(-1)", Box::new(|| Calculator::power(0.0, -1.0))),
    ];

    for (desc, operation) in calc_tests {
        match operation() {
            Ok(result) => println!("  {} = {:.2}", desc, result),
            Err(error) => println!("  {} -> Error: {}", desc, error),
        }
    }

    println!();

    // Problem 2: Error conversion chain
    println!("Problem 2: Error Conversion Chain");

    #[derive(Debug)]
    enum AppError {
        Io(io::Error),
        Calculator(CalculatorError),
        Parse(String),
    }

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                AppError::Io(err) => write!(f, "IO error: {}", err),
                AppError::Calculator(err) => write!(f, "Calculator error: {}", err),
                AppError::Parse(msg) => write!(f, "Parse error: {}", msg),
            }
        }
    }

    impl Error for AppError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                AppError::Io(err) => Some(err),
                AppError::Calculator(err) => Some(err),
                AppError::Parse(_) => None,
            }
        }
    }

    impl From<io::Error> for AppError {
        fn from(err: io::Error) -> Self {
            AppError::Io(err)
        }
    }

    impl From<CalculatorError> for AppError {
        fn from(err: CalculatorError) -> Self {
            AppError::Calculator(err)
        }
    }

    fn process_calculation_from_file(filename: &str) -> Result<f64, AppError> {
        let content = std::fs::read_to_string(filename)?; // Auto-converts io::Error

        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(AppError::Parse("Expected two numbers".to_string()));
        }

        let a: f64 = parts[0]
            .parse()
            .map_err(|_| AppError::Parse(format!("Invalid first number: {}", parts[0])))?;
        let b: f64 = parts[1]
            .parse()
            .map_err(|_| AppError::Parse(format!("Invalid second number: {}", parts[1])))?;

        Calculator::divide(a, b).map_err(AppError::from) // Auto-converts CalculatorError
    }

    // Create test file for calculation
    let _ = std::fs::write("calc_input.txt", "20.0 4.0");

    match process_calculation_from_file("calc_input.txt") {
        Ok(result) => println!("  Calculation result: {}", result),
        Err(error) => {
            println!("  ❌ {}", error);
            if let Some(source) = error.source() {
                println!("    Caused by: {}", source);
            }
        }
    }

    println!("\n");
}

impl Calculator {
    fn divide(a: f64, b: f64) -> Result<f64, CalculatorError> {
        if b == 0.0 {
            Err(CalculatorError::DivisionByZero)
        } else if a.is_infinite() || b.is_infinite() {
            Err(CalculatorError::Overflow)
        } else {
            let result = a / b;
            if result.is_infinite() {
                Err(CalculatorError::Overflow)
            } else {
                Ok(result)
            }
        }
    }

    fn power(base: f64, exp: f64) -> Result<f64, CalculatorError> {
        if base == 0.0 && exp < 0.0 {
            Err(CalculatorError::DivisionByZero)
        } else {
            let result = base.powf(exp);
            if result.is_infinite() {
                Err(CalculatorError::Overflow)
            } else if result == 0.0 && base != 0.0 {
                Err(CalculatorError::UnderFlow)
            } else {
                Ok(result)
            }
        }
    }
}

/// Exercise 4: Error Propagation Chains
///
/// Practice complex error propagation scenarios
fn exercise_4_error_propagation() {
    println!("🔗 Exercise 4: Error Propagation Chains");
    println!("======================================\n");

    // Problem 1: Multi-step file processing
    println!("Problem 1: Multi-Step File Processing");

    fn process_numbers_file(filename: &str) -> Result<(i32, f64), String> {
        let content =
            std::fs::read_to_string(filename).map_err(|e| format!("Cannot read file: {}", e))?;

        let mut numbers = Vec::new();
        for (line_num, line) in content.lines().enumerate() {
            let num: i32 = line
                .trim()
                .parse()
                .map_err(|_| format!("Invalid number on line {}: '{}'", line_num + 1, line))?;
            numbers.push(num);
        }

        if numbers.is_empty() {
            return Err("No numbers found in file".to_string());
        }

        let sum: i32 = numbers.iter().sum();
        let average = sum as f64 / numbers.len() as f64;

        Ok((sum, average))
    }

    // Create test file with numbers
    let test_numbers = "10\n20\n30\n40\n50";
    let _ = std::fs::write("numbers.txt", test_numbers);

    match process_numbers_file("numbers.txt") {
        Ok((sum, avg)) => println!("  ✅ Sum: {}, Average: {:.2}", sum, avg),
        Err(msg) => println!("  ❌ {}", msg),
    }

    // Test with invalid file
    let invalid_numbers = "10\n20\nabc\n40";
    let _ = std::fs::write("invalid_numbers.txt", invalid_numbers);

    match process_numbers_file("invalid_numbers.txt") {
        Ok((sum, avg)) => println!("  Sum: {}, Average: {:.2}", sum, avg),
        Err(msg) => println!("  ❌ {}", msg),
    }

    println!();

    // Problem 2: Nested Result operations
    println!("Problem 2: Nested Result Operations");

    fn validate_and_process_config() -> Result<String, String> {
        let config_data = std::fs::read_to_string("app_config.txt")
            .map_err(|e| format!("Config file error: {}", e))?;

        let mut config = HashMap::new();
        for (line_num, line) in config_data.lines().enumerate() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue; // Skip empty lines and comments
            }

            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                return Err(format!(
                    "Invalid config format on line {}: '{}'",
                    line_num + 1,
                    line
                ));
            }

            config.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
        }

        // Validate required keys
        let required_keys = vec!["host", "port", "database"];
        for key in required_keys {
            if !config.contains_key(key) {
                return Err(format!("Missing required config key: {}", key));
            }
        }

        // Validate port is a number
        let port_str = config.get("port").unwrap();
        let _port: u16 = port_str
            .parse()
            .map_err(|_| format!("Invalid port number: {}", port_str))?;

        Ok(format!("Config valid: {} keys loaded", config.len()))
    }

    // Create valid config file
    let valid_config =
        "# Application Configuration\nhost=localhost\nport=8080\ndatabase=myapp\ndebug=true";
    let _ = std::fs::write("app_config.txt", valid_config);

    match validate_and_process_config() {
        Ok(msg) => println!("  ✅ {}", msg),
        Err(error) => println!("  ❌ {}", error),
    }

    println!("\n");
}

/// Exercise 5: Configuration Parser (Real-world Scenario)
///
/// Comprehensive error handling for a configuration parser
fn exercise_5_config_parser() {
    println!("⚙️ Exercise 5: Configuration Parser");
    println!("===================================\n");

    #[derive(Debug, Clone)]
    struct AppConfig {
        host: String,
        port: u16,
        database: String,
        #[allow(dead_code)]
        max_connections: u32,
        #[allow(dead_code)]
        timeout_seconds: u64,
        #[allow(dead_code)]
        debug_mode: bool,
    }

    #[derive(Debug)]
    enum ConfigError {
        FileNotFound(String),
        ParseError { line: usize, message: String },
        ValidationError(String),
        MissingField(String),
    }

    impl fmt::Display for ConfigError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ConfigError::FileNotFound(path) => write!(f, "Config file not found: {}", path),
                ConfigError::ParseError { line, message } => {
                    write!(f, "Parse error on line {}: {}", line, message)
                }
                ConfigError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
                ConfigError::MissingField(field) => write!(f, "Missing required field: {}", field),
            }
        }
    }

    impl Error for ConfigError {}

    impl AppConfig {
        fn from_file(filename: &str) -> Result<Self, ConfigError> {
            let content = std::fs::read_to_string(filename)
                .map_err(|_| ConfigError::FileNotFound(filename.to_string()))?;

            let mut config_map = HashMap::new();

            for (line_num, line) in content.lines().enumerate() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() != 2 {
                    return Err(ConfigError::ParseError {
                        line: line_num + 1,
                        message: format!("Expected key=value format, got: '{}'", line),
                    });
                }

                let key = parts[0].trim();
                let value = parts[1].trim();
                config_map.insert(key.to_string(), value.to_string());
            }

            // Helper function to get required field
            let get_required = |key: &str| -> Result<String, ConfigError> {
                config_map
                    .get(key)
                    .cloned()
                    .ok_or_else(|| ConfigError::MissingField(key.to_string()))
            };

            // Parse and validate all fields
            let host = get_required("host")?;

            let port_str = get_required("port")?;
            let port: u16 = port_str
                .parse()
                .map_err(|_| ConfigError::ValidationError(format!("Invalid port: {}", port_str)))?;

            let database = get_required("database")?;

            let max_conn_str = get_required("max_connections")?;
            let max_connections: u32 = max_conn_str.parse().map_err(|_| {
                ConfigError::ValidationError(format!("Invalid max_connections: {}", max_conn_str))
            })?;

            let timeout_str = get_required("timeout_seconds")?;
            let timeout_seconds: u64 = timeout_str.parse().map_err(|_| {
                ConfigError::ValidationError(format!("Invalid timeout_seconds: {}", timeout_str))
            })?;

            let debug_str = config_map
                .get("debug_mode")
                .unwrap_or(&"false".to_string())
                .clone();
            let debug_mode: bool = debug_str.parse().map_err(|_| {
                ConfigError::ValidationError(format!("Invalid debug_mode: {}", debug_str))
            })?;

            // Business logic validation
            if port < 1024 {
                return Err(ConfigError::ValidationError(
                    "Port must be >= 1024".to_string(),
                ));
            }

            if max_connections == 0 || max_connections > 10000 {
                return Err(ConfigError::ValidationError(
                    "max_connections must be 1-10000".to_string(),
                ));
            }

            if timeout_seconds == 0 || timeout_seconds > 3600 {
                return Err(ConfigError::ValidationError(
                    "timeout_seconds must be 1-3600".to_string(),
                ));
            }

            Ok(AppConfig {
                host,
                port,
                database,
                max_connections,
                timeout_seconds,
                debug_mode,
            })
        }
    }

    // Test valid configuration
    let valid_config = r#"# Database Configuration
host=localhost
port=5432
database=myapp_db
max_connections=100
timeout_seconds=30
debug_mode=true"#;

    let _ = std::fs::write("valid_app_config.txt", valid_config);

    match AppConfig::from_file("valid_app_config.txt") {
        Ok(config) => println!(
            "  ✅ Valid config loaded: {}@{}:{}",
            config.database, config.host, config.port
        ),
        Err(error) => println!("  ❌ Config error: {}", error),
    }

    // Test invalid configuration
    let invalid_config = r#"host=localhost
port=abc
database=myapp_db
max_connections=100"#;

    let _ = std::fs::write("invalid_app_config.txt", invalid_config);

    match AppConfig::from_file("invalid_app_config.txt") {
        Ok(config) => println!("  Config loaded: {:?}", config),
        Err(error) => println!("  ❌ Expected error: {}", error),
    }

    println!("\n");
}

/// Exercise 6: Advanced Error Handling Patterns
///
/// Advanced techniques and best practices
fn exercise_6_advanced_patterns() {
    println!("🎯 Exercise 6: Advanced Error Handling Patterns");
    println!("===============================================\n");

    // Problem 1: Error recovery with fallbacks
    println!("Problem 1: Error Recovery with Fallbacks");

    fn load_config_with_fallback(primary: &str, fallback: &str) -> Result<String, String> {
        std::fs::read_to_string(primary).or_else(|_| {
            println!(
                "    Primary config '{}' failed, trying fallback...",
                primary
            );
            std::fs::read_to_string(fallback)
                .map_err(|_| format!("Both '{}' and '{}' failed", primary, fallback))
        })
    }

    // Create fallback file
    let _ = std::fs::write("fallback_config.txt", "fallback=configuration");

    match load_config_with_fallback("missing_primary.txt", "fallback_config.txt") {
        Ok(content) => println!("  ✅ Config loaded: {}", content.trim()),
        Err(msg) => println!("  ❌ {}", msg),
    }

    println!();

    // Problem 2: Collecting multiple errors
    println!("Problem 2: Collecting Multiple Errors");

    fn validate_multiple_files(filenames: &[&str]) -> Result<Vec<usize>, Vec<String>> {
        let mut results = Vec::new();
        let mut errors = Vec::new();

        for filename in filenames {
            match std::fs::metadata(filename) {
                Ok(metadata) => results.push(metadata.len() as usize),
                Err(e) => errors.push(format!("'{}': {}", filename, e)),
            }
        }

        if errors.is_empty() {
            Ok(results)
        } else {
            Err(errors)
        }
    }

    let test_files = vec![
        "test_file.txt",
        "nonexistent1.txt",
        "fallback_config.txt",
        "nonexistent2.txt",
    ];
    match validate_multiple_files(&test_files) {
        Ok(sizes) => println!("  ✅ All files valid, sizes: {:?}", sizes),
        Err(errors) => {
            println!("  ❌ Some files failed:");
            for error in errors {
                println!("    - {}", error);
            }
        }
    }

    println!();

    // Problem 3: Timeout and retry patterns
    println!("Problem 3: Retry Pattern with Error Accumulation");

    fn simulate_unreliable_operation(attempt: u32) -> Result<String, String> {
        if attempt <= 2 {
            Err(format!("Temporary failure on attempt {}", attempt))
        } else {
            Ok("Success after retries".to_string())
        }
    }

    fn retry_with_backoff<T, E, F>(mut operation: F, max_attempts: u32) -> Result<T, Vec<E>>
    where
        F: FnMut(u32) -> Result<T, E>,
    {
        let mut errors = Vec::new();

        for attempt in 1..=max_attempts {
            match operation(attempt) {
                Ok(result) => return Ok(result),
                Err(error) => {
                    errors.push(error);
                    if attempt < max_attempts {
                        println!("    Attempt {} failed, retrying...", attempt);
                        // In real code, you might add std::thread::sleep here
                    }
                }
            }
        }

        Err(errors)
    }

    match retry_with_backoff(simulate_unreliable_operation, 3) {
        Ok(result) => println!("  ✅ Operation succeeded: {}", result),
        Err(errors) => {
            println!("  ❌ All attempts failed:");
            for (i, error) in errors.iter().enumerate() {
                println!("    Attempt {}: {}", i + 1, error);
            }
        }
    }

    // Cleanup test files
    cleanup_exercise_files();

    println!("\n🎉 All exercises completed!");
}

/// Cleanup function to remove test files
fn cleanup_exercise_files() {
    let files_to_remove = vec![
        "test_file.txt",
        "test_file_backup.txt",
        "calc_input.txt",
        "numbers.txt",
        "invalid_numbers.txt",
        "app_config.txt",
        "valid_app_config.txt",
        "invalid_app_config.txt",
        "fallback_config.txt",
    ];

    for file in files_to_remove {
        let _ = std::fs::remove_file(file);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide() {
        fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
            if b == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(a / b)
            }
        }

        assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
        assert_eq!(safe_divide(10.0, 0.0), Err("Division by zero".to_string()));
    }

    #[test]
    fn test_option_chaining() {
        fn get_first_even(numbers: &[i32]) -> Option<i32> {
            let first = numbers.get(0)?;
            if first % 2 == 0 {
                Some(*first)
            } else {
                None
            }
        }

        assert_eq!(get_first_even(&[2, 3, 4]), Some(2));
        assert_eq!(get_first_even(&[1, 3, 5]), None);
        assert_eq!(get_first_even(&[]), None);
    }

    #[test]
    fn test_calculator_errors() {
        assert_eq!(Calculator::divide(10.0, 2.0), Ok(5.0));
        assert_eq!(
            Calculator::divide(10.0, 0.0),
            Err(CalculatorError::DivisionByZero)
        );
        assert!(matches!(
            Calculator::power(2.0, 1000.0),
            Err(CalculatorError::Overflow)
        ));
    }

    #[test]
    fn test_error_propagation() {
        fn parse_and_double(s: &str) -> Result<i32, String> {
            let num: i32 = s.parse().map_err(|_| "Parse error".to_string())?;
            Ok(num * 2)
        }

        assert_eq!(parse_and_double("21"), Ok(42));
        assert_eq!(parse_and_double("abc"), Err("Parse error".to_string()));
    }

    #[test]
    fn test_retry_pattern() {
        fn retry_operation<T, E, F>(mut op: F, max_attempts: u32) -> Result<T, Vec<E>>
        where
            F: FnMut() -> Result<T, E>,
        {
            let mut errors = Vec::new();
            for _ in 0..max_attempts {
                match op() {
                    Ok(result) => return Ok(result),
                    Err(e) => errors.push(e),
                }
            }
            Err(errors)
        }

        let mut counter = 0;
        let operation = || {
            counter += 1;
            if counter < 3 {
                Err("Temporary failure")
            } else {
                Ok("Success")
            }
        };

        assert_eq!(retry_operation(operation, 3), Ok("Success"));
    }
}
