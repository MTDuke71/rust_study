# 🐮 Cow: Borrowed vs Owned

**Clone-on-Write - Conditional ownership for performance**

**Tags:** #cow #ownership #performance #memory #rust-for-rustaceans-ch3 #api-design

**Related:** [[ownership-fundamentals]], [[Clone vs Copy]], [[asref-trait-ergonomics]], [[Memory Optimization]], [[Performance Optimization]], [[API Design Principles]]

---

## 🎯 Core Concept

`Cow<'a, B>` (Clone on Write) is an enum that can hold either borrowed or owned data, deferring allocation until mutation is needed. It's a **performance optimization** that avoids unnecessary clones.

```rust
pub enum Cow<'a, B> 
where
    B: 'a + ToOwned + ?Sized,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

**Key Principle:** Only require ownership when necessary. Return borrowed data by default, allocate only when modification is required.

---

## 📋 When to Use Cow

### **Problem: Uncertain Ownership Requirements**

Sometimes you don't know at compile time whether:
- You can return a reference to existing data (zero-cost)
- You need to create new owned data (allocation required)

### **Solution: Cow Defers the Decision**

```rust
use std::borrow::Cow;

/// Returns borrowed data if no modification needed
/// Returns owned data if prefix must be added
fn add_prefix<'a>(text: &'a str, add: bool) -> Cow<'a, str> {
    if add {
        Cow::Owned(format!("PREFIX: {}", text))  // Allocate only when needed
    } else {
        Cow::Borrowed(text)  // Zero allocation - just return reference
    }
}

fn main() {
    let result1 = add_prefix("Hello", false);  // Borrowed - no allocation
    let result2 = add_prefix("World", true);   // Owned - allocated
    
    // Both can be used the same way via Deref
    println!("{}", result1);  // "Hello"
    println!("{}", result2);  // "PREFIX: World"
}
```

---

## 🔍 Common Patterns

### **Pattern 1: Conditional Modification**

```rust
use std::borrow::Cow;

fn normalize_path(path: &str) -> Cow<str> {
    if path.starts_with("./") {
        // Need to modify - must allocate
        Cow::Owned(path.strip_prefix("./").unwrap().to_string())
    } else {
        // No modification needed - zero cost
        Cow::Borrowed(path)
    }
}
```

### **Pattern 2: Transformation with Fallback**

```rust
fn escape_html(text: &str) -> Cow<str> {
    if text.contains(&['<', '>', '&'][..]) {
        // Contains special chars - must escape (allocate)
        Cow::Owned(
            text.replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('&', "&amp;")
        )
    } else {
        // No special chars - return as-is
        Cow::Borrowed(text)
    }
}
```

### **Pattern 3: Format String Optimization**

```rust
fn format_message<'a>(msg: &'a str, debug: bool) -> Cow<'a, str> {
    if debug {
        Cow::Owned(format!("[DEBUG] {}", msg))  // Allocate for debug info
    } else {
        Cow::Borrowed(msg)  // Production: no overhead
    }
}
```

---

## ⚙️ How Cow Works

### **Deref Coercion**

`Cow<'a, B>` implements `Deref<Target = B>`, so you can use it like the borrowed type:

```rust
let cow: Cow<str> = Cow::Borrowed("hello");
println!("{}", cow.to_uppercase());  // Works like &str
```

### **to_mut() - Lazy Cloning**

```rust
let mut cow = Cow::Borrowed("hello");
cow.to_mut().push_str(" world");  // Clones to owned if borrowed
println!("{}", cow);  // "hello world" (now owned)
```

### **into_owned() - Extract Owned Value**

```rust
let cow = Cow::Borrowed("test");
let owned: String = cow.into_owned();  // Clones if borrowed, moves if owned
```

---

## 🎯 Design Guideline: Only Require Ownership When Necessary

From **Rust for Rustaceans Chapter 3 - Designing Interfaces**:

> "Don't force callers to give up ownership if you don't need it"

### **❌ Bad: Always Require Ownership**

```rust
// Forces caller to give up ownership even if we don't modify
fn process(data: String) -> usize {
    data.len()  // Wasteful - didn't need ownership
}

let s = String::from("hello");
let len = process(s);
// s is now gone! Forced move for no reason
```

### **✅ Good: Accept Reference by Default**

```rust
// Only borrows - caller retains ownership
fn process(data: &str) -> usize {
    data.len()
}

let s = String::from("hello");
let len = process(&s);
println!("{}", s);  // ✅ s still available
```

### **✅ Better: Use Cow When Modification is Conditional**

