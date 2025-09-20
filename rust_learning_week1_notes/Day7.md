# Day 7 · Mission 1 Review (Stack)

## Key Points
- Reimplement Stack from scratch using `Vec<T>` inside.
- Use `Option<T>` for safe pop.
- Requirements testing vs unit testing:
  - Unit: test `push/pop` directly.
  - Requirements: test REQ-1..REQ-5 explicitly.
- Ownership: push moves values in, pop moves them out.
- Borrowing: `peek` returns `&T`, `peek_mut` returns `&mut T`.

## Exercises
- Write REQ tests for generic support, push/pop, peek, ownership transfer.
- Run `cargo clippy` and resolve warnings.

## Takeaway
Mission 1 ties together ownership, borrowing, generics, and requirements traceability. Sets the stage for more complex structures.
