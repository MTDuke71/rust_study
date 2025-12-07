# Edge Case Testing: Boundary Conditions and Corner Cases

*Tags: #testing #edge-cases #boundary-testing #quality-assurance #defensive-programming*  
*Links: [[zettel-index]] | [[test-pyramid]] | [[Unit Testing]] | [[Integration Testing]] | [[TDD (Test-Driven Development)]] | [[V-Cycle Methodology]]*

---

## 🎯 Overview

**Edge Case Testing** focuses on validating software behavior at the boundaries of input domains, unusual states, and corner cases where bugs commonly lurk. These tests catch issues that normal "happy path" testing misses.

---

## 📚 Categories of Edge Cases

### **1. Empty/Zero Cases**
The most fundamental boundary - what happens with nothing?

```rust
#[test]
fn req1_empty_input_returns_valid_result() {
    let validator = BracketValidator::new();
    // Empty string is valid (no unmatched brackets)
    assert!(validator.validate("").is_ok());
}

#[test]
fn req2_empty_stack_pop_returns_none() {
    let mut stack: Stack<i32> = Stack::new();
    assert_eq!(stack.pop(), None);  // Not panic!
}

#[test]
fn req3_zero_capacity_handled_gracefully() {
    let mut vec: Vec<i32> = Vec::with_capacity(0);
    vec.push(1);  // Should auto-grow, not crash
    assert_eq!(vec.len(), 1);
}
```

### **2. Single Element Cases**
The simplest non-empty case - often reveals off-by-one errors:

```rust
#[test]
fn single_element_push_pop() {
    let mut stack = Stack::new();
    stack.push(42);
    assert_eq!(stack.pop(), Some(42));
    assert_eq!(stack.pop(), None);  // Now empty
}

#[test]
fn single_char_bracket_validation() {
    let validator = BracketValidator::new();
    assert!(validator.validate("(").is_err());  // Unclosed
    assert!(validator.validate(")").is_err());  // Unexpected closer
    assert!(validator.validate("a").is_ok());   // Non-bracket OK
}
```

### **3. Maximum/Overflow Cases**
Testing limits and potential overflow conditions:

```rust
#[test]
fn large_input_handles_gracefully() {
    let large_input = "(".repeat(10_000) + &")".repeat(10_000);
    let validator = BracketValidator::new();
    // Should complete without stack overflow or timeout
    assert!(validator.validate(&large_input).is_ok());
}

#[test]
fn usize_max_index_handling() {
    // Test that position tracking doesn't overflow
    let error = BracketError::unexpected_closing(')', usize::MAX);
    assert_eq!(error.position(), usize::MAX);
}
```

### **4. Boundary Transitions**
Testing behavior right at state transitions:

```rust
#[test]
fn exact_capacity_boundary() {
    let mut stack = Stack::with_capacity(4);
    for i in 0..4 {
        stack.push(i);
    }
    assert_eq!(stack.len(), 4);
    
    // One more triggers reallocation
    stack.push(4);
    assert_eq!(stack.len(), 5);
}

#[test]
fn nested_depth_boundary() {
    // Testing at max reasonable nesting depth
    let deep_nesting = "(".repeat(100) + &")".repeat(100);
    assert!(validator.validate(&deep_nesting).is_ok());
}
```

---

## 🔧 Rust-Specific Edge Cases

### **UTF-8 and String Boundaries**

```rust
#[test]
fn unicode_character_boundaries() {
    let input = "（）";  // Full-width brackets (3 bytes each)
    // char_indices() gives byte positions, not char positions
    for (byte_idx, ch) in input.char_indices() {
        println!("Byte {}: '{}'", byte_idx, ch);
    }
    // Output: Byte 0: '（', Byte 3: '）'
}

#[test]
fn empty_string_vs_none() {
    let empty: &str = "";
    let result: Option<&str> = Some("");
    
    assert!(empty.is_empty());
    assert!(result.is_some());  // Some("") != None
}
```

