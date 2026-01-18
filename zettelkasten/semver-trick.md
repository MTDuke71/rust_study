# SemVer Trick - Supporting Multiple Dependency Versions

The SemVer trick is a pattern for library authors to support users on different versions of a dependency, enabling gradual migration without breaking changes.

---

## Core Concept

**Problem**: Your library depends on `serde v1.0`, but some users are still on `serde v0.9`. Updating your dependency is a breaking change for users on the old version.

**Solution**: Re-export both versions' types with distinct names, allowing users to migrate at their own pace.

```rust
// Support both old and new versions
pub use serde_v1::{Serialize as Serialize_v1};
pub use serde_v0_9::{Serialize as Serialize_v0_9};

// Prefer new version by default
pub use serde_v1::Serialize;
```

This pattern is called the "SemVer trick" because it allows you to maintain semantic versioning compatibility while upgrading dependencies.

## Mental Models

**Bridge Between Versions**
- Your library acts as a translation layer
- Users on old version use old types
- Users on new version use new types
- Both can coexist temporarily

**Gradual Migration Path**
- Phase 1: Library supports both versions
- Phase 2: Users migrate at their own pace
- Phase 3: Old version deprecated after sufficient time
- Phase 4: Remove old version support (major version bump)

## Detailed Content

### Basic Pattern

```rust
// In your Cargo.toml
[dependencies]
serde_new = { package = "serde", version = "1.0" }
serde_old = { package = "serde", version = "0.9", optional = true }

// In your lib.rs
// Primary API uses new version
pub use serde_new::{Serialize, Deserialize};

// Compatibility re-exports for old version
#[cfg(feature = "serde-0.9-compat")]
pub use serde_old::{
    Serialize as Serialize_v0_9,
    Deserialize as Deserialize_v0_9,
};
```

### Type Aliasing Pattern

```rust
// Expose versions as type aliases
pub mod v1 {
    pub use new_dep::Type;
}

pub mod v0 {
    pub use old_dep::Type;
}

// Default to latest
pub use v1::Type;
```

### Re-export Wrapper Pattern

Instead of exposing foreign types directly, wrap them:

```rust
// Bad: Direct re-export
pub use foreign_crate::ForeignType;  // ❌ Breaking change if dependency updates

// Good: Wrap the foreign type
pub struct MyType {
    inner: foreign_crate::ForeignType,  // Hidden implementation detail
}

impl MyType {
    pub fn new(data: String) -> Self {
        Self {
            inner: foreign_crate::ForeignType::from(data),
        }
    }
    
    // Expose only needed functionality
    pub fn process(&self) -> String {
        self.inner.process()
    }
}
```

### Feature Flag Pattern

```rust
// Cargo.toml
[features]
default = ["new-version"]
new-version = ["dep_v2"]
old-version = ["dep_v1"]

[dependencies]
dep_v1 = { package = "dependency", version = "1.0", optional = true }
dep_v2 = { package = "dependency", version = "2.0", optional = true }

// lib.rs
#[cfg(feature = "new-version")]
pub use dep_v2::Type;

#[cfg(feature = "old-version")]
pub use dep_v1::Type;
```

### When NOT to Use SemVer Trick

**Don't expose foreign types in public API**:
```rust
// ❌ BAD - dependency leaks into your public API
pub fn process(data: DependencyType) -> Result<DependencyType, DependencyError> {
    // ...
}

// ✅ GOOD - wrap or abstract
pub fn process(data: String) -> Result<String, MyError> {
    let dep_data = DependencyType::from(data);
    // ...
}
```

**Use `impl Trait` to hide implementation**:
```rust
// ❌ BAD - exposes concrete iterator type
pub fn get_items() -> DependencyIterator<Item = String> {
    // ...
}

// ✅ GOOD - hides concrete type
pub fn get_items() -> impl Iterator<Item = String> {
    // Implementation can change without breaking API
}
```

## Rust for Rustaceans Connection

From **Chapter 3: Designing Interfaces**, the SemVer trick addresses:

### Re-export Hazards

