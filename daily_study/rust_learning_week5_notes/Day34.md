# Day 34 - Error Handling Patterns

**Learning Focus**: When to panic vs return errors, error handling decision guidelines

---

## 🎯 Learning Objectives

By the end of this day, you should understand:
- When to use `panic!` vs `Result<T, E>`
- Error handling decision guidelines for different scenarios
- The "fail fast" vs "graceful degradation" approaches
- Error handling patterns for different application types
- How to design error types for your domain
- Best practices for error handling in production systems

---

## 📚 Core Concepts

### **Panic vs Result: Decision Guidelines**

```rust
// Use panic! for programming errors (bugs)
fn get_element(slice: &[i32], index: usize) -> i32 {
    if index >= slice.len() {
        panic!("Index {} out of bounds for slice of length {}", index, slice.len());
    }
    slice[index]
}

// Use Result for recoverable errors
fn parse_age(input: &str) -> Result<u32, ParseError> {
    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }
    
    input.parse::<u32>()
        .map_err(|_| ParseError::InvalidFormat)
}

#[derive(Debug)]
enum ParseError {
    EmptyInput,
    InvalidFormat,
}
```

### **Error Handling Decision Tree**

```rust
fn should_i_panic_or_return_error() {
    // Ask yourself:
    // 1. Is this a programming error (bug) that should never happen?
    //    → Use panic!
    // 2. Is this a recoverable condition that the caller can handle?
    //    → Use Result<T, E>
    // 3. Is this an external failure (network, file system, user input)?
    //    → Use Result<T, E>
    // 4. Is this an invariant violation that indicates a bug?
    //    → Use panic!
}
```

---

## 🔧 Implementation Patterns

### **Pattern 1: Library vs Application Error Handling**

```rust
// LIBRARY CODE: Use specific error types
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Connection failed: {message}")]
    ConnectionFailed { message: String },
    
    #[error("Query failed: {sql}")]
    QueryFailed { sql: String, source: Box<dyn std::error::Error + Send + Sync> },
    
    #[error("Transaction failed: {message}")]
    TransactionFailed { message: String },
}

pub fn connect_to_database(url: &str) -> Result<Connection, DatabaseError> {
    // Library should never panic for external failures
    if url.is_empty() {
        return Err(DatabaseError::ConnectionFailed {
            message: "Empty connection URL".to_string(),
        });
    }
    
    // Simulate connection
    Ok(Connection { url: url.to_string() })
}

// APPLICATION CODE: Use anyhow for simplicity
use anyhow::{Result, Context};

fn main() -> Result<()> {
    let connection = connect_to_database("database://localhost")
        .context("Failed to connect to database")?;
    
    // Application can use ? for error propagation
    process_data(&connection)?;
    
    Ok(())
}

fn process_data(connection: &Connection) -> Result<()> {
    // Application logic with error context
    let data = fetch_data(connection)
        .context("Failed to fetch data")?;
    
    Ok(())
}
```

### **Pattern 2: Fail Fast vs Graceful Degradation**

```rust
// FAIL FAST: Use panic! for invariant violations
struct BankAccount {
    balance: i64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: u64) {
        // This should never happen in correct code
        if amount > self.balance as u64 {
            panic!("Insufficient funds: attempted to withdraw {} but only have {}", 
                   amount, self.balance);
        }
        
        self.balance -= amount as i64;
    }
    
    fn safe_withdraw(&mut self, amount: u64) -> Result<(), WithdrawalError> {
        if amount > self.balance as u64 {
            return Err(WithdrawalError::InsufficientFunds {
                requested: amount,
                available: self.balance as u64,
            });
        }
        
        self.balance -= amount as i64;
        Ok(())
    }
}

#[derive(Debug)]
enum WithdrawalError {
    InsufficientFunds { requested: u64, available: u64 },
}

// GRACEFUL DEGRADATION: Use Result for recoverable failures
fn load_user_preferences(user_id: u32) -> Result<UserPreferences, PreferenceError> {
    // Try primary storage
    match load_from_primary_storage(user_id) {
        Ok(prefs) => Ok(prefs),
        Err(_) => {
            // Fallback to secondary storage
            match load_from_secondary_storage(user_id) {
                Ok(prefs) => Ok(prefs),
                Err(_) => {
                    // Final fallback to defaults
                    Ok(UserPreferences::default())
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct UserPreferences {
    theme: String,
    language: String,
}

#[derive(Debug)]
enum PreferenceError {
    PrimaryStorageFailed,
    SecondaryStorageFailed,
}
```

