# Predicates and Higher-Order Functions Deep Dive

**Tags:** #rust #predicates #closures #higher-order-functions #functional-programming #iterators

**Related:** [[daily-study/Day17]] | [[Collections MOC]] | [[rust-concepts-MOC]]

## Overview

Predicates are functions that test conditions and return boolean values. In Rust, they're implemented through closures and higher-order functions, enabling powerful functional programming patterns throughout the standard library and custom code.

## What Are Predicates?

### Core Definition
A **predicate** is a function that:
- **Takes some input** and returns a **boolean** (`true` or `false`)
- **Tests a condition** or **asks a question** about the input
- **Determines** whether something meets certain criteria

### Rust Implementation
```rust
// Predicate as a closure
let is_even = |x: i32| x % 2 == 0;

// Predicate as a function
fn is_positive(x: i32) -> bool {
    x > 0
}

// Predicate as a higher-order function parameter
fn find_first<T, F>(items: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    for item in items {
        if predicate(item) {
            return Some(item);
        }
    }
    None
}
```

## Higher-Order Functions

### Definition
**Higher-order functions** are functions that:
- **Take other functions as parameters**
- **Return functions as results**
- **Operate on functions** rather than just data

### Rust Syntax
```rust
// Function that takes a function as parameter
fn apply_operation<F>(value: i32, operation: F) -> i32
where
    F: Fn(i32) -> i32,
{
    operation(value)
}

// Usage
let doubled = apply_operation(5, |x| x * 2);
let squared = apply_operation(5, |x| x * x);
```

## Common Predicate Patterns

### 1. Filtering Collections
```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// Filter even numbers
let evens: Vec<i32> = numbers
    .iter()
    .filter(|&x| x % 2 == 0)  // Predicate: is even
    .copied()
    .collect();

// Filter by range
let in_range: Vec<i32> = numbers
    .iter()
    .filter(|&x| x >= 3 && x <= 7)  // Predicate: in range
    .copied()
    .collect();
```

### 2. Finding Elements
```rust
let words = vec!["apple", "banana", "cherry", "date"];

// Find first word starting with 'b'
let result = words
    .iter()
    .find(|word| word.starts_with('b'));  // Predicate: starts with 'b'

// Find longest word
let longest = words
    .iter()
    .max_by_key(|word| word.len());  // Predicate: length comparison
```

### 3. Validation Patterns
```rust
// Email validation predicate
fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

// Password strength predicate
fn is_strong_password(password: &str) -> bool {
    password.len() >= 8 
        && password.chars().any(|c| c.is_uppercase())
        && password.chars().any(|c| c.is_lowercase())
        && password.chars().any(|c| c.is_numeric())
}

// Usage
let emails = vec!["user@example.com", "invalid-email", "admin@site.org"];
let valid_emails: Vec<&str> = emails
    .iter()
    .filter(|email| is_valid_email(email))
    .copied()
    .collect();
```

## Advanced Predicate Patterns

### 1. Predicate Composition
```rust
// Combine multiple predicates
fn and<F, G, T>(f: F, g: G) -> impl Fn(&T) -> bool
where
    F: Fn(&T) -> bool,
    G: Fn(&T) -> bool,
{
    move |x| f(x) && g(x)
}

fn or<F, G, T>(f: F, g: G) -> impl Fn(&T) -> bool
where
    F: Fn(&T) -> bool,
    G: Fn(&T) -> bool,
{
    move |x| f(x) || g(x)
}

// Usage
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let is_even = |x: &i32| x % 2 == 0;
let is_greater_than_5 = |x: &i32| *x > 5;

let result: Vec<i32> = numbers
    .iter()
    .filter(and(is_even, is_greater_than_5))  // Even AND > 5
    .copied()
    .collect();
```

### 2. Stateful Predicates
```rust
// Predicate that remembers state
fn create_counter_predicate(threshold: i32) -> impl Fn(&i32) -> bool {
    let mut count = 0;
    move |x| {
        count += 1;
        *x > threshold && count <= 3  // First 3 items above threshold
    }
}

let numbers = vec![1, 5, 3, 8, 2, 9, 4, 7];
let result: Vec<i32> = numbers
    .iter()
    .filter(create_counter_predicate(4))
    .copied()
    .collect();
```

### 3. Generic Predicate Functions
```rust
// Generic predicate function for HashMap
impl<K, V> HashMap<K, V> {
    pub fn find_key<'a, 'b>(
        &'a self,
        predicate: impl Fn(&'b K) -> bool
    ) -> Option<&'a K>
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
}
```

## Iterator Methods Using Predicates

