# Day 2 · Variables, Mutability, Moves vs Copies

## 🔗 Zettelkasten Links
- **Previous**: [[zettelkasten/daily-study/Day01]] - Toolchain basics
- **Next**: [[daily-study/Day03]] - Reference rules
- **Concept**: [[rust-concepts-MOC]] - Ownership system
- **Rust Book**: [[Chapter 4.1 - Ownership]] - What is Ownership?
- **Week Summary**: [[zettelkasten/daily-study/Day07]] - Foundations review

## Key Points
- Variables immutable by default (`let`), mutable with `mut`.
- Shadowing: rebind name to new value.
- Move semantics: ownership transferred on assignment or function call (`String`).
- Copy semantics: simple scalar types implement `Copy` (`i32`, `bool`, etc.).
- Ownership rules:
  1. Each value has one owner.
  2. Owner drop = value drop.
  3. Moves transfer ownership; copies duplicate.
- Borrow checker enforces rules at compile time.

## Exercises
- Move a `String` into a function → compile error on later use.
- Pass `i32` into function → works (Copy).

## Takeaway
Ownership + moves/copies replace GC. Foundation for all Rust reasoning.

---

## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as day2_demo.rs

fn main() {
    println!("=== Ownership & Moves Demo from Day 2 ===\n");

    // 1. Move semantics (Heap types)
    println!("1. Move Semantics:");
    let s1 = String::from("hello");
    println!("  s1 before move: {}", s1);
    let s2 = s1;  // s1 is moved to s2
    println!("  s2 after move: {}", s2);
    // println!("{}", s1);  // ❌ Would not compile - s1 is invalid

    // 2. Copy semantics (Stack types)
    println!("\n2. Copy Semantics:");
    let x = 42;
    let y = x;  // x is copied to y
    println!("  x: {}, y: {} (both still valid)", x, y);

    // 3. Function ownership transfer
    println!("\n3. Ownership Transfer to Functions:");
    let text = String::from("function call");
    takes_ownership(text);
    // println!("{}", text);  // ❌ text is no longer valid

    let number = 100;
    makes_copy(number);
    println!("  number still valid: {}", number);  // ✅ Still usable

    // 4. Returning ownership
    println!("\n4. Returning Ownership:");
    let s3 = gives_ownership();
    println!("  Received: {}", s3);

    let s4 = String::from("original");
    let s5 = takes_and_gives_back(s4);
    println!("  Got back: {}", s5);

    // 5. Cloning (explicit deep copy)
    println!("\n5. Explicit Cloning:");
    let original = String::from("clone me");
    let cloned = original.clone();  // Expensive operation
    println!("  Original: {}, Clone: {}", original, cloned);

    // 6. Mutability
    println!("\n6. Mutability:");
    let mut mutable_string = String::from("start");
    println!("  Before: {}", mutable_string);
    mutable_string.push_str(" + end");
    println!("  After: {}", mutable_string);
}

fn takes_ownership(s: String) {
    println!("  takes_ownership received: {}", s);
}  // s dropped here

fn makes_copy(x: i32) {
    println!("  makes_copy received: {}", x);
}  // x dropped, but it's just a copy

fn gives_ownership() -> String {
    String::from("returned value")
}

fn takes_and_gives_back(s: String) -> String {
    s  // Returns ownership back
}
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day2_demo.rs` and run `rustc day2_demo.rs && ./day2_demo`
3. **In this workspace**:
   ```bash
   .\scripts\run_md.bat daily_study\rust_learning_week1_notes\Day2.md
   ```

---

*Links: [[zettelkasten/daily-study/Day01]] | [[daily-study/Day03]] | [[rust-concepts-MOC]]*
*Tags: #ownership #moves #copy #daily-study #rust-book #chapter4 #foundation*
