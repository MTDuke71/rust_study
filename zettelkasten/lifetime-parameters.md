# Lifetime Parameters

**Tags:** #rust #lifetimes #memory-safety #concept #rust-book #chapter10  
**Created:** 2025-10-22  
**Related:** [[Ownership and Borrowing]], [[daily-study/Day17]], [[zettelkasten/rust_book/rust-book-ch10]], [[Generic Programming]], [[mission-3]], [[mission-4]], [[rust-concepts-MOC]]

## Overview

Lifetime parameters are Rust's way of ensuring memory safety by tracking how long references are valid. They're crucial for preventing dangling pointers and use-after-free errors at compile time.

## Key Characteristics

- **Compile-time guarantee** - Lifetimes are checked at compile time, not runtime
- **Reference validity** - Ensure references don't outlive the data they point to
- **Generic parameters** - Can be generic over lifetimes, just like types
- **Automatic inference** - Often inferred by the compiler (lifetime elision)

## Basic Usage

```rust
// Explicit lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with lifetime parameter
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

## Advanced Patterns

```rust
// Multiple lifetime parameters
fn announce_and_return_part<'a, 'b>(
    announcement: &'a str,
    part: &'b str,
) -> &'b str {
    println!("Attention please: {}", announcement);
    part
}

// Static lifetime
let s: &'static str = "I have a static lifetime.";
```

### Independent Lifetimes

**When to use multiple lifetime parameters:**

```rust
// Struct with independent lifetimes
struct StrSplit<'s, 'p> {
    document: &'s str,    // Short-lived reference
    delimiter: &'p str,   // Could be 'static
}

// Allows mixing lifetime sources:
fn example() {
    let delimiter = ",";  // 'static lifetime
    {
        let doc = String::from("hello,world");
        let splitter = StrSplit { 
            document: &doc,   // Short lifetime
            delimiter         // Long lifetime ('static)
        };
        // Works! Independent lifetimes provide flexibility
    }
}
```

**Single vs Multiple Lifetimes:**

| **Single Lifetime `<'a>`** | **Multiple Lifetimes `<'s, 'p>`** |
|----------------------------|-----------------------------------|
| Simpler to write | More flexible |
| Both references tied together | References can have different sources |
| Compiler forces same constraint | Each reference tracked independently |
| Use when references logically connected | Use when references truly independent |

## Compiler Enforcement

**Critical guarantee:** Lifetime errors are **impossible to ship** - they're caught at compile time.

### What the Compiler Prevents

```rust
// ❌ Returning reference to local variable
fn broken() -> &str {
    let s = String::from("hello");
    &s  // ERROR: `s` does not live long enough
}

// ❌ Using reference after owner dropped
let r;
{
    let x = 5;
    r = &x;
}  // x dropped
println!("{}", r);  // ERROR: `x` does not live long enough

// ❌ Lifetime mismatch in returns
fn longer<'a>(x: &'a str, y: &str) -> &'a str {
    if y.len() > x.len() { 
        y  // ERROR: lifetime mismatch
    } else { x }
}
```

**If your code compiles, these bugs are impossible:**
- Dangling references
- Use-after-free
- Double-free
- Returning references to dropped values

### Learning Strategy

1. **Start simple** - let compiler infer lifetimes (lifetime elision)
2. **Read error messages** - they guide you to the solution
3. **Add annotations only when needed** - compiler will tell you
4. **Trust the compiler** - if it compiles, it's safe

## Common Use Cases

### 1. Function Parameters

```rust
// When function returns a reference derived from input
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

### 2. Struct Definitions

```rust
// When struct holds references
struct ParsedData<'a> {
    header: &'a str,
    body: &'a [u8],
}
```

## Performance Considerations

- **Memory:** Zero runtime cost - lifetimes are compile-time only
- **Speed:** No performance overhead, purely compile-time checking
- **Trade-offs:** Increased compilation complexity for better safety guarantees

## Best Practices

1. **Let the compiler infer** - Use explicit lifetimes only when necessary
2. **Use meaningful names** - `'a`, `'b` for simple cases, descriptive names for complex ones
3. **Keep lifetimes simple** - Avoid overly complex lifetime relationships

## Common Pitfalls

### Unnecessary Lifetime Annotations

```rust
// Problem: Unnecessary explicit lifetime
fn first_word<'a>(s: &'a str) -> &'a str { /* ... */ }

// Better: Let compiler infer (lifetime elision)
fn first_word(s: &str) -> &str { /* ... */ }
```

**Solution:** Trust lifetime elision rules for simple cases

## Integration with Other Concepts

- **[[Ownership and Borrowing]]** - Lifetimes ensure borrowing rules are followed
- **[[Generic Programming]]** - Lifetimes are a form of generic parameter
- **[[mission-3]]** - Binary search with lifetime-aware APIs
- **[[mission-4]]** - Smart pointers and lifetime management

## Use Cases in Rust Study Projects

### Mission3 - Binary Search

- **Usage:** Iterator APIs with lifetime parameters
- **Example:** Search functions returning references into input data

### Mission4 - Linked Lists  

- **Usage:** Node references and iterator implementations
- **Example:** Safe traversal with lifetime guarantees

## Testing Patterns

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lifetime_compilation() {
        let string1 = String::from("abcd");
        let string2 = "xyz";
        
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
}
```

## Related Concepts

- [[rust-book-ch9-12-review]] - Comprehensive review of generics, traits, and lifetimes (Chapter 10)
- [[Ownership and Borrowing]] - Foundation for understanding lifetimes
- [[daily-study/Day17]] - Daily study progression  
- [[zettelkasten/rust_book/rust-book-ch10]] - Generics, traits, and lifetimes together
- [[Generic Programming]] - Lifetimes as generic parameters
- [[mission-3]] - Practical lifetime usage
- [[Multiple Lifetimes Deep Dive]] - Advanced lifetime patterns
- [[rfr-ch01-summary]] - Low-level memory model and lifetime foundations
- [[variance]] - How lifetime subtyping interacts with generic types (covariance, invariance, contravariance)

## Quick Reference

```rust
// Basic lifetime annotation
fn func<'a>(param: &'a Type) -> &'a Type { /* */ }

// Multiple lifetimes  
fn func<'a, 'b>(x: &'a str, y: &'b str) -> &'a str { /* */ }

// Struct with lifetime
struct Wrapper<'a> { data: &'a str }

// Static lifetime
let s: &'static str = "lives for entire program";
```

---

*Lifetime parameters ensure memory safety by tracking reference validity at compile time, enabling Rust's zero-cost abstractions while preventing common memory errors.*
