# AoC Instructions - Advent of Code Problem Solving

**Purpose**: Solve Advent of Code problems efficiently with parse-once pattern, comprehensive documentation, and performance optimization.

---

## 🎯 **Core Principles**

### **CRITICAL: Parse-Once Pattern**

**Violations detected**: Days 9 and 10 (both caught by user review)  
**Performance impact**: 40-50% slowdown when violated  
**Status**: Fundamental 2026 principle - MUST be followed from the start

**The Problem**:
```rust
// ❌ WRONG: Parses input twice
pub fn solve(input: &str) -> (usize, usize) {
    (solve_part1(input), solve_part2(input))  // Each parses internally!
}
```

**The Solution**:
```rust
// ✅ CORRECT: Parse once, share data
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);  // ← Parse ONCE
    (
        solve_part1_with_data(&data),
        solve_part2_with_data(&data),
    )
}
```

**See**: `zettelkasten/parse-once-pattern.md` for comprehensive reference

---

## 📋 **Mandatory Code Structure**

### **4-Function Pattern (Use Every Time)**

```rust
//! Day XX: [Problem Title]
//!
//! **Brief problem description**

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Clone)]
struct ParsedData {
    // Your parsed representation
}

// ============================================================================
// Parsing
// ============================================================================

fn parse_input(input: &str) -> ParsedData {
    // Parse logic here
    todo!()
}

// ============================================================================
// Part 1 Logic (Internal - accepts parsed data)
// ============================================================================

fn solve_part1_with_data(data: &ParsedData) -> usize {
    // Part 1 algorithm
    // NOTE: NO parsing happens here!
    todo!()
}

// ============================================================================
// Part 2 Logic (Internal - accepts parsed data)
// ============================================================================

fn solve_part2_with_data(data: &ParsedData) -> usize {
    // Part 2 algorithm (revealed after Part 1 complete)
    // NOTE: NO parsing happens here!
    todo!()
}

// ============================================================================
// Public API
// ============================================================================

/// Solve both parts efficiently (PARSE ONCE!)
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);  // ← Single parse
    (
        solve_part1_with_data(&data),
        solve_part2_with_data(&data),
    )
}

/// Solve Part 1 only (for testing)
pub fn solve_part1(input: &str) -> usize {
    let data = parse_input(input);
    solve_part1_with_data(&data)
}

/// Solve Part 2 only (for testing)
pub fn solve_part2(input: &str) -> usize {
    let data = parse_input(input);
    solve_part2_with_data(&data)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "...example input...";

    #[test]
    fn test_parse() {
        let data = parse_input(EXAMPLE);
        // Verify parsing is correct
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 0); // Replace 0 with expected
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 0); // Replace 0 with expected
    }

    #[test]
    fn test_part1_actual() {
        let input = include_str!("../../inputs/dayXX.txt");
        let (part1, _) = solve(input);
        assert_eq!(part1, 0); // Replace 0 with actual answer after submitting
    }

    #[test]
    fn test_part2_actual() {
        let input = include_str!("../../inputs/dayXX.txt");
        let (_, part2) = solve(input);
        assert_eq!(part2, 0); // Replace 0 with actual answer after submitting
    }

    #[test]
    fn test_both_parts_actual() {
        let input = include_str!("../../inputs/dayXX.txt");
        assert_eq!(solve(input), (0, 0)); // Replace with (part1, part2) answers
    }
}
```

### **Key Characteristics**
✅ **Internal functions** named `solve_partX_with_data()` that accept parsed data  
✅ **No parsing** in internal functions - data already prepared  
✅ **Public wrappers** for individual tests (`solve_part1()`, `solve_part2()`)  
✅ **Combined solver** parses once and calls both internal functions  
✅ **Clear sections** with banner comments  
✅ **Actual answer tests** to validate puzzle solutions remain correct

---

## ⚡ **Development Workflow**

### **Step-by-Step Process**

1. **Create skeleton** (5 min)
   ```bash
   # Copy template to dayXX.rs
   # Add to src/solver/mod.rs
   cargo test dayXX  # Should fail but compile
   ```

