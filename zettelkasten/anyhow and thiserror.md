# 🚨 anyhow and thiserror

*Comprehensive guide to Rust's premier error handling crates for application and library development*

---

## 🎯 **Overview**

`anyhow` and `thiserror` are complementary crates that revolutionize error handling in Rust, each serving distinct use cases in the error handling ecosystem.

### **Quick Decision Matrix**

| Use Case | Choose | Why |
|----------|--------|-----|
| **Application development** | `anyhow` | Easy error propagation, rich context |
| **Library development** | `thiserror` | Structured errors, stable API |
| **Prototyping/scripting** | `anyhow` | Minimal boilerplate, fast iteration |
| **Production libraries** | `thiserror` | Type safety, downstream compatibility |

## 📦 **anyhow: Application Error Handling**

### **Core Philosophy**

`anyhow` prioritizes **developer ergonomics** and **error context** over type safety, perfect for applications where you need to handle diverse error types efficiently.

### **Key Features**

```rust
use anyhow::{Result, Context, bail, ensure};

// Simple Result alias - no need to specify error type
fn parse_config() -> anyhow::Result<Config> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    
    let config: Config = toml::from_str(&content)
        .context("Failed to parse TOML")?;
    
    ensure!(config.port > 0, "Port must be positive");
    
    if config.host.is_empty() {
        bail!("Host cannot be empty");
    }
    
    Ok(config)
}
```

### **Error Context Chaining**

```rust
use anyhow::Context;

fn process_user_data(user_id: u64) -> anyhow::Result<UserProfile> {
    let user = database::fetch_user(user_id)
        .with_context(|| format!("Failed to fetch user {}", user_id))?;
    
    let preferences = load_preferences(&user.email)
        .context("Failed to load user preferences")?;
    
    let profile = UserProfile::new(user, preferences)
        .context("Failed to create user profile")?;
    
    Ok(profile)
}
```

### **Advanced anyhow Patterns**

```rust
use anyhow::{Result, Context, ensure, bail};

// Custom error creation
fn validate_input(input: &str) -> Result<()> {
    ensure!(!input.is_empty(), "Input cannot be empty");
    ensure!(input.len() <= 100, "Input too long: {} chars", input.len());
    
    if input.contains("forbidden") {
        bail!("Input contains forbidden content");
    }
    
    Ok(())
}

// Error downcasting for specific handling
fn handle_specific_errors() -> Result<()> {
    match risky_operation() {
        Ok(result) => Ok(result),
        Err(err) => {
            // Check for specific error types
            if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
                match io_err.kind() {
                    std::io::ErrorKind::NotFound => {
                        println!("File not found, creating default");
                        create_default_file()?
                    }
                    _ => return Err(err),
                }
            } else {
                return Err(err);
            }
            Ok(())
        }
    }
}
```

## 🏗️ **thiserror: Library Error Design**

### **Core Philosophy**

`thiserror` prioritizes **type safety** and **API stability** for libraries, enabling structured error types that downstream users can handle programmatically.

