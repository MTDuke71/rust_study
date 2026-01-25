# AoC 2022 - Function Guide Template

Copy this template to create function guides for each day.

**Usage**: 
```bash
cp templates/aoc_documentation/function_guide_template.md \
   advent_of_code/aoc2022/Problem_Statements/days/day01_function_guide.md
```

Then fill in the sections below.

---

# Day X: [Problem Title]

## Problem Overview
[1-2 sentence summary of what Part 1 and Part 2 ask]

**Answer**: Part 1: XXXXX | Part 2: XXXXX

## Performance Benchmarks
- **Part 1**: XX.Xµs
- **Part 2**: XX.Xµs
- **Total**: XX.Xµs

## Core Algorithm: [Name]

[Brief description of main approach]

**Algorithm**:
1. Step 1
2. Step 2
3. Step 3

**Time Complexity**: O(?)  
**Space Complexity**: O(?)

---

## Function Reference

### Main Entry Points

#### `solve_part1(input: &str) -> [ReturnType]`
**Purpose**: [What this solves]

**Algorithm**:
```rust
pub fn solve_part1(input: &str) -> [ReturnType] {
    // High-level pseudocode
}
```

**Example**:
```
Input: [example]
Output: XXX
```

---

#### `solve_part2(input: &str) -> [ReturnType]`
**Purpose**: [How Part 2 differs]

**Algorithm**:
```rust
pub fn solve_part2(input: &str) -> [ReturnType] {
    // High-level pseudocode
}
```

**Key Insight**: [What makes Part 2 work]

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

---

## Type Definitions

#### `[StructName]`
```rust
#[derive(Debug, Clone)]
struct [StructName] {
    field1: Type,
    field2: Type,
}
```

**Purpose**: [Why this structure?]  
**Design Decision**: [Trade-offs]

---

## Algorithm Deep Dive

### Why This Approach Works

[Explain the intuition]

### Complexity Analysis

**Part 1**: O(?) because [reason]  
**Part 2**: O(?) because [reason]

### Alternative Approaches

**Brute Force**: O(?) - [Why it doesn't work]  
**Optimized**: O(?) - [Why this works]

---

## Test Cases

### Example Input
```
[Paste example from problem]
```

**Expected Output**:
- Part 1: XXX
- Part 2: XXX

### Edge Cases Handled
- [Edge case 1]: [How handled]
- [Edge case 2]: [How handled]

---

## Key Insights

1. **[Insight 1]**: [Explanation]
2. **[Insight 2]**: [Explanation]
3. **[Insight 3]**: [Explanation]

---

## Rust Patterns Used

- **[Pattern 1]**: [Where and why]
- **[Pattern 2]**: [Where and why]

### Mission Integration
- **Mission X**: [Which component? How used?]
- **Integrator Philosophy**: [How composition helped]

---

## Related Problems

- **AoC 2023 Day X**: Similar [algorithm/pattern]
- **Mission X**: Uses same [component]

---

## Zettelkasten Links
- [[algorithm-name]]
- [[pattern-name]]
- [[aoc2022-dayXX]]

---

**Date Completed**: YYYY-MM-DD  
**Solving Time**: [How long for both parts]  
**Documentation Time**: [How long to document]  
**Difficulty**: ⭐⭐⭐ (1-5 stars)
