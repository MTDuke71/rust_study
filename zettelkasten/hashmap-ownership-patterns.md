# HashMap Ownership Patterns - Keys, Values, and References

> **Core Principle**: HashMap can store values by ownership or by reference, each with different trade-offs for lifetime management, mutability, and memory efficiency.

## 🎯 The Fundamental Choice

When inserting into a HashMap, you have three patterns:

| Pattern | Syntax | Ownership | Can Modify Original? | Use Case |
|---------|--------|-----------|---------------------|----------|
| **By Value** | `map.insert(k, v)` | HashMap owns | No (moved) | Standard - HashMap controls lifetime |
| **By Immutable Ref** | `map.insert(k, &v)` | Original owns | No (borrowed) | Temporary maps, read-only views |
| **By Mutable Ref** | `map.insert(k, &mut v)` | Original owns | Via map only | Rare - complex lifetime constraints |

## 📦 Pattern 1: Ownership Transfer (Standard Pattern)

### **HashMap Takes Ownership**

```rust
let mut map = HashMap::new();

let key = String::from("name");
let value = String::from("Alice");

map.insert(key, value);  // Both MOVED into HashMap

// ❌ Can't use them anymore:
// println!("{}", key);    // Error: value moved
// println!("{}", value);  // Error: value moved

// ✅ Access through HashMap:
if let Some(v) = map.get(&String::from("name")) {
    println!("Value: {}", v);  // Prints "Alice"
}
```

**What Actually Happens:**

```
Before insert:
  key:   String { ptr → "name" on heap, len: 4, cap: 4 }
  value: String { ptr → "Alice" on heap, len: 5, cap: 5 }
  map:   HashMap { buckets: [], len: 0 }

After insert:
  key:   ❌ MOVED (can't access)
  value: ❌ MOVED (can't access)
  map:   HashMap { 
           buckets: [(String("name"), String("Alice"))],
           len: 1 
         }
         └─ Now OWNS both Strings
```

**Memory Diagram:**

```
Stack:
  map.hash_builder: RandomState { k0, k1 }
  map.buckets: Vec ptr ──┐
                         │
Heap:                    ↓
  Bucket Array: [Some((key_string, value_string)), None, None, ...]
                       ↓                ↓
                   "name" heap      "Alice" heap
```

### **Modifying Owned Values**

```rust
let mut map = HashMap::new();
map.insert(String::from("name"), String::from("Alice"));

// Get mutable reference to the value
if let Some(name) = map.get_mut(&String::from("name")) {
    name.push_str(" Smith");  // Modify the String in-place
}

println!("{}", map[&String::from("name")]);  // "Alice Smith"
```

**What Happens:**

1. `get_mut()` returns `Option<&mut String>` - mutable reference to the **owned** String
2. You can modify the String because HashMap owns it
3. Changes persist in the HashMap
4. No borrowing conflicts because you're borrowing from HashMap, not the original owner

### **When Original Data Needs to Survive**

```rust
let key = String::from("user_id");
let value = String::from("12345");

// Clone to preserve originals
let mut map = HashMap::new();
map.insert(key.clone(), value.clone());

// Now you have both:
println!("Original: {} = {}", key, value);  // Still valid

// And the map has its own copy:
println!("Map: {}", map[&key]);

// They're independent:
if let Some(v) = map.get_mut(&key) {
    v.push_str("-UPDATED");
}

println!("Original value: {}", value);  // "12345" (unchanged)
println!("Map value: {}", map[&key]);   // "12345-UPDATED"
```

## 🔗 Pattern 2: Immutable References (Read-Only Access)

### **HashMap Stores `&T` Instead of `T`**

```rust
let mut map: HashMap<&str, &String> = HashMap::new();
//           Type:     ^key  ^value

let key = "name";
let value = String::from("Alice");

map.insert(key, &value);  // Store reference, not ownership

// ✅ Original still accessible:
println!("Original value: {}", value);

// ✅ Access through map:
println!("Map value: {}", map[&"name"]);
```

