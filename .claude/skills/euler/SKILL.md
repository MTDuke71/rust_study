---
name: euler
description: Solve Project Euler problems with mathematical rigor, shared utility libraries, performance optimization, and zettelkasten math integration
---

# Project Euler Instructions - Mathematical Problem Solving

**Purpose**: Solve Project Euler problems combining mathematical insight with efficient Rust implementations, building shared utility libraries and connecting to the math zettelkasten layer.

---

## Project Euler Philosophy

Project Euler is the **"Double Helix" math track** where:
- Mathematical understanding drives algorithm selection
- Shared utility libraries (`utils/`) grow with each problem
- Performance matters: brute force is the baseline, not the goal
- Math zettelkasten notes connect theory to implementation
- Problems build on each other (primes, combinatorics, number theory)

**Core Principle**: "Understand the math first, then write efficient Rust."

---

## Codebase Structure

```
project_euler/
├── Cargo.toml
├── src/
│   ├── main.rs                 # CLI runner (solve by problem number)
│   ├── lib.rs                  # Public API
│   ├── problems/
│   │   ├── mod.rs              # Problem module declarations
│   │   ├── registry.rs         # ProblemSolver dispatch (add new problems here!)
│   │   ├── p001.rs             # Problem implementations
│   │   ├── p002.rs
│   │   └── ...
│   └── utils/
│       ├── mod.rs              # Utility module declarations
│       ├── primes.rs           # Prime generation, sieve, primality testing
│       ├── number_theory.rs    # GCD, LCM, divisors, modular arithmetic
│       └── combinatorics.rs    # Permutations, combinations, factorials
├── tests/
│   └── solutions.rs            # Verified answer regression tests
└── benches/
    └── benchmarks.rs           # Criterion benchmarks
```

---

## Problem File Template

```rust
//! # Problem XXX: [Problem Title]
//!
//! [Problem description from projecteuler.net]
//!
//! ## Mathematical Foundation
//!
//! [Key math concepts: what theorem/identity/technique applies]
//! [Link to zettelkasten: See [[math-foundations/relevant-note]]]
//!
//! ## Approach
//!
//! [Algorithm description: why this approach, not brute force]
//!
//! ## Complexity
//!
//! - Time: O(...)
//! - Space: O(...)

use crate::utils::primes; // Reuse shared utilities!

/// [Public function with doctest]
///
/// # Examples
/// ```
/// use project_euler::problems::pXXX::some_function;
/// assert_eq!(some_function(10), 23);
/// ```
pub fn some_function(n: u64) -> u64 {
    // Implementation
    todo!()
}

/// Solve Problem XXX with default parameters
pub fn solve() -> u64 {
    some_function(DEFAULT_PARAM)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_from_problem() {
        // Example given in problem statement
        assert_eq!(some_function(10), 23);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(some_function(0), 0);
        assert_eq!(some_function(1), 0);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), EXPECTED_ANSWER);
    }
}
```

---

## Development Workflow

### Step 1: Read & Understand the Math (5-10 min)
- Read problem statement carefully
- Identify mathematical domain (number theory, combinatorics, geometry, etc.)
- Check if related math zettelkasten note exists
- Consider: Is there a closed-form solution? A known algorithm?

### Step 2: Check Shared Utilities (2 min)
- **Primes needed?** → `utils/primes.rs` (sieve, nth_prime, is_prime)
- **Number theory?** → `utils/number_theory.rs` (GCD, LCM, divisors)
- **Combinatorics?** → `utils/combinatorics.rs` (permutations, combinations)
- **New utility needed?** → Add to appropriate utils file for reuse

### Step 3: Implement Solution (15-30 min)
- Start with the example from the problem statement as first test
- Implement the mathematical approach, not brute force
- Use shared utilities, don't reinvent
- Add doctests for public functions

### Step 4: Verify & Optimize (5-10 min)
```bash
cargo test -p project_euler pXXX           # Tests pass
cargo clippy -p project_euler -- -D warnings  # Zero warnings
cargo bench --bench benchmarks pXXX        # Performance check
```

### Step 5: Register & Document (10 min)
- Add to `registry.rs` match arm and `implemented()` list
- Add regression test to `tests/solutions.rs`
- Create Problem Statement + Zettelkasten notes (see Documentation below)

---

## Registration Checklist (Don't Forget!)

When adding problem pXXX:

1. **Create** `src/problems/pXXX.rs`
2. **Add module** to `src/problems/mod.rs`: `pub mod pXXX;`
3. **Register** in `src/problems/registry.rs`:
   - Add match arm: `XXX => Some(super::pXXX::solve())`
   - Add to `implemented()` vec
4. **Add regression test** to `tests/solutions.rs`:
   ```rust
   #[test]
   fn test_problem_XXX() {
       assert_eq!(pXXX::solve(), ANSWER);
   }
   ```
5. **Add benchmark** to `benches/benchmarks.rs` (if non-trivial timing)

---

## Utility Library Growth Strategy

### When to Add to Utils
- Function used by 2+ problems → extract to utils
- General mathematical operation (not problem-specific)
- Performance-critical code worth optimizing once

### Current Utilities
- **`primes.rs`**: Sieve of Eratosthenes, primality testing, prime generation
- **`number_theory.rs`**: GCD, LCM, divisor counting, modular arithmetic
- **`combinatorics.rs`**: Permutations, combinations, factorial

### Growing the Library
When you find yourself writing a general math function:
1. Implement in the problem file first
2. Add tests proving correctness
3. Extract to appropriate utils module
4. Add doctests with examples
5. Update existing problems to use the shared version

---

## Performance Standards

- **Target**: Most problems should solve in < 100ms
- **Acceptable**: < 1 second for computationally intensive problems
- **Optimization priority**:
  1. Better algorithm (math insight beats brute force)
  2. Memoization / caching for overlapping subproblems
  3. Sieve-based approaches for prime/divisor problems
  4. Iterator chains over collected vectors
  5. Bit manipulation for combinatorial problems

---

## Documentation: Two-File Pattern (MANDATORY)

Every problem gets **two** documentation files that separate concerns:

### File 1: `Problem_Statements/pXXX.md` (Lightweight)
**Purpose**: Problem statement only + links to detailed analysis. NO algorithm details here.

```markdown
# Problem XXX: [Title]

