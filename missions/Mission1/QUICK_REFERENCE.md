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