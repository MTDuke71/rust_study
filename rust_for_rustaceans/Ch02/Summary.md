# Chapter 2: Types

This chapter explores how the Rust compiler represents types in memory and how the trait system functions under the hood, covering alignment, layout, dispatch, and coherence rules.

## 1. Types in Memory

### a. Alignment

**Alignment**: Hardware constrains where bytes can be stored. Values must be at least byte-aligned, but often require natural alignment (alignment matches the size of the value) for efficient CPU access.

- The compiler inserts padding (bytes with indeterminate values) to ensure fields satisfy alignment requirements.

### b. Layout

**Layout Representations**:

- **`repr(Rust)`**: The default. No guarantees on field ordering. The compiler reorders fields to minimize padding and size.
- **`repr(C)`**: Compatible with C compilers. Preserves field order, useful for FFI and raw pointer operations.
- **`repr(packed)`**: Removes padding between fields. May lead to misaligned accesses (slower) or crashes on some architectures.
- **`repr(transparent)`**: Used on single-field structs (newtypes). Guarantees the layout is identical to the inner type.

### c. Complex Types

**Symbol Representations**:

- **Enums**: Represented like unions with a hidden discriminant field (tag).
- **Tuples/Arrays**: Contiguous sequences with no padding between elements (arrays) or ordered fields (tuples).

### d. Dynamically Sized Types and Wide Pointers

**Dynamically Sized Types (DSTs)**: Types without a known size at compile time (e.g., `dyn Trait`, `[u8]`).

- **Wide Pointers**: Pointers to DSTs are "wide" or "fat." They consist of two `usize` fields: the pointer to the data and extra information (length for slices, vtable for trait objects).

---

## 2. Traits and Trait Bounds

### a. Compilation and Dispatch

**Dispatch Mechanisms**:

**1. Static Dispatch**:
- Uses monomorphization. The compiler creates a unique copy of the function/type for every concrete type used.
- **Pros**: Highly optimized (inlining), faster execution.
- **Cons**: Slower compile times, larger binary size (code bloat).

**2. Dynamic Dispatch**:
- Uses Trait Objects (`dyn Trait`).
- Relies on a Vtable (virtual method table) containing pointers to method implementations and the type's layout/alignment.
- **Object Safety**: Not all traits can be trait objects. Traits are not object-safe if methods return `Self`, use generics, or are static methods.

### b. Generic Traits

**Generic Type Parameters vs. Associated Types**:

- **Generic Type Parameters** (`trait Foo<T>`): Use when a type needs multiple implementations of the same trait (e.g., `From<T>`).
- **Associated Types** (`trait Foo { type Bar; }`): Use when there should only be one implementation per type. This improves ergonomics by not requiring users to specify generic parameters on every use.

### c. Coherence and the Orphan Rule

**Coherence** ensures there is exactly one implementation of a trait for any given type.

**Orphan Rule**: You can implement a trait for a type only if the trait or the type is local to your crate.

- **Blanket Implementations**: `impl<T> Trait for T` are allowed only by the defining crate and are considered breaking changes to add later.
- **Fundamental Types**: Types like `&`, `&mut`, and `Box` are `#[fundamental]`. They are essentially erased for orphan rule checks, allowing implementations on `&LocalType`.
- **Covered Implementations**: You can implement a foreign trait for a foreign type if a local type appears as a generic type parameter (e.g., `impl From<LocalType> for Vec<ForeignType>`).

### d. Trait Bounds

**Advanced Trait Bounds**:

- **Arbitrary Bounds**: Bounds do not have to reference the generic parameter directly (e.g., `where String: Clone` is valid).
- **Higher-Ranked Trait Bounds (HRTB)**: Using `for<'a>`, you can specify that a bound must hold for all lifetimes. This is common with `Fn` traits taking references.

### e. Marker Traits

**Marker Traits**: Traits with no methods (e.g., `Send`, `Sync`, `Copy`) used to declare properties about a type.

**Marker Types**: Zero-sized types (Unit structs) used to indicate state at the type level (e.g., `Unauthenticated`) to prevent API misuse.

---

## 3. Existential Types

**Existential Types**: Allow returning a type without naming it, asserting that "there exists" a type that satisfies the bounds.

**`impl Trait`**:

- Used in return positions (and async functions).
- Enables zero-cost type erasure: The compiler knows the concrete type (allowing optimization), but the user/API sees only the trait capabilities.
- Useful for closures and iterators where the concrete type is unnameable or complex.

---

## 4. Summary

This chapter covers:
- How Rust represents types in memory (alignment, layout, DSTs)
- The trait system (dispatch, generics, coherence)
- Advanced trait features (bounds, markers, existential types)