### **Pattern 3: Error Context and Wrapping**

```rust
use anyhow::{Result, Context};

fn process_user_data(user_id: u32, data: &str) -> Result<ProcessedData> {
    // Add context at each step
    let parsed = parse_user_data(data)
        .with_context(|| format!("Failed to parse data for user {}", user_id))?;
    
    let validated = validate_user_data(&parsed)
        .with_context(|| format!("Validation failed for user {}", user_id))?;
    
    let enriched = enrich_user_data(validated)
        .context("Failed to enrich user data")?;
    
    Ok(enriched)
}

fn parse_user_data(data: &str) -> Result<ParsedUserData, ParseError> {
    // Parsing logic
    Ok(ParsedUserData { content: data.to_string() })
}

fn validate_user_data(data: &ParsedUserData) -> Result<ValidatedUserData, ValidationError> {
    // Validation logic
    Ok(ValidatedUserData { content: data.content.clone() })
}

fn enrich_user_data(data: ValidatedUserData) -> Result<ProcessedData, EnrichmentError> {
    // Enrichment logic
    Ok(ProcessedData { content: data.content })
}

#[derive(Debug)]
struct ParsedUserData {
    content: String,
}

#[derive(Debug)]
struct ValidatedUserData {
    content: String,
}

#[derive(Debug)]
struct ProcessedData {
    content: String,
}

#[derive(Debug)]
enum ParseError {
    InvalidFormat,
}

#[derive(Debug)]
enum ValidationError {
    InvalidData,
}

#[derive(Debug)]
enum EnrichmentError {
    ServiceUnavailable,
}
```

---

## 🎮 Practical Applications

### **Application 1: Web API Error Handling**

