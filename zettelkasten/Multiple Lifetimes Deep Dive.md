# Multiple Lifetimes Deep Dive

**Tags:** #rust #lifetimes #memory-safety #advanced-rust #borrowing

**Related:** [[Day17 - Lifetimes Deep Dive|Day 17]] | [[Rust Concepts MOC]] | [[Error Handling Deep Dive]]

## Overview

Multiple lifetimes in Rust allow you to handle references with different lifetimes independently, providing precise control over memory safety and reference relationships. This is essential for complex data structures and functions that work with multiple references.

## When You Need Multiple Lifetimes

### 1. Different Input Lifetimes, Different Return Lifetime

```rust
fn complex<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("y is: {}", y); // y has lifetime 'b
    x  // Return has lifetime 'a (same as x)
}
```

**Why multiple lifetimes?**
- `x` has lifetime `'a` 
- `y` has lifetime `'b`
- Return value has lifetime `'a` (same as `x`)
- The function can return a reference that lives **as long as `x`**, regardless of how long `y` lives

### 2. Struct with Multiple References

```rust
struct BookReview<'a, 'b> {
    title: &'a str,    // Title lives for 'a
    content: &'b str,  // Content lives for 'b
}

impl<'a, 'b> BookReview<'a, 'b> {
    fn get_title(&self) -> &'a str {
        self.title  // Returns reference with lifetime 'a
    }
    
    fn get_content(&self) -> &'b str {
        self.content  // Returns reference with lifetime 'b
    }
}
```

### 3. Iterator with Different Lifetimes

```rust
fn find_first<'a, 'b>(haystack: &'a str, needles: &'b [&str]) -> Option<&'a str> {
    for needle in needles {
        if haystack.contains(needle) {
            return Some(haystack); // Return haystack with its lifetime 'a
        }
    }
    None
}
```

## Key Scenarios for Multiple Lifetimes

### Scenario 1: Different Input Lifetimes
```rust
fn combine<'a, 'b>(short: &'a str, long: &'b str) -> &'b str {
    // Return the longer-lived reference
    long
}
```

### Scenario 2: One Input, Multiple Outputs
```rust
fn split<'a>(text: &'a str) -> (&'a str, &'a str) {
    let mid = text.len() / 2;
    (&text[..mid], &text[mid..])
}
```

### Scenario 3: Conditional Returns
```rust
fn get_best<'a, 'b>(option1: &'a str, option2: &'b str) -> &'a str {
    if option1.len() > option2.len() {
        option1  // Return with lifetime 'a
    } else {
        option1  // Still return with lifetime 'a
    }
}
```

## Real-World Examples

### Web Framework
```rust
fn handle_request<'req, 'resp>(
    request: &'req Request,
    response: &'resp mut Response
) -> Result<&'resp str, Error> {
    // Process request, write to response
    response.write("OK")
}
```

### File Processing
```rust
fn process_file<'file, 'config>(
    file_content: &'file str,
    config: &'config Config
) -> ProcessedFile<'file> {
    // Process file using config
    ProcessedFile { content: file_content }
}
```

### Database-like Operations
```rust
struct Database<'db> {
    connection: &'db mut Connection,
}

struct Query<'q> {
    sql: &'q str,
}

fn execute_query<'db, 'q>(
    db: &'db mut Database, 
    query: &'q Query
) -> Result<&'db str, Error> {
    // Execute query and return result with database lifetime
    db.connection.execute(query.sql)
}
```

## Common Patterns

### Pattern 1: Lifetime Elision with Multiple Parameters
```rust
// The compiler can often infer lifetimes
fn process<'a, 'b>(data: &'a str, config: &'b Config) -> &'a str {
    // Return data with its lifetime
    data
}
```

### Pattern 2: Lifetime Bounds
```rust
fn process_with_bound<'a, 'b>(x: &'a str, y: &'b str) -> &'a str 
where 
    'b: 'a  // 'b must live at least as long as 'a
{
    x
}
```

### Pattern 3: Static Lifetime
```rust
fn get_static_string<'a>(input: &'a str) -> &'static str {
    "This is a static string"  // Lives for the entire program
}
```

## Error Patterns and Solutions

### Error: Conflicting Lifetime Requirements
```rust
// ❌ This won't compile
fn problematic<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x  // Returns 'a
    } else {
        y  // Returns 'b - CONFLICT!
    }
}
```

