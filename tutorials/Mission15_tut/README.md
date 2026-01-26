# Mission 15 Tutorial: String Algorithms & Pattern Matching (10 Steps)

**Tutorial Focus**: Tries, suffix trees, string matching algorithms, text processing

**Zettelkasten**: [[mission-15]] | [[trie-data-structure]] | [[string-algorithms]]

---

## 📚 Overview

This tutorial explores **string algorithms and pattern matching** - essential techniques for text processing, searching, and parsing. Learn trie data structures, efficient string matching algorithms (KMP, Rabin-Karp), and practical applications in autocomplete, spell-checking, and parsing.

**Key Concepts**:
- Trie (prefix tree) data structure
- String matching algorithms (naive, KMP, Rabin-Karp, Boyer-Moore)
- Suffix trees and suffix arrays
- Text search and pattern recognition
- Unicode handling in Rust

**Why String Algorithms Matter**:
- **AoC Parsing**: Many problems require complex input parsing
- **Text Processing**: Search, autocomplete, spell-check
- **Bioinformatics**: DNA sequence matching
- **Practical**: Real-world applications in editors, databases, search engines

---

## 🎯 Learning Path (7 Days)

### **Day 1: Trie Basics** (Step 1)
**File**: `examples/step1_trie_basics.rs`

**Topics**:
- Trie (prefix tree) structure
- Trie node representation
- Insertion and search operations
- Prefix matching

**Structure**:
```rust
struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    is_end_of_word: bool,
}

pub struct Trie {
    root: TrieNode,
    size: usize,  // Number of words
}
```

**Example Trie**:
```
Words: ["cat", "car", "card", "care", "dog"]

        (root)
        /    \
       c      d
       |      |
       a      o
      / \     |
     t   r    g*
     *  / \
       d   e
       |   |
       *   *
```

**Operations**:
- `insert(word)` - Add word to trie
- `search(word)` - Check if exact word exists
- `starts_with(prefix)` - Check if any word has prefix

**Learning Outcomes**:
- ✅ Understand trie structure vs HashMap
- ✅ Implement basic trie operations
- ✅ Recognize O(m) complexity (m = word length)

---

### **Day 2: Trie Applications** (Step 2)
**File**: `examples/step2_trie_applications.rs`

**Topics**:
- Autocomplete with tries
- Spell-checking (finding all words)
- Longest common prefix
- Word frequency counting

**Autocomplete**:
```rust
impl Trie {
    pub fn autocomplete(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        
        // Navigate to prefix node
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return vec![],  // Prefix not found
            }
        }
        
        // Collect all words from this subtree
        let mut results = vec![];
        self.collect_words(node, prefix.to_string(), &mut results);
        results
    }
    
    fn collect_words(&self, node: &TrieNode, current: String, results: &mut Vec<String>) {
        if node.is_end_of_word {
            results.push(current.clone());
        }
        for (ch, child) in &node.children {
            self.collect_words(child, format!("{}{}", current, ch), results);
        }
    }
}
```

**Applications**:
- Search bar autocomplete
- Spell-checker suggestions
- DNA sequence prefix matching
- IP routing tables (longest prefix match)

**Learning Outcomes**:
- ✅ Build autocomplete system
- ✅ Traverse trie for all matching words
- ✅ Understand trie advantages (O(m) vs O(n log n) for sorted array)

---

### **Day 3: Naive & Rabin-Karp String Matching** (Step 3)
**File**: `examples/step3_string_matching_basic.rs`

**Topics**:
- Naive string matching (brute force)
- Rabin-Karp algorithm (rolling hash)
- Hash collision handling

**Naive Algorithm**:
```rust
pub fn naive_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut positions = vec![];
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    
    for i in 0..=(text_chars.len() - pattern_chars.len()) {
        let mut match_found = true;
        for j in 0..pattern_chars.len() {
            if text_chars[i + j] != pattern_chars[j] {
                match_found = false;
                break;
            }
        }
        if match_found {
            positions.push(i);
        }
    }
    positions
}
// Time: O(nm) where n = text length, m = pattern length
```

**Rabin-Karp (Rolling Hash)**:
```rust
pub fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    let pattern_hash = hash(pattern);
    let mut positions = vec![];
    
    for i in 0..=(text.len() - pattern.len()) {
        let window = &text[i..i+pattern.len()];
        if hash(window) == pattern_hash && window == pattern {
            positions.push(i);
        }
    }
    positions
}
// Average: O(n+m), Worst: O(nm)
```

