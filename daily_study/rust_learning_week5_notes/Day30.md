# Day 30 - Error Propagation

**Learning Focus**: The `?` operator, error conversion, and error forwarding patterns

---

## 🎯 Learning Objectives

By the end of this day, you should understand:
- How the `?` operator works and when to use it
- Error conversion patterns with `From` trait
- Error propagation vs error handling strategies
- When to propagate vs when to handle errors locally
- Error context preservation during propagation
- Best practices for error propagation chains

---

## 📚 Core Concepts

### **The `?` Operator**

The `?` operator is syntactic sugar for early return on errors:

```rust
// ❌ WITHOUT ? operator (verbose)
fn read_file_content(path: &str) -> Result<String, std::io::Error> {
    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut contents = String::new();
    match std::io::Read::read_to_string(&mut file, &mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

// ✅ WITH ? operator (concise)
fn read_file_content(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;
    Ok(contents)
}
```

### **Error Conversion with `From`**

```rust
use std::error::Error;
use std::fmt;

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

// Now we can use ? with automatic conversion
fn process_file(path: &str) -> Result<i32, AppError> {
    let content = std::fs::read_to_string(path)?; // std::io::Error -> AppError
    let number = content.trim().parse::<i32>()?;  // ParseIntError -> AppError
    Ok(number * 2)
}
```

---

## 🔧 Implementation Patterns

### **Pattern 1: Simple Propagation**

```rust
fn read_config() -> Result<Config, AppError> {
    let content = std::fs::read_to_string("config.txt")?;
    let config = parse_config(&content)?;
    Ok(config)
}

fn parse_config(content: &str) -> Result<Config, AppError> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Err(AppError::Custom("Empty config file".to_string()));
    }
    
    let port = lines[0].parse::<u16>()?;
    Ok(Config { port })
}
```

### **Pattern 2: Error Context Preservation**

```rust
fn process_user_input(input: &str) -> Result<UserData, AppError> {
    let lines: Vec<&str> = input.lines().collect();
    
    for (line_num, line) in lines.iter().enumerate() {
        if let Err(e) = process_line(line) {
            return Err(AppError::Custom(format!(
                "Error processing line {}: {}", 
                line_num + 1, e
            )));
        }
    }
    
    Ok(UserData::default())
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
```

### **Pattern 3: Selective Error Handling**

```rust
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
    
    let lines: Vec<String> = content.lines()
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
```

### **Pattern 4: Error Aggregation**

```rust
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

fn process_single_file(path: &str) -> Result<ProcessedFile, AppError> {
    let content = std::fs::read_to_string(path)?;
    let processed = content.to_uppercase();
    Ok(ProcessedFile { path: path.to_string(), content: processed })
}

#[derive(Debug)]
struct ProcessedFile {
    path: String,
    content: String,
}
```

---

## 🎮 Practical Applications

### **Application 1: Web Request Chain**

```rust
use std::collections::HashMap;

#[derive(Debug)]
enum WebError {
    Network(std::io::Error),
    Parse(serde_json::Error),
    Http(u16),
    Auth(String),
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebError::Network(e) => write!(f, "Network error: {}", e),
            WebError::Parse(e) => write!(f, "Parse error: {}", e),
            WebError::Http(code) => write!(f, "HTTP error: {}", code),
            WebError::Auth(msg) => write!(f, "Auth error: {}", msg),
        }
    }
}

impl Error for WebError {}

impl From<std::io::Error> for WebError {
    fn from(error: std::io::Error) -> Self {
        WebError::Network(error)
    }
}

impl From<serde_json::Error> for WebError {
    fn from(error: serde_json::Error) -> Self {
        WebError::Parse(error)
    }
}

// Simulate web operations
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
    let parsed: HashMap<String, serde_json::Value> = serde_json::from_str(response)?;
    Ok(parsed)
}

fn fetch_user_data(user_id: u32) -> Result<UserData, WebError> {
    let url = format!("https://api.example.com/users/{}", user_id);
    let response = make_request(&url)?;
    let data = parse_json_response(&response)?;
    
    let user_id = data.get("data")
        .and_then(|d| d.get("user_id"))
        .and_then(|v| v.as_u64())
        .ok_or_else(|| WebError::Parse(serde_json::Error::custom("Missing user_id")))?;
    
    Ok(UserData { id: user_id as u32, name: "John Doe".to_string() })
}

#[derive(Debug)]
struct UserData {
    id: u32,
    name: String,
}
```

