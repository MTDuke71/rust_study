# AI Agent Instructions for Rust Study Codebase

This is a **V-Cycle learning workspace** for systematic Rust development using formal software engineering practices. Understanding the V-Cycle methodology is critical for productive work in this codebase.

## 🎯 Core Development Philosophy: V-Cycle Methodology

This codebase follows **requirements-driven development** with complete traceability from requirements → design → implementation → verification → validation.

### V-Cycle Structure (Used in Every Mission)
```
Requirements (REQ-1, REQ-2, etc.)
    ↓
Design Specification  
    ↓
Implementation
    ↓
Verification (Unit Tests)
    ↓
Validation (Integration Tests)
    ↓
Traceability Matrix
```

**Key Pattern**: Every feature starts with numbered requirements (REQ-G1, REQ-R1, REQ-L1) that are directly traceable to tests and implementation.

## 📂 Workspace Architecture

### Progressive Learning Structure
- **Ch1-Ch3/**: Basic Rust book exercises (hello_cargo, guessing_game, variables)
- **Mission1/**: Stack implementation with full V-Cycle (first major data structure)
- **Mission2/**: Queue implementations (RingBufferQueue + LinkedQueue) with performance focus
- **Brackets_Basic/Brackets_Ext/**: Balanced brackets validator with AoC-style validation
- **aoc_scaffold_templates_with_tests/**: Advent of Code template with testing framework

### Cargo Workspace Pattern
```toml
[workspace]
members = ["Mission1", "Mission2", "Brackets_Basic", "Brackets_Ext", ...]
```
Each mission is a separate crate with its own `Cargo.toml`, allowing independent development and testing.

## 🧪 Testing Methodology

### Requirement-Based Test Naming
Tests are explicitly named to trace back to requirements:
```rust
#[test] // REQ-1
fn req1_generic_support() { ... }

#[test] // REQ-2  
fn req2_push_amortized_constant() { ... }

#[test] // REQ-G2, REQ-R1
fn ring_basic_wrap_and_full() { ... }
```

### Test Categories
1. **Unit Tests**: Function-level testing in `tests/` directory
2. **Requirements Tests**: Named `req{X}_*` to verify specific requirements
3. **Integration Tests**: Real-world scenarios (e.g., BFS simulation, performance comparison)
4. **Property Tests**: Using randomized testing against reference implementations (VecDeque)

### Data-Driven Testing Pattern
```rust
// AoC-style validation with CSV expected results
tests/data/brackets_small.txt
tests/data/brackets_small.expected.csv
tests/brackets_checker_test.rs  // Integration test
```

## 📝 Documentation Standards

### Module Documentation (`//!`)
Every `lib.rs` includes:
- Requirements fulfilled section
- Quick start examples
- Performance characteristics
- Use case guidance

### Function Documentation (`///`)
- Requirements satisfied (e.g., `/// # Requirements Satisfied: REQ-G1, REQ-R2`)
- Complexity guarantees (O(1), amortized O(1))
- Ownership behavior
- Example usage with doctests

### Comment Patterns
```rust
// REQ-G1: FIFO queue API contract
// REQ-R3: Fixed capacity with Vec<Option<T>>
```

## 🚀 Common Development Workflows

### Standard Build/Test Commands
```powershell
cargo test                    # Run all tests
cargo clippy -- -D warnings  # Enforce design contracts
cargo test req1               # Run specific requirement tests
cargo run --example demo     # Mission demos
```

### Mission Development Pattern
1. **Requirements Phase**: Write REQ-X statements in README.md
2. **Design Phase**: API contracts, data structures, invariants
3. **Implementation**: Focus on one requirement at a time
4. **Verification**: Unit tests for each REQ-X
5. **Validation**: Integration tests, performance benchmarks
6. **Documentation**: V-Cycle summary in README.md

### Demo Applications
Each mission includes a comprehensive demo:
- `src/main.rs`: Real-world usage examples
- Performance comparisons with std library
- Specific algorithm simulations (BFS, message buffers)

## 🏗️ Architecture Patterns

### Generic Data Structures
Focus on `<T>` generic implementations suitable for competitive programming:
```rust
pub struct Stack<T> { items: Vec<T> }
pub struct RingBufferQueue<T> { data: Vec<Option<T>>, ... }
```

### Ownership Patterns
- **Move semantics**: `enqueue(value: T)`, `pop() -> Option<T>`
- **Borrowing**: `peek() -> Option<&T>`, `peek_mut() -> Option<&mut T>`
- **Error handling**: `Result<(), T>` for capacity limits

### Performance-First Design
- Ring buffers for cache-friendly access patterns
- Linked structures for dynamic growth
- O(1) operation guarantees with amortized analysis

## 🎄 AoC Integration Points

### Problem-Solving Focus
Data structures designed for competitive programming:
- **BFS/DFS**: Queue/Stack with grid traversal
- **Parsing**: Bracket validation with error reporting
- **Buffer management**: Ring buffers for streaming data

### Testing with Real Datasets
```rust
tests/data/*.txt          // Input datasets
tests/data/*.expected.csv // Expected outputs
tests/*_checker_test.rs   // Black-box validation
```

## 🔧 Extension Integration

### VS Code Setup Mentioned in Docs
- `rust-analyzer` (essential)
- `CodeLLDB` (debugging)
- Coverage tools with `cargo-tarpaulin`
- Error visualization with inline diagnostics

### Performance Tooling
```rust
// Criterion benchmarking integration
// Memory allocation tracking
// Comparison with std library implementations
```

## ⚡ Quick Start for New Features

1. **Identify the mission context** (Mission1=Stack, Mission2=Queue, etc.)
2. **Review existing REQ-X patterns** in the README.md
3. **Follow test-first development**: Write `req{N}_*` tests before implementation
4. **Validate against reference implementations** (Vec, VecDeque, etc.)
5. **Add integration examples** showing real-world usage
6. **Update V-Cycle documentation** with traceability

## 💡 When Working on This Codebase

- **Always trace features back to requirements** - if there's no REQ-X, create one
- **Test naming is semantic** - `req2_push_amortized_constant` tells you exactly what it verifies
- **Performance matters** - include Big-O analysis and benchmarks
- **Documentation is part of the design** - README.md includes full V-Cycle summary
- **Integration over isolation** - demos show how data structures solve real problems

This codebase treats learning Rust as an engineering discipline, not just syntax practice. Every implementation follows professional software development standards with complete traceability and validation.