**Learning Outcomes**:
- ✅ Implement naive string search
- ✅ Understand rolling hash technique
- ✅ Handle hash collisions

---

### **Day 4: KMP Algorithm** (Step 4)
**File**: `examples/step4_kmp_algorithm.rs`

**Topics**:
- Knuth-Morris-Pratt algorithm
- Failure function (LPS array)
- Avoiding redundant comparisons

**KMP Algorithm**:
```rust
pub fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let lps = compute_lps(pattern);
    let mut positions = vec![];
    
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    
    let mut i = 0;  // text index
    let mut j = 0;  // pattern index
    
    while i < text_chars.len() {
        if text_chars[i] == pattern_chars[j] {
            i += 1;
            j += 1;
        }
        
        if j == pattern_chars.len() {
            positions.push(i - j);
            j = lps[j - 1];
        } else if i < text_chars.len() && text_chars[i] != pattern_chars[j] {
            if j != 0 {
                j = lps[j - 1];  // Use failure function
            } else {
                i += 1;
            }
        }
    }
    positions
}

// LPS: Longest Proper Prefix which is also Suffix
fn compute_lps(pattern: &str) -> Vec<usize> {
    let chars: Vec<char> = pattern.chars().collect();
    let mut lps = vec![0; chars.len()];
    let mut len = 0;
    let mut i = 1;
    
    while i < chars.len() {
        if chars[i] == chars[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
    lps
}
// Time: O(n+m) guaranteed
```

**Why KMP Wins**:
- Naive: O(nm) - rechecks characters
- KMP: O(n+m) - never rechecks, uses LPS array

**Learning Outcomes**:
- ✅ Implement KMP algorithm
- ✅ Compute LPS (failure function)
- ✅ Understand why KMP is O(n+m)

---

### **Day 5: Suffix Trees & Arrays** (Step 5)
**File**: `examples/step5_suffix_structures.rs`

**Topics**:
- Suffix trees (conceptual)
- Suffix arrays (practical)
- Longest repeated substring
- Pattern matching with suffix arrays

**Suffix Array**:
```rust
pub struct SuffixArray {
    text: String,
    suffixes: Vec<usize>,  // Starting positions, sorted
}

impl SuffixArray {
    pub fn new(text: &str) -> Self {
        let mut suffixes: Vec<usize> = (0..text.len()).collect();
        
        // Sort suffixes lexicographically
        suffixes.sort_by(|&a, &b| text[a..].cmp(&text[b..]));
        
        SuffixArray {
            text: text.to_string(),
            suffixes,
        }
    }
    
    pub fn search(&self, pattern: &str) -> Vec<usize> {
        // Binary search in sorted suffix array
        let start = self.suffixes.binary_search_by(|&pos| {
            self.text[pos..].cmp(pattern)
        });
        
        // Find all matches (consecutive in sorted array)
        // ...
    }
}
```

**Example**:
```
Text: "banana"
Suffixes sorted:
  a
  ana
  anana
  banana
  na
  nana
  
Suffix array: [5, 3, 1, 0, 4, 2]
```

**Applications**:
- Substring search in large texts
- Longest repeated substring
- DNA sequence analysis

**Learning Outcomes**:
- ✅ Build suffix array
- ✅ Use binary search for pattern matching
- ✅ Find repeated substrings

---

### **Day 6: Regular Expression Engine** (Step 6)
**File**: `examples/step6_regex_engine.rs`

**Topics**:
- Simple regex pattern matching
- NFA (Non-deterministic Finite Automaton)
- Backtracking vs Thompson's construction
- Using Rust's `regex` crate

**Simple Regex Matcher**:
```rust
pub enum Pattern {
    Literal(char),
    Wildcard,           // . matches any char
    ZeroOrMore(char),   // a* matches "", "a", "aa", ...
    OneOrMore(char),    // a+ matches "a", "aa", ...
    Optional(char),     // a? matches "" or "a"
}

pub fn regex_match(text: &str, pattern: &[Pattern]) -> bool {
    match_recursive(text.chars().collect::<Vec<_>>().as_slice(), pattern)
}

fn match_recursive(text: &[char], pattern: &[Pattern]) -> bool {
    if pattern.is_empty() {
        return text.is_empty();
    }
    
    match pattern[0] {
        Pattern::Literal(ch) => {
            !text.is_empty() 
                && text[0] == ch 
                && match_recursive(&text[1..], &pattern[1..])
        }
        Pattern::Wildcard => {
            !text.is_empty() 
                && match_recursive(&text[1..], &pattern[1..])
        }
        Pattern::ZeroOrMore(ch) => {
            // Try matching zero occurrences
            match_recursive(text, &pattern[1..])
                // Or try matching one and continue
                || (!text.is_empty() 
                    && text[0] == ch 
                    && match_recursive(&text[1..], pattern))
        }
        // ... other patterns
    }
}
```

