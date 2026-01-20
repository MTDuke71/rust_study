# Rust for Rustaceans - Chapter 4: Error Handling

This directory contains comprehensive examples for Chapter 4 on Error Handling.

## 📚 Chapter Summary

See [`rfr-ch04-summary.md`](rfr-ch04-summary.md) for detailed notes on error handling concepts.

## 🎯 Weekly Plan Integration

Based on **Week 4 Plan (2026-W04)**:
- **Monday (1/19)**: Enumeration errors (domain-specific)
- **Tuesday (1/20)**: Opaque errors (type erasure)
- **Wednesday (1/21)**: Special error cases (unit errors, never type)
- **Thursday (1/22)**: Error propagation (`?` operator, `From` trait)
- **Friday (1/23)**: Summary + Review

## 🏃 Running Examples

### Main Book Example
```bash
cargo run --example ch04_book
```

### Daily Examples (Week 4)

#### Monday - Enumeration Errors
```bash
cargo run --example enumeration_errors
```
**Topics**: Domain-specific error types, `Error` trait, `Display`, `Debug`, error chains

#### Tuesday - Opaque Errors  
```bash
cargo run --example opaque_errors
```
**Topics**: `Box<dyn Error>`, type erasure, error downcasting, composing multiple error types

#### Wednesday - Special Cases
```bash
cargo run --example special_cases
```
**Topics**: `Result<T, ()>` vs `Option<T>`, thread panics, `Box<dyn Any>`, never type concept

#### Thursday - Error Propagation
```bash
cargo run --example error_propagation
```
**Topics**: `?` operator, `From` trait, error conversion, error chains, context preservation

#### Friday - Custom Errors
```bash
cargo run --example custom_errors
```
**Topics**: Comprehensive error types, builder pattern, validation errors, position tracking

### Supplemental Examples

#### Using thiserror and anyhow
```bash
cargo run --example thiserror_anyhow
```
**Topics**: `thiserror` derive macros, `anyhow::Context`, library vs application errors

#### Comprehensive Review
```bash
cargo run --example review
```
**Topics**: All concepts combined in a realistic file processing pipeline

## 📝 Code Examples Overview

| Example | Description | Key Concepts |
|---------|-------------|--------------|
| **ch04_book** | Main chapter examples | All core error handling patterns |
| **enumeration_errors** | Domain-specific error enums | `Error` trait, `source()`, error chains |
| **opaque_errors** | Type-erased errors | `Box<dyn Error>`, downcasting, flexibility |
| **special_cases** | Unit errors & special patterns | `Result<T, ()>`, thread panics, never type |
| **error_propagation** | `?` operator & error chains | `From` trait, context, propagation |
| **custom_errors** | Production error types | Builder pattern, position info, validation |
| **thiserror_anyhow** | Error handling crates | `thiserror`, `anyhow`, best practices |
| **review** | Complete pipeline example | All concepts integrated |

## 🔑 Key Concepts Demonstrated

### 1. Error Representation Patterns
- **Enumeration** (`enum`): When callers need to distinguish error cases
- **Opaque** (`Box<dyn Error>`): When specific type doesn't matter
- **Special**: Unit errors `()`, never type `!`, thread results

### 2. Error Trait Implementation
- Implementing `std::error::Error`
- Providing `source()` for error chains
- `Display` for user-facing messages
- `Debug` for developer diagnostics
- Ensuring `Send + Sync + 'static` for thread safety

### 3. Error Propagation
- `?` operator for concise propagation
- `From` trait for automatic conversion
- `map_err` for error transformation
- Context preservation through error chains

### 4. Production Patterns
- **thiserror**: Derive macros for library errors
- **anyhow**: Context-rich application errors
- Error downcasting for specific handling
- Builder pattern for complex errors

## 🎨 Design Principles

### When to Use Each Pattern

| Pattern | Use When | Example Use Case |
|---------|----------|------------------|
| **Enumeration** | Callers need exhaustive matching | Library APIs, domain logic |
| **Opaque** | Error type details don't matter | Application code, composition |
| **Unit `()`** | Failure without details | Simple validation |
| **Never `!`** | Operation is infallible | Type system guarantees |

### Error Type Checklist
- ✅ Implements `Error` trait
- ✅ Provides `Display` (one-line, lowercase, no trailing punctuation)
- ✅ Provides `Debug` for detailed output
- ✅ Implements `source()` for error chains
- ✅ Is `Send + Sync + 'static` for thread safety
- ✅ Derives or implements `From` for conversions

## 📦 Dependencies

```toml
[dependencies]
thiserror = "2.0"  # Derive macros for error types
anyhow = "1.0"     # Context-rich error handling
```

## 🔗 Related Content

- **Rust Book Ch9**: Basic error handling with `Result` and `panic!`
- **Mission 5**: HashMap/HashSet with error handling patterns
- **AoC 2023**: Real-world error handling in puzzle solutions

## 📊 Example Output

Running `cargo run --example review` demonstrates a complete file processing pipeline with:
1. File I/O errors (enumeration)
2. Parse errors with line/column information
3. Validation errors
4. Error chain traversal
5. Opaque error comparison

## 🚀 Next Steps

1. Run each example in sequence (Monday → Friday)
2. Examine the code to understand implementation details
3. Modify examples to experiment with different error types
4. Apply patterns to AoC problems (Day 19: workflow processing)
5. Create zettelkasten notes linking error patterns to practical use

## 📚 Further Reading

- [Rust Error Handling - The Book](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [thiserror documentation](https://docs.rs/thiserror)
- [anyhow documentation](https://docs.rs/anyhow)
- [Rust API Guidelines: Error Handling](https://rust-lang.github.io/api-guidelines/interoperability.html#error-handling-c)

---

*Week 4 (2026-W04) - Rust for Rustaceans Chapter 4*
