# Formal Systems & Invariant Proofs

*A formal system defines rules for manipulating symbols. An invariant proof shows that some property is preserved by ALL rules, making certain goals provably unreachable — no matter how clever you are.*

**Field**: Mathematical Logic / Discrete Mathematics

**Created**: 2026-03-07
**Tags**: #mathematics #formal-systems #invariants #geb #proof-technique

---

## 📐 Definition

### Formal System

A **formal system** consists of:

1. **Alphabet**: A finite set of symbols (e.g., {M, I, U})
2. **Axioms**: Starting strings (e.g., "MI")
3. **Production rules**: Ways to derive new strings from existing ones
4. **Theorems**: All strings derivable from the axioms via the rules

**Intuition**: A formal system is like a game — you start with a position (axiom) and have legal moves (rules). The "theorems" are all positions you can reach. The key question: *can you reach a specific position?*

### Invariant

A **quantity or property** that is **preserved by every rule** in the system.

If the axiom has the property, and every rule preserves it, then **every reachable string has the property** — by induction.

**Invariant proof structure**:
```
1. Define a property P
2. Show: P(axiom) is true
3. Show: For every rule R, if P(s) then P(R(s))
4. Conclude: P holds for ALL derivable strings
5. If the goal violates P, the goal is UNREACHABLE
```

---

## 🧠 Mental Models

### The MIU Game (GEB Chapter 1)

Hofstadter's MIU system from *Gödel, Escher, Bach*:

**Alphabet**: {M, I, U}
**Axiom**: MI
**Rules**:
| Rule | Pattern | Effect | Example |
|------|---------|--------|---------|
| 1 | xI → xIU | Append U if ends in I | MI → MIU |
| 2 | Mx → Mxx | Double everything after M | MII → MIIII |
| 3 | xIIIy → xUy | Replace any III with U | MIII → MU |
| 4 | xUUy → xy | Drop any UU | MUUI → MI |

**The Puzzle**: Can you derive **MU** from **MI**?

### Working Inside vs. Outside the System

This is Hofstadter's central metaphor:

- **Inside the system**: Apply rules, generate strings, try to reach MU.
  You can play forever and never know if you'll succeed.

- **Outside the system**: Analyze what the rules *can and cannot do*.
  Find an invariant. Prove impossibility in one page.

This mirrors **Gödel's Incompleteness Theorem**: some truths about a formal system can only be seen from *outside* that system.

### AUTOSAR Analogy

- **Formal system** = Behavior specification with defined interfaces
- **Axiom** = Initial system configuration
- **Rules** = Valid state transitions (runnable executions)
- **Invariant** = Safety property that must hold in all reachable states
- **Invariant proof** = Showing the safety property is preserved by every transition

---

## 🔍 The MIU Impossibility Proof

### Step 1: Choose the Right Invariant

Track the **number of I's modulo 3**:

| Rule | Effect on I-count | Effect on I-count mod 3 |
|------|-------------------|-------------------------|
| 1 (append U) | Unchanged | Unchanged |
| 2 (double after M) | Doubles | 1→2, 2→1 (toggles) |
| 3 (III → U) | Decreases by 3 | Unchanged |
| 4 (drop UU) | Unchanged | Unchanged |

### Step 2: Verify the Axiom

MI has **1** I → 1 mod 3 = **1** ≠ 0 ✓

### Step 3: Show No Rule Can Reach mod 3 = 0

Starting from I-count ≡ 1 (mod 3):

```
Rule 2: 1×2 = 2  → 2 mod 3 = 2
Rule 2: 2×2 = 4  → 4 mod 3 = 1
Rule 2: 1×2 = 2  → 2 mod 3 = 2
... forever alternating between 1 and 2
```

Rule 3 subtracts 3, which doesn't change the mod-3 value.
Rules 1 and 4 don't affect I-count at all.

**Reachable mod-3 values: {1, 2}. Never 0.**

### Step 4: MU Requires mod 3 = 0

MU has **0** I's → 0 mod 3 = **0**

Since 0 ∉ {1, 2}, **MU is unreachable from MI**. QED.

---

## 💻 Rust Implementation

