# 🎯 API Design Patterns in Rust

**Comprehensive guide to designing clean, idiomatic, and maintainable public interfaces in Rust**

*This guide focuses on concrete implementation patterns. For the foundational philosophy and abstract guidelines, see [[API Design Principles]].*

---

## 🏗️ **Core Design Principles**

### **Rust's API Design Philosophy**
Rust's approach to API design emphasizes:
- **Zero-cost abstractions** - High-level APIs with no runtime overhead
- **Explicit error handling** - No hidden failures or exceptions
- **Memory safety** - APIs that prevent undefined behavior
- **Ownership clarity** - Clear semantics about who owns what
- **Ergonomic usage** - Easy to use correctly, hard to use incorrectly

*For detailed exploration of these foundational principles, see [[API Design Principles]].*

### **The "Principle of Least Surprise"**
```rust
// ✅ GOOD: Follows Rust conventions
impl HashMap<K, V> {
    pub fn new() -> Self { /* ... */ }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> { /* ... */ }
    pub fn get(&self, key: &K) -> Option<&V> { /* ... */ }
    pub fn remove(&mut self, key: &K) -> Option<V> { /* ... */ }
}

// ❌ BAD: Surprising return types and naming
impl HashMap<K, V> {
    pub fn create() -> Result<Self, ()> { /* ... */ }  // new() should not fail
    pub fn add(&mut self, key: K, value: V) -> bool { /* ... */ }  // Should return replaced value
    pub fn find(&self, key: &K) -> V { /* ... */ }  // Should return Option, not panic
}
```

---

## 🔧 **Constructor Patterns**

### **Standard Constructor: `new()`**
```rust
// ✅ Simple constructor for zero-parameter initialization
impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec { /* ... */ }
    }
}

// ✅ Constructor with parameters
impl<T> HashMap<K, V> {
    pub fn with_capacity(capacity: usize) -> Self {
        HashMap { /* ... */ }
    }
}
```

### **Builder Pattern for Complex Construction**
```rust
pub struct ConfigBuilder {
    debug: bool,
    max_connections: usize,
    timeout: Duration,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            debug: false,
            max_connections: 100,
            timeout: Duration::from_secs(30),
        }
    }
    
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
    
    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }
    
    pub fn build(self) -> Config {
        Config {
            debug: self.debug,
            max_connections: self.max_connections,
            timeout: self.timeout,
        }
    }
}

// Usage: Fluent and readable
let config = ConfigBuilder::new()
    .debug(true)
    .max_connections(200)
    .build();
```

### **Type-State Pattern for Compile-Time Safety**
```rust
pub struct ConnectionBuilder<State> {
    url: Option<String>,
    _state: PhantomData<State>,
}

pub struct NeedsUrl;
pub struct NeedsAuth;
pub struct Ready;

impl ConnectionBuilder<NeedsUrl> {
    pub fn new() -> Self {
        Self { url: None, _state: PhantomData }
    }
    
    pub fn url(self, url: String) -> ConnectionBuilder<NeedsAuth> {
        ConnectionBuilder { url: Some(url), _state: PhantomData }
    }
}

impl ConnectionBuilder<NeedsAuth> {
    pub fn auth(self, token: String) -> ConnectionBuilder<Ready> {
        // Add auth logic
        ConnectionBuilder { url: self.url, _state: PhantomData }
    }
}

impl ConnectionBuilder<Ready> {
    pub fn connect(self) -> Result<Connection, Error> {
        // Only Ready state can actually connect
        Ok(Connection { /* ... */ })
    }
}
```

---

## 📋 **Method Design Patterns**

### **Ownership and Borrowing Conventions**
```rust
impl<T> MyCollection<T> {
    // Taking ownership - consumes the value
    pub fn push(&mut self, item: T) { /* ... */ }
    
    // Borrowing immutably - most common for queries
    pub fn get(&self, index: usize) -> Option<&T> { /* ... */ }
    
    // Borrowing mutably - for in-place modifications
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> { /* ... */ }
    
    // Returning owned values - when caller needs ownership
    pub fn pop(&mut self) -> Option<T> { /* ... */ }
    
    // Into methods - consume self and transform
    pub fn into_inner(self) -> Vec<T> { /* ... */ }
}
```

