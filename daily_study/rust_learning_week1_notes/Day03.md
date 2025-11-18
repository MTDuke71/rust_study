# Day 3 · Functions & References

## 🔗 Zettelkasten Links
- **Previous**: [[daily-study/Day02]] - Move and copy semantics
- **Next**: [[daily-study/Day04]] - Lifetime annotations
- **Concept**: [[rust-concepts-MOC]] - Borrowing rules
- **Rust Book**: [[Chapter 4.2 - References and Borrowing]] - Reference rules
- **Week Summary**: [[zettelkasten/daily-study/Day07]] - Foundations review

## Key Points
- Functions declared with `fn`.
- Parameters passed by value unless explicitly a reference.
- Ownership transfer: passing by value moves, unless type is Copy.
- References `&T` (immutable) and `&mut T` (mutable):
  - Multiple immutable allowed simultaneously.
  - Only one mutable allowed, and no immutables at the same time.
- Return values transfer ownership.

## Example
```rust
fn take(s: String) {}
fn borrow(s: &String) {}
fn borrow_mut(s: &mut String) {}
```

## Takeaway
Function signatures communicate ownership/borrowing clearly. This is how lifetimes are controlled.

---

## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as day03_demo.rs

fn main() {
    println!("=== References & Borrowing Demo from Day 3 ===\n");

    // 1. Immutable references (borrowing)
    println!("1. Immutable Borrowing:");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // Borrow, don't move
    println!("  String '{}' has length {}", s1, len);  // s1 still valid

    // 2. Multiple immutable references allowed
    println!("\n2. Multiple Immutable References:");
    let s = String::from("world");
    let r1 = &s;
    let r2 = &s;
    println!("  r1: {}, r2: {} (both OK simultaneously)", r1, r2);

    // 3. Mutable references
    println!("\n3. Mutable Borrowing:");
    let mut s2 = String::from("start");
    println!("  Before: {}", s2);
    append_text(&mut s2);  // Mutable borrow
    println!("  After: {}", s2);

    // 4. Reference scope rules
    println!("\n4. Reference Scoping:");
    let mut s3 = String::from("scopes");
    {
        let r1 = &s3;  // Immutable borrow in inner scope
        println!("  Inner scope: {}", r1);
    }  // r1 goes out of scope
    let r2 = &mut s3;  // Now mutable borrow is OK
    r2.push_str(" work!");
    println!("  After scope: {}", r2);

    // 5. Dangling reference prevention
    println!("\n5. No Dangling References:");
    let valid_ref = no_dangle();
    println!("  Valid string: {}", valid_ref);

    // 6. Function ownership patterns
    println!("\n6. Different Function Patterns:");
    let original = String::from("test");

    // Pattern A: Take ownership
    let owned = takes_ownership_pattern(original);
    // original is no longer valid
    println!("  Owned result: {}", owned);

    // Pattern B: Borrow immutably
    let borrowed_text = String::from("borrow");
    let length = borrow_pattern(&borrowed_text);
    println!("  Text '{}' has length {}", borrowed_text, length);

    // Pattern C: Borrow mutably
    let mut mutable_text = String::from("modify");
    modify_pattern(&mut mutable_text);
    println!("  Modified: {}", mutable_text);

    // 7. Practical example: Counting words
    println!("\n7. Practical Example - Word Counting:");
    let sentence = String::from("The quick brown fox jumps");
    let word_count = count_words(&sentence);
    println!("  '{}' has {} words", sentence, word_count);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope, but doesn't drop the String (not owner)

fn append_text(s: &mut String) {
    s.push_str(" + added text");
}

// ❌ This would create a dangling reference:
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // ❌ s is dropped, reference would be invalid
// }

// ✅ Correct: Return owned value
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // Move ownership out
}

fn takes_ownership_pattern(s: String) -> String {
    format!("{} (processed)", s)
}

fn borrow_pattern(s: &String) -> usize {
    s.len()
}

fn modify_pattern(s: &mut String) {
    s.push_str(" (modified)");
}

fn count_words(text: &String) -> usize {
    text.split_whitespace().count()
}
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day03_demo.rs` and run `rustc day03_demo.rs && ./day03_demo`
3. **In this workspace**:
   ```bash
   .\scripts\run_md.bat daily_study\rust_learning_week1_notes\Day03.md
   ```

### **Key Borrowing Rules Demonstrated:**

- ✅ Many immutable references allowed (`&T`)
- ✅ Only ONE mutable reference at a time (`&mut T`)
- ❌ Cannot have mutable + immutable references simultaneously
- ✅ References must always be valid (no dangling references)

---

*Links: [[daily-study/Day02]] | [[daily-study/Day04]] | [[rust-concepts-MOC]]*
*Tags: #borrowing #references #daily-study #rust-book #chapter4 #foundation*
