# Chapter 7.1: Packages and Crates

This directory demonstrates the fundamental concepts of **packages** and **crates** in Rust.

## 📦 What's a Package?

A **package** is a bundle of one or more crates that provides a set of functionality. A package contains a `Cargo.toml` file that describes how to build those crates.

## 📚 What's a Crate?

A **crate** is a binary or library. The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.

## 🏗️ This Package Structure

This package contains **one library crate** and **two binary crates**:

```
crates/
├── Cargo.toml          # Package configuration
├── src/
│   ├── lib.rs          # Library crate (no main() function)
│   ├── binary_example.rs    # Binary crate (standalone)
│   └── library_example.rs   # Binary crate (uses the library)
└── README.md           # This file
```

## 🔍 Key Differences

### Library Crate (`src/lib.rs`)
- **No `main()` function**
- Provides functionality to other programs
- Can be used as a dependency
- Entry point for other crates to import from
- Contains public functions, structs, enums, traits, etc.

### Binary Crate (`src/binary_example.rs`)
- **Has a `main()` function**
- Can be executed directly
- Standalone program
- Doesn't depend on external libraries

### Binary Crate Using Library (`src/library_example.rs`)
- **Has a `main()` function**
- Can be executed directly
- **Uses the library crate** as a dependency
- Demonstrates how to import and use library functionality

## 🚀 How to Run

### Run the standalone binary:
```bash
cargo run --bin binary_example
```

### Run the binary that uses the library:
```bash
cargo run --bin library_example
```

### Test the library:
```bash
cargo test
```

### Build everything:
```bash
cargo build
```

## 📖 Learning Objectives

After studying this example, you should understand:

1. **Package vs Crate**: A package contains crates, a crate is a binary or library
2. **Library Crates**: Provide functionality, no main() function, can be imported
3. **Binary Crates**: Executable programs with main() function
4. **Dependencies**: How binary crates can use library crates
5. **Cargo.toml**: How to configure multiple crates in one package

## 🔗 Related Concepts

- **Modules**: How to organize code within crates (see `../modules/`)
- **Paths**: How to reference items in modules (see `../paths/`)
- **Visibility**: How to control what's public vs private (see `../visibility/`)

## 💡 Real-World Examples

- **Library Crates**: `serde`, `tokio`, `clap` - provide functionality to other programs
- **Binary Crates**: `cargo`, `rustc`, `ripgrep` - executable command-line tools
- **Mixed Packages**: Many packages contain both library and binary crates

---
*Tags: #rust-book #chapter7 #crates #packages #modules #organization #library #binary*
*Links: [[../../../zettelkasten/zettel-index|Zettelkasten Index]] | [[../../../zettelkasten/Rust Concepts MOC|Rust Concepts]] | [[../../../zettelkasten/Missions Overview|Missions Overview]] | [[../../../missions/Mission7/README|Mission7 - Modules in Practice]]*
