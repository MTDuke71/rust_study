# AoC 2024 Day 7: Bridge Repair - TDD Solution Outline

## 📋 Problem Summary

**Goal**: Determine which calibration equations can be made true by inserting operators (`+`, `*`, and `||` for part 2) between numbers.

**Key Constraints**:
- Operators evaluated left-to-right (no precedence)
- Numbers cannot be rearranged
- Part 1: Only `+` and `*` operators
- Part 2: Add concatenation `||` operator

**Example**:
```
190: 10 19        → 10 * 19 = 190 ✓
3267: 81 40 27    → 81 + 40 * 27 = 3267 ✓ (left-to-right: (81+40)*27)
292: 11 6 16 20   → 11 + 6 * 16 + 20 = 292 ✓
```

## 🏗️ High-Level Architecture

### Core Components
1. **Parser**: Parse input into equation structures
2. **Evaluator**: Try all operator combinations for an equation
3. **Validator**: Check if any combination produces target value
4. **Calculator**: Sum valid equation test values

### Data Structures
```rust
#[derive(Debug, Clone, PartialEq)]
struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concatenate, // Part 2 only
}
```

## 📝 TDD Implementation Steps

### Step 1: Input Parsing (TDD)
**Objective**: Parse text input into `Equation` structs

**Test Cases to Write**:
```rust
#[test]
fn test_parse_single_equation() {
    // "190: 10 19" → Equation { test_value: 190, numbers: [10, 19] }
}

#[test]
fn test_parse_multiple_equations() {
    // Multi-line input parsing
}

#[test]
fn test_parse_edge_cases() {
    // Single number, large numbers, empty input
}
```

**Implementation Target**:
```rust
fn parse_equations(input: &str) -> Vec<Equation>
```

---

### Step 2: Expression Evaluation (TDD)
**Objective**: Evaluate expression with given operators left-to-right

**Test Cases to Write**:
```rust
#[test]
fn test_evaluate_addition() {
    // [10, 19] with [Add] → 29
}

#[test]
fn test_evaluate_multiplication() {
    // [10, 19] with [Multiply] → 190
}

#[test]
fn test_evaluate_left_to_right() {
    // [81, 40, 27] with [Add, Multiply] → (81+40)*27 = 3267
}

#[test]
fn test_evaluate_concatenation() {
    // [15, 6] with [Concatenate] → 156 (Part 2)
}

#[test]
fn test_complex_expression() {
    // [11, 6, 16, 20] with [Add, Multiply, Add] → 292
}
```

**Implementation Target**:
```rust
fn evaluate_expression(numbers: &[u64], operators: &[Operator]) -> u64
```

---

### Step 3: Operator Combination Generation (TDD)
**Objective**: Generate all possible operator combinations for N-1 positions

**Test Cases to Write**:
```rust
#[test]
fn test_generate_combinations_two_ops() {
    // 2 numbers → 1 operator position → [Add], [Multiply]
}

#[test]
fn test_generate_combinations_three_ops() {
    // 3 numbers → 2 positions → [Add,Add], [Add,Mul], [Mul,Add], [Mul,Mul]
}

#[test]
fn test_generate_combinations_part2() {
    // Include Concatenate operator in combinations
}

#[test]
fn test_combination_count() {
    // N positions with 2 operators → 2^N combinations
    // N positions with 3 operators → 3^N combinations
}
```

**Implementation Target**:
```rust
fn generate_operator_combinations(positions: usize, include_concat: bool) -> Vec<Vec<Operator>>
```

---

### Step 4: Equation Validation (TDD)
**Objective**: Check if any operator combination makes equation true

**Test Cases to Write**:
```rust
#[test]
fn test_validate_solvable_equation() {
    // 190: 10 19 → true (10 * 19 = 190)
}

#[test]
fn test_validate_unsolvable_equation() {
    // 83: 17 5 → false (17+5=22, 17*5=85, neither matches 83)
}

#[test]
fn test_validate_multiple_solutions() {
    // 3267: 81 40 27 → true (multiple valid operator combinations)
}

#[test]
fn test_validate_with_concatenation() {
    // 156: 15 6 → true with concatenation (15 || 6 = 156)
}

#[test]
fn test_validate_single_number() {
    // Edge case: equation with only one number
}
```

**Implementation Target**:
```rust
fn can_equation_be_solved(equation: &Equation, include_concat: bool) -> bool
```