2. **Parse input** (10-15 min)
   ```rust
   fn parse_input(input: &str) -> ParsedData {
       // Write parsing logic
   }
   
   #[test]
   fn test_parse() {
       let data = parse_input(EXAMPLE);
       // Verify structure
   }
   ```

3. **Solve Part 1** (20-40 min)
   ```rust
   fn solve_part1_with_data(data: &ParsedData) -> usize {
       // Implement algorithm
   }
   ```
   - Test with example data
   - Run on puzzle input
   - Submit answer ⭐
   - Add `test_part1_actual()` with verified answer

4. **Solve Part 2** (20-40 min)
   ```rust
   fn solve_part2_with_data(data: &ParsedData) -> usize {
       // Extend or modify Part 1 logic
       // REUSES parsed data - no re-parsing!
   }
   ```
   - Test with example data
   - Run on puzzle input
   - Submit answer ⭐
   - Add `test_part2_actual()` and `test_both_parts_actual()` with verified answers

5. **Benchmark** (2 min)
   ```bash
   cargo bench --bench benchmarks dayXX_combined
   # Verify: combined < part1 + part2 (proves parse-once works!)
   ```

6. **Clippy** (1 min)
   ```bash
   cargo clippy -p aoc2022 --all-targets -- -D warnings
   # MUST pass with zero warnings before committing
   ```

7. **Document** (20-25 min)
   - Update `Problem_Statements/summary_2022.md` (2 min)
   - Create `Problem_Statements/days/dayXX_function_guide.md` (15-20 min)
   - Update `zettelkasten/Daily Notes/YYYY-MM-DD.md` (5 min)

**Total time**: ~90 min per day (solving + documentation)

---

## 📊 **Parse-Once Verification**

### **How to Know If You Got It Right**

#### Check 1: Code Structure
```rust
// ✅ PASS: Internal functions accept data
fn solve_part1_with_data(data: &ParsedData) -> usize { /* ... */ }
fn solve_part2_with_data(data: &ParsedData) -> usize { /* ... */ }

// ✅ PASS: Combined function parses once
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}

// ❌ FAIL: No internal functions - wrong pattern!
pub fn solve_part1(input: &str) -> usize {
    let data = parse_input(input);  // Parsing in public function
    // ... solve
}
```

#### Check 2: Benchmark Results
```bash
cargo bench dayXX

# ✅ PASS: Combined < sum of parts (parse shared)
day10_combined   7.5µs
day10_part1      7.3µs  
day10_part2      7.3µs
# 7.5µs < (7.3µs + 7.3µs) ← Parse is shared!

# ❌ FAIL: Combined ≈ sum of parts (parsing twice)
day10_combined   14.4µs
day10_part1      7.3µs
day10_part2      7.3µs
# 14.4µs ≈ (7.3µs + 7.3µs) ← Parsing twice!
```

#### Check 3: User Review
- Have user review code before committing
- Days 9 and 10 violations both caught by user
- Better to catch early than fix later!

---

## ✅ **Testing Best Practices**

### **Test Actual Puzzle Answers**

**CRITICAL**: Always add tests validating your actual submitted answers!

**Why this matters**:
- **Prevents regressions** during refactoring/optimization
- **Guards against copy-paste errors** when restructuring code
- **Validates parse-once pattern** preserves correctness
- **Enables confident code changes** with immediate verification
- **Documents the correct output** for future reference

**Example from Day 12**:
```rust
#[test]
fn test_part1_actual() {
    let input = include_str!("../../inputs/day12.txt");
    let (part1, _) = solve(input);
    assert_eq!(part1, 484);  // Actual submitted answer
}

#[test]
fn test_part2_actual() {
    let input = include_str!("../../inputs/day12.txt");
    let (_, part2) = solve(input);
    assert_eq!(part2, 478);  // Actual submitted answer
}

#[test]
fn test_both_parts_actual() {
    let input = include_str!("../../inputs/day12.txt");
    assert_eq!(solve(input), (484, 478));  // Both answers
}
```

