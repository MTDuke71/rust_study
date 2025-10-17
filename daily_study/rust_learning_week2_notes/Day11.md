# Day 11 · HashSets (unique collections, set operations)

## 🔗 Zettelkasten Connections  
- **Today's Focus**: [[Mission5 Overview]] - REQ-4 Iterator Implementation aligns with today
- **Implementation**: [[Mission5 HashSet]] - Custom HashSet wrapper in mission
- **Tutorial**: [[Mission5_tut Step 5]] - MemoCache integration (today's tutorial step)
- **Theory**: [[HashMap Internals]] - Understanding the backing HashMap structure
- **Previous**: [[Day 10 - HashMap Basics]] - Foundation concepts
- **Next**: [[daily-study/Day12]] - Ordered collections comparison
- **Applications**: [[AoC HashSet Patterns]] - Competitive programming applications

## Core Concepts

### HashSet<T> Fundamentals
- **Purpose**: Collection of unique values with fast membership testing
- **Internal Structure**: `HashSet<T> = HashMap<T, ()>` - zero-cost abstraction
- **Performance**: O(1) average insert, remove, and contains operations
- **Trait Requirements**: `T: Eq + Hash` (same as HashMap keys)

### Key Characteristics
```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert("rust");
set.insert("programming");
set.insert("rust"); // Duplicate - ignored

assert_eq!(set.len(), 2); // Only unique values
assert!(set.contains("rust"));
```

## Essential Operations

### Creation and Initialization
```rust
// Empty set
let mut languages = HashSet::new();

// From iterator
let numbers: HashSet<i32> = (1..=5).collect();

// From array/vec
let frameworks: HashSet<&str> = ["React", "Vue", "Angular"].into_iter().collect();

// 🔍 Breaking down the above line:
// 1. ["React", "Vue", "Angular"]        - Array literal containing 3 string slices
// 2. .into_iter()                      - Converts array into an owned iterator
// 3. .collect()                        - Consumes iterator and builds HashSet
// 4. HashSet<&str>                     - Type annotation (could be inferred)
// 5. let frameworks                    - Variable binding to store the result

// Alternative ways to write the same thing:
let frameworks2 = HashSet::from(["React", "Vue", "Angular"]);  // Using From trait
let mut frameworks3 = HashSet::new();                          // Manual insertion
frameworks3.insert("React");
frameworks3.insert("Vue");  
frameworks3.insert("Angular");

// 🌐 Try it online: https://play.rust-lang.org/
// Copy-paste the complete example below into the playground

// With capacity
let mut large_set = HashSet::with_capacity(1000);
```

### Basic Operations
```rust
let mut set = HashSet::new();

// Insert (returns bool - true if new)
let was_new = set.insert("value");

// Remove (returns bool - true if existed)
let was_present = set.remove("value");

// Check membership
if set.contains("key") {
    println!("Found it!");
}

// Size operations
println!("Size: {}, Empty: {}", set.len(), set.is_empty());
```

## Set Theory Operations

### Mathematical Set Operations
```rust
let set1: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
let set2: HashSet<i32> = [3, 4, 5, 6].into_iter().collect();

// Union (all elements from both sets)
let union: HashSet<_> = set1.union(&set2).collect();
// {1, 2, 3, 4, 5, 6}

// Intersection (common elements)
let intersection: HashSet<_> = set1.intersection(&set2).collect();
// {3, 4}

// Difference (elements in set1 but not set2)
let difference: HashSet<_> = set1.difference(&set2).collect();
// {1, 2}

// Symmetric difference (elements in either set, but not both)
let sym_diff: HashSet<_> = set1.symmetric_difference(&set2).collect();
// {1, 2, 5, 6}
```

### Set Relationships
```rust
let small_set: HashSet<i32> = [1, 2].into_iter().collect();
let large_set: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
let other_set: HashSet<i32> = [5, 6].into_iter().collect();

// Subset testing
assert!(small_set.is_subset(&large_set));
assert!(large_set.is_superset(&small_set));

// Disjoint testing (no common elements)
assert!(small_set.is_disjoint(&other_set));
```

## Common Use Cases

### Deduplication
```rust
// Remove duplicates from a vector
let numbers = vec![1, 2, 2, 3, 3, 3, 4];
let unique: HashSet<_> = numbers.into_iter().collect();
let deduplicated: Vec<_> = unique.into_iter().collect();
```

### Membership Testing
```rust
// Fast lookups in large collections
let valid_extensions: HashSet<&str> = 
    ["rs", "toml", "md", "txt", "json"].into_iter().collect();

fn is_valid_file(filename: &str) -> bool {
    filename.split('.').last()
        .map(|ext| valid_extensions.contains(ext))
        .unwrap_or(false)
}
```

### Tag Systems
```rust
// User permissions/roles system
#[derive(Hash, Eq, PartialEq)]
enum Permission {
    Read, Write, Execute, Admin
}

let user_permissions: HashSet<Permission> = 
    [Permission::Read, Permission::Write].into_iter().collect();

if user_permissions.contains(&Permission::Admin) {
    println!("Admin access granted");
}
```

## Advanced Patterns

### Custom Hash Types
```rust
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct CaseInsensitiveString(String);

impl Hash for CaseInsensitiveString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_lowercase().hash(state);
    }
}

impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl Eq for CaseInsensitiveString {}

let mut tags = HashSet::new();
tags.insert(CaseInsensitiveString("RUST".to_string()));
tags.insert(CaseInsensitiveString("rust".to_string())); // Duplicate!
assert_eq!(tags.len(), 1); // Case-insensitive deduplication
```

**🎓 Key Insight**: When implementing custom Hash types:
1. **Hash consistency**: If `a == b`, then `hash(a) == hash(b)` MUST be true
2. **Trait trio**: Always implement `Hash + Eq + PartialEq` together
3. **Selective hashing**: Only hash fields that matter for equality
4. **Performance**: Good hash distribution prevents O(n) degradation

**🚀 Run the full tutorial**: `cargo run --example day11_custom_hash_tutorial`

### Performance Considerations
```rust
// Pre-allocate for known size
let mut large_set = HashSet::with_capacity(10_000);

// Drain for efficient moves
let mut set1 = HashSet::new();
let mut set2 = HashSet::new();
// ... populate sets ...

// Move all elements from set1 to set2
set2.extend(set1.drain());
```

## AoC Applications

### Grid Coordinate Tracking
```rust
use std::collections::HashSet;

type Point = (i32, i32);

// Track visited positions
let mut visited: HashSet<Point> = HashSet::new();
let mut position = (0, 0);

for direction in "UDLR".chars() {
    visited.insert(position);
    match direction {
        'U' => position.1 += 1,
        'D' => position.1 -= 1,
        'L' => position.0 -= 1,
        'R' => position.0 += 1,
        _ => {}
    }
}

println!("Unique positions visited: {}", visited.len());
```

### Finding Cycles
```rust
// Detect repeated states
let mut seen_states = HashSet::new();
let mut current_state = initial_state();

loop {
    if !seen_states.insert(current_state.clone()) {
        println!("Cycle detected!");
        break;
    }
    current_state = next_state(current_state);
}
```

## Best Practices

### When to Use HashSet vs Vec
```rust
// ✅ Use HashSet when:
// - Need fast membership testing: O(1) vs O(n)
// - Want automatic deduplication
// - Performing set operations (union, intersection)
// - Don't care about order
// - NOT in safety-critical real-time systems

// ✅ Use Vec when:
// - Order matters
// - Need indexing by position
// - Allowing duplicates
// - Iterating in insertion order
// - Safety-critical systems (with pre-allocation)

// 🚨 NEVER use HashSet in:
// - Automotive safety functions (ASIL B/C/D)
// - Medical device critical paths
// - Aerospace flight control
// - Industrial safety systems
// - Any system with hard real-time deadlines
```

### Memory and Performance Tips
- **Capacity**: Pre-allocate with `with_capacity()` for known sizes
- **Cloning**: `HashSet::clone()` is expensive - prefer references
- **Iteration**: No guaranteed order - don't rely on iteration sequence
- **Hash Quality**: Poor hash functions cause clustering and performance degradation

### ⚠️ **Safety-Critical Systems Warning**
**For automotive, medical, aerospace, and other safety-critical systems:**

❌ **NEVER use HashSet/HashMap in safety functions** because:
- **Unpredictable rehashing timing** (can violate real-time deadlines)
- **Dynamic memory allocation** (forbidden in many safety standards)
- **No bounded worst-case execution time** (WCET analysis impossible)
- **Potential for catastrophic timing spikes** during emergency scenarios

✅ **Use instead:**
- **Fixed arrays** for bounded collections
- **Ring buffers** for sensor data  
- **Pre-allocated collections** with `with_capacity()`
- **Static lookup tables** for deterministic access

**Example Safety-Critical Alternative:**
```rust
// ❌ DANGEROUS for brake systems
let mut active_faults = HashSet::new(); // Could rehash during emergency!

// ✅ SAFE for brake systems  
const MAX_FAULTS: usize = 32;
let mut active_faults = [false; MAX_FAULTS]; // Predictable timing always
```

This applies to **ASIL D automotive systems** like electronic braking, where a 15μs HashSet rehashing delay during emergency braking could be the difference between stopping safely and a collision.

## Learning Progression Summary

From Day 11, you should understand:
1. **Set Theory**: Union, intersection, difference operations
2. **Uniqueness**: Automatic duplicate elimination
3. **Performance**: O(1) membership testing vs O(n) vector scanning
4. **Use Cases**: Deduplication, fast lookups, coordinate tracking
5. **Implementation**: Zero-cost abstraction over HashMap
6. **Requirements**: Same `Eq + Hash` trait bounds as HashMap keys

**Next**: Day 12 will cover **BTreeMap & BTreeSet** - ordered alternatives to hash-based collections!

## 🚀 **Complete Runnable Example**

```rust
// Copy this entire block to Rust Playground or save as a .rs file
use std::collections::HashSet;

fn main() {
    println!("=== HashSet Demo from Day 11 ===\n");
    
    // Creation methods (fixed for modern Rust)
    let frameworks: HashSet<&str> = ["React", "Vue", "Angular"].iter().cloned().collect();
    // Alternative: HashSet::from(["React", "Vue", "Angular"])
    println!("Frameworks: {:?}", frameworks);
    
    // Set operations (using Vec to avoid array iter issues)
    let set1: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
    let set2: HashSet<i32> = vec![3, 4, 5, 6].into_iter().collect();
    
    println!("\nSet 1: {:?}", set1);
    println!("Set 2: {:?}", set2);
    
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("Union: {:?}", union);
    
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("Intersection: {:?}", intersection);
    
    // Deduplication demo
    let numbers = vec![1, 2, 2, 3, 3, 3, 4];
    let unique: HashSet<_> = numbers.into_iter().collect();
    println!("\nOriginal: [1, 2, 2, 3, 3, 3, 4]");
    println!("Deduplicated: {:?}", unique);
}
```

### **🛠️ How to Run This Code:**

1. **Online**: Copy to [Rust Playground](https://play.rust-lang.org/)
2. **Local file**: Save as `day11_demo.rs` and run `rustc day11_demo.rs && ./day11_demo`
3. **In this workspace**: Add as an example to any existing crate
