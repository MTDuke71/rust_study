# Day 17 · Lifetime Annotations (explicit syntax)

> **Learning Context**: Day 17 introduces lifetime annotations, Rust's solution to memory safety in generic code that enables Mission5's safe reference handling without garbage collection.

**Cross-Track Integration:**
- **Mission Focus**: Lifetimes ensure Mission5 HashMap references remain valid across function boundaries - see [[Mission5 Overview]]
- **Daily Study**: Builds on Days 15-16 (Traits + Generics) to complete type system foundation
- **Rust Book**: Core of Chapter 10.3 Validating References with Lifetimes

**Related Zettelkasten Notes:**
- [[Collections MOC]] - Reference patterns across data structures
- [[Mission5 Overview]] - REQ-3 safe reference handling with lifetimes
- [[zettel-index]] - Main learning hub

## Core Concepts

### What Are Lifetimes?
- **Memory Safety**: Ensure references don't outlive the data they point to
- **Compile-Time Checks**: No runtime overhead - all validation at compile time
- **Relationship Contracts**: Express how long references should live relative to each other
- **Borrow Checker Friend**: Help the borrow checker understand your intent

### The Dangling Reference Problem
```rust
// ❌ This won't compile - dangling reference
fn broken_function() -> &str {
    let s = String::from("hello");
    &s  // ❌ s is dropped here, but we're returning a reference to it
}

// ✅ This works - return owned data
fn working_function() -> String {
    let s = String::from("hello");
    s   // ✅ Transfer ownership
}

// ✅ This works - reference lives long enough
fn working_reference(s: &str) -> &str {
    s   // ✅ Input reference is returned, caller ensures lifetime
}
```

## Basic Lifetime Syntax

### Lifetime Parameters
```rust
// Lifetime parameter 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Multiple lifetime parameters
fn complex<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("y is: {}", y); // y has lifetime 'b
    x  // Return has lifetime 'a (same as x)
}

// Usage - Rust infers actual lifetimes
let string1 = String::from("long string");
let result;
{
    let string2 = String::from("short");
    result = longest(&string1, &string2); // ❌ Won't compile
    // string2 dropped here, but result tries to live longer
}
// println!("{}", result); // Would be dangling reference
```

### Lifetime in Function Signatures
```rust
// When do you need lifetime annotations?
// 1. Function returns reference derived from parameters
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 2. Multiple references with different lifetimes
fn announce_and_return<'a, 'b>(announcement: &'a str, x: &'b str) -> &'b str {
    println!("Announcement: {}", announcement);
    x  // Return x, which has lifetime 'b
}
```

## Lifetime Elision Rules

### When Lifetimes are Inferred
```rust
// These functions don't need explicit lifetime annotations:

// Rule 1: Each reference parameter gets its own lifetime
fn simple(s: &str) -> usize {
    s.len()
}
// Compiler infers: fn simple<'a>(s: &'a str) -> usize

// Rule 2: If there's one input lifetime, it's assigned to output
fn echo(s: &str) -> &str {
    s
}
// Compiler infers: fn echo<'a>(s: &'a str) -> &'a str

// Rule 3: If one parameter is &self or &mut self, its lifetime is assigned to output
impl MyStruct {
    fn get_data(&self) -> &str {
        &self.data
    }
    // Compiler infers: fn get_data<'a>(&'a self) -> &'a str
}
```

### When You Must Specify Lifetimes
```rust
// Multiple input references, ambiguous output lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Different lifetimes for different purposes
fn complex_case<'a, 'b>(x: &'a str, y: &'b str, z: &'a str) -> &'a str {
    println!("y: {}", y); // 'b lifetime, just for printing
    if x.len() > z.len() { x } else { z } // Both 'a lifetime
}
```

## Lifetimes in Structs

### Structs Holding References
```rust
// Struct containing references needs lifetime parameters
struct ImportantExcerpt<'a> {
    part: &'a str,  // This reference must live as long as the struct
}

impl<'a> ImportantExcerpt<'a> {
    // Method doesn't need lifetime annotation (elision rule 3)
    fn level(&self) -> i32 {
        3
    }
    
    // Method returning reference from struct
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part  // Returns reference with same lifetime as self
    }
}

// Usage
let novel = String::from("Call me Ishmael. Some years ago...");
let first_sentence = novel.split('.').next().expect("Could not find a '.'");
let excerpt = ImportantExcerpt {
    part: first_sentence,
};
// excerpt.part must not outlive novel
```

