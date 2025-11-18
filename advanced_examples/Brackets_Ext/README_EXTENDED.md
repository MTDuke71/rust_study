# Balanced Brackets Validator — Extended (AoC)

> **Navigation**: [[../../zettelkasten/zettel-index|zettel-index]] | [[../../zettelkasten/Collections MOC|Collections MOC]] | [[../../zettelkasten/AoC Patterns MOC|AoC Patterns MOC]] | [[../Brackets_Basic/README|Brackets Basic]]

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

## 🔗 **Related Documentation**

### **Bracket Validation Series**
- **[[../Brackets_Basic/README|Brackets Basic]]** - Basic bracket validation (REQ-1 to REQ-6)
- **[[../../zettelkasten/AoC Collection Problems|AoC Collection Problems]]** - Stack-based parsing patterns
- **[[../../Mission1/README|Mission1 Stack]]** - Stack implementation used for validation

### **Core Concepts**
- **[[../../zettelkasten/Day 01 - Setup|Day 01 - Setup]]** - Rust toolchain and cargo basics
- **[[../../zettelkasten/Day 02 - Ownership Basics|Day 02 - Ownership]]** - Ownership in iterators
- **[[../../zettelkasten/Day 05 - Option and Result|Day 05 - Option and Result]]** - Error handling patterns used
- **[[../../zettelkasten/Day 06 - Pattern Matching|Day 06 - Pattern Matching]]** - Match expressions for validation

### **Design Patterns**
- **[[../../zettelkasten/Error Handling Deep Dive|Error Handling Deep Dive]]** - Multi-error collection patterns
- **Iterator API Design** - Streaming validation over chars
- **Configuration Builder Pattern** - Options struct for flexible validation

### **Knowledge Hubs**
- **[[../../zettelkasten/Collections MOC|Collections MOC]]** - Data structures and algorithms
- **[[../../zettelkasten/AoC Patterns MOC|AoC Patterns MOC]]** - Competitive programming patterns
- **[[rust-concepts-MOC|rust-concepts-MOC]]** - Core language features

---

*Tags: #brackets #validation #stack #aoc #iterators #error-handling #configuration #advanced-examples #string-parsing*

*Links: [[../../zettelkasten/zettel-index|zettel-index]] | [[../Brackets_Basic/README|Brackets Basic]] | [[../../zettelkasten/Collections MOC|Collections MOC]] | [[../../zettelkasten/AoC Patterns MOC|AoC Patterns MOC]]*

