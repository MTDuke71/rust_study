# Appendix B: Operators and Symbols

This appendix contains a reference guide for Rust's operators and symbols, organized by category.

## Operators (Overloadable)

Many Rust operators can be overloaded by implementing the corresponding trait.

### Arithmetic Operators

| Operator | Example | Description | Trait |
|----------|---------|-------------|-------|
| `+` | `expr + expr` | Addition | `Add` |
| `+=` | `var += expr` | Addition and assignment | `AddAssign` |
| `-` | `expr - expr` | Subtraction | `Sub` |
| `-=` | `var -= expr` | Subtraction and assignment | `SubAssign` |
| `-` | `-expr` | Negation | `Neg` |
| `*` | `expr * expr` | Multiplication | `Mul` |
| `*=` | `var *= expr` | Multiplication and assignment | `MulAssign` |
| `/` | `expr / expr` | Division | `Div` |
| `/=` | `var /= expr` | Division and assignment | `DivAssign` |
| `%` | `expr % expr` | Remainder | `Rem` |
| `%=` | `var %= expr` | Remainder and assignment | `RemAssign` |

### Bitwise Operators

| Operator | Example | Description | Trait |
|----------|---------|-------------|-------|
| `&` | `expr & expr` | Bitwise AND | `BitAnd` |
| `&=` | `var &= expr` | Bitwise AND and assignment | `BitAndAssign` |
| `\|` | `expr \| expr` | Bitwise OR | `BitOr` |
| `\|=` | `var \|= expr` | Bitwise OR and assignment | `BitOrAssign` |
| `^` | `expr ^ expr` | Bitwise XOR | `BitXor` |
| `^=` | `var ^= expr` | Bitwise XOR and assignment | `BitXorAssign` |
| `!` | `!expr` | Bitwise/logical complement | `Not` |
| `<<` | `expr << expr` | Left shift | `Shl` |
| `<<=` | `var <<= expr` | Left shift and assignment | `ShlAssign` |
| `>>` | `expr >> expr` | Right shift | `Shr` |
| `>>=` | `var >>= expr` | Right shift and assignment | `ShrAssign` |

### Comparison Operators

| Operator | Example        | Description           | Trait        |     |
| -------- | -------------- | --------------------- | ------------ | --- |
| "=="     | `expr == expr` | Equality              | `PartialEq`  |     |
| `!=`     | `expr != expr` | Not equal             | `PartialEq`  |     |
| `<`      | `expr < expr`  | Less than             | `PartialOrd` |     |
| `<=`     | `expr <= expr` | Less than or equal    | `PartialOrd` |     |
| `>`      | `expr > expr`  | Greater than          | `PartialOrd` |     |
| `>=`     | `expr >= expr` | Greater than or equal | `PartialOrd` |     |

### Range Operators

| Operator | Example | Description | Trait |
|----------|---------|-------------|-------|
| `..` | `expr..expr` | Right-exclusive range | `PartialOrd` |
| `..=` | `expr..=expr` | Right-inclusive range | `PartialOrd` |
| `..` | `expr..` | Range from | - |
| `..` | `..expr` | Range to | - |

### Other Overloadable Operators

| Operator | Example | Description | Trait |
|----------|---------|-------------|-------|
| `*` | `*expr` | Dereference | `Deref` |
| `[]` | `expr[expr]` | Indexing | `Index`, `IndexMut` |

## Non-Operator Symbols

### Pointer and Reference Symbols

| Symbol | Example | Description |
|--------|---------|-------------|
| `&` | `&expr`, `&mut expr` | Borrow (immutable/mutable) |
| `&` | `&type`, `&mut type` | Borrowed pointer type |
| `*` | `*const type`, `*mut type` | Raw pointer type |

### Function and Closure Symbols

| Symbol | Example | Description |
|--------|---------|-------------|
| `->` | `fn(...) -> type` | Function return type |
| `->` | `\|...\| -> type` | Closure return type |
| `\|...\|` | `\|x, y\| expr` | Closure |

### Pattern Matching Symbols

| Symbol | Example | Description |
|--------|---------|-------------|
| `\|` | `pat \| pat` | Pattern alternatives |
| `@` | `ident @ pat` | Pattern binding |
| `..` | `..`, `variant(x, ..)` | "And the rest" pattern binding |
| `...` | `expr...expr` | (Deprecated) Inclusive range pattern (use `..=`) |

### Struct and Tuple Symbols

| Symbol | Example | Description |
|--------|---------|-------------|
| `.` | `expr.ident` | Field access |
| `.` | `expr.ident(...)` | Method call |
| `.` | `expr.0`, `expr.1` | Tuple indexing |
| `..` | `Struct { x, .. }` | Struct update syntax |