### The MIU System as BFS Exploration

```rust
/// Apply all four MIU rules to a string
fn apply_rules(s: &str) -> Vec<(String, Rule)> {
    let mut results = Vec::new();

    // Rule 1: xI → xIU
    if s.ends_with('I') {
        results.push((format!("{s}U"), Rule::AppendU));
    }

    // Rule 2: Mx → Mxx
    if s.starts_with('M') && s.len() > 1 {
        let after_m = &s[1..];
        results.push((format!("M{after_m}{after_m}"), Rule::Double));
    }

    // Rule 3: replace any III with U (multiple positions!)
    for i in 0..s.len().saturating_sub(2) {
        if &s[i..i+3] == "III" {
            let new = format!("{}U{}", &s[..i], &s[i+3..]);
            results.push((new, Rule::ReplaceIII(i)));
        }
    }

    // Rule 4: drop any UU (multiple positions!)
    for i in 0..s.len().saturating_sub(1) {
        if &s[i..i+2] == "UU" {
            let new = format!("{}{}", &s[..i], &s[i+2..]);
            results.push((new, Rule::DropUU(i)));
        }
    }

    results
}
```

**Key Rust patterns**:
- `saturating_sub` prevents underflow on empty/short strings
- BFS with `HashSet<String>` for deduplication (visited set)
- Derivation path reconstruction via parent indices

### Verifying the Invariant Computationally

```rust
/// Every reachable string has I-count NOT divisible by 3
fn verify_invariant(history: &[Step]) -> bool {
    history.iter().all(|step| {
        let i_count = step.string.chars().filter(|&c| c == 'I').count();
        i_count % 3 != 0
    })
}
```

BFS explores 200+ strings from MI — **zero** have I-count divisible by 3.

**See**: `advanced_examples/geb_mu_puzzle/src/main.rs` for full implementation

---

## 💡 Key Takeaways

1. **Invariants prove impossibility** — no amount of rule application can change a preserved property
2. **Working inside a system has limits** — you need to step outside to see what's unreachable (Gödel's insight)
3. **The right invariant is everything** — mod 3 works here; mod 2 wouldn't (1→0 possible by Rule 3)
4. **BFS confirms but doesn't prove** — 200 strings with no counterexample is evidence, not proof; the invariant argument is the proof
5. **Production rules are like type system rules** — Rust's borrow checker is a formal system where "memory safety" is the invariant

## 🎯 The Deeper GEB Connection

Hofstadter uses the MIU puzzle to introduce:

- **Formal systems**: Rules + axioms = derivable theorems
- **Decision procedures**: Can we *mechanically* decide if a string is a theorem?
- **Gödel numbering**: Encoding strings as numbers (the mod-3 trick foreshadows this)
- **Incompleteness**: Some truths about the system require stepping outside it

The MIU puzzle is the simplest example of a profound idea: **no formal system can capture all truths about itself**.

---

## 🔗 Integration Points

### Builds On
- [[math-foundations/modular-arithmetic]] - The mod-3 invariant uses modular arithmetic directly
- [[math-foundations/set-theory-fundamentals]] - BFS explores reachable set of strings
- [[math-foundations/graph-theory-fundamentals]] - Derivation tree is a directed graph

### Enables
- Understanding of **Gödel's Incompleteness Theorem** (GEB's ultimate destination)
- **Loop invariant proofs** in algorithm correctness (same structure)
- **Type system reasoning** — Rust's type system is a formal system with safety invariants

### Related Concepts
- [[math-foundations/collatz-conjecture]] - Another sequence transformation system (open problem: no invariant found!)
- [[math-foundations/number-theory-basics]] - Divisibility and modular properties
- [[math-foundations/state-machines]] - State transition systems (formal systems with finite states)

### Code Implementation
- `advanced_examples/geb_mu_puzzle/` - Full BFS explorer with invariant verification (9 tests)

---

*Tags: #mathematics #formal-systems #invariants #geb #proof-technique #modular-arithmetic #intermediate*

*Links: [[zettel-index]] | [[math-foundations/modular-arithmetic]] | [[math-foundations/collatz-conjecture]] | [[math-foundations/state-machines]]*
