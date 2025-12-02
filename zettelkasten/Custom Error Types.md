# 🚨 Custom Error Types in Rust

**Comprehensive guide to implementing custom error types with `std::error::Error` trait for domain-specific error handling**

---

## 🎯 **Error Type Philosophy**

### **Why Custom Error Types?**

- **Domain specificity** - Errors that are meaningful to your application domain
- **Error composition** - Building hierarchical error types for complex systems
- **User experience** - Providing clear, actionable error messages
- **Debugging support** - Rich context for error diagnosis
- **API clarity** - Self-documenting error conditions in function signatures

### **The `std::error::Error` Trait**

```rust
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> { None }
    fn backtrace(&self) -> Option<&Backtrace> { None }
    
    // Deprecated methods (still available for backwards compatibility)
    fn description(&self) -> &str { "description() is deprecated; use Display" }
    fn cause(&self) -> Option<&dyn Error> { self.source() }
}
```

---

## 🏗️ **Basic Custom Error Implementation**

### **Simple Custom Error**

```rust
use std::fmt;

#[derive(Debug)]
pub struct ValidationError {
    message: String,
}

impl ValidationError {
    pub fn new(msg: &str) -> ValidationError {
        ValidationError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Validation error: {}", self.message)
    }
}

impl std::error::Error for ValidationError {}

// Usage
fn validate_age(age: i32) -> Result<(), ValidationError> {
    if age < 0 {
        Err(ValidationError::new("Age cannot be negative"))
    } else if age > 150 {
        Err(ValidationError::new("Age seems unrealistic"))
    } else {
        Ok(())
    }
}
```

### **Enum-Based Error Types**

```rust
#[derive(Debug)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
    InvalidInput { input: String, expected: String },
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeSquareRoot => write!(f, "Cannot take square root of negative number"),
            MathError::Overflow => write!(f, "Mathematical operation resulted in overflow"),
            MathError::InvalidInput { input, expected } => {
                write!(f, "Invalid input '{}', expected {}", input, expected)
            }
        }
    }
}

impl std::error::Error for MathError {}
```

---

## 🔗 **Error Chaining and Source**

### **Error with Source**

```rust
use std::num::ParseIntError;

#[derive(Debug)]
pub struct ConfigError {
    message: String,
    source: Option<Box<dyn std::error::Error>>,
}

impl ConfigError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
            source: None,
        }
    }
    
    pub fn with_source(msg: &str, source: Box<dyn std::error::Error>) -> Self {
        Self {
            message: msg.to_string(),
            source: Some(source),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Configuration error: {}", self.message)
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

// Automatic conversion from ParseIntError
impl From<ParseIntError> for ConfigError {
    fn from(err: ParseIntError) -> Self {
        ConfigError::with_source("Failed to parse integer", Box::new(err))
    }
}

// Usage with error chaining
fn load_port_from_config(config: &str) -> Result<u16, ConfigError> {
    let port_str = config.lines()
        .find(|line| line.starts_with("port="))
        .ok_or_else(|| ConfigError::new("Missing port configuration"))?
        .strip_prefix("port=")
        .ok_or_else(|| ConfigError::new("Invalid port format"))?;
    
    let port: u16 = port_str.parse()?; // Automatic conversion from ParseIntError
    
    if port < 1024 {
        return Err(ConfigError::new("Port number too low (reserved range)"));
    }
    
    Ok(port)
}
```

### **Error Context Preservation**

```rust
#[derive(Debug)]
pub struct DatabaseError {
    operation: String,
    table: String,
    source: Option<Box<dyn std::error::Error>>,
}

impl DatabaseError {
    pub fn new(operation: &str, table: &str) -> Self {
        Self {
            operation: operation.to_string(),
            table: table.to_string(),
            source: None,
        }
    }
    
    pub fn with_context(operation: &str, table: &str, source: Box<dyn std::error::Error>) -> Self {
        Self {
            operation: operation.to_string(),
            table: table.to_string(),
            source: Some(source),
        }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Database error during {} on table '{}'", self.operation, self.table)
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}
```

---

## 📦 **Using `thiserror` Crate**

### **Simplified Error Definition**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PathfindingError {
    #[error("No path exists from {start} to {goal}")]
    NoPathExists { start: u32, goal: u32 },
    
    #[error("Invalid node: {node}")]
    InvalidNode { node: u32 },
    
    #[error("Graph is empty")]
    EmptyGraph,
    
    #[error("Heuristic calculation failed")]
    HeuristicError(#[from] HeuristicError),
    
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    ParseError(String),
}

#[derive(Error, Debug)]
pub enum HeuristicError {
    #[error("Distance calculation overflow")]
    Overflow,
    
    #[error("Invalid coordinates: ({x}, {y})")]
    InvalidCoordinates { x: i32, y: i32 },
}

