# Project Euler Solutions

**Start Date**: January 26, 2026  
**Goal**: Explore mathematical programming problems to deepen understanding of discrete mathematics, number theory, combinatorics, and algorithmic thinking.

## 📋 Repository Policy

**IMPORTANT**: [Project Euler requests](https://projecteuler.net/about#publish) that solutions beyond **Problem 100** not be published in public repositories.

### What's Allowed in Public Repo
- ✅ Problem statements (1-∞) - descriptions only
- ✅ Solutions (1-100) - code implementations
- ✅ Mathematical write-ups in `zettelkasten/math-foundations/` (all problems)
- ✅ Performance benchmarks (1-100)

### What's NOT Allowed
- ❌ Solution code for problems 101+ (keep private or use `.gitignore`)
- ❌ Direct spoilers in commit messages

## 🏗️ Directory Structure

```
project_euler/
├── README.md                    # This file
├── Cargo.toml                   # Rust workspace member
├── src/
│   ├── lib.rs                   # Library exports
│   ├── main.rs                  # CLI runner (optional)
│   ├── problems/
│   │   ├── mod.rs               # Problem registry
│   │   ├── p001.rs              # Problem 1 (Multiples of 3 or 5)
│   │   ├── p002.rs              # Problem 2 (Even Fibonacci)
│   │   └── ...                  # Up to p100.rs (public)
│   │   └── private/             # 101+ solutions (.gitignored)
│   └── utils/
│       ├── mod.rs               # Common utilities
│       ├── primes.rs            # Prime number utilities
│       ├── combinatorics.rs     # Factorials, combinations
│       └── number_theory.rs     # GCD, LCM, divisors, etc.
├── Problem_Statements/
│   ├── p001.md                  # Problem descriptions
│   ├── p002.md
│   └── ...
├── tests/
│   └── solutions.rs             # Verify correct answers
└── benches/
    └── benchmarks.rs            # Performance tracking
```

## 🔗 Integration with Math Foundations

**Solutions live here**, **mathematical write-ups live in zettelkasten**:

**Documentation Template**: See [MATH_DOCUMENTATION_TEMPLATE.md](MATH_DOCUMENTATION_TEMPLATE.md) for creating zettelkasten mathematical write-ups.

### Problem Solving Workflow
1. **Read problem** → `Problem_Statements/p001.md`
2. **Identify math concepts** → Set theory, number theory, graph theory, etc.
3. **Implement solution** → `src/problems/p001.rs`
4. **Document mathematics** → `zettelkasten/math-foundations/project-euler-p001.md` (using template)
5. **Bidirectional linking**:
   - Code → Math note (doc comment with link)
   - Math note → Code (implementation section with link)

### Example Bidirectional Linking

**In `src/problems/p001.rs`:**
```rust
//! # Problem 1: Multiples of 3 or 5
//!
//! ## Mathematical Foundation
//!
//! Uses **arithmetic series** and **inclusion-exclusion principle**.
//! See `zettelkasten/math-foundations/arithmetic-series.md` for theory.
```

**In `zettelkasten/math-foundations/arithmetic-series.md`:**
```markdown
## Rust Implementations

- [[Project Euler Problem 1|project_euler/src/problems/p001.rs]] - Sum multiples using series formula
```

## 🎯 Problem Template

Each problem file follows this structure:

```rust
//! # Problem N: [Title]
//!
//! [Problem description summary]
//!
//! ## Mathematical Foundation
//!
//! [Key concepts used]
//! See `zettelkasten/math-foundations/[concept].md` for theory.
//!
//! ## Approach
//!
//! [Brief explanation of algorithm]
//!
//! ## Complexity
//!
//! - Time: O(?)
//! - Space: O(?)

/// Solve Problem N
///
/// # Examples
/// ```
/// use project_euler::problems::p001::solve;
/// assert_eq!(solve(), expected_answer);
/// ```
pub fn solve() -> u64 {
    // Implementation
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Example from problem statement
    }

    #[test]
    fn test_solution() {
        assert_eq!(solve(), expected_answer);
    }
}
```

## 🧮 Mathematical Categories

Problems will be tagged by mathematical concept:

- **Number Theory**: Primes, divisors, modular arithmetic
- **Combinatorics**: Permutations, combinations, counting
- **Graph Theory**: Paths, trees, networks
- **Dynamic Programming**: Optimal substructure
- **Set Theory**: Unions, intersections, membership
- **Algebra**: Series, sequences, equations
- **Geometry**: Points, polygons, lattices

## 📊 Progress Tracking

| Category | Problems Solved | Math Notes Created |
|----------|-----------------|-------------------|
| Number Theory | 0 | 0 |
| Combinatorics | 0 | 0 |
| Graph Theory | 0 | 0 |
| Dynamic Programming | 0 | 0 |
| Set Theory | 0 | 0 |
| Algebra | 0 | 0 |
| Geometry | 0 | 0 |
| **TOTAL** | **0/100** | **0** |

## 🚀 Running Solutions

```bash
# Run specific problem
cargo run -p project_euler --bin p001

# Run tests
cargo test -p project_euler

# Run benchmarks
cargo bench -p project_euler
```

## 📚 Resources

- [Project Euler](https://projecteuler.net/) - Official site
- [OEIS](https://oeis.org/) - Online Encyclopedia of Integer Sequences
- [MathWorld](https://mathworld.wolfram.com/) - Comprehensive mathematics resource
- Zettelkasten math-foundations/ - Local knowledge base

---

**Learning Philosophy**: Use Project Euler to make **implicit mathematical knowledge explicit**. Every problem is an opportunity to document mathematical concepts, create reusable utilities, and build a comprehensive mathematics library alongside Rust mastery.