**Production Regex** (using `regex` crate):
```rust
use regex::Regex;

let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
if let Some(caps) = re.captures("Date: 2026-01-25") {
    println!("Year: {}", &caps[1]);   // 2026
    println!("Month: {}", &caps[2]);  // 01
    println!("Day: {}", &caps[3]);    // 25
}
```

**Learning Outcomes**:
- ✅ Implement simple regex engine
- ✅ Understand backtracking
- ✅ Use production regex crate for AoC parsing

---

### **Day 7: Text Processing & AoC Integration** (Step 7)
**File**: `examples/step7_text_processing.rs`

**Topics**:
- Parsing complex AoC inputs
- Tokenization and lexing
- String interning for performance
- Unicode handling (grapheme clusters)

**AoC Parser Pattern**:
```rust
pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }
    
    pub fn consume(&mut self, pattern: &str) -> bool {
        if self.input[self.pos..].starts_with(pattern) {
            self.pos += pattern.len();
            true
        } else {
            false
        }
    }
    
    pub fn parse_number(&mut self) -> Option<i32> {
        let start = self.pos;
        while self.pos < self.input.len() 
            && self.input.as_bytes()[self.pos].is_ascii_digit() 
        {
            self.pos += 1;
        }
        
        if self.pos > start {
            self.input[start..self.pos].parse().ok()
        } else {
            None
        }
    }
    
    pub fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() 
            && self.input.as_bytes()[self.pos].is_ascii_whitespace() 
        {
            self.pos += 1;
        }
    }
}
```

**String Interning** (for memory efficiency):
```rust
use std::collections::HashMap;

pub struct StringInterner {
    map: HashMap<String, usize>,
    strings: Vec<String>,
}

impl StringInterner {
    pub fn intern(&mut self, s: &str) -> usize {
        if let Some(&id) = self.map.get(s) {
            id
        } else {
            let id = self.strings.len();
            self.strings.push(s.to_string());
            self.map.insert(s.to_string(), id);
            id
        }
    }
    
    pub fn get(&self, id: usize) -> &str {
        &self.strings[id]
    }
}
```

**Unicode Handling**:
```rust
// Grapheme clusters (user-perceived characters)
use unicode_segmentation::UnicodeSegmentation;

let text = "नमस्ते";  // Hindi "hello"
// .chars() gives individual Unicode scalars (wrong!)
// .graphemes() gives user-perceived characters (correct!)

for grapheme in text.graphemes(true) {
    println!("{}", grapheme);
}
```

**Learning Outcomes**:
- ✅ Build parsers for AoC input formats
- ✅ Use string interning for memory efficiency
- ✅ Handle Unicode correctly (graphemes vs chars)

---

## 🔗 Integration Points

### **AoC Applications**
- **Input Parsing**: Almost every day requires parsing complex formats
- **Pattern Matching**: Grid-based pattern recognition
- **Text Processing**: String manipulation puzzles
- **Search**: Finding specific patterns in large inputs

### **Mission 5 (HashMap)**
- Trie can be built with HashMap children
- String interning uses HashMap

### **Mission 13 (Heaps)**
- Priority queue for Huffman coding (text compression)
- Top-K frequent words

### **Real-World Tools**
- Text editors (search, autocomplete)
- Databases (full-text search)
- Compilers (lexing, parsing)
- Bioinformatics (DNA sequence matching)

---

## 📊 Performance Characteristics

| Algorithm | Preprocessing | Search | Space | Best For |
|-----------|--------------|--------|-------|----------|
| **Naive** | O(1) | O(nm) | O(1) | Small patterns |
| **Rabin-Karp** | O(m) | O(n+m) avg | O(1) | Multiple patterns |
| **KMP** | O(m) | O(n+m) | O(m) | Single pattern (guaranteed) |
| **Boyer-Moore** | O(m+Σ) | O(n/m) avg | O(m+Σ) | Large alphabet |
| **Suffix Array** | O(n log n) | O(m log n) | O(n) | Many searches |
| **Trie** | O(L) | O(m) | O(ALPHABET_SIZE × N) | Prefix matching |

