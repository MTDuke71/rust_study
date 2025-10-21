# AI Agent Instructions for Rust Study Codebase

## ⚡ Quick Start - Essential Knowledge

**This is a V-Cycle learning workspace with 3 parallel tracks**. New agents should understand these critical patterns immediately:

### Architecture Overview
- **Workspace Structure**: `missions/` (core implementations) + `tutorials/` (step-by-step learning) + `advanced_examples/` (real-world apps) + `daily_study/` (concepts) + `rust_book/` (fundamentals) + `zettelkasten/` (knowledge management)
- **Cargo Workspace**: 40+ independent crates, run with `cargo run -p <package-name> --example <example-name>`
- **Requirements Traceability**: Every feature links to REQ-X statements, tests named `req{N}_*` verify specific requirements

### Critical Workflows
```powershell
# Test mission implementations
cargo test -p mission5                              # Run all tests for Mission 5
cargo test -p mission5 req2                         # Test specific requirement
cargo clippy -p mission5 -- -D warnings            # Zero-warnings enforcement

# Run learning examples
cargo run -p mission5_tut --example step3_hashmap  # Tutorial progression
.\scripts\run_md.bat daily_study\rust_learning_week2_notes\Day10.md  # Execute markdown examples

# Validate documentation
cargo test --doc -p mission5                       # Run all doctests
cargo doc -p mission5 --open                       # Generate and view docs
```

### Project-Specific Conventions
1. **REQ-X Traceability**: All code/tests reference requirement IDs (REQ-1, REQ-2, etc.) from mission README.md
2. **Complete Runnable Examples**: Every `daily_study/DayXX.md` must include self-contained executable code (use template in `.github/COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md`)
3. **Documentation Standards**: Follow `.github/RUST_DOCUMENTATION_STANDARDS.md` (use `///` for items, `//!` for modules, always include Examples section)
4. **Tutorial-Mission Alignment**: `MissionX_tut/` steps must map to main mission REQ-X and align with `MONTHLY_CALENDAR.md` daily focus
5. **Zettelkasten Links**: All zettelkasten files use standardized link format: `[[mission-X]]`, `[[daily-study/DayXX]]`, `[[aoc-YYYY-dayDD]]`

### Non-Obvious Integration Points
- **MONTHLY_CALENDAR.md**: Master coordination file - check before creating Mission + Tutorial pairs
- **Tutorial Synchronization**: Each `stepN_*.rs` in tutorials must advance toward specific main mission requirements
- **Multi-Value Returns**: Tests often use `.unwrap()` freely since they're allowed to panic
- **Performance Baselines**: Compare implementations against std library (`Vec`, `VecDeque`, `HashMap`) in demos
- **AoC Patterns**: Real-world validation uses Advent of Code problems (`tests/data/*.txt` + `*.expected.csv`)

### Quality Gates (Must Pass)
```bash
cargo clippy -- -D warnings        # Zero warnings required
cargo test                          # All tests pass
cargo test --doc                    # All doctests pass
cargo build --workspace            # All crates compile
```

---

## 🧭 Quick Decision Guide

### When to Use What

**Data Structure Choice**:
- Need FIFO? → `VecDeque` (Mission2)
- Need lookup? → `HashMap` (Mission5)
- Need order? → `BTreeMap` (Day12)
- Need uniqueness? → `HashSet` (Day11)
- Need LIFO? → `Vec` as stack (Mission1)
- Need graph? → `Vec<Vec<NodeId>>` adjacency list (Mission7)

**Documentation Style**:
- Public API function → `///` with Examples section (see RUST_DOCUMENTATION_STANDARDS.md)
- Module/crate level → `//!` with Quick Start section
- Test function → Descriptive name only (no `///` needed)
- Implementation detail → `//` inline comment
- Requirement reference → `// REQ-X: description`

**Test Organization**:
- Unit test → `tests/` directory, named `req{N}_*` for traceability
- Integration → `tests/` directory, `*_integration.rs` or `*_checker_test.rs`
- Doctest → `/// # Examples` in function docs
- Tutorial → `examples/step{N}_*.rs` with progressive complexity

