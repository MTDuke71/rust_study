# HashMap Entry API - Efficient Single-Lookup Patterns

> **Core Pattern**: The Entry API provides efficient HashMap access by performing hash computation once, returning an enum that represents the map position for subsequent operations.

## 🎯 The Problem It Solves

### **Naive Approach - Double Lookup**

```rust
// ❌ INEFFICIENT - Two expensive hash computations!
if !map.contains_key(&key) {      // Hash + Lookup #1
    map.insert(key, value);        // Hash + Lookup #2
}

// ❌ ALSO INEFFICIENT
if let Some(v) = map.get_mut(&key) {  // Hash + Lookup #1
    *v += 1;
} else {
    map.insert(key, 1);                // Hash + Lookup #2
}
```

**Performance Issue**: Each operation hashes the key and searches the bucket chain. For complex keys or hash collisions, this overhead multiplies.

### **Entry API - Single Lookup**

```rust
// ✅ EFFICIENT - One hash computation, one lookup!
*map.entry(key).or_insert(0) += 1;
```

**Performance Win**: Hash computed once, position remembered, operation performed. This is a **zero-cost abstraction** - compiles to optimal machine code.

## 🏗️ Entry Enum Structure

The `entry()` method returns an `Entry<'a, K, V>` enum with two variants:

```rust
pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),  // Key exists in map
    Vacant(VacantEntry<'a, K, V>),      // Key does not exist
}
```

**Key Insight**: The Entry holds a reference to the map's internal position. It "remembers" where the key is (or should be) without re-hashing.

## 📚 Core Methods

### **1. `or_insert(default: V) -> &mut V`**

Inserts default if key is vacant, returns mutable reference to value.

```rust
let count = map.entry(word).or_insert(0);
*count += 1;
```

**Use Case**: Frequency counting, initialization with defaults

**Behavior**:
- **Occupied**: Returns `&mut V` to existing value (no modification)
- **Vacant**: Inserts default, returns `&mut V` to newly inserted value

### **2. `or_insert_with<F>(f: F) -> &mut V` where `F: FnOnce() -> V`**

Lazy version - closure only runs if key is vacant.

```rust
let result = cache
    .entry(input)
    .or_insert_with(|| expensive_computation(input));
```

**Use Case**: Memoization, avoiding expensive default computation

**Why Better Than `or_insert`**:
- Closure not evaluated if key exists
- Critical for expensive operations
- Enables side effects only when needed

**Example - Avoiding Waste**:
```rust
// ❌ Default always computed even if not needed
map.entry(key).or_insert(expensive_function());

// ✅ Default only computed when key missing
map.entry(key).or_insert_with(|| expensive_function());
```

### **3. `and_modify<F>(f: F) -> Entry<'a, K, V>` where `F: FnOnce(&mut V)`**

Modifies value if occupied, returns Entry for chaining.

```rust
map.entry(key)
    .and_modify(|v| *v += 1)  // Only if key exists
    .or_insert(1);             // Otherwise insert 1
```

**Use Case**: Increment existing, initialize missing (counter pattern)

**Chaining Pattern**:
```rust
map.entry(key)
    .and_modify(|count| *count += 1)
    .or_insert(0);
```

This is more efficient than separate get/insert for "update or create" patterns.

### **4. `key(&self) -> &K`**

Returns reference to the entry's key.

```rust
let entry = map.entry(my_key);
let key_ref = entry.key();
```

**Use Case**: Accessing key when working with Entry, debug logging

## 🎨 Common Patterns

### **Pattern 1: Frequency Counter**

```rust
let mut frequencies = HashMap::new();

for word in text.split_whitespace() {
    *frequencies.entry(word).or_insert(0) += 1;
}
```

**Why Optimal**:
- Single hash per word
- No conditional logic needed
- Returns mutable reference for immediate increment

### **Pattern 2: Grouping Data**

```rust
let mut groups: HashMap<&str, Vec<&str>> = HashMap::new();

for (name, category) in items {
    groups.entry(category)
          .or_insert(Vec::new())
          .push(name);
}
```

