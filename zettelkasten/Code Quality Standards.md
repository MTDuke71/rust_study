# Code Quality Standards - Metrics and Requirements for Rust

A comprehensive framework defining measurable standards for maintainable, idiomatic, and robust Rust code.

🎯 Core Concept
Code Quality Standards represent the agreed-upon contract for what constitutes "acceptable" code within the engineering system. In Rust, this goes beyond simple formatting; it encompasses memory safety, idiomatic usage of the type system, and strict adherence to the compiler's guidance. These standards shift the focus from "code that works" to "code that can be maintained and evolved."

🧠 Mental Models
The "Broken Window" Theory
If you leave one function with a 100-line body or an `unwrap()` panic, it signals that low quality is acceptable. Strict standards prevent the first broken window, ensuring the codebase remains pristine.

The Compiler as a Teammate
In other languages, linters are critics. In Rust, the compiler and clippy are senior engineers doing code review. Adhering to standards means listening to this automated senior engineer before asking a human to review.

Cognitive Load Budget
Every line of code consumes "cognitive RAM" to understand. Quality standards (like low cyclomatic complexity) ensure that a function fits entirely within a developer's short-term memory, preventing bugs caused by mental overflow.

🔍 Detailed Content
1. Fundamental Hygiene (The "Must-Haves")
These are binary states: the code either passes or it fails.

Formatting: Zero tolerance for custom formatting. All code must pass `cargo fmt`.

Compilation: Code must compile with zero warnings.

Linter: `cargo clippy -- -D warnings` must return clean.

Doc Tests: Code must pass `cargo test --doc` to ensure all documentation examples are correct and compile.

Naming: See [[NAMING_CONVENTIONS]] for full details.

- **snake_case** for variables/functions.
- **PascalCase** for types/traits.
- **SCREAMING_SNAKE_CASE** for constants.

2. Complexity Metrics
To ensure maintainability, we enforce strict limits on code density.

| Metric              | Limit         | Reason                                                        |
|---------------------|--------------|---------------------------------------------------------------|
| Function length     | < 20 lines   | Forces logic decomposition and SRP (Single Responsibility).   |
| Cyclomatic complexity | < 10       | Limits branches/paths to ensure testability.                  |
| Argument count      | < 4          | More than 3 arguments usually suggests a struct is needed.    |
| Nesting depth       | < 3 levels   | Prevents "arrow code"; encourages early returns.             |

3. Idiomatic Rust Patterns
Quality Rust code utilizes the specific strengths of the language.

Expression-Oriented: Prefer implicit returns (omitting `;` on the last expression) over explicit `return` where possible.

Iterators vs. Loops: Prefer iterator chains (`map`, `filter`, `fold`, etc.) instead of C-style `for` loops with mutable state.

Type-Driven Development: Use the type system to make invalid states unrepresentable (e.g., using an `enum` instead of `bool` flags).

Error Handling:

❌ Ban: `unwrap()`, `expect()` (outside of tests/prototyping). It is acceptable in tests where a panic on `None`/`Err` is the desired behavior to fail the test.

✅ Require: `Result<T, E>`, `Option<T>`, and the `?` operator for all fallible logic.

4. Documentation Standards
Code describes how; documentation describes why.

Public API: Every `pub` item must have `///` doc comments.

Examples: Documentation must include at least one runnable Rust code block example:

```rust
// example usage goes here
```

Traceability: Comments must link to Requirements IDs (e.g., `// Implements REQ-3`).

💡 Key Takeaways
Automation over Discipline: Use `cargo fmt` and `cargo clippy` to enforce standards automatically; do not rely on willpower.

Low Complexity = High Reliability: Small functions with low cyclomatic complexity are exponentially easier to test and debug.

Types allow "Local Reasoning": Strong types and ownership rules allow you to understand a function without reading the whole codebase.

Documentation is Code: If the examples in the documentation don't compile, the code is considered broken (use `cargo test --doc`; see Fundamental Hygiene).

🔗 Integration Points
Builds On
[[ownership-fundamentals]] - Understanding ownership is a prerequisite for idiomatic standards.

[[error-handling-patterns]] - Standards rely heavily on Result over panic.

Enables
[[automated-quality-pipeline]] - These standards form the logic gates for the CI pipeline.

[[test-quality-framework]] - Low complexity allows for easier, higher-coverage testing.

Related Concepts
[[refactoring-patterns]] - Techniques to bring legacy code up to standard.

[[Quality Assurance]] - The parent framework this file supports.

🚀 Mission Applications
Mission 1: Stack Implementation
Standard: Ensure push and pop do not panic on empty/full states; return Result or Option.

Metric: Achieve 100% documentation on the public struct Stack.

Mission 5: HashMap
Standard: Manage complexity in the resize function (often a source of high cyclomatic complexity). Break it into helper functions like rehash_entries.

Idiom: Use the Entry API pattern to handle insertion logic idiomatically.

📚 Learning Progression
Introduction:
Introduction to cargo fmt and basic variable naming conventions.

Application:
Applying clippy lints and resolving common warnings regarding ownership and borrowing.

Mastery:
Refactoring complex nested loops into cleaner Iterator chains to reduce cyclomatic complexity.

📖 Official Documentation
[[rust_book/rust-book-ch12]] - "Building a Command Line Program" (Standard reference for code organization).

[[rust_book/rust-book-ch20]] - Final project structure.

Rust API Guidelines: Official style guide for public interfaces.

🔍 Code Examples
❌ Non-Compliant (Bad Quality)
```rust
// Violation: Bad naming, high nesting, manual loop, unwrap
pub fn Process_Data(input: Vec<i32>) -> i32 {
    let mut total = 0;
    for i in 0..input.len() {
        if input[i] > 10 {
            if input[i] % 2 == 0 {
                // Violation: Mutable state in loop
                total += input[i]; 
            }
        }
    }
    // Violation: Explicit return where not needed
    return total; 
}
```
✅ Compliant (Good Quality)
```rust
/// Processes data by summing even numbers greater than 10.
///
/// # Examples
/// ```
/// let data = vec![5, 12, 3, 14];
/// assert_eq!(process_data(&data), 26);
/// ```
pub fn process_data(input: &[i32]) -> i32 {
    input.iter()
        .filter(|&&x| x > 10 && x % 2 == 0)
        .sum()
}
```
Tags: #code-quality #standards #best-practices #rust-idioms #clean-code #intermediate #maintainability

Links: [[zettel-index]] | [[Quality Assurance]] | [[test-quality-framework]] | [[documentation-standards]]