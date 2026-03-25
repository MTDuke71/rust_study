# Day 19: An Elephant Named Joseph — Function Guide

## Problem Summary

Elves sit in a circle numbered 1 to n. They take turns stealing presents until one elf has them all.

**Part 1**: Each elf steals from the **next** elf in the circle (Josephus problem, k=2)
**Part 2**: Each elf steals from the elf **directly across** the circle

---

## Answers

| Part | Answer | Time |
|------|--------|------|
| 1 | `1834471` | ~0ns |
| 2 | `1420064` | ~0ns |
| Combined | — | 73ns |

---

## Function Map

```
solve(input) -> (String, String)
  ├── parse_input(input) -> u64              # trim + parse single number
  ├── part1(n) -> u64                        # Josephus k=2: bit rotation
  └── part2(n) -> u64                        # Across-circle: power-of-3 formula
```

---

## Algorithm Details

### The Josephus Problem

This puzzle is a direct encoding of the [Josephus problem](https://en.wikipedia.org/wiki/Josephus_problem), a classic in combinatorial mathematics dating to the 1st century historian Flavius Josephus. The puzzle title "An Elephant Named Joseph" is a pun combining "White Elephant" gift exchange with the Josephus problem.

### Part 1: Steal from Next (Josephus k=2)

With n people eliminating every 2nd person, the survivor position follows a binary trick:

1. Find `L` = largest power of 2 <= n
2. Winner = `2 * (n - L) + 1`

**Binary interpretation**: Write n in binary, rotate the leading bit to the end.

```
n = 3014387 = 10110111111110100000011 (binary)
Rotate:        01101111111101000001011
             = 1834471
```

**Implementation**: `1 << (63 - n.leading_zeros())` gives L using hardware bit intrinsics.

**Example** (n=5): L=4, winner = 2*(5-4)+1 = **3**

### Part 2: Steal from Across

When eliminating the elf directly opposite, the pattern resets at powers of 3:

1. Find `p` = largest power of 3 <= n
2. Three cases:
   - `n == p` => winner is `n`
   - `n <= 2*p` => winner is `n - p` (linear climb by 1s)
   - `n > 2*p` => winner is `2*n - 3*p` (linear climb by 2s)

**Example** (n=5): p=3, 5 <= 6, winner = 5-3 = **2**

### Why Powers of 3?

When stealing across, each elimination removes the elf at distance `floor(remaining/2)`. After `p` eliminations (where `p` is a power of 3), the circle returns to a power-of-3 size. Between powers of 3, the winner index climbs linearly — first by 1 (positions p+1 to 2p), then by 2 (positions 2p+1 to 3p).

### Complexity

- **Time**: O(log n) for finding the power — effectively O(1)
- **Space**: O(1)

---

## Key Observations

1. **Pure math, no simulation**: Both parts have closed-form solutions — no circle data structure needed.
2. **Part 1 uses bit manipulation**: `leading_zeros()` maps to hardware `lzcnt` instruction.
3. **Part 2 uses power-of-3 piecewise formula**: Three branches cover all inputs.
4. **73ns total**: Fastest day in the entire AoC 2016 set — cheaper than parsing most inputs.
5. **Historical connection**: The Josephus problem is ~2000 years old; one of the earliest known algorithmic puzzles.

---

## Benchmarks

```
Day 19 | Part 1: 1834471 | Part 2: 1420064 | 73ns (10000 iters)
```

Both parts together are faster than a single cache miss.

---

## References

- **Numberphile — "The Josephus Problem"**: [youtube.com/watch?v=uCsD3ZGzMgE](https://www.youtube.com/watch?v=uCsD3ZGzMgE)
  Covers the history and the Part 1 binary bit-rotation formula.
- **0xdf — AoC 2016 Day 19 walkthrough**: [youtube.com/watch?v=M-V_gvJomzU](https://www.youtube.com/watch?v=M-V_gvJomzU)
  Derives the Part 2 power-of-3 piecewise formula by tabulating winners and spotting the pattern. Emphasizes math observation over brute-force simulation.

---

**See also**: [AoC 2016 Summary](../summary_2016.md)
