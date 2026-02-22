# Algebraic Inversion

**Field**: Algebra / Expression Trees

**Prerequisites**: Basic arithmetic operations, [[set-theory-fundamentals]] (for tree concepts)

---

## Definition

**Algebraic Inversion**: Given an equation `a op b = target` where one of `a` or `b` is unknown, solve for the unknown by applying the *inverse operation*.

**Formal Statement**:
- If $a + b = t$, then $a = t - b$ and $b = t - a$
- If $a - b = t$, then $a = t + b$ and $b = a - t$
- If $a \times b = t$, then $a = t / b$ and $b = t / a$
- If $a / b = t$, then $a = t \times b$ and $b = a / t$

**Intuition**: Every arithmetic operation has an inverse that "undoes" it. Addition undoes subtraction, multiplication undoes division. But for non-commutative operations (subtraction, division), the inversion depends on *which side* the unknown is on.

---

## Key Properties/Theorems

### **Property 1**: Commutative Operations Invert Symmetrically

For addition and multiplication, the unknown's position doesn't matter:
- $x + k = t \Rightarrow x = t - k$
- $k + x = t \Rightarrow x = t - k$ (same!)
- $x \times k = t \Rightarrow x = t / k$
- $k \times x = t \Rightarrow x = t / k$ (same!)

### **Property 2**: Non-Commutative Operations Invert Asymmetrically

For subtraction and division, the unknown's position changes the inversion:

| Expression | Solve for left | Solve for right |
|-----------|----------------|-----------------|
| $L + R = T$ | $L = T - R$ | $R = T - L$ |
| $L - R = T$ | $L = T + R$ | $R = L - T$ |
| $L \times R = T$ | $L = T / R$ | $R = T / L$ |
| $L / R = T$ | $L = T \times R$ | $R = L / T$ |

**The non-commutative trap**: Subtraction and division produce different inversions depending on which operand is unknown. This is the #1 source of bugs.

### **Property 3**: Tree Walk Inversion

When an unknown variable is buried deep in an expression tree, inversion can be applied level by level from the root down to the unknown. At each node:
1. Determine which child contains the unknown
2. Evaluate the other child to get the known value
3. Invert the operation to compute what the unknown child must equal
4. Recurse into the unknown child with the new target

This works because each node in the path from root to unknown has exactly one unknown child (the tree property — no shared subexpressions).

### **Theorem**: Unique Solution Guarantee

If an expression tree has exactly one unknown leaf and all operations are invertible (no $x / 0$ etc.), the tree walk produces exactly one solution. This follows from:
- Each level reduces the problem by one operation
- Each inversion has a unique result (for non-zero divisors)
- The unknown is reached after $d$ inversions (where $d$ = depth)

---

## Rust Implementations

### **AoC 2022 Day 21**: Monkey Math - Expression Tree Inversion

- **What**: ~1,933 monkeys form a binary expression tree. Part 2: find what value `humn` must yell so that `root`'s two children are equal.
- **How it uses algebraic inversion**:
  - `root` becomes equality check: `left == right`
  - One subtree contains `humn`, the other is fully evaluable
  - Walk from `root` toward `humn`, inverting operations at each level
- **Link**: `advent_of_code/aoc2022/src/solver/day21.rs`

**Core inversion code from Day 21**:
```rust
let new_target = if humn_in_left {
    match op {
        '+' => target - known_val,   // left + k = t  ->  left = t - k
        '-' => target + known_val,   // left - k = t  ->  left = t + k
        '*' => target / known_val,   // left * k = t  ->  left = t / k
        '/' => target * known_val,   // left / k = t  ->  left = t * k
        _ => unreachable!(),
    }
} else {
    match op {
        '+' => target - known_val,   // k + right = t ->  right = t - k
        '-' => known_val - target,   // k - right = t ->  right = k - t  <-- TRAP!
        '*' => target / known_val,   // k * right = t ->  right = t / k
        '/' => known_val / target,   // k / right = t ->  right = k / t  <-- TRAP!
        _ => unreachable!(),
    }
};
```