### Standard Library Examples
```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// filter - Keep items that match predicate
let evens: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).copied().collect();

// find - Find first item that matches predicate
let first_odd = numbers.iter().find(|&x| x % 2 == 1);

// any - Check if any item matches predicate
let has_negative = numbers.iter().any(|&x| x < 0);

// all - Check if all items match predicate
let all_positive = numbers.iter().all(|&x| x > 0);

// position - Find position of first match
let pos = numbers.iter().position(|&x| x == 5);

// rposition - Find position of last match
let rpos = numbers.iter().rposition(|&x| x == 5);
```

## Performance Considerations

### Zero-Cost Abstractions
```rust
// This predicate usage has zero runtime overhead
let result: Vec<i32> = numbers
    .iter()
    .filter(|&x| x % 2 == 0)  // Compiler optimizes this away
    .map(|x| x * 2)
    .collect();

// Equivalent to manual loop (after optimization)
let mut result = Vec::new();
for &x in &numbers {
    if x % 2 == 0 {
        result.push(x * 2);
    }
}
```

### Closure Capture Performance
```rust
// Efficient: capture by reference
let threshold = 5;
let predicate = |x: &i32| *x > threshold;  // Captures threshold by reference

// Less efficient: capture by value (if large)
let large_data = vec![1; 1000];
let predicate = move |x: &i32| large_data.contains(x);  // Moves large_data
```

## Real-World Applications

### 1. Data Validation
```rust
struct User {
    name: String,
    email: String,
    age: u32,
}

impl User {
    fn is_valid(&self) -> bool {
        !self.name.is_empty()
            && self.email.contains('@')
            && self.age >= 13
    }
}

let users = vec![
    User { name: "Alice".to_string(), email: "alice@example.com".to_string(), age: 25 },
    User { name: "".to_string(), email: "invalid".to_string(), age: 10 },
    User { name: "Bob".to_string(), email: "bob@test.org".to_string(), age: 30 },
];

let valid_users: Vec<&User> = users
    .iter()
    .filter(|user| user.is_valid())
    .collect();
```

### 2. Search and Filter
```rust
struct Product {
    name: String,
    price: f64,
    category: String,
}

impl Product {
    fn matches_search(&self, query: &str) -> bool {
        self.name.to_lowercase().contains(&query.to_lowercase())
    }
    
    fn in_price_range(&self, min: f64, max: f64) -> bool {
        self.price >= min && self.price <= max
    }
    
    fn in_category(&self, category: &str) -> bool {
        self.category == category
    }
}

let products = vec![
    Product { name: "Laptop".to_string(), price: 999.99, category: "Electronics".to_string() },
    Product { name: "Book".to_string(), price: 19.99, category: "Education".to_string() },
    Product { name: "Phone".to_string(), price: 699.99, category: "Electronics".to_string() },
];

// Complex filtering with multiple predicates
let electronics_under_800: Vec<&Product> = products
    .iter()
    .filter(|p| p.in_category("Electronics"))
    .filter(|p| p.in_price_range(0.0, 800.0))
    .collect();
```

### 3. Configuration Validation
```rust
struct Config {
    host: String,
    port: u16,
    timeout: u64,
}

impl Config {
    fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        let predicates = [
            ("host", |c: &Config| !c.host.is_empty(), "Host cannot be empty"),
            ("port", |c: &Config| c.port > 0, "Port must be positive"),
            ("timeout", |c: &Config| c.timeout > 0, "Timeout must be positive"),
        ];
        
        for (field, predicate, error_msg) in predicates {
            if !predicate(self) {
                errors.push(format!("{}: {}", field, error_msg));
            }
        }
        
        errors
    }
}
```

## Testing Predicates

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even_predicate() {
        let is_even = |x: i32| x % 2 == 0;
        
        assert!(is_even(2));
        assert!(is_even(4));
        assert!(!is_even(3));
        assert!(!is_even(5));
    }

    #[test]
    fn test_find_key_predicate() {
        let mut map = HashMap::new();
        map.insert("short", 1);
        map.insert("medium_length", 2);
        map.insert("very_long_key", 3);
        
        // Test length predicate
        let result = map.find_key(|key| key.len() > 10);
        assert_eq!(result, Some(&"very_long_key"));
        
        // Test pattern predicate
        let result = map.find_key(|key| key.starts_with("med"));
        assert_eq!(result, Some(&"medium_length"));
    }

    #[test]
    fn test_predicate_composition() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let is_even = |x: &i32| x % 2 == 0;
        let is_greater_than_5 = |x: &i32| *x > 5;
        
        let result: Vec<i32> = numbers
            .iter()
            .filter(and(is_even, is_greater_than_5))
            .copied()
            .collect();
            
        assert_eq!(result, vec![6, 8, 10]);
    }
}
```

## Integration with Mission Work

### Mission 5: HashMap Patterns
```rust
impl<K, V> HashMap<K, V> {
    // Find key by predicate
    pub fn find_key<F>(&self, predicate: F) -> Option<&K>
    where
        F: Fn(&K) -> bool,
    {
        for (key, _) in self.iter() {
            if predicate(key) {
                return Some(key);
            }
        }
        None
    }
    
