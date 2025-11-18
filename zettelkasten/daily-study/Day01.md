# Day 1 - Setup & Toolchain

**Quick Reference Note**
*For full content, see: [[daily_study/rust_learning_week1_notes/Day01]]*

---

## Core Concepts

### Rust Toolchain Components
- **rustup**: Toolchain installer and version manager
- **cargo**: Build system and package manager
- **rustc**: Rust compiler
- **rust-analyzer**: IDE language server

### Essential Cargo Commands
```bash
cargo new project_name    # Create new project
cargo build              # Compile project
cargo run                # Build and run
cargo test               # Run tests
cargo clippy             # Lint code
cargo fmt                # Format code
```

### Project Structure
```
my_project/
├── Cargo.toml           # Package manifest
├── src/
│   ├── main.rs         # Binary entry point
│   └── lib.rs          # Library root
└── tests/              # Integration tests
```

---

## Quick Navigation

- **Full Details**: [[daily_study/rust_learning_week1_notes/Day01]]
- **Next**: [[daily-study/Day02]]
- **Week**: [[Week 1 Overview]]
- **MOC**: [[rust-concepts-MOC]]

---

*Tags: #toolchain #setup #cargo #rustup #quick-ref*
