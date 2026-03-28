# Project Euler Problem 20: Factorial Digit Sum

**Solved**: 2026-03-27
**Difficulty**: 5%
**Category**: Big Number Arithmetic

## Problem Summary

Find the sum of the digits in 100!.

## Mathematical Concepts

### Primary Concepts
- **Big number arithmetic** — Digit-array multiplication with carry propagation
- **Factorial** — n! = product of all positive integers up to n

### Supporting Concepts
- **Stirling's approximation** — Estimating digit count: log10(n!) ≈ n·log10(n/e) + ½·log10(2πn)

## Solution Approach

### Key Insights

1. **Same technique as Problem 16**: P016 multiplied by 2 repeatedly (exponentiation). Here we multiply by 2, 3, 4, ..., 100 (factorial). The digit-array carry logic is identical.

2. **Larger carries**: When multiplying by factor `k` (up to 100), a single digit × factor can produce up to 9×100 = 900, so carry values are larger than P016's max of 9×2 = 18. The `while carry > 0` loop handles this naturally.

3. **No overflow risk**: Each digit stays in 0-9, carry is at most ~900, and u32 handles this easily.

### Algorithm

```text
digits = [1]  // least-significant first

for factor in 2..=100:
    carry = 0
    for each digit d:
        product = d * factor + carry
        d = product % 10
        carry = product / 10
    while carry > 0:
        digits.push(carry % 10)
        carry /= 10

return sum(digits)  // 648
```

## Complexity Analysis

- **Time**: O(n × d) where n = 100, d ≈ 158 digits → ~15,800 operations
- **Space**: O(d) ≈ O(158)
- **Benchmark**: **8.19 µs** (Criterion, release build)
- **Justification**: Each of n factors requires a pass over all d digits

## Rust Implementation

See [[project_euler/src/problems/p020.rs]] for complete code.

### Key Code Pattern

```rust
// Identical carry-propagation loop as P016, just variable factor
for factor in 2..=n {
    let mut carry = 0u32;
    for d in digits.iter_mut() {
        let product = *d * factor + carry;
        *d = product % 10;
        carry = product / 10;
    }
    while carry > 0 {
        digits.push(carry % 10);
        carry /= 10;
    }
}
```

### Connection to Problem 16

| Aspect | P016 (2^1000) | P020 (100!) |
|--------|---------------|-------------|
| Multiplications | 1,000 | 99 |
| Factor per step | Always 2 | 2, 3, ..., 100 |
| Max single-digit carry | 9×2+1 = 19 | 9×100+carry |
| Result digits | ~302 | ~158 |
| Digit sum | 1,366 | 648 |

## Related Problems

- **[[project-euler-p016]]** — Power digit sum (same digit-array technique)
- **Future**: Any problem needing big number products can reuse this pattern

## Learning Insights

- The digit-array big-number technique is a reusable pattern: once you have multiply-with-carry, you can compute any large product
- Stirling's approximation lets you estimate result size before computing: 100! ≈ 9.33 × 10^157, so 158 digits
- In production Rust you'd use the `num-bigint` crate, but implementing from scratch builds understanding of how big integer libraries work internally

## References

- [Stirling's Approximation (Wikipedia)](https://en.wikipedia.org/wiki/Stirling%27s_approximation)
- [[project-euler-p016]] — Companion problem using same technique

---

*Links:*
- **Backlinks**: [[project-euler-p020|Problem Statement]], [[project-euler-p016]]
- **Concept Tags**: #big-number #factorial #digit-sum #project-euler
- **Difficulty**: #euler-easy
