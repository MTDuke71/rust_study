# Parse-Once Pattern: Fundamental AoC Optimization Principle

*Tags: #aoc #optimization #performance #code-structure #fundamental-principle #2026-standard*  
*Links: [[zettel-index]] | [[aoc-2022-summary]] | [[Performance Optimization]] | [[Code Patterns]] | [[daily-study/Day09]] | [[daily-study/Day10]]*

---

## 🎯 Overview

The **Parse-Once Pattern** is a fundamental optimization principle for Advent of Code (2026): parse the input string **exactly once** and share the parsed data between Part 1 and Part 2 solutions. This eliminates duplicate parsing overhead and ensures clean code structure.

**Performance Impact**: Typical speedup of 40-50% when input parsing is non-trivial.

**Critical Principle**: Established as a 2026 standard after violations on Days 9 and 10 (both caught by user review).

---

## ❌ Common Violation Pattern (AVOID!)

```rust
// ❌ BAD: Parses input twice (once per part)
pub fn solve_part1(input: &str) -> usize {
    let data = parse_input(input);  // Parse #1
    // ... solve Part 1
}

pub fn solve_part2(input: &str) -> usize {
    let data = parse_input(input);  // Parse #2 (DUPLICATE!)
    // ... solve Part 2
}

pub fn solve(input: &str) -> (usize, usize) {
    // This calls parse_input() TWICE!
    (solve_part1(input), solve_part2(input))
}
```

**Symptoms of violation**:
- `solve()` function calls `solve_part1(input)` and `solve_part2(input)` directly
- Each public function starts with `let data = parse_input(input);`
- No internal functions that accept pre-parsed data
- Benchmark shows `combined` time ≈ `part1_time + part2_time` (should be less due to shared parse)

---

## ✅ Correct Pattern (USE THIS!)

```rust
// ✅ GOOD: Internal functions accept pre-parsed data
fn solve_part1_with_data(data: &ParsedType) -> usize {
    // Work with already-parsed data
    // No parsing happens here!
}

fn solve_part2_with_data(data: &ParsedType) -> usize {
    // Work with already-parsed data
    // No parsing happens here!
}

// Public wrappers for individual tests
pub fn solve_part1(input: &str) -> usize {
    let data = parse_input(input);
    solve_part1_with_data(&data)
}

pub fn solve_part2(input: &str) -> usize {
    let data = parse_input(input);
    solve_part2_with_data(&data)
}

// Combined function - PARSE ONCE!
pub fn solve(input: &str) -> (usize, usize) {
    let data = parse_input(input);  // ← Single parse
    (
        solve_part1_with_data(&data),
        solve_part2_with_data(&data),
    )
}
```

**Key characteristics**:
- Internal `_with_data()` functions that accept parsed data
- Public functions parse and delegate to internal functions
- `solve()` combines both parts with **single parse**
- Clean separation of parsing from solving logic

---

## 📊 Performance Impact

### Real AoC 2022 Examples

#### Day 9: Rope Bridge
```
Before (parse twice):  756µs
After (parse once):    726µs
Improvement:           -4.0% (30µs saved)
```

#### Day 10: Cathode-Ray Tube
```
Before (parse twice):  14.4µs
After (parse once):     7.5µs  
Improvement:           -48% (6.9µs saved, nearly 2× faster!)
```

**Why such variation?**
- Day 9: Complex logic dominates, parsing is small fraction
- Day 10: Simple logic, parsing is ~50% of runtime
- **General rule**: Bigger speedup when parsing is more expensive relative to solving

---

## 🔍 When Parse-Once Matters Most

### High Impact (Large Speedup Expected)
- String splitting/tokenization (`.split()`, `.lines()`)
- Regex matching and capture groups
- Number parsing (`.parse::<i32>()` in loops)
- Complex structure building (HashMaps, Vecs, custom structs)
- Multi-pass parsing (parse → validate → convert)

### Medium Impact
- Simple iteration over lines/chars
- ASCII byte manipulation
- Lookup table construction

### Low Impact (Still Do It for Principle!)
- Direct byte/char access without allocation
- Single-pass iteration with no conversion
- Pre-computed constant data

**Critical**: Apply the pattern **regardless of impact**. It's about code structure and consistency, not just optimization.

---

## 🛠️ Implementation Checklist

When solving a new AoC day, follow this checklist:

