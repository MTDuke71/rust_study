# Collision Resolution

**Tags**: #hash #collision #data_structures #algorithms #rust #performance

## 🎯 Core Idea

A **collision** occurs when two different keys produce the same hash code and therefore map to the same bucket in a hash table. Since this is inevitable (by the pigeonhole principle—unlimited possible keys, finite number of buckets), hash tables must have a strategy for resolving these conflicts.

The collision resolution strategy you choose fundamentally impacts the performance, memory usage, and implementation complexity of your hash table. The two main approaches are **chaining** (storing multiple entries per bucket) and **open addressing** (probing for an alternative bucket).

---

## ⚙️ Key Collision Resolution Strategies

### 1. **Chaining (Separate Chaining)**

Each bucket in the hash table contains a linked list (or vector) of all key-value pairs that hash to that bucket.

**How it works**:
1. Hash the key to get the bucket index.
2. If the bucket is empty, insert the entry.
3. If the bucket already has entries, append or prepend to the list.
4. For lookups, search through the list in the bucket.

**Advantages**:
- Simple to implement and understand.
- Handles high load factors gracefully (performance degrades gradually).
- Deletion is straightforward—just remove from the list.

**Disadvantages**:
- Extra memory overhead for pointers/metadata.
- Cache-unfriendly due to pointer chasing.
- Worst-case lookup is O(n) if all keys hash to the same bucket.

**Rust Implementation Pattern**:
```rust
use std::collections::HashMap;

struct ChainingHashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,  // Each bucket is a Vec of entries
    size: usize,
    capacity: usize,
}

impl<K, V> ChainingHashMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key) % self.capacity;
        
        // Check if key already exists in the chain
        for entry in &mut self.buckets[index] {
            if entry.0 == key {
                entry.1 = value; // Update existing
                return;
            }
        }
        
        // Key not found, add new entry
        self.buckets[index].push((key, value));
        self.size += 1;
    }
    
    fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash(key) % self.capacity;
        
        // Linear search through the chain
        self.buckets[index]
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }
    
    fn hash(&self, key: &K) -> usize {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize
    }
}
```

### 2. **Open Addressing (Closed Hashing)**

All entries are stored directly in the bucket array. When a collision occurs, the algorithm "probes" for the next available slot using a deterministic sequence.

**Common Probing Methods**:

#### a. **Linear Probing**
- **Formula**: `index = (hash + i) % capacity` where `i = 0, 1, 2, 3, ...`
- **Behavior**: Check the next slot sequentially until an empty one is found.
- **Issue**: Causes "primary clustering"—long runs of occupied slots that slow down further insertions.

#### b. **Quadratic Probing**
- **Formula**: `index = (hash + i²) % capacity` where `i = 0, 1, 4, 9, ...`
- **Behavior**: Jump by increasing distances to avoid primary clustering.
- **Issue**: Can cause "secondary clustering" and may fail to find an empty slot if the table is more than half full.

#### c. **Double Hashing**
- **Formula**: `index = (hash1 + i * hash2) % capacity`
- **Behavior**: Use a second hash function to determine the probe step size.
- **Advantage**: Minimizes clustering, provides better distribution.

**Advantages of Open Addressing**:
- Better cache locality (all data in one contiguous array).
- No extra memory for pointers.
- Can be faster for small tables with low load factors.

**Disadvantages**:
- Deletion is complex (requires "tombstone" markers or rehashing).
- Performance degrades sharply as the table fills (must keep load factor low, e.g., < 0.5).
- More complex implementation.

**Rust Implementation Pattern (Linear Probing)**:
```rust
#[derive(Clone)]
enum Slot<K, V> {
    Empty,
    Occupied(K, V),
    Tombstone, // Marks a deleted entry
}

struct OpenAddressingHashMap<K, V> {
    slots: Vec<Slot<K, V>>,
    size: usize,
    capacity: usize,
}

impl<K, V> OpenAddressingHashMap<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    fn insert(&mut self, key: K, value: V) {
        let mut index = self.hash(&key) % self.capacity;
        
        // Linear probing
        for _ in 0..self.capacity {
            match &self.slots[index] {
                Slot::Empty | Slot::Tombstone => {
                    self.slots[index] = Slot::Occupied(key, value);
                    self.size += 1;
                    return;
                }
                Slot::Occupied(k, _) if k == &key => {
                    // Update existing key
                    self.slots[index] = Slot::Occupied(key, value);
                    return;
                }
                _ => {
                    // Collision, probe next slot
                    index = (index + 1) % self.capacity;
                }
            }
        }
        
        panic!("Hash table is full!");
    }
    
    fn get(&self, key: &K) -> Option<&V> {
        let mut index = self.hash(key) % self.capacity;
        
        for _ in 0..self.capacity {
            match &self.slots[index] {
                Slot::Occupied(k, v) if k == key => return Some(v),
                Slot::Empty => return None, // Key not found
                _ => index = (index + 1) % self.capacity, // Keep probing
            }
        }
        
        None
    }
    
    fn hash(&self, key: &K) -> usize {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize
    }
}
```

---

## 📊 Performance Comparison

| Strategy | Insert (Avg) | Lookup (Avg) | Delete | Memory Overhead | Cache Friendly |
|----------|--------------|--------------|--------|-----------------|----------------|
| **Chaining** | O(1) | O(1) | Simple | High (pointers) | Low (pointer chasing) |
| **Linear Probing** | O(1)* | O(1)* | Complex | Low | High (contiguous) |
| **Quadratic Probing** | O(1)* | O(1)* | Complex | Low | Medium |
| **Double Hashing** | O(1)* | O(1)* | Complex | Low | Medium |

*Performance degrades as load factor increases. Open addressing requires load factor < 0.7 for good performance.

---

## ❓ Key Questions & Trade-offs

**When should you use chaining?**
- When deletions are frequent.
- When you expect a high load factor (> 0.75).
- When simplicity and maintainability are priorities.
- This is the approach used in [[Mission5 HashMap]].

**When should you use open addressing?**
- When memory is constrained.
- When you can keep the load factor low (< 0.5).
- When cache performance is critical (e.g., high-frequency lookups).
- When deletions are rare.

**Why does Rust's `std::collections::HashMap` use a hybrid approach?**
- Rust's standard library uses a variant called **Robin Hood hashing** (a form of open addressing with additional optimizations).
- It provides excellent average-case performance while mitigating the worst-case scenarios of pure open addressing.

---

## 🔗 Related Concepts

*   **Primary Dependency**: [[Hash Function Design]], [[HashMap Internals]]
*   **Performance Analysis**: [[Load Factor Management]], [[Performance Analysis]]
*   **Implementation Examples**: [[Mission5 HashMap]] (uses chaining)
*   **Advanced Topics**: Robin Hood Hashing, Cuckoo Hashing, Hopscotch Hashing
*   **Broader Context**: [[Collections MOC]], [[rust-concepts-MOC]]

---

*Related Links: [[Hash Function Design]] | [[HashMap Internals]] | [[Mission5 Overview]]*
