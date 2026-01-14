# 🔪 Zero-Copy String Slicing

**Rust performance optimization: borrowing string slices without heap allocation**

**Tags:** #performance #zero-copy #string #lifetime #optimization #mission-11 #rust-performance

**Related:** [[String Type]], [[top-down-dp-pattern]], [[Performance Optimization]], [[Memory Optimization]], [[ownership-fundamentals]], [[mission-11]]

---

## 🎯 Core Concept

**Zero-Copy String Slicing** is a Rust performance technique that manipulates strings through borrowed `&str` slices instead of creating owned `String` allocations. This **eliminates heap allocations** in performance-critical code paths.

**Key Principle:** If you don't need to own the string, don't copy it — borrow a slice instead.

---

## 💡 The Problem: Allocation Overhead

### **❌ Naive Approach: Allocating Everywhere**

```rust
fn process_substrings(text: String) -> Vec<String> {
    let mut results = Vec::new();
    
    for i in 0..text.len() {
        // ❌ to_string() allocates new String on heap
        let substring = text[i..].to_string();  // ALLOCATION
        results.push(substring);  // ALLOCATION
    }
    
    results
}
```

**Cost for 100-character string:**
- **100 heap allocations** (one per substring)
- **5,050 bytes allocated** (sum of substring lengths)
- **100 memory copies** of string data

**Performance:** O(n²) memory allocations for n-length string

### **✅ Zero-Copy Approach: Borrowing Slices**

```rust
fn process_substrings(text: &str) -> Vec<&str> {
    let mut results = Vec::new();
    
    for i in 0..text.len() {
        // ✅ Slice is just a pointer + length (no allocation!)
        let substring = &text[i..];  // ZERO ALLOCATION
        results.push(substring);  // Just copying pointer+len
    }
    
    results
}
```

**Cost for 100-character string:**
- **0 heap allocations** (slices are stack-only)
- **1,600 bytes on stack** (Vec storage for 100 slice references)
- **0 memory copies** of string data

**Performance:** O(1) memory allocations regardless of string length

**Speedup:** **10-100× faster** for substring-heavy operations

---

## 🔧 Core Techniques

### **1. String Slicing (No Allocation)**

```rust
let s = "hello world";

// ✅ Zero-copy slicing
let hello = &s[0..5];     // Just pointer to s + length 5
let world = &s[6..11];    // Just pointer to s + length 5
let all = &s[..];         // Entire string (still zero-copy)

// ❌ Allocation (avoid in hot paths)
let hello_owned = s[0..5].to_string();  // Heap allocation
```

**What's happening:**
- `&s[range]` creates a **fat pointer**: `(ptr: *const u8, len: usize)`
- **No data copying** - just pointer arithmetic
- Slice references original string's memory

### **2. strip_prefix() / strip_suffix() (Zero-Copy Matching)**

```rust
let pattern = "hello";
let text = "hello world";

// ✅ Zero-copy: Returns Option<&str> slice
if let Some(remainder) = text.strip_prefix(pattern) {
    // remainder = " world" (slice of original text, no allocation)
    assert_eq!(remainder, " world");
}

// ❌ Allocation equivalent (what NOT to do)
if text.starts_with(pattern) {
    let remainder = text[pattern.len()..].to_string();  // ALLOCATION
}
```

**Mission 11 Usage:**
```rust
// Pattern matching with zero allocations
for pattern in patterns {
    if let Some(remainder) = target.strip_prefix(pattern) {
        // remainder is &str slice - recurse with zero overhead
        solve(remainder, patterns, memo);
    }
}
```

### **3. Lifetime-Parametric HashMap Keys**

```rust
use std::collections::HashMap;

// ✅ Cache with borrowed keys (zero-copy)
fn memoized_solve<'a>(
    text: &'a str,  // Lifetime 'a: input string
    memo: &mut HashMap<&'a str, bool>,  // Cache keys are slices of input
) -> bool {
    if let Some(&result) = memo.get(text) {
        return result;
    }
    
    // ... computation ...
    
    memo.insert(text, result);  // Key is borrowed slice (no allocation)
    result
}
```

