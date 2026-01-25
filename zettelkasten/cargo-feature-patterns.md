# Cargo Feature Flag Design Patterns

*Tags: #rust #cargo #features #conditional-compilation #library-design #no-std*

---

## What Are Features?

**Features** are Cargo's mechanism for **optional functionality** and **conditional compilation**:

```toml
[features]
default = ["std"]         # Enabled by default
std = []                  # Standard library support
advanced = ["networking"] # Meta-feature (bundles others)
networking = []           # Individual feature
```

**Enable features:**
```bash
cargo build --features advanced
cargo build --features "std,networking"
cargo build --all-features
cargo build --no-default-features
```

---

## The Additive Principle

**Critical Rule:** Features are **additive only** - enabling a feature **NEVER disables functionality**.

```toml
# ✅ GOOD - Features add capabilities
[features]
compression = []     # Adds compression
encryption = []      # Adds encryption
advanced = ["compression", "encryption"]  # Adds both

# ❌ BAD - Would remove functionality
[features]
fast = []           # Disables safety checks
unsafe-mode = []    # Removes bounds checking
```

**Why?** If crate A enables `compression` and crate B enables `encryption`, the final build has **both** features. You can't have crate C that removes features - would create conflicts.

---

## Pattern 1: Meta-Features (Feature Bundling)

**Bundle related features** for user convenience:

```toml
[features]
# Individual features
networking = []
compression = []
async = []

# Meta-feature bundles them
advanced = ["networking", "compression", "async"]
```

**Usage:**
```bash
# These are EQUIVALENT:
cargo build --features advanced
cargo build --features "networking,compression,async"
```

**Real-world example (tokio):**
```toml
[features]
full = ["fs", "io-util", "net", "rt", "sync", "time"]
fs = []
io-util = []
net = []
rt = []
sync = []
time = []
```

---

## Pattern 2: Weak Dependencies (`?` syntax)

**Control dependency features conditionally** without forcing the dependency:

```toml
[features]
default = ["std"]
std = ["serde?/std"]  # ← "?" means "if serde exists"

[dependencies]
serde = { version = "1.0", optional = true, default-features = false }
```

**What happens:**

| Command | serde enabled? | serde's std feature? |
|---------|----------------|---------------------|
| `cargo build` | ❌ No | N/A |
| `cargo build --features serde` | ✅ Yes | ✅ Yes (via default std) |
| `cargo build --no-default-features --features serde` | ✅ Yes | ❌ No (no_std) |

**Without `?` (forced dependency):**
```toml
std = ["serde/std"]  # ❌ Forces serde to compile whenever std is enabled!
```

**With `?` (weak/optional):**
```toml
std = ["serde?/std"]  # ✅ Only affects serde IF it's already enabled
```

---

## Pattern 3: Cross-Crate Feature Control

**Control your dependencies' features** from your Cargo.toml:

```toml
[features]
std = ["serde?/std"]
      ^^^^^^^^^^^^^^
      └─┬─┘  └─┬──┘
        │      └── serde's "std" feature (different from your "std")
        └───────── serde dependency name
```

**Syntax:** `<dependency>/<feature>`

**Example - tokio runtime selection:**
```toml
[features]
rt-multi-thread = ["tokio/rt-multi-thread"]
rt-current-thread = ["tokio/rt"]

[dependencies]
tokio = { version = "1", features = ["sync"], default-features = false }
```

**Usage:**
```bash
cargo build --features rt-multi-thread  # Enables tokio's multi-threaded runtime
```

---

## Pattern 4: Conditional Compilation

**Use features to gate code** at compile time:

```rust
// Function only exists when feature is enabled
#[cfg(feature = "networking")]
pub fn network_request() {
    // Implementation
}

// Conditional compilation in function body
pub fn process() {
    #[cfg(feature = "compression")]
    {
        compress_data();
    }
    
    #[cfg(not(feature = "compression"))]
    {
        // Alternative path
    }
}

// Runtime check (still compiles, returns bool)
if cfg!(feature = "advanced") {
    println!("Advanced features enabled");
}
```

---

## Pattern 5: Optional Dependencies

**Dependencies only compile when needed:**

```toml
[dependencies]
# Required dependency
serde = "1.0"

# Optional dependencies (create features automatically)
tokio = { version = "1", optional = true }
regex = { version = "1", optional = true }

[features]
# Automatically created features:
# - "tokio" enables the tokio dependency
# - "regex" enables the regex dependency

# Can bundle optional deps:
async = ["tokio"]
parsing = ["regex"]
```

**Effect:**
- `--features tokio` → compiles tokio
- Default build → tokio NOT compiled
- Reduces compile time and binary size!

---

## Pattern 6: no_std Support

**Support embedded/no_std environments:**

```toml
[features]
default = ["std"]
std = ["serde?/std", "alloc"]
alloc = []  # Heap allocation without full std

[dependencies]
serde = { version = "1.0", optional = true, default-features = false }
```

**Code:**
```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
use std::vec::Vec;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;
```

**Usage:**
```bash
# Standard library
cargo build

# no_std with allocator
cargo build --no-default-features --features alloc

# Fully no_std (no heap)
cargo build --no-default-features
```

---

## Pattern 7: Platform-Specific Features

**Combine features with platform detection:**

