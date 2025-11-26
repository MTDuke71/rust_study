# Spaced Repetition Cards - Rust Learning System

*Created: 2025-10-10*  
*Navigation: [[zettel-index]] | [[developer-learning-habits]] | [[MONTHLY_CALENDAR]]*

---

## 🎯 **Purpose**

This file tracks spaced repetition cards for systematic Rust concept review. Cards use evidence-based spacing intervals: **1→3→7→21→60 days** to optimize long-term retention.

**Review Protocol**: Check this file daily for cards due today. Practice retrieval before looking at answers.

---

## 📅 **Review Schedule**

### **Due Today (2025-10-10)**

- [ ] `RUST-001`: Explain ownership rules (3 rules from memory)
- [ ] `RUST-015`: Implement basic HashMap get() operation
- [ ] `ALG-008`: Binary search invariant explanation

### **Due 2025-10-12**  

- [ ] `RUST-003`: String vs &str differences with code examples
- [ ] `RUST-012`: Iterator lazy evaluation benefits

### **Due 2025-10-15**

- [ ] `RUST-007`: Lifetime elision rules (3 rules)
- [ ] `ALG-004`: BFS vs DFS when to use each

---

## 🃏 **Card Templates**

### **Concept Recall Cards**

```markdown
## RUST-[ID]: [Topic Name]

**Prompt**: [Question requiring explanation/recall]

**Answer**: [Hidden until after attempt]
- Key point 1
- Key point 2  
- Code example if applicable

**Tags**: #rust #[subtopic] #[difficulty]
**Next Review**: [Date + spacing interval]
```

### **Code Implementation Cards**

```markdown  
## ALG-[ID]: [Algorithm/Pattern Name]

**Prompt**: Implement [specific function] from memory

**Answer**: [Hidden until after attempt]
```rust
// Clean implementation with comments
```

**Gotchas**: [Common mistakes to avoid]
**Tags**: #algorithms #[topic] #[difficulty]
**Next Review**: [Date + spacing interval]

```

---

## 📚 **Active Cards**

### **RUST-001: Ownership Rules**
**Prompt**: Explain Rust's three ownership rules from memory

**Answer**: 
1. Each value in Rust has a single owner
2. There can only be one owner at a time  
3. When the owner goes out of scope, the value is dropped

**Tags**: #rust #ownership #fundamental
**Created**: 2025-10-07  
**Next Review**: 2025-10-10 (due today!)

---

### **RUST-003: String vs &str**
**Prompt**: What's the difference between `String` and `&str`? Give code examples.

**Answer**:
- `String`: Owned, growable, heap-allocated
- `&str`: Borrowed string slice, immutable reference
```rust
let owned: String = String::from("hello");  // Owns the data
let borrowed: &str = "hello";               // Points to string literal
let slice: &str = &owned[0..2];            // Borrows from String
```

**Tags**: #rust #strings #ownership  
**Created**: 2025-10-07
**Next Review**: 2025-10-12

---

### **RUST-007: Lifetime Elision Rules**

**Prompt**: What are Rust's three lifetime elision rules?

**Answer**:

1. Each input parameter gets its own lifetime parameter
2. If exactly one input lifetime, that lifetime is assigned to all outputs
3. If multiple input lifetimes and one is `&self` or `&mut self`, that lifetime is assigned to all outputs

**Tags**: #rust #lifetimes #advanced
**Created**: 2025-10-08  
**Next Review**: 2025-10-15

---

### **RUST-012: Iterator Lazy Evaluation**

**Prompt**: Why are Rust iterators "lazy" and what are the benefits?

**Answer**:

- Iterators do no work until consumed by `collect()`, `for`, `fold()`, etc.
- Benefits: Zero-cost abstractions, composability, memory efficiency
- Example: `.map().filter().take(5)` only processes first 5 matching items

**Tags**: #rust #iterators #performance
**Created**: 2025-10-09
**Next Review**: 2025-10-12

---

### **RUST-015: HashMap Basic get() Implementation**  

**Prompt**: Implement a basic `get()` method for HashMap from memory

**Answer**:

```rust
impl<K: Hash + Eq, V> HashMap<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash(key);
        let bucket = hash % self.buckets.len();
        
        for (k, v) in &self.buckets[bucket] {
            if k == key {
                return Some(v);
            }
        }
        None
    }
}
```