### **Option and Result Edge Cases**

```rust
#[test]
fn option_unwrap_or_default_edge_cases() {
    let none: Option<Vec<i32>> = None;
    let empty_vec = none.unwrap_or_default();
    assert!(empty_vec.is_empty());  // Not None, but empty
}

#[test]
fn result_chain_edge_cases() {
    fn may_fail(input: &str) -> Result<usize, &str> {
        if input.is_empty() {
            Err("empty input")
        } else {
            Ok(input.len())
        }
    }
    
    // Edge case: and_then on Err
    let result = may_fail("")
        .and_then(|len| Ok(len * 2));
    assert!(result.is_err());  // Short-circuits
}
```

### **Iterator Edge Cases**

```rust
#[test]
fn iterator_edge_cases() {
    let empty: Vec<i32> = vec![];
    
    // next() on empty iterator
    assert_eq!(empty.iter().next(), None);
    
    // fold on empty iterator
    let sum: i32 = empty.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 0);  // Returns initial value
    
    // collect empty iterator
    let collected: Vec<i32> = empty.iter().cloned().collect();
    assert!(collected.is_empty());
}
```

---

## 📋 Edge Case Checklist

Use this checklist when designing tests:

### **Numeric Boundaries**
- [ ] Zero / empty
- [ ] One / single element
- [ ] Maximum value (`usize::MAX`, `i32::MAX`)
- [ ] Minimum value (`i32::MIN`)
- [ ] Negative numbers (where applicable)
- [ ] Powers of 2 (capacity boundaries)

### **String Boundaries**
- [ ] Empty string `""`
- [ ] Single character
- [ ] Multi-byte UTF-8 characters
- [ ] Very long strings
- [ ] Strings with only whitespace

### **Collection Boundaries**
- [ ] Empty collection
- [ ] Single element
- [ ] Exact capacity
- [ ] Just over capacity (triggers reallocation)
- [ ] Duplicate elements

### **State Transitions**
- [ ] Initial state
- [ ] After single operation
- [ ] After returning to initial state
- [ ] After error recovery

---

## 🔗 Integration with V-Cycle

In V-Cycle methodology, edge case tests map to specific requirements:

| Requirement | Edge Cases to Test |
|-------------|-------------------|
| REQ-1: Input handling | Empty input, single char, max length |
| REQ-2: Matching rules | All bracket types, mixed brackets |
| REQ-3: Error detection | First position, last position, middle |
| REQ-4: Complexity | Single element O(1), max input O(n) |

```rust
// Naming convention: req{N}_edge_{description}
#[test]
fn req1_edge_empty_input() { /* ... */ }

#[test]
fn req3_edge_error_at_position_zero() { /* ... */ }

#[test]
fn req4_edge_single_element_constant_time() { /* ... */ }
```

---

## 📖 Related Concepts

### Testing Strategies
- [[test-pyramid]] - Balancing test types
- [[proptest-property-based-testing]] - Automated edge case discovery
- [[Unit Testing]] - Component-level testing
- [[Integration Testing]] - Cross-component boundaries

### Rust Patterns
- [[Error Handling]] - Result and Option edge cases
- [[Iterator Patterns]] - Empty iterator behavior
- [[UTF-8 Handling]] - String boundary conditions

### Quality Assurance
- [[V-Cycle Methodology]] - Requirements traceability for edge cases
- [[mission-1]] - Stack edge cases (empty pop, single push/pop)
- [[advanced_examples/Brackets_Basic/README|Brackets Basic]] - Bracket validation edge cases

---

*Links: [[zettel-index]] | [[test-pyramid]] | [[TDD (Test-Driven Development)]] | [[V-Cycle Methodology]] | [[mission-1]]*

*Tags: #testing #edge-cases #boundary-testing #quality-assurance #rust #defensive-programming*