// Usage is much simpler
fn find_path(start: u32, goal: u32) -> Result<Vec<u32>, PathfindingError> {
    if start == goal {
        return Ok(vec![start]);
    }
    
    // This automatically converts HeuristicError to PathfindingError
    let distance = calculate_heuristic(start, goal)?;
    
    if distance.is_infinite() {
        return Err(PathfindingError::NoPathExists { start, goal });
    }
    
    // Implementation continues...
    Ok(vec![start, goal])
}
```

### **Complex Error Hierarchies**

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Database error")]
    Database(#[from] DatabaseError),
    
    #[error("Network error")]
    Network(#[from] NetworkError),
    
    #[error("Configuration error")]
    Config(#[from] ConfigError),
    
    #[error("Business logic error")]
    BusinessLogic(#[from] BusinessError),
}

#[derive(Error, Debug)]
pub enum BusinessError {
    #[error("Insufficient funds: need {needed}, have {available}")]
    InsufficientFunds { needed: u64, available: u64 },
    
    #[error("Invalid transaction type: {transaction_type}")]
    InvalidTransactionType { transaction_type: String },
    
    #[error("Account locked: {account_id}")]
    AccountLocked { account_id: String },
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Connection timeout")]
    Timeout,
    
    #[error("DNS resolution failed for {hostname}")]
    DnsFailure { hostname: String },
    
    #[error("HTTP error: {status}")]
    HttpError { status: u16 },
    
    #[error("TLS error")]
    TlsError(#[source] Box<dyn std::error::Error>),
}
```

---

## 🔧 **Error Handling Patterns**

### **Result Type Patterns**

```rust
// Type alias for domain-specific Results
pub type PathfindingResult<T> = Result<T, PathfindingError>;
pub type DatabaseResult<T> = Result<T, DatabaseError>;

// Function signatures become cleaner
pub fn dijkstra_shortest_path(
    graph: &Graph, 
    start: NodeId, 
    goal: NodeId
) -> PathfindingResult<(Vec<NodeId>, f64)> {
    // Implementation
    todo!()
}

// Error context enhancement
impl PathfindingError {
    pub fn add_context(self, context: &str) -> Self {
        match self {
            PathfindingError::NoPathExists { start, goal } => {
                PathfindingError::ParseError(
                    format!("{}: No path from {} to {}", context, start, goal)
                )
            }
            other => other,
        }
    }
}

// Usage with context
fn find_route() -> PathfindingResult<Vec<NodeId>> {
    dijkstra_shortest_path(&graph, 0, 10)
        .map(|(path, _cost)| path)
        .map_err(|e| e.add_context("Failed to find route in city map"))
}
```

### **Error Recovery Strategies**

```rust
pub enum ErrorSeverity {
    Warning,    // Log but continue
    Recoverable, // Try alternative approach
    Fatal,      // Stop execution
}

impl PathfindingError {
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            PathfindingError::NoPathExists { .. } => ErrorSeverity::Recoverable,
            PathfindingError::InvalidNode { .. } => ErrorSeverity::Fatal,
            PathfindingError::EmptyGraph => ErrorSeverity::Fatal,
            PathfindingError::HeuristicError(_) => ErrorSeverity::Warning,
            PathfindingError::IoError(_) => ErrorSeverity::Fatal,
            PathfindingError::ParseError(_) => ErrorSeverity::Recoverable,
        }
    }
    
    pub fn is_recoverable(&self) -> bool {
        matches!(self.severity(), ErrorSeverity::Warning | ErrorSeverity::Recoverable)
    }
}

// Usage with error recovery
fn find_path_with_fallback(start: NodeId, goal: NodeId) -> PathfindingResult<Vec<NodeId>> {
    match dijkstra_shortest_path(&graph, start, goal) {
        Ok(result) => Ok(result.0),
        Err(e) if e.is_recoverable() => {
            // Try A* as fallback
            astar_path(&graph, start, goal, manhattan_heuristic)
                .map(|(path, _)| path)
        }
        Err(e) => Err(e),
    }
}
```

---

## 🧪 **Testing Error Types**

### **Error Testing Patterns**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_display() {
        let error = PathfindingError::NoPathExists { start: 1, goal: 5 };
        assert_eq!(error.to_string(), "No path exists from 1 to 5");
    }
    
    #[test]
    fn test_error_source_chain() {
        let heuristic_error = HeuristicError::Overflow;
        let pathfinding_error = PathfindingError::HeuristicError(heuristic_error);
        
        assert!(pathfinding_error.source().is_some());
        assert!(pathfinding_error.source().unwrap().is::<HeuristicError>());
    }
    
    #[test]
    fn test_error_conversion() {
        let parse_error = "invalid".parse::<i32>().unwrap_err();
        let config_error: ConfigError = parse_error.into();
        
        assert!(config_error.source().is_some());
    }
    
    #[test]
    fn test_error_severity() {
        let warning_error = PathfindingError::HeuristicError(HeuristicError::Overflow);
        let fatal_error = PathfindingError::EmptyGraph;
        
        assert!(warning_error.is_recoverable());
        assert!(!fatal_error.is_recoverable());
    }
    
    #[test] 
    fn test_error_context() {
        let original = PathfindingError::NoPathExists { start: 1, goal: 5 };
        let with_context = original.add_context("Route planning");
        
        assert!(with_context.to_string().contains("Route planning"));
    }
}
```

### **Integration Testing with Errors**

```rust
#[test]
fn test_pathfinding_error_integration() {
    let empty_graph = Graph::new();
    let result = dijkstra_shortest_path(&empty_graph, 0, 5);
    
    match result {
        Err(PathfindingError::EmptyGraph) => {
            // Expected error
            assert_eq!(result.unwrap_err().severity(), ErrorSeverity::Fatal);
        }
        _ => panic!("Expected EmptyGraph error"),
    }
}

