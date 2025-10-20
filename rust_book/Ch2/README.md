# Chapter 2: Programming a Guessing Game

This chapter provides a hands-on introduction to Rust programming through building a complete guessing game. You'll learn fundamental concepts by doing, not just reading.

## 🎯 Learning Objectives

By the end of this chapter, you will:
- Write a complete Rust program from scratch
- Handle user input and output
- Use external crates (dependencies)
- Implement control flow with loops and conditionals
- Handle errors gracefully
- Generate random numbers

## 📚 Chapter Concepts

### Core Language Features
- **Variables and Mutability**: `let` and `let mut`
- **String Handling**: `String` vs `&str`, user input processing
- **Pattern Matching**: Using `match` expressions
- **Error Handling**: `Result` type and error propagation
- **Loops**: Infinite loops with `loop` and `break`
- **Comparisons**: Ordering and conditional logic

### Rust Ecosystem
- **Crates.io**: Using external dependencies
- **Cargo.toml**: Managing dependencies
- **Documentation**: Reading crate documentation
- **Prelude**: Understanding what's available by default

## 🛠️ Hands-On Project

### The Guessing Game
**Location**: `guessing_game/`

**Features Implemented:**
- Generate random secret number (1-100)
- Accept user input for guesses
- Compare guess with secret number
- Provide feedback (too high, too low, correct)
- Loop until correct guess is made
- Handle invalid input gracefully

**Key Code Patterns:**
```rust
// User input handling
let mut guess = String::new();
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

// String to number conversion with error handling
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

// Pattern matching for comparison
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```

## 🔍 Key Concepts Deep Dive

### Variables and Mutability
- **Immutable by default**: `let x = 5;`
- **Explicit mutability**: `let mut guess = String::new();`
- **Shadowing**: Reusing variable names with different types

### Error Handling Philosophy
- **Explicit error handling**: No hidden exceptions
- **Result type**: `Ok(value)` vs `Err(error)`
- **Panic vs graceful handling**: When to use `expect()` vs `match`

### External Dependencies
- **Adding crates**: `cargo add rand` or manual `Cargo.toml` editing
- **Semantic versioning**: Understanding version specifiers
- **Documentation**: Using `cargo doc --open`

## 💡 Best Practices Introduced

1. **Error Handling**: Always handle `Result` types explicitly
2. **Input Validation**: Sanitize and validate user input
3. **Dependency Management**: Use specific version ranges
4. **Code Organization**: Keep `main()` focused and readable
5. **Documentation**: Read crate docs before using

## 🚀 Real-World Applications

This project introduces patterns used in:
- **CLI Applications**: User input and feedback loops
- **Game Development**: State management and user interaction
- **Web Applications**: Input validation and error handling
- **System Programming**: Resource management and error recovery

## 🔗 Mission Integration

Concepts from this chapter apply to:
- **All Missions**: Error handling patterns established here
- **[[../../missions/Mission1/README.md|Mission 1-2]]**: User input in demonstrations
- **[[../../missions/Mission5/README.md|Mission 5]]**: Input validation for HashMap operations
- **[[../../missions/Mission6/README.md|Mission 6+]]**: Interactive grid applications

## 🔗 Cross-References

- **[[../../zettelkasten/rust_book/rust-book-ch1.md|Chapter 1]]**: Getting Started (Previous)
- **[[../../zettelkasten/rust_book/rust-book-ch3.md|Chapter 3]]**: Common Programming Concepts (Next)
- **[[../../zettelkasten/rust_book/rust-book-ch9.md|Chapter 9]]**: Advanced Error Handling
- **[[../../zettelkasten/Error Handling Deep Dive.md|Error Handling Deep Dive]]**: Comprehensive error patterns
- **[[../../daily_study/rust_learning_week5_notes/Day30.md|Day 30]]**: Error propagation in practice

## 🧪 Extension Exercises

Try enhancing the guessing game:
1. **Difficulty Levels**: Different number ranges
2. **Attempt Counting**: Track and display guess count
3. **High Scores**: Persist best scores to file
4. **Input Validation**: Handle edge cases more robustly
5. **Game Variants**: Different game modes

## ✅ Chapter Completion Checklist

- [ ] Guessing game runs successfully
- [ ] Understand `let` vs `let mut`
- [ ] Can add external crates with Cargo
- [ ] Comfortable with `match` expressions
- [ ] Understand basic error handling with `Result`
- [ ] Can handle user input and output

## 🎓 Next Steps

Once you complete this chapter:
1. Move to **Chapter 3** for fundamental programming concepts
2. Apply input handling in **Mission demonstrations**
3. Practice error handling patterns in mission implementations

---

*This chapter bridges the gap from installation to actual programming, providing essential hands-on experience with Rust's unique features.*