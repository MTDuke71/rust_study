# Command Line Argument Methods

## 🔗 Zettelkasten Links

- **Source**: [[zettelkasten/rust_book/rust-book-ch12]] - Detailed implementation in Ch12/accepting_arguments
- **Related Concepts**: [[Error Handling Patterns]], [[Rust CLI Applications]], [[Input Validation]]
- **Examples**: [[CLI Best Practices]], [[Defensive Programming]]
- **Book Connection**: [[Rust Book MOC]]
- **Missions**: [[mission-8]] - Advanced argument parsing | [[Mission9 TUT]] - CLI pathfinding args

## 📋 Overview

Four progressive methods for handling command-line arguments in Rust applications, from basic collection to robust validation with proper error handling.

## 🎯 The Four Methods

### Method 1: Basic Collection

**Pattern**: Simple argument vector collection

```rust
let args: Vec<String> = env::args().collect();
println!("Arguments: {:?}", &args[1..]);
```

- ✅ Simple and clear
- ❌ No validation or error handling
- **Use when**: Learning or debugging

### Method 2: Direct Parsing  

**Pattern**: Assume argument positions

```rust
if args.len() < 3 {
    println!("Usage: {} <query> <filename>", args[0]);
    return;
}
let query = &args[1];
let filename = &args[2];
```

- ✅ Direct and minimal
- ❌ Can panic on insufficient args
- **Use when**: Simple scripts with known usage

### Method 3: Validation with Result

**Pattern**: Comprehensive validation returning Result

```rust
fn validate_args(args: &[String]) -> Result<(&str, &str), &'static str> {
    if args.len() < 3 { return Err("Not enough arguments"); }
    if args.len() > 3 { return Err("Too many arguments"); }
    // Additional validation...
    Ok((&args[1], &args[2]))
}
```

- ✅ Comprehensive error handling
- ✅ Type safety with Result
- **Use when**: Production applications

### Method 4: Safe Access Patterns

**Pattern**: Multiple defensive programming techniques

```rust
// Safe with get()
if let Some(query) = args.get(1) { /* use query */ }

// Pattern matching on length
match args.len() {
    1 => println!("No arguments"),
    2 => println!("Only query: {}", args[1]),
    3 => println!("Query: {}, File: {}", args[1], args[2]),
    _ => println!("Too many arguments"),
}
```

- ✅ Panic-safe
- ✅ Multiple approaches shown
- **Use when**: Learning defensive patterns

## 🛡️ Safety Principles

### Avoid Direct Indexing

```rust
// ❌ Dangerous - can panic
let query = &args[1];

// ✅ Safe alternatives
let query = args.get(1).unwrap_or("default");
let query = match args.get(1) { Some(q) => q, None => return };
```

### Error Handling Evolution

1. **Basic**: Print message and return
2. **Better**: Use Result types for composability  
3. **Best**: Structured error types with context

## 🔗 Integration Points

### Chapter 12 Connection

- **File Location**: `rust_book/Ch12/accepting_arguments/ARGUMENT_METHODS.md`
- **Implementation**: Complete working examples in `main.rs`
- **Testing**: Multiple test cases for all methods

### Mission Connections

- **Mission 8**: Advanced CLI argument structures
- **Mission 9**: Pathfinding tools with complex argument parsing
- **Daily Study**: [[daily-study/Day42]]

### Error Handling Links

- **Patterns**: [[Error Propagation]], [[Result Type Usage]]
- **Best Practices**: [[Defensive Programming]], [[Input Validation]]

## 📚 Learning Progression

### Beginner Path

1. Start with Method 1 (basic collection)
2. Understand argument indexing
3. Learn about panic risks

### Intermediate Path  

1. Master Method 2 (direct parsing)
2. Add basic error checking
3. Understand when each method applies

### Advanced Path

1. Implement Method 3 (Result validation)
2. Design custom error types
3. Consider argument parsing libraries (clap, structopt)

## 🛠️ Real-World Applications

### When to Use Each Method

- **Method 1**: Debugging, learning, argument inspection
- **Method 2**: Simple utilities, internal tools
- **Method 3**: Production CLI applications
- **Method 4**: Educational examples, defensive coding

### Library Considerations

For complex applications, consider:

- **clap**: Full-featured argument parsing
- **structopt**: Derive-based approach
- **argh**: Lightweight alternative

## 🎓 Key Takeaways

1. **Progressive Complexity**: Start simple, add validation as needed
2. **Safety First**: Avoid panicking on user input
3. **Error Context**: Provide helpful error messages
4. **Composability**: Use Result types for error handling
5. **Documentation**: Always document expected arguments

---

*Comprehensive guide to command-line argument handling in Rust, demonstrating the evolution from basic to production-ready patterns.*

*Links: [[zettelkasten/rust_book/rust-book-ch12]] | [[Rust CLI Applications]] | [[Error Handling Patterns]]*
*Tags: #rust #cli #arguments #error-handling #validation #defensive-programming #learning-progression*
