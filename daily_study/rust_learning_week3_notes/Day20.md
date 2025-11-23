# Day 20 · Advanced Lifetimes (elision rules, `'static`, complex relationships)

> **Learning Context**: Day 20 explores advanced lifetime patterns essential for Mission5's zero-copy operations, memory safety guarantees, and complex data structure relationships.

**Cross-Track Integration:**
- **Mission Focus**: Advanced lifetimes enable Mission5's efficient borrowing patterns and zero-copy HashMap operations - see [[mission-5]]
- **Daily Study**: Completes Week 3's type system mastery with lifetime relationship modeling
- **Rust Book**: Chapter 10.3 Validating References with Lifetimes and Chapter 19 Advanced Lifetimes

**Related Zettelkasten Notes:**
- [[Collections MOC]] - Lifetime patterns in collection APIs
- [[mission-5]] - REQ-7 zero-copy operations with lifetime management
- [[HashMap Internals]] - Lifetime relationships in hash table implementations
- [[zettel-index]] - Main learning hub

## Core Concepts

### Lifetime Elision Rules Revisited
```rust
// Elision Rule 1: Each parameter gets its own lifetime
fn first_word(s: &str) -> &str {
    // Actually: fn first_word<'a>(s: &'a str) -> &'a str
    s.split_whitespace().next().unwrap_or("")
}

// Elision Rule 2: If exactly one input lifetime, output gets that lifetime
fn get_prefix(text: &str, len: usize) -> &str {
    // Actually: fn get_prefix<'a>(text: &'a str, len: usize) -> &'a str
    &text[..len.min(text.len())]
}

// Elision Rule 3: If multiple inputs with &self, output gets self's lifetime
impl<T> std::collections::HashMap<String, T> {
    fn get_value(&self, key: &str) -> Option<&T> {
        // Actually: fn get_value<'a, 'b>(&'a self, key: &'b str) -> Option<&'a T>
        self.get(key)
    }
}

// When elision fails - explicit lifetimes required
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### Complex Lifetime Relationships
```rust
// Multiple lifetime parameters
struct Parser<'input, 'config> {
    input: &'input str,
    config: &'config ParserConfig,
}

struct ParserConfig {
    delimiter: char,
    skip_empty: bool,
}

impl<'input, 'config> Parser<'input, 'config> {
    fn new(input: &'input str, config: &'config ParserConfig) -> Self {
        Parser { input, config }
    }
    
    // Return type tied to input lifetime, not config
    fn next_token(&mut self) -> Option<&'input str> {
        if self.input.is_empty() {
            return None;
        }
        
        let delimiter = self.config.delimiter;
        let end = self.input.find(delimiter).unwrap_or(self.input.len());
        let token = &self.input[..end];
        
        // Update input for next call
        self.input = if end < self.input.len() {
            &self.input[end + 1..]
        } else {
            ""
        };
        
        if self.config.skip_empty && token.is_empty() {
            self.next_token()  // Recursive call for empty tokens
        } else {
            Some(token)
        }
    }
}

// Usage demonstrating lifetime independence
fn parse_example() {
    let input = "hello,world,rust";
    let config = ParserConfig {
        delimiter: ',',
        skip_empty: true,
    };
    
    let mut parser = Parser::new(&input, &config);
    
    while let Some(token) = parser.next_token() {
        println!("Token: {}", token);  // token lives as long as input
    }
    // config can be dropped here, but tokens are still valid
}
```

### The `'static` Lifetime
```rust
// 'static means "lives for entire program duration"

// 1. String literals are 'static
let greeting: &'static str = "Hello, World!";

// 2. Static variables
static GLOBAL_CONFIG: &str = "production";

// 3. 'static bound - doesn't mean the value must live forever
fn store_reference<T: 'static>(value: T) -> T {
    // T must not contain any non-'static references
    value  // T can be moved and doesn't need to live forever
}

// Examples of 'static bounds
fn static_examples() {
    // ✅ Works - owns data, no references
    let owned = store_reference(String::from("owned"));
    
    // ✅ Works - 'static reference
    let static_ref = store_reference("static string");
    
    // ❌ Won't compile - contains non-'static reference
    let local = String::from("local");
    // let bad = store_reference(&local);  // Error: local doesn't live long enough
}

// 'static in trait objects
trait Processor: 'static {
    fn process(&self) -> String;
}

// Can store trait objects that don't contain non-'static references
struct ProcessorStore {
    processors: Vec<Box<dyn Processor>>,
}
```

