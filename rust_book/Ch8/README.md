# Chapter 8: Common Collections

---
*Navigation: [[zettel-index]] | [[Collections MOC]] | [[Vec]] | [[String]] | [[HashMap]]*
*Quick Links: [[Memory Management]] | [[Ownership and Borrowing]] | [[Daily Study MOC]]*
*Related Concepts: [[Iterators]] | [[Generics]] | [[Heap Allocation]] | [[Performance Optimization]]*
*Deep Dives: [[Unicode, UTF-8, and Rust]] | [[String Performance]]*
---

## 📚 Overview

Chapter 8 introduces Rust's **common collections** - data structures stored on the heap that can grow or shrink at runtime. Unlike arrays and tuples (which have fixed sizes known at compile time), collections are essential for managing dynamic data.

**Three Most Common Collections:**
1. **Vector (`Vec<T>`)** - Growable array of values
2. **String (`String`)** - Collection of characters (UTF-8)
3. **Hash Map (`HashMap<K, V>`)** - Key-value associations

---

## 🎯 Key Concepts

### 1. **Vectors: `Vec<T>`**

Vectors store multiple values of the same type in contiguous memory.

**Key Operations:**
```rust
// Creating vectors
let v1: Vec<i32> = Vec::new();
let v2 = vec![1, 2, 3];  // vec! macro

// Adding elements
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);

// Reading elements
let third: &i32 = &v[2];              // Panics if out of bounds
let third: Option<&i32> = v.get(2);   // Returns None if out of bounds

// Iterating
for i in &v {
    println!("{}", i);
}

// Mutable iteration
for i in &mut v {
    *i += 50;
}
```

**Memory Layout:**
- Stored on the heap
- Contiguous memory allocation
- Automatic resizing with capacity management
- O(1) append (amortized), O(1) index access

---

### 2. **Strings: `String`**

Strings are growable, mutable, owned, UTF-8 encoded text collections.

**Key Operations:**
```rust
// Creating strings
let mut s = String::new();
let s = "hello".to_string();
let s = String::from("hello");

// Updating strings
let mut s = String::from("foo");
s.push_str("bar");   // Append string slice
s.push('!');         // Append single character

// Concatenation
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;   // s1 moved, s2 borrowed

// Format macro
let s = format!("{}-{}-{}", "tic", "tac", "toe");

// Indexing (Not allowed!)
// let h = s[0];  // ❌ Compile error!

// Slicing (with caution)
let hello = "Здравствуйте";
let s = &hello[0..4];  // "Зд" (each Cyrillic char is 2 bytes)

// Iterating
for c in "नमस्ते".chars() {
    println!("{}", c);  // Individual Unicode scalar values
}

for b in "नमस्ते".bytes() {
    println!("{}", b);  // Raw bytes
}
```

**UTF-8 Encoding:**
- Variable-width encoding (1-4 bytes per character)
- No direct indexing by position
- Use `.chars()` for Unicode scalar values
- Use `.bytes()` for raw bytes
- Use slicing with extreme caution

**📖 Deep Dive:** For a comprehensive understanding of Unicode, UTF-8, and why Rust strings work the way they do, see the dedicated Zettelkasten article: **[[Unicode, UTF-8, and Rust]]** (`zettelkasten/unicode-utf8-rust.md`)

This article covers:
- Unicode fundamentals and code points
- UTF-8 encoding rules and byte patterns
- Why Rust disallows `string[index]`
- The three string perspectives (bytes, scalar values, grapheme clusters)
- Performance characteristics and common pitfalls
- Real-world examples with multilingual text
- Best practices for safe string handling

---

### 3. **Hash Maps: `HashMap<K, V>`**

Hash maps store key-value pairs using a hashing function.

**Key Operations:**
```rust
use std::collections::HashMap;

// Creating hash maps
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Accessing values
let team_name = String::from("Blue");
let score = scores.get(&team_name);  // Returns Option<&V>

// Iterating
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// Updating values
scores.insert(String::from("Blue"), 25);  // Overwrite

// Only insert if key doesn't exist
scores.entry(String::from("Yellow")).or_insert(50);

// Update based on old value
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;  // Frequency counting pattern
}
```

**Ownership Rules:**
- Types that implement `Copy` trait (like `i32`) are copied into the map
- Owned values (like `String`) are moved into the map
- References can be inserted but must have valid lifetimes

**Performance:**
- Average O(1) for insert, get, remove
- Hashing function is SipHash (secure but not fastest)
- Can use custom hashers for performance

---

## 🔑 Key Takeaways

### Collection Comparison

| Feature | Vector | String | HashMap |
|---------|--------|--------|---------|
| **Storage** | Contiguous | Contiguous | Hash table |
| **Element Type** | Same type | Bytes (UTF-8) | Key-value pairs |
| **Indexing** | `v[i]` allowed | `s[i]` not allowed | `map[k]` not typical |
| **Growth** | Automatic | Automatic | Automatic |
| **Iteration** | `for i in &v` | `for c in s.chars()` | `for (k, v) in &map` |
| **Use Case** | List of items | Text | Lookups |

### Common Patterns

#### **1. Storing Related Data**
```rust
// Enum with different variants
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

#### **2. Frequency Counting**
```rust
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

