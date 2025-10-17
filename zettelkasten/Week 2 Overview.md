# Week 2 Overview - Collections Mastery

**Learning Period**: Days 8-14
**Theme**: Data Structures and Collection Types
**Mission Alignment**: Foundation for Mission5 (HashMap/HashSet)

---

## 🎯 Week Objectives

Master Rust's **standard library collections** and understand how they leverage Week 1's ownership system:

1. **Vec<T>** - Dynamic arrays and growable storage
2. **String** - UTF-8 text handling and string slices
3. **HashMap/HashSet** - Hash-based key-value and unique collections
4. **BTreeMap/BTreeSet** - Ordered tree-based collections
5. **Iterators** - Functional data processing patterns
6. **Error Handling** - Robust error management across operations

---

## 📚 Daily Progression

### **Day 8: Vectors (Vec<T>)** [[Day 08 - Vectors]]
- Growable, heap-allocated sequences
- Creation: Vec::new(), vec![], with_capacity()
- Operations: push, pop, insert, remove
- Iteration patterns and slicing
- Foundation for Mission1 Stack and Mission2 Queue

**Key Takeaway**: Vec<T> is the workhorse data structure in Rust

### **Day 9: Strings** [[Day 09 - Strings]]
- String vs &str distinction (owned vs borrowed)
- UTF-8 encoding and Unicode handling
- String manipulation: split, trim, replace
- Parsing and formatting
- Common AoC text processing patterns

**Key Takeaway**: Strings are UTF-8 and require special handling

### **Day 10: HashMap Basics** [[daily-study/Day10]]
- Key-value storage with O(1) average access
- Trait requirements: K: Eq + Hash
- Entry API patterns
- Interior mutability and wrapper patterns
- Direct alignment with [[Mission5 Overview]]

**Key Takeaway**: HashMap provides fast lookups with hash-based indexing

### **Day 11: HashSet Operations** [[daily-study/Day11]]
- Unique value collections
- Set theory operations (union, intersection, difference)
- HashSet<T> = HashMap<T, ()> zero-cost abstraction
- Deduplication and membership testing
- Mission5 REQ-4 iterator implementation alignment

**Key Takeaway**: HashSet enables efficient uniqueness and set operations

### **Day 12: BTreeMap & BTreeSet** [[Day 12 - BTreeMap]]
- Ordered collections with O(log n) operations
- Range queries (impossible with HashMap)
- Custom Ord implementations
- Performance trade-offs vs hash collections
- Mission5 alternative comparison

**Key Takeaway**: BTreeMap provides guaranteed performance with ordering

### **Day 13: Advanced Iterators** [[Day 13 - Advanced Iterators]]
- Lazy evaluation and zero-cost abstractions
- Iterator adaptors (map, filter, chain)
- Consuming adaptors (collect, fold, sum)
- Custom iterator implementations
- Functional programming patterns

**Key Takeaway**: Iterators enable expressive, efficient data processing

### **Day 14: Error Handling Patterns** [[Day 14 - Error Handling]]
- Result<T, E> deep dive
- Custom error types with enums
- Error propagation with ? operator
- anyhow and thiserror patterns
- Robust Mission5 operations

**Key Takeaway**: Comprehensive error handling makes code production-ready

---

## 🔗 Cross-Track Integration

### **Mission Track Connections**

**Primary Focus: Mission5 (HashMap/HashSet)**
- Days 10-11 directly align with Mission5 requirements
- Day 12 provides performance comparison context
- Day 13 covers iterator requirements (REQ-4)
- Day 14 enables robust error handling (REQ-5)

**Supporting Missions:**
- [[Mission1 Overview]] - Stack uses Vec<T> internally
- [[Mission2 Overview]] - Queue ring buffer with Vec<T>
- All collections apply ownership from Week 1

### **Rust Book Alignment**
- **Chapter 8**: Common Collections (Days 8-11)
- **Chapter 9**: Error Handling (Day 14)
- **Chapter 13**: Functional Features (Day 13)
- Collections extend ownership concepts from Chapters 4-6

### **Zettelkasten Network**
- [[Collections MOC]] - Complete collection map
- [[HashMap Internals]] - Deep dive on hash tables
- [[Mission5 Overview]] - V-Cycle project integration
- [[Rust Concepts MOC]] - Foundation concepts
- [[zettel-index]] - Master index

---

## 📊 Learning Outcomes

By the end of Week 2, you should be able to:

### **Conceptual Understanding**
- ✅ Explain when to use Vec vs HashMap vs BTreeMap
- ✅ Describe hash-based vs tree-based collection trade-offs
- ✅ Understand String vs &str memory models
- ✅ Apply Eq + Hash vs Ord trait requirements
- ✅ Use iterators for data transformation
- ✅ Design custom error types