### Path Symbols

| Symbol | Example | Description |
|--------|---------|-------------|
| `::` | `ident::ident` | Namespace path |
| `::` | `::path` | Path relative to crate root |
| `::` | `self::path` | Path relative to current module |
| `::` | `super::path` | Path relative to parent module |
| `::` | `<type>::ident` | Associated item |

### Generics Symbols

| Symbol | Example | Description |
|--------|---------|-------------|
| `<...>` | `Vec<T>` | Generic type parameters |
| `::<...>` | `"42".parse::<i32>()` | Turbofish syntax |
| `'a` | `&'a str` | Lifetime annotation |

### Trait Bound Symbols

| Symbol | Example | Description |
|--------|---------|-------------|
| `:` | `T: Trait` | Type constraint |
| `+` | `T: Trait1 + Trait2` | Compound constraint |
| `?Sized` | `T: ?Sized` | Allow dynamically sized types |
| `'a: 'b` | `'a: 'b` | Lifetime outlives |

### Literal Symbols

| Symbol | Example | Description |
|--------|---------|-------------|
| `"..."` | `"hello"` | String literal |
| `r"..."` | `r"C:\path"` | Raw string (no escape processing) |
| `b"..."` | `b"bytes"` | Byte string literal |
| `'...'` | `'a'` | Character literal |
| `b'...'` | `b'A'` | ASCII byte literal |
| `_` | `1_000_000` | Numeric literal separator |
| `_` | `let _ = x;` | Ignored pattern binding |

### Macro Symbols

| Symbol | Example | Description |
|--------|---------|-------------|
| `!` | `println!(...)` | Macro invocation |
| `#[...]` | `#[derive(Debug)]` | Outer attribute |
| `#![...]` | `#![allow(dead_code)]` | Inner attribute |
| `$` | `$ident` | Macro substitution |

### Comment Symbols

| Symbol | Example | Description |
|--------|---------|-------------|
| `//` | `// comment` | Line comment |
| `///` | `/// docs` | Outer line doc comment |
| `//!` | `//! module docs` | Inner line doc comment |
| `/*...*/` | `/* block */` | Block comment |
| `/**...*/` | `/** outer docs */` | Outer block doc comment |
| `/*!...*/` | `/*! inner docs */` | Inner block doc comment |

### Control Flow Symbols

| Symbol | Example               | Description          |
| ------ | --------------------- | -------------------- |
| `?`    | `expr?`               | Error propagation    |
| "=>"   | `pat => expr`         | Match arm separator  |
| `;`    | `expr;`               | Statement terminator |
| `:`    | `'label\: loop {...}` | Loop label           |

### Delimiters

| Symbol | Example | Description |
|--------|---------|-------------|
| `()` | `fn()`, `(1, 2)` | Parentheses (tuples, function calls) |
| `{}` | `{ expr }`, `Struct { }` | Curly brackets (blocks, structs) |
| `[]` | `[1, 2, 3]`, `arr[0]` | Square brackets (arrays, indexing) |

### Special Symbols

| Symbol | Example | Description |
|--------|---------|-------------|
| `!` | `fn diverges() -> !` | Never type (diverging function) |
| `,` | `expr, expr` | Argument/element separator |
| `=` | `let x = 5` | Assignment |

## Operator Precedence and Associativity

Rust follows standard operator precedence rules:
1. **Highest**: Field access (`.`), method calls, array indexing
2. **Unary operators**: `!`, `-`, `*`, `&`
3. **Type cast**: `as`
4. **Multiplicative**: `*`, `/`, `%`
5. **Additive**: `+`, `-`
6. **Shift**: `<<`, `>>`
7. **Bitwise AND**: `&`
8. **Bitwise XOR**: `^`
9. **Bitwise OR**: `|`
10. **Comparison**: "==", `!=`, `<`, `>`, `<=`, `>=`
11. **Logical AND**: `&&`
12. **Logical OR**: `||`
13. **Range**: `..`, `..=`
14. **Assignment**: `=`, `+=`, `-=`, etc.
15. **Lowest**: Return, break

## Key Insights

- **Overloadability**: Most operators can be customized via trait implementations
- **Consistency**: Similar operators in different contexts often have related meanings
- **Safety**: Type system ensures operators are used correctly
- **Ergonomics**: Rust provides rich operator syntax while maintaining explicitness
- **Turbofish**: The `::<>` syntax helps the compiler when type inference is ambiguous

---

**Book Reference**: [Appendix B: Operators and Symbols](https://doc.rust-lang.org/stable/book/appendix-02-operators.html)

**Zettelkasten Links**: [[rust-operators-reference]] | [[trait-implementation]]
