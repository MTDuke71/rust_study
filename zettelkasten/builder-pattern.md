# Builder Pattern - Flexible Object Construction

*Tags: #rust #design-pattern #api-design #construction #type-states*

## Overview

The **Builder Pattern** separates object construction from its final representation, enabling:
- Step-by-step configuration with sensible defaults
- Validation before object creation
- Compile-time enforcement of required fields (type-state builders)
- API evolution without breaking changes

**Source**: Common pattern in Rust ecosystem, referenced in RfR Ch3 (Interface Design)

**Related**: [[non-exhaustive-pattern]], [[common-traits-pattern]], [[asref-trait-ergonomics]], [[API Design Principles]], [[API Design Patterns]]

---

## The Problem: Complex Object Construction

```rust
// ❌ Constructor with many parameters - easy to mix up
let config = ServerConfig::new(
    8080,           // Is this port or timeout?
    30,             // Timeout? Max connections?
    true,           // What does this enable?
    false,
    "localhost".into(),
);

// ❌ Struct literal with #[non_exhaustive] - not allowed externally
let config = ServerConfig {
    port: 8080,
    host: "localhost".into(),
    // Error: can't use struct literal for #[non_exhaustive]
};
```

---

## Solution: Builder Pattern

### Basic Builder

```rust
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub timeout: u32,
    pub max_connections: usize,
}

#[derive(Debug, Default)]
pub struct ServerConfigBuilder {
    port: Option<u16>,
    host: Option<String>,
    timeout: Option<u32>,
    max_connections: Option<usize>,
}

impl ServerConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    pub fn timeout(mut self, seconds: u32) -> Self {
        self.timeout = Some(seconds);
        self
    }

    pub fn max_connections(mut self, count: usize) -> Self {
        self.max_connections = Some(count);
        self
    }

    pub fn build(self) -> Result<ServerConfig, &'static str> {
        Ok(ServerConfig {
            port: self.port.ok_or("port is required")?,
            host: self.host.unwrap_or_else(|| "localhost".into()),
            timeout: self.timeout.unwrap_or(30),
            max_connections: self.max_connections.unwrap_or(100),
        })
    }
}
```

**Usage**:
```rust
let config = ServerConfig::builder()
    .port(8080)
    .host("0.0.0.0")
    .timeout(60)
    .build()?;

// Sensible defaults - only required fields needed
let minimal = ServerConfig::builder()
    .port(3000)
    .build()?;
```

---

## Type-State Builder Pattern

**Problem**: Basic builder defers validation to runtime. Can we catch missing required fields at **compile time**?

**Solution**: Use PhantomData markers to track builder state.

```rust
use std::marker::PhantomData;

// State markers
pub struct NoPort;
pub struct HasPort;

pub struct ServerConfigBuilder<PortState> {
    _port_state: PhantomData<PortState>,
    port: Option<u16>,
    host: String,
    timeout: u32,
}

impl ServerConfigBuilder<NoPort> {
    pub fn new() -> Self {
        Self {
            _port_state: PhantomData,
            port: None,
            host: "localhost".into(),
            timeout: 30,
        }
    }

    // State transition: NoPort -> HasPort
    pub fn port(self, port: u16) -> ServerConfigBuilder<HasPort> {
        ServerConfigBuilder {
            _port_state: PhantomData,
            port: Some(port),
            host: self.host,
            timeout: self.timeout,
        }
    }
}

impl<S> ServerConfigBuilder<S> {
    // Optional fields - available in any state
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    pub fn timeout(mut self, seconds: u32) -> Self {
        self.timeout = seconds;
        self
    }
}

impl ServerConfigBuilder<HasPort> {
    // build() only available when port is set!
    pub fn build(self) -> ServerConfig {
        ServerConfig {
            port: self.port.unwrap(), // Safe: type guarantees Some
            host: self.host,
            timeout: self.timeout,
        }
    }
}
```

**Compile-time enforcement**:
```rust
// ✅ Compiles - port is set
let config = ServerConfigBuilder::new()
    .port(8080)
    .host("0.0.0.0")
    .build();

// ❌ Won't compile - build() not available without port
let config = ServerConfigBuilder::new()
    .host("0.0.0.0")
    .build(); // Error: method `build` not found for `ServerConfigBuilder<NoPort>`
```

---

## Pattern: Builder with #[non_exhaustive]

The Builder pattern complements [[non-exhaustive-pattern]] perfectly:

