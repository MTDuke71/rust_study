# 🎯 Day 11 Quick Reference Card

## 📝 **Problem Summary**
Generate valid passwords by incrementing like base-26 counting until all 3 rules pass.

---

## 🔑 **Three Validation Rules**

| Rule | Description | Example |
|------|-------------|---------|
| **1. Straight** | 3+ consecutive letters | abc, bcd, xyz ✅ |
| **2. No i/o/l** | Forbidden characters | No i, o, or l ❌ |
| **3. Two Pairs** | Non-overlapping pairs | aa, bb ✅ |

---

## 🚀 **Quick Implementation Order**

```
1. increment_password()      ← Start here (base-26 counting)
2. has_no_forbidden_chars()  ← Easiest rule
3. has_increasing_straight() ← Medium difficulty
4. has_two_pairs()          ← Trickiest (overlapping!)
5. is_valid_password()      ← Combine all three
6. solve_part1()            ← Main loop
```

---

## 🧪 **Test As You Go**

```bash
# Test each function individually
cargo test test_increment_simple
cargo test test_no_forbidden
cargo test test_has_straight
cargo test test_has_two_pairs

# Test full validation
cargo test test_example_failures
cargo test test_example_successes

# Integration tests (once implemented)
cargo test --test day11_test
```

---

## ⚡ **Key Code Patterns**

### **Incrementing (Base-26)**
```rust
for i in (0..chars.len()).rev() {
    if chars[i] != 'z' {
        chars[i] = ((chars[i] as u8) + 1) as char;
        break;
    } else {
        chars[i] = 'a';  // Continue to carry
    }
}
```

### **Checking Straight**
```rust
for i in 0..bytes.len().saturating_sub(2) {
    if bytes[i + 1] == bytes[i] + 1 && 
       bytes[i + 2] == bytes[i] + 2 {
        return true;
    }
}
```

### **Counting Non-Overlapping Pairs**
```rust
while i < chars.len() - 1 {
    if chars[i] == chars[i + 1] {
        count += 1;
        i += 2;  // ← Skip next!
    } else {
        i += 1;
    }
}
```

---

## 🎯 **Example Test Cases**

| Input | Has Straight? | No i/o/l? | 2 Pairs? | Valid? |
|-------|---------------|-----------|----------|--------|
| hijklmmn | ✅ (hij) | ❌ (i, l) | ✅ (mm, nn) | ❌ |
| abbceffg | ❌ | ✅ | ✅ (bb, ff) | ❌ |
| abbcegjk | ❌ | ✅ | ❌ (only bb) | ❌ |
| abcdffaa | ✅ (abc) | ✅ | ✅ (ff, aa) | ✅ |

---

## 🐛 **Common Pitfalls**

### ❌ **Wrong**: Overlapping Pairs
```rust
// "aaa" counted as 2 pairs - WRONG!
i += 1  // Only moves forward by 1
```

### ✅ **Right**: Skip Next Character
```rust
// "aaa" counted as 1 pair - CORRECT!
i += 2  // Skips both chars in pair
```

### ❌ **Wrong**: Bounds Error
```rust
for i in 0..bytes.len() {
    if bytes[i + 2] == ...  // Panic!
}
```

### ✅ **Right**: Stop Early
```rust
for i in 0..bytes.len().saturating_sub(2) {
    if bytes[i + 2] == ...  // Safe!
}
```

---

## ⚡ **Optimization Tip**

When you find forbidden char, **skip ahead**:

```rust
"abci" → "abcj" + "aaa" → "abcjaaa"
```

This can save **thousands** of iterations!

---

## 📊 **Performance Targets**

| Strategy | Time | Iterations |
|----------|------|------------|
| Naive | 1-5 sec | 100,000+ |
| **With skip** | **< 100ms** | **1,000s** |
| Optimized | < 10ms | 100s |

---

## 📂 **File Locations**

```
src/solver/day11.rs              # ← Your implementation
tests/day11_test.rs              # ← 21 tests
examples/DAY11_IMPLEMENTATION_GUIDE.md  # ← Full guide
inputs/day11_example.txt         # ← Puzzle input
```

---

## 🎄 **Quick Start Commands**

```bash
# Run all Day 11 tests
cargo test day11

# Run specific test groups
cargo test test_increment
cargo test test_has_straight
cargo test test_has_two_pairs

# Run integration tests
cargo test --test day11_test

# Check compilation
cargo check --lib
```

---

## 💡 **Problem-Solving Strategy**

1. **Implement increment_password()** first
2. **Test it** thoroughly (wrap cases!)
3. **Implement each rule** separately
4. **Test each rule** with examples
5. **Combine in is_valid_password()**
6. **Build main loop** (simple first)
7. **Test with examples**: "abcdefgh" → "abcdffaa"
8. **Optimize** if needed (skip forbidden)

---

## 🚦 **Current Status**

- ✅ Template created with TODOs
- ✅ 21 comprehensive tests ready
- ✅ Full implementation guide available
- ✅ Compiles (8 warnings normal for templates)
- 🎯 Ready for your implementation!

---

## 📚 **Your Input**
`hxbxwxba`

Find the next valid password! 🔐

---

*Quick Reference Card for Day 11: Corporate Policy*
