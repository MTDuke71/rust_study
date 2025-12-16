# Appendix E: Editions

Rust uses an **edition system** to evolve the language while maintaining backward compatibility. Editions bring together incremental features into cohesive packages released approximately every three years.

## What Are Editions?

Editions are opt-in milestones that bundle language changes, new keywords, and idiom updates into a unified release. They allow Rust to:

- **Evolve**: Introduce breaking changes (like new keywords) without affecting existing code
- **Maintain compatibility**: Different editions can coexist in the same project
- **Provide clarity**: Major milestones that showcase language improvements

### Key Principle: Backward Compatibility

**Critical insight**: All Rust compiler versions support all prior editions. Crates using different editions can depend on each other without issues.

## Available Editions

As of 2024, four Rust editions exist:

| Edition | Release Date | Key Features |
|---------|--------------|--------------|
| **Rust 2015** | December 2015 | Original edition (Rust 1.0) |
| **Rust 2018** | December 2018 | Module system improvements, `async`/`await`, `?` operator changes, `dyn Trait` |
| **Rust 2021** | October 2021 | Disjoint closure captures, panic macro consistency, reserved prefixes |
| **Rust 2024** | February 2024 | `gen` keyword, improved error handling, async improvements |

### This Book's Edition

The Rust Programming Language book uses **Rust 2024 edition idioms**.

## Specifying an Edition

### In `Cargo.toml`

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2024"  # Specify edition here
```

**Default**: If `edition` is omitted, Rust defaults to `2015` for backward compatibility.

### Creating New Projects

```bash
# cargo new defaults to the latest stable edition
$ cargo new my_project

# Explicitly specify edition
$ cargo new my_project --edition 2021
```

## How Editions Work

### Compilation

Editions only affect how the compiler **parses code initially**. After parsing:
- All editions compile to the same internal representation
- Binary compatibility is maintained across editions
- No runtime differences between editions

### Example: Cross-Edition Dependencies

```toml
# Your project (Rust 2024)
[package]
edition = "2024"

[dependencies]
old_library = "1.0"  # Uses Rust 2015
new_library = "2.0"  # Uses Rust 2021
```

✅ **This works perfectly!** The compiler seamlessly links crates from different editions.

## Edition Differences

### Rust 2018 Highlights

**Module system improvements**:
```rust
// Rust 2015: Required explicit extern crate
extern crate serde;

// Rust 2018+: Automatically imports dependencies
use serde::Serialize;
```

**`async`/`await` keywords**:
```rust
// New keywords: async, await, try (reserved)
async fn fetch_data() -> Result<String, Error> {
    let response = http_client.get(url).await?;
    Ok(response.text().await?)
}
```

**`dyn Trait` for trait objects**:
```rust
// Rust 2015
let obj: Box<Trait> = ...;

// Rust 2018+: Explicit dynamic dispatch
let obj: Box<dyn Trait> = ...;
```

### Rust 2021 Highlights

**Disjoint closure captures**:
```rust
// Captures only the specific field, not entire struct
let closure = || {
    println!("{}", point.x);  // Captures only point.x
};
```

**Consistent panic macros**:
```rust
// All panic macros now use format strings consistently
panic!("error: {}", value);
```

**Reserved prefixes**:
- `prefix#identifier` reserved for future macro expansion

### Rust 2024 Highlights

**`gen` keyword**: Reserved for future generator syntax

**Improved async error handling**: Better error messages and patterns

**Refined macro hygiene**: Cleaner macro expansion behavior

## Migrating Between Editions

### Automatic Migration with `cargo fix`

Rust provides tooling to automatically upgrade code:

```bash
# 1. Update Cargo.toml to new edition
# Edit Cargo.toml:
[package]
edition = "2024"

# 2. Run cargo fix with --edition flag
$ cargo fix --edition

# 3. Review changes and test
$ cargo test
```

### Migration Process

1. **Update `Cargo.toml`**: Change edition field to target edition
2. **Run `cargo fix --edition`**: Automatically update code idioms
3. **Review changes**: Check what `cargo fix` modified
4. **Run tests**: Ensure behavior is preserved
5. **Update dependencies**: Consider upgrading deps to newer editions

### Migration Warnings

The compiler provides **edition compatibility warnings** to guide migration:

```rust
// Rust 2015 code using 2018 keyword as identifier
fn try() { }  // Warning in 2015, error in 2018+
```

When migrating, these warnings help identify code that needs updating.

## When to Upgrade Editions

### Reasons to Upgrade

✅ **New language features**: Access latest keywords and syntax improvements
✅ **Better error messages**: Newer editions often have improved diagnostics
✅ **Ecosystem alignment**: Libraries increasingly target newer editions
✅ **Long-term maintenance**: Keep codebase modern and maintainable

### When to Stay on Older Editions

⚠️ **Legacy compatibility**: If upgrading breaks specific workflows
⚠️ **Large codebase**: Migration effort might be substantial
⚠️ **Dependencies**: If critical dependencies haven't upgraded

**Reality check**: Most projects should migrate. The process is usually straightforward thanks to `cargo fix`.

## Edition Compatibility Rules

### What Changes Between Editions

- **Keywords**: New reserved words (e.g., `async`, `await`)
- **Syntax**: New or changed syntax (e.g., `dyn Trait`)
- **Lints**: Different default warnings
- **Standard library**: New APIs (though most APIs work across editions)

### What Stays the Same

- **Binary compatibility**: Crates from different editions link seamlessly
- **Feature availability**: Most features available across all editions
- **Tooling**: `cargo`, `rustc`, and tools work with all editions
- **Semantics**: Core language behavior remains consistent

## The Edition Guide

For comprehensive edition details, see **[The Rust Edition Guide](https://doc.rust-lang.org/stable/edition-guide)**:

- Complete feature breakdown by edition
- Migration guides with examples
- Edition-specific idioms and best practices
- Troubleshooting migration issues

## Key Insights

1. **Editions enable evolution**: Rust can introduce breaking changes without fragmenting the ecosystem
2. **Automatic migration**: `cargo fix --edition` handles most upgrade work
3. **Interoperability**: Different editions coexist seamlessly in dependencies
4. **Regular cadence**: ~3-year cycle provides predictable milestones
5. **Opt-in model**: Projects upgrade when ready, no forced migrations

## Edition Selection Strategy

| Project Type | Recommendation |
|--------------|----------------|
| **New projects** | Use latest stable edition (2024) |
| **Maintained projects** | Upgrade to latest edition during major releases |
| **Legacy projects** | Upgrade when introducing new features or dependencies |
| **Libraries** | Consider user base - balance new features vs. broad compatibility |

## Edition vs. Rust Version

**Important distinction**:
- **Edition**: Language syntax and idioms (2015, 2018, 2021, 2024)
- **Rust version**: Compiler release (1.75, 1.76, etc., released every 6 weeks)

You can use **Rust 1.82** (version) to compile **Rust 2015** (edition) code. The compiler supports all prior editions.

---

**Book Reference**: [Appendix E: Editions](https://doc.rust-lang.org/stable/book/appendix-05-editions.html)

**Additional Resource**: [The Rust Edition Guide](https://doc.rust-lang.org/stable/edition-guide)

**Zettelkasten Links**: [[rust-editions-guide]] | [[cargo-ecosystem]] | [[rust-keywords-reference]]
