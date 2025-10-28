# Rust Study Workspace - AI Agent Instructions

**Context**: A professional Rust learning workspace following V-Cycle software engineering methodology with integrated zettelkasten knowledge management.

## 🏗️ Architecture Overview

This is NOT a typical Rust project - it's a **comprehensive learning system** with three integrated tracks:

### Core Components
- **`missions/MissionX/`** - V-Cycle data structure implementations (Stack, Queue, HashMap, Graph, etc.)
- **`tutorials/MissionX_tut/`** - Step-by-step learning progressions that build toward missions
- **`advanced_examples/`** - Real-world applications demonstrating mission concepts
- **`daily_study/rust_learning_weekX_notes/`** - Systematic concept progression (5 weeks completed)
- **`rust_book/ChX/`** - Rust Book chapter exercises (50+ workspace members)
- **`zettelkasten/`** - Bidirectional knowledge graph with 488+ interconnected markdown files
- **`advent_of_code/`** - AoC preparation with pattern recognition and complete 2015 solutions

### Key Architectural Decisions
- **Hybrid structure**: Keeps tutorials separate from missions to avoid root clutter while maintaining clear educational flow
- **Workspace organization**: 60+ crate members in `Cargo.toml` for independent compilation and testing
- **Knowledge overlay**: Zettelkasten provides navigation without disrupting existing code structure
- **Cross-track alignment**: Daily study, missions, and tutorials coordinate through `MONTHLY_CALENDAR.md`

## 🔄 V-Cycle Development Workflow

**CRITICAL**: All mission code follows formal V-Cycle methodology. Never implement without following this pattern:

### 1. Requirements Phase
```rust
// missions/MissionX/README.md must define numbered requirements:
// REQ-1: Generic Stack<T> with push/pop operations
// REQ-2: O(1) amortized time complexity for all operations
// REQ-3: Ownership transfer semantics
```

### 2. Test-First Implementation
```rust
// tests/unit_tests.rs - Tests MUST be named with requirement IDs
#[test]
fn req1_generic_support() { /* ... */ }

#[test]
fn req2_push_amortized_constant() { /* ... */ }
```

### 3. Traceability
- Every test name includes REQ-ID: `req1_*`, `req2_*`, etc.
- Function documentation links to requirements: `/// # Requirements Satisfied: REQ-1, REQ-3`
- README.md includes traceability matrix mapping REQ-ID → Implementation → Tests

### 4. Quality Gates
```bash
# MUST pass before committing:
cargo test --workspace                  # All tests pass
cargo clippy --workspace -- -D warnings # Zero warnings
cargo fmt --all --check                 # Formatted
```

## 🧠 Zettelkasten Knowledge Management

### Navigation Conventions
The zettelkasten uses **strict link naming** to prevent collisions:
- Daily study: `[[daily-study/Day24]]` NOT `[[Day24]]`
- Missions: `[[mission-5]]` NOT `[[Mission5]]`
- Rust Book: `[[rust_book/rust-book-ch8]]` NOT `[[Ch8]]`
- Concepts: `[[find-all-components]]` (lowercase-with-dashes)

### Creating Bidirectional Links
When creating/updating zettelkasten files:
1. Add outgoing links in new file to related concepts
2. Update related files with incoming links back to new file
3. Update `zettelkasten/zettel-index.md` if creating MOC or major concept
4. Follow existing link patterns in `*Links:` sections at file end

**Example**: See recent commit `d6c7b06` which added 25 bidirectional links for `rust-book-ch9-12-review.md`

## 📋 Common Development Tasks

### Running Specific Mission/Tutorial
```bash
# Mission implementations (production code)
cargo test -p mission5
cargo run -p mission5 --example demo
cargo bench -p mission5

# Tutorials (learning progressions)
cd tutorials/Mission5_tut
cargo run --example step1_basic_hashmap
cargo run --example step2_collision_handling
```

### Testing Markdown Examples
```powershell
# Scripts extract and run Rust code from markdown "Complete Runnable Example" sections
.\scripts\run_md.bat daily_study\rust_learning_week5_notes\Day34.md
.\scripts\run_markdown_code.ps1 missions\Mission5\README.md
```

### Quality Pipeline
```powershell
# Automated quality checks (nightly at 2-3 AM)
.\scripts\quality-pipeline.ps1          # Full pipeline
.\scripts\quality-pipeline.ps1 -Quick   # Skip coverage
# Generates reports in reports/ directory with timestamp
```

### Daily Study Workflow
```bash
# See MONTHLY_CALENDAR.md for current day's activities
# Example daily routine:
cd daily_study/rust_learning_week5_notes
cargo run --example day34_standalone
cargo test
```

## 🎯 Mission-Specific Patterns