### **Application 2: Database Transaction Chain**

```rust
#[derive(Debug)]
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
            return Err(DatabaseError::Constraint("Unique constraint violated".to_string()));
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
    
    fn rollback(self) -> Result<(), DatabaseError> {
        // Simulate rollback
        Ok(())
    }
}

struct QueryResult {
    rows_affected: usize,
}

fn transfer_money(from_account: u32, to_account: u32, amount: i32) -> Result<(), DatabaseError> {
    let db = Database::connect()?;
    let mut tx = db.begin_transaction()?;
    
    // Debit from source account
    tx.execute(&format!("UPDATE accounts SET balance = balance - {} WHERE id = {}", amount, from_account))?;
    
    // Credit to destination account
    tx.execute(&format!("UPDATE accounts SET balance = balance + {} WHERE id = {}", amount, to_account))?;
    
    // Commit transaction
    tx.commit()?;
    
    Ok(())
}
```

---

## 🧪 Error Propagation Best Practices

### **Best Practice 1: Early Returns**

```rust
// ✅ GOOD: Early return with ?
fn process_data(input: &str) -> Result<ProcessedData, AppError> {
    let parsed = parse_input(input)?;
    let validated = validate_data(&parsed)?;
    let processed = transform_data(validated)?;
    Ok(processed)
}

// ❌ AVOID: Nested match statements
fn process_data_bad(input: &str) -> Result<ProcessedData, AppError> {
    match parse_input(input) {
        Ok(parsed) => {
            match validate_data(&parsed) {
                Ok(validated) => {
                    match transform_data(validated) {
                        Ok(processed) => Ok(processed),
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}
```

### **Best Practice 2: Context Preservation**

```rust
// ✅ GOOD: Adding context during propagation
fn load_user_profile(user_id: u32) -> Result<UserProfile, AppError> {
    let user_data = fetch_user_data(user_id)
        .map_err(|e| AppError::Custom(format!("Failed to fetch user {}: {}", user_id, e)))?;
    
    let preferences = load_user_preferences(user_id)
        .map_err(|e| AppError::Custom(format!("Failed to load preferences for user {}: {}", user_id, e)))?;
    
    Ok(UserProfile { user_data, preferences })
}
```

### **Best Practice 3: Error Recovery**

```rust
// ✅ GOOD: Attempting recovery before propagation
fn load_config_with_fallback(path: &str) -> Result<Config, AppError> {
    match std::fs::read_to_string(path) {
        Ok(content) => parse_config(&content),
        Err(_) => {
            // Try fallback location
            let fallback_path = format!("{}.default", path);
            match std::fs::read_to_string(&fallback_path) {
                Ok(content) => parse_config(&content),
                Err(e) => Err(AppError::from(e)),
            }
        }
    }
}
```

---

## 🎯 Key Takeaways

1. **Use `?` for early returns** - Much cleaner than nested match statements
2. **Implement `From` conversions** - Enables automatic error conversion
3. **Preserve error context** - Add information about what was being attempted
4. **Consider error recovery** - Try alternative approaches before propagating
5. **Aggregate related errors** - Collect multiple errors before failing
6. **Handle specific error cases** - Don't always propagate immediately

---

## 🔗 Related Concepts

- **[[Custom Error Types]]** - Creating domain-specific error types
- **[[Result Type]]** - The Result<T, E> type for error handling
- **[[anyhow and thiserror]]** - Advanced error handling crates
- **[[Result Combinators]]** - Functional error handling methods
- **[[Error Handling Patterns]]** - When to propagate vs handle

---

*Tags: #error-handling #error-propagation #question-mark-operator #from-trait #error-conversion #early-return*

*Links: [[zettel-index]] | [[Custom Error Types]] | [[Result Type]] | [[anyhow and thiserror]] | [[Result Combinators]] | [[Week 5 Overview]]*