```rust
use std::collections::HashMap;

// API Error Types
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Authentication failed: {reason}")]
    AuthenticationFailed { reason: String },
    
    #[error("Authorization failed: user {user_id} lacks permission {permission}")]
    AuthorizationFailed { user_id: u32, permission: String },
    
    #[error("Resource not found: {resource_type} with id {id}")]
    NotFound { resource_type: String, id: String },
    
    #[error("Validation failed: {field} - {message}")]
    ValidationFailed { field: String, message: String },
    
    #[error("Internal server error")]
    InternalServerError,
    
    #[error("External service error: {service}")]
    ExternalServiceError { service: String },
}

impl ApiError {
    fn status_code(&self) -> u16 {
        match self {
            ApiError::AuthenticationFailed { .. } => 401,
            ApiError::AuthorizationFailed { .. } => 403,
            ApiError::NotFound { .. } => 404,
            ApiError::ValidationFailed { .. } => 400,
            ApiError::InternalServerError => 500,
            ApiError::ExternalServiceError { .. } => 502,
        }
    }
    
    fn error_code(&self) -> &'static str {
        match self {
            ApiError::AuthenticationFailed { .. } => "AUTH_FAILED",
            ApiError::AuthorizationFailed { .. } => "AUTHZ_FAILED",
            ApiError::NotFound { .. } => "NOT_FOUND",
            ApiError::ValidationFailed { .. } => "VALIDATION_FAILED",
            ApiError::InternalServerError => "INTERNAL_ERROR",
            ApiError::ExternalServiceError { .. } => "EXTERNAL_ERROR",
        }
    }
}

// API Handler
struct ApiHandler {
    user_service: UserService,
    auth_service: AuthService,
}

impl ApiHandler {
    fn handle_get_user(&self, user_id: u32, token: &str) -> Result<UserResponse, ApiError> {
        // Authentication
        let auth_user = self.auth_service.authenticate(token)
            .map_err(|e| ApiError::AuthenticationFailed { reason: e })?;
        
        // Authorization
        if !self.auth_service.can_read_user(&auth_user, user_id) {
            return Err(ApiError::AuthorizationFailed {
                user_id: auth_user.id,
                permission: "read_user".to_string(),
            });
        }
        
        // Business logic
        let user = self.user_service.get_user(user_id)
            .map_err(|_| ApiError::NotFound {
                resource_type: "User".to_string(),
                id: user_id.to_string(),
            })?;
        
        Ok(UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }
    
    fn handle_create_user(&self, request: CreateUserRequest, token: &str) -> Result<UserResponse, ApiError> {
        // Authentication
        let auth_user = self.auth_service.authenticate(token)
            .map_err(|e| ApiError::AuthenticationFailed { reason: e })?;
        
        // Authorization
        if !self.auth_service.can_create_user(&auth_user) {
            return Err(ApiError::AuthorizationFailed {
                user_id: auth_user.id,
                permission: "create_user".to_string(),
            });
        }
        
        // Validation
        if request.name.is_empty() {
            return Err(ApiError::ValidationFailed {
                field: "name".to_string(),
                message: "Name cannot be empty".to_string(),
            });
        }
        
        if !request.email.contains('@') {
            return Err(ApiError::ValidationFailed {
                field: "email".to_string(),
                message: "Invalid email format".to_string(),
            });
        }
        
        // Business logic
        let user = self.user_service.create_user(request)
            .map_err(|e| match e {
                UserServiceError::DuplicateEmail => ApiError::ValidationFailed {
                    field: "email".to_string(),
                    message: "Email already exists".to_string(),
                },
                UserServiceError::ExternalServiceError => ApiError::ExternalServiceError {
                    service: "user_database".to_string(),
                },
            })?;
        
        Ok(UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }
}

// Supporting types and services
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug)]
struct UserResponse {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug)]
struct CreateUserRequest {
    name: String,
    email: String,
}

struct UserService;

impl UserService {
    fn get_user(&self, user_id: u32) -> Result<User, UserServiceError> {
        // Simulate database lookup
        if user_id == 0 {
            Err(UserServiceError::NotFound)
        } else {
            Ok(User {
                id: user_id,
                name: format!("User {}", user_id),
                email: format!("user{}@example.com", user_id),
            })
        }
    }
    
    fn create_user(&self, request: CreateUserRequest) -> Result<User, UserServiceError> {
        // Simulate user creation
        if request.email.contains("duplicate") {
            Err(UserServiceError::DuplicateEmail)
        } else if request.email.contains("error") {
            Err(UserServiceError::ExternalServiceError)
        } else {
            Ok(User {
                id: 1,
                name: request.name,
                email: request.email,
            })
        }
    }
}

#[derive(Debug)]
enum UserServiceError {
    NotFound,
    DuplicateEmail,
    ExternalServiceError,
}

struct AuthService;

impl AuthService {
    fn authenticate(&self, token: &str) -> Result<AuthenticatedUser, String> {
        if token == "invalid" {
            Err("Invalid token".to_string())
        } else {
            Ok(AuthenticatedUser { id: 1 })
        }
    }
    
    fn can_read_user(&self, _auth_user: &AuthenticatedUser, _target_user_id: u32) -> bool {
        true // Simplified
    }
    
    fn can_create_user(&self, _auth_user: &AuthenticatedUser) -> bool {
        true // Simplified
    }
}

#[derive(Debug)]
struct AuthenticatedUser {
    id: u32,
}
```

### **Application 2: File Processing with Error Recovery**