**How it works:**
1. Input string `text` has lifetime `'a`
2. All slices created from `text` also have lifetime `'a`
3. HashMap keys are `&'a str` - guaranteed to live as long as `text`
4. **No allocations** - cache stores pointers, not owned strings

**Compiler Guarantee:** Slices can't outlive the original string (enforced by lifetime `'a`)

### **4. chars() Iterator (Zero-Copy Character Access)**

```rust
let s = "hello";

// ✅ Zero-copy iteration
for c in s.chars() {
    // c is char, derived from slice (no allocation)
}

// ❌ Allocation (collect to String)
let chars: String = s.chars().collect();  // ALLOCATION
```

---

## 🎓 Real-World Example: String DP with Memoization

### **Problem:** Can we construct `target` from `patterns`?

**Naive (Allocating) Version:**

```rust
fn can_construct_slow(
    target: String,  // ❌ Owned - forces moves and clones
    patterns: &[String],
    memo: &mut HashMap<String, bool>,  // ❌ Owned keys = allocations
) -> bool {
    if target.is_empty() { return true; }
    
    if let Some(&cached) = memo.get(&target) { return cached; }
    
    for pattern in patterns {
        if target.starts_with(pattern) {
            // ❌ Allocates new String for remainder
            let remainder = target[pattern.len()..].to_string();
            if can_construct_slow(remainder, patterns, memo) {
                memo.insert(target.clone(), true);  // ❌ Clones target
                return true;
            }
        }
    }
    
    memo.insert(target, false);  // ❌ Moves target into cache
    false
}
```

**Allocations per call:** 2-4 (remainder, target clone, cache insertion)

**Zero-Copy (Optimized) Version:**

```rust
fn can_construct_fast<'a>(
    target: &'a str,  // ✅ Borrowed slice
    patterns: &[&str],
    memo: &mut HashMap<&'a str, bool>,  // ✅ Borrowed keys
) -> bool {
    if target.is_empty() { return true; }
    
    if let Some(&cached) = memo.get(target) { return cached; }
    
    for pattern in patterns {
        // ✅ strip_prefix returns Option<&str> (zero-copy!)
        if let Some(remainder) = target.strip_prefix(pattern) {
            if can_construct_fast(remainder, patterns, memo) {
                memo.insert(target, true);  // ✅ Just stores pointer
                return true;
            }
        }
    }
    
    memo.insert(target, false);
    false
}
```

**Allocations per call:** **0**

**Performance Difference (10-character target, 5 patterns):**
- Naive: ~50 allocations, ~300 bytes, 100µs
- Zero-copy: 0 allocations, 0 bytes, 10µs
- **10× speedup**

---

## ⚡ Performance Characteristics

### **Memory Comparison**

| Operation | `String` (Owned) | `&str` (Slice) | Difference |
|-----------|------------------|----------------|------------|
| **Storage** | Heap allocation + metadata | Fat pointer (16 bytes) | ~10× savings |
| **Substring** | O(n) allocation + copy | O(1) pointer + length | Infinite speedup |
| **Concatenation** | O(n) allocation | ❌ Can't concat (immutable) | N/A |
| **HashMap Key** | Clone on insert | Just pointer | ~100× faster |

### **Time Complexity**

```rust
let s = "a".repeat(1000);  // 1000-char string

// Substring creation
let slice = &s[500..];     // O(1) - just pointer arithmetic
let owned = s[500..].to_string();  // O(n) - allocates and copies 500 chars

