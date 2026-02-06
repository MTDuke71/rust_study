# Sliding Window Patterns

*Tags: #algorithms #pattern #optimization #competitive-programming #aoc #sliding-window*  
*Links: [[zettel-index]] | [[Algorithms MOC]] | [[Collections MOC]] | [[10-common-interview-problems]] | [[String Algorithms]] | [[xor-properties]] | [[aoc-optimization-strategies]] | [[monotonic-stack]]*

---

## 🎯 Overview

A **sliding window** maintains a moving subrange over a sequence, updating state incrementally as the window slides. Instead of recomputing from scratch at each position, you add the entering element and remove the leaving element — reducing O(n×w) brute force to O(n).

**Core Principle**: Exploit overlap between consecutive windows. Adjacent windows share `w-1` elements — only 1 element changes per slide.

```
Input:  [a, b, c, d, e, f, g]     window_size = 4

Window 1: [a, b, c, d]             ← build from scratch
Window 2:    [b, c, d, e]          ← remove a, add e
Window 3:       [c, d, e, f]       ← remove b, add f
Window 4:          [d, e, f, g]    ← remove c, add g

Shared: 3 of 4 elements carry over each slide
```

**When to use**: Any problem asking about contiguous subarrays/substrings of fixed or variable size — sums, uniqueness, frequencies, min/max, averages.

---

## 🔍 Core Concepts

### Fixed-Size vs Variable-Size Windows

| Type | Window Size | Trigger to Slide | Examples |
|------|------------|-------------------|----------|
| **Fixed** | Constant `w` | Every step | AoC Day 6, moving average, max in window |
| **Variable** | Grows/shrinks | Constraint violated | Longest substring without repeats, min window containing all chars |

### The Rolling State Pattern

The key optimization: maintain **rolling state** that can be updated in O(1) per slide.

| State Type | Add Entering | Remove Leaving | Check | Example |
|------------|-------------|----------------|-------|---------|
| **Sum** | `sum += entering` | `sum -= leaving` | `sum == target` | Subarray sum |
| **Frequency map** | `freq[e] += 1` | `freq[l] -= 1` | `unique == w` | AoC Day 6 v2 |
| **XOR bitset** | `bits ^= entering` | `bits ^= leaving` | `popcount == w` | AoC Day 6 v3 |
| **Min/Max** | Push to deque | Pop expired | `deque.front()` | Sliding window maximum |
| **Hash** | `hash = hash*base + e` | `hash -= l*base^w` | `hash == target` | Rolling hash / Rabin-Karp |

---

## 💡 Patterns in Rust

### Pattern 1: Fixed Window with `.windows()` Iterator

Rust's stdlib provides `.windows(n)` on slices — zero-allocation overlapping views:

```rust
fn max_sum_window(data: &[i32], w: usize) -> i32 {
    data.windows(w)
        .map(|window| window.iter().sum::<i32>())
        .max()
        .unwrap()
}

// Usage:
// max_sum_window(&[1, 3, -1, 5, 2, 7], 3) → 14 (5+2+7)
```

**Caveat**: This recomputes per window (O(n×w)). Fine for small `w`, but for large windows use rolling state.

### Pattern 2: Rolling State (O(n) Total)

Manual index tracking with incremental updates:

```rust
fn max_sum_window_rolling(data: &[i32], w: usize) -> i32 {
    // Build first window
    let mut sum: i32 = data[..w].iter().sum();
    let mut best = sum;

    // Slide: remove leaving, add entering
    for i in w..data.len() {
        sum += data[i];          // add entering
        sum -= data[i - w];      // remove leaving
        best = best.max(sum);
    }
    best
}
```

### Pattern 3: XOR Rolling Bitset (AoC 2022 Day 6)

For uniqueness detection over bounded character sets — the fastest approach:

```rust
/// Find first position where last `w` characters are all unique.
/// XOR toggles bits: odd count = ON, even count = OFF.
/// popcount == w means all w chars appear exactly once.
fn find_unique_window(input: &[u8], w: usize) -> usize {
    let mut bits: u32 = 0;

    for (i, &b) in input.iter().enumerate() {
        bits ^= 1 << (b - b'a');           // XOR in entering char

        if i >= w {
            bits ^= 1 << (input[i - w] - b'a');  // XOR out leaving char
        }

        if i >= w - 1 && bits.count_ones() as usize == w {
            return i + 1;  // 1-based position
        }
    }
    panic!("No marker found");
}
```

**Why this works**: See [[xor-properties]] — XOR is self-inverse, so toggling in/out maintains correct parity count with zero branches.

### Pattern 4: Variable-Size Window (Expand/Contract)

