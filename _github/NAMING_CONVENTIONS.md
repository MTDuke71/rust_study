# 📋 Rust Study Project - Naming Conventions

**Part of**: Coding Standards and Guidelines  
**Status**: Active  
**Last Updated**: October 17, 2025

---

## 🎯 Overview

This document establishes consistent naming conventions across the Rust Study workspace to prevent collisions, improve clarity, and maintain professional standards.

---

## 📚 Example Names - MOST IMPORTANT

### **Rule: Example Names MUST Be Unique Across Entire Workspace**

When creating new examples in `Cargo.toml` files, **ALWAYS** use unique names across the entire workspace to avoid filename collisions.

### **✅ CORRECT Naming Pattern**

```toml
[[example]]
name = "mission3_demo"  # Prefixed with package context
path = "examples/demo.rs"

[[example]]
name = "aoc_pattern_simple_demo"  # Descriptive and unique
path = "examples/simple_demo.rs"

[[example]]
name = "mission4_performance_comparison"  # Specific to package
path = "examples/performance_comparison.rs"
```

### **❌ INCORRECT Naming Pattern**

```toml
[[example]]
name = "demo"  # ❌ Too generic, will collide with other packages
path = "examples/demo.rs"

[[example]]
name = "simple_demo"  # ❌ Common name, will collide
path = "examples/simple_demo.rs"

[[example]]
name = "performance_comparison"  # ❌ Generic, will collide
path = "examples/performance_comparison.rs"
```

### **Naming Rules**

1. **✅ Use package context prefix**: `{package_name}_{example_name}`
2. **✅ Be descriptive**: Include the purpose or functionality
3. **✅ Avoid generic names**: Never use `demo`, `test`, `example`, `simple`, `basic` alone
4. **✅ Check existing names**: Search workspace for existing example names before creating new ones
5. **✅ Use snake_case**: `mission3_queue_demo` not `mission3QueueDemo`

### **Example Name Templates**

Choose the appropriate pattern based on package type:

| Package Type | Template | Example |
|--------------|----------|---------|
| **Mission** | `mission{N}_{descriptive}` | `mission5_hashmap_demo` |
| **Mission Tutorial** | `mission{N}_tut_{descriptive}` | `mission5_tut_step3_operations` |
| **AoC** | `aoc{YYYY}_day{DD}_{descriptive}` | `aoc2023_day12_pathfinding` |
| **Advanced Example** | `{package_name}_{descriptive}` | `competitive_ring_bfs_demo` |
| **Daily Study** | `day{XX}_{descriptive}` | `day24_bfs_visualization` |

---

## 🔤 Function & Variable Naming

### **Functions**

Use **snake_case** with clear, descriptive names:

```rust
// ✅ CORRECT
pub fn validate_bracket_sequence(input: &str) -> Result<(), BracketError> { }
pub fn calculate_shortest_path(graph: &Graph, start: NodeId) -> Vec<NodeId> { }
pub fn insert_with_collision_handling(key: K, value: V) -> Result<(), InsertError> { }

// ❌ INCORRECT
pub fn validate(input: &str) -> Result<(), BracketError> { }  // Too vague
pub fn calc_path(graph: &Graph, start: NodeId) -> Vec<NodeId> { }  // Abbreviated
pub fn insert(key: K, value: V) -> Result<(), InsertError> { }  // Missing context
```

### **Variables**

Use **snake_case** with intention-revealing names:

```rust
// ✅ CORRECT
let mut visited_nodes: HashSet<NodeId> = HashSet::new();
let bracket_stack: Vec<BracketType> = Vec::new();
let max_iterations = 100;

// ❌ INCORRECT
let mut visited: HashSet<NodeId> = HashSet::new();  // Vague
let stack: Vec<BracketType> = Vec::new();  // Unclear purpose
let max_i = 100;  // Abbreviated
```

### **Unused Variables**

Prefix unused variables with underscore:

```rust
// ✅ CORRECT
let _unused_value = calculate_something();
let mut _graph_backup = graph.clone();

// ❌ INCORRECT
let unused_value = calculate_something();  // Will trigger clippy warning
```

---

## 📦 Struct & Type Naming

### **Structs**

Use **PascalCase** with descriptive names:

```rust
// ✅ CORRECT
pub struct BracketValidator { }
pub struct GraphAdjacencyList<T> { }
pub struct RingBufferQueue<T> { }

// ❌ INCORRECT
pub struct Validator { }  // Too generic
pub struct Graph { }  // Unclear type
pub struct Queue { }  // Which queue implementation?
```

### **Enums**

