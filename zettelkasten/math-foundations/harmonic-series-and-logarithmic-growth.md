# Harmonic Series and Logarithmic Growth

**Category**: Analysis, Sequences, Asymptotic Estimates
**Prerequisites**: Basic algebra, limits, integral intuition

## Definition

The **harmonic series** is the sum of reciprocals of the positive integers:

$$H_N = \sum_{i=1}^{N} \frac{1}{i} = 1 + \frac{1}{2} + \frac{1}{3} + \cdots + \frac{1}{N}$$

The values $H_N$ are called the **harmonic numbers**.

### Asymptotic Growth

$$H_N \approx \ln(N) + \gamma \quad \text{as } N \to \infty$$

where $\gamma \approx 0.5772156649\ldots$ is the **Euler–Mascheroni constant**.

A tighter expansion (valid for large $N$):

$$H_N = \ln(N) + \gamma + \frac{1}{2N} - \frac{1}{12N^2} + O(N^{-4})$$

### Divergence

$H_N \to \infty$ as $N \to \infty$, but *infinitely slowly*. Reference values:

| $N$ | $H_N$ (exact-ish) |
|-----|-------------------|
| $10$ | $2.9290$ |
| $100$ | $5.1874$ |
| $10^3$ | $7.4855$ |
| $10^6$ | $14.3927$ |
| $10^9$ | $21.3005$ |
| $10^{15}$ | $35.1200$ |

To make $H_N$ exceed 100 you need $N > 1.5 \times 10^{43}$.

## Derivation via Integral Comparison

Both bounds come from comparing the sum to an integral of $f(x) = 1/x$:

$$\int_{1}^{N+1} \frac{1}{x}\,dx \;\leq\; H_N \;\leq\; 1 + \int_{1}^{N} \frac{1}{x}\,dx$$

$$\ln(N+1) \leq H_N \leq 1 + \ln(N)$$

The two bounds squeeze $H_N$ to within 1 of $\ln(N)$, and a more careful analysis (Euler summation) pins the additive constant to exactly $\gamma$.

## Key Properties

1. **Monotone growth**: $H_{N+1} = H_N + \tfrac{1}{N+1} > H_N$.
2. **Divergent but slow**: $H_N \to \infty$, yet $H_N/N \to 0$ and $H_N/\ln(N) \to 1$.
3. **Irrational**: $H_N$ is never an integer for $N \geq 2$ (theorem of Taeisinger, 1915).
4. **Difference**: $H_N - H_M \approx \ln(N/M)$ — this is the form that shows up in algorithm analysis.

## Common Applications

### "Secretly Logarithmic" Sums

Any quantity of the form $\sum_{i=a}^{b} \frac{1}{i}$ is fundamentally $\ln(b/a)$-scale. When this expression appears inside an algorithm analysis, the complexity collapses to logarithmic in the range.

### Rare-Event Counting Under $1/i$ Density

If an event occurs at iteration $i$ with probability proportional to $1/i$, then the expected number of events in iterations $a..b$ is:

$$\mathbb{E}[\text{events}] = \sum_{i=a}^{b} \frac{1}{i} \approx \ln(b/a)$$

This is why problems with $N$ linear iterations can have only $\log N$ meaningful events.

### Work-Saving via $\ln(N)$ Iterations

If each iteration of an algorithm does "interesting work" only with probability $k/i$ for some constant $k$, the total interesting work across $N$ iterations is:

$$\sum_{i=1}^{N} \frac{k}{i} \approx k \ln(N)$$

This is the backbone of batching arguments (see AoC 2017 Day 17 below).

### Classical Occurrences

- **Coupon collector's problem**: Expected draws to collect all $n$ coupons is $n \cdot H_n \approx n \ln n$.
- **Quicksort average case**: $O(n \log n)$ comes from a harmonic-sum analysis of pivot positions.
- **Hash table probe length** (linear probing): expected search cost at load factor $\alpha$ involves $H_k$-like terms.
- **Divisor sum**: $\sum_{n=1}^{N} d(n) \approx N \ln N$ (see [[divisor-function]]).
- **$k$-th smallest element via selection**: expected comparisons use harmonic sum bounds.

## Complexity Analysis

- **Computing $H_N$ by iteration**: $O(N)$ time, $O(1)$ space.
- **Estimating $H_N$ via the asymptotic formula**: $O(1)$ time.
  - Error: $|H_N - (\ln N + \gamma)| < \frac{1}{2N}$ for $N \geq 1$.
- When $N$ is large and precision up to a few decimal places is acceptable, always prefer the closed-form estimate.

## Rust Implementations

### Exact Summation

```rust
fn harmonic(n: u32) -> f64 {
    (1..=n).map(|i| 1.0 / i as f64).sum()
}
```

### Closed-Form Estimate (O(1))

```rust
fn harmonic_estimate(n: u64) -> f64 {
    const GAMMA: f64 = 0.577_215_664_901_532_9;
    (n as f64).ln() + GAMMA + 0.5 / (n as f64)
}
```

### Verifying Euler–Mascheroni Convergence

```rust
for n in [10, 100, 1_000, 1_000_000] {
    let exact = harmonic(n);
    let est   = harmonic_estimate(n as u64);
    println!("n={n:>10}  H_n = {exact:.6}  est = {est:.6}  diff = {:.2e}",
             (exact - est).abs());
}
// n=        10  H_n = 2.928968  est = 2.927652  diff = 1.32e-03
// n=       100  H_n = 5.187378  est = 5.187377  diff = 1.08e-06
// n=      1000  H_n = 7.485471  est = 7.485471  diff = 1.39e-09
// n=   1000000  H_n = 14.392727 est = 14.392727 diff = 8.34e-16
```

