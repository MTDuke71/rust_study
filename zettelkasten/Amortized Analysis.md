# Amortized Analysis - Average Cost Over Operation Sequences

*Understanding performance by analyzing the average cost per operation over a worst-case sequence of operations*

---

## 🎯 **Core Concept**

**Amortized analysis** answers: *"What is the average cost per operation when we consider a sequence of operations?"*

Unlike average-case analysis (which considers probability distributions), amortized analysis guarantees that the average cost per operation is low, even if some individual operations are expensive.

**Key Insight**: An occasional expensive operation can be "paid for" by many cheap operations, giving a low amortized cost.

---

## 🧠 **Mental Models**

### **The Savings Account Model**

Think of cheap operations as "deposits" into a savings account:

- Each cheap O(1) operation deposits extra "credits"
- When an expensive O(n) operation occurs, it withdraws from the accumulated credits
- As long as we've saved enough, the average cost stays low

```rust
// Vec::push example - most pushes are cheap, occasional resize is expensive
let mut vec = Vec::new();

// 7 cheap pushes (O(1) each) - "depositing credits"
for i in 0..7 {
    vec.push(i);  // Just writes to pre-allocated memory
}

// 8th push triggers resize (O(n)) - "withdrawing credits"
vec.push(7);  // Must allocate new memory and copy 7 elements

// But the average cost over 8 operations is still O(1)!
// Total work: 7 × O(1) + O(8) = O(15) / 8 operations ≈ O(2) per operation = O(1) amortized
```

### **The Aggregate Method**

Sum the total cost of n operations, then divide by n:

```
Amortized cost = (Total cost for n operations) / n
```

### **The Potential Method**

Associate a "potential energy" with the data structure:

- Cheap operations increase potential (store energy)
- Expensive operations decrease potential (release energy)
- Amortized cost = actual cost + change in potential

---

## 🔍 **Classic Examples**

### **1. Dynamic Array (Vec<T>) Growth**

Rust's `Vec<T>` doubles capacity when full. This gives O(1) amortized push:

```rust
/// Demonstrates Vec's amortized O(1) push operation
fn demonstrate_vec_amortization() {
    let mut vec: Vec<i32> = Vec::new();
    let mut resize_count = 0;
    let mut total_copies = 0;
    
    for i in 0..32 {
        let old_capacity = vec.capacity();
        vec.push(i);
        let new_capacity = vec.capacity();
        
        if new_capacity > old_capacity && old_capacity > 0 {
            resize_count += 1;
            let copies = old_capacity;  // All elements copied during resize
            total_copies += copies;
            println!("Push {}: Resized {} -> {} (copied {} elements)", 
                     i, old_capacity, new_capacity, copies);
        }
    }
    
    println!("\nSummary:");
    println!("  Total pushes: 32");
    println!("  Resize events: {}", resize_count);
    println!("  Total element copies: {}", total_copies);
    println!("  Average copies per push: {:.2}", total_copies as f64 / 32.0);
}

// Output:
// Push 1: Resized 1 -> 4 (copied 1 elements)
// Push 4: Resized 4 -> 8 (copied 4 elements)
// Push 8: Resized 8 -> 16 (copied 8 elements)
// Push 16: Resized 16 -> 32 (copied 16 elements)
//
// Summary:
//   Total pushes: 32
//   Resize events: 4
//   Total element copies: 29
//   Average copies per push: 0.91
```

**Mathematical Analysis**:

- After n pushes, we've resized at most log₂(n) times
- Total copies: 1 + 2 + 4 + 8 + ... + n/2 = n - 1
- Amortized cost per push: (n - 1 + n) / n = (2n - 1) / n ≈ 2 = **O(1)**

### **2. Stack with Multipop**

A stack supporting push (O(1)) and multipop (O(k) to pop k elements):

