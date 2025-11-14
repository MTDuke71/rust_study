# String Algorithms

High-level patterns and techniques for solving string-based interview problems efficiently.

## Core Problem Patterns

- **Substring / Subsequence search**: sliding window, two pointers, KMP, Z-algorithm
- **Frequency-based questions**: hash maps / arrays for counts, anagram grouping
- **Palindrome / symmetry**: two pointers from ends, expand-around-center
- **Parsing and validation**: stacks for parentheses/brackets, state machines
- **Dynamic programming on strings**: edit distance, longest common subsequence, longest palindromic substring

## Typical Techniques

- Sliding window over `&[u8]` or `&str` slices
- Two-pointer scans with index arithmetic
- `HashMap<char, usize>` / `HashMap<String, usize>` frequency maps
- Precomputing prefix hashes for substring equality (rolling hash)
- `Vec<char>` vs `&str` tradeoffs for mutation vs slicing

---
*Links: [[10-common-interview-problems]] [[string-processing-patterns]] [[two-pointer-techniques]] [[sliding-window-patterns]]*