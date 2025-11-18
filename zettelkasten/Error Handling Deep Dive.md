# 🚨 Error Handling Deep Dive

**Comprehensive guide to Rust's error handling philosophy and practical patterns**

## 🎯 Core Philosophy

Rust's error handling is built on the principle that **errors are values, not exceptions**. This means:
- Errors are explicit and must be handled
- No hidden control flow or stack unwinding
- Compiler enforces error handling
- Zero-cost abstractions for error cases

## 📚 The Two Main Error Types

### **Option<T>** - "Maybe" Values
```rust
enum Option<T> {
    Some(T),    // Value exists
    None,       // No value
}
```

**When to use**: When a value might not exist
- Array access that might be out of bounds
- Hash map lookup that might fail
- Parsing that might not succeed

### **Result<T, E>** - "Success or Error" Values
```rust
enum Result<T, E> {
    Ok(T),      // Success with value
    Err(E),     // Error with details
}
```

**When to use**: When an operation might fail
- File I/O operations
- Network requests
- Parsing user input
- Database queries

## 🛠️ Practical Error Handling Patterns

### **Pattern 1: Match Expressions**
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Using match
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(error) => println!("Error: {}", error),
}
```

### **Pattern 2: The ? Operator**
```rust
fn process_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;  // Returns early on error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;   // Returns early on error
    Ok(contents)
}
```

**Key benefits**:
- Cleaner code than match expressions
- Automatic error propagation
- Compiler enforces error handling

### **Pattern 3: unwrap() and expect()**
```rust
// unwrap() - panic on error
let value = some_result.unwrap();

// expect() - panic with custom message
let value = some_result.expect("Failed to parse number");
```

**When to use**:
- **unwrap()**: Prototyping, tests, when you're 100% sure it won't fail
- **expect()**: When you want a better error message for debugging
- **Never in production code** unless you're certain

### **Pattern 4: unwrap_or() and unwrap_or_else()**
```rust
// Provide default value
let value = some_option.unwrap_or(0);

// Provide computed default
let value = some_option.unwrap_or_else(|| {
    println!("Computing default value");
    42
});
```

## 🎨 Advanced Error Handling

### **Custom Error Types**
```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
            MathError::NegativeSquareRoot => write!(f, "Cannot take square root of negative number"),
            MathError::Overflow => write!(f, "Arithmetic overflow"),
        }
    }
}

impl std::error::Error for MathError {}
```

### **Error Propagation with ?**
```rust
fn complex_calculation(x: i32, y: i32) -> Result<f64, MathError> {
    let quotient = divide(x, y)?;           // Propagates MathError
    let root = square_root(quotient)?;      // Propagates MathError
    Ok(root)
}

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}
```

### **Error Chaining and Context**
```rust
use std::error::Error;

fn process_data() -> Result<(), Box<dyn Error>> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line?;
        let number: i32 = line.parse()?;
        println!("Processed: {}", number);
    }
    
    Ok(())
}
```

## 🚀 **Advanced Error Handling (Week 5)**

### **Production-Ready Error Handling**
For comprehensive error handling patterns and real-world examples, see [[Week 5 Overview]]:

- **[[daily-study/Day29]]** - Building robust error types with `Display` and `Error` traits
- **[[daily-study/Day30]]** - Mastering the `?` operator and `From` trait conversions  
- **[[daily-study/Day31]]** - Real-world error handling with industry-standard crates
- **[[daily-study/Day32]]** - Chaining operations with `map`, `and_then`, `or_else`
- **[[daily-study/Day33]]** - `catch_unwind` and graceful error recovery
- **[[daily-study/Day34]]** - When to panic vs return errors, best practices
- **[[daily-study/Day35]]** - Building fault-tolerant parsers with error recovery

### **Real-World Examples**
- **[[../daily_study/rust_learning_week5_notes/examples/web_api_errors|Web API Error Handling]]** - HTTP status codes, validation errors
- **[[../daily_study/rust_learning_week5_notes/examples/file_processor|File Processing Pipeline]]** - Multi-format parsing with recovery

## 🎯 AoC-Specific Error Patterns

### **Input Parsing Errors**
```rust
fn parse_coordinate(input: &str) -> Result<(i32, i32), String> {
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() != 2 {
        return Err(format!("Expected 'x,y', got '{}'", input));
    }
    
    let x = parts[0].parse::<i32>()
        .map_err(|_| format!("Invalid x coordinate: {}", parts[0]))?;
    let y = parts[1].parse::<i32>()
        .map_err(|_| format!("Invalid y coordinate: {}", parts[1]))?;
    
    Ok((x, y))
}
```

### **Grid Boundary Errors**
```rust
fn get_grid_value(grid: &Vec<Vec<i32>>, x: usize, y: usize) -> Option<i32> {
    grid.get(y)?.get(x).copied()
}

