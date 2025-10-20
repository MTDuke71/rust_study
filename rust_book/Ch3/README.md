# Chapter 3: Common Programming Concepts

This chapter covers fundamental programming concepts in Rust: variables, data types, functions, and control flow. These are the building blocks for all Rust programs.

## 🎯 Learning Objectives

By the end of this chapter, you will:
- Understand Rust's variable system and mutability rules
- Know all of Rust's primitive data types and when to use them
- Write and call functions with parameters and return values
- Use comments effectively for documentation
- Implement control flow with conditionals and loops
- Apply these concepts in practical programming scenarios

## 📚 Chapter Sections

### 3.1 Variables and Mutability
- **Immutability by default**: Rust's safety-first approach
- **Explicit mutability**: When and why to use `mut`
- **Shadowing**: Reusing names with type changes
- **Constants**: Compile-time known values

### 3.2 Data Types
- **Scalar types**: Integers, floats, booleans, characters
- **Compound types**: Tuples and arrays
- **Type inference**: When Rust can figure it out
- **Type annotations**: When you need to be explicit

### 3.3 Functions
- **Function definition**: `fn` keyword and naming
- **Parameters**: Typed function inputs
- **Return values**: Expressions vs statements
- **Function scope**: Local variables and ownership

### 3.4 Comments
- **Line comments**: `//` for single-line documentation
- **Documentation comments**: `///` for API docs
- **Block comments**: `/* */` for multi-line (rarely used)

### 3.5 Control Flow
- **Conditionals**: `if`, `else if`, `else` expressions
- **Loops**: `loop`, `while`, `for` with different use cases
- **Loop control**: `break` and `continue`

## 🛠️ Hands-On Practice

### Project: Variables and Mutability
**Location**: `variables/`
- Exploring immutability and mutability
- Shadowing examples and use cases
- Constant definitions and usage

### Project: Data Types
**Location**: `data_types/`
- Working with all scalar types
- Tuple and array manipulation
- Type inference vs explicit annotations

### Project: Functions
**Location**: `functions/`
- Function definition and calling
- Parameter passing and return values
- Expression vs statement examples

### Project: Control Flow
**Location**: `control_flow/`
- Conditional logic with `if`
- Different loop types and their use cases
- Loop control flow examples

## 🔍 Key Concepts Deep Dive

### Variables and Mutability
```rust
// Immutable by default
let x = 5;
// x = 6; // This would cause a compile error

// Explicit mutability
let mut y = 5;
y = 6; // This is fine

// Shadowing - same name, different type
let spaces = "   ";
let spaces = spaces.len(); // Now it's a number
```

### Data Types Mastery
```rust
// Scalar types
let decimal = 98_222;        // Integer with separator
let hex = 0xff;              // Hexadecimal
let binary = 0b1111_0000;    // Binary
let byte = b'A';             // Byte (u8 only)
let floating = 2.0;          // f64 by default
let boolean = true;          // bool
let character = 'Z';         // char (Unicode)

// Compound types
let tup: (i32, f64, u8) = (500, 6.4, 1);
let array = [1, 2, 3, 4, 5];
let same_values = [3; 5]; // [3, 3, 3, 3, 3]
```

### Function Patterns
```rust
// Function with parameters and return value
fn add_one(x: i32) -> i32 {
    x + 1  // Expression (no semicolon)
}

// Function with multiple parameters
fn print_coordinates(x: i32, y: i32) {
    println!("Coordinates: ({}, {})", x, y);
}
```

### Control Flow Patterns
```rust
// Conditional as expression
let number = if condition { 5 } else { 6 };

// Different loop types
loop {
    // Infinite loop
    break; // Exit condition
}

while condition {
    // Conditional loop
}

for element in array {
    // Iterator loop
}

for number in 1..4 {
    // Range loop: 1, 2, 3
}
```

## 💡 Best Practices

1. **Default to Immutability**: Only use `mut` when necessary
2. **Meaningful Names**: Use descriptive variable and function names
3. **Type Annotations**: Add them when clarity is needed
4. **Function Size**: Keep functions focused and small
5. **Documentation**: Use `///` for public function documentation
6. **Loop Choice**: Use the most appropriate loop type

## 🚀 Real-World Applications

These concepts enable:
- **Data Processing**: Working with different data types efficiently
- **Algorithm Implementation**: Control flow for complex logic
- **API Design**: Well-structured functions with clear interfaces
- **System Programming**: Low-level data manipulation
- **Application Logic**: Business rule implementation

## 🔗 Mission Integration

These fundamentals appear in every mission:
- **[[../../missions/Mission1/README.md|Mission 1 (Stack)]]**: Functions and data types for stack operations
- **[[../../missions/Mission2/README.md|Mission 2 (Queue)]]**: Control flow for circular buffer logic
- **[[../../missions/Mission3/README.md|Mission 3 (BST)]]**: Recursive functions and tree navigation
- **[[../../missions/Mission4/README.md|Mission 4 (LinkedList)]]**: Pointer types and iteration patterns
- **[[../../missions/Mission5/README.md|Mission 5 (HashMap)]]**: Hash calculation and collision handling
- **[[../../missions/Mission6/README.md|Mission 6+ (Grids)]]**: Nested loops and coordinate systems

## 🔗 Cross-References

- **[[../../zettelkasten/rust_book/rust-book-ch2.md|Chapter 2]]**: Programming a Guessing Game (Previous)
- **[[../../zettelkasten/rust_book/rust-book-ch4.md|Chapter 4]]**: Ownership and Borrowing (Next)
- **[[../../zettelkasten/rust_book/rust-book-ch5.md|Chapter 5]]**: Structs - Custom Data Types
- **[[../../zettelkasten/Memory Management.md|Memory Management]]**: Deep dive into Rust's memory model
- **[[../../daily_study/rust_learning_week1_notes/Day02.md|Day 2]]**: Variables and ownership basics

## 🧪 Practice Exercises

1. **Temperature Converter**: Celsius ↔ Fahrenheit conversion
2. **Fibonacci Sequence**: Generate first N Fibonacci numbers
3. **Calculator**: Basic arithmetic with function organization
4. **Number Guessing**: Enhance Chapter 2's game with better structure
5. **Prime Checker**: Determine if a number is prime

## ✅ Chapter Completion Checklist

- [ ] Understand variable mutability and shadowing
- [ ] Know all primitive data types and their uses
- [ ] Can write functions with parameters and return values
- [ ] Comfortable with all loop types and conditionals
- [ ] Can choose appropriate data types for different scenarios
- [ ] Understand expressions vs statements

## 🎓 Next Steps

Once you complete this chapter:
1. Move to **Chapter 4** (Ownership) - Rust's unique memory model
2. Apply these concepts in **Mission 1** implementation
3. Practice with the provided exercises and examples

---

*This chapter provides the essential building blocks that every Rust programmer must master. These concepts form the foundation for all advanced Rust features and mission implementations.*