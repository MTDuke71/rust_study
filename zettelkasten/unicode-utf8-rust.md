# Unicode, UTF-8, and Rust String Handling

*Created: 2025-10-10*  
*Source: Rust Book Chapter 8 (Strings), Unicode Consortium Standards*

---

## Overview

Unicode and UTF-8 are fundamental to understanding Rust's `String` type and why string handling in Rust is different from languages like C or Python. Rust's strict approach to string indexing prevents common bugs while requiring developers to understand text encoding at a deeper level.

**Key Insight:** Rust's `String` type is a wrapper around `Vec<u8>` with UTF-8 encoding guarantees. This design choice prioritizes correctness and international text support over convenience.

---

## What is Unicode?

### The Problem Unicode Solves

Before Unicode, different character encoding systems existed for different languages:
- **ASCII** (American): 128 characters (7 bits)
- **Latin-1** (Western Europe): 256 characters (8 bits)
- **Shift-JIS** (Japanese), **GB2312** (Chinese), **KOI8-R** (Russian), etc.

**Problem:** Files created in one encoding became gibberish when opened with another. International communication was fragmented.

### Unicode's Solution

**Unicode** is a universal character set that assigns a unique number (called a **code point**) to every character in every writing system.

**Key Concepts:**
- **Code Point:** A number from `U+0000` to `U+10FFFF` (over 1.1 million possible values)
- **Plane:** Unicode is divided into 17 planes (0-16)
  - **Plane 0 (BMP - Basic Multilingual Plane):** `U+0000` to `U+FFFF` - Most common characters
  - **Planes 1-16:** Supplementary characters (emoji, historical scripts, etc.)

**Examples:**
```
'A' = U+0041  (Latin capital letter A)
'文' = U+6587  (Chinese character for "text")
'🦀' = U+1F980 (Crab emoji - Rust's unofficial mascot!)
'नम' = U+0928 U+092E (Devanagari "na" + "ma")
```

### Scalar Values vs. Grapheme Clusters

**Unicode Scalar Value:** A single code point (U+0000 to U+D7FF and U+E000 to U+10FFFF)
- Excludes surrogate pairs (U+D800 to U+DFFF) used in UTF-16

**Grapheme Cluster:** What humans perceive as a single "character"
- May be composed of multiple scalar values
- Example: "é" can be:
  - Single scalar: `U+00E9` (é precomposed)
  - Two scalars: `U+0065` (e) + `U+0301` (combining acute accent)

**Rust's `.chars()` iterates over scalar values, NOT grapheme clusters!**

---

## UTF-8 Encoding

### Why UTF-8?

**UTF-8** is a variable-width character encoding that represents Unicode code points using 1 to 4 bytes.

**Advantages:**
1. **ASCII Compatibility:** First 128 characters (U+0000 to U+007F) use 1 byte with same values as ASCII
2. **Space Efficient:** Common characters use fewer bytes
3. **Self-Synchronizing:** You can find character boundaries by examining bytes
4. **No Byte Order Mark (BOM) needed:** Unlike UTF-16/UTF-32

**Why Rust Chose UTF-8:**
- Web standard (HTML, JSON, URLs)
- Most efficient for English/Latin text (dominant in programming)
- Prevents endianness issues
- Widely supported across platforms

### UTF-8 Encoding Rules

| Unicode Range         | UTF-8 Bytes | Byte Pattern                                    | Example           |
| --------------------- | ----------- | ----------------------------------------------- | ----------------- |
| U+0000 to U+007F      | 1 byte      | `0xxxxxxx`                                      | 'A' = `0x41`      |
| U+0080 to U+07FF      | 2 bytes     | `110xxxxx 10xxxxxx`                             | '¢' = `0xC2 0xA2` |
| U+0800 to U+FFFF      | 3 bytes     | `1110xxxx 10xxxxxx 10xxxxxx`                    | '€' = `0xE2 0x82 0xAC` |
| U+10000 to U+10FFFF   | 4 bytes     | `11110xxx 10xxxxxx 10xxxxxx 10xxxxxx`           | '🦀' = `0xF0 0x9F 0xA6 0x80` |

**Key Pattern:** 
- Leading byte indicates how many bytes in the character
- Continuation bytes always start with `10`

### UTF-8 Example Breakdown

**Example: "Hello, 世界!"**