```rust
use std::path::Path;

// File Processing Error Types
#[derive(Debug, thiserror::Error)]
pub enum FileProcessingError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },
    
    #[error("Invalid file format: {expected}, got {actual}")]
    InvalidFormat { expected: String, actual: String },
    
    #[error("Processing failed at stage {stage}: {reason}")]
    ProcessingFailed { stage: String, reason: String },
    
    #[error("Output directory not writable: {path}")]
    OutputNotWritable { path: String },
    
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
}

// File Processor
struct FileProcessor {
    input_dir: String,
    output_dir: String,
    temp_dir: String,
}

impl FileProcessor {
    fn new(input_dir: String, output_dir: String, temp_dir: String) -> Self {
        Self {
            input_dir,
            output_dir,
            temp_dir,
        }
    }
    
    fn process_all_files(&self) -> Result<ProcessingSummary, FileProcessingError> {
        let files = self.list_input_files()?;
        let mut summary = ProcessingSummary::new();
        
        for file_path in files {
            match self.process_single_file(&file_path) {
                Ok(processed) => {
                    summary.success_count += 1;
                    summary.processed_files.push(processed);
                }
                Err(e) => {
                    summary.error_count += 1;
                    summary.errors.push(ProcessingError {
                        file: file_path,
                        error: e.to_string(),
                    });
                }
            }
        }
        
        Ok(summary)
    }
    
    fn process_single_file(&self, file_path: &str) -> Result<ProcessedFile, FileProcessingError> {
        // Read file
        let content = std::fs::read_to_string(file_path)
            .map_err(|_| FileProcessingError::FileNotFound { 
                path: file_path.to_string() 
            })?;
        
        // Validate format
        let validated_content = self.validate_format(&content, file_path)?;
        
        // Parse content
        let parsed_data = self.parse_content(&validated_content, file_path)?;
        
        // Transform data
        let transformed_data = self.transform_data(parsed_data, file_path)?;
        
        // Write output
        let output_path = self.generate_output_path(file_path)?;
        self.write_output(&transformed_data, &output_path)?;
        
        Ok(ProcessedFile {
            input_path: file_path.to_string(),
            output_path,
            size: transformed_data.len(),
        })
    }
    
    fn list_input_files(&self) -> Result<Vec<String>, FileProcessingError> {
        // Simulate file listing
        Ok(vec![
            format!("{}/file1.txt", self.input_dir),
            format!("{}/file2.csv", self.input_dir),
            format!("{}/file3.json", self.input_dir),
        ])
    }
    
    fn validate_format(&self, content: &str, file_path: &str) -> Result<String, FileProcessingError> {
        if content.trim().is_empty() {
            return Err(FileProcessingError::ProcessingFailed {
                stage: "validation".to_string(),
                reason: "File is empty".to_string(),
            });
        }
        
        // Check if file has expected format based on extension
        let extension = Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        match extension {
            "csv" => {
                if !content.contains(',') {
                    return Err(FileProcessingError::InvalidFormat {
                        expected: "CSV format".to_string(),
                        actual: "No comma separators found".to_string(),
                    });
                }
            }
            "json" => {
                // Simple JSON validation
                if !content.trim().starts_with('{') && !content.trim().starts_with('[') {
                    return Err(FileProcessingError::InvalidFormat {
                        expected: "JSON format".to_string(),
                        actual: "Does not start with { or [".to_string(),
                    });
                }
            }
            _ => {} // Accept other formats
        }
        
        Ok(content.to_string())
    }
    
    fn parse_content(&self, content: &str, file_path: &str) -> Result<ParsedData, FileProcessingError> {
        let extension = Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        match extension {
            "csv" => self.parse_csv(content),
            "json" => self.parse_json(content),
            _ => self.parse_text(content),
        }
    }
    
    fn parse_csv(&self, content: &str) -> Result<ParsedData, FileProcessingError> {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Err(FileProcessingError::ProcessingFailed {
                stage: "CSV parsing".to_string(),
                reason: "No lines found".to_string(),
            });
        }
        
        Ok(ParsedData {
            records: lines.len(),
            content: content.to_string(),
        })
    }
    
    fn parse_json(&self, content: &str) -> Result<ParsedData, FileProcessingError> {
        // Simple JSON parsing simulation
        if content.contains("invalid") {
            return Err(FileProcessingError::ProcessingFailed {
                stage: "JSON parsing".to_string(),
                reason: "Invalid JSON structure".to_string(),
            });
        }
        
        Ok(ParsedData {
            records: 1,
            content: content.to_string(),
        })
    }
    
    fn parse_text(&self, content: &str) -> Result<ParsedData, FileProcessingError> {
        Ok(ParsedData {
            records: content.lines().count(),
            content: content.to_string(),
        })
    }
    
    fn transform_data(&self, data: ParsedData, file_path: &str) -> Result<String, FileProcessingError> {
        if data.content.contains("TRANSFORM_ERROR") {
            return Err(FileProcessingError::ProcessingFailed {
                stage: "transformation".to_string(),
                reason: "Data contains transform error marker".to_string(),
            });
        }
        
        Ok(format!("PROCESSED: {}\nRecords: {}", data.content, data.records))
    }
    
    fn generate_output_path(&self, input_path: &str) -> Result<String, FileProcessingError> {
        let filename = Path::new(input_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");
        
        Ok(format!("{}/processed_{}", self.output_dir, filename))
    }
    
    fn write_output(&self, content: &str, output_path: &str) -> Result<(), FileProcessingError> {
        std::fs::write(output_path, content)?;
        Ok(())
    }
}

#[derive(Debug)]
struct ParsedData {
    records: usize,
    content: String,
}

#[derive(Debug)]
struct ProcessedFile {
    input_path: String,
    output_path: String,
    size: usize,
}

#[derive(Debug)]
struct ProcessingSummary {
    success_count: usize,
    error_count: usize,
    processed_files: Vec<ProcessedFile>,
    errors: Vec<ProcessingError>,
}

impl ProcessingSummary {
    fn new() -> Self {
        Self {
            success_count: 0,
            error_count: 0,
            processed_files: Vec::new(),
            errors: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct ProcessingError {
    file: String,
    error: String,
}
```