### Lifetime Bounds and Where Clauses
```rust
// Lifetime bounds in where clauses
fn complex_lifetime_bounds<'a, 'b, T, U>(
    first: &'a T,
    second: &'b U,
) -> impl Iterator<Item = String> + 'a
where
    T: std::fmt::Display + 'a,
    U: std::fmt::Debug + 'b,
    'b: 'a,  // 'b outlives 'a
{
    std::iter::once(format!("{}", first))
}

// Higher-Ranked Trait Bounds (HRTB)
fn for_all_lifetimes<F>(f: F) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,  // Function works for ANY lifetime
{
    let text = "hello world";
    f(text).to_string()
}

// Usage with closures
fn hrtb_example() {
    let result = for_all_lifetimes(|s| {
        s.split_whitespace().next().unwrap_or("")
    });
    
    println!("Result: {}", result);
}
```

## Mission5 Integration: Zero-Copy HashMap Operations

### Lifetime-Aware HashMap API
```rust
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::borrow::{Borrow, Cow};

// Mission5: Zero-copy operations with lifetime management
pub struct LifetimeHashMap<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> LifetimeHashMap<K, V>
where 
    K: Hash + Eq,
{
    pub fn new() -> Self {
        LifetimeHashMap {
            data: HashMap::new(),
        }
    }
    
    // Standard insertion - takes ownership
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.data.insert(key, value)
    }
    
    // Zero-copy lookup - lifetime tied to self
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.data.get(key)
    }
    
    // Mutable zero-copy access
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.data.get_mut(key)
    }
    
    // Complex operation: get multiple values with single lifetime
    pub fn get_many<'a, Q>(&'a self, keys: &[&Q]) -> Vec<Option<&'a V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        keys.iter().map(|&key| self.data.get(key)).collect()
    }
    
    // Advanced: Entry API with lifetimes
    pub fn get_or_insert_with<'a, F>(&'a mut self, key: K, f: F) -> &'a V
    where
        F: FnOnce() -> V,
    {
        self.data.entry(key).or_insert_with(f)
    }
}

// String optimization with Cow (Clone on Write)
impl LifetimeHashMap<String, String> {
    // Accept both owned and borrowed strings
    pub fn insert_cow<'a>(&mut self, key: Cow<'a, str>, value: Cow<'a, str>) {
        self.data.insert(key.into_owned(), value.into_owned());
    }
    
    // Zero-copy lookup with string keys
    pub fn get_str(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}
```

### Reference Iterator Patterns
```rust
// Iterator that borrows from HashMap without taking ownership
impl<K, V> LifetimeHashMap<K, V> {
    // Iterator over references - lifetime tied to self
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.data.iter()
    }
    
    // Values iterator with explicit lifetime
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.data.values()
    }
    
    // Keys iterator
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.data.keys()
    }
    
    // Filtered iteration with lifetime preservation
    pub fn iter_filtered<'a, F>(&'a self, predicate: F) -> impl Iterator<Item = (&'a K, &'a V)>
    where
        F: Fn(&K, &V) -> bool,
    {
        self.data.iter().filter(move |(k, v)| predicate(k, v))
    }
}

// Usage demonstrating zero-copy operations
fn zero_copy_operations() {
    let mut map = LifetimeHashMap::new();
    map.insert("name".to_string(), "Alice".to_string());
    map.insert("role".to_string(), "Developer".to_string());
    map.insert("level".to_string(), "Senior".to_string());
    
    // Zero-copy lookups - no string allocation
    if let Some(name) = map.get("name") {
        println!("Found: {}", name);  // Borrows from map
    }
    
    // Multiple lookups with single lifetime
    let keys = ["name", "role", "unknown"];
    let values = map.get_many(&keys);
    
    for (key, value) in keys.iter().zip(values.iter()) {
        match value {
            Some(v) => println!("{}: {}", key, v),
            None => println!("{}: not found", key),
        }
    }
}
```

