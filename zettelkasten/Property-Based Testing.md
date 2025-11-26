# Property-Based Testing - Testing Invariants with Generated Data

*Automatically generate test inputs to verify properties that should always hold*

---

## 🎯 **Core Concept**

**Property-based testing** verifies that code satisfies general properties (invariants) across many automatically generated inputs, rather than testing specific example cases.

**Key Insight**: Instead of "does f(3) return 6?", ask "for all n, does f(n) = 2*n?"

```rust
// Example-based: Tests specific cases
#[test]
fn test_double_examples() {
    assert_eq!(double(0), 0);
    assert_eq!(double(1), 2);
    assert_eq!(double(5), 10);
}

// Property-based: Tests the property holds for all inputs
#[test]
fn test_double_property() {
    // For any n: double(n) == n + n
    proptest!(|(n: i32)| {
        prop_assert_eq!(double(n), n + n);
    });
}
```

---

## 🧠 **Mental Models**

### **The Oracle Model**

Property tests define an "oracle" that can verify any output:

- Instead of pre-computing expected outputs
- Define relationships that must always hold
- Let the framework find counterexamples

### **The Fuzzer Model**

Property testing is like intelligent fuzzing:

- Generates diverse inputs (including edge cases)
- Shrinks failing cases to minimal reproductions
- Explores the input space systematically

---

## 🔍 **Property Types**

### **1. Roundtrip Properties**

*Operation and inverse should cancel out*

```rust
use proptest::prelude::*;

proptest! {
    // Parsing and serialization are inverses
    #[test]
    fn roundtrip_json(value: String) {
        let json = serde_json::to_string(&value).unwrap();
        let parsed: String = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(value, parsed);
    }
    
    // Encoding and decoding are inverses
    #[test]
    fn roundtrip_base64(data: Vec<u8>) {
        let encoded = base64::encode(&data);
        let decoded = base64::decode(&encoded).unwrap();
        prop_assert_eq!(data, decoded);
    }
    
    // Sorting is idempotent (roundtrip with itself)
    #[test]
    fn sort_idempotent(mut data: Vec<i32>) {
        data.sort();
        let once_sorted = data.clone();
        data.sort();
        prop_assert_eq!(data, once_sorted);
    }
}
```

### **2. Invariant Properties**

*Property that must always hold*

```rust
proptest! {
    // Sorted output is always sorted
    #[test]
    fn sort_produces_sorted(mut data: Vec<i32>) {
        data.sort();
        for window in data.windows(2) {
            prop_assert!(window[0] <= window[1]);
        }
    }
    
    // HashMap invariant: get after insert returns value
    #[test]
    fn hashmap_get_after_insert(key: String, value: i32) {
        let mut map = std::collections::HashMap::new();
        map.insert(key.clone(), value);
        prop_assert_eq!(map.get(&key), Some(&value));
    }
    
    // Vec length after push
    #[test]
    fn vec_push_increases_len(mut vec: Vec<i32>, item: i32) {
        let old_len = vec.len();
        vec.push(item);
        prop_assert_eq!(vec.len(), old_len + 1);
    }
}
```

### **3. Algebraic Properties**

*Mathematical laws the operation must satisfy*

```rust
proptest! {
    // Addition is commutative: a + b == b + a
    #[test]
    fn add_commutative(a: i32, b: i32) {
        prop_assert_eq!(a.wrapping_add(b), b.wrapping_add(a));
    }
    
    // Addition is associative: (a + b) + c == a + (b + c)
    #[test]
    fn add_associative(a: i32, b: i32, c: i32) {
        let left = a.wrapping_add(b).wrapping_add(c);
        let right = a.wrapping_add(b.wrapping_add(c));
        prop_assert_eq!(left, right);
    }
    
    // Addition has identity: a + 0 == a
    #[test]
    fn add_identity(a: i32) {
        prop_assert_eq!(a + 0, a);
    }
    
    // String concatenation is associative
    #[test]
    fn concat_associative(a: String, b: String, c: String) {
        let left = format!("{}{}{}", a, b, c);
        let right = format!("{}{}", format!("{}{}", a, b), c);
        prop_assert_eq!(left, right);
    }
}
```

### **4. Comparison Properties**

*Comparing against a simpler/reference implementation*

```rust
proptest! {
    // Optimized implementation matches naive implementation
    #[test]
    fn optimized_matches_naive(data: Vec<i32>) {
        let naive_result = naive_sum(&data);
        let optimized_result = optimized_sum(&data);
        prop_assert_eq!(naive_result, optimized_result);
    }
    
    // Custom binary search matches std
    #[test]
    fn binary_search_matches_std(mut data: Vec<i32>, target: i32) {
        data.sort();
        let custom = custom_binary_search(&data, &target);
        let std = data.binary_search(&target).ok();
        prop_assert_eq!(custom, std);
    }
}

fn naive_sum(data: &[i32]) -> i64 {
    data.iter().map(|&x| x as i64).sum()
}

fn optimized_sum(data: &[i32]) -> i64 {
    // SIMD or other optimization
    data.iter().map(|&x| x as i64).sum()
}
```

### **5. State Machine Properties**

