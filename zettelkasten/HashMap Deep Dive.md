# HashMap Deep Dive

*Created: 2025-11-08*
*Tags: #rust-collections #hashmap #data-structures #performance #hash-functions #collision-resolution*

## Overview

HashMap in Rust is a **hash table implementation** providing average O(1) insertion, deletion, and lookup operations. It uses **Robin Hood hashing** with **linear probing** for excellent performance characteristics and **SipHash** by default for security against hash collision attacks.

## Core Architecture

### Internal Structure
```rust
use std::collections::HashMap;

// Conceptual internal structure (simplified)
struct HashMapInternal<K, V> {
    buckets: Vec<Option<(K, V, u64)>>, // (key, value, hash_value)
    len: usize,
    capacity: usize,
    load_factor_threshold: f64, // Typically 0.75
}
```

### Hash Function and Bucket Selection
```rust
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn demonstrate_hashing<K: Hash>(key: &K) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

// Bucket selection process
fn bucket_index(hash: u64, capacity: usize) -> usize {
    (hash as usize) % capacity // Modulo for bucket selection
}
```

## Robin Hood Hashing Implementation

### Collision Resolution Strategy
```rust
// Robin Hood hashing - steals from rich buckets to give to poor ones
// Distance = how far element is from its ideal position

struct RobinHoodEntry<K, V> {
    key: K,
    value: V,
    hash: u64,
    distance: usize, // Distance from ideal position
}

// Insertion algorithm concept
fn robin_hood_insert<K, V>(
    buckets: &mut Vec<Option<RobinHoodEntry<K, V>>>,
    mut entry: RobinHoodEntry<K, V>
) {
    let mut index = bucket_index(entry.hash, buckets.len());
    
    loop {
        match &buckets[index] {
            None => {
                // Empty slot - insert here
                buckets[index] = Some(entry);
                break;
            }
            Some(existing) => {
                if existing.distance < entry.distance {
                    // Rob from the rich - swap entries
                    let old_entry = buckets[index].take().unwrap();
                    buckets[index] = Some(entry);
                    entry = old_entry;
                    entry.distance += 1;
                } else {
                    // Continue probing
                    entry.distance += 1;
                }
            }
        }
        index = (index + 1) % buckets.len();
    }
}
```

### Performance Characteristics
```rust
use std::collections::HashMap;
use criterion::{black_box, Criterion};

fn benchmark_hashmap_operations(c: &mut Criterion) {
    let mut map = HashMap::new();
    
    // Pre-populate map
    for i in 0..10000 {
        map.insert(i, i * 2);
    }
    
    c.bench_function("hashmap_lookup", |b| {
        b.iter(|| {
            for i in 0..1000 {
                black_box(map.get(&i));
            }
        });
    });
    
    c.bench_function("hashmap_insert", |b| {
        b.iter(|| {
            let mut local_map = HashMap::new();
            for i in 0..1000 {
                local_map.insert(black_box(i), black_box(i * 2));
            }
        });
    });
}
```

## Mission 5 Integration

### Generic HashMap Implementation
```rust
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct MyHashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>, // Chaining approach for simplicity
    size: usize,
    capacity: usize,
}

impl<K, V> MyHashMap<K, V> 
where 
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self::with_capacity(16)
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        let mut buckets = Vec::with_capacity(capacity);
        buckets.resize_with(capacity, Vec::new);
        
        Self {
            buckets,
            size: 0,
            capacity,
        }
    }
    
    fn hash_key(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }
    
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.size >= self.capacity * 3 / 4 {
            self.resize();
        }
        
        let index = self.hash_key(&key);
        let bucket = &mut self.buckets[index];
        
        // Check if key already exists
        for (existing_key, existing_value) in bucket.iter_mut() {
            if *existing_key == key {
                return Some(std::mem::replace(existing_value, value));
            }
        }
        
        // Key doesn't exist - add new entry
        bucket.push((key, value));
        self.size += 1;
        None
    }
    
    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash_key(key);
        let bucket = &self.buckets[index];
        
        bucket.iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }
    
    fn resize(&mut self) {
        let old_buckets = std::mem::replace(
            &mut self.buckets, 
            {
                let new_capacity = self.capacity * 2;
                let mut new_buckets = Vec::with_capacity(new_capacity);
                new_buckets.resize_with(new_capacity, Vec::new);
                new_buckets
            }
        );
        
        self.capacity *= 2;
        self.size = 0;
        
        // Rehash all elements
        for bucket in old_buckets {
            for (key, value) in bucket {
                self.insert(key, value);
            }
        }
    }
}
```

## AoC HashMap Usage Patterns