### Complex Lifetime Relationships in Data Structures
```rust
// Mission5: HashMap with borrowed keys and complex lifetimes
pub struct BorrowingHashMap<'data> {
    // Keys are borrowed from external data
    map: HashMap<&'data str, String>,
    // Source data that keys borrow from
    _source: &'data str,
}

impl<'data> BorrowingHashMap<'data> {
    pub fn new(source: &'data str) -> Self {
        BorrowingHashMap {
            map: HashMap::new(),
            _source: source,
        }
    }
    
    // Insert using borrowed key from source
    pub fn insert_from_source(&mut self, key_start: usize, key_end: usize, value: String) {
        if key_end <= self._source.len() {
            let key = &self._source[key_start..key_end];
            self.map.insert(key, value);
        }
    }
    
    // Lookup with borrowed key
    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
    
    // Iterator over borrowed keys and owned values
    pub fn iter(&self) -> impl Iterator<Item = (&'data str, &String)> {
        self.map.iter().map(|(&k, v)| (k, v))
    }
}

// Usage showing complex lifetime relationships
fn borrowing_hashmap_example() {
    let data = "name:Alice,role:Developer,level:Senior";
    let mut map = BorrowingHashMap::new(data);
    
    // Parse and insert using borrowed keys
    for (i, field) in data.split(',').enumerate() {
        if let Some(colon_pos) = field.find(':') {
            let key_start = field.as_ptr() as usize - data.as_ptr() as usize;
            let key_end = key_start + colon_pos;
            let value = field[colon_pos + 1..].to_string();
            
            // This is complex - in practice, you'd use a different approach
            // This example shows the lifetime concepts
        }
    }
    
    // Keys borrow from original data, values are owned
    for (key, value) in map.iter() {
        println!("{} -> {}", key, value);
    }
}
```

### Lifetime Subtyping and Variance
```rust
// Covariance: 'long can be used where 'short is expected
fn covariance_example() {
    let long_lived = String::from("long lived");
    
    {
        let short_lived = String::from("short lived");
        
        // Function expecting 'short lifetime
        fn use_short_ref<'short>(r: &'short str) {
            println!("{}", r);
        }
        
        // ✅ Can pass 'long reference where 'short expected
        use_short_ref(&long_lived);  // 'long: 'short (covariance)
        use_short_ref(&short_lived);
    }
    // short_lived dropped here, but that's fine
}

// Invariance in mutable references
fn invariance_example() {
    let mut value = String::from("hello");
    
    // Mutable references are invariant - exact lifetime match required
    fn extend_string<'a>(s: &'a mut String) -> &'a str {
        s.push_str(" world");
        s.as_str()
    }
    
    let result = extend_string(&mut value);
    println!("{}", result);
}
```

## Advanced Patterns

### Self-Referential Structures (Careful!)
```rust
// This pattern is generally discouraged, but shows lifetime complexity
use std::pin::Pin;

struct SelfReferential<'a> {
    data: String,
    reference: Option<&'a str>,  // This is problematic!
}

// Better approach: Use indices instead of references
struct SafeSelfReferential {
    data: String,
    reference_start: usize,
    reference_end: usize,
}

impl SafeSelfReferential {
    fn new(data: String) -> Self {
        SafeSelfReferential {
            data,
            reference_start: 0,
            reference_end: 0,
        }
    }
    
    fn set_reference(&mut self, start: usize, end: usize) {
        if end <= self.data.len() {
            self.reference_start = start;
            self.reference_end = end;
        }
    }
    
    fn get_reference(&self) -> &str {
        &self.data[self.reference_start..self.reference_end]
    }
}

// Usage of safe pattern
fn self_referential_example() {
    let mut sr = SafeSelfReferential::new("hello world".to_string());
    sr.set_reference(6, 11);  // Point to "world"
    
    println!("Reference: {}", sr.get_reference());
}
```

### Lifetime Bounds in Generics
```rust
// Generic with lifetime bounds
struct Container<'a, T>
where
    T: 'a,  // T must live at least as long as 'a
{
    data: &'a T,
    metadata: String,
}

impl<'a, T> Container<'a, T>
where
    T: 'a + std::fmt::Display,
{
    fn new(data: &'a T, metadata: String) -> Self {
        Container { data, metadata }
    }
    
    fn display_info(&self) -> String {
        format!("{}: {}", self.metadata, self.data)
    }
}

// Multiple lifetime parameters with bounds
struct ComplexContainer<'data, 'meta, T, U>
where
    T: 'data,
    U: 'meta,
    'data: 'meta,  // 'data outlives 'meta
{
    primary: &'data T,
    secondary: &'meta U,
}

// Function with complex lifetime relationships
fn process_containers<'long, 'short, T>(
    long_data: &'long T,
    short_data: &'short T,
) -> Container<'short, T>
where
    T: std::fmt::Display,
    'long: 'short,  // 'long outlives 'short
{
    // Can use long_data where short_data expected due to subtyping
    Container::new(long_data, "processed".to_string())
}
```

