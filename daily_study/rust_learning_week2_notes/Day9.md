# Day 9 · Strings - `String` vs `&str`, UTF-8, Manipulation

## 🔗 Zettelkasten Links
- **Previous**: [[Day 8 - Vectors]] - Dynamic arrays
- **Next**: [[Day 10 - HashMap Basics]] - Key-value storage
- **Concept**: [[Rust Concepts MOC]] - String types and UTF-8
- **Rust Book**: [[Chapter 8.2 - Strings]] - Storing UTF-8 text
- **Week Summary**: [[Day 14 - Week 2 Summary]] - Collections review

## 🎯 Learning Objectives
- Understand the difference between `String` and `&str`
- Master UTF-8 encoding and Unicode handling
- Learn efficient string manipulation patterns
- Practice common string operations for AoC problems

---

## 📚 Core Concepts

### **String vs &str - The Fundamental Distinction**

| Type | Ownership | Location | Mutability | Use Case |
|------|-----------|----------|------------|----------|
| `String` | Owned | Heap | Mutable | Dynamic content, building strings |
| `&str` | Borrowed | Stack/Heap/Binary | Immutable | String literals, function parameters |

```rust
// String - owned, heap-allocated, growable
let mut owned = String::from("Hello");
owned.push_str(", World!");
println!("{}", owned); // "Hello, World!"

// &str - borrowed string slice, immutable
let borrowed: &str = "Hello, World!";      // String literal
let slice: &str = &owned[0..5];            // Slice of String
println!("{}", borrowed); // "Hello, World!"
println!("{}", slice);    // "Hello"
```

### **Memory Layout Visualization**
```
String {                    &str
  ptr: *mut u8,  ─────────►  "Hello, World!"
  len: 13,                   │ │ │ │ │ │ │
  cap: 16,                   H e l l o , space W o r l d !
}                            └─ UTF-8 bytes ─┘
│
Heap: [H][e][l][l][o][,][ ][W][o][r][l][d][!][?][?][?]
       └────────── used ──────────┘└─── capacity ─┘
```

---

## 🔤 UTF-8 and Unicode Deep Dive

### **Understanding UTF-8 Encoding**

```rust
// UTF-8 is variable-width encoding
let ascii = "Hello";           // 1 byte per char
let emoji = "Hello 👋";        // 4 bytes for 👋
let chinese = "你好";          // 3 bytes per Chinese character

println!("ASCII: {} bytes, {} chars", ascii.len(), ascii.chars().count());
// Output: ASCII: 5 bytes, 5 chars

println!("Emoji: {} bytes, {} chars", emoji.len(), emoji.chars().count());  
// Output: Emoji: 10 bytes, 7 chars

println!("Chinese: {} bytes, {} chars", chinese.len(), chinese.chars().count());
// Output: Chinese: 6 bytes, 2 chars
```

### **Safe Unicode Handling**
```rust
// ❌ WRONG - byte indexing can panic on non-ASCII
let text = "café";
// let c = text[2]; // ❌ Compile error - no Index trait

// ✅ CORRECT - use char-based indexing
let chars: Vec<char> = text.chars().collect();
let c = chars[2]; // 'f'

// ✅ Or use safe slicing methods
let slice = text.get(0..2); // Option<&str>
match slice {
    Some(s) => println!("Safe slice: {}", s),
    None => println!("Invalid byte boundary"),
}
```

### **Unicode Normalization Issues**
```rust
// These look the same but are different!
let composed = "é";       // Single char: U+00E9 (LATIN SMALL LETTER E WITH ACUTE)
let decomposed = "é";     // Two chars: U+0065 + U+0301 (e + combining acute)

println!("Composed len: {}", composed.chars().count());    // 1
println!("Decomposed len: {}", decomposed.chars().count()); // 2
println!("Equal? {}", composed == decomposed);              // false!

// For proper Unicode comparison, use unicode-normalization crate
```

---

## 🛠️ String Creation Patterns

### **From Literals**
```rust
// String literals are &str
let literal = "hello world";

// Convert to owned String
let owned1 = String::from("hello world");
let owned2 = "hello world".to_string();
let owned3 = "hello world".to_owned();

// All are equivalent, to_string() is most common
```

### **From Other Types**
```rust
// From numbers
let num_str = 42.to_string();
let formatted = format!("Number: {}", 42);

// From bytes (be careful with UTF-8!)
let bytes = vec![72, 101, 108, 108, 111]; // "Hello" in ASCII
let from_bytes = String::from_utf8(bytes).unwrap();

// From iterator of chars
let chars = vec!['H', 'e', 'l', 'l', 'o'];
let from_chars: String = chars.into_iter().collect();
```

### **Building Strings Efficiently**
```rust
// ❌ Inefficient - multiple allocations
let mut result = String::new();
for i in 0..1000 {
    result = result + &i.to_string(); // Creates new String each time!
}

// ✅ Efficient - single allocation with capacity
let mut result = String::with_capacity(4000); // Pre-allocate
for i in 0..1000 {
    result.push_str(&i.to_string()); // Reuses existing allocation
}

// ✅ Most efficient for known data
let result: String = (0..1000).map(|i| i.to_string()).collect();
```

