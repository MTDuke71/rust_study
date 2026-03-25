# Day 14: One-Time Pad — Function Guide

**Problem**: Generate one-time pad keys by finding MD5 hashes with triplets
that are confirmed by a matching quintuplet within the next 1000 hashes.

**Part 1**: Index of 64th key (plain MD5)? → **18,626**
**Part 2**: Index of 64th key (key-stretched, 2017× MD5)? → **20,092**

---

## Performance

| Metric | Value |
|--------|-------|
| Combined | 593ms |
| Part 1 | 25.4ms |
| Part 2 | 571ms |
| Parse | negligible (single string trim) |

Combined ≈ Part 1 + Part 2 (no shared state between plain and stretched hashing).

**Rayon speedup**: Part 2 went from ~5.7s → ~571ms (~10× speedup) by parallelizing
hash computation across cores.

---

## Architecture

```
Input: "ngcjuoqr"
    │
    ▼
parse_input(input)           ──→ &str (salt)
    │
    ├── find_64th_key(salt, plain_hash)      ──→ Part 1: 18626
    │     ├── Batch compute hashes (Rayon par_iter)
    │     ├── find_triplet(hex)              ──→ first 3-repeat char
    │     └── has_quintuplet(hex, ch)        ──→ 5-repeat confirmation
    │
    └── find_64th_key(salt, stretched_hash)  ──→ Part 2: 20092
          ├── md5(salt+index) then 2016 more rounds
          └── Same triplet/quintuplet matching
```

---

## Data Structures

### Hash Cache (`Vec<String>`)
```rust
let mut cache: Vec<String> = Vec::new();
```
Stores computed hex hashes indexed by their integer index. Pre-computed in
parallel batches of `BATCH_SIZE` (2000) to amortize Rayon overhead. Essential
because each candidate hash needs to look ahead 1000 positions.

---

## Function-by-Function

### `md5_hex(input: &[u8]) -> String`
Computes MD5 and converts to 32-character lowercase hex string. Uses manual
nibble-to-char conversion instead of `format!("{:032x}")` to reduce allocation
overhead in the hot loop (called millions of times for Part 2).

### `plain_hash(salt: &str, index: u64) -> String`
Single MD5: `md5_hex(format!("{salt}{index}"))`. Used for Part 1.

### `stretched_hash(salt: &str, index: u64) -> String`
Key-stretched MD5: compute `plain_hash`, then hash the resulting hex string
2016 more times (2017 total rounds). This is the Part 2 bottleneck —
each index requires 2017 MD5 computations.

### `find_triplet(hex: &str) -> Option<u8>`
Scans the hex string for the first occurrence of three consecutive identical
characters. Uses `windows(3)` — returns `Some(b'e')` for `"...eee..."`,
`None` if no triplet exists. Only the **first** triplet matters per the rules.

### `has_quintuplet(hex: &str, ch: u8) -> bool`
Checks if the hex string contains five consecutive copies of `ch`.
Uses `windows(5)` with array comparison.

### `find_64th_key(salt, hash_fn) -> u64`
Core algorithm. Generic over the hash function (`plain_hash` or `stretched_hash`):

1. **Batch pre-compute** hashes using `Rayon::par_iter()` in chunks of 2000+
2. **Sequential scan** through cached hashes:
   - Find triplet in current hash
   - If found, check next 1000 cached hashes for matching quintuplet
   - If confirmed, increment key counter
3. **Return** when 64th key is found

The `+ Sync` bound on `hash_fn` enables Rayon to distribute hash computation
across threads.

### `parse_input(input: &str) -> &str`
Trims whitespace from the salt string. Zero-copy — returns a borrowed slice.

### `solve(input) -> (u64, u64)`
Entry point. Runs Part 1 then Part 2 sequentially (no shared state to reuse).

---

## Algorithm: Triplet-Quintuplet Key Validation

### The Rules
1. Hash `salt + index` (e.g., `MD5("ngcjuoqr18626")`)
2. If the hex hash contains a **triplet** (e.g., `"...eee..."`), it's a **candidate**
3. Search the next 1000 hashes for a **quintuplet** of the same character (`"eeeee"`)
4. If found → valid key. Count until 64 keys found.

### Why Caching Matters
Each candidate triggers a 1000-hash lookahead. Without caching, hash `N+500`
would be recomputed for every candidate between `N-500` and `N`. The cache
ensures each hash is computed exactly once.

### Why Rayon Works Here
Each hash computation is **independent** — no shared state between
`hash(salt, 1000)` and `hash(salt, 1001)`. This is the classic
"embarrassingly parallel" pattern. The sequential matching phase is
trivial compared to hash computation.

### Part 2: Key Stretching
The 2017× MD5 rounds per index make Part 2 ~22× slower per hash than Part 1.
With ~21,000 indices to compute, that's ~43 million MD5 operations.
Rayon distributes these across all CPU cores for a ~10× speedup.

### Complexity
- **Per hash**: O(1) for plain, O(2017) for stretched
- **Total hashes**: ~22,000 (to find 64th key around index 20,000 + 1000 lookahead)
- **Matching**: O(32) per triplet check, O(1000 × 32) per confirmation
- **Overall**: Dominated by hash computation, especially Part 2

---

## Patterns Used

| Pattern | Description |
|---------|-------------|
| Rayon parallel batches | Pre-compute hash batches with `into_par_iter()` |
| Hash caching | `Vec<String>` avoids recomputation during lookahead |
| Generic hash function | `Fn(&str, u64) -> String + Sync` — same logic for both parts |
| Manual hex formatting | Nibble-to-char avoids `format!` overhead in hot loop |
| Sliding window | `windows(3)` for triplets, `windows(5)` for quintuplets |

---

## Potential Optimizations (Not Applied)

| Optimization | Expected Impact | Why Not |
|-------------|-----------------|---------|
| Byte-level hash work (no hex string) | ~30% less allocation | Stretched hashing requires hex string as input |
| Larger batch sizes | Marginal Rayon overhead reduction | 2000 already amortizes well |
| Pre-compute all quintuplets per hash | Skip per-candidate scan | Memory tradeoff, current approach is fast enough |
| FxHashMap for quintuplet lookup | O(1) confirmation | Vec index lookup is already O(1) |

---

## Input Analysis

- **Salt**: `ngcjuoqr`
- **Part 1 answer index**: 18,626 (~18,600 hashes + 1000 lookahead)
- **Part 2 answer index**: 20,092 (~20,100 hashes × 2017 rounds each)
- **Total MD5 operations**: ~19,600 (Part 1) + ~42.5M (Part 2)

---

**See also**: [AoC 2016 Summary](../summary_2016.md)