**When to add**:
1. ✅ After submitting Part 1 answer - add `test_part1_actual()`
2. ✅ After submitting Part 2 answer - add `test_part2_actual()` + `test_both_parts_actual()`
3. ✅ Before any refactoring/optimization work
4. ✅ Before committing final solution

**Benefits demonstrated**:
- Day 12 refactoring: Sequential → Parallel → Backward BFS (3 major rewrites)
- All 3 approaches verified to give same answer (478) via tests
- Confident optimization because tests catch any breakage instantly

---

## 🔧 **Mission Integration**

### **When to Use Mission Components**

Before implementing any AoC solution, **ALWAYS scan existing missions** for reusable components:

**Mission-to-AoC Mapping Examples**:
- **Grid problems** (pathfinding, regions, areas) → Mission 6 `Grid<T>`
- **Graph traversal** (BFS/DFS, shortest paths) → Mission 8 `Graph` trait
- **Union-Find** (connected components) → Mission 10 `UnionFind`
- **HashMap/HashSet** → Mission 5 optimized implementations
- **Stack/Queue** → Mission 1/Mission 2 validated structures

**Benefits**:
✅ Proven correctness (V-Cycle validated)  
✅ Performance optimized (benchmarked)  
✅ Time efficiency (focus on problem logic)  
✅ Learning reinforcement (practical mission application)

**When to implement custom**:
- Problem requires truly novel data structure
- Mission component doesn't fit problem constraints
- Extension would be as complex as new implementation

---

## 🎯 **Performance Standards**

### **Goals**
- **Per day**: < 100ms (most days)
- **Total runtime**: < 1 second for all 25 days
- **Optimization**: Apply only if needed to meet goals

### **Optimization Checklist**
1. **Parse-once pattern** (ALWAYS - 40-50% speedup)
2. **FxHashSet/FxHashMap** for integer keys (20-30% speedup)
3. **Bitset** when domain is bounded ASCII (10-15× speedup)
4. **Rayon** for parallelizable operations (1.5-3× speedup)
5. **Pre-allocation** (`Vec::with_capacity`, `String::with_capacity`)
6. **Avoid allocations** in hot loops

### **When NOT to Optimize**
- Already under 100ms
- Complexity doesn't justify speedup
- Readability significantly impacted

---

## 📚 **Documentation Standards**

### **2+1 Structure (Streamlined from AoC 2023's 5-file approach)**

#### 1. **`summary_2022.md`** - Single source of truth (5 min update)
```markdown
| Day | Part 1 | Part 2 | Total | Algorithm | Mission | Notes |
|-----|--------|--------|-------|-----------|---------|-------|
| [10](days/day10.md) | 7.3µs | 7.3µs | ~~14.4µs~~ **7.5µs** | CPU simulation | - | **Optimized**: Parse-once (48×) · [Guide →](days/day10_function_guide.md) |
```

#### 2. **`days/dayXX_function_guide.md`** - Deep dive (15-20 min)
```markdown
# Day XX: [Title] - Function Guide

**Answer**: Part 1: `12345` | Part 2: `67890`

## Performance Benchmarks
- **Combined**: 7.5µs
- **Part 1**: ~3.8µs
- **Part 2**: ~3.7µs

## Algorithm
[Explanation with code examples]

## Optimization Journey
[If applicable - before/after comparisons]
```

#### 3. **`zettelkasten/Daily Notes/YYYY-MM-DD.md`** - Session summary (5 min)
```markdown
### Day 10: Cathode-Ray Tube (Complete + Optimized)
✅ Part 1: Answer, algorithm
✅ Part 2: Answer, algorithm
**Performance**: 14.4µs → 7.5µs (48% improvement from parse-once fix)
**Violation caught**: Parse-once principle (2nd time!)
```

---

## 🚨 **Common Mistakes (Learn from History!)**

### **Parse-Once Violations (Days 9 & 10)**

**Symptom**: `solve()` calls `solve_part1(input)` and `solve_part2(input)` directly

**Fix**: Create internal functions, parse once
```rust
// Before (WRONG)
pub fn solve(input: &str) -> (usize, usize) {
    (solve_part1(input), solve_part2(input))  // Parses twice!
}

// After (CORRECT)
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    (solve_part1_with_data(&data), solve_part2_with_data(&data))
}
```

