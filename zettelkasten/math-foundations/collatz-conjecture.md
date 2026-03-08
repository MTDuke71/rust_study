# Collatz Conjecture

**Category**: Number Theory, Dynamical Systems
**Prerequisites**: Basic arithmetic, modular arithmetic

## Definition

The **Collatz conjecture** (also known as the 3n+1 problem, Ulam conjecture, or Syracuse problem) states that for every positive integer n, the sequence defined by:

$$T(n) = \begin{cases} n/2 & \text{if } n \equiv 0 \pmod{2} \\ 3n+1 & \text{if } n \equiv 1 \pmod{2} \end{cases}$$

eventually reaches 1.

> **Paul Erdos**: "Mathematics may not be ready for such problems."

**Status**: Unproven as of 2026. Verified computationally for all n < 2^68 (~2.95 x 10^20).

## Key Terminology

- **Collatz sequence**: The sequence n, T(n), T(T(n)), ... until reaching 1
- **Total stopping time** σ(n): Number of steps to reach 1
- **Stopping time**: Number of steps until the sequence first drops below n
- **Orbit**: The set of values visited by the sequence
- **Cycle**: A repeating sequence (the trivial cycle is 4 → 2 → 1)

## Example

Starting from 13:

```
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
```

- Total stopping time: σ(13) = 9 (9 steps from 13 to 1)
- Chain length: 10 terms (counting the starting value)
- Maximum value reached: 40

## Properties

### Parity Observations

- **Odd step always produces even**: If n is odd, 3n+1 = 3(2k+1)+1 = 6k+4 = 2(3k+2), always even
- **Consequence**: An odd step is always followed by at least one even step
- **Shortcut**: Combine odd+even into n → (3n+1)/2 (saves one iteration)

### Growth and Decay

- **Even step**: Divides by 2 (loses exactly 1 bit in binary)
- **Odd step**: Multiplies by ~3 (grows by ~1.58 bits in binary)
- **Combined shortcut**: Multiplies by ~1.5 (grows by ~0.58 bits)
- **On average**: If odd and even steps are roughly equal, sequences tend to shrink (log₂(3/2) < 1)

### Binary Interpretation

```
n/2:    Right-shift by 1 bit (drop least significant bit)
3n+1:   Complex bit manipulation, always clears the LSB after next /2
```

Trailing zeros in binary representation allow multiple halvings in succession. Numbers like 2^k reach 1 in exactly k steps.

## Statistical Behavior

### Benford's Law Connection

The leading digits of Collatz sequence values roughly follow Benford's law — smaller leading digits are more common. This suggests pseudo-random behavior.

### Average Stopping Time

For numbers up to N, the average total stopping time grows approximately as:

```
σ_avg(N) ≈ c · log(N)
```

where c ≈ 6.95 (empirically observed).

For N = 1,000,000: average ≈ ~120 steps (observed in Project Euler P14).

### Record Holders

Notable long chains under various limits:

| Limit | Starting Number | Chain Length |
|-------|----------------|-------------|
| 10 | 9 | 20 |
| 100 | 97 | 119 |
| 1,000 | 871 | 179 |
| 10,000 | 6,171 | 262 |
| 100,000 | 77,031 | 351 |
| 1,000,000 | 837,799 | 525 |

Record holders tend to be odd numbers — even numbers always have shorter chains than their odd "source" in the sequence.

## Why It's Hard

1. **No algebraic structure**: The function mixes multiplication (3n+1) with division (n/2) in a way that destroys algebraic patterns
2. **Chaotic behavior**: Small changes in input produce wildly different trajectories
3. **No known invariant**: No quantity is known to decrease monotonically along the sequence (which would immediately prove convergence)
4. **Connections to deep mathematics**: Related to problems in ergodic theory, p-adic analysis, and automata theory

## Computational Approaches

### Memoization

For finding the longest chain under a limit (Project Euler P14):

- Cache chain lengths in a dense array
- Walk forward until hitting a cached value, then backfill
- O(n) amortized time, O(n) space

### Optimizations

1. **Even-number elimination**: For even e < limit/2, 2e always has a longer chain. Skip e as a candidate.
2. **Odd shortcut**: n → (3n+1)/2 combines two steps (count as 2)
3. **Trailing zeros**: Shift right by `n.trailing_zeros()` bits at once
4. **Sieve-like approaches**: Precompute small values, use lookup for tail of chain

## Rust Implementation

```rust
/// Compute Collatz chain length (naive, for verification)
pub fn collatz_chain_length(mut n: u64) -> u64 {
    let mut count = 1;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        count += 1;
    }
    count
}
```

With shortcut:
```rust
while n != 1 {
    if n % 2 == 0 {
        n /= 2;
        count += 1;
    } else {
        n = (3 * n + 1) / 2;
        count += 2;  // two logical steps
    }
}
```

## Historical Notes

- **1937**: Lothar Collatz first proposed the problem
- **1950s-60s**: Independently discovered by Kakutani, Ulam, and others
- **1985**: Lagarias published comprehensive survey, establishing modern notation
- **2019**: Terence Tao proved "almost all" starting values eventually reach values below any fixed bound — the strongest partial result to date

## Key Takeaways

- Simplest unsolved problem in mathematics — easy to state, impossibly hard to prove
- Deterministic yet chaotic — each starting value has a unique, unpredictable trajectory
- Computational approaches work well for finite ranges (memoization, caching)
- The odd shortcut (3n+1 is always even) is fundamental to efficient implementation
- Dense arrays beat HashMaps for caching when the key space is bounded and nearly full

## Integration Points

**Builds On**:
- [[modular-arithmetic]] - Parity testing (n mod 2)
- [[set-theory-fundamentals]] - Orbits and cycles as sets

**Enables**:
- [[project-euler-p014]] - Direct application (longest chain under 1M)
- [[memoization-pattern]] - Classic memoization use case

**Related Concepts**:
- [[dynamic-programming]] - Overlapping subproblems in chain computation
- [[number-theory]] - Open problem in number theory
- [[math-foundations/formal-systems-invariants]] - Collatz is a transformation system where no invariant has been found to prove/disprove convergence (contrast with MIU's clean mod-3 proof)

---

**Tags**: `#number-theory` `#unsolved-conjecture` `#dynamical-systems` `#collatz` `#memoization`

*Created: 2026-02-26*