### **Infallible vs Fallible Operations**
```rust
impl<T> MyCollection<T> {
    // Infallible operations - always succeed
    pub fn len(&self) -> usize { /* ... */ }
    pub fn is_empty(&self) -> bool { /* ... */ }
    pub fn push(&mut self, item: T) { /* ... */ }
    
    // Fallible operations - return Option for logical failure
    pub fn get(&self, index: usize) -> Option<&T> { /* ... */ }
    pub fn pop(&mut self) -> Option<T> { /* ... */ }
    
    // Fallible operations - return Result for error conditions
    pub fn save_to_file(&self, path: &Path) -> Result<(), IoError> { /* ... */ }
    pub fn parse_from_str(s: &str) -> Result<Self, ParseError> { /* ... */ }
}
```

### **Method Naming Conventions**
```rust
impl MyType {
    // Query methods - immutable access
    pub fn get(&self, key: &K) -> Option<&V> { /* ... */ }
    pub fn contains(&self, key: &K) -> bool { /* ... */ }
    pub fn find(&self, predicate: impl Fn(&T) -> bool) -> Option<&T> { /* ... */ }
    
    // Modification methods - mutable access
    pub fn insert(&mut self, key: K, value: V) -> Option<V> { /* ... */ }
    pub fn remove(&mut self, key: &K) -> Option<V> { /* ... */ }
    pub fn update(&mut self, key: &K, f: impl FnOnce(&mut V)) -> bool { /* ... */ }
    
    // Conversion methods
    pub fn as_slice(&self) -> &[T] { /* ... */ }       // Cheap reference conversion
    pub fn to_vec(&self) -> Vec<T> { /* ... */ }       // Expensive clone conversion
    pub fn into_vec(self) -> Vec<T> { /* ... */ }      // Zero-cost ownership transfer
}
```

---

## 🛡️ **Error Handling Patterns**

### **Error Type Design**
```rust
// ✅ Comprehensive error type with context
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Invalid email format: {email}")]
    InvalidEmail { email: String },
    
    #[error("Password too short: {length} characters (minimum {min})")]
    PasswordTooShort { length: usize, min: usize },
    
    #[error("Missing required field: {field}")]
    MissingField { field: String },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// Usage provides clear error context
impl User {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if !self.email.contains('@') {
            return Err(ValidationError::InvalidEmail { 
                email: self.email.clone() 
            });
        }
        
        if self.password.len() < 8 {
            return Err(ValidationError::PasswordTooShort { 
                length: self.password.len(), 
                min: 8 
            });
        }
        
        Ok(())
    }
}
```

### **Result vs Option Guidelines**
```rust
impl Configuration {
    // Use Option for missing/optional values
    pub fn get_setting(&self, key: &str) -> Option<&str> { /* ... */ }
    
    // Use Result for operations that can fail with detailed errors
    pub fn load_from_file(path: &Path) -> Result<Self, ConfigError> { /* ... */ }
    pub fn validate(&self) -> Result<(), ValidationError> { /* ... */ }
    
    // Panic for programming errors (use sparingly)
    pub fn get_required_setting(&self, key: &str) -> &str {
        self.get_setting(key)
            .unwrap_or_else(|| panic!("Required setting '{}' not found", key))
    }
}
```

---

## 🔄 **Iterator Design Patterns**

### **Standard Iterator Implementation**
```rust
impl<T> MyCollection<T> {
    // Standard iteration patterns
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
    
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }
    
    // IntoIterator for owned iteration
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.data.into_iter()
    }
}

// Enable for-loop syntax
impl<T> IntoIterator for MyCollection<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> IntoIterator for &MyCollection<T> {
    type Item = &T;
    type IntoIter = std::slice::Iter<T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
```

### **Custom Iterator Types**
```rust
pub struct FilteredIter<I, F> {
    inner: I,
    predicate: F,
}

impl<I, F, T> Iterator for FilteredIter<I, F>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> bool,
{
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.find(&self.predicate)
    }
}

impl<T> MyCollection<T> {
    pub fn filter<F>(&self, predicate: F) -> FilteredIter<std::slice::Iter<T>, F>
    where
        F: Fn(&T) -> bool,
    {
        FilteredIter {
            inner: self.iter(),
            predicate,
        }
    }
}
```

