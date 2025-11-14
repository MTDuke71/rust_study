---
title: Sliding Window Patterns
---

Sliding window patterns maintain a moving subrange over a sequence to compute aggregates or satisfy constraints in linear time. Typical interview problems include longest/shortest substring with a property, frequency-constrained windows, and streaming statistics.

This note links conceptual sliding-window strategies to Rust implementations using index ranges, `VecDeque`, and iterator adapters, and to AoC problems where windowed aggregation appears.

*Links: [[10-common-interview-problems]] [[String Algorithms]] [[Collections MOC]] [[Algorithms MOC]]*
