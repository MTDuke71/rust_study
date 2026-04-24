# Primality Fundamentals

**Field**: Number Theory

**Prerequisites**: [[number-theory-basics]], [[modular-arithmetic]], [[modular-exponentiation]]

---

## 📐 Definition

**Prime**: An integer $p \geq 2$ whose only positive divisors are $1$ and $p$.

**Composite**: An integer $n \geq 2$ that is not prime, i.e., $n = a \cdot b$ for some $2 \leq a, b < n$.

**Units and non-positives**: $1$ is *neither prime nor composite* (a unit). $0$ and negative integers are not considered prime in the standard definition.

**Coprime** (relatively prime): Two integers $a, b$ are coprime if $\gcd(a, b) = 1$. Primes with any smaller positive integer (other than multiples of themselves) are automatically coprime.

**Fundamental Theorem of Arithmetic**: Every integer $n \geq 2$ has a unique factorization into primes (up to ordering):

$$n = p_1^{e_1} \cdot p_2^{e_2} \cdots p_k^{e_k}$$

This is the reason primes are the "atoms" of integer arithmetic — every question about $n$'s divisibility, factor counts, Euler's totient, etc., reduces to a question about its prime factorization. *Primality testing* is the yes/no question that sits just before *factorization*.

**Intuition**: Primality is about **existence** (∃ a non-trivial divisor?), not **construction** (*what are* the divisors?). This matters algorithmically — proving compositeness only needs a single witness; proving primality requires ruling out every candidate.

---

## 🔑 Key Theorems and Properties

### **Theorem 1**: The √n Bound

- **Statement**: If $n$ is composite, then $n$ has at least one prime factor $p \leq \sqrt{n}$.
- **Proof**: If $n = a \cdot b$ with $a, b \geq 2$, then $\min(a, b) \leq \sqrt{n}$ (else $a \cdot b > n$). Any prime factor of $\min(a, b)$ is $\leq \sqrt{n}$.
- **Algorithmic consequence**: Trial division needs only check divisors up to $\sqrt{n}$, not up to $n$. This drops complexity from $O(n)$ to $O(\sqrt{n})$.
- **Loop idiom**: `while i * i <= n` — avoids computing $\sqrt{n}$ as a float.

### **Theorem 2**: Every Prime $> 3$ Is of Form $6k \pm 1$

- **Statement**: If $p$ is prime and $p > 3$, then $p \equiv 1 \pmod{6}$ or $p \equiv 5 \pmod{6}$.
- **Proof**: Any integer is $6k, 6k+1, 6k+2, 6k+3, 6k+4, 6k+5$. Forms $6k$, $6k+2$, $6k+4$ are even; $6k+3$ is divisible by 3. So primes $> 3$ are $6k+1$ or $6k+5 = 6k-1$.
- **Algorithmic consequence**: After handling 2 and 3, trial division only needs to check divisors of form $6k \pm 1$, cutting the candidate count by another 3× (checking 2 of every 6 instead of 1 of every 2).

### **Theorem 3**: Fermat's Little Theorem

- **Statement**: If $p$ is prime and $\gcd(a, p) = 1$, then $a^{p-1} \equiv 1 \pmod{p}$.
- **Significance**: Gives a **necessary** condition for primality, usable as a **fast** compositeness witness.
  - If $a^{n-1} \not\equiv 1 \pmod{n}$, then $n$ is **definitely composite** (a is a "Fermat witness").
  - If $a^{n-1} \equiv 1 \pmod{n}$, then $n$ is **probably prime** (could be a Fermat *liar*).
- **Limitation**: *Carmichael numbers* (561, 1105, 1729, …) pass the Fermat test for every coprime base but are still composite. So Fermat alone is not enough.

### **Theorem 4**: Miller–Rabin Witness Set (Deterministic Below 2⁶⁴)

- **Statement**: For $n < 3.32 \times 10^{14}$, it is sufficient to test Miller–Rabin with witnesses $\{2, 3, 5, 7, 11, 13, 17\}$; for $n < 2^{64}$, the witness set $\{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37\}$ is deterministic.
- **Significance**: Turns a **probabilistic** test into a **deterministic** one within fixed integer widths — extremely useful for $n > 10^9$ where trial division is too slow.
- **Source**: Jaeschke (1993), Forišek & Jančina (2015) refinements.

### **Property**: Smallest Prime Factor Heuristic

- Roughly half of all integers are even (div by 2); a third by 3; a fifth by 5. So over 70% of composite integers are caught by checking divisibility by {2, 3, 5} alone. This is why trial division runs so fast on "typical" inputs — the expected number of divisions before a witness is found is small.