```toml
[features]
windows-specific = []
unix-specific = []

[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", optional = true }

[target.'cfg(unix)'.dependencies]
libc = { version = "0.2", optional = true }
```

**Code:**
```rust
#[cfg(all(feature = "windows-specific", target_os = "windows"))]
pub fn platform_operation() {
    // Windows implementation
}

#[cfg(all(feature = "unix-specific", target_os = "linux"))]
pub fn platform_operation() {
    // Linux implementation
}
```

---

## Pattern 8: Version-Gated Features

**Gradually deprecate or introduce features:**

```toml
[features]
default = ["v2"]
v1 = []  # Legacy API
v2 = []  # Current API
v3 = []  # Experimental API
```

**Code:**
```rust
#[cfg(feature = "v1")]
#[deprecated(since = "2.0.0", note = "Use v2 API instead")]
pub fn old_api() { }

#[cfg(feature = "v2")]
pub fn current_api() { }

#[cfg(feature = "v3")]
#[doc = "⚠️ Experimental - API may change"]
pub fn experimental_api() { }
```

---

## Design Guidelines

### DO ✅

1. **Use default features for common cases:**
   ```toml
   default = ["std"]
   ```

2. **Make features additive only:**
   - Features enable functionality
   - Never disable or change behavior

3. **Document feature requirements:**
   ```rust
   /// Requires feature: `networking`
   #[cfg(feature = "networking")]
   pub fn connect() { }
   ```

4. **Use weak dependencies for optional deps:**
   ```toml
   std = ["serde?/std"]  # Not ["serde/std"]
   ```

5. **Bundle related features:**
   ```toml
   full = ["compression", "encryption", "networking"]
   ```

### DON'T ❌

1. **Don't make features mutually exclusive:**
   ```toml
   # ❌ BAD - can't enable both!
   sqlite = []
   postgres = []  # Conflicts with sqlite
   ```

2. **Don't use features for configuration:**
   ```toml
   # ❌ BAD - use environment variables or config files
   debug-mode = []
   production-mode = []
   ```

3. **Don't create too many granular features:**
   ```toml
   # ❌ BAD - too fine-grained
   parse-json = []
   parse-yaml = []
   parse-toml = []
   parse-xml = []
   # ✅ BETTER
   parsing = ["json", "yaml", "toml", "xml"]
   ```

4. **Don't force dependencies without `?`:**
   ```toml
   # ❌ BAD
   std = ["serde/std"]  # Forces serde always
   # ✅ GOOD
   std = ["serde?/std"]  # Only if serde enabled
   ```

---

## Real-World Examples

### Ch05 Day 1 Example
```toml
[features]
default = ["std"]
std = ["serde?/std"]                        # Weak dependency
advanced = ["networking", "compression"]    # Meta-feature
networking = []
compression = []
experimental = []

[dependencies]
serde = { version = "1.0", optional = true, default-features = false }
```

### serde (Production Example)
```toml
[features]
default = ["std"]
std = []
alloc = []
derive = ["serde_derive"]
rc = []
```

### tokio (Complex Example)
```toml
[features]
full = ["fs", "io-util", "io-std", "macros", "net", "parking_lot", 
        "process", "rt", "rt-multi-thread", "signal", "sync", "time"]
fs = []
io-util = ["bytes"]
net = ["libc", "mio/os-poll", "mio/os-ext", "mio/net"]
rt = []
rt-multi-thread = ["rt", "parking_lot"]
```

---

## Testing Features

```bash
# Test all feature combinations
cargo test --all-features
cargo test --no-default-features
cargo test --features "compression,networking"

# Check feature-gated code compiles
cargo check --features experimental

# Build docs with all features visible
cargo doc --all-features --open
```

---

## Integration with Missions

**Mission implementations as feature-gated libraries:**
```toml
# Future: missions/Mission5/Cargo.toml
[features]
default = ["std"]
std = []
serde = ["dep:serde"]  # Optional serialization
benchmarking = []      # Enable criterion benchmarks
```

**Using missions in AoC:**
```toml
# advent_of_code/aoc2024/Cargo.toml
[dependencies]
mission6 = { path = "../../missions/Mission6", features = ["serde"] }
mission8 = { path = "../../missions/Mission8" }
```

---

## Performance Impact

| Aspect | Impact |
|--------|--------|
| **Compile time** | Fewer features = faster compilation |
| **Binary size** | Unused features not compiled in |
| **Runtime** | Zero-cost (compiled out) |
| **Maintenance** | More features = more combinations to test |

**Best practice:** Use features to reduce binary bloat, not for runtime configuration.

---

*Links:*
- [[rfr-ch05-summary]] - Rust for Rustaceans Chapter 5
- [[rust-book-ch14]] - Cargo and Crates.io
- [[rust-box-recursive-structures]] - When to use Box vs Rc based on features
- [[mission-5]] - Example: HashMap with optional features
- [[mission-6]] - Example: Grid with feature-gated functionality

*Related Concepts:*
- [[conditional-compilation]] - Platform and feature-based compilation
- [[rust-ownership]] - Why features affect smart pointer choices
- [[workspace-organization]] - Feature management in workspaces

*Created: 2026-01-25*  
*Context: Rust for Rustaceans Ch05 Day 1 - Feature flag exploration, meta-features, weak dependencies, cross-crate feature control*