- [ ] **Step 1**: Write parsing function first
  ```rust
  fn parse_input(input: &str) -> ParsedType { /* ... */ }
  ```

- [ ] **Step 2**: Create internal solving functions
  ```rust
  fn solve_part1_with_data(data: &ParsedType) -> Answer { /* ... */ }
  fn solve_part2_with_data(data: &ParsedType) -> Answer { /* ... */ }
  ```

- [ ] **Step 3**: Create public wrappers (for tests)
  ```rust
  pub fn solve_part1(input: &str) -> Answer {
      let data = parse_input(input);
      solve_part1_with_data(&data)
  }
  ```

- [ ] **Step 4**: Combine with single parse
  ```rust
  pub fn solve(input: &str) -> (Answer, Answer) {
      let data = parse_input(input);
      (solve_part1_with_data(&data), solve_part2_with_data(&data))
  }
  ```

- [ ] **Step 5**: Verify in benchmark
  - Run `cargo bench day{N}_combined`
  - Ensure `combined_time < part1_time + part2_time`
  - If not, you're parsing twice!

---

## 🎓 Lessons Learned (Historical Context)

### Day 9 Violation (2026-02-09)
**What happened**: Initial implementation called `solve_part1(input)` and `solve_part2(input)` directly  
**How caught**: User noticed during code review  
**Impact**: 4% speedup after fix (756µs → 726µs)  
**Lesson**: Pattern must be applied from the start, not added later

### Day 10 Violation (2026-02-10)
**What happened**: **Same mistake repeated** - forgot the lesson from Day 9!  
**How caught**: User immediately noticed the violation  
**Impact**: 48% speedup after fix (14.4µs → 7.5µs)  
**Lesson**: **This is the SECOND time!** Need to make this pattern automatic/instinctive

**Action taken**: Created this zettelkasten page to document the pattern formally and prevent future violations.

---

## 🔗 Related Patterns

### Parsing Strategies
- **Eager parsing**: Parse everything upfront (most common for AoC)
- **Lazy parsing**: Parse on-demand (rare, useful for huge inputs)
- **Memoized parsing**: Cache parse results (overkill for AoC)

### Code Structure
- **Separation of concerns**: Parsing vs. solving logic
- **Testability**: Internal functions easy to test with mock data
- **Reusability**: Parsed data can be used for additional analysis

### Performance
- **Allocation reduction**: Parse once = allocate once
- **Cache efficiency**: Parsed data stays hot in CPU cache
- **Benchmark accuracy**: Clear separation makes profiling easier

---

## 🧪 Testing the Pattern

### How to Verify You're Doing It Right

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_once_used() {
        let input = "test input";
        let parsed = parse_input(input);
        
        // Both functions can use same parsed data
        let p1 = solve_part1_with_data(&parsed);
        let p2 = solve_part2_with_data(&parsed);
        
        // Should match public API
        assert_eq!(p1, solve_part1(input));
        assert_eq!(p2, solve_part2(input));
        assert_eq!((p1, p2), solve(input));
    }
}
```

### Benchmark Validation

```bash
# Run benchmarks for a day
cargo bench day10

# Combined should be LESS than sum of parts
# ✅ Good: combined=7.5µs, part1=7.3µs, part2=7.3µs (parse shared!)
# ❌ Bad:  combined=14.4µs, part1=7.3µs, part2=7.3µs (parsing twice!)
```

---

## 📝 Summary

**Core Principle**: Parse input exactly once per `solve()` call.

**Why it matters**:
1. **Performance**: 5-50% speedup depending on parsing cost
2. **Code clarity**: Clean separation of parsing vs. solving
3. **Maintainability**: Easy to modify parsing without touching logic
4. **Testability**: Internal functions testable with mock data

**How to apply**:
1. Create internal `_with_data()` functions
2. Public functions parse and delegate
3. `solve()` parses once and calls both internal functions

**Verification**:
- Review code structure: Does `solve()` parse twice?
- Check benchmarks: Is combined time < sum of parts?
- User review: Have someone else verify before committing

**Historical violations**: Days 9 and 10 (both caught by user review)

**Moving forward**: Apply this pattern **FROM THE START** on every future AoC day!

---

*Related Links: [[2026-02-09]] | [[2026-02-10]] | [[aoc-optimization-patterns]] | [[Performance Optimization]] | [[Code Structure Patterns]]*

*Backlinks: [[daily-study/Day09]] | [[daily-study/Day10]]*