### **Practical Skills**
- ✅ Choose appropriate collections for problems
- ✅ Handle UTF-8 text safely
- ✅ Implement Entry API patterns
- ✅ Write iterator chains for data processing
- ✅ Create custom error types with context
- ✅ Optimize collection operations

### **Code Patterns**

```rust
// Vec creation and manipulation
let mut vec = Vec::with_capacity(100);
vec.push(42);
let item = vec.pop();

// String handling
let owned = String::from("hello");
let borrowed: &str = &owned;
let parsed: i32 = "42".parse()?;

// HashMap with Entry API
let mut map = HashMap::new();
*map.entry("key").or_insert(0) += 1;

// HashSet operations
let set1: HashSet<_> = [1, 2, 3].into_iter().collect();
let set2: HashSet<_> = [2, 3, 4].into_iter().collect();
let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();

// Iterator chain
let result: Vec<_> = data
    .iter()
    .filter(|&&x| x > 10)
    .map(|x| x * 2)
    .collect();

// Custom error handling
#[derive(Debug)]
enum MyError {
    NotFound,
    InvalidInput(String),
}

fn process(input: &str) -> Result<i32, MyError> {
    input.parse()
        .map_err(|e| MyError::InvalidInput(e.to_string()))
}
```

---

## 🎓 Mastery Checkpoints

### **Self-Assessment Questions**

1. **Collection Choice**: When would you use HashMap vs BTreeMap?
   ```rust
   // HashMap: Need O(1) lookups, don't care about order
   let mut cache: HashMap<String, Data> = HashMap::new();

   // BTreeMap: Need sorted iteration or range queries
   let mut timeline: BTreeMap<u64, Event> = BTreeMap::new();
   ```

2. **String Ownership**: What's the difference?
   ```rust
   fn take_ownership(s: String) { }     // Consumes string
   fn borrow_string(s: &str) { }        // Borrows (preferred)
   ```

3. **Iterator Efficiency**: Which is more efficient?
   ```rust
   // ❌ Collects intermediate vector
   let result = vec.iter()
       .map(|x| x * 2)
       .collect::<Vec<_>>()
       .into_iter()
       .sum();

   // ✅ No intermediate allocation
   let result: i32 = vec.iter()
       .map(|x| x * 2)
       .sum();
   ```

4. **Error Propagation**: How does this work?
   ```rust
   fn process() -> Result<i32, Error> {
       let val1 = parse_number()?;  // Returns early if Err
       let val2 = parse_number()?;
       Ok(val1 + val2)
   }
   ```

**If you can answer all four**, you've mastered Week 2! 🎯

---

## 🚀 Mission5 Integration

**Week 2 → Mission5 Direct Mapping:**

| Day | Mission5 Connection |
|-----|---------------------|
| **Day 10** | REQ-1: Generic HashMap<K,V> storage |
| **Day 11** | REQ-2: HashSet wrapper implementation |
| **Day 13** | REQ-4: Iterator trait implementation |
| **Day 14** | REQ-5: Robust error handling |
| **Day 12** | Performance comparison context |

### **Mission5 Preparation Checklist**
- [x] Understand Eq + Hash trait requirements
- [x] Master Entry API patterns
- [x] Implement custom iterator types
- [x] Design error types for operations
- [x] Compare hash vs tree trade-offs

**After Week 2, you're ready to build Mission5!** 🎯

---

## 📈 Progress Tracking

### **Completed Materials**
- [x] Day 8: Vectors (Vec<T>)
- [x] Day 9: Strings & UTF-8
- [x] Day 10: HashMap Basics
- [x] Day 11: HashSet Operations
- [x] Day 12: BTreeMap & BTreeSet
- [x] Day 13: Advanced Iterators
- [x] Day 14: Error Handling Patterns

### **Skills Acquired**
- [x] Collection selection for performance
- [x] UTF-8 text processing
- [x] Hash-based data structures
- [x] Tree-based data structures
- [x] Functional iterator patterns
- [x] Production error handling

### **Mission Readiness**
- [x] Ready for Mission5 (HashMap/HashSet)
- [x] Can optimize collection operations
- [x] Can handle complex data structures

---

## 🔍 Common Week 2 Challenges

### **Challenge 1: Borrowing While Modifying**
```rust
// ❌ Problem
let mut vec = vec![1, 2, 3];
let first = &vec[0];
vec.push(4); // ❌ Can't modify while borrowed
println!("{}", first);

// ✅ Solution
let first_value = vec[0]; // Copy the value
vec.push(4);
println!("{}", first_value);
```

### **Challenge 2: String Concatenation**
```rust
// ❌ Inefficient
let mut result = String::new();
for i in 0..1000 {
    result = result + &i.to_string(); // Many allocations
}

// ✅ Efficient
let mut result = String::with_capacity(4000);
for i in 0..1000 {
    result.push_str(&i.to_string());
}
```