---

## 💻 Rust Implementations

### Level 1: Trial Division (Textbook)

```rust
/// O(√n) primality test for small to moderate n (≤ 10⁸ practical).
fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;           // 2, 3
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i: i64 = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
```

**Gotchas**:
- `n < 2`: `0`, `1`, and all negatives must return false — dropping this check gives wrong results for zero and panics on negatives inside the loop.
- `i * i <= n`: not `i <= n.isqrt()` — faster, no float conversion, no overflow below ~3×10⁹ for `i64`.
- Starting at `i = 3, step = 2` skips all even candidates — the 2× speedup for the cost of one extra `if` at the top.

### Level 2: 6k±1 Trial Division

```rust
/// ~3× faster than Level 1 for large n: only tests divisors of form 6k±1.
fn is_prime_6k(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i: i64 = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
```

**When to prefer**: Any time trial division is the right tool and $n \geq 10^6$. For smaller $n$ the constant-factor savings are swamped by the branch in the preamble.

### Level 3: Miller–Rabin (Deterministic for u64)

```rust
/// Miller–Rabin with a fixed witness set. Deterministic for all n < 2⁶⁴.
fn is_prime_mr(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }
    // Write n - 1 = d · 2^s with d odd.
    let mut d = n - 1;
    let mut s = 0u32;
    while d & 1 == 0 {
        d >>= 1;
        s += 1;
    }
    'witness: for &a in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

/// (a * b) mod m for u64 without overflow. Use u128 as a wide intermediate.
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128) * (b as u128) % (m as u128)) as u64
}

/// (base^exp) mod m.
fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mul_mod(result, base, m);
        }
        exp >>= 1;
        base = mul_mod(base, base, m);
    }
    result
}
```

**When to prefer**: $n \geq 10^9$, or anywhere latency per call matters more than simplicity. Each call does $O(k \log n)$ modular multiplications where $k = 12$ (the witness set size) — so about 400 multiplications for a 64-bit input, vs. up to $\sqrt{n} \approx 4.3 \times 10^9$ divisions for trial division. Orders of magnitude faster above $\sim 10^{10}$.

**Mathematical background**: See [[modular-exponentiation]] for the square-and-multiply algorithm underpinning `mod_pow`. The $d \cdot 2^s$ decomposition of $n - 1$ is why Miller–Rabin is strictly stronger than Fermat's test — it catches Carmichael numbers by looking at the *sequence* of squarings, not just the final value.

---

## 📚 Code Examples

### **AoC 2017 Day 23**: Coprocessor Conflagration

**What**: A small 4-opcode VM runs a program that, for each `b` in an arithmetic progression, asks "does there exist $d, e \in [2, b)$ with $d \cdot e = b$?" — exactly the naive definition of compositeness. Part 2 of the puzzle is an O(b²)-per-value disaster if simulated literally.

**How it uses primality**: Recognise the `d`/`e` double-loop as a primality test by trial multiplication and replace it with `is_prime` (Level 1 above). 1001 candidates ≤ 126,300 resolve in ~26 µs total.

```rust
// Part 2 reduced to:
(b_start..=b_end)
    .step_by(step as usize)
    .filter(|&n| !is_prime(n))
    .count()
```

**Link**: [advent_of_code/aoc2017/src/solver/day23.rs](../../advent_of_code/aoc2017/src/solver/day23.rs), function guide [day23](../../advent_of_code/aoc2017/Problem_Statements/days/day23_function_guide.md).

**Performance**: 26 µs for 1001 primality tests on values ≤ 126,300 — ~25 ns each with Level 1 trial division.

### **Project Euler**: The Muscle Memory Example

Primality is load-bearing for nearly every early Euler problem:

- **P3** — Largest prime factor: trial division until quotient is 1, each step checks primality implicitly by "smallest divisor found = a prime factor".
- **P7** — 10,001st prime: generate primes via [[sieve-of-eratosthenes]]; primality is baked into the sieve.
- **P10** — Sum of primes below 2M: again sieve territory; individual `is_prime` calls would be slower.
- **P27** — Quadratic primes: `is_prime` called inside a search over coefficients; Level 1 trial division is fine.
- **P41** — Pandigital primes: Level 1 trial division on permutations.
- **P50** — Consecutive prime sum: sieve for generation, individual `is_prime` for partial sums.

**Pattern**: if you're generating *all* primes up to a bound, sieve. If you're testing isolated values, trial division (below 10⁸) or Miller–Rabin (above).

---

## 🌳 Decision Table: Which Algorithm When

