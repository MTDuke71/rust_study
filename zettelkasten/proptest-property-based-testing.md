# Proptest - Property-Based Testing in Rust

*Created: 2025-11-05*
*Tags: #rust #testing #property-based-testing #proptest #test-generation #shrinking #fuzzing*

## Overview

**Proptest** is Rust's premier property-based testing framework that revolutionizes testing by automatically generating test inputs and verifying that mathematical properties hold across the entire input space. Instead of writing specific test cases, you define properties that should always be true, and proptest generates hundreds or thousands of inputs to validate these invariants.

## Core Philosophy: Properties vs Examples

### Traditional Example-Based Testing

```rust
#[test]
fn test_sort_specific_cases() {
    assert_eq!(sort(vec![3, 1, 2]), vec![1, 2, 3]);
    assert_eq!(sort(vec![]), vec![]);
    assert_eq!(sort(vec![42]), vec![42]);
}
```

### Property-Based Testing Approach

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn sort_properties(mut input: Vec<i32>) {
        let original_len = input.len();
        let sorted = sort(input.clone());
        
        // Property 1: Length preserved
        prop_assert_eq!(sorted.len(), original_len);
        
        // Property 2: All elements preserved (multiset equality)
        input.sort_unstable();
        prop_assert_eq!(sorted, input);
        
        // Property 3: Result is sorted
        for i in 1..sorted.len() {
            prop_assert!(sorted[i-1] <= sorted[i]);
        }
    }
}
```

## Key Advantages Over Traditional Testing

### **1. Comprehensive Coverage**

Property-based testing explores a vastly larger input space than manually written test cases:

- **Boundary conditions**: Automatically finds edge cases like empty collections, maximum values, Unicode edge cases
- **Unexpected combinations**: Discovers interactions between inputs that humans rarely consider
- **Systematic exploration**: Tests across different data sizes, types, and combinations

### **2. Automatic Shrinking**

When proptest finds a failing case, it performs **automatic shrinking** to find the minimal example:

```rust
// Original failing input: vec![1000, -500, 999, 0, -1, 42, 777, ...]
// After shrinking: vec![1, 0]  ← Minimal case that reproduces the bug
```

Shrinking dramatically reduces debugging time by eliminating irrelevant complexity from failing test cases.

### **3. Better Bug Discovery**

Proptest excels at finding bugs that traditional testing misses:

- **Off-by-one errors** in boundary conditions
- **Integer overflow** and underflow scenarios  
- **Unicode handling issues** with edge case characters
- **Memory safety violations** through unusual input patterns
- **Logic errors** that only manifest with specific input relationships

## Proptest Architecture

### **Generators and Strategies**

Proptest uses **strategies** to generate test data:

```rust
use proptest::prelude::*;

// Built-in generators
any::<i32>()                    // Any i32 value
0..100i32                       // Range of integers
"[a-z]{3,10}"                   // Regex-based strings
prop::collection::vec(any::<i32>(), 0..20)  // Vectors of variable size

// Custom generators
fn positive_even() -> impl Strategy<Value = i32> {
    (1..1000i32).prop_map(|x| x * 2)
}

// Conditional generators
fn valid_email() -> impl Strategy<Value = String> {
    "[a-z]{3,10}@[a-z]{3,8}\\.[a-z]{2,3}"
}
```

### **Strategy Combinators**

Combine simple strategies to create complex data:

```rust
// Tuples
(any::<String>(), 0..100u32)   // (String, u32) pairs

// Options and Results
prop::option::of(any::<i32>())  // Option<i32>
prop::result::maybe_err(any::<String>(), any::<i32>()) // Result<String, i32>

// Conditional generation
any::<bool>().prop_flat_map(|b| {
    if b {
        Just(vec![]).boxed()
    } else {
        prop::collection::vec(any::<i32>(), 1..10).boxed()
    }
})
```

## Advanced Proptest Features

### **State Machine Testing**

Test complex stateful systems by modeling operations and invariants:

```rust
use proptest::test_runner::TestRunner;
use proptest_state_machine::{StateMachineTest, ReferenceStateMachine};

