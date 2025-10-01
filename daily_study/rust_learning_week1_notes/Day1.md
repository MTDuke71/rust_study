# Day 1 · Setup & Toolchain

## 🔗 Zettelkasten Links
- **Overview**: [[Rust Concepts MOC]] - Foundation concepts
- **Next**: [[Day 2 - Ownership Basics]] - Core ownership concepts
- **Week Summary**: [[Day 7 - Week 1 Summary]] - Review all foundations
- **Rust Book**: [[Chapter 1 - Getting Started]] - Installation and Hello World

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

---

## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as day1_demo.rs

fn main() {
    println!("=== Rust Toolchain Demo from Day 1 ===\n");

    // 1. Basic Hello World
    println!("1. Hello, Rust!");

    // 2. Variables and Types
    println!("\n2. Variables and Basic Types:");
    let x: i32 = 42;
    let name: &str = "Rustacean";
    let is_learning = true;
    println!("  Number: {}, Name: {}, Learning: {}", x, name, is_learning);

    // 3. Functions
    println!("\n3. Function Calls:");
    greet("World");
    let sum = add(10, 20);
    println!("  10 + 20 = {}", sum);

    // 4. Basic control flow
    println!("\n4. Control Flow:");
    for i in 1..=5 {
        if i % 2 == 0 {
            println!("  {} is even", i);
        } else {
            println!("  {} is odd", i);
        }
    }

    // 5. Collections preview
    println!("\n5. Basic Collections:");
    let numbers = vec![1, 2, 3, 4, 5];
    println!("  Vector: {:?}", numbers);
    println!("  Length: {}, First: {}", numbers.len(), numbers[0]);
}

fn greet(name: &str) {
    println!("  Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day1_demo.rs` and run `rustc day1_demo.rs && ./day1_demo`
3. **In this workspace**:
   ```bash
   # PowerShell
   .\scripts\run_md.bat daily_study\rust_learning_week1_notes\Day1.md

   # Or create a new project
   cargo new day1_demo
   # Replace src/main.rs with code above
   cargo run
   ```

---

*Links: [[Day 2 - Ownership Basics]] | [[Rust Concepts MOC]]*
*Tags: #rust-book #chapter1 #toolchain #setup #daily-study #foundation*
