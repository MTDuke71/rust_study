# ⚡ Error Propagation

*Mastering the art of efficiently and safely propagating errors through Rust code*

---

## 🎯 **Core Concept**

Error propagation is the process of passing errors up the call stack without losing context or information. In Rust, this is primarily achieved through the `Result<T, E>` type and the `?` operator, enabling clean error handling without the complexity of traditional exception mechanisms.

### **The Error Propagation Spectrum**
```rust
// Manual propagation (verbose)
match operation() {
    Ok(value) => value,
    Err(e) => return Err(e),
}

// ? operator (ergonomic)
let value = operation()?;

// With context (anyhow)
let value = operation().context("Operation failed")?;

// With conversion (thiserror)
let value = operation().map_err(|e| MyError::from(e))?;
```

## 🔧 **The ? Operator: Rust's Propagation Superpower**

### **Basic Mechanics**
```rust
use std::fs::File;
use std::io::Read;

// Without ? operator (manual propagation)
fn read_file_manual(path: &str) -> Result<String, std::io::Error> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

// With ? operator (ergonomic propagation)
fn read_file_elegant(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Even more concise
fn read_file_concise(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
```

### **? Operator Mechanics Deep Dive**
```rust
// What ? operator actually does
let result = operation()?;

// Expands to approximately:
let result = match operation() {
    Ok(value) => value,
    Err(e) => return Err(e.into()), // Note: .into() for conversion
};
```

## 🔄 **Error Conversion and Propagation**

### **Automatic Error Conversion**
```rust
use std::num::ParseIntError;
use std::io::Error as IoError;

// Error types that need conversion
#[derive(Debug)]
enum AppError {
    Io(IoError),
    Parse(ParseIntError),
    Custom(String),
}

// Manual conversion implementations
impl From<IoError> for AppError {
    fn from(err: IoError) -> Self {
        AppError::Io(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::Parse(err)
    }
}

// Now ? operator works with conversions
fn process_file(path: &str) -> Result<i32, AppError> {
    let content = std::fs::read_to_string(path)?; // IoError -> AppError
    let number = content.trim().parse::<i32>()?;  // ParseIntError -> AppError
    Ok(number * 2)
}
```

### **thiserror for Automatic Conversions**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("IO operation failed")]
    Io(#[from] std::io::Error),  // Automatic From implementation
    
    #[error("Failed to parse number")]
    Parse(#[from] std::num::ParseIntError),  // Automatic From implementation
    
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    #[error("Network timeout after {seconds}s")]
    NetworkTimeout { seconds: u64 },
}

// Clean propagation with automatic conversions
fn process_data(path: &str) -> Result<Vec<i32>, ProcessError> {
    let content = std::fs::read_to_string(path)?;  // Auto-converts IoError
    
    let mut numbers = Vec::new();
    for line in content.lines() {
        let num = line.trim().parse::<i32>()?;  // Auto-converts ParseIntError
        
        // Manual error creation for custom logic
        if num < 0 {
            return Err(ProcessError::Validation {
                message: format!("Negative number not allowed: {}", num),
            });
        }
        
        numbers.push(num);
    }
    
    Ok(numbers)
}
```

## 🏗️ **Propagation Patterns and Strategies**

### **Early Return Pattern**
```rust
fn validate_user_input(input: &UserInput) -> Result<ValidatedInput, ValidationError> {
    // Chain of validations with early returns
    let email = validate_email(&input.email)?;
    let age = validate_age(input.age)?;
    let password = validate_password(&input.password)?;
    
    // All validations passed
    Ok(ValidatedInput { email, age, password })
}

fn validate_email(email: &str) -> Result<String, ValidationError> {
    if email.is_empty() {
        return Err(ValidationError::EmptyEmail);
    }
    
    if !email.contains('@') {
        return Err(ValidationError::InvalidEmailFormat);
    }
    
    Ok(email.to_string())
}
```

### **Accumulating Errors Pattern**
```rust
// When you want to collect ALL errors, not just the first one
fn validate_all_fields(input: &UserInput) -> Result<ValidatedInput, Vec<ValidationError>> {
    let mut errors = Vec::new();
    
    let email = match validate_email(&input.email) {
        Ok(email) => Some(email),
        Err(e) => {
            errors.push(e);
            None
        }
    };
    
    let age = match validate_age(input.age) {
        Ok(age) => Some(age),
        Err(e) => {
            errors.push(e);
            None
        }
    };
    
    if !errors.is_empty() {
        return Err(errors);
    }
    
    Ok(ValidatedInput {
        email: email.unwrap(),
        age: age.unwrap(),
        password: input.password.clone(),
    })
}

// Using iterators for cleaner accumulation
fn validate_numbers(inputs: &[&str]) -> Result<Vec<i32>, Vec<String>> {
    let mut numbers = Vec::new();
    let mut errors = Vec::new();
    
    for (i, input) in inputs.iter().enumerate() {
        match input.parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(e) => errors.push(format!("Line {}: {}", i + 1, e)),
        }
    }
    
    if errors.is_empty() {
        Ok(numbers)
    } else {
        Err(errors)
    }
}
```

### **Chain Propagation with Context**
```rust
use anyhow::{Result, Context};

