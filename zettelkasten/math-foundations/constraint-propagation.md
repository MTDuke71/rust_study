# Constraint Propagation

**Field**: Computer Science / Computational Mathematics

**Prerequisites**: [[set-theory-fundamentals]], [[combinatorics-fundamentals]]

---

## 📐 Definition

**Constraint Propagation**: A technique for reducing the search space of a problem by systematically applying constraints to narrow the possible values of variables.

**Formal Concept**: 
- Variables: $V = \{v_1, v_2, ..., v_n\}$ each with domain $D_i$
- Constraints: $C = \{c_1, c_2, ..., c_m\}$ restricting valid combinations
- Propagation: Iteratively reduce domains $D_i$ by eliminating values inconsistent with constraints

**Intuition**: Instead of testing all combinations, progressively narrow possibilities by applying each constraint. A value eliminated for one variable may eliminate values for others (cascading effect).

---

## 🔑 Key Concepts/Techniques

### **Technique 1**: Range Splitting
- **Description**: Partition a continuous range into subranges based on comparison operators
- **Operation**: Split $[min, max]$ at threshold $t$:
  - Less than: $[min, t-1]$
  - Greater than: $[t+1, max]$
  - Greater or equal: $[t, max]$
  - Less or equal: $[min, t]$
- **Significance**: Transforms constraint into disjoint subproblems

**Example from AoC Day 19**:
```
Original range: x ∈ [1, 4000]
Constraint: x < 2000
Split:
  - Matching: x ∈ [1, 1999]
  - Non-matching: x ∈ [2000, 4000]
```

### **Technique 2**: Forward Propagation
- **Description**: Apply constraint immediately, reduce domain before recursing
- **Pattern**:
  1. Apply constraint → Split range
  2. Recurse on matching range with new constraint
  3. Continue on non-matching range with remaining constraints
- **Significance**: Prunes search space early

### **Technique 3**: Interval Arithmetic
- **Description**: Perform arithmetic operations on intervals/ranges
- **Operations**:
  - Addition: $[a,b] + [c,d] = [a+c, b+d]$
  - Multiplication: $[a,b] \times [c,d] = [\min, \max]$ of products
  - Size: $|[a,b]| = b - a + 1$ (inclusive)
- **Significance**: Work with ranges instead of individual values

### **Theorem 1**: Domain Consistency
- **Statement**: A constraint is domain-consistent if for every value in each variable's domain, there exists values for other variables that satisfy the constraint
- **Significance**: Removing inconsistent values doesn't lose solutions
- **Applications**: Constraint satisfaction problems (CSP), Sudoku solvers

### **Theorem 2**: Range Decomposition
- **Statement**: A range with $n$ constraints can be decomposed into at most $2^n$ disjoint subranges
- **Proof sketch**: Each constraint can split a range into 2 parts (matching/non-matching)
- **Significance**: Bounds worst-case complexity of range-based approaches

---

## 💻 Rust Implementations

### **AoC 2023 Day 19**: Aplenty - Range Splitting with Constraint Propagation
- **What**: Count valid part combinations by propagating range constraints through workflow rules
- **How it uses constraint propagation**:
  - Start with full range: all attributes in $[1, 4000]$
  - Apply each rule as constraint, split ranges
  - Propagate narrowed ranges to next workflow
  - Count combinations in accepted ranges (product rule)
- **Link**: [[workflow-pattern-matching]] | `advent_of_code/aoc2023/src/solver/day19.rs`

**Range Splitting Implementation**:
```rust
#[derive(Clone, Copy, Debug)]
struct Range {
    min: u64,
    max: u64,  // Inclusive
}

impl Range {
    /// Split range based on comparison: value < threshold or value > threshold
    fn split(&self, op: Op, threshold: u64) -> (Range, Range) {
        match op {
            Op::LessThan => {
                // Matching: [min, threshold-1]
                // Non-matching: [threshold, max]
                (
                    Range { min: self.min, max: threshold - 1 },
                    Range { min: threshold, max: self.max },
                )
            }
            Op::GreaterThan => {
                // Matching: [threshold+1, max]
                // Non-matching: [min, threshold]
                (
                    Range { min: threshold + 1, max: self.max },
                    Range { min: self.min, max: threshold },
                )
            }
        }
    }
    
    fn size(&self) -> u64 {
        if self.max < self.min {
            0  // Empty range
        } else {
            self.max - self.min + 1
        }
    }
    
    fn is_empty(&self) -> bool {
        self.max < self.min
    }
}
```

