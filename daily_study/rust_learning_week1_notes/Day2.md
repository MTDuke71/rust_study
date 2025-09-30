# Day 2 · Variables, Mutability, Moves vs Copies

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