**Memory Diagram:**

```
Stack:
  value: String { ptr → "Alice" heap, len: 5, cap: 5 }
  map.buckets: Vec ptr ──┐
                         │
Heap:                    ↓
  Bucket Array: [Some(("name", &value_ref)), None, ...]
                              └─ Points to stack's value
```

### **The Lifetime Constraint**

```rust
let mut map: HashMap<&str, &String> = HashMap::new();

{
    let value = String::from("Alice");
    map.insert("name", &value);
    
    // ❌ This won't compile!
} // value dropped here, but map still holds reference!

// map reference is now DANGLING - compiler prevents this!
```

**Compiler Error:**

```
error[E0597]: `value` does not live long enough
  --> src/main.rs:5:24
   |
4  |     let value = String::from("Alice");
   |         ----- binding `value` declared here
5  |     map.insert("name", &value);
   |                        ^^^^^^ borrowed value does not live long enough
6  | } // value dropped here
   | - `value` dropped here while still borrowed
7  | 
8  | println!("{}", map[&"name"]);
   |                --- borrow later used here
```

**Rust's Guarantee:** References in HashMap **must** outlive the HashMap itself.

### **Valid Use Case: References to Static Data**

```rust
let mut map: HashMap<&str, &str> = HashMap::new();

map.insert("greeting", "Hello");      // String literal - 'static lifetime
map.insert("farewell", "Goodbye");    // String literal - 'static lifetime

// ✅ Valid - string literals live for entire program duration
```

### **Valid Use Case: Temporary Aggregation**

```rust
fn analyze_strings(strings: &[String]) -> HashMap<&str, usize> {
    let mut lengths = HashMap::new();
    
    for s in strings {
        lengths.insert(s.as_str(), s.len());
        // References valid for function duration
    }
    
    lengths  // Return map of references
    // References still valid - they point to input slice
}

fn main() {
    let data = vec![
        String::from("hello"),
        String::from("world"),
    ];
    
    let analysis = analyze_strings(&data);
    // analysis holds references to data
    
    println!("{:?}", analysis);
    // ✅ Valid - data still alive
    
} // data and analysis dropped together - no dangling refs
```

## 🔄 Pattern 3: Mutable References (Rare Pattern)

### **HashMap Stores `&mut T`**

```rust
let mut map: HashMap<&str, &mut String> = HashMap::new();

let key = "name";
let mut value = String::from("Alice");

map.insert(key, &mut value);

// ⚠️ Now you CANNOT access value directly:
// println!("{}", value);  // Error: value is mutably borrowed

// ✅ Access through map only:
if let Some(v) = map.get_mut(&"name") {
    v.push_str(" Smith");  // Modify through map's reference
}

// Drop map to regain access to value:
drop(map);

// ✅ Now you can access value again:
println!("Value: {}", value);  // "Alice Smith"
```

**The Mutable Borrow Rule:**

```
While map holds &mut value:
  ❌ Can't read value directly
  ❌ Can't create another &value
  ❌ Can't create another &mut value
  ✅ Can modify through map.get_mut()
  
After map dropped:
  ✅ Full access to value restored
```

### **Why This Pattern is Rare**

```rust
// ❌ DOESN'T WORK - can't have two mutable references
let mut map: HashMap<&str, &mut String> = HashMap::new();

let mut value1 = String::from("Alice");
let mut value2 = String::from("Bob");

map.insert("first", &mut value1);
map.insert("second", &mut value2);

// Now trying to modify both:
if let Some(v1) = map.get_mut(&"first") {
    if let Some(v2) = map.get_mut(&"second") {
        // ❌ Compiler error: can't have two mutable borrows
        v1.push_str(" X");
        v2.push_str(" Y");
    }
}
```

**Problem:** You can't have two mutable references active simultaneously, even through the map!

### **When Mutable References Make Sense**

Very specialized scenarios where you need:

1. External ownership of data
2. Temporary mutation through map
3. Single-access pattern (one at a time)

