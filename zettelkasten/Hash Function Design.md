# Hash Function Design

**Tags**: #hash #hash_function #data_structures #algorithms #rust #performance

## 🎯 Core Idea

A hash function is a deterministic algorithm that transforms an input of arbitrary size (the "key") into a fixed-size integer (the "hash code" or "hash value"). Its primary role in data structures like [[HashMap Internals]] is to enable **O(1) average-time complexity** for insertions, lookups, and deletions by calculating the "bucket" index where a value should be stored.

The quality of the hash function is the single most important factor in a hash map's performance. A well-designed function distributes keys evenly, minimizing collisions, while a poor one can degrade performance to O(n).

---

## ⚙️ Key Principles of Effective Hash Functions

A high-quality hash function must balance four critical properties:

1.  **Determinism**: The same input key must *always* produce the same output hash code. This is non-negotiable.
2.  **Uniform Distribution**: Keys should be spread as evenly as possible across all available buckets. This is the primary defense against [[Collision Resolution|collisions]].
3.  **Speed**: The function must be computationally inexpensive. A slow hash function would negate the O(1) benefits of the hash map itself.
4.  **Sensitivity (Avalanche Effect)**: A small change in the input key (e.g., flipping a single bit) should produce a drastically different hash code. This prevents keys that are "close" to each other from clustering in the same buckets.

---

## 🛠️ Implementation Patterns & Code Examples

In Rust, hash function logic is handled by two traits: `std::hash::Hash` (for types that can be hashed) and `std::hash::Hasher` (the algorithm that performs the hashing).

### Example 1: A Flawed Hash Function (Anti-Pattern)

A simple function that just sums the ASCII values of a string is a classic anti-pattern. It fails on sensitivity and distribution.

```rust
/// ANTI-PATTERN: A poor hash function that sums character bytes.
/// It suffers from frequent collisions for anagrams ("abc", "cba")
/// and keys with the same characters in different orders.
fn bad_string_hash(s: &str) -> u64 {
    s.bytes().map(|b| b as u64).sum()
}

// Demonstrate the flaw
let hash1 = bad_string_hash("listen");
let hash2 = bad_string_hash("silent");

// Collision! These anagrams produce the same hash.
assert_eq!(hash1, hash2);
println!("'listen' and 'silent' both hash to: {}", hash1);
```

This example demonstrates why simple arithmetic is insufficient. Anagrams like "listen" and "silent" will collide, leading to poor performance.

### Example 2: Correctly Implementing `Hash` for a Custom Struct

The idiomatic way to make a custom type hashable is to derive `Hash`. If you need custom logic, you implement the `hash` method, feeding each field to the `Hasher`.

```rust
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

// A custom struct representing a user
#[derive(Debug, PartialEq, Eq)]
struct User {
    id: u32,
    username: String,
    is_active: bool,
}

// Idiomatic implementation of the Hash trait
impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Feed each field into the hasher in a defined order.
        // This ensures that the entire state of the struct contributes
        // to the final hash value.
        self.id.hash(state);
        self.username.hash(state);
        self.is_active.hash(state);
    }
}

let user1 = User {
    id: 101,
    username: "alice".to_string(),
    is_active: true,
};

let mut hasher1 = DefaultHasher::new();
user1.hash(&mut hasher1);
let hash1 = hasher1.finish();

println!("Hash for user1: {}", hash1);
```

### Example 3: The Avalanche Effect with `DefaultHasher`

Rust's `DefaultHasher` (SipHash) is designed for security and excellent distribution, showcasing the avalanche effect.

```rust
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

// Two strings that differ by only one character
let s1 = "hash me please";
let s2 = "hash me please!"; // Added "!"

let hash1 = calculate_hash(&s1);
let hash2 = calculate_hash(&s2);

println!("Hash for '{}': {}", s1, hash1);
println!("Hash for '{}': {}", s2, hash2);

// The hashes are completely different, demonstrating the avalanche effect.
assert_ne!(hash1, hash2);
```

---

## ❓ Key Questions & Trade-offs

*   **What are the trade-offs between speed and distribution quality?**
    *   **Fast but Simple (e.g., FNV, xxHash):** Excellent for scenarios where keys are known to be well-distributed and performance is critical (e.g., game engines, short-lived caches). They have higher collision rates for adversarial inputs.
    *   **Secure but Slower (e.g., SipHash):** Rust's default. Provides resistance against HashDoS (Denial of Service) attacks where an attacker intentionally sends keys that all hash to the same bucket. It's a safe default for public-facing services.

*   **How does hash function choice affect security?**
    *   A predictable hash function allows attackers to craft inputs that cause massive collisions, degrading a hash map's performance from O(1) to O(n) and potentially freezing a server. SipHash was chosen specifically to mitigate this risk.

*   **When would you implement a custom `Hasher`?**
    *   You have a very specific performance requirement and have benchmarked that `DefaultHasher` is a bottleneck.
    *   You need a consistent hash value across different program executions or architectures (SipHash can produce different values). In this case, a stable algorithm like FNV or xxHash might be used, often through a crate.
    *   You are integrating with a system that requires a specific, non-standard hash algorithm.

---

## 🔗 Related Concepts

*   **Primary Dependency**: [[HashMap Internals]], [[Collision Resolution]]
*   **Core Rust Concepts**: [[Generic Programming]] (for `K: Hash` trait bounds), [[Traits]]
*   **Implementation Details**: [[Mission5 HashMap]], [[entry-api-hashmap]]
*   **Performance**: [[Load Factor Management]], [[Performance Analysis]]
*   **Broader Context**: [[Collections MOC]], [[Rust Concepts MOC]]