**Constraint Propagation Pattern**:
```rust
fn count_accepted(&self, workflow_name: &str, mut ranges: PartRange) -> u64 {
    // Terminal states
    if workflow_name == "A" {
        return ranges.combinations();  // Count with product rule
    }
    if workflow_name == "R" {
        return 0;  // Rejected, no valid combinations
    }
    
    let workflow = &self.workflows[workflow_name];
    let mut total = 0;
    
    // Apply each rule as a constraint
    for rule in &workflow.rules {
        match rule {
            Rule::Conditional { attr, op, value, dest } => {
                // Split range based on constraint
                let (matching, non_matching) = split_range(ranges, *attr, *op, *value);
                
                // FORWARD PROPAGATION:
                // 1. Recurse with NARROWED range (matching constraint)
                if !matching.is_empty() {
                    total += self.count_accepted(dest, matching);
                }
                
                // 2. CRITICAL: Continue with NON-matching range
                //    This is progressive narrowing!
                ranges = non_matching;
                
                // If non-matching is empty, no more possibilities
                if ranges.is_empty() {
                    break;
                }
            }
            Rule::Unconditional { dest } => {
                // No constraint, use current (narrowed) ranges
                total += self.count_accepted(dest, ranges);
            }
        }
    }
    
    total
}
```

**Key Insight - Progressive Narrowing**:
```
Initial: x ∈ [1, 4000]

Rule 1: x < 1500
  - Matching: x ∈ [1, 1499] → Process with this constraint
  - Continue: x ∈ [1500, 4000]

Rule 2 (on non-matching): x > 3000
  - Matching: x ∈ [3001, 4000] → Process with this constraint  
  - Continue: x ∈ [1500, 3000]

Rule 3 (on remaining): unconditional
  - Process: x ∈ [1500, 3000]

Result: Original range split into 3 disjoint subranges
```

**Performance Analysis**:
- **Without constraint propagation**: Test 256 trillion combinations individually
- **With constraint propagation**: Split into ~180 ranges, count mathematically
- **Complexity**: $O(\text{workflows} \times \text{rules})$ vs $O(4000^4)$
- **Runtime**: 190µs vs hours/days

### **Future: Sudoku Solver with Arc Consistency**
- **What**: Solve 9×9 Sudoku puzzles using constraint propagation
- **How it uses constraint propagation**:
  - Each cell has domain [1-9]
  - Constraints: Row unique, column unique, 3×3 box unique
  - Propagate: If cell assigned, remove value from peers' domains
  - Iterate until solved or no more reductions
- **Link**: TBD

---

## 📚 Code Examples

### **Range Splitting Helper**

```rust
/// Split a PartRange on a specific attribute
fn split_range(ranges: PartRange, attr: char, op: Op, value: u64) -> (PartRange, PartRange) {
    let mut matching = ranges;
    let mut non_matching = ranges;
    
    match attr {
        'x' => {
            let (m, nm) = ranges.x.split(op, value);
            matching.x = m;
            non_matching.x = nm;
        }
        'm' => {
            let (m, nm) = ranges.m.split(op, value);
            matching.m = m;
            non_matching.m = nm;
        }
        'a' => {
            let (m, nm) = ranges.a.split(op, value);
            matching.a = m;
            non_matching.a = nm;
        }
        's' => {
            let (m, nm) = ranges.s.split(op, value);
            matching.s = m;
            non_matching.s = nm;
        }
        _ => panic!("Invalid attribute: {}", attr),
    }
    
    (matching, non_matching)
}
```

### **Interval Arithmetic Operations**

```rust
impl Range {
    /// Check if value is in range
    fn contains(&self, value: u64) -> bool {
        value >= self.min && value <= self.max
    }
    
    /// Intersection of two ranges
    fn intersect(&self, other: &Range) -> Range {
        Range {
            min: self.min.max(other.min),
            max: self.max.min(other.max),
        }
    }
    
    /// Union (assumes ranges overlap or adjacent)
    fn union(&self, other: &Range) -> Option<Range> {
        if self.max + 1 < other.min || other.max + 1 < self.min {
            None  // Disjoint ranges
        } else {
            Some(Range {
                min: self.min.min(other.min),
                max: self.max.max(other.max),
            })
        }
    }
}

// Example: Constraint intersection
let constraint1 = Range { min: 1, max: 3000 };    // x < 3001
let constraint2 = Range { min: 500, max: 4000 };  // x > 499
let valid = constraint1.intersect(&constraint2);   // [500, 3000]
assert_eq!(valid.size(), 2501);
```

### **Multi-Dimensional Range Constraints**