```rust
struct Database {
    records: Vec<Record>,
}

impl Database {
    fn create_index(&mut self) -> HashMap<u64, &mut Record> {
        let mut index = HashMap::new();
        
        for record in &mut self.records {
            index.insert(record.id, record);
        }
        
        index
        // Lifetime: index borrows self.records mutably
        // Can't use self.records while index exists
    }
}
```

## ⚖️ Comparison: Ownership vs References

### **Memory Cost**

```rust
// By Value (Ownership)
map.insert(String::from("key"), String::from("value"));
// Stored: String struct (24 bytes) + heap data

// By Reference
map.insert("key", &value);
// Stored: pointer (8 bytes) to existing data
```

**Trade-off:**

- **References**: Less memory in map (just pointers)
- **Ownership**: More memory, but simpler lifetime management

### **Flexibility**

```rust
// ✅ Owned - modify anytime through map
let mut map = HashMap::new();
map.insert(String::from("key"), String::from("value"));
map.get_mut(&String::from("key")).unwrap().push_str("!");

// ⚠️ References - can't modify originals while map exists
let mut value = String::from("value");
let mut map: HashMap<&str, &String> = HashMap::new();
map.insert("key", &value);
// value.push_str("!");  // ❌ Error: value is borrowed
```

### **Lifetime Complexity**

```rust
// ✅ Owned - no lifetime annotations needed
fn create_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert(String::from("key"), String::from("value"));
    map  // Simple return
}

// ⚠️ References - lifetime annotations required
fn create_ref_map<'a>(data: &'a [String]) -> HashMap<&'a str, &'a String> {
    let mut map = HashMap::new();
    for s in data {
        map.insert(s.as_str(), s);
    }
    map
}
```

## 🎓 Copy Types: Special Case

### **Copy Types Don't Move**

```rust
let mut map = HashMap::new();

let key = 42;           // i32 implements Copy
let value = 100;        // i32 implements Copy

map.insert(key, value);  // Values COPIED, not moved

// ✅ Still accessible:
println!("Original: {} = {}", key, value);
println!("Map: {} = {}", key, map[&key]);

// Map has its own copy
```

**What's Copied:**

```
Stack:
  key: 42
  value: 100
  map.buckets ──┐
                │
Heap:           ↓
  Bucket: [(42, 100)]  ← Independent copies
```

**Types That Implement Copy:**

- All integer types (`i32`, `u64`, etc.)
- Floating point types (`f32`, `f64`)
- `bool`, `char`
- Tuples of Copy types: `(i32, bool)` ✅
- Arrays of Copy types: `[i32; 5]` ✅
- **NOT**: `String`, `Vec<T>`, most structs

## 🔍 Practical Patterns

### **Pattern: Configuration Map**

```rust
// Owned - configs live in map
let mut config: HashMap<String, String> = HashMap::new();
config.insert("host".to_string(), "localhost".to_string());
config.insert("port".to_string(), "8080".to_string());

// Can be passed around, modified, stored
fn apply_config(config: HashMap<String, String>) {
    // Takes ownership - simple!
}
```

### **Pattern: Temporary Index**

```rust
// References - index into existing data
fn build_index<'a>(items: &'a [Item]) -> HashMap<u64, &'a Item> {
    let mut index = HashMap::new();
    for item in items {
        index.insert(item.id, item);
    }
    index
}

// Index is cheap - just pointers
// But tied to items lifetime
```

### **Pattern: Frequency Counter**

```rust
// Owned keys, primitive values
let text = "hello world hello";
let mut frequencies: HashMap<String, usize> = HashMap::new();

for word in text.split_whitespace() {
    *frequencies.entry(word.to_string()).or_insert(0) += 1;
}

// Keys are owned Strings
// Values are Copy type (usize)
```

### **Pattern: Cache with Cloning**