## Mission5 Integration: Safe Reference Patterns

### HashMap Iterator with Lifetimes
```rust
use std::hash::Hash;

// Mission5: HashMap with lifetime-aware methods
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
}

impl<K, V> HashMap<K, V>
where 
    K: Hash + Eq,
{
    // Method returning reference - lifetime tied to self
    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        let index = self.hash_key(key);
        
        for (existing_key, existing_value) in &self.buckets[index] {
            if existing_key == key {
                return Some(existing_value); // Lifetime 'a from &self
            }
        }
        None
    }
    
    // Iterator with explicit lifetime
    pub fn iter<'a>(&'a self) -> HashMapIterator<'a, K, V> {
        HashMapIterator {
            map: self,
            bucket_index: 0,
            item_index: 0,
        }
    }
    
    // Method with multiple lifetime parameters
    pub fn get_or_insert<'a>(&'a mut self, key: K, default: V) -> &'a mut V
    where 
        K: Clone,
    {
        // Complex lifetime relationships between self and return value
        if self.get(&key).is_none() {
            self.insert(key.clone(), default);
        }
        
        // Find and return mutable reference
        let index = self.hash_key(&key);
        for (existing_key, existing_value) in &mut self.buckets[index] {
            if *existing_key == key {
                return existing_value;
            }
        }
        unreachable!()
    }
    
    fn hash_key(&self, key: &K) -> usize {
        // Simplified hash function
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.buckets.len()
    }
}

// Iterator struct with lifetime parameter
pub struct HashMapIterator<'a, K, V> {
    map: &'a HashMap<K, V>,
    bucket_index: usize,
    item_index: usize,
}

impl<'a, K, V> Iterator for HashMapIterator<'a, K, V> {
    type Item = (&'a K, &'a V);  // References with lifetime 'a
    
    fn next(&mut self) -> Option<Self::Item> {
        while self.bucket_index < self.map.buckets.len() {
            let bucket = &self.map.buckets[self.bucket_index];
            
            if self.item_index < bucket.len() {
                let (key, value) = &bucket[self.item_index];
                self.item_index += 1;
                return Some((key, value)); // Lifetime tied to original map
            }
            
            self.bucket_index += 1;
            self.item_index = 0;
        }
        None
    }
}
```

### Safe Reference Return Patterns
```rust
// REQ-3: Safe reference handling in Mission5
impl<K, V> HashMap<K, V>
where 
    K: Hash + Eq,
{
    // Pattern 1: Reference with same lifetime as container
    pub fn keys<'a>(&'a self) -> impl Iterator<Item = &'a K> {
        self.iter().map(|(k, _)| k)
    }
    
    // Pattern 2: Reference with caller-controlled lifetime
    pub fn find_key<'a, 'b>(&'a self, predicate: impl Fn(&K) -> bool) -> Option<&'a K>
    where 
        'b: 'a,  // 'b outlives 'a
    {
        for (key, _) in self.iter() {
            if predicate(key) {
                return Some(key);
            }
        }
        None
    }
    
    // Pattern 3: Multiple references with constraints
    pub fn compare_values<'a>(
        &'a self,
        key1: &K,
        key2: &K,
    ) -> Option<(&'a V, &'a V)> {
        match (self.get(key1), self.get(key2)) {
            (Some(v1), Some(v2)) => Some((v1, v2)),
            _ => None,
        }
    }
}
```

## Advanced Lifetime Patterns

### Lifetime Bounds
```rust
// Lifetime bounds in generic functions
fn store_reference<'a, T>(storage: &mut Option<&'a T>, value: &'a T)
where 
    T: 'a,  // T must live at least as long as 'a
{
    *storage = Some(value);
}

// Static lifetime bound
fn store_static_str(storage: &mut Option<&'static str>, value: &'static str) {
    *storage = Some(value);
}
```

### Higher-Ranked Trait Bounds (HRTB)
```rust
// Function that works with any lifetime
fn apply_to_string(f: impl for<'a> Fn(&'a str) -> &'a str) -> String {
    let s = String::from("hello");
    f(&s).to_string()
}

// Usage
let result = apply_to_string(|s| {
    if s.len() > 3 { &s[..3] } else { s }
});
```

