# Mersenne Primes: Fast Arithmetic

**Concept**: Mersenne primes (2^p - 1) enable division-free modular arithmetic via bit manipulation, replacing ~35-90 cycle `DIV` instructions with ~5 cycle shift+add sequences.

**Created**: 2026-04-15
**Tags**: #mathematics #number-theory #mersenne-primes #optimization #bit-manipulation

---

## What Are Mersenne Primes?

A **Mersenne number** is any number of the form `M_p = 2^p - 1` where p is a positive integer.

A **Mersenne prime** is a Mersenne number that is also prime:

```
M_2  = 2^2  - 1 = 3          (prime)
M_3  = 2^3  - 1 = 7          (prime)
M_5  = 2^5  - 1 = 31         (prime)
M_7  = 2^7  - 1 = 127        (prime)
M_11 = 2^11 - 1 = 2047       (NOT prime: 23 × 89)
M_13 = 2^13 - 1 = 8191       (prime)
M_17 = 2^17 - 1 = 131071     (prime)
M_19 = 2^19 - 1 = 524287     (prime)
M_31 = 2^31 - 1 = 2147483647 (prime — the Park-Miller modulus!)
```

As of 2024, the largest known prime is `2^136,279,841 - 1` (a Mersenne prime with 41 million digits).

Not all Mersenne numbers are prime, but `p` must be prime for `M_p` to have a chance.

---

## The Key Identity

For any Mersenne number `M = 2^p - 1`:

```
2^p ≡ 1 (mod 2^p - 1)
```

**Proof**: `2^p = (2^p - 1) + 1 = M + 1 ≡ 1 (mod M)`.

This means any power of 2 at or above position `p` "wraps around" to its lower equivalent:

```
2^31 ≡ 1    (mod 2^31 - 1)
2^32 ≡ 2    (mod 2^31 - 1)
2^33 ≡ 4    (mod 2^31 - 1)
2^(31+k) ≡ 2^k (mod 2^31 - 1)
```

---

## Fast Modulo: Replacing Division with Bit Ops

### The Standard Way (Slow)

```rust
let result = (a * b) % (2_u64.pow(31) - 1);  // hardware DIV: 35-90 cycles
```

The x86 `div` instruction for 64-bit operands is one of the slowest instructions on the CPU.

### The Mersenne Way (Fast)

For `n mod (2^31 - 1)`, split `n` at bit position 31:

```
n = (high bits) × 2^31 + (low 31 bits)
  ≡ (high bits) × 1   + (low 31 bits)   (mod 2^31 - 1)
  = high + low
```

The upper bits contribute exactly their face value (because `2^31 ≡ 1`).

```rust
const MODULUS: u64 = 0x7FFF_FFFF; // 2^31 - 1

#[inline]
fn mersenne_mod(n: u64) -> u64 {
    let low  = n & MODULUS;     // lower 31 bits
    let high = n >> 31;         // upper bits (the "overflow")
    let sum  = low + high;
    if sum >= MODULUS { sum - MODULUS } else { sum }
}
```

### Why the Conditional Subtract?

After adding `low + high`, the sum might still be ≥ MODULUS. But it can be at most `(2^31 - 2) + high_max`, which is always less than `2 × MODULUS`. So exactly one subtraction suffices.

For the Park-Miller generator (factor 48271, values < 2^31):
- Maximum product: `(2^31 - 2) × 48271 ≈ 2^46.5`
- Maximum `high` after `>> 31`: `48270`
- Maximum `sum`: `(2^31 - 2) + 48270 = 2,147,532,116`
- `sum < 2 × MODULUS = 4,294,967,294` — one subtract always works

### Branchless Variant (C-style)

Two passes eliminate the branch entirely:

```c
// Voltara's branchless version
x =  (x >> 31) + (x & 0x7fffffff);
x = ((x >> 31) + x) & 0x7fffffff;
```

The second pass handles the edge case where the first sum has bit 31 set.

---

## Assembly-Level Impact

### Before (with `%` operator)

```asm
xor   edx, edx           ; zero-extend for div
div   rcx                 ; 64-bit divide: 35-90 cycles, not pipelineable
mov   r11, rdx           ; remainder = result
```

### After (with `mersenne_mod`)

```asm
mov   r14d, r11d                    ; extract low 32 bits
and   r14d, 2147483647              ; low = n & 0x7FFFFFFF    (1 cycle)
shr   r11, 31                       ; high = n >> 31           (1 cycle)
lea   r15, [r14 + r11]             ; sum = low + high         (1 cycle)
cmp   r15, 2147483647              ; compare to MODULUS       (1 cycle)
lea   r11, [r14 + r11 - 2147483647] ; speculative subtract   (0 cycles — parallel)
cmovb r11, r15                      ; branchless select       (1 cycle)
```