---

## ✂️ String Manipulation Masterclass

### **Slicing and Substrings**
```rust
let text = "Hello, 世界!";

// Safe slicing with get()
let hello = text.get(0..5).unwrap_or("");        // "Hello"
let world = text.get(7..13).unwrap_or("");       // "世界" (6 bytes!)

// Char-based operations
let first_char = text.chars().next().unwrap();            // 'H'
let last_char = text.chars().last().unwrap();             // '!'
let nth_char = text.chars().nth(7).unwrap();               // '世'

// Split operations
let parts: Vec<&str> = text.split(", ").collect();        // ["Hello", "世界!"]
let words: Vec<&str> = text.split_whitespace().collect(); // ["Hello,", "世界!"]
```

### **Pattern Matching and Searching**
```rust
let text = "The quick brown fox jumps over the lazy dog";

// Basic searching
let contains = text.contains("fox");                    // true
let starts = text.starts_with("The");                  // true
let ends = text.ends_with("dog");                      // true

// Position finding
let position = text.find("fox").unwrap();              // 16
let last_pos = text.rfind("the").unwrap();             // 31 (case sensitive!)

// Case insensitive search
let lower_text = text.to_lowercase();
let fox_pos = lower_text.find("fox");

// Multiple matches
let indices: Vec<_> = text.match_indices("the").collect();
// [(31, "the")] - only finds "the", not "The"
```

### **String Replacement and Transformation**
```rust
let text = "Hello, World! World is great!";

// Simple replacement
let replaced = text.replace("World", "Rust");
// "Hello, Rust! Rust is great!"

// Replace first occurrence only
let replaced_once = text.replacen("World", "Rust", 1);
// "Hello, Rust! World is great!"

// Case transformations
let upper = text.to_uppercase();      // "HELLO, WORLD! WORLD IS GREAT!"
let lower = text.to_lowercase();      // "hello, world! world is great!"

// Trimming whitespace
let messy = "  \n  hello world  \t  ";
let clean = messy.trim();             // "hello world"
let left_clean = messy.trim_start();  // "hello world  \t  "
let right_clean = messy.trim_end();   // "  \n  hello world"
```

---

## 🧮 Advanced String Operations

### **Parsing and Validation**
```rust
// Parse from strings
let num: i32 = "42".parse().unwrap();
let float: f64 = "3.14159".parse().unwrap();

// Handle parse errors
let maybe_num = "not_a_number".parse::<i32>();
match maybe_num {
    Ok(n) => println!("Number: {}", n),
    Err(e) => println!("Parse error: {}", e),
}

// Validate string content
fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5
}

// Custom parsing with FromStr trait
use std::str::FromStr;

#[derive(Debug)]
struct Point { x: i32, y: i32 }

impl FromStr for Point {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')').split(',').collect();
        
        if coords.len() != 2 {
            return Err("Invalid format".to_string());
        }
        
        let x = coords[0].trim().parse().map_err(|_| "Invalid x")?;
        let y = coords[1].trim().parse().map_err(|_| "Invalid y")?;
        
        Ok(Point { x, y })
    }
}

// Usage: let point: Point = "(10, 20)".parse().unwrap();
```

### **Format Strings and Interpolation**
```rust
// Basic formatting
let name = "Alice";
let age = 30;
let intro = format!("Hi, I'm {} and I'm {} years old", name, age);

// Positional arguments
let formatted = format!("{1} is {0} years old", age, name);
// "Alice is 30 years old"

// Named arguments
let formatted = format!("{name} is {age} years old", name=name, age=age);

// Formatting options
let pi = 3.14159;
let formatted = format!("{:.2}", pi);           // "3.14" (2 decimal places)
let padded = format!("{:>10}", "hello");        // "     hello" (right-aligned, width 10)
let filled = format!("{:*<10}", "hello");       // "hello*****" (left-aligned, pad with *)

// Debug formatting
let point = Point { x: 10, y: 20 };
println!("{:?}", point);      // Debug format
println!("{:#?}", point);     // Pretty debug format
```

---

## 🎮 AoC-Style String Problems

### **Character Frequency Counting**
```rust
use std::collections::HashMap;

fn count_chars(text: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for ch in text.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts
}

// AoC 2018 Day 2 style
fn checksum(box_ids: &[&str]) -> i32 {
    let mut twos = 0;
    let mut threes = 0;
    
    for id in box_ids {
        let counts = count_chars(id);
        if counts.values().any(|&count| count == 2) { twos += 1; }
        if counts.values().any(|&count| count == 3) { threes += 1; }
    }
    
    twos * threes
}
```

