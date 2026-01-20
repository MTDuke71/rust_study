//! # Error Handling Review - Comprehensive Examples
//!
//! **Week 4 Review**: All Error Handling Concepts
//!
//! This file demonstrates all error handling patterns covered in Chapter 4:
//! 1. Enumeration errors (domain-specific)
//! 2. Opaque errors (type erasure)
//! 3. Special cases (unit errors, never type)
//! 4. Error propagation (?, From trait)
//! 5. Custom error types
//! 6. Production patterns (thiserror, anyhow)

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

// ============================================================================
// SCENARIO: File Processing Pipeline
// ============================================================================

// 1. ENUMERATION ERROR - Domain-specific error for file operations
#[derive(Debug)]
enum FileError {
    NotFound(String),
    PermissionDenied(String),
    Io(io::Error),
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::NotFound(path) => write!(f, "file not found: {}", path),
            FileError::PermissionDenied(path) => write!(f, "permission denied: {}", path),
            FileError::Io(e) => write!(f, "io error: {}", e),
        }
    }
}

impl Error for FileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FileError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for FileError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => FileError::NotFound("unknown".to_string()),
            io::ErrorKind::PermissionDenied => FileError::PermissionDenied("unknown".to_string()),
            _ => FileError::Io(err),
        }
    }
}

// 2. CUSTOM ERROR - Parse error with context
#[derive(Debug)]
struct ParseError {
    line: usize,
    column: usize,
    kind: ParseErrorKind,
}

#[derive(Debug)]
enum ParseErrorKind {
    InvalidNumber(ParseIntError),
    InvalidFormat(String),
    EmptyLine,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error at {}:{}: ", self.line, self.column)?;
        match &self.kind {
            ParseErrorKind::InvalidNumber(e) => write!(f, "invalid number: {}", e),
            ParseErrorKind::InvalidFormat(msg) => write!(f, "invalid format: {}", msg),
            ParseErrorKind::EmptyLine => write!(f, "empty line"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            ParseErrorKind::InvalidNumber(e) => Some(e),
            _ => None,
        }
    }
}

// 3. TOP-LEVEL ERROR - Combines file and parse errors
#[derive(Debug)]
enum ProcessError {
    File(FileError),
    Parse(ParseError),
    Validation(String),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessError::File(e) => write!(f, "file error: {}", e),
            ProcessError::Parse(e) => write!(f, "parse error: {}", e),
            ProcessError::Validation(msg) => write!(f, "validation error: {}", msg),
        }
    }
}

impl Error for ProcessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ProcessError::File(e) => Some(e),
            ProcessError::Parse(e) => Some(e),
            ProcessError::Validation(_) => None,
        }
    }
}

impl From<FileError> for ProcessError {
    fn from(err: FileError) -> Self {
        ProcessError::File(err)
    }
}

impl From<ParseError> for ProcessError {
    fn from(err: ParseError) -> Self {
        ProcessError::Parse(err)
    }
}

impl From<io::Error> for ProcessError {
    fn from(err: io::Error) -> Self {
        ProcessError::File(FileError::from(err))
    }
}

// ============================================================================
// DEMONSTRATION: Complete Processing Pipeline
// ============================================================================

/// Step 1: Read file (can fail with FileError)
fn read_file(path: &str) -> Result<String, FileError> {
    fs::read_to_string(path)
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => FileError::NotFound(path.to_string()),
            io::ErrorKind::PermissionDenied => FileError::PermissionDenied(path.to_string()),
            _ => FileError::Io(e),
        })
}

/// Step 2: Parse lines (can fail with ParseError)
fn parse_numbers(content: &str) -> Result<Vec<i32>, ParseError> {
    let mut numbers = Vec::new();
    
    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            return Err(ParseError {
                line: line_idx + 1,
                column: 1,
                kind: ParseErrorKind::EmptyLine,
            });
        }
        
        match trimmed.parse::<i32>() {
            Ok(n) => numbers.push(n),
            Err(e) => {
                return Err(ParseError {
                    line: line_idx + 1,
                    column: 1,
                    kind: ParseErrorKind::InvalidNumber(e),
                });
            }
        }
    }
    
    Ok(numbers)
}

/// Step 3: Validate (can fail with validation error)
fn validate_numbers(numbers: &[i32]) -> Result<(), String> {
    if numbers.is_empty() {
        return Err("no numbers found".to_string());
    }
    
    if numbers.iter().any(|&n| n < 0) {
        return Err("negative numbers not allowed".to_string());
    }
    
    Ok(())
}