**Problem**: Public API exposes dependency types
- Updating dependency = breaking change for users
- Users' code depends on specific dependency version
- Your semver promises are entangled with dependency's semver

**Solution 1**: Wrapper types (preferred)
```rust
pub struct MyWrapper {
    inner: ForeignType,
}
```

**Solution 2**: `impl Trait`
```rust
pub fn get_data() -> impl Iterator<Item = String> {
    dependency::get_items()
}
```

**Solution 3**: SemVer trick (when migration support needed)
```rust
pub use dep_v1::Type as Type_v1;
pub use dep_v2::Type;
```

### Hidden Contracts

Auto-traits (`Send`, `Sync`) are part of the hidden contract:

```rust
// Before: Send + Sync
pub struct Cache {
    data: Vec<String>,
}

// After: NOT Send + NOT Sync (BREAKING!)
use std::rc::Rc;
pub struct Cache {
    data: Vec<String>,
    shared: Rc<i32>,  // ❌ Dependency change broke auto-traits
}
```

**Prevention**: Test auto-traits
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn ensure_thread_safe() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Cache>();
    }
}
```

## Common Use Cases

### Database Crate Migration

```rust
// Support both diesel 1.x and 2.x
pub use diesel_v1::Connection as Connection_v1;
pub use diesel_v2::Connection;

pub trait DatabaseConnection {
    fn execute(&mut self, query: &str) -> Result<(), Error>;
}

// Implement for both versions
impl DatabaseConnection for diesel_v1::Connection { /* ... */ }
impl DatabaseConnection for diesel_v2::Connection { /* ... */ }
```

### Serialization Format Changes

```rust
// Support both JSON formats
pub mod v1 {
    use serde::{Serialize, Deserialize};
    
    #[derive(Serialize, Deserialize)]
    pub struct Data {
        pub name: String,
    }
}

pub mod v2 {
    use serde::{Serialize, Deserialize};
    
    #[derive(Serialize, Deserialize)]
    pub struct Data {
        pub name: String,
        pub version: u32,
    }
}

// Default to latest
pub use v2::Data;
```

### HTTP Client Updates

```rust
// reqwest 0.11 -> 0.12 migration support
#[cfg(feature = "reqwest-0-11")]
pub use reqwest_v11::Client;

#[cfg(feature = "reqwest-0-12")]
pub use reqwest_v12::Client;
```

## Best Practices

1. **Document the migration path** in your CHANGELOG and README
2. **Set deprecation timeline** for old version support
3. **Test both versions** if supporting multiple simultaneously
4. **Use feature flags** to make old version optional
5. **Prefer wrapping** over re-exporting when possible
6. **Bump major version** when finally removing old version support

## Migration Timeline Example

```markdown
## Version 2.0.0
- Add support for dependency v2.0
- Maintain compatibility with dependency v1.0 (feature flag)
- Old version deprecated, will be removed in v3.0.0

## Version 2.5.0
- Dependency v2.0 is now default
- Dependency v1.0 still available with `old-version` feature

## Version 3.0.0
- Remove dependency v1.0 support
- Breaking change: users must upgrade to dependency v2.0
```

## Key Takeaways

- **SemVer trick = temporary bridge** between dependency versions
- **Re-exports leak dependencies** into your public API (avoid when possible)
- **Wrapper types isolate changes** (preferred approach)
- **`impl Trait` hides concrete types** (good for iterators, futures)
- **Auto-traits are hidden contracts** (test for Send/Sync)
- **Plan migration timeline** (deprecation → removal)

---

## Related Concepts

*Links:*
- [[auto-traits-send-sync]] - Hidden contracts that can break with dependency updates
- [[rust-for-rustaceans]] - Chapter 3: Designing Interfaces
- [[API Design Patterns]] - Public API stability patterns
- [[API Design Principles]] - Interface design best practices
- [[sealed-traits]] - Preventing external implementations
- [[orphan-rule]] - Trait implementation restrictions
- [[impl-trait]] - Hiding concrete types in return position

*Tags: #rust #semver #api-design #dependencies #compatibility #rustaceans #versioning*
