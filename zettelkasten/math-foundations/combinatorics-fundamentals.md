# Combinatorics Fundamentals

**Field**: Discrete Mathematics

**Prerequisites**: [[set-theory-fundamentals]]

---

## 📐 Definition

**Combinatorics**: The mathematical study of counting, arrangement, and combination of objects.

**Formal Foundation**: Given a finite set of objects, combinatorics answers questions about:
- How many ways to arrange them (permutations)
- How many ways to select them (combinations)
- How many satisfy certain constraints (counting principles)

**Intuition**: Instead of enumerating all possibilities, use mathematical formulas to count them efficiently. Essential when enumeration is computationally infeasible.

---

## 🔑 Key Principles/Theorems

### **Principle 1**: Product Rule (Multiplication Principle)
- **Statement**: If there are $m$ ways to do task A and $n$ ways to do task B, then there are $m \times n$ ways to do both tasks.
- **Generalization**: For $k$ independent tasks with $n_1, n_2, ..., n_k$ ways each, total ways = $n_1 \times n_2 \times ... \times n_k$
- **Significance**: Fundamental for counting without enumeration

**Example - AoC Day 19 Part 2**:
```
Part has 4 attributes (x, m, a, s), each in range [1, 4000]
Total combinations = 4000 × 4000 × 4000 × 4000 = 256,000,000,000,000
```

### **Principle 2**: Addition Rule (Sum Principle)
- **Statement**: If there are $m$ ways to do task A and $n$ ways to do task B, and these tasks are **mutually exclusive**, then there are $m + n$ ways to do either task.
- **Significance**: Count disjoint alternatives

**Example - Workflow Destinations**:
```
Parts can be: Accepted OR Rejected OR sent to another Workflow
Total outcomes = count(Accept) + count(Reject) + count(Workflow transitions)
```

### **Theorem 1**: Permutations (Ordered Arrangements)
- **Statement**: Number of ways to arrange $n$ distinct objects = $n! = n \times (n-1) \times ... \times 2 \times 1$
- **Partial permutations**: Arrange $r$ objects from $n$ = $P(n,r) = \frac{n!}{(n-r)!}$
- **Applications**: Ordering problems, sequence generation

