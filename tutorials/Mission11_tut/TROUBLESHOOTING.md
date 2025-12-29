# Troubleshooting Guide - Mission 11 Tutorial

Common issues encountered during the tutorial and their solutions.

---

## Compilation Errors

### Error: Lifetime parameter mismatch

**Symptom**:
```
error[E0623]: lifetime mismatch
  |
  | fn can_make(design: &str, memo: &mut HashMap<&'a str, bool>) -> bool
  |                     ----                      ^^
  |                     |
  |                     this parameter and the return type are declared with different lifetimes...
```

**Cause**: The `design` parameter doesn't have a lifetime annotation, but the HashMap expects keys with lifetime `'a`.

**Solution**: Add lifetime parameter to `design`:
```rust
// ❌ Wrong
fn can_make(design: &str, memo: &mut HashMap<&'a str, bool>) -> bool

// ✅ Correct
fn can_make<'a>(design: &'a str, memo: &mut HashMap<&'a str, bool>) -> bool
```

**See**: Step 3 for detailed lifetime explanation.

---

### Error: Cannot infer type

**Symptom**:
```
error[E0282]: type annotations needed
  |
  | let cache = HashMap::new();
  |     ^^^^^ consider giving `cache` a type
```

**Cause**: Rust can't infer the HashMap's key and value types.

**Solution**: Add type annotations:
```rust
// ❌ Ambiguous
let mut cache = HashMap::new();

// ✅ Explicit
let mut cache: HashMap<&str, bool> = HashMap::new();

// ✅ Or let Rust infer from usage
let mut cache = HashMap::new();
can_make(&patterns, design, &mut cache);  // Now Rust knows the types
```

---

### Error: Borrowed value does not live long enough

**Symptom**:
```
error[E0597]: `design_string` does not live long enough
  |
  | let design_string = String::from("test");
  | let design = &design_string;
  | ...
  | } // `design_string` dropped here while still borrowed
```

**Cause**: Cache contains references to `design_string`, but `design_string` goes out of scope.

**Solution**: Ensure the source string lives as long as the cache:
```rust
// ❌ String dropped too early
{
    let design_string = String::from("test");
    can_make(&patterns, &design_string, &mut cache);
}  // design_string dropped, but cache still has references!

// ✅ Keep string in scope
let design_string = String::from("test");
can_make(&patterns, &design_string, &mut cache);
// Use cache while design_string is still alive
```

**See**: Step 3, Edge Case 1 for more details.

---

## Runtime Errors

### Stack Overflow on Large Inputs

**Symptom**:
```
thread 'main' has overflowed its stack
fatal runtime error: stack overflow
```

**Cause**: Deep recursion exceeds stack size (typically ~2MB).

**Solution**: Use bottom-up approach or increase stack size:
```rust
// Option 1: Bottom-up (no recursion)
fn solve_large(n: usize) -> u64 {
    let mut dp = vec![0; n + 1];
    // Iterative solution
}

// Option 2: Increase stack size
use std::thread;

fn main() {
    let builder = thread::Builder::new()
        .stack_size(8 * 1024 * 1024); // 8MB stack
    
    builder.spawn(|| {
        // Your recursive code here
    }).unwrap().join().unwrap();
}
```

**See**: Step 6 for bottom-up alternatives.

---

### Incorrect Results (Off-by-one errors)

**Symptom**: Tests pass for small inputs but fail for larger ones.

**Cause**: Common in bottom-up approaches with incorrect index bounds.

**Solution**: Verify loop bounds and array indices:
```rust
// ❌ Common mistakes
for i in 1..n {  // Missing n itself!
    dp[i] = dp[i-1] + dp[i-2];  // Off-by-one
}

// ✅ Correct bounds
for i in 2..=n {  // Inclusive range
    dp[i] = dp[i-1] + dp[i-2];
}
```

**Debug Tip**: Print intermediate values to catch boundary issues:
```rust
for i in 2..=n {
    dp[i] = dp[i-1] + dp[i-2];
    println!("dp[{}] = {}", i, dp[i]);  // Debug output
}
```

---

## Performance Issues

### Exponential Time Complexity (Forgot Memoization)

**Symptom**: Code runs forever or takes minutes for inputs >15.

**Cause**: Missing memoization - solving same subproblems repeatedly.

**Solution**: Add cache lookup and insertion:
```rust
// ❌ No memoization
fn fib(n: u64) -> u64 {
    if n <= 1 { return n; }
    fib(n-1) + fib(n-2)  // Exponential time!
}

// ✅ With memoization
fn fib(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 { return n; }
    
    if let Some(&result) = memo.get(&n) {
        return result;  // O(1) cache hit
    }
    
    let result = fib(n-1, memo) + fib(n-2, memo);
    memo.insert(n, result);
    result
}
```

**See**: Step 2 for memoization fundamentals.

---

### Cache Not Working (Incorrect Key Type)

**Symptom**: Cache size stays at 0 or grows unexpectedly large.

**Cause**: Using wrong type for cache keys.

**Solution**: Ensure keys match the problem's state:
```rust
// ❌ Wrong: Allocating new Strings for each key
let mut cache: HashMap<String, bool> = HashMap::new();
cache.insert(design.to_string(), result);  // Allocates!