fn process_config_pipeline(config_path: &str) -> Result<AppConfig> {
    let raw_config = std::fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read config from {}", config_path))?;
    
    let parsed_config: TomlConfig = toml::from_str(&raw_config)
        .context("Failed to parse TOML configuration")?;
    
    let validated_config = validate_config(parsed_config)
        .context("Configuration validation failed")?;
    
    let normalized_config = normalize_paths(validated_config)
        .context("Failed to normalize configuration paths")?;
    
    let final_config = apply_defaults(normalized_config)
        .context("Failed to apply default configuration values")?;
    
    Ok(final_config)
}
```

## 🎯 **Mission Integration Applications**

### **Mission2: Queue Error Propagation**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueueError {
    #[error("Queue is full (capacity: {capacity})")]
    QueueFull { capacity: usize },
    
    #[error("Queue is empty")]
    QueueEmpty,
    
    #[error("Invalid capacity: {capacity} (must be > 0)")]
    InvalidCapacity { capacity: usize },
    
    #[error("Index out of bounds: {index} >= {size}")]
    IndexOutOfBounds { index: usize, size: usize },
}

impl<T> RingBufferQueue<T> {
    pub fn enqueue(&mut self, item: T) -> Result<(), QueueError> {
        if self.is_full() {
            return Err(QueueError::QueueFull { 
                capacity: self.capacity 
            });
        }
        
        // Implementation details...
        Ok(())
    }
    
    pub fn dequeue(&mut self) -> Result<T, QueueError> {
        if self.is_empty() {
            return Err(QueueError::QueueEmpty);
        }
        
        // Implementation details...
    }
    
    // Chain operations with propagation
    pub fn process_batch(&mut self, items: Vec<T>) -> Result<Vec<T>, QueueError> {
        let mut results = Vec::new();
        
        // Enqueue all items first
        for item in items {
            self.enqueue(item)?;  // Propagate any queue full errors
        }
        
        // Process all items
        while !self.is_empty() {
            let processed = self.dequeue()?;  // Propagate any empty errors
            results.push(processed);
        }
        
        Ok(results)
    }
}
```