*Model system state transitions*

```rust
#[derive(Debug, Clone)]
enum StackOp {
    Push(i32),
    Pop,
}

proptest! {
    #[test]
    fn stack_model_matches(ops: Vec<StackOp>) {
        let mut stack = mission1::Stack::new();
        let mut model: Vec<i32> = Vec::new();  // Simple model
        
        for op in ops {
            match op {
                StackOp::Push(x) => {
                    stack.push(x);
                    model.push(x);
                }
                StackOp::Pop => {
                    let stack_result = stack.pop();
                    let model_result = model.pop();
                    prop_assert_eq!(stack_result, model_result);
                }
            }
        }
        
        prop_assert_eq!(stack.len(), model.len());
    }
}
```

---

## 🛠️ **Proptest Framework**

### **Basic Setup**

```rust
// Cargo.toml
// [dev-dependencies]
// proptest = "1.0"

use proptest::prelude::*;

// Basic property test
proptest! {
    #[test]
    fn test_name(input: InputType) {
        // Property assertion
        prop_assert!(condition);
        prop_assert_eq!(left, right);
    }
}
```

### **Custom Strategies**

```rust
use proptest::prelude::*;

// Generate valid email-like strings
fn email_strategy() -> impl Strategy<Value = String> {
    (
        "[a-z]{1,10}",      // username
        "[a-z]{2,5}",       // domain
        "[a-z]{2,3}",       // tld
    )
        .prop_map(|(user, domain, tld)| format!("{}@{}.{}", user, domain, tld))
}

// Generate sorted vectors
fn sorted_vec_strategy(len: usize) -> impl Strategy<Value = Vec<i32>> {
    proptest::collection::vec(any::<i32>(), 0..len)
        .prop_map(|mut v| {
            v.sort();
            v
        })
}

// Generate valid grid coordinates
fn coord_strategy(width: usize, height: usize) -> impl Strategy<Value = (usize, usize)> {
    (0..width, 0..height)
}

proptest! {
    #[test]
    fn test_email_parsing(email in email_strategy()) {
        prop_assert!(email.contains('@'));
    }
    
    #[test]
    fn test_binary_search_on_sorted(
        data in sorted_vec_strategy(100),
        target: i32
    ) {
        // Binary search requires sorted data
        let result = data.binary_search(&target);
        // Verify: if found, element is at returned index
        if let Ok(idx) = result {
            prop_assert_eq!(data[idx], target);
        }
    }
}
```

### **Shrinking**

Proptest automatically shrinks failing cases:

```rust
proptest! {
    #[test]
    fn finds_minimal_failing_case(data: Vec<i32>) {
        // This will fail for non-empty vectors
        // Proptest shrinks to find smallest failing case: vec![0] or similar
        prop_assert!(data.is_empty());
    }
}

// Output:
// proptest: Failing input: data = [0]
// (Shrunk from something like [43, -17, 892, ...])
```

---

## 📊 **Mission Applications**

### **Mission 1: Stack Properties**

```rust
proptest! {
    // Stack LIFO property
    #[test]
    fn stack_lifo(items: Vec<i32>) {
        let mut stack = Stack::new();
        
        // Push all items
        for &item in &items {
            stack.push(item);
        }
        
        // Pop should return in reverse order
        for &expected in items.iter().rev() {
            prop_assert_eq!(stack.pop(), Some(expected));
        }
        
        prop_assert!(stack.is_empty());
    }
    
    // Peek doesn't modify stack
    #[test]
    fn peek_nondestructive(mut stack: Stack<i32>) {
        let len_before = stack.len();
        let _ = stack.peek();
        prop_assert_eq!(stack.len(), len_before);
    }
}
```

### **Mission 5: HashMap Properties**

```rust
proptest! {
    // Insert then get returns value
    #[test]
    fn insert_get(entries: Vec<(String, i32)>) {
        let mut map = HashMap::new();
        
        for (k, v) in &entries {
            map.insert(k.clone(), *v);
        }
        
        // Last insert wins for duplicate keys
        let mut expected = std::collections::HashMap::new();
        for (k, v) in &entries {
            expected.insert(k.clone(), *v);
        }
        
        for (k, v) in expected {
            prop_assert_eq!(map.get(&k), Some(&v));
        }
    }
    
    // Remove then get returns None
    #[test]
    fn remove_get(key: String, value: i32) {
        let mut map = HashMap::new();
        map.insert(key.clone(), value);
        map.remove(&key);
        prop_assert_eq!(map.get(&key), None);
    }
}
```

### **Mission 6: Grid Properties**

```rust
proptest! {
    // Grid access in bounds
    #[test]
    fn grid_access_bounds(
        width in 1usize..100,
        height in 1usize..100,
    ) {
        let grid = Grid::new(width, height, 0);
        
        for x in 0..width {
            for y in 0..height {
                let _ = grid.get(x, y);  // Should not panic
            }
        }
    }
    
    // Grid set/get roundtrip
    #[test]
    fn grid_set_get(
        width in 1usize..50,
        height in 1usize..50,
        x in 0usize..50,
        y in 0usize..50,
        value: i32,
    ) {
        prop_assume!(x < width && y < height);
        
        let mut grid = Grid::new(width, height, 0);
        grid.set(x, y, value);
        prop_assert_eq!(grid.get(x, y), Some(&value));
    }
}
```