/// Complete pipeline using error propagation
fn process_file(path: &str) -> Result<Vec<i32>, ProcessError> {
    // Read file - io::Error -> FileError -> ProcessError
    let content = read_file(path)?;
    
    // Parse content - ParseError -> ProcessError
    let numbers = parse_numbers(&content)?;
    
    // Validate - String -> ProcessError
    validate_numbers(&numbers)
        .map_err(ProcessError::Validation)?;
    
    Ok(numbers)
}

// ============================================================================
// OPAQUE ERROR VERSION - Simpler but less specific
// ============================================================================

fn process_file_opaque(path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let numbers: Result<Vec<i32>, _> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse())
        .collect();
    
    let numbers = numbers?;
    
    if numbers.is_empty() {
        return Err("no numbers found".into());
    }
    
    Ok(numbers)
}

// ============================================================================
// SPECIAL CASES
// ============================================================================

// Unit error - validation without details
fn validate_positive(n: i32) -> Result<i32, ()> {
    if n > 0 {
        Ok(n)
    } else {
        Err(())
    }
}

// Result<(), E> - side effect operation
fn log_result(numbers: &[i32]) -> Result<(), io::Error> {
    fs::write("output.log", format!("{:?}", numbers))
}

// ============================================================================
// ERROR CHAIN TRAVERSAL
// ============================================================================

fn print_error_chain(err: &dyn Error) {
    eprintln!("Error: {}", err);
    
    let mut current = err.source();
    let mut level = 1;
    
    while let Some(source) = current {
        eprintln!("  {}: Caused by: {}", level, source);
        current = source.source();
        level += 1;
    }
}

// ============================================================================
// MAIN DEMONSTRATION
// ============================================================================

fn main() {
    println!("=== Error Handling Review ===\n");

    // Test data
    let test_files = vec![
        ("numbers.txt", "1\n2\n3\n4\n5"),
        ("numbers_with_error.txt", "1\n2\ninvalid\n4"),
        ("numbers_negative.txt", "1\n2\n-3\n4"),
    ];

    // Create test files
    for (name, content) in &test_files {
        let _ = fs::write(name, content);
    }

    println!("1. Specific Error Types (Enumeration):");
    match process_file("numbers.txt") {
        Ok(numbers) => println!("   Success: {:?}", numbers),
        Err(e) => {
            println!("   Error occurred:");
            print_error_chain(&e);
        }
    }
    println!();

    println!("2. Error with Parse Failure:");
    match process_file("numbers_with_error.txt") {
        Ok(numbers) => println!("   Success: {:?}", numbers),
        Err(e) => {
            println!("   Error occurred:");
            print_error_chain(&e);
        }
    }
    println!();

    println!("3. Error with Validation Failure:");
    match process_file("numbers_negative.txt") {
        Ok(numbers) => println!("   Success: {:?}", numbers),
        Err(e) => {
            println!("   Error occurred:");
            print_error_chain(&e);
        }
    }
    println!();

    println!("4. Opaque Error Version:");
    match process_file_opaque("numbers.txt") {
        Ok(numbers) => println!("   Success: {:?}", numbers),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    println!("5. Special Cases:");
    match validate_positive(42) {
        Ok(n) => println!("   Valid: {}", n),
        Err(()) => println!("   Invalid"),
    }
    match validate_positive(-5) {
        Ok(n) => println!("   Valid: {}", n),
        Err(()) => println!("   Invalid"),
    }
    println!();

    println!("6. Side Effect with Result<(), E>:");
    match log_result(&[1, 2, 3]) {
        Ok(()) => println!("   Logged successfully"),
        Err(e) => println!("   Failed to log: {}", e),
    }
    println!();

    // Cleanup
    for (name, _) in &test_files {
        let _ = fs::remove_file(name);
    }
    let _ = fs::remove_file("output.log");

    println!("Summary:");
    println!("  ✓ Enumeration errors for exhaustive error handling");
    println!("  ✓ Opaque errors for simplified interfaces");
    println!("  ✓ Unit errors for simple validation");
    println!("  ✓ Error propagation with ? operator");
    println!("  ✓ Error chains with source() method");
    println!("  ✓ Custom error types with context");
}
