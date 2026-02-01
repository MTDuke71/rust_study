# Project Euler Mathematical Documentation Template

**Quick Links**: [← Project Euler Main](README.md) | [Math Foundations](../zettelkasten/math-foundations/README.md) | [Problem Statements](Problem_Statements/README.md)

---

**Location**: `zettelkasten/math-foundations/project-euler-pXXX.md`

This template guides creating mathematical write-ups for Project Euler solutions, integrating code implementations with theoretical foundations.

---

## Template Structure

```markdown
# Project Euler Problem XXX: [Problem Title]

**Solved**: [Date]  
**Difficulty**: [5%, 10%, etc. from Project Euler]  
**Category**: [Number Theory | Combinatorics | Graph Theory | etc.]

## Problem Summary

[Brief description of what the problem asks]

## Mathematical Concepts

### Primary Concepts
- **[[Concept 1]]** - [How it's used]
- **[[Concept 2]]** - [How it's used]

### Supporting Concepts
- **[[Supporting Concept]]** - [How it helps]

## Solution Approach

[High-level algorithmic strategy]

### Key Insights
1. [Critical observation 1]
2. [Critical observation 2]

### Algorithm
[Step-by-step description or pseudocode]

## Complexity Analysis

- **Time**: O(?)
- **Space**: O(?)
- **Justification**: [Why this complexity]

## Rust Implementation

See [[project_euler/src/problems/pXXX.rs]] for complete code.

### Key Code Patterns
```rust
// Example of key mathematical pattern used
```

## Related Problems

- **Project Euler**: [[project-euler-pYYY]] - Similar technique
- **AoC**: [[aoc-2023-dayXX]] - Related concept
- **Mission**: [[mission-X]] - Data structure used

## Learning Insights

[What mathematical concepts were reinforced or newly understood]

## References

- [External resource 1]
- [External resource 2]

---

*Links:*
- **Backlinks**: [What links here]
- **Concept Tags**: #number-theory #primes #modular-arithmetic (example)
- **Difficulty**: #euler-easy | #euler-medium | #euler-hard
```

---

## Integration Workflow

### 1. Before Solving
- Read problem → Identify mathematical concepts
- Check if math notes exist: `grep -r "concept" zettelkasten/math-foundations/`
- If not: Create foundational concept note first

### 2. While Solving
- Document insights as you discover them
- Note which utilities are reusable (`utils/`)

### 3. After Solving
- Create `zettelkasten/math-foundations/project-euler-pXXX.md`
- Add bidirectional links:
  - From solution code → math note (doc comment)
  - From math note → solution code (implementation section)
  - From concept notes → this problem (examples section)
  - From this problem → concept notes (concepts section)

### 4. Update Indexes
- Add to `zettelkasten/math-foundations/README.md` tracking table
- Add tags for searchability
- Link from related AoC/Mission solutions if applicable

---

## Example: Bidirectional Linking

**In `project_euler/src/problems/p001.rs`:**
```rust
//! # Problem 1: Multiples of 3 or 5
//!
//! Find sum of multiples of 3 or 5 below 1000.
//!
//! ## Mathematical Foundation
//!
//! Uses **arithmetic series** formula and **inclusion-exclusion principle**.
//! See `zettelkasten/math-foundations/arithmetic-series.md` for theory.
//! See `zettelkasten/math-foundations/inclusion-exclusion.md` for technique.
//! 
//! Full write-up: `zettelkasten/math-foundations/project-euler-p001.md`
```

**In `zettelkasten/math-foundations/arithmetic-series.md`:**
```markdown
## Rust Implementations

### Project Euler
- [[project-euler-p001]] - Sum of multiples using series formula

### Other Applications
- [[aoc-2023-day09]] - Extrapolating sequences
```

**In `zettelkasten/math-foundations/project-euler-p001.md`:**
```markdown
## Mathematical Concepts

### Primary Concepts
- **[[arithmetic-series]]** - Sum = n(first + last)/2
- **[[inclusion-exclusion]]** - Count(A∪B) = Count(A) + Count(B) - Count(A∩B)
```

---

## Naming Conventions

**Files**: `project-euler-p001.md`, `project-euler-p002.md`, etc.  
**Links**: `[[project-euler-p001]]`, `[[project-euler-p002]]`  
**Tags**: `#project-euler`, `#number-theory`, `#primes`, etc.

---

**Navigation**: [← Project Euler Main](README.md) | [Math Foundations](../zettelkasten/math-foundations/README.md)

**See Also**:
- [Math Integration Plan](../.github/MATH_INTEGRATION_PLAN.md) - Overall strategy for mathematical documentation
- [Zettelkasten Math Foundations](../zettelkasten/math-foundations/README.md) - Central math knowledge base
- [AoC Solver Template](../advent_of_code/AOC_SOLVER_TEMPLATE.md) - Similar pattern for AoC documentation

---

**Created**: 2026-01-20  
**Updated**: 2026-02-01 - Added navigation links  
**Ready to start**: January 26, 2026 with Problem 1!
