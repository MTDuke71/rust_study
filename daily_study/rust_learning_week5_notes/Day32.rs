// Day 32 - Result Combinators
// Runnable examples demonstrating functional error handling methods

use std::collections::HashMap;

// Custom error types for examples
#[derive(Debug)]
#[allow(dead_code)]
enum ProcessError {
    ParseError(std::num::ParseIntError),
    EmptyInput,
    ValidationError(String),
    TransformError(String),
    ProcessingError,
}

#[derive(Debug)]
#[allow(dead_code)]
enum ConfigError {
    FileNotFound(String),
    EmptyFile(String),
    NoConfigFound,
}

#[derive(Debug)]
#[allow(dead_code)]
enum ApiError {
    InvalidId,
    NotFound,
    NetworkError,
    ParseError,
}

#[derive(Debug)]
#[allow(dead_code)]
enum PipelineError {
    FileNotFound(String),
    EmptyFile,
    InvalidFormat(String),
    ParseError(String),
    NoValidRecords,
    WriteError(String),
    ProcessingFailed(Box<PipelineError>),
}

#[derive(Debug)]
#[allow(dead_code)]
enum ValidationError {
    EmptyName,
    NameTooLong,
    InvalidEmail,
    TooYoung,
    TooOld,
}

// Data structures
#[derive(Debug)]
#[allow(dead_code)]
struct Config {
    host: String,
    port: u16,
}