---

## 🏷️ **Trait Design Patterns**

### **Marker Traits for Capabilities**
```rust
// Marker trait for types that can be safely shared between threads
pub trait ThreadSafe: Send + Sync {}

// Automatic implementation for qualifying types
impl<T: Send + Sync> ThreadSafe for T {}

// Usage in API bounds
pub fn process_concurrently<T: ThreadSafe>(data: T) -> impl Future<Output = T> {
    // ...
}
```

### **Extension Traits for Additional Functionality**
```rust
// Extend existing types with domain-specific methods
pub trait VecExt<T> {
    fn push_unique(&mut self, item: T) -> bool;
    fn remove_item(&mut self, item: &T) -> bool;
}

impl<T: PartialEq> VecExt<T> for Vec<T> {
    fn push_unique(&mut self, item: T) -> bool {
        if !self.contains(&item) {
            self.push(item);
            true
        } else {
            false
        }
    }
    
    fn remove_item(&mut self, item: &T) -> bool {
        if let Some(pos) = self.iter().position(|x| x == item) {
            self.remove(pos);
            true
        } else {
            false
        }
    }
}
```

### **Associated Type vs Generic Parameter Trade-offs**
```rust
// ✅ Associated types - one natural output type per implementation
pub trait Collect<T> {
    type Output;
    fn collect(self) -> Self::Output;
}

impl<T> Collect<T> for Vec<T> {
    type Output = Vec<T>;  // Natural output for Vec
    fn collect(self) -> Self::Output { self }
}

// ✅ Generic parameters - multiple possible output types
pub trait ConvertTo<T, U> {
    fn convert(self) -> Result<U, ConversionError>;
}

impl ConvertTo<String, i32> for String {
    fn convert(self) -> Result<i32, ConversionError> {
        self.parse().map_err(|_| ConversionError::InvalidFormat)
    }
}

impl ConvertTo<String, f64> for String {
    fn convert(self) -> Result<f64, ConversionError> {
        self.parse().map_err(|_| ConversionError::InvalidFormat)
    }
}
```

---

## 📖 **Documentation Patterns**

### **Comprehensive API Documentation**
```rust
/// Validates bracket sequences for proper nesting and matching.
/// 
/// This function implements a stack-based algorithm to verify that bracket
/// sequences follow proper nesting rules. Only `()`, `[]`, and `{}` are
/// considered brackets; all other characters are ignored.
/// 
/// # Arguments
/// 
/// * `input` - The string containing brackets to validate
/// 
/// # Returns
/// 
/// * `Ok(())` - All brackets are properly matched and nested
/// * `Err(BracketError)` - Validation failed with error details
/// 
/// # Errors
/// 
/// Returns [`BracketError`] in these cases:
/// * [`UnexpectedClosing`] - Found closing bracket without matching opener
/// * [`MismatchedPair`] - Wrong closing bracket type
/// * [`UnclosedOpenings`] - Input ended with unmatched opening brackets
/// 
/// # Performance
/// 
/// Time complexity: O(n) where n is the length of the input string.
/// Space complexity: O(n) in worst case (all opening brackets).
/// 
/// # Examples
/// 
/// ```rust
/// use my_crate::validate_brackets;
/// 
/// // Valid sequences
/// assert!(validate_brackets("()").is_ok());
/// assert!(validate_brackets("([{}])").is_ok());
/// assert!(validate_brackets("hello(world)").is_ok());
/// 
/// // Invalid sequences
/// assert!(validate_brackets("(]").is_err());
/// assert!(validate_brackets("((").is_err());
/// ```
/// 
/// # See Also
/// 
/// * [`BracketError`] - Error type returned by this function
/// * [`parse_brackets`] - For extracting bracket information
pub fn validate_brackets(input: &str) -> Result<(), BracketError> {
    // Implementation...
}
```

### **Module-Level Documentation**
```rust
//! # Bracket Validation Library
//! 
//! This crate provides robust bracket sequence validation with detailed
//! error reporting and position tracking.
//! 
//! ## Quick Start
//! 
//! ```rust
//! use bracket_validator::validate_brackets;
//! 
//! let result = validate_brackets("([{}])");
//! assert!(result.is_ok());
//! ```
//! 
//! ## Features
//! 
//! * **Zero-copy validation** - No string allocation during processing
//! * **Detailed error reporting** - Exact position and error type
//! * **Configurable bracket types** - Support custom bracket sets
//! * **Performance optimized** - O(n) time, minimal memory usage
//! 
//! ## Error Handling
//! 
//! All validation functions return [`Result<(), BracketError>`] with
//! comprehensive error information including position and error type.
```

---

## 🔧 **Configuration and Customization Patterns**

### **Configuration Structs with Defaults**
```rust
#[derive(Debug, Clone)]
pub struct ParserConfig {
    pub ignore_whitespace: bool,
    pub case_sensitive: bool,
    pub max_depth: usize,
    pub timeout: Duration,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            ignore_whitespace: true,
            case_sensitive: false,
            max_depth: 100,
            timeout: Duration::from_secs(10),
        }
    }
}

