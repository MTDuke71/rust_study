# Progressive Disclosure - Layered Learning Architecture

*Introducing complexity gradually through carefully sequenced information layers*

---

## 🎯 **Core Concept**

Progressive Disclosure is the pedagogical principle of revealing information **incrementally** rather than all at once. Learners encounter concepts in deliberate layers, each building on previous understanding, preventing cognitive overload while maintaining engagement.

**Key Insight**: Human working memory can hold ~7 items. Progressive disclosure respects this limit by chunking complex topics into digestible steps.

---

## 📐 **The Progressive Disclosure Model**

### **Layer Architecture**

```
Level 1: Concrete Foundation
    ↓ (Build on working code)
Level 2: Abstract Concept
    ↓ (Introduce theory after practice)
Level 3: Edge Cases & Nuances
    ↓ (Handle complexity after basics work)
Level 4: Optimization & Patterns
    ↓ (Refine after understanding is solid)
Level 5: Integration & Advanced Use
```

**Anti-Pattern**: Introducing generics, lifetimes, traits, and error handling simultaneously.

**Progressive Pattern**: 
1. Concrete Vec<i32> (Level 1)
2. Generic Vec<T> (Level 2)
3. Lifetime annotations (Level 3)
4. Trait bounds (Level 4)
5. Advanced patterns (Level 5)

---

## 🏗️ **Implementation in Rust Study Workspace**

### **Example: Mission1 Stack Tutorial (7-Layer Progression)**

#### **Layer 1: Concrete Foundation**
```rust
// Step 1: Work with basic Vec operations
fn main() {
    let mut stack = Vec::new();
    
    stack.push(1);      // Add element
    stack.push(2);
    stack.push(3);
    
    let top = stack.pop();  // Remove element
    println!("Popped: {:?}", top);  // Some(3)
}
```

**Learning**: LIFO semantics, push/pop operations, Option for empty case.

---

#### **Layer 2: Abstraction Introduction**
```rust
// Step 2: Wrap Vec in custom type
struct Stack {
    items: Vec<i32>,  // Still concrete type
}

impl Stack {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    fn push(&mut self, item: i32) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }
}
```

**Learning**: Encapsulation, method syntax, ownership with `self`.

---

#### **Layer 3: Generic Types**
```rust
// Step 3: Now introduce generics (after concrete works)
struct Stack<T> {  // Generic parameter added
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}
```

**Learning**: Type parameters, generic instantiation, monomorphization.

---

#### **Layer 4: Trait Bounds & Constraints**
```rust
// Step 4: Add trait bounds (after generics are comfortable)
impl<T: std::fmt::Display> Stack<T> {
    fn peek_display(&self) -> Option<String> {
        self.items.last()
            .map(|item| format!("{}", item))
    }
}

impl<T: Clone> Stack<T> {
    fn peek(&self) -> Option<T> {
        self.items.last().cloned()
    }
}
```

**Learning**: Trait constraints, conditional implementations, Clone semantics.

---

#### **Layer 5: Lifetimes & Borrowing**
```rust
// Step 5: Introduce lifetime annotations (after understanding references)
impl<T> Stack<T> {
    fn peek_ref(&self) -> Option<&T> {
        self.items.last()  // Returns reference with implicit lifetime
    }
    
    // Explicit lifetime when needed
    fn top_n<'a>(&'a self, n: usize) -> &'a [T] {
        let len = self.items.len();
        let start = len.saturating_sub(n);
        &self.items[start..]
    }
}
```

**Learning**: Reference lifetimes, borrow checker, lifetime elision rules.

---

#### **Layer 6: Iterator Patterns**
```rust
// Step 6: Advanced iterator integration (after comfort with basics)
impl<T> Stack<T> {
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter().rev()  // Stack iteration is reversed
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
```

**Learning**: Iterator traits, consuming vs borrowing iteration, adapter chains.

---