```rust
let s = String::from("Hello, 世界!");

// Byte representation
println!("Bytes: {:?}", s.as_bytes());
// [72, 101, 108, 108, 111, 44, 32, 228, 184, 150, 231, 149, 140, 33]

// Character representation
for c in s.chars() {
    println!("{} = U+{:04X}", c, c as u32);
}
```

**Output:**
```
H = U+0048  (1 byte)
e = U+0065  (1 byte)
l = U+006C  (1 byte)
l = U+006C  (1 byte)
o = U+006F  (1 byte)
, = U+002C  (1 byte)
  = U+0020  (1 byte)
世 = U+4E16  (3 bytes: 0xE4 0xB8 0x96)
界 = U+754C  (3 bytes: 0xE7 0x95 0x8C)
! = U+0021  (1 byte)
```

**Total:** 14 bytes for 10 characters

---

## Why Rust Doesn't Allow `string[index]`

### The Three Perspectives on Strings

Rust considers strings from three viewpoints:

#### 1. **Bytes** (`u8` values)
```rust
let s = String::from("नमस्ते");
println!("{:?}", s.as_bytes());
// [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
// 18 bytes total
```

#### 2. **Scalar Values** (Unicode code points)
```rust
let s = String::from("नमस्ते");
for c in s.chars() {
    println!("{}", c);
}
// Output: न, म, स, ्, त, े
// 6 scalar values (note: ् and े are combining marks)
```

#### 3. **Grapheme Clusters** (perceived characters)
```rust
// Requires external crate: unicode-segmentation
use unicode_segmentation::UnicodeSegmentation;

let s = String::from("नमस्ते");
for g in s.graphemes(true) {
    println!("{}", g);
}
// Output: न, म, स्, ते
// 4 grapheme clusters (what humans see as "letters")
```

### The Indexing Problem

**What should `s[0]` return?**

```rust
let hello = String::from("Здравствуйте");

// Option 1: First byte?
// s[0] = 208 (0xD0) - Not useful, partial character!

// Option 2: First scalar value?
// s[0] = 'З' - Reasonable, but requires scanning for UTF-8 boundaries

// Option 3: First grapheme cluster?
// s[0] = "З" - Most intuitive, but complex (needs Unicode algorithm)
```

**Rust's Decision:** **Disallow indexing entirely** to avoid:
1. **Performance surprises:** `s[n]` would be O(n), not O(1) as expected
2. **Ambiguity:** No clear consensus on what should be returned
3. **Bugs:** Easy to accidentally slice mid-character

### Safe Alternatives to Indexing

#### 1. **Iterating Over Characters**
```rust
let s = String::from("नमस्ते");

// Unicode scalar values
for c in s.chars() {
    println!("{}", c);
}

// Bytes
for b in s.bytes() {
    println!("{}", b);
}

// Grapheme clusters (requires external crate)
use unicode_segmentation::UnicodeSegmentation;
for g in s.graphemes(true) {
    println!("{}", g);
}
```

#### 2. **Slicing (With Caution)**
```rust
let hello = "Здравствуйте";

// Valid slice (on character boundaries)
let s = &hello[0..4];  // "Зд" (each char is 2 bytes)
println!("{}", s);

// Invalid slice (mid-character) - PANICS!
// let s = &hello[0..1];  // ❌ Panic: byte index 1 is not a char boundary
```

**Rule:** Only slice at valid UTF-8 character boundaries. Use `.char_indices()` to find safe boundaries.

#### 3. **Using `.chars().nth(n)`**
```rust
let s = String::from("Hello");

// Get the 2nd character (0-indexed)
if let Some(c) = s.chars().nth(2) {
    println!("3rd character: {}", c);  // 'l'
}

// ⚠️ Warning: O(n) operation!
```

#### 4. **Collecting into Vec for Random Access**
```rust
let s = String::from("Hello, 世界!");
let chars: Vec<char> = s.chars().collect();

// Now O(1) indexing on scalar values
println!("4th char: {}", chars[3]);  // 'l'
println!("9th char: {}", chars[8]);  // '界'
```

---

## Rust String Types Compared

### `String` vs `&str`

| Feature              | `String`                    | `&str`                        |
| -------------------- | --------------------------- | ----------------------------- |
| **Ownership**        | Owned                       | Borrowed reference            |
| **Mutability**       | Can be mutable              | Immutable                     |
| **Storage**          | Heap-allocated              | Can be stack, heap, or static |
| **Growth**           | Growable                    | Fixed size                    |
| **Type**             | `Vec<u8>` wrapper           | Slice `&[u8]`                 |
| **Common Source**    | `String::from("hello")`     | `"hello"` (string literal)    |
| **Conversion**       | `s.as_str()` or `&s`        | `s.to_string()` or `.into()`  |