impl ParserConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn strict() -> Self {
        Self {
            ignore_whitespace: false,
            case_sensitive: true,
            ..Self::default()
        }
    }
    
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}
```

### **Strategy Pattern with Trait Objects**
```rust
pub trait HashStrategy {
    fn hash(&self, data: &[u8]) -> u64;
}

pub struct SimpleHash;
impl HashStrategy for SimpleHash {
    fn hash(&self, data: &[u8]) -> u64 {
        // Simple hash implementation
    }
}

pub struct CryptoHash;
impl HashStrategy for CryptoHash {
    fn hash(&self, data: &[u8]) -> u64 {
        // Cryptographic hash implementation
    }
}

pub struct HashTable {
    strategy: Box<dyn HashStrategy>,
    // ...
}

impl HashTable {
    pub fn new(strategy: Box<dyn HashStrategy>) -> Self {
        Self { strategy, /* ... */ }
    }
    
    pub fn with_simple_hash() -> Self {
        Self::new(Box::new(SimpleHash))
    }
    
    pub fn with_crypto_hash() -> Self {
        Self::new(Box::new(CryptoHash))
    }
}
```

---

## 🎯 **Forward Compatibility Patterns**

### **Rest Patterns for Struct Evolution**
```rust
pub struct Config {
    pub debug: bool,
    pub max_connections: usize,
    // Future fields can be added here
}

// Forward-compatible pattern matching
fn process_config(config: Config) {
    let Config { debug, max_connections, .. } = config;
    // `..` allows future fields to be added without breaking this code
}
```

### **Non-Exhaustive Enums**
```rust
#[non_exhaustive]
pub enum ApiVersion {
    V1,
    V2,
    // Future versions can be added without breaking client code
}

// Clients must handle unknown variants
fn handle_version(version: ApiVersion) {
    match version {
        ApiVersion::V1 => { /* ... */ },
        ApiVersion::V2 => { /* ... */ },
        _ => { /* Handle unknown versions gracefully */ },
    }
}
```

### **Feature Flags and Conditional Compilation**
```rust
#[cfg(feature = "advanced-parsing")]
impl Parser {
    pub fn parse_advanced(&self, input: &str) -> Result<Ast, ParseError> {
        // Advanced parsing only available with feature flag
    }
}

#[cfg(not(feature = "advanced-parsing"))]
impl Parser {
    pub fn parse_advanced(&self, _input: &str) -> Result<Ast, ParseError> {
        Err(ParseError::FeatureNotEnabled("advanced-parsing"))
    }
}
```

---

## 📊 **Performance-Conscious API Design**

### **Zero-Cost Abstractions**
```rust
// Iterator that compiles to the same assembly as hand-written loop
impl<T> MyCollection<T> {
    pub fn find<P>(&self, predicate: P) -> Option<&T>
    where
        P: Fn(&T) -> bool,
    {
        for item in &self.data {
            if predicate(item) {
                return Some(item);
            }
        }
        None
    }
}