#### **Layer 7: Production Polish**
```rust
// Step 7: Add error handling, documentation, testing
impl<T> Stack<T> {
    /// Creates a new empty stack
    /// 
    /// # Examples
    /// ```
    /// let mut stack = Stack::<i32>::new();
    /// stack.push(42);
    /// ```
    pub fn new() -> Self {
        Stack { items: Vec::with_capacity(10) }
    }
    
    /// Pushes an element onto the stack
    /// 
    /// # Time Complexity
    /// O(1) amortized
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_push_pop_order() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));  // LIFO verified
    }
}
```

**Learning**: Documentation best practices, testing conventions, performance characteristics.

---

## 🎯 **Progressive Disclosure Patterns**

### **Pattern 1: Concrete → Abstract**

**Start**: `Vec::new()` with specific types
```rust
let mut numbers = Vec::new();
numbers.push(1);
numbers.push(2);
```

**Progress**: Generic collections
```rust
fn create_collection<T>() -> Vec<T> {
    Vec::new()
}
```

**Rationale**: Learners understand concrete operations first, then generalize.

---

### **Pattern 2: Happy Path → Error Cases**

**Start**: Assume success
```rust
fn divide(a: i32, b: i32) -> i32 {
    a / b  // Ignores division by zero
}
```

**Progress**: Handle errors
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

**Rationale**: Learn the concept before dealing with edge cases.

---

### **Pattern 3: Single Feature → Composed Features**

**Start**: One operation
```rust
fn filter_evens(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for n in nums {
        if n % 2 == 0 {
            result.push(n);
        }
    }
    result
}
```

**Progress**: Chained operations
```rust
fn filter_and_double_evens(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter()
        .filter(|n| n % 2 == 0)
        .map(|n| n * 2)
        .collect()
}
```

**Progress further**: Multiple transformations
```rust
fn complex_pipeline(nums: Vec<i32>) -> Vec<i32> {
    nums.into_iter()
        .filter(|n| n % 2 == 0)
        .map(|n| n * 2)
        .filter(|n| n > 10)
        .take(5)
        .collect()
}
```

**Rationale**: Master single operations before composition.

---

### **Pattern 4: Mutable → Immutable**

**Start**: Mutable operations (easier mental model)
```rust
fn update_scores(scores: &mut HashMap<String, i32>, name: &str) {
    let score = scores.get_mut(name).unwrap();
    *score += 10;  // Direct mutation
}
```

**Progress**: Immutable transformations
```rust
fn update_scores(scores: HashMap<String, i32>, name: &str) -> HashMap<String, i32> {
    scores.into_iter()
        .map(|(k, v)| {
            if k == name {
                (k, v + 10)
            } else {
                (k, v)
            }
        })
        .collect()
}
```

**Rationale**: Mutation is familiar; immutability requires new thinking patterns.

---

## 📊 **Mission Tutorial Examples**

### **Mission2: Queue (VecDeque) Progression**

| Layer | Concept | Code Focus |
|-------|---------|-----------|
| 1 | FIFO semantics | Manual Vec with remove(0) |
| 2 | Performance problem | Benchmark Vec::remove(0) - O(n) |
| 3 | VecDeque introduction | Replace with VecDeque::pop_front() |
| 4 | Ring buffer concept | Visualize circular buffer |
| 5 | Generic Queue<T> | Wrap VecDeque with custom type |
| 6 | Iterator integration | Implement IntoIterator |
| 7 | Production features | Capacity management, testing |

**Key**: Each layer solves a problem revealed in the previous layer.

---

### **Mission5: HashMap Progression**

| Layer | Concept | Code Focus |
|-------|---------|-----------|
| 1 | Key-value concept | Vec<(K, V)> with linear search |
| 2 | Performance bottleneck | Benchmark O(n) lookup |
| 3 | Hash function basics | Simple modulo hash |
| 4 | Collision handling | Chaining with Vec in each bucket |
| 5 | Generic hashing | BuildHasher trait |
| 6 | Resizing strategy | Load factor, rehashing |
| 7 | std::HashMap | Compare with standard library |

**Key**: Build intuition with simple implementation before production API.

---

## 🚫 **Anti-Patterns to Avoid**

### **Anti-Pattern 1: Information Dumping**

❌ **Bad**: Introduce all concepts at once
```rust
// Step 1: Here's a generic HashMap with BuildHasher!
struct HashMap<K, V, S = RandomState> 
where
    K: Eq + Hash,
    S: BuildHasher,
{
    buckets: Vec<Vec<(K, V)>>,
    hash_builder: S,
    size: usize,
    load_factor: f32,
}
```

**Problem**: Learners face generics, trait bounds, default type parameters, and implementation details simultaneously.

✅ **Good**: Start simple
```rust
// Step 1: Just a Vec of key-value pairs
struct SimpleMap {
    pairs: Vec<(String, i32)>,
}
```

---

### **Anti-Pattern 2: Premature Optimization**

❌ **Bad**: Optimize before understanding
```rust
// Step 1: Here's an optimized lock-free concurrent queue!
use std::sync::atomic::{AtomicPtr, Ordering};
struct LockFreeQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}
```

**Problem**: Introduces concurrency, unsafe code, atomics before basic queue semantics.

✅ **Good**: Optimize after mastery
```rust
// Step 1: Single-threaded queue with VecDeque
struct Queue<T> {
    items: VecDeque<T>,
}
// ... much later, in advanced examples ...
// Step 10: Concurrent queue with proper explanation
```

---

### **Anti-Pattern 3: Skipping Intermediate Steps**

❌ **Bad**: Jump from basic to advanced
```rust
// Step 1: Vec basics
let mut v = vec![1, 2, 3];

// Step 2: Advanced iterator combinators (too big a jump!)
let result: HashMap<_, _> = v.into_iter()
    .enumerate()
    .filter_map(|(i, x)| (i % 2 == 0).then(|| (x, i)))
    .collect();
