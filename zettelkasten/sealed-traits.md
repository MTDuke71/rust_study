# Sealed Traits

*Tags: #rust #traits #api-design #breaking-changes #coherence*

## Overview

The **sealed trait pattern** is a design technique that prevents external crates from implementing a trait, giving library authors control over all implementations. This allows adding trait methods (even without defaults) without breaking changes, since all implementations are within the library's control.

## The Problem: Trait Implementation Breaking Changes

Rust's **coherence rules** ensure a trait is implemented at most once for any given type. However, this creates several breaking change scenarios when evolving traits:

### Breaking Changes with Trait Implementations

1. **Blanket Implementations**
   ```rust
   // Adding this to an existing trait is BREAKING:
   impl<T> MyTrait for T { /* ... */ }
   ```
   - Downstream crates may have already implemented `MyTrait` for their types
   - Creates conflicting implementations (coherence violation)

2. **Foreign Trait/Type Implementations**
   ```rust
   // BREAKING: Implementing foreign trait for your type
   impl std::fmt::Display for MyType { /* ... */ }
   
   // BREAKING: Implementing your trait for foreign type
   impl MyTrait for Vec<T> { /* ... */ }
   ```
   - Foreign item owner might add same implementation
   - Creates ambiguity or conflict

3. **Method Name Ambiguity**
   ```rust
   // Library adds:
   pub trait MyTrait {
       fn foo(&self);
   }
   impl MyTrait for ExistingType { /* ... */ }
   
   // Downstream user already had:
   trait TheirTrait {
       fn foo(&self);
   }
   impl TheirTrait for ExistingType { /* ... */ }
   
   // Now: type.foo() is ambiguous!
   ```

## The Sealed Trait Pattern

**Solution**: Make trait only implementable within your crate by requiring a sealed supertrait.

### Implementation

```rust
// Private module prevents external access
mod sealed {
    pub trait Sealed {}
    
    // Only types you control can implement Sealed
    impl Sealed for TypeA {}
    impl Sealed for TypeB {}
}

// Public trait requires sealed supertrait
pub trait MyTrait: sealed::Sealed {
    fn method(&self);
}

// You control all implementations
impl MyTrait for TypeA {
    fn method(&self) { /* ... */ }
}

impl MyTrait for TypeB {
    fn method(&self) { /* ... */ }
}
```

**Why This Works**:
- `sealed::Sealed` is private → external crates cannot import it
- Cannot implement `MyTrait` without implementing `Sealed`
- Cannot implement `Sealed` because it's private
- Therefore: Only your crate can implement `MyTrait`

## Benefits

1. **Non-Breaking Method Additions**
   ```rust
   pub trait MyTrait: sealed::Sealed {
       fn existing(&self);
       fn new_method(&self); // ✅ Not breaking! No external impls exist
   }
   ```

2. **Exhaustive Match Safety**
   - Can safely match on all trait implementors
   - No risk of external types causing match incompleteness

3. **Implementation Evolution**
   - Can change blanket impl constraints
   - Can add/modify default implementations safely

## When to Use Sealed Traits

**Use when**:
- Trait represents a fixed set of types (e.g., primitive numeric types)
- You need to add methods without breaking changes
- Exhaustive matching on implementors is important
- Trait is primarily for internal organization

**Don't use when**:
- Trait is meant to be extensible (like `Iterator`, `Debug`)
- Users should define their own implementations
- Trait is a fundamental extension point of your API

## Real-World Examples

### Standard Library
```rust
// std::io::Seek is effectively sealed
pub trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64>;
    // Can add methods because limited implementors
}
```

### Common Pattern: Type-Level Booleans
```rust
mod sealed {
    pub trait Sealed {}
    impl Sealed for True {}
    impl Sealed for False {}
}

pub trait Bool: sealed::Sealed {
    const VALUE: bool;
}

pub struct True;
pub struct False;

impl Bool for True { const VALUE: bool = true; }
impl Bool for False { const VALUE: bool = false; }
```

## Trade-offs

**Advantages**:
- ✅ Future-proof trait evolution
- ✅ No breaking changes when adding methods
- ✅ Clear documentation of intended usage
- ✅ Exhaustive matching possible

**Disadvantages**:
- ❌ Reduces extensibility
- ❌ Users cannot define custom implementations
- ❌ Less flexible API design
- ❌ Pattern not immediately obvious to users

## Related Patterns

- **[[non-exhaustive-pattern]]**: Prevents exhaustive struct construction/matching
- **Builder Pattern**: Alternative to sealed traits for constrained construction
- **Newtype Pattern**: Wrapping types to control trait implementations

## Source

- **Rust for Rustaceans**, Jon Gjengset, Chapter 3: Designing Interfaces, pp. 50-51
- Related to coherence rules and API evolution strategies

---

*Links: [[trait-coherence]] | [[api-design-principles]] | [[breaking-changes]] | [[rust-for-rustaceans-ch3]]*

*Created: 2026-01-16*