#[test]
fn test_error_recovery_workflow() {
    // Test that error recovery mechanisms work correctly
    let result = find_path_with_fallback(0, 999); // Non-existent node
    
    // Should try multiple algorithms before giving up
    match result {
        Err(PathfindingError::InvalidNode { node: 999 }) => {
            // Expected final error after all recovery attempts
        }
        _ => panic!("Expected InvalidNode error after fallback attempts"),
    }
}
```

---

## 🏆 **Mission-Specific Error Applications**

### **Pathfinding Domain Errors (Mission 9)**

```rust
#[derive(Error, Debug)]
pub enum PathfindingError {
    #[error("No path exists between nodes {start} and {goal}")]
    NoPathExists { start: NodeId, goal: NodeId },
    
    #[error("Invalid start node: {node} (graph has {max_nodes} nodes)")]
    InvalidStartNode { node: NodeId, max_nodes: usize },
    
    #[error("Invalid goal node: {node} (graph has {max_nodes} nodes)")]
    InvalidGoalNode { node: NodeId, max_nodes: usize },
    
    #[error("Graph contains negative edge weights (not supported by Dijkstra)")]
    NegativeWeights,
    
    #[error("Heuristic function returned invalid value: {value}")]
    InvalidHeuristic { value: f64 },
    
    #[error("Priority queue error: {message}")]
    QueueError { message: String },
    
    #[error("Graph is disconnected: no edges found")]
    DisconnectedGraph,
    
    #[error("Algorithm timeout after {seconds} seconds")]
    Timeout { seconds: u64 },
}
```

### **Graph Structure Errors (Mission 7)**

```rust
#[derive(Error, Debug)]
pub enum GraphError {
    #[error("Node {node} already exists")]
    NodeAlreadyExists { node: NodeId },
    
    #[error("Node {node} not found")]
    NodeNotFound { node: NodeId },
    
    #[error("Edge from {from} to {to} already exists")]
    EdgeAlreadyExists { from: NodeId, to: NodeId },
    
    #[error("Edge from {from} to {to} not found")]
    EdgeNotFound { from: NodeId, to: NodeId },
    
    #[error("Invalid edge weight: {weight} (must be positive)")]
    InvalidWeight { weight: f64 },
    
    #[error("Graph capacity exceeded: cannot add more than {max} nodes")]
    CapacityExceeded { max: usize },
}
```

---

## 🔗 **Integration with Learning System**

### **Mission Integration**

- **[[mission-9]]** - Pathfinding error types for Dijkstra and A*
- **[[mission-7]]** - Graph structure error handling patterns
- **[[Mission11 Overview]]** - Dynamic programming error types and memoization failures
- **[[Mission12 Overview]]** - Parser error types for input processing

### **Pattern Integration**

- **[[API Design Patterns]]** - Error types as part of robust API design
- **[[Testing Patterns]]** - Comprehensive error testing strategies
- **[[CLI Design Patterns]]** - Command line error handling and user feedback

### **Rust Book Integration**

- **[[rust-book]]** - Chapter 9 error handling fundamentals
- **Chapter 12** - CLI error handling in I/O project context

### **Daily Study Integration**

- **[[Daily Study MOC]]** - Week 5 error handling track
- **Error propagation patterns** and `?` operator mastery
- **Result combinators** and functional error handling

---

## 📚 **External Resources**

### **Official Documentation**

- **[std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html)** - Error trait reference
- **[Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)** - Rust Book error handling
- **[thiserror](https://docs.rs/thiserror/)** - Derive Error trait automatically

### **Best Practices**

- **[Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/)** - Comprehensive error handling guide
- **[Rust Error Handling Patterns](https://nick.groenen.me/posts/rust-error-handling/)** - Practical patterns and examples
- **[anyhow vs thiserror](https://github.com/dtolnay/anyhow/blob/master/README.md#comparison-with-thiserror)** - When to use which crate

---

*Tags: #custom-errors #error-handling #std-error #thiserror #rust #mission9 #pathfinding #api-design #testing*
*Links: [[zettel-index]] | [[API Design Patterns]] | [[Testing Patterns]] | [[mission-9]] | [[CLI Design Patterns]] | [[rust-book]] | [[Daily Study MOC]]*
