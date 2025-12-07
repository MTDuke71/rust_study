# Balanced Brackets Validator — Extended (AoC)

## Added Requirements
- **REQ-7 (Configurable alphabet):** arbitrary opener→closer mapping (e.g., add `< >`).
- **REQ-8 (Report all errors):** optional mode to collect all errors.
- **REQ-9 (Unclosed policy):** choose **LatestOpen** or **EarliestOpen** at EOF.
- **Iterator API:** streaming validation over `Iterator<Item=char>` or `(index, char)`.

## APIs
- `validate_brackets(&str) -> Result<(), BracketError>` — original convenience.
- `validate_with_options(&str, &Options) -> Result<(), Vec<BracketError>>`
- `validate_iter<I: IntoIterator<Item=char>>(iter, &Options) -> Result<(), Vec<BracketError>>`
- `validate_indexed<I: IntoIterator<Item=(usize,char)>>(iter, &Options) -> Result<(), Vec<BracketError>>`

## Options
```rust
#[derive(Default)]
pub struct Options { pub alphabet: Alphabet, pub error_mode: ErrorMode, pub unclosed_policy: UnclosedPolicy }
enum ErrorMode { StopAtFirst, CollectAll }
enum UnclosedPolicy { LatestOpen, EarliestOpen }
```

## Quick Start
1. Put `src/brackets.rs` in your crate; expose in `src/lib.rs`:
   ```rust
   pub mod stack;
   pub mod brackets;
   ```
2. Place datasets in `tests/data/` (from the package).
3. Run tests:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   ```

## Traceability
| REQ | Design/Code | Tests |
|-----|-------------|-------|
| REQ-7 | `Alphabet`, `Options.alphabet` | `req7_configurable_alphabet_with_angles` |
| REQ-8 | `ErrorMode::CollectAll` path | `req8_collect_all_errors`, checker integration |
| REQ-9 | `UnclosedPolicy` handling at EOF | `req9_unclosed_policy_latest_vs_earliest` |
| Iterator | `validate_iter`, `validate_indexed`, core engine | iterator API tests |

---

## 📚 Related Zettelkasten Concepts

### Core Concepts
- [[Configurable Alphabet]] - Arbitrary opener-closer mapping (REQ-7)
- [[Error Collection Mode]] - StopAtFirst vs CollectAll (REQ-8)
- [[Unclosed Policy]] - LatestOpen vs EarliestOpen (REQ-9)
- [[Iterator API Design]] - Streaming validation patterns

### Data Structures
- [[mission-1|Stack Data Structure]] - LIFO bracket tracking
- [[HashMap]] - Alphabet configuration storage
- [[Vec Type]] - Multiple error collection
- [[Options Pattern]] - Configuration struct

### Advanced Patterns
- [[Builder Pattern]] - Options configuration with defaults
- [[Strategy Pattern]] - Pluggable ErrorMode and UnclosedPolicy
- [[Iterator Patterns]] - IntoIterator trait usage
- [[Generic Programming]] - Flexible input types

### Error Handling
- [[Multiple Error Collection]] - Vec<BracketError> result
- [[Error Context]] - Detailed error information
- [[BracketError Type]] - Structured error reporting
- [[Result Composition]] - Result<(), Vec<BracketError>>

### API Design
- [[Progressive Enhancement]] - Basic to extended APIs
- [[Backward Compatibility]] - Preserving validate_brackets
- [[Smart Defaults]] - Options::default() convenience
- [[Flexible Interfaces]] - validate_iter, validate_indexed

### Requirements Engineering
- [[Iterator API Requirements]] - Streaming support
- [[V-Cycle Methodology]] - Requirements-driven development

### Testing Strategies
- [[Requirements-Based Testing]] - REQ-7, REQ-8, REQ-9 tests
- [[Integration Testing]] - CSV dataset validation
- [[Iterator Testing]] - Stream validation tests
- [[Traceability Matrix]] - Requirements to tests mapping

### Mission Integration
- [[Brackets Basic]] - Foundation implementation
- [[mission-1]] - Stack implementation
- [[Brackets Extended]] - Full implementation
- [[Missions Overview]] - V-Cycle projects

### AoC Applications
- [[Custom Delimiters]] - HTML/XML tag validation
- [[IDE Integration]] - Linting and error reporting
- [[Educational Use Cases]] - Step-by-step error feedback
- [[Mathematical Notation]] - Custom bracket types

### Learning Resources
- [[rust-book]] - Language fundamentals
- [[Daily Study MOC]] - Structured learning
- [[AoC Patterns]] - Problem-solving strategies
- [[RUST_DOCUMENTATION_STANDARDS]] - Documentation guide

*Links: [[zettel-index]] | [[Brackets Basic]] | [[Brackets Extended]] | [[V-Cycle Methodology]] | [[Configurable Alphabet]]*

*Tags: #brackets-extended #req7 #req8 #req9 #iterator-api #configuration #error-collection #aoc-patterns #extensibility*