**Mental Model:**
- `String` is like `Vec<T>` - owns its data
- `&str` is like `&[T]` - borrows a view into string data

### String Literal Storage

```rust
// String literal: stored in binary, lives for entire program
let s1: &str = "Hello, world!";  // &'static str

// Heap-allocated String: can grow and shrink
let s2: String = String::from("Hello");

// Conversion
let s3: &str = &s2;           // String -> &str (deref coercion)
let s4: String = s1.to_string();  // &str -> String (allocation)
```

---

## Common Rust String Operations

### 1. **Creating Strings**

```rust
// From string literals
let s1 = String::from("hello");
let s2 = "hello".to_string();
let s3 = String::new();  // Empty string

// From other types
let s4 = format!("The answer is {}", 42);
let s5 = 123.to_string();
```

### 2. **Appending**

```rust
let mut s = String::from("foo");

// Append string slice
s.push_str("bar");

// Append single character
s.push('!');

println!("{}", s);  // "foobar!"
```

### 3. **Concatenation**

```rust
// Using + operator (takes ownership of left operand)
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1 moved here, s2 borrowed
// println!("{}", s1);  // ❌ Error: s1 moved

// Using format! macro (doesn't take ownership)
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);
println!("{}-{}-{}", s1, s2, s3);  // ✅ Still valid
```

**Why does `+` take ownership?**
```rust
// The + operator uses this signature:
fn add(self, s: &str) -> String { /* ... */ }

// It takes ownership of 'self' (left operand) and borrows right operand
```

### 4. **Searching and Pattern Matching**

```rust
let s = String::from("hello world");

// Contains
assert!(s.contains("world"));

// Starts with / ends with
assert!(s.starts_with("hello"));
assert!(s.ends_with("world"));

// Find position
assert_eq!(s.find("world"), Some(6));

// Replace
let s2 = s.replace("world", "Rust");
assert_eq!(s2, "hello Rust");

// Split
for word in s.split_whitespace() {
    println!("{}", word);
}
```

### 5. **Case Conversion**

```rust
let s = String::from("Hello, World!");

let lower = s.to_lowercase();  // "hello, world!"
let upper = s.to_uppercase();  // "HELLO, WORLD!"

// ⚠️ These allocate new Strings!
```

### 6. **Trimming**

```rust
let s = String::from("  hello  \n");

let trimmed = s.trim();        // "hello"
let left = s.trim_start();     // "hello  \n"
let right = s.trim_end();      // "  hello"

// Trim specific characters
let s2 = "***hello***";
let trimmed = s2.trim_matches('*');  // "hello"
```

### 7. **Safe Character Access**

```rust
let s = String::from("Hello, 世界!");

// Get character at position (O(n))
if let Some(c) = s.chars().nth(7) {
    println!("Char at position 7: {}", c);  // '世'
}

// Iterate with indices
for (i, c) in s.char_indices() {
    println!("Byte {} = {}", i, c);
}
// Output:
// Byte 0 = H
// Byte 1 = e
// Byte 2 = l
// Byte 3 = l
// Byte 4 = o
// Byte 5 = ,
// Byte 6 =  
// Byte 7 = 世  (3 bytes)
// Byte 10 = 界 (3 bytes)
// Byte 13 = !
```

---

## Performance Characteristics

### Time Complexity

| Operation                | Complexity | Notes                                    |
| ------------------------ | ---------- | ---------------------------------------- |
| `s.len()`                | O(1)       | Returns byte count, not char count       |
| `s.is_empty()`           | O(1)       |                                          |
| `s.push(ch)`             | O(1)*      | Amortized, may reallocate                |
| `s.push_str(slice)`      | O(m)       | m = length of slice                      |
| `s.chars().count()`      | O(n)       | Must scan for UTF-8 boundaries           |
| `s.chars().nth(k)`       | O(k)       | Must iterate to position k               |
| `s[i..j]`                | O(1)       | If valid boundaries (panics otherwise)   |
| `s.chars().collect()`    | O(n)       | Builds Vec<char>                         |

**Key Insight:** `.len()` returns **byte count**, not **character count**!