**Solution:** Use lifetime bounds or return owned data
```rust
// ✅ Solution 1: Lifetime bounds
fn fixed1<'a, 'b>(x: &'a str, y: &'b str) -> &'a str 
where 
    'b: 'a  // 'b must live at least as long as 'a
{
    if x.len() > y.len() {
        x
    } else {
        y  // Now this is safe because 'b: 'a
    }
}

// ✅ Solution 2: Return owned data
fn fixed2<'a, 'b>(x: &'a str, y: &'b str) -> String {
    if x.len() > y.len() {
        x.to_string()
    } else {
        y.to_string()
    }
}
```

## Testing Multiple Lifetimes

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_lifetimes() {
        let long_lived = String::from("long");
        let result;
        {
            let short_lived = String::from("short");
            result = complex(&long_lived, &short_lived);
        } // short_lived goes out of scope here
        assert_eq!(result, "long"); // result still valid
    }

    #[test]
    fn test_struct_multiple_lifetimes() {
        let title = String::from("Rust Book");
        let content = String::from("Great content");
        
        let review = BookReview {
            title: &title,
            content: &content,
        };
        
        assert_eq!(review.get_title(), "Rust Book");
        assert_eq!(review.get_content(), "Great content");
    }
}
```

## Performance Considerations

### Zero-Cost Abstractions
- Multiple lifetimes are **compile-time only**
- No runtime overhead
- Compiler optimizes away lifetime annotations

### Memory Safety Benefits
- Prevents dangling references
- Enables safe concurrent access
- Allows complex reference relationships

## Integration with Mission Work

### Mission 5: HashMap Patterns
```rust
struct Cache<'key, 'value> {
    keys: &'key [String],
    values: &'value [String],
}

impl<'key, 'value> Cache<'key, 'value> {
    fn get<'a>(&'a self, key: &str) -> Option<&'a str> {
        // Return reference with self's lifetime
        self.values.get(0).map(|s| s.as_str())
    }
}
```

### Mission 6: Grid Operations
```rust
struct GridProcessor<'grid, 'data> {
    grid: &'grid Grid,
    data: &'data [u8],
}

impl<'grid, 'data> GridProcessor<'grid, 'data> {
    fn process_cell<'a>(&'a self, coord: Coord) -> Option<&'a str> {
        // Process grid cell using data
        Some("processed")
    }
}
```

## Best Practices

### 1. Start Simple
- Begin with single lifetime parameters
- Add multiple lifetimes only when needed
- Let the compiler guide you

### 2. Use Descriptive Names
```rust
// ✅ Good
fn process<'input, 'config>(input: &'input str, config: &'config Config) -> &'input str

// ❌ Less clear
fn process<'a, 'b>(x: &'a str, y: &'b Config) -> &'a str
```

### 3. Lifetime Bounds When Needed
```rust
// Use lifetime bounds to express relationships
fn combine<'a, 'b>(x: &'a str, y: &'b str) -> &'a str 
where 
    'b: 'a  // 'b must live at least as long as 'a
{
    x
}
```

### 4. Consider Owned Data
```rust
// Sometimes returning owned data is simpler
fn process_simple<'a, 'b>(x: &'a str, y: &'b str) -> String {
    format!("{}{}", x, y)
}
```

## Common Pitfalls

### Pitfall 1: Over-complicating
```rust
// ❌ Unnecessarily complex
fn overcomplicated<'a, 'b, 'c>(x: &'a str, y: &'b str, z: &'c str) -> &'a str {
    x  // Only using x, don't need 'b and 'c
}

// ✅ Simpler
fn simple<'a>(x: &'a str, y: &str, z: &str) -> &'a str {
    x
}
```

### Pitfall 2: Ignoring Lifetime Bounds
```rust
// ❌ May not compile in all cases
fn problematic<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if condition() { x } else { y }  // Lifetime conflict
}

// ✅ Use bounds or owned data
fn fixed<'a, 'b>(x: &'a str, y: &'b str) -> String 
where 
    'b: 'a 
{
    if condition() { x.to_string() } else { y.to_string() }
}
```

## Summary

Multiple lifetimes are essential for:

1. **Different Input Lifetimes** - When inputs have different lifetimes
2. **Structs with Multiple References** - Each field can have its own lifetime
3. **Precise Control** - Specify exactly which references can live how long
4. **Complex Data Structures** - Enable safe reference relationships
5. **API Design** - Create flexible, safe interfaces

**Key Insight:** Multiple lifetimes give you precise control over which references can live how long, rather than forcing everything to have the same lifetime.

**Next Steps:** Practice with real-world examples, understand lifetime bounds, and integrate with your mission work.

---

**References:**
- [[Day17 - Lifetimes Deep Dive|Day 17 Learning Notes]]
- [The Rust Book - Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [[Rust Concepts MOC]] for related concepts