---

### Step 5: Solution Calculator (TDD)
**Objective**: Sum test values of all solvable equations

**Test Cases to Write**:
```rust
#[test]
fn test_calculate_sample_part1() {
    // Sample input → sum of valid equations = 3749
}

#[test]
fn test_calculate_sample_part2() {
    // Sample input with concatenation → sum = 11387
}

#[test]
fn test_calculate_empty_input() {
    // Empty input → 0
}

#[test]
fn test_calculate_no_valid_equations() {
    // All equations unsolvable → 0
}
```

**Implementation Target**:
```rust
fn solve_part1(input: &str) -> u64
fn solve_part2(input: &str) -> u64
```

---

## 🧪 Test Data

### Sample Input
```
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
```

### Expected Results
- **Part 1**: 3749 (equations 190, 3267, 292)
- **Part 2**: 11387 (adds equations 156, 7290, 192)

### Edge Cases to Test
- Single number equations
- Very large numbers (u64 bounds)
- Long equations (many numbers)
- No valid solutions
- All operators produce same result

---

## ⚡ Performance Considerations

### Complexity Analysis
- **N numbers** → **N-1 operator positions**
- **Part 1**: 2^(N-1) combinations per equation
- **Part 2**: 3^(N-1) combinations per equation

### Optimization Opportunities (Future)
1. **Early termination**: Stop if intermediate result exceeds target
2. **Memoization**: Cache results for repeated sub-expressions
3. **Pruning**: Skip branches that can't possibly reach target
4. **Parallel processing**: Evaluate equations concurrently

### Worst Case Scenarios
- Long equations (10+ numbers) → 3^9 = 19,683 combinations
- Need efficient evaluation to avoid timeout

---

## 🔄 Implementation Workflow

### Phase 1: Foundation
1. Create basic data structures (`Equation`, `Operator`)
2. Implement and test input parsing
3. Implement and test expression evaluation

### Phase 2: Core Logic
4. Implement and test operator combination generation
5. Implement and test equation validation
6. Test against sample data

### Phase 3: Integration
7. Implement solution calculators
8. Test against known answers
9. Optimize if needed

### Phase 4: Production
10. Run against actual puzzle input
11. Verify answers match expected results
12. Add comprehensive error handling

---

## 📋 Implementation Checklist

### Data Structures
- [ ] Define `Equation` struct
- [ ] Define `Operator` enum
- [ ] Add necessary trait implementations (Debug, Clone, etc.)

### Parsing Module
- [ ] Parse single equation line
- [ ] Parse multiple equations
- [ ] Handle parsing errors gracefully
- [ ] Test with edge cases

### Evaluation Engine
- [ ] Left-to-right expression evaluation
- [ ] Support all three operators
- [ ] Handle arithmetic overflow
- [ ] Test evaluation correctness

### Combination Generator
- [ ] Generate all operator combinations
- [ ] Support both 2-operator and 3-operator modes
- [ ] Efficient enumeration algorithm
- [ ] Test combination count and correctness

### Validation Logic
- [ ] Try all combinations for equation
- [ ] Early exit on first match
- [ ] Support both parts (with/without concat)
- [ ] Test with sample equations

### Solution Integration
- [ ] Part 1 solver
- [ ] Part 2 solver
- [ ] Input file handling
- [ ] Result verification

### Testing Coverage
- [ ] Unit tests for all functions
- [ ] Integration tests with sample data
- [ ] Edge case testing
- [ ] Performance benchmarks (optional)

---

## 🎯 Success Criteria

### Functional Requirements
- ✅ Correctly parse all input formats
- ✅ Evaluate expressions left-to-right
- ✅ Generate all operator combinations
- ✅ Identify solvable equations
- ✅ Calculate correct sums for both parts

### Quality Requirements
- ✅ 100% test coverage for core functions
- ✅ All tests pass
- ✅ Code follows Rust best practices
- ✅ Clear, readable implementation
- ✅ Efficient enough for puzzle constraints

### Verification
- ✅ Sample input produces expected results (3749, 11387)
- ✅ Actual puzzle input produces correct answers
- ✅ No clippy warnings
- ✅ All tests pass with `cargo test`

---

*Ready to implement step by step with TDD approach! 🚀*

*Links: [[DAY07_TDD_COMPLETION_REPORT]] [[AoC Integration]] [[tdd-aoc-patterns]]*