```

✅ **Good**: Add intermediate steps
```rust
// Step 1: Vec basics
let mut v = vec![1, 2, 3];

// Step 2: Simple iteration
for item in &v {
    println!("{}", item);
}

// Step 3: Iterator basics
let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();

// Step 4: Filter and map
let evens: Vec<_> = v.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 2)
    .collect();

// ... later ...
// Step 8: Complex combinators
```

---

## 📚 **Cognitive Load Management**

### **Working Memory Constraints**

**Research**: Miller's Law - humans can hold 7±2 items in working memory.

**Application in Tutorials**:
- Introduce **max 3-4 new concepts** per step
- Reinforce through **immediate practice**
- Use **spaced repetition** across missions

**Example: Mission1 Stack Layer Breakdown**

| Layer | New Concepts | Working Memory Load |
|-------|-------------|---------------------|
| 1 | Vec, push, pop | 3 items ✅ |
| 2 | struct, impl, self | 3 items ✅ |
| 3 | Generics `<T>` | 1 item (builds on Layer 2) ✅ |
| 4 | Trait bounds | 1-2 items ✅ |
| 5 | Lifetimes `'a` | 2 items ✅ |

Each layer stays within cognitive capacity.

---

### **Scaffolding Techniques**

**Technique 1: Fill-in-the-Blank** (Reduces cognitive load)
```rust
// Complete this function (scaffolding provided)
fn push_item<T>(stack: &mut Stack<T>, item: ___) {
    stack.items.___(item);
}
```

**Technique 2: Guided Transformation** (Step-by-step changes)
```rust
// Transform this code from concrete to generic in 3 steps

// Step 1: Original (concrete)
struct Stack {
    items: Vec<i32>,
}

// Step 2: Add type parameter (you do this)
struct Stack<___> {
    items: Vec<___>,
}

// Step 3: Complete impl block (you do this)
impl<___> Stack<___> {
    // ...
}
```

**Technique 3: Progressive Uncommenting** (Controlled revelation)
```rust
fn example() {
    let mut stack = Stack::new();
    stack.push(1);
    
    // TODO: Uncomment when ready for generics
    // let mut string_stack: Stack<String> = Stack::new();
    // string_stack.push("hello".to_string());
    
    // TODO: Uncomment when ready for trait bounds
    // fn print_top<T: Display>(stack: &Stack<T>) { ... }
}
```

---

## 🎓 **Learning Progression Checklist**

### **Designing a Progressive Tutorial**

- [ ] **Layer 0: Prerequisites** - State required knowledge upfront
- [ ] **Layer 1: Concrete example** - Working code with specific types
- [ ] **Layer 2: Introduce one abstraction** - Generic types OR traits OR lifetimes (not all)
- [ ] **Layer 3: Add complexity incrementally** - One new concept per step
- [ ] **Layer 4: Edge cases & error handling** - After happy path works
- [ ] **Layer 5: Optimization & patterns** - After correctness established
- [ ] **Layer 6: Integration** - Connect to other concepts
- [ ] **Layer 7: Production polish** - Documentation, testing, benchmarks

**Validation**: Can a learner skip layers 4-7 and still have working code?

---

## 🔗 **Related Concepts**

### **Tutorial Engineering**
- [[Tutorial Engineering]] - Parent methodology
- [[Error Anticipation]] - Complementary principle
- [[Hands-On Learning]] - Practice integration
- [[Incremental Complexity]] - Building block approach

### **Cognitive Science**
- [[Working Memory Limitations]] - Miller's Law application
- [[Cognitive Load Theory]] - Managing mental effort
- [[Chunking Strategies]] - Grouping related concepts

### **Mission Applications**
- [[Mission1 Tutorial]] - 7-layer stack progression
- [[Mission5 Tutorial]] - HashMap complexity layers
- [[Daily Study MOC]] - Week-by-week disclosure

---

## 💡 **Key Takeaways**

1. **Introduce complexity gradually** - One layer at a time prevents overwhelm
2. **Start concrete, end abstract** - Specific types before generics
3. **Happy path before edge cases** - Working code before error handling
4. **Respect working memory limits** - Max 3-4 new concepts per step
5. **Provide scaffolding** - Fill-in-blank, guided transformations
6. **Make layers optional** - Advanced users can skip ahead
7. **Validate with real learners** - Test if progression makes sense

**Remember**: "Give me six hours to chop down a tree and I will spend the first four sharpening the axe." - Progressive disclosure is sharpening the axe.

---

*Tags: #progressive-disclosure #pedagogy #cognitive-load #tutorial-design #learning-architecture #scaffolding #incremental-complexity #education*

*Links: [[zettel-index]] | [[Tutorial Engineering]] | [[Error Anticipation]] | [[Hands-On Learning]] | [[Daily Study MOC]] | [[3-Track Integration]]*