```rust
use std::borrow::Cow;

// Borrows when possible, owns when needed
fn process(data: &str, uppercase: bool) -> Cow<str> {
    if uppercase {
        Cow::Owned(data.to_uppercase())  // Need to modify
    } else {
        Cow::Borrowed(data)  // Return reference
    }
}
```

---

## 📊 Performance Comparison

```rust
// Benchmark: No modification needed
let data = "hello world";

// ❌ Always clone - 100% allocation overhead
let owned = data.to_string();  // Allocates every time

// ✅ Cow borrows - 0% allocation overhead
let cow = Cow::Borrowed(data);  // Zero allocation
```

**Results:**
- **Owned:** Always allocates heap memory
- **Cow (Borrowed):** Zero allocation when no modification
- **Cow (Owned):** Only allocates when modification required

---

## 🔄 Integration with Other Types

### **From String/&str to Cow**

```rust
let cow1: Cow<str> = Cow::Borrowed("borrowed");
let cow2: Cow<str> = Cow::Owned(String::from("owned"));
let cow3: Cow<str> = "literal".into();  // Borrowed
let cow4: Cow<str> = String::from("owned").into();  // Owned
```

### **Vec/Slice Variant**

```rust
use std::borrow::Cow;

fn process_numbers(nums: &[i32], filter: bool) -> Cow<[i32]> {
    if filter {
        Cow::Owned(nums.iter().filter(|&&x| x > 0).copied().collect())
    } else {
        Cow::Borrowed(nums)
    }
}
```

---

## 🎓 Learning Resources

### **Rust for Rustaceans Ch3: Designing Interfaces**

Key insight: **Flexible ownership through Cow**

- Don't require ownership unless necessary
- Use `&T` for read-only access
- Use `Cow<'a, T>` when modification is conditional
- Only use `T` (owned) when you truly need ownership

### **Implementation Example**

See: [[rust-for-rustaceans]] Ch3, `flexible_ownership.rs`

---

## ⚠️ Common Pitfalls

### **❌ Don't Use Cow for Always-Modified Data**

```rust
// BAD: If you always modify, just return String
fn always_uppercase(s: &str) -> Cow<str> {
    Cow::Owned(s.to_uppercase())  // Always owned - Cow is overhead
}

// GOOD: Direct String return is clearer
fn always_uppercase(s: &str) -> String {
    s.to_uppercase()
}
```

### **❌ Don't Use Cow for Always-Borrowed Data**

```rust
// BAD: If you never modify, just use &str
fn never_modify(s: &str) -> Cow<str> {
    Cow::Borrowed(s)  // Always borrowed - Cow is overhead
}

// GOOD: Direct reference is clearer
fn never_modify(s: &str) -> &str {
    s
}
```

### **✅ Use Cow Only When Ownership is Conditional**

```rust
// GOOD: Sometimes borrowed, sometimes owned
fn conditional_modify(s: &str, modify: bool) -> Cow<str> {
    if modify {
        Cow::Owned(s.to_uppercase())
    } else {
        Cow::Borrowed(s)
    }
}
```

---

## 🔗 Related Concepts

- [[ownership-fundamentals]] - Foundation of Rust ownership system
- [[Clone vs Copy]] - Explicit vs implicit duplication
- [[asref-trait-ergonomics]] - Flexible function parameters
- [[Memory Optimization]] - Avoiding unnecessary allocations
- [[Performance Optimization]] - Performance patterns including Cow
- [[API Design Principles]] - Designing flexible interfaces
- [[rust-for-rustaceans]] - Source material (Ch3)

---

## 📝 Quick Reference

```rust
use std::borrow::Cow;

// Create
let borrowed = Cow::Borrowed("text");
let owned = Cow::Owned(String::from("text"));

// Access (via Deref)
println!("{}", borrowed);  // Works like &str

// Mutation (clones if borrowed)
let mut cow = Cow::Borrowed("hello");
cow.to_mut().push_str(" world");

// Extract owned value
let string: String = cow.into_owned();

// Pattern matching
match cow {
    Cow::Borrowed(s) => println!("Borrowed: {}", s),
    Cow::Owned(s) => println!("Owned: {}", s),
}
```

---

**Key Takeaway:** `Cow` is a **performance optimization** for APIs where ownership requirements are conditional. Only allocate when modification is needed, otherwise return borrowed data for zero cost.

*Links:*
- **Outgoing:** [[ownership-fundamentals]], [[Clone vs Copy]], [[asref-trait-ergonomics]], [[Memory Optimization]], [[Performance Optimization]], [[API Design Principles]], [[rust-for-rustaceans]]
- **Incoming:** (To be added by related notes)