// Usage in AoC problems
if let Some(value) = get_grid_value(&grid, x, y) {
    // Process valid grid position
    process_cell(value);
} else {
    // Handle out-of-bounds
    println!("Position ({}, {}) is out of bounds", x, y);
}
```

### **Algorithm State Errors**
```rust
fn bfs_search(graph: &HashMap<String, Vec<String>>, start: &str, target: &str) 
    -> Result<Vec<String>, String> {
    
    if !graph.contains_key(start) {
        return Err(format!("Start node '{}' not found in graph", start));
    }
    
    if !graph.contains_key(target) {
        return Err(format!("Target node '{}' not found in graph", target));
    }
    
    // BFS implementation...
    Ok(path)
}
```

## 🧪 Testing Error Handling

### **Testing Error Cases**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division_by_zero() {
        let result = divide(10, 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero");
    }

    #[test]
    fn test_successful_division() {
        let result = divide(10, 2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
    }

    #[test]
    fn test_parse_coordinate_valid() {
        let result = parse_coordinate("3,4");
        assert_eq!(result.unwrap(), (3, 4));
    }

    #[test]
    fn test_parse_coordinate_invalid() {
        let result = parse_coordinate("3,4,5");
        assert!(result.is_err());
    }
}
```

### **Property-Based Testing**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_division_properties(a: i32, b: i32) {
        let result = divide(a, b);
        
        if b == 0 {
            prop_assert!(result.is_err());
        } else {
            prop_assert!(result.is_ok());
            prop_assert_eq!(result.unwrap(), a / b);
        }
    }
}
```

## 🚀 Performance Considerations

### **Zero-Cost Error Handling**
- `Result<T, E>` has no runtime overhead when there's no error
- Error cases are handled at compile time
- No exception handling machinery

### **Error vs Panic Performance**
```rust
// Fast path - no error checking overhead
fn fast_calculation(x: i32) -> i32 {
    x * 2  // No error handling needed
}

// Error path - explicit but fast
fn safe_calculation(x: i32) -> Result<i32, String> {
    if x < 0 {
        Err("Negative input".to_string())
    } else {
        Ok(x * 2)
    }
}

// Panic path - avoid in production
fn panic_calculation(x: i32) -> i32 {
    assert!(x >= 0, "Negative input not allowed");
    x * 2
}
```

## 🎓 Best Practices

### **DO:**
- Use `Result<T, E>` for operations that can fail
- Use `Option<T>` for values that might not exist
- Use `?` operator for error propagation
- Create custom error types for your domain
- Handle errors at appropriate boundaries
- Use `unwrap_or()` for safe defaults

### **DON'T:**
- Use `unwrap()` in production code
- Ignore errors with `let _ = result;`
- Use `panic!()` for recoverable errors
- Mix error types unnecessarily
- Forget to handle `Result` values

### **Error Handling Checklist:**
- [ ] All fallible operations return `Result<T, E>`
- [ ] Error types are descriptive and helpful
- [ ] Error messages include context
- [ ] Tests cover both success and error cases
- [ ] Error boundaries are clearly defined
- [ ] No `unwrap()` calls in production code

## 🔗 Integration with Learning Tracks

### **Mission Integration**
- **Mission 1-4**: Stack/Queue operations with bounds checking
- **Mission 5**: HashMap operations with key validation
- **Mission 6**: Grid algorithms with boundary checking

### **Daily Study Integration**
- **Day 5**: Option and Result fundamentals
- **Day 6**: Pattern matching for error handling
- **Day 15+**: Advanced error handling patterns

### **AoC Applications**
- Input parsing with validation
- Algorithm state management
- Boundary condition handling
- Resource cleanup on errors

## 📚 Further Reading

- [[../rust_book/Ch9/README]] - Complete Chapter 9 implementation with runnable examples
- [[Standard Error and Stream Separation]] - CLI error output and stream separation patterns
- [[daily-study/Day05]] - Basic error handling concepts
- [[zettelkasten/daily-study/Day06]] - Using match for error handling
- [[Mission5 API Reference]] - Error handling in HashMap implementation
- [[Performance Optimization Guide]] - Zero-cost error handling
- **Week 5 Examples**: [[../../daily_study/rust_learning_week5_notes/examples/README]] - Comprehensive examples documentation and usage instructions

---

*This comprehensive guide covers Rust's error handling from basic concepts to advanced patterns, with practical examples for competitive programming and mission work.*

*Tags: #error-handling #result #option #pattern-matching #aoc #mission5 #daily-study #cross-track*
*Links: [[zettel-index]] | [[rust-book-ch9-12-review]] | [[rust-concepts-MOC]] | [[Collections MOC]] | [[daily-study/Day05]]*