*(Σ = alphabet size, L = total length of all words)*

---

## 🎓 Learning Objectives

By completing this tutorial, you will:

### Trie Data Structures
- ✅ Implement trie with HashMap children
- ✅ Build autocomplete system
- ✅ Understand prefix matching advantages

### String Matching
- ✅ Implement naive, Rabin-Karp, KMP algorithms
- ✅ Compute failure function (LPS array)
- ✅ Choose appropriate algorithm for use case

### Advanced Structures
- ✅ Build suffix arrays
- ✅ Find repeated substrings
- ✅ Binary search in sorted suffixes

### Practical Skills
- ✅ Parse complex input formats
- ✅ Handle Unicode correctly
- ✅ Use production regex crate
- ✅ Optimize string storage with interning

---

## 🚀 Getting Started

```bash
cd tutorials/Mission15_tut

# Run examples in sequence
cargo run --example step1_trie_basics
cargo run --example step2_trie_applications
cargo run --example step3_string_matching_basic
cargo run --example step4_kmp_algorithm
cargo run --example step5_suffix_structures
cargo run --example step6_regex_engine
cargo run --example step7_text_processing

# Run all tests
cargo test

# Benchmarks comparing algorithms
cargo bench
```

---

## 📚 References

**Standard Library**:
- `str` methods: `chars()`, `bytes()`, `split()`, `contains()`
- Pattern trait for flexible matching

**External Crates** (for reference):
- `regex` - Production regex engine
- `aho-corasick` - Multiple pattern matching
- `unicode-segmentation` - Grapheme cluster iteration
- `bstr` - Byte string utilities

**AoC Problems Using String Algorithms**:
- Parsing structured inputs (most days!)
- Grid pattern matching
- Cryptographic puzzles
- Sequence analysis

---

## ⚠️ Common Pitfalls

**1. Unicode vs ASCII**:
```rust
// ❌ BAD - assumes ASCII
let s = "café";
let bytes = s.as_bytes();
bytes[3]  // Not 'é'! UTF-8 encoding is multi-byte

// ✅ GOOD - use chars or graphemes
s.chars().nth(3)  // Some('é')
```

**2. String Slicing**:
```rust
// ❌ BAD - panics if not on char boundary
let s = "hello";
let sub = &s[0..3];  // OK for ASCII, fails for multi-byte UTF-8

// ✅ GOOD - use char indices
let sub: String = s.chars().take(3).collect();
```

**3. Performance**:
```rust
// ❌ BAD - repeated allocations
let mut result = String::new();
for word in words {
    result = result + word;  // Allocates new string each time!
}

// ✅ GOOD - use push_str
let mut result = String::new();
for word in words {
    result.push_str(word);  // Mutates in place
}
```

---

### **Day 8: Boyer-Moore Algorithm** (Step 8)
**File**: `examples/step8_boyer_moore.rs`

**Topics**:
- Boyer-Moore bad character rule
- Good suffix rule
- Skip characters intelligently (better than KMP for large alphabets)
- Preprocessing tables