---

## 🧪 Best Practices

### **Error Handling Decision Guidelines**

```rust
// ✅ USE PANIC! FOR:
// 1. Programming errors (bugs)
// 2. Invariant violations
// 3. Unreachable code
// 4. Index out of bounds (in debug builds)

fn get_element(slice: &[i32], index: usize) -> i32 {
    if cfg!(debug_assertions) && index >= slice.len() {
        panic!("Index {} out of bounds for slice of length {}", index, slice.len());
    }
    slice[index]
}

// ✅ USE RESULT<T, E> FOR:
// 1. External failures (network, file system, user input)
// 2. Recoverable conditions
// 3. Expected failures
// 4. Third-party library errors

fn read_config_file(path: &str) -> Result<Config, ConfigError> {
    std::fs::read_to_string(path)
        .map_err(|_| ConfigError::FileNotFound(path.to_string()))
        .and_then(|content| Config::parse(&content))
}

// ❌ DON'T USE PANIC! FOR:
// 1. User input validation
// 2. Network failures
// 3. File system errors
// 4. Business logic errors

// ❌ DON'T USE RESULT<T, E> FOR:
// 1. Programming bugs
// 2. Invariant violations
// 3. Unreachable code paths
```

### **Error Type Design**

```rust
// ✅ GOOD: Specific error types for libraries
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Connection failed: {message}")]
    ConnectionFailed { message: String },
    
    #[error("Query failed: {sql}")]
    QueryFailed { 
        sql: String, 
        #[source] 
        source: Box<dyn std::error::Error + Send + Sync> 
    },
    
    #[error("Transaction failed: {message}")]
    TransactionFailed { message: String },
}

// ✅ GOOD: Generic error types for applications
use anyhow::{Result, Context};

fn main() -> Result<()> {
    let config = load_config("config.toml")
        .context("Failed to load configuration")?;
    
    run_application(&config)
        .context("Application execution failed")?;
    
    Ok(())
}
```

---

## 🎯 Key Takeaways

1. **Use panic! for programming errors** - Bugs and invariant violations
2. **Use Result<T, E> for recoverable errors** - External failures and expected conditions
3. **Design error types for your use case** - Specific for libraries, generic for applications
4. **Add context to errors** - Help with debugging and user experience
5. **Consider graceful degradation** - Provide fallbacks when possible
6. **Fail fast for bugs, gracefully for external failures**

---

## 🔗 Related Concepts

- **[[Custom Error Types]]** - Creating domain-specific error types
- **[[Error Propagation]]** - The ? operator and error forwarding
- **[[anyhow and thiserror]]** - Advanced error handling crates
- **[[Result Combinators]]** - Functional error handling methods
- **[[Panic Recovery]]** - Handling panics with catch_unwind

---

*Tags: #error-handling #panic-vs-result #error-patterns #decision-guidelines #fail-fast #graceful-degradation*

*Links: [[zettel-index]] | [[Custom Error Types]] | [[Error Propagation]] | [[anyhow and thiserror]] | [[Result Combinators]] | [[Panic Recovery]] | [[Week 5 Overview]]*