```rust
// Clone on read to avoid borrow issues
let mut cache: HashMap<String, String> = HashMap::new();
cache.insert("key".to_string(), "expensive_value".to_string());

fn get_cached(cache: &HashMap<String, String>, key: &str) -> Option<String> {
    cache.get(key).cloned()  // Clone to return owned value
}

// Caller gets owned value, cache still has its copy
let value = get_cached(&cache, "key");
```

## ⚠️ Common Pitfalls

### **Pitfall 1: Trying to Modify While Borrowed**

```rust
let mut map: HashMap<&str, &String> = HashMap::new();
let mut value = String::from("Alice");

map.insert("name", &value);

// ❌ Can't modify while map holds reference
// value.push_str(" Smith");  // Error!

// ✅ Drop map first
drop(map);
value.push_str(" Smith");  // Now OK
```

### **Pitfall 2: Lifetime Too Short**

```rust
fn broken() -> HashMap<&str, &String> {
    let mut map = HashMap::new();
    let value = String::from("Alice");
    
    map.insert("name", &value);
    
    map  // ❌ Error: value dropped, reference dangles
}

// ✅ Fix: Return owned data
fn fixed() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("name".to_string(), "Alice".to_string());
    map
}
```

### **Pitfall 3: Unnecessary Cloning**

```rust
// ❌ Cloning when you don't need the original
let key = expensive_key_computation();
let value = expensive_value_computation();

map.insert(key.clone(), value.clone());
// Oops - key and value never used again!

// ✅ Just move them
map.insert(key, value);
```

### **Pitfall 4: Wrong Reference Type**

```rust
// ❌ Immutable reference when you need mutation
let mut map: HashMap<&str, &String> = HashMap::new();
let value = String::from("Alice");

map.insert("name", &value);

// Can't modify through immutable reference!
// map.get_mut(&"name").unwrap().push_str(" Smith");  // Wrong type!

// ✅ Use mutable reference or owned value
let mut map: HashMap<&str, &mut String> = HashMap::new();
// OR
let mut map: HashMap<String, String> = HashMap::new();
```

## 🎯 Decision Guide

**Use Owned Values (`T`) When:**

- ✅ HashMap should control lifetime
- ✅ Need to modify values through map
- ✅ Passing HashMap between functions
- ✅ Storing configuration or application state
- ✅ Default choice (simplest)

**Use Immutable References (`&T`) When:**

- ✅ Data outlives the map
- ✅ Creating temporary indexes
- ✅ Read-only aggregations
- ✅ Memory optimization (large values)
- ⚠️ Comfortable with lifetime annotations

**Use Mutable References (`&mut T`) When:**

- ⚠️ Rarely - very specialized scenarios
- ⚠️ External ownership required
- ⚠️ Single-access mutation pattern
- ❌ Not recommended for beginners

**Use Copy Types When:**

- ✅ Dealing with primitives (integers, bools)
- ✅ Small, cheap-to-copy data
- ✅ Want to keep original and copy in map

## 🧪 Testing Different Patterns

```rust
#[test]
fn test_owned_values() {
    let mut map = HashMap::new();
    map.insert(String::from("key"), String::from("value"));
    
    // Can modify through map
    map.get_mut(&String::from("key")).unwrap().push_str("!");
    assert_eq!(map[&String::from("key")], "value!");
}

#[test]
fn test_immutable_references() {
    let value = String::from("value");
    let mut map: HashMap<&str, &String> = HashMap::new();
    map.insert("key", &value);
    
    // Can't modify original or through map
    assert_eq!(map[&"key"], "value");
    assert_eq!(value, "value");  // Original unchanged
}

#[test]
fn test_mutable_references() {
    let mut value = String::from("value");
    let mut map: HashMap<&str, &mut String> = HashMap::new();
    map.insert("key", &mut value);
    
    // Modify through map
    map.get_mut(&"key").unwrap().push_str("!");
    
    // Drop map to regain access
    drop(map);
    
    // Original was modified
    assert_eq!(value, "value!");
}

#[test]
fn test_copy_types() {
    let key = 42;
    let value = 100;
    
    let mut map = HashMap::new();
    map.insert(key, value);
    
    // Both still accessible
    assert_eq!(key, 42);
    assert_eq!(value, 100);
    assert_eq!(map[&42], 100);
}
```

