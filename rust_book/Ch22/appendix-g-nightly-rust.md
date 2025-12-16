# Appendix G: How Rust is Made and "Nightly Rust"

This appendix explains Rust's development model, release process, and how the language evolves while maintaining stability.

## Core Philosophy: Stability Without Stagnation

**Guiding principle**: You should never fear upgrading to a new version of stable Rust.

Each upgrade should be:
- **Painless**: No breaking changes to stable features
- **Beneficial**: New features, fewer bugs, faster compile times
- **Predictable**: Regular release schedule

Rust achieves this through a sophisticated release process that allows experimentation without compromising stability.

## Release Channels

Rust has **three release channels**:

| Channel | Purpose | Audience |
|---------|---------|----------|
| **Nightly** | Bleeding-edge features, daily builds | Early adopters, feature developers |
| **Beta** | Release candidates, 6-week testing | CI/CD testing, regression detection |
| **Stable** | Production-ready, guaranteed stability | Most Rust developers |

## The Train Model

Rust uses a **"train schedule" release model** based on software release trains (used by Cisco IOS and others).

### How It Works

Every **six weeks**, a new stable release is published. Here's the progression:

```
Day 0: Start of cycle
nightly: * - - * - - *
                     |
beta:                *
stable:              [Previous release]

Day 1-41: Development continues
nightly: * - - * - - * - - * - - *
                     |
beta:                * (testing)

Day 42: New stable release
nightly: * - - * - - * - - * - - * - * - *
                     |                   |
beta:                * - - - - - - - - * *
                                       |
stable:                                *
```

### Detailed Timeline

1. **Nightly development**: All new features land on the `main` branch
2. **Beta branching**: Every 6 weeks, `main` becomes the new `beta`
3. **Beta testing**: Community tests beta for regressions
4. **Backporting fixes**: Critical bug fixes backported to beta
5. **Stable release**: After 6 weeks in beta, becomes stable
6. **Cycle repeats**: New beta branches from nightly again

### Example: Rust 1.75 Release Timeline

```
Week 0:    Rust 1.74 stable released
           Rust 1.75 enters beta
Week 1-5:  Beta testing, bug fixes
Week 6:    Rust 1.75 stable released
           Rust 1.76 enters beta
```

## Benefits of the Train Model

### Predictable Releases
- **Fixed schedule**: New stable release every 6 weeks
- **No feature pressure**: Missed features catch next train
- **Planning**: Teams can plan upgrades around known dates

### Early Testing
- **Beta feedback**: 6 weeks to catch regressions before stable
- **CI integration**: Projects test against beta in automated pipelines
- **Community validation**: Real-world testing before stable release

### Rapid Iteration
- **Smaller changes**: Incremental improvements rather than large batches
- **Faster feedback**: Features reach users quickly
- **Reduced risk**: Smaller releases = easier to debug issues

## Release Support

### Maintenance Policy

- **Current stable**: Fully supported
- **Previous stable**: Reaches end-of-life (EOL) when new stable releases
- **Support window**: Each version supported for 6 weeks

This means:
- Rust 1.75 released → Rust 1.74 reaches EOL
- No long-term support (LTS) versions
- Always use latest stable

## Unstable Features

### Feature Flags

New features start as **unstable** and require:
1. **Nightly Rust**: Only available on nightly channel
2. **Feature gates**: Explicit opt-in via attributes

```rust
// Enable unstable feature (nightly only)
#![feature(async_closure)]

fn main() {
    let closure = async || {
        // Async closure (unstable as of writing)
    };
}
```

### Stabilization Process

1. **Implementation**: Feature implemented behind feature flag
2. **Nightly experimentation**: Users try feature, provide feedback
3. **Refinement**: Based on feedback, feature is improved
4. **RFC approval**: Team decides to stabilize
5. **Feature gate removed**: Feature available on stable

This process ensures features are battle-tested before stabilization.

### Why Unstable Features?

- **Experimentation**: Try new ideas without commitment
- **Feedback**: Real-world usage informs design
- **Safety net**: Can change/remove if flawed
- **Stability guarantee**: Only stable features guaranteed forever

## Using Different Channels

### Installing Toolchains

```bash
# Install nightly
$ rustup toolchain install nightly

# Install beta
$ rustup toolchain install beta

# Install specific version
$ rustup toolchain install 1.75.0
```

### Listing Installed Toolchains

```bash
$ rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

### Setting Default Toolchain

```bash
# Set stable as default
$ rustup default stable

