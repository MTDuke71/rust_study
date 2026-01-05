# Set Theory Fundamentals

**Field**: Discrete Mathematics

**Prerequisites**: None (foundational concept)

---

## 📐 Definition

**Set**: A collection of distinct objects, called *elements* or *members*.

**Formal Notation**:
- Set: $S = \{a, b, c\}$
- Membership: $x \in S$ (x is in S), $x \notin S$ (x is not in S)
- Empty set: $\emptyset$ or $\{\}$
- Cardinality: $|S|$ = number of elements in S

**Intuition**: A set is like a bag containing unique items—no duplicates allowed. Order doesn't matter: $\{1, 2, 3\} = \{3, 1, 2\}$.

---

## 🔑 Key Properties/Theorems

### **Property 1**: Uniqueness
- **Statement**: Each element appears at most once in a set
- **Significance**: Adding an existing element doesn't change the set
- **Rust**: `HashSet::insert()` returns `false` if element already exists

### **Property 2**: Unordered
- **Statement**: Sets have no inherent ordering
- **Significance**: $\{1, 2\} = \{2, 1\}$
- **Rust**: HashSet iteration order is arbitrary (hash-dependent)

### **Set Operations**

#### **Union**: $A \cup B$
Elements in A **or** B (or both)
- $\{1, 2\} \cup \{2, 3\} = \{1, 2, 3\}$
- **Rust**: `set_a.union(&set_b)` or `set_a | &set_b`

#### **Intersection**: $A \cap B$
Elements in **both** A **and** B
- $\{1, 2, 3\} \cap \{2, 3, 4\} = \{2, 3\}$
- **Rust**: `set_a.intersection(&set_b)` or `set_a & &set_b`

#### **Difference**: $A \setminus B$ or $A - B$
Elements in A but **not** in B
- $\{1, 2, 3\} \setminus \{2, 4\} = \{1, 3\}$
- **Rust**: `set_a.difference(&set_b)` or `set_a - &set_b`

#### **Symmetric Difference**: $A \triangle B$
Elements in A **or** B but **not** in both
- $\{1, 2, 3\} \triangle \{2, 3, 4\} = \{1, 4\}$
- **Rust**: `set_a.symmetric_difference(&set_b)` or `set_a ^ &set_b`

#### **Subset**: $A \subseteq B$
All elements of A are in B
- $\{1, 2\} \subseteq \{1, 2, 3\}$ is true
- **Rust**: `set_a.is_subset(&set_b)`

#### **Superset**: $A \supseteq B$
All elements of B are in A
- $\{1, 2, 3\} \supseteq \{2, 3\}$ is true
- **Rust**: `set_a.is_superset(&set_b)`

### **Theorem 1**: Cardinality of Union
- **Statement**: $|A \cup B| = |A| + |B| - |A \cap B|$
- **Proof sketch**: Count elements in A, add elements in B, subtract overlap (counted twice)
- **Applications**: Inclusion-exclusion principle, counting problems

### **Theorem 2**: De Morgan's Laws
- **Statement**: 
  - $(A \cup B)^c = A^c \cap B^c$
  - $(A \cap B)^c = A^c \cup B^c$
- **Significance**: Complement of union is intersection of complements (and vice versa)
- **Applications**: Boolean algebra, logic simplification

---

## 💻 Rust Implementations

### **AoC 2023 Day 4**: Scratchcards - Set Membership Testing
- **What**: Find matching numbers between winning numbers and player numbers
- **How it uses set theory**: 
  - Build set of winning numbers: $W = \{w_1, w_2, ..., w_n\}$
  - For each player number $p$, test if $p \in W$
  - Count matches: $|P \cap W|$ (cardinality of intersection)
- **Link**: [[2026-01-04]] or `advent_of_code/aoc2023/src/solver/day04.rs`

**Code from Day 4 solution**:
```rust
use std::collections::HashSet;

// Build set W (winning numbers)
let winning: HashSet<u32> = winning_numbers
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect();

// Count intersection: |our_numbers ∩ winning|
let matches = our_numbers
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .filter(|n| winning.contains(n))  // Test membership: n ∈ W
    .count();  // Cardinality of intersection
```