**File Location Rules**:
- Mission implementation → `missions/Mission{N}/src/lib.rs`
- Mission tests → `missions/Mission{N}/tests/`
- Tutorial steps → `tutorials/Mission{N}_tut/examples/step{N}_*.rs`
- Daily concepts → `daily_study/rust_learning_week{N}_notes/Day{N}.md`
- Rust book practice → `rust_book/Ch{N}/{topic}/`
- Real-world apps → `advanced_examples/{name}/`
- Knowledge notes → `zettelkasten/{topic}.md`
- Scripts/tools → `scripts/{tool_name}.ps1` or `.bat`

---

## ✅ Pre-Completion Verification Checklist

Before marking work complete or creating a pull request, verify:

### Code Quality
- [ ] `cargo test -p {package}` passes (all unit tests)
- [ ] `cargo clippy -p {package} -- -D warnings` clean (zero warnings mandatory)
- [ ] `cargo test --doc -p {package}` passes (all doctests execute)
- [ ] `cargo build --workspace` succeeds (no breaking changes)

### Documentation
- [ ] README.md updated with REQ-X statements if new feature
- [ ] All public functions have `///` documentation with Examples
- [ ] Module-level `//!` docs updated if API changed
- [ ] Doctests include error cases, not just happy path

### Testing
- [ ] Tests named `req{N}_*` for requirement traceability
- [ ] Edge cases covered (empty input, boundary conditions)
- [ ] Performance characteristics verified (O(1), O(n) claims)
- [ ] Integration tests pass with real-world data

### Project-Specific
- [ ] `MONTHLY_CALENDAR.md` checked if creating tutorial content
- [ ] Tutorial steps map to mission requirements (REQ-1 → step1_*, etc.)
- [ ] Daily study files include "Complete Runnable Example" section
- [ ] Zettelkasten links follow naming convention (`[[mission-X]]` not `[[MissionX]]`)
- [ ] All struct field renames reflected in ALL instantiations
- [ ] Unused fields prefixed with `_`, unused variables prefixed with `_`

### Common Gotchas to Check
- [ ] Type annotations added for generic `::new()` methods (e.g., `HashSet<T>::new()`)
- [ ] Grid dimension calculations verified (format placeholders = arguments)
- [ ] Imports cleaned up (remove unused imports caught by clippy)
- [ ] Variables are `mut` only if actually modified
- [ ] AoC test data files paired with `.expected.csv` results

---

## ⚠️ Common Error Patterns & Quick Fixes

### Struct Field Renaming Issues
**Problem**: Renamed struct fields but forgot to update instantiations
```rust
// ❌ WRONG - Will cause "no such field" error
struct MyStruct { _unused_field: i32 }
let s = MyStruct { unused_field: 5 };  // Error!

// ✅ CORRECT - Field names must match exactly
struct MyStruct { _unused_field: i32 }
let s = MyStruct { _unused_field: 5 };  // Works!
```
**Fix**: Update ALL instantiations when renaming fields. Use global find-replace.

### Type Annotation Required for Generics
**Problem**: Compiler can't infer type for generic `::new()` methods
```rust
// ❌ WRONG - Compiler doesn't know what T is
let visited = HashSet::new();

// ✅ CORRECT - Explicit type annotation
let visited: HashSet<NodeId> = HashSet::new();
// OR use turbofish syntax
let visited = HashSet::<NodeId>::new();
```
**Fix**: Always annotate types when using generic constructors.

### Unnecessary Mutability Warnings
**Problem**: Variable declared `mut` but never modified
```rust
// ❌ WRONG - Unnecessary mut for display-only variable
let mut result = calculate_something();
println!("{}", result);  // Never modified!

// ✅ CORRECT - Remove mut if not modifying
let result = calculate_something();
println!("{}", result);
```
**Fix**: Only use `mut` when actually modifying the variable.

### Grid Dimension Calculation Errors
**Problem**: Format string placeholders don't match arguments
```rust
// ❌ WRONG - 3 placeholders but only 2 arguments
format!("#{}{}{}", ".".repeat(20), ".".repeat(10));  // Missing 3rd arg!

// ✅ CORRECT - Match placeholders to arguments
format!("#{}{}#{}", ".".repeat(20), ".".repeat(19), "");  // Correct count
```
**Fix**: Count format `{}` placeholders and verify argument list matches.

