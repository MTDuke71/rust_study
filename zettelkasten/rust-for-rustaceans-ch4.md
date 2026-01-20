# 🦀 Rust for Rustaceans - Chapter 4: Error Handling

**Deep dive into error representation strategies, propagation patterns, and production-ready error types**

*From: Rust for Rustaceans by Jon Gjengset - Chapter 4*

---

## 🎯 Overview

Chapter 4 explores the three fundamental strategies for representing errors in Rust, when to use each approach, and how to implement robust error handling in both library and application code.

**Core Question**: How should errors be represented to balance:
- **Caller needs**: Type information vs. flexibility
- **Performance**: Stack vs. heap allocation
- **Ergonomics**: Pattern matching vs. error propagation
- **Maintainability**: Precise types vs. adaptable interfaces

---

## 🏗️ Three Error Representation Strategies

### 1. Enumeration Errors - Detailed Type Information

**Use when**: Callers need to match on specific error variants, library APIs, domain-specific errors

**Characteristics**:
- ✅ Precise type information - callers can pattern match
- ✅ Exhaustive matching enforced by compiler
- ✅ Self-documenting error cases
- ❌ Less flexible - adding variants is breaking change
- ❌ Callers must handle all variants

**Implementation**: `rust_for_rustaceans/Ch04/examples/enumeration_errors.rs`

```rust
#[derive(Debug)]
pub enum CopyError {
    ReadError { path: PathBuf, source: io::Error },
    WriteError { path: PathBuf, source: io::Error },
}

impl std::error::Error for CopyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CopyError::ReadError { source, .. } => Some(source),
            CopyError::WriteError { source, .. } => Some(source),
        }
    }
}
```

**Examples in Ch04**:
- `CopyError` - File operations with read/write distinction
- `ConfigError` - Configuration loading with file/parse/validation errors
- `HttpError` - HTTP requests with network/status/parse errors
- `DbError` - Database operations with connection/query/constraint errors

**Real-world application**: [[workflow-pattern-matching]] (AoC Day 19) uses `Destination` enum with same philosophy

### 2. Opaque Errors - Type-Erased Flexibility

**Use when**: Application code, error types may change, propagating diverse errors, don't need to match variants

**Characteristics**:
- ✅ Maximum flexibility - accepts any error type
- ✅ Easy composition - mix different error sources
- ✅ Non-breaking changes - can change underlying errors
- ❌ No pattern matching - must downcast to recover type
- ❌ Heap allocation overhead
- ❌ Dynamic dispatch cost

**Implementation**: `rust_for_rustaceans/Ch04/examples/opaque_errors.rs`

```rust
// Type alias for convenience
pub type AppResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn process_data(path: &str) -> AppResult<Data> {
    let content = std::fs::read_to_string(path)?;  // io::Error
    let data = serde_json::from_str(&content)?;    // serde_json::Error
    Ok(data)
}
```

**Trait bounds**:
- `Send` - Error can be transferred across threads
- `Sync` - Error can be shared across threads
- Both required for async/multi-threaded applications

### 3. Special Cases - Unit Errors and Never Type

**Use when**: Single error mode, marker types, operations that never fail

**Implementation**: `rust_for_rustaceans/Ch04/examples/special_cases.rs`

**Unit Error** - Single error possibility:
```rust
#[derive(Debug)]
pub struct ParseError;  // No additional information needed

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to parse input")
    }
}

impl std::error::Error for ParseError {}
```

**Never Type** - Operations that can't fail:
```rust
// Infallible conversion
impl TryFrom<String> for ValidatedString {
    type Error = !;  // Never fails (unstable)
    
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(ValidatedString(s))
    }
}
```

---

## 🔄 Error Propagation Patterns

### The `?` Operator

**Implementation**: `rust_for_rustaceans/Ch04/examples/error_propagation.rs`

```rust
fn copy_file(src: &Path, dst: &Path) -> Result<u64, io::Error> {
    let mut input = File::open(src)?;   // Early return on error
    let mut output = File::create(dst)?; // Early return on error
    io::copy(&mut input, &mut output)    // Return result directly
}
```

**Desugared equivalent**:
```rust
let mut input = match File::open(src) {
    Ok(file) => file,
    Err(e) => return Err(e.into()),  // Calls From::from(e)
};
```

### From Trait for Error Conversion

```rust
#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
}

// Automatic conversion via From trait
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::Parse(err)
    }
}

fn process() -> Result<Data, AppError> {
    let content = std::fs::read_to_string("file.txt")?;  // io::Error → AppError
    let number: u32 = content.trim().parse()?;             // ParseIntError → AppError
    Ok(Data { number })
}
```

**Key insight**: `?` operator uses `From` trait to convert between error types automatically!

### Error Source Chains

**Pattern**: Traverse `source()` chain to display full error context

```rust
fn print_error_chain(mut err: &dyn std::error::Error) {
    eprintln!("Error: {}", err);
    while let Some(source) = err.source() {
        eprintln!("  Caused by: {}", source);
        err = source;
    }
}
```

**Example output**:
```
Error: Failed to copy file 'input.txt' to 'output.txt'
  Caused by: Failed to read input file 'input.txt'
  Caused by: No such file or directory (os error 2)
```

---

## 🛠️ Production Error Handling: thiserror + anyhow

### For Libraries: Use `thiserror`