| Regime             | Best Algorithm             | Per-call cost         | Notes                                                                                |
|--------------------|----------------------------|-----------------------|--------------------------------------------------------------------------------------|
| $n \leq 10^6$      | Level 1 trial division     | ~25 ns                | Simple, no dependencies; below this size the constant dominates any cleverness.      |
| $10^6 < n \leq 10^9$ | Level 2 (6k±1) trial div | ~100 ns – 10 µs       | Sweet spot for 32-bit inputs. ~3× faster than Level 1 from fewer candidates.         |
| $n > 10^9$         | Miller–Rabin (Level 3)     | ~1 µs                 | Trial division would take milliseconds; MR does ~400 multiplications regardless.     |
| Many primes < $N$  | [[sieve-of-eratosthenes]]  | $O(N \log \log N)$ total | Amortise across all primes up to $N$. Beats individual `is_prime` for bulk queries. |
| Cryptographic $n$  | Miller–Rabin + BPSW        | ~1 ms                 | Beyond this file's scope — arbitrary-precision integers, stronger witness regimes.  |

**Rule of thumb**: below 10⁶ nobody ever wishes they'd written Miller–Rabin. Above 10¹⁰ nobody ever wishes they'd stuck with trial division. Between them it's taste.

---

## 🪤 Common Pitfalls

### Off-by-one at n = 1

Every textbook primality test must special-case $n < 2$. `is_prime(1) == true` is the classic bug; it propagates into factor counts, divisor sums, Euler's totient, and every Project Euler problem that leans on them.

### Treating negatives as primes

Rust's `i64` lets you pass `-7` to `is_prime`. Mathematical convention says primes are positive; most textbook implementations return `false` for all $n < 2$, which covers the negatives automatically.

### `i * i` overflow

At $i \approx 3 \times 10^9$ on `i32`, `i * i` overflows and wraps. For `i64` the safe ceiling is $\sim 3 \times 10^9$; for `u64` it's $\sim 4.3 \times 10^9$. If you need primality tests on $n > 10^{18}$, Miller–Rabin via `mul_mod` with a `u128` intermediate is not optional.

### Using Fermat without fallback

Fermat's test alone produces wrong answers on Carmichael numbers (561 = 3 × 11 × 17 passes for *every* coprime base). Production-grade code uses Miller–Rabin or Baillie–PSW; Fermat is pedagogical, not operational.

### Sieve vs. individual test misapplied

If you're going to test 1000 values, a sieve is overkill (and may not even fit the value range). If you're going to test 10⁶ values, individual trial division is ~100× slower than a sieve pre-computation. Pick based on *how many* values you'll test, not how big each one is.

---

## 🌳 Related Concepts

- **Prerequisites**: [[number-theory-basics]], [[modular-arithmetic]], [[modular-exponentiation]]
- **Siblings**:
  - [[prime-number-theory]] — Distribution, PNT, prime gaps, Riemann Hypothesis (the *theory* around primes)
  - [[prime-factorization]] — Decomposition into prime factors (after a primality-negative, the next question)
  - [[sieve-of-eratosthenes]] — Bulk generation (alternate strategy when you need *every* prime up to N)
  - [[mersenne-primes-fast-arithmetic]] — Special-case primality for $2^p - 1$
- **Applications**: [[project-euler-p003]], [[project-euler-p007]], [[project-euler-p010]]
- **AoC**: [advent_of_code/aoc2017 Day 23](../../advent_of_code/aoc2017/src/solver/day23.rs) — first appearance of primality in the AoC 2017 track

---

## 📖 Resources

- *An Introduction to the Theory of Numbers* — Hardy & Wright, Ch. 1–2 (definitions), Ch. 6 (Fermat, Euler).
- *Prime Numbers: A Computational Perspective* — Crandall & Pomerance, Ch. 3 (Miller–Rabin, BPSW).
- Jaeschke, "On strong pseudoprimes to several bases" (1993) — the Miller–Rabin witness bounds used above.
- Forišek & Jančina, "Fast Primality Testing for Integers That Fit into a Machine Word" (2015) — the modern witness set for $n < 2^{64}$.
- [Wikipedia: Miller–Rabin primality test](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test) — clean pseudocode + known deterministic witness sets.
- [[sieve-of-eratosthenes]] — when you need the complementary bulk-generation tool.

---

*Tags:* #primality #number-theory #primes #trial-division #miller-rabin #fermat #algorithms #mathematics

*Created*: 2026-04-23
*Last Updated*: 2026-04-23
*Implementations*: AoC 2017 Day 23 (Level 1 trial division)
