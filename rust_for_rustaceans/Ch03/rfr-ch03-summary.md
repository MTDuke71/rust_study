
# Chapter 3: Designing Interfaces

**Zettelkasten**: [[rust-for-rustaceans]] - Rustaceans book knowledge hub

This chapter focuses on four core principles for creating idiomatic, reliable, and usable Rust interfaces: making them **Unsurprising**, **Flexible**, **Obvious**, and **Constrained**.

---

## 1. Unsurprising

Interfaces should be intuitive, allowing users to rely on prior knowledge and common conventions to guess how code works.

### a. Naming Practices

- Reuse common names from the standard library (e.g., `iter`, `into_inner`) to signal expected behavior.
- Ensure things sharing a name function similarly to avoid user frustration.

### b. Common Traits for Types

- Implement standard traits eagerly so types "just work." Users expect `Debug`, `Send`, `Sync`, `Clone`, and `Default`.
- Implement comparison traits (`PartialEq`, `PartialOrd`, `Hash`, `Eq`, `Ord`) where appropriate, especially for types used as keys.
- Consider `serde` support via a feature flag.
- **Note on Copy**: Be cautious implementing `Copy`. It changes move semantics, and removing it later is a breaking change.

### c. Ergonomic Trait Implementations

- Implement traits for references (`&T`, `&mut T`, `Box<T>`) so users can pass borrowed values to functions expecting generics.
- Implement `IntoIterator` for references to collection types to allow iterating over borrowed instances.

### d. Wrapper Types

- Use `Deref` and `AsRef` to provide "inheritance-like" behavior for transparent wrappers (e.g., `Arc`, `Box`).
- Use `Borrow` specifically when a type is essentially equivalent to another (e.g., `String` and `str`) regarding hashing and comparison.

---

## 2. Flexible

Code implies a contract of requirements (restrictions) and promises (guarantees). Interfaces should avoid unnecessary restrictions and only make promises they can keep.

### a. Generic Arguments

- Use generics (e.g., `impl AsRef<str>`) rather than concrete types to accept a wider range of inputs.
- Consider the trade-off between generics (monomorphization bloat) and dynamic dispatch (using `dyn Trait` to reduce binary size).

### b. Object Safety

- Design traits to be object-safe (no generic methods, no methods returning `Self`) to allow users to use `dyn Trait`.
- If a method cannot be object-safe, add `where Self: Sized` to exclude it from the trait object vtable.

### c. Borrowed vs. Owned

- If code needs ownership, require owned data. If not, operate on references.
- Use `Cow` (Clone on Write) for return types that might be owned or borrowed depending on runtime conditions.

### d. Fallible and Blocking Destructors

- `Drop` cannot fail or run async code. Provide explicit destructors (e.g., a `close` method returning `Result`) for graceful teardown, using `Drop` only as a best-effort fallback.

---

## 3. Obvious

Interfaces should be easy to understand and hard to misuse.

### a. Documentation

- Document unexpected behaviors like panics, error conditions, and unsafe guarantees.
- Include end-to-end usage examples at the module level.
- Use `#[doc(cfg(..))]` to highlight feature-gated items and `#[doc(alias = "...")]` to aid discoverability.

### b. Type System Guidance

**Semantic Typing**: Use specific types (enums, newtypes) rather than primitives (like `bool`) to prevent argument ordering errors.

**Zero-Sized Types / Typestates**: Use `PhantomData` and marker types (e.g., `Rocket<Grounded>`) to make illegal states unrepresentable at compile time.

**`#[must_use]`**: Annotate types or functions (like `Result`) where ignoring the return value is likely a bug.

---

## 4. Constrained

Public changes often break downstream code. Interfaces should be constrained to allow future evolution without breaking changes.

### a. Type Modifications

- Minimize public types. Adding private fields to public structs breaks code that uses exhaustive pattern matching. Use `#[non_exhaustive]` to mitigate this.

### b. Trait Implementations

- Adding blanket implementations or implementing foreign traits for existing types can violate coherence rules and break downstream crates.
- **Sealed Traits**: Use a private supertrait to prevent downstream users from implementing a trait, allowing the maintainer to add methods safely.

### c. Hidden Contracts

**Re-exports**: Exposing dependencies' types in a public API makes updates to those dependencies breaking changes. Wrap foreign types or use `impl Trait` to hide them.

**The SemVer Trick**: A technique to support multiple major versions of a dependency by re-exporting the old types from the new version.

**Auto-Traits**: Types automatically implement `Send` and `Sync`. Internal changes that remove these properties are breaking changes. Test to ensure these auto-traits are preserved.

---

## 5. Summary

This chapter covers:
- **Unsurprising**: Naming conventions, common traits, ergonomic implementations, wrapper types
- **Flexible**: Generic arguments, object safety, ownership patterns, fallible destructors
- **Obvious**: Documentation best practices, semantic typing, type-state pattern, `#[must_use]`
- **Constrained**: Type evolution strategies, sealed traits, re-export considerations, auto-trait preservation