### Anonymous Lifetimes and Impl Trait
```rust
// Anonymous lifetimes with impl trait
impl LifetimeHashMap<String, String> {
    // Anonymous lifetime in return type
    fn find_longest_key(&self) -> Option<&str> {
        self.data
            .keys()
            .max_by_key(|k| k.len())
            .map(|s| s.as_str())
    }
    
    // Impl trait with lifetime bounds
    fn keys_longer_than(&self, min_len: usize) -> impl Iterator<Item = &str> + '_ {
        self.data
            .keys()
            .filter(move |k| k.len() > min_len)
            .map(|s| s.as_str())
    }
    
    // Complex impl trait return with multiple lifetimes
    fn compare_with<'other, V2>(
        &self, 
        other: &'other LifetimeHashMap<String, V2>
    ) -> impl Iterator<Item = (&str, bool)> + '_
    where
        V2: PartialEq,
    {
        self.data
            .keys()
            .map(move |key| {
                let exists_in_other = other.data.contains_key(key);
                (key.as_str(), exists_in_other)
            })
    }
}
```

## Performance and Memory Safety

### Zero-Copy String Processing
```rust
// Mission5: String processing without allocation
pub struct StringProcessor<'input> {
    input: &'input str,
    position: usize,
}

impl<'input> StringProcessor<'input> {
    pub fn new(input: &'input str) -> Self {
        StringProcessor { input, position: 0 }
    }
    
    // Return string slices - no allocation
    pub fn next_word(&mut self) -> Option<&'input str> {
        if self.position >= self.input.len() {
            return None;
        }
        
        // Skip whitespace
        while self.position < self.input.len() 
              && self.input.chars().nth(self.position).unwrap().is_whitespace() {
            self.position += 1;
        }
        
        if self.position >= self.input.len() {
            return None;
        }
        
        let start = self.position;
        
        // Find word end
        while self.position < self.input.len() 
              && !self.input.chars().nth(self.position).unwrap().is_whitespace() {
            self.position += 1;
        }
        
        Some(&self.input[start..self.position])
    }
    
    // Process all words without allocation
    pub fn word_count(&mut self) -> usize {
        let mut count = 0;
        while self.next_word().is_some() {
            count += 1;
        }
        count
    }
}

// Usage demonstrating zero-copy processing
fn zero_copy_string_processing() {
    let text = "hello world rust programming";
    let mut processor = StringProcessor::new(text);
    
    // Process without any string allocations
    println!("Words:");
    while let Some(word) = processor.next_word() {
        println!("  {}", word);  // word borrows from original text
    }
    
    // Reset and count
    let mut counter = StringProcessor::new(text);
    let count = counter.word_count();
    println!("Total words: {}", count);
}
```

### Lifetime Elision in Practice
```rust
// Examples showing when elision works and when it doesn't

// ✅ Elision works - single input parameter
impl LifetimeHashMap<String, String> {
    fn get_value(&self, key: &str) -> Option<&String> {
        // Elided: fn get_value<'a>(&'a self, key: &str) -> Option<&'a String>
        self.data.get(key)
    }
    
    // ✅ Elision works - self parameter present
    fn get_key_value(&self, key: &str) -> Option<(&String, &String)> {
        // Elided: fn get_key_value<'a, 'b>(&'a self, key: &'b str) -> Option<(&'a String, &'a String)>
        self.data.get_key_value(key)
    }
}

// ❌ Elision fails - multiple input parameters, no self
fn find_common_prefix<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // Explicit lifetimes required - which input does output borrow from?
    let mut i = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            break;
        }
        i += c1.len_utf8();
    }
    &s1[..i]
}

// ❌ Elision fails - unclear which lifetime to use
fn choose_string<'a, 'b>(first: &'a str, second: &'b str, use_first: bool) -> &'_ str {
    // Would need explicit lifetime relationships
    if use_first { first } else { second }
}
```

## Real-World Applications

