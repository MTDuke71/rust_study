# Balanced Brackets Validator – AoC V-Cycle Package

This package includes:
- **src/brackets.rs** – reference implementation.
- **tests/brackets_unit_test.rs** – unit tests (fine-grained, function-level).
- **tests/brackets_requirements_test.rs** – requirements-based tests (REQ-1…REQ-6).
- **tests/brackets_checker_test.rs** – integration checker vs AoC-style datasets.
- **tests/data/** – small & large datasets + expected CSVs.

## Run
```bash
cargo test
cargo clippy -- -D warnings
```

## V-Cycle Summary
- **Requirements:** REQ-1 scope, REQ-2 correctness, REQ-3 error reporting, REQ-4 complexity, REQ-5 API, REQ-6 determinism.
- **Design:** Stack of (expected_closer, open_index). One-pass `char_indices()`.
- **Implementation:** `validate_brackets(&str) -> Result<(), BracketError>`.
- **Verification:** unit + requirements tests, integration checker vs CSVs.
- **Validation:** AoC-style datasets (small+large).
- **Traceability:** tests named with `reqX_*` and data-driven checker.

## 📝 Documentation Standards

This project follows the workspace documentation standards:
- **Code Documentation**: [RUST_DOCUMENTATION_STANDARDS.md](../.github/RUST_DOCUMENTATION_STANDARDS.md)  
- **Test Documentation**: [RUST_TEST_DOCUMENTATION_STANDARDS.md](../.github/RUST_TEST_DOCUMENTATION_STANDARDS.md)

---

## 📚 **Additional Resources**

### **Deep Dive Documentation**
- [[Q and A|Q and A]] - Comprehensive technical analysis answering complex questions about:
  - Stack invariants and state management
  - UTF-8 handling with char_indices()
  - Algorithm complexity proofs
  - Memory safety and aliasing analysis
  - API design tradeoffs
  
- [[README_EXTENDED|README_EXTENDED]] - Extended bracket validator with advanced features:
  - REQ-7: Configurable alphabet (arbitrary opener-closer mappings)
  - REQ-8: Multiple error collection mode
  - REQ-9: Unclosed bracket policies (LatestOpen vs EarliestOpen)
  - Iterator API for streaming validation

### **Related Projects**
- [[Brackets_Ext]] - Full implementation with extended requirements
- [[mission-1]] - Stack data structure (foundational for bracket validation)

### **Learning Resources**
- [[V-Cycle Methodology]] - Requirements-driven development approach
- [[REQ-1 Test Strategy]] - Requirements-based testing patterns
- [[Stack Invariants]] - Maintaining correctness guarantees
- [[AoC Patterns MOC]] - Advent of Code problem-solving strategies

---

*Tags: #brackets-basic #aoc #v-cycle #stack #validation #requirements*

Happy hacking! 🎄