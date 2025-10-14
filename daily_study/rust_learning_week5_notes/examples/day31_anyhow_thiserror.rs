// Day 31 - anyhow and thiserror
// Runnable examples demonstrating practical error handling crates


// Simulate anyhow and thiserror functionality with standard library
// In real usage, you would add these to Cargo.toml:
// anyhow = "1.0"
// thiserror = "1.0"

// Simulate anyhow::Result
type AnyhowResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

// Simulate anyhow::Context
#[allow(dead_code)]
trait Context<T, E> {
    fn context<C>(self, context: C) -> AnyhowResult<T>
    where
        C: std::fmt::Display + Send + Sync + 'static;
    
    fn with_context<C, F>(self, f: F) -> AnyhowResult<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E> Context<T, E> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> AnyhowResult<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
    {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(Box::new(SimpleError::new(format!("{}: {}", context, e)))),
        }
    }
    
    fn with_context<C, F>(self, f: F) -> AnyhowResult<T>
    where
        C: std::fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(Box::new(SimpleError::new(format!("{}: {}", f(), e)))),
        }
    }
}

// Simple error type to simulate anyhow
#[derive(Debug)]
struct SimpleError {
    message: String,
}

impl SimpleError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for SimpleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SimpleError {}

// Simulate thiserror with manual implementations
#[derive(Debug)]
#[allow(dead_code)]
enum ConfigError {
    FileNotFound { path: String },
    Io { source: std::io::Error },
    Parse { line: usize, message: String },
    Validation { field: String, reason: String },
    Config { message: String },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound { path } => write!(f, "File not found: {}", path),
            ConfigError::Io { source } => write!(f, "IO error: {}", source),
            ConfigError::Parse { line, message } => write!(f, "Parse error at line {}: {}", line, message),
            ConfigError::Validation { field, reason } => write!(f, "Validation error: {} - {}", field, reason),
            ConfigError::Config { message } => write!(f, "Configuration error: {}", message),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(source: std::io::Error) -> Self {
        ConfigError::Io { source }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum DatabaseError {
    ConnectionFailed,
    QueryTimeout,
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionFailed => write!(f, "Connection failed"),
            DatabaseError::QueryTimeout => write!(f, "Query timeout"),
        }
    }
}

impl std::error::Error for DatabaseError {}

#[derive(Debug)]
#[allow(dead_code)]
enum WebError {
    InvalidRequest { message: String },
    Database { source: DatabaseError },
    Auth { reason: String },
    RateLimited,
}

impl std::fmt::Display for WebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebError::InvalidRequest { message } => write!(f, "Invalid request: {}", message),
            WebError::Database { source } => write!(f, "Database error: {}", source),
            WebError::Auth { reason } => write!(f, "Authentication failed: {}", reason),
            WebError::RateLimited => write!(f, "Rate limit exceeded"),
        }
    }
}

impl std::error::Error for WebError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WebError::Database { source } => Some(source),
            _ => None,
        }
    }
}

impl From<DatabaseError> for WebError {
    fn from(source: DatabaseError) -> Self {
        WebError::Database { source }
    }
}

#[derive(Debug)]
enum ProcessingError {
    FileNotFound { path: String },
    InvalidFormat { expected: String, actual: String },
    ProcessingFailed { stage: String },
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessingError::FileNotFound { path } => write!(f, "File not found: {}", path),
            ProcessingError::InvalidFormat { expected, actual } => {
                write!(f, "Invalid file format: {}, got {}", expected, actual)
            }
            ProcessingError::ProcessingFailed { stage } => write!(f, "Processing failed: {}", stage),
        }
    }
}

impl std::error::Error for ProcessingError {}

// Data structures
#[derive(Debug)]
#[allow(dead_code)]
struct Config {
    host: String,
    port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
        }
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
struct Permissions {
    can_access_api: bool,
}

// Example functions demonstrating anyhow-style error handling
#[allow(dead_code)]
fn process_file(path: &str) -> AnyhowResult<String> {
    let content = std::fs::read_to_string(path)
        .context(format!("Failed to read file: {}", path))?;
    
    let processed = content.trim();
    if processed.is_empty() {
        return Err(Box::new(SimpleError::new(format!("File is empty: {}", path))));
    }
    
    Ok(processed.to_uppercase())
}

fn process_line(line: &str) -> AnyhowResult<()> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 2 {
        return Err(Box::new(SimpleError::new(format!(
            "Invalid format: expected 'key,value', got '{}'", line
        ))));
    }
    
    let key = parts[0].trim();
    let value = parts[1].trim().parse::<i32>()
        .with_context(|| format!("Invalid number: '{}'", parts[1]))?;
    
    println!("Key: {}, Value: {}", key, value);
    Ok(())
}

fn process_user_input(input: &str) -> AnyhowResult<()> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Err(Box::new(SimpleError::new("Input is empty".to_string())));
    }
    
    for (line_num, line) in lines.iter().enumerate() {
        process_line(line)
            .map_err(|e| anyhow::anyhow!("Failed to process line {} of input: {}", line_num + 1, e))?;
    }
    
    println!("Successfully processed {} lines", lines.len());
    Ok(())
}