```rust
#[non_exhaustive]  // Future-proof the struct
pub struct ParseOptions {
    pub case_sensitive: bool,
    pub max_depth: usize,
    // Can add fields in future versions
}

impl ParseOptions {
    // Provide builder as the ONLY way to construct
    pub fn builder() -> ParseOptionsBuilder {
        ParseOptionsBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ParseOptionsBuilder {
    case_sensitive: bool,
    max_depth: usize,
}

impl ParseOptionsBuilder {
    pub fn case_sensitive(mut self, value: bool) -> Self {
        self.case_sensitive = value;
        self
    }

    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn build(self) -> ParseOptions {
        ParseOptions {
            case_sensitive: self.case_sensitive,
            max_depth: self.max_depth,
        }
    }
}
```

**Why they work together**:
- `#[non_exhaustive]` prevents struct literal syntax externally
- Builder provides the **approved construction path**
- New fields get defaults in builder - no breaking changes

---

## Real-World Example: [[Brackets Extended]] Validator

From this repository's advanced examples:

```rust
// Options struct with defaults
pub struct ValidatorOptions {
    pub error_mode: ErrorMode,
    pub unclosed_policy: UnclosedPolicy,
    pub custom_pairs: Vec<(char, char)>,
}

impl Default for ValidatorOptions {
    fn default() -> Self {
        Self {
            error_mode: ErrorMode::FirstOnly,
            unclosed_policy: UnclosedPolicy::Report,
            custom_pairs: vec![
                ('(', ')'),
                ('[', ']'),
                ('{', '}'),
            ],
        }
    }
}

// Builder for fluent configuration
impl ValidatorOptions {
    pub fn builder() -> ValidatorOptionsBuilder {
        ValidatorOptionsBuilder::default()
    }
}

// Usage in tests or application code
let opts = ValidatorOptions::builder()
    .error_mode(ErrorMode::CollectAll)
    .add_pair('<', '>')
    .build();
```

---

## Common [[Traits]] for Builders

From [[common-traits-pattern]]:

| Trait | Rationale |
|-------|-----------|
| `Debug` | Inspect builder state during construction |
| `Default` | Create builder with sensible defaults |
| `Clone` | Fork builder for variations |

```rust
#[derive(Debug, Default, Clone)]
pub struct RequestBuilder {
    method: Option<String>,
    url: Option<String>,
    headers: Vec<(String, String)>,
}
```

---

## Flexible Parameter Types with AsRef

From [[asref-trait-ergonomics]]:

```rust
impl ServerConfigBuilder {
    // Accept anything that can become &str or String
    pub fn host<S: AsRef<str>>(mut self, host: S) -> Self {
        self.host = Some(host.as_ref().to_owned());
        self
    }

    // Or for zero-copy when possible
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }
}

// All of these work:
builder.host("localhost")        // &str
builder.host(String::from("x"))  // String
builder.host(&some_string)       // &String
```

---

## When to Use Builder Pattern

**Use Builder When**:
- Object has many optional fields with defaults
- Construction requires validation
- API needs to evolve without breaking changes
- Want method chaining for fluent configuration
- Some combinations of fields are invalid

**Skip Builder When**:
- Only 1-2 fields, all required
- Simple constructor suffices
- Type is internal/private

---

## Comparison: Construction Approaches

| Approach | Pros | Cons |
|----------|------|------|
| **Constructor** | Simple, obvious | Parameter order ambiguity |
| **Struct literal** | Clear field names | Breaks with new fields, no validation |
| **Basic Builder** | Fluent, defaults | Runtime validation only |
| **Type-State Builder** | Compile-time safety | More boilerplate |

---

## Ecosystem Examples

- **clap**: CLI argument parsing with builder pattern
- **reqwest**: HTTP client request building
- **tokio**: Runtime configuration
- **serde_json**: JSON value construction

```rust
// clap example
let matches = Command::new("myapp")
    .version("1.0")
    .author("Me")
    .arg(Arg::new("config")
        .short('c')
        .long("config")
        .required(true))
    .get_matches();
```

---

## Mission Connections

- **[[mission-5]]**: [[HashMap]] builder with custom hasher
- **[[mission-9]]**: Dijkstra/A* builder for algorithm configuration
- **[[mission-10]]**: REST API request/response builders

---

## Related Notes

- [[non-exhaustive-pattern]] - Struct evolution without breaking changes
- [[common-traits-pattern]] - [[Traits]] to implement on builders
- [[asref-trait-ergonomics]] - Flexible builder parameter types
- [[API Design Principles]] - Type-state builder for invalid state prevention
- [[API Design Patterns]] - clap as builder pattern example
- [[Brackets Extended]] - Real-world Options/builder configuration

---

*Created: 2026-01-17*
*Source: RfR Ch3 study, advanced_examples/Brackets_Ext context*
