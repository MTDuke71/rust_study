# Non-Exhaustive Pattern (#[non_exhaustive])

*Tags: #rust #api-design #breaking-changes #struct-evolution #enum-evolution*

## Overview

The `#[non_exhaustive]` attribute tells Rust that a struct or enum may gain additional fields/variants in the future. This prevents downstream code from breaking when you add fields/variants, enabling **non-breaking API evolution**.

## The Problem: Field Addition Breaks Code

Even adding **private fields** to a public struct can break downstream code in two ways:

### 1. Constructor Breakage

```rust
// Library v1.0
pub struct Config {
    pub timeout: u32,
}

// User code:
let cfg = Config { timeout: 30 }; // ✅ Works
```

```rust
// Library v1.1 - Add private field
pub struct Config {
    pub timeout: u32,
    cache_size: usize, // ❌ BREAKS user code!
}

// User code now fails:
let cfg = Config { timeout: 30 }; // ❌ Error: missing field `cache_size`
```

**Problem**: Users relied on **implicit struct constructor**, which changes when fields are added.

### 2. Pattern Matching Breakage

```rust
// Library v1.0
pub struct Unit {
    pub field: bool,
}

// User code:
match unit {
    Unit { field: true } => { /* ... */ } // ✅ Works
    Unit { field: false } => { /* ... */ }
}
```

```rust
// Library v1.1 - Add private field
pub struct Unit {
    pub field: bool,
    internal: u32, // ❌ BREAKS user code!
}

// User code now fails:
match unit {
    Unit { field: true } => { /* ... */ } // ❌ Error: missing field `internal`
    Unit { field: false } => { /* ... */ }
}
```

**Problem**: Exhaustive pattern matching requires all fields to be listed.

## The Solution: #[non_exhaustive]

```rust
#[non_exhaustive]
pub struct Config {
    pub timeout: u32,
}
```

**Effects**:
1. **Disables struct literal construction** outside defining crate
   ```rust
   // Users CANNOT do:
   let cfg = Config { timeout: 30 }; // ❌ Error
   
   // Must use constructor:
   let cfg = Config::new(30); // ✅ OK
   ```

2. **Requires wildcard in pattern matching**
   ```rust
   // Users MUST include ..:
   match config {
       Config { timeout, .. } => { /* ... */ } // ✅ OK
   }
   
   // Cannot do exhaustive match:
   match config {
       Config { timeout } => { /* ... */ } // ❌ Error: missing ..
   }
   ```

3. **Allows adding fields without breaking changes**
   ```rust
   // Adding fields is now safe:
   #[non_exhaustive]
   pub struct Config {
       pub timeout: u32,
       pub cache_size: usize, // ✅ Not breaking!
       internal: String,       // ✅ Not breaking!
   }
   ```

## Usage Patterns

### Structs with #[non_exhaustive]

```rust
#[non_exhaustive]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

impl ServerConfig {
    // Provide constructor
    pub fn new(port: u16, host: String) -> Self {
        Self { port, host }
    }
    
    // Or builder pattern
    pub fn builder() -> ServerConfigBuilder { /* ... */ }
}
```

**User code**:
```rust
// Must use constructor:
let cfg = ServerConfig::new(8080, "localhost".into());

// Or builder:
let cfg = ServerConfig::builder()
    .port(8080)
    .host("localhost")
    .build();

// Pattern matching requires wildcard:
match cfg {
    ServerConfig { port, .. } => println!("Port: {}", port),
}
```

### Enums with #[non_exhaustive]

```rust
#[non_exhaustive]
pub enum Error {
    IoError(io::Error),
    ParseError(String),
    // Can add variants later without breaking
}
```

**Effect**: Users must include wildcard in match:
```rust
match error {
    Error::IoError(e) => { /* ... */ },
    Error::ParseError(s) => { /* ... */ },
    _ => { /* catch future variants */ } // ✅ Required!
}
```

## When to Use #[non_exhaustive]

**Use for structs when**:
- ✅ You expect to add fields in future versions
- ✅ Struct is configuration/options type
- ✅ You want to enforce constructor usage
- ✅ Preventing exhaustive matching is acceptable

**Use for enums when**:
- ✅ You plan to add variants over time
- ✅ Error types that may grow
- ✅ Event/message types that evolve

**Don't use when**:
- ❌ Type is truly final and won't change
- ❌ Users need direct field access patterns
- ❌ Small, stable types (unnecessary friction)

