# Variance - How Subtyping Works with Generic Types

*Variance determines how subtyping relationships between types (like lifetimes) are preserved, reversed, or eliminated when those types are used as generic parameters.*

---

## 🎯 **Core Concept**

**Variance** describes the rules for how **Type\<A\>** relates to **Type\<B\>** when **A** is a subtype of **B**. This is critical for Rust's type system because lifetimes form a subtype hierarchy (longer lifetimes are subtypes of shorter ones), and variance determines when the borrow checker accepts type substitutions.

**Three Kinds of Variance:**
1. **Covariant** - Subtyping preserved: If `A` is subtype of `B`, then `Type<A>` is subtype of `Type<B>`
2. **Invariant** - No subtyping: `Type<A>` and `Type<B>` are unrelated regardless of `A` and `B`
3. **Contravariant** - Subtyping reversed: If `A` is subtype of `B`, then `Type<B>` is subtype of `Type<A>`

---

## 🧠 **Mental Models**

### **The Library Analogy**

Think of variance like library access policies:

- **Covariant (Shared Reference `&T`)**: Like a library book on loan
  - You have read-only access
  - A permanent collection book (long lifetime) can be checked out with a temporary library card (short lifetime)
  - ✓ Longer lifetime → Shorter lifetime is safe (read-only)

- **Invariant (Mutable Reference `&mut T`)**: Like renting an apartment
  - You can modify the space
  - The lease duration must match exactly - can't substitute a yearly lease for a monthly one
  - ✗ Lifetime substitution is unsafe (could cause use-after-free)

- **Contravariant (Function Arguments)**: Like job qualifications
  - A function accepting "any employee" (short requirement) can handle "senior engineer" (longer requirement)
  - ✓ Shorter requirement → Longer qualification is safe

### **Lifetime Subtyping Foundation**

Before understanding variance, remember how lifetimes form subtypes:

```rust
// Longer lifetimes are subtypes of shorter lifetimes
'static: 'a   // 'static is subtype of any 'a
'a: 'b        // 'a is subtype of 'b (if 'a outlives 'b)

// This means:
// - 'static can substitute for any lifetime
// - Longer-lived references can substitute for shorter-lived ones
```

---

## 🔍 **Detailed Content**

### **1. Covariance - Subtyping Preserved**

**Definition**: If `'a: 'b` (longer lifetime), then `Type<'a>` can be used as `Type<'b>`.

**Why It's Safe**: Read-only access means longer-lived data can safely be used where shorter-lived data is expected.

```rust
// &'a T is COVARIANT over 'a
fn example() {
    let static_str: &'static str = "hello";
    
    fn takes_short_lifetime(s: &str) {  // Generic 'a lifetime
        println!("{}", s);
    }
    
    // ✓ Works! 'static is longer than 'a
    // Covariance: &'static str → &'a str
    takes_short_lifetime(static_str);
}
```

**Common Covariant Types:**
- `&'a T` - Shared references (covariant over both `'a` and `T`)
- `Box<T>` - Heap allocation (covariant over `T`)
- `Vec<T>` - Dynamic arrays (covariant over `T`)
- `*const T` - Const raw pointers (covariant over `T`)

**Real-World Example:**

```rust
fn print_slice(slice: &[i32]) {  // Generic 'a lifetime
    println!("{:?}", slice);
}

let vec = vec![1, 2, 3];
let slice: &[i32] = &vec;  // Lifetime tied to vec

// ✓ Covariance allows vec's lifetime to be used for shorter 'a
print_slice(slice);
```

### **2. Invariance - No Subtyping**

**Definition**: `Type<'a>` and `Type<'b>` are completely unrelated, even if `'a: 'b`.

**Why It's Necessary**: Mutation requires exact lifetime matching to prevent lifetime shrinking bugs.

```rust
// &'a mut T is INVARIANT over 'a
// This prevents a critical soundness hole

// Hypothetical bug if &mut were covariant:
fn would_be_unsound() {
    let mut long_lived = String::from("data");
    let long_ref: &'static mut String = /* ... */;
    
    // If covariant, could pass to function expecting shorter lifetime
    fn shrink_lifetime<'a>(r: &'a mut String) {
        // Store reference with shorter lifetime
    }
    
    // shrink_lifetime(long_ref);  // Would compile if covariant
    // Now long_ref points to freed memory! 💥
    // Invariance prevents this bug
}
```