### Mission Structure (ALL missions follow this)
```
missions/MissionX/
├── README.md              # V-Cycle requirements, traceability matrix
├── Cargo.toml            # With [package.metadata.docs.rs]
├── src/
│   ├── lib.rs            # Implementation with /// Requirements comments
│   └── main.rs           # Optional demo application
├── tests/
│   ├── unit_tests.rs     # req1_*, req2_* named tests
│   └── integration_tests.rs
├── examples/
│   └── demo.rs           # Usage demonstrations
└── benches/
    └── performance.rs    # Criterion benchmarks
```

### Tutorial Structure (7-day progressions)
```
tutorials/MissionX_tut/
├── README.md             # 7-step tutorial roadmap
├── examples/
│   ├── step1_basics.rs
│   ├── step2_intermediate.rs
│   └── step7_complete.rs
└── exercises/            # Practice problems
```

## 🔍 Finding Information

### Key Entry Points
- **`zettelkasten/zettel-index.md`** - Master navigation hub
- **`MONTHLY_CALENDAR.md`** - Daily activities and learning schedule
- **`zettelkasten/Missions Overview.md`** - Mission progress tracking
- **`.github/README.md`** - Comprehensive workspace documentation

### Common Searches
```bash
# Find specific day's work
rg "Day 34" --type md

# Find requirement definitions
rg "REQ-[0-9]:" missions/*/README.md

# Find test implementations
rg "fn req[0-9]_" missions/*/tests/

# Find zettelkasten references
rg "\[\[mission-5\]\]" zettelkasten/
```

## ⚡ Performance Considerations

### Benchmarking Pattern
```rust
// benches/performance.rs using Criterion
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_operation(c: &mut Criterion) {
    c.bench_function("operation_name", |b| {
        b.iter(|| {
            // REQ-2: Verify O(1) amortized complexity
            operation(black_box(data))
        });
    });
}
```

### Memory Analysis
```bash
# Check for memory leaks in Mission4 (LinkedList with Rc/RefCell)
cargo test -p mission4
# All Rc should be properly cleaned up - check Drop implementations
```

## 🧪 Testing Standards

### Test Naming Convention (ENFORCED)
- `req{N}_description` - Maps to specific requirement
- `test_edge_case_description` - Edge case coverage
- `test_integration_scenario` - Integration tests

### Dead Code Annotations
```rust
// For demonstration code in daily_study/ that isn't executed in tests:
#[allow(dead_code)]
enum DemoError { ... }
```

### Documentation Tests
```rust
/// # Examples
/// ```
/// use mission1::Stack;
/// let mut stack = Stack::new();
/// stack.push(42);
/// assert_eq!(stack.pop(), Some(42));
/// ```
```

## 🚨 Common Pitfalls

1. **DON'T** create code without REQ-IDs in missions
2. **DON'T** use ambiguous zettelkasten links like `[[Day24]]`
3. **DON'T** skip the traceability matrix in mission READMEs
4. **DON'T** commit without running `cargo clippy -- -D warnings`
5. **DON'T** modify existing missions without checking integration tests
6. **DO** align tutorial activities with mission work per `MONTHLY_CALENDAR.md`
7. **DO** add bidirectional links when creating zettelkasten files
8. **DO** run `.\scripts\run_md.bat` before committing markdown with code examples

## 📚 Documentation Standards

### Mission Documentation
- README.md includes V-Cycle traceability matrix
- Public APIs have rustdoc with examples
- Performance characteristics documented (Big-O notation)
- Requirements satisfied listed in function comments

### Zettelkasten Documentation
- Every concept file has `*Links:` section at bottom
- Related concepts bidirectionally linked
- Tags in format: `*Tags: #tag1 #tag2 #tag3*`
- MOC files (Maps of Content) organize related concepts

## 🔗 Integration Points

### Cross-Track Coordination
- `MONTHLY_CALENDAR.md` synchronizes all three tracks
- Tutorial steps align with mission requirements
- Daily study examples reference mission implementations
- AoC problems use mission data structures

### External Dependencies
- **Criterion** - Benchmarking (all missions with performance requirements)
- **anyhow/thiserror** - Error handling (Week 5 daily study, Mission 5+)
- **clap** - CLI parsing (Mission 9+, AoC utilities)

## 🎓 Learning Philosophy

This workspace treats Rust learning as **professional software engineering discipline**:
- Requirements before implementation (V-Cycle)
- Test-driven development (TDD)
- Formal verification and validation
- Comprehensive documentation standards
- Evidence-based learning protocols (see `MONTHLY_CALENDAR.md` header)

When working in this codebase, maintain this professional standard - every change should trace to a requirement, have tests, and integrate with the knowledge system.
