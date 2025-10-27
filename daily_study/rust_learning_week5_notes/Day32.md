# Day 32 - Result Combinators

**Learning Focus**: Functional error handling methods (`and_then`, `or_else`, `map_err`)

---

## 🎯 Learning Objectives

By the end of this day, you should understand:
- How to use `and_then` for chaining operations that return Results
- When and how to use `or_else` for error recovery
- How `map_err` transforms error types
- Other useful Result combinators (`map`, `unwrap_or`, `unwrap_or_else`)
- Functional programming patterns for error handling
- When to use combinators vs the `?` operator

---

## 📚 Core Concepts

### **The Result Combinator Family**

```rust
// map: Transform success value
let result: Result<i32, &str> = Ok(42);
let doubled: Result<String, &str> = result.map(|x| (x * 2).to_string());

// map_err: Transform error value
let result: Result<i32, &str> = Err("invalid input");
let mapped: Result<i32, String> = result.map_err(|e| format!("Error: {}", e));

// and_then: Chain operations that return Results
let result: Result<i32, &str> = Ok(5);
let chained: Result<String, &str> = result
    .and_then(|x| if x > 0 { Ok(x.to_string()) } else { Err("negative") })
    .and_then(|s| Ok(format!("Number: {}", s)));

// or_else: Handle errors with fallback operations
let result: Result<i32, &str> = Err("failed");
let recovered: Result<i32, &str> = result.or_else(|_| Ok(42));
```

### **Functional Error Handling Patterns**

```rust
// Pipeline processing with combinators
fn process_user_data(input: &str) -> Result<User, ProcessError> {
    parse_input(input)
        .and_then(validate_user)
        .and_then(enrich_user_data)
        .map_err(|e| ProcessError::from(e))
}

fn parse_input(input: &str) -> Result<RawUser, ParseError> {
    // Parsing logic
}

fn validate_user(raw: RawUser) -> Result<ValidatedUser, ValidationError> {
    // Validation logic
}

fn enrich_user_data(user: ValidatedUser) -> Result<User, EnrichmentError> {
    // Enrichment logic
}
```

---

## 🔧 Implementation Patterns

### **Pattern 1: Chaining Transformations**

```rust
fn process_numbers(input: &str) -> Result<Vec<i32>, ProcessError> {
    input
        .split(',')
        .map(|s| s.trim().parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .map_err(|e| ProcessError::ParseError(e))
        .and_then(|numbers| {
            if numbers.is_empty() {
                Err(ProcessError::EmptyInput)
            } else {
                Ok(numbers)
            }
        })
        .map(|numbers| numbers.into_iter().map(|n| n * 2).collect())
}

#[derive(Debug)]
enum ProcessError {
    ParseError(std::num::ParseIntError),
    EmptyInput,
}
```

### **Pattern 2: Error Recovery**

```rust
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
```

### **Pattern 3: Conditional Processing**

```rust
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

#[derive(Debug)]
struct ProcessOptions {
    validate: bool,
    transform: bool,
    normalize: bool,
}

#[derive(Debug)]
struct RawData {
    content: String,
}

#[derive(Debug)]
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

#[derive(Debug)]
enum ProcessError {
    ValidationError(String),
    TransformError(String),
}
```

---

## 🎮 Practical Applications

### **Application 1: API Request Pipeline**

```rust
use std::collections::HashMap;

fn fetch_user_profile(user_id: u32) -> Result<UserProfile, ApiError> {
    validate_user_id(user_id)
        .and_then(|id| fetch_user_data(id))
        .and_then(|user| fetch_user_preferences(user.id))
        .and_then(|(user, prefs)| fetch_user_avatar(user.id).map(|avatar| (user, prefs, avatar)))
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

fn fetch_user_preferences(user_id: u32) -> Result<HashMap<String, String>, ApiError> {
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

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug)]
struct UserProfile {
    user: User,
    preferences: HashMap<String, String>,
    avatar_url: String,
}

#[derive(Debug)]
enum ApiError {
    InvalidId,
    NotFound,
    NetworkError,
    ParseError,
}
```