---

## 📦 Pattern 4: Memoization Caches with Composite State Keys

**Context:** Dynamic programming memoization (DP) uses HashMap to cache subproblem results. The ownership pattern depends on state representation.

**Developed in:** [[missions/mission-11|Mission 11]] - Dynamic Programming with Memoization  
**Real-world validation:** AoC 2023 Day 12 (Hot Springs), AoC 2024 Day 19 (Linen Layout)

### **Composite State Tuples (Owned Keys)**

For DP problems, state is typically a **tuple of primitive values** - all `Copy` types that get owned by the cache:

```rust
use std::collections::HashMap;

// AoC 2023 Day 12 Pattern: 3D state tuple
type State = (usize, usize, usize);  // (position, group_idx, current_run)
type Memo = HashMap<State, usize>;   // State → count of valid arrangements

fn count_arrangements(
    springs: &[u8],
    groups: &[usize],
    pos: usize,
    group_idx: usize,
    current_run: usize,
    memo: &mut Memo,
) -> usize {
    // Base case
    if pos == springs.len() {
        return if group_idx == groups.len() && current_run == 0 { 1 } else { 0 };
    }

    // Memoization check - tuple key is COPIED (all usize are Copy)
    let key = (pos, group_idx, current_run);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    // Compute result (branching DP logic)
    let mut count = 0;
    // ... DP logic ...

    // Cache result - key is copied, value is copied
    memo.insert(key, count);
    count
}
```

**Ownership Flow:**

```
Stack:
  pos: 7
  group_idx: 2
  current_run: 3
  
  key = (7, 2, 3)  ← Tuple created from COPIES

memo.insert(key, count):
  Heap HashMap:
    (7, 2, 3) → 42  ← HashMap owns INDEPENDENT copy of tuple
    
Stack variables (pos, group_idx, current_run) unchanged
```

**Why This Works:**

- ✅ **All primitives**: `usize`, `i32`, `u64` implement `Copy`
- ✅ **Tuples of Copy types**: Also implement `Copy`
- ✅ **No lifetime annotations**: Everything is owned, no borrowing
- ✅ **Zero overhead**: Copying small tuples is free (register operations)

### **Multi-Dimensional State Spaces**

```rust
// 2D State: Grid pathfinding
type State2D = (usize, usize);  // (row, col)
let mut memo: HashMap<State2D, bool> = HashMap::new();

// 3D State: Constraint satisfaction (AoC 2023 Day 12)
type State3D = (usize, usize, usize);  // (pos, group, run)
let mut memo: HashMap<State3D, usize> = HashMap::new();

// 4D State: Complex DP
type State4D = (usize, u64, usize, usize);  // (pos, value, count, depth)
let mut memo: HashMap<State4D, u64> = HashMap::new();

// All keys are owned, all types are Copy
// No lifetime management needed!
```

### **String Slice Keys with Lifetimes (Zero-Copy Pattern)**

For string-based DP (e.g., pattern matching), use **borrowed string slices** to avoid allocations:

```rust
use std::collections::HashMap;

// Zero-copy memoization with string slices
fn can_match<'a>(
    pattern: &'a str,
    towels: &[&str],
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    // Base case
    if pattern.is_empty() {
        return true;
    }

    // Memoization check - pattern is a REFERENCE
    if let Some(&cached) = memo.get(pattern) {
        return cached;
    }

    // Try each towel prefix
    for towel in towels {
        if let Some(remaining) = pattern.strip_prefix(towel) {
            // ✅ `remaining` is a SLICE of original `pattern`
            // No allocation! Just pointer arithmetic.
            if can_match(remaining, towels, memo) {
                memo.insert(pattern, true);
                return true;
            }
        }
    }

    memo.insert(pattern, false);
    false
}

// Usage
fn main() {
    let input = "rrbgbr";  // Lives on stack/static
    let towels = vec!["r", "rb", "g", "b", "br"];
    
    let mut memo: HashMap<&str, bool> = HashMap::new();
    let result = can_match(input, &towels, &mut memo);
    
    println!("Memo size: {}", memo.len());  // Shows subproblems explored
}
```