### Lifetime Subtyping
```rust
// Longer lifetimes can be coerced to shorter ones
fn demonstrate_subtyping() {
    let string1 = String::from("long lived");
    
    {
        let string2 = String::from("short");
        
        // 'long can be used where 'short is expected
        let result: &str = choose_first(&string1, &string2);
        println!("{}", result);
    } // string2 dropped, but result doesn't reference it
}

fn choose_first<'a>(x: &'a str, _y: &str) -> &'a str {
    x  // Always return first parameter
}
```

## The `'static` Lifetime

### Static Lifetime Meaning
```rust
// String literals have 'static lifetime
let s: &'static str = "hello world";

// Static variables
static GLOBAL_STR: &str = "This lives for entire program";

// Functions requiring 'static
fn takes_static(s: &'static str) {
    println!("{}", s);
}

takes_static("string literal");  // ✅ Works
takes_static(GLOBAL_STR);       // ✅ Works

let owned = String::from("owned");
// takes_static(&owned);        // ❌ Won't compile - owned doesn't live long enough
```

### Static vs Generic Lifetimes
```rust
// Different meanings of 'static
fn needs_static_ref(s: &'static str) {
    // s must be a reference that lives for entire program
}

fn needs_static_bound<T: 'static>(t: T) {
    // T must not contain any non-static references
    // But T itself doesn't need to live forever
}

// Example: Box<String> is 'static even if created at runtime
fn example() {
    let boxed = Box::new(String::from("hello"));
    needs_static_bound(boxed);  // ✅ Works - no references inside
}
```

## Common Lifetime Patterns

### Builder Pattern with Lifetimes
```rust
struct QueryBuilder<'a> {
    table: &'a str,
    conditions: Vec<&'a str>,
}

impl<'a> QueryBuilder<'a> {
    fn new(table: &'a str) -> Self {
        QueryBuilder {
            table,
            conditions: Vec::new(),
        }
    }
    
    fn where_clause(mut self, condition: &'a str) -> Self {
        self.conditions.push(condition);
        self
    }
    
    fn build(&self) -> String {
        let mut query = format!("SELECT * FROM {}", self.table);
        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }
        query
    }
}

// Usage
let table_name = "users";
let condition1 = "age > 18";
let condition2 = "active = true";

let query = QueryBuilder::new(table_name)
    .where_clause(condition1)
    .where_clause(condition2)
    .build();
```

### Cache with Lifetime Management
```rust
use std::collections::HashMap;

struct Cache<'a, K, V>
where 
    K: Hash + Eq,
{
    data: HashMap<K, &'a V>,
}

impl<'a, K, V> Cache<'a, K, V>
where 
    K: Hash + Eq,
{
    fn new() -> Self {
        Cache {
            data: HashMap::new(),
        }
    }
    
    fn insert(&mut self, key: K, value: &'a V) {
        self.data.insert(key, value);
    }
    
    fn get(&self, key: &K) -> Option<&&'a V> {
        self.data.get(key)
    }
}
```

## Best Practices

### Lifetime Design Guidelines
```rust
// ✅ Good: Use elision when possible
impl MyStruct {
    fn get_name(&self) -> &str {  // No need for explicit lifetimes
        &self.name
    }
}

// ✅ Good: Explicit lifetimes when needed
fn merge_strings<'a>(s1: &'a str, s2: &'a str) -> String {
    format!("{} {}", s1, s2)  // Return owned data when combining
}

// ✅ Good: Separate lifetimes for different purposes
fn process<'a, 'b>(data: &'a str, config: &'b Config) -> &'a str
where 
    'b: 'a,  // Config must live at least as long as data
{
    // Process data using config
    data
}

// ❌ Avoid: Unnecessary lifetime constraints
fn bad_example<'a>(s: &'a str) -> String {  // Lifetime not needed
    s.to_uppercase()  // Returns owned String
}
```

## Learning Progression Summary

From Day 17, you should understand:
1. **Lifetime Purpose**: Preventing dangling references at compile time
2. **Explicit Syntax**: `'a`, `'b` lifetime parameters in functions and structs
3. **Elision Rules**: When Rust infers lifetimes automatically
4. **Mission5 Integration**: Safe reference handling in HashMap methods and iterators
5. **Common Patterns**: Iterator lifetimes, builder patterns, reference constraints
6. **Static Lifetime**: `'static` for program-duration references