**Why Optimal**:
- Creates empty Vec only when needed
- Returns mutable reference to Vec for push
- One lookup per item

### **Pattern 3: Memoization Cache**

```rust
let mut cache = HashMap::new();

let result = cache
    .entry(input)
    .or_insert_with(|| {
        println!("Cache miss - computing...");
        expensive_computation(input)
    });
```

**Why Optimal**:
- Computation only happens on cache miss
- Closure captures context if needed
- Clear cache hit/miss semantics

### **Pattern 4: Conditional Update**

```rust
map.entry(key)
    .and_modify(|e| *e += 10)  // Bonus for existing customers
    .or_insert(100);            // New customer gets base amount
```

**Why Optimal**:
- Different logic for existing vs new
- Single lookup
- Chainable operations

### **Pattern 5: Complex Initialization**

```rust
map.entry(key).or_insert_with(|| {
    let mut state = ComplexState::new();
    state.initialize();
    state.configure(params);
    state
});
```

**Why Optimal**:
- Multi-statement initialization
- Only runs when needed
- Type inference works naturally

## 🔍 Advanced Entry Operations

### **Pattern Matching on Entry**

```rust
match map.entry(key) {
    Entry::Occupied(mut entry) => {
        println!("Found: {}", entry.get());
        *entry.get_mut() += 1;
        // or: entry.remove();
    }
    Entry::Vacant(entry) => {
        entry.insert(default_value);
        println!("Inserted new value");
    }
}
```

**When to Use**:
- Need different behavior for occupied vs vacant
- Want to remove occupied entries conditionally
- Need to log or report state

### **OccupiedEntry Methods**

```rust
if let Entry::Occupied(mut e) = map.entry(key) {
    let value_ref = e.get();           // &V
    let value_mut = e.get_mut();       // &mut V
    let key_ref = e.key();             // &K
    let removed = e.remove();          // V (consumes entry)
    let (k, v) = e.remove_entry();     // (K, V) (returns both)
}
```

### **VacantEntry Methods**

```rust
if let Entry::Vacant(e) = map.entry(key) {
    let key_ref = e.key();             // &K
    let inserted = e.insert(value);    // &mut V (to new value)
}
```

## ⚡ Performance Characteristics

### **Hash Computation Cost**

For a type like `String`:
```rust
// ❌ Two hash computations (~100-200 CPU cycles each)
if !map.contains_key(&key) {
    map.insert(key, value);
}

// ✅ One hash computation
map.entry(key).or_insert(value);
```

**Savings**: 50% reduction in hash computations, more with complex keys.

### **Bucket Search Cost**

With hash collisions:
```rust
// ❌ Two bucket chain traversals
if let Some(v) = map.get_mut(&key) {
    *v += 1;
} else {
    map.insert(key, 1);
}

// ✅ One bucket chain traversal
*map.entry(key).or_insert(0) += 1;
```

**Savings**: Eliminates redundant linear probe or chain walk.

### **Time Complexity**

- **Without Entry API**: O(1) amortized × 2 = 2 hash computations + 2 lookups
- **With Entry API**: O(1) amortized × 1 = 1 hash computation + 1 lookup

**Cache Efficiency**: Entry API has better cache locality since the position is already in cache from the first lookup.

## 🎓 Mission 5 Implementation Challenge

When implementing your own HashMap in Mission 5, the Entry API is one of the most challenging features:

### **Core Implementation Challenge**

```rust
pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
    // 1. Hash the key
    let hash = self.hash(&key);
    let index = hash % self.capacity();
    
    // 2. Search bucket (remember position)
    // Need to return enum that "remembers" this position
    // without borrowing self mutably twice
    
    // 3. Return Entry enum with lifetime tied to self
}
```

**Difficulty**: The Entry must hold a mutable reference to the HashMap's internal data structure while allowing subsequent operations. This requires careful lifetime management and potentially unsafe code.

### **Why It's Hard**

