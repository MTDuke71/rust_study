# 🔐 Day 11: Corporate Policy - Implementation Guide

## 📋 **Problem Summary**

Generate valid passwords by incrementing strings like base-26 counting until all validation rules pass.

**Rules:**
1. ✅ Must have increasing straight of 3+ letters (abc, bcd, xyz)
2. ❌ Must NOT contain 'i', 'o', or 'l'
3. ✅ Must have at least 2 different, non-overlapping pairs (aa, bb)

---

## 🎯 **Step-by-Step Implementation Plan**

### **Phase 1: Base-26 Incrementing (Easiest)**

#### **Function**: `increment_password(password: &str) -> String`

**Goal**: Increment like counting - "xx" → "xy" → "xz" → "ya"

**Algorithm**:
```
1. Convert string to Vec<char>
2. Start from rightmost character
3. Loop backwards:
   - If char < 'z': increment and return
   - If char == 'z': set to 'a' and continue (carry)
4. Convert back to String
```

**Test Cases**:
```rust
increment_password("xx") == "xy"
increment_password("xz") == "ya"   // Wrap!
increment_password("azz") == "baa" // Double wrap!
```

**Implementation Template**:
```rust
pub fn increment_password(password: &str) -> String {
    let mut chars: Vec<char> = password.chars().collect();
    
    // Start from rightmost character
    for i in (0..chars.len()).rev() {
        if chars[i] != 'z' {
            // Simple increment: 'a' → 'b', 'b' → 'c', etc.
            chars[i] = ((chars[i] as u8) + 1) as char;
            break;  // Done!
        } else {
            // Wrap 'z' → 'a' and carry to next position
            chars[i] = 'a';
            // Continue loop to carry left
        }
    }
    
    chars.into_iter().collect()
}
```

**Testing**:
```bash
cargo test test_increment
```

---

### **Phase 2: Rule Validation Functions**

#### **Function 1**: `has_no_forbidden_chars(password: &str) -> bool`

**Goal**: Return `true` if NO 'i', 'o', or 'l' present

**Easiest Implementation**:
```rust
pub fn has_no_forbidden_chars(password: &str) -> bool {
    !password.contains('i') && !password.contains('o') && !password.contains('l')
}
```

**Alternative** (slightly faster):
```rust
pub fn has_no_forbidden_chars(password: &str) -> bool {
    !password.chars().any(|c| c == 'i' || c == 'o' || c == 'l')
}
```

**Test**:
```bash
cargo test test_no_forbidden
cargo test test_has_forbidden
```

---

#### **Function 2**: `has_increasing_straight(password: &str) -> bool`

**Goal**: Find 3 consecutive increasing letters (abc, bcd, xyz)

**Algorithm**:
```
1. Check every 3-character window
2. For each window: char[i+1] == char[i]+1 && char[i+2] == char[i]+2
3. Return true if any window is a straight
```

**Implementation Approach 1** (Using bytes for arithmetic):
```rust
pub fn has_increasing_straight(password: &str) -> bool {
    let bytes = password.as_bytes();
    
    for i in 0..bytes.len().saturating_sub(2) {
        if bytes[i + 1] == bytes[i] + 1 && bytes[i + 2] == bytes[i] + 2 {
            return true;
        }
    }
    
    false
}
```

**Implementation Approach 2** (Using windows):
```rust
pub fn has_increasing_straight(password: &str) -> bool {
    let chars: Vec<char> = password.chars().collect();
    
    chars.windows(3).any(|window| {
        (window[1] as u8) == (window[0] as u8) + 1 &&
        (window[2] as u8) == (window[1] as u8) + 1
    })
}
```

**Test**:
```bash
cargo test test_has_straight
cargo test test_no_straight
```

---

#### **Function 3**: `has_two_pairs(password: &str) -> bool`

**Goal**: Find at least 2 different, **non-overlapping** pairs

**Tricky Part**: Non-overlapping means "aaa" = 1 pair, "aaaa" = 2 pairs

**Algorithm**:
```
1. Iterate through characters
2. When you find a pair (char[i] == char[i+1]):
   - Increment counter
   - Skip next character (i += 2) to avoid overlap
3. Return counter >= 2
```

**Implementation**:
```rust
pub fn has_two_pairs(password: &str) -> bool {
    let chars: Vec<char> = password.chars().collect();
    let mut pair_count = 0;
    let mut i = 0;
    
    while i < chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            pair_count += 1;
            i += 2;  // Skip next to avoid overlap
        } else {
            i += 1;
        }
    }
    
    pair_count >= 2
}
```