### **Theorem 2**: Combinations (Unordered Selections)
- **Statement**: Number of ways to choose $r$ objects from $n$ (order doesn't matter):
$$C(n,r) = \binom{n}{r} = \frac{n!}{r!(n-r)!}$$
- **Pascal's identity**: $\binom{n}{r} = \binom{n-1}{r-1} + \binom{n-1}{r}$
- **Applications**: Subset selection, lottery problems

### **Theorem 3**: Inclusion-Exclusion Principle
- **Statement**: For overlapping sets A and B:
$$|A \cup B| = |A| + |B| - |A \cap B|$$
- **Generalization**: For $n$ sets, alternately add and subtract intersections
- **Applications**: Counting with constraints, deduplication

---

## 💻 Rust Implementations

### **AoC 2023 Day 19**: Aplenty - Product Rule for Counting Without Enumeration
- **What**: Count accepted part combinations across 4 attributes with constraints
- **How it uses combinatorics**: 
  - Each attribute has independent range: $[x_{min}, x_{max}]$
  - Valid combinations = product of range sizes
  - **Critical insight**: Count 256 trillion combinations without enumerating them!
- **Link**: [[workflow-pattern-matching]] | `advent_of_code/aoc2023/src/solver/day19.rs`

**Mathematical Translation**:
```rust
// Part can have values in 4 independent ranges
struct PartRange {
    x: Range,  // e.g., [1, 2500]
    m: Range,  // e.g., [1000, 4000]
    a: Range,  // e.g., [1, 1500]
    s: Range,  // e.g., [2000, 3000]
}

// Product rule: total = |x_range| × |m_range| × |a_range| × |s_range|
fn combinations(&self) -> u64 {
    self.x.size() * self.m.size() * self.a.size() * self.s.size()
}
```

**Performance Impact**:
- **Enumeration approach**: Generate and test 256 trillion combinations → infeasible
- **Combinatorial approach**: Compute products of ~180 ranges → 190µs
- **Complexity reduction**: $O(4000^4)$ → $O(\text{workflows} \times \text{rules})$

**From Day 19 Solution**:
```rust
fn count_accepted(&self, workflow_name: &str, ranges: PartRange) -> u64 {
    // Base case: reached Accept state
    if workflow_name == "A" {
        return ranges.combinations();  // Product rule!
    }
    
    // Recursively split ranges and count
    // Each split multiplies independent range sizes
    let mut count = 0;
    for rule in &workflow.rules {
        match rule {
            Rule::Conditional { attr, op, value, dest } => {
                let (matching, non_matching) = split_range(ranges, *attr, *op, *value);
                count += self.count_accepted(dest, matching);  // Product rule
                ranges = non_matching;  // Continue with remainder
            }
            Rule::Unconditional { dest } => {
                count += self.count_accepted(dest, ranges);
            }
        }
    }
    count
}
```

**Why This Works**:
1. **Independence**: Attributes (x, m, a, s) are independent variables
2. **Product Rule**: Independent choices multiply
3. **No enumeration**: Count directly from range arithmetic
4. **Optimization**: Exponential problem becomes graph traversal

### **Project Euler P15**: Lattice Paths - Central Binomial Coefficient
- **What**: Count paths through a 20x20 grid moving only right and down
- **How it uses combinatorics**:
  - Path = sequence of 20 R's and 20 D's = 40 moves total
  - Count = C(40, 20) = "choose which 20 of 40 steps are right"
  - This is the **central binomial coefficient** (middle of Pascal's triangle row 40)
- **Integrator win**: One-liner using shared `combinatorics::binomial(2*n, n)`
- **Link**: [[project-euler-p015]] | `project_euler/src/problems/p015.rs`

### **Project Euler P24**: Lexicographic Permutations - Factoradic Number System
- **What**: Find the 1,000,000th lexicographic permutation of digits 0–9
- **How it uses combinatorics**:
  - 10 digits have 10! = 3,628,800 total permutations (Theorem 1)
  - Each "first digit" block contains (n-1)! permutations
  - **Factoradic decomposition**: Repeatedly divide by (n-1)! to select each digit position
  - Direct O(n²) computation instead of generating all permutations
- **Key insight**: The factoradic number system creates a bijection between [0, n!) and permutations
- **Link**: [[project-euler-p024]] | `project_euler/src/problems/p024.rs`

### **Project Euler P32**: Pandigital Products - Digit-Count Constraint Reasoning
- **What**: Find all 1–9 pandigital identities a × b = c (each digit used once across a, b, c)
- **How it uses combinatorics**:
  - Pandigital constraint: a + b + c = 9 (where each variable is a digit count)
  - Multiplication rule: a-digit × b-digit yields (a+b−1) or (a+b) digits
  - Combining: only a+b = 5, c = 4 has integer solutions → shapes 1×4=4 and 2×3=4
- **Key insight**: Constraint reasoning on digit counts eliminates entire shape classes without enumeration
- **Link**: [[project-euler-p032]] | `project_euler/src/problems/p032.rs`

### **Future AoC**: Subset Selection Problems
- **What**: Choose $r$ items from $n$ candidates
- **How it uses combinatorics**: Combination formula $\binom{n}{r}$
- **Implementation**: Dynamic programming or mathematical formula
- **Link**: TBD

---

## 📚 Code Examples

### **Product Rule in Rust**

```rust
/// Count total combinations using product rule
/// 
/// # Mathematical Foundation
/// Product rule: Independent choices multiply
/// Total ways = n₁ × n₂ × ... × nₖ
fn count_combinations(dimensions: &[usize]) -> usize {
    dimensions.iter().product()
}

// Example: 3 shirt colors × 4 pants colors × 2 shoes
let outfits = count_combinations(&[3, 4, 2]);
assert_eq!(outfits, 24);  // 3 × 4 × 2 = 24
```

### **Combination Formula**

```rust
/// Calculate binomial coefficient C(n, r) = n! / (r! × (n-r)!)
/// Number of ways to choose r items from n (order doesn't matter)
fn binomial(n: u64, r: u64) -> u64 {
    if r > n {
        return 0;
    }
    if r == 0 || r == n {
        return 1;
    }
    
    // Optimize: C(n,r) = C(n, n-r), use smaller r
    let r = r.min(n - r);
    
    // Calculate iteratively to avoid overflow from factorials
    // C(n,r) = (n × (n-1) × ... × (n-r+1)) / (r × (r-1) × ... × 1)
    (1..=r).fold(1, |acc, i| acc * (n - i + 1) / i)
}

assert_eq!(binomial(5, 2), 10);  // Choose 2 from 5: C(5,2) = 10
assert_eq!(binomial(10, 3), 120); // Choose 3 from 10: C(10,3) = 120
```

### **Range-Based Counting (Day 19 Pattern)**

```rust
/// Count combinations in multi-dimensional range space
#[derive(Clone, Copy)]
struct Range {
    min: u64,
    max: u64,  // Inclusive
}

impl Range {
    fn size(&self) -> u64 {
        if self.max < self.min {
            0  // Empty range
        } else {
            self.max - self.min + 1  // Inclusive range
        }
    }
    
    fn is_empty(&self) -> bool {
        self.max < self.min
    }
}

struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    /// Product rule: multiply independent dimensions
    fn combinations(&self) -> u64 {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }
}

// Example: Count combinations in constrained space
let ranges = PartRange {
    x: Range { min: 1, max: 1000 },    // 1000 values
    m: Range { min: 500, max: 1500 },  // 1001 values
    a: Range { min: 1, max: 100 },     // 100 values
    s: Range { min: 1, max: 10 },      // 10 values
};

// Total = 1000 × 1001 × 100 × 10 = 1,001,000,000
let total = ranges.combinations();
```

**Explanation**: Each attribute can vary independently. If x has 1000 possible values and m has 1001, there are 1000 × 1001 combinations of (x, m) pairs. The product rule extends to all 4 dimensions.

---

## 🌳 Related Concepts

**Prerequisites**:
- [[set-theory-fundamentals]] - Sets, cardinality, operations

**Related**:
- [[constraint-propagation]] - Range splitting uses combinatorics for counting
- [[dynamic-programming-theory]] - DP often uses combinatorial counting
- [[probability-theory]] - Probability builds on combinatorics

**Applications**:
- [[workflow-pattern-matching]] - Product rule for counting workflow paths
- [[graph-theory-fundamentals]] - Counting paths in graphs
- [[number-theory-basics]] - Prime factorization uses combinatorics

---

## 📖 Resources

- [Combinatorics - Wikipedia](https://en.wikipedia.org/wiki/Combinatorics)
- [Counting Principles - Khan Academy](https://www.khanacademy.org/math/precalculus/x9e81a4f98389efdf:prob-comb)
- [Binomial Coefficients - MathWorld](https://mathworld.wolfram.com/BinomialCoefficient.html)
- [Product Rule - Brilliant.org](https://brilliant.org/wiki/rule-of-product/)

---

## 💡 Key Insights for Programmers

### **When Enumeration is Infeasible**
```
Problem: Count valid states in 4D space with 4000 values per dimension
Enumeration: 256 trillion iterations → impossible
Combinatorics: Product of range sizes → instant
```

### **Independence is Critical**
- Product rule requires **independent** choices
- If choices affect each other, use constraint propagation
- Day 19: Workflow rules create dependencies → split ranges + recurse

### **Avoid Factorial Overflow**
- Don't compute $n!$ directly for large $n$
- Use iterative methods, cancellation, or DP
- Example: `binomial()` multiplies and divides incrementally

### **Counting ≠ Enumerating**
- Counting: How many? (mathematical formula)
- Enumerating: What are they? (generate all)
- Often only the count is needed → massive optimization

---

*Tags: #mathematics #combinatorics #discrete-math #counting #product-rule #permutations #combinations #aoc2023 #day19*

*Created*: 2026-01-19
*Last Updated*: 2026-02-26
*Implementations*: 4 (AoC 2023 Day 19, Project Euler P15, Project Euler P24, Project Euler P32)

*Links: [[zettel-index]] | [[math-foundations/README]] | [[set-theory-fundamentals]] | [[constraint-propagation]] | [[workflow-pattern-matching]] | [[dynamic-programming-theory]] | [[project-euler-p015]] | [[project-euler-p024]] | [[project-euler-p032]]*