### **String Transformation Puzzles**
```rust
// Caesar cipher (AoC-style encoding)
fn caesar_cipher(text: &str, shift: u8) -> String {
    text.chars().map(|ch| {
        if ch.is_ascii_lowercase() {
            let shifted = ((ch as u8 - b'a' + shift) % 26) + b'a';
            shifted as char
        } else if ch.is_ascii_uppercase() {
            let shifted = ((ch as u8 - b'A' + shift) % 26) + b'A';
            shifted as char
        } else {
            ch
        }
    }).collect()
}

// Password validation (AoC 2020 Day 2 style)
#[derive(Debug)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl FromStr for PasswordPolicy {
    type Err = String;
    
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // Parse "1-3 a: abcde"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("Invalid format".to_string());
        }
        
        let range_parts: Vec<&str> = parts[0].split('-').collect();
        let min = range_parts[0].parse().map_err(|_| "Invalid min")?;
        let max = range_parts[1].parse().map_err(|_| "Invalid max")?;
        
        let letter = parts[1].chars().next().ok_or("No letter")?;
        let password = parts[2].to_string();
        
        Ok(PasswordPolicy { min, max, letter, password })
    }
}

fn is_valid_password(policy: &PasswordPolicy) -> bool {
    let count = policy.password.chars().filter(|&c| c == policy.letter).count();
    count >= policy.min && count <= policy.max
}
```

---

## ⚡ Performance Tips

### **Memory Management**
```rust
// ❌ Inefficient - creates temporary String
fn process_lines_bad(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_uppercase().to_string()).collect()
}

// ✅ Better - avoid unnecessary allocations where possible
fn process_lines_good(input: &str) -> Vec<String> {
    let mut result = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        result.push(line.to_uppercase());
    }
    result
}

// ✅ Best for this case - work with &str when possible
fn count_uppercase_words(input: &str) -> usize {
    input.lines()
        .map(|line| line.split_whitespace().count())
        .sum()
}
```

### **When to Use String vs &str**
```rust
// Use &str for:
fn analyze_text(text: &str) -> usize {     // Function parameters
    text.len()
}

const GREETING: &str = "Hello";            // Constants
let literal = "Hello, World!";             // String literals

// Use String for:
fn build_greeting(name: &str) -> String {  // Return owned strings
    format!("Hello, {}!", name)
}

let mut buffer = String::new();            // Mutable strings
buffer.push_str("Hello");
```

---

## 🧪 Practice Exercises

### **Exercise 1: Word Frequency**
```rust
// Count word frequencies in text (case insensitive)
fn word_frequencies(text: &str) -> HashMap<String, usize> {
    // Your implementation here
    // Hint: split_whitespace(), to_lowercase(), HashMap
    todo!()
}

#[test]
fn test_word_frequencies() {
    let text = "The quick brown fox jumps over the lazy dog";
    let freqs = word_frequencies(text);
    assert_eq!(freqs.get("the"), Some(&2));
    assert_eq!(freqs.get("quick"), Some(&1));
}
```

### **Exercise 2: String Rotation**
```rust
// Check if one string is a rotation of another
fn is_rotation(s1: &str, s2: &str) -> bool {
    // Hint: s1 + s1 contains all rotations of s1
    todo!()
}

#[test]
fn test_rotation() {
    assert!(is_rotation("abcde", "cdeab"));
    assert!(is_rotation("abcde", "eabcd"));
    assert!(!is_rotation("abcde", "abced"));
}
```

### **Exercise 3: AoC-Style Parsing**
```rust
// Parse coordinate pairs from strings like "x=10, y=20"
fn parse_coordinates(input: &str) -> Result<(i32, i32), String> {
    // Your implementation here
    todo!()
}

#[test]
fn test_parse_coordinates() {
    assert_eq!(parse_coordinates("x=10, y=20"), Ok((10, 20)));
    assert_eq!(parse_coordinates("x=-5, y=100"), Ok((-5, 100)));
    assert!(parse_coordinates("invalid").is_err());
}
```

---

## 🎯 Key Takeaways

### **Memory Model Understanding**
- `String` owns its data, can grow, lives on heap
- `&str` borrows data, immutable, can point anywhere
- Always prefer `&str` for function parameters unless you need ownership

### **UTF-8 Safety**
- Rust strings are always valid UTF-8
- Byte indexing is not available - use char-based operations
- Be aware of variable-width encoding for international text

### **Performance Principles**
- Pre-allocate with `String::with_capacity()` when building large strings
- Use string slices (`&str`) to avoid unnecessary copies
- `format!()` is convenient but creates allocations

### **AoC Applications**
- String parsing is crucial for input processing
- Character counting and frequency analysis are common patterns
- String transformations often hide algorithmic problems

## 📚 Further Reading

- [The Rust Book - Chapter 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [String vs &str explanation](https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str)
- [UTF-8 Everywhere manifesto](http://utf8everywhere.org/)
- [Unicode normalization crate](https://docs.rs/unicode-normalization/)

---

**Remember**: Strings in Rust are safe, fast, and Unicode-correct by default. Master the `String` vs `&str` distinction and you'll handle text processing with confidence! 🦀

---

*Links: [[Day 8 - Vectors]] | [[Day 10 - HashMap Basics]] | [[Rust Concepts MOC]]*
*Tags: #string #str #utf8 #text-processing #daily-study #week2 #rust-book #chapter8 #parsing*
