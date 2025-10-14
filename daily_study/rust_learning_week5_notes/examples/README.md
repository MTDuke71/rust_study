# Week 5 Error Handling Examples

This directory contains comprehensive examples demonstrating Rust error handling patterns and best practices.

## 🚀 Quick Start

```bash
# Navigate to the examples directory
cd daily_study/rust_learning_week5_notes

# Run individual day examples (using standard library only)
cargo run --example day29_custom_errors
cargo run --example day30_error_propagation
cargo run --example day31_anyhow_thiserror
cargo run --example day32_result_combinators
cargo run --example day33_panic_recovery
cargo run --example day34_error_patterns
cargo run --example day35_robust_parsing

# Run advanced examples (using real crates)
cargo run --example web_api_errors
cargo run --example file_processor
cargo run --example config_validator
```

## 📁 Example Files

### **Daily Learning Examples**
These examples correspond to each day of Week 5 and are designed to work with just the standard library for educational purposes:

- **`day29_custom_errors.rs`** - Custom error types and `std::error::Error` implementation
- **`day30_error_propagation.rs`** - Error propagation with the `?` operator
- **`day31_anyhow_thiserror.rs`** - Practical error handling crates (simulated)
- **`day32_result_combinators.rs`** - Functional error handling methods
- **`day33_panic_recovery.rs`** - Panic recovery and handling strategies
- **`day34_error_patterns.rs`** - Error handling decision patterns
- **`day35_robust_parsing.rs`** - Building robust parsers with error handling

### **Advanced Examples**
These examples use real crates and demonstrate production-ready error handling:

- **`web_api_errors.rs`** - Comprehensive web API error handling
- **`file_processor.rs`** - Robust file processing with error recovery
- **`config_validator.rs`** - Configuration parsing with validation

## 🛠️ Running Examples

### **Method 1: Using Cargo (Recommended)**
```bash
# Run a specific example
cargo run --example day29_custom_errors

# Run with output
cargo run --example web_api_errors

# Run with features (if applicable)
cargo run --features async --example async_example
```

### **Method 2: Direct Compilation**
```bash
# Compile and run individual files
rustc day29_custom_errors.rs && ./day29_custom_errors
```

### **Method 3: Interactive Learning**
```bash
# Run examples interactively to see error handling in action
cargo run --example day33_panic_recovery
# This will demonstrate panic recovery patterns

cargo run --example day34_error_patterns
# This will show different error handling strategies
```

## 📚 Learning Path

### **Beginner Path**
1. Start with `day29_custom_errors.rs` to understand basic error types
2. Move to `day30_error_propagation.rs` for error forwarding
3. Try `day31_anyhow_thiserror.rs` for practical error handling
4. Experiment with `day32_result_combinators.rs` for functional patterns

### **Intermediate Path**
1. Study `day33_panic_recovery.rs` for panic handling
2. Analyze `day34_error_patterns.rs` for decision guidelines
3. Build on `day35_robust_parsing.rs` for real-world applications

### **Advanced Path**
1. Implement `web_api_errors.rs` patterns in your own APIs
2. Use `file_processor.rs` techniques for data processing
3. Apply `config_validator.rs` patterns for configuration management

## 🔧 Dependencies

The examples use the following crates (defined in `Cargo.toml`):

```toml
[dependencies]
anyhow = "1.0"           # Application-level error handling
thiserror = "1.0"        # Library-level error types
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"       # JSON parsing
```

## 🎯 Key Learning Outcomes

After running these examples, you should understand:

1. **Custom Error Types**: How to create domain-specific error types
2. **Error Propagation**: When and how to use the `?` operator
3. **Error Handling Crates**: When to use `anyhow` vs `thiserror`
4. **Result Combinators**: Functional error handling patterns
5. **Panic Recovery**: How to handle panics safely
6. **Error Patterns**: Decision guidelines for error handling
7. **Robust Parsing**: Building parsers with comprehensive error handling

## 🐛 Common Issues

### **Compilation Errors**
If you get compilation errors, make sure you have the latest Rust version:
```bash
rustup update
```

### **Missing Dependencies**
If examples fail to run, ensure all dependencies are installed:
```bash
cargo build
```

### **Permission Errors**
Some examples create temporary files. Make sure you have write permissions in the current directory.

## 📖 Further Reading

- **Day 29-35 Markdown Files**: Detailed explanations and patterns
- **Week 5 README**: Complete learning guide
- **Rust Book Chapter 9**: Error Handling
- **anyhow Documentation**: https://docs.rs/anyhow/
- **thiserror Documentation**: https://docs.rs/thiserror/

## 🤝 Contributing

Feel free to:
- Add new examples demonstrating different error handling patterns
- Improve existing examples with better error messages
- Add tests for error conditions
- Suggest new practical applications

## 📝 Notes

- Examples are designed to be educational and demonstrate concepts clearly
- Production code may need additional error handling and logging
- Some examples use simulated services for demonstration purposes
- All examples include comprehensive error handling patterns

---

*Tags: #week5 #error-handling #examples #anyhow #thiserror #result-combinators #panic-recovery*

*Links: [[../README]] | [[../../README]] | [[../../Day29]] | [[../../Day30]] | [[../../Day31]] | [[../../Day32]] | [[../../Day33]] | [[../../Day34]] | [[../../Day35]] | [[Error Handling Deep Dive]] | [[Week 5 Overview]] | [[zettel-index]]*