```rust
let s = String::from("Hello");
assert_eq!(s.len(), 5);  // 5 bytes, 5 characters

let s = String::from("世界");
assert_eq!(s.len(), 6);  // 6 bytes, 2 characters
assert_eq!(s.chars().count(), 2);  // ⚠️ O(n) operation!
```

### Memory Layout

```rust
let s = String::from("Hello");

// String struct (stack):
// - ptr: pointer to heap data
// - len: 5 (current bytes used)
// - capacity: 5 (allocated bytes)

// Heap: [H][e][l][l][o]
```

**Growth Strategy:**
- Doubling capacity when full (similar to `Vec<T>`)
- `String::with_capacity(n)` pre-allocates to avoid reallocations

---

## Common Pitfalls and Solutions

### Pitfall 1: Assuming `len()` Returns Character Count

```rust
// ❌ Wrong
let s = String::from("世界");
println!("Length: {}", s.len());  // 6 (bytes), not 2!

// ✅ Correct
println!("Character count: {}", s.chars().count());  // 2
```

### Pitfall 2: Slicing at Invalid Boundaries

```rust
let s = String::from("世界");

// ❌ Panic: byte index 1 is not a char boundary
// let slice = &s[0..1];

// ✅ Use char_indices() to find safe boundaries
for (i, c) in s.char_indices() {
    println!("Valid slice start: {}", i);  // 0, 3
}

let slice = &s[0..3];  // "世" (safe)
```

### Pitfall 3: Expecting O(1) Character Access

```rust
// ❌ Inefficient: O(n) for each access
let s = String::from("Hello");
for i in 0..s.len() {
    if let Some(c) = s.chars().nth(i) {
        println!("{}", c);
    }
}

// ✅ Efficient: O(n) total
for c in s.chars() {
    println!("{}", c);
}

// ✅ If random access needed: convert once
let chars: Vec<char> = s.chars().collect();
for i in 0..chars.len() {
    println!("{}", chars[i]);  // O(1) access
}
```

### Pitfall 4: Using `+` with Multiple Strings

```rust
// ❌ Ugly and inefficient
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = s1 + "-" + &s2 + "-" + &s3;  // s1 moved

// ✅ Use format! macro
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);  // No moves
```

### Pitfall 5: Forgetting UTF-8 Validation

```rust
use std::str;

// ❌ Invalid UTF-8
let bytes = vec![0xFF, 0xFF];
// let s = String::from_utf8(bytes).unwrap();  // Panics!

// ✅ Handle invalid UTF-8
let bytes = vec![0xFF, 0xFF];
match String::from_utf8(bytes) {
    Ok(s) => println!("Valid: {}", s),
    Err(e) => println!("Invalid UTF-8: {}", e),
}

// ✅ Lossy conversion (replaces invalid with �)
let bytes = vec![0xFF, 0xFF];
let s = String::from_utf8_lossy(&bytes);
println!("{}", s);  // "��"
```

---

## Working with Non-ASCII Text

### Example: Processing Multilingual Text

```rust
fn analyze_string(s: &str) {
    println!("String: \"{}\"", s);
    println!("Byte length: {}", s.len());
    println!("Character count: {}", s.chars().count());
    
    println!("\nCharacters:");
    for (i, c) in s.char_indices() {
        println!("  Byte {}: {} (U+{:04X})", i, c, c as u32);
    }
    
    println!("\nBytes:");
    for (i, b) in s.as_bytes().iter().enumerate() {
        println!("  {}: 0x{:02X}", i, b);
    }
}

fn main() {
    analyze_string("Hello");
    analyze_string("世界");
    analyze_string("नमस्ते");
    analyze_string("🦀");
}
```

### Example: Safe String Truncation

```rust
fn truncate_at_char_boundary(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    
    // Find the last valid character boundary <= max_bytes
    let mut boundary = max_bytes;
    while boundary > 0 && !s.is_char_boundary(boundary) {
        boundary -= 1;
    }
    
    &s[..boundary]
}

fn main() {
    let s = "Hello, 世界!";
    println!("{}", truncate_at_char_boundary(s, 10));  // "Hello, 世"
    println!("{}", truncate_at_char_boundary(s, 9));   // "Hello, "
}
```

### Example: Grapheme Cluster Handling