**The asymmetry in action**:
- `left - k = t` => `left = t + k` (add k to both sides)
- `k - right = t` => `right = k - t` (NOT `t - k`! Rearrange: `k - t = right`)

### **Worked Example**: Day 21 Example Input

```
root: pppw + sjmn     (Part 2: pppw == sjmn)
pppw: cczh / lfqf
sjmn: drzm * dbpl     = 150 (known side, no humn)
cczh: sllz + lgvd
lfqf: 4
sllz: 4
lgvd: ljgn * ptdq
ljgn: 2
ptdq: humn - dvpt     <-- humn is here!
dvpt: 3
```

**Walk from root to humn**:
1. `root`: pppw must equal sjmn. eval(sjmn) = 150. Target for pppw = **150**
2. `pppw: cczh / lfqf = 150`. lfqf = 4. Unknown is left (cczh). `cczh = 150 * 4 = `**600**
3. `cczh: sllz + lgvd = 600`. sllz = 4. Unknown is right (lgvd). `lgvd = 600 - 4 = `**596**
4. `lgvd: ljgn * ptdq = 596`. ljgn = 2. Unknown is right (ptdq). `ptdq = 596 / 2 = `**298**
5. `ptdq: humn - dvpt = 298`. dvpt = 3. Unknown is left (humn). `humn = 298 + 3 = `**301**

---

## Code Examples

### **Generic Inversion Function**

```rust
/// Invert `left op right = target` to solve for the unknown side.
fn invert(op: char, target: i64, known: i64, unknown_is_left: bool) -> i64 {
    if unknown_is_left {
        // Solving: unknown OP known = target
        match op {
            '+' => target - known,
            '-' => target + known,
            '*' => target / known,
            '/' => target * known,
            _ => panic!("unknown operator"),
        }
    } else {
        // Solving: known OP unknown = target
        match op {
            '+' => target - known,    // commutative: same as left
            '-' => known - target,    // NOT target - known!
            '*' => target / known,    // commutative: same as left
            '/' => known / target,    // NOT target / known!
            _ => panic!("unknown operator"),
        }
    }
}
```

### **Why the Asymmetry Matters: A Concrete Example**

```rust
// Equation: 10 - x = 3
// Correct: x = 10 - 3 = 7   (known - target)
// Wrong:   x = 3 - 10 = -7  (target - known)  <-- WRONG!

// Verify: 10 - 7 = 3  ✓
// Verify: 10 - (-7) = 17  ✗

let result = invert('-', 3, 10, false);  // known=10, target=3, unknown on right
assert_eq!(result, 7);  // 10 - 7 = 3 ✓
```

### **Pattern: Expression Tree with HashMap**

```rust
use std::collections::HashMap;

enum Expr<'a> {
    Num(i64),
    Op { left: &'a str, op: char, right: &'a str },
}

fn eval(exprs: &HashMap<&str, Expr<'_>>, name: &str) -> i64 {
    match &exprs[name] {
        Expr::Num(n) => *n,
        Expr::Op { left, op, right } => {
            let l = eval(exprs, left);
            let r = eval(exprs, right);
            match op {
                '+' => l + r,
                '-' => l - r,
                '*' => l * r,
                '/' => l / r,
                _ => unreachable!(),
            }
        }
    }
}
```

This recursive DFS evaluation visits each node exactly once (O(n) total) because the structure is a tree with no shared subexpressions.

---

## Related Concepts

### **Prerequisites**:
- Basic algebra (inverse operations)