```rust
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    /// Check if ranges are valid (non-empty)
    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.m.is_empty() || self.a.is_empty() || self.s.is_empty()
    }
    
    /// Count valid combinations (product rule from combinatorics)
    fn combinations(&self) -> u64 {
        if self.is_empty() {
            0
        } else {
            self.x.size() * self.m.size() * self.a.size() * self.s.size()
        }
    }
    
    /// Apply constraint to single attribute
    fn constrain(&mut self, attr: char, range: Range) {
        match attr {
            'x' => self.x = self.x.intersect(&range),
            'm' => self.m = self.m.intersect(&range),
            'a' => self.a = self.a.intersect(&range),
            's' => self.s = self.s.intersect(&range),
            _ => {}
        }
    }
}
```

**Explanation**: Multi-dimensional constraints are applied independently to each dimension. The range remains valid as long as all dimensions are non-empty. Combinations are counted using the product rule from combinatorics.

---

## 🌳 Related Concepts

**Prerequisites**:
- [[set-theory-fundamentals]] - Intersection, union of ranges
- [[combinatorics-fundamentals]] - Product rule for counting range combinations

**Related**:
- [[dynamic-programming-theory]] - Memoization can optimize constraint propagation
- [[graph-theory-fundamentals]] - CSP can be modeled as constraint graphs
- [[complexity-theory]] - Analysis of pruning effectiveness

**Applications**:
- [[workflow-pattern-matching]] - Range splitting for workflow rules
- **CSP Solvers** - Sudoku, N-Queens, scheduling problems
- **Symbolic Execution** - Track variable constraints in program analysis
- **Interval Analysis** - Numerical methods with uncertainty bounds

---

## 📖 Resources

- [Constraint Satisfaction Problems - Wikipedia](https://en.wikipedia.org/wiki/Constraint_satisfaction_problem)
- [Arc Consistency - Stanford CS](http://ai.stanford.edu/~nilsson/OnlinePubs-Nils/PublishedPapers/arc.pdf)
- [Interval Arithmetic - Wikipedia](https://en.wikipedia.org/wiki/Interval_arithmetic)
- [Constraint Propagation - AI Modern Approach Ch 6](http://aima.cs.berkeley.edu/)

---

## 💡 Key Insights for Programmers

### **Range Splitting Pattern**
```rust
// PATTERN: Split ranges and recurse on disjoint subproblems
fn process(ranges: Ranges, constraints: &[Constraint]) -> Result {
    if constraints.is_empty() {
        return ranges.count();  // Base case: count combinations
    }
    
    let (matching, non_matching) = ranges.split(constraints[0]);
    
    // Recurse on both partitions with remaining constraints
    process(matching, &constraints[1..]) + process(non_matching, &constraints[1..])
}
```

### **Progressive Narrowing (Day 19 Key Pattern)**
```rust
// CRITICAL: Use non-matching range for next rule
let mut ranges = initial_ranges;
for rule in rules {
    let (matching, non_matching) = ranges.split(rule);
    process(matching);        // Handle matching case
    ranges = non_matching;    // CONTINUE with non-matching!
}
```

**Why Progressive Narrowing Works**:
- Each rule applies to parts that DIDN'T match previous rules
- Like if/else chain: only reaches later rules if earlier ones failed
- Mathematically: Partition range into disjoint subranges

### **When to Use Constraint Propagation**
- ✅ **Large discrete search space**: Too many combinations to enumerate
- ✅ **Independent variables**: Can split dimensions separately
- ✅ **Comparison constraints**: <, >, <=, >= naturally split ranges
- ✅ **Count solutions**: Need total, not explicit enumeration
- ❌ **Complex interdependencies**: Variables deeply coupled
- ❌ **Continuous optimization**: Better suited for calculus/optimization

### **Complexity Reduction**
```
Without propagation: O(|D₁| × |D₂| × ... × |Dₙ|)  - Cartesian product
With propagation: O(constraints × branching_factor) - Much smaller!

Day 19 example:
  Without: O(4000⁴) = 256 trillion
  With: O(workflows × rules) ≈ 180 operations
  Speedup: ~10¹² reduction!
```

---

## 🔬 Advanced Topics

### **Arc Consistency (AC-3 Algorithm)**
Constraint propagation algorithm for CSPs that removes values from domains that cannot be part of any solution.

### **Symbolic Execution**
Track constraints on variables during program execution to reason about all possible paths.

### **Interval Analysis**
Arithmetic on intervals for numerical methods with bounded error.

### **Abstract Interpretation**
Static analysis technique using constraint domains to prove program properties.

---

*Tags: #mathematics #computer-science #constraints #csp #interval-arithmetic #range-splitting #optimization #aoc2023 #day19 #forward-propagation*

*Created*: 2026-01-19  
*Last Updated*: 2026-01-19  
*Implementations*: 1 (AoC 2023 Day 19)

*Links: [[zettel-index]] | [[math-foundations/README]] | [[set-theory-fundamentals]] | [[combinatorics-fundamentals]] | [[workflow-pattern-matching]] | [[dynamic-programming-theory]]*
