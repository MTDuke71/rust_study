# Advanced Traits Patterns - Associated Types, Default Type Parameters, Supertraits

A practical set of patterns for designing trait APIs that are expressive, ergonomic, and easy to implement.

---

## Core Concept

“Advanced traits” features aren’t about making code clever; they’re about making interfaces (trait contracts) that scale.

Three techniques show up repeatedly in real Rust APIs:

- **Associated types**: a trait chooses *one* “output type family” per implementor.
- **Default type parameters**: a trait or type can be used in the common case without extra annotations, but still supports customization.
- **Supertraits**: a trait can require that implementors also satisfy other trait contracts.

These are the tools that let you expose clean “integration points” between components, without forcing callers to thread generics everywhere.

## Mental Models

- **Associated type = “a fixed plug shape”**
  - Once a type implements the trait, the associated type is a fixed part of the contract (e.g., `Iterator::Item`).
  - This prevents the “same implementor, many output types” ambiguity.

- **Default type parameter = “sane configuration with an escape hatch”**
  - The common case should be ergonomic.
  - The uncommon case should still be possible without copy/paste traits.

- **Supertrait = “requires capability X”**
  - Your trait can assume functionality from another trait.
  - This is trait composition: a contract built from smaller contracts.

## Detailed Content

### Associated Types

Associated types are used when the trait needs to talk about a type that is *determined by the implementor*.

Classic example: `Iterator`.

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

#### Why not a generic parameter?

A generic form can be more flexible, but it can also create ambiguity:

```rust
// Generic parameter form (more flexible, sometimes too flexible)
trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

// Associated type form (preferred for Iterator)
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

With associated types, a concrete type implements `Iterator` once, and the `Item` is uniquely determined.

#### Pattern: “Output type belongs to the implementor”

Use an associated type when your trait is describing what a type *produces* or *contains*.

- `Iterator::Item`
- `Future::Output`
- Builders often use `type Output`

#### Pitfall: attempting multiple output types

If you need the same implementor to support multiple “output types” depending on usage, a generic parameter is usually the right tool.

### Default Type Parameters

Default type parameters make the common case short, while still allowing customization.

#### Pattern: operator traits and `Rhs = Self`

The standard library uses this heavily. For example, `Add` is defined as:

```rust
// simplified
trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

This means “adding `Self + Self`” is the default, but you can provide a different right-hand side type.

#### Example: “units” addition

```rust
use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}
```

#### Orphan rule connection (why examples often define local types)

When you want to implement an **external trait** (like `Add`) for a type, Rust’s orphan rule applies:

- You can implement an external trait for a local type.
- Or implement a local trait for an external type.
- But you cannot implement an external trait for an external type.

If your “units” types live in another crate and you don’t own them, the usual escape hatch is the **newtype pattern**.

### Supertraits

Supertraits are trait composition: a trait can require that implementors also implement other traits.

#### Pattern: “extension trait needs Display”

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

This makes the contract explicit: `OutlinePrint` can call `to_string()` because `Display` is required.

#### Pattern: “trait layering”

Supertraits are a clean way to define layered capabilities:

- base trait: minimal required operations
- advanced trait: higher-level helpers built from base operations

This mirrors how component interfaces evolve: simple ports first, then richer adapters.

## Key Takeaways

- Associated types are best when the output type is owned by the implementor’s contract.
- Default type parameters keep the common API ergonomic while preserving customization.
- Supertraits let you compose capability contracts and keep trait method bodies simple.
- These features are primarily API design tools, not “syntax tricks.”

## Integration Points

### Builds On

- [[Traits]] - Trait contracts, bounds, and implementation patterns
- [[Generics]] - Type parameters, bounds, and associated-types-vs-generics comparisons
- [[Ownership and Borrowing]] - Ownership implications of trait method signatures

### Enables

- [[Trait Design Patterns - Mission3 Lessons]] - Designing trait APIs that scale
- [[deref-trait]] - Ergonomic APIs via trait-driven behavior
- [[mission-composition-patterns]] - Connecting components through interface contracts

### Related Concepts

- [[rust_book/rust-book-ch20]] - Rust Book Chapter 20 advanced features context
- [[daily-study/Day18]] - Prior deep-dive on advanced traits
- [[state-pattern-rust]] - State and interface contracts

---

*Tags: #concept #pattern #rust-book #traits #generics #advanced*

*Links: [[zettel-index]] | [[rust-concepts-MOC]] | [[rust_book/rust-book-ch20]] | [[Traits]] | [[Generics]] | [[daily-study/Day18]]*