#### **3. String Building**
```rust
let mut s = String::new();
for i in 0..10 {
    s.push_str(&format!("Line {}\n", i));
}
```

---

## 📁 Project Structure

This chapter includes four example projects:

### **1. `vectors/`** - Vector Operations
Complete examples of vector creation, manipulation, and iteration:
- Creating and initializing vectors
- Adding and removing elements
- Accessing elements safely
- Iterating and transforming
- Using enums for multiple types
- Performance considerations

### **2. `strings/`** - String Manipulation
Comprehensive string handling examples:
- Creating and updating strings
- Concatenation methods
- UTF-8 encoding and slicing
- Character and byte iteration
- Common string operations
- Unicode handling

### **3. `hashmaps/`** - Hash Map Usage
Hash map patterns and best practices:
- Creating and populating maps
- Accessing and updating values
- Entry API patterns
- Frequency counting
- Ownership and borrowing with maps
- Performance considerations

### **4. `exercises/`** - Practice Problems
End-of-chapter exercises from the Rust Book:
- Convert integers to Pig Latin
- Store employee names in departments
- Retrieve and display department data

---

## 🚀 Running the Examples

### **Vector Examples**
```bash
cd rust_book/Ch8/vectors
cargo run
```

### **String Examples**
```bash
cd rust_book/Ch8/strings
cargo run
```

### **HashMap Examples**
```bash
cd rust_book/Ch8/hashmaps
cargo run
```

### **Practice Exercises**
```bash
cd rust_book/Ch8/exercises
cargo run
```

---

## 🎓 Learning Progression

**Prerequisites:**
- ✅ Ch4: Ownership, borrowing, references
- ✅ Ch5: Structs and methods
- ✅ Ch6: Enums and pattern matching

**Current Focus:**
- 🔄 Ch8: Common collections (this chapter)

**Next Steps:**
- ⏭️ Ch9: Error handling with Result and Option
- ⏭️ Ch10: Generics, traits, and lifetimes

---

## 🔗 Integration with Learning System

### **Mission Applications**
Collections are fundamental to all missions:
- [[Mission1 Overview]] - Stack using Vec<T>
- [[Mission2 Overview]] - Queue implementations with Vec
- [[Mission3 Overview]] - Binary search on sorted vectors
- [[Mission4 Overview]] - Linked lists as alternative to Vec
- [[Mission5 Overview]] - Custom HashMap implementation
- [[Mission6 Overview]] - Grid storage with Vec<Vec<T>>

### **Daily Study Connections**
- [[Day 10 - HashMap Basics]] - HashMap fundamentals
- [[Day 11 - HashSet Operations]] - Set-based collections
- [[daily_study/rust_learning_week2_notes/Day12]] - Ordered collections comparison
- [[Week 2 Overview]] - Collections mastery week

### **Related Rust Book Chapters**
- [[Ch4 - Ownership]] - Foundation for collection ownership
- [[Ch6 - Enums]] - Option and Result with collections
- [[Ch10 - Generics]] - Generic collection types
- [[Ch13 - Iterators]] - Functional programming with collections
- [[Ch15 - Smart Pointers]] - Box, Rc, RefCell with collections

### **Core Concepts Deep Dives**
- [[Vec]] - Vector internals and patterns
- [[String]] - String internals and UTF-8
- [[HashMap]] - Hash map implementation details
- [[Collections MOC]] - Overview of all collection types
- [[Iterators]] - Iterator patterns and combinators
- [[Heap Allocation]] - Dynamic memory management
- [[Performance Optimization]] - Collection performance tips

---

## 💡 Best Practices

### **Choosing the Right Collection**

**Use `Vec<T>` when:**
- You need ordered, indexed access
- You'll mostly append to the end
- You need contiguous memory
- You want cache-friendly iteration

**Use `String` when:**
- You need owned, mutable text
- You're building strings dynamically
- You need UTF-8 validation
- You want string-specific methods

**Use `HashMap<K, V>` when:**
- You need fast key-based lookups
- Order doesn't matter
- You need to associate values with keys
- You're doing frequency counting or caching

### **Performance Tips**

1. **Pre-allocate capacity** if size is known:
   ```rust
   let mut v = Vec::with_capacity(100);
   ```

2. **Use string slices `&str`** when you don't need ownership:
   ```rust
   fn process(s: &str) { /* ... */ }
   ```

3. **Reuse allocations** instead of creating new collections:
   ```rust
   v.clear();  // Keeps capacity
   ```

4. **Use iterators** for functional transformations:
   ```rust
   let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
   ```

---

## 📖 Further Reading

- [The Rust Book Chapter 8](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Vec Documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [String Documentation](https://doc.rust-lang.org/std/string/struct.String.html)
- [HashMap Documentation](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [Collections in std::collections](https://doc.rust-lang.org/std/collections/)

---

*Tags: #rust-book #chapter8 #collections #vector #string #hashmap #heap-allocation #dynamic-data*

*Links: [[zettel-index]] | [[Collections MOC]] | [[Vec]] | [[String]] | [[HashMap]] | [[Mission5 Overview]] | [[Daily Study MOC]] | [[MONTHLY_CALENDAR]]*

---

*This chapter teaches essential data structures for managing dynamic data in Rust, forming the foundation for real-world application development.*