// HashMap insertion
let mut map: HashMap<&str, i32> = HashMap::new();
map.insert(&s[..], 1);     // O(1) - hash pointer value
let mut map2: HashMap<String, i32> = HashMap::new();
map2.insert(s.clone(), 1); // O(n) - must clone entire string
```

---

## 🎯 When to Use Zero-Copy Slicing

### **✅ Use &str Slicing When:**

1. **Read-Only Access** - You don't need to modify the string
2. **Temporary Processing** - Substrings live only during function scope
3. **Performance Critical** - Hot path with many substring operations
4. **Memoization** - Caching intermediate string states
5. **Pattern Matching** - prefix/suffix/substring checks

### **❌ Use String When:**

1. **Ownership Required** - Need to store string beyond input lifetime
2. **Mutation Needed** - Must append, modify, or build strings
3. **Return from Function** - Can't return `&str` tied to local data
4. **Cross-Thread** - Need `'static` or `Send` strings

---

## 🔍 Advanced Patterns

### **Pattern 1: Recursive DP with String Slices**

```rust
fn solve_dp<'a>(
    remaining: &'a str,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if remaining.is_empty() { return 1; }
    if let Some(&cached) = memo.get(remaining) { return cached; }
    
    let mut total = 0;
    
    // Try all possible prefix splits (zero-copy!)
    for i in 1..=remaining.len() {
        let prefix = &remaining[..i];      // ✅ Zero-copy slice
        let suffix = &remaining[i..];      // ✅ Zero-copy slice
        
        if is_valid(prefix) {
            total += solve_dp(suffix, memo);
        }
    }
    
    memo.insert(remaining, total);
    total
}
```

**Key:** All slices (`prefix`, `suffix`, `remaining`) are zero-copy views into the original input.

### **Pattern 2: Multi-Level Slicing**

```rust
fn analyze<'a>(text: &'a str) -> Vec<Vec<&'a str>> {
    let lines: Vec<&str> = text.lines().collect();  // ✅ Zero-copy
    
    lines.iter().map(|line| {
        line.split_whitespace().collect()  // ✅ Zero-copy words
    }).collect()
}
```

**Result:** Nested structure of slices, **zero allocations** for string data.

### **Pattern 3: State Machine with Slices**

```rust
enum ParseState<'a> {
    Start,
    InToken(&'a str),
    InString(&'a str),
}

fn parse<'a>(input: &'a str) -> Vec<&'a str> {
    let mut tokens = Vec::new();
    let mut state = ParseState::Start;
    
    for (i, c) in input.char_indices() {
        state = match state {
            ParseState::Start if c == '"' => {
                ParseState::InString(&input[i+1..])  // ✅ Zero-copy
            }
            // ... state transitions with slices
            _ => state,
        };
    }
    
    tokens
}
```

---

## ⚠️ Common Pitfalls

### **❌ Pitfall 1: Returning Local String Slices**

```rust
// ❌ WON'T COMPILE - lifetime error
fn get_hello() -> &str {
    let s = String::from("hello");
    &s[..]  // Error: s is dropped, can't return slice of it
}

// ✅ Return owned String instead
fn get_hello() -> String {
    String::from("hello")
}

// ✅ Or use static string literal
fn get_hello() -> &'static str {
    "hello"  // Lives forever
}
```

### **❌ Pitfall 2: Slice Invalidation After Mutation**

```rust
let mut s = String::from("hello");
let slice = &s[..];  // Borrow s as immutable

s.push_str(" world");  // ❌ Error: can't mutate while borrowed
println!("{}", slice); // Slice would be dangling!
```

**Fix:** Drop slice before mutation or use `clone()`.

### **❌ Pitfall 3: HashMap Lifetime Confusion**

```rust
// ❌ WON'T COMPILE
fn broken<'a>(memo: &mut HashMap<&'a str, i32>) {
    let local = String::from("key");
    memo.insert(&local, 42);  // Error: local doesn't live long enough
}

// ✅ Use owned keys if data is local
fn fixed(memo: &mut HashMap<String, i32>) {
    let local = String::from("key");
    memo.insert(local, 42);  // OK: moves ownership
}
```

### **❌ Pitfall 4: Accidental Allocation in Loop**

```rust
// ❌ Allocates on every iteration
for i in 0..text.len() {
    let substring = text[i..].to_string();  // ALLOCATION
    process(substring);
}

// ✅ Zero-copy
for i in 0..text.len() {
    let substring = &text[i..];  // Just pointer
    process_slice(substring);
}
```

