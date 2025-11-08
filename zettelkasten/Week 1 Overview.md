# Week 1 Overview - Rust Foundations Mastery

**Learning Period**: Days 1-7
**Theme**: Core Ownership and Memory Safety
**Mission Alignment**: Foundation for all V-Cycle projects

---

## 🎯 Week Objectives

Master the **fundamental pillars** of Rust that make it unique:
1. **Ownership System** - Memory safety without garbage collection
2. **Borrowing Rules** - Safe references and mutable access
3. **Lifetime Annotations** - Compile-time reference validity
4. **Error Handling** - Type-safe Option and Result
5. **Pattern Matching** - Exhaustive control flow

---

## 📚 Daily Progression

### **Day 1: Setup & Toolchain** [[daily-study/Day01]]
- Install Rust toolchain (rustup, cargo, rustc)
- Understand project structure (Cargo.toml, src/)
- Run first "Hello, World!" program
- Learn cargo commands (build, run, test, clippy)

**Key Takeaway**: Rust toolchain is unified and ergonomic

### **Day 2: Ownership & Move Semantics** [[daily-study/Day02]]
- Ownership rules: each value has one owner
- Move semantics for heap types (String)
- Copy semantics for stack types (i32, bool)
- Ownership transfer in functions

**Key Takeaway**: Ownership replaces garbage collection

### **Day 3: References & Borrowing** [[daily-study/Day03]]
- Immutable references (&T) - many allowed
- Mutable references (&mut T) - only one at a time
- Borrowing rules prevent data races
- Function signatures communicate ownership

**Key Takeaway**: Borrowing enables safe sharing without ownership transfer

