# Day 3 · Functions & References

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