### Parser with Lifetime Management
```rust
// Real-world example: CSV parser with zero-copy field extraction
struct CsvParser<'input> {
    input: &'input str,
    position: usize,
    line_number: usize,
}

struct CsvRecord<'input> {
    fields: Vec<&'input str>,
    line_number: usize,
}

impl<'input> CsvParser<'input> {
    fn new(input: &'input str) -> Self {
        CsvParser {
            input,
            position: 0,
            line_number: 1,
        }
    }
    
    fn next_record(&mut self) -> Option<CsvRecord<'input>> {
        if self.position >= self.input.len() {
            return None;
        }
        
        let line_start = self.position;
        
        // Find end of line
        while self.position < self.input.len() {
            if self.input.chars().nth(self.position) == Some('\n') {
                break;
            }
            self.position += 1;
        }
        
        let line = &self.input[line_start..self.position];
        
        // Skip newline for next iteration
        if self.position < self.input.len() {
            self.position += 1;
        }
        
        let fields: Vec<&str> = line.split(',').collect();
        let record = CsvRecord {
            fields,
            line_number: self.line_number,
        };
        
        self.line_number += 1;
        Some(record)
    }
}

impl<'input> CsvRecord<'input> {
    fn get_field(&self, index: usize) -> Option<&'input str> {
        self.fields.get(index).copied()
    }
    
    fn field_count(&self) -> usize {
        self.fields.len()
    }
}

// Usage showing zero-copy CSV parsing
fn csv_parsing_example() {
    let csv_data = "name,age,city\nAlice,30,Seattle\nBob,25,Portland\nCarol,35,Denver";
    let mut parser = CsvParser::new(csv_data);
    
    // Skip header
    let _header = parser.next_record();
    
    // Process records without string allocation
    while let Some(record) = parser.next_record() {
        if let (Some(name), Some(age), Some(city)) = 
           (record.get_field(0), record.get_field(1), record.get_field(2)) {
            println!("Line {}: {} ({}) from {}", 
                    record.line_number, name, age, city);
        }
    }
}
```

## Best Practices and Common Pitfalls

### Lifetime Best Practices
```rust
// ✅ Good: Minimal lifetime annotations
impl<K, V> LifetimeHashMap<K, V> {
    // Let elision work when possible
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    // Explicit only when necessary
    fn get_or_default<'a>(&'a self, key: &K, default: &'a V) -> &'a V
    where
        K: Hash + Eq,
    {
        self.data.get(key).unwrap_or(default)
    }
}

// ❌ Avoid: Unnecessary lifetime parameters
// fn bad_example<'a, 'b>(x: &'a str, y: i32) -> i32 {  // 'a unused!
//     y * 2
// }

// ✅ Better: Only necessary lifetimes
fn good_example(x: &str, y: i32) -> i32 {  // 'a elided
    x.len() as i32 + y * 2
}
```

### Common Lifetime Errors and Solutions
```rust
// Error: Returning reference to local variable
// fn bad_function() -> &str {
//     let local = String::from("local");
//     &local  // Error: local dropped at end of function
// }

// ✅ Solution 1: Return owned value
fn solution1() -> String {
    String::from("owned")
}

// ✅ Solution 2: Accept parameter with lifetime
fn solution2(input: &str) -> &str {
    input  // Lifetime tied to parameter
}

// ✅ Solution 3: Use 'static for constants
fn solution3() -> &'static str {
    "static string literal"
}
```

## Learning Progression Summary

From Day 20, you should understand:
1. **Lifetime Elision Rules**: When Rust infers lifetimes automatically
2. **Complex Relationships**: Multiple lifetime parameters and bounds
3. **'static Lifetime**: Program-duration vs 'static bound differences  
4. **Mission5 Integration**: Zero-copy operations and lifetime-aware APIs
5. **Advanced Patterns**: HRTB, variance, and self-referential considerations
6. **Performance Benefits**: Memory safety without allocation overhead

**3-Track Learning Integration:**
- **Mission5**: Zero-copy HashMap operations with lifetime management enable high-performance data processing
- **Week 3 Mastery**: Days 15-20 provide complete advanced type system understanding
- **Real-World Applications**: CSV parsing, string processing, complex data structures

**Cross-References:**
- [[Collections MOC]] - Lifetime patterns across different collection types
- [[mission-5]] - REQ-7 zero-copy operations with advanced lifetime management
- [[HashMap Internals]] - Lifetime relationships in hash table API design

**Next**: Day 21 will integrate **Generics + Traits Practice** - combining all Week 3 concepts for flexible, performant APIs!

---
**Zettelkasten Integration:**
*Links: [[Collections MOC]] | [[mission-5]] | [[HashMap Internals]] | [[zettel-index]]*