#[derive(Debug, Clone)]
enum HashMapOp {
    Insert { key: u32, value: String },
    Remove { key: u32 },
    Get { key: u32 },
    Clear,
}

// Reference implementation (simple, obviously correct)
struct ReferenceHashMap {
    data: std::collections::HashMap<u32, String>,
}

impl ReferenceStateMachine for ReferenceHashMap {
    type State = std::collections::HashMap<u32, String>;
    type Transition = HashMapOp;
    
    fn init_state() -> BoxedStrategy<Self::State> {
        Just(std::collections::HashMap::new()).boxed()
    }
    
    fn transitions(state: &Self::State) -> BoxedStrategy<Self::Transition> {
        prop_oneof![
            (any::<u32>(), "[a-z]{1,10}").prop_map(|(k, v)| HashMapOp::Insert { key: k, value: v }),
            any::<u32>().prop_map(|k| HashMapOp::Remove { key: k }),
            any::<u32>().prop_map(|k| HashMapOp::Get { key: k }),
            Just(HashMapOp::Clear),
        ].boxed()
    }
    
    fn apply(mut state: Self::State, transition: &Self::Transition) -> Self::State {
        match transition {
            HashMapOp::Insert { key, value } => { state.insert(*key, value.clone()); },
            HashMapOp::Remove { key } => { state.remove(key); },
            HashMapOp::Get { .. } => {}, // Read-only operation
            HashMapOp::Clear => state.clear(),
        }
        state
    }
}
```

### **Regression Testing**

Proptest can save failing cases for future regression testing:

```rust
// In Cargo.toml
[dev-dependencies]
proptest = { version = "1.0", default-features = false, features = ["std"] }

// Configuration
proptest! {
    #![proptest_config(ProptestConfig {
        cases: 1000,                    // Number of test cases
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("regressions"))),
        ..ProptestConfig::default()
    })]
    
    #[test]
    fn my_property_test(input: Vec<i32>) {
        // Test logic here
    }
}
```

### **Performance-Aware Testing**

Control generation complexity for performance-critical testing:

```rust
// Size-bounded collections
prop::collection::vec(any::<i32>(), 0..=100)

// Complexity limits
prop::strategy::Strategy::prop_map(
    (0..1000u32, prop::collection::vec(any::<u8>(), 0..20)),
    |(size, mut data)| {
        data.resize(size as usize, 0);
        data
    }
)

// Timeout configuration
proptest! {
    #![proptest_config(ProptestConfig {
        timeout: 5000,  // 5 second timeout per test
        ..ProptestConfig::default()
    })]
}
```

## Integration Patterns

### **With Standard Test Framework**

Proptest generates standard `#[test]` functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    // Mix with regular unit tests
    #[test]
    fn basic_functionality() {
        assert_eq!(add(2, 3), 5);
    }
    
    // Property-based tests
    proptest! {
        #[test]
        fn add_commutative(a: i32, b: i32) {
            prop_assert_eq!(add(a, b), add(b, a));
        }
        
        #[test] 
        fn add_associative(a: i32, b: i32, c: i32) {
            prop_assert_eq!(add(add(a, b), c), add(a, add(b, c)));
        }
    }
}
```

### **Custom Derive for Complex Types**

Generate tests for custom data structures:

```rust
use proptest_derive::Arbitrary;

#[derive(Debug, Clone, Arbitrary)]
struct Person {
    #[proptest(regex = "[A-Z][a-z]{2,20}")]
    name: String,
    
    #[proptest(strategy = "18..100u8")]
    age: u8,
    
    #[proptest(strategy = "valid_email()")]
    email: String,
}

// Automatically generates Strategy<Value = Person>
proptest! {
    #[test]
    fn person_validation_properties(person: Person) {
        prop_assert!(person.age >= 18);
        prop_assert!(person.name.len() >= 3);
        prop_assert!(person.email.contains('@'));
    }
}
```

### **Workspace Integration**

In multi-crate workspaces, configure proptest consistently:

```toml
# Workspace Cargo.toml
[workspace.dependencies]
proptest = { version = "1.0", features = ["std"] }
proptest-derive = "0.4"

