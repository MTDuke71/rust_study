# Type-State Pattern

*Tags: #rust #design-patterns #type-system #zero-cost-abstractions #compile-time-safety*

## Overview

The **type-state pattern** uses Rust's type system to encode state transitions at compile time, making invalid state transitions impossible. Instead of runtime checks, the compiler enforces valid sequences of operations.

## Core Concept

Different states are represented as **distinct types**, not runtime values. Methods that change state consume `self` and return a new type representing the new state.

```rust
// States are empty structs (zero-sized types)
struct Locked;
struct Unlocked;

// Door is generic over its state
struct Door<State> {
    _state: std::marker::PhantomData<State>,
}

impl Door<Locked> {
    fn unlock(self) -> Door<Unlocked> {
        println!("Door unlocked");
        Door { _state: PhantomData }
    }
}

impl Door<Unlocked> {
    fn lock(self) -> Door<Locked> {
        println!("Door locked");
        Door { _state: PhantomData }
    }
    
    fn open(&self) {
        println!("Door opened");
    }
}

// Usage - compile-time enforced!
let door = Door::<Locked>::new();
// door.open();  // ❌ ERROR: no method `open` for Door<Locked>
let door = door.unlock();
door.open();     // ✅ Works - door is now Door<Unlocked>
```

## Key Components

### 1. PhantomData
Since state types are zero-sized, we use `PhantomData<State>` to:
- Tell compiler the struct is generic over `State`
- Maintain zero runtime cost (no actual storage)
- Enable proper variance and drop behavior

### 2. Consuming Self
Methods that transition state take `self` by value (not `&self`):
```rust
fn unlock(self) -> Door<Unlocked>  // Consumes Door<Locked>, returns Door<Unlocked>
```
This prevents using the old state after transition.

### 3. State-Specific Methods
Each `impl` block targets a specific state:
```rust
impl Door<Locked> { ... }    // Methods only for locked doors
impl Door<Unlocked> { ... }  // Methods only for unlocked doors
```

## Real-World Examples

### Builder Pattern with Required Fields
```rust
struct NoName;
struct HasName;
struct NoEmail;
struct HasEmail;

struct UserBuilder<N, E> {
    name: Option<String>,
    email: Option<String>,
    _state: PhantomData<(N, E)>,
}

impl UserBuilder<NoName, NoEmail> {
    fn new() -> Self { ... }
}

impl<E> UserBuilder<NoName, E> {
    fn name(self, name: &str) -> UserBuilder<HasName, E> { ... }
}

impl<N> UserBuilder<N, NoEmail> {
    fn email(self, email: &str) -> UserBuilder<N, HasEmail> { ... }
}

impl UserBuilder<HasName, HasEmail> {
    fn build(self) -> User { ... }  // Only available when both are set!
}
```

### Network Connection States
```rust
struct Disconnected;
struct Connected;
struct Authenticated;

struct Connection<State> { ... }

impl Connection<Disconnected> {
    fn connect(self) -> Result<Connection<Connected>, Error> { ... }
}

impl Connection<Connected> {
    fn authenticate(self, creds: &Credentials) -> Result<Connection<Authenticated>, Error> { ... }
}

impl Connection<Authenticated> {
    fn query(&self, sql: &str) -> QueryResult { ... }  // Only authenticated can query
}
```

## Benefits

| Benefit | Description |
|---------|-------------|
| **Compile-time safety** | Invalid state transitions are caught by compiler |
| **Zero runtime cost** | PhantomData is zero-sized, states are types not values |
| **Self-documenting** | API shows valid transitions in method signatures |
| **No runtime checks** | No `if state == X` conditionals needed |
| **IDE support** | Autocomplete shows only valid methods for current state |

## When to Use

✅ **Good fit:**
- Protocols with strict state sequences (TCP, authentication flows)
- Builders with required vs optional fields
- Resource lifecycle (open/close, lock/unlock)
- Workflows with defined stages

❌ **Not ideal:**
- Many possible states (combinatorial explosion)
- States determined at runtime (use enums instead)
- Simple on/off toggles (overkill)

## Comparison with Enum State

| Type-State Pattern | Enum State |
|-------------------|------------|
| Compile-time enforcement | Runtime enforcement |
| Zero runtime cost | Small enum discriminant |
| States are types | States are values |
| Different methods per state | Match on state in methods |
| Complex generic signatures | Simpler signatures |

## Connection to Marker Traits

The type-state pattern often combines with marker traits like `Send`, `Sync`:
- States can implement different traits
- `PhantomData` affects Send/Sync derivation
- Can encode thread-safety in state types

---

## Related Concepts

*Links:*
- [[phantom-data-type-safety]] - Zero-sized type marker
- [[marker-traits]] - Send, Sync, Copy, etc.
- [[builder-pattern]] - Common use case for type-state
- [[zero-cost-abstractions]] - Compile-time guarantees without runtime overhead
- [[rust-for-rustaceans-ch2]] - Source material on marker traits

## References

- Rust for Rustaceans, Ch 2.2e - Marker Traits
- [Rust API Guidelines - Type Safety](https://rust-lang.github.io/api-guidelines/)
- [Typestate Pattern in Rust (blog)](https://cliffle.com/blog/rust-typestate/)
