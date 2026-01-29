# Sliding Window Pattern

**Category**: Algorithm Design Pattern  
**Difficulty**: Fundamental  
**Applications**: Array processing, string algorithms, time series  
**Time Complexity**: Typically $O(n)$ to $O(n \times k)$  
**Related**: [[two-pointer-technique]], [[kadanes-algorithm]], [[array-algorithms]]

## Definition

The **sliding window pattern** is an algorithmic technique for processing contiguous subsequences (windows) of a fixed or variable size within an array, string, or sequence.

**Metaphor**: Imagine a physical window sliding across a row of items, allowing you to see only a fixed number of items at any time.

## Core Concept

### Fixed-Size Window

**Pattern**:
```
Array: [a, b, c, d, e, f, g, h]
         ↓
Window of size 3:
         [a, b, c]              → Process
            [b, c, d]           → Process
               [c, d, e]        → Process
                  [d, e, f]     → Process
                     [e, f, g]  → Process
                        [f, g, h] → Process
```

**Properties**:
- Window size $k$ is constant
- Process $(n - k + 1)$ windows for array of length $n$
- Each window shares $k-1$ elements with adjacent windows

### Variable-Size Window

**Pattern**:
```
Expand window when condition met:
[a, b, c, d, e, f]
 ↑     ↑
 L     R  → Expand right (R++)

Contract window when condition violated:
[a, b, c, d, e, f]
    ↑     ↑
    L     R  → Contract left (L++)
```

**Properties**:
- Window size changes dynamically
- Two pointers: left (L) and right (R)
- Adjust pointers based on problem constraints

## Types of Sliding Window Problems

### 1. Fixed Window Size

**Characteristics**:
- Window size $k$ given explicitly
- Process every window position
- Track aggregate (sum, product, max, etc.)

**Examples**:
- Maximum sum of $k$ consecutive elements
- [[project-euler-p008]] - Maximum product of 13 consecutive digits
- Average of sliding windows (moving average)

**Template**:
```rust
fn fixed_window<T>(arr: &[T], k: usize) -> Result<T> {
    if arr.len() < k {
        return Err("Array too short");
    }
    
    let mut best = process_window(&arr[0..k]);
    
    for i in 1..=arr.len() - k {
        let window = &arr[i..i + k];
        let current = process_window(window);
        best = update_best(best, current);
    }
    
    Ok(best)
}
```

### 2. Variable Window Size (Two Pointers)

**Characteristics**:
- Window expands/contracts based on constraints
- Maintain validity condition
- Often finds optimal window

**Examples**:
- Smallest subarray with sum ≥ target
- Longest substring without repeating characters
- Minimum window substring containing all characters

**Template**:
```rust
fn variable_window<T>(arr: &[T], target: T) -> usize {
    let mut left = 0;
    let mut best_size = 0;
    
    for right in 0..arr.len() {
        // Expand window by including arr[right]
        add_to_window(arr[right]);
        
        // Contract window while invalid
        while !is_valid() && left <= right {
            remove_from_window(arr[left]);
            left += 1;
        }
        
        // Update best result
        if is_valid() {
            best_size = max(best_size, right - left + 1);
        }
    }
    
    best_size
}
```

### 3. Sliding Window with Auxiliary Data Structure

**Characteristics**:
- Use HashMap, HashSet, Deque, etc. to track window state
- Efficiently add/remove elements
- Maintain window invariants

**Examples**:
- Longest substring with at most K distinct characters (HashMap)
- Sliding window maximum (Deque - monotonic queue)
- Anagrams in string (HashMap for character counts)