**Implementation**: `rust_for_rustaceans/Ch04/examples/thiserror_anyhow.rs`

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file at {path}")]
    FileRead { path: String, #[source] source: io::Error },
    
    #[error("Failed to parse config: {0}")]
    ParseError(#[from] toml::de::Error),
    
    #[error("Invalid configuration: {field} must be {constraint}")]
    ValidationError { field: String, constraint: String },
}
```

**Benefits**:
- Derive macros reduce boilerplate
- `#[error(...)]` generates Display implementation
- `#[source]` and `#[from]` handle error chaining
- Type-safe, precise error information for library consumers

### For Applications: Use `anyhow`

```rust
use anyhow::{Context, Result};

fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config from {}", path))?;
    
    let config: Config = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config from {}", path))?;
    
    Ok(config)
}
```

**Benefits**:
- `anyhow::Error` accepts any error type
- `.context()` adds rich error messages
- Backtrace support (when RUST_BACKTRACE=1)
- Ergonomic for applications (not libraries!)

### Decision Matrix

| Situation | Use | Why |
|-----------|-----|-----|
| **Library public API** | `thiserror` | Precise types, callers can match variants |
| **Application code** | `anyhow` | Ergonomic, flexible, rich context |
| **Internal library code** | `thiserror` | Consistency, may expose later |
| **CLI tool** | `anyhow` | User-facing errors need context |
| **Long-lived service** | `thiserror` | Type stability, structured logging |

---

## 🎯 Error Design Principles

### 1. Caller-Centric Design

**Question**: What do callers need to do with this error?

- **Match and handle differently** → Enumeration error
- **Log and continue** → Opaque error with good Display
- **Recover programmatically** → Enumeration with data
- **Display to user** → Rich context (anyhow)

### 2. Error Composition

**Pattern**: Layer errors with context at each level

```rust
// Low-level: Precise database error
#[derive(Error, Debug)]
pub enum DbError {
    #[error("Connection failed")]
    ConnectionError(#[from] ConnectionError),
    
    #[error("Query failed: {query}")]
    QueryError { query: String, #[source] source: QueryError },
}

// Mid-level: Business logic error
#[derive(Error, Debug)]
pub enum UserError {
    #[error("User {user_id} not found")]
    NotFound { user_id: u64 },
    
    #[error("Database error")]
    DatabaseError(#[from] DbError),
}

// Top-level: Application error
fn process_user(user_id: u64) -> anyhow::Result<ProcessedUser> {
    let user = load_user(user_id)  // Returns Result<User, UserError>
        .with_context(|| format!("Failed to process user {}", user_id))?;
    
    Ok(process(user))
}
```

**Result**: Error messages show full context from application → business logic → database

### 3. Performance Considerations

**Enumeration errors** (stack allocated):
```rust
#[derive(Debug)]
pub enum FastError {
    InvalidInput,
    NotFound,
    PermissionDenied,
}

impl std::error::Error for FastError {}
```

**Opaque errors** (heap allocated):
```rust
pub type SlowResult<T> = Result<T, Box<dyn Error>>;  // Heap allocation
```

**Guideline**: Use enumeration errors in hot paths, opaque errors where ergonomics matter more

---

## 📊 Examples in Ch04 Code

### Example Programs

1. **`enumeration_errors.rs`** (237 lines)
   - CopyError, ConfigError, HttpError, DbError
   - Error source chain traversal
   - Pattern matching on error variants

2. **`opaque_errors.rs`** (194 lines)
   - Box`<dyn Error>` patterns
   - Downcast examples
   - Flexibility vs type safety tradeoffs

3. **`special_cases.rs`** (156 lines)
   - Unit errors (single error mode)
   - Never type (infallible operations)
   - Marker types for state

4. **`error_propagation.rs`** (289 lines)
   - ? operator mechanics
   - From trait implementations
   - Error chains and context
   - Custom ResultExt trait

5. **`custom_errors.rs`** (374 lines)
   - Production-quality error types
   - Multiple error sources
   - Display + Error trait implementations
   - Error recovery patterns

6. **`thiserror_anyhow.rs`** (412 lines)
   - thiserror for libraries
   - anyhow for applications
   - Context and backtrace
   - Real-world patterns

7. **`review.rs`** (1039 lines)
   - Comprehensive review of all patterns
   - Integration examples
   - Best practices demonstration

---

## 🔗 Related Concepts

**Mathematical Foundations**:
- [[math-foundations/combinatorics-fundamentals]] - Error enumeration = product type (variants × data)

**Zettelkasten**:
- [[Error Handling Patterns]] - Comprehensive error handling guide (updated with Ch4 cross-references)
- [[workflow-pattern-matching]] - Enum-based state machines (similar to enumeration errors)

**Daily Study**:
- [[daily-study/Day30]] - Error propagation with ? operator
- [[daily-study/Day31]] - anyhow and thiserror introduction
- [[daily-study/Day34]] - Advanced error patterns

**Rust Book**:
- [[../rust_book/Ch9/README]] - Foundational error handling (Result, panic, ?)

**Missions**:
- [[mission-1]] - Stack bounds checking errors
- [[mission-4]] - Interior mutability errors
- [[mission-5]] - HashMap validation errors

---

## 🎓 Key Takeaways

1. **Three strategies**: Enumeration (precise), Opaque (flexible), Special cases (specific situations)
2. **Library vs Application**: Libraries use `thiserror`, applications use `anyhow`
3. **Caller-centric design**: Choose error representation based on what callers need to do
4. **Error composition**: Layer errors with context at each abstraction level
5. **From trait magic**: Enables automatic error conversion with `?` operator
6. **Source chains**: Preserve context through error hierarchies
7. **Performance matters**: Enumeration = stack, Opaque = heap

---

*Tags: #rust-for-rustaceans #error-handling #thiserror #anyhow #enumeration-errors #opaque-errors #error-propagation #from-trait*

*Links: [[zettel-index]] | [[Error Handling Patterns]] | [[workflow-pattern-matching]] | [[daily-study/Day30]] | [[daily-study/Day31]] | [[../rust_book/Ch9/README]]*