**3-Track Learning Integration:**
- **Mission5**: Lifetime annotations ensure safe HashMap reference patterns (REQ-3)
- **Week 3 Foundation**: Days 15-17 (Traits → Generics → Lifetimes) complete type system
- **Type Safety**: Understanding how Rust prevents memory errors without garbage collection

**Cross-References:**
- [[Collections MOC]] - Reference safety patterns across Vec, HashMap, BTreeMap
- [[Mission5 Overview]] - REQ-3 safe reference handling with lifetime annotations
- [[HashMap Internals]] - Memory safety in hash table implementation

**Next**: Day 18 will cover **Advanced Traits** - associated types, defaults, and trait bounds!

---
**Zettelkasten Integration:**
*Links: [[Collections MOC]] | [[Mission5 Overview]] | [[HashMap Internals]] | [[zettel-index]]*

*Tags: #lifetimes #memory-safety #references #borrow-checker #mission5 #daily-study #week3 #type-system*

## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as a .rs file
use std::collections::HashMap;

// 1. Basic lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 2. Struct with lifetime parameter
#[derive(Debug)]
struct BookExcerpt<'a> {
    title: &'a str,
    content: &'a str,
}

impl<'a> BookExcerpt<'a> {
    fn new(title: &'a str, content: &'a str) -> Self {
        BookExcerpt { title, content }
    }
    
    // Lifetime elision - compiler infers same lifetime as self
    fn get_title(&self) -> &str {
        self.title
    }
    
    // Explicit lifetime annotation when needed
    fn compare_with<'b>(&'a self, other: &'b BookExcerpt) -> &'a str
    where 
        'b: 'a,  // other must live at least as long as self
    {
        if self.content.len() > other.content.len() {
            self.title
        } else {
            "Other book is longer"
        }
    }
}

// 3. Mission5-inspired HashMap with lifetimes
#[derive(Debug)]
struct SimpleMap<K, V> {
    entries: Vec<(K, V)>,
}

impl<K, V> SimpleMap<K, V>
where 
    K: PartialEq,
{
    fn new() -> Self {
        SimpleMap {
            entries: Vec::new(),
        }
    }
    
    fn insert(&mut self, key: K, value: V) {
        // Update existing key or add new entry
        for (existing_key, existing_value) in &mut self.entries {
            if *existing_key == key {
                *existing_value = value;
                return;
            }
        }
        self.entries.push((key, value));
    }
    
    // Method with lifetime - return reference tied to self
    fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        for (existing_key, existing_value) in &self.entries {
            if existing_key == key {
                return Some(existing_value);
            }
        }
        None
    }
    
    // Iterator with lifetime parameter
    fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a V)> {
        self.entries.iter().map(|(k, v)| (k, v))
    }
    
    // Method with multiple references
    fn compare_values<'a>(
        &'a self,
        key1: &K,
        key2: &K,
    ) -> Option<(&'a V, &'a V)> {
        match (self.get(key1), self.get(key2)) {
            (Some(v1), Some(v2)) => Some((v1, v2)),
            _ => None,
        }
    }
}

// 4. Builder pattern with lifetimes
struct MessageBuilder<'a> {
    recipient: &'a str,
    subject: &'a str,
    body: String,
}

impl<'a> MessageBuilder<'a> {
    fn new(recipient: &'a str) -> Self {
        MessageBuilder {
            recipient,
            subject: "No Subject",
            body: String::new(),
        }
    }
    
    fn subject(mut self, subject: &'a str) -> Self {
        self.subject = subject;
        self
    }
    
    fn body(mut self, body: String) -> Self {
        self.body = body;
        self
    }
    
    fn build(&self) -> String {
        format!("To: {}\nSubject: {}\n\n{}", 
                self.recipient, self.subject, self.body)
    }
}

// 5. Function demonstrating lifetime constraints
fn process_strings<'a, 'b>(
    data: &'a str,
    config: &'b str,
    temp: &str,  // Separate lifetime, inferred
) -> (&'a str, usize)
where 
    'b: 'a,  // config must live at least as long as data
{
    println!("Processing with config: {}", config);
    println!("Temporary data: {}", temp);
    (data, data.len())
}

// 6. Cache example with lifetime management
struct StringCache<'a> {
    entries: HashMap<String, &'a str>,
}