```rust
// Requires: cargo add unicode-segmentation
use unicode_segmentation::UnicodeSegmentation;

fn count_graphemes(s: &str) -> usize {
    s.graphemes(true).count()
}

fn main() {
    let s = "नमस्ते";
    
    println!("String: {}", s);
    println!("Bytes: {}", s.len());                    // 18
    println!("Scalar values: {}", s.chars().count());  // 6
    println!("Grapheme clusters: {}", count_graphemes(s));  // 4
    
    // What humans see as "letters"
    println!("\nGrapheme clusters:");
    for (i, g) in s.graphemes(true).enumerate() {
        println!("  {}: {}", i, g);
    }
    // Output:
    // 0: न
    // 1: म
    // 2: स्
    // 3: ते
}
```

---

## Rust vs Other Languages

### C/C++: Undefined Behavior Risk

```c
// C: char is 1 byte, no UTF-8 awareness
char str[] = "世界";
char first = str[0];  // Gets 0xE4 (meaningless byte)
```

**Rust prevents this at compile time!**

### Python: Runtime Overhead

```python
# Python 3: strings are Unicode by default
s = "Hello"
c = s[2]  # O(1) for ASCII, but...

s = "世界"
c = s[0]  # "世" - looks O(1), but internal complexity
```

**Rust makes the cost explicit.**

### JavaScript: UTF-16 Confusion

```javascript
// JavaScript: strings are UTF-16
let s = "🦀";  // U+1F980 (requires 2 UTF-16 code units)
console.log(s.length);     // 2 (not 1!)
console.log(s.charAt(0));  // "\uD83E" (high surrogate - nonsense)
```

**Rust's UTF-8 is more consistent with byte reality.**

### Go: Similar Philosophy

```go
// Go: strings are UTF-8, no direct indexing
s := "世界"
// c := s[0]  // Gets byte 0xE4, not character

// Use rune (code point) iteration
for i, r := range s {
    fmt.Printf("%d: %c\n", i, r)
}
```

**Rust and Go share similar UTF-8 design philosophy.**

---

## Best Practices

### 1. **Use `&str` for Function Parameters**

```rust
// ❌ Less flexible
fn process(s: String) { /* ... */ }

// ✅ More flexible (accepts String, &str, etc.)
fn process(s: &str) { /* ... */ }

// Caller
let s1 = String::from("hello");
let s2 = "world";
process(&s1);  // Works
process(s2);   // Works
```

### 2. **Avoid Repeated `chars().nth(i)` in Loops**

```rust
// ❌ O(n²) - repeatedly scans from start
for i in 0..s.chars().count() {
    if let Some(c) = s.chars().nth(i) {
        println!("{}", c);
    }
}

// ✅ O(n) - single iteration
for c in s.chars() {
    println!("{}", c);
}
```

### 3. **Use `format!` for Complex String Building**

```rust
// ❌ Hard to read
let s = name.clone() + " is " + &age.to_string() + " years old";

// ✅ Clear and efficient
let s = format!("{} is {} years old", name, age);
```

### 4. **Pre-allocate for Known Sizes**

```rust
// ❌ Multiple reallocations
let mut s = String::new();
for i in 0..1000 {
    s.push_str("data");
}

// ✅ Single allocation
let mut s = String::with_capacity(1000 * 4);
for i in 0..1000 {
    s.push_str("data");
}
```

### 5. **Use External Crates for Advanced Unicode**

```rust
// For grapheme clusters
use unicode_segmentation::UnicodeSegmentation;

// For Unicode normalization (NFD, NFC, NFKD, NFKC)
use unicode_normalization::UnicodeNormalization;

// For Unicode width (terminal display width)
use unicode_width::UnicodeWidthStr;

let s = "世界";
println!("Display width: {}", s.width());  // 4 (CJK chars are wide)
```

---

## Testing String Code

