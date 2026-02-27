---
name: rust-book
description: Process Rust Book and Rust for Rustaceans chapters with exercises, summaries, and cross-content integration
---

# Rust Book & Rustaceans Instructions - Systematic Content Integration

**Purpose**: Create comprehensive summaries and exercises integrating The Rust Book and Rust for Rustaceans with the repository's learning systems.

**Applies To**:
- `rust_book/ChX/` - The Rust Book (official language fundamentals)
- `rust_for_rustaceans/ChX/` - Rust for Rustaceans (intermediate/advanced)

---

## Three-Phase Chapter Integration

```
Phase 1: Content Analysis       # Identify key concepts and integration points
Phase 2: Exercise Creation      # Build practical exercises demonstrating concepts
Phase 3: Summary Generation     # Create comprehensive chapter summaries with links
```

---

## Chapter Directory Structure (MANDATORY)

```
[book_dir]/ChX/
├── README.md                   # Chapter overview and learning objectives
├── [package_name]/             # Named package (e.g., foundations, ownership)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs             # Helper to list examples
│   │   └── lib.rs              # Public API (optional)
│   ├── examples/
│   │   ├── [concept1].rs       # Focused concept demonstration
│   │   ├── [concept2].rs       # Another concept
│   │   └── full_chapter.rs     # All listings together (optional)
│   └── tests/
│       └── concept_tests.rs    # Validate understanding
├── Summary.md                  # Concise summary with zettelkasten links
└── Ch0X.rs                     # Original code (if migrating)
```

---

## Chapter README Template

```markdown
# [Book Name] - Chapter X: [Chapter Title]

**Learning Objectives**: By completing this chapter, you will understand:
1. [Primary concept 1]
2. [Primary concept 2]
3. [Primary concept 3]

**Integration Points**:
- [[zettelkasten/concept-name]] - [Relationship]
- [[missions/MissionX]] - [Practical application]
```

---

## Rust for Rustaceans Specifics

Key differences from The Rust Book - dives deeper into:
- Memory models (high-level vs low-level semantics)
- Type system (variance, subtyping, trait object internals)
- Unsafe Rust (raw pointers, UnsafeCell, FFI)
- Async programming (Futures, Pin, Waker internals)
- Production patterns (error handling, API design, testing)

### Rustaceans Chapter Workflow
1. Read chapter thoroughly - take notes on advanced concepts
2. Create Summary.md - add zettelkasten links
3. Identify code listings - extract all book code examples
4. Create focused examples - one per major section/concept
5. Create full_chapter.rs - all listings in one comprehensive file
6. Add to workspace - update root Cargo.toml
7. Write README.md - learning path, quick reference
8. Cross-reference - link to missions, daily study, Rust Book chapters

---

## Quality Standards

```bash
cargo check --package rust_book_chX
cargo test --package rust_book_chX
cargo clippy --package rust_book_chX -- -D warnings
cargo doc --package rust_book_chX
```

- All chapter concepts have working examples
- Exercises test understanding, not just correctness
- Clear connection to mission requirements
- Zettelkasten links created and bidirectional
- Summary captures key insights and connections