## Comparison: Within vs Outside Defining Crate

```rust
// library crate
#[non_exhaustive]
pub struct Config {
    pub timeout: u32,
}

impl Config {
    // WITHIN crate: full access
    pub fn new(timeout: u32) -> Self {
        Self { timeout } // ✅ OK - same crate
    }
    
    pub fn advanced(timeout: u32, internal: String) -> Self {
        Self {
            timeout,
            // Can add private fields here
        }
    }
}
```

```rust
// user crate
use library::Config;

// OUTSIDE crate: restricted
let cfg = Config { timeout: 30 }; // ❌ Error: non_exhaustive
let cfg = Config::new(30);        // ✅ OK - use constructor

match cfg {
    Config { timeout } => { }      // ❌ Error: missing ..
    Config { timeout, .. } => { }  // ✅ OK
}
```

## Real-World Examples

### Standard Library

```rust
#[non_exhaustive]
pub struct SocketAddr {
    // Can add IPv6-specific fields later
}

#[non_exhaustive]
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
    // Reserved right to add future IP versions
}
```

### Common Pattern: Config/Options Structs

```rust
#[non_exhaustive]
pub struct HttpClientOptions {
    pub timeout: Duration,
    pub max_redirects: usize,
    pub user_agent: String,
    // Can add: compression, proxies, etc. later
}

impl Default for HttpClientOptions {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_redirects: 10,
            user_agent: "MyClient/1.0".into(),
        }
    }
}
```

## Interaction with Other Patterns

### With Builder Pattern

```rust
#[non_exhaustive]
pub struct Request {
    pub url: String,
    pub method: Method,
    pub headers: HashMap<String, String>,
}

// Builder provides construction
pub struct RequestBuilder { /* ... */ }

impl RequestBuilder {
    pub fn new(url: impl Into<String>) -> Self { /* ... */ }
    pub fn method(mut self, method: Method) -> Self { /* ... */ }
    pub fn header(mut self, k: String, v: String) -> Self { /* ... */ }
    pub fn build(self) -> Request { /* ... */ }
}
```

### With Default Trait

```rust
#[non_exhaustive]
pub struct Config {
    pub port: u16,
    pub host: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "localhost".into(),
        }
    }
}

// Users can:
let cfg = Config::default();
let cfg = Config { port: 3000, ..Config::default() }; // ❌ Still error!
```

**Note**: Even with `Default`, struct literal syntax is disallowed outside crate.

## Migration Strategy

### Adding #[non_exhaustive] to Existing Type

**This is a BREAKING change** because it disables existing struct literals:

```rust
// v1.0 (without attribute)
pub struct Config {
    pub timeout: u32,
}

// Users wrote:
let cfg = Config { timeout: 30 }; // ✅ Works

// v2.0 (with attribute) - BREAKING!
#[non_exhaustive]
pub struct Config {
    pub timeout: u32,
}

// Users' code now breaks:
let cfg = Config { timeout: 30 }; // ❌ Error
```

**Migration path**:
1. Add constructors in v1.1 (non-breaking)
2. Deprecate struct literal usage (warnings)
3. Add `#[non_exhaustive]` in v2.0 (breaking change, major version bump)
4. Document migration in changelog

## Benefits and Trade-offs

**Benefits**:
- ✅ Add fields without breaking changes
- ✅ Enforces constructor usage (better API control)
- ✅ Prevents exhaustive matching issues
- ✅ Clear signal of evolution intent

**Trade-offs**:
- ❌ Less ergonomic for users (can't use literals)
- ❌ Requires more boilerplate (constructors/builders)
- ❌ Pattern matching needs wildcards (less precise)
- ❌ Adding attribute itself is breaking change

## Related Patterns

- **[[sealed-traits]]**: Prevents external trait implementations
- **Builder Pattern**: Provides ergonomic construction for #[non_exhaustive] types
- **Default Trait**: Provides baseline values for extensible types
- **Newtype Pattern**: Alternative to adding fields (wrap existing type)

## Source

- **Rust for Rustaceans**, Jon Gjengset, Chapter 3: Designing Interfaces, pp. 50-51
- Related to API evolution and breaking changes

---

*Links: [[api-design-principles]] | [[breaking-changes]] | [[struct-evolution]] | [[rust-for-rustaceans-ch3]]*

*Created: 2026-01-16*