impl Config {
    fn parse(content: &str) -> Result<Self, ConfigError> {
        let mut host = None;
        let mut port = None;
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((key, value)) = parse_key_value(line) {
                match key.as_str() {
                    "host" => host = Some(value),
                    "port" => {
                        port = Some(value.parse::<u16>()
                            .map_err(|_| ConfigError::FileNotFound("Invalid port".to_string()))?);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(Config {
            host: host.unwrap_or_else(|| "localhost".to_string()),
            port: port.unwrap_or(8080),
        })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct ProcessOptions {
    validate: bool,
    transform: bool,
    normalize: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
struct RawData {
    content: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct ProcessedData {
    content: String,
}

impl ProcessedData {
    fn from(raw: RawData) -> Self {
        Self { content: raw.content }
    }
    
    fn normalize(mut self) -> Self {
        self.content = self.content.to_lowercase();
        self
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct UserProfile {
    user: User,
    preferences: HashMap<String, String>,
    avatar_url: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct RawUser {
    name: String,
    email: String,
    age: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct ValidatedUser {
    name: String,
    email: String,
    age: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Record {
    id: String,
    name: String,
    value: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct ProcessedFile {
    input_path: String,
    output_path: String,
    size: u64,
}

#[derive(Debug)]
#[allow(dead_code)]
struct ParsedData {
    content: String,
}

// Helper functions
fn parse_key_value(line: &str) -> Option<(String, String)> {
    if let Some(pos) = line.find('=') {
        let key = line[..pos].trim().to_string();
        let value = line[pos + 1..].trim().to_string();
        Some((key, value))
    } else {
        None
    }
}

// Example functions demonstrating Result combinators

// Pattern 1: Chaining Transformations
fn process_numbers(input: &str) -> Result<Vec<i32>, ProcessError> {
    input
        .split(',')
        .map(|s| s.trim().parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .map_err(ProcessError::ParseError)
        .and_then(|numbers| {
            if numbers.is_empty() {
                Err(ProcessError::EmptyInput)
            } else {
                Ok(numbers)
            }
        })
        .map(|numbers| numbers.into_iter().map(|n| n * 2).collect())
}

// Pattern 2: Error Recovery
fn load_config_with_fallbacks(paths: &[String]) -> Result<Config, ConfigError> {
    let mut last_error = None;
    
    for path in paths {
        match load_single_config(path) {
            Ok(config) => return Ok(config),
            Err(e) => last_error = Some(e),
        }
    }
    
    Err(last_error.unwrap_or(ConfigError::NoConfigFound))
}

fn load_single_config(path: &str) -> Result<Config, ConfigError> {
    std::fs::read_to_string(path)
        .map_err(|_| ConfigError::FileNotFound(path.to_string()))
        .and_then(|content| {
            if content.trim().is_empty() {
                Err(ConfigError::EmptyFile(path.to_string()))
            } else {
                Ok(content)
            }
        })
        .and_then(|content| Config::parse(&content))
        .or_else(|e| {
            // Try to load default config as fallback
            load_default_config().map_err(|_| e)
        })
}

fn load_default_config() -> Result<Config, ConfigError> {
    Ok(Config {
        host: "localhost".to_string(),
        port: 8080,
    })
}

// Pattern 3: Conditional Processing
fn process_data_conditionally(data: &str, options: ProcessOptions) -> Result<ProcessedData, ProcessError> {
    parse_raw_data(data)
        .and_then(|raw| {
            if options.validate {
                validate_data(&raw).map(|_| raw)
            } else {
                Ok(raw)
            }
        })
        .and_then(|raw| {
            if options.transform {
                transform_data(raw)
            } else {
                Ok(ProcessedData::from(raw))
            }
        })
        .map(|processed| {
            if options.normalize {
                processed.normalize()
            } else {
                processed
            }
        })
}

fn parse_raw_data(data: &str) -> Result<RawData, ProcessError> {
    Ok(RawData { content: data.to_string() })
}

fn validate_data(raw: &RawData) -> Result<(), ProcessError> {
    if raw.content.is_empty() {
        Err(ProcessError::ValidationError("Empty data".to_string()))
    } else {
        Ok(())
    }
}

fn transform_data(raw: RawData) -> Result<ProcessedData, ProcessError> {
    Ok(ProcessedData { content: raw.content.to_uppercase() })
}

// Pattern 4: API Request Pipeline
fn fetch_user_profile(user_id: u32) -> Result<UserProfile, ApiError> {
    validate_user_id(user_id)
        .and_then(fetch_user_data)
        .and_then(|user| {
            fetch_user_preferences(user.id)
                .map(|prefs| (user, prefs))
        })
        .and_then(|(user, prefs)| {
            fetch_user_avatar(user.id)
                .map(|avatar| (user, prefs, avatar))
        })
        .map(|(user, prefs, avatar)| UserProfile {
            user,
            preferences: prefs,
            avatar_url: avatar,
        })
        .or_else(|e| {
            // Fallback to default profile
            create_default_profile(user_id).map_err(|_| e)
        })
}

fn validate_user_id(user_id: u32) -> Result<u32, ApiError> {
    if user_id == 0 {
        Err(ApiError::InvalidId)
    } else {
        Ok(user_id)
    }
}

fn fetch_user_data(user_id: u32) -> Result<User, ApiError> {
    // Simulate API call
    if user_id == 999 {
        Err(ApiError::NotFound)
    } else {
        Ok(User {
            id: user_id,
            name: format!("User {}", user_id),
            email: format!("user{}@example.com", user_id),
        })
    }
}

fn fetch_user_preferences(_user_id: u32) -> Result<HashMap<String, String>, ApiError> {
    // Simulate API call
    let mut prefs = HashMap::new();
    prefs.insert("theme".to_string(), "dark".to_string());
    prefs.insert("language".to_string(), "en".to_string());
    Ok(prefs)
}

fn fetch_user_avatar(user_id: u32) -> Result<String, ApiError> {
    // Simulate API call
    Ok(format!("https://example.com/avatars/{}.jpg", user_id))
}

fn create_default_profile(user_id: u32) -> Result<UserProfile, ApiError> {
    Ok(UserProfile {
        user: User {
            id: user_id,
            name: "Default User".to_string(),
            email: "default@example.com".to_string(),
        },
        preferences: HashMap::new(),
        avatar_url: "https://example.com/default-avatar.jpg".to_string(),
    })
}

// Pattern 5: File Processing Pipeline
fn process_file_pipeline(input_path: &str, output_path: &str) -> Result<ProcessedFile, PipelineError> {
    read_file(input_path)
        .and_then(|content| validate_file_format(&content))
        .and_then(parse_file_content)
        .and_then(transform_content)
        .and_then(|content| write_file(output_path, &content))
        .map(|_| ProcessedFile {
            input_path: input_path.to_string(),
            output_path: output_path.to_string(),
            size: std::fs::metadata(output_path)
                .map(|m| m.len())
                .unwrap_or(0),
        })
        .map_err(|e| PipelineError::ProcessingFailed(Box::new(e)))
}

fn read_file(path: &str) -> Result<String, PipelineError> {
    std::fs::read_to_string(path)
        .map_err(|_| PipelineError::FileNotFound(path.to_string()))
}

fn validate_file_format(content: &str) -> Result<String, PipelineError> {
    if content.trim().is_empty() {
        Err(PipelineError::EmptyFile)
    } else if !content.contains("CSV") {
        Err(PipelineError::InvalidFormat("Expected CSV format".to_string()))
    } else {
        Ok(content.to_string())
    }
}

fn parse_file_content(content: String) -> Result<Vec<Record>, PipelineError> {
    let mut records = Vec::new();
    
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            // Only process lines with exactly 3 fields (skip headers, comments, etc.)
            records.push(Record {
                id: parts[0].to_string(),
                name: parts[1].to_string(),
                value: parts[2].to_string(),
            });
        }
        // Skip lines that don't have 3 fields (headers, comments, etc.)
    }
    
    if records.is_empty() {
        Err(PipelineError::NoValidRecords)
    } else {
        Ok(records)
    }
}

fn transform_content(records: Vec<Record>) -> Result<String, PipelineError> {
    let transformed: Vec<String> = records
        .into_iter()
        .map(|record| {
            format!("ID: {}, Name: {}, Value: {}", record.id, record.name, record.value)
        })
        .collect();
    
    Ok(transformed.join("\n"))
}

fn write_file(path: &str, content: &str) -> Result<(), PipelineError> {
    std::fs::write(path, content)
        .map_err(|_| PipelineError::WriteError(path.to_string()))
}

// Pattern 6: Error Accumulation
fn batch_validate_users(users: Vec<RawUser>) -> Result<Vec<ValidatedUser>, Vec<ValidationError>> {
    let mut validated = Vec::new();
    let mut errors = Vec::new();
    
    for user in users {
        match validate_single_user(user) {
            Ok(valid) => validated.push(valid),
            Err(e) => errors.push(e),
        }
    }
    
    if errors.is_empty() {
        Ok(validated)
    } else {
        Err(errors)
    }
}

fn validate_single_user(user: RawUser) -> Result<ValidatedUser, ValidationError> {
    validate_name(&user.name)
        .and_then(|_| validate_email(&user.email))
        .and_then(|_| validate_age(user.age))
        .map(|_| ValidatedUser {
            name: user.name,
            email: user.email,
            age: user.age,
        })
}

fn validate_name(name: &str) -> Result<(), ValidationError> {
    if name.is_empty() {
        Err(ValidationError::EmptyName)
    } else if name.len() > 50 {
        Err(ValidationError::NameTooLong)
    } else {
        Ok(())
    }
}

fn validate_email(email: &str) -> Result<(), ValidationError> {
    if email.contains('@') {
        Ok(())
    } else {
        Err(ValidationError::InvalidEmail)
    }
}

fn validate_age(age: u32) -> Result<(), ValidationError> {
    if age < 18 {
        Err(ValidationError::TooYoung)
    } else if age > 120 {
        Err(ValidationError::TooOld)
    } else {
        Ok(())
    }
}

// Pattern 7: Optional Error Recovery
fn process_with_recovery(input: &str) -> Result<String, ProcessError> {
    parse_input(input)
        .map_err(|_e| ProcessError::ValidationError("Parse failed".to_string()))
        .or_else(|_e| {
            // Try to recover by cleaning the input
            let cleaned = clean_input(input);
            parse_input(&cleaned)
                .map_err(|_| ProcessError::ValidationError("Recovery parse failed".to_string()))
        })
        .and_then(process_data)
        .map(|data| format!("Processed: {}", data))
}

fn clean_input(input: &str) -> String {
    input.trim().to_lowercase()
}

fn parse_input(input: &str) -> Result<ParsedData, String> {
    if input.is_empty() {
        Err("Empty input".to_string())
    } else {
        Ok(ParsedData { content: input.to_string() })
    }
}

fn process_data(data: ParsedData) -> Result<String, ProcessError> {
    if data.content.contains("error") {
        Err(ProcessError::ProcessingError)
    } else {
        Ok(data.content)
    }
}

fn main() {
    println!("=== Day 32: Result Combinators Examples ===\n");
    
    // Example 1: Chaining Transformations
    println!("1. Chaining Transformations:");
    let numbers_input = "1,2,3,4,5";
    match process_numbers(numbers_input) {
        Ok(doubled) => println!("Doubled numbers: {:?}", doubled),
        Err(e) => println!("Processing error: {:?}", e),
    }
    println!();
    
    // Example 2: Error Recovery
    println!("2. Error Recovery:");
    let config_paths = vec!["config1.txt".to_string(), "config2.txt".to_string()];
    match load_config_with_fallbacks(&config_paths) {
        Ok(config) => println!("Config loaded: {:?}", config),
        Err(e) => println!("Config error: {:?}", e),
    }
    println!();
    
    // Example 3: Conditional Processing
    println!("3. Conditional Processing:");
    let options = ProcessOptions {
        validate: true,
        transform: true,
        normalize: true,
    };
    match process_data_conditionally("Hello World", options) {
        Ok(processed) => println!("Processed data: {:?}", processed),
        Err(e) => println!("Processing error: {:?}", e),
    }
    println!();
    
    // Example 4: API Request Pipeline
    println!("4. API Request Pipeline:");
    match fetch_user_profile(123) {
        Ok(profile) => println!("User profile: {:?}", profile),
        Err(e) => println!("API error: {:?}", e),
    }
    
    // Simulate error case
    match fetch_user_profile(999) {
        Ok(profile) => println!("User profile: {:?}", profile),
        Err(e) => println!("API error: {:?}", e),
    }
    println!();
    
    // Example 5: File Processing Pipeline
    println!("5. File Processing Pipeline:");
    let csv_content = "CSV Header: ID,Name,Value\nID1,Name1,Value1\nID2,Name2,Value2";
    std::fs::write("temp_input.csv", csv_content).unwrap();
    
    match process_file_pipeline("temp_input.csv", "temp_output.txt") {
        Ok(processed) => println!("File processed: {:?}", processed),
        Err(e) => println!("Pipeline error: {:?}", e),
    }
    
    // Clean up
    let _ = std::fs::remove_file("temp_input.csv");
    let _ = std::fs::remove_file("temp_output.txt");
    println!();
    
    // Example 5a: File Processing Pipeline with Errors
    println!("5a. File Processing Pipeline - Error Cases:");
    
    // Case 1: File not found
    match process_file_pipeline("nonexistent.csv", "output.txt") {
        Ok(processed) => println!("File processed: {:?}", processed),
        Err(e) => println!("Pipeline error (file not found): {:?}", e),
    }
    
    // Case 2: Empty file
    std::fs::write("empty.csv", "").unwrap();
    match process_file_pipeline("empty.csv", "output.txt") {
        Ok(processed) => println!("File processed: {:?}", processed),
        Err(e) => println!("Pipeline error (empty file): {:?}", e),
    }
    
    // Case 3: Invalid format (missing required format indicator)
    let invalid_content = "Just some text\nNot a proper format\nNo required header";
    std::fs::write("invalid.txt", invalid_content).unwrap();
    match process_file_pipeline("invalid.txt", "output.txt") {
        Ok(processed) => println!("File processed: {:?}", processed),
        Err(e) => println!("Pipeline error (invalid format): {:?}", e),
    }
    
    // Case 4: Valid format indicator but no valid records
    let no_records_content = "CSV Header: This is a CSV file\nSingle Column\nAnother Single Column";
    std::fs::write("no_records.csv", no_records_content).unwrap();
    match process_file_pipeline("no_records.csv", "output.txt") {
        Ok(processed) => println!("File processed: {:?}", processed),
        Err(e) => println!("Pipeline error (no valid records): {:?}", e),
    }
    
    // Clean up error test files
    let _ = std::fs::remove_file("empty.csv");
    let _ = std::fs::remove_file("invalid.txt");
    let _ = std::fs::remove_file("no_records.csv");
    let _ = std::fs::remove_file("output.txt");
    println!();
    
    // Example 6: Error Accumulation
    println!("6. Error Accumulation:");
    let users = vec![
        RawUser { name: "John".to_string(), email: "john@example.com".to_string(), age: 25 },
        RawUser { name: "".to_string(), email: "invalid".to_string(), age: 15 },
        RawUser { name: "Jane".to_string(), email: "jane@example.com".to_string(), age: 30 },
    ];
    
    match batch_validate_users(users) {
        Ok(validated) => println!("Validated users: {:?}", validated),
        Err(errors) => println!("Validation errors: {:?}", errors),
    }
    println!();
    
    // Example 7: Optional Error Recovery
    println!("7. Optional Error Recovery:");
    match process_with_recovery("") {
        Ok(result) => println!("Recovery result: {}", result),
        Err(e) => println!("Recovery error: {:?}", e),
    }
    
    match process_with_recovery("   HELLO   ") {
        Ok(result) => println!("Recovery result: {}", result),
        Err(e) => println!("Recovery error: {:?}", e),
    }
    println!();
    
    println!("=== End of Day 32 Examples ===");
}