# Use nightly globally
$ rustup default nightly
```

### Per-Project Override

```bash
# Use nightly for specific project
$ cd ~/projects/experimental-project
$ rustup override set nightly

# Now rustc and cargo use nightly in this directory
$ cargo --version
cargo 1.xx.0-nightly (abc123 2024-12-01)
```

### Toolchain Selection Hierarchy

1. **`RUSTUP_TOOLCHAIN` environment variable**
2. **Directory override** (via `rustup override set`)
3. **`rust-toolchain.toml` file** in project root
4. **Global default** (via `rustup default`)

### Using `rust-toolchain.toml`

```toml
# rust-toolchain.toml
[toolchain]
channel = "nightly-2024-12-01"
components = ["rustfmt", "clippy"]
```

This ensures all contributors use the same toolchain.

## The RFC Process

### Request For Comments (RFC)

Rust's development follows a formal RFC process for major changes.

### RFC Workflow

1. **Proposal**: Anyone can write an RFC (proposal document)
2. **Discussion**: Community and teams discuss on GitHub
3. **Feedback**: Teams provide technical review
4. **Consensus**: Team decides to accept or reject
5. **Implementation**: If accepted, feature is implemented
6. **Stabilization**: After nightly testing, feature stabilizes

### What Requires an RFC?

- **Language features**: New syntax, keywords
- **Major compiler changes**: Significant architectural changes
- **Standard library additions**: New APIs
- **Policy changes**: Process or governance updates

### Small Changes

Not everything needs an RFC:
- Bug fixes
- Documentation improvements
- Minor API additions

## Rust Teams

Rust development is organized into **specialized teams**:

- **Language team**: Language design and features
- **Compiler team**: `rustc` implementation
- **Library team**: Standard library APIs
- **Cargo team**: Package manager
- **Infrastructure team**: CI/CD, releases, hosting
- **Documentation team**: Books, guides, API docs
- **Community team**: Events, communication
- **Moderation team**: Code of conduct

See [rust-lang.org/governance](https://www.rust-lang.org/governance) for complete team list.

## Key Insights

1. **Six-week release cycle**: Predictable, frequent stable releases
2. **Three-channel model**: Nightly (experimental), Beta (testing), Stable (production)
3. **Stability guarantee**: Stable features never break
4. **Unstable experimentation**: Feature flags allow testing without commitment
5. **Community-driven**: RFC process ensures transparent decision-making
6. **Train model**: Continuous delivery without feature pressure

## When to Use Each Channel

| Channel | Use Case |
|---------|----------|
| **Stable** | Production code, libraries, most development |
| **Beta** | CI testing, pre-release validation |
| **Nightly** | Experimenting with new features, contributing to Rust |

## Nightly Use Cases

### Valid Reasons to Use Nightly

✅ **Trying new features**: Experimenting with unstable APIs
✅ **Contributing to Rust**: Testing and providing feedback
✅ **Tool development**: Building dev tools (like rustfmt internals)
✅ **Research**: Academic or experimental projects

### When to Avoid Nightly

❌ **Production applications**: Nightly has no stability guarantees
❌ **Published libraries**: Users expect stable compatibility
❌ **Team projects**: Unless all members understand trade-offs

## Upgrading Rust

### Stable Upgrades

```bash
# Update to latest stable
$ rustup update stable

# Check current version
$ rustc --version
rustc 1.75.0 (abc123 2024-01-01)
```

### No Breaking Changes

Stable upgrades should **never** break existing code. If they do, it's considered a critical bug.

### Minor Breakage Exceptions

Very rarely, minor breakage is allowed for:
- Security fixes
- Soundness bugs (type system holes)
- Compiler bugs causing wrong code generation

These are documented in release notes.

## Resources

- **[Rust Blog](https://blog.rust-lang.org/)**: Release announcements and development updates
- **[Rust RFC Repository](https://github.com/rust-lang/rfcs)**: Browse and propose RFCs
- **[This Week in Rust](https://this-week-in-rust.org/)**: Weekly newsletter
- **[Rust Forge](https://forge.rust-lang.org/)**: Internal Rust development documentation

---

**Book Reference**: [Appendix G: How Rust is Made and "Nightly Rust"](https://doc.rust-lang.org/stable/book/appendix-07-nightly-rust.html)

**Zettelkasten Links**: [[rust-development-process]] | [[rust-editions-guide]] | [[rust-community]]