### Day 6: Light Grid Optimization
```rust
use std::collections::HashMap;

// Sparse grid representation using HashMap
struct LightGrid {
    lights: HashMap<(usize, usize), i32>, // Only store non-zero lights
    default_brightness: i32,
}

impl LightGrid {
    fn new() -> Self {
        Self {
            lights: HashMap::new(),
            default_brightness: 0,
        }
    }
    
    fn toggle(&mut self, row: usize, col: usize) {
        let current = self.lights.get(&(row, col)).unwrap_or(&self.default_brightness);
        let new_value = current + 2;
        
        if new_value == self.default_brightness {
            self.lights.remove(&(row, col)); // Remove if back to default
        } else {
            self.lights.insert((row, col), new_value);
        }
    }
    
    fn total_brightness(&self) -> i32 {
        self.lights.values().sum::<i32>() + 
        (1_000_000 - self.lights.len() as i32) * self.default_brightness
    }
}
```

### Day 12: JSON Object Processing
```rust
use std::collections::HashMap;
use serde_json::Value;

fn sum_numbers_excluding_red(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap_or(0),
        Value::Array(arr) => {
            arr.iter().map(sum_numbers_excluding_red).sum()
        }
        Value::Object(obj) => {
            // Use HashMap iteration to check for "red" values
            let has_red = obj.values().any(|v| {
                v.as_str() == Some("red")
            });
            
            if has_red {
                0 // Skip objects containing "red"
            } else {
                obj.values().map(sum_numbers_excluding_red).sum()
            }
        }
        _ => 0,
    }
}
```

### Day 16: Aunt Sue Detection
```rust
use std::collections::HashMap;

#[derive(Debug)]
struct AuntSue {
    number: u32,
    properties: HashMap<String, u32>,
}

impl AuntSue {
    fn matches_mfcsam(&self, target: &HashMap<String, u32>) -> bool {
        self.properties.iter().all(|(prop, &value)| {
            match target.get(prop) {
                Some(&target_value) => {
                    match prop.as_str() {
                        "cats" | "trees" => value > target_value,
                        "pomeranians" | "goldfish" => value < target_value,
                        _ => value == target_value,
                    }
                }
                None => false,
            }
        })
    }
}

fn find_matching_aunt(aunts: &[AuntSue], mfcsam: &HashMap<String, u32>) -> Option<u32> {
    aunts.iter()
        .find(|aunt| aunt.matches_mfcsam(mfcsam))
        .map(|aunt| aunt.number)
}
```

## Advanced HashMap Patterns

### Custom Hash Functions
```rust
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};

// Fast hash for integer keys
struct FastIntHasher(u64);

impl Hasher for FastIntHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 = self.0.wrapping_mul(31).wrapping_add(byte as u64);
        }
    }
    
    fn finish(&self) -> u64 {
        self.0
    }
}

struct FastIntBuildHasher;

impl BuildHasher for FastIntBuildHasher {
    type Hasher = FastIntHasher;
    
    fn build_hasher(&self) -> Self::Hasher {
        FastIntHasher(0)
    }
}

// Usage for specific patterns
fn create_int_map() -> HashMap<i32, String, FastIntBuildHasher> {
    HashMap::with_hasher(FastIntBuildHasher)
}
```

### Memory-Efficient String Keys
```rust
use std::collections::HashMap;
use std::rc::Rc;

// Interned strings to reduce memory usage
struct StringInterner {
    map: HashMap<Rc<str>, u32>,
    next_id: u32,
}

impl StringInterner {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            next_id: 0,
        }
    }
    
    fn intern(&mut self, s: &str) -> Rc<str> {
        // Check if string already exists
        for key in self.map.keys() {
            if key.as_ref() == s {
                return key.clone();
            }
        }
        
        // Create new interned string
        let interned: Rc<str> = s.into();
        self.map.insert(interned.clone(), self.next_id);
        self.next_id += 1;
        interned
    }
    
    fn get_id(&self, s: &str) -> Option<u32> {
        self.map.iter()
            .find(|(key, _)| key.as_ref() == s)
            .map(|(_, &id)| id)
    }
}
```

### Entry API Patterns
```rust
use std::collections::HashMap;

// Efficient word counting with entry API
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    
    for word in text.split_whitespace() {
        // Entry API avoids double lookup
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    
    counts
}

// Complex state updates with entry API
fn update_player_stats(
    stats: &mut HashMap<String, PlayerStats>,
    player: String,
    score: i32
) {
    let player_stats = stats.entry(player).or_insert_with(|| PlayerStats {
        games_played: 0,
        total_score: 0,
        high_score: 0,
    });
    
    player_stats.games_played += 1;
    player_stats.total_score += score;
    player_stats.high_score = player_stats.high_score.max(score);
}

#[derive(Default)]
struct PlayerStats {
    games_played: u32,
    total_score: i32,
    high_score: i32,
}
```

## Performance Optimization Techniques

