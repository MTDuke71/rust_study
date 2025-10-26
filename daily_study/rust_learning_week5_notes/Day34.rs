// Day 34 - Error Handling Patterns
// Runnable examples demonstrating when to panic vs return errors

#[allow(dead_code, unused_variables, unused_imports)]
use std::collections::HashMap;
use std::path::Path;

// Custom error types for examples
#[derive(Debug)]
enum ParseError {
    EmptyInput,
    InvalidFormat,
}

#[derive(Debug)]
enum WithdrawalError {
    InsufficientFunds { requested: u64, available: u64 },
}

#[derive(Debug)]
enum PreferenceError {
    PrimaryStorageFailed,
    SecondaryStorageFailed,
}

#[derive(Debug)]
enum ApiError {
    AuthenticationFailed { reason: String },
    AuthorizationFailed { user_id: u32, permission: String },
    NotFound { resource_type: String, id: String },
    ValidationFailed { field: String, message: String },
    InternalServerError,
    ExternalServiceError { service: String },
}

#[derive(Debug)]
enum FileProcessingError {
    FileNotFound { path: String },
    PermissionDenied { path: String },
    InvalidFormat { expected: String, actual: String },
    ProcessingFailed { stage: String, reason: String },
    OutputNotWritable { path: String },
    Io { source: String },
}

impl std::fmt::Display for FileProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileProcessingError::FileNotFound { path } => write!(f, "File not found: {}", path),
            FileProcessingError::PermissionDenied { path } => write!(f, "Permission denied: {}", path),
            FileProcessingError::InvalidFormat { expected, actual } => write!(f, "Invalid format: expected {}, got {}", expected, actual),
            FileProcessingError::ProcessingFailed { stage, reason } => write!(f, "Processing failed at {}: {}", stage, reason),
            FileProcessingError::OutputNotWritable { path } => write!(f, "Output not writable: {}", path),
            FileProcessingError::Io { source } => write!(f, "IO error: {}", source),
        }
    }
}

impl std::error::Error for FileProcessingError {}

#[derive(Debug)]
enum UserServiceError {
    NotFound,
    DuplicateEmail,
    ExternalServiceError,
}

// Data structures
#[derive(Debug)]
struct BankAccount {
    balance: i64,
}

#[derive(Debug, Default)]
struct UserPreferences {
    theme: String,
    language: String,
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

#[derive(Debug)]
struct AuthenticatedUser {
    id: u32,
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

#[derive(Debug)]
struct ProcessingError {
    file: String,
    error: String,
}

// Example functions demonstrating error handling patterns

// Pattern 1: Panic vs Result
fn get_element(slice: &[i32], index: usize) -> i32 {
    if index >= slice.len() {
        panic!("Index {} out of bounds for slice of length {}", index, slice.len());
    }
    slice[index]
}

fn parse_age(input: &str) -> Result<u32, ParseError> {
    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }
    
    input.parse::<u32>()
        .map_err(|_| ParseError::InvalidFormat)
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

fn load_from_primary_storage(user_id: u32) -> Result<UserPreferences, PreferenceError> {
    if user_id == 0 {
        Err(PreferenceError::PrimaryStorageFailed)
    } else {
        Ok(UserPreferences {
            theme: "dark".to_string(),
            language: "en".to_string(),
        })
    }
}

fn load_from_secondary_storage(user_id: u32) -> Result<UserPreferences, PreferenceError> {
    if user_id == 999 {
        Err(PreferenceError::SecondaryStorageFailed)
    } else {
        Ok(UserPreferences {
            theme: "light".to_string(),
            language: "es".to_string(),
        })
    }
}

// Pattern 2: Error Context and Wrapping
fn process_user_data(user_id: u32, data: &str) -> Result<ProcessedData, String> {
    // Add context at each step
    let parsed = parse_user_data(data)
        .map_err(|e| format!("Failed to parse data for user {}: {:?}", user_id, e))?;
    
    let validated = validate_user_data(&parsed)
        .map_err(|e| format!("Validation failed for user {}: {:?}", user_id, e))?;
    
    let enriched = enrich_user_data(validated)
        .map_err(|e| format!("Failed to enrich user data: {:?}", e))?;
    
    Ok(enriched)
}

fn parse_user_data(data: &str) -> Result<ParsedUserData, ParseError> {
    // Parsing logic
    Ok(ParsedUserData { content: data.to_string() })
}

fn validate_user_data(data: &ParsedUserData) -> Result<ValidatedUserData, String> {
    // Validation logic
    if data.content.is_empty() {
        Err("Empty data".to_string())
    } else {
        Ok(ValidatedUserData { content: data.content.clone() })
    }
}

fn enrich_user_data(data: ValidatedUserData) -> Result<ProcessedData, String> {
    // Enrichment logic
    Ok(ProcessedData { content: data.content })
}

// Pattern 3: API Error Handling
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

struct ApiHandler {
    user_service: UserService,
    auth_service: AuthService,
}

impl ApiHandler {
    fn new() -> Self {
        Self {
            user_service: UserService,
            auth_service: AuthService,
        }
    }
    
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
                _ => ApiError::InternalServerError,
            })?;
        
        Ok(UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }
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

// Pattern 4: File Processing with Error Recovery
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
        std::fs::write(output_path, content)
            .map_err(|_| FileProcessingError::Io { 
                source: format!("Failed to write to {}", output_path) 
            })?;
        Ok(())
    }
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