### Unused Import Warnings
**Problem**: Imported items never used in code
```rust
// ❌ WRONG - GraphType imported but never used
use crate::{GraphType, NodeId, Graph};  // GraphType not used!

// ✅ CORRECT - Remove unused imports
use crate::{NodeId, Graph};
```
**Fix**: Run `cargo clippy` and remove all unused imports it reports.

---

## 📝 Standard Commit Message Template

Use this format for comprehensive commits:

```
[Category] Brief summary (50 characters max)

Detailed changes:
- ✅ Section/File 1: Specific change description
- ✅ Section/File 2: Specific change description  
- 🔧 Fixed: Issue that was resolved
- 📝 Updated: Documentation improvements
- ✨ Added: New features or functionality

Context:
- Why: Rationale for these changes
- Impact: What improved (performance, clarity, correctness)
- Related: Links to issues, requirements, or other commits

Requirements: REQ-X, REQ-Y (if applicable)
Testing: All tests pass, zero clippy warnings
```

**Example from actual session:**
```
[Daily Study] Enhance Day25 BFS documentation and fix Mission7_tut compilation

Detailed changes:
- ✅ Day25.md: Fixed efficiency calculation using Manhattan distance (was hardcoded 20)
- ✅ Day25.md: Fixed Example 7 grid dimensions (50×50 now correct)
- ✅ Day25.md: Added 120-line game AI search section
- ✅ step4_algorithm_foundation.rs: Added type annotations for HashSet, Vec, VecDeque
- ✅ step4_algorithm_foundation.rs: Removed unnecessary mut keywords
- ✅ step4_algorithm_foundation.rs: Fixed unused struct field warnings
- ✅ step7_integration_project.rs: Cleaned up unused imports

Context:
- Why: Improve learning materials accuracy and fix tutorial compilation
- Impact: Examples now executable, meaningful metrics, zero compilation warnings
- Related: BFS learning progression, Mission7 graph algorithms

Testing: All tutorial examples compile cleanly with zero warnings
```

---

## 💾 Session Handoff Template

When work spans multiple sessions or needs continuation:

```markdown
### Session Context: [Brief Task Description]

**Status**: 🔄 In Progress | ✅ Complete | ❌ Blocked

**Files Modified**:
- ✅ `path/to/file1.rs` - Description (COMPLETE)
- ⏳ `path/to/file2.rs` - Description (IN PROGRESS - line X)
- ❌ `path/to/file3.rs` - Description (BLOCKED - reason)

**Current Position**: 
Working on [specific function/section] in [file], currently at line [N].

**Next Steps**:
1. [Specific action required]
2. [Verification step]
3. [Documentation update]

**Blockers**: 
- [What's preventing completion]
- [What information is needed]

**Context & Decisions**:
- Chose [approach A] over [approach B] because [reason]
- Following pattern from [reference file/section]
- Aligns with [requirement REQ-X]

**Testing Status**:
- [ ] Unit tests pass
- [ ] Clippy clean  
- [ ] Doctests pass
- [ ] Integration tests verified

**Commands to Resume**:
```bash
cd path/to/workspace
cargo test -p package_name
# ... other relevant commands
```
```

**Example from today's session:**
```markdown
### Session Context: Fix Mission7_tut Compilation Warnings

**Status**: ✅ Complete (step4 and step7 fixed)

**Files Modified**:
- ✅ `tutorials/Mission7_tut/examples/step7_integration_project.rs` (COMPLETE)
- ✅ `tutorials/Mission7_tut/examples/step4_algorithm_foundation.rs` (COMPLETE)

**Current Position**: 
All warnings fixed, examples compile cleanly.

**Completed Steps**:
1. ✅ Removed unused imports (GraphType, tutorial_utils)
2. ✅ Added type annotations for generic types
3. ✅ Removed unnecessary `mut` keywords  
4. ✅ Prefixed unused struct fields with underscore
5. ✅ Updated all struct instantiations to match

**Testing Status**:
- ✅ Unit tests pass
- ✅ Clippy clean (zero warnings)
- ✅ Examples run successfully

**Commands Used**:
```bash
cargo run -p mission7_tut --example step4_algorithm_foundation
cargo clippy -p mission7_tut -- -D warnings
```
```

