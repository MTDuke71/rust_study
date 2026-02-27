---
name: mission
description: Implement production-quality Rust data structures following V-Cycle methodology with REQ-ID traceability
---

# Mission Instructions - V-Cycle Engineering Methodology

**Purpose**: Implement production-quality Rust data structures following formal V-Cycle software engineering methodology with complete requirements traceability.

---

## Mission Philosophy

Missions are **professional software engineering projects** where:
- Requirements drive implementation through formal V-Cycle methodology
- Every feature maps to a specific REQ-ID with traceability
- Test-driven development ensures correctness and completeness
- Performance requirements have measurable validation

**Core Principle**: "Every line of code traces to a requirement, every requirement has tests, every test validates a measurable outcome."

---

## V-Cycle Methodology (MANDATORY)

### Phase 1: Requirements Analysis
Every mission MUST start with numbered requirements in `README.md`:

```markdown
### **REQ-X: [Descriptive Title]**
- **Description**: Clear, unambiguous statement
- **Acceptance Criteria**: Specific, testable conditions
- **Test Strategy**: How requirement will be validated
- **Performance**: Measurable performance characteristics
```

### Phase 2: Test-First Design
Before writing implementation, create comprehensive test suite:

```rust
#[test]
fn req1_generic_support_integers() { /* ... */ }
#[test]
fn req2_memory_safety_no_leaks() { /* ... */ }
#[test]
fn req3_push_pop_performance() { /* ... */ }
```

### Phase 3: Implementation with Traceability
Every function must document which REQ-IDs it satisfies:

```rust
/// # Requirements Satisfied
/// - REQ-1: Generic type support through `T` parameter
/// - REQ-2: Memory safety through Vec<T> wrapper and RAII
pub struct Stack<T> { items: Vec<T> }
```

### Phase 4: Verification and Validation
```bash
cargo test --package missionX                    # All REQ tests pass
cargo clippy --package missionX -- -D warnings  # Zero warnings
cargo bench --package missionX                  # Performance validation
cargo doc --package missionX                    # Documentation builds
```

---

## Mission Directory Structure (MANDATORY)

```
missions/MissionX/
├── README.md                    # V-Cycle requirements and traceability
├── Cargo.toml
├── src/
│   ├── lib.rs                  # Main implementation
│   ├── main.rs                 # Optional demo
│   └── error.rs                # Error types (if needed)
├── tests/
│   ├── unit_tests.rs           # REQ-mapped unit tests
│   ├── integration_tests.rs    # Cross-component tests
│   └── property_tests.rs       # Property-based tests (advanced)
├── examples/
│   ├── demo.rs                 # Basic usage
│   └── advanced_usage.rs       # Complex scenarios
├── benches/
│   └── performance.rs          # Criterion benchmarks
└── docs/
    ├── DESIGN.md               # Design decisions
    └── PERFORMANCE.md           # Performance analysis
```

---

## Test Naming Convention (ENFORCED)

```rust
// ✅ REQUIRED: Tests MUST map to requirement IDs
#[test]
fn req1_generic_support_basic() { }
#[test]
fn req2_memory_safety_drop_cleanup() { }

// ❌ FORBIDDEN: Vague or unmapped test names
#[test]
fn test_stack() { }    // NO
#[test]
fn it_works() { }      // NO
```

---

## Traceability Matrix (REQUIRED in README.md)

```markdown
| REQ-ID | Requirement | Implementation | Tests | Verification |
|--------|-------------|----------------|--------|--------------|
| REQ-1 | Generic Support | `Stack<T>` struct | `req1_*` | ✅ Passes |
| REQ-2 | Memory Safety | RAII via Vec | `req2_*` | ✅ Passes |
```

---

## Mission Success Criteria

### Technical Quality Gates
- [ ] All requirements have acceptance criteria and test strategies
- [ ] Every test maps to a specific REQ-ID
- [ ] All clippy warnings resolved (zero tolerance)
- [ ] All benchmarks meet performance requirements
- [ ] Complete rustdoc documentation for public API

### Process Quality Gates
- [ ] Requirements traceability matrix complete
- [ ] V-Cycle phases followed in sequence
- [ ] Test-first development demonstrated
- [ ] Cross-content integration documented (zettelkasten, tutorials)