**Gotchas**: Handle collisions, use proper hashing, borrow key correctly
**Tags**: #rust #hashmap #implementation
**Created**: 2025-10-10
**Next Review**: 2025-10-10 (due today!)

---

### **ALG-004: BFS vs DFS Usage**

**Prompt**: When should you use BFS vs DFS? Give specific scenarios.

**Answer**:
**BFS (Breadth-First)**:

- Shortest path in unweighted graphs
- Level-order traversal  
- Finding connected components of similar "distance"

**DFS (Depth-First)**:

- Detecting cycles
- Topological sorting
- Pathfinding when any path is acceptable
- Tree/graph structure analysis

**Tags**: #algorithms #graph #traversal
**Created**: 2025-10-08
**Next Review**: 2025-10-15

---

### **ALG-008: Binary Search Invariant**

**Prompt**: Explain the key invariant that makes binary search correct

**Answer**:

- **Invariant**: If target exists, it must be in the range `[left, right)`
- This means: all elements before `left` are < target, all elements at `right` and after are >= target
- **Why it works**: We maintain this invariant by updating `left = mid + 1` or `right = mid`

**Tags**: #algorithms #binary-search #correctness
**Created**: 2025-10-10  
**Next Review**: 2025-10-10 (due today!)

---

## 🔄 **Spacing Algorithm**

**Initial Schedule**: Day 1 → Day 3 → Day 7 → Day 21 → Day 60

**Adjustment Rules**:

- **Too Easy**: Double the interval (3→6, 7→14, etc.)
- **Too Hard**: Reset to Day 1 spacing, add worked examples
- **Partially Correct**: Repeat current interval

**Example Progression**:

```
RUST-001: Created 2025-10-07
├─ First Review: 2025-10-10 (+3 days)  
├─ Second Review: 2025-10-17 (+7 days)
├─ Third Review: 2025-11-07 (+21 days)  
└─ Fourth Review: 2026-01-06 (+60 days)
```

---

## 📊 **Progress Tracking**

### **Cards by Topic**

- **Rust Fundamentals**: 5 cards (RUST-001 to RUST-015)
- **Algorithms**: 3 cards (ALG-004 to ALG-008)  
- **Data Structures**: 2 cards (planned)

### **Success Metrics**

- **Cards reviewed today**: [Track daily]
- **Success rate**: [% recalled correctly]
- **Topics needing reinforcement**: [Failed cards]

### **Weekly Card Creation Schedule**

- **Week 1**: Ownership, borrowing, basic types (5 cards)
- **Week 2**: Collections, iterators, error handling (5 cards)
- **Week 3**: Traits, generics, lifetimes (5 cards)
- **Week 4**: Algorithms, patterns, best practices (5 cards)

---

## 🛠️ **Usage Instructions**

### **Daily Review Process**

1. **Check "Due Today" section** each morning
2. **For each due card**: Try to recall answer completely before looking
3. **Grade yourself**: Correct/Partial/Failed
4. **Reschedule based on performance**:
   - Correct: Move to next interval
   - Partial: Repeat current interval  
   - Failed: Reset to 1-day interval

### **Adding New Cards**

1. **Use next available ID** (RUST-016, ALG-009, etc.)
2. **Follow template format** for consistency
3. **Set initial review date** (+3 days from creation)
4. **Tag appropriately** for filtering

### **Integration with Learning System**

- **Mission completion**: Create cards for key concepts learned
- **Daily study**: Add cards for difficult topics that need repetition
- **Error bank**: Convert common mistakes into recall cards
- **Weekly retrospective**: Analyze card success rates and adjust difficulty

---

*Tags: #spaced-repetition #retrieval-practice #learning-system #rust #algorithms #memory-retention*

*Links: [[zettel-index]] | [[developer-learning-habits]] | [[Progress Tracking]] | [[MONTHLY_CALENDAR]]*

**Next Actions:**

- [ ] Review cards due today (morning routine)
- [ ] Add 2 new cards from Mission 5 concepts
- [ ] Schedule weekly card calibration (Friday)
- [ ] Update success metrics after each review session
