# 🎯 Rust Ownership: The 2-Minute Version

*When you feel overwhelmed, come back to this simple page!*

## The Two Types of Things in Rust

### 📋 Type 1: Numbers (and friends)
- **What they do:** Make COPIES
- **Real life:** Like photocopying a document
- **In code:** You can use them after giving them away

```rust
let number = 42;
stack.push(number);    // Copies the number
println!("{}", number); // ✅ Still works! You kept a copy
```

### 📄 Type 2: Strings (and collections)
- **What they do:** MOVE (give away the original)
- **Real life:** Like handing someone your only copy of a document
- **In code:** You CAN'T use them after giving them away

```rust
let text = String::from("Hello");
stack.push(text);      // Gives away the string
// println!("{}", text); // ❌ ERROR! You don't have it anymore
```

## The Golden Rules

1. **Numbers are photocopied** 📋 → You keep yours, they get a copy
2. **Strings are given away** 📄 → They get yours, you have nothing
3. **You can get things back** with `pop()` 
4. **You can just look** with `peek()` (like reading over someone's shoulder)

## When You See Errors

- **"value borrowed here after move"** = You gave something away and tried to use it
- **"cannot borrow as mutable"** = Two people tried to write at the same time

## That's It!

**Copy vs Move** is 90% of Rust ownership. Once you get this, the rest is just details! 🌟

---

*Remember: It's okay to not understand everything at once. Even experienced programmers found this confusing at first!*

---

## 🔗 Related Resources

**Mission1 Documentation:**
- [[README|missions/Mission1/README]] - Full V-Cycle documentation for Stack implementation
- [[SIMPLE_GUIDE|missions/Mission1/SIMPLE_GUIDE]] - Detailed learning guide with exercises
- [[KEY_TAKEAWAYS|missions/Mission1/KEY_TAKEAWAYS]] - Core lessons from Mission1

**Real-World Applications:**
- [[Brackets_Basic Q and A|advanced_examples/Brackets_Basic/Q and A]] - Deep technical analysis using Stack:
  - Stack invariants in bracket validation
  - Memory safety and aliasing prevention
  - Algorithm complexity proofs (O(n) analysis)
  - UTF-8 handling and error reporting
- [[Brackets_Basic README_EXTENDED|advanced_examples/Brackets_Basic/README_EXTENDED]] - Extended bracket validator:
  - Advanced Stack usage patterns
  - Configurable alphabet with Stack
  - Multiple error collection strategies

**Zettelkasten Knowledge Base:**
- [[Ownership and Borrowing|../zettelkasten/Ownership and Borrowing]] - Deep dive into ownership
- [[Copy vs Clone vs Move|../zettelkasten/Copy vs Clone vs Move]] - Understanding data transfer
- [[Stack Data Structure|../zettelkasten/Stack Data Structure]] - Stack patterns and use cases
- [[rust-concepts-MOC|../zettelkasten/Rust Concepts MOC]] - Navigate all Rust concepts

**Rust Book Integration:**
- [[Chapter 4|../rust_book/Ch4/README]] - Ownership fundamentals
- [[Week 1 Overview|../zettelkasten/Week 1 Overview]] - Collections and ownership basics

*Tags: #mission1 #ownership #quick-reference #learning-guide #beginner*