### **Derive Macro Magic**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file not found at {path}")]
    FileNotFound { path: String },
    
    #[error("Invalid configuration format")]
    InvalidFormat(#[from] toml::de::Error),
    
    #[error("Missing required field: {field}")]
    MissingField { field: String },
    
    #[error("Invalid port number: {port} (must be 1-65535)")]
    InvalidPort { port: u32 },
    
    #[error("Network error")]
    Network(#[from] std::io::Error),
    
    #[error("Validation failed: {0}")]
    ValidationError(String),
}
```

### **Error Source Chaining**

```rust
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed")]
    ConnectionFailed(#[from] sqlx::Error),
    
    #[error("Query timeout after {timeout}s")]
    QueryTimeout { timeout: u64 },
    
    #[error("Invalid query: {query}")]
    InvalidQuery { 
        query: String,
        #[source]
        source: QueryParseError,
    },
}

// Usage provides structured error handling
match database_operation() {
    Err(DatabaseError::ConnectionFailed(sqlx_err)) => {
        // Handle connection issues specifically
        retry_connection()?;
    }
    Err(DatabaseError::QueryTimeout { timeout }) => {
        // Handle timeouts with context
        log::warn!("Query timed out after {}s", timeout);
    }
    Err(e) => return Err(e.into()),
    Ok(result) => result,
}
```

### **Advanced thiserror Patterns**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected character '{char}' at position {pos}")]
    UnexpectedChar { char: char, pos: usize },
    
    #[error("Expected {expected}, found {found}")]
    ExpectedToken { expected: String, found: String },
    
    #[error("Integer overflow: {value} > {max}")]
    IntegerOverflow { value: String, max: u64 },
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    #[error("Multiple parsing errors")]
    Multiple(Vec<ParseError>),
}

// Custom Display implementation for complex formatting
#[derive(Error, Debug)]
pub struct ValidationErrors {
    pub errors: Vec<String>,
}

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Validation failed with {} errors:", self.errors.len())?;
        for (i, error) in self.errors.iter().enumerate() {
            write!(f, "\n  {}: {}", i + 1, error)?;
        }
        Ok(())
    }
}
```

## 🔄 **Combining anyhow and thiserror**

### **Library + Application Pattern**

```rust
// Library crate: uses thiserror for structured errors
pub use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyLibError {
    #[error("Configuration error: {message}")]
    Config { message: String },
    
    #[error("Network error")]
    Network(#[from] std::io::Error),
}

// Application crate: uses anyhow for ergonomics
use anyhow::{Result, Context};
use my_lib::MyLibError;

fn main() -> Result<()> {
    match my_lib::initialize() {
        Err(MyLibError::Config { message }) => {
            // Handle specific library error
            eprintln!("Config error: {}", message);
            std::process::exit(1);
        }
        Err(e) => {
            // Convert to anyhow for general handling
            return Err(e.into());
        }
        Ok(lib) => lib,
    };
    
    // Rest of application uses anyhow
    process_data()
        .context("Failed to process data")?;
    
    Ok(())
}
```

## 🎯 **Mission Integration Applications**

### **Mission5: HashMap Error Handling**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashMapError {
    #[error("Key not found: {key:?}")]
    KeyNotFound { key: String },
    
    #[error("Hash collision detected at bucket {bucket}")]
    HashCollision { bucket: usize },
    
    #[error("Capacity exceeded: {current}/{max}")]
    CapacityExceeded { current: usize, max: usize },
    
    #[error("Invalid hash function")]
    InvalidHashFunction,
}

// Application usage with anyhow
use anyhow::{Result, Context};

fn process_hashmap_data() -> Result<()> {
    let mut map = HashMap::new();
    
    map.insert("key1", "value1")
        .context("Failed to insert initial data")?;
    
    let value = map.get("key2")
        .with_context(|| "Failed to retrieve key2 from hashmap")?;
    
    Ok(())
}
```

### **AoC Problem Error Handling**

```rust
use anyhow::{Result, Context, bail, ensure};

fn solve_day10(input: &str) -> Result<(i32, i32)> {
    let lines: Vec<&str> = input.lines().collect();
    ensure!(!lines.is_empty(), "Input cannot be empty");
    
    let mut instructions = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let instruction = parse_instruction(line)
            .with_context(|| format!("Failed to parse line {}: '{}'", line_num + 1, line))?;
        instructions.push(instruction);
    }
    
    let part1 = simulate_program(&instructions)
        .context("Part 1 simulation failed")?;
    
    let part2 = find_cycle(&instructions)
        .context("Part 2 cycle detection failed")?;
    
    Ok((part1, part2))
}

fn parse_instruction(line: &str) -> Result<Instruction> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    match parts.as_slice() {
        ["noop"] => Ok(Instruction::Noop),
        ["addx", value] => {
            let num = value.parse::<i32>()
                .with_context(|| format!("Invalid number: '{}'", value))?;
            Ok(Instruction::AddX(num))
        }
        _ => bail!("Unknown instruction format: '{}'", line),
    }
}
```

## 📊 **Performance Considerations**

### **anyhow Performance**

- **Low overhead**: Uses `Box<dyn Error>` internally
- **Context chains**: Each `.context()` adds minimal cost
- **String formatting**: Only happens when error is displayed
- **Good for applications**: Performance impact acceptable

### **thiserror Performance**

- **Zero cost**: Compiles to same code as manual `impl Error`
- **Type safety**: No boxing unless converted to `Box<dyn Error>`
- **Enum dispatch**: Fast pattern matching on error variants
- **Optimal for libraries**: No runtime overhead

### **Benchmarking Example**

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_error_handling(c: &mut Criterion) {
    c.bench_function("anyhow_chain", |b| {
        b.iter(|| {
            let result: anyhow::Result<()> = operation_that_fails()
                .context("First context")
                .context("Second context")
                .context("Third context");
            black_box(result)
        })
    });
    
    c.bench_function("thiserror_match", |b| {
        b.iter(|| {
            let result = match library_operation() {
                Err(LibError::Network(_)) => handle_network_error(),
                Err(LibError::Config { .. }) => handle_config_error(),
                Ok(val) => Ok(val),
            };
            black_box(result)
        })
    });
}
```