1. **Lifetime Constraints**: Entry<'a, K, V> needs `'a` tied to HashMap
2. **Interior Mutability**: Entry methods need to mutate map state
3. **Avoiding Double Borrow**: Can't borrow map mutably while Entry exists
4. **Position Tracking**: Must remember bucket index and chain position

## 🔗 Connections to Other Concepts

### **Ownership Implications**

```rust
// Entry takes ownership of key if vacant
let key = String::from("hello");
map.entry(key).or_insert(10);
// `key` is moved (if vacant) or dropped (if occupied)
```

**Design Decision**: Entry API takes ownership to avoid cloning keys that won't be inserted.

### **Borrowing Rules**

```rust
let mut map = HashMap::new();
map.insert("a", 1);

let entry = map.entry("b");
// map is now borrowed mutably through entry
// println!("{}", map.get("a")); // ❌ Won't compile!

*entry.or_insert(2) += 1;
// entry goes out of scope, borrow ends

println!("{}", map.get("a").unwrap()); // ✅ Now works
```

**Lesson**: Entry holds a mutable borrow of the entire map until it goes out of scope.

### **Iterator Invalidation Prevention**

```rust
let mut map = HashMap::new();
map.insert("a", 1);

for key in map.keys() {
    // map.entry(key).or_insert(2); // ❌ Won't compile!
    // Can't mutate map while iterating
}
```

**Safety**: Rust prevents iterator invalidation at compile time through borrowing rules.

## 📊 Real-World Use Cases

### **1. Web Server Request Tracking**

```rust
let mut request_counts: HashMap<IpAddr, u32> = HashMap::new();

fn handle_request(ip: IpAddr) {
    *request_counts.entry(ip).or_insert(0) += 1;
    
    if request_counts[&ip] > RATE_LIMIT {
        return Err(TooManyRequests);
    }
}
```

### **2. Game Entity Management**

```rust
let mut entities: HashMap<EntityId, Entity> = HashMap::new();

entities.entry(id)
    .and_modify(|e| e.update(delta))
    .or_insert_with(|| Entity::spawn_at(position));
```

### **3. Configuration Merging**

```rust
fn merge_configs(base: &HashMap<String, Value>, 
                 override_cfg: HashMap<String, Value>) 
                 -> HashMap<String, Value> {
    let mut merged = base.clone();
    
    for (key, value) in override_cfg {
        merged.insert(key, value);  // Simple overwrite
    }
    
    merged
}

// Better with Entry API for conditional merge:
fn smart_merge(base: &HashMap<String, Value>,
               override_cfg: HashMap<String, Value>)
               -> HashMap<String, Value> {
    let mut merged = base.clone();
    
    for (key, value) in override_cfg {
        merged.entry(key)
            .and_modify(|v| v.merge(&value))  // Custom merge
            .or_insert(value);                 // Use override if not in base
    }
    
    merged
}
```

### **4. Log Aggregation**

```rust
let mut logs_by_level: HashMap<LogLevel, Vec<LogEntry>> = HashMap::new();

logs_by_level
    .entry(entry.level)
    .or_insert_with(Vec::new)
    .push(entry);
```

### **5. Graph Adjacency List**

```rust
let mut graph: HashMap<NodeId, Vec<NodeId>> = HashMap::new();

graph.entry(from)
     .or_insert_with(Vec::new)
     .push(to);
```

## ⚠️ Common Pitfalls

### **Pitfall 1: Cloning When Not Needed**

```rust
// ❌ Unnecessary clone
let key = expensive_key.clone();
if map.contains_key(&key) {
    // key clone wasted if it exists
}

// ✅ Entry API moves key only if needed
map.entry(expensive_key).or_insert(value);
```

### **Pitfall 2: Using `or_insert` with Expensive Defaults**

```rust
// ❌ Default always computed
map.entry(key).or_insert(expensive_default());