### **Related Mathematical Concepts**:
- [[modular-arithmetic]] - Modular inverse for multiplication in modular contexts
- [[linear-algebra-fundamentals]] - Matrix inversion generalizes scalar inversion
- [[number-theory-basics]] - Modular multiplicative inverse (Fermat's little theorem)

### **Related Rust Concepts**:
- [[common-traits-pattern]] - Pattern matching on enum variants for clean dispatch
- [[expression-tree]] - Data structure for representing mathematical expressions

### **Applications**:
- **Symbolic algebra**: Simplifying equations, solving for variables
- **Compiler optimization**: Constant folding, strength reduction
- **AoC expression puzzles**: Any problem where you evaluate a tree then solve for an unknown
- **Automatic differentiation**: Chain rule is related to walking expression trees

---

## Common Patterns in AoC/Competitive Programming

### **Pattern 1: Evaluate-Then-Invert**
The most common pattern: first evaluate the expression tree normally (Part 1), then for Part 2, one value becomes unknown and you invert the tree to solve for it.

```
Part 1: eval(root) → answer
Part 2: root becomes equality, solve for unknown leaf
```

### **Pattern 2: contains_unknown Check**
At each node during the inversion walk, determine which subtree contains the unknown:

```rust
fn contains_unknown(tree: &Tree, name: &str) -> bool {
    if name == "unknown" { return true; }
    match &tree[name] {
        Leaf(_) => false,
        Node { left, right, .. } => {
            contains_unknown(tree, left) || contains_unknown(tree, right)
        }
    }
}
```

This can be optimized by caching results, but for tree structures each node is visited at most once per query.

### **Pattern 3: Verify Solution**
After solving for the unknown, substitute it back and verify:

```rust
// Solve for humn
let humn_value = solve_for_humn(&monkeys, humn_side, target);

// Verify: modify humn in the tree, re-evaluate, check equality
// eval(left_of_root) should now equal eval(right_of_root)
```

---

## Resources

### **Mathematical Foundations**:
- [Inverse Operations (Wikipedia)](https://en.wikipedia.org/wiki/Inverse_function)
- [Expression Trees (Wikipedia)](https://en.wikipedia.org/wiki/Binary_expression_tree)

### **Rust Documentation**:
- [`std::collections::HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) - Used for O(1) node lookup in expression trees
- [Pattern matching](https://doc.rust-lang.org/book/ch06-02-match.html) - Clean dispatch on enum variants

---

## Key Insights

**The Non-Commutative Trap**:
- Addition and multiplication are commutative: inversion is the same regardless of which side is unknown
- Subtraction and division are NOT commutative: `a - b != b - a`, so the inversion formula differs
- Memorize: when unknown is on the RIGHT of `-` or `/`, the formula is `known OP target` (not `target OP known`)

**Expression Tree = Recursive HashMap**:
- Each node is a HashMap entry keyed by name
- Evaluation is DFS: resolve children before combining
- No memoization needed for trees (each node visited exactly once)

**Tree Walk Inversion is O(n)**:
- Walk from root to unknown leaf: O(d) levels where d = tree depth
- At each level: one `contains_unknown` check + one `eval` of the known subtree
- Total work across all levels = O(n) since the known subtrees partition the non-path nodes

**From Day 21 Experience**:
> "The non-commutative trap for subtraction and division is the #1 bug source. Draw out the algebra on paper first: `k - x = t` means `x = k - t`, NOT `x = t - k`."

---

## Complexity Reference

| Operation | Time | Notes |
|-----------|------|-------|
| Tree evaluation (Part 1) | O(n) | Visit each of n nodes once |
| contains_unknown per call | O(subtree size) | DFS through subtree |
| Tree walk inversion (Part 2) | O(n) | Eval known sides + walk path |
| Total solve | O(n) | Single pass through all nodes |

**Space**: O(d) call stack depth, where d = tree depth (~11 for balanced tree of 1,933 nodes)

---

*Tags: #mathematics #algebra #inversion #expression-tree #aoc2022-day21 #tree-walk*

*Created*: 2026-02-22
*Last Updated*: 2026-02-22
*Implementations*: 1 (AoC 2022 Day 21)