// ✅ Correct: Zero-copy references
let mut cache: HashMap<&str, bool> = HashMap::new();
cache.insert(design, result);  // No allocation
```

**See**: Step 3 for zero-copy caching.

---

## Logic Errors

### Boolean vs Counting Confusion

**Symptom**: Part 2 (counting) returns 0 or 1 when multiple solutions exist.

**Cause**: Using boolean logic (OR/short-circuit) instead of sum aggregation.

**Solution**: Transform boolean to counting:
```rust
// Boolean (Part 1): Stop at first success
for pattern in patterns {
    if let Some(rest) = design.strip_prefix(pattern) {
        if can_make(patterns, rest, memo) {
            return true;  // ← Short-circuit!
        }
    }
}
return false;

// Counting (Part 2): Sum all paths
let mut total = 0;
for pattern in patterns {
    if let Some(rest) = design.strip_prefix(pattern) {
        total += count_ways(patterns, rest, memo);  // ← Accumulate!
    }
}
return total;
```

**See**: Step 5 for boolean → counting transformation.

---

### Wrong Base Case

**Symptom**: All results return 0 or incorrect small values.

**Cause**: Incorrect base case value.

**Solution**: Match base case to problem semantics:
```rust
// Boolean existence
if design.is_empty() {
    return true;  // Empty string is valid (base case succeeded)
}

// Counting ways
if design.is_empty() {
    return 1;  // One way to make empty string
}

// Optimization (minimum)
if amount == 0 {
    return 0;  // Zero coins needed for zero amount
}
```

**Debug Tip**: Manually trace through the smallest possible input to verify base case.

---

## Testing Issues

### Tests Fail with "Assertion failed"

**Symptom**:
```
thread 'tests::test_basic' panicked at 'assertion failed: `(left == right)`
  left: `false`,
 right: `true`
```

**Solution**: Add debug output to understand the failure:
```rust
#[test]
fn test_basic() {
    let mut cache = HashMap::new();
    let result = can_make(&patterns, "xyz", &mut cache);
    
    // Add debug output
    println!("Result: {}", result);
    println!("Cache: {:?}", cache);
    
    assert_eq!(result, expected);
}
```

Run with `--nocapture` to see println output:
```bash
cargo test -- --nocapture
```

---

### Performance Tests Timeout

**Symptom**: Tests run forever or exceed time limit.

**Solution**: Use smaller test cases or increase timeout:
```rust
#[test]
#[timeout(5000)]  // 5 second timeout (requires test-timeout crate)
fn test_large_input() {
    // Test with reasonable size
    let n = 1000;  // Not 1000000!
    let result = solve(n);
    assert!(result > 0);
}
```

Or use `cargo test --release` for optimized builds.

---

## Common Pitfalls

### 1. Forgetting to Clear Cache Between Test Cases

**Problem**: Cache from previous test affects current test.

**Solution**:
```rust
#[test]
fn test_each_case() {
    for (input, expected) in test_cases {
        let mut cache = HashMap::new();  // ← Fresh cache each time
        let result = solve(input, &mut cache);
        assert_eq!(result, expected);
    }
}
```

---

### 2. Using `==` Instead of `.get()` for HashMap

**Problem**: Ownership/borrowing errors.

**Solution**:
```rust
// ❌ Wrong
if memo[design] {  // Moves design or requires Copy trait
    return memo[design];
}

// ✅ Correct
if let Some(&result) = memo.get(design) {
    return result;
}
```

---

### 3. Mutable Borrow While Immutable Borrow Exists

**Problem**:
```
error[E0502]: cannot borrow `*memo` as mutable because it is also borrowed as immutable
```

**Solution**: Separate the lookup from the recursive call:
```rust
// ❌ Causes borrow checker error
if memo.get(design).is_some() {
    return memo.get(design).unwrap();  // Two borrows!
}

// ✅ Pattern match consumes the borrow
if let Some(&result) = memo.get(design) {
    return result;  // Borrow ends here
}
```

---

## Getting Help

If you encounter an issue not listed here:

1. **Check the step examples**: Each step has detailed comments and tests
2. **Review Step 3**: Most issues relate to lifetimes and borrowing
3. **Add debug output**: `println!` is your friend
4. **Simplify the input**: Test with the smallest possible case
5. **Compare with working code**: Look at the step examples

### Useful Commands

```bash
# Check compilation without running
cargo check --examples

# Run specific example
cargo run --example step1_naive_recursion

# Run tests with output
cargo test -- --nocapture

# Run tests with backtrace
RUST_BACKTRACE=1 cargo test

# Check for common mistakes
cargo clippy --examples
```

---

**Remember**: The tutorial is designed to be a learning experience. Encountering and fixing errors is part of mastering dynamic programming in Rust!

*Last updated: Tutorial creation*