*Tags: #lifetimes #advanced-lifetimes #static-lifetime #elision-rules #zero-copy #mission5 #daily-study #week3 #memory-safety*

## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as a .rs file

fn main() {
    println!("=== Advanced Lifetimes Demo from Day 20 ===\n");
    
    lifetime_elision_demo();
    complex_lifetimes_demo();
    static_lifetime_demo();
    zero_copy_operations_demo();
    csv_parser_demo();
    string_processor_demo();
}

// 1. Lifetime elision rules in action
fn first_word(s: &str) -> &str {
    // Elided: fn first_word<'a>(s: &'a str) -> &'a str
    s.split_whitespace().next().unwrap_or("")
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Explicit lifetimes required - multiple inputs
    if x.len() > y.len() { x } else { y }
}

fn lifetime_elision_demo() {
    println!("1. Lifetime Elision Rules:");
    
    let text = "hello world rust";
    let word = first_word(text);
    println!("   First word: '{}'", word);
    
    let str1 = "short";
    let str2 = "much longer string";
    let longer = longest(str1, str2);
    println!("   Longest: '{}'", longer);
    println!();
}

// 2. Complex lifetime relationships
struct Parser<'input, 'config> {
    input: &'input str,
    config: &'config Config,
    position: usize,
}

struct Config {
    delimiter: char,
    skip_empty: bool,
}

impl<'input, 'config> Parser<'input, 'config> {
    fn new(input: &'input str, config: &'config Config) -> Self {
        Parser {
            input,
            config,
            position: 0,
        }
    }
    
    fn next_token(&mut self) -> Option<&'input str> {
        if self.position >= self.input.len() {
            return None;
        }
        
        let remaining = &self.input[self.position..];
        let delimiter_pos = remaining.find(self.config.delimiter);
        
        let (token, advance) = if let Some(pos) = delimiter_pos {
            (&remaining[..pos], pos + 1)
        } else {
            (remaining, remaining.len())
        };
        
        self.position += advance;
        
        if self.config.skip_empty && token.is_empty() {
            self.next_token()  // Skip empty tokens
        } else {
            Some(token)
        }
    }
}

fn complex_lifetimes_demo() {
    println!("2. Complex Lifetime Relationships:");
    
    let data = "apple,banana,,cherry,";
    let config = Config {
        delimiter: ',',
        skip_empty: true,
    };
    
    let mut parser = Parser::new(&data, &config);
    
    println!("   Parsing: '{}'", data);
    while let Some(token) = parser.next_token() {
        println!("   Token: '{}'", token);
    }
    println!();
}

// 3. 'static lifetime examples
static GLOBAL_MESSAGE: &str = "This is global!";

fn process_static<T: 'static>(value: T) -> T {
    // T must not contain non-'static references
    value
}

fn static_lifetime_demo() {
    println!("3. 'static Lifetime:");
    
    // String literals are 'static
    let literal = "string literal";
    println!("   Literal: {}", literal);
    
    // Global static
    println!("   Global: {}", GLOBAL_MESSAGE);
    
    // 'static bound - doesn't mean lives forever
    let owned = process_static(String::from("owned data"));
    println!("   Processed owned: {}", owned);
    
    let static_ref = process_static("static reference");
    println!("   Processed static: {}", static_ref);
    println!();
}

// 4. Zero-copy operations
use std::collections::HashMap;

struct ZeroCopyMap<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> ZeroCopyMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn new() -> Self {
        ZeroCopyMap {
            data: HashMap::new(),
        }
    }
    
    fn insert(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }
    
    // Zero-copy lookup - returns reference
    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }
    
    // Multiple lookups with same lifetime
    fn get_many<'a>(&'a self, keys: &[&K]) -> Vec<Option<&'a V>> {
        keys.iter().map(|&key| self.data.get(key)).collect()
    }
    
    // Iterator over references
    fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.data.iter()
    }
}

fn zero_copy_operations_demo() {
    println!("4. Zero-Copy HashMap Operations:");
    
    let mut map = ZeroCopyMap::new();
    map.insert("name".to_string(), "Alice".to_string());
    map.insert("role".to_string(), "Developer".to_string());
    map.insert("team".to_string(), "Backend".to_string());
    
    // Zero-copy lookup
    if let Some(name) = map.get(&"name".to_string()) {
        println!("   Name: {}", name);
    }
    
    // Multiple zero-copy lookups
    let keys = [&"name".to_string(), &"role".to_string(), &"level".to_string()];
    let values = map.get_many(&keys);
    
    for (key, value) in keys.iter().zip(values.iter()) {
        match value {
            Some(v) => println!("   {}: {}", key, v),
            None => println!("   {}: not found", key),
        }
    }
    
    println!("   All entries:");
    for (key, value) in map.iter() {
        println!("     {} -> {}", key, value);
    }
    println!();
}