### **Application 2: File Processing Pipeline**

```rust
fn process_file_pipeline(input_path: &str, output_path: &str) -> Result<ProcessedFile, PipelineError> {
    read_file(input_path)
        .and_then(validate_file_format)
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
        .map_err(|e| PipelineError::ProcessingFailed(e))
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
    
    for (line_num, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            return Err(PipelineError::ParseError(format!(
                "Invalid record format at line {}: expected 3 fields, got {}", 
                line_num + 1, parts.len()
            )));
        }
        
        records.push(Record {
            id: parts[0].to_string(),
            name: parts[1].to_string(),
            value: parts[2].to_string(),
        });
    }
    
    if records.is_empty() {
        Err(PipelineError::NoValidRecords)
    } else {
        Ok(records)
    }
}

fn transform_content(records: Vec<Record>) -> Result<String, PipelineError> {
    let transformed: Result<Vec<String>, _> = records
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

#[derive(Debug)]
struct Record {
    id: String,
    name: String,
    value: String,
}

#[derive(Debug)]
struct ProcessedFile {
    input_path: String,
    output_path: String,
    size: u64,
}

#[derive(Debug)]
enum PipelineError {
    FileNotFound(String),
    EmptyFile,
    InvalidFormat(String),
    ParseError(String),
    NoValidRecords,
    WriteError(String),
    ProcessingFailed(PipelineError),
}
```

---

## 🧪 Advanced Combinator Patterns

### **Pattern 4: Error Accumulation**

```rust
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

#[derive(Debug)]
struct RawUser {
    name: String,
    email: String,
    age: u32,
}

#[derive(Debug)]
struct ValidatedUser {
    name: String,
    email: String,
    age: u32,
}

#[derive(Debug)]
enum ValidationError {
    EmptyName,
    NameTooLong,
    InvalidEmail,
    TooYoung,
    TooOld,
}
```

### **Pattern 5: Optional Error Recovery**

```rust
fn process_with_recovery(input: &str) -> Result<String, ProcessError> {
    parse_input(input)
        .map_err(|e| ProcessError::ParseError(e))
        .or_else(|e| {
            // Try to recover by cleaning the input
            let cleaned = clean_input(input);
            parse_input(&cleaned)
                .map_err(|_| e)
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

#[derive(Debug)]
struct ParsedData {
    content: String,
}

#[derive(Debug)]
enum ProcessError {
    ParseError(String),
    ProcessingError,
}
```

---

## 🎯 Key Takeaways

1. **Use `and_then` for chaining** - Operations that return Results
2. **Use `or_else` for recovery** - Provide fallback operations
3. **Use `map_err` for error transformation** - Convert error types
4. **Combine combinators for complex logic** - Build sophisticated error handling pipelines
5. **Consider when to use combinators vs `?`** - Combinators for functional style, `?` for early returns
6. **Accumulate errors when needed** - Don't always fail fast

---

## 🔗 Related Concepts

- **[[Error Propagation]]** - The ? operator and error forwarding
- **[[anyhow and thiserror]]** - Advanced error handling crates
- **[[Error Handling Patterns]]** - When to propagate vs handle
- **[[Error Handling Practice]]** - Building robust error handling systems
- **[[Functional Programming]]** - Functional error handling patterns
- **[[../../zettelkasten/daily-study/Day32]]** - Zettelkasten redirect page

---

## 🔗 Navigation

**Previous**: [[Day31]] | **Next**: [[Day33]]

**Week Overview**: [[README|Week 5 Overview]]

**Zettelkasten**: [[../../zettelkasten/daily-study/Day32|Day32 (Zettelkasten)]]

---

*Tags: #error-handling #result-combinators #functional-programming #and-then #or-else #map-err #error-recovery*

*Links: [[zettel-index]] | [[Error Propagation]] | [[anyhow and thiserror]] | [[Error Handling Patterns]] | [[Error Handling Practice]] | [[Week 5 Overview]]*