---

## 🧪 Benchmark Example

```rust
use std::time::Instant;

fn benchmark_comparison() {
    let text = "a".repeat(1000);
    
    // ❌ Allocating version
    let start = Instant::now();
    for _ in 0..10_000 {
        let _substring = text[500..].to_string();
    }
    println!("Allocating: {:?}", start.elapsed());
    // Output: ~50ms
    
    // ✅ Zero-copy version
    let start = Instant::now();
    for _ in 0..10_000 {
        let _substring = &text[500..];
    }
    println!("Zero-copy: {:?}", start.elapsed());
    // Output: ~0.5ms (100× faster!)
}
```

---

## 📚 Learning Resources

### **Mission 11 Implementation**

See: [missions/Mission11/src/string_dp.rs](../../../missions/Mission11/src/string_dp.rs)

**Key Functions:**
- `can_construct()` - Demonstrates `strip_prefix()` and `HashMap<&'a str, bool>`
- `count_constructions()` - Zero-copy recursive string processing
- Tests validate zero allocations via lifetime checks

**REQ-3:** Zero-Copy String Slice Caching
- 4 dedicated tests (`req3_*`)
- Performance benchmarks showing allocation elimination
- Lifetime safety validation

### **Related Concepts**

- [[String Type]] - Rust string fundamentals (`String` vs `&str`)
- [[Performance Optimization]] - General optimization strategies
- [[Memory Optimization]] - Memory-efficient patterns
- [[top-down-dp-pattern]] - DP pattern using zero-copy slicing

---

## 📝 Quick Reference

```rust
// ═══════════════════════════════════════════════════
// ZERO-COPY STRING OPERATIONS - Cheat Sheet
// ═══════════════════════════════════════════════════

// ✅ ZERO-COPY (Fast)
let slice = &s[start..end];           // Slice (just pointer + len)
let prefix = &s[..n];                 // First n chars
let suffix = &s[n..];                 // After n chars
let whole = &s[..];                   // Entire string

if let Some(rest) = s.strip_prefix(p) { }  // Remove prefix
if let Some(rest) = s.strip_suffix(p) { }  // Remove suffix

let words: Vec<&str> = s.split_whitespace().collect();  // Split
let lines: Vec<&str> = s.lines().collect();  // By newline

// Cache with borrowed keys
let mut memo: HashMap<&str, T> = HashMap::new();

// ❌ ALLOCATES (Avoid in hot paths)
let owned = s[..n].to_string();       // Creates new String
let concat = format!("{}{}", s1, s2); // Allocates result
let cloned = s.clone();               // Duplicates data
let replaced = s.replace("old", "new"); // New String

// Cache with owned keys (allocates on insert)
let mut memo: HashMap<String, T> = HashMap::new();
```

---

## 🎯 Decision Matrix

| Need | Use `&str` | Use `String` |
|------|-----------|--------------|
| Read-only substring | ✅ Zero-copy | ❌ Allocates |
| Modify string | ❌ Immutable | ✅ Mutable |
| Return from function | ⚠️ Lifetime issues | ✅ Owned |
| HashMap key (temporary) | ✅ Fast | ❌ Slow (clones) |
| HashMap key (persistent) | ⚠️ Lifetime ties | ✅ Owns data |
| Cross-thread | ❌ Not `Send` (if borrowed) | ✅ `Send + Sync` |
| Performance critical | ✅ Zero overhead | ❌ Allocation cost |

---

**Key Takeaway:** Zero-copy string slicing transforms string-heavy algorithms from **allocation-bound** to **compute-bound**. In Mission 11, this enables O(P×L) DP to run in microseconds instead of milliseconds by eliminating 100+ allocations per query.

*Links:*
- **Outgoing:** [[String Type]], [[top-down-dp-pattern]], [[Performance Optimization]], [[Memory Optimization]], [[ownership-fundamentals]], [[mission-11]]
- **Incoming:** (To be added by related notes)