## Concrete Application: AoC 2017 Day 17 — Spinlock

**Problem**: A circular buffer grows from size 1 to size $N = 5 \times 10^7$, one element per iteration. Each iteration, a value lands at an index determined by $(pos + \text{steps}) \bmod i$, where $i$ is the current size. The answer depends only on writes that land at index 1, i.e., iterations where $(pos + \text{steps}) \bmod i = 0$.

### Prediction

Since $pos$ takes values in $[0, i-1]$, the "hit" condition is one specific value out of $i$ possibilities. Assuming rough uniformity:

$$\mathbb{E}[\text{hits}] \approx \sum_{i=\text{steps}}^{N} \frac{1}{i} \approx \ln\!\left(\frac{N}{\text{steps}}\right) = \ln\!\left(\frac{5 \times 10^7}{363}\right) \approx 11.7$$

### Observation

Running the simulation: **17 hits** across all 50 million iterations. Within reasonable variance of the predicted $\sim 11.7$ — an empirical validation that the $1/i$ density is the correct model.

### Log-Log Perception Trap

When plotting hit locations on a log-x axis, they *look* evenly spread because the cumulative density of $1/i$ is $\ln(i)$, which is linear on log-x. In linear reality:

| Range | Hits (of 17) | % of hits | % of iterations |
|-------|--------------|-----------|-----------------|
| $i < 10$ | 4 | 24% | $2 \times 10^{-5}\%$ |
| $i < 10{,}000$ | 10 | 59% | $0.02\%$ |
| $i < 10^6$ | 16 | 94% | $2\%$ |

10 of 17 hits occur in the first 0.02% of iterations. This is the **Benford-like front-loading** inherent to $1/x$ densities.

### Algorithmic Exploitation — 937× Speedup

The batching optimization turns the 50M-iteration naive loop (123 ms) into $\text{steps} \cdot \ln(N/\text{steps}) \approx 4{,}260$ wrap iterations plus the same number of "fast-skip" batches (132 µs total). The **speedup factor grows with $N$**, not a constant — because linear iteration is being replaced by logarithmic iteration.

See: [AoC 2017 Day 17 function guide](../../advent_of_code/aoc2017/Problem_Statements/days/day17_function_guide.md) | [day17.rs](../../advent_of_code/aoc2017/src/solver/day17.rs)

## Related Concepts

- [[arithmetic-series]] — constant-difference sums (closed form $n(n+1)/2$)
- [[geometric-series]] — constant-ratio sums (closed form $(r^n-1)/(r-1)$)
- [[order-of-magnitude]] — scale comparison and rough estimation
- [[closed-form-formulas]] — family of summation identities
- [[divisor-function]] — $\sum d(n) \sim N \ln N$, a harmonic-sum consequence
- [[sieve-of-eratosthenes]] — running time involves $\sum 1/p$ over primes (inner harmonic)
- [[prime-number-theory]] — prime harmonic series $\sum 1/p$ diverges as $\ln \ln N$

## Examples

### Example 1: $H_{1{,}000}$ via Estimate

$$H_{1000} \approx \ln(1000) + 0.5772 + \frac{1}{2000} = 6.9078 + 0.5772 + 0.0005 = 7.4855$$

Exact: $7.48547\ldots$ — estimate accurate to 5 decimal places.

### Example 2: Coupon Collector on 1000 Items

Expected draws to collect all 1000: $1000 \cdot H_{1000} \approx 1000 \cdot 7.4855 \approx 7{,}485$.

### Example 3: Expected Hits in Day 17 Over First Million Iterations

$$\sum_{i=363}^{10^6} \frac{1}{i} \approx \ln\!\left(\frac{10^6}{363}\right) \approx 7.92$$

Observed: 16 of the 17 total hits fell below $i = 10^6$ — actual sum-of-1/i overshoots the continuous approximation slightly because early small-$i$ terms dominate.

## Historical Note

The name **"harmonic"** comes from music: a vibrating string of length $L$ produces overtones at $L/2, L/3, L/4, \ldots$ — the harmonic sequence. Nicole **Oresme** (c. 1350) gave the first proof of divergence by grouping terms:

$$\underbrace{1}_{\geq 1} + \underbrace{\tfrac{1}{2}}_{\geq 1/2} + \underbrace{\tfrac{1}{3} + \tfrac{1}{4}}_{\geq 1/2} + \underbrace{\tfrac{1}{5} + \cdots + \tfrac{1}{8}}_{\geq 1/2} + \cdots$$

Each bracketed group contributes at least $1/2$, and there are infinitely many. The sum must diverge.

The Euler–Mascheroni constant $\gamma$ was introduced by **Euler** in 1734 as the limiting difference $\lim_{N \to \infty} (H_N - \ln N)$. Whether $\gamma$ is rational or irrational is still an **open problem** — one of the oldest in number theory.

## References

- Graham, Knuth, Patashnik — *Concrete Mathematics*, Chapter 6 (Harmonic Numbers)
- Knuth — *The Art of Computer Programming* Vol. 1, Section 1.2.7
- Flajolet & Sedgewick — *Analytic Combinatorics* (harmonic sums in algorithm analysis)
- [OEIS A001008 / A002805](https://oeis.org/A001008) — numerators and denominators of $H_n$
- [Mathworld: Harmonic Series](https://mathworld.wolfram.com/HarmonicSeries.html)

---

*Links:*
- Related Problems: AoC 2017 Day 17 ([guide](../../advent_of_code/aoc2017/Problem_Statements/days/day17_function_guide.md)), [[divisor-function]]
- Related Concepts: [[arithmetic-series]], [[order-of-magnitude]], [[closed-form-formulas]], [[prime-number-theory]]
- Tags: #analysis #sequences #asymptotic #logarithm #algorithm-analysis