fn main() {
    println!("=== Day 34: Error Handling Patterns Examples ===\n");
    
    // Example 1: Panic vs Result
    println!("1. Panic vs Result:");
    
    // Panic case (programming error)
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Element at index 2: {}", get_element(&numbers, 2));
    
    // This would panic: get_element(&numbers, 10);
    
    // Result case (recoverable error)
    match parse_age("25") {
        Ok(age) => println!("Parsed age: {}", age),
        Err(e) => println!("Parse error: {:?}", e),
    }
    
    match parse_age("") {
        Ok(age) => println!("Parsed age: {}", age),
        Err(e) => println!("Parse error: {:?}", e),
    }
    println!();
    
    // Example 2: Bank account withdrawal
    println!("2. Bank Account Withdrawal:");
    let mut account = BankAccount { balance: 100 };
    
    // Safe withdrawal
    match account.safe_withdraw(50) {
        Ok(()) => println!("Withdrawal successful, balance: {}", account.balance),
        Err(e) => println!("Withdrawal failed: {:?}", e),
    }
    
    // This would panic: account.withdraw(100);
    
    // Safe withdrawal that fails
    match account.safe_withdraw(100) {
        Ok(()) => println!("Withdrawal successful, balance: {}", account.balance),
        Err(e) => println!("Withdrawal failed: {:?}", e),
    }
    println!();
    
    // Example 3: Graceful degradation
    println!("3. Graceful Degradation:");
    match load_user_preferences(1) {
        Ok(prefs) => println!("Loaded preferences: {:?}", prefs),
        Err(e) => println!("Failed to load preferences: {:?}", e),
    }
    
    match load_user_preferences(0) {
        Ok(prefs) => println!("Loaded preferences: {:?}", prefs),
        Err(e) => println!("Failed to load preferences: {:?}", e),
    }
    
    match load_user_preferences(999) {
        Ok(prefs) => println!("Loaded preferences: {:?}", prefs),
        Err(e) => println!("Failed to load preferences: {:?}", e),
    }
    println!();
    
    // Example 4: Error context and wrapping
    println!("4. Error Context and Wrapping:");
    match process_user_data(123, "test data") {
        Ok(processed) => println!("Processed data: {:?}", processed),
        Err(e) => println!("Processing failed: {}", e),
    }
    
    match process_user_data(456, "") {
        Ok(processed) => println!("Processed data: {:?}", processed),
        Err(e) => println!("Processing failed: {}", e),
    }
    println!();
    
    // Example 5: API error handling
    println!("5. API Error Handling:");
    let api_handler = ApiHandler::new();
    
    // Successful user retrieval
    match api_handler.handle_get_user(123, "valid_token") {
        Ok(response) => println!("User response: {:?}", response),
        Err(e) => println!("API error: {:?} (status: {})", e, e.status_code()),
    }
    
    // Authentication failure
    match api_handler.handle_get_user(123, "invalid") {
        Ok(response) => println!("User response: {:?}", response),
        Err(e) => println!("API error: {:?} (status: {})", e, e.status_code()),
    }
    
    // User not found
    match api_handler.handle_get_user(0, "valid_token") {
        Ok(response) => println!("User response: {:?}", response),
        Err(e) => println!("API error: {:?} (status: {})", e, e.status_code()),
    }
    
    // User creation
    let create_request = CreateUserRequest {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };
    match api_handler.handle_create_user(create_request, "valid_token") {
        Ok(response) => println!("Created user: {:?}", response),
        Err(e) => println!("API error: {:?} (status: {})", e, e.status_code()),
    }
    
    // Validation failure
    let invalid_request = CreateUserRequest {
        name: "".to_string(),
        email: "invalid_email".to_string(),
    };
    match api_handler.handle_create_user(invalid_request, "valid_token") {
        Ok(response) => println!("Created user: {:?}", response),
        Err(e) => println!("API error: {:?} (status: {})", e, e.status_code()),
    }
    println!();
    
    // Example 6: File processing with error recovery
    println!("6. File Processing with Error Recovery:");
    let processor = FileProcessor::new(
        "input".to_string(),
        "output".to_string(),
        "temp".to_string(),
    );
    
    // Create some test files
    std::fs::create_dir_all("input").unwrap();
    std::fs::create_dir_all("output").unwrap();
    
    std::fs::write("input/file1.txt", "Hello, World!").unwrap();
    std::fs::write("input/file2.csv", "name,age\nJohn,25\nJane,30").unwrap();
    std::fs::write("input/file3.json", r#"{"name": "John", "age": 25}"#).unwrap();
    
    match processor.process_all_files() {
        Ok(summary) => {
            println!("Processing summary:");
            println!("  Success count: {}", summary.success_count);
            println!("  Error count: {}", summary.error_count);
            println!("  Processed files: {}", summary.processed_files.len());
            for error in summary.errors {
                println!("  Error: {} - {}", error.file, error.error);
            }
        }
        Err(e) => println!("Processing failed: {:?}", e),
    }
    
    // Clean up test files
    let _ = std::fs::remove_dir_all("input");
    let _ = std::fs::remove_dir_all("output");
    println!();
    
    println!("=== End of Day 34 Examples ===");
}