# Individual crate Cargo.toml
[dev-dependencies]
proptest = { workspace = true }
proptest-derive = { workspace = true }
```

## Common Property Patterns

### **1. Invariant Properties**

Properties that must always hold:

```rust
proptest! {
    #[test]
    fn stack_size_invariant(ops: Vec<StackOp>) {
        let mut stack = Stack::new();
        let mut expected_size = 0;
        
        for op in ops {
            match op {
                StackOp::Push(_) => {
                    stack.push(42);
                    expected_size += 1;
                }
                StackOp::Pop => {
                    if expected_size > 0 {
                        stack.pop();
                        expected_size -= 1;
                    }
                }
            }
            prop_assert_eq!(stack.len(), expected_size);
        }
    }
}
```

### **2. Round-Trip Properties**

Serialization, encoding, transformations that should be reversible:

```rust
proptest! {
    #[test]
    fn json_round_trip(data: MyStruct) {
        let json = serde_json::to_string(&data)?;
        let recovered: MyStruct = serde_json::from_str(&json)?;
        prop_assert_eq!(data, recovered);
    }
    
    #[test]
    fn compress_round_trip(data: Vec<u8>) {
        let compressed = compress(&data);
        let decompressed = decompress(&compressed)?;
        prop_assert_eq!(data, decompressed);
    }
}
```

### **3. Metamorphic Properties**

Relationships between different inputs:

```rust
proptest! {
    #[test]
    fn sort_stability(mut data: Vec<(i32, String)>) {
        // Sort by first element
        data.sort_by_key(|(i, _)| *i);
        
        // Elements with same key should maintain relative order
        for window in data.windows(2) {
            if window[0].0 == window[1].0 {
                // Check original relative positions were preserved
                // (requires tracking original indices)
            }
        }
    }
}
```

### **4. Performance Properties**

Validate algorithmic complexity:

```rust
proptest! {
    #[test]
    fn hash_table_performance(operations: Vec<HashOp>) {
        prop_assume!(operations.len() <= 10000);
        
        let start = std::time::Instant::now();
        let mut map = HashMap::new();
        
        for op in operations {
            match op {
                HashOp::Insert(k, v) => { map.insert(k, v); }
                HashOp::Lookup(k) => { map.get(&k); }
            }
        }
        
        let elapsed = start.elapsed();
        // Should be roughly O(n) for n operations
        prop_assert!(elapsed.as_millis() < operations.len() as u128);
    }
}
```

## Best Practices

### **1. Start Simple, Build Complexity**

```rust
// Begin with basic properties
proptest! {
    #[test]
    fn length_preserved(input: Vec<i32>) {
        let result = process(input.clone());
        prop_assert_eq!(result.len(), input.len());
    }
}

// Add complexity gradually
proptest! {
    #[test]
    fn comprehensive_properties(input: Vec<i32>) {
        prop_assume!(!input.is_empty());  // Preconditions
        
        let result = process(input.clone());
        
        // Multiple properties in one test
        prop_assert_eq!(result.len(), input.len());
        prop_assert!(result.iter().all(|&x| x >= 0));
        prop_assert_eq!(result.iter().sum::<i32>(), input.iter().sum::<i32>());
    }
}
```

### **2. Use Appropriate Generators**

```rust
// Too general - may generate unhelpful cases
any::<String>()

// Better - focused on relevant domain
"[a-zA-Z0-9]{1,50}"

// Best - domain-specific constraints
prop::strategy::Strategy::prop_filter_map(
    "[a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}",
    |s| if is_valid_email(&s) { Some(s) } else { None }
)
```

### **3. Handle Assumptions Carefully**

```rust
proptest! {
    #[test]
    fn division_properties(a: i32, b: i32) {
        prop_assume!(b != 0);  // Precondition
        prop_assume!(a != i32::MIN || b != -1);  // Overflow prevention
        
        let result = a / b;
        prop_assert_eq!(result * b + (a % b), a);
    }
}
```

### **4. Combine with Other Testing Strategies**

```rust
// Unit tests for specific cases
#[test]
fn edge_case_empty_input() {
    assert_eq!(process(vec![]), vec![]);
}