**Lifetime Requirement:**

```rust
// Pattern string must outlive the memo cache
fn solve(input: &str) -> bool {
    let mut memo: HashMap<&str, bool> = HashMap::new();
    //                    ^^^^ Lifetime tied to `input`
    
    can_match(input, towels, &mut memo)
    // ✅ memo dropped before input - safe!
}
```

**Why Zero-Copy Matters:**

```
Input: "abcdefgh"
       ┌────────┐
Memo:  │        │
  "abcdefgh" → false  ← Reference to original (no allocation)
   "bcdefgh" → false  ← Slice of original (no allocation)
    "cdefgh" → true   ← Slice of original (no allocation)
    
vs. String keys:

Memo:
  String("abcdefgh") → false  ← Heap allocation! 
  String("bcdefgh")  → false  ← Heap allocation!
  String("cdefgh")   → true   ← Heap allocation!
  
Zero-copy: 0 allocations
String keys: N allocations (where N = unique substrings)
```

**Performance Impact (AoC 2024 Day 19):**

- **With `&str` keys**: 0 allocations, 41.26ms
- **With `String` keys**: ~300K allocations, ~120ms (3x slower)

### **Generic MemoCache Pattern (Mission 11)**

**Production-ready wrapper** abstracting HashMap with statistics:

```rust
use std::collections::HashMap;
use std::hash::Hash;

pub struct MemoCache<K, V> 
where
    K: Hash + Eq,
    V: Clone,
{
    cache: HashMap<K, V>,
    hits: usize,
    misses: usize,
}

impl<K, V> MemoCache<K, V>
where
    K: Hash + Eq,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    pub fn memoize<F>(&mut self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(cached) = self.cache.get(&key) {
            self.hits += 1;
            return cached.clone();
        }

        self.misses += 1;
        let value = compute();
        self.cache.insert(key, value.clone());
        value
    }

    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 { 0.0 } else { self.hits as f64 / total as f64 }
    }
}

// Usage with composite state
let mut cache = MemoCache::<(usize, usize, usize), usize>::new();
let result = cache.memoize((7, 2, 3), || expensive_computation(7, 2, 3));

println!("Hit ratio: {:.2}%", cache.hit_ratio() * 100.0);
// AoC 2023 Day 12 Part 2: 95% hit ratio (massive overlap!)
```

**Ownership in MemoCache:**

- **Keys**: Generic `K` - can be owned tuples, references, or custom types
- **Values**: Must be `Clone` - HashMap stores owned copy
- **Statistics**: Track memoization effectiveness (hit ratio reveals problem structure)

### **Custom State Structs (Hash + Eq)**

For complex state, use custom structs:

```rust
#[derive(Hash, Eq, PartialEq, Clone)]
struct DPState {
    position: usize,
    remaining_items: Vec<usize>,
    current_value: u64,
}

let mut memo: HashMap<DPState, bool> = HashMap::new();

let state = DPState {
    position: 5,
    remaining_items: vec![1, 2, 3],
    current_value: 100,
};

memo.insert(state, true);  // Struct is MOVED (not Copy)
```

**⚠️ Performance Warning:**

```rust
// ❌ BAD - Vec<usize> in key is expensive to hash and clone
#[derive(Hash, Eq, PartialEq, Clone)]
struct ExpensiveState {
    items: Vec<usize>,  // Hashing this is O(n)!
}

// ✅ BETTER - Use bitmask or index instead
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct EfficientState {
    items_mask: u64,  // Bitmask for small sets, O(1) hash
}

// Or reference to external storage
#[derive(Hash, Eq, PartialEq)]
struct RefState<'a> {
    items: &'a [usize],  // Reference - no cloning
}
```

