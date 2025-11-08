# Rust Book Chapters 1-4: Comprehensive Review

> **Foundational Knowledge**: Mastering Rust's core principles from setup through ownership

## 📚 Overview

This review covers the foundational four chapters that establish Rust's unique approach to systems programming. These chapters introduce the language's core innovation - the ownership system - and provide the essential knowledge needed for all subsequent Rust programming. Understanding these concepts deeply is critical because they underpin every feature and pattern in the language.

**Chapter Coverage:**
- **Chapter 1**: Getting Started - Toolchain, Hello World, and Cargo fundamentals
- **Chapter 2**: Programming a Guessing Game - Hands-on introduction to core concepts
- **Chapter 3**: Common Programming Concepts - Variables, types, functions, and control flow
- **Chapter 4**: Understanding Ownership - Rust's memory safety foundation

## 🚀 Chapter 1: Getting Started - The Rust Toolchain

### **Core Concepts**

Chapter 1 establishes the development environment and introduces Rust's philosophy through practical setup. Unlike languages that require complex IDE configurations or multiple tools, Rust provides a unified toolchain that handles compilation, dependency management, project scaffolding, and testing through a single interface.

### **Installation and Rustup**

**Rustup** is Rust's toolchain installer and version manager. It manages multiple Rust versions, allowing per-project toolchain specification. This flexibility enables working with stable releases for production code while experimenting with nightly features for research. The concept of toolchain management is essential because Rust evolves rapidly with a six-week release cycle, and different projects may target different compiler versions.

The installation process installs three critical components: the Rust compiler (`rustc`), the package manager and build tool (`cargo`), and the standard library. This minimal but complete toolchain provides everything needed for professional Rust development without additional dependencies.

### **Cargo: The Build System and Package Manager**

**Cargo** is Rust's integrated project management tool, combining the roles of Maven, npm, and Make from other ecosystems. It provides a conventional project structure, dependency resolution, build orchestration, test execution, and documentation generation through a single tool.

**Project structure conventions**: Cargo establishes standard layouts where `src/main.rs` is the default binary crate entry point and `src/lib.rs` is the library crate root. This convention eliminates bikeshedding about project organization and makes any Rust project immediately familiar to other Rust developers.