```rust
struct AmortizedStack<T> {
    items: Vec<T>,
}

impl<T> AmortizedStack<T> {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
    
    /// O(1) worst case
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    /// O(k) where k is min(count, len)
    /// But O(1) amortized!
    fn multipop(&mut self, count: usize) -> Vec<T> {
        let actual = count.min(self.items.len());
        let start = self.items.len() - actual;
        self.items.drain(start..).collect()
    }
}

// Analysis: Each element is pushed once and popped at most once.
// Total work for n operations ≤ 2n = O(n)
// Amortized cost per operation = O(n) / n = O(1)
```

### **3. Hash Table Operations**

`HashMap` insert is O(1) amortized due to occasional rehashing:

```rust
use std::collections::HashMap;

fn demonstrate_hashmap_amortization() {
    let mut map = HashMap::new();
    
    // Most insertions are O(1)
    for i in 0..1000 {
        map.insert(i, i * 2);  // O(1) most of the time
        
        // Occasionally HashMap rehashes (resizes buckets)
        // This is O(n) but happens infrequently
    }
    
    // Amortized insert: O(1)
    // Same analysis as Vec - capacity doubles when load factor exceeded
}
```

### **4. Union-Find (Disjoint Set Union)**

With path compression and union by rank:

```rust
/// Union-Find with path compression and union by rank
/// 
/// # Amortized Complexity
/// - Find: O(α(n)) amortized where α is inverse Ackermann function
/// - Union: O(α(n)) amortized
/// - α(n) < 5 for all practical n, so effectively O(1)
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    
    /// Find with path compression - O(α(n)) amortized
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            // Path compression: point directly to root
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    
    /// Union by rank - O(α(n)) amortized
    fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        
        if px == py {
            return false;  // Already in same set
        }
        
        // Union by rank: attach smaller tree under larger
        match self.rank[px].cmp(&self.rank[py]) {
            std::cmp::Ordering::Less => self.parent[px] = py,
            std::cmp::Ordering::Greater => self.parent[py] = px,
            std::cmp::Ordering::Equal => {
                self.parent[py] = px;
                self.rank[px] += 1;
            }
        }
        true
    }
}

// See [[mission-10]] for complete implementation
// See [[union-find-algorithm]] for detailed analysis
```

---

## 📊 **Analysis Techniques**

### **Aggregate Method**

1. Count total cost T(n) for worst-case sequence of n operations
2. Amortized cost = T(n) / n

```rust
// Example: Incrementing a binary counter
// Each bit flip costs O(1)
// How many flips total for n increments?

fn count_bit_flips(n: u32) -> u32 {
    // Bit 0 flips every increment:       n times
    // Bit 1 flips every 2 increments:    n/2 times
    // Bit 2 flips every 4 increments:    n/4 times
    // ...
    // Total: n + n/2 + n/4 + ... < 2n flips
    
    // Amortized flips per increment: 2n/n = 2 = O(1)
    
    let mut total_flips = 0;
    let mut counter = 0u32;
    
    for _ in 0..n {
        let old = counter;
        counter += 1;
        // Count differing bits (XOR gives 1s where bits differ)
        total_flips += (old ^ counter).count_ones();
    }
    
    total_flips
}

#[test]
fn test_bit_flips() {
    // For n=16 increments, expect < 32 flips
    let flips = count_bit_flips(16);
    assert!(flips < 32, "Expected < 32 flips, got {}", flips);
    println!("16 increments: {} flips (avg {:.2})", flips, flips as f64 / 16.0);
}
```

### **Accounting Method**

Assign "amortized costs" to operations that may differ from actual costs:

- Cheap operations are charged more than they cost (bank savings)
- Expensive operations are charged less (use savings)
- Credit must never go negative

```rust
/// Accounting method for Vec push
/// 
/// Assign amortized cost of 3 to each push:
/// - 1 for the actual insertion
/// - 1 to pay for copying this element during future resize
/// - 1 to pay for copying one already-present element
/// 
/// When resize happens:
/// - We have n/2 elements that haven't been copied yet
/// - Each contributed 2 credits
/// - Total credits = n (enough to copy n elements)
fn accounting_analysis() {
    // This is a proof technique, not runtime code
    // It shows Vec::push is O(1) amortized
}
```