### **Key Ownership Decision Matrix**

| **State Type** | **Key Ownership** | **Lifetime Annotations** | **Performance** | **Use Case** |
|----------------|-------------------|-------------------------|----------------|--------------|
| Primitives (`usize`, `i32`) | Owned (`Copy`) | None | ⚡ Excellent | Positions, indices, counts |
| Tuples of primitives | Owned (`Copy`) | None | ⚡ Excellent | Multi-dimensional DP state |
| String slices (`&str`) | Borrowed | `<'a>` required | ⚡ Excellent (zero-copy) | Pattern matching, substring DP |
| Custom structs (small) | Owned (moved) | None | ⚡ Good | Complex state with few fields |
| Custom structs (large) | Reference (`&T`) | `<'a>` required | ⚠️ Depends on cloning | Avoid if possible |
| Vec/String in key | ❌ Avoid | - | ❌ Poor | Hash/clone overhead kills performance |

### **Summary: Memoization Cache Ownership**

**Golden Rules:**

1. **Prefer owned primitive tuples** - Copy types, zero overhead, no lifetimes
2. **Use string slices for zero-copy** - Requires `<'a>` lifetime but eliminates allocations
3. **Avoid Vec/String in keys** - Expensive to hash and clone
4. **Track hit ratios** - Low ratio (<50%) suggests memoization isn't helping
5. **State space size matters** - Cache size reveals problem complexity

**Real-World Impact:**

- **AoC 2023 Day 12**: 300K states cached, 95% hit ratio, 41.26ms
- **Fibonacci(90)**: 89 states cached, instant vs hours without memoization
- **Zero-copy vs String keys**: 3x speedup, zero allocations

See [[memoization-comprehensive-guide]] for complete DP patterns and [[missions/mission-11|Mission 11]] for production implementation.

---

## 🔗 Mission 5 Implications

When implementing your own HashMap:

**Type Parameters:**

```rust
pub struct MyHashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,  // Stores OWNED keys and values
}

// Or with references:
pub struct MyHashMap<'a, K, V> {
    buckets: Vec<Vec<(&'a K, &'a V)>>,  // Stores references
    // Need lifetime parameter!
}
```

**Implementation Choice:**

- **Recommended**: Store owned values `(K, V)` for simplicity
- **Advanced**: Support references with lifetime parameters
- **Most Flexible**: Generic over storage strategy (like std::HashMap with `S: BuildHasher`)

---

## 🏷️ Tags & Links

*Tags: #hashmap #ownership #borrowing #references #lifetimes #memory-management #mission5 #rust-patterns #copy-trait*

*Links: [[zettel-index]] | [[ownership]] | [[Borrow Checker Fundamentals]] | [[lifetimes]] | [[mission-5]] | [[entry-api-hashmap]] | [[rust-book-ch8]] | [[copy-trait]]*

*Related Concepts:*

- [[ownership]] - Fundamental ownership system
- [[Borrow Checker Fundamentals]] - Reference rules and constraints
- [[lifetimes]] - Reference lifetime annotations
- [[copy-trait]] - Copy vs Move semantics
- [[entry-api-hashmap]] - Efficient HashMap access patterns
- [[hashmap-internals]] - Internal implementation details

*Related Daily Study:*

- [[daily-study/Day02]] - Ownership basics
- [[daily-study/Day03]] - Borrowing and references
- [[daily-study/Day04]] - Lifetime fundamentals
- [[daily-study/Day10]] - HashMap operations

*Practical Applications:*

- Configuration storage (owned values)
- Temporary indexes (immutable references)
- Data aggregation (owned or references depending on use case)
- Caching (owned values with cloning on access)

---

*Created: October 12, 2025*  
*Context: Deep dive into HashMap ownership patterns, references vs owned values*  
*Source: Rust Book Chapter 8.3, Example 4 ownership discussion*
