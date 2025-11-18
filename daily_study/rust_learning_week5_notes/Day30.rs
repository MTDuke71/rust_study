// Day 30 - Error Propagation
// Runnable examples demonstrating error propagation with the ? operator

#![allow(dead_code, unused_mut)]

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

// Custom error types for examples
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::Custom(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}

impl Error for AppError {}

// Enable automatic conversion from std::io::Error
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::Io(error)
    }
}

// Enable automatic conversion from ParseIntError
impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::Parse(error)
    }
}

// Web Error for web request examples
#[derive(Debug)]
#[allow(dead_code)] // Demo types for learning purposes
enum WebError {
    Network(String),
    Parse(String),
    Http(u16),
    Auth(String),
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebError::Network(msg) => write!(f, "Network error: {}", msg),
            WebError::Parse(msg) => write!(f, "Parse error: {}", msg),
            WebError::Http(code) => write!(f, "HTTP error: {}", code),
            WebError::Auth(msg) => write!(f, "Auth error: {}", msg),
        }
    }
}

impl Error for WebError {}

// Database Error for transaction examples
#[derive(Debug)]
#[allow(dead_code)] // Demo types for learning purposes
enum DatabaseError {
    Connection(String),
    Query(String),
    Transaction(String),
    Constraint(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::Connection(msg) => write!(f, "Connection error: {}", msg),
            DatabaseError::Query(msg) => write!(f, "Query error: {}", msg),
            DatabaseError::Transaction(msg) => write!(f, "Transaction error: {}", msg),
            DatabaseError::Constraint(msg) => write!(f, "Constraint error: {}", msg),
        }
    }
}

impl Error for DatabaseError {}

// Data structures
#[derive(Debug)]
#[allow(dead_code)] // Demo types for learning purposes
struct Config {
    port: u16,
}

#[derive(Debug)]
#[allow(dead_code)] // Demo types for learning purposes
struct UserData {
    id: u32,
    name: String,
}

#[derive(Debug)]
#[allow(dead_code)] // Demo types for learning purposes
struct UserProfile {
    user_data: UserData,
    preferences: HashMap<String, String>,
}

#[derive(Debug)]
#[allow(dead_code)] // Demo types for learning purposes
struct ProcessedFile {
    path: String,
    content: String,
}

#[derive(Debug)]
#[allow(dead_code)] // Demo types for learning purposes
struct QueryResult {
    rows_affected: usize,
}

// Example functions demonstrating error propagation

fn parse_config(content: &str) -> Result<Config, AppError> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Err(AppError::Custom("Empty config file".to_string()));
    }

    let port = lines[0].parse::<u16>()?;
    Ok(Config { port })
}

fn process_line(line: &str) -> Result<(), AppError> {
    if line.is_empty() {
        return Ok(());
    }

    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 2 {
        return Err(AppError::Custom("Invalid format".to_string()));
    }

    let _id = parts[0].parse::<u32>()?;
    let _name = parts[1].to_string();

    Ok(())
}

fn process_user_input(input: &str) -> Result<UserData, AppError> {
    let lines: Vec<&str> = input.lines().collect();

    for (line_num, line) in lines.iter().enumerate() {
        if let Err(e) = process_line(line) {
            return Err(AppError::Custom(format!(
                "Error processing line {}: {}",
                line_num + 1,
                e
            )));
        }
    }

    Ok(UserData {
        id: 1,
        name: "John Doe".to_string(),
    })
}

fn robust_file_processor(path: &str) -> Result<Vec<String>, AppError> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Handle missing file gracefully
            println!("Warning: File {} not found, using defaults", path);
            return Ok(vec!["default".to_string()]);
        }
        Err(e) => return Err(AppError::from(e)), // Propagate other errors
    };

    let lines: Vec<String> = content
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                None
            } else {
                Some(line.to_string())
            }
        })
        .collect();

    Ok(lines)
}

fn process_single_file(path: &str) -> Result<ProcessedFile, AppError> {
    let content = std::fs::read_to_string(path)?;
    let processed = content.to_uppercase();
    Ok(ProcessedFile {
        path: path.to_string(),
        content: processed,
    })
}

fn batch_process_files(paths: &[String]) -> Result<Vec<ProcessedFile>, AppError> {
    let mut results = Vec::new();
    let mut errors = Vec::new();

    for path in paths {
        match process_single_file(path) {
            Ok(result) => results.push(result),
            Err(e) => {
                errors.push(format!("Failed to process {}: {}", path, e));
                // Continue processing other files
            }
        }
    }

    if !errors.is_empty() {
        return Err(AppError::Custom(format!(
            "Batch processing failed with {} errors: {}",
            errors.len(),
            errors.join("; ")
        )));
    }

    Ok(results)
}