### **Potential Method**

Define potential function Φ(D) on data structure state:

- Amortized cost = actual cost + ΔΦ
- Choose Φ so expensive operations have large negative ΔΦ

```rust
/// Potential method for Vec
/// 
/// Define Φ(vec) = 2 * len - capacity
/// 
/// For push when not at capacity:
///   Actual cost: 1
///   ΔΦ = (2(len+1) - cap) - (2*len - cap) = 2
///   Amortized = 1 + 2 = 3
/// 
/// For push with resize (doubling):
///   Actual cost: len + 1 (copy len elements + insert new)
///   New capacity = 2 * old_cap = 2 * len
///   ΔΦ = (2(len+1) - 2*len) - (2*len - len) = 2 - len
///   Amortized = (len + 1) + (2 - len) = 3
/// 
/// Both cases give amortized cost of 3 = O(1)
fn potential_analysis() {
    // This is a proof technique, not runtime code
}
```

---

## 💡 **Key Takeaways**

1. **Amortized ≠ Average Case**: Amortized is a guarantee; average case depends on input distribution

2. **Expensive operations are okay** if they're infrequent and "paid for" by cheap operations

3. **Common amortized O(1) operations**:
   - `Vec::push` - resizes double capacity
   - `HashMap::insert` - rehashes when load factor exceeded
   - `String::push_str` - similar to Vec
   - Union-Find find/union - with path compression

4. **When to use amortized analysis**:
   - Data structures with occasional expensive operations
   - Sequences of operations (not single operations in isolation)
   - When worst-case is misleading (too pessimistic)

5. **Mission Applications**:
   - [[mission-1]]: Stack push is O(1) amortized
   - [[mission-2]]: Ring buffer avoids amortization (true O(1))
   - [[mission-5]]: HashMap operations are O(1) amortized
   - [[mission-10]]: Union-Find is O(α(n)) amortized

---

## 🔗 **Integration Points**

### **Builds On**

- [[Big-O Analysis]] - Foundation for complexity analysis
- [[Algorithm Analysis]] - Comprehensive analysis techniques
- [[Performance Engineering]] - Optimization strategies

### **Enables**

- [[Dynamic Programming Patterns]] - Understanding memoization costs
- [[Cache Efficiency]] - Analyzing memory access patterns
- [[Benchmarking]] - Validating amortized claims empirically

### **Related Concepts**

- [[Memory Optimization]] - Space-time tradeoffs
- [[Iteration Patterns]] - Iterator adaptor costs
- [[zero-cost-abstractions]] - Rust's performance guarantees

### **Mission Applications**

- [[mission-1]] - Stack: Vec-based O(1) amortized push/pop
- [[mission-2]] - Queue: Ring buffer avoids resize (true O(1))
- [[mission-5]] - HashMap: O(1) amortized with good hash
- [[mission-10]] - Union-Find: O(α(n)) amortized operations

---

## 📚 **Rust Standard Library Examples**

| Type | Operation | Worst Case | Amortized |
|------|-----------|------------|-----------|
| `Vec<T>` | push | O(n) | O(1) |
| `Vec<T>` | pop | O(1) | O(1) |
| `String` | push_str | O(n) | O(1) |
| `HashMap<K,V>` | insert | O(n) | O(1) |
| `HashMap<K,V>` | get | O(n)* | O(1) |
| `VecDeque<T>` | push_back | O(n) | O(1) |
| `BinaryHeap<T>` | push | O(log n) | O(log n) |

*HashMap worst case requires many collisions (rare with good hash)

---

*Tags: #algorithm-analysis #complexity #amortized-analysis #performance #data-structures #computer-science #mission-1 #mission-5 #mission-10*

*Links: [[Algorithm Analysis]] | [[Big-O Analysis]] | [[Performance Engineering]] | [[mission-1]] | [[mission-5]] | [[mission-10]] | [[union-find-algorithm]] | [[Algorithms MOC]] | [[Collections MOC]] | [[zettel-index]]*