### Load Factor Management
```rust
use std::collections::HashMap;

// Monitor and optimize load factor
fn analyze_hashmap_efficiency<K, V>(map: &HashMap<K, V>) {
    let capacity = map.capacity();
    let len = map.len();
    let load_factor = len as f64 / capacity as f64;
    
    println!("HashMap stats:");
    println!("  Capacity: {}", capacity);
    println!("  Length: {}", len);
    println!("  Load factor: {:.2}", load_factor);
    
    if load_factor > 0.8 {
        println!("  Warning: High load factor may impact performance");
    }
}

// Pre-size HashMap for known data sets
fn create_optimally_sized_map<K, V>(expected_size: usize) -> HashMap<K, V> {
    // Reserve capacity for expected size with some buffer
    HashMap::with_capacity((expected_size as f64 * 1.25) as usize)
}
```

### Bulk Operations
```rust
use std::collections::HashMap;

// Efficient batch insertion
fn bulk_insert_efficient<K, V>(
    map: &mut HashMap<K, V>,
    items: impl IntoIterator<Item = (K, V)>
) where
    K: Eq + std::hash::Hash,
{
    // Extend is more efficient than individual inserts
    map.extend(items);
}

// Efficient filtering and collection
fn filter_map_entries<K, V>(
    source: &HashMap<K, V>, 
    predicate: impl Fn(&K, &V) -> bool
) -> HashMap<K, V> 
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    source.iter()
        .filter(|(&ref k, &ref v)| predicate(k, v))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect() // Single allocation with size hint
}
```

## Error Handling and Safety

### Safe HashMap Operations
```rust
use std::collections::HashMap;

// Safe key access with detailed error handling
fn safe_get_multiple<K, V>(
    map: &HashMap<K, V>,
    keys: &[K]
) -> Result<Vec<&V>, String>
where
    K: Eq + std::hash::Hash + std::fmt::Debug,
{
    let mut results = Vec::with_capacity(keys.len());
    
    for key in keys {
        match map.get(key) {
            Some(value) => results.push(value),
            None => return Err(format!("Key not found: {:?}", key)),
        }
    }
    
    Ok(results)
}

// Graceful HashMap merging
fn merge_hashmaps<K, V>(
    map1: HashMap<K, V>,
    map2: HashMap<K, V>,
    resolver: impl Fn(V, V) -> V
) -> HashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    let mut result = map1;
    
    for (key, value2) in map2 {
        match result.remove(&key) {
            Some(value1) => {
                let merged = resolver(value1, value2);
                result.insert(key, merged);
            }
            None => {
                result.insert(key, value2);
            }
        }
    }
    
    result
}
```

## Integration with Other Concepts

- **[[Performance Benchmarking]]**: Measuring HashMap operation costs
- **[[zero-cost-abstractions]]**: HashMap iterator optimization
- **[[Performance Patterns]]**: Efficient HashMap usage patterns
- **[[Memory Safety]]**: Safe concurrent HashMap access
- **[[String Manipulation]]**: String keys and memory management

## Daily Study Applications

### Week 2: Collections Deep Dive
- HashMap vs BTreeMap performance comparison
- Custom hash function implementation
- Load factor optimization strategies

### Week 3: Advanced Data Structures
- HashMap-based caching patterns
- Composite key strategies
- Memory-efficient value storage

### Week 5: Error Handling Integration
- Result<T, E> with HashMap operations
- Graceful key lookup failure handling
- Validation patterns for HashMap data

## Mission Integration Examples

### Mission 5 Requirements Traceability
- **REQ-1**: Generic HashMap<K, V> implementation ✓
- **REQ-2**: O(1) average-case operations ✓
- **REQ-3**: Collision resolution strategy ✓
- **REQ-4**: Dynamic resizing capability ✓
- **REQ-5**: Iterator support for traversal ✓

### Performance Validation
- **Insertion**: O(1) amortized with load factor management
- **Lookup**: O(1) average case with Robin Hood hashing
- **Deletion**: O(1) average case with tombstone handling
- **Resize**: O(n) when triggered, amortized to O(1) per operation

## Further Reading

- **[[Performance Benchmarking]]**: HashMap performance measurement
- **[[Performance Patterns]]**: Optimization techniques
- **[[zero-cost-abstractions]]**: Iterator fusion over HashMap
- **[[mission-5]]**: Complete HashMap implementation

---

*HashMap Deep Dive Links:*
- [[Performance Benchmarking]] - HashMap performance measurement
- [[zero-cost-abstractions]] - Iterator optimization
- [[Performance Patterns]] - HashMap optimization techniques  
- [[String Manipulation]] - String key handling
- [[Memory Safety]] - Safe HashMap operations
- [[mission-5]] - HashMap implementation project
- [[Big-O Notation]] - Complexity analysis