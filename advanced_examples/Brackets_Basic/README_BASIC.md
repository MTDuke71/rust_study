# Balanced Brackets Validator (V-Cycle)

## Requirements
- **REQ-1**: Recognize `()[]{}` only, ignore all other chars.
- **REQ-2**: A string is valid iff brackets are properly matched and nested.
- **REQ-3**: On failure, return earliest error (UnexpectedClosing, MismatchedPair, UnclosedOpenings).
- **REQ-4**: Complexity O(n) time, O(n) space.
- **REQ-5**: API: `fn validate_brackets(&str) -> Result<(), BracketError>`
- **REQ-6**: Deterministic across platforms.

## Design
- Use a `Stack<(expected_closer, open_index)>` to track "promises."
- Push expected closer when encountering an opener.
- On closer: pop and check.
- At end: if leftover, return UnclosedOpenings.

## Implementation
See `src/brackets.rs`.

## Verification
- Unit tests in `tests/brackets_test.rs`
- Integration checker test (`tests/brackets_checker_test.rs`) compares actual output with expected CSVs.

## Validation
- Small and large datasets in `tests/data`.
- Run `cargo test` to confirm.

## Traceability Matrix
| REQ | Design | Test |
|-----|--------|------|
| REQ-1 | Ignore non-brackets | `valid_with_noise` |
| REQ-2 | Stack of expected closers | `valid_simple_pairs`, `nested_deep` |
| REQ-3 | BracketError | `unexpected_closing_reports_index`, `mismatched_pair_reports_expected_and_found`, `unclosed_openings_reports_first_unclosed` |
| REQ-4 | Linear scan | `long_valid_smoke` |
| REQ-5 | API fn | All compile |
| REQ-6 | Pure function, deterministic | All tests |

---
To run:

```bash
cargo test
cargo clippy -- -D warnings
```

---

## 📚 Related Zettelkasten Concepts

### Core Concepts
- [[Stack Data Structure]] - LIFO structure for bracket matching
- [[V-Cycle Methodology]] - Requirements-driven development (REQ-1 to REQ-6)
- [[Algorithm Design]] - Stack-based validation algorithm
- [[Big-O Notation]] - Time and space complexity analysis

### Rust Fundamentals
- [[Result Type]] - Error handling with Result<(), BracketError>
- [[String Processing]] - Working with &str and char_indices
- [[Pattern Matching]] - Match expressions for validation logic
- [[UTF-8 Handling]] - Unicode string processing

### Data Structures
- [[Stack Implementation]] - Push, pop, peek operations
- [[Vec]] - Underlying storage for stack
- [[Tuples]] - (expected_closer, open_index) pairs

### Error Handling
- [[Error Types]] - BracketError and BracketErrorKind
- [[Early Return]] - Fail-fast error detection
- [[Error Reporting]] - Position-based error messages
- [[UnexpectedClosing]] - Error variant patterns
- [[MismatchedPair]] - Type mismatch detection
- [[UnclosedOpenings]] - EOF error handling

### Testing Strategies
- [[Integration Testing]] - CSV-based dataset validation
- [[Unit Testing]] - Individual requirement tests
- [[Test Data Organization]] - tests/data structure
- [[Traceability Matrix]] - Requirements to tests mapping

### Requirements Engineering
- [[REQ-1]] - Bracket recognition and character filtering
- [[REQ-2]] - Proper matching and nesting rules
- [[REQ-3]] - Earliest error detection
- [[REQ-4]] - Complexity guarantees (O(n) time, O(n) space)
- [[REQ-5]] - API contract definition
- [[REQ-6]] - Deterministic behavior

### Mission Integration
- [[Mission1]] - Stack implementation foundation
- [[Brackets Extended]] - Advanced features extension
- [[Missions MOC]] - V-Cycle mission overview

### AoC Patterns
- [[Bracket Validation]] - Classic competitive programming pattern
- [[LIFO Processing]] - Last-in-first-out algorithms
- [[Character Scanning]] - Single-pass string processing
- [[Stack-Based Parsing]] - Delimiter matching

### Learning Resources
- [[Rust Book]] - Language fundamentals
- [[Daily Study MOC]] - Structured learning
- [[Competitive Programming]] - AoC-style problems
- [[RUST_TEST_DOCUMENTATION_STANDARDS]] - Testing guide

*Links: [[zettel-index]] | [[Brackets Extended]] | [[Mission1]] | [[V-Cycle Methodology]] | [[Stack Data Structure]]*

*Tags: #brackets-basic #v-cycle #stack #validation #requirements #testing #error-handling #aoc-patterns #req1-6*
