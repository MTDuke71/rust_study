# Day 10: Look-and-Say Sequence - Learning Guide

This directory contains both a **skeleton** (`day10.rs`) for step-by-step learning and a **complete solution** (`day10_sol.rs`) for reference.

---

## 📚 **Files Overview**

### **day10.rs** (Skeleton - Your Workspace)
Minimal implementation with TODOs and hints. This is where you'll work!

### **day10_sol.rs** (Complete Solution - Reference)
Full working implementation. Check this when you're stuck or want to compare approaches.

---

## 🎯 **Learning Objectives**

By implementing this solution, you'll learn:
1. **String manipulation** - Working with characters and building strings
2. **Run-length encoding** - A common compression/transformation algorithm
3. **Iteration patterns** - Counting consecutive elements
4. **Edge case handling** - Empty strings, single characters
5. **Test-driven development** - Using tests to guide implementation

---

## 🚀 **Step-by-Step Guide**

### **Step 1: Understand the Problem**

The look-and-say sequence reads the previous term and describes what you see:

```
"1"      → "11"        (one 1)
"11"     → "21"        (two 1s)
"21"     → "1211"      (one 2, one 1)
"1211"   → "111221"    (one 1, one 2, two 1s)
"111221" → "312211"    (three 1s, two 2s, one 1)
```

### **Step 2: Implement `look_and_say()` Function**

Start with the edge cases and work your way up:

#### **2a. Handle Empty String**
```bash
cargo test day10::tests::test_look_and_say_empty
```

**Hint**: Check if the input is empty and return early.

#### **2b. Handle Single Character**
```bash
cargo test day10::tests::test_look_and_say_single
```

**Expected**: `look_and_say("5")` should return `"15"` (one 5)

**Algorithm Outline**:
```rust
pub fn look_and_say(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    
    if chars.is_empty() {
        return result;
    }
    
    let mut i = 0;
    while i < chars.len() {
        let current_char = chars[i];
        let mut count = 1;
        
        // TODO: Count consecutive occurrences of current_char
        // while next char exists AND equals current_char {
        //     count += 1;
        // }
        
        // TODO: Append count and character to result
        // result.push_str(&count.to_string());
        // result.push(current_char);
        
        // TODO: Move index forward by count
        // i += count;
    }
    
    result
}
```

#### **2c. Test Basic Examples**
```bash
cargo test day10::tests::test_look_and_say_examples
```

This will test the 5 examples from the problem statement.

#### **2d. Test Multiple Groups**
```bash
cargo test day10::tests::test_look_and_say_multiple_groups
```

**Example**: `"1223"` has 3 groups: one `1`, two `2`s, one `3` → `"112213"`

### **Step 3: Implement `solve_part1()`**

Once `look_and_say()` works, implement the solution:

```rust
pub fn solve_part1(input: &str) -> Result<String> {
    let start_sequence = input.trim();
    let mut current = start_sequence.to_string();
    
    for _ in 0..40 {
        current = look_and_say(&current);
    }
    
    Ok(current.len().to_string())
}
```

**Test**: `cargo run -- 10`

### **Step 4: Implement `solve_part2()`**

Same as Part 1, but iterate 50 times instead of 40.

---

## 🧪 **Testing Workflow**

### **Run Individual Tests**
```bash
# Test empty string handling
cargo test day10::tests::test_look_and_say_empty

# Test single character
cargo test day10::tests::test_look_and_say_single

# Test basic examples
cargo test day10::tests::test_look_and_say_examples

# Test multiple groups
cargo test day10::tests::test_look_and_say_multiple_groups
```

### **Run All Day 10 Tests**
```bash
cargo test day10
```

### **Run the Full Solution**
```bash
cargo run -- 10
```

---

## 💡 **Key Concepts**

### **Run-Length Encoding**
The look-and-say sequence is a form of run-length encoding:
- Scan through sequence
- Count consecutive identical elements
- Output: count + element

### **Consecutive Counting Pattern**
```rust
let mut i = 0;
while i < chars.len() {
    let current = chars[i];
    let mut count = 1;
    
    // Count consecutive occurrences
    while i + count < chars.len() && chars[i + count] == current {
        count += 1;
    }
    
    // Process the group (current appears 'count' times)
    // ...
    
    // Move to next group
    i += count;
}
```

### **String Building in Rust**
```rust
let mut result = String::new();
result.push_str("42");      // Append string
result.push('X');           // Append single char
result.to_string()          // Convert to owned String
```

---

## 🎓 **Expected Results**

When your implementation is complete:
- ✅ All 4 tests pass
- ✅ Part 1 (40 iterations): `492982`
- ✅ Part 2 (50 iterations): `6989950`

---

## 🔍 **Common Pitfalls**

1. **Off-by-one errors** when counting consecutive characters
2. **Forgetting to advance the index** by the count
3. **Not handling empty strings** (edge case)
4. **Infinite loops** if index isn't advanced properly

---

## 📖 **Reference Solution**

If you get stuck, check `day10_sol.rs` for the complete working implementation.

You can also run the solution tests:
```bash
cargo test day10_sol
```

---

## 🔗 **Related Resources**

- **Mission5 Integration**: See `examples/day10_with_memo.rs` for memoization approach
- **Run-Length Encoding**: Classic compression algorithm used in image formats
- **AoC Patterns**: This problem teaches sequence transformation patterns

---

## 🎯 **Challenge Yourself**

After completing the basic solution:

1. **Optimize**: Can you reduce memory allocations?
2. **Analyze**: Why does the sequence grow so rapidly?
3. **Extend**: Can you find a pattern in sequence lengths?
4. **Compare**: How does memoization help? (See `day10_with_memo.rs`)

---

**Happy Coding!** 🎄

Remember: The goal is learning, not speed. Take your time, test incrementally, and refer to `day10_sol.rs` when needed.