    // Filter entries by predicate
    pub fn filter_entries<F>(&self, predicate: F) -> Vec<(&K, &V)>
    where
        F: Fn(&K, &V) -> bool,
    {
        self.iter()
            .filter(|(k, v)| predicate(k, v))
            .collect()
    }
    
    // Remove entries matching predicate
    pub fn remove_if<F>(&mut self, predicate: F) -> Vec<(K, V)>
    where
        F: Fn(&K, &V) -> bool,
    {
        let mut removed = Vec::new();
        self.retain(|k, v| {
            if predicate(k, v) {
                removed.push((k.clone(), v.clone()));
                false
            } else {
                true
            }
        });
        removed
    }
}
```

## Best Practices

### 1. Clear Naming
```rust
// ✅ Good: Descriptive predicate names
let is_valid_email = |email: &str| email.contains('@');
let has_minimum_length = |text: &str| text.len() >= 8;
let is_positive_number = |x: &i32| *x > 0;

// ❌ Poor: Unclear names
let check1 = |email: &str| email.contains('@');
let test = |text: &str| text.len() >= 8;
let func = |x: &i32| *x > 0;
```

### 2. Reusable Predicates
```rust
// ✅ Good: Reusable predicate functions
fn create_length_predicate(min_length: usize) -> impl Fn(&str) -> bool {
    move |text| text.len() >= min_length
}

fn create_range_predicate(min: i32, max: i32) -> impl Fn(&i32) -> bool {
    move |x| *x >= min && *x <= max
}

// Usage
let has_min_5_chars = create_length_predicate(5);
let in_range_1_10 = create_range_predicate(1, 10);
```

### 3. Performance Considerations
```rust
// ✅ Good: Efficient predicate
let is_expensive = |item: &Product| item.price > 100.0;  // Simple comparison

// ❌ Poor: Expensive predicate
let is_expensive = |item: &Product| {
    // Expensive database lookup for each item
    database.lookup_price(item.id) > 100.0
};
```

## Common Pitfalls

### 1. Moving vs Borrowing
```rust
// ❌ Problem: Moving captured values
let data = vec![1, 2, 3, 4, 5];
let predicate = move |x: &i32| data.contains(x);  // data is moved

// ✅ Solution: Borrow instead
let data = vec![1, 2, 3, 4, 5];
let predicate = |x: &i32| data.contains(x);  // data is borrowed
```

### 2. Lifetime Issues
```rust
// ❌ Problem: Lifetime mismatch
fn create_predicate() -> impl Fn(&str) -> bool {
    let target = "hello".to_string();
    move |s| s == &target  // target doesn't live long enough
}

// ✅ Solution: Use static or owned data
fn create_predicate() -> impl Fn(&str) -> bool {
    move |s| s == "hello"  // Static string
}
```

### 3. Over-complex Predicates
```rust
// ❌ Problem: Too complex
let complex_predicate = |user: &User| {
    user.age >= 18 
        && user.email.contains('@') 
        && !user.name.is_empty()
        && user.name.len() > 2
        && user.name.chars().all(|c| c.is_alphabetic())
        && user.email.split('@').count() == 2
        && user.email.ends_with(".com")
};

// ✅ Solution: Break into smaller predicates
let is_adult = |user: &User| user.age >= 18;
let has_valid_email = |user: &User| user.email.contains('@') && user.email.ends_with(".com");
let has_valid_name = |user: &User| !user.name.is_empty() && user.name.len() > 2;

let simple_predicate = |user: &User| is_adult(user) && has_valid_email(user) && has_valid_name(user);
```

## Summary

Predicates and higher-order functions are fundamental to Rust's functional programming capabilities:

1. **Predicates** - Functions that test conditions and return booleans
2. **Higher-order functions** - Functions that take other functions as parameters
3. **Iterator methods** - Built-in predicate usage throughout the standard library
4. **Performance** - Zero-cost abstractions with compile-time optimization
5. **Real-world applications** - Validation, filtering, searching, and data processing
6. **Mission integration** - Essential for advanced collection operations

**Key Insight:** Predicates enable flexible, reusable code by separating the "what to test" from the "how to test it" logic.

---

**References:**
- [[daily-study/Day17]] for lifetime patterns with predicates
- [[Collections MOC]] for collection-specific predicate patterns
- [The Rust Book - Chapter 13: Functional Language Features](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- [Iterator Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