### Unit Test Examples

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_length() {
        let s = String::from("hello");
        assert_eq!(s.len(), 5);
        assert_eq!(s.chars().count(), 5);
    }

    #[test]
    fn test_multibyte_length() {
        let s = String::from("世界");
        assert_eq!(s.len(), 6);  // 6 bytes
        assert_eq!(s.chars().count(), 2);  // 2 characters
    }

    #[test]
    fn test_emoji() {
        let s = String::from("🦀");
        assert_eq!(s.len(), 4);  // 4 bytes
        assert_eq!(s.chars().count(), 1);  // 1 scalar value
    }

    #[test]
    fn test_char_iteration() {
        let s = String::from("Hello");
        let chars: Vec<char> = s.chars().collect();
        assert_eq!(chars, vec!['H', 'e', 'l', 'l', 'o']);
    }

    #[test]
    fn test_valid_slice() {
        let s = String::from("Hello, 世界!");
        let slice = &s[0..7];  // "Hello, " (7 bytes, 7 chars)
        assert_eq!(slice, "Hello, ");
    }

    #[test]
    #[should_panic(expected = "byte index 1 is not a char boundary")]
    fn test_invalid_slice_panics() {
        let s = String::from("世界");
        let _ = &s[0..1];  // Mid-character slice
    }

    #[test]
    fn test_char_indices() {
        let s = String::from("a世b");
        let indices: Vec<usize> = s.char_indices().map(|(i, _)| i).collect();
        assert_eq!(indices, vec![0, 1, 4]);  // 'a' at 0, '世' at 1, 'b' at 4
    }
}
```

---

## Real-World Applications

### 1. **Parsing Log Files with Mixed Encodings**

```rust
fn sanitize_log_line(line: &str) -> String {
    line.chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
        .collect()
}
```

### 2. **Generating Slugs from Unicode Titles**

```rust
fn make_slug(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

// "Hello, 世界!" → "hello--世界-"
```

### 3. **Validating User Input**

```rust
fn is_valid_username(name: &str) -> bool {
    const MIN_LEN: usize = 3;
    const MAX_LEN: usize = 20;
    
    let char_count = name.chars().count();
    char_count >= MIN_LEN 
        && char_count <= MAX_LEN
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}
```

### 4. **Text Truncation for Display**

```rust
fn truncate_display(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().take(max_chars).collect();
    let mut result: String = chars.into_iter().collect();
    
    if s.chars().count() > max_chars {
        result.push_str("...");
    }
    
    result
}
```

---

## Related Concepts for Further Study

### 1. **String Interning**
Storing only one copy of each distinct string value. Rust doesn't have built-in interning, but crates like `string_cache` provide it.

### 2. **Zero-Copy String Parsing**
Using `&str` slices to avoid allocations when parsing text.

### 3. **Rope Data Structures**
Efficient string representation for text editors (large insertions/deletions).

### 4. **Small String Optimization (SSO)**
Storing short strings inline to avoid heap allocation. `SmartString` and `SmallVec` crates provide this.

### 5. **Unicode Normalization**
Converting strings to canonical forms (NFC, NFD, NFKC, NFKD) for comparison.

---

## Summary: Key Takeaways

1. **Unicode assigns code points** to all characters; **UTF-8 encodes** them in 1-4 bytes
2. **Rust's `String` is UTF-8 encoded** - guarantees valid Unicode at all times
3. **No direct indexing** (`s[i]`) because:
   - Not clear what to return (byte? scalar? grapheme?)
   - Would be O(n), not O(1) as expected
   - Easy to slice mid-character and panic
4. **Use `.chars()`** for Unicode scalar values (most common use case)
5. **Use `.bytes()`** for raw byte access (parsing protocols)
6. **Use `unicode-segmentation`** for grapheme clusters (what humans see)
7. **`.len()` returns bytes**, not character count
8. **`.chars().count()` is O(n)** - avoid in loops
9. **`&str` is preferred** for function parameters (more flexible)
10. **`format!` macro** is idiomatic for string building

---

*Tags: #unicode #utf8 #string #rust #text-encoding #internationalization #performance #collections #rust-book-ch8*

*Links: [[zettel-index]] | [[rust_book/rust-book-ch8]] | [[String Type]] | [[Vec Type]] | [[Collections MOC]] | [[Performance Optimization]] | [[Daily Study MOC]]*

---

## Quick Reference Card

```rust
// Creation
let s = String::from("hello");
let s = "hello".to_string();

// Iteration
for c in s.chars() { }      // Unicode scalar values
for b in s.bytes() { }      // Raw bytes
for (i, c) in s.char_indices() { }  // Index + char

// Length
s.len()                     // Byte count (O(1))
s.chars().count()           // Character count (O(n))
s.is_empty()                // Check if empty (O(1))

// Access
s.chars().nth(n)            // nth character (O(n))
&s[i..j]                    // Byte slice (must be valid boundaries)

// Modification
s.push_str("world");        // Append &str
s.push('!');                // Append char
format!("{}{}", s1, s2)     // Concatenate without move

// Conversion
s.as_str()                  // String → &str
s.to_string()               // &str → String
String::from_utf8(vec)      // Vec<u8> → Result<String>
String::from_utf8_lossy(v)  // Vec<u8> → Cow<str> (replaces invalid)
```

**Print this card and keep it nearby while learning Rust strings!**