// 5. CSV parser with zero-copy field extraction
struct CsvParser<'input> {
    input: &'input str,
    position: usize,
    line_number: usize,
}

struct CsvRecord<'input> {
    fields: Vec<&'input str>,
    line_number: usize,
}

impl<'input> CsvParser<'input> {
    fn new(input: &'input str) -> Self {
        CsvParser {
            input,
            position: 0,
            line_number: 1,
        }
    }
    
    fn next_record(&mut self) -> Option<CsvRecord<'input>> {
        if self.position >= self.input.len() {
            return None;
        }
        
        let line_start = self.position;
        
        // Find end of current line
        while self.position < self.input.len() 
              && self.input.chars().nth(self.position) != Some('\n') {
            self.position += 1;
        }
        
        let line = &self.input[line_start..self.position];
        
        // Skip newline
        if self.position < self.input.len() {
            self.position += 1;
        }
        
        let fields = line.split(',').collect();
        let record = CsvRecord {
            fields,
            line_number: self.line_number,
        };
        
        self.line_number += 1;
        Some(record)
    }
}

impl<'input> CsvRecord<'input> {
    fn get_field(&self, index: usize) -> Option<&'input str> {
        self.fields.get(index).copied()
    }
    
    fn field_count(&self) -> usize {
        self.fields.len()
    }
}

fn csv_parser_demo() {
    println!("5. Zero-Copy CSV Parser:");
    
    let csv_data = "name,age,city\nAlice,30,Seattle\nBob,25,Portland\nCarol,35,Denver";
    println!("   Parsing CSV data:");
    println!("   {}", csv_data.replace('\n', "\\n"));
    
    let mut parser = CsvParser::new(csv_data);
    
    // Parse header
    if let Some(header) = parser.next_record() {
        println!("   Header fields: {:?}", header.fields);
    }
    
    // Parse data records
    while let Some(record) = parser.next_record() {
        if let (Some(name), Some(age), Some(city)) = 
           (record.get_field(0), record.get_field(1), record.get_field(2)) {
            println!("   Line {}: {} ({}) from {}", 
                    record.line_number, name, age, city);
        }
    }
    println!();
}

// 6. String processor with zero allocations
struct StringProcessor<'input> {
    input: &'input str,
    position: usize,
}

impl<'input> StringProcessor<'input> {
    fn new(input: &'input str) -> Self {
        StringProcessor { input, position: 0 }
    }
    
    fn next_word(&mut self) -> Option<&'input str> {
        // Skip whitespace
        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap_or(' ');
            if !ch.is_whitespace() {
                break;
            }
            self.position += ch.len_utf8();
        }
        
        if self.position >= self.input.len() {
            return None;
        }
        
        let start = self.position;
        
        // Find word end
        while self.position < self.input.len() {
            let ch = self.input.chars().nth(self.position).unwrap_or(' ');
            if ch.is_whitespace() {
                break;
            }
            self.position += ch.len_utf8();
        }
        
        Some(&self.input[start..self.position])
    }
    
    fn reset(&mut self) {
        self.position = 0;
    }
    
    fn remaining(&self) -> &'input str {
        &self.input[self.position..]
    }
}

fn string_processor_demo() {
    println!("6. Zero-Copy String Processing:");
    
    let text = "  hello    world   rust   programming  ";
    println!("   Processing: '{}'", text);
    
    let mut processor = StringProcessor::new(text);
    
    println!("   Words found:");
    while let Some(word) = processor.next_word() {
        println!("     '{}'", word);
    }
    
    // Reset and count words
    processor.reset();
    let mut count = 0;
    while processor.next_word().is_some() {
        count += 1;
    }
    
    println!("   Total words: {}", count);
    println!();
}
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day20_demo.rs` and run `rustc day20_demo.rs && ./day20_demo`
3. **In this workspace**: `.\scripts\run_md.bat daily_study\rust_learning_week3_notes\Day20.md`
4. **As Cargo example**: `cargo run --example day20_advanced_lifetimes_demo` (if you add it to Mission5_tut)
