# String Type

*Rust's heap-allocated, growable, UTF-8 encoded string type and its relationship to string literals and slices*

---

## 🎯 **String Type Fundamentals**

### **What is String?**

`String` is Rust's **owned, heap-allocated, growable** string type:

```rust
pub struct String {
    vec: Vec<u8>,  // UTF-8 bytes stored in a Vec
}
```

**Key Properties**:

- ✅ **Owned** - Has exclusive ownership of its data
- ✅ **Growable** - Can be extended and modified
- ✅ **UTF-8 encoded** - Guarantees valid Unicode text
- ✅ **Heap allocated** - Stored on the heap, not the stack

### **String vs &str Relationship**

```rust
// String: Owned, heap-allocated, growable
let owned: String = String::from("Hello, world!");

// &str: Borrowed, can point to stack or heap, immutable
let borrowed: &str = "Hello, world!";        // Points to program binary
let slice: &str = &owned[0..5];              // Points to String's heap data
let static_str: &'static str = "constant";   // Points to program binary
```

## 🏗️ **Creation Patterns**

### **1. From String Literals**

```rust
// Multiple ways to create from &str
let s1 = String::from("hello");
let s2 = "hello".to_string();
let s3 = "hello".to_owned();
let s4 = String::new() + "hello";  // Less efficient

// Performance comparison
// String::from() and to_string() are equivalent and preferred
```

### **2. Empty String Creation**

```rust
// Create empty string
let mut s = String::new();

// With capacity pre-allocation
let mut s = String::with_capacity(100);  // Avoids reallocations

// From iterator
let s: String = ['h', 'e', 'l', 'l', 'o'].iter().collect();
```

### **3. From Other Types**

```rust
// From numbers
let num = 42;
let s = num.to_string();
let s = format!("{}", num);

// From bytes (if valid UTF-8)
let bytes = vec![72, 101, 108, 108, 111]; // "Hello"
let s = String::from_utf8(bytes).unwrap();

// Unsafe from bytes (no UTF-8 validation)
let s = unsafe { String::from_utf8_unchecked(bytes) };
```

## 📝 **Modification Operations**

### **Growing Operations**

```rust
let mut s = String::from("Hello");

// Append string slice
s.push_str(", world!");      // "Hello, world!"

// Append single character
s.push('!');                 // "Hello, world!!"

// Insert at position (expensive - O(n))
s.insert_str(5, " beautiful");   // "Hello beautiful, world!!"
s.insert(0, '→');               // "→Hello beautiful, world!!"

// Extend from iterator
s.extend(" 🦀".chars());        // "→Hello beautiful, world!! 🦀"
```

### **Replacement Operations**

```rust
let mut s = String::from("Hello, world!");

// Replace all occurrences
s = s.replace("world", "Rust");     // "Hello, Rust!"

// Replace first n occurrences
s = s.replacen("l", "L", 2);        // "HeLLo, Rust!"

// In-place replacement patterns
s.replace_range(0..5, "Hi");        // "Hi, Rust!"
```

### **Removal Operations**

```rust
let mut s = String::from("Hello, world!");

// Remove characters
s.pop();                    // Returns Some('!'), s = "Hello, world"
s.remove(5);               // Returns ',', s = "Hello world"

// Remove ranges
s.drain(5..6);             // Remove " ", s = "Helloworld"
s.truncate(5);             // s = "Hello"
s.clear();                 // s = ""
```

## 🔄 **Conversion Patterns**

### **String ↔ &str Conversions**

```rust
let owned = String::from("hello");
let borrowed: &str = &owned;        // Deref coercion
let borrowed: &str = owned.as_str(); // Explicit conversion

// &str → String
let slice = "hello";
let owned = slice.to_string();
let owned = String::from(slice);
let owned = slice.to_owned();
```

### **String ↔ Bytes Conversions**

```rust
let s = String::from("Hello 🦀");

// String → bytes
let bytes: Vec<u8> = s.into_bytes();
let bytes: &[u8] = s.as_bytes();

// bytes → String (with validation)
let s = String::from_utf8(bytes).unwrap();

// bytes → String (unsafe, no validation)
let s = unsafe { String::from_utf8_unchecked(bytes) };

// Lossy conversion (replace invalid UTF-8 with �)
let s = String::from_utf8_lossy(&bytes);
```

### **String ↔ OsString Conversions**

```rust
use std::ffi::OsString;

let s = String::from("hello");
let os_string = OsString::from(s);

// Back to String (may fail on invalid Unicode)
let s = os_string.into_string().unwrap();
```

## 🎭 **String Formatting**

### **format! Macro Family**