### **Mission5: HashMap Error Propagation**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashMapError {
    #[error("Key not found: {key:?}")]
    KeyNotFound { key: String },
    
    #[error("Hash computation failed")]
    HashFailure(#[from] HashError),
    
    #[error("Capacity limit reached: {current}/{max}")]
    CapacityExceeded { current: usize, max: usize },
    
    #[error("Invalid hash table state")]
    InvalidState(#[from] StateError),
}

impl<K, V> HashMap<K, V> 
where 
    K: Hash + Eq + Clone + std::fmt::Debug,
{
    pub fn get(&self, key: &K) -> Result<&V, HashMapError> {
        let hash = self.hash_key(key)?;  // Propagate hash errors
        let bucket = self.find_bucket(hash)?;  // Propagate state errors
        
        bucket.find(key)
            .ok_or_else(|| HashMapError::KeyNotFound { 
                key: format!("{:?}", key) 
            })
    }
    
    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, HashMapError> {
        if self.should_resize()? {  // Propagate capacity errors
            self.resize()?;  // Propagate resize errors
        }
        
        let hash = self.hash_key(&key)?;  // Propagate hash errors
        let bucket = self.find_bucket_mut(hash)?;  // Propagate state errors
        
        Ok(bucket.insert(key, value))
    }
    
    // Complex operation with multiple error sources
    pub fn bulk_insert(&mut self, items: Vec<(K, V)>) -> Result<usize, HashMapError> {
        let mut inserted_count = 0;
        
        for (key, value) in items {
            match self.insert(key, value) {
                Ok(_) => inserted_count += 1,
                Err(HashMapError::CapacityExceeded { .. }) => {
                    // Try to resize and retry
                    self.resize()?;
                    self.insert(key, value)?;
                    inserted_count += 1;
                }
                Err(e) => return Err(e),  // Propagate other errors
            }
        }
        
        Ok(inserted_count)
    }
}
```

### **AoC Problem Error Propagation**
```rust
use anyhow::{Result, Context, bail, ensure};

fn solve_aoc_day12(input: &str) -> Result<(i32, i32)> {
    let data = parse_input(input)
        .context("Failed to parse input data")?;
    
    let part1 = solve_part1(&data)
        .context("Part 1 solution failed")?;
    
    let part2 = solve_part2(&data)
        .context("Part 2 solution failed")?;
    
    Ok((part1, part2))
}

fn parse_input(input: &str) -> Result<GameData> {
    ensure!(!input.trim().is_empty(), "Input cannot be empty");
    
    let mut nodes = Vec::new();
    
    for (line_num, line) in input.lines().enumerate() {
        let node = parse_line(line)
            .with_context(|| format!("Failed to parse line {}: '{}'", line_num + 1, line))?;
        nodes.push(node);
    }
    
    if nodes.is_empty() {
        bail!("No valid nodes found in input");
    }
    
    Ok(GameData { nodes })
}

fn parse_line(line: &str) -> Result<Node> {
    let parts: Vec<&str> = line.split(',').collect();
    
    ensure!(parts.len() >= 2, "Line must have at least 2 comma-separated values");
    
    let x = parts[0].trim().parse::<i32>()
        .with_context(|| format!("Invalid x coordinate: '{}'", parts[0]))?;
    
    let y = parts[1].trim().parse::<i32>()
        .with_context(|| format!("Invalid y coordinate: '{}'", parts[1]))?;
    
    let node_type = if parts.len() > 2 {
        parse_node_type(parts[2])?
    } else {
        NodeType::Default
    };
    
    Ok(Node { x, y, node_type })
}
```

## 🚀 **Advanced Propagation Techniques**

### **Result Chaining and Composition**
```rust
// Functional style error propagation
fn process_user_pipeline(user_id: u64) -> Result<ProcessedUser, AppError> {
    fetch_user(user_id)?
        .validate()?
        .enrich_with_profile()?
        .apply_business_rules()?
        .save_to_cache()
}

// Result extension trait for chaining
trait ResultExt<T, E> {
    fn and_then_log<F, U>(self, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> Result<U, E>;
    
    fn log_error(self) -> Self;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn and_then_log<F, U>(self, f: F) -> Result<U, E>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        match self {
            Ok(value) => {
                log::debug!("Operation succeeded, continuing chain");
                f(value)
            }
            Err(e) => {
                log::error!("Chain operation failed: {}", e);
                Err(e)
            }
        }
    }
    
    fn log_error(self) -> Self {
        if let Err(ref e) = self {
            log::error!("Operation failed: {}", e);
        }
        self
    }
}
```

### **Parallel Error Propagation**
```rust
use std::thread;
use std::sync::mpsc;

fn process_parallel_tasks(tasks: Vec<Task>) -> Result<Vec<TaskResult>, Vec<TaskError>> {
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    
    // Spawn parallel workers
    for (i, task) in tasks.into_iter().enumerate() {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            let result = match process_task(task) {
                Ok(result) => Ok((i, result)),
                Err(e) => Err((i, e)),
            };
            tx.send(result).unwrap();
        });
        handles.push(handle);
    }
    
    drop(tx); // Close the sending side
    
    // Collect results
    let mut results = Vec::new();
    let mut errors = Vec::new();
    
    for received in rx {
        match received {
            Ok((i, result)) => results.push((i, result)),
            Err((i, error)) => errors.push((i, error)),
        }
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    if errors.is_empty() {
        results.sort_by_key(|(i, _)| *i);
        Ok(results.into_iter().map(|(_, result)| result).collect())
    } else {
        errors.sort_by_key(|(i, _)| *i);
        Err(errors.into_iter().map(|(_, error)| error).collect())
    }
}
```

### **Async Error Propagation**
```rust
use tokio;
use anyhow::{Result, Context};