// ✅ Default only computed when needed
map.entry(key).or_insert_with(|| expensive_default());
```

### **Pitfall 3: Holding Entry Too Long**

```rust
// ❌ Blocks other map operations
let entry = map.entry(key);
some_long_operation();
entry.or_insert(value);  // Borrow held entire time

// ✅ Use entry immediately
let value = map.entry(key).or_insert(default);
some_long_operation();
```

### **Pitfall 4: Pattern Matching Doesn't Consume Entry**

```rust
match map.entry(key) {
    Entry::Occupied(e) => {
        // e is OccupiedEntry, not Entry
        // Can call e.get(), e.get_mut(), e.remove()
    }
    Entry::Vacant(e) => {
        // e is VacantEntry, not Entry
        // Can call e.insert()
    }
}
// Both branches must handle their specific entry type
```

## 🎯 Key Takeaways

1. **Entry API performs hash computation once** - Eliminates redundant work in check-then-modify patterns
2. **Returns enum representing map position** - Occupied or Vacant, both enable efficient operations
3. **`or_insert()` for simple defaults** - Use when default is cheap to create
4. **`or_insert_with()` for expensive defaults** - Use closures for lazy evaluation
5. **`and_modify()` enables chaining** - Update existing or insert new in one expression
6. **Zero-cost abstraction** - Compiles to optimal machine code, no runtime overhead
7. **Essential for Mission 5** - Implementing Entry API is a key learning challenge
8. **Frequency counting canonical use case** - `*map.entry(key).or_insert(0) += 1` is idiomatic Rust

## 🧪 Testing Entry API Behavior

```rust
#[test]
fn test_entry_api_vacant() {
    let mut map = HashMap::new();
    
    let entry = map.entry("key");
    assert!(matches!(entry, Entry::Vacant(_)));
    
    let value = entry.or_insert(42);
    assert_eq!(*value, 42);
    assert_eq!(map.get("key"), Some(&42));
}

#[test]
fn test_entry_api_occupied() {
    let mut map = HashMap::new();
    map.insert("key", 42);
    
    let entry = map.entry("key");
    assert!(matches!(entry, Entry::Occupied(_)));
    
    let value = entry.or_insert(100);  // Won't overwrite
    assert_eq!(*value, 42);            // Still original value
}

#[test]
fn test_entry_modify_and_insert() {
    let mut map = HashMap::new();
    
    // First access - vacant, inserts 1
    map.entry("a").and_modify(|v| *v += 1).or_insert(1);
    assert_eq!(map["a"], 1);
    
    // Second access - occupied, increments to 2
    map.entry("a").and_modify(|v| *v += 1).or_insert(1);
    assert_eq!(map["a"], 2);
}
```

---

## 🏷️ Tags & Links

*Tags: #hashmap #entry-api #performance #optimization #zero-cost-abstraction #mission5 #collections #rust-patterns #api-design*

*Links: [[zettel-index]] | [[mission-5]] | [[rust-book-ch8]] | [[hashmap-internals]] | [[daily-study/Day10]] | [[rust-book-ch5-8-review]]*

*Related Concepts:*
- [[hashmap-internals]] - Understanding hash computation and bucket chains
- [[or-insert-pattern]] - Common frequency counting idiom
- [[memoization-patterns]] - Caching with `or_insert_with`
- [[ownership]] - Entry API ownership of keys
- [[Borrow Checker Fundamentals]] - Entry holds mutable borrow of map
- [[zero-cost-abstractions]] - Entry API compiles to optimal code

*Related Mission Work:*
- [[mission-5]] - Implementing Entry API is a key Mission 5 challenge
- [[daily-study/Day10]] - HashMap fundamentals and Entry API usage

*Real-World Applications:*
- Frequency analysis (word count, character frequency)
- Request tracking and rate limiting
- Configuration merging and defaults
- Graph adjacency list construction
- Memoization and caching patterns

---

*Created: October 12, 2025*  
*Context: Deep dive into HashMap Entry API from Rust Book Chapter 8.3*  
*Source: [[rust-book-ch8]] - Common Collections, Hash Maps section*