```rust
let name = "Alice";
let age = 30;

// Basic formatting
let s = format!("Hello, {}!", name);
let s = format!("{} is {} years old", name, age);

// Positional arguments
let s = format!("{0} is {1} years old. {0} likes Rust.", name, age);

// Named arguments
let s = format!("{name} is {age} years old", name = name, age = age);

// Formatting specifiers
let pi = 3.14159;
let s = format!("π ≈ {:.2}", pi);           // "π ≈ 3.14"
let s = format!("Hex: 0x{:X}", 255);        // "Hex: 0xFF"
let s = format!("Binary: {:08b}", 42);      // "Binary: 00101010"
```

### **Write Trait Implementation**

```rust
use std::fmt::Write;

let mut s = String::new();
write!(&mut s, "Hello, {}!", "world").unwrap();
writeln!(&mut s, " How are you?").unwrap();
// s = "Hello, world! How are you?\n"
```

## 🧠 **Memory Management**

### **Capacity vs Length**

```rust
let mut s = String::with_capacity(10);
println!("Capacity: {}, Length: {}", s.capacity(), s.len()); // 10, 0

s.push_str("hello");
println!("Capacity: {}, Length: {}", s.capacity(), s.len()); // 10, 5

s.push_str(" world!"); // Exceeds capacity
println!("Capacity: {}, Length: {}", s.capacity(), s.len()); // ≥12, 12
```

### **Memory Optimization**

```rust
let mut s = String::with_capacity(1000);
s.push_str("small");

// Shrink to fit actual content
s.shrink_to_fit();
println!("Capacity after shrink: {}", s.capacity()); // ~5

// Reserve additional capacity
s.reserve(100);
println!("Capacity after reserve: {}", s.capacity()); // ≥105

// Reserve exact capacity
s.reserve_exact(50);
```

### **Avoiding Reallocations**

```rust
// Bad - multiple reallocations
let mut s = String::new();
for i in 0..1000 {
    s.push_str(&i.to_string());
}

// Better - pre-allocate capacity
let mut s = String::with_capacity(4000); // Estimate capacity
for i in 0..1000 {
    s.push_str(&i.to_string());
}

// Best - collect from iterator
let s: String = (0..1000).map(|i| i.to_string()).collect();
```

## 🔍 **String Analysis and Querying**

### **Length and Indexing**

```rust
let s = "Hello 🦀 world";

// Byte length (not character count!)
println!("Byte length: {}", s.len());        // 15 (not 13!)

// Character count
println!("Char count: {}", s.chars().count()); // 13

// No direct indexing - use ranges carefully
// let c = s[0];  // ERROR: cannot index into string

// Safe character access
if let Some(first_char) = s.chars().next() {
    println!("First char: {}", first_char);   // 'H'
}

// Byte slicing (dangerous if not on character boundary)
let hello = &s[0..5];  // "Hello" - safe because ASCII
// let invalid = &s[0..7];  // PANIC: not on char boundary
```

### **Pattern Matching**

```rust
let s = "The quick brown fox jumps over the lazy dog";

// Contains
assert!(s.contains("fox"));
assert!(s.contains("🦀") == false);

// Starts/ends with
assert!(s.starts_with("The"));
assert!(s.ends_with("dog"));

// Find position
assert_eq!(s.find("fox"), Some(16));
assert_eq!(s.find("🦀"), None);

// Split operations
let words: Vec<&str> = s.split_whitespace().collect();
let parts: Vec<&str> = s.split("the").collect();
```

## ⚡ **Performance Considerations**

### **String Concatenation**

```rust
// Inefficient - creates new String each time
let mut result = String::new();
result = result + "hello";  // Moves result, creates new String
result = result + " ";      // Moves result, creates new String  
result = result + "world";  // Moves result, creates new String

// Efficient - reuses same String
let mut result = String::new();
result.push_str("hello");   // Modifies in place
result.push_str(" ");       // Modifies in place
result.push_str("world");   // Modifies in place

// Most efficient - format! or collect
let result = format!("{} {}", "hello", "world");
let result: String = ["hello", " ", "world"].concat();
```

### **Clone vs Reference**

```rust
fn process_string_bad(s: String) {  // Takes ownership
    println!("{}", s);
}

fn process_string_good(s: &str) {   // Borrows
    println!("{}", s);
}

let s = String::from("hello");
// process_string_bad(s.clone());  // Expensive clone
process_string_good(&s);            // Cheap borrow
```

### **UTF-8 Validation Costs**

```rust
// Validates UTF-8 every time
let bytes = vec![72, 101, 108, 108, 111];
let s = String::from_utf8(bytes).unwrap();  // O(n) validation

// If you know bytes are valid UTF-8
let bytes = vec![72, 101, 108, 108, 111];
let s = unsafe { String::from_utf8_unchecked(bytes) };  // O(1) no validation
```