### **Challenge 3: HashMap Key Mutability**
```rust
// ❌ Problem - Key must be immutable!
let mut map = HashMap::new();
let mut key = vec![1, 2, 3];
map.insert(key.clone(), "value");
key.push(4); // Changes hash!

// ✅ Solution - Clone or use immutable keys
let key = vec![1, 2, 3];
map.insert(key, "value");
// Don't modify key after inserting
```

### **Challenge 4: Iterator Ownership**
```rust
// ❌ Problem
let vec = vec![1, 2, 3];
let doubled = vec.into_iter().map(|x| x * 2);
// vec is now consumed!

// ✅ Solution
let vec = vec![1, 2, 3];
let doubled: Vec<_> = vec.iter().map(|x| x * 2).collect();
// vec is still available
```

---

## 💡 Week 2 Study Tips

1. **Visualize Memory Layouts**: Draw how Vec, String, HashMap store data
2. **Compare Collections**: Create a table of when to use each type
3. **Practice Iterator Chains**: Solve problems functionally vs imperatively
4. **Read Standard Library Docs**: [docs.rs](https://doc.rust-lang.org/std/collections/)
5. **Benchmark Performance**: Use `cargo bench` to measure collection operations
6. **AoC Practice**: Apply patterns to competitive programming problems

---

## 🎮 AoC Collection Patterns

Common patterns from Advent of Code that use Week 2 knowledge:

### **Coordinate Tracking**
```rust
use std::collections::HashSet;
type Point = (i32, i32);

let mut visited: HashSet<Point> = HashSet::new();
visited.insert((0, 0));
```

### **Frequency Counting**
```rust
use std::collections::HashMap;

let mut counts = HashMap::new();
for item in data {
    *counts.entry(item).or_insert(0) += 1;
}
```

### **Graph Representation**
```rust
use std::collections::HashMap;
type Graph = HashMap<String, Vec<String>>;

let mut graph: Graph = HashMap::new();
graph.entry("A".to_string()).or_insert(vec![]).push("B".to_string());
```

### **Time Series Processing**
```rust
use std::collections::BTreeMap;

let mut events: BTreeMap<u64, Event> = BTreeMap::new();
for (timestamp, event) in events.range(start..end) {
    process(event);
}
```

---

## 🔄 Transition to Week 3

**Week 1** gave you **ownership discipline**.
**Week 2** gave you **collection expertise**.
**Week 3** will cover **traits, generics, and advanced patterns**.

### **What's Next:**
- Traits and trait bounds
- Generic programming
- Smart pointers (Box, Rc, Arc)
- Concurrency basics
- Advanced type system features

### **How Week 2 Prepares You:**
```rust
// Week 2 knowledge enables advanced patterns:
impl<K, V> MyHashMap<K, V>
where
    K: Eq + Hash,  // Week 2: trait requirements
{
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        // Week 2: custom iterators
        self.buckets.iter()
            .flat_map(|bucket| bucket.iter())
    }
}
```

---

## 📚 Further Resources

### **Official Documentation**
- [Rust Book Chapter 8 - Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Rust Book Chapter 13 - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [std::collections Module](https://doc.rust-lang.org/std/collections/)

### **Zettelkasten Notes**
- [[Collections MOC]] - Complete collection map
- [[HashMap Internals]] - Hash table deep dive
- [[Mission5 Overview]] - V-Cycle integration
- [[Performance Optimization Guide]] - Collection performance tips
- [[zettel-index]] - Master index

### **Advanced Reading**
- [Rust Performance Book - Collections](https://nnethercote.github.io/perf-book/standard-library-types.html)
- [Jon Gjengset - Crust of Rust: Iterators](https://www.youtube.com/watch?v=yozQ9C69pNs)

---

## 📊 Week 2 vs Week 1 Comparison

| Aspect | Week 1 | Week 2 |
|--------|---------|---------|
| **Focus** | Memory safety fundamentals | Data structure expertise |
| **Key Types** | References, lifetimes | Vec, String, HashMap |
| **Complexity** | Understanding core rules | Applying rules to collections |
| **Mission Prep** | Foundation for all | Mission5 directly |
| **Rust Book** | Chapters 1-10 | Chapters 8, 9, 13 |
| **Skills** | Safe memory management | Efficient data processing |

---

**Week 2 Complete!** You now have **collection mastery** to build efficient, safe data structures. Combined with Week 1's ownership, you're ready to implement **Mission5's HashMap/HashSet** from scratch! 🦀

*Last Updated: Week 2 completion*
*Next: [[Week 3 Overview]] - Traits, Generics & Advanced Patterns*

---

*Links: [[Day 08 - Vectors]] | [[Day 14 - Error Handling]] | [[Week 1 Overview]] | [[Week 3 Overview]] | [[Collections MOC]] | [[Mission5 Overview]] | [[zettel-index]]*
*Tags: #week-overview #week2 #collections #hashmap #iterators #daily-study #learning-path #mission5*