**Performance Impact**:
- Day 9: 4% speedup (756µs → 726µs)
- Day 10: 48% speedup (14.4µs → 7.5µs)

### **Forgetting Mission Components**

**Symptom**: Reimplementing Grid/Graph/UnionFind from scratch

**Fix**: Check missions FIRST before implementing
- Mission 6: Grid utilities
- Mission 8: Graph algorithms (BFS, DFS, shortest path)
- Mission 10: Union-Find

### **Over-Engineering Parsing**

**Symptom**: Complex regex, multiple passes, allocations

**Fix**: Keep parsing simple and direct
```rust
// ✅ Good: Direct, simple
fn parse_input(input: &str) -> Vec<(Direction, i32)> {
    input.lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            (parse_direction(parts[0]), parts[1].parse().unwrap())
        })
        .collect()
}

// ❌ Overkill: Regex for simple split
```

---

## 🧪 **Testing Standards**

### **Minimum Test Coverage**

```rust
#[test]
fn test_parse() {
    let data = parse_input(EXAMPLE);
    // Verify structure/count
}

#[test]
fn test_part1_example() {
    assert_eq!(solve_part1(EXAMPLE), EXPECTED_P1);
}

#[test]
fn test_part2_example() {
    assert_eq!(solve_part2(EXAMPLE), EXPECTED_P2);
}

#[test]
fn test_combined() {
    let (p1, p2) = solve(EXAMPLE);
    assert_eq!(p1, EXPECTED_P1);
    assert_eq!(p2, EXPECTED_P2);
}
```

### **Edge Cases** (if applicable)
- Empty input
- Single element
- Large input (if performance-critical)
- Boundary conditions

---

## 📝 **Quick Reference Checklist**

Before committing any AoC day:

- [ ] **Structure**: Internal `_with_data()` functions exist
- [ ] **Parse-once**: `solve()` parses exactly once
- [ ] **Tests**: All example tests pass
- [ ] **Benchmark**: Combined < sum of parts
- [ ] **Clippy**: `cargo clippy -p aoc2022 --all-targets -- -D warnings` passes (zero warnings)
- [ ] **Mission scan**: Checked for reusable components
- [ ] **Documentation**: summary_2022.md + function_guide.md + daily note updated
- [ ] **User review**: Had code reviewed before commit (if possible)

**Remember**: Days 9 and 10 violations were both caught by user review. Better to check structure early than fix later!

---

## 🔗 **Related Resources**

**Templates**:
- `advent_of_code/aoc2022/README.md` - Full reference
- `advent_of_code/AOC_SOLVER_TEMPLATE.md` - Comprehensive guide
- `templates/aoc_documentation/` - Documentation templates

**Zettelkasten Reference**:
- `zettelkasten/parse-once-pattern.md` - Parse-once deep dive
- `zettelkasten/cpu-cycle-simulation.md` - CPU simulation patterns
- `zettelkasten/aoc-optimization-patterns.md` - Performance patterns

**Examples**:
- Day 9: Parse-once fix (4% speedup)
- Day 10: Parse-once fix (48% speedup)
- Day 9: FxHashSet optimization (20% speedup)

---

## 🎯 **Summary**

**Core Pattern**: Parse once, solve both parts  
**Why**: 40-50% speedup, cleaner code, better testing  
**How**: Internal functions accept parsed data  
**Verify**: Benchmark shows combined < sum of parts  

**Critical**: Apply parse-once pattern FROM THE START. Do not wait until after implementation!

**Mission Integration**: Scan missions BEFORE implementing. Reuse validated components when possible.

**Documentation**: 2+1 structure (summary + guide + daily note) = 25 min total

**Historical context**: Parse-once violations on Days 9 & 10. Created this instruction file to prevent future violations.

---

*Related: [[parse-once-pattern]] | [[cpu-cycle-simulation]] | [[aoc-2022-summary]] | [[mission-6]] | [[mission-8]] | [[mission-10]]*