**Template**:
```rust
fn window_with_map<T: Hash + Eq>(arr: &[T], k: usize) -> Result {
    let mut window_state = HashMap::new();
    let mut left = 0;
    
    for right in 0..arr.len() {
        // Add element to window
        *window_state.entry(arr[right]).or_insert(0) += 1;
        
        // Maintain window size or validity
        while should_contract() {
            *window_state.get_mut(&arr[left]).unwrap() -= 1;
            if window_state[&arr[left]] == 0 {
                window_state.remove(&arr[left]);
            }
            left += 1;
        }
        
        // Process current window
        update_result(&window_state);
    }
    
    result
}
```

## Complexity Analysis

### Time Complexity

**Fixed Window**:
- **Setup**: $O(k)$ to initialize first window
- **Sliding**: $(n - k)$ iterations
- **Per iteration**: $O(k)$ to process window (naive)
- **Total**: $O(n \times k)$ or $O(n)$ with optimization

**Variable Window**:
- Each element added once (right pointer)
- Each element removed at most once (left pointer)
- **Total**: $O(n)$ with proper implementation

### Space Complexity

**Basic Window**: $O(1)$ if using slice/pointer
**With State**: $O(k)$ to $O(n)$ depending on auxiliary data structure

## Optimization Techniques

### 1. Incremental Updates

**Problem**: Recalculating entire window is wasteful

**Solution**: Update incrementally when sliding

**Example - Sum of K elements**:
```rust
// Naive: O(n × k)
for i in 0..=arr.len() - k {
    sum = arr[i..i+k].iter().sum();  // Recalculate entire sum!
}

// Optimized: O(n)
let mut sum = arr[0..k].iter().sum();  // Initial window
max_sum = sum;

for i in k..arr.len() {
    sum = sum - arr[i - k] + arr[i];  // Drop left, add right
    max_sum = max(max_sum, sum);
}
```

**Savings**: $O(k)$ → $O(1)$ per iteration

### 2. Early Termination

**Strategy**: Stop processing when optimal solution found

**Example - Product contains zero**:
```rust
// Check for zero before multiplying
if window.contains(&0) {
    continue;  // Product will be 0, skip entire window
}

let product = window.iter().product();
```

**Benefit**: Skip unnecessary computation

### 3. Precomputation

**Strategy**: Precompute cumulative values for range queries

**Example - Prefix sums for sum queries**:
```rust
// Precompute prefix sums: O(n)
let prefix: Vec<i32> = arr.iter()
    .scan(0, |sum, &x| { *sum += x; Some(*sum) })
    .collect();

// Range sum [i, j] in O(1)
fn range_sum(prefix: &[i32], i: usize, j: usize) -> i32 {
    if i == 0 { prefix[j] } 
    else { prefix[j] - prefix[i - 1] }
}
```

**Trade-off**: $O(n)$ preprocessing, $O(1)$ queries

## Common Pitfalls

### 1. Off-by-One Errors

**Problem**: Incorrect window boundaries

**Example**:
```rust
// ❌ Wrong: Skips last window
for i in 0..arr.len() - k {  // Should be arr.len() - k + 1
    process(&arr[i..i+k]);
}

// ✅ Correct:
for i in 0..=arr.len() - k {  // Inclusive range
    process(&arr[i..i+k]);
}
```

### 2. Empty Window Handling

**Problem**: Array shorter than window size

**Solution**:
```rust
if arr.len() < k {
    return Err("Window size exceeds array length");
}
```

### 3. Integer Overflow in Products

**Problem**: Product of large numbers overflows

**Solution**:
```rust
// Use larger type or check for overflow
let product = window.iter()
    .try_fold(1u64, |acc, &x| acc.checked_mul(x as u64))
    .unwrap_or(u64::MAX);
```

### 4. Variable Window Pointer Logic

**Problem**: Left pointer overtakes right pointer

**Solution**:
```rust
while condition_violated() && left <= right {
    left += 1;  // Always check left <= right
}
```

## Real-World Applications

### 1. Network Traffic Analysis

**Problem**: Detect traffic spikes in sliding time window