**Cargo.toml manifest**: This configuration file uses TOML (Tom's Obvious, Minimal Language) format to declare project metadata, dependencies, build configurations, and feature flags. The manifest is the single source of truth for project configuration, eliminating the need for multiple configuration files scattered across the project.

**Build profiles**: Cargo provides built-in profiles for different scenarios. The `dev` profile optimizes for compilation speed and includes debug symbols, making development iteration fast. The `release` profile enables aggressive optimizations at the cost of longer compilation times, producing performant production binaries. These profiles can be customized in Cargo.toml but provide sensible defaults for most use cases.

### **Hello World and Compilation**

The traditional "Hello, World!" program in Rust demonstrates several important characteristics:

- **Explicit `main` function**: Like C, Rust requires an explicit entry point, making program flow obvious
- **Macros with `!` syntax**: `println!` is a macro, not a function, indicated by the exclamation mark. Macros can accept variable numbers of arguments and perform compile-time code generation
- **Type inference**: The string literal's type is inferred without explicit annotation
- **Zero-cost abstractions**: The formatted printing has no runtime overhead compared to lower-level alternatives

**Compilation model**: Rust is ahead-of-time compiled to native machine code, unlike interpreted languages or JIT-compiled languages. This provides predictable performance without warmup time and enables deployment without runtime dependencies. The compiler performs extensive static analysis, catching many bugs at compile time that would be runtime errors in other languages.

## 🎮 Chapter 2: Programming a Guessing Game - Learning by Doing

### **Core Concepts**

Chapter 2 provides a hands-on introduction to Rust through building a complete program. Rather than explaining concepts in isolation, it demonstrates how variables, input/output, control flow, error handling, and dependencies work together in a practical application. This approach reveals Rust's philosophy of explicit error handling and type safety in a realistic context.

### **Variables and Mutability**

**Immutability by default**: Variables are immutable unless explicitly marked `mut`. This conservative default prevents accidental mutation bugs and enables compiler optimizations. The compiler can reason more aggressively about immutable data, and concurrent code becomes safer because immutable data can be freely shared between threads.

**Shadowing vs mutation**: Shadowing allows reusing a variable name with a potentially different type, while mutation changes the value in-place. Shadowing creates a new variable that shadows the previous binding, enabling type transformations like parsing a string to a number while reusing the same conceptual name. This pattern is particularly useful in parsing pipelines where data undergoes progressive transformation.

### **Type System Introduction**

**Static typing with inference**: Rust requires knowing all types at compile time but infers most types from usage. This provides static type safety without the verbosity of explicitly annotating every variable. The compiler uses a sophisticated type inference algorithm that propagates type information bidirectionally through expressions.

**Type annotations when needed**: Inference can't always determine types, particularly when multiple valid types exist. The `parse()` method can return any type implementing `FromStr`, so an explicit type annotation tells the compiler which type to produce. This explicitness at ambiguous points improves code clarity and prevents subtle bugs from incorrect type choices.

**Integer types and sizing**: Rust provides sized integer types (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`) and unsigned variants (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`). The `isize` and `usize` types match pointer width, being 32-bit or 64-bit depending on the target architecture. This explicit sizing prevents integer overflow surprises and enables precise memory layout control.

### **Input/Output and Error Handling**

**Standard library modules**: The `std::io` module provides input/output functionality. The module system organizes the standard library into logical groups, and the `use` statement brings items into scope. This namespace management prevents name collisions and makes dependencies explicit.

**Result type for fallible operations**: Operations that can fail return `Result<T, E>`, an enum with `Ok(T)` for success and `Err(E)` for errors. This makes error handling explicit in function signatures - you can't ignore that an operation might fail. The type system forces acknowledgment of error possibilities.

**The `expect` method**: While production code should handle errors gracefully, `expect()` provides a way to acknowledge potential failure while deferring proper error handling during prototyping. It panics on error with a custom message, making it clear where crashes occur. This trade-off between safety and development velocity is intentional.

### **Control Flow with Loops**

**Loop constructs**: Rust provides three loop forms. `loop` creates infinite loops that exit via `break`, `while` runs while a condition holds, and `for` iterates over collections or ranges. Each has specific use cases and semantic meaning that communicates programmer intent.

**Breaking from loops**: The `break` statement exits the innermost loop. In game or server contexts, infinite loops with conditional breaks are common patterns for main loops that run until shutdown conditions occur.

**Match expressions for control flow**: The guessing game uses `match` to handle the comparison result. This pattern matching is exhaustive - the compiler ensures all possible outcomes are handled. This eliminates entire classes of bugs where code paths are forgotten.

### **External Dependencies with Cargo**

**Crates.io ecosystem**: Rust's package registry hosts thousands of libraries. Adding dependencies is as simple as listing them in Cargo.toml with semantic versioning specifiers. Cargo handles downloading, versioning, and building dependencies automatically.

**Semantic versioning**: Version specifiers like `0.8.5` use semantic versioning. Cargo interprets `0.8.5` as "compatible with 0.8.5", allowing patch and minor updates but not major breaking changes. This balances stability with receiving bug fixes.

**Cargo.lock for reproducibility**: Cargo generates a lockfile recording exact dependency versions used. This ensures builds are reproducible across machines and time - you get the same dependencies whether building today or in six months. The lockfile should be committed for applications but not for libraries.

## 🔤 Chapter 3: Common Programming Concepts - Language Fundamentals

### **Core Concepts**

Chapter 3 covers fundamental programming concepts that exist in most languages but explains Rust's specific approach to each. These concepts form the vocabulary needed to express algorithms and data transformations. While the concepts are familiar, Rust's particular implementations often emphasize safety and explicitness over brevity.

### **Variables and Mutability**

**Constants vs immutable variables**: Constants (`const`) must be compile-time computable and have a fixed value for the program's entire runtime. Immutable variables are bound at runtime and can hold computed values. Constants can be defined in any scope including global, while variables exist only in their declared scope. Constants are typically used for magic numbers and configuration values that truly never change.

**Scope and shadowing**: Variables exist only within their declaring scope, usually a block delimited by curly braces. Shadowing creates a new variable with the same name, potentially of a different type. This enables progressive refinement of values through parsing or transformation pipelines while maintaining readable variable names.

**Type transformation through shadowing**: A particularly useful shadowing pattern is parsing strings to numbers. You can shadow a `String` variable with an integer of the same name, making the transformation obvious at the name level while being two distinct variables at the type system level.

### **Data Types**

**Scalar types**: Rust provides four primary scalar types. Integers (signed and unsigned, various sizes), floating-point numbers (`f32` and `f64`), booleans (`bool`), and characters (`char` - a Unicode scalar value, not a byte). Each scalar type has a known fixed size, enabling efficient memory layout and predictable performance.

**Integer type selection**: The default integer type is `i32` because it's generally fastest even on 64-bit systems due to optimization patterns. Use `i64` when you need larger ranges. Use `isize`/`usize` for indexing and pointer arithmetic since they match the architecture's word size. Explicitly sized types document size requirements and prevent platform-dependent bugs.

**Numeric operations and overflow**: Rust provides standard arithmetic operations but handles overflow differently between debug and release builds. Debug builds panic on integer overflow, catching bugs early. Release builds wrap by default for performance, though you can opt into checked, wrapping, saturating, or overflowing arithmetic explicitly through methods like `checked_add()` or `wrapping_mul()`.

**Compound types**: Tuples group values of different types with fixed size, accessed by position. Arrays hold multiple values of the same type with compile-time-known length. These compound types live on the stack with a known size, unlike heap-allocated collections like vectors. The fixed size and stack allocation provide predictable performance but less flexibility than dynamic collections.

**Tuple destructuring**: Tuples can be destructured with pattern matching, extracting components into separate variables. This is particularly useful for functions returning multiple values, providing a lightweight alternative to defining custom structs for simple multi-value returns.

**Array bounds checking**: Array access is bounds-checked at runtime, preventing buffer overflows. If you access `arr[10]` but the array has only 5 elements, Rust panics rather than accessing invalid memory. This safety comes with minimal performance cost for simple indices, and the bounds check is often optimized away when the compiler can prove safety statically.

### **Functions**

**Function declarations**: Functions are declared with `fn`, followed by the name, parameter list, and optional return type. The return type uses `->` syntax rather than the common `: Type` pattern used for variable annotations. This syntactic distinction makes function signatures visually distinct from variable declarations.

**Parameters require type annotations**: Function parameters must have explicit types. The compiler doesn't infer parameter types because it would require analyzing all call sites, slowing compilation and making function interfaces unclear. Explicit parameter types serve as documentation and enable modular compilation where functions are compiled without examining callers.

**Statements vs expressions**: Statements perform actions without returning values, like `let x = 5;`. Expressions evaluate to values, like `5 + 6` or `{ let x = 3; x + 1 }`. This distinction matters because function bodies are expressions that can return values, blocks are expressions, and control flow constructs are expressions. Understanding the expression-oriented nature of Rust enables more concise code.

**Implicit returns**: Functions return the last expression without a semicolon. Adding a semicolon turns the expression into a statement, changing the return to `()` (the unit type, equivalent to void). This pattern reduces clutter from explicit return keywords while making returns obvious - they're the expressions that aren't terminated with semicolons.

**Early returns**: The `return` keyword provides explicit early returns for error handling or special cases. This combines with Rust's expression-based syntax - most functions can use implicit returns for the success path and explicit returns for early exits.

### **Control Flow**

**If expressions**: Unlike many languages where `if` is a statement, Rust's `if` is an expression that evaluates to a value. This enables patterns like `let number = if condition { 5 } else { 6 };`. The consequence and alternative must have the same type because the `if` expression must have a single, known type at compile time.

**Condition types**: The condition in an `if` must be a `bool`. Rust won't implicitly convert numbers or other types to booleans like C or JavaScript. This explicitness prevents bugs from truthy/falsy confusion and makes intent clear - you must write `if x != 0` rather than `if x`.

**Loop variants and their uses**: `loop` creates infinite loops, suitable for server main loops or retry logic. `while` loops while a condition holds, suitable for iterating until convergence or processing until empty. `for` iterates over collections or ranges, the idiomatic choice for known iteration counts or collection traversal. Each loop type communicates different intent.

**Loop labels and nested breaks**: For nested loops, you can label loops and break to specific labels. This eliminates the need for flags or awkward control flow when breaking from inner loops should exit outer loops. The syntax `'label_name: loop` and `break 'label_name` makes multi-level breaks explicit and clear.

**Returning values from loops**: Loops can return values through `break value`, turning the loop into an expression. This pattern is useful for search loops where you break when finding the target, returning it. The loop expression evaluates to the break value, integrating naturally with Rust's expression-oriented design.

**For loops and iterators**: The `for x in collection` syntax works with any type implementing `IntoIterator`. This abstraction means `for` loops work uniformly over arrays, vectors, ranges, and custom types. The iterator protocol is a core abstraction in Rust, and `for` provides convenient syntax for it.

## 🔐 Chapter 4: Understanding Ownership - The Core Innovation

### **Core Concepts**

Chapter 4 introduces ownership, Rust's signature feature that enables memory safety without garbage collection. Understanding ownership is essential for writing Rust because it determines how memory is managed, how data is shared between functions, and how long data remains valid. Ownership is Rust's fundamental innovation that distinguishes it from other systems programming languages.

### **Memory Management Context**

**Manual management challenges**: Languages like C and C++ give programmers full control over memory allocation and deallocation. This power comes with responsibility - forget to free memory and you leak, free it twice and you corrupt state, use it after freeing and you have undefined behavior. These bugs are hard to detect and can lie dormant until specific conditions trigger them.

**Garbage collection trade-offs**: Languages like Java, Python, and Go use garbage collectors to automatically reclaim unused memory. This eliminates manual memory management bugs but introduces runtime overhead, unpredictable pause times, and increased memory usage. The garbage collector must periodically scan memory to find unreachable objects, interrupting program execution.

**Rust's ownership system**: Rust provides a third way - compile-time memory management through ownership rules. The compiler tracks ownership and automatically inserts deallocation code at the right points. This gives the control and performance of manual management with the safety of garbage collection, but moves the complexity to compile time.

### **The Stack and The Heap**

**Stack characteristics**: The stack stores values with known, fixed size. Allocation is a single pointer bump (extremely fast), and deallocation happens automatically when functions return. Stack memory is automatically managed through the call stack - when a function returns, its stack frame is popped, deallocating all local variables. Stack access is cache-friendly due to sequential memory access.

**Heap characteristics**: The heap stores values with unknown or variable size. Allocation requires finding a sufficiently large free space (slower than stack), and deallocation must be explicitly managed. Heap memory persists beyond function boundaries, enabling data to outlive the function that created it. Heap access can be cache-unfriendly due to pointer indirection and non-sequential allocation patterns.

**Performance implications**: Pushing to the stack is much faster than allocating on the heap. Following pointers to heap data is slower than accessing stack data. Copying stack data is cheap for small types, while copying heap data requires duplicating potentially large amounts of memory. These performance characteristics guide design decisions about where to allocate data.

### **Ownership Rules**

The ownership system is built on three fundamental rules that the compiler enforces:

1. **Each value has an owner**: Every piece of data has exactly one variable that owns it
2. **Only one owner at a time**: A value can't have multiple owners simultaneously
3. **When the owner goes out of scope, the value is dropped**: The owner's scope ending triggers automatic deallocation

These simple rules have profound implications for how Rust programs are structured.

### **Move Semantics**

**Heap-allocated types move by default**: Types like `String` that allocate heap memory implement move semantics. Assignment or passing to functions transfers ownership rather than copying. This prevents accidental expensive copies and eliminates double-free bugs - only one owner exists, so only one deallocation occurs.

**Shallow copy prevention**: Languages that use deep copy by default can have performance surprises when large data structures are implicitly copied. Languages that use shallow copy (copying pointers) can have memory safety issues when both copies try to manage the same memory. Rust's moves give predictable performance (cheap pointer copy) with safety (only one owner).

**Invalidating previous owners**: After a move, the previous owner becomes invalid and the compiler prevents using it. This might seem restrictive, but it prevents use-after-free bugs at compile time. The error messages clearly indicate where ownership transfers occur, making it easy to understand data flow through the program.

### **Copy Types**

**Stack-only types implement Copy**: Types stored entirely on the stack with known size at compile time can implement `Copy`. This includes integers, floats, booleans, characters, and tuples/arrays containing only `Copy` types. These types are cheap to copy bitwise, and copying doesn't require heap allocation or deallocation.

**Copy vs Clone semantics**: `Copy` types are copied implicitly on assignment or parameter passing. This maintains the familiar semantics from other languages for simple types. `Clone` provides explicit deep copying through the `clone()` method, making expensive operations visible at call sites. The distinction between implicit `Copy` and explicit `Clone` makes performance characteristics obvious.

**Ownership doesn't transfer for Copy types**: When you assign a `Copy` type or pass it to a function, the original variable remains valid because a copy was made. This enables using simple values repeatedly without explicit cloning or working around ownership restrictions.

### **Functions and Ownership**

**Passing values to functions**: Passing a value to a function moves or copies it, depending on the type. For move types, the function becomes the owner. For copy types, a copy is passed and the original remains valid. This uniform rule means you always know what happens to values based on their type.

**Returning values from functions**: Returning a value transfers ownership to the caller. This enables functions to create and return heap-allocated data without leaking it. The return value's ownership transfers to whatever variable binds it in the caller.

**The problem with move semantics**: If passing to a function always transfers ownership, you can't use data after passing it to a function unless the function returns it. This would require threading ownership through return values for every function that needs to observe but not consume data. This would be tedious and limit code structure.

### **References and Borrowing**

**References allow access without ownership transfer**: References (`&T`) provide read-only access to data without transferring ownership. The `&` operator creates a reference, borrowing the value. Borrowing means getting temporary access while the original owner retains ownership. When the reference goes out of scope, nothing happens to the referenced data because the reference never owned it.

**Immutable references**: `&T` creates an immutable reference that allows reading but not modifying. Multiple immutable references can coexist because reading is safe - no data races or inconsistency. This enables sharing data widely without copying while maintaining safety.

**Mutable references**: `&mut T` creates a mutable reference that allows modification. Only one mutable reference can exist to a piece of data in a scope, and no immutable references can coexist with it. This prevents data races at compile time - either many readers or one writer, never both simultaneously.

**The borrowing rules**: You can have either one mutable reference OR any number of immutable references to a piece of data in a scope, but not both. These rules prevent data races at compile time and eliminate entire categories of concurrency bugs without runtime overhead.

**Dangling reference prevention**: The compiler ensures references never outlive the data they point to. This prevents use-after-free bugs at compile time. If you try to return a reference to local data, the compiler rejects it because the local data would be dropped when the function returns, making the reference invalid.

### **Slices**

**Slices reference contiguous sequences**: A slice is a reference to a portion of a collection. String slices (`&str`) reference part of a `String` or string literal. Array slices (`&[T]`) reference part of an array or vector. Slices don't own the data they reference, following borrowing rules.

**Slices store a pointer and length**: Unlike references to single values, slices need two pieces of information - where the sequence starts and how long it is. This is a "fat pointer" - larger than a simple address. The size information enables bounds checking without requiring access to the original collection.

**String literals are slices**: String literals like `"hello"` have type `&str` - a reference to UTF-8 data stored in the binary's data section. This explains why string literals are immutable and have static lifetime - they're references to read-only memory that exists for the program's entire duration.

**Slices and ownership boundaries**: Taking a slice creates an immutable reference to the sliced data, preventing mutation of the original collection while the slice exists. This prevents iterator invalidation bugs - you can't modify a string while holding a slice to it, because modifying might reallocate the string's buffer, invalidating the slice.

## 🔗 Synthesis: Building the Foundation

### **From Installation to Memory Safety**

The progression through chapters 1-4 is carefully constructed. Chapter 1 establishes the practical environment and tools. Chapter 2 provides hands-on experience with Rust's syntax and patterns without deep explanation. Chapter 3 covers familiar programming concepts with Rust-specific details. Chapter 4 introduces ownership, the conceptual foundation that explains why earlier chapters' patterns exist.

This sequence works because ownership is abstract and becomes easier to understand after seeing concrete Rust code. The guessing game introduces concepts like `Result` and `match` that are motivated by ownership but can be used without understanding ownership deeply. Once you've seen these patterns, ownership explains why they exist and how they work.

### **Ownership Enables Everything Else**

Understanding ownership is prerequisite for advanced Rust features:
- **Lifetimes** make ownership relationships explicit when the compiler can't infer them
- **Traits** often have ownership implications in method signatures
- **Closures** capture variables from their environment, raising ownership questions
- **Iterators** work with borrowed data, requiring careful attention to borrowing rules
- **Concurrency** is safe because ownership prevents data races at compile time

Every Rust feature connects back to ownership. The type system, the trait system, and the module system all work together to enable memory safety without garbage collection through ownership tracking.

### **The Learning Curve Reality**

Rust has a reputation for a steep learning curve, and ownership is why. Most languages either hide memory management (garbage collection) or make it manual and unsafe (C/C++). Rust makes you think about ownership explicitly. This upfront cost pays dividends in reliability and performance, but it requires building new mental models.

The key insight is that ownership isn't arbitrary complexity - it's inherent complexity made explicit. Memory must be managed somehow. Rust's approach makes you think about it at compile time, catching bugs before they reach production. The compiler is teaching you to think about memory safety.

## 🎯 Key Takeaways

1. **Cargo provides a complete development environment** - Build tool, package manager, test runner, and documentation generator in one tool
2. **Type inference reduces verbosity without sacrificing safety** - The compiler figures out types from context, but they're still checked statically
3. **Immutability is the default** - Explicit `mut` marks mutation, making it visible and intentional
4. **Expressions return values** - Control flow constructs and blocks are expressions, enabling concise code
5. **Ownership eliminates entire bug categories** - Use-after-free, double-free, and data races are prevented at compile time
6. **Borrowing enables access without transfer** - References let functions observe data without taking ownership
7. **The stack and heap have different characteristics** - Understanding where data lives informs performance and design decisions
8. **Move semantics by default prevent accidental copies** - Expensive operations require explicit `clone()` calls

## 🧪 Testing Implications

Understanding these foundations is critical for testing:
- Ownership affects how test fixtures are structured - you may need to clone data for multiple tests
- Borrowing rules determine how test helpers can access data
- The expression-based syntax enables compact test assertions
- Error handling through `Result` integrates with test frameworks' assertion mechanisms

## 📚 Connections to Other Topics

**Related Rust Book Chapters:**
- [[rust-book-ch5-8-review]] - Builds on ownership to create structs, enums, and collections
- [[rust-book-ch10]] - Generics, Traits, Lifetimes (extends ownership with lifetime parameters)
- [[rust-book-ch15]] - Smart Pointers (advanced ownership patterns)
- [[rust-book-ch16]] - Concurrency (ownership prevents data races)

**Related Mission Work:**
- [[mission-1|Mission1]] - Stack implementation (ownership through push/pop)
- [[mission-2]] - Queue implementation (borrowing through peek operations)
- [[mission-4]] - Linked lists (complex ownership relationships)

**Related Daily Study:**
- [[daily-study/Day02]] - Ownership basics with practical examples
- [[daily-study/Day03]] - Borrowing and references deep dive
- [[daily-study/Day04]] - Lifetime fundamentals

**Related Concepts:**
- [[ownership]] - Deep dive into ownership mechanics
- [[Borrow Checker Fundamentals]] - Comprehensive borrowing rules and patterns
- [[move-semantics]] - Understanding value transfers
- [[copy-trait]] - When and why types copy automatically
- [[memory-layout]] - Stack vs heap performance implications

## 🏗️ Practical Applications

**Mission Integration:**
All missions build on these foundations:
- Every data structure must consider ownership of its elements
- Methods must decide whether to borrow, take ownership, or return ownership
- Tests must work within borrowing rules, sometimes requiring clones
- Documentation must explain ownership behaviors in public APIs

**Real-World Development:**
These concepts apply universally:
- Web frameworks use ownership to prevent request data races
- Game engines use borrowing to share world state safely
- System tools use move semantics for efficient resource management
- Parsers use references to avoid copying input strings

**Learning Strategy:**
Master these chapters before advancing because:
- Later chapters assume ownership understanding
- Advanced features are variations on ownership themes
- Fighting the borrow checker becomes easier with ownership intuition
- Design patterns emerge from ownership constraints

---

## 🏷️ Tags & Links

*Tags: #rust-book #foundation #ownership #borrowing #cargo #memory-safety #ch1 #ch2 #ch3 #ch4 #getting-started #ownership-system*

*Links: [[zettel-index]] | [[rust-book-ch1]] | [[rust-book-ch2]] | [[rust-book-ch3]] | [[rust-book-ch4]] | [[ownership]] | [[Borrow Checker Fundamentals]] | [[daily-study/Day02]]*

---

*Created: October 12, 2025*  
*Context: Rust Book foundation review, ownership system deep dive*  
*Next: [[rust-book-ch5-8-review]] - Building on ownership with structs, enums, and collections*
