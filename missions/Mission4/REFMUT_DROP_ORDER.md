# RefMut Drop-Order Gotcha in `Rc<RefCell<T>>` APIs

## Discovery

Found during test coverage improvement (Feb 2026). Adding tests for
`try_peek_front_mut()` revealed a borrow-checker sharp edge that affects
**all callers** of this function, not just the tests.

## The Problem

`try_peek_front_mut` returns `Result<Option<RefMut<'_, T>>, LinkedListError>`.

The lifetime `'_` ties the **entire** `Result` to the list — even the `Err`
variant that contains no `RefMut`. Rust's borrow checker doesn't do
variant-aware lifetime analysis; it sees the return type as a whole.

This means the `Result` must be dropped **before** the list, or you get E0597:

```rust
// FAILS TO COMPILE — list dropped while Result still borrows it
fn example() {
    let list = RcLinkedList::new();
    list.push_front(42);
    let result = list.try_peek_front_mut();
    assert!(result.is_ok());
    // result borrows list → list can't drop first → E0597
}
```

The same issue applies to `try_peek_front()` which returns
`Result<Option<Ref<'_, T>>, LinkedListError>`.

## The Fix: Block Scoping

Wrap usage in an inner block so the `Result` (and any `RefMut` inside it)
drops before the list:

```rust
fn example() {
    let list = RcLinkedList::new();
    list.push_front(42);

    {
        let result = list.try_peek_front_mut();
        assert!(result.is_ok());
    } // result dropped here

    // list safe to use or drop after this point
}
```

Or use the match-with-semicolon pattern (the compiler sometimes suggests this):

```rust
fn example() {
    let list = RcLinkedList::new();
    list.push_front(42);

    match list.try_peek_front_mut() {
        Ok(Some(mut val)) => *val = 100,
        Ok(None) => {},
        Err(e) => panic!("unexpected: {e}"),
    }; // semicolon ensures temporary drops here
}
```

## Why This Happens

Rust assigns lifetimes to the **type**, not per-variant:

```text
Result<Option<RefMut<'a, T>>, LinkedListError>
       ^^^^^^^^^^^^^^^^^^^^^^^^
       'a borrows the list in ALL variants (compiler's view)
```

The compiler doesn't reason "the `Err` path has no `RefMut`, so no borrow
exists." It conservatively assumes the borrow is live until the `Result` drops.

This is a known limitation. There's been discussion in the Rust community about
"variant-aware" borrow analysis, but it's not implemented.

## General Rule for `Rc<RefCell<T>>` APIs

Any function returning `Ref<'_, T>` or `RefMut<'_, T>` (even wrapped in
`Result` or `Option`) creates this constraint. Callers must ensure the return
value is dropped before the source. Strategies:

1. **Block scope** — most explicit, always works
2. **Semicolon after match/if-let** — drops temporaries at statement end
3. **Explicit `drop()`** — works but ordering can be subtle
4. **Consume immediately** — `list.try_peek_front_mut().unwrap().unwrap()` in
   an expression context where the temporary dies at the semicolon

## Affected Functions

| Function | Return Type |
|----------|-------------|
| `try_peek_front` | `Result<Option<Ref<'_, T>>, LinkedListError>` |
| `try_peek_front_mut` | `Result<Option<RefMut<'_, T>>, LinkedListError>` |

## Related

- [[README|./README]] — Mission 4 overview (Known Limitations section)
- [[interior-mutability|../../zettelkasten/interior-mutability]] — Interior mutability patterns
- [[Rc and RefCell|../../zettelkasten/Rc and RefCell]] — Shared ownership and runtime borrow checking