// Example functions demonstrating thiserror-style error handling
impl Config {
    #[allow(dead_code)]
    pub fn load_from_file(path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content)
    }
    
    pub fn parse(content: &str) -> Result<Self, ConfigError> {
        let mut host = None;
        let mut port = None;
        
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((key, value)) = parse_key_value(line) {
                match key.as_str() {
                    "host" => {
                        if value.is_empty() {
                            return Err(ConfigError::Validation {
                                field: "host".to_string(),
                                reason: "Host cannot be empty".to_string(),
                            });
                        }
                        host = Some(value);
                    }
                    "port" => {
                        port = Some(value.parse::<u16>()
                            .map_err(|_| ConfigError::Parse {
                                line: line_num + 1,
                                message: format!("Invalid port number: {}", value),
                            })?);
                    }
                    _ => {
                        return Err(ConfigError::Parse {
                            line: line_num + 1,
                            message: format!("Unknown configuration key: {}", key),
                        });
                    }
                }
            } else {
                return Err(ConfigError::Parse {
                    line: line_num + 1,
                    message: "Invalid key=value format".to_string(),
                });
            }
        }
        
        Ok(Config {
            host: host.ok_or_else(|| ConfigError::Config {
                message: "Missing required field: host".to_string(),
            })?,
            port: port.unwrap_or(8080),
        })
    }
}

fn parse_key_value(line: &str) -> Option<(String, String)> {
    if let Some(pos) = line.find('=') {
        let key = line[..pos].trim().to_string();
        let value = line[pos + 1..].trim().to_string();
        Some((key, value))
    } else {
        None
    }
}

// Web server simulation
async fn fetch_user_from_database(user_id: u32) -> Result<User, WebError> {
    // Simulate database call
    if user_id == 0 {
        return Err(WebError::InvalidRequest {
            message: "Invalid user ID".to_string(),
        });
    }
    
    if user_id == 999 {
        return Err(WebError::Database {
            source: DatabaseError::ConnectionFailed,
        });
    }
    
    Ok(User {
        id: user_id,
        name: format!("User {}", user_id),
        email: format!("user{}@example.com", user_id),
    })
}

#[allow(dead_code)]
fn check_user_permissions(user: &User) -> Result<Permissions, anyhow::Error> {
    // Simulate permission check
    Ok(Permissions {
        can_access_api: !user.email.contains("blocked"),
    })
}

#[allow(dead_code)]
fn generate_response(user: &User) -> Result<String, anyhow::Error> {
    Ok(format!(r#"{{"id": {}, "name": "{}", "email": "{}"}}"#, 
               user.id, user.name, user.email))
}

// File processing pipeline
fn list_input_files(dir: &str) -> AnyhowResult<Vec<String>> {
    let mut files = Vec::new();
    
    // Simulate directory listing
    if dir == "empty" {
        return Ok(files);
    }
    
    files.push(format!("{}/data1.csv", dir));
    files.push(format!("{}/data2.csv", dir));
    
    Ok(files)
}

fn process_single_file(input_path: &str, output_dir: &str) -> Result<(), ProcessingError> {
    let content = std::fs::read_to_string(input_path)
        .map_err(|_| ProcessingError::FileNotFound {
            path: input_path.to_string(),
        })?;
    
    if !content.contains("CSV") {
        return Err(ProcessingError::InvalidFormat {
            expected: "CSV".to_string(),
            actual: "Unknown".to_string(),
        });
    }
    
    let processed = transform_data(&content)?;
    
    let output_path = format!("{}/processed_{}", 
                             output_dir, 
                             std::path::Path::new(input_path).file_name().unwrap().to_string_lossy());
    
    std::fs::write(&output_path, processed)
        .map_err(|_| ProcessingError::ProcessingFailed {
            stage: "Writing output file".to_string(),
        })?;
    
    println!("Processed: {} -> {}", input_path, output_path);
    Ok(())
}

fn transform_data(content: &str) -> Result<String, ProcessingError> {
    if content.contains("ERROR") {
        return Err(ProcessingError::ProcessingFailed {
            stage: "Data transformation".to_string(),
        });
    }
    
    Ok(content.to_uppercase())
}

fn main() {
    println!("=== Day 31: anyhow and thiserror Examples ===\n");
    
    // Example 1: anyhow-style error handling
    println!("1. anyhow-style Error Handling:");
    let input_data = "key1,100\nkey2,200\nkey3,invalid";
    match process_user_input(input_data) {
        Ok(()) => println!("Processing completed successfully"),
        Err(e) => println!("Processing failed: {}", e),
    }
    println!();
    
    // Example 2: thiserror-style error handling
    println!("2. thiserror-style Error Handling:");
    let config_content = "host=example.com\nport=9090\ninvalid_key=should_fail";
    match Config::parse(config_content) {
        Ok(config) => println!("Config parsed: {:?}", config),
        Err(e) => println!("Config error: {}", e),
    }
    println!();
    
    // Example 3: Web server error handling
    println!("3. Web Server Error Handling:");
    match futures::executor::block_on(fetch_user_from_database(123)) {
        Ok(user) => println!("User fetched: {:?}", user),
        Err(e) => println!("Fetch error: {}", e),
    }
    
    // Simulate database error
    match futures::executor::block_on(fetch_user_from_database(999)) {
        Ok(user) => println!("User fetched: {:?}", user),
        Err(e) => println!("Fetch error: {}", e),
    }
    println!();
    
    // Example 4: File processing pipeline
    println!("4. File Processing Pipeline:");
    let files = match list_input_files("data") {
        Ok(files) => files,
        Err(e) => {
            println!("Failed to list files: {}", e);
            Vec::new()
        }
    };
    
    for file in files {
        match process_single_file(&file, "output") {
            Ok(()) => println!("File processed successfully"),
            Err(e) => println!("File processing failed: {}", e),
        }
    }
    println!();
    
    println!("=== End of Day 31 Examples ===");
}