**Difficulty**: X%
**Category**: [Math Domain(s)]

## Problem Statement
[Exact problem description from projecteuler.net]

## Examples
[Examples from problem statement]

## Constraints
[Input constraints and bounds]

## Detailed Analysis
See [Mathematical Analysis](../../zettelkasten/math-foundations/project-euler-pXXX.md) for:
- [Key math concept 1]
- [Key math concept 2]
- [Optimization details]

## Source
[Project Euler Problem XXX](https://projecteuler.net/problem=XXX)

---
**Navigation**: [← Problem XXX-1](pXXX-1.md) | [All Problems](README.md) | [Problem XXX+1 →](pXXX+1.md)
```

### File 2: `zettelkasten/math-foundations/project-euler-pXXX.md` (Detailed)
**Purpose**: Full mathematical analysis, approach comparison, Rust implementation details, bidirectional links.

Must include:
- **Problem Statement** (brief)
- **Mathematical Foundation** (theorems, identities, proofs)
- **Approach** (algorithm with complexity analysis)
- **Rust Implementation Details** (key code patterns)
- **Related Problems** (Project Euler + Mission connections)
- **Bidirectional Links** (to concept notes like combinatorics-fundamentals, etc.)

### File 3: Update existing concept note
If the problem uses a math concept that has a zettelkasten note (e.g., `combinatorics-fundamentals.md`),
add the problem as an implementation example in that note.

### Why This Split?
- **Problem_Statements/**: Quick reference, "what does the problem ask?"
- **math-foundations/project-euler-pXXX.md**: Deep analysis, "how and why does the solution work?"
- **math-foundations/[concept].md**: Growing theory library, connects across problems

## Math Zettelkasten Integration

**Every new mathematical concept encountered should connect to `zettelkasten/math-foundations/`**

### When to Create/Update Math Notes
- New theorem or identity used in solution
- Mathematical technique applied for first time
- Connection between problems reveals deeper pattern

### Linking Pattern
```markdown
# In problem file header:
//! See [[math-foundations/combinatorics-fundamentals]] for binomial coefficient theory

# In zettelkasten concept note:
## Implementations
- [[project-euler-p015]]: Lattice paths via central binomial coefficient C(40,20)
```

---

## Common Patterns Across Problems

| Pattern | Problems | Utility |
|---------|----------|---------|
| Prime sieve | p003, p005, p007, p010 | `utils/primes.rs` |
| Divisor counting | p012 | `utils/number_theory.rs` |
| Large number products | p008, p011 | Iterator chains |
| Pythagorean triplets | p009 | Algebraic constraints |
| Memoization/DP | p014 | Problem-specific caches |
| Digit manipulation | p004, p008 | String/math conversion |

---

## Quick Reference Checklist

Before committing a Project Euler solution:

- [ ] **Math**: Mathematical approach documented in module header
- [ ] **Example test**: Problem statement example passes
- [ ] **Answer test**: Verified answer in `tests/solutions.rs`
- [ ] **Registration**: Added to `registry.rs` (match arm + implemented list)
- [ ] **Utilities**: Shared functions used where available, new ones extracted if general
- [ ] **Clippy**: `cargo clippy -p project_euler -- -D warnings` (zero warnings)
- [ ] **Performance**: Reasonable execution time (< 100ms target)
- [ ] **Problem Statement**: `Problem_Statements/pXXX.md` created (statement + links only)
- [ ] **Analysis Note**: `zettelkasten/math-foundations/project-euler-pXXX.md` created (detailed)
- [ ] **Concept Note**: Existing math concept note updated with problem reference