## 🌐 **Unicode Handling**

### **Character Iteration**

```rust
let s = "नमस्ते"; // Hindi greeting

// Iterate by bytes (usually not what you want)
for byte in s.bytes() {
    println!("{}", byte);  // 224, 164, 168, 224, 164, 174, ...
}

// Iterate by Unicode scalar values (chars)
for ch in s.chars() {
    println!("{}", ch);    // न, म, स्, त्, े
}

// Iterate by grapheme clusters (what humans see as "characters")
// Requires external crate like `unicode-segmentation`
```

### **String Normalization**

```rust
// Different Unicode representations for same text
let s1 = "é";           // Single character: é (U+00E9)
let s2 = "é";           // Composed: e (U+0065) + ́ (U+0301)

assert_ne!(s1, s2);     // Different byte sequences
assert_ne!(s1.len(), s2.len()); // Different lengths

// Use unicode-normalization crate for proper comparison
// use unicode_normalization::UnicodeNormalization;
// assert_eq!(s1.nfc().collect::<String>(), s2.nfc().collect::<String>());
```

## 🛡️ **Error Handling**

### **UTF-8 Validation Errors**

```rust
// Invalid UTF-8 bytes
let invalid_bytes = vec![0xFF, 0xFE];

match String::from_utf8(invalid_bytes) {
    Ok(s) => println!("Valid string: {}", s),
    Err(e) => {
        println!("Invalid UTF-8: {}", e);
        
        // Recover the original bytes
        let bytes = e.into_bytes();
        
        // Convert with replacement characters
        let s = String::from_utf8_lossy(&bytes);
        println!("Lossy conversion: {}", s); // Contains � characters
    }
}
```

### **Index Out of Bounds**

```rust
let s = String::from("hello");

// Runtime panic
// let ch = s.chars().nth(10).unwrap(); // PANIC

// Safe access
if let Some(ch) = s.chars().nth(2) {
    println!("Character at index 2: {}", ch);  // 'l'
} else {
    println!("Index out of bounds");
}
```

## 🔗 **Common Patterns**

### **String Builder Pattern**

```rust
struct StringBuilder {
    buffer: String,
}

impl StringBuilder {
    fn new() -> Self {
        Self { buffer: String::new() }
    }
    
    fn with_capacity(capacity: usize) -> Self {
        Self { buffer: String::with_capacity(capacity) }
    }
    
    fn append(&mut self, s: &str) -> &mut Self {
        self.buffer.push_str(s);
        self
    }
    
    fn append_line(&mut self, s: &str) -> &mut Self {
        self.buffer.push_str(s);
        self.buffer.push('\n');
        self
    }
    
    fn build(self) -> String {
        self.buffer
    }
}

// Usage
let result = StringBuilder::with_capacity(1000)
    .append("Hello")
    .append(" ")
    .append("world")
    .append_line("!")
    .append("From Rust")
    .build();
```

### **String Interning**

```rust
use std::collections::HashMap;

struct StringInterner {
    strings: HashMap<String, usize>,
    ids: Vec<String>,
}

impl StringInterner {
    fn new() -> Self {
        Self {
            strings: HashMap::new(),
            ids: Vec::new(),
        }
    }
    
    fn intern(&mut self, s: &str) -> usize {
        if let Some(&id) = self.strings.get(s) {
            id
        } else {
            let id = self.ids.len();
            self.ids.push(s.to_string());
            self.strings.insert(s.to_string(), id);
            id
        }
    }
    
    fn get(&self, id: usize) -> Option<&str> {
        self.ids.get(id).map(|s| s.as_str())
    }
}
```

## 📊 **Memory Layout**

### **String Structure**

```rust
// Simplified representation
struct String {
    ptr: *mut u8,    // Pointer to heap data
    len: usize,      // Current length in bytes
    cap: usize,      // Allocated capacity in bytes
}

// Size: 24 bytes on 64-bit systems (3 × 8 bytes)
assert_eq!(std::mem::size_of::<String>(), 24);
```

### **Small String Optimization (Not in std)**

```rust
// Some crates implement SSO to avoid heap allocation for small strings
// Example concept (not actual std implementation):
enum SmallString {
    Heap {
        ptr: *mut u8,
        len: usize,
        cap: usize,
    },
    Inline {
        data: [u8; 23],  // 23 bytes inline storage
        len: u8,         // 1 byte length
    },
}
```

---

*Tags: #string #utf8 #unicode #heap-allocation #text-processing #rust-types #memory-management*

*Links: [[zettel-index]] | [[unicode-utf8-rust]] | [[Vec Type]] | [[Collections MOC]] | [[rust_book/rust-book-ch8]] | [[Text Processing Patterns]] | [[Memory Management Patterns]]*
