# Day X: [Problem Title]

**Problem**: [One-sentence summary of what Part 1 and Part 2 ask you to do]

**Key Insights**: 
- Part 1: [Core algorithm/approach]
- Part 2: [How Part 2 differs or scales the problem]

---

## 📊 Stats

| Metric | Value |
|--------|-------|
| **Part 1 Answer** | XXXXX |
| **Part 2 Answer** | XXXXX |
| **Part 1 Runtime** | XX.Xµs |
| **Part 2 Runtime** | XX.Xµs |
| **Total Runtime** | XX.Xµs |
| **Complexity** | O(?) Part 1, O(?) Part 2 |
| **Mission Integration** | Mission X (Component) |
| **Patterns Used** | Pattern 1, Pattern 2 |

**Performance Notes**: [e.g., "Part 2 faster than Part 1 due to X", "Optimized with Y"]

---

## 🎯 Problem Summary

### Part 1
**Goal**: [What are we trying to compute/find?]

**Example**:
```
[Input example]
```
Expected output: `XXX`

**Constraints**: [Input size, edge cases, etc.]

### Part 2
**Goal**: [How does Part 2 extend Part 1?]

**Example**:
```
[Input example if different]
```
Expected output: `XXX`

**Scaling Factor**: [e.g., "1000x more data", "new constraint added"]

---

## 🧠 Key Insights

### Part 1 Approach
[Explain the core algorithm/approach]

**Why This Works**: [Intuition behind the approach]

**Example Walkthrough**:
```
Step 1: [What happens]
Step 2: [What happens]
...
Result: XXX
```

### Part 2 Breakthrough
[What's the key insight that makes Part 2 tractable?]

**Naive Approach Would**: [Why brute force fails]
**Optimized Approach**: [The clever solution]

**Complexity Comparison**:
- Brute force: O(?) - [Why infeasible]
- Optimized: O(?) - [Why tractable]

---

## 🏗️ Implementation Details

### Type Definitions

#### `[StructName]` - [Purpose]
```rust
#[derive(Debug, Clone)]
struct [StructName] {
    field1: Type,
    field2: Type,
}

impl [StructName] {
    fn method_name(&self) -> ReturnType {
        // Brief implementation note
    }
}
```

**Design Decision**: [Why this structure? What trade-offs?]

[Repeat for other key types: enums, structs]

---

### Core Functions

#### `parse_input(input: &str) -> ParsedType`
**Purpose**: [What does this parse?]

```rust
fn parse_input(input: &str) -> ParsedType {
    // Key parsing logic
}
```

**Algorithm**: [How it works]  
**Complexity**: O(?)  
**Edge Cases**: [What to watch for]

---

#### `solve_part1(input: &str) -> String`
**Purpose**: [Main solving logic for Part 1]

```rust
pub fn solve_part1(input: &str) -> String {
    // High-level algorithm
}
```

**Algorithm**: [Step-by-step approach]
1. Parse input
2. [Key step]
3. [Key step]
4. Return result

**Complexity**: O(?)

**Example Trace**: [Walk through with example input]
```
Input: [example]
Step 1: [state]
Step 2: [state]
Result: XXX
```

---

#### `solve_part2(input: &str) -> String`
**Purpose**: [Main solving logic for Part 2]

```rust
pub fn solve_part2(input: &str) -> String {
    // High-level algorithm
}
```

**Algorithm**: [How it differs from Part 1]

**Optimization**: [If applicable - what makes this efficient?]

**Complexity**: O(?)

---

### Helper Functions

#### `helper_function_name(params) -> ReturnType`
**Purpose**: [What does this do?]

```rust
fn helper_function_name(params) -> ReturnType {
    // Key logic
}
```

**Algorithm**: [Brief explanation]  
**Complexity**: O(?)  
**Used By**: `solve_part1()`, `solve_part2()`

[Repeat for other important helpers]

---

## 🎨 Design Patterns

### 1. [Pattern Name]
**Where Used**: [Which functions use this?]

**Pattern**:
```rust
// Code example showing the pattern
```

**Why**: [What problem does this solve?]  
**Trade-offs**: [Any downsides?]

### 2. [Pattern Name]
[Repeat for other notable patterns]

---

## ⚡ Performance Analysis

### Benchmark Results
```
Part 1: XX.Xµs (XX.X% of total)
Part 2: XX.Xµs (XX.X% of total)
Total:  XX.Xµs
```

### Breakdown
| Component | Time | % |
|-----------|------|---|
| Parsing | XX.Xµs | XX% |
| [Core computation] | XX.Xµs | XX% |
| [Other step] | XX.Xµs | XX% |

### Optimization History

**Initial Implementation**: XX.Xms  
**After [Optimization 1]**: XX.Xµs (XXx speedup)  
**After [Optimization 2]**: XX.Xµs (XXx speedup)  

**Key Optimization**: [What was the breakthrough?]

**Code Comparison**:
```rust
// ❌ Before: O(n²)
for item in items {
    for other in items {
        // expensive
    }
}

// ✅ After: O(n)
let index = build_index(items);
for item in items {
    // O(1) lookup
}
```

---

## ✅ Tests

### Test Coverage
- ✅ Part 1 example (expected: XXX)
- ✅ Part 2 example (expected: XXX)
- ✅ [Helper function test]
- ✅ [Edge case test]
- ✅ [Boundary condition test]

### Test Examples
```rust
#[test]
fn test_part1_example() {
    let input = "[example input]";
    assert_eq!(solve_part1(input), "XXX");
}

#[test]
fn test_part2_example() {
    let input = "[example input]";
    assert_eq!(solve_part2(input), "XXX");
}
```

---

## 🔗 Related Content

### Zettelkasten
- [[pattern-name]] - [Brief description]
- [[algorithm-name]] - [Brief description]

### Patterns Catalog
- [Pattern 1](../patterns_catalog.md#pattern-1) - [Where used]
- [Pattern 2](../patterns_catalog.md#pattern-2) - [Where used]

### Algorithms Reference
- [Algorithm 1](../algorithms_reference.md#algorithm-1) - [Complexity]
- [Algorithm 2](../algorithms_reference.md#algorithm-2) - [Complexity]

### Related Days
- [Day X](dayXX_comprehensive.md) - [Similar pattern/algorithm]
- [Day Y](dayYY_comprehensive.md) - [Related concept]

---

## 📝 Implementation Notes

### Rust Techniques Applied
- [Technique 1]: [Where and why used]
- [Technique 2]: [Where and why used]

### Mission Integration
- **Mission X**: [Which component? How used?]
- **Integrator Philosophy**: [How composition helped]

### Lessons Learned
1. [Lesson 1]
2. [Lesson 2]
3. [Lesson 3]

### Edge Cases Handled
- [Edge case 1]: [How handled]
- [Edge case 2]: [How handled]

---

**Date Completed**: YYYY-MM-DD  
**Completion Time**: [How long to solve both parts]  
**Difficulty**: ⭐⭐⭐ (1-5 stars)