Use **PascalCase** for enum names and variants:

```rust
// ✅ CORRECT
pub enum BracketErrorKind {
    UnexpectedClosing,
    MismatchedPair,
    UnclosedOpenings,
}

pub enum GraphTraversalMethod {
    DepthFirstSearch,
    BreadthFirstSearch,
}

// ❌ INCORRECT
pub enum Error {
    UnexpectedClosing,
    MismatchedPair,
}  // Which error?

pub enum Method {
    DFS,
    BFS,
}  // Abbreviated, unclear
```

### **Type Aliases**

Use **PascalCase** for type aliases:

```rust
// ✅ CORRECT
pub type NodeId = usize;
pub type GraphAdjacency = Vec<Vec<NodeId>>;
pub type ValidationResult = Result<(), BracketError>;

// ❌ INCORRECT
pub type node_id = usize;  // Should be PascalCase
pub type result = Result<(), BracketError>;  // Too generic
```

---

## 🏷️ Module & File Naming

### **Modules & Files**

Use **snake_case** for module and file names:

```
✅ CORRECT:
src/
├── lib.rs
├── bracket_validator.rs
├── graph_algorithms.rs
├── ring_buffer_queue.rs
└── error_types.rs

❌ INCORRECT:
src/
├── BracketValidator.rs  # Should be snake_case
├── graph-algorithms.rs  # Use underscore, not hyphen
├── RingBufferQueue.rs   # Should be snake_case
```

### **Directory Structure**

Use **snake_case** for directory names:

```
✅ CORRECT:
missions/
├── Mission1/
├── Mission2_tut/
└── mission_shared_utils/

daily_study/
├── rust_learning_week1_notes/
└── rust_learning_week2_notes/

❌ INCORRECT:
missions/
├── mission-1/  # Use underscore
├── Mission2Tutorial/  # Use snake_case
```

---

## 🔗 Zettelkasten File Naming

Use standardized naming conventions from `.github/copilot-zettelkasten-tags.md`:

```
✅ CORRECT:
zettelkasten/
├── hashmap-internals.md
├── [[mission-5]]
├── [[daily-study/Day24]]
├── [[aoc-2023-day12]]
└── ownership-and-borrowing.md

Links in zettelkasten files:
- [[mission-5]] (not [[Mission5]])
- [[daily-study/Day24]] (not [[Daily24]])
- [[aoc-2023-day12]] (not [[AoC23-12]])
```

---

## 📄 Test Function Naming

Use descriptive names that explain what is being tested:

```rust
// ✅ CORRECT - Requirement-based naming
#[test]
fn req1_empty_stack_returns_none() { }

#[test]
fn req2_push_operation_adds_element_to_stack() { }

#[test]
fn mismatched_brackets_should_report_correct_position() { }

// ❌ INCORRECT
#[test]
fn test1() { }  // Too vague

#[test]
fn it_works() { }  // Meaningless

#[test]
fn test_push() { }  // Too generic
```

**Pattern**: `req{N}_{what_should_happen}` or `{scenario}_{expected_result}`

---

## 🏛️ Architecture & Pattern Naming

### **Trait Names**

Use **PascalCase** with action verbs or descriptions:

```rust
// ✅ CORRECT
pub trait Validatable {
    fn validate(&self) -> Result<(), ValidationError>;
}

pub trait GraphTraversable {
    fn traverse(&self) -> Vec<NodeId>;
}

// ❌ INCORRECT
pub trait Validate { }  // Should be noun describing what implements it
pub trait Traversable { }  // Too generic
```

### **Lifetime Names**

Use single letters for common lifetimes, descriptive names for complex ones:

```rust
// ✅ CORRECT - Simple cases
fn borrow<'a>(reference: &'a str) -> &'a str { }

// ✅ CORRECT - Complex cases with multiple lifetimes
fn with_borrowed_and_owned<'borrowed, 'owned>(
    borrowed: &'borrowed str,
    owned: String,
) { }

// ❌ INCORRECT
fn borrow<'lifetime>(reference: &'lifetime str) -> &'lifetime str { }
```

---

## ✅ Validation Commands

### **Check for Naming Issues**

Before committing, run these commands to catch naming problems:

```bash
# Test compilation with specific checks
cargo test --workspace --no-run

# Check for clippy warnings (includes unused variable warnings)
cargo clippy --workspace -- -D warnings

# Check example names don't collide
cargo build --examples

# Verify all examples can be run
cargo run --example {example_name} --package {package_name}
```

### **Search for Naming Conventions**

Before creating new names, search for existing similar names:

```bash
# Search for similar example names
grep -r "name = " Cargo.toml | grep example

# Search for similar function names  
grep -r "pub fn " src/ | grep {keyword}

# Search for similar struct names
grep -r "pub struct " src/ | grep {pattern}
```

---

## 📋 Naming Checklist

Before submitting code, verify:

### **Example Names**
- [ ] Example name is unique across workspace
- [ ] Example name includes package prefix
- [ ] Example name is descriptive
- [ ] Example name uses snake_case
- [ ] No generic names like `demo`, `test`, `simple`

### **Function Names**
- [ ] Function names are snake_case
- [ ] Function names clearly describe purpose
- [ ] Function names are not abbreviations
- [ ] Function names include context when needed

### **Variable Names**
- [ ] Variable names are snake_case
- [ ] Variable names are intention-revealing
- [ ] Unused variables prefixed with `_`
- [ ] No single-letter variables except `i`, `j`, `k` in loops

### **Struct/Enum Names**
- [ ] Struct/enum names are PascalCase
- [ ] Names clearly describe the type
- [ ] Names are not overly generic
- [ ] Enum variants follow PascalCase

### **Module/File Names**
- [ ] File names are snake_case
- [ ] Directory names are snake_case
- [ ] Names match Rust module naming conventions
- [ ] Names are not overly abbreviated

---

## 🚀 Integration with Other Guidelines

This naming convention document works together with:

- `.github/RUST_DOCUMENTATION_STANDARDS.md` - How to document named items
- `.github/RUST_TEST_DOCUMENTATION_STANDARDS.md` - How to name tests
- `.github/copilot-instructions.md` - Naming in code structure
- `.github/copilot-zettelkasten-tags.md` - Zettelkasten file naming
- `.github/tutorial.engineer.md` - Tutorial naming and structure

---

## 📖 Examples by Category

### **Mission Examples**
```
✅ mission5_hashmap_basic_operations
✅ mission5_collision_resolution_demo
✅ mission5_performance_benchmark
```

### **Daily Study Examples**
```
✅ day24_bfs_grid_visualization
✅ day24_priority_queue_demo
✅ day24_complete_runnable_example
```

### **AoC Examples**
```
✅ aoc2023_day12_pattern_matching_solution
✅ aoc2023_day12_optimization_techniques
✅ aoc2015_day01_santa_elevator_tracker
```

### **Tutorial Examples**
```
✅ mission5_tut_step1_basic_structure
✅ mission5_tut_step3_operations
✅ mission5_tut_step7_integration_project
```

---

## ⚠️ Common Naming Mistakes to Avoid

| ❌ Wrong | ✅ Correct | Issue |
|---------|-----------|-------|
| `demo.rs` | `mission5_demo.rs` | Missing context |
| `test_validation` | `validate_bracket_pairs` | Test naming in production |
| `stack` | `StackLifo<T>` | Too generic |
| `calc()` | `calculate_total_price()` | Abbreviated |
| `i` (for string index) | `char_index` | Unclear what i represents |
| `tmp` | `temp_result` or `intermediate_value` | Too abbreviated |
| `unused_var` | `_unused_var` | Should use underscore |
| `simple_demo` | `mission5_simple_demo` | Not unique |

---

**Status**: ✅ Active  
**Applies To**: All code in this workspace  
**Review Date**: October 17, 2025

---

## 🔗 Related Documentation

*This document is part of the comprehensive Rust Study documentation system.*

### **Core Documentation**
- **[[../zettelkasten/Documentation Standards]]** - Complete documentation standards and guidelines
- **[[../zettelkasten/Project Management and Session Reports]]** - Project tracking and session summaries
- **[[../zettelkasten/API Design Patterns]]** - Code interface design principles
- **[[../zettelkasten/Quality Assurance]]** - Testing and quality standards

### **Learning System Integration**
- **[[../zettelkasten/Rust Concepts MOC]]** - Core Rust language concepts
- **[[../zettelkasten/Daily Study MOC]]** - Daily learning progression
- **[[../zettelkasten/Missions Overview]]** - Hands-on project implementations  
- **[[../zettelkasten/V-Cycle Methodology]]** - Requirements-driven development approach

### **Workspace Navigation**
- **[[../zettelkasten/zettel-index]]** - Master index of all zettelkasten notes
- **[[WORKFLOW_DOCUMENTATION_INDEX]]** - Complete workflow documentation index

---

*Tags: #documentation #standards #workflow #rust-study #project-management*
*Part of: [[../zettelkasten/Documentation Standards]] | [[../zettelkasten/Project Management and Session Reports]]*