// Web request simulation
fn make_request(url: &str) -> Result<String, WebError> {
    // Simulate network request
    if url.contains("unauthorized") {
        return Err(WebError::Auth("Invalid credentials".to_string()));
    }

    if url.contains("error") {
        return Err(WebError::Http(500));
    }

    Ok(r#"{"status": "success", "data": {"user_id": 123}}"#.to_string())
}

fn parse_json_response(response: &str) -> Result<HashMap<String, serde_json::Value>, WebError> {
    let parsed: HashMap<String, serde_json::Value> = serde_json::from_str(response)
        .map_err(|e| WebError::Parse(format!("JSON parse error: {}", e)))?;
    Ok(parsed)
}

fn fetch_user_data(user_id: u32) -> Result<UserData, WebError> {
    let url = format!("https://api.example.com/users/{}", user_id);
    let response = make_request(&url)?;
    let data = parse_json_response(&response)?;

    let user_id = data
        .get("data")
        .and_then(|d| d.get("user_id"))
        .and_then(|v| v.as_u64())
        .ok_or_else(|| WebError::Parse("Missing user_id".to_string()))?;

    Ok(UserData {
        id: user_id as u32,
        name: "John Doe".to_string(),
    })
}

// Database simulation
struct Database;

impl Database {
    fn connect() -> Result<Connection, DatabaseError> {
        // Simulate connection
        Ok(Connection)
    }
}

struct Connection;

impl Connection {
    fn begin_transaction(&self) -> Result<Transaction, DatabaseError> {
        // Simulate transaction start
        Ok(Transaction)
    }
}

struct Transaction;

impl Transaction {
    fn execute(&self, query: &str) -> Result<QueryResult, DatabaseError> {
        if query.contains("CONSTRAINT") {
            return Err(DatabaseError::Constraint(
                "Unique constraint violated".to_string(),
            ));
        }

        if query.is_empty() {
            return Err(DatabaseError::Query("Empty query".to_string()));
        }

        Ok(QueryResult { rows_affected: 1 })
    }

    fn commit(self) -> Result<(), DatabaseError> {
        // Simulate commit
        Ok(())
    }
}

fn transfer_money(from_account: u32, to_account: u32, amount: i32) -> Result<(), DatabaseError> {
    let db = Database::connect()?;
    let tx = db.begin_transaction()?;

    // Debit from source account
    tx.execute(&format!(
        "UPDATE accounts SET balance = balance - {} WHERE id = {}",
        amount, from_account
    ))?;

    // Credit to destination account
    tx.execute(&format!(
        "UPDATE accounts SET balance = balance + {} WHERE id = {}",
        amount, to_account
    ))?;

    // Commit transaction
    tx.commit()?;

    Ok(())
}

fn load_user_profile(user_id: u32) -> Result<UserProfile, AppError> {
    let user_data = fetch_user_data(user_id)
        .map_err(|e| AppError::Custom(format!("Failed to fetch user {}: {}", user_id, e)))?;

    let mut preferences = HashMap::new();
    preferences.insert("theme".to_string(), "dark".to_string());

    Ok(UserProfile {
        user_data,
        preferences,
    })
}

fn main() {
    println!("=== Day 30: Error Propagation Examples ===\n");

    // Example 1: Basic error propagation
    println!("1. Basic Error Propagation:");
    let config_content = "8080\n";
    match parse_config(config_content) {
        Ok(config) => println!("Config loaded: {:?}", config),
        Err(e) => println!("Config error: {}", e),
    }
    println!();

    // Example 2: Context preservation
    println!("2. Error Context Preservation:");
    let user_input = "1,John\n2,Jane\ninvalid";
    match process_user_input(user_input) {
        Ok(user_data) => println!("User data processed: {:?}", user_data),
        Err(e) => println!("Processing error: {}", e),
    }
    println!();

    // Example 3: Error recovery
    println!("3. Error Recovery:");
    match robust_file_processor("nonexistent.txt") {
        Ok(lines) => println!("Processed lines: {:?}", lines),
        Err(e) => println!("Processing error: {}", e),
    }
    println!();

    // Example 4: Batch processing with error aggregation
    println!("4. Batch Processing:");
    let files = vec!["file1.txt".to_string(), "file2.txt".to_string()];
    match batch_process_files(&files) {
        Ok(results) => println!("Batch processing successful: {} files", results.len()),
        Err(e) => println!("Batch processing failed: {}", e),
    }
    println!();

    // Example 5: Web request chain
    println!("5. Web Request Chain:");
    match fetch_user_data(123) {
        Ok(user_data) => println!("User data fetched: {:?}", user_data),
        Err(e) => println!("Web request failed: {}", e),
    }

    // Simulate auth error
    match fetch_user_data(999) {
        Ok(user_data) => println!("User data fetched: {:?}", user_data),
        Err(e) => println!("Web request failed: {}", e),
    }
    println!();

    // Example 6: Database transaction
    println!("6. Database Transaction:");
    match transfer_money(1, 2, 100) {
        Ok(()) => println!("Money transfer successful"),
        Err(e) => println!("Transfer failed: {}", e),
    }

    // Simulate constraint violation
    match transfer_money(1, 2, -100) {
        Ok(()) => println!("Money transfer successful"),
        Err(e) => println!("Transfer failed: {}", e),
    }
    println!();

    // Example 7: Complex error propagation chain
    println!("7. Complex Error Propagation:");
    match load_user_profile(123) {
        Ok(profile) => println!("User profile loaded: {:?}", profile),
        Err(e) => println!("Profile loading failed: {}", e),
    }
    println!();

    println!("=== End of Day 30 Examples ===");
}