## 🧪 **Testing Error Handling**

### **Testing anyhow Errors**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_error_context() -> Result<()> {
        let result = failing_operation();
        
        assert!(result.is_err());
        let error_msg = format!("{:#}", result.unwrap_err());
        assert!(error_msg.contains("Expected context message"));
        
        Ok(())
    }
    
    #[test]
    fn test_error_chain() {
        let error = complex_operation().unwrap_err();
        
        // Check error chain
        let mut source = error.source();
        let mut depth = 0;
        while let Some(err) = source {
            depth += 1;
            source = err.source();
        }
        
        assert_eq!(depth, 3); // Expected chain depth
    }
}
```

### **Testing thiserror Errors**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structured_errors() {
        let error = ConfigError::InvalidPort { port: 999999 };
        
        // Test error message formatting
        let msg = format!("{}", error);
        assert!(msg.contains("999999"));
        assert!(msg.contains("1-65535"));
        
        // Test error matching
        match error {
            ConfigError::InvalidPort { port } if port > 65535 => {
                // Expected path
            }
            _ => panic!("Wrong error variant"),
        }
    }
    
    #[test]
    fn test_error_conversion() {
        let io_error = std::io::Error::new(
            std::io::ErrorKind::NotFound, 
            "file not found"
        );
        let app_error: ConfigError = io_error.into();
        
        match app_error {
            ConfigError::Network(_) => {
                // Expected conversion
            }
            _ => panic!("Wrong error conversion"),
        }
    }
}
```

## 🔗 **Integration with Learning Tracks**

### **Daily Study Applications**

- **Week 5**: Error handling patterns and recovery strategies
- **Error banks**: Collect and categorize common error patterns
- **AoC solutions**: Robust input validation and error reporting

### **Mission Applications**

- **Mission implementations**: Use `thiserror` for library-like APIs
- **Application demos**: Use `anyhow` for ergonomic error handling
- **Testing strategies**: Comprehensive error condition coverage

### **Rust Book Connections**

- **Chapter 9**: Error handling fundamentals (`Result`, `panic!`)
- **Chapter 10**: Traits and generics (understanding `Error` trait)
- **Chapter 17**: Advanced error handling patterns

## 📚 **Best Practices Summary**

### **anyhow Best Practices**

- ✅ Use for **applications** and **scripts**
- ✅ Add **context** at each error boundary
- ✅ Use `ensure!` and `bail!` for validations
- ✅ Leverage **error downcasting** when needed
- ❌ Don't use in **public library APIs**
- ❌ Don't over-context (each level should add value)

### **thiserror Best Practices**

- ✅ Use for **library APIs** and **structured errors**
- ✅ Design **meaningful error variants**
- ✅ Use `#[from]` for **automatic conversions**
- ✅ Provide **detailed error messages**
- ✅ Use `#[source]` for **error chaining**
- ❌ Don't create **too many variants** (prefer composition)
- ❌ Don't expose **internal implementation details**

## 🎓 **Learning Path**

### **Beginner (Week 1-2)**

1. Understand `Result<T, E>` and `?` operator
2. Learn basic `anyhow::Result` usage
3. Practice with `.context()` method
4. Simple error handling in AoC solutions

### **Intermediate (Week 3-4)**

1. Design structured errors with `thiserror`
2. Understand error trait bounds and conversions
3. Implement error handling in mission projects
4. Error testing strategies

### **Advanced (Week 5+)**

1. Custom error trait implementations
2. Error handling in async contexts
3. Performance optimization for error paths
4. Library API design with robust error handling

---

*Tags: #error-handling #anyhow #thiserror #result-type #library-design #application-development #rust-patterns #mission-integration #aoc-patterns #testing-strategies #performance*

*Links: [[zettel-index]] | [[Result Type]] | [[Error Handling Patterns]] | [[Error Handling Deep Dive]] | [[mission-5]] | [[Week 5 Overview]] | [[Testing Strategies]] | [[API Design Patterns]]*