async fn async_error_propagation() -> Result<ProcessedData> {
    let config = load_config().await
        .context("Failed to load configuration")?;
    
    let data = fetch_remote_data(&config.api_url).await
        .context("Failed to fetch remote data")?;
    
    let processed = process_data_async(data).await
        .context("Failed to process data")?;
    
    save_results(&processed).await
        .context("Failed to save results")?;
    
    Ok(processed)
}

// Handling multiple async operations
async fn fetch_multiple_sources() -> Result<CombinedData> {
    // Sequential with early return on error
    let source1 = fetch_source1().await?;
    let source2 = fetch_source2().await?;
    let source3 = fetch_source3().await?;
    
    Ok(CombinedData::new(source1, source2, source3))
}

// Parallel async with error propagation
async fn fetch_parallel_sources() -> Result<CombinedData> {
    let (result1, result2, result3) = tokio::try_join!(
        fetch_source1(),
        fetch_source2(),
        fetch_source3()
    )?;
    
    Ok(CombinedData::new(result1, result2, result3))
}
```

## 🧪 **Testing Error Propagation**

### **Unit Testing Error Paths**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_propagation_chain() {
        let result = process_file("nonexistent.txt");
        
        // Test that error propagates correctly
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        
        // Test error type
        match error {
            AppError::Io(_) => {
                // Expected path for file operations
            }
            _ => panic!("Expected IoError, got {:?}", error),
        }
        
        // Test error chain
        let error_chain: Vec<String> = std::iter::successors(
            Some(&error as &dyn std::error::Error),
            |e| e.source(),
        )
        .map(|e| e.to_string())
        .collect();
        
        assert!(!error_chain.is_empty());
        assert!(error_chain[0].contains("No such file"));
    }
    
    #[test]
    fn test_error_conversion() {
        // Test that automatic conversion works
        let parse_error = "not_a_number".parse::<i32>().unwrap_err();
        let app_error: AppError = parse_error.into();
        
        match app_error {
            AppError::Parse(_) => {
                // Expected conversion
            }
            _ => panic!("Conversion failed"),
        }
    }
    
    #[test]
    fn test_accumulating_errors() {
        let inputs = vec!["1", "not_a_number", "3", "also_not_a_number"];
        let result = validate_numbers(&inputs);
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2); // Two parsing errors
        
        assert!(errors[0].contains("Line 2"));
        assert!(errors[1].contains("Line 4"));
    }
}
```

