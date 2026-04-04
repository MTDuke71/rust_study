# Day 4: High-Entropy Passphrases — Function Guide

**Problem**: Validate passphrases — no duplicate words (Part 1), no anagram pairs (Part 2).
**Answers**: Part 1 = **325**, Part 2 = **119**
**Code**: [day04.rs](../../src/solver/day04.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Data Structures](#data-structures)
3. [parse_input](#parse_input)
4. [solve_part1_with_data](#solve_part1_with_data)
5. [solve_part2_with_data](#solve_part2_with_data)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: 512 passphrases, one per line, each containing space-separated words.

**Part 1**: Count passphrases where no word appears more than once.

```
aa bb cc dd ee  → VALID   (all unique)
aa bb cc dd aa  → INVALID (aa appears twice)
aa bb cc dd aaa → VALID   (aa ≠ aaa)
```

**Part 2**: Count passphrases where no two words are anagrams of each other.

```
abcde fghij       → VALID   (not anagrams)
abcde xyz ecdab   → INVALID (abcde and ecdab are anagrams)
iiii oiii ooii ... → VALID   (different letter counts)
oiii ioii iioi ... → INVALID (all anagrams of each other)
```

---

## Data Structures

### Parsed input
```rust
Vec<Passphrase<'a>>  // where Passphrase<'a> = Vec<&'a str>
```

Zero-copy parsing — each word is a `&str` slice into the original input. 512 passphrases, ~8 words each on average.

---

## `parse_input`

```rust
fn parse_input(input: &str) -> Vec<Passphrase<'_>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_whitespace().collect())
        .collect()
}
```

Straightforward: split lines, split words. `split_whitespace()` handles any whitespace and trims edges. Zero-copy via `&str` slices.

---

## `solve_part1_with_data`

```rust
fn has_no_duplicates(phrase: &[&str]) -> bool {
    let mut seen = HashSet::with_capacity(phrase.len());
    phrase.iter().all(|word| seen.insert(*word))
}
```

**Strategy**: `HashSet::insert` returns `false` if the element already exists. Combined with `.all()`, this short-circuits on the first duplicate.

**Why `with_capacity`**: Pre-allocates for the expected number of words, avoiding rehashing.

**Trace** for `["aa", "bb", "cc", "dd", "aa"]`:
```
insert("aa") → true  (new)
insert("bb") → true  (new)
insert("cc") → true  (new)
insert("dd") → true  (new)
insert("aa") → false (duplicate!) → short-circuit, return false
```

---

## `solve_part2_with_data`

```rust
fn has_no_anagrams(phrase: &[&str]) -> bool {
    let mut seen = HashSet::with_capacity(phrase.len());
    phrase.iter().all(|word| {
        let mut sorted: Vec<u8> = word.bytes().collect();
        sorted.sort_unstable();
        seen.insert(sorted)
    })
}
```

**Key insight**: Two words are anagrams if and only if their letters, sorted alphabetically, are identical. By sorting each word's letters into this canonical form, anagram detection reduces to duplicate detection.

**Why bytes, not chars**: Input is ASCII-only, so `bytes()` avoids UTF-8 overhead. `sort_unstable` is faster than `sort` for `u8` (no stability needed for identical elements).

**Trace** for `["abcde", "xyz", "ecdab"]`:
```
"abcde" → alphabetical: "abcde" → insert → true
"xyz"   → alphabetical: "xyz"   → insert → true
"ecdab" → alphabetical: "abcde" → insert → false (anagram!) → short-circuit
```

---

## Benchmarks

| Function | Time |
|----------|------|
| `solve` (combined) | 360.49µs |

**Performance breakdown**: Most time is spent in Part 2 due to byte sorting and Vec allocation per word. Part 1 is cheaper since `&str` hashing is pointer+length comparison.

**Optimization opportunities** (not needed — well under 100ms):
- Could use a fixed-size byte-frequency array `[u8; 26]` instead of sorting (O(n) vs O(n log n) per word)
- Could reuse a single `Vec<u8>` buffer across words to reduce allocations

---

## Key Patterns

### Canonicalization for equivalence classes

Anagram detection is a specific case of the general pattern: to check if elements belong to the same equivalence class, map each element to a canonical representative, then check for duplicates. Here the canonical form is letters sorted alphabetically. This same pattern appears in:
- Grouping anagrams (same technique, collecting into `HashMap<Vec<u8>, Vec<&str>>`)
- Isomorphic strings (map character positions)
- Rotation equivalence (double the string, check substring)

### Short-circuit with `HashSet::insert` + `.all()`

Instead of building a full HashSet then comparing `.len()`, we check each insertion immediately. This is both cleaner and faster — fails on the first duplicate without processing remaining words.

### Zero-copy parsing with lifetime-bound slices

`Passphrase<'a> = Vec<&'a str>` borrows directly from the input string. No `String` allocation needed since we never modify the words.

### Parse-once pattern

Both parts share the same `Vec<Passphrase>`. Parsing is lightweight here but the pattern ensures consistency and correctness.