---

## 🎯 Core Development Philosophy: 3-Track Learning System

This codebase integrates three parallel learning tracks for comprehensive Rust mastery, guided by established software engineering principles:

### 🏗️ **Software Engineering Principles Integration**

**Clean Code Principles** (Robert Martin):
- **Meaningful Names**: All functions, structs, and variables use expressive, intention-revealing names
- **Single Responsibility**: Each function/struct does one thing well
- **Small Functions**: Keep functions focused and under 20 lines when possible  
- **No Side Effects**: Pure functions preferred, mutations clearly documented
- **Command-Query Separation**: Functions either do something or return something, not both
- **DRY Principle**: Don't repeat yourself - extract common patterns into reusable components

**Software Architecture in Practice** (Bass, Clements, Kazman):
- **Quality Attributes Focus**: Architecture decisions explicitly target performance, modifiability, testability
- **Deep Modules**: Simple interfaces hiding complex implementations (Rust's zero-cost abstractions)
- **Information Hiding**: Implementation details encapsulated behind clear APIs
- **Architecture Documentation**: Document architectural decisions, rationale, and trade-offs in README files
- **Component-Based Design**: Each Mission is a self-contained component with well-defined interfaces
- **Layered Architecture**: Clear separation between data structures → algorithms → applications

**When Creating New Content:**
- Apply **Clean Code naming conventions** - use `calculate_total_price()` not `calc()`
- Follow **Single Responsibility Principle** - one REQ per function when possible
- Design **Deep Modules** - simple public APIs hiding complex internal logic
- Document **architectural decisions** and quality attribute trade-offs
- Create **testable architectures** with clear component boundaries
- Prioritize **modifiability** through trait-based design and separation of concerns

This codebase integrates three parallel learning tracks for comprehensive Rust mastery:

### **Track 1: V-Cycle Missions** (Engineering Discipline)
Requirements-driven development with complete traceability:
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

### **Track 2: Daily Study** (Systematic Learning)
Structured daily practice covering core concepts:
- **Week 1-2**: Collections (HashMap, HashSet, BTreeMap, Iterators)
- **Week 3**: Traits, Generics, Lifetimes
- **Week 4-5**: Grids, Parsing, Error Handling
- Each day includes **Complete Runnable Examples**

### **Track 3: Rust Book** (Foundation Knowledge)
Progressive chapter-by-chapter study:
- Ownership → Structs → Collections → Error Handling → Generics → Testing
- Integrated with practical mission work
- Hands-on exercises and examples

**Key Pattern**: All three tracks reinforce each other - missions provide depth, daily study provides breadth, and the Rust book provides foundational understanding.

## 📂 Workspace Architecture

### 3-Track Learning Structure
**V-Cycle Missions** (Professional Engineering):
- **Mission1-5/**: Core data structures (Stack, Queue, Search, LinkedList, HashMap)
- **Mission6+/**: Advanced algorithms (Grids, Graphs, BFS/DFS)
- **Brackets_*/competitive_*/**: Real-world applications with AoC validation

**Daily Study Notes** (Systematic Practice):
- **daily_study/rust_learning_week*_notes/**: Structured daily learning files
- **Day01-14**: Collections mastery (HashMap → BTreeMap → Iterators → Errors)
- **Day15+**: Advanced topics (Traits, Generics, Lifetimes)
- **Complete Runnable Examples**: Every day file includes executable code

**Rust Book Integration** (Foundation):
- **rust_book/Ch1-Ch5/**: Basic Rust concepts with hands-on examples  
- **Ch6+/**: Advanced language features
- Coordinated with mission and daily study progress

### Learning Resources Integration
```
MONTHLY_CALENDAR.md           # 30-day learning plan with 3-track coordination
                             # ⚠️  CRITICAL: Contains alignment requirements for Mission + Tutorial coordination
.github/COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md  # Standards for executable examples
run_markdown_code.ps1         # Tool to execute examples from .md files
```

### 🔗 **MANDATORY Mission-Tutorial Alignment Protocol**
**BEFORE creating any new Mission + Tutorial pair:**

1. **Review MONTHLY_CALENDAR.md** - Check alignment requirements in "Track Alignment & Coordination" section
2. **Daily Focus Mapping** - Ensure MissionX_tut steps correspond to daily mission focus goals
3. **Tutorial Synchronization** - Design tutorial progression to support main mission REQ-X completion
4. **Integration Timeline** - Plan so tutorial completion = mission mastery achievement

**Alignment Verification Checklist:**
- [ ] Tutorial steps map to specific mission requirements (REQ-1 → step1_*, REQ-2 → step2_*, etc.)
- [ ] Daily tutorial activities build toward main mission implementation
- [ ] Tutorial completion provides all knowledge needed for mission success
- [ ] Both MissionX/ and MissionX_tut/ can be completed within planned calendar timeframe

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

### Complete Runnable Example Requirements
**MANDATORY** for all daily study files (`daily_study/rust_learning_week*_notes/DayXX.md`):

```markdown
## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as a .rs file
fn main() {
    println!("=== [Topic] Demo from Day [X] ===\n");
    // 4-7 educational sections with progressive complexity
}
```

### **🛠️ How to Run This Code:**
1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day[X]_demo.rs` and run `rustc day[X]_demo.rs && ./day[X]_demo`
3. **In this workspace**: `.\scripts\run_md.bat daily_study\rust_learning_week*_notes\Day[X].md`
```

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
.\scripts\run_md.bat DayXX.md        # Run complete runnable examples from markdown
```

### 3-Track Daily Workflow (30-45 minutes)
1. **Mission Work** (15 min): Focus on current V-Cycle mission requirements
2. **Daily Study** (15 min): Complete the day's concept with runnable example
3. **Rust Book** (15 min): Read assigned chapter with hands-on practice

### Complete Runnable Example Workflow
When creating/updating daily study files:
1. **Follow template** from `.github/COMPLETE_RUNNABLE_EXAMPLE_TEMPLATE.md`
2. **Test immediately**: `.\scripts\run_md.bat daily_study\rust_learning_week*_notes\DayXX.md`
3. **Include 4-7 sections**: Basic → Advanced → Real-world → AoC patterns
4. **Self-contained**: All helper functions included
5. **Educational**: Progressive complexity with clear section headers

### Mission Development Pattern
1. **Requirements Phase**: Write REQ-X statements in README.md
2. **Design Phase**: API contracts, data structures, invariants
3. **Implementation**: Focus on one requirement at a time
4. **Verification**: Unit tests for each REQ-X
5. **Validation**: Integration tests, performance benchmarks
6. **Documentation**: V-Cycle summary in README.md
7. **Tutorial Creation**: Build companion MissionX_tut project following tutorial.engineer.md guidelines

### Tutorial Project Structure
Each mission should have a companion tutorial project (`MissionX_tut/`) that includes:
- **Step-by-step learning progression** following pedagogical design principles
- **Progressive disclosure** breaking complex concepts into digestible steps
- **Hands-on exercises** with runnable examples at each stage
- **Error anticipation** showing common mistakes and solutions
- **Multiple learning styles** support (visual, textual, kinesthetic)

### Tutorial Development Requirements
Following `tutorial.engineer.md` specifications:
- **Learning Objectives**: Clear "what you'll learn" statements
- **Prerequisites**: Required knowledge and setup
- **Progressive Sections**: Concept introduction → minimal example → guided practice → variations → challenges
- **Complete Runnable Examples**: Every tutorial step includes executable code
- **Self-Assessment Checkpoints**: Readers can validate their understanding
- **Troubleshooting Sections**: Address common errors proactively

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
3. **Check MONTHLY_CALENDAR.md** for daily alignment requirements before starting
4. **Follow test-first development**: Write `req{N}_*` tests before implementation
5. **Validate against reference implementations** (Vec, VecDeque, etc.)
6. **Add integration examples** showing real-world usage
7. **Update V-Cycle documentation** with traceability
8. **Create companion tutorial project** (MissionX_tut) with progressive learning structure
9. **Verify alignment**: Ensure tutorial steps map to calendar daily mission focus goals

## 🎓 Tutorial Engineering Integration

When creating new missions, simultaneously build tutorial projects that:
- **Transform complex concepts** into step-by-step learning experiences
- **Follow pedagogical design principles** from tutorial.engineer.md
- **Support multiple learning modalities** (visual, hands-on, conceptual)
- **Include error anticipation** and troubleshooting guidance
- **Provide self-assessment checkpoints** for learner validation
- **Create hands-on coding exercises** that build from simple to complex

### 📅 **CRITICAL: MONTHLY_CALENDAR.md Integration Requirements**

**Mission-Tutorial Coordination Protocol:**
1. **Daily Focus Alignment**: Each MissionX_tut step must correspond to specific MONTHLY_CALENDAR.md daily mission focus
2. **Requirement Mapping**: Tutorial exercises must build toward fulfilling main mission REQ-X statements  
3. **Timeline Synchronization**: Tutorial completion should align with mission completion within calendar schedule
4. **Knowledge Transfer**: By tutorial end, learner should possess all skills needed for mission success

**Example Required Alignment Pattern:**
```
Day 1: Mission Setup → step1_basic_structure.rs (REQ-1 foundation)
Day 2: Requirements → step2_operations.rs (REQ-2, REQ-3 practice) 
Day 3: Implementation → step3_advanced_features.rs (REQ-4 mastery)
Day 4: Testing → step4_integration.rs (REQ-5 validation)
Day 5: Documentation → step5_final_project.rs (complete mission review)
```

**⚠️  MANDATORY CHECK**: Before submitting Mission + Tutorial pair, verify:
- Tutorial steps appear in MONTHLY_CALENDAR.md daily activities
- Each tutorial step advances toward specific mission requirements
- Tutorial completion timeline matches mission completion timeline
- Combined effort (Mission + Tutorial) fits within allocated daily time budget (30-45 minutes)

## 💡 When Working on This Codebase

### 3-Track Integration Principles
- **Cross-track reinforcement**: Connect mission concepts to daily study topics
- **Progressive complexity**: Daily study feeds into mission requirements
- **Practical application**: Rust book concepts appear in AoC-style problems
- **Complete examples**: Every learning concept has runnable demonstration

### V-Cycle Mission Requirements
- **Always trace features back to requirements** - if there's no REQ-X, create one
- **Test naming is semantic** - `req2_push_amortized_constant` tells you exactly what it verifies
- **Performance matters** - include Big-O analysis and benchmarks
- **Documentation is part of the design** - README.md includes full V-Cycle summary
- **Integration over isolation** - demos show how data structures solve real problems
- **Tutorial companion projects** - Create MissionX_tut following tutorial.engineer.md for educational reinforcement
- **⚠️  CALENDAR ALIGNMENT** - Ensure MissionX_tut activities align with MONTHLY_CALENDAR.md daily focus goals
- **📋 REQUIREMENT MAPPING** - Tutorial exercises must directly support mission REQ-X completion

### Software Engineering Discipline Application
- **Clean Code naming** - Use `validate_brackets()` not `check()`, `calculate_hash()` not `hash()`
- **Single Responsibility functions** - Each function should fulfill one REQ or sub-requirement
- **Deep module design** - Simple public APIs (`insert()`, `get()`) hiding complex internals (hashing, collision resolution)
- **Information hiding** - Keep implementation details private, expose only necessary interfaces
- **Architecture documentation** - Document quality attribute decisions (performance vs. memory, flexibility vs. simplicity)
- **Testable design** - Structure code so each architectural component can be tested independently
- **Quality attribute focus** - Explicitly design for performance, modifiability, testability in each Mission

### Daily Study Standards
- **Complete Runnable Examples** - MANDATORY for every Day file
- **Test immediately** - Use `.\scripts\run_md.bat` to verify examples work
- **4-7 educational sections** - Progressive complexity from basic to AoC-style
- **Self-contained code** - Include all helper functions needed
- **Multiple execution paths** - Playground, local file, markdown runner, cargo examples

### 🏷️ Zettelkasten Tagging Standards

**MANDATORY** for all new content creation in the `zettelkasten/` directory:

#### **Tag Categories & Patterns**
```markdown
---
*Tags: #primary-topic #secondary-topic #content-type #learning-track*
*Links: [[zettel-index]] | [[Related MOC]] | [[Connected Concept]]*
---
```

#### **Primary Topic Tags** (Choose ONE per note)
- `#hashmap` `#hashset` `#btreemap` `#vector` `#linkedlist` - Data structures
- `#ownership` `#borrowing` `#lifetimes` `#traits` `#generics` - Rust concepts  
- `#algorithms` `#sorting` `#searching` `#graph` `#tree` - Algorithm types
- `#parsing` `#regex` `#iterators` `#error-handling` - Processing patterns
- `#performance` `#benchmarking` `#optimization` - Performance topics
- `#testing` `#documentation` `#v-cycle` - Engineering practices

#### **Content Type Tags** (Choose ONE per note)
- `#concept` - Theoretical explanations and deep dives
- `#implementation` - Code examples and practical demonstrations  
- `#tutorial` - Step-by-step learning progressions
- `#overview` - High-level summaries and navigation hubs
- `#reference` - Quick lookup and API documentation
- `#pattern` - Reusable design patterns and best practices

#### **Learning Track Tags** (Choose ALL that apply)
- `#mission1` `#mission2` `#mission5` etc. - V-Cycle mission integration
- `#daily-study` - Daily study track concepts
- `#rust-book` - Rust Book chapter integration  
- `#aoc` - Advent of Code applications
- `#competitive-programming` - Algorithm competition focus

#### **Cross-Reference Tags** (Use for connections)
- `#cross-track` - Links multiple learning tracks
- `#prerequisite` - Required knowledge for other concepts
- `#application` - Practical usage of theoretical concepts
- `#comparison` - Comparative analysis between options
- `#troubleshooting` - Common errors and solutions

#### **Example Tag Combinations**
```markdown
# HashMap Internals Example
*Tags: #hashmap #concept #data-structures #mission5 #daily-study #performance*

# Mission5 Tutorial Step 3 Example  
*Tags: #hashmap #tutorial #implementation #mission5 #step-by-step*

# Collections MOC Example
*Tags: #collections #overview #data-structures #cross-track #navigation*

# Day 10 Study Notes Example
*Tags: #hashmap #daily-study #concept #rust-book #prerequisite*
```

#### **Tag Placement Requirements**
1. **Bottom of every zettelkasten file** - Include tag line before final links
2. **Consistent format** - Always use `*Tags: #tag1 #tag2 #tag3*`
3. **3-6 tags per note** - Balance discoverability with specificity
4. **Update existing files** - Add tags when editing existing zettelkasten content
5. **Cross-reference validation** - Ensure tagged concepts have corresponding `[[links]]`

#### **Tag-Based Navigation Workflows**  
- **By Topic**: `#hashmap` finds all HashMap-related content across tracks
- **By Type**: `#tutorial` finds all step-by-step learning content
- **By Mission**: `#mission5` finds all content related to Mission 5
- **By Application**: `#aoc` finds all Advent of Code usage patterns
- **Cross-Track**: `#cross-track` finds integration points between learning systems

#### **Obsidian Tag Integration**
- Use Obsidian's **Tag Pane** for tag-based navigation
- Create **Tag Queries** for complex tag combinations
- Use **Graph View** with tag filtering for visual knowledge networks
- Set up **Tag Templates** in Obsidian for consistent tagging

**⚠️ CRITICAL**: Every new zettelkasten file MUST include appropriate tags. This enables powerful filtering, discovery, and cross-referencing within the knowledge management system.

### 🔗 **Zettelkasten Link Naming Convention**

**MANDATORY** for all internal links in `zettelkasten/` directory to prevent naming collisions:

#### **Link Format Standards**

**Daily Study Notes:**
- **Format**: `[[daily-study/DayXX]]` or `[[ds-dayXX]]`
- **Examples**: 
  - `[[daily-study/Day24]]` (full format)
  - `[[ds-day24]]` (short format)
- **Rationale**: Distinguishes from AoC days, matches directory structure

**Advent of Code Problems:**
- **Format**: `[[aoc-YYYY-dayDD]]` or `[[aocYY-DD]]`
- **Examples**: 
  - `[[aoc-2023-day12]]` (full format - unambiguous)
  - `[[aoc23-12]]` (short format - compact)
  - `[[aoc-2015-day01]]` (full format with zero-padded day)
  - `[[aoc15-01]]` (short format)
- **Rationale**: Year makes it unique and searchable, prevents confusion with daily study

**Mission References:**
- **Format**: `[[mission-X]]` or `[[mX]]`
- **Examples**: 
  - `[[mission-6]]` or `[[m6]]`
  - `[[mission-5-tutorial]]` or `[[m5-tut]]`
- **Rationale**: Clear mission identifier, supports tutorial sub-references

**Rust Book Chapters:**
- **Format**: `[[rust-book-chX]]` or `[[rb-chX]]`
- **Examples**: 
  - `[[rust-book-ch8]]` or `[[rb-ch8]]`
  - `[[rust-book-ch4-ownership]]` or `[[rb-ch4-ownership]]`
- **Rationale**: Disambiguates from chapters in other contexts

**Concept/Implementation Notes:**
- **Format**: `[[lowercase-with-dashes]]`
- **Examples**: 
  - `[[find-all-components]]`
  - `[[hashmap-internals]]`
  - `[[flood-fill]]`
  - `[[4-connectivity]]`
- **Rationale**: Standard wiki-style naming, easy to type and read

#### **Naming Collision Prevention**

| Ambiguous (❌ Avoid) | Clear (✅ Use) | Context |
|----------------------|---------------|---------|
| `[[Day24]]` | `[[daily-study/Day24]]` or `[[ds-day24]]` | Daily study |
| `[[Day12]]` | `[[aoc-2023-day12]]` or `[[aoc23-12]]` | AoC problem |
| `[[Mission6]]` | `[[mission-6]]` or `[[m6]]` | Mission reference |
| `[[Ch8]]` | `[[rust-book-ch8]]` or `[[rb-ch8]]` | Rust Book |

#### **Link Usage Examples**

```markdown
# Example Zettelkasten Note

See [[daily-study/Day24]] for visual demonstrations.
Compare with [[aoc-2023-day12]] for practical application.
Implemented in [[mission-6]] using patterns from [[rust-book-ch8]].

Related concepts: [[flood-fill]], [[4-connectivity]], [[dfs-patterns]]

*Links: [[zettel-index]] | [[daily-study/Day24]] | [[mission-6]] | [[flood-fill]]*
```

#### **Quick Reference Table**

| Content Type | Full Format | Short Format |
|--------------|-------------|--------------|
| Daily Study Day 24 | `[[daily-study/Day24]]` | `[[ds-day24]]` |
| AoC 2023 Day 12 | `[[aoc-2023-day12]]` | `[[aoc23-12]]` |
| Mission 6 | `[[mission-6]]` | `[[m6]]` |
| Rust Book Ch 8 | `[[rust-book-ch8]]` | `[[rb-ch8]]` |
| Concepts | `[[concept-name]]` | - |

**⚠️ CRITICAL**: Always use the full or short format specified above. Never use ambiguous forms like `[[Day24]]` or `[[Mission6]]` which could refer to multiple content types.

### Quality Assurance
- **Zero warnings tolerance** - `cargo clippy -- -D warnings` must pass
- **Professional standards** - Every feature follows engineering discipline
- **Comprehensive testing** - Unit, integration, and requirement-based tests
- **Documentation completeness** - Module docs, function docs, and examples
- **Documentation standards** - Follow [RUST_DOCUMENTATION_STANDARDS.md](RUST_DOCUMENTATION_STANDARDS.md) and [RUST_TEST_DOCUMENTATION_STANDARDS.md](RUST_TEST_DOCUMENTATION_STANDARDS.md)
- **Clean Code compliance** - Apply meaningful names, single responsibility, and DRY principles
- **Architecture quality** - Ensure deep modules, information hiding, and quality attribute achievement

This codebase treats learning Rust as an engineering discipline with integrated tracks that reinforce each other through practical, runnable examples and formal V-Cycle methodology.