impl<'a> StringCache<'a> {
    fn new() -> Self {
        StringCache {
            entries: HashMap::new(),
        }
    }
    
    fn store(&mut self, key: String, value: &'a str) {
        self.entries.insert(key, value);
    }
    
    fn retrieve(&self, key: &str) -> Option<&'a str> {
        self.entries.get(key).copied()
    }
    
    fn len(&self) -> usize {
        self.entries.len()
    }
}

fn main() {
    println!("=== Lifetime Annotations Demo from Day 17 ===\n");
    
    // 1. Basic lifetime functions
    println!("1. Basic Lifetime Functions:");
    let string1 = String::from("Hello, world!");
    let string2 = String::from("Rust");
    
    let longer = longest(&string1, &string2);
    println!("   Longer string: '{}'", longer);
    
    let sentence = "Hello Rust programming world";
    let word = first_word(sentence);
    println!("   First word: '{}'", word);
    
    // 2. Structs with lifetimes
    println!("\n2. Structs with Lifetimes:");
    let book_title = "The Rust Programming Language";
    let book_content = "Rust is a systems programming language focused on safety...";
    
    let excerpt = BookExcerpt::new(book_title, book_content);
    println!("   Book excerpt: {:?}", excerpt);
    println!("   Title: {}", excerpt.get_title());
    
    // 3. Mission5-style map with lifetimes
    println!("\n3. HashMap-style Container with Lifetimes:");
    let mut map: SimpleMap<String, i32> = SimpleMap::new();
    
    map.insert("score".to_string(), 100);
    map.insert("lives".to_string(), 3);
    map.insert("level".to_string(), 5);
    
    if let Some(score) = map.get(&"score".to_string()) {
        println!("   Current score: {}", score);
    }
    
    println!("   All entries:");
    for (key, value) in map.iter() {
        println!("     {}: {}", key, value);
    }
    
    // Compare values using lifetime constraints
    if let Some((score, lives)) = map.compare_values(&"score".to_string(), &"lives".to_string()) {
        println!("   Score: {}, Lives: {}", score, lives);
    }
    
    // 4. Builder pattern with lifetimes
    println!("\n4. Builder Pattern with Lifetimes:");
    let recipient = "alice@example.com";
    let subject = "Meeting Tomorrow";
    let body_content = "Don't forget about our meeting tomorrow at 2 PM.".to_string();
    
    let message = MessageBuilder::new(recipient)
        .subject(subject)
        .body(body_content)
        .build();
    
    println!("   Message:\n{}", message);
    
    // 5. Multiple lifetime parameters
    println!("\n5. Multiple Lifetime Parameters:");
    let data_str = "Important data";
    let config_str = "Configuration settings";
    let temp_str = "Temporary information";
    
    let (processed, length) = process_strings(data_str, config_str, temp_str);
    println!("   Processed: '{}', Length: {}", processed, length);
    
    // 6. Cache with lifetime management
    println!("\n6. Cache with Lifetime Management:");
    let cached_str1 = "This string lives long enough";
    let cached_str2 = "Another long-lived string";
    
    let mut cache: StringCache = StringCache::new();
    cache.store("key1".to_string(), cached_str1);
    cache.store("key2".to_string(), cached_str2);
    
    println!("   Cache size: {}", cache.len());
    if let Some(value) = cache.retrieve("key1") {
        println!("   Retrieved: '{}'", value);
    }
    
    // 7. Demonstrating lifetime constraints
    println!("\n7. Lifetime Relationships:");
    let long_lived = String::from("This lives for the whole scope");
    
    {
        let medium_lived = String::from("This lives for inner scope");
        
        // Both references have different lifetimes, but function handles it
        let result = longest(&long_lived, &medium_lived);
        println!("   Within scope - longer: '{}'", result);
        
        // result can't outlive this scope because medium_lived doesn't
    }
    
    // long_lived still accessible here
    println!("   Outside scope - long_lived still valid: '{}'", long_lived);
    
    println!("\n=== Lifetimes ensure memory safety at compile time! ===");
}
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day17_demo.rs` and run `rustc day17_demo.rs && ./day17_demo`
3. **In this workspace**: `.\scripts\run_md.bat daily_study\rust_learning_week3_notes\Day17.md`
4. **As Cargo example**: `cargo run --example day17_lifetimes_demo` (if you add it to Mission5_tut)