**Edge Cases to Test**:
- "aaa" → 1 pair ❌
- "aaaa" → 2 pairs ✅
- "aabb" → 2 pairs ✅
- "aabaa" → 2 pairs ✅

**Test**:
```bash
cargo test test_has_two_pairs
cargo test test_not_enough_pairs
```

---

### **Phase 3: Full Validation**

#### **Function**: `is_valid_password(password: &str) -> bool`

**Goal**: All three rules must pass

**Optimization Tip**: Check cheapest/most-likely-to-fail rules first!

```rust
pub fn is_valid_password(password: &str) -> bool {
    // Rule 2: Most likely to fail, cheapest to check
    if !has_no_forbidden_chars(password) {
        return false;
    }
    
    // Rule 3: Medium likelihood
    if !has_two_pairs(password) {
        return false;
    }
    
    // Rule 1: Least likely to fail
    if !has_increasing_straight(password) {
        return false;
    }
    
    true
}
```

**Test**:
```bash
cargo test test_example_failures
cargo test test_example_successes
```

---

### **Phase 4: Password Generation (Main Loop)**

#### **Function**: `solve_part1(input: &str) -> Result<String>`

**Goal**: Find the next valid password after the input

**Basic Algorithm**:
```rust
pub fn solve_part1(input: &str) -> Result<String> {
    let mut password = input.trim().to_string();
    
    loop {
        password = increment_password(&password);
        
        if is_valid_password(&password) {
            return Ok(password);
        }
    }
}
```

**Test**:
```bash
# Run the ignored integration tests
cargo test test_find_next_password -- --ignored
```

---

### **Phase 5: Optimization (Optional but Recommended)**

#### **Skip Forbidden Characters**

When you encounter a forbidden char, **skip ahead** instead of incrementing one by one!

**Example**:
- Current: "abci"
- Forbidden char: 'i' at position 3
- Skip to: "abcj" → then reset rest to "a" → "abcjaaa"

**This can save thousands of iterations!**

**Implementation**:
```rust
pub fn skip_forbidden_char(password: &str) -> String {
    let mut chars: Vec<char> = password.chars().collect();
    
    for i in 0..chars.len() {
        if chars[i] == 'i' || chars[i] == 'o' || chars[i] == 'l' {
            // Replace with next valid letter
            chars[i] = match chars[i] {
                'i' => 'j',
                'o' => 'p',
                'l' => 'm',
                _ => chars[i],
            };
            
            // Reset everything to the right to 'a'
            for j in (i + 1)..chars.len() {
                chars[j] = 'a';
            }
            
            break;
        }
    }
    
    chars.into_iter().collect()
}
```

**Optimized Main Loop**:
```rust
pub fn solve_part1(input: &str) -> Result<String> {
    let mut password = input.trim().to_string();
    
    loop {
        password = increment_password(&password);
        
        // OPTIMIZATION: Skip entire ranges with forbidden chars
        if !has_no_forbidden_chars(&password) {
            password = skip_forbidden_char(&password);
            continue;
        }
        
        if is_valid_password(&password) {
            return Ok(password);
        }
    }
}
```

---

### **Phase 6: Part 2**

#### **Function**: `solve_part2(input: &str) -> Result<String>`

**Hint**: Part 2 asks for the password **after** the one from Part 1.

**Strategy**:
1. Find Part 1 password
2. Increment it once
3. Find the next valid password from there

```rust
pub fn solve_part2(input: &str) -> Result<String> {
    // Start by finding Part 1 answer
    let part1_password = solve_part1(input)?;
    
    // Increment once to move past Part 1 answer
    let mut password = increment_password(&part1_password);
    
    // Find next valid password
    loop {
        if is_valid_password(&password) {
            return Ok(password);
        }
        password = increment_password(&password);
    }
}
```

---

## 🧪 **Testing Strategy**

### **Run Tests Incrementally**

```bash
# Phase 1: Incrementing
cargo test test_increment_simple
cargo test test_increment_wrap

# Phase 2: Individual Rules
cargo test test_no_forbidden
cargo test test_has_straight
cargo test test_has_two_pairs

# Phase 3: Full Validation
cargo test test_example_failures
cargo test test_example_successes

# Phase 4: Integration
cargo test test_find_next_password -- --ignored

# Run all Day 11 tests
cargo test day11
```

---