**The Soundness Issue Explained:**

```rust
// Why &mut is invariant over its lifetime:
// 
// 1. Start with &'static mut reference
// 2. If covariant, could pass to fn(&'a mut T)
// 3. Function stores reference with lifetime 'a (short)
// 4. 'a expires, data is freed
// 5. Original &'static mut reference now dangles! 💥
//
// Invariance solution: &'static mut ≠ &'a mut (no substitution)
```

**Common Invariant Types:**
- `&'a mut T` - Mutable references (invariant over `'a`, covariant over `T`)
- `UnsafeCell<T>` - Interior mutability primitive (invariant over `T`)
- `Cell<T>` - Single-threaded interior mutability (invariant over `T`)
- `RefCell<T>` - Runtime-checked borrowing (invariant over `T`)

**Real-World Example:**

```rust
fn exact_lifetime_required<'a>(r: &'a mut &'a str) {
    // &mut is invariant - must match 'a exactly
}

let mut s = "hello";
let r = &mut s;

exact_lifetime_required(r);  // ✓ Exact lifetime match

// ✗ Cannot substitute longer or shorter lifetimes
// Invariance enforces exact match
```

### **3. Contravariance - Subtyping Reversed**

**Definition**: If `'a: 'b` (longer lifetime), then `Type<'b>` can be used as `Type<'a>`.

**Why It Exists**: Function arguments are contravariant - a function accepting shorter-lived data can handle longer-lived data.

```rust
// fn(&'a T) is CONTRAVARIANT over 'a in the argument position

// Function accepting short lifetime
fn takes_any_lifetime(s: &str) {  // Generic 'a
    println!("{}", s);
}

// Can be called with longer lifetime
let static_str: &'static str = "hello";
takes_any_lifetime(static_str);  // ✓ Works

// Contravariance: fn(&'short) can accept &'long
// Reversed subtyping - shorter requirement accepts longer data
```

**Return Types Are Covariant:**

```rust
// fn() -> &'a T is COVARIANT over 'a in return position

fn returns_static() -> &'static str {
    "hello"
}

// Can assign to shorter lifetime
let s: &str = returns_static();  // ✓ Covariant over return
```

**Why Contravariance is Rare:**

Contravariance appears primarily in function pointer argument positions. Most Rust code doesn't directly interact with contravariance - it's handled transparently by the compiler.

---

## 📊 **Variance Quick Reference Table**

| **Type** | **Variance over T** | **Variance over 'a** |
|----------|---------------------|----------------------|
| `&'a T` | Covariant | Covariant |
| `&'a mut T` | Covariant | **Invariant** |
| `Box<T>` | Covariant | N/A |
| `Vec<T>` | Covariant | N/A |
| `UnsafeCell<T>` | **Invariant** | N/A |
| `Cell<T>` | **Invariant** | N/A |
| `RefCell<T>` | **Invariant** | N/A |
| `fn(T) -> U` | **Contravariant** (T) | Covariant (U) |
| `*const T` | Covariant | N/A |
| `*mut T` | **Invariant** | N/A |

**Key Patterns:**
- **Shared access** → Covariant (safe to use longer lifetimes)
- **Mutable access** → Invariant (must match exactly)
- **Function args** → Contravariant (can accept longer lifetimes)
- **Function returns** → Covariant (can return longer lifetimes)

---

## 🛠️ **PhantomData for Controlling Variance**

`PhantomData<T>` allows you to explicitly control variance in custom types:

```rust
use std::marker::PhantomData;

// Make MyType covariant over T
struct MyType<T> {
    _marker: PhantomData<T>,  // Covariant
}

// Force invariance over T
struct InvariantType<T> {
    _marker: PhantomData<Cell<T>>,  // Invariant (Cell is invariant)
}

// Force invariance over lifetime
struct InvariantLifetime<'a> {
    _marker: PhantomData<&'a mut ()>,  // Invariant over 'a
}
```