### **Day 4: Lifetimes** [[daily-study/Day04]]
- Lifetime annotations ('a syntax)
- Lifetime elision rules (compiler inference)
- Structs with references need lifetimes
- Function return references and lifetime constraints

**Key Takeaway**: Lifetimes prevent dangling references at compile time

### **Day 5: Option & Result** [[daily-study/Day05]]
- Option<T> for nullable values (no null pointers!)
- Result<T, E> for recoverable errors
- The ? operator for error propagation
- Pattern matching for exhaustive handling

**Key Takeaway**: Explicit error handling makes programs reliable

### **Day 6: Pattern Matching** [[daily-study/Day06]]
- match expressions for exhaustive matching
- Destructuring tuples, structs, enums
- if let and while let shortcuts
- Guards for conditional patterns

**Key Takeaway**: Pattern matching is both control flow and data extraction

### **Day 7: Week Summary** [[daily-study/Day07]]
- Code cheat sheet for all concepts
- Mental models for ownership and borrowing
- Common patterns for competitive programming
- Mastery check: can you explain why code compiles?

**Key Takeaway**: All concepts build on ownership for memory safety

---

## 🔗 Cross-Track Integration

### **Mission Track Connections**
- [[mission-1]] - Stack implementation uses ownership
- [[mission-2]] - Queue uses borrowing patterns
- All missions build on Week 1 foundations

### **Rust Book Alignment**
- **Chapter 1**: Getting Started (Day 1)
- **Chapter 4**: Understanding Ownership (Days 2-4)
- **Chapter 6**: Enums and Pattern Matching (Days 5-6)
- **Chapter 9**: Error Handling (Day 5)
- **Chapter 10**: Lifetimes (Day 4)

### **Zettelkasten Network**
- [[Rust Concepts MOC]] - All concepts in one map
- [[Collections MOC]] - Prepares for Week 2
- [[zettel-index]] - Master index

---

## 📊 Learning Outcomes

By the end of Week 1, you should be able to:

### **Conceptual Understanding**
- ✅ Explain the three ownership rules
- ✅ Describe when moves vs copies occur
- ✅ Apply borrowing rules to prevent data races
- ✅ Annotate lifetimes when required
- ✅ Choose between Option and Result appropriately
- ✅ Write exhaustive pattern matches

### **Practical Skills**
- ✅ Write functions with correct ownership semantics
- ✅ Use references without creating dangling pointers
- ✅ Handle errors with ? operator
- ✅ Match on enums and destructure data
- ✅ Debug common borrowing errors
- ✅ Read and understand Rust compiler errors

### **Code Patterns**
```rust
// Ownership transfer
fn consume(s: String) { }

// Immutable borrowing
fn read(s: &String) -> usize { s.len() }

// Mutable borrowing
fn modify(s: &mut String) { s.push_str("!"); }

// Error handling
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { Err("div by zero".into()) }
    else { Ok(a / b) }
}

// Pattern matching
match result {
    Ok(val) => println!("Success: {}", val),
    Err(e) => eprintln!("Error: {}", e),
}
```

---

## 🎓 Mastery Checkpoints

### **Self-Assessment Questions**

1. **Ownership**: Why does this code fail?
   ```rust
   let s1 = String::from("hello");
   let s2 = s1;
   println!("{}", s1); // ❌ Error
   ```

2. **Borrowing**: Why is this error caught at compile time?
   ```rust
   let mut s = String::from("hello");
   let r1 = &s;
   let r2 = &mut s; // ❌ Error
   println!("{}", r1);
   ```

3. **Lifetimes**: What lifetime annotation is needed?
   ```rust
   fn longest(x: &str, y: &str) -> &str { // ❌ Missing lifetime
       if x.len() > y.len() { x } else { y }
   }
   ```

4. **Error Handling**: How does ? operator work?
   ```rust
   fn process() -> Result<i32, String> {
       let val = parse_number()?; // What happens on error?
       Ok(val * 2)
   }
   ```

**If you can answer all four**, you've mastered Week 1! 🎯

---

## 🚀 Transition to Week 2

**Week 1** gave you the **ownership discipline**.
**Week 2** shows you how Rust's **collections** leverage these rules.

### **What's Next:**
- [[daily-study/Day08]] - Dynamic arrays with ownership
- [[daily-study/Day09]] - UTF-8 text and String vs &str
- [[daily-study/Day10]] - Key-value storage
- [[Week 2 Overview]] - Collections mastery

### **How Week 1 Prepares You:**
```rust
// Week 1 knowledge in action:
let mut vec = Vec::new();           // Ownership
vec.push(String::from("hello"));    // Move
let item = &vec[0];                 // Borrow
// vec.push("world");               // ❌ Can't modify while borrowed
println!("{}", item);               // Borrow ends here
vec.push("world".to_string());      // ✅ Now OK
```

---

## 📈 Progress Tracking

### **Completed Materials**
- [x] Day 1: Setup & Toolchain
- [x] Day 2: Ownership & Moves
- [x] Day 3: References & Borrowing
- [x] Day 4: Lifetimes
- [x] Day 5: Option & Result
- [x] Day 6: Pattern Matching
- [x] Day 7: Week Summary

### **Skills Acquired**
- [x] Ownership reasoning
- [x] Borrowing rules
- [x] Lifetime annotations
- [x] Error handling
- [x] Pattern matching
- [x] Compiler error interpretation

### **Mission Readiness**
- [x] Can implement Stack (Mission1)
- [x] Can implement Queue (Mission2)
- [x] Ready for HashMap (Mission5)

---

## 🔍 Common Week 1 Challenges

### **Challenge 1: "Cannot move out of borrowed content"**
```rust
// ❌ Problem
let v = vec![String::from("hello")];
let s = &v[0];
let owned = v[0]; // ❌ Can't move while borrowed

// ✅ Solution
let owned = v[0].clone(); // Clone instead of move
```

### **Challenge 2: "Cannot borrow as mutable more than once"**
```rust
// ❌ Problem
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s; // ❌ Two mutable borrows

// ✅ Solution
let r1 = &mut s;
r1.push_str(" world");
// r1 scope ends
let r2 = &mut s; // ✅ Now OK
```

### **Challenge 3: "Missing lifetime specifier"**
```rust
// ❌ Problem
fn first_word(s: &str) -> &str { // Inferred automatically
    s.split_whitespace().next().unwrap()
}

fn choose(x: &str, y: &str) -> &str { // ❌ Ambiguous
    if x.len() > y.len() { x } else { y }
}

// ✅ Solution
fn choose<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

## 💡 Week 1 Study Tips

1. **Run All Examples**: Don't just read - type and execute every code snippet
2. **Break Things**: Intentionally create errors to understand compiler messages
3. **Draw Diagrams**: Visualize ownership transfers and borrowing scopes
4. **Use Playground**: [play.rust-lang.org](https://play.rust-lang.org/) for quick experiments
5. **Read Errors Carefully**: Rust compiler errors are extremely helpful
6. **Ask "Who Owns This?"**: For every value, know its current owner

---

## 📚 Further Resources

### **Official Documentation**
- [The Rust Book Chapters 1-10](https://doc.rust-lang.org/book/)
- [Rust By Example - Ownership](https://doc.rust-lang.org/rust-by-example/scope.html)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)

### **Zettelkasten Notes**
- [[Rust Concepts MOC]] - Concept map
- [[Collections MOC]] - Week 2 preview
- [[zettel-index]] - Full index

### **Community Resources**
- [r/rust](https://reddit.com/r/rust) - Reddit community
- [Rust Users Forum](https://users.rust-lang.org/)
- [Rust Discord](https://discord.gg/rust-lang)

---

**Week 1 Complete!** You now have the foundation for **safe, fast, concurrent systems programming**. The ownership system that seemed strange at first is now your superpower for writing correct code. 🦀

*Last Updated: Week 1 completion*
*Next: [[Week 2 Overview]] - Collections Mastery*

---

*Links: [[daily-study/Day01]] | [[daily-study/Day07]] | [[Week 2 Overview]] | [[Rust Concepts MOC]] | [[zettel-index]]*
*Tags: #week-overview #week1 #foundations #ownership #borrowing #lifetimes #daily-study #learning-path*
