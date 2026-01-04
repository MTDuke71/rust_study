//! Chapter 3: Unsurprising Interfaces
//! 
//! Topics: Naming Practices, Common Traits, Ergonomic Implementations, Wrapper Types

use std::collections::HashMap;
use std::ops::Deref;

fn main() {
    println!("=== Unsurprising Interfaces ===\n");
    
    naming_practices();
    common_traits();
    ergonomic_implementations();
    wrapper_types();
    static_methods_pattern();
}

/// Naming Practices: Use familiar names from std
fn naming_practices() {
    println!("--- Naming Practices ---");
    
    let data = CustomCollection::new();
    
    // Familiar naming: iter() is expected to return an iterator
    for item in data.iter() {
        println!("  Item: {}", item);
    }
    
    // into_inner() is a common pattern for extracting wrapped values
    let wrapper = Wrapper(42);
    let inner = wrapper.into_inner();
    println!("  Extracted inner value: {}\n", inner);
}

/// Common Traits: Implement Debug, Clone, Default, etc.
fn common_traits() {
    println!("--- Common Traits for Types ---");
    
    let data = DataType::default();
    println!("  Default: {:?}", data);
    
    let cloned = data.clone();
    println!("  Cloned: {:?}", cloned);
    
    // PartialEq for comparisons
    println!("  Equal? {}", data == cloned);
    
    // Hash for use in HashMaps
    let mut map = HashMap::new();
    map.insert(data, "value");
    println!("  Can use as HashMap key\n");
}

/// Ergonomic Implementations: Implement traits for &T, &mut T
fn ergonomic_implementations() {
    println!("--- Ergonomic Trait Implementations ---");
    
    let collection = CustomCollection::new();
    
    // Can iterate over &CustomCollection thanks to IntoIterator for &T
    for item in &collection {
        print!("  {}", item);
    }
    println!("\n");
}

/// Wrapper Types: Deref and AsRef for transparent wrappers
fn wrapper_types() {
    println!("--- Wrapper Types ---");
    
    let wrapper = Wrapper(42);
    
    // Deref allows calling i32 methods directly
    let str_rep = wrapper.to_string();
    println!("  Deref to_string: {}", str_rep);
    
    // AsRef for generic conversions
    print_number(&wrapper);
    println!();
}

fn print_number<T: AsRef<i32>>(value: &T) {
    println!("  Number via AsRef: {}", value.as_ref());
}

/// Static Methods Pattern: Avoid ambiguity with Deref
fn static_methods_pattern() {
    println!("--- Static Methods Pattern for Wrappers ---");
    
    let tracked = TrackedString::new("hello");
    
    // Static methods for wrapper-specific operations
    println!("  Access count: {}", TrackedString::accesses(&tracked));
    
    // Instance methods via Deref - unambiguous!
    println!("  Length: {}", tracked.len());           // String::len
    println!("  Uppercase: {}", tracked.to_uppercase()); // String::to_uppercase
    
    // More deref usage increments counter
    let _ = tracked.chars();
    println!("  Access count after chars: {}", TrackedString::accesses(&tracked));
    
    // Unwrap with static method
    let inner = TrackedString::into_inner(tracked);
    println!("  Extracted: {}", inner);
    println!();
}

// === Helper Types ===

struct CustomCollection {
    items: Vec<i32>,
}

impl CustomCollection {
    fn new() -> Self {
        Self { items: vec![1, 2, 3] }
    }
    
    // Familiar naming pattern
    fn iter(&self) -> impl Iterator<Item = &i32> {
        self.items.iter()
    }
}

// Ergonomic: Implement IntoIterator for &CustomCollection
impl<'a> IntoIterator for &'a CustomCollection {
    type Item = &'a i32;
    type IntoIter = std::slice::Iter<'a, i32>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct DataType {
    value: i32,
}

struct Wrapper(i32);

impl Wrapper {
    // Familiar naming: into_inner extracts the wrapped value
    fn into_inner(self) -> i32 {
        self.0
    }
}

// Deref for transparent wrapper behavior
impl Deref for Wrapper {
    type Target = i32;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<i32> for Wrapper {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

// Wrapper with static methods pattern (RECOMMENDED)
use std::cell::Cell;

struct TrackedString {
    inner: String,
    access_count: Cell<usize>,
}

impl TrackedString {
    // ✅ Static method for construction
    fn new(s: &str) -> Self {
        Self {
            inner: s.to_string(),
            access_count: Cell::new(0),
        }
    }
    
    // ✅ Static method for wrapper-specific data
    // Takes &TrackedString explicitly - no ambiguity
    fn accesses(wrapper: &TrackedString) -> usize {
        wrapper.access_count.get()
    }
    
    // ✅ Static method to unwrap
    // Takes TrackedString by value - clear intent
    fn into_inner(wrapper: TrackedString) -> String {
        wrapper.inner
    }
}

impl Deref for TrackedString {
    type Target = String;
    
    fn deref(&self) -> &String {
        // Track each deref access
        self.access_count.set(self.access_count.get() + 1);
        &self.inner
    }
}

// Compare with BAD pattern (commented out):
// 
// impl TrackedString {
//     // ❌ BAD: Instance method creates ambiguity
//     fn accesses(&self) -> usize {
//         self.access_count.get()
//     }
//     // Now what if String had an accesses() method?
//     // Which one would wrapper.accesses() call?
// }