**Solution**:
```rust
// Track request count in last 60 seconds
struct RateLimiter {
    window: VecDeque<Instant>,
    window_size: Duration,
}

impl RateLimiter {
    fn add_request(&mut self, now: Instant) -> bool {
        // Remove requests older than window
        while let Some(&t) = self.window.front() {
            if now.duration_since(t) > self.window_size {
                self.window.pop_front();
            } else {
                break;
            }
        }
        
        // Check if under limit
        if self.window.len() < MAX_REQUESTS {
            self.window.push_back(now);
            true
        } else {
            false
        }
    }
}
```

### 2. Moving Averages in Finance

**Problem**: Calculate moving average of stock prices

**Solution**:
```rust
fn moving_average(prices: &[f64], period: usize) -> Vec<f64> {
    let mut sum = prices[..period].iter().sum();
    let mut averages = vec![sum / period as f64];
    
    for i in period..prices.len() {
        sum = sum - prices[i - period] + prices[i];
        averages.push(sum / period as f64);
    }
    
    averages
}
```

### 3. DNA Sequence Analysis

**Problem**: Find GC-content in sliding windows

**Solution**:
```rust
fn gc_content(dna: &str, window_size: usize) -> Vec<f64> {
    dna.as_bytes()
        .windows(window_size)
        .map(|window| {
            let gc_count = window.iter()
                .filter(|&&b| b == b'G' || b == b'C')
                .count();
            gc_count as f64 / window_size as f64
        })
        .collect()
}
```

## LeetCode & AoC Examples

### Classic Problems

**LeetCode**:
- #3: Longest Substring Without Repeating Characters
- #76: Minimum Window Substring  
- #209: Minimum Size Subarray Sum
- #239: Sliding Window Maximum
- #438: Find All Anagrams in a String
- #567: Permutation in String

**Advent of Code**:
- [[project-euler-p008]] - Maximum product in series
- AoC 2022 Day 6 - Tuning Trouble (first unique marker)
- AoC 2021 Day 1 - Sonar Sweep (sliding sums)

### Pattern Recognition

**Identify sliding window when**:
- Problem mentions "consecutive", "contiguous", "subarray"
- Need to find optimal window (min/max length, sum, etc.)
- Constraints involve window size or window properties
- Brute force would check all subarrays ($O(n^2)$ or worse)

## Comparison with Related Techniques

### Sliding Window vs Two Pointers

**Sliding Window**:
- Specific application of two pointers
- Focus: Contiguous subsequence processing
- Pointers move together (window slides)

**Two Pointers** (general):
- Broader technique
- Pointers can be anywhere (not just window boundaries)
- Examples: Pair sum, remove duplicates

### Sliding Window vs Dynamic Programming

**Sliding Window**:
- ✅ $O(n)$ time with proper optimization
- ✅ $O(1)$ to $O(k)$ space
- ❌ Only works for contiguous sequences
- ❌ Limited to problems with sliding property

**Dynamic Programming**:
- ✅ Handles non-contiguous subsequences
- ✅ Solves wider class of optimization problems
- ❌ Often $O(n^2)$ time
- ❌ $O(n)$ to $O(n^2)$ space

**Example**: Longest increasing subsequence
- Non-contiguous → DP required
- Sliding window doesn't apply

## Code Templates

### Rust: Fixed Window Maximum

```rust
pub fn max_sliding_window(nums: &[i32], k: usize) -> Vec<i32> {
    if nums.len() < k { return vec![]; }
    
    let mut result = Vec::new();
    
    for i in 0..=nums.len() - k {
        let window_max = nums[i..i + k].iter().max().unwrap();
        result.push(*window_max);
    }
    
    result
}
```

### Rust: Variable Window (Longest Substring)

```rust
use std::collections::HashSet;

pub fn longest_unique_substring(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut seen = HashSet::new();
    let mut left = 0;
    let mut max_len = 0;
    
    for right in 0..chars.len() {
        // Contract window until unique
        while seen.contains(&chars[right]) {
            seen.remove(&chars[left]);
            left += 1;
        }
        
        // Add current character
        seen.insert(chars[right]);
        max_len = max_len.max(right - left + 1);
    }
    
    max_len
}
```