**Why Boyer-Moore?**
- **KMP**: Efficient for small alphabets (DNA: {A, C, G, T})
- **Boyer-Moore**: Efficient for large alphabets (text: a-z, A-Z, 0-9, ...)
- Can skip characters: O(n/m) best case (vs KMP's O(n))

**Bad Character Rule**:
```rust
pub struct BoyerMoore {
    pattern: Vec<char>,
    bad_char: HashMap<char, usize>,  // Last occurrence in pattern
}

impl BoyerMoore {
    pub fn new(pattern: &str) -> Self {
        let pattern: Vec<char> = pattern.chars().collect();
        let mut bad_char = HashMap::new();
        
        // Preprocess: last occurrence of each character
        for (i, &ch) in pattern.iter().enumerate() {
            bad_char.insert(ch, i);
        }
        
        Self { pattern, bad_char }
    }
    
    pub fn search(&self, text: &str) -> Vec<usize> {
        let text: Vec<char> = text.chars().collect();
        let mut matches = Vec::new();
        let m = self.pattern.len();
        let n = text.len();
        
        let mut s = 0;  // Shift of pattern relative to text
        while s <= n - m {
            let mut j = m - 1;  // Start from end of pattern
            
            // Match backwards
            while j > 0 && self.pattern[j] == text[s + j] {
                j -= 1;
            }
            
            if j == 0 && self.pattern[0] == text[s] {
                matches.push(s);
                s += m;  // Shift past match
            } else {
                // Bad character rule: skip based on mismatch
                let bad_char_shift = if let Some(&last) = self.bad_char.get(&text[s + j]) {
                    j.saturating_sub(last)
                } else {
                    j + 1  // Character not in pattern - skip whole pattern
                };
                s += bad_char_shift.max(1);
            }
        }
        
        matches
    }
}
```

**Example**:
```
Text:    "GCTTCTGCTACCTTTTGCGCGCGCGCGGAA"
Pattern: "GCGCG"

Mismatch at 'T':
  GCTTCTGCTACCTTTTGCGCGCGCGCGGAA
  GCGCG     ← 'T' not in pattern, skip 5!
       GCGCG  ← Next alignment
```

**Learning Outcomes**:
- ✅ Implement Boyer-Moore with bad character rule
- ✅ Understand when to use vs KMP
- ✅ Preprocess pattern for efficient skips

---

### **Day 9: Aho-Corasick Multiple Pattern Matching** (Step 9)
**File**: `examples/step9_aho_corasick.rs`

**Topics**:
- Search for multiple patterns simultaneously
- Failure links (like KMP for tries)
- O(n + m + z) where z = number of matches
- Applications: virus scanning, log analysis

**Why Aho-Corasick?**
- Search for K patterns in one pass: O(n + Σm_i + z)
- KMP K times: O(K·n + Σm_i)
- Boyer-Moore K times: O(K·n·m_i) worst case

**Structure**:
```rust
use std::collections::{HashMap, VecDeque};

#[derive(Default)]
struct ACNode {
    children: HashMap<char, usize>,
    failure: Option<usize>,  // Failure link (like KMP)
    output: Vec<usize>,      // Pattern IDs ending here
}

pub struct AhoCorasick {
    nodes: Vec<ACNode>,
    patterns: Vec<String>,
}

impl AhoCorasick {
    pub fn new(patterns: Vec<String>) -> Self {
        let mut ac = Self {
            nodes: vec![ACNode::default()],  // Root
            patterns,
        };
        
        // Build trie
        for (pattern_id, pattern) in ac.patterns.iter().enumerate() {
            let mut node_id = 0;
            for ch in pattern.chars() {
                node_id = *ac.nodes[node_id].children.entry(ch)
                    .or_insert_with(|| {
                        ac.nodes.push(ACNode::default());
                        ac.nodes.len() - 1
                    });
            }
            ac.nodes[node_id].output.push(pattern_id);
        }
        
        // Build failure links (BFS)
        ac.build_failure_links();
        ac
    }
    
    fn build_failure_links(&mut self) {
        let mut queue = VecDeque::new();
        
        // Root's children have failure = root
        for &child_id in self.nodes[0].children.values() {
            self.nodes[child_id].failure = Some(0);
            queue.push_back(child_id);
        }
        
        while let Some(node_id) = queue.pop_front() {
            for (&ch, &child_id) in &self.nodes[node_id].children {
                queue.push_back(child_id);
                
                // Find failure link
                let mut fail_id = self.nodes[node_id].failure.unwrap_or(0);
                while fail_id != 0 && !self.nodes[fail_id].children.contains_key(&ch) {
                    fail_id = self.nodes[fail_id].failure.unwrap_or(0);
                }
                
                self.nodes[child_id].failure = Some(
                    self.nodes[fail_id].children.get(&ch).copied().unwrap_or(0)
                );
                
                // Inherit output from failure link
                if let Some(fail) = self.nodes[child_id].failure {
                    let output = self.nodes[fail].output.clone();
                    self.nodes[child_id].output.extend(output);
                }
            }
        }
    }
    
    pub fn search(&self, text: &str) -> Vec<(usize, usize)> {  // (position, pattern_id)
        let mut matches = Vec::new();
        let mut node_id = 0;
        
        for (i, ch) in text.chars().enumerate() {
            // Follow failure links until we find a transition
            while node_id != 0 && !self.nodes[node_id].children.contains_key(&ch) {
                node_id = self.nodes[node_id].failure.unwrap_or(0);
            }
            
            node_id = self.nodes[node_id].children.get(&ch).copied().unwrap_or(0);
            
            // Report all matches at this position
            for &pattern_id in &self.nodes[node_id].output {
                matches.push((i - self.patterns[pattern_id].len() + 1, pattern_id));
            }
        }
        
        matches
    }
}
```

**Applications**:
- **Virus scanning**: Check file for 1000s of malware signatures
- **Log analysis**: Find multiple error patterns in logs
- **AoC parsing**: Extract all occurrences of multiple keywords

**Learning Outcomes**:
- ✅ Search for multiple patterns in single pass
- ✅ Build failure links for tries
- ✅ Understand when to use Aho-Corasick vs single-pattern algorithms

---

### **Day 10: Compression Algorithms** (Step 10)
**File**: `examples/step10_compression.rs`

**Topics**:
- Huffman coding (variable-length encoding)
- Run-length encoding (RLE)
- LZ77 (sliding window compression)
- Applications: File compression, network protocols

**Huffman Coding**:
```rust
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

#[derive(Debug, Clone)]
enum HuffmanTree {
    Leaf { ch: char, freq: usize },
    Internal { freq: usize, left: Box<HuffmanTree>, right: Box<HuffmanTree> },
}

impl HuffmanTree {
    fn freq(&self) -> usize {
        match self {
            HuffmanTree::Leaf { freq, .. } => *freq,
            HuffmanTree::Internal { freq, .. } => *freq,
        }
    }
}

pub fn build_huffman_tree(text: &str) -> HuffmanTree {
    // Count frequencies
    let mut freq_map = HashMap::new();
    for ch in text.chars() {
        *freq_map.entry(ch).or_insert(0) += 1;
    }
    
    // Build min-heap of nodes
    let mut heap: BinaryHeap<Reverse<(usize, HuffmanTree)>> = freq_map
        .into_iter()
        .map(|(ch, freq)| Reverse((freq, HuffmanTree::Leaf { ch, freq })))
        .collect();
    
    // Merge until one tree remains
    while heap.len() > 1 {
        let Reverse((freq1, tree1)) = heap.pop().unwrap();
        let Reverse((freq2, tree2)) = heap.pop().unwrap();
        
        let merged = HuffmanTree::Internal {
            freq: freq1 + freq2,
            left: Box::new(tree1),
            right: Box::new(tree2),
        };
        
        heap.push(Reverse((merged.freq(), merged)));
    }
    
    heap.pop().unwrap().0.1
}

pub fn build_codebook(tree: &HuffmanTree) -> HashMap<char, Vec<bool>> {
    let mut codebook = HashMap::new();
    build_codebook_rec(tree, Vec::new(), &mut codebook);
    codebook
}

fn build_codebook_rec(tree: &HuffmanTree, prefix: Vec<bool>, codebook: &mut HashMap<char, Vec<bool>>) {
    match tree {
        HuffmanTree::Leaf { ch, .. } => {
            codebook.insert(*ch, prefix);
        }
        HuffmanTree::Internal { left, right, .. } => {
            let mut left_prefix = prefix.clone();
            left_prefix.push(false);
            build_codebook_rec(left, left_prefix, codebook);
            
            let mut right_prefix = prefix;
            right_prefix.push(true);
            build_codebook_rec(right, right_prefix, codebook);
        }
    }
}
```

**Run-Length Encoding** (simple but effective for repetitive data):
```rust
pub fn rle_encode(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return String::new();
    }
    
    let mut result = String::new();
    let mut current = chars[0];
    let mut count = 1;
    
    for &ch in &chars[1..] {
        if ch == current {
            count += 1;
        } else {
            result.push_str(&format!("{}{}", count, current));
            current = ch;
            count = 1;
        }
    }
    result.push_str(&format!("{}{}", count, current));
    result
}

// "aaabbbccc" → "3a3b3c"
// "aaaaa" → "5a" (60% compression)
```

**Applications**:
- **AoC**: Compress large input files
- **Network**: Reduce bandwidth usage
- **Databases**: Column compression (repeated values)

**Learning Outcomes**:
- ✅ Build Huffman tree from frequencies
- ✅ Generate variable-length codes
- ✅ Understand compression trade-offs

---

## 🎯 Next Steps

After completing this tutorial:
- [ ] **Mission 15**: Formal V-Cycle string data structures
- [ ] Apply to AoC parsing (create parsing library)
- [ ] Build text search tool using tries
- [ ] Explore text compression (Huffman, LZ77)

---

*Zettelkasten Integration*:
- [[trie-data-structure]] - Prefix trees
- [[string-algorithms]] - Pattern matching
- [[mission-5]] - HashMap for trie nodes
- [[aoc-parsing-patterns]] - Input processing
- [[unicode-rust]] - Proper string handling

*Created: 2026-01-25*  
*Part of: Mission Track - Algorithms & Text Processing*