**When to Use PhantomData for Variance:**
- Writing unsafe code that requires specific variance properties
- Creating wrapper types with controlled subtyping behavior
- Ensuring soundness in custom smart pointers or collections

Related: [[PhantomData Type Safety Patterns]]

---

## 💡 **Key Takeaways**

1. **Covariance (common)**: `&'a T`, `Box<T>`, `Vec<T>` - Longer lifetimes can substitute for shorter ones
2. **Invariance (safety-critical)**: `&'a mut T`, `UnsafeCell<T>` - Exact lifetime match required to prevent bugs
3. **Contravariance (rare)**: Function arguments - Shorter requirements can accept longer-lived data
4. **Variance is mostly invisible**: The compiler handles it automatically - you notice it only when you get lifetime errors
5. **Debugging tip**: "Lifetime mismatch" errors with `&mut` often mean you're hitting invariance - lifetimes must match exactly
6. **Unsafe code**: Understanding variance is critical when writing unsafe code or custom smart pointers

---

## 🎯 **Real-World Impact**

### **Borrow Checker Integration**

The borrow checker uses variance rules extensively:

```rust
// Covariance example - Borrow checker allows this
fn works_due_to_covariance() {
    let long: &'static str = "data";
    let short: &str = long;  // ✓ Covariance: &'static → &'a
}

// Invariance example - Borrow checker rejects this
fn blocked_by_invariance() {
    let mut s = String::from("data");
    let long_ref: &mut String = &mut s;
    
    // ✗ Cannot shorten &mut lifetime - invariant!
    // let short_ref: &'short mut String = long_ref;
}
```

### **When Variance Matters Most**

1. **Writing unsafe code**: Must understand variance to maintain soundness
2. **Custom smart pointers**: Need `PhantomData` to set correct variance
3. **Debugging lifetime errors**: Understanding invariance explains `&mut` restrictions
4. **API design**: Affects what lifetime patterns users can express

### **Common Errors Explained by Variance**

```rust
// Error: "lifetime mismatch"
fn buggy<'a, 'b>(x: &'a mut &'b str) -> &'a mut &'a str {
    x  // ✗ Doesn't compile!
}

// Explanation: &mut is invariant over 'b
// Cannot convert &'a mut &'b str → &'a mut &'a str
// Would require 'b == 'a exactly (invariance)
```

---

## 🔗 **Integration Points**

### **Builds On**
- [[lifetime-parameters]] - Lifetime subtyping and generic lifetimes are the foundation
- [[Borrow Checker Fundamentals]] - Borrow checker uses variance rules for type checking
- [[Ownership and Borrowing]] - Shared vs mutable references have different variance

### **Enables**
- [[PhantomData Type Safety Patterns]] - Using `PhantomData` to control variance in custom types
- [[interior-mutability]] - Understanding why `Cell`/`RefCell` are invariant over `T`
- [[Unsafe Rust - Raw Pointers and Safety Contracts]] - Variance is critical for unsafe code soundness

### **Related Concepts**
- [[Generic Programming]] - Variance applies to all generic parameters (lifetimes and types)
- [[Smart Pointers MOC]] - Smart pointer variance affects borrow checker behavior
- [[refcell-interior-mutability]] - Interior mutability types use invariance for safety
- [[Send and Sync Deep Dive]] - Thread safety traits interact with variance rules

### **Practical Applications**
- [[mission-4]] - `LinkedList<T>` with `Rc<RefCell<T>>` demonstrates invariance challenges
- [[rust_book/rust-book-ch10]] - Generic lifetimes and variance in practice
- [[Memory Safety]] - Variance prevents memory safety violations

---

*Tags: #rust-for-rustaceans #variance #type-system #lifetimes #subtyping #covariance #invariance #contravariance #borrow-checker #advanced*

*Links: [[zettel-index]] | [[rust-concepts-MOC]] | [[lifetime-parameters]] | [[Borrow Checker Fundamentals]] | [[PhantomData Type Safety Patterns]] | [[interior-mutability]] | [[Generic Programming]]*