---

## 🎮 **AoC Applications**

### **Parser Verification**

```rust
proptest! {
    // Parser handles any valid-looking input
    #[test]
    fn parser_doesnt_panic(line in "[0-9a-zA-Z\\s,;:-]{0,100}") {
        // Should not panic, even on weird input
        let _ = parse_line(&line);
    }
    
    // Roundtrip: generate -> serialize -> parse
    #[test]
    fn coordinate_roundtrip(x: i32, y: i32) {
        let serialized = format!("{},{}", x, y);
        let parsed = parse_coordinate(&serialized);
        prop_assert_eq!(parsed, Some((x, y)));
    }
}
```

### **Algorithm Verification**

```rust
proptest! {
    // BFS finds shortest path (if path exists)
    #[test]
    fn bfs_optimal_path(
        grid_size in 5usize..20,
        wall_probability in 0.0..0.5f64,
    ) {
        let grid = generate_random_grid(grid_size, wall_probability);
        let start = (0, 0);
        let end = (grid_size - 1, grid_size - 1);
        
        if let Some(bfs_path) = bfs(&grid, start, end) {
            // If BFS finds a path, DFS should also find one
            // BFS path should be no longer than any DFS path
            if let Some(dfs_path) = dfs(&grid, start, end) {
                prop_assert!(bfs_path.len() <= dfs_path.len());
            }
        }
    }
}
```

---

## ⚠️ **Common Pitfalls**

### **1. Overly Specific Properties**

```rust
// ❌ BAD: Too specific, just repeats implementation
#[test]
fn bad_property(x: i32) {
    let result = my_double(x);
    prop_assert_eq!(result, x * 2);  // Just reimplements the function!
}

// ✅ GOOD: Tests a property without reimplementing
#[test]
fn good_property(x: i32) {
    let result = my_double(x);
    prop_assert_eq!(result, x + x);  // Different way to express "double"
}
```

### **2. Ignoring Edge Cases**

```rust
// ❌ BAD: Assumes positive inputs
#[test]
fn bad_sqrt(n: u32) {
    let result = my_sqrt(n);
    prop_assert_eq!(result * result, n);  // Fails for non-perfect squares
}

// ✅ GOOD: Account for integer sqrt semantics
#[test]
fn good_sqrt(n: u32) {
    let result = my_sqrt(n);
    prop_assert!(result * result <= n);
    prop_assert!((result + 1) * (result + 1) > n);
}
```

### **3. Too Few Iterations**

```rust
// ❌ BAD: Default iterations might miss rare bugs
proptest! {
    #[test]
    fn might_miss_bugs(data: Vec<i32>) {
        // Bug only triggers for specific data patterns
    }
}

// ✅ GOOD: Increase cases for critical tests
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10000))]
    
    #[test]
    fn thorough_testing(data: Vec<i32>) {
        // More iterations = more likely to find edge cases
    }
}
```

---

## 💡 **Key Takeaways**

1. **Think in properties**: What should *always* be true?

2. **Common property patterns**:
   - Roundtrip (encode/decode, serialize/parse)
   - Invariants (sorted stays sorted, length consistency)
   - Algebraic laws (commutativity, associativity)
   - Model comparison (optimized matches naive)

3. **Shrinking is powerful**: Framework finds minimal failing cases

4. **Complement, don't replace**: Use with example-based tests
   - Property tests: "does it work in general?"
   - Example tests: "does it work for this specific important case?"

5. **Custom strategies**: Generate domain-specific valid inputs

---

## 🔗 **Integration Points**

### **Builds On**

- [[Testing Strategies]] - Foundation for test approaches
- [[Mission Testing Strategies]] - V-Cycle test requirements
- [[Debugging Strategies]] - Using failing cases to find bugs

### **Enables**

- [[Benchmarking]] - Generate diverse benchmark inputs
- [[Fuzzing]] - Property testing as structured fuzzing
- [[Regression Testing]] - Catch edge cases automatically

### **Related Concepts**

- [[Invariants]] - Properties encode invariants
- [[QuickCheck]] - Original property testing (Haskell)
- [[Hypothesis]] - Python property testing

### **Workspace Applications**

- **Missions**: Verify data structure invariants
- **AoC**: Test parser robustness
- **Tutorials**: Demonstrate testing approaches

---

## 📚 **Resources**

- [Proptest Book](https://proptest-rs.github.io/proptest/intro.html)
- [Property-Based Testing with PropEr, Erlang, and Elixir](https://pragprog.com/titles/fhproper/)
- [[test-pyramid]] - Where property tests fit

---

*Tags: #testing #property-based-testing #proptest #quickcheck #invariants #tdd #quality-assurance #missions #aoc*

*Links: [[Testing Strategies]] | [[Mission Testing Strategies]] | [[Debugging Strategies]] | [[Benchmarking]] | [[test-pyramid]] | [[Algorithms MOC]] | [[zettel-index]]*
