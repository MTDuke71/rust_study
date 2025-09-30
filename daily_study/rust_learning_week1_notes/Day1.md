# Day 1 · Setup & Toolchain

## Key Points
- Install Rust with `rustup` (includes `cargo`, `rustc`, `rustup`).
- Cargo basics:
  - `cargo new project`
  - `cargo run`
  - `cargo test`
  - `cargo build --release`
- Rust Analyzer (IDE plugin) is essential for autocomplete, type hints.
- Hello world program with `println!`.
- Project layout: `src/main.rs`, `src/lib.rs`, `Cargo.toml`.
- Tests: inline with `#[cfg(test)]`, or integration in `tests/` dir.
- Clippy for linting: `cargo clippy -- -D warnings`.

## Takeaway
Rust toolchain is unified, ergonomic. Cargo handles build, deps, tests, docs.
