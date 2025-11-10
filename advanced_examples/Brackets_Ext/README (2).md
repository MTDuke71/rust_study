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
- [[Stack Data Structure]] - LIFO structure for bracket tracking
- [[V-Cycle Methodology]] - Requirements-driven development process
- [[Big-O Notation]] - Time and space complexity analysis
- [[Algorithm Design]] - Problem-solving patterns

### Rust Fundamentals
- [[Result Type]] - Error handling with Result<(), E>
- [[String Slicing]] - Working with &str
- [[pattern-matching]] - Match expressions for error handling
- [[Unit Testing]] - Test organization and naming

### Data Structures
- [[Stack Implementation]] - Stack operations (push, pop, peek)
- [[Vec Type]] - Underlying storage for stack

### Error Handling
- [[Error Types]] - BracketError and BracketErrorKind
- [[Early Return]] - Fail-fast error detection
- [[Error Reporting]] - Detailed error messages with positions

### Testing Strategies
- [[Integration Testing]] - CSV-based validation testing
- [[Test Data Organization]] - tests/data structure
- [[Traceability Matrix]] - Requirements to tests mapping
- [[Cargo Test]] - Running tests with cargo

### Mission Integration
- [[mission-1]] - Stack implementation foundation
- [[Brackets Extended]] - Advanced features building on this
- [[Missions Overview]] - V-Cycle mission overview

### AoC Patterns
- [[Bracket Validation]] - Classic AoC pattern
- [[LIFO Processing]] - Last-in-first-out algorithms
- [[String Processing]] - Character-by-character scanning

### Learning Resources
- [[Rust Book]] - The Rust Programming Language
- [[Daily Study MOC]] - Structured learning
- [[Competitive Programming]] - AoC-style problems

*Links: [[zettel-index]] | [[Brackets Extended]] | [[Missions Overview]] | [[V-Cycle Methodology]]*

*Tags: #brackets #validation #v-cycle #requirements #stack #testing #error-handling #aoc-patterns*