## ⚡ **Performance Expectations**

| Implementation | Expected Time |
|----------------|---------------|
| **Naive** (no optimization) | 1-5 seconds |
| **With skip_forbidden_char** | < 100ms |
| **Highly optimized** | < 10ms |

---

## 🎯 **Common Pitfalls**

### **1. Overlapping Pairs**
```rust
// ❌ WRONG: This counts "aaa" as 2 pairs
for i in 0..chars.len() - 1 {
    if chars[i] == chars[i + 1] {
        pair_count += 1;
        i += 1;  // Only moves forward by 1!
    }
}

// ✅ CORRECT: Skip next character
while i < chars.len() - 1 {
    if chars[i] == chars[i + 1] {
        pair_count += 1;
        i += 2;  // Skip both chars in the pair
    } else {
        i += 1;
    }
}
```

### **2. Off-by-One in Straight Detection**
```rust
// ❌ WRONG: Can go out of bounds
for i in 0..bytes.len() {
    if bytes[i + 2] == bytes[i] + 2 { ... }  // Panic!
}

// ✅ CORRECT: Stop before last 2 chars
for i in 0..bytes.len().saturating_sub(2) {
    if bytes[i + 2] == bytes[i] + 2 { ... }
}
```

### **3. Incrementing Logic**
```rust
// ❌ WRONG: Doesn't handle carry properly
if chars[i] == 'z' {
    chars[i] = 'a';
    break;  // Stops! Should continue carrying left
}

// ✅ CORRECT: Continue loop to carry
if chars[i] == 'z' {
    chars[i] = 'a';
    // Continue loop - no break!
} else {
    chars[i] = ((chars[i] as u8) + 1) as char;
    break;  // Only break when no carry needed
}
```

---

## 🚀 **Quick Start**

1. **Implement incrementing first** - It's the foundation
2. **Test each rule function separately** - Unit test everything
3. **Combine in is_valid_password** - Test with examples
4. **Write main loop** - Keep it simple first
5. **Optimize later** - Get correctness first, speed second

---

## 📝 **Example Test Run**

```bash
# Your input
hxbxwxba

# Expected workflow:
hxbxwxba → increment → hxbxwxbb → invalid (no straight?)
         → keep incrementing...
         → eventually find valid password

# Your answer will be the first valid password found!
```

---

## 🎄 **Final Tips**

1. **Use the examples** - They're perfect test cases
2. **Test incrementally** - Don't write everything at once
3. **Print debug info** - See what passwords you're checking
4. **Optimization matters** - Part 2 might be slow without skip logic
5. **Have fun!** - This is a satisfying problem when it clicks! 🎉

Good luck! 🚀

---

## 🔗 Related Resources

**Day 11 Documentation:**
- [[day11.md|../../Problem_Statements/day11]] - Official problem statement
- [[DAY11_QUICK_REFERENCE|DAY11_QUICK_REFERENCE]] - Quick reference card
- [[DAY11_SETUP_COMPLETE|DAY11_SETUP_COMPLETE]] - Template summary

**Implementation Files:**
- [[day11.rs|../../src/solver/day11]] - Your implementation (with TODOs)
- [[day11_test.rs|../../tests/day11_test]] - Test suite (21 comprehensive tests)
- [[day11_example.txt|../../inputs/day11_example.txt]] - Puzzle input (`hxbxwxba`)

**Zettelkasten Learning:**
- [[String Manipulation|../../../zettelkasten/String Manipulation]] - String processing in Rust
- [[Iteration Patterns|../../../zettelkasten/Iteration Patterns]] - Loop and windowing techniques
- [[Validation Patterns|../../../zettelkasten/Validation Patterns]] - Rule-based validation
- [[Test-Driven Development|../../../zettelkasten/Test-Driven Development]] - TDD approach
- [[AoC 2015 MOC|../../../zettelkasten/AoC 2015 MOC]] - Navigate all 2015 problems
- [[AoC Patterns MOC|../../../zettelkasten/AoC Patterns MOC]] - Common AoC patterns

**Rust Book Integration:**
- [[Chapter 8|../../../rust_book/Ch8/README]] - Collections and strings
- [[Week 1 Overview|../../../zettelkasten/Week 1 Overview]] - String fundamentals

**Problem Catalog:**
- [[summary.md|../../Problem_Statements/summary]] - All AoC 2015 problems

*Tags: #aoc2015 #day11 #tutorial #implementation-guide #password-validation #step-by-step*