### **Integration Testing with Error Scenarios**
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    
    #[test]
    fn test_full_pipeline_with_errors() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        
        // Test with invalid config
        fs::write(&config_path, "invalid toml content [[[").unwrap();
        
        let result = process_config_pipeline(config_path.to_str().unwrap());
        assert!(result.is_err());
        
        let error_msg = format!("{:#}", result.unwrap_err());
        assert!(error_msg.contains("Failed to parse TOML"));
    }
    
    #[test]
    fn test_error_context_preservation() {
        let result = process_user_data(999999); // Non-existent user
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        
        // Check that context is preserved through the chain
        let full_error = format!("{:#}", error);
        assert!(full_error.contains("user 999999"));
        assert!(full_error.contains("Failed to fetch user"));
    }
}
```

## 📊 **Performance Considerations**

### **Error Propagation Overhead**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_error_propagation(c: &mut Criterion) {
    c.bench_function("manual_propagation", |b| {
        b.iter(|| {
            let result = manual_error_chain();
            black_box(result)
        })
    });
    
    c.bench_function("question_mark_propagation", |b| {
        b.iter(|| {
            let result = question_mark_chain();
            black_box(result)
        })
    });
    
    c.bench_function("anyhow_context_propagation", |b| {
        b.iter(|| {
            let result = anyhow_context_chain();
            black_box(result)
        })
    });
}

// The ? operator is typically zero-cost when no errors occur
fn question_mark_chain() -> Result<i32, std::io::Error> {
    let value1 = may_fail_operation()?;
    let value2 = another_operation(value1)?;
    let value3 = final_operation(value2)?;
    Ok(value3)
}
```

### **Memory Efficiency**
```rust
// Efficient error types avoid unnecessary allocations
#[derive(Error, Debug)]
pub enum EfficientError {
    #[error("Static message")]
    StaticMessage,
    
    #[error("Parameterized: {value}")]
    Parameterized { value: u32 }, // No String allocation
    
    #[error("With source")]
    WithSource(#[from] SomeOtherError), // Zero-cost forwarding
}

// Avoid when possible (allocates)
#[derive(Error, Debug)]
pub enum ExpensiveError {
    #[error("Dynamic message: {msg}")]
    Dynamic { msg: String }, // Heap allocation
}
```

## 🔗 **Integration with Learning Tracks**

### **Daily Study Applications**
- **Week 5**: Advanced error handling patterns and propagation strategies
- **Error analysis**: Building error banks for common patterns
- **AoC solutions**: Robust error propagation in competitive programming

### **Mission Applications**
- **All Missions**: Consistent error propagation patterns
- **Mission5**: HashMap error handling with proper propagation
- **Mission7**: Graph algorithm error handling and propagation

### **Rust Book Connections**
- **Chapter 9**: `Result<T, E>` and `?` operator fundamentals
- **Chapter 10**: Trait bounds and error conversion patterns
- **Chapter 16**: Error handling in concurrent contexts

## 📚 **Best Practices Summary**

### **✅ Do**
- Use the `?` operator for clean error propagation
- Implement `From` traits for automatic error conversion
- Add context at meaningful boundaries
- Design error types that compose well
- Test error propagation paths explicitly
- Use `thiserror` for structured library errors
- Use `anyhow` for application error context

### **❌ Don't**
- Ignore errors with `.unwrap()` in production code
- Create overly complex error hierarchies
- Lose error context during propagation
- Mix different error handling paradigms inconsistently
- Propagate errors without adding value
- Create error types that are expensive to construct

## 🎓 **Learning Progression**

### **Beginner**
1. Master the `?` operator mechanics
2. Understand `Result<T, E>` composition
3. Practice basic error propagation patterns

### **Intermediate**
1. Implement custom error types with `thiserror`
2. Use `anyhow` for application error context
3. Design error handling for mission projects

### **Advanced**
1. Optimize error propagation performance
2. Handle errors in async and concurrent contexts
3. Design composable error handling systems

---

*Tags: #error-propagation #result-type #question-mark-operator #error-conversion #error-handling #anyhow #thiserror #mission-integration #performance #async-errors #testing #rust-patterns*

*Links: [[zettel-index]] | [[anyhow and thiserror]] | [[Result Type]] | [[Error Handling Patterns]] | [[Error Handling Deep Dive]] | [[mission-5]] | [[Testing Strategies]] | [[Performance Optimization]]*