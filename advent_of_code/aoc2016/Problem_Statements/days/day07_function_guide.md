# Day 7: Internet Protocol Version 7 --- Function Guide

**Problem**: Determine which IPv7 addresses support TLS and SSL based on palindromic subsequences.
**Answers**: Part 1 = **118**, Part 2 = **260**
**Code**: [day07.rs](../../src/solver/day07.rs)

---

## Table of Contents
1. [Problem Summary](#problem-summary)
2. [Algorithm Overview](#algorithm-overview)
3. [has_abba](#has_abba)
4. [supports_tls](#supports_tls)
5. [supports_ssl](#supports_ssl)
6. [Benchmarks](#benchmarks)
7. [Key Patterns](#key-patterns)

---

## Problem Summary

**Input**: ~2000 IPv7 addresses containing alphanumeric sequences separated by bracket-delimited hypernet sequences (e.g., `abcd[efgh]ijkl[mnop]qrst`).

**Part 1 (TLS)**: Count addresses with an **ABBA** (4-char palindrome like `xyyx`, where `x != y`) in any supernet sequence (outside brackets), but **no** ABBA in any hypernet sequence (inside brackets).

**Part 2 (SSL)**: Count addresses with an **ABA** (3-char palindrome like `xyx`) in any supernet sequence that has a corresponding **BAB** (`yxy`) in any hypernet sequence.

**Examples**:
```
abba[mnop]qrst     -> TLS: yes  (abba outside, no ABBA inside)
abcd[bddb]xyyx     -> TLS: no   (bddb inside brackets)
aba[bab]xyz        -> SSL: yes  (aba outside, bab inside)
zazbz[bzb]cdb      -> SSL: yes  (zbz outside, bzb inside)
```

---

## Algorithm Overview

Both parts scan each line, splitting into supernet (outside `[]`) and hypernet (inside `[]`) segments. No allocation needed for Part 1 (streaming check); Part 2 collects segment slices for cross-matching.

```
Part 1: Stream through line, track in/out bracket state
        -> ABBA in hypernet = reject immediately
        -> ABBA in supernet = candidate (still check all hypernets)

Part 2: Collect supernet/hypernet byte slices
        -> For each ABA in supernets, check if BAB exists in any hypernet
```

**Complexity**: O(n * L) where n = number of IPs, L = average line length. Each line scanned with sliding windows (size 4 for ABBA, size 3 for ABA/BAB).

---

## `has_abba`

```rust
fn has_abba(s: &[u8]) -> bool
```

Checks if a byte slice contains any ABBA pattern using `windows(4)`. An ABBA matches when `w[0] == w[3] && w[1] == w[2] && w[0] != w[1]` — a 4-char palindrome with distinct outer/inner characters.

**Key detail**: `aaaa` is NOT an ABBA because the inequality check `w[0] != w[1]` rejects it.

---

## `supports_tls`

```rust
fn supports_tls(line: &str) -> bool
```

Streams through the line tracking bracket state. On `[`, checks the preceding supernet segment for ABBA. On `]`, checks the hypernet segment — if ABBA found inside brackets, returns `false` immediately (early exit). After the final segment, checks for ABBA in the trailing supernet.

**Design choice**: No allocation. Uses byte slice indices to define segments on the fly.

---

## `supports_ssl`

```rust
fn supports_ssl(line: &str) -> bool
```

First pass collects `&[u8]` slices for all supernet and hypernet segments. Second pass iterates all ABA patterns in supernets (`windows(3)` where `w[0] == w[2] && w[0] != w[1]`), constructs the corresponding BAB `[w[1], w[0], w[1]]`, and checks if any hypernet contains it.

**Design choice**: Collects segments into `Vec<&[u8]>` because cross-matching requires access to all hypernets for each ABA found in supernets. Borrows from input — no string allocation.

---

## Benchmarks

| Benchmark | Time |
|-----------|------|
| Part 1    | 255µs |
| Part 2    | 436µs |
| Combined  | 713µs |

Part 1 is ~1.7x faster than Part 2: it streams through each line with zero allocation and early-exits on hypernet ABBA. Part 2 must collect segment slices into vectors, then do a nested scan (each ABA × all hypernet windows). Combined ≈ sum of parts confirms no shared parse step.

---

## Key Patterns

| Pattern | Description |
|---------|-------------|
| Sliding window | `windows(4)` for ABBA, `windows(3)` for ABA/BAB — standard palindrome detection |
| Bracket state machine | Track `in_bracket` flag with `start` index to segment lines without splitting |
| Early rejection | ABBA inside hypernet → immediate `false` for TLS (skip remaining segments) |
| Zero-copy segments | `&[u8]` slices borrow from input string — no allocation for segment extraction |
| Cross-segment matching | ABA in supernets matched against BAB in hypernets — requires collecting segments |
| Byte-level comparison | Work with `&[u8]` throughout — avoids char boundary overhead on ASCII input |