```rust
/// Longest substring with at most `k` distinct characters
fn longest_k_distinct(s: &[u8], k: usize) -> usize {
    let mut freq = [0u32; 26];
    let mut distinct = 0;
    let mut left = 0;
    let mut best = 0;

    for right in 0..s.len() {
        // Expand: add s[right]
        let r = (s[right] - b'a') as usize;
        if freq[r] == 0 { distinct += 1; }
        freq[r] += 1;

        // Contract: while constraint violated
        while distinct > k {
            let l = (s[left] - b'a') as usize;
            freq[l] -= 1;
            if freq[l] == 0 { distinct -= 1; }
            left += 1;
        }

        best = best.max(right - left + 1);
    }
    best
}
```

**Key**: `left` pointer only moves forward — total work across all slides is O(n).

---

## 📊 Complexity Analysis

| Approach | Per Position | Total | Space | Notes |
|----------|-------------|-------|-------|-------|
| Brute force (rebuild) | O(w) | O(n×w) | O(w) | Simple but redundant |
| Rolling sum/count | O(1) | O(n) | O(1) | Optimal for numeric state |
| Rolling XOR bitset | O(1) | O(n) | O(1) | Branchless, bounded charset |
| Rolling freq counter | O(1) | O(n) | O(alphabet) | Handles variable windows |
| Monotonic deque (min/max) | O(1) amortized | O(n) | O(w) | Each element pushed/popped once |

---

## 🏗️ AoC Applications

| Problem | Window Type | Rolling State | Key Technique |
|---------|-------------|---------------|---------------|
| **AoC 2022 Day 6** | Fixed (4, 14) | XOR bitset | [[xor-properties]] for branchless uniqueness |
| **AoC subarray sums** | Fixed/Variable | Running sum | Add entering, subtract leaving |
| **Longest unique substring** | Variable | Frequency map | Expand right, contract left |
| **DNA sequence detection** | Fixed | Rolling hash | Rabin-Karp fingerprint |

### Day 6 Optimization Journey (Case Study)

This problem demonstrated why understanding sliding window state matters:

| Version | State Management | Combined Time |
|---------|-----------------|---------------|
| v1: `.windows()` + rebuild bitset | O(w) per position | 11.43µs |
| v2: Rolling frequency counter | O(1) per position, 4 branches | 5.96µs |
| v3: Rolling XOR bitset | O(1) per position, branchless | **4.80µs** |

The problem was simple enough to isolate **what makes sliding windows fast**: minimizing per-slide work. See the full journey in the [Day 6 Function Guide](../advent_of_code/aoc2022/Problem_Statements/days/day06_function_guide.md).

---

## 🧠 Mental Models

### The Train Window Analogy
Imagine looking out a train window of fixed width. As the train moves forward:
- New scenery enters from the right
- Old scenery exits from the left
- You don't re-examine the entire view — you just note what changed

### The Integration Perspective
A sliding window is a **stateful component** that processes a stream. Its interface is:
- **Input**: One element at a time (entering)
- **Output**: Query result (sum, unique count, etc.)
- **State**: Compact rolling state updated incrementally
- **Contract**: State always reflects the current window, updated in O(1)

This is exactly how an integrator thinks: what goes in, what comes out, what state is maintained.

---

## 💡 Key Takeaways

1. **Exploit overlap**: Adjacent windows share `w-1` elements. Rolling updates make each slide O(1) instead of O(w).

2. **Choose your rolling state wisely**: Sum for numeric aggregation, frequency array for character counting, XOR bitset for uniqueness, monotonic deque for min/max.

3. **`.windows()` is fine for small w**: For w ≤ ~16, the stdlib iterator is clean and fast enough. For larger windows or hot inner loops, manual rolling state wins.

4. **Variable-size windows need two pointers**: The left pointer only moves forward, ensuring O(n) total even though the window size changes.

5. **XOR is the king of uniqueness detection**: When the domain is bounded (e.g., 26 lowercase letters), XOR rolling bitset gives branchless O(1) per slide with a single register of state. See [[xor-properties]].

---

## 🔗 Integration Points

### **Builds On**
- [[two-pointer-techniques]] - Variable-size window is a two-pointer pattern
- [[Algorithms MOC]] - Fundamental algorithm design pattern

### **Enables**
- [[xor-properties]] - Rolling XOR bitset for branchless uniqueness
- [[aoc-optimization-strategies]] - Window-based optimization for AoC problems

### **Related Concepts**
- [[monotonic-stack]] - Monotonic deque is the window variant of monotonic stack
- [[string-processing-patterns]] - Many string problems use sliding windows
- [[Algorithm Design Patterns]] - Sliding window as a standard algorithm strategy

---

*Tags: #algorithms #pattern #optimization #competitive-programming #aoc #sliding-window*

*Links: [[zettel-index]] | [[Algorithms MOC]] | [[Collections MOC]] | [[10-common-interview-problems]] | [[String Algorithms]] | [[xor-properties]] | [[aoc-optimization-strategies]] | [[monotonic-stack]]*
