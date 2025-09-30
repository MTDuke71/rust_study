# AI Agent Instructions for Rust Study Codebase

This is a **V-Cycle learning workspace** for systematic Rust development using formal software engineering practices. The workspace follows a **3-track learning approach** with complete traceability from requirements through validation.

## 🎯 Core Development Philosophy: 3-Track Learning System

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

### Quality Assurance
- **Zero warnings tolerance** - `cargo clippy -- -D warnings` must pass
- **Professional standards** - Every feature follows engineering discipline
- **Comprehensive testing** - Unit, integration, and requirement-based tests
- **Documentation completeness** - Module docs, function docs, and examples
- **Documentation standards** - Follow [RUST_DOCUMENTATION_STANDARDS.md](RUST_DOCUMENTATION_STANDARDS.md) and [RUST_TEST_DOCUMENTATION_STANDARDS.md](RUST_TEST_DOCUMENTATION_STANDARDS.md)

This codebase treats learning Rust as an engineering discipline with integrated tracks that reinforce each other through practical, runnable examples and formal V-Cycle methodology.