## Rust-Specific Features

### Using `.windows()` Iterator

```rust
// Built-in sliding window iterator
let nums = vec![1, 2, 3, 4, 5];
for window in nums.windows(3) {
    println!("{:?}", window);  // [1,2,3], [2,3,4], [3,4,5]
}
```

**Advantages**:
- ✅ Immutable borrows (safe)
- ✅ Zero-copy slices
- ✅ Iterator combinators

**Limitations**:
- ❌ Fixed size only (no variable windows)
- ❌ Immutable (can't modify during iteration)

### Zero-Copy Window Processing

```rust
// Slices don't copy data - they reference original array
let arr = [1, 2, 3, 4, 5, 6, 7, 8];
let window = &arr[2..5];  // Slice: [3, 4, 5] - no allocation!
```

## Performance Considerations

### When to Use Sliding Window

✅ **Use when**:
- Processing contiguous sequences
- Window size is small relative to array
- Need $O(n)$ solution for subarray problems

❌ **Don't use when**:
- Need non-contiguous subsequences (use DP)
- Random access patterns (different data structure)
- Window doesn't capture problem structure

### Benchmarking

**Measure impact of optimizations**:
```rust
use criterion::{black_box, Criterion};

fn bench_sliding_window(c: &mut Criterion) {
    let data: Vec<u64> = (0..10000).collect();
    
    c.bench_function("naive", |b| {
        b.iter(|| naive_max_window(black_box(&data), 100))
    });
    
    c.bench_function("optimized", |b| {
        b.iter(|| optimized_max_window(black_box(&data), 100))
    });
}
```

## Connections to Mathematics

### Combinatorics

**Number of windows** of size $k$ in array of length $n$:
$$\binom{n}{k, 1} = n - k + 1$$

**Interpretation**: Choose starting position for window

### Recurrence Relations

**Window sum update**:
$$S_i = S_{i-1} - a_{i-k} + a_i$$

Where $S_i$ = sum of window ending at position $i$

**Derivation**:
```
Window [i-k+1, ..., i-1, i]:
  S_i = sum(a[i-k+1..=i])
  
Previous window [i-k, ..., i-2, i-1]:
  S_{i-1} = sum(a[i-k..=i-1])
  
Relationship:
  S_i = S_{i-1} - a[i-k] + a[i]  (drop left, add right)
```

## Summary

**Key Takeaways**:
1. Sliding window transforms $O(n \times k)$ to $O(n)$ with incremental updates
2. Two main types: fixed size and variable size
3. Essential pattern for contiguous subsequence problems
4. Rust's `.windows()` provides safe, efficient iteration
5. Always validate window boundaries and handle edge cases

**When to reach for sliding window**:
- ✅ "consecutive", "contiguous", "subarray" in problem description
- ✅ Optimization over all windows needed
- ✅ Can maintain window state incrementally

## References

### Implementations
- [[project-euler-p008]] - Fixed window maximum product
- AoC solutions using sliding window pattern

### Related Concepts
- [[two-pointer-technique]] - General two-pointer patterns
- [[kadanes-algorithm]] - Maximum subarray (DP + sliding window hybrid)
- [[array-algorithms]] - Comprehensive array processing techniques
- [[monotonic-queue]] - Sliding window maximum optimization

### Further Reading
- "Elements of Programming Interviews" - Sliding window chapter
- LeetCode Sliding Window Problems: https://leetcode.com/tag/sliding-window/

## Tags
*Tags: #algorithms #sliding-window #two-pointers #array-processing #optimization #pattern #time-complexity #problem-solving*

---

**Created**: January 29, 2026  
**Last Updated**: January 29, 2026  
**Status**: Complete ✓