// Generic implementation that monomorphizes to specialized versions
pub fn sort_by<T, F>(slice: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    // Each instantiation gets its own optimized implementation
    slice.sort_by(compare);
}
```

### **Avoiding Unnecessary Allocations**
```rust
impl MyString {
    // ✅ Returns iterator instead of allocating Vec
    pub fn words(&self) -> impl Iterator<Item = &str> {
        self.data.split_whitespace()
    }
    
    // ✅ In-place modification when possible
    pub fn truncate(&mut self, new_len: usize) {
        self.data.truncate(new_len);
    }
    
    // ✅ Cow for conditional allocation
    pub fn ensure_prefix(&self, prefix: &str) -> Cow<str> {
        if self.data.starts_with(prefix) {
            Cow::Borrowed(&self.data)
        } else {
            Cow::Owned(format!("{}{}", prefix, self.data))
        }
    }
}
```

---

## ✅ **API Design Checklist**

### **Consistency Checklist**
- [ ] **Naming follows Rust conventions** (snake_case for functions, PascalCase for types)
- [ ] **Method names are descriptive and unambiguous**
- [ ] **Similar operations have consistent signatures**
- [ ] **Error types provide sufficient context for debugging**
- [ ] **Documentation includes examples and error conditions**

### **Usability Checklist**
- [ ] **Common operations are easy and obvious**
- [ ] **Uncommon operations are possible but clearly marked**
- [ ] **Impossible operations are prevented at compile time**
- [ ] **Default behavior is sensible for most use cases**
- [ ] **Configuration is available for edge cases**

### **Performance Checklist**
- [ ] **No unnecessary allocations in hot paths**
- [ ] **Iterator-based APIs where appropriate**
- [ ] **Zero-cost abstractions are truly zero-cost**
- [ ] **Large objects use references rather than copying**
- [ ] **Performance characteristics are documented**

### **Forward Compatibility Checklist**
- [ ] **Structs use rest patterns where evolution is expected**
- [ ] **Enums are marked #[non_exhaustive] for public APIs**
- [ ] **New functionality can be added without breaking changes**
- [ ] **Feature flags separate optional functionality**
- [ ] **Version compatibility is clearly documented**

---

## 🔗 **Real-World Examples**

### **Mission Implementations**
- **[[mission-1]]** - Stack API with push/pop operations
- **[[mission-2]]** - Queue API with FIFO semantics
- **[[Mission5 Overview]]** - HashMap API with key-value operations

### **Standard Library Patterns**
- **[`Vec<T>`]** - Dynamic array with growth strategies
- **[`HashMap<K, V>`]** - Hash table with configurable hasher
- **[`Result<T, E>`]** - Explicit error handling without exceptions
- **[`Option<T>`]** - Null safety with explicit absence

### **External Crate Examples**
- **[[anyhow and thiserror]]** - Error handling library design patterns
- **[`serde`]** - Serialization API with derive macros
- **[`tokio`]** - Async runtime with trait-based abstractions
- **[`clap`]** - Command-line parsing with builder pattern

---

## 📚 **Learning Resources**

### **Rust API Guidelines**
- [Official Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - Comprehensive style guide
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - Language fundamentals
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples

### **Advanced Topics**
- **[[Unsafe Rust - Raw Pointers and Safety Contracts]]** - Safe abstractions over unsafe code
- **[[PhantomData Type Safety Patterns]]** - Zero-cost type safety
- **[[Memory Address Analysis]]** - Understanding memory layout and performance

### **Project-Specific Resources**
- **[[RUST_DOCUMENTATION_STANDARDS]]** - Documentation guidelines for this workspace
- **[[V-Cycle in Rust Development]]** - Requirements-driven API design
- **[[Testing Strategies]]** - Comprehensive API testing approaches

---

*Tags: #api-design #patterns #rust #public-interface #ergonomics #performance #forward-compatibility #best-practices*
*Links: [[API Design Principles]] | [[zettel-index]] | [[rust-book-ch9-12-review]] | [[rust-concepts-MOC]] | [[Mission5 Overview]] | [[Error Handling Patterns]] | [[Iterator Patterns]] | [[anyhow and thiserror]] | [[Rest Patterns]] | [[Memory Address Analysis]]*