**Mathematical Translation**:
- `winning.contains(n)` = membership test $n \in W$
- `.filter(|n| winning.contains(n))` = intersection $\text{our\_numbers} \cap W$
- `.count()` = cardinality $|\text{our\_numbers} \cap W|$

**Performance**: 
- Membership test: $O(1)$ average (hash-based)
- vs. Linear search: $O(n)$ per test
- Total: $O(m + n)$ vs $O(m \times n)$ for nested loops

### **Mission 5**: HashMap/HashSet Implementation
- **What**: Implement HashSet with collision handling
- **How it uses set theory**: Implements mathematical set operations with hash table
- **Key operations**:
  - `insert(x)`: Add element to set (ensures uniqueness)
  - `contains(x)`: Membership testing ($x \in S$)
  - `remove(x)`: Remove element from set
  - Set operations: union, intersection, difference
- **Link**: [[mission-5]]

**HashSet Properties**:
```rust
use std::collections::HashSet;

let mut set = HashSet::new();

// Uniqueness property
assert_eq!(set.insert(1), true);   // First insert succeeds
assert_eq!(set.insert(1), false);  // Duplicate returns false
assert_eq!(set.len(), 1);          // Still only 1 element

// Membership testing O(1) average
assert!(set.contains(&1));         // 1 ∈ S
assert!(!set.contains(&2));        // 2 ∉ S
```

---

## 📚 Code Examples

### **Basic Set Operations in Rust**

```rust
use std::collections::HashSet;

// Create sets
let a: HashSet<i32> = [1, 2, 3, 4].iter().copied().collect();
let b: HashSet<i32> = [3, 4, 5, 6].iter().copied().collect();

// Union: A ∪ B = {1, 2, 3, 4, 5, 6}
let union: HashSet<_> = a.union(&b).copied().collect();
assert_eq!(union.len(), 6);

// Intersection: A ∩ B = {3, 4}
let intersection: HashSet<_> = a.intersection(&b).copied().collect();
assert_eq!(intersection, HashSet::from([3, 4]));

// Difference: A - B = {1, 2}
let difference: HashSet<_> = a.difference(&b).copied().collect();
assert_eq!(difference, HashSet::from([1, 2]));

// Symmetric difference: A △ B = {1, 2, 5, 6}
let sym_diff: HashSet<_> = a.symmetric_difference(&b).copied().collect();
assert_eq!(sym_diff.len(), 4);

// Subset testing
let small = HashSet::from([1, 2]);
assert!(small.is_subset(&a));      // {1, 2} ⊆ {1, 2, 3, 4}
assert!(a.is_superset(&small));    // {1, 2, 3, 4} ⊇ {1, 2}

// Disjoint sets (no overlap)
let c = HashSet::from([7, 8, 9]);
assert!(a.is_disjoint(&c));        // A ∩ C = ∅
```

### **Practical Pattern: Membership Testing**

```rust
// ❌ Slow: Linear search O(m × n)
fn count_matches_slow(candidates: &[i32], valid: &[i32]) -> usize {
    candidates.iter()
        .filter(|&n| valid.contains(n))  // O(n) per test!
        .count()
}

// ✅ Fast: Set-based O(m + n)
fn count_matches_fast(candidates: &[i32], valid: &[i32]) -> usize {
    let valid_set: HashSet<_> = valid.iter().collect();
    candidates.iter()
        .filter(|&n| valid_set.contains(n))  // O(1) average per test!
        .count()
}
```

**From AoC Day 4 Analysis**:
- Nested loops (no set): ~500µs (estimated)
- HashSet approach: 176.6µs (measured)
- **Speedup**: ~3× faster

### **Advanced: Custom Types in Sets**

```rust
use std::collections::HashSet;

// Must implement Hash + Eq for HashSet
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

let mut visited = HashSet::new();
visited.insert(Point { x: 0, y: 0 });
visited.insert(Point { x: 1, y: 1 });

// Duplicate insertion has no effect (set property)
assert!(!visited.insert(Point { x: 0, y: 0 }));
assert_eq!(visited.len(), 2);

// Membership testing for path tracking
let current = Point { x: 1, y: 1 };
if visited.contains(&current) {
    println!("Already visited this location!");
}
```

---

## 🌳 Related Concepts

### **Prerequisites**:
- None (set theory is foundational)

### **Related Mathematical Concepts**:
- [[number-theory-basics]] - Sets of integers with special properties (primes, etc.)
- [[graph-theory-fundamentals]] - Graphs use sets of vertices and edges
- [[combinatorics-basics]] - Counting subsets, permutations of sets
- [[probability-theory]] - Sample spaces as sets of outcomes

### **Related Rust Concepts**:
- [[common-traits-pattern]] - Hash + Eq requirements for HashSet elements
- [[mission-5]] - Hash function implementation details
- [[complexity-theory]] - Time complexity of set operations

### **Applications**:
- **Uniqueness checking**: Remove duplicates from collection
- **Membership testing**: Fast lookup (visited nodes, valid states)
- **Set algebra**: Combine/compare collections efficiently
- **Graph algorithms**: Track visited vertices in BFS/DFS
- **Deduplication**: Filter unique items from stream

---

## 🎯 Common Patterns in AoC/Competitive Programming

### **Pattern 1: Visited Tracking**
```rust
let mut visited = HashSet::new();
for node in graph.nodes() {
    if !visited.contains(&node) {
        visited.insert(node);
        explore(node, &mut visited);
    }
}
```

### **Pattern 2: Fast Lookup Tables**
```rust
// Build set once, query many times
let valid_positions: HashSet<_> = calculate_valid_positions();

for candidate in candidates {
    if valid_positions.contains(&candidate) {
        // Process valid position
    }
}
```

### **Pattern 3: Finding Common Elements**
```rust
// Intersection of multiple sets
let common: HashSet<_> = sets.iter()
    .skip(1)
    .fold(sets[0].clone(), |acc, set| {
        acc.intersection(set).copied().collect()
    });
```

### **Pattern 4: Counting Unique Elements**
```rust
// Cardinality of set (number of unique elements)
let unique_count = items.iter().collect::<HashSet<_>>().len();
```

---

## 📖 Resources

### **Mathematical Foundations**:
- [Set Theory (Wikipedia)](https://en.wikipedia.org/wiki/Set_theory)
- [Discrete Mathematics - Sets Chapter](https://en.wikibooks.org/wiki/Discrete_Mathematics/Set_theory)
- [Venn Diagrams](https://en.wikipedia.org/wiki/Venn_diagram) - Visual representation of sets

### **Rust Documentation**:
- [`std::collections::HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
- [`std::collections::BTreeSet`](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html) - Ordered alternative

### **Performance**:
- Hash-based sets: O(1) average operations
- Tree-based sets: O(log n) guaranteed operations
- Trade-off: Speed vs ordering guarantees

---

## 💡 Key Insights

**Mathematical Elegance**:
- Set theory provides clean abstractions for membership and uniqueness
- Operations (union, intersection) compose naturally
- Cardinality gives counting power

**Rust Implementation**:
- `HashSet<T>` embodies mathematical set properties
- Requires `T: Hash + Eq` (mathematical consistency)
- Zero-cost abstraction: Set operations compile to efficient code

**Problem-Solving Power**:
- Transform "find common items" → set intersection
- Transform "remove duplicates" → collect into set
- Transform "fast lookup" → membership testing
- AoC Day 4: Set membership **3× faster** than linear search

**From Day 4 Experience**:
> "Data structure choice matters more than algorithm cleverness for membership testing. Mission 5 taught HashSet, Day 4 proved why it's essential."

---

## 📊 Complexity Reference

| Operation | HashSet | Vec (linear) | BTreeSet |
|-----------|---------|--------------|----------|
| Insert | O(1) avg | O(1) amortized | O(log n) |
| Contains | O(1) avg | O(n) | O(log n) |
| Remove | O(1) avg | O(n) | O(log n) |
| Union | O(n + m) | O(n × m) | O(n + m) |
| Intersection | O(min(n,m)) | O(n × m) | O(n + m) |

**When to use HashSet**:
- ✅ Need fast membership testing
- ✅ Need to remove duplicates
- ✅ Don't care about order
- ✅ Elements implement Hash + Eq

**When to use BTreeSet**:
- Need ordered iteration
- Need range queries
- Deterministic behavior (no hash randomization)

---

*Tags: #mathematics #discrete-math #set-theory #hash-set #collections #aoc2023-day4 #mission5*

*Created*: 2026-01-04  
*Last Updated*: 2026-01-04  
*Implementations*: 2 (AoC Day 4, Mission 5)