**Total: ~5 cycles** vs **35-90 cycles** for `div`. The compiler uses `cmovb` (conditional move if below) to avoid branch misprediction — zero branches in the hot loop.

See the full annotated assembly in AoC 2017 Day 15 ASM guide.

---

## Why Park-Miller Chose 2^31 - 1

The Park-Miller "minimal standard" random number generator (1988) uses:

```
x_{n+1} = 16807 * x_n mod 2147483647
```

The modulus `2^31 - 1` was chosen specifically because:

1. **It's prime** — guarantees full period of `m - 1 = 2,147,483,646` values
2. **It's a Mersenne prime** — enables the fast modulo trick described above
3. **It fits in 31 bits** — products fit in 62 bits, well within `u64`
4. **The factor 16807 = 7^5** — a primitive root mod `2^31 - 1`, ensuring the generator visits every value from 1 to `m - 1` before repeating

This was designed in an era when hardware division was even more expensive than today. The fast-mod trick was the entire motivation.

---

## Generalization: Any Mersenne Number

The same trick works for ANY modulus of the form `2^p - 1` (prime or not):

| Modulus | p | Mask | Shift | Use Case |
|---------|---|------|-------|----------|
| `2^7 - 1 = 127` | 7 | 0x7F | 7 | Toy examples |
| `2^13 - 1 = 8191` | 13 | 0x1FFF | 13 | Small state |
| `2^17 - 1 = 131071` | 17 | 0x1FFFF | 17 | Hash functions |
| `2^31 - 1 = 2147483647` | 31 | 0x7FFFFFFF | 31 | Park-Miller LCG |
| `2^61 - 1` | 61 | 0x1FFF... | 61 | Large hash, needs u128 product |

```rust
// Generic Mersenne modulo
fn mersenne_mod_generic(n: u64, p: u32) -> u64 {
    let m = (1u64 << p) - 1;
    let sum = (n & m) + (n >> p);
    if sum >= m { sum - m } else { sum }
}
```

---

## Performance Impact: AoC 2017 Day 15

The Dueling Generators problem runs 45M+ LCG steps. Replacing `%` with `mersenne_mod`:

| Version | Time | Speedup | Instructions/iter |
|---------|------|---------|-------------------|
| Naive `%` | 333ms | 1.0× | ~80-190 cycles (two `div`) |
| Mersenne fast-mod | 186ms | 1.8× | ~7 cycles (shift+add+cmov) |
| + Rayon parallelism | 22.7ms | 14.7× | Same per-iter, spread across cores |

The fast-mod was the single highest-leverage optimization — **1.8× from changing one function**.

---

## When to Use

**Use Mersenne fast-mod when**:
- Modulus is `2^p - 1` (or you can choose it to be)
- Modulo is called in a hot loop (millions+ iterations)
- Each cycle matters (inner loops, PRNG, hashing)

**Don't bother when**:
- Modulo called infrequently (function overhead dominates)
- Compiler already optimizes constant modulus (powers of 2: `x % 16 → x & 15`)
- Modulus isn't Mersenne form (use standard `%`)

---

## Related Concepts

**Prerequisites**:
- [[math-foundations/modular-arithmetic]] - Modular operations and congruence
- [[math-foundations/prime-number-theory]] - Mersenne primes in broader context

**Applications**:
- [[math-foundations/modular-exponentiation]] - mod_pow using fast-mod in each step
- AoC 2017 Day 15 — Park-Miller LCG with 14.7× optimization

**Related Techniques**:
- [[math-foundations/number-theory-basics]] - Divisibility and prime factorization
- [[bitmask-representation]] - Bit manipulation patterns

---

**References**:
- [Wikipedia: Mersenne Prime](https://en.wikipedia.org/wiki/Mersenne_prime)
- [Wikipedia: Lehmer Random Number Generator](https://en.wikipedia.org/wiki/Lehmer_random_number_generator)
- Park & Miller, "Random Number Generators: Good Ones Are Hard to Find", 1988
- [AoC 2017 Day 15 Implementation](../../advent_of_code/aoc2017/src/solver/day15.rs)
- [AoC 2017 Day 15 ASM Guide](../../advent_of_code/aoc2017/Problem_Statements/days/day15_asm_guide.md)