// Property tests for general behavior
proptest! {
    #[test]
    fn general_properties(input: Vec<i32>) {
        // General invariants
    }
}

// Integration tests for system behavior
#[test]
fn integration_specific_scenario() {
    // End-to-end testing
}
```

## Limitations and Considerations

### **Performance Impact**

- Property tests are **slower** than unit tests (1000+ generated cases vs single examples)
- **Memory usage** can be significant for large generated data
- Consider **test time budgets** in CI/CD pipelines

### **Test Flakiness**

- **Random generation** can occasionally produce different results
- Use **seeds** for reproducible test runs when debugging
- **Shrinking** helps but may not always find the minimal case

### **Property Design Challenges**

- **Difficult to specify properties** for some algorithms
- **Tautological properties** that don't catch real bugs
- **Over-constraining** that makes tests pass trivially

## Integration with Mission Work

### **Union-Find Property Testing**

```rust
proptest! {
    #[test]
    fn union_find_properties(operations: Vec<UnionFindOp>) {
        let mut uf = UnionFind::new();
        let mut reference = ReferenceUnionFind::new();
        
        for op in operations {
            match op {
                UnionFindOp::MakeSet(x) => {
                    let uf_result = uf.make_set(x);
                    let ref_result = reference.make_set(x);
                    prop_assert_eq!(uf_result, ref_result);
                }
                UnionFindOp::Union(x, y) => {
                    let uf_result = uf.union(&x, &y);
                    let ref_result = reference.union(&x, &y);
                    prop_assert_eq!(uf_result, ref_result);
                }
                UnionFindOp::Connected(x, y) => {
                    let uf_result = uf.connected(&x, &y);
                    let ref_result = reference.connected(&x, &y);
                    prop_assert_eq!(uf_result, ref_result);
                }
            }
            
            // Invariant: count should always match reference
            prop_assert_eq!(uf.count(), reference.count());
        }
    }
}
```

## Tools and Ecosystem

### **Related Crates**

- **`proptest-derive`**: Automatic `Arbitrary` implementations for custom types
- **`proptest-state-machine`**: State machine testing framework
- **`bolero`**: Alternative property-based testing framework with different design
- **`arbitrary`**: Trait for generating arbitrary instances (used by other fuzzers)

### **IDE Integration**

- Works with standard Rust tooling (`cargo test`)
- **VS Code extensions** show property test results like normal tests
- **Failure persistence** creates regression test files automatically

### **CI/CD Considerations**

```toml
# Reduce test cases in CI for speed
[dev-dependencies]
proptest = { version = "1.0", features = ["std"] }

# In test configuration
proptest! {
    #![proptest_config(ProptestConfig {
        cases: if cfg!(debug_assertions) { 100 } else { 1000 },
        ..ProptestConfig::default()
    })]
}
```

## See Also

- **[[black-box-benchmarking]]** - Complementary testing technique for performance validation
- **[[unit-testing-rust]]** - Traditional testing approaches that complement property testing
- **[[fuzzing-techniques]]** - Related automated testing methodologies
- **[[test-driven-development]]** - Development methodology that can incorporate property testing
- **[[formal-verification]]** - Mathematical proof techniques beyond testing

---

## Links

**Outgoing:**

- **[[black-box-benchmarking]]** - Performance testing techniques that complement proptest
- **[[unit-testing-rust]]** - Traditional testing patterns to combine with property testing
- **[[fuzzing-techniques]]** - Related automated input generation approaches
- **[[mission-testing-strategies]]** - How to apply proptest to mission implementations

**Incoming:**

- **[[Day39]]** - Day 39 workspace management where proptest is demonstrated
- **[[testing-best-practices]]** - Comprehensive testing strategy including property testing
- **[[data-structure-validation]]** - Using proptest for algorithm correctness validation
- **[[mission-quality-assurance]]** - Quality assurance methodologies for learning projects

---

*Last Updated: 2025-11-05*
*Navigation: [[zettel-index]] | [[testing-best-practices]] | [[Day39]] | [[black-box-